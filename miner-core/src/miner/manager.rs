use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use futures_util::future::join_all;
use tokio::sync::{broadcast, oneshot, Mutex, Notify, RwLock};
use tokio_util::sync::CancellationToken;

use super::account::{self, AccountCommand, AccountContext, AccountHandle};
use super::banners::Banners;
use super::record_event;
use super::spam_points::{spam_channels, SpamAccess, SpamPoints};
use crate::api::{
    AccountSnapshot, AccountStats, AccountStatus, CatalogRefresh, CatalogView,
    ChannelsLookupResponse, Command, HistoryQuery, HistoryResponse, LoginFlowStatus, LoginFlowView,
    LogsQuery, LogsReportResponse, LogsResponse, NotificationTest, ProxyTestResponse,
    RedeemRequest, RequestsQuery, RequestsResponse, Shell, Snapshot, CHANNELS_LOOKUP_MAX,
    FOLLOWS_MAX, HISTORY_LIMIT_DEFAULT, HISTORY_LIMIT_MAX, LOGS_LIMIT_DEFAULT, LOGS_LIMIT_MAX,
    REQUESTS_LIMIT_DEFAULT, REQUESTS_LIMIT_MAX,
};
use crate::catalog::{Catalog, CatalogClient};
use crate::config::{AccountSettings, GlobalSettings};
use crate::domain::Channel;
use crate::error::{Error, ErrorCode, Result};
use crate::events::{EventContext, EventKind, MinerEvent};
use crate::logs::LogFilter;
use crate::notifications::Notifier;
use crate::requests::{RequestCursor, RequestFilter, RequestLog, REPORT_MAX};
use crate::spam::{BurstProgress, SpamBalance, SpamChannelView};
use crate::store::vault::{random_salt, Vault, VaultMode};
use crate::store::{AccountRecord, Credential, HistoryFilter, Store, VaultStatus};
use crate::twitch::auth::{self, DeviceCode, Poll};
use crate::twitch::{TwitchClient, TwitchHttp};
use crate::util::{locked, read_locked, write_locked};
use zeroize::Zeroizing;

const RECENT_EVENTS: usize = 100;

fn recent_kinds() -> Vec<EventKind> {
    EventKind::ALL
        .into_iter()
        .filter(|kind| kind.in_recent())
        .collect()
}

fn read_recent(store: &Store) -> Result<Vec<MinerEvent>> {
    let kinds = recent_kinds();
    let filter = HistoryFilter {
        kinds: Some(&kinds),
        ..HistoryFilter::default()
    };

    Ok(store.history(&filter, None, RECENT_EVENTS)?.0)
}
// ! Twitch's search suggests nothing under this many letters.
const SEARCH_MIN_CHARS: usize = 2;
const SEARCH_MAX_CHARS: usize = 60;
// ! Twitch logins are 4 to 25 characters long: shorter text is only a
// fragment, not worth reading apart.
const LOGIN_MIN_CHARS: usize = 4;

const MANUAL_REFRESH_GAP: Duration = Duration::from_secs(300);
const CATALOG_CACHE_KEY: &str = "catalog_cache";
const CATALOG_CACHE_MAX_AGE: chrono::Duration = chrono::Duration::days(7);
const HISTORY_PRUNE_EVERY: Duration = Duration::from_secs(3600);
const REQUESTS_TICK: Duration = Duration::from_secs(60);
const TASK_STOP_TIMEOUT: Duration = Duration::from_secs(5);
const PASSPHRASE_MIN_CHARS: usize = 4;
const PASSPHRASE_TRIES: u32 = 5;
const PASSPHRASE_PAUSE: Duration = Duration::from_secs(30);
const UPDATE_ANNOUNCED_KEY: &str = "update_announced";
const UPDATE_FAILED_KEY: &str = "update_failed";

struct LoginFlow {
    view: LoginFlowView,
    cancel: CancellationToken,
}

#[derive(Default)]
pub(crate) struct WrongPassphrases {
    count: u32,
    paused_until: Option<tokio::time::Instant>,
}

impl WrongPassphrases {
    pub(crate) fn paused(&self, now: tokio::time::Instant) -> bool {
        self.paused_until.is_some_and(|until| now < until)
    }

    pub(crate) fn record(&mut self, right: bool, now: tokio::time::Instant) {
        if right {
            *self = Self::default();

            return;
        }

        self.count += 1;

        if self.count >= PASSPHRASE_TRIES {
            self.count = 0;
            self.paused_until = Some(now + PASSPHRASE_PAUSE);
        }
    }
}

pub struct Manager {
    pub store: Store,
    vault: RwLock<Option<Vault>>,
    // ! Swapped when the default proxy changes: read through `http()` at each use.
    http: std::sync::RwLock<Arc<TwitchHttp>>,
    device_id: String,
    catalog: Arc<RwLock<Catalog>>,
    catalog_client: CatalogClient,
    catalog_refresh: Notify,
    catalog_fetching: Mutex<()>,
    catalog_next_manual: std::sync::Mutex<chrono::DateTime<Utc>>,
    shell: Shell,
    settings: RwLock<GlobalSettings>,
    accounts: Mutex<HashMap<String, AccountHandle>>,
    carried_stats: std::sync::Mutex<HashMap<String, AccountStats>>,
    events: broadcast::Sender<MinerEvent>,
    notifier: Notifier,
    login: Mutex<Option<LoginFlow>>,
    pub changed: Arc<Notify>,
    pub logs: crate::logs::LogBuffer,
    pub requests: RequestLog,
    cancel: CancellationToken,
    banners: Banners,
    recent: std::sync::Mutex<VecDeque<MinerEvent>>,
    start_failures: std::sync::Mutex<HashMap<String, Error>>,
    announcing: std::sync::Mutex<()>,
    update_available: std::sync::Mutex<Option<String>>,
    update_checked_at: std::sync::Mutex<Option<chrono::DateTime<Utc>>>,
    // ! A key derivation takes 64 MiB and a good part of a second: one at a time.
    deriving: Mutex<()>,
    wrong_passphrases: std::sync::Mutex<WrongPassphrases>,
    restarts: std::sync::Mutex<HashMap<String, Arc<Mutex<()>>>>,
    spam: Arc<SpamPoints>,
}

