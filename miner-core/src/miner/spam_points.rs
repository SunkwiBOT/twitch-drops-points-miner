use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use chrono::Utc;
use tokio::sync::{broadcast, mpsc, watch, Notify};
use tokio_util::sync::CancellationToken;

use super::account::AccountCommand;
use super::record_event;
use crate::api::{AccountSnapshot, RedeemRequest};
use crate::error::{Error, Result};
use crate::events::{EventContext, EventKind, MinerEvent};
use crate::spam::{
    self, Answer, BurstProgress, BurstResult, BurstSpends, PointsChannel, SpamBalance,
    SpamChannelEntry, SpamChannelView, SpamReward, SpamRow, Tally,
};
use crate::store::{AccountRecord, Store};
use crate::twitch::rewards::RewardsContext;
use crate::twitch::TwitchClient;
use crate::util::locked;

const PROGRESS_BUFFER: usize = 256;

pub(super) struct Burst {
    pub(super) account_id: String,
    pub(super) cancel: CancellationToken,
    pub(super) line: Option<BurstLine>,
}

#[derive(Clone)]
pub(super) struct BurstLine {
    pub(super) record: AccountRecord,
    pub(super) login: String,
    pub(super) reward: SpamReward,
    pub(super) count: u32,
    pub(super) tally: Tally,
}

pub(super) struct SpamAccess {
    pub(super) client: TwitchClient,
    pub(super) commands: mpsc::Sender<AccountCommand>,
    pub(super) snapshot: watch::Receiver<AccountSnapshot>,
}

pub(super) struct BurstPlan {
    pub(super) burst_id: String,
    pub(super) line: BurstLine,
    pub(super) channel_id: String,
    pub(super) text: Option<String>,
    pub(super) delay: Duration,
    pub(super) cancel: CancellationToken,
    pub(super) client: TwitchClient,
}

pub(super) struct SpamPoints {
    store: Store,
    events: broadcast::Sender<MinerEvent>,
    changed: Arc<Notify>,
    cancel: CancellationToken,
    bursts: Mutex<HashMap<String, Burst>>,
    progress: broadcast::Sender<BurstProgress>,
    // ! One per account, not per task: a task restarted during a burst must
    // still know its spends, or the Journal counts them a second time.
    spends: Mutex<HashMap<String, BurstSpends>>,
}

impl SpamPoints {
    pub(super) fn new(
        store: Store,
        events: broadcast::Sender<MinerEvent>,
        changed: Arc<Notify>,
        cancel: CancellationToken,
    ) -> Self {
        let (progress, _) = broadcast::channel(PROGRESS_BUFFER);

        Self {
            store,
            events,
            changed,
            cancel,
            bursts: Mutex::new(HashMap::new()),
            progress,
            spends: Mutex::new(HashMap::new()),
        }
    }

    pub(super) fn subscribe(&self) -> broadcast::Receiver<BurstProgress> {
        self.progress.subscribe()
    }

