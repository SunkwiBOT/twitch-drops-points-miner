use std::collections::{BTreeSet, HashMap};
use std::time::Duration;

use chrono::{DateTime, Utc};
use futures_util::{SinkExt, StreamExt};
use reqwest_websocket::Message;
use serde_json::{json, Value};
use tokio::sync::{mpsc, watch};
use tokio::task::JoinSet;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use super::{
    hold_socket, open_socket, text, Socket, SocketPlace, TwitchClient, PUBSUB_URL,
    SOCKET_WARN_AFTER_TRIES,
};
use crate::domain::{PredictionEvent, PredictionOutcome};
use crate::error::{Error, Result};
use crate::requests::{RequestKind, SocketEvent};
use crate::util::random_hex;

// ! Twitch PubSub takes at most 50 topics per socket.
const TOPICS_PER_SOCKET: usize = 50;
const SHARDS: usize = 4;
pub const MAX_CHANNELS: usize = (SHARDS * TOPICS_PER_SOCKET - 4) / 5;
const PING_INTERVAL: Duration = Duration::from_secs(240);
const REPLY_TIMEOUT: Duration = Duration::from_secs(10);
// ! Twitch's prediction windows last 30 seconds to 30 minutes: a longer one in
// a message is a broken payload.
const PREDICTION_WINDOW_MAX_SECONDS: i64 = 3600;

pub const WATCH_STREAK_REASON: &str = "WATCH_STREAK";

const PLACE: SocketPlace = SocketPlace {
    kind: RequestKind::Pubsub,
    address: PUBSUB_URL,
    name: "pubsub",
};

#[derive(Debug, Clone, PartialEq)]
pub enum PubSubEvent {
    StreamUp {
        channel_id: String,
    },
    StreamDown {
        channel_id: String,
    },
    ViewCount {
        channel_id: String,
        viewers: u64,
    },
    PointsEarned {
        channel_id: String,
        earned: i64,
        balance: Option<i64>,
        reason: String,
    },
    PointsSpent {
        channel_id: String,
        balance: i64,
    },
    ClaimAvailable {
        channel_id: String,
        claim_id: String,
    },
    Raid {
        channel_id: String,
        raid_id: String,
        target_login: String,
    },
    Moment {
        channel_id: String,
        moment_id: String,
    },
    PredictionCreated(PredictionEvent),
    PredictionUpdated(PredictionEvent),
    PredictionMade {
        event_id: String,
        outcome_id: String,
        points: i64,
    },
    PredictionResult {
        event_id: String,
        result: String,
        points_won: i64,
    },
    DropProgress {
        drop_id: String,
        current_minutes: u32,
        required_minutes: u32,
        channel_id: Option<String>,
    },
    DropClaim {
        drop_id: String,
        instance_id: String,
    },
    StreakRecovered {
        channel_id: String,
    },
    Unauthorized,
    Reconnected,
}

// ! Twitch repeats `raid_update_v2` every second of the countdown. A failed
// join is tried again no more often than this, or a refusal would repeat
// every second.
pub const RAID_JOIN_RETRY: Duration = Duration::from_secs(10);

#[derive(Debug)]
struct RaidJoin {
    raid_id: String,
    joined: bool,
    tried_at: Instant,
}

#[derive(Debug, Default)]
pub struct RaidJoins(HashMap<String, RaidJoin>);

impl RaidJoins {
    pub fn due(&mut self, channel_id: &str, raid_id: &str, now: Instant) -> bool {
        if let Some(join) = self.0.get(channel_id) {
            let same = join.raid_id == raid_id;

            if same && (join.joined || now.duration_since(join.tried_at) < RAID_JOIN_RETRY) {
                return false;
            }
        }

        self.0.insert(
            channel_id.to_owned(),
            RaidJoin {
                raid_id: raid_id.to_owned(),
                joined: false,
                tried_at: now,
            },
        );

        true
    }

    pub fn joined(&mut self, channel_id: &str, raid_id: &str) {
        if let Some(join) = self.0.get_mut(channel_id) {
            if join.raid_id == raid_id {
                join.joined = true;
            }
        }
    }
}

pub fn topics_for(user_id: &str, channels: &[String], predictions: &[String]) -> Vec<String> {
    let mut topics = vec![
        format!("community-points-user-v1.{user_id}"),
        format!("user-drop-events.{user_id}"),
        format!("predictions-user-v1.{user_id}"),
        format!("viewer-milestones.{user_id}"),
    ];

    for id in channels.iter().take(MAX_CHANNELS).collect::<BTreeSet<_>>() {
        topics.push(format!("video-playback-by-id.{id}"));
        topics.push(format!("raid.{id}"));
        topics.push(format!("community-moments-channel-v1.{id}"));

        if predictions.contains(id) {
            topics.push(format!("predictions-channel-v1.{id}"));
        }
    }

    topics
}