impl Manager {
    pub fn open(data_dir: &Path, logs: crate::logs::LogBuffer, shell: Shell) -> Result<Arc<Self>> {
        crate::logs::log_panics();

        let store = Store::open(data_dir)?;
        let settings = store.global_settings()?;

        let device_id = match store.get_setting("device_id")? {
            Some(id) => id,
            None => {
                let id = crate::util::random_device_id();
                store.set_setting("device_id", &id)?;
                id
            }
        };

        let vault = store.open_default_vault()?;
        let catalog = cached_catalog(&store);
        let next_manual = catalog.fetched_at.map_or_else(Utc::now, |at| {
            at + chrono::Duration::from_std(MANUAL_REFRESH_GAP).unwrap_or_default()
        });
        let recent = read_recent(&store)?;
        let (events, _) = broadcast::channel(512);
        let changed = Arc::new(Notify::new());
        let cancel = CancellationToken::new();
        let spam = SpamPoints::new(
            store.clone(),
            events.clone(),
            changed.clone(),
            cancel.clone(),
        );
        let requests = RequestLog::new(Some(store.clone()));
        let http = shared_http(&device_id, &settings.default_proxy, &requests)?;

        Ok(Arc::new(Self {
            notifier: Notifier::new(
                settings.notifications.clone(),
                &settings.language,
                requests.clone(),
            )?,
            store,
            vault: RwLock::new(vault),
            http: std::sync::RwLock::new(Arc::new(http)),
            device_id,
            catalog: Arc::new(RwLock::new(catalog)),
            catalog_client: CatalogClient::new(requests.clone())?,
            catalog_refresh: Notify::new(),
            catalog_fetching: Mutex::new(()),
            catalog_next_manual: std::sync::Mutex::new(next_manual),
            shell,
            settings: RwLock::new(settings),
            accounts: Mutex::new(HashMap::new()),
            carried_stats: std::sync::Mutex::new(HashMap::new()),
            events,
            login: Mutex::new(None),
            changed,
            logs,
            requests,
            cancel,
            banners: Banners::default(),
            recent: std::sync::Mutex::new(recent.into()),
            start_failures: std::sync::Mutex::new(HashMap::new()),
            announcing: std::sync::Mutex::new(()),
            update_available: std::sync::Mutex::new(None),
            update_checked_at: std::sync::Mutex::new(None),
            deriving: Mutex::new(()),
            wrong_passphrases: std::sync::Mutex::new(WrongPassphrases::default()),
            restarts: std::sync::Mutex::new(HashMap::new()),
            spam: Arc::new(spam),
        }))
    }

    pub async fn start(self: &Arc<Self>) {
        let this = self.clone();
        tokio::spawn(async move { this.catalog_loop().await });

        let this = self.clone();
        tokio::spawn(async move { this.event_loop().await });

        if self.vault.read().await.is_some() {
            self.spawn_all_accounts().await;
        }
    }

    pub async fn shutdown(&self) {
        self.cancel.cancel();
        self.spam.interrupt().await;

        let handles: Vec<AccountHandle> =
            self.accounts.lock().await.drain().map(|(_, h)| h).collect();

        join_all(handles.into_iter().map(stop_task)).await;
        self.notifier.shutdown();
        self.requests.flush();
    }

    pub fn subscribe(&self) -> broadcast::Receiver<MinerEvent> {
        self.events.subscribe()
    }

    pub fn subscribe_bursts(&self) -> broadcast::Receiver<BurstProgress> {
        self.spam.subscribe()
    }

    async fn event_loop(self: Arc<Self>) {
        let mut receiver = self.events.subscribe();

        self.prune_history().await;

        let mut pruned = tokio::time::Instant::now();
        let mut tick = tokio::time::interval(REQUESTS_TICK);

        loop {
            tokio::select! {
                _ = self.cancel.cancelled() => break,
                _ = tick.tick() => {
                    self.requests.close_due();

                    if pruned.elapsed() > HISTORY_PRUNE_EVERY {
                        pruned = tokio::time::Instant::now();
                        self.prune_history().await;
                    }
                }
                event = receiver.recv() => match event {
                    Ok(event) => {
                        self.remember(&event);
                        self.notifier.notify(event);
                        self.changed.notify_waiters();
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        self.reload_recent();
                        self.changed.notify_waiters();
                    }
                    Err(_) => break,
                }
            }
        }
    }