    pub(super) fn bursts(&self) -> MutexGuard<'_, HashMap<String, Burst>> {
        locked(&self.bursts)
    }

    pub(super) fn spends(&self, account_id: &str) -> BurstSpends {
        locked(&self.spends)
            .entry(account_id.to_owned())
            .or_default()
            .clone()
    }

    pub(super) fn cancel_bursts(&self, account_id: Option<&str>) {
        for burst in self.bursts().values() {
            if account_id.is_none_or(|id| id == burst.account_id) {
                burst.cancel.cancel();
            }
        }
    }

    pub(super) fn forget_account(&self, account_id: &str) {
        self.cancel_bursts(Some(account_id));
        locked(&self.spends).remove(account_id);
    }

    pub(super) fn cancel_burst(&self, burst_id: &str) {
        if let Some(burst) = self.bursts().get(burst_id) {
            burst.cancel.cancel();
        }
    }

    // ! Nothing waits for the answers still on their way, as for the
    // notifications: each burst writes its Journal line from what it counted.
    pub(super) async fn interrupt(&self) {
        let bursts: Vec<Burst> = self.bursts().drain().map(|(_, burst)| burst).collect();

        if bursts.is_empty() {
            return;
        }

        let interrupted = bursts.len();
        let lines: Vec<(BurstLine, BurstResult)> = bursts
            .into_iter()
            .filter_map(|burst| {
                burst.cancel.cancel();

                burst.line.map(|line| {
                    let result = line.tally.interrupted(line.count);

                    (line, result)
                })
            })
            .collect();

        tracing::warn!(
            "{interrupted} Spam Points {} interrupted at shutdown",
            if interrupted == 1 { "burst" } else { "bursts" }
        );

        let (store, events) = (self.store.clone(), self.events.clone());
        let written = tokio::task::spawn_blocking(move || {
            for (line, result) in &lines {
                journal_line(&store, &events, line, result);
            }
        })
        .await;

        if let Err(error) = written {
            tracing::warn!(%error, "Spam Points lines not written");
        }
    }

    pub(super) fn add_channel(&self, account_id: &str, login: &str) -> Result<()> {
        let login = spam_login(login)?;
        let record = self.account_record(account_id)?;

        self.store
            .add_spam_channel(&record.id, &login, Utc::now())?;
        self.changed.notify_waiters();

        Ok(())
    }

    pub(super) fn forget_channel(&self, account_id: &str, login: &str) -> Result<()> {
        let login = spam_login(login)?;
        let record = self.account_record(account_id)?;

        self.store.forget_spam_channel(&record.id, &login)?;
        self.changed.notify_waiters();

        Ok(())
    }

    fn account_record(&self, account_id: &str) -> Result<AccountRecord> {
        self.store
            .account(account_id)?
            .ok_or_else(|| Error::UnknownAccount(account_id.to_owned()))
    }

    pub(super) async fn channel(
        &self,
        account_id: &str,
        login: &str,
        access: impl Future<Output = Result<SpamAccess>>,
    ) -> Result<SpamChannelView> {
        let login = spam_login(login)?;
        let (access, read) = read_rewards(&login, access).await?;
        let balance = self
            .note_balance(account_id, &login, read.balance, &access)
            .await?;

        Ok(SpamChannelView {
            login,
            balance,
            rewards: read.rewards,
        })
    }

    pub(super) async fn balance(
        &self,
        account_id: &str,
        login: &str,
        access: impl Future<Output = Result<SpamAccess>>,
    ) -> Result<SpamBalance> {
        let login = spam_login(login)?;
        let (access, read) = read_rewards(&login, access).await?;

        self.note_balance(account_id, &login, read.balance, &access)
            .await
    }

    async fn note_balance(
        &self,
        account_id: &str,
        login: &str,
        balance: i64,
        access: &SpamAccess,
    ) -> Result<SpamBalance> {
        let (in_points, behind) = {
            let snapshot = access.snapshot.borrow();
            let in_points = snapshot
                .settings
                .points
                .streamers
                .iter()
                .any(|streamer| streamer.login == login);
            let behind = snapshot
                .streamers
                .iter()
                .any(|streamer| streamer.login == login && streamer.points.balance != balance);

            (in_points, behind)
        };
        let store = self.store.clone();
        let (account_id, kept_login) = (account_id.to_owned(), login.to_owned());
        let (kept, changed) = tokio::task::spawn_blocking(move || {
            store.record_spam_balance(&account_id, &kept_login, balance, Utc::now(), in_points)
        })
        .await
        .map_err(|error| Error::Storage(error.to_string()))??;

        if changed {
            self.changed.notify_waiters();
        }

        if behind {
            let _ = access.commands.try_send(AccountCommand::Balance {
                login: login.to_owned(),
                balance,
            });
        }

        Ok(kept)
    }

    pub(super) async fn redeem(
        self: &Arc<Self>,
        request: RedeemRequest,
        delay: Duration,
        access: impl Future<Output = Result<SpamAccess>>,
    ) -> Result<String> {
        let login = spam_login(&request.login)?;

        request.validate()?;

        let text = request.message().map(str::to_owned);
        let RedeemRequest {
            account_id,
            burst_id,
            reward_id,
            count,
            ..
        } = request;
        let record = self.account_record(&account_id)?;
        let cancel = self.cancel.child_token();

        // ! Known before the channel is read: a page left during that read
        // cancels the burst too.
        {
            let mut bursts = self.bursts();

            if self.cancel.is_cancelled() {
                return Err(Error::InvalidRequest("the app is stopping".to_owned()));
            }

            if bursts.contains_key(&burst_id) {
                return Err(Error::InvalidRequest("burst id already running".to_owned()));
            }

            bursts.insert(
                burst_id.clone(),
                Burst {
                    account_id: record.id.clone(),
                    cancel: cancel.clone(),
                    line: None,
                },
            );
        }

        let prepared = async {
            let (access, read) = read_rewards(&login, access).await?;
            let reward = read
                .rewards
                .into_iter()
                .find(|reward| reward.id == reward_id)
                .ok_or_else(|| Error::UnknownReward(reward_id.clone()))?;

            spam::check_redeemable(&reward, count, text.is_some(), read.balance, Utc::now())?;

            Ok::<_, Error>((access.client, read.channel_id, reward))
        }
        .await;
        let (client, channel_id, reward) = match prepared {
            Ok(prepared) => prepared,
            Err(error) => {
                self.bursts().remove(&burst_id);

                return Err(error);
            }
        };
        let line = BurstLine {
            record,
            login,
            reward,
            count,
            tally: Tally::default(),
        };

        if let Some(burst) = self.bursts().get_mut(&burst_id) {
            burst.line = Some(line.clone());
        }

        let plan = BurstPlan {
            burst_id: burst_id.clone(),
            line,
            channel_id,
            text,
            delay,
            cancel,
            client,
        };
        let this = self.clone();

        tokio::spawn(async move { this.run_burst(plan).await });

        Ok(burst_id)
    }

    pub(super) async fn run_burst(&self, plan: BurstPlan) {
        let BurstPlan {
            burst_id,
            line,
            channel_id,
            text,
            delay,
            cancel,
            client,
        } = plan;
        let spends = self.spends(&line.record.id);
        let client = &client;
        let (channel_id, reward, text) = (&channel_id, &line.reward, text.as_deref());
        let (progress, count) = (&self.progress, line.count);

        spends.start(&line.login);

        let result = spam::run_burst(
            count,
            delay,
            &cancel,
            &line.tally,
            || async move { Answer::of(client.redeem_reward(channel_id, reward, text).await) },
            |done| {
                let _ = progress.send(BurstProgress {
                    burst_id: burst_id.clone(),
                    done,
                    total: count,
                    summary: None,
                });
            },
        )
        .await;

        spends.end(&line.login);

        if result.failed > 0 {
            tracing::warn!(
                account = %line.record.login,
                codes = ?result.failures,
                "Spam Points burst on {}: {} of {count} redemptions failed",
                line.login,
                result.failed
            );
        }

        // ! The shutdown takes the bursts it interrupts out of the list and
        // writes their lines itself.
        if self.bursts().remove(&burst_id).is_none() {
            return;
        }

        let done = result.answered();

        self.write_line(line, result.clone()).await;

        let _ = self.progress.send(BurstProgress {
            burst_id,
            done,
            total: count,
            summary: Some(result),
        });
    }

    pub(super) async fn write_line(&self, line: BurstLine, result: BurstResult) {
        let (store, events) = (self.store.clone(), self.events.clone());
        let written =
            tokio::task::spawn_blocking(move || journal_line(&store, &events, &line, &result))
                .await;

        if let Err(error) = written {
            tracing::warn!(%error, "Spam Points line not written");
        }
    }
}

