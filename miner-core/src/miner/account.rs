use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use reqwest::StatusCode;
use tokio::sync::{broadcast, mpsc, oneshot, watch, RwLock};
use tokio_util::sync::CancellationToken;
use url::Url;

use super::twitch_api::TwitchApi;
use crate::api::{
    AccountSnapshot, AccountStats, AccountStatus, DropProgressView, SlotView, StreamerView,
    FOLLOWS_MAX,
};
use crate::catalog::Catalog;
use crate::config::{AccountSettings, ChatPresence, Mode};
use crate::domain::{Campaign, Channel, Game, PointsState, PredictionEvent};
use crate::error::{Error, ErrorCode, NetworkErrorKind, Result};
use crate::events::{BetResult, EventContext, EventKind, MinerEvent, MinerKind};
use crate::predictions;
use crate::scheduler::{self, QueueItem, SlotPurpose, StreakBonus, StreamerState};
use crate::store::vault::Vault;
use crate::store::{AccountRecord, Credential, Store};
use crate::twitch::chat::{self, Chat, ChatMention};
use crate::twitch::drops::ClaimOutcome;
use crate::twitch::playback::{self, Playback};
use crate::twitch::points::Goal;
use crate::twitch::pubsub::{self, PubSub, PubSubEvent, RaidJoins};
use crate::twitch::{TwitchClient, TwitchHttp};

const TICK: Duration = Duration::from_secs(20);
const INVENTORY_INTERVAL: Duration = Duration::from_secs(5 * 60);
const CHANNELS_INTERVAL: Duration = Duration::from_secs(90);
const BEACON_INTERVAL: Duration = Duration::from_secs(60);
const BEACON_UNREACHABLE_AFTER: u32 = 3;
const VALIDATE_INTERVAL: Duration = Duration::from_secs(55 * 60);
const REFUSAL_RECHECK_GAP: Duration = Duration::from_secs(60);
const FOLLOWS_INTERVAL: Duration = Duration::from_secs(30 * 60);
const POINTS_INTERVAL: Duration = Duration::from_secs(30 * 60);
const GOALS_INTERVAL: Duration = Duration::from_secs(10 * 60);
const LOOKUP_TOP: usize = 8;
const LOOKUP_ROTATING: usize = 4;
const DIRECTORIES_MAX: usize = 10;
const ALLOWED_CHANNELS_MAX: usize = 60;
// ! Below the HTTP semaphore, so the slots and claims still get a request in
// while balances or directories are read.
const POINTS_READS_AT_ONCE: usize = 4;
const DIRECTORY_READS_AT_ONCE: usize = 4;
const MENTION_COOLDOWN: Duration = Duration::from_secs(60);
const OWN_SPEND_WAIT: Duration = Duration::from_secs(2 * 60);
// ! Twitch's answer for a bonus chest another session took first, or that
// expired: nothing went wrong here.
const BONUS_GONE: [&str; 1] = ["NOT_FOUND"];
// ! Parts of Twitch's refusal codes for a bet on a prediction that closed
// before it (a streamer can lock one early): no bet, not an error.
const PREDICTION_CLOSED: [&str; 6] = [
    "LOCKED",
    "NOT_ACTIVE",
    "CLOSED",
    "ENDED",
    "RESOLVED",
    "CANCEL",
];

#[derive(Debug)]
pub enum AccountCommand {
    Settings(Box<AccountSettings>),
    Refresh,
    ClaimDrop(String, oneshot::Sender<Result<()>>),
}

enum Wake {
    Bets,
    Command(AccountCommand),
    PubSub(PubSubEvent),
    Mention(ChatMention),
    Tick,
}

pub struct AccountHandle {
    pub id: String,
    pub commands: mpsc::Sender<AccountCommand>,
    pub snapshot: watch::Receiver<AccountSnapshot>,
    pub cancel: CancellationToken,
    pub task: tokio::task::JoinHandle<()>,
}

struct ActiveSlot {
    channel: Channel,
    purpose: SlotPurpose,
    since: DateTime<Utc>,
    beacon: Option<Url>,
    last_beacon_at: Option<DateTime<Utc>>,
    next_beacon: tokio::time::Instant,
    playback: Option<Playback>,
    next_playback: tokio::time::Instant,
    beacon_failures: u32,
}

struct OwnSpend {
    amount: i64,
    at: tokio::time::Instant,
}

#[derive(Default)]
struct OwnSpends(HashMap<String, Vec<OwnSpend>>);

impl OwnSpends {
    fn add(&mut self, login: &str, amount: i64) {
        self.0.entry(login.to_owned()).or_default().push(OwnSpend {
            amount,
            at: tokio::time::Instant::now(),
        });
    }

    fn pending(&self, login: &str) -> i64 {
        self.0.get(login).map_or(0, |spends| {
            spends
                .iter()
                .filter(|spend| spend.at.elapsed() < OWN_SPEND_WAIT)
                .map(|spend| spend.amount)
                .sum()
        })
    }

    fn settle(&mut self, login: &str, amount: Option<i64>) -> Option<i64> {
        for spends in self.0.values_mut() {
            spends.retain(|spend| spend.at.elapsed() < OWN_SPEND_WAIT);
        }

        self.0.retain(|_, spends| !spends.is_empty());

        let Some(spends) = self.0.get_mut(login) else {
            return amount;
        };

        let left = match amount {
            // ! Twitch sends one message per spend: one whose amount is not
            // known is the oldest spend's.
            None => {
                spends.remove(0);

                0
            }
            Some(mut left) => {
                for spend in spends.iter_mut() {
                    let taken = spend.amount.min(left);

                    spend.amount -= taken;
                    left -= taken;
                }

                spends.retain(|spend| spend.amount > 0);

                left
            }
        };

        if spends.is_empty() {
            self.0.remove(login);
        }

        Some(left)
    }
}

struct OfferedDrops {
    broadcast_id: String,
    campaign_ids: Vec<String>,
}

pub struct AccountContext {
    pub store: Store,
    pub vault: Vault,
    pub http: Arc<TwitchHttp>,
    pub catalog: Arc<RwLock<Catalog>>,
    pub events: broadcast::Sender<MinerEvent>,
    pub stats: AccountStats,
}

struct AccountMiner<T> {
    id: String,
    login: String,
    display_name: String,
    avatar_url: Option<String>,
    settings: AccountSettings,
    context: AccountContext,
    client: T,
    credential: Credential,
    status: AccountStatus,
    problem: Option<ErrorCode>,
    reported: Option<ErrorCode>,
    stats: AccountStats,
    campaigns: HashMap<String, Campaign>,
    inventory_at: Option<tokio::time::Instant>,
    live_by_campaign: HashMap<String, Vec<Channel>>,
    channels_at: Option<tokio::time::Instant>,
    lookup_offset: usize,
    streamer_channels: HashMap<String, Channel>,
    streamer_points: HashMap<String, PointsState>,
    streak_bonuses: HashMap<String, StreakBonus>,
    own_spends: OwnSpends,
    offered_drops: HashMap<String, OfferedDrops>,
    points_at: HashMap<String, tokio::time::Instant>,
    goals_at: HashMap<String, tokio::time::Instant>,
    slots: Vec<ActiveSlot>,
    predictions_placed: HashMap<String, (String, i64)>,
    claimed_drops: HashSet<String>,
    refused_claims: HashMap<String, Option<bool>>,
    // ! PubSub and the balance read can both bring the same chest.
    bonus_claims: HashMap<String, String>,
    finished_campaigns: HashSet<String>,
    beacon_blocked: bool,
    beacon_connect_failures: u32,
    beacon_blocked_reported: bool,
    raid_joins: RaidJoins,
    pending_bets: Vec<(PredictionEvent, DateTime<Utc>)>,
    last_queue: Vec<QueueItem>,
    snapshot: watch::Sender<AccountSnapshot>,
    pubsub: Option<PubSub>,
    chat: Option<Chat>,
    mentioned_at: HashMap<(String, String), tokio::time::Instant>,
    followed: HashSet<String>,
    followed_at: Option<tokio::time::Instant>,
    pubsub_events: mpsc::Receiver<PubSubEvent>,
    pubsub_sender: mpsc::Sender<PubSubEvent>,
    cancel: CancellationToken,
    session_valid_at: Option<tokio::time::Instant>,
    login_checked_at: Option<tokio::time::Instant>,
}

pub fn spawn(
    record: AccountRecord,
    credential: Credential,
    context: AccountContext,
) -> AccountHandle {
    let (commands, receiver) = mpsc::channel(16);
    let cancel = CancellationToken::new();
    let client = TwitchClient::new(
        context.http.clone(),
        record.id.clone(),
        record.login.clone(),
        credential.tokens.access_token.clone(),
    );
    let (mut miner, snapshot) =
        AccountMiner::new(record, credential, context, client, cancel.clone());
    let id = miner.id.clone();
    let task = tokio::spawn(async move { miner.run(receiver).await });

    AccountHandle {
        id,
        commands,
        snapshot,
        cancel,
        task,
    }
}

impl<T: TwitchApi> AccountMiner<T> {
    fn new(
        record: AccountRecord,
        credential: Credential,
        context: AccountContext,
        client: T,
        cancel: CancellationToken,
    ) -> (Self, watch::Receiver<AccountSnapshot>) {
        let initial = AccountSnapshot {
            id: record.id.clone(),
            login: record.login.clone(),
            display_name: record.display_name.clone(),
            avatar_url: record.avatar_url.clone(),
            settings: record.settings.clone(),
            status: AccountStatus::Starting,
            problem: None,
            slots: vec![],
            queue: vec![],
            streamers: vec![],
            campaigns: vec![],
            stats: context.stats.clone(),
            updated_at: Utc::now(),
        };
        let (snapshot, receiver) = watch::channel(initial);
        let (pubsub_sender, pubsub_events) = mpsc::channel(256);
        let stats = context.stats.clone();
        let miner = Self {
            id: record.id,
            login: record.login,
            display_name: record.display_name,
            avatar_url: record.avatar_url,
            settings: record.settings,
            context,
            client,
            credential,
            status: AccountStatus::Starting,
            problem: None,
            reported: None,
            stats,
            campaigns: HashMap::new(),
            inventory_at: None,
            live_by_campaign: HashMap::new(),
            channels_at: None,
            lookup_offset: 0,
            streamer_channels: HashMap::new(),
            streamer_points: HashMap::new(),
            streak_bonuses: HashMap::new(),
            own_spends: OwnSpends::default(),
            offered_drops: HashMap::new(),
            points_at: HashMap::new(),
            goals_at: HashMap::new(),
            slots: Vec::new(),
            predictions_placed: HashMap::new(),
            claimed_drops: HashSet::new(),
            refused_claims: HashMap::new(),
            bonus_claims: HashMap::new(),
            finished_campaigns: HashSet::new(),
            beacon_blocked: false,
            beacon_connect_failures: 0,
            beacon_blocked_reported: false,
            raid_joins: RaidJoins::default(),
            pending_bets: Vec::new(),
            last_queue: Vec::new(),
            snapshot,
            pubsub: None,
            chat: None,
            mentioned_at: HashMap::new(),
            followed: HashSet::new(),
            followed_at: None,
            pubsub_events,
            pubsub_sender,
            cancel,
            session_valid_at: None,
            login_checked_at: None,
        };

        (miner, receiver)
    }