    fn recent(&self) -> std::sync::MutexGuard<'_, VecDeque<MinerEvent>> {
        locked(&self.recent)
    }

    fn remember(&self, event: &MinerEvent) {
        if !event.kind.in_recent() {
            return;
        }

        let mut recent = self.recent();

        recent.push_front(event.clone());
        recent.truncate(RECENT_EVENTS);
    }

    fn reload_recent(&self) {
        match read_recent(&self.store) {
            Ok(events) => *self.recent() = events.into(),
            Err(error) => tracing::warn!(%error, "recent events not read again"),
        }
    }

    fn start_failures(&self) -> std::sync::MutexGuard<'_, HashMap<String, Error>> {
        locked(&self.start_failures)
    }

    async fn catalog_loop(self: Arc<Self>) {
        let mut wait = self
            .catalog
            .read()
            .await
            .fetched_at
            .map_or(Duration::ZERO, |at| {
                let due =
                    at + chrono::Duration::from_std(CatalogClient::interval()).unwrap_or_default();

                (due - Utc::now()).to_std().unwrap_or(Duration::ZERO)
            });

        loop {
            tokio::select! {
                _ = self.cancel.cancelled() => return,
                _ = tokio::time::sleep(wait) => {}
                _ = self.catalog_refresh.notified() => {
                    wait = CatalogClient::interval();

                    continue;
                }
            }

            wait = CatalogClient::interval().max(self.fetch_catalog().await);
        }
    }

    async fn fetch_catalog(&self) -> Duration {
        let _one_at_a_time = self.catalog_fetching.lock().await;

        if Utc::now() < self.next_manual_refresh() {
            return Duration::ZERO;
        }

        self.fetch_catalog_locked().await.1
    }

    async fn fetch_catalog_locked(&self) -> (Result<usize>, Duration) {
        let fetch = self.catalog_client.fetch().await;
        let gap = match &fetch.result {
            Ok(_) | Err(Error::CatalogRateLimited) => MANUAL_REFRESH_GAP.max(fetch.retry_after),
            Err(_) => Duration::ZERO,
        };

        *locked(&self.catalog_next_manual) =
            Utc::now() + chrono::Duration::from_std(gap).unwrap_or_default();

        let result = match fetch.result {
            Ok(next) => {
                let count = next.campaigns.len();
                let json = serde_json::to_string(&next);

                tracing::info!(campaigns = count, "catalogue refreshed");
                *self.catalog.write().await = next;
                self.keep_catalog(json).await;

                Ok(count)
            }
            Err(Error::CatalogRateLimited) => {
                let retry_in_seconds = CatalogClient::interval().max(fetch.retry_after).as_secs();
                let mut catalog = self.catalog.write().await;

                if catalog.campaigns.is_empty() {
                    tracing::info!(
                        retry_in_seconds,
                        "catalogue refresh postponed: the Twitch Drops API asks to wait"
                    );
                } else {
                    tracing::info!(
                        retry_in_seconds,
                        "catalogue refresh postponed: the Twitch Drops API asks to wait, the saved catalogue is used"
                    );
                }

                catalog.problem = Some(Error::CatalogRateLimited.code());
                Err(Error::CatalogRateLimited)
            }
            Err(error) => {
                tracing::warn!(%error, "catalogue refresh failed");
                self.catalog.write().await.problem = Some(error.code());
                Err(error)
            }
        };

        self.changed.notify_waiters();

        if result.is_ok() {
            self.refresh_all().await;
        }

        (result, fetch.retry_after)
    }

    async fn keep_catalog(&self, json: serde_json::Result<String>) {
        let kept = async {
            let json = json?;
            let store = self.store.clone();

            tokio::task::spawn_blocking(move || store.set_setting(CATALOG_CACHE_KEY, &json))
                .await
                .map_err(|error| Error::Storage(error.to_string()))?
        }
        .await;

        if let Err(error) = kept {
            tracing::warn!(%error, "catalogue not kept for the next start");
        }
    }

    fn next_manual_refresh(&self) -> chrono::DateTime<Utc> {
        *locked(&self.catalog_next_manual)
    }

    pub async fn refresh_catalog(&self) -> Result<CatalogRefresh> {
        let _one_at_a_time = self.catalog_fetching.lock().await;

        if Utc::now() < self.next_manual_refresh() {
            return Err(Error::CatalogRateLimited);
        }

        let (result, _) = self.fetch_catalog_locked().await;

        self.catalog_refresh.notify_one();

        Ok(CatalogRefresh {
            campaign_count: u32::try_from(result?).unwrap_or(u32::MAX),
            next_refresh_at: self.next_manual_refresh(),
        })
    }

    pub fn logs_page(&self, query: &LogsQuery) -> LogsResponse {
        let limit = query
            .limit
            .unwrap_or(LOGS_LIMIT_DEFAULT)
            .clamp(1, LOGS_LIMIT_MAX) as usize;
        let login = self.log_login(query.account_id.as_deref());
        let filter = LogFilter {
            account: login.as_deref(),
            levels: query.levels.as_deref(),
        };
        let page = self.logs.page(&filter, query.before_id, limit);

        LogsResponse {
            lines: page.lines,
            next_before_id: page.next_before_id,
        }
    }

    pub fn logs_report(&self, query: &LogsQuery) -> LogsReportResponse {
        let limit = query
            .limit
            .map_or(crate::logs::CAPACITY, |limit| limit as usize);

        LogsReportResponse {
            report: self.logs_export(query, limit),
        }
    }

    pub fn history_page(&self, query: &HistoryQuery) -> Result<HistoryResponse> {
        let limit = query
            .limit
            .unwrap_or(HISTORY_LIMIT_DEFAULT)
            .clamp(1, HISTORY_LIMIT_MAX) as usize;
        let filter = HistoryFilter {
            account_id: query.account_id.as_deref(),
            category: query.category,
            kinds: query.kinds.as_deref(),
        };
        let (events, next_before_id) = self.store.history(&filter, query.before_id, limit)?;

        Ok(HistoryResponse {
            events,
            next_before_id,
        })
    }

    async fn prune_history(&self) {
        let retention = self.settings.read().await.history_retention;

        if let Some(age) = retention.max_age() {
            let cutoff = Utc::now() - age;
            let store = self.store.clone();
            let pruned = tokio::task::spawn_blocking(move || {
                store.prune_history(cutoff)?;
                store.prune_requests(cutoff)
            })
            .await;

            match pruned {
                Ok(Ok(_)) => {}
                Ok(Err(error)) => tracing::warn!(%error, "history pruning failed"),
                Err(error) => tracing::warn!(%error, "history pruning stopped"),
            }

            self.recent().retain(|event| event.at >= cutoff);
        }
    }

    pub fn requests_page(&self, query: &RequestsQuery) -> Result<RequestsResponse> {
        let limit = query
            .limit
            .unwrap_or(REQUESTS_LIMIT_DEFAULT)
            .clamp(1, REQUESTS_LIMIT_MAX) as usize;
        let filter = RequestFilter {
            account_id: query.account_id.as_deref(),
            kinds: query.kinds.as_deref(),
            errors_only: query.errors_only.unwrap_or(false),
        };
        let cursor = RequestCursor {
            live_id: query.before_live_id,
            stored_id: query.before_stored_id,
        };
        let page = self.requests.page(&filter, cursor, limit)?;

        Ok(RequestsResponse {
            requests: page.requests,
            next_live_id: page.next.live_id,
            next_stored_id: page.next.stored_id,
        })
    }

    pub fn requests_report(&self, query: &RequestsQuery) -> Result<LogsReportResponse> {
        let filter = RequestFilter {
            account_id: query.account_id.as_deref(),
            kinds: query.kinds.as_deref(),
            errors_only: query.errors_only.unwrap_or(false),
        };
        let limit = query
            .limit
            .map_or(REPORT_MAX, |limit| limit as usize)
            .clamp(1, REPORT_MAX);
        let logins: HashMap<String, String> = self
            .store
            .accounts()?
            .into_iter()
            .map(|account| (account.id, account.login))
            .collect();

        Ok(LogsReportResponse {
            report: format!(
                "{}\n{}",
                report_header(),
                self.requests.report(&filter, &logins, limit)?
            ),
        })
    }

    pub async fn vault_status(&self) -> VaultStatus {
        VaultStatus {
            mode: self.store.vault_mode().unwrap_or(VaultMode::KeyFile),
            unlocked: self.vault.read().await.is_some(),
        }
    }

    pub async fn unlock(self: &Arc<Self>, passphrase: &str) -> Result<()> {
        if self.vault.read().await.is_some() {
            return Ok(());
        }

        let vault = self.open_with_passphrase(passphrase).await?;

        *self.vault.write().await = Some(vault);
        self.spawn_all_accounts().await;
        self.changed.notify_waiters();

        Ok(())
    }

    pub async fn set_passphrase(
        self: &Arc<Self>,
        current: Option<&str>,
        next: Option<&str>,
    ) -> Result<()> {
        if self.shell == Shell::Server {
            return Err(Error::NotSupported);
        }

        // ! Only `None` removes the password: a blank one would silently
        // switch the vault back to the key file.
        if next.is_some_and(|passphrase| passphrase.trim().chars().count() < PASSPHRASE_MIN_CHARS) {
            return Err(Error::InvalidRequest("passphrase too short".to_owned()));
        }

        let mode = self.store.vault_mode()?;

        if mode == VaultMode::KeyFile && next.is_none() {
            return Err(Error::InvalidRequest(
                "no app password to remove".to_owned(),
            ));
        }

        let current_vault = match mode {
            VaultMode::KeyFile => Vault::from_key_file(&self.store.dir)?,
            VaultMode::Passphrase => {
                self.open_with_passphrase(current.unwrap_or_default())
                    .await?
            }
        };

        let unlocked = self.vault.read().await.is_some();

        if !unlocked {
            return Err(Error::VaultLocked);
        }

        // ! Running accounts and a pending sign-in hold the old key: a session
        // they renew during the rekey would be sealed with it, then unreadable.
        self.cancel_login().await;
        self.stop_all_accounts().await;

        let switched = self.switch_vault(&current_vault, next).await;

        self.spawn_all_accounts().await;
        self.changed.notify_waiters();

        switched
    }

    async fn open_with_passphrase(&self, passphrase: &str) -> Result<Vault> {
        let _one_at_a_time = self.deriving.lock().await;

        if self.wrong_passphrases().paused(tokio::time::Instant::now()) {
            return Err(Error::TooManyAttempts);
        }

        let store = self.store.clone();
        let passphrase = Zeroizing::new(passphrase.to_owned());
        let opened = tokio::task::spawn_blocking(move || store.unlock_with_passphrase(&passphrase))
            .await
            .map_err(|_| Error::Storage("the key derivation stopped".to_owned()))?;

        match &opened {
            Ok(_) => self
                .wrong_passphrases()
                .record(true, tokio::time::Instant::now()),
            Err(Error::WrongPassphrase) => self
                .wrong_passphrases()
                .record(false, tokio::time::Instant::now()),
            Err(_) => {}
        }

        opened
    }

    fn wrong_passphrases(&self) -> std::sync::MutexGuard<'_, WrongPassphrases> {
        locked(&self.wrong_passphrases)
    }

    // ! Kept exactly as typed: unlocking compares it as typed too.
    async fn switch_vault(&self, current: &Vault, next: Option<&str>) -> Result<()> {
        let next_vault = match next {
            Some(passphrase) => {
                let salt = random_salt();
                let vault = {
                    let _one_at_a_time = self.deriving.lock().await;
                    let passphrase = Zeroizing::new(passphrase.to_owned());
                    let salt = salt.clone();

                    tokio::task::spawn_blocking(move || Vault::from_passphrase(&passphrase, &salt))
                        .await
                        .map_err(|_| Error::Storage("the key derivation stopped".to_owned()))??
                };

                self.store
                    .rekey(current, &vault, VaultMode::Passphrase, Some(&salt))?;

                vault
            }
            None => {
                crate::store::vault::remove_key_file(&self.store.dir);

                let vault = Vault::from_key_file(&self.store.dir)?;

                self.store
                    .rekey(current, &vault, VaultMode::KeyFile, None)?;

                vault
            }
        };

        *self.vault.write().await = Some(next_vault);

        Ok(())
    }

    async fn stop_all_accounts(&self) {
        let handles: Vec<AccountHandle> =
            self.accounts.lock().await.drain().map(|(_, h)| h).collect();

        for handle in &handles {
            self.keep_stats(handle);
        }

        join_all(handles.into_iter().map(stop_task)).await;
    }

    // ! Only with a passphrase: a key-file vault would reopen by itself.
    pub async fn lock(&self) -> Result<()> {
        if self.shell == Shell::Server {
            return Err(Error::NotSupported);
        }

        if self.store.vault_mode()? != VaultMode::Passphrase {
            return Err(Error::PassphraseRequired);
        }

        self.cancel_login().await;
        self.spam.cancel_bursts(None);
        self.stop_all_accounts().await;
        *self.vault.write().await = None;
        self.changed.notify_waiters();

        Ok(())
    }

    fn http(&self) -> Arc<TwitchHttp> {
        read_locked(&self.http).clone()
    }

    async fn vault(&self) -> Result<Vault> {
        self.vault.read().await.clone().ok_or(Error::VaultLocked)
    }

    async fn spawn_all_accounts(self: &Arc<Self>) {
        let Ok(vault) = self.vault().await else {
            return;
        };
        let records = match self.store.accounts() {
            Ok(records) => records,
            Err(error) => {
                tracing::warn!(%error, "no account could start");

                return;
            }
        };

        for record in records {
            let restart = self.restart_lock(&record.id);
            let _one_at_a_time = restart.lock().await;

            if let Err(error) = self.spawn_account(record, &vault).await {
                tracing::warn!(%error, "account could not start");
            }
        }
    }

    async fn spawn_account(self: &Arc<Self>, record: AccountRecord, vault: &Vault) -> Result<()> {
        let mut accounts = self.accounts.lock().await;

        if accounts.contains_key(&record.id) {
            return Ok(());
        }

        let id = record.id.clone();

        match self.start_task(record, vault).await {
            Ok(handle) => {
                self.start_failures().remove(&id);
                accounts.insert(id, handle);

                Ok(())
            }
            Err(error) => {
                let failure = match error {
                    Error::WrongPassphrase => Error::InvalidVault,
                    ref other => other.clone(),
                };

                self.start_failures().insert(id, failure);

                Err(error)
            }
        }
    }

    async fn start_task(
        self: &Arc<Self>,
        record: AccountRecord,
        vault: &Vault,
    ) -> Result<AccountHandle> {
        let credential = self
            .store
            .load_credential(&record.id, vault)?
            .ok_or_else(|| Error::UnknownAccount(record.id.clone()))?;
        let proxy = self.effective_proxy(&record.settings.proxy).await;
        // ! Never without the proxy: the account would reach Twitch from the
        // real address.
        let http = TwitchHttp::new(
            self.device_id.clone(),
            proxy.as_deref(),
            self.requests.clone(),
        )
        .map_err(|error| match (error, &proxy) {
            (Error::InvalidProxy(why), _) => Error::InvalidProxy(why),
            (other, Some(_)) => Error::InvalidProxy(other.to_string()),
            (other, None) => other,
        })?;
        let stats = locked(&self.carried_stats)
            .remove(&record.id)
            .unwrap_or_default();
        let context = AccountContext {
            store: self.store.clone(),
            vault: vault.clone(),
            http: Arc::new(http),
            catalog: self.catalog.clone(),
            events: self.events.clone(),
            stats,
            spends: self.spam.spends(&record.id),
        };
        let handle = account::spawn(record, credential, context);
        let mut watcher = handle.snapshot.clone();
        let this = self.clone();

        tokio::spawn(async move {
            while watcher.changed().await.is_ok() {
                this.changed.notify_waiters();
            }
        });

        Ok(handle)
    }

    async fn effective_proxy(&self, account_proxy: &str) -> Option<String> {
        let own = account_proxy.trim();

        if !own.is_empty() {
            return Some(own.to_owned());
        }

        let default = self.settings.read().await.default_proxy.trim().to_owned();

        (!default.is_empty()).then_some(default)
    }

    fn keep_stats(&self, handle: &AccountHandle) {
        let stats = handle.snapshot.borrow().stats.clone();

        locked(&self.carried_stats).insert(handle.id.clone(), stats);
    }

    fn restart_lock(&self, account_id: &str) -> Arc<Mutex<()>> {
        locked(&self.restarts)
            .entry(account_id.to_owned())
            .or_default()
            .clone()
    }

    async fn respawn_account(self: &Arc<Self>, account_id: &str) -> Result<()> {
        let vault = self.vault().await?;
        let restart = self.restart_lock(account_id);
        let _one_at_a_time = restart.lock().await;

        // ! Taken out in its own statement: in an `if let` the lock would stay
        // held while the task stops, and every snapshot and command would wait.
        let running = self.accounts.lock().await.remove(account_id);

        if let Some(handle) = running {
            self.keep_stats(&handle);
            stop_task(handle).await;
        }

        let record = self
            .store
            .account(account_id)?
            .ok_or_else(|| Error::UnknownAccount(account_id.to_owned()))?;

        self.spawn_account(record, &vault).await
    }

    pub async fn order_campaign(
        self: &Arc<Self>,
        account_id: &str,
        campaign_id: &str,
        position: Option<usize>,
    ) -> Result<()> {
        let record = self
            .store
            .account(account_id)?
            .ok_or_else(|| Error::UnknownAccount(account_id.to_owned()))?;
        let mut settings = record.settings;

        settings.drops.campaign_order.retain(|id| id != campaign_id);

        if let Some(position) = position {
            let position = position.min(settings.drops.campaign_order.len());

            settings
                .drops
                .campaign_order
                .insert(position, campaign_id.to_owned());
        }

        self.set_account_settings(account_id, settings).await
    }

    async fn refresh_all(&self) {
        let accounts = self.accounts.lock().await;

        for handle in accounts.values() {
            let _ = handle.commands.try_send(AccountCommand::Refresh);
        }
    }

    // ! The queue is waited on outside the lock: an account busy with a long
    // pass must not hold every snapshot and every other command.
    async fn send_command(&self, account_id: &str, command: AccountCommand) -> Result<()> {
        let commands = self
            .accounts
            .lock()
            .await
            .get(account_id)
            .map(|handle| handle.commands.clone())
            .ok_or_else(|| Error::UnknownAccount(account_id.to_owned()))?;

        commands
            .send(command)
            .await
            .map_err(|_| Error::UnknownAccount(account_id.to_owned()))
    }

    pub async fn set_account_settings(
        self: &Arc<Self>,
        account_id: &str,
        settings: AccountSettings,
    ) -> Result<()> {
        settings.validate()?;

        let mut settings = settings;

        settings.proxy = settings.proxy.trim().to_owned();

        if !settings.proxy.is_empty() {
            crate::twitch::parse_proxy(&settings.proxy)?;
        }

        let previous_proxy = self
            .store
            .account(account_id)?
            .map(|a| a.settings.proxy)
            .unwrap_or_default();

        settings
            .points
            .streamers
            .retain(|s| !s.login.trim().is_empty());

        for streamer in &mut settings.points.streamers {
            streamer.login = crate::util::normalize_login(&streamer.login)
                .ok_or_else(|| Error::InvalidStreamerLogin(streamer.login.clone()))?;
        }

        let mut seen = std::collections::HashSet::new();

        settings
            .points
            .streamers
            .retain(|streamer| seen.insert(streamer.login.clone()));

        let defaults = &mut settings.points.streamer_defaults;

        defaults.login = String::new();
        defaults.enabled = true;

        self.store.save_account_settings(account_id, &settings)?;

        if previous_proxy != settings.proxy {
            self.respawn_account(account_id).await?;
        } else {
            self.send_command(account_id, AccountCommand::Settings(Box::new(settings)))
                .await?;
        }

        self.changed.notify_waiters();

        Ok(())
    }

    pub async fn reorder_accounts(&self, ids: &[String]) -> Result<()> {
        self.store.reorder_accounts(ids)?;
        self.changed.notify_waiters();
        Ok(())
    }

    pub async fn remove_account(&self, account_id: &str) -> Result<()> {
        let restart = self.restart_lock(account_id);
        let _one_at_a_time = restart.lock().await;
        let running = self.accounts.lock().await.remove(account_id);

        self.spam.forget_account(account_id);

        if let Some(handle) = running {
            stop_task(handle).await;
        }

        if let Ok(vault) = self.vault().await {
            if let Ok(Some(credential)) = self.store.load_credential(account_id, &vault) {
                auth::revoke(
                    &self.http(),
                    Some(account_id),
                    &credential.tokens.access_token,
                )
                .await;
            }
        }

        self.requests.forget(account_id);
        self.store.delete_account(account_id)?;
        self.recent().retain(|event| event.account_id != account_id);
        self.start_failures().remove(account_id);
        self.changed.notify_waiters();

        Ok(())
    }

    pub async fn refresh_account(&self, account_id: &str) -> Result<()> {
        self.send_command(account_id, AccountCommand::Refresh).await
    }

    pub async fn claim_drop(&self, account_id: &str, instance: &str) -> Result<()> {
        let (reply, answer) = oneshot::channel();

        self.send_command(
            account_id,
            AccountCommand::ClaimDrop(instance.to_owned(), reply),
        )
        .await?;

        answer
            .await
            .map_err(|_| Error::UnknownAccount(account_id.to_owned()))?
    }

    pub async fn login_status(&self) -> Option<LoginFlowView> {
        self.login.lock().await.as_ref().map(|f| f.view.clone())
    }

    pub async fn cancel_login(&self) {
        if let Some(flow) = self.login.lock().await.take() {
            flow.cancel.cancel();
        }

        self.changed.notify_waiters();
    }

    pub async fn start_login(self: &Arc<Self>) -> Result<LoginFlowView> {
        let vault = self.vault().await?;

        self.cancel_login().await;

        let code = auth::start_device_flow(&self.http()).await?;
        let view = LoginFlowView {
            flow_id: crate::util::random_hex(8),
            user_code: code.user_code.clone(),
            verification_uri: code.verification_uri.clone(),
            issued_at: code.issued_at,
            expires_at: code.expires_at,
            status: LoginFlowStatus::Pending,
            problem: None,
            account_id: None,
        };
        let cancel = CancellationToken::new();

        *self.login.lock().await = Some(LoginFlow {
            view: view.clone(),
            cancel: cancel.clone(),
        });

        let this = self.clone();
        let flow_id = view.flow_id.clone();

        tokio::spawn(async move {
            let outcome = tokio::select! {
                _ = cancel.cancelled() => return,
                outcome = this.poll_login(code, &vault) => outcome,
            };

            let mut login = this.login.lock().await;

            if let Some(flow) = login.as_mut().filter(|f| f.view.flow_id == flow_id) {
                match outcome {
                    Ok(account_id) => {
                        flow.view.status = LoginFlowStatus::Completed;
                        flow.view.account_id = Some(account_id);
                    }
                    Err(Error::DeviceCodeExpired) => flow.view.status = LoginFlowStatus::Expired,
                    Err(error) => {
                        flow.view.status = LoginFlowStatus::Failed;
                        tracing::warn!("sign-in failed: {error}");
                        flow.view.problem = Some(error.code());
                    }
                }
            }

            this.changed.notify_waiters();
        });

        self.changed.notify_waiters();

        Ok(view)
    }

    async fn poll_login(self: &Arc<Self>, code: DeviceCode, vault: &Vault) -> Result<String> {
        let interval = Duration::from_secs(code.interval_seconds.max(2));

        loop {
            tokio::time::sleep(interval).await;

            let http = self.http();

            match auth::poll_device_flow(&http, &code).await? {
                Poll::Pending => continue,
                Poll::Token(tokens) => {
                    let validation = auth::validate(&http, None, &tokens.access_token)
                        .await?
                        .ok_or(Error::Unauthorized)?;
                    let existing = self.store.account(&validation.user_id)?;
                    let (display_name, avatar_url) = {
                        let client = TwitchClient::new(
                            http.clone(),
                            validation.user_id.clone(),
                            validation.login.clone(),
                            tokens.access_token.clone(),
                        );

                        match client.channel(&validation.login).await {
                            Ok(Some(channel)) => (channel.display_name, channel.avatar_url),
                            _ => (validation.login.clone(), None),
                        }
                    };
                    let position = self.store.accounts()?.len() as i64;
                    let record = AccountRecord {
                        id: validation.user_id.clone(),
                        login: validation.login.clone(),
                        display_name,
                        avatar_url,
                        settings: existing.as_ref().map(|e| e.settings.clone()).unwrap_or(
                            AccountSettings {
                                enabled: false,
                                ..AccountSettings::default()
                            },
                        ),
                        position: existing.as_ref().map(|e| e.position).unwrap_or(position),
                        created_at: existing
                            .as_ref()
                            .map(|e| e.created_at)
                            .unwrap_or_else(Utc::now),
                    };

                    self.store.upsert_account(&record)?;

                    let restart = self.restart_lock(&record.id);
                    let _one_at_a_time = restart.lock().await;

                    // ! A re-login replaces the running task, stopped first: a
                    // session it renews now would overwrite the new credential.
                    let running = self.accounts.lock().await.remove(&record.id);

                    if let Some(handle) = running {
                        self.keep_stats(&handle);
                        stop_task(handle).await;
                    }

                    let credential = Credential {
                        tokens,
                        device_id: self.device_id.clone(),
                    };

                    self.store.save_credential(&record.id, &credential, vault)?;

                    let id = record.id.clone();
                    let context = EventContext {
                        account_name: Some(record.display_name.clone()),
                        account_avatar: record.avatar_url.clone(),
                        ..EventContext::default()
                    };

                    self.spawn_account(record, vault).await?;

                    let event = MinerEvent::new(
                        &id,
                        &validation.login,
                        EventKind::LoginCompleted,
                        format!("{} signed in", validation.login),
                    )
                    .context(context);

                    record_event(&self.store, &self.events, event);

                    return Ok(id);
                }
            }
        }
    }

    pub fn clear_update(&self) {
        let shown = locked(&self.update_available).take();

        if shown.is_some() {
            self.changed.notify_waiters();
        }
    }

    pub fn update_checked(&self) {
        *locked(&self.update_checked_at) = Some(Utc::now());

        self.changed.notify_waiters();
    }

    pub fn note_failed_update(&self, version: &str) -> Result<()> {
        self.store.set_setting(UPDATE_FAILED_KEY, version)
    }

    pub fn take_failed_update(&self) -> Result<Option<String>> {
        let version = self.store.get_setting(UPDATE_FAILED_KEY)?;

        if version.is_some() {
            self.store.delete_setting(UPDATE_FAILED_KEY)?;
        }

        Ok(version)
    }

    pub fn announce_update(&self, version: &str) -> Result<bool> {
        let _one_at_a_time = locked(&self.announcing);
        let shown = locked(&self.update_available).replace(version.to_owned());

        if shown.as_deref() != Some(version) {
            self.changed.notify_waiters();
        }

        if self.store.get_setting(UPDATE_ANNOUNCED_KEY)?.as_deref() == Some(version) {
            return Ok(false);
        }

        self.store.set_setting(UPDATE_ANNOUNCED_KEY, version)?;

        let event = MinerEvent::new(
            "",
            "",
            EventKind::UpdateAvailable,
            format!("Version {version} is available"),
        )
        .context(EventContext {
            version: Some(version.to_owned()),
            ..EventContext::default()
        });

        record_event(&self.store, &self.events, event);

        Ok(true)
    }

    pub async fn global_settings(&self) -> GlobalSettings {
        self.settings.read().await.clone()
    }

    pub async fn set_global_settings(self: &Arc<Self>, settings: GlobalSettings) -> Result<()> {
        settings.validate()?;

        let mut settings = settings;

        settings.default_proxy = settings.default_proxy.trim().to_owned();

        if !settings.default_proxy.is_empty() {
            crate::twitch::parse_proxy(&settings.default_proxy)?;
        }

        let proxy_changed = self.settings.read().await.default_proxy != settings.default_proxy;
        let http = if proxy_changed {
            Some(shared_http(
                &self.device_id,
                &settings.default_proxy,
                &self.requests,
            )?)
        } else {
            None
        };

        self.store.save_global_settings(&settings)?;
        self.notifier
            .update(settings.notifications.clone(), &settings.language);
        *self.settings.write().await = settings;

        if let Some(http) = http {
            *write_locked(&self.http) = Arc::new(http);

            let ids: Vec<String> = self
                .store
                .accounts()?
                .into_iter()
                .filter(|a| a.settings.proxy.trim().is_empty())
                .map(|a| a.id)
                .collect();
            let restarts = ids.iter().map(|id| async move {
                if let Err(error) = self.respawn_account(id).await {
                    tracing::warn!(%error, account = %id, "account could not restart with the new proxy");
                }
            });

            join_all(restarts).await;
        }

        self.changed.notify_waiters();

        Ok(())
    }

    fn log_login(&self, account_id: Option<&str>) -> Option<String> {
        account_id
            .and_then(|id| self.store.account(id).ok().flatten())
            .map(|a| a.login)
    }

    fn logs_export(&self, query: &LogsQuery, limit: usize) -> String {
        let login = self.log_login(query.account_id.as_deref());
        let filter = LogFilter {
            account: login.as_deref(),
            levels: query.levels.as_deref(),
        };

        format!(
            "{}\n{}",
            report_header(),
            self.logs
                .export(&filter, limit.clamp(1, crate::logs::CAPACITY))
        )
    }

    pub async fn snapshot(&self) -> Result<Snapshot> {
        let records = self.store.accounts()?;
        let vault = self.vault_status().await;
        let accounts_lock = self.accounts.lock().await;
        let start_failures: HashMap<String, ErrorCode> = self
            .start_failures()
            .iter()
            .map(|(id, error)| (id.clone(), error.code()))
            .collect();
        let accounts: Vec<AccountSnapshot> = records
            .iter()
            .map(|record| match accounts_lock.get(&record.id) {
                // ! The stored settings, not the task's: it applies a change only after its
                // current pass, and a snapshot taken in between would show the old value (a
                // Pause flipping back).
                Some(handle) => {
                    let mut account = handle.snapshot.borrow().clone();

                    account.settings = record.settings.clone();

                    if !account.settings.enabled && account.status != AccountStatus::LoginRequired {
                        account.status = AccountStatus::Paused;
                    }

                    account
                }
                None => {
                    let (status, problem) = match (vault.unlocked, start_failures.get(&record.id)) {
                        (false, _) => (AccountStatus::LoginRequired, Some(ErrorCode::VaultLocked)),
                        (true, Some(code)) => (failed_start_status(*code), Some(*code)),
                        (true, None) => (AccountStatus::Starting, None),
                    };

                    AccountSnapshot {
                        id: record.id.clone(),
                        login: record.login.clone(),
                        display_name: record.display_name.clone(),
                        avatar_url: record.avatar_url.clone(),
                        settings: record.settings.clone(),
                        status,
                        problem,
                        slots: vec![],
                        queue: vec![],
                        streamers: vec![],
                        campaigns: vec![],
                        stats: Default::default(),
                        updated_at: Utc::now(),
                        spam_channels: vec![],
                    }
                }
            })
            .collect();

        drop(accounts_lock);

        let mut spam_rows = self.store.spam_rows()?;
        let mut accounts = accounts;

        for account in &mut accounts {
            let rows = spam_rows.remove(&account.id).unwrap_or_default();

            account.spam_channels = spam_channels(account, &rows);
        }

        let catalog = self.catalog.read().await;
        let settings = self.settings.read().await.clone();
        let interval = CatalogClient::interval().as_secs();
        let mut recent_events: Vec<MinerEvent> = self.recent().iter().cloned().collect();

        for event in &mut recent_events {
            if let Some(account) = records.iter().find(|r| r.id == event.account_id) {
                event.account_login = account.login.clone();
            }
        }

        let update_available = locked(&self.update_available).clone();
        let update_checked_at = *locked(&self.update_checked_at);

        Ok(Snapshot {
            version: crate::VERSION.to_owned(),
            update_available,
            update_checked_at,
            accounts,
            catalog: CatalogView {
                campaigns: catalog.campaigns.clone(),
                updated_at: catalog.updated_at,
                fetched_at: catalog.fetched_at,
                problem: catalog.problem,
                interval_seconds: interval,
                next_refresh_at: self.next_manual_refresh(),
            },
            settings,
            vault,
            recent_events,
            login: self.login_status().await,
        })
    }

    pub async fn execute(self: &Arc<Self>, command: Command) -> Result<()> {
        match command {
            Command::CancelLogin {} => self.cancel_login().await,
            Command::RemoveAccount { account_id } => self.remove_account(&account_id).await?,
            Command::SetAccountSettings {
                account_id,
                settings,
            } => self.set_account_settings(&account_id, settings).await?,
            Command::ReorderAccounts { account_ids } => self.reorder_accounts(&account_ids).await?,
            Command::SetGlobalSettings { settings } => self.set_global_settings(settings).await?,
            Command::RefreshAccount { account_id } => self.refresh_account(&account_id).await?,
            Command::ClaimDrop {
                account_id,
                drop_instance_id,
            } => self.claim_drop(&account_id, &drop_instance_id).await?,
            Command::SetPassphrase { current, next } => {
                self.set_passphrase(current.as_deref(), next.as_deref())
                    .await?
            }
            Command::Unlock { passphrase } => self.unlock(&passphrase).await?,
            Command::Lock {} => self.lock().await?,
            Command::OrderCampaign {
                account_id,
                campaign_id,
                position,
            } => {
                self.order_campaign(&account_id, &campaign_id, position.map(|p| p as usize))
                    .await?
            }
            Command::AddSpamChannel { account_id, login } => {
                self.add_spam_channel(&account_id, &login)?
            }
            Command::ForgetSpamChannel { account_id, login } => {
                self.forget_spam_channel(&account_id, &login)?
            }
            Command::CancelBurst { burst_id } => self.cancel_burst(&burst_id),
        }

        Ok(())
    }

    pub async fn lookup_channel(&self, login: &str) -> Result<Channel> {
        let login = crate::util::normalize_login(login)
            .ok_or_else(|| Error::UnknownChannel(login.to_owned()))?;

        crate::twitch::channels::lookup_public(&self.http(), &login)
            .await?
            .ok_or(Error::UnknownChannel(login))
    }

    pub async fn channel_banners(
        &self,
        logins: &[String],
    ) -> Result<crate::api::ChannelBannersResponse> {
        let mut seen: HashSet<String> = HashSet::new();
        let wanted: Vec<String> = logins
            .iter()
            .filter_map(|login| crate::util::normalize_login(login))
            .filter(|login| seen.insert(login.clone()))
            .collect();

        within_lookup_max(wanted.len())?;

        let missing = self.banners.missing(&wanted);

        if !missing.is_empty() {
            self.banners
                .keep(crate::twitch::channels::banners(&self.http(), &missing).await?);
        }

        Ok(crate::api::ChannelBannersResponse {
            banners: self.banners.found(wanted),
        })
    }

    pub async fn search(&self, query: &str) -> Result<crate::api::SearchResponse> {
        let query = query.trim();

        if query.chars().count() < SEARCH_MIN_CHARS {
            return Ok(crate::api::SearchResponse::default());
        }

        let fragment: String = query.chars().take(SEARCH_MAX_CHARS).collect();
        let http = self.http();
        let exact =
            crate::util::normalize_login(query).filter(|login| login.len() >= LOGIN_MIN_CHARS);
        let (suggested, read) = tokio::join!(
            crate::twitch::search::suggestions(&http, &fragment),
            async {
                match &exact {
                    // ! Only an extra: a failed read leaves the suggestions as they are.
                    Some(login) => crate::twitch::channels::lookup_public(&http, login)
                        .await
                        .ok()
                        .flatten(),
                    None => None,
                }
            },
        );
        let (games, channels) = suggested?;

        Ok(crate::api::SearchResponse {
            games,
            channels: crate::twitch::search::exact_first(channels, read),
        })
    }

    pub async fn lookup_channels(&self, entries: &[String]) -> Result<ChannelsLookupResponse> {
        let mut wanted: Vec<String> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();
        let mut missing: Vec<String> = Vec::new();

        for entry in entries {
            match crate::util::normalize_login(entry) {
                Some(login) => {
                    if seen.insert(login.clone()) {
                        wanted.push(login);
                    }
                }
                None => {
                    let entry = entry.trim();

                    if !entry.is_empty() && seen.insert(entry.to_owned()) {
                        missing.push(entry.to_owned());
                    }
                }
            }
        }

        within_lookup_max(wanted.len())?;

        let read = crate::twitch::channels::lookup_public_many(&self.http(), &wanted).await?;
        let known: HashSet<&str> = read
            .channels
            .iter()
            .map(|c| c.login.as_str())
            .chain(read.unchecked.iter().map(String::as_str))
            .collect();

        missing.extend(
            wanted
                .iter()
                .filter(|login| !known.contains(login.as_str()))
                .cloned(),
        );

        Ok(ChannelsLookupResponse {
            channels: read.channels,
            missing,
            unchecked: read.unchecked,
        })
    }

    pub async fn followed_channels(&self, account_id: &str) -> Result<Vec<Channel>> {
        let record = self
            .store
            .account(account_id)?
            .ok_or_else(|| Error::UnknownAccount(account_id.to_owned()))?;
        let vault = self.vault().await?;
        let credential = self
            .store
            .load_credential(&record.id, &vault)?
            .ok_or_else(|| Error::UnknownAccount(record.id.clone()))?;
        let proxy = self.effective_proxy(&record.settings.proxy).await;
        let http = Arc::new(TwitchHttp::new(
            self.device_id.clone(),
            proxy.as_deref(),
            self.requests.clone(),
        )?);
        let client = TwitchClient::new(
            http,
            record.id.clone(),
            record.login.clone(),
            credential.tokens.access_token.clone(),
        );

        client.followed_channels(FOLLOWS_MAX).await
    }

    pub async fn test_notification_targets(&self) -> Vec<NotificationTest> {
        self.notifier
            .test()
            .await
            .into_iter()
            .map(|(target, result)| NotificationTest {
                target,
                problem: result
                    .map_err(|error| {
                        tracing::warn!(?target, "notification test failed: {error}");
                        error.code()
                    })
                    .err(),
            })
            .collect()
    }

    pub fn cancel_burst(&self, burst_id: &str) {
        self.spam.cancel_burst(burst_id);
    }

    pub fn add_spam_channel(&self, account_id: &str, login: &str) -> Result<()> {
        self.spam.add_channel(account_id, login)
    }

    pub fn forget_spam_channel(&self, account_id: &str, login: &str) -> Result<()> {
        self.spam.forget_channel(account_id, login)
    }

    pub async fn spam_channel(&self, account_id: &str, login: &str) -> Result<SpamChannelView> {
        self.spam
            .channel(account_id, login, self.spam_access(account_id))
            .await
    }

    pub async fn spam_balance(&self, account_id: &str, login: &str) -> Result<SpamBalance> {
        self.spam
            .balance(account_id, login, self.spam_access(account_id))
            .await
    }

    pub async fn redeem(&self, request: RedeemRequest) -> Result<String> {
        let account_id = request.account_id.clone();
        let delay = Duration::from_millis(self.settings.read().await.spam_points.delay_ms.into());

        self.spam
            .redeem(request, delay, self.spam_access(&account_id))
            .await
    }

    async fn spam_access(&self, account_id: &str) -> Result<SpamAccess> {
        let access = self
            .accounts
            .lock()
            .await
            .get(account_id)
            .map(|handle| SpamAccess {
                client: handle.client.clone(),
                commands: handle.commands.clone(),
                snapshot: handle.snapshot.clone(),
            });

        match access {
            Some(access) => Ok(access),
            None if !self.vault_status().await.unlocked => Err(Error::VaultLocked),
            None => Err(self
                .start_failures()
                .get(account_id)
                .cloned()
                .unwrap_or_else(|| Error::UnknownAccount(account_id.to_owned()))),
        }
    }

    pub async fn test_proxy(&self, url: &str) -> Result<ProxyTestResponse> {
        let latency = crate::twitch::test_proxy(url, self.requests.clone()).await?;

        Ok(ProxyTestResponse {
            latency_ms: u64::try_from(latency.as_millis()).unwrap_or(u64::MAX),
        })
    }
}

