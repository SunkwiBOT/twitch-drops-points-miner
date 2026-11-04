use std::collections::BTreeSet;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use reqwest_websocket::Message;
use tokio::sync::{mpsc, watch};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use super::{
    hold_socket, open_socket, Socket, SocketPlace, TwitchClient, CHAT_URL, SOCKET_WARN_AFTER_TRIES,
};
use crate::error::{Error, Result};
use crate::requests::{RequestKind, SocketEvent};

pub const MAX_CHANNELS: usize = 100;
// ! Twitch limits joins to 20 every 10 seconds for a normal account.
const JOINS_PER_BURST: usize = 20;
const JOIN_BURST_PAUSE: Duration = Duration::from_secs(11);
const PING_INTERVAL: Duration = Duration::from_secs(240);
const REPLY_TIMEOUT: Duration = Duration::from_secs(15);
const REFUSED_PAUSE: Duration = Duration::from_secs(30 * 60);
const MENTIONS_QUEUE: usize = 32;
const MENTION_TEXT_MAX: usize = 300;

const PLACE: SocketPlace = SocketPlace {
    kind: RequestKind::Chat,
    address: CHAT_URL,
    name: "chat",
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMention {
    pub channel: String,
    pub chatter: String,
    pub text: String,
}

pub struct Chat {
    desired: watch::Sender<BTreeSet<String>>,
    mentions: mpsc::Receiver<ChatMention>,
    cancel: CancellationToken,
    task: Option<JoinHandle<()>>,
}

impl Chat {
    pub fn start(client: TwitchClient, cancel: CancellationToken) -> Self {
        let (desired, receiver) = watch::channel(BTreeSet::new());
        let (mentioned, mentions) = mpsc::channel(MENTIONS_QUEUE);
        let task = tokio::spawn(worker(client, receiver, mentioned, cancel.clone()));

        Self {
            desired,
            mentions,
            cancel,
            task: Some(task),
        }
    }

    // ! Waits forever once the chat is closed, so it can sit in a `select!`.
    pub async fn next_mention(&mut self) -> ChatMention {
        match self.mentions.recv().await {
            Some(mention) => mention,
            None => std::future::pending().await,
        }
    }

    pub fn set_channels(&self, channels: BTreeSet<String>) {
        self.desired.send_if_modified(|current| {
            if *current == channels {
                false
            } else {
                *current = channels;
                true
            }
        });
    }

    pub async fn close(&mut self) {
        self.cancel.cancel();

        if let Some(task) = self.task.take() {
            let _ = task.await;
        }
    }
}

async fn worker(
    client: TwitchClient,
    mut desired: watch::Receiver<BTreeSet<String>>,
    mentioned: mpsc::Sender<ChatMention>,
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

        let started = tokio::time::Instant::now();
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
                let session = connection(&client, &mut socket, &mut desired, &mentioned);
                let Some(result) = hold_socket(&client, &PLACE, &cancel, session).await else {
                    return;
                };

                result
            }
            Err(error) => Err(error),
        };

        dropped = result.is_err();

        let delay = match result {
            Ok(()) => continue,
            Err(Error::Unauthorized) => {
                tracing::warn!(account = %client.login, "Twitch refused the chat sign-in; chat presence paused");
                REFUSED_PAUSE
            }
            Err(error) => {
                if started.elapsed() > Duration::from_secs(60) {
                    failures = 0;
                }

                if failures == 0 {
                    tracing::info!(account = %client.login, %error, "chat connection dropped; reconnecting");
                } else if failures == SOCKET_WARN_AFTER_TRIES {
                    tracing::warn!(
                        account = %client.login,
                        tries = failures,
                        %error,
                        "chat still cannot reconnect; trying again"
                    );
                }

                let delay = Duration::from_secs(1 << failures.min(6));

                failures = failures.saturating_add(1);

                delay
            }
        };

        tokio::select! { biased;
            _ = cancel.cancelled() => return,
            _ = tokio::time::sleep(delay) => {}
        }
    }
}

async fn send(socket: &mut Socket, line: &str) -> Result<()> {
    tokio::time::timeout(
        REPLY_TIMEOUT,
        socket.send(Message::Text(format!("{line}\r\n"))),
    )
    .await
    .map_err(|_| Error::Network("chat send timeout".into()))?
    .map_err(|e| Error::Network(format!("chat send: {e}")))
}

async fn command(socket: &mut Socket, verb: &str, channels: &[String]) -> Result<()> {
    for (index, chunk) in channels.chunks(JOINS_PER_BURST).enumerate() {
        if index > 0 && verb == "JOIN" {
            tokio::time::sleep(JOIN_BURST_PAUSE).await;
        }

        let list: Vec<String> = chunk.iter().map(|login| format!("#{login}")).collect();

        send(socket, &format!("{verb} {}", list.join(","))).await?;
    }

    Ok(())
}

async fn signed_in(socket: &mut Socket) -> Result<()> {
    loop {
        let message = tokio::time::timeout(REPLY_TIMEOUT, socket.next())
            .await
            .map_err(|_| Error::Network("chat sign-in timeout".into()))?
            .ok_or_else(|| Error::Network("chat closed".into()))?
            .map_err(|e| Error::Network(format!("chat read: {e}")))?;
        let Message::Text(text) = message else {
            continue;
        };

        for line in text.lines() {
            if line.contains(" 001 ") {
                return Ok(());
            }

            if line.contains("NOTICE") && line.to_ascii_lowercase().contains("auth") {
                return Err(Error::Unauthorized);
            }
        }
    }
}

