use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use chrono::{DateTime, Utc};
use futures_util::stream::{FuturesUnordered, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
use ts_rs::TS;

use crate::error::{Error, ErrorCode, Result};
use crate::twitch::POINTS_READS_AT_ONCE;

pub const BURST_MAX: u32 = 25;
pub const BURST_ID_MAX_CHARS: usize = 64;
pub const REWARD_ID_MAX_CHARS: usize = 64;
pub const REDEEM_TEXT_MAX_CHARS: usize = 200;
// ! Twitch tells the spends of a burst over PubSub a moment after the answers.
const QUIET_AFTER_BURST: Duration = Duration::from_secs(30);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "SpamRewardType")]
pub struct SpamReward {
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub prompt: Option<String>,
    pub cost: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub background_color: Option<String>,
    pub image_url: String,
    pub is_enabled: bool,
    pub is_paused: bool,
    pub is_in_stock: bool,
    pub is_user_input_required: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub cooldown_expires_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub cooldown_seconds: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "SpamBalanceType")]
pub struct SpamBalance {
    pub balance: i64,
    pub changed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "SpamChannelViewType")]
pub struct SpamChannelView {
    pub login: String,
    pub balance: SpamBalance,
    pub rewards: Vec<SpamReward>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "SpamChannelEntryType")]
pub struct SpamChannelEntry {
    pub login: String,
    pub in_points: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub balance: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "BurstResultType")]
pub struct BurstResult {
    pub sent: u32,
    pub refused: u32,
    pub failed: u32,
    pub cancelled: u32,
    pub refusals: Vec<String>,
    pub failures: Vec<ErrorCode>,
}