// ! A proxy the client refuses leaves these requests direct, rather than
// failing every sign-in and lookup.
fn shared_http(device_id: &str, proxy: &str, requests: &RequestLog) -> Result<TwitchHttp> {
    TwitchHttp::new(device_id.to_owned(), Some(proxy), requests.clone()).or_else(|error| {
        tracing::warn!(%error, "default proxy rejected, continuing without it");
        TwitchHttp::new(device_id.to_owned(), None, requests.clone())
    })
}

// ! Only a credential the task could not use asks for a new sign-in: a
// refused proxy is fixed in the settings.
fn failed_start_status(code: ErrorCode) -> AccountStatus {
    match code {
        ErrorCode::InvalidProxy => AccountStatus::Idle,
        _ => AccountStatus::LoginRequired,
    }
}

fn report_header() -> String {
    format!(
        "{} {} ({}, {})",
        crate::APP_NAME,
        crate::VERSION,
        std::env::consts::OS,
        std::env::consts::ARCH,
    )
}

fn within_lookup_max(count: usize) -> Result<()> {
    if count > CHANNELS_LOOKUP_MAX {
        return Err(Error::InvalidRequest(format!(
            "{count} channels, at most {CHANNELS_LOOKUP_MAX}"
        )));
    }

    Ok(())
}