async fn read_rewards(
    login: &str,
    access: impl Future<Output = Result<SpamAccess>>,
) -> Result<(SpamAccess, RewardsContext)> {
    let access = access.await?;
    let read = access
        .client
        .custom_rewards(login)
        .await?
        .ok_or_else(|| Error::UnknownChannel(login.to_owned()))?;

    Ok((access, read))
}

pub(super) fn spam_channels(account: &AccountSnapshot, rows: &[SpamRow]) -> Vec<SpamChannelEntry> {
    let points: Vec<PointsChannel> = account
        .settings
        .points
        .streamers
        .iter()
        .map(|streamer| PointsChannel {
            login: streamer.login.clone(),
            balance: account
                .streamers
                .iter()
                .find(|view| view.login == streamer.login && view.points.enabled)
                .map(|view| view.points.balance),
        })
        .collect();

    spam::channel_list(&points, rows)
}

fn spam_login(login: &str) -> Result<String> {
    crate::util::normalize_login(login).ok_or_else(|| Error::UnknownChannel(login.to_owned()))
}

// ! A burst cancelled before its first request leaves no line: Twitch was
// asked nothing.
fn journal_line(
    store: &Store,
    events: &broadcast::Sender<MinerEvent>,
    line: &BurstLine,
    result: &BurstResult,
) {
    if result.cancelled >= line.count || !matches!(store.account(&line.record.id), Ok(Some(_))) {
        return;
    }

    if let Err(error) = store.mark_spam_used(&line.record.id, &line.login, Utc::now()) {
        tracing::warn!(%error, "Spam Points use not kept");
    }

    record_event(
        store,
        events,
        burst_event(&line.record, &line.login, &line.reward, line.count, result),
    );
}

pub(super) fn burst_event(
    record: &AccountRecord,
    login: &str,
    reward: &SpamReward,
    count: u32,
    result: &BurstResult,
) -> MinerEvent {
    let title = format!(
        "{count} × {} on {login}: {} sent, {} refused, {} failed, {} cancelled",
        reward.title, result.sent, result.refused, result.failed, result.cancelled
    );

    MinerEvent::new(&record.id, &record.login, EventKind::RewardRedeemed, title).context(
        EventContext {
            account_name: Some(record.display_name.clone()),
            account_avatar: record.avatar_url.clone(),
            reward: Some(reward.image_url.clone()),
            reward_title: Some(reward.title.clone()),
            reward_color: reward.background_color.clone(),
            count: Some(count),
            sent: Some(result.sent),
            cancelled: Some(result.cancelled).filter(|cancelled| *cancelled > 0),
            spent: Some(i64::from(result.sent) * reward.cost),
            refusals: Some(result.refusals.clone()).filter(|codes| !codes.is_empty()),
            failures: Some(result.failures.clone()).filter(|codes| !codes.is_empty()),
            ..EventContext::channel(login)
        },
    )
}