impl BurstResult {
    pub fn answered(&self) -> u32 {
        self.sent + self.refused + self.failed
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "BurstProgressType")]
pub struct BurstProgress {
    pub burst_id: String,
    pub done: u32,
    pub total: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub summary: Option<BurstResult>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SpamRow {
    pub login: String,
    pub added_at: Option<DateTime<Utc>>,
    pub used_at: Option<DateTime<Utc>>,
    pub balance: Option<i64>,
    pub balance_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointsChannel {
    pub login: String,
    pub balance: Option<i64>,
}

// ! Opening a channel moves nothing: the row under the pointer stays put.
pub fn channel_list(points: &[PointsChannel], rows: &[SpamRow]) -> Vec<SpamChannelEntry> {
    struct Listed {
        entry: SpamChannelEntry,
        used_at: Option<DateTime<Utc>>,
        added_at: Option<DateTime<Utc>>,
        index: usize,
    }

    let row = |login: &str| rows.iter().find(|row| row.login == login);
    let mut listed: Vec<Listed> = points
        .iter()
        .enumerate()
        .map(|(index, channel)| {
            let kept = row(&channel.login);

            Listed {
                entry: SpamChannelEntry {
                    login: channel.login.clone(),
                    in_points: true,
                    balance: channel.balance.or(kept.and_then(|row| row.balance)),
                },
                used_at: kept.and_then(|row| row.used_at),
                added_at: None,
                index,
            }
        })
        .collect();

    for kept in rows {
        let listed_already = points.iter().any(|channel| channel.login == kept.login);

        if listed_already || (kept.added_at.is_none() && kept.used_at.is_none()) {
            continue;
        }

        listed.push(Listed {
            entry: SpamChannelEntry {
                login: kept.login.clone(),
                in_points: false,
                balance: kept.balance,
            },
            used_at: kept.used_at,
            added_at: kept.added_at,
            index: usize::MAX,
        });
    }

    let group = |item: &Listed| match (item.used_at, item.entry.in_points) {
        (Some(_), _) => 0,
        (None, false) => 1,
        (None, true) => 2,
    };

    listed.sort_by(|a, b| {
        group(a)
            .cmp(&group(b))
            .then_with(|| b.used_at.cmp(&a.used_at))
            .then_with(|| b.added_at.cmp(&a.added_at))
            .then_with(|| a.index.cmp(&b.index))
    });

    listed.into_iter().map(|item| item.entry).collect()
}

pub enum Answer {
    Taken,
    Refused(String),
    Failed(ErrorCode),
}

impl Answer {
    pub fn of(sent: Result<Option<String>>) -> Self {
        match sent {
            Ok(None) => Self::Taken,
            Ok(Some(code)) => Self::Refused(code),
            Err(error) => {
                tracing::debug!(%error, "a Spam Points redemption failed");

                Self::Failed(error.code())
            }
        }
    }
}

pub fn check_redeemable(
    reward: &SpamReward,
    count: u32,
    has_text: bool,
    balance: i64,
    now: DateTime<Utc>,
) -> Result<()> {
    let refusal = if reward.is_user_input_required && !has_text {
        Some("a message is required")
    } else if !reward.is_enabled {
        Some("disabled")
    } else if reward.is_paused {
        Some("paused")
    } else if !reward.is_in_stock {
        Some("out of stock")
    } else if reward.cooldown_expires_at.is_some_and(|at| at > now) {
        Some("on cooldown")
    } else if i64::from(count).saturating_mul(reward.cost) > balance {
        Some("not enough points")
    } else {
        None
    };

    match refusal {
        Some(why) => Err(Error::RewardNotRedeemable(why.to_owned())),
        None => Ok(()),
    }
}

fn add_once<T: PartialEq>(list: &mut Vec<T>, item: T) {
    if !list.contains(&item) {
        list.push(item);
    }
}

#[derive(Default)]
struct Counted {
    started: u32,
    left: u32,
    result: BurstResult,
}

#[derive(Clone, Default)]
pub struct Tally(Arc<Mutex<Counted>>);

impl Tally {
    fn lock(&self) -> MutexGuard<'_, Counted> {
        crate::util::locked(&self.0)
    }

    pub fn start(&self) {
        self.lock().started += 1;
    }

    pub fn left(&self) {
        self.lock().left += 1;
    }

    pub fn answer(&self, answer: Answer) -> u32 {
        let mut counted = self.lock();
        let result = &mut counted.result;

        match answer {
            Answer::Taken => result.sent += 1,
            Answer::Refused(code) => {
                result.refused += 1;
                add_once(&mut result.refusals, code);
            }
            Answer::Failed(code) => {
                result.failed += 1;
                add_once(&mut result.failures, code);
            }
        }

        result.answered()
    }

    fn finished(&self, count: u32) -> BurstResult {
        let mut counted = self.lock();

        counted.result.cancelled = count.saturating_sub(counted.started);

        counted.result.clone()
    }

    // ! The requests on their way when the app stops count as sent: they left,
    // and the Journal must not hide points Twitch may still take. One still
    // waiting for a slot never left: cancelled.
    pub fn interrupted(&self, count: u32) -> BurstResult {
        let counted = self.lock();
        let answered = counted.result.answered();

        BurstResult {
            sent: counted.result.sent + counted.left.saturating_sub(answered),
            cancelled: count.saturating_sub(counted.left),
            ..counted.result.clone()
        }
    }
}

// ! A request leaves `delay` after the previous one without waiting for its
// answer, at most POINTS_READS_AT_ONCE on their way: a cancel stops only the
// requests not sent yet.
pub async fn run_burst<F, Fut>(
    count: u32,
    delay: Duration,
    cancel: &CancellationToken,
    tally: &Tally,
    send: F,
    mut progress: impl FnMut(u32),
) -> BurstResult
where
    F: Fn() -> Fut,
    Fut: Future<Output = Answer>,
{
    let mut on_their_way = FuturesUnordered::new();
    let mut started = 0;
    let mut next_at = Instant::now();

    loop {
        let sending = started < count && !cancel.is_cancelled();
        let room = on_their_way.len() < POINTS_READS_AT_ONCE;

        if sending && room && Instant::now() >= next_at {
            on_their_way.push(send());
            started += 1;
            tally.start();
            next_at = Instant::now() + delay;

            continue;
        }

        if !sending && on_their_way.is_empty() {
            break;
        }

        tokio::select! {
            biased;

            Some(answer) = on_their_way.next(), if !on_their_way.is_empty() => {
                progress(tally.answer(answer));
            }
            _ = cancel.cancelled(), if sending => {}
            _ = tokio::time::sleep_until(next_at), if sending && room => {}
        }
    }

    tally.finished(count)
}

pub fn valid_burst_id(id: &str) -> bool {
    !id.is_empty()
        && id.chars().count() <= BURST_ID_MAX_CHARS
        && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
}

#[derive(Default)]
struct Spending {
    running: u32,
    quiet_until: Option<Instant>,
}

// ! The PubSub spends of a running Spam Points burst are its own: the Journal
// keeps one line for the burst instead of one per redemption.
#[derive(Clone, Default)]
pub struct BurstSpends(Arc<Mutex<HashMap<String, Spending>>>);

impl BurstSpends {
    fn lock(&self) -> MutexGuard<'_, HashMap<String, Spending>> {
        crate::util::locked(&self.0)
    }

    pub fn start(&self, login: &str) {
        self.lock().entry(login.to_owned()).or_default().running += 1;
    }

    pub fn end(&self, login: &str) {
        let mut spends = self.lock();
        let now = Instant::now();

        spends.retain(|_, spending| {
            spending.running > 0 || spending.quiet_until.is_some_and(|until| now < until)
        });

        let spending = spends.entry(login.to_owned()).or_default();

        spending.running = spending.running.saturating_sub(1);
        spending.quiet_until = Some(now + QUIET_AFTER_BURST);
    }

    pub fn covers(&self, login: &str) -> bool {
        self.lock().get(login).is_some_and(|spending| {
            spending.running > 0
                || spending
                    .quiet_until
                    .is_some_and(|until| Instant::now() < until)
        })
    }
}