async fn connection(
    client: &TwitchClient,
    socket: &mut Socket,
    desired: &mut watch::Receiver<BTreeSet<String>>,
    mentioned: &mpsc::Sender<ChatMention>,
) -> Result<()> {
    let token = client.token().await;

    send(socket, &format!("PASS oauth:{token}")).await?;
    send(socket, &format!("NICK {}", client.login)).await?;
    signed_in(socket).await?;

    let mut joined: BTreeSet<String> = BTreeSet::new();
    let mut ping = tokio::time::interval(PING_INTERVAL);

    ping.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    ping.tick().await;

    let mut awaiting_pong: Option<tokio::time::Instant> = None;

    loop {
        let wanted = desired.borrow().clone();
        let to_join: Vec<String> = wanted.difference(&joined).cloned().collect();
        let to_part: Vec<String> = joined.difference(&wanted).cloned().collect();

        if !to_part.is_empty() {
            command(socket, "PART", &to_part).await?;

            for login in &to_part {
                joined.remove(login);
            }
        }

        if !to_join.is_empty() {
            command(socket, "JOIN", &to_join).await?;
            joined.extend(to_join);
        }

        if joined.is_empty() {
            let _ = socket.send(Message::Text("QUIT\r\n".into())).await;

            return Ok(());
        }

        tokio::select! { biased;
            changed = desired.changed() => {
                if changed.is_err() { return Ok(()); }
            }
            _ = sleep_until_some(awaiting_pong.map(|sent| sent + REPLY_TIMEOUT)) => {
                return Err(Error::Network("chat pong timeout".into()));
            }
            _ = ping.tick() => {
                send(socket, "PING :tmi.twitch.tv").await?;
                awaiting_pong = Some(tokio::time::Instant::now());
            }
            message = socket.next() => {
                let Some(message) = message else { return Err(Error::Network("chat closed".into())); };
                let message = message.map_err(|e| Error::Network(format!("chat read: {e}")))?;
                let text = match message {
                    Message::Text(text) => text,
                    Message::Ping(data) => {
                        let _ = socket.send(Message::Pong(data)).await;
                        continue;
                    }
                    Message::Close { .. } => return Err(Error::Network("chat close frame".into())),
                    _ => continue,
                };

                for line in text.lines() {
                    match reply(line) {
                        Reply::Ping(server) => send(socket, &format!("PONG {server}")).await?,
                        Reply::Pong => awaiting_pong = None,
                        Reply::Reconnect => return Err(Error::Network("chat asked to reconnect".into())),
                        Reply::Message { channel, chatter, text } => {
                            if let Some(mention) = mention(&client.login, channel, chatter, text) {
                                let _ = mentioned.try_send(mention);
                            }
                        }
                        Reply::Other => {}
                    }
                }
            }
        }
    }
}

async fn sleep_until_some(at: Option<tokio::time::Instant>) {
    match at {
        Some(at) => tokio::time::sleep_until(at).await,
        None => std::future::pending().await,
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Reply<'a> {
    Ping(&'a str),
    Pong,
    Reconnect,
    Message {
        channel: &'a str,
        chatter: &'a str,
        text: &'a str,
    },
    Other,
}

fn mention(login: &str, channel: &str, chatter: &str, text: &str) -> Option<ChatMention> {
    if chatter.eq_ignore_ascii_case(login) {
        return None;
    }

    let lower = text.to_ascii_lowercase();
    let wanted = format!("@{}", login.to_ascii_lowercase());
    let named = lower.match_indices(&wanted).any(|(at, _)| {
        lower[at + wanted.len()..]
            .chars()
            .next()
            .is_none_or(|next| !(next.is_ascii_alphanumeric() || next == '_'))
    });

    if !named {
        return None;
    }

    let text: String = text.chars().take(MENTION_TEXT_MAX).collect();

    Some(ChatMention {
        channel: channel.to_ascii_lowercase(),
        chatter: chatter.to_ascii_lowercase(),
        text,
    })
}

fn reply(line: &str) -> Reply<'_> {
    let line = line.trim_end_matches('\r');

    if let Some(server) = line.strip_prefix("PING ") {
        return Reply::Ping(server);
    }

    let rest = line.strip_prefix('@').map_or(line, |tagged| {
        tagged.split_once(' ').map_or("", |(_, rest)| rest)
    });
    let (prefix, rest) = match rest.strip_prefix(':') {
        Some(prefixed) => prefixed.split_once(' ').unwrap_or(("", "")),
        None => ("", rest),
    };
    let (command, params) = rest.split_once(' ').unwrap_or((rest, ""));

    match command {
        "PONG" => Reply::Pong,
        "RECONNECT" => Reply::Reconnect,
        "PRIVMSG" => {
            let chatter = prefix.split_once('!').map_or(prefix, |(nick, _)| nick);

            match params.split_once(" :") {
                Some((target, text)) if !chatter.is_empty() => Reply::Message {
                    channel: target.trim_start_matches('#'),
                    chatter,
                    text,
                },
                _ => Reply::Other,
            }
        }
        _ => Reply::Other,
    }
}