pub struct PubSub {
    shards: Vec<watch::Sender<BTreeSet<String>>>,
    cancel: CancellationToken,
    tasks: JoinSet<()>,
}

impl PubSub {
    pub fn start(
        client: TwitchClient,
        events: mpsc::Sender<PubSubEvent>,
        cancel: CancellationToken,
    ) -> Self {
        let mut tasks = JoinSet::new();
        let shards = (0..SHARDS)
            .map(|_| {
                let (send, receive) = watch::channel(BTreeSet::new());
                tasks.spawn(worker(
                    client.clone(),
                    receive,
                    events.clone(),
                    cancel.clone(),
                ));

                send
            })
            .collect();

        Self {
            shards,
            cancel,
            tasks,
        }
    }

    pub fn set_topics(&self, topics: Vec<String>) {
        for (index, sender) in self.shards.iter().enumerate() {
            let desired: BTreeSet<String> = topics
                .iter()
                .skip(index * TOPICS_PER_SOCKET)
                .take(TOPICS_PER_SOCKET)
                .cloned()
                .collect();

            sender.send_if_modified(|current| {
                if *current == desired {
                    false
                } else {
                    *current = desired;
                    true
                }
            });
        }
    }

    pub async fn close(&mut self) {
        self.cancel.cancel();

        while self.tasks.join_next().await.is_some() {}
    }
}

async fn worker(
    client: TwitchClient,
    mut desired: watch::Receiver<BTreeSet<String>>,
    events: mpsc::Sender<PubSubEvent>,
    cancel: CancellationToken,
) {
    let mut failures = 0u32;
    let mut dropped = false;

    loop {
        if desired.borrow().is_empty() {
            tokio::select! { biased;
                _ = cancel.cancelled() => return,
                changed = desired.changed() => { if changed.is_err() { return; } }
            }

            continue;
        }

        let started = Instant::now();
        let event = if dropped {
            SocketEvent::Reconnect
        } else {
            SocketEvent::Connect
        };
        let opened = tokio::select! { biased;
            _ = cancel.cancelled() => return,
            opened = open_socket(&client, &PLACE, event) => opened,
        };
        let result = match opened {
            Ok(mut socket) => {
                let session = connection(&client, &mut socket, &mut desired, &events);
                let Some(result) = hold_socket(&client, &PLACE, &cancel, session).await else {
                    return;
                };

                result
            }
            Err(error) => Err(error),
        };

        dropped = result.is_err();

        // ! The account may be waiting for this worker to stop, its queue
        // full: the stop wins over the message.
        if matches!(result, Err(Error::Unauthorized)) {
            tokio::select! { biased;
                _ = cancel.cancelled() => return,
                _ = events.send(PubSubEvent::Unauthorized) => {}
            }

            tokio::select! { biased;
                _ = cancel.cancelled() => return,
                _ = tokio::time::sleep(Duration::from_secs(60)) => {}
            }

            continue;
        }

        if started.elapsed() > Duration::from_secs(60) {
            failures = 0;
        }

        let Err(error) = result else {
            failures = 0;

            continue;
        };

        if failures == 0 {
            tracing::info!(account = %client.login, %error, "PubSub connection dropped; reconnecting");
        } else if failures == SOCKET_WARN_AFTER_TRIES {
            tracing::warn!(
                account = %client.login,
                tries = failures,
                %error,
                "PubSub still cannot reconnect; trying again"
            );
        }

        let delay = Duration::from_secs(1 << failures.min(6));

        failures = failures.saturating_add(1);

        tokio::select! { biased;
            _ = cancel.cancelled() => return,
            _ = tokio::time::sleep(delay) => {}
        }
    }
}

async fn send(socket: &mut Socket, value: Value) -> Result<()> {
    tokio::time::timeout(REPLY_TIMEOUT, socket.send(Message::Text(value.to_string())))
        .await
        .map_err(|_| Error::Network("pubsub send timeout".into()))?
        .map_err(|e| Error::Network(format!("pubsub send: {e}")))
}

async fn listen(
    socket: &mut Socket,
    verb: &str,
    topics: Vec<String>,
    token: &str,
    pending: &mut HashMap<String, Vec<String>>,
) -> Result<()> {
    for chunk in topics.chunks(10) {
        let nonce = random_hex(16);

        send(
            socket,
            json!({"type": verb, "nonce": nonce, "data": {"topics": chunk, "auth_token": token}}),
        )
        .await?;
        pending.insert(nonce, chunk.to_vec());
    }

    Ok(())
}