// ! Two tasks of one account must never run together: both would send the
// beacon, and the old one could save a stale session.
async fn stop_task(handle: AccountHandle) {
    handle.cancel.cancel();

    let mut task = handle.task;

    if tokio::time::timeout(TASK_STOP_TIMEOUT, &mut task)
        .await
        .is_err()
    {
        tracing::info!(account = %handle.id, "account task still busy after the stop timeout, aborted");

        task.abort();

        let _ = task.await;
    }
}

fn cached_catalog(store: &Store) -> Catalog {
    let kept = match store.get_setting(CATALOG_CACHE_KEY) {
        Ok(Some(json)) => serde_json::from_str::<Catalog>(&json)
            .inspect_err(|error| tracing::warn!(%error, "kept catalogue unreadable, fetched again"))
            .ok(),
        Ok(None) => None,
        Err(error) => {
            tracing::warn!(%error, "kept catalogue unreadable");
            None
        }
    };

    match kept {
        Some(catalog)
            if catalog
                .fetched_at
                .is_some_and(|at| Utc::now() - at < CATALOG_CACHE_MAX_AGE) =>
        {
            tracing::info!(
                campaigns = catalog.campaigns.len(),
                "catalogue kept from the last start"
            );
            Catalog {
                problem: None,
                ..catalog
            }
        }
        _ => Catalog::default(),
    }
}