    fn emit(&mut self, kind: EventKind, title: impl Into<String>) -> MinerEvent {
        MinerEvent::new(&self.id, &self.login, kind, title)
    }

    fn illustrate(&self, event: &mut MinerEvent) {
        let context = &mut event.context;

        context.account_name = Some(self.display_name.clone());
        context.account_avatar = self.avatar_url.clone();

        if let Some(login) = context.channel.clone() {
            let channel = self
                .streamer_channels
                .get(&login)
                .or_else(|| {
                    self.slots
                        .iter()
                        .map(|s| &s.channel)
                        .find(|c| c.login == login)
                })
                .or_else(|| {
                    self.live_by_campaign
                        .values()
                        .flatten()
                        .find(|c| c.login == login)
                });

            if let Some(avatar) = channel.and_then(|c| c.avatar_url.clone()) {
                context.avatar = Some(avatar);
            }

            context.channel_name = Some(channel.map(|c| c.display_name.clone()).unwrap_or(login));
        }

        if let Some(campaign) = context
            .campaign_id
            .as_deref()
            .and_then(|id| self.campaigns.get(id))
        {
            if let Some(art) = &campaign.game.box_art_url {
                context.art = Some(art.clone());
            }

            context.game = Some(campaign.game.name.clone());
            context.game_slug = Some(campaign.game.slug()).filter(|slug| !slug.is_empty());
            context.campaign_name = Some(campaign.name.clone());

            let drop = context
                .drop_id
                .as_deref()
                .and_then(|id| campaign.drops.iter().find(|d| d.id == id));

            if let Some(drop) = drop {
                context.drop_name = Some(drop.name.clone());

                if let Some(reward) = drop.benefits.first().and_then(|b| b.image_url.clone()) {
                    context.reward = Some(reward);
                }
            }
        }
    }

    fn publish(&mut self, mut event: MinerEvent) {
        if event.context.channel.is_none() {
            if let Some(campaign_id) = event.context.campaign_id.as_deref() {
                event.context.channel = self
                    .slots
                    .iter()
                    .find(|slot| {
                        matches!(&slot.purpose, SlotPurpose::Drop { campaign_id: id, .. } if id == campaign_id)
                    })
                    .map(|slot| slot.channel.login.clone());
            }
        }

        self.illustrate(&mut event);

        if event.context.miner.is_none() {
            event.context.miner = event
                .context
                .channel
                .as_deref()
                .and_then(|login| self.slots.iter().find(|s| s.channel.login == login))
                .map(|slot| miner_of(&slot.purpose))
                .or_else(|| event.kind.miner());
        }

        event.categorize();

        match self.context.store.push_history(&event) {
            Ok(id) => event.id = id,
            Err(error) => tracing::warn!(%error, "history write failed"),
        }

        match event.kind {
            EventKind::Error => tracing::warn!(account = %self.login, "{}", event.title),
            _ => tracing::info!(account = %self.login, "{}", event.title),
        }

        let _ = self.context.events.send(event);
    }

    fn on_mention(&mut self, mention: ChatMention) {
        let now = tokio::time::Instant::now();

        self.mentioned_at
            .retain(|_, at| now.duration_since(*at) < MENTION_COOLDOWN);

        let key = (mention.channel.clone(), mention.chatter.clone());

        if self.mentioned_at.contains_key(&key) {
            return;
        }

        self.mentioned_at.insert(key, now);

        let known = self
            .streamer_channels
            .get(&mention.channel)
            .or_else(|| {
                self.slots
                    .iter()
                    .map(|slot| &slot.channel)
                    .find(|channel| channel.login == mention.channel)
            })
            .map(|channel| (channel.display_name.clone(), channel.avatar_url.clone()));

        let (channel_name, avatar) = match known {
            Some((name, avatar)) => (Some(name), avatar),
            None => (None, None),
        };

        let event = self
            .emit(
                EventKind::ChatMention,
                format!(
                    "{} mentioned the account in {}'s chat",
                    mention.chatter, mention.channel
                ),
            )
            .detail(mention.text)
            .context(EventContext {
                chatter: Some(mention.chatter),
                miner: Some(MinerKind::Points),
                channel_name,
                avatar,
                ..EventContext::channel(mention.channel)
            });

        self.publish(event);
    }

    fn set_status(&mut self, status: AccountStatus, problem: Option<ErrorCode>) {
        self.status = status;
        self.problem = problem;
    }

    async fn run(&mut self, mut commands: mpsc::Receiver<AccountCommand>) {
        if !self.settings.enabled {
            self.set_status(AccountStatus::Paused, None);
        }

        self.push_snapshot();

        let mut tick = tokio::time::interval(TICK);
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        let cancel = self.cancel.clone();

        loop {
            let next_bet = self.next_bet_at();

            let wake = tokio::select! { biased;
                _ = cancel.cancelled() => break,
                _ = sleep_until_some(next_bet) => Wake::Bets,
                command = commands.recv() => match command {
                    Some(command) => Wake::Command(command),
                    None => break,
                },
                Some(event) = self.pubsub_events.recv() => Wake::PubSub(event),
                mention = next_mention(&mut self.chat) => Wake::Mention(mention),
                _ = tick.tick() => Wake::Tick,
            };

            tokio::select! { biased;
                _ = cancel.cancelled() => break,
                _ = self.wake(wake) => {}
            }
        }

        self.stop_all_slots("stopped").await;
        self.close_sockets().await;
    }

    async fn wake(&mut self, wake: Wake) {
        match wake {
            Wake::Bets => {
                self.place_due_bets().await;
                self.push_snapshot();
            }
            Wake::Command(AccountCommand::Settings(settings)) => {
                self.apply_settings(*settings).await
            }
            Wake::Command(AccountCommand::Refresh) => {
                self.inventory_at = None;
                self.channels_at = None;
                self.tick().await;
            }
            Wake::Command(AccountCommand::ClaimDrop(instance, reply)) => {
                let _ = reply.send(self.claim_instance(&instance).await);
            }
            Wake::PubSub(event) => {
                let mut shown = shows(&event);

                self.handle_pubsub(event).await;

                for _ in 0..self.pubsub_events.len() {
                    let Ok(event) = self.pubsub_events.try_recv() else {
                        break;
                    };

                    shown |= shows(&event);
                    self.handle_pubsub(event).await;
                }

                if shown {
                    self.push_snapshot();
                }
            }
            Wake::Mention(mention) => self.on_mention(mention),
            Wake::Tick => self.tick().await,
        }
    }

    async fn serve_waiting(&mut self) {
        self.place_due_bets().await;

        // ! Only what is queued now: a busy socket does not hold the pass.
        for _ in 0..self.pubsub_events.len() {
            let Ok(event) = self.pubsub_events.try_recv() else {
                break;
            };

            self.handle_pubsub(event).await;
        }
    }

    async fn apply_settings(&mut self, settings: AccountSettings) {
        let was_enabled = self.settings.enabled;
        let channels_changed = watched_logins(&self.settings) != watched_logins(&settings)
            || self.settings.mode != settings.mode
            || self.settings.slots != settings.slots;

        self.settings = settings;

        if self.settings.enabled != was_enabled {
            if self.settings.enabled {
                self.set_status(AccountStatus::Starting, None);
            } else {
                self.stop_all_slots("paused").await;
                self.set_status(AccountStatus::Paused, None);
            }
        }

        if channels_changed {
            self.channels_at = None;
        }

        self.tick().await;
    }

    async fn tick(&mut self) {
        if self.status != AccountStatus::LoginRequired {
            self.problem = None;
        }

        if !self.settings.enabled {
            self.push_snapshot();

            return;
        }

        if self.status == AccountStatus::LoginRequired && !self.session_back().await {
            self.push_snapshot();

            return;
        }

        if let Err(error) = self.ensure_session().await {
            match error {
                Error::Unauthorized => self.require_login().await,
                other => self.report_problem(other),
            }

            self.push_snapshot();

            return;
        }

        let now = tokio::time::Instant::now();

        if self
            .inventory_at
            .is_none_or(|at| now.duration_since(at) >= INVENTORY_INTERVAL)
        {
            if let Err(error) = self.refresh_inventory().await {
                self.handle_error(error).await;
            } else {
                self.inventory_at = Some(now);
            }

            self.serve_waiting().await;
        }

        if self
            .channels_at
            .is_none_or(|at| now.duration_since(at) >= CHANNELS_INTERVAL)
        {
            if let Err(error) = self.refresh_channels().await {
                self.handle_error(error).await;
            } else {
                self.channels_at = Some(now);
            }

            self.serve_waiting().await;
        }

        // ! A sign-in asked by a step ends the pass: the steps after it would
        // watch again and reopen the sockets `require_login` closed.
        if self.status == AccountStatus::LoginRequired {
            self.push_snapshot();

            return;
        }

        self.apply_plan().await;
        self.refresh_follows(now).await;
        self.pump_slots().await;
        self.serve_waiting().await;
        self.contribute_goals(now).await;
        self.serve_waiting().await;
        self.claim_available_drops().await;

        if self.status == AccountStatus::LoginRequired {
            self.push_snapshot();

            return;
        }

        self.sync_pubsub();
        self.sync_chat();

        let status = if self.slots.is_empty() {
            AccountStatus::Idle
        } else {
            AccountStatus::Mining
        };

        if self.problem.is_none() {
            self.reported = None;
        }

        self.set_status(status, self.problem);
        self.push_snapshot();
    }

    async fn handle_error(&mut self, error: Error) {
        match error {
            Error::Unauthorized => self.session_refused().await,
            other => self.report_problem(other),
        }
    }