async fn connection(
    client: &TwitchClient,
    socket: &mut Socket,
    desired: &mut watch::Receiver<BTreeSet<String>>,
    events: &mpsc::Sender<PubSubEvent>,
) -> Result<()> {
    let token = client.token().await;
    let mut subscribed: BTreeSet<String> = BTreeSet::new();
    let mut pending: HashMap<String, Vec<String>> = HashMap::new();
    let mut ping = tokio::time::interval(PING_INTERVAL);

    ping.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    let mut awaiting_pong: Option<Instant> = None;
    let mut first = true;

    loop {
        let wanted = desired.borrow().clone();
        let to_add: Vec<String> = wanted.difference(&subscribed).cloned().collect();
        let to_remove: Vec<String> = subscribed.difference(&wanted).cloned().collect();

        if !to_add.is_empty() {
            listen(socket, "LISTEN", to_add.clone(), &token, &mut pending).await?;
            subscribed.extend(to_add);
        }

        if !to_remove.is_empty() {
            listen(socket, "UNLISTEN", to_remove.clone(), &token, &mut pending).await?;

            for topic in to_remove {
                subscribed.remove(&topic);
            }
        }

        if first {
            first = false;
            let _ = events.try_send(PubSubEvent::Reconnected);
        }

        if subscribed.is_empty() {
            return Ok(());
        }

        tokio::select! { biased;
            changed = desired.changed() => {
                if changed.is_err() { return Ok(()); }
            }
            _ = sleep_until_some(awaiting_pong.map(|sent| sent + REPLY_TIMEOUT)) => {
                return Err(Error::Network("pubsub pong timeout".into()));
            }
            _ = ping.tick() => {
                send(socket, json!({"type": "PING"})).await?;
                awaiting_pong = Some(Instant::now());
            }
            message = socket.next() => {
                let Some(message) = message else { return Err(Error::Network("pubsub closed".into())); };
                let message = message.map_err(|e| Error::Network(format!("pubsub read: {e}")))?;
                let payload = match message {
                    Message::Text(text) => text,
                    Message::Ping(data) => {
                        let _ = socket.send(Message::Pong(data)).await;
                        continue;
                    }
                    Message::Close { .. } => return Err(Error::Network("pubsub close frame".into())),
                    _ => continue,
                };
                let value: Value = match serde_json::from_str(&payload) { Ok(v) => v, Err(_) => continue };

                match value["type"].as_str().unwrap_or("") {
                    "PONG" => awaiting_pong = None,
                    "RECONNECT" => return Err(Error::Network("pubsub asked to reconnect".into())),
                    "RESPONSE" => {
                        let asked = value["nonce"].as_str().and_then(|nonce| pending.remove(nonce)).unwrap_or_default();

                        match value["error"].as_str().unwrap_or("") {
                            "" => {}
                            "ERR_BADAUTH" => return Err(Error::Unauthorized),
                            other => tracing::warn!(
                                account = %client.login,
                                error = other,
                                topics = %topic_kinds(&asked),
                                "PubSub LISTEN refused"
                            ),
                        }
                    }
                    "MESSAGE" => {
                        if let Some(event) = parse_message(&value["data"]) {
                            if events.send(event).await.is_err() { return Ok(()); }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

async fn sleep_until_some(at: Option<Instant>) {
    match at {
        Some(at) => tokio::time::sleep_until(at).await,
        None => std::future::pending().await,
    }
}

fn topic_kinds(topics: &[String]) -> String {
    topics
        .iter()
        .filter_map(|topic| topic.split('.').next())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
        .join(",")
}

fn topic_channel(topic: &str) -> Option<String> {
    topic
        .rsplit('.')
        .next()
        .filter(|v| !v.is_empty())
        .map(str::to_owned)
}

pub fn parse_message(data: &Value) -> Option<PubSubEvent> {
    let topic = data["topic"].as_str()?;
    let message: Value = serde_json::from_str(data["message"].as_str()?).ok()?;
    let kind = message["type"].as_str().unwrap_or("");
    let body = &message["data"];
    let prefix = topic.split('.').next().unwrap_or("");

    match prefix {
        "video-playback-by-id" => {
            let channel_id = topic_channel(topic)?;

            match kind {
                "stream-up" => Some(PubSubEvent::StreamUp { channel_id }),
                "stream-down" => Some(PubSubEvent::StreamDown { channel_id }),
                "viewcount" => Some(PubSubEvent::ViewCount {
                    channel_id,
                    viewers: message["viewers"].as_u64().unwrap_or(0),
                }),
                _ => None,
            }
        }
        "community-points-user-v1" => match kind {
            "points-earned" => Some(PubSubEvent::PointsEarned {
                channel_id: text(&body["channel_id"])?,
                earned: body["point_gain"]["total_points"].as_i64()?,
                balance: body["balance"]["balance"].as_i64(),
                reason: text(&body["point_gain"]["reason_code"]).unwrap_or_default(),
            }),
            "points-spent" => Some(PubSubEvent::PointsSpent {
                channel_id: text(&body["balance"]["channel_id"])?,
                balance: body["balance"]["balance"].as_i64()?,
            }),
            "claim-available" => Some(PubSubEvent::ClaimAvailable {
                channel_id: text(&body["claim"]["channel_id"])?,
                claim_id: text(&body["claim"]["id"])?,
            }),
            _ => None,
        },
        "raid" => {
            if kind != "raid_update_v2" && kind != "raid_update" {
                return None;
            }

            let raid = &message["raid"];

            Some(PubSubEvent::Raid {
                channel_id: topic_channel(topic)?,
                raid_id: text(&raid["id"])?,
                target_login: text(&raid["target_login"]).unwrap_or_default(),
            })
        }
        "community-moments-channel-v1" => {
            if kind != "active" {
                return None;
            }

            Some(PubSubEvent::Moment {
                channel_id: topic_channel(topic)?,
                moment_id: text(&body["moment_id"])?,
            })
        }
        "predictions-channel-v1" => {
            let event = parse_prediction(&body["event"], &topic_channel(topic)?)?;

            match kind {
                "event-created" => Some(PubSubEvent::PredictionCreated(event)),
                "event-updated" => Some(PubSubEvent::PredictionUpdated(event)),
                _ => None,
            }
        }
        "predictions-user-v1" => {
            let prediction = &body["prediction"];

            match kind {
                "prediction-made" => Some(PubSubEvent::PredictionMade {
                    event_id: text(&prediction["event_id"])?,
                    outcome_id: text(&prediction["outcome_id"])?,
                    points: prediction["points"].as_i64().unwrap_or(0),
                }),
                "prediction-result" => Some(PubSubEvent::PredictionResult {
                    event_id: text(&prediction["event_id"])?,
                    result: text(&prediction["result"]["type"]).unwrap_or_default(),
                    points_won: prediction["result"]["points_won"].as_i64().unwrap_or(0),
                }),
                _ => None,
            }
        }
        "user-drop-events" => match kind {
            "drop-progress" => Some(PubSubEvent::DropProgress {
                drop_id: text(&body["drop_id"])?,
                current_minutes: body["current_progress_min"].as_u64().unwrap_or(0) as u32,
                required_minutes: body["required_progress_min"].as_u64().unwrap_or(0) as u32,
                channel_id: text(&body["channel_id"]),
            }),
            "drop-claim" => Some(PubSubEvent::DropClaim {
                drop_id: text(&body["drop_id"])?,
                instance_id: text(&body["drop_instance_id"])?,
            }),
            _ => None,
        },
        "viewer-milestones" => {
            if kind != "streak-recovered" {
                return None;
            }

            Some(PubSubEvent::StreakRecovered {
                channel_id: text(&body["channel_id"]).unwrap_or_default(),
            })
        }
        _ => None,
    }
}

fn parse_prediction(event: &Value, channel_id: &str) -> Option<PredictionEvent> {
    let outcomes = event["outcomes"]
        .as_array()?
        .iter()
        .filter_map(|o| {
            Some(PredictionOutcome {
                id: text(&o["id"])?,
                title: text(&o["title"]).unwrap_or_default(),
                color: text(&o["color"]).unwrap_or_default(),
                total_users: o["total_users"].as_i64().unwrap_or(0),
                total_points: o["total_points"].as_i64().unwrap_or(0),
                top_points: o["top_predictors"]
                    .as_array()
                    .and_then(|p| p.iter().filter_map(|p| p["points"].as_i64()).max())
                    .unwrap_or(0),
                percentage_users: 0.0,
                odds: 0.0,
                odds_percentage: 0.0,
            })
        })
        .collect();
    let mut prediction = PredictionEvent {
        id: text(&event["id"])?,
        channel_id: text(&event["channel_id"]).unwrap_or_else(|| channel_id.to_owned()),
        title: text(&event["title"]).unwrap_or_default(),
        status: text(&event["status"]).unwrap_or_default(),
        created_at: text(&event["created_at"])
            .and_then(|v| v.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(Utc::now),
        window_seconds: event["prediction_window_seconds"]
            .as_i64()
            .unwrap_or(0)
            .clamp(0, PREDICTION_WINDOW_MAX_SECONDS),
        outcomes,
        winning_outcome_id: text(&event["winning_outcome_id"]),
    };

    prediction.update_outcomes();

    Some(prediction)
}