    fn report_problem(&mut self, error: Error) {
        if self.reported == Some(error.code()) {
            tracing::debug!(account = %self.login, "{error}");
        } else {
            tracing::warn!(account = %self.login, "{error}");
        }

        self.set_problem(error.code());
    }

    fn set_problem(&mut self, code: ErrorCode) {
        self.reported = Some(code);

        if self.problem != Some(ErrorCode::BeaconBlocked) {
            self.problem = Some(code);
        }
    }

    // ! Twitch, a proxy or Cloudflare can refuse a request while the token is
    // still good: only the validation endpoint decides that a sign-in is needed.
    async fn session_refused(&mut self) {
        if self.status == AccountStatus::LoginRequired {
            return;
        }

        let checked_lately = self
            .session_valid_at
            .is_some_and(|at| at.elapsed() < REFUSAL_RECHECK_GAP);

        let checked = if checked_lately {
            Ok(())
        } else {
            self.validate_session().await
        };

        match checked {
            Ok(()) => {
                let refused = Error::Status(StatusCode::UNAUTHORIZED);

                if self.reported == Some(refused.code()) {
                    tracing::debug!(account = %self.login, "{refused}; the session is valid");
                } else {
                    tracing::info!(account = %self.login, "{refused}; the session is valid");
                }

                self.set_problem(refused.code());
            }
            Err(Error::Unauthorized) => self.require_login().await,
            Err(other) => self.report_problem(other),
        }
    }

    async fn require_login(&mut self) {
        self.set_status(
            AccountStatus::LoginRequired,
            Some(ErrorCode::SessionExpired),
        );
        self.login_checked_at = Some(tokio::time::Instant::now());

        let event = self.emit(
            EventKind::SessionExpired,
            "Twitch session expired, sign in again",
        );

        self.publish(event);
        self.stop_all_slots("sign-in required").await;
        self.close_sockets().await;
    }

    async fn close_sockets(&mut self) {
        if let Some(mut pubsub) = self.pubsub.take() {
            pubsub.close().await;
        }

        if let Some(mut chat) = self.chat.take() {
            chat.close().await;
        }
    }

    async fn session_back(&mut self) -> bool {
        if self
            .login_checked_at
            .is_some_and(|at| at.elapsed() < VALIDATE_INTERVAL)
        {
            return false;
        }

        self.login_checked_at = Some(tokio::time::Instant::now());

        if let Err(error) = self.validate_session().await {
            tracing::debug!(account = %self.login, %error, "the session is still refused");

            return false;
        }

        self.set_status(AccountStatus::Starting, None);

        let event = self.emit(EventKind::Info, "Twitch session valid again");

        self.publish(event);

        true
    }

    async fn ensure_session(&mut self) -> Result<()> {
        if self
            .session_valid_at
            .is_some_and(|at| at.elapsed() < VALIDATE_INTERVAL)
        {
            return Ok(());
        }

        self.validate_session().await
    }

    async fn validate_session(&mut self) -> Result<()> {
        match self.client.validate().await? {
            Some(validation) if validation.user_id == self.id => {
                self.session_valid_at = Some(tokio::time::Instant::now());

                Ok(())
            }
            Some(_) => Err(Error::Unauthorized),
            None => self.refresh_session().await,
        }
    }

    async fn refresh_session(&mut self) -> Result<()> {
        let Some(refresh) = self.credential.tokens.refresh_token.clone() else {
            return Err(Error::Unauthorized);
        };
        let tokens = self.client.refresh_tokens(&refresh).await?;

        self.credential.tokens = tokens;
        self.context
            .store
            .save_credential(&self.id, &self.credential, &self.context.vault)?;
        self.session_valid_at = Some(tokio::time::Instant::now());

        let event = self.emit(EventKind::Info, "Twitch session renewed");

        self.publish(event);

        self.close_sockets().await;

        Ok(())
    }

    async fn refresh_inventory(&mut self) -> Result<()> {
        let inventory = self.client.inventory().await?;
        let catalog = self.context.catalog.read().await.campaigns.clone();
        let mut merged: HashMap<String, Campaign> = HashMap::new();

        for campaign in catalog {
            merged.insert(campaign.id.clone(), campaign);
        }

        for campaign in inventory.campaigns {
            match merged.get_mut(&campaign.id) {
                Some(existing) => {
                    existing.linked = campaign.linked.or(existing.linked);
                    existing.in_inventory = true;

                    if existing.link_url.is_none() {
                        existing.link_url = campaign.link_url.clone();
                    }

                    for drop in &campaign.drops {
                        if let Some(target) = existing.drops.iter_mut().find(|d| d.id == drop.id) {
                            target.current_minutes =
                                target.current_minutes.max(drop.current_minutes);
                            target.instance_id =
                                drop.instance_id.clone().or(target.instance_id.clone());
                            target.claimed = target.claimed || drop.claimed;
                            target.preconditions_met =
                                drop.preconditions_met.or(target.preconditions_met);
                        } else {
                            existing.drops.push(drop.clone());
                        }
                    }

                    if existing.allowed_channels.is_empty() {
                        existing.allowed_channels = campaign.allowed_channels.clone();
                    }
                }
                None => {
                    merged.insert(campaign.id.clone(), campaign);
                }
            }
        }

        for campaign in merged.values_mut() {
            for drop in &mut campaign.drops {
                if !drop.claimed
                    && drop
                        .benefits
                        .iter()
                        .any(|b| inventory.awards.contains_key(&b.id))
                    && drop.instance_id.is_none()
                {
                    drop.claimed = true;
                }
            }
        }

        for (id, previous) in &self.campaigns {
            if let Some(next) = merged.get_mut(id) {
                for drop in &mut next.drops {
                    if let Some(old) = previous.drops.iter().find(|d| d.id == drop.id) {
                        drop.current_minutes = drop.current_minutes.max(old.current_minutes);
                        drop.claimed = drop.claimed || old.claimed;
                    }
                }
            }
        }

        for campaign in merged.values_mut() {
            for drop in &mut campaign.drops {
                if self.claimed_drops.contains(&drop.id) {
                    drop.claimed = true;
                }
            }
        }

        self.campaigns = merged;

        Ok(())
    }

    async fn refresh_channels(&mut self) -> Result<()> {
        // ! The listed streamers first: the Drops lookups then see their broadcasts
        // as they are now.
        let streamers = self.refresh_streamers().await;

        if matches!(streamers, Err(Error::Unauthorized)) {
            return streamers;
        }

        if self.settings.mode == Mode::Points {
            self.live_by_campaign.clear();
            self.offered_drops.clear();
        } else {
            self.refresh_drops_channels().await?;
        }

        streamers
    }

    async fn refresh_streamers(&mut self) -> Result<()> {
        let logins: Vec<String> = self
            .settings
            .points
            .streamers
            .iter()
            .filter(|s| s.enabled)
            .map(|s| s.login.clone())
            .collect();

        if !logins.is_empty() {
            let channels = self.client.channels(&logins).await?;
            let mut seen: HashSet<String> = HashSet::new();
            let mut went_live: HashSet<String> = HashSet::new();

            for channel in channels {
                seen.insert(channel.login.clone());

                let was_live = self
                    .streamer_channels
                    .get(&channel.login)
                    .is_some_and(|c| c.live);

                if channel.live && !was_live {
                    went_live.insert(channel.login.clone());
                }

                if let Some(broadcast) = channel.broadcast_id.as_deref().filter(|_| channel.live) {
                    let known = self
                        .streak_bonuses
                        .get(&channel.login)
                        .is_some_and(|bonus| bonus.broadcast_id == broadcast);

                    if !known {
                        self.streak_bonuses
                            .insert(channel.login.clone(), StreakBonus::new(broadcast));
                    }
                }

                if channel.live != was_live && self.streamer_channels.contains_key(&channel.login) {
                    let kind = if channel.live {
                        EventKind::StreamOnline
                    } else {
                        EventKind::StreamOffline
                    };

                    let title = format!(
                        "{} is {}",
                        channel.display_name,
                        if channel.live { "live" } else { "offline" }
                    );
                    let event = self
                        .emit(kind, title)
                        .context(EventContext::channel(&channel.login));

                    self.publish(event);
                }

                self.streamer_channels
                    .insert(channel.login.clone(), channel);
            }

            let now = tokio::time::Instant::now();
            let stale: Vec<String> = logins
                .iter()
                .filter(|login| seen.contains(*login))
                .filter(|login| {
                    went_live.contains(*login)
                        || self
                            .points_at
                            .get(*login)
                            .is_none_or(|at| now.duration_since(*at) >= POINTS_INTERVAL)
                })
                .cloned()
                .collect();
            let client = &self.client;
            let reads: Vec<_> = futures_util::stream::iter(stale)
                .map(|login| async move {
                    let context = client.points_context(&login).await;
                    let streak = match &context {
                        Ok(Some(_)) => Some(client.watch_streak(&login).await),
                        _ => None,
                    };

                    (login, context, streak)
                })
                .buffer_unordered(POINTS_READS_AT_ONCE)
                .collect()
                .await;

            for (login, context, streak) in reads {
                let context = match context {
                    Ok(Some(context)) => context,
                    Ok(None) => continue,
                    Err(error) => {
                        tracing::debug!(account = %self.login, channel = %login, %error, "points balance read failed");

                        continue;
                    }
                };

                self.points_at.insert(login.clone(), now);

                let state = self.streamer_points.entry(login.clone()).or_default();

                state.balance = context.balance;
                state.enabled = context.enabled;
                state.subscribed = context.subscribed;

                match streak {
                    Some(Ok(streak)) => state.streak = streak,
                    Some(Err(error)) => {
                        tracing::debug!(account = %self.login, channel = %login, %error, "watch streak read failed")
                    }
                    None => {}
                }

                if let Some(claim) = context.available_claim_id {
                    self.claim_bonus(&context.channel_id, &claim).await;
                }
            }
        }

        self.streamer_channels
            .retain(|login, _| logins.contains(login));
        self.streak_bonuses
            .retain(|login, _| logins.contains(login));

        Ok(())
    }

    async fn refresh_drops_channels(&mut self) -> Result<()> {
        let now = Utc::now();
        let campaigns: Vec<Campaign> = self.campaigns.values().cloned().collect();
        let ranked = scheduler::rank_campaigns(&self.settings, &campaigns, now);
        let watched = self.drops_watch();

        let mut chosen: Vec<&Campaign> = ranked
            .iter()
            .map(|(c, _)| *c)
            .filter(|c| watched.as_ref().is_some_and(|(id, _)| *id == c.id))
            .collect();

        for (campaign, _) in ranked.iter().take(LOOKUP_TOP) {
            if !chosen.iter().any(|c| c.id == campaign.id) {
                chosen.push(campaign);
            }
        }

        let rest: Vec<&Campaign> = ranked
            .iter()
            .skip(LOOKUP_TOP)
            .map(|(c, _)| *c)
            .filter(|c| !chosen.iter().any(|known| known.id == c.id))
            .collect();

        if !rest.is_empty() {
            for step in 0..LOOKUP_ROTATING.min(rest.len()) {
                chosen.push(rest[(self.lookup_offset + step) % rest.len()]);
            }

            self.lookup_offset = (self.lookup_offset + LOOKUP_ROTATING) % rest.len();
        }

        let mut games: Vec<Game> = Vec::new();
        let mut allowed_logins: Vec<String> = Vec::new();

        for campaign in &chosen {
            if campaign.allowed_channels.is_empty() {
                if !games.iter().any(|game| game.id == campaign.game.id) {
                    games.push(campaign.game.clone());
                }
            } else {
                for channel in &campaign.allowed_channels {
                    if !channel.login.is_empty() && !allowed_logins.contains(&channel.login) {
                        allowed_logins.push(channel.login.clone());
                    }
                }
            }
        }

        games.truncate(DIRECTORIES_MAX);
        allowed_logins.truncate(ALLOWED_CHANNELS_MAX);

        let client = &self.client;
        let reads: Vec<(Game, Result<Vec<Channel>>)> = futures_util::stream::iter(games)
            .map(|game| async move {
                let read = client.game_directory(&game).await;

                (game, read)
            })
            .buffered(DIRECTORY_READS_AT_ONCE)
            .collect()
            .await;
        let mut directory: HashMap<String, Vec<Channel>> = HashMap::new();

        for (game, read) in reads {
            match read {
                Ok(channels) => {
                    directory.insert(game.id, channels);
                }
                Err(Error::Unauthorized) => return Err(Error::Unauthorized),
                Err(error) => {
                    tracing::debug!(account = %self.login, game = %game.name, %error, "directory lookup failed")
                }
            }
        }

        // ! A game's directory lists only 30 channels, by relevance: the channel the
        // Drops slot watches can be missing from it.
        let missing = watched.as_ref().filter(|(campaign_id, channel)| {
            chosen.iter().any(|campaign| campaign.id == *campaign_id)
                && self.campaigns.get(campaign_id).is_some_and(|campaign| {
                    campaign.allowed_channels.is_empty()
                        && directory
                            .get(&campaign.game.id)
                            .is_some_and(|listed| !listed.iter().any(|c| c.id == channel.id))
                })
        });
        let unlisted = missing
            .map(|(_, channel)| channel)
            .filter(|channel| !self.streamer_channels.contains_key(&channel.login));
        let mut logins = allowed_logins.clone();

        if let Some(channel) = unlisted {
            logins.push(channel.login.clone());
        }

        let read: Option<Vec<Channel>> = if logins.is_empty() {
            Some(Vec::new())
        } else {
            match self.client.channels(&logins).await {
                Ok(channels) => Some(channels),
                Err(Error::Unauthorized) => return Err(Error::Unauthorized),
                Err(error) => {
                    tracing::debug!(account = %self.login, %error, "channel lookup failed");

                    None
                }
            }
        };

        let missing_now: Option<(String, Option<Channel>)> =
            missing.map(|(campaign_id, channel)| {
                let now = self
                    .streamer_channels
                    .get(&channel.login)
                    .cloned()
                    .or_else(|| {
                        read.as_ref()
                            .and_then(|read| read.iter().find(|c| c.id == channel.id).cloned())
                    });

                (campaign_id.clone(), now)
            });
        let allowed: Option<Vec<Channel>> = read.map(|read| {
            read.into_iter()
                .filter(|channel| allowed_logins.contains(&channel.login))
                .collect()
        });

        let recheck = missing_now.as_ref().and_then(|(campaign_id, now)| {
            let campaign = self.campaigns.get(campaign_id)?;

            now.as_ref()
                .filter(|channel| channel.live && plays(channel, &campaign.game))
        });

        self.ask_offered_drops(&chosen, &directory, recheck.cloned())
            .await;

        let looked_up: HashSet<&str> = chosen.iter().map(|c| c.id.as_str()).collect();
        let mut live_by_campaign: HashMap<String, Vec<Channel>> = HashMap::new();

        for (campaign, _) in &ranked {
            let fresh = if !looked_up.contains(campaign.id.as_str()) {
                None
            } else if campaign.allowed_channels.is_empty() {
                directory.get(&campaign.game.id).cloned()
            } else {
                allowed.as_ref().map(|allowed| {
                    allowed
                        .iter()
                        .filter(|c| c.live && campaign.accepts_channel(&c.id))
                        .cloned()
                        .collect()
                })
            };

            let Some(mut live) = fresh else {
                if let Some(previous) = self.live_by_campaign.get(&campaign.id) {
                    live_by_campaign.insert(campaign.id.clone(), previous.clone());
                }

                continue;
            };

            if let Some((campaign_id, now)) = &missing_now {
                if *campaign_id == campaign.id {
                    if let Some(kept) = self.still_watched(campaign, watched.as_ref(), now.as_ref())
                    {
                        live.push(kept);
                    }
                }
            }

            for channel in self.streamer_channels.values() {
                if !channel.live
                    || !campaign.accepts_channel(&channel.id)
                    || !plays(channel, &campaign.game)
                {
                    continue;
                }

                let offered = self.offers(channel, &campaign.id);

                match live.iter_mut().find(|c| c.id == channel.id) {
                    Some(known) => known.drops_enabled |= offered,
                    None => live.push(Channel {
                        drops_enabled: offered,
                        ..channel.clone()
                    }),
                }
            }

            live_by_campaign.insert(campaign.id.clone(), live);
        }

        self.live_by_campaign = live_by_campaign;

        let streamers = &self.streamer_channels;
        let rechecked = recheck.cloned();

        // ! A channel out of every list may come back on the same broadcast:
        // its answer goes only once the channel is seen offline or on another.
        self.offered_drops.retain(|id, offered| {
            let seen = streamers
                .values()
                .chain(rechecked.as_ref())
                .chain(directory.values().flatten())
                .find(|channel| &channel.id == id);

            seen.is_none_or(|channel| {
                channel.live
                    && channel.broadcast_id.as_deref() == Some(offered.broadcast_id.as_str())
            })
        });

        Ok(())
    }

    fn still_watched(
        &self,
        campaign: &Campaign,
        watched: Option<&(String, Channel)>,
        now: Option<&Channel>,
    ) -> Option<Channel> {
        let (_, before) = watched?;

        let Some(now) = now else {
            return Some(before.clone());
        };

        if !now.live || !plays(now, &campaign.game) {
            return None;
        }

        let drops_enabled = match self.offered(now) {
            Some(offered) if !offered.campaign_ids.contains(&campaign.id) => return None,
            Some(_) => true,
            None => before.drops_enabled,
        };

        Some(Channel {
            drops_enabled,
            ..now.clone()
        })
    }

    fn drops_watch(&self) -> Option<(String, Channel)> {
        self.slots.iter().find_map(|slot| match &slot.purpose {
            SlotPurpose::Drop { campaign_id, .. } => {
                Some((campaign_id.clone(), slot.channel.clone()))
            }
            SlotPurpose::Points { .. } => None,
        })
    }

    async fn ask_offered_drops(
        &mut self,
        campaigns: &[&Campaign],
        directory: &HashMap<String, Vec<Channel>>,
        recheck: Option<Channel>,
    ) {
        if self.settings.mode == Mode::Points {
            return;
        }

        let mut asked: Vec<Channel> = Vec::new();

        if self.settings.drops.streamers_first {
            asked = self
                .settings
                .points
                .streamers
                .iter()
                .filter(|streamer| streamer.enabled)
                .filter_map(|streamer| self.streamer_channels.get(&streamer.login))
                .filter(|channel| {
                    campaigns.iter().any(|campaign| {
                        let listed = directory
                            .get(&campaign.game.id)
                            .is_some_and(|channels| channels.iter().any(|c| c.id == channel.id));

                        campaign.allowed_channels.is_empty()
                            && plays(channel, &campaign.game)
                            && !listed
                    })
                })
                .cloned()
                .collect();
        }

        if let Some(channel) = recheck {
            if !asked.iter().any(|c| c.id == channel.id) {
                asked.push(channel);
            }
        }

        asked.retain(|channel| {
            channel.live && channel.broadcast_id.is_some() && self.offered(channel).is_none()
        });

        if asked.is_empty() {
            return;
        }

        let ids: Vec<String> = asked.iter().map(|channel| channel.id.clone()).collect();
        let answers = match self.client.available_drops(&ids).await {
            Ok(answers) => answers,
            Err(error) => {
                tracing::debug!(account = %self.login, %error, "available drops lookup failed");

                return;
            }
        };

        for (channel_id, campaign_ids) in answers {
            let broadcast = asked
                .iter()
                .find(|channel| channel.id == channel_id)
                .and_then(|channel| channel.broadcast_id.clone());

            if let Some(broadcast_id) = broadcast {
                self.offered_drops.insert(
                    channel_id,
                    OfferedDrops {
                        broadcast_id,
                        campaign_ids,
                    },
                );
            }
        }
    }

    fn offered(&self, channel: &Channel) -> Option<&OfferedDrops> {
        self.offered_drops.get(&channel.id).filter(|offered| {
            channel.broadcast_id.as_deref() == Some(offered.broadcast_id.as_str())
        })
    }

    fn offers(&self, channel: &Channel, campaign_id: &str) -> bool {
        self.offered(channel)
            .is_some_and(|offered| offered.campaign_ids.iter().any(|id| id == campaign_id))
    }

    async fn apply_plan(&mut self) {
        let now = Utc::now();

        self.settle_streak_bonuses(now);

        let campaigns: Vec<Campaign> = self.campaigns.values().cloned().collect();
        let empty = PointsState::default();
        let states: Vec<StreamerState<'_>> = self
            .settings
            .points
            .streamers
            .iter()
            .map(|s| StreamerState {
                settings: s,
                channel: self.streamer_channels.get(&s.login),
                points: self.streamer_points.get(&s.login).unwrap_or(&empty),
                streak_bonus: self.streak_bonuses.get(&s.login),
            })
            .collect();
        let current = self.slots.iter().find_map(|s| match &s.purpose {
            SlotPurpose::Drop { campaign_id, .. } => Some(scheduler::CurrentDrop {
                campaign_id,
                channel_id: &s.channel.id,
            }),
            SlotPurpose::Points { .. } => None,
        });
        let plan = scheduler::plan(
            &self.settings,
            &campaigns,
            &self.live_by_campaign,
            &states,
            current,
            now,
        );

        self.last_queue = plan.queue.clone();

        // ! Twitch pays the streak bonus to a Drops watch too: the bonus window counts
        // from the first minute any slot watches this broadcast.
        for slot in &plan.slots {
            if let Some(bonus) = self.streak_bonuses.get_mut(&slot.channel.login) {
                bonus.watched_since.get_or_insert(now);
            }
        }

        let mut index = 0;

        while index < self.slots.len() {
            let wanted = plan.slots.iter().find(|wanted| {
                wanted.channel.id == self.slots[index].channel.id
                    && self.slots[index].purpose.keeps_stream(&wanted.purpose)
            });

            match wanted {
                Some(wanted) => {
                    let told = !self.slots[index].purpose.same_watch(&wanted.purpose);
                    let live = self.slots[index].channel.live;

                    self.slots[index].channel = wanted.channel.clone();
                    self.slots[index].channel.live &= live;
                    self.slots[index].purpose = wanted.purpose.clone();

                    if told {
                        let event = self.watch_started(&wanted.channel, &wanted.purpose);

                        self.publish(event);
                    }

                    index += 1;
                }
                None => {
                    let slot = self.slots.remove(index);
                    let event = self
                        .emit(
                            EventKind::WatchStopped,
                            format!("Stopped watching {}", slot.channel.display_name),
                        )
                        .context(EventContext {
                            avatar: slot.channel.avatar_url.clone(),
                            miner: Some(miner_of(&slot.purpose)),
                            ..EventContext::channel(&slot.channel.login)
                        });

                    self.publish(event);
                }
            }
        }

        for slot in plan.slots {
            if self.slots.iter().any(|s| s.channel.id == slot.channel.id) {
                continue;
            }

            let event = self.watch_started(&slot.channel, &slot.purpose);

            self.publish(event);

            let playback = slot
                .channel
                .broadcast_id
                .as_deref()
                .map(|b| Playback::new(&slot.channel.id, &slot.channel.login, b));

            self.slots.push(ActiveSlot {
                channel: slot.channel,
                purpose: slot.purpose,
                since: Utc::now(),
                beacon: None,
                last_beacon_at: None,
                next_beacon: tokio::time::Instant::now(),
                playback,
                next_playback: tokio::time::Instant::now(),
                beacon_failures: 0,
            });
        }
    }

    fn watch_started(&mut self, channel: &Channel, purpose: &SlotPurpose) -> MinerEvent {
        let title = match purpose {
            SlotPurpose::Drop { campaign_id, .. } => {
                let name = self
                    .campaigns
                    .get(campaign_id)
                    .map(|c| c.name.clone())
                    .unwrap_or_default();

                format!("Watching {} for {}", channel.display_name, name)
            }
            SlotPurpose::Points { reason } => {
                format!(
                    "Watching {} for channel points ({})",
                    channel.display_name,
                    reason.describe()
                )
            }
        };

        self.emit(EventKind::WatchStarted, title)
            .context(match purpose {
                SlotPurpose::Drop {
                    campaign_id,
                    drop_id,
                } => EventContext {
                    channel: Some(channel.login.clone()),
                    miner: Some(MinerKind::Drops),
                    ..EventContext::drop(campaign_id, drop_id)
                },
                SlotPurpose::Points { reason } => EventContext {
                    miner: Some(MinerKind::Points),
                    watch_reason: Some(*reason),
                    ..EventContext::channel(&channel.login)
                },
            })
    }

    async fn pump_slots(&mut self) {
        let now = tokio::time::Instant::now();
        let mut errors: Vec<Error> = Vec::new();
        let mut minutes = 0u64;

        for index in 0..self.slots.len() {
            if self.slots[index].next_beacon <= now {
                if self.slots[index].beacon.is_none() {
                    match self
                        .client
                        .resolve_beacon(&self.slots[index].channel.login)
                        .await
                    {
                        Ok(url) => self.slots[index].beacon = Some(url),
                        Err(error) => {
                            tracing::debug!(account = %self.login, %error, "beacon lookup failed");
                        }
                    }
                }

                if let Some(beacon) = self.slots[index].beacon.clone() {
                    match self
                        .client
                        .send_minute_watched(&beacon, &self.slots[index].channel)
                        .await
                    {
                        Ok(true) => {
                            self.slots[index].last_beacon_at = Some(Utc::now());
                            self.slots[index].beacon_failures = 0;
                            self.beacon_connect_failures = 0;
                            self.beacon_blocked = false;
                            minutes += 1;
                        }
                        Ok(false) => {
                            self.slots[index].beacon_failures += 1;

                            if self.slots[index].beacon_failures >= 3 {
                                self.slots[index].beacon = None;
                            }
                        }
                        Err(Error::Transport(NetworkErrorKind::Connect)) => {
                            self.slots[index].beacon_failures += 1;
                            self.beacon_connect_failures += 1;
                            self.beacon_unreachable(&beacon).await;
                        }
                        Err(error @ Error::Transport(NetworkErrorKind::Timeout)) => {
                            self.slots[index].beacon_failures += 1;

                            if self.slots[index].beacon_failures >= BEACON_UNREACHABLE_AFTER {
                                errors.push(error);
                            } else {
                                tracing::debug!(account = %self.login, %error, "beacon timed out, retried next minute");
                            }
                        }
                        Err(error) => errors.push(error),
                    }
                }

                let backoff = if self.slots[index].beacon_failures >= 3 {
                    BEACON_INTERVAL * 10
                } else {
                    BEACON_INTERVAL
                };

                self.slots[index].next_beacon = now + backoff;
            }

            if self.slots[index].next_playback <= now {
                if let Some(mut playback) = self.slots[index].playback.take() {
                    match self.client.poll_playback(&mut playback).await {
                        Ok(_) => {}
                        Err(Error::Status(status)) if status == StatusCode::NOT_FOUND => {
                            self.slots[index].channel.live = false;
                        }
                        Err(Error::Unauthorized) => errors.push(Error::Unauthorized),
                        Err(error) => {
                            tracing::debug!(account = %self.login, %error, "playback poll failed")
                        }
                    }

                    self.slots[index].playback = Some(playback);
                }

                self.slots[index].next_playback = now + playback::POLL_INTERVAL;
            }
        }

        self.stats.minutes_watched += minutes;

        for error in errors {
            self.handle_error(error).await;
        }

        let mut index = 0;

        while index < self.slots.len() {
            if !self.slots[index].channel.live {
                let slot = self.slots.remove(index);

                self.channels_at = None;

                if let Some(channel) = self.streamer_channels.get_mut(&slot.channel.login) {
                    channel.live = false;
                }

                for list in self.live_by_campaign.values_mut() {
                    list.retain(|c| c.id != slot.channel.id);
                }

                let event = self
                    .emit(
                        EventKind::WatchStopped,
                        format!("{} went offline", slot.channel.display_name),
                    )
                    .context(EventContext {
                        avatar: slot.channel.avatar_url.clone(),
                        miner: Some(miner_of(&slot.purpose)),
                        ..EventContext::channel(&slot.channel.login)
                    });

                self.publish(event);
            } else {
                index += 1;
            }
        }

        if self.slots.is_empty() {
            self.beacon_blocked = false;
            self.beacon_connect_failures = 0;
        }

        if self.beacon_blocked {
            self.problem = Some(ErrorCode::BeaconBlocked);
        }
    }

    // ! Channel points are not credited while the beacon is blocked; drops
    // still progress, through the video playlist.
    async fn beacon_unreachable(&mut self, beacon: &Url) {
        let message = match self.client.beacon_blocked_by(beacon).await {
            Some(address) => format!(
                "spade.twitch.tv is blocked by your DNS (it answers {address}): allowing spade.twitch.tv alone is not enough, also allow the names it points to (spade.sci.twitch.tv and science-edge-external-prod-*.elb.amazonaws.com), otherwise channel points are not credited"
            ),
            None if self.beacon_connect_failures >= BEACON_UNREACHABLE_AFTER => format!(
                "spade.twitch.tv could not be reached {BEACON_UNREACHABLE_AFTER} times in a row: check your firewall, proxy or DNS, otherwise channel points are not credited"
            ),
            None => {
                tracing::debug!(
                    account = %self.login,
                    failures = self.beacon_connect_failures,
                    "beacon connection failed, retried next minute"
                );

                return;
            }
        };

        self.beacon_blocked = true;

        if !self.beacon_blocked_reported {
            self.beacon_blocked_reported = true;

            let event = self
                .emit(EventKind::Error, message)
                .detail("Drops still progress through the video playlist.");

            self.publish(event);
        }
    }

    async fn stop_all_slots(&mut self, reason: &str) {
        let stopped: Vec<(String, String, MinerKind)> = self
            .slots
            .drain(..)
            .map(|s| {
                (
                    s.channel.login,
                    s.channel.display_name,
                    miner_of(&s.purpose),
                )
            })
            .collect();

        for (login, name, miner) in stopped {
            let event = self
                .emit(
                    EventKind::WatchStopped,
                    format!("Stopped watching {name} ({reason})"),
                )
                .context(EventContext {
                    miner: Some(miner),
                    ..EventContext::channel(login)
                });

            self.publish(event);
        }

        if let Some(pubsub) = &self.pubsub {
            pubsub.set_topics(Vec::new());
        }

        if let Some(chat) = &self.chat {
            chat.set_channels(BTreeSet::new());
        }
    }

    async fn claim_available_drops(&mut self) {
        if !self.settings.drops.auto_claim {
            return;
        }

        let campaigns = &self.campaigns;

        self.refused_claims.retain(|drop_id, linked| {
            campaigns
                .values()
                .find(|c| c.drops.iter().any(|d| &d.id == drop_id))
                .is_some_and(|c| c.linked == *linked)
        });

        let ignored_drops = &self.settings.drops.exclude_drop_ids;
        let claimable: Vec<(String, String, String, String, Option<String>)> = self
            .campaigns
            .values()
            .filter(|c| !scheduler::excluded(&self.settings, c))
            .flat_map(|c| {
                c.drops
                    .iter()
                    .filter(|d| d.claimable() && !ignored_drops.contains(&d.id))
                    .map(move |d| {
                        (
                            c.id.clone(),
                            c.name.clone(),
                            d.id.clone(),
                            d.name.clone(),
                            d.instance_id.clone(),
                        )
                    })
            })
            .collect();

        for (campaign_id, campaign_name, drop_id, drop_name, instance) in claimable {
            let Some(instance) = instance else { continue };

            match self.client.claim_drop(&instance).await {
                Ok(ClaimOutcome::AlreadyClaimed) => {
                    self.claimed_drops.insert(drop_id.clone());

                    if let Some(drop) = self
                        .campaigns
                        .get_mut(&campaign_id)
                        .and_then(|c| c.drops.iter_mut().find(|d| d.id == drop_id))
                    {
                        drop.claimed = true;
                    }
                }
                Ok(ClaimOutcome::Claimed) => {
                    self.stats.drops_claimed += 1;
                    self.claimed_drops.insert(drop_id.clone());

                    let image = self
                        .campaigns
                        .get(&campaign_id)
                        .and_then(|c| c.drops.iter().find(|d| d.id == drop_id))
                        .and_then(|d| d.benefits.first())
                        .and_then(|b| b.image_url.clone());

                    if let Some(drop) = self
                        .campaigns
                        .get_mut(&campaign_id)
                        .and_then(|c| c.drops.iter_mut().find(|d| d.id == drop_id))
                    {
                        drop.claimed = true;
                        drop.current_minutes = drop.required_minutes;
                    }

                    let event = self
                        .emit(
                            EventKind::DropClaimed,
                            format!("Claimed {drop_name} ({campaign_name})"),
                        )
                        .context(EventContext {
                            reward: image,
                            ..EventContext::drop(&campaign_id, &drop_id)
                        });

                    self.publish(event);

                    if self
                        .campaigns
                        .get(&campaign_id)
                        .is_some_and(Campaign::finished)
                        && self.finished_campaigns.insert(campaign_id.clone())
                    {
                        let event = self
                            .emit(
                                EventKind::CampaignFinished,
                                format!("Finished {campaign_name}"),
                            )
                            .context(EventContext::campaign(&campaign_id));

                        self.publish(event);
                        self.channels_at = None;
                    }
                }
                Ok(ClaimOutcome::Refused(status)) => {
                    let linked = self.campaigns.get(&campaign_id).and_then(|c| c.linked);

                    // ! Twitch refuses it again at every inventory read until the game account
                    // is linked: told once.
                    if self
                        .refused_claims
                        .insert(drop_id.clone(), linked)
                        .is_none()
                    {
                        let event = self
                            .emit(
                                EventKind::Error,
                                format!(
                                    "Twitch refused the claim of {drop_name} ({status}): link the game account"
                                ),
                            )
                            .context(EventContext {
                                status: Some(status.to_string()),
                                ..EventContext::campaign(&campaign_id)
                            });

                        self.publish(event);
                    } else {
                        tracing::debug!(account = %self.login, drop = %drop_name, %status, "claim still refused");
                    }

                    if let Some(drop) = self
                        .campaigns
                        .get_mut(&campaign_id)
                        .and_then(|c| c.drops.iter_mut().find(|d| d.id == drop_id))
                    {
                        drop.instance_id = None;
                    }
                }
                Err(error) => self.handle_error(error).await,
            }
        }
    }

    async fn claim_instance(&mut self, instance: &str) -> Result<()> {
        let target = self.campaigns.values().find_map(|campaign| {
            campaign
                .drops
                .iter()
                .find(|drop| drop.instance_id.as_deref() == Some(instance))
                .map(|drop| {
                    (
                        campaign.id.clone(),
                        campaign.name.clone(),
                        drop.id.clone(),
                        drop.name.clone(),
                        drop.benefits.first().and_then(|b| b.image_url.clone()),
                    )
                })
        });

        match self.client.claim_drop(instance).await {
            Ok(outcome @ (ClaimOutcome::Claimed | ClaimOutcome::AlreadyClaimed)) => {
                self.inventory_at = None;

                if let Some((campaign_id, _, drop_id, _, _)) = &target {
                    self.claimed_drops.insert(drop_id.clone());

                    if let Some(drop) = self
                        .campaigns
                        .get_mut(campaign_id)
                        .and_then(|c| c.drops.iter_mut().find(|d| &d.id == drop_id))
                    {
                        drop.claimed = true;
                        drop.instance_id = None;
                        drop.current_minutes = drop.required_minutes;
                    }
                }

                if matches!(outcome, ClaimOutcome::Claimed) {
                    self.stats.drops_claimed += 1;

                    let event = match target {
                        Some((campaign_id, campaign_name, drop_id, drop_name, image)) => self
                            .emit(
                                EventKind::DropClaimed,
                                format!("Claimed {drop_name} ({campaign_name})"),
                            )
                            .context(EventContext {
                                reward: image,
                                ..EventContext::drop(&campaign_id, &drop_id)
                            }),
                        None => self.emit(EventKind::DropClaimed, "Drop claimed"),
                    };

                    self.publish(event);
                }
            }
            Ok(ClaimOutcome::Refused(status)) => {
                let event = self.emit(
                    EventKind::Error,
                    format!("Twitch refused the claim ({status})"),
                );

                self.publish(event);
                self.push_snapshot();

                return Err(Error::ClaimRefused(status));
            }
            Err(error) => {
                match &error {
                    Error::Unauthorized => self.session_refused().await,
                    other => self.set_problem(other.code()),
                }

                self.push_snapshot();

                return Err(error);
            }
        }

        self.push_snapshot();

        Ok(())
    }

    async fn claim_bonus(&mut self, channel_id: &str, claim_id: &str) {
        let listed = self
            .streamer_channels
            .values()
            .find(|c| c.id == channel_id)
            .map(|c| c.login.clone());

        let login = match listed {
            Some(login) => {
                let allowed = self
                    .settings
                    .points
                    .streamers
                    .iter()
                    .any(|s| s.login == login && s.claim_bonus);

                if !allowed {
                    return;
                }

                login
            }
            None => match self.drops_points_channel(channel_id) {
                Some(channel) => channel.login.clone(),
                None => return,
            },
        };

        if self
            .bonus_claims
            .get(channel_id)
            .is_some_and(|claimed| claimed == claim_id)
        {
            return;
        }

        self.bonus_claims
            .insert(channel_id.to_owned(), claim_id.to_owned());

        match self.client.claim_bonus(channel_id, claim_id).await {
            Ok(balance) => {
                self.stats.bonuses_claimed += 1;

                let name = self.channel_name(&login);

                if let Some(points) = self.streamer_points.get_mut(&login) {
                    if balance > 0 {
                        points.balance = balance;
                    }

                    points.last_bonus_at = Some(Utc::now());
                }

                let event = self
                    .emit(
                        EventKind::BonusClaimed,
                        format!("Bonus chest claimed on {name}"),
                    )
                    .context(EventContext {
                        balance: Some(balance).filter(|balance| *balance > 0),
                        ..EventContext::channel(&login)
                    });

                self.publish(event);
            }
            Err(Error::Gql { code, .. }) if BONUS_GONE.contains(&code.as_str()) => {
                tracing::debug!(account = %self.login, channel = %login, %code, "bonus chest already gone");
            }
            Err(error) => {
                self.bonus_claims.remove(channel_id);
                self.handle_error(error).await;
            }
        }
    }

    async fn handle_pubsub(&mut self, event: PubSubEvent) {
        match event {
            PubSubEvent::Unauthorized => self.handle_error(Error::Unauthorized).await,
            PubSubEvent::Reconnected => {}
            PubSubEvent::StreamUp { channel_id } => {
                if let Some(channel) = self
                    .streamer_channels
                    .values_mut()
                    .find(|c| c.id == channel_id)
                {
                    channel.live = true;
                }

                self.channels_at = None;
            }
            PubSubEvent::StreamDown { channel_id } => {
                if let Some(slot) = self.slots.iter_mut().find(|s| s.channel.id == channel_id) {
                    slot.channel.live = false;
                }

                if let Some(channel) = self
                    .streamer_channels
                    .values_mut()
                    .find(|c| c.id == channel_id)
                {
                    channel.live = false;
                }

                self.channels_at = None;
            }
            PubSubEvent::ViewCount {
                channel_id,
                viewers,
            } => {
                for channel in self
                    .streamer_channels
                    .values_mut()
                    .filter(|c| c.id == channel_id)
                {
                    channel.viewers = Some(viewers);
                }
            }
            PubSubEvent::PointsEarned {
                channel_id,
                earned,
                balance,
                reason,
            } => {
                let listed = self
                    .streamer_channels
                    .values()
                    .find(|c| c.id == channel_id)
                    .map(|c| c.login.clone());

                if let Some(login) = &listed {
                    let waiting = self.own_spends.pending(login);
                    let points = self.streamer_points.entry(login.clone()).or_default();

                    // ! Until Twitch's `points-spent` arrives, a balance it gives may or may not
                    // count a spend of the miner's own: the earned points are added, and that
                    // message settles the balance.
                    match balance {
                        Some(balance) if waiting == 0 => points.balance = balance,
                        _ => points.balance += earned,
                    }

                    points.earned_session += earned;

                    if reason == pubsub::WATCH_STREAK_REASON {
                        self.streak_bonus_paid(login);
                    }
                }

                let login = listed.or_else(|| {
                    self.drops_points_channel(&channel_id)
                        .map(|channel| channel.login.clone())
                });

                if let Some(login) = login {
                    self.stats.points_earned += earned;

                    let event = self
                        .emit(
                            EventKind::PointsEarned,
                            format!("+{earned} points on {login} ({reason})"),
                        )
                        .context(EventContext {
                            earned: Some(earned),
                            balance,
                            reason: Some(reason.to_string()),
                            ..EventContext::channel(&login)
                        });

                    self.publish(event);
                }
            }
            PubSubEvent::PointsSpent {
                channel_id,
                balance,
            } => {
                let listed = self
                    .streamer_channels
                    .values()
                    .find(|c| c.id == channel_id)
                    .map(|c| c.login.clone());
                // ! Twitch sends only the new balance: the amount is what the balance it gave
                // last lost. The miner's own spends are off the local balance already, so
                // they are added back to find it.
                let mut spent = None;

                if let Some(login) = &listed {
                    let points = self.streamer_points.entry(login.clone()).or_default();
                    let before = points.balance + self.own_spends.pending(login);
                    let amount = Some(before - balance).filter(|amount| *amount > 0);

                    spent = self.own_spends.settle(login, amount);
                    points.balance = balance - self.own_spends.pending(login);
                }

                let login = listed.or_else(|| {
                    self.drops_points_channel(&channel_id)
                        .map(|channel| channel.login.clone())
                });

                if let Some(login) = login.filter(|_| spent != Some(0)) {
                    let title = match spent {
                        Some(amount) => format!("-{amount} points on {login}"),
                        None => format!("Points spent on {login} (balance {balance})"),
                    };
                    let event = self
                        .emit(EventKind::PointsSpent, title)
                        .context(EventContext {
                            spent,
                            balance: Some(balance),
                            ..EventContext::channel(&login)
                        });

                    self.publish(event);
                }
            }
            PubSubEvent::ClaimAvailable {
                channel_id,
                claim_id,
            } => self.claim_bonus(&channel_id, &claim_id).await,
            PubSubEvent::Raid {
                channel_id,
                raid_id,
                target_login,
            } => {
                let login = self
                    .streamer_channels
                    .values()
                    .find(|c| c.id == channel_id)
                    .map(|c| c.login.clone());
                let allowed = login.as_ref().is_some_and(|l| {
                    self.settings
                        .points
                        .streamers
                        .iter()
                        .any(|s| &s.login == l && s.follow_raids)
                });

                if allowed
                    && self
                        .raid_joins
                        .due(&channel_id, &raid_id, tokio::time::Instant::now())
                {
                    match self.client.join_raid(&raid_id).await {
                        Ok(()) => {
                            self.raid_joins.joined(&channel_id, &raid_id);

                            let event = self
                                .emit(
                                    EventKind::RaidJoined,
                                    format!(
                                        "Joined the raid of {} to {target_login}",
                                        login.clone().unwrap_or_default()
                                    ),
                                )
                                .context(EventContext {
                                    channel: login.clone(),
                                    ..EventContext::default()
                                });

                            self.publish(event);
                        }
                        Err(error) => {
                            tracing::debug!(account = %self.login, %error, "raid join failed")
                        }
                    }
                }
            }
            PubSubEvent::Moment {
                channel_id,
                moment_id,
            } => {
                let login = self
                    .streamer_channels
                    .values()
                    .find(|c| c.id == channel_id)
                    .map(|c| c.login.clone());
                let allowed = login.as_ref().is_some_and(|l| {
                    self.settings
                        .points
                        .streamers
                        .iter()
                        .any(|s| &s.login == l && s.claim_moments)
                });

                if allowed {
                    match self.client.claim_moment(&moment_id).await {
                        Ok(()) => {
                            let event = self
                                .emit(
                                    EventKind::MomentClaimed,
                                    format!(
                                        "Moment claimed on {}",
                                        login.clone().unwrap_or_default()
                                    ),
                                )
                                .context(EventContext {
                                    channel: login.clone(),
                                    ..EventContext::default()
                                });

                            self.publish(event);
                        }
                        Err(error) => {
                            tracing::debug!(account = %self.login, %error, "moment claim failed")
                        }
                    }
                }
            }
            PubSubEvent::PredictionCreated(event) | PubSubEvent::PredictionUpdated(event) => {
                self.consider_prediction(event);
            }
            PubSubEvent::PredictionMade {
                event_id,
                outcome_id,
                points,
            } => {
                self.predictions_placed
                    .insert(event_id, (outcome_id, points));
            }
            PubSubEvent::PredictionResult {
                event_id,
                result,
                points_won,
            } => {
                let placed = self.predictions_placed.remove(&event_id);
                let staked = placed.map(|(_, p)| p).unwrap_or(0);
                let result = BetResult::from_twitch(&result);

                let title = match result {
                    BetResult::Win => {
                        self.stats.predictions_won += 1;

                        format!("Prediction won: +{points_won} points (staked {staked})")
                    }
                    BetResult::Lose => {
                        self.stats.predictions_lost += 1;

                        format!("Prediction lost: -{staked} points")
                    }
                    BetResult::Refund => format!("Prediction refunded: {staked} points returned"),
                };

                let event = self
                    .emit(EventKind::PredictionResult, title)
                    .context(EventContext {
                        bet_result: Some(result),
                        points_won: Some(points_won),
                        staked: Some(staked),
                        ..EventContext::default()
                    });

                self.publish(event);
            }
            PubSubEvent::DropProgress {
                drop_id,
                current_minutes,
                required_minutes,
                ..
            } => {
                let mut matched: Option<(String, String, String, u32)> = None;

                for campaign in self.campaigns.values_mut() {
                    if let Some(drop) = campaign.drops.iter_mut().find(|d| d.id == drop_id) {
                        drop.current_minutes = drop.current_minutes.max(current_minutes);

                        if required_minutes > 0 {
                            drop.required_minutes = required_minutes;
                        }

                        matched = Some((
                            campaign.id.clone(),
                            campaign.name.clone(),
                            drop.name.clone(),
                            drop.required_minutes,
                        ));
                        break;
                    }
                }

                if let Some((campaign_id, campaign_name, drop_name, required)) = matched {
                    let event = self
                        .emit(
                            EventKind::DropProgress,
                            format!(
                                "{drop_name}: {current_minutes}/{required} min ({campaign_name})"
                            ),
                        )
                        .context(EventContext {
                            current: Some(current_minutes),
                            required: Some(required),
                            ..EventContext::drop(&campaign_id, &drop_id)
                        });

                    self.publish(event);

                    if current_minutes >= required {
                        self.inventory_at = None;
                    }
                }
            }
            PubSubEvent::DropClaim {
                drop_id,
                instance_id,
            } => {
                for campaign in self.campaigns.values_mut() {
                    if let Some(drop) = campaign.drops.iter_mut().find(|d| d.id == drop_id) {
                        drop.instance_id = Some(instance_id.clone());
                        drop.current_minutes = drop.required_minutes;
                    }
                }

                self.claim_available_drops().await;
            }
            PubSubEvent::StreakRecovered { channel_id } => {
                let login = self
                    .streamer_channels
                    .values()
                    .find(|c| c.id == channel_id)
                    .map(|c| c.login.clone());
                let name = login
                    .as_deref()
                    .map_or_else(|| channel_id.clone(), |l| self.channel_name(l));
                let event = self
                    .emit(
                        EventKind::StreakRecovered,
                        format!("Watch streak recovered on {name}"),
                    )
                    .context(login.map(EventContext::channel).unwrap_or_default());

                self.publish(event);
            }
        }
    }

    fn consider_prediction(&mut self, event: PredictionEvent) {
        if event.status != "ACTIVE" {
            self.pending_bets.retain(|(p, _)| p.id != event.id);

            return;
        }

        let login = self
            .streamer_channels
            .values()
            .find(|c| c.id == event.channel_id)
            .map(|c| c.login.clone());
        let Some(login) = login else { return };
        let Some(settings) = self
            .settings
            .points
            .streamers
            .iter()
            .find(|s| s.login == login)
        else {
            return;
        };

        if !settings.predictions.enabled || self.predictions_placed.contains_key(&event.id) {
            return;
        }

        let due = predictions::bet_time(&event, &settings.predictions);

        if let Some(pending) = self.pending_bets.iter_mut().find(|(p, _)| p.id == event.id) {
            pending.0 = event;
        } else {
            self.pending_bets.push((event, due));
        }
    }

    fn next_bet_at(&self) -> Option<tokio::time::Instant> {
        let due = self.pending_bets.iter().map(|(_, at)| *at).min()?;
        let wait = (due - Utc::now()).to_std().unwrap_or(Duration::ZERO);
        Some(tokio::time::Instant::now() + wait)
    }

    async fn place_due_bets(&mut self) {
        let now = Utc::now();
        let due: Vec<PredictionEvent> = self
            .pending_bets
            .iter()
            .filter(|(_, at)| *at <= now)
            .map(|(e, _)| e.clone())
            .collect();

        self.pending_bets.retain(|(_, at)| *at > now);

        for event in due {
            if now > event.closes_at() {
                continue;
            }

            let login = self
                .streamer_channels
                .values()
                .find(|c| c.id == event.channel_id)
                .map(|c| c.login.clone());
            let Some(login) = login else { continue };
            let Some(settings) = self
                .settings
                .points
                .streamers
                .iter()
                .find(|s| s.login == login)
                .map(|s| s.predictions.clone())
            else {
                continue;
            };
            let balance = self
                .streamer_points
                .get(&login)
                .map(|p| p.balance)
                .unwrap_or(0);
            let Some(decision) = predictions::decide(&event, &settings, balance) else {
                continue;
            };

            match self
                .client
                .make_prediction(&event.id, &decision.outcome_id, decision.points)
                .await
            {
                Ok(()) => {
                    self.spent_here(&login, decision.points);
                    self.predictions_placed.insert(
                        event.id.clone(),
                        (decision.outcome_id.clone(), decision.points),
                    );

                    let outcome = event
                        .outcomes
                        .get(decision.outcome_index)
                        .map(|o| o.title.clone())
                        .unwrap_or_default();
                    let title = format!(
                        "Bet {} points on \"{outcome}\" ({}) for {login}",
                        decision.points, event.title
                    );
                    let miner_event =
                        self.emit(EventKind::PredictionPlaced, title)
                            .context(EventContext {
                                points: Some(decision.points),
                                outcome: Some(outcome),
                                ..EventContext::channel(&login)
                            });

                    self.publish(miner_event);
                }
                Err(Error::Gql { code, .. })
                    if PREDICTION_CLOSED.iter().any(|part| code.contains(part)) =>
                {
                    let miner_event = self
                        .emit(
                            EventKind::Info,
                            format!(
                                "Bet not placed on {login}: the prediction closed first ({code})"
                            ),
                        )
                        .context(EventContext::channel(&login));

                    self.publish(miner_event);
                }
                Err(error) => {
                    let miner_event = self.emit(
                        EventKind::Error,
                        format!("Prediction on {login} failed: {error}"),
                    );

                    self.publish(miner_event);
                }
            }
        }
    }

    async fn contribute_goals(&mut self, now: tokio::time::Instant) {
        if self.settings.mode == Mode::Drops {
            return;
        }

        let due: Vec<(String, String)> = self
            .settings
            .points
            .streamers
            .iter()
            .filter(|s| s.enabled && s.community_goals)
            .filter_map(|s| {
                let channel = self.streamer_channels.get(&s.login).filter(|c| c.live)?;
                Some((s.login.clone(), channel.id.clone()))
            })
            .filter(|(login, _)| {
                self.goals_at
                    .get(login)
                    .is_none_or(|at| now.duration_since(*at) >= GOALS_INTERVAL)
            })
            .collect();

        for (login, channel_id) in due {
            self.goals_at.insert(login.clone(), now);

            let goals = match self.client.community_goals(&login).await {
                Ok(goals) => goals,
                Err(error) => {
                    self.handle_error(error).await;
                    continue;
                }
            };

            let open: Vec<Goal> = goals.into_iter().filter(Goal::open).collect();

            if open.is_empty() {
                continue;
            }

            let given = match self.client.goal_contributions(&login).await {
                Ok(given) => given,
                Err(error) => {
                    self.handle_error(error).await;
                    continue;
                }
            };

            for goal in open {
                let balance = self.streamer_points.get(&login).map_or(0, |p| p.balance);
                let left_for_user =
                    goal.per_stream_user_maximum - given.get(&goal.id).copied().unwrap_or(0);
                let amount = (goal.amount_needed - goal.points_contributed)
                    .min(left_for_user)
                    .min(balance);

                if amount <= 0 {
                    continue;
                }

                match self
                    .client
                    .contribute_goal(&channel_id, &goal.id, amount)
                    .await
                {
                    Ok(()) => {
                        self.spent_here(&login, amount);

                        let name = self.channel_name(&login);
                        let event = self
                            .emit(
                                EventKind::GoalContributed,
                                format!(
                                    "{amount} points given to the goal {} of {name}",
                                    goal.title
                                ),
                            )
                            .context(EventContext {
                                points: Some(amount),
                                goal: Some(goal.title.clone()),
                                ..EventContext::channel(&login)
                            });

                        self.publish(event);
                    }
                    Err(error) => self.handle_error(error).await,
                }
            }
        }
    }

    fn streak_bonus_paid(&mut self, login: &str) {
        let Some(broadcast) = self
            .streamer_channels
            .get(login)
            .and_then(|channel| channel.broadcast_id.clone())
        else {
            return;
        };

        let bonus = self
            .streak_bonuses
            .entry(login.to_owned())
            .or_insert_with(|| StreakBonus::new(&broadcast));

        if bonus.broadcast_id != broadcast {
            *bonus = StreakBonus::new(&broadcast);
        }

        bonus.settled = true;
    }

    fn settle_streak_bonuses(&mut self, now: DateTime<Utc>) {
        for (login, bonus) in &mut self.streak_bonuses {
            if !bonus.settled && bonus.window_passed(now) {
                bonus.settled = true;

                tracing::debug!(
                    account = %self.login,
                    channel = %login,
                    "no watch streak bonus on this broadcast within {} minutes",
                    scheduler::STREAK_BONUS_WINDOW.num_minutes()
                );
            }
        }
    }

    fn drops_points_channel(&self, channel_id: &str) -> Option<&Channel> {
        let drops = &self.settings.drops;

        if !drops.mine_points {
            return None;
        }

        self.slots
            .iter()
            .filter(|slot| matches!(slot.purpose, SlotPurpose::Drop { .. }))
            .map(|slot| &slot.channel)
            .find(|channel| channel.id == channel_id)
            .filter(|channel| !drops.points_followed_only || self.followed.contains(&channel.login))
    }

    fn spent_here(&mut self, login: &str, amount: i64) {
        if let Some(points) = self.streamer_points.get_mut(login) {
            points.balance -= amount;
        }

        self.own_spends.add(login, amount);
    }

    fn channel_name(&self, login: &str) -> String {
        self.streamer_channels
            .get(login)
            .or_else(|| {
                self.slots
                    .iter()
                    .map(|s| &s.channel)
                    .find(|c| c.login == login)
            })
            .map(|c| c.display_name.clone())
            .unwrap_or_else(|| login.to_owned())
    }

    async fn refresh_follows(&mut self, now: tokio::time::Instant) {
        let drops = &self.settings.drops;
        let needed =
            drops.mine_points && drops.points_followed_only && self.settings.mode != Mode::Points;

        if !needed
            || self
                .followed_at
                .is_some_and(|at| now.duration_since(at) < FOLLOWS_INTERVAL)
        {
            return;
        }

        match self.client.follows(FOLLOWS_MAX).await {
            Ok(channels) => {
                self.followed = channels.into_iter().map(|c| c.login).collect();
                self.followed_at = Some(now);
            }
            Err(error) => self.handle_error(error).await,
        }
    }

    fn sync_chat(&mut self) {
        let wanted: BTreeSet<String> = if self.settings.mode == Mode::Drops {
            BTreeSet::new()
        } else {
            self.settings
                .points
                .streamers
                .iter()
                .filter(|s| s.enabled && s.chat != ChatPresence::Never)
                .filter(|s| {
                    let live = self.streamer_channels.get(&s.login).map(|c| c.live);

                    match s.chat {
                        ChatPresence::Always => true,
                        ChatPresence::Online => live == Some(true),
                        ChatPresence::Offline => live == Some(false),
                        ChatPresence::Never => false,
                    }
                })
                .map(|s| s.login.clone())
                .take(chat::MAX_CHANNELS)
                .collect()
        };

        if wanted.is_empty() && self.chat.is_none() {
            return;
        }

        let chat = self
            .chat
            .get_or_insert_with(|| self.client.start_chat(self.cancel.child_token()));

        chat.set_channels(wanted);
    }

    fn sync_pubsub(&mut self) {
        if self.pubsub.is_none() {
            self.pubsub = Some(
                self.client
                    .start_pubsub(self.pubsub_sender.clone(), self.cancel.child_token()),
            );
        }

        let mut channels: Vec<String> = self.slots.iter().map(|s| s.channel.id.clone()).collect();

        for streamer in &self.settings.points.streamers {
            if let Some(channel) = self.streamer_channels.get(&streamer.login) {
                if streamer.enabled && !channels.contains(&channel.id) {
                    channels.push(channel.id.clone());
                }
            }
        }

        let predictions: Vec<String> = self
            .settings
            .points
            .streamers
            .iter()
            .filter(|s| s.enabled && s.predictions.enabled)
            .filter_map(|s| self.streamer_channels.get(&s.login).map(|c| c.id.clone()))
            .collect();

        if let Some(pubsub) = &self.pubsub {
            pubsub.set_topics(pubsub::topics_for(&self.id, &channels, &predictions));
        }
    }

    fn push_snapshot(&mut self) {
        let now = Utc::now();
        let slots = self
            .slots
            .iter()
            .map(|slot| {
                let drop = match &slot.purpose {
                    SlotPurpose::Drop {
                        campaign_id,
                        drop_id,
                    } => self.campaigns.get(campaign_id).and_then(|c| {
                        c.drops
                            .iter()
                            .find(|d| &d.id == drop_id)
                            .map(|d| DropProgressView {
                                campaign_id: c.id.clone(),
                                campaign_name: c.name.clone(),
                                game: c.game.name.clone(),
                                drop_id: d.id.clone(),
                                drop_name: d.name.clone(),
                                current_minutes: d.current_minutes,
                                required_minutes: d.required_minutes,
                                image_url: d
                                    .benefits
                                    .first()
                                    .and_then(|b| b.image_url.clone())
                                    .or_else(|| c.image_url.clone()),
                            })
                    }),
                    SlotPurpose::Points { .. } => None,
                };

                SlotView {
                    channel: slot.channel.clone(),
                    purpose: slot.purpose.clone(),
                    since: slot.since,
                    last_beacon_at: slot.last_beacon_at,
                    segments_requested: slot
                        .playback
                        .as_ref()
                        .map(|p| p.segments_requested)
                        .unwrap_or(0),
                    drop,
                    points: self.streamer_points.get(&slot.channel.login).cloned(),
                }
            })
            .collect();
        let streamers = self
            .settings
            .points
            .streamers
            .iter()
            .map(|s| {
                let points = self
                    .streamer_points
                    .get(&s.login)
                    .cloned()
                    .unwrap_or_default();

                StreamerView {
                    login: s.login.clone(),
                    channel: self.streamer_channels.get(&s.login).cloned(),
                    streak_at_risk: points
                        .streak
                        .as_ref()
                        .is_some_and(|streak| streak.at_risk(now)),
                    points,
                    watching: self.slots.iter().any(|slot| slot.channel.login == s.login),
                }
            })
            .collect();
        let mut campaigns: Vec<Campaign> = self
            .campaigns
            .values()
            .filter(|c| c.active_at(now) || c.in_inventory)
            .cloned()
            .collect();

        campaigns.sort_by_key(|c| c.ends_at);

        let snapshot = AccountSnapshot {
            id: self.id.clone(),
            login: self.login.clone(),
            display_name: self.display_name.clone(),
            avatar_url: self.avatar_url.clone(),
            settings: self.settings.clone(),
            status: self.status,
            problem: self.problem,
            slots,
            queue: self.last_queue.clone(),
            streamers,
            campaigns,
            stats: self.stats.clone(),
            updated_at: now,
        };

        let _ = self.snapshot.send(snapshot);
    }
}

fn watched_logins(settings: &AccountSettings) -> BTreeSet<&str> {
    settings
        .points
        .streamers
        .iter()
        .filter(|streamer| streamer.enabled)
        .map(|streamer| streamer.login.as_str())
        .collect()
}

fn shows(event: &PubSubEvent) -> bool {
    !matches!(
        event,
        PubSubEvent::ViewCount { .. } | PubSubEvent::Reconnected
    )
}

fn plays(channel: &Channel, game: &Game) -> bool {
    channel
        .game
        .as_ref()
        .is_some_and(|played| played.id == game.id)
}

fn miner_of(purpose: &SlotPurpose) -> MinerKind {
    match purpose {
        SlotPurpose::Drop { .. } => MinerKind::Drops,
        SlotPurpose::Points { .. } => MinerKind::Points,
    }
}

async fn next_mention(chat: &mut Option<Chat>) -> ChatMention {
    match chat {
        Some(chat) => chat.next_mention().await,
        None => std::future::pending().await,
    }
}

async fn sleep_until_some(at: Option<tokio::time::Instant>) {
    match at {
        Some(at) => tokio::time::sleep_until(at).await,
        None => std::future::pending().await,
    }
}
