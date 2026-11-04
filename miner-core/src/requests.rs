use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError, Weak};
use std::time::Instant;

use chrono::{DateTime, DurationRound, TimeDelta, Utc};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;
use url::Url;

use crate::error::{Error, NetworkErrorKind, Result};
use crate::logs::redact;
use crate::store::Store;

pub const LIVE_CAPACITY: usize = 1000;
pub const BUCKET_MINUTES: i64 = 10;
pub const REPORT_MAX: usize = 4000;

const UNSAVED_MAX: usize = 10_000;
const OPENED_KEPT: usize = 6;
const SEGMENT_MAX_CHARS: usize = 32;
const DETAIL_MAX_CHARS: usize = 160;
const MASKED_SEGMENT: &str = "…";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum RequestKind {
    Gql,
    Auth,
    Playlist,
    Segment,
    ChannelPage,
    Beacon,
    Pubsub,
    Chat,
    Catalog,
    Discord,
    Telegram,
    Webhook,
    Picture,
    UpdateCheck,
    UpdateDownload,
    ProxyTest,
}

impl RequestKind {
    pub const ALL: [Self; 16] = [
        Self::Gql,
        Self::Auth,
        Self::Playlist,
        Self::Segment,
        Self::ChannelPage,
        Self::Beacon,
        Self::Pubsub,
        Self::Chat,
        Self::Catalog,
        Self::Discord,
        Self::Telegram,
        Self::Webhook,
        Self::Picture,
        Self::UpdateCheck,
        Self::UpdateDownload,
        Self::ProxyTest,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Gql => "GQL",
            Self::Auth => "AUTH",
            Self::Playlist => "PLAYLIST",
            Self::Segment => "SEGMENT",
            Self::ChannelPage => "CHANNEL_PAGE",
            Self::Beacon => "BEACON",
            Self::Pubsub => "PUBSUB",
            Self::Chat => "CHAT",
            Self::Catalog => "CATALOG",
            Self::Discord => "DISCORD",
            Self::Telegram => "TELEGRAM",
            Self::Webhook => "WEBHOOK",
            Self::Picture => "PICTURE",
            Self::UpdateCheck => "UPDATE_CHECK",
            Self::UpdateDownload => "UPDATE_DOWNLOAD",
            Self::ProxyTest => "PROXY_TEST",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.as_str() == name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum RequestOutcome {
    Ok,
    Status,
    Timeout,
    Connect,
    Network,
    Graphql,
    Closed,
}

impl RequestOutcome {
    pub const ALL: [Self; 7] = [
        Self::Ok,
        Self::Status,
        Self::Timeout,
        Self::Connect,
        Self::Network,
        Self::Graphql,
        Self::Closed,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::Status => "STATUS",
            Self::Timeout => "TIMEOUT",
            Self::Connect => "CONNECT",
            Self::Network => "NETWORK",
            Self::Graphql => "GRAPHQL",
            Self::Closed => "CLOSED",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|outcome| outcome.as_str() == name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum SocketEvent {
    Connect,
    Reconnect,
    Close,
}

impl SocketEvent {
    pub const ALL: [Self; 3] = [Self::Connect, Self::Reconnect, Self::Close];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Connect => "CONNECT",
            Self::Reconnect => "RECONNECT",
            Self::Close => "CLOSE",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|event| event.as_str() == name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum RequestSource {
    Live,
    Stored,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "RequestGroupType")]
pub struct RequestGroup {
    pub count: u32,
    pub max_ms: u64,
    pub until: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "RequestEntryType")]
pub struct RequestEntry {
    pub id: u64,
    pub source: RequestSource,
    pub at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub account_id: Option<String>,
    pub kind: RequestKind,
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub socket: Option<SocketEvent>,
    pub outcome: RequestOutcome,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub status: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub detail: Option<String>,
    pub duration_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub group: Option<RequestGroup>,
}

impl RequestEntry {
    pub fn failed(&self) -> bool {
        self.outcome != RequestOutcome::Ok
    }

    pub fn report_line(&self, login: Option<&str>) -> String {
        let mut line = format!("{} ", self.at.format("%Y-%m-%d %H:%M:%S"));

        if let Some(login) = login {
            line.push_str(&format!("[{login}] "));
        }

        line.push_str(self.kind.as_str());

        if !self.target.is_empty() {
            line.push_str(&format!(" {}", self.target));
        }

        if let Some(socket) = self.socket {
            line.push_str(&format!(" {}", socket.as_str()));
        }

        if let Some(status) = self.status {
            line.push_str(&format!(" {status}"));
        }

        if self.failed() {
            line.push_str(&format!(" {}", self.outcome.as_str()));
        }

        if let Some(detail) = &self.detail {
            line.push_str(&format!(" ({detail})"));
        }

        match &self.group {
            Some(group) => line.push_str(&format!(
                " ×{} until {}, mean {} ms, max {} ms",
                group.count,
                group.until.format("%H:%M"),
                self.duration_ms,
                group.max_ms
            )),
            None => line.push_str(&format!(" {} ms", self.duration_ms)),
        }

        line
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RequestFilter<'a> {
    pub account_id: Option<&'a str>,
    pub kinds: Option<&'a [RequestKind]>,
    pub errors_only: bool,
}

impl RequestFilter<'_> {
    pub fn keeps(&self, entry: &RequestEntry) -> bool {
        self.account_id
            .is_none_or(|account| entry.account_id.as_deref() == Some(account))
            && self.kinds.is_none_or(|kinds| kinds.contains(&entry.kind))
            && (!self.errors_only || entry.failed())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RequestCursor {
    pub live_id: Option<u64>,
    pub stored_id: Option<u64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RequestPage {
    pub requests: Vec<RequestEntry>,
    pub next: RequestCursor,
}

pub fn bucket_of(at: DateTime<Utc>) -> DateTime<Utc> {
    at.duration_trunc(TimeDelta::minutes(BUCKET_MINUTES))
        .unwrap_or(at)
}

pub fn bucket_end(start: DateTime<Utc>) -> DateTime<Utc> {
    start + TimeDelta::minutes(BUCKET_MINUTES)
}

// ! Host and path only, never the query: playback and beacon addresses carry
// signatures there. A long path segment is an opaque id or a signed token.
pub fn url_target(url: &Url) -> String {
    let path: Vec<&str> = url
        .path_segments()
        .into_iter()
        .flatten()
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            if segment.chars().count() > SEGMENT_MAX_CHARS {
                MASKED_SEGMENT
            } else {
                segment
            }
        })
        .collect();
    let host = url.host_str().unwrap_or_default();

    if path.is_empty() {
        return host.to_owned();
    }

    format!("{host}/{}", path.join("/"))
}

pub fn host_target(url: &Url) -> String {
    url.host_str().unwrap_or_default().to_owned()
}

pub fn address_target(address: &str) -> String {
    Url::parse(address)
        .map(|url| url_target(&url))
        .unwrap_or_default()
}

fn clipped(text: &str) -> String {
    let redacted = redact(text);

    if redacted.chars().count() <= DETAIL_MAX_CHARS {
        return redacted;
    }

    let mut cut: String = redacted.chars().take(DETAIL_MAX_CHARS - 1).collect();

    cut.push('…');

    cut
}

fn millis(started: Instant) -> u64 {
    u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX)
}

fn graphql_error(body: &Value) -> Option<String> {
    let first = |item: &Value| {
        item.get("errors")
            .and_then(Value::as_array)
            .and_then(|errors| errors.first())
            .map(|error| {
                error
                    .get("message")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_owned()
            })
    };

    match body {
        Value::Array(items) => items.iter().find_map(first),
        _ => first(body),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GroupKey {
    account_id: Option<String>,
    kind: RequestKind,
    target: String,
    socket: Option<SocketEvent>,
    outcome: RequestOutcome,
    status: Option<u16>,
}

#[derive(Debug, Clone, Copy, Default)]
struct Sum {
    count: u32,
    total_ms: u64,
    max_ms: u64,
}

#[derive(Default)]
struct State {
    live: VecDeque<RequestEntry>,
    next_id: u64,
    bucket: Option<DateTime<Utc>>,
    opened: VecDeque<(DateTime<Utc>, u64)>,
    groups: HashMap<GroupKey, Sum>,
    failures: Vec<RequestEntry>,
    unsaved: VecDeque<RequestEntry>,
    saving_fails: bool,
}

impl State {
    fn grouped(&mut self, bucket: DateTime<Utc>) -> Vec<RequestEntry> {
        self.groups
            .drain()
            .map(|(key, sum)| RequestEntry {
                id: 0,
                source: RequestSource::Stored,
                at: bucket,
                account_id: key.account_id,
                kind: key.kind,
                target: key.target,
                socket: key.socket,
                outcome: key.outcome,
                status: key.status,
                detail: None,
                duration_ms: sum.total_ms / u64::from(sum.count.max(1)),
                group: Some(RequestGroup {
                    count: sum.count,
                    max_ms: sum.max_ms,
                    until: bucket_end(bucket),
                }),
            })
            .collect()
    }

    fn roll(&mut self, now: DateTime<Utc>) -> Vec<RequestEntry> {
        let current = bucket_of(now);
        let closed = match self.bucket {
            Some(open) if open < current => {
                let closed = self.grouped(open);

                self.failures.clear();

                closed
            }
            Some(_) => return Vec::new(),
            None => Vec::new(),
        };

        self.bucket = Some(current);
        self.opened.push_back((current, self.next_id));

        if self.opened.len() > OPENED_KEPT {
            self.opened.pop_front();
        }

        closed
    }

    fn opened_with(&self, live_id: u64) -> Option<DateTime<Utc>> {
        self.opened
            .iter()
            .rev()
            .find(|(_, first_id)| *first_id <= live_id)
            .map(|(start, _)| *start)
    }

    fn keep_unsaved(&mut self, rows: Vec<RequestEntry>) {
        self.unsaved.extend(rows);

        let over = self.unsaved.len().saturating_sub(UNSAVED_MAX);

        if over > 0 {
            self.unsaved.drain(..over);
            tracing::warn!(rows = over, "requests dropped: they could not be saved");
        }
    }

    // ! Live entries are those of the bucket not stored yet: the older ones are
    // in the stored groups already. A failure pushed out of the ring by a burst
    // is still shown from the bucket's own list.
    fn fresh(&self, since: DateTime<Utc>) -> Vec<RequestEntry> {
        let oldest_live = self.live.front().map_or(u64::MAX, |entry| entry.id);
        let mut fresh: Vec<RequestEntry> = self
            .failures
            .iter()
            .filter(|entry| entry.id < oldest_live)
            .chain(self.live.iter().filter(|entry| entry.at >= since))
            .cloned()
            .collect();

        fresh.sort_by_key(|entry| std::cmp::Reverse(entry.id));

        fresh
    }
}

struct Shared {
    state: Mutex<State>,
    store: Option<Store>,
    wake: Option<SyncSender<()>>,
    writing: Mutex<()>,
}

impl Shared {
    fn state(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn wake(&self) {
        if let Some(wake) = &self.wake {
            let _ = wake.try_send(());
        }
    }

    fn save(&self) {
        let Some(store) = &self.store else {
            return;
        };
        let _one_writer = self.writing.lock().unwrap_or_else(PoisonError::into_inner);
        let rows: Vec<RequestEntry> = self.state().unsaved.drain(..).collect();

        if rows.is_empty() {
            return;
        }

        let saved = store.push_requests(&rows);
        let mut state = self.state();

        match saved {
            Ok(()) => state.saving_fails = false,
            Err(error) => {
                if !std::mem::replace(&mut state.saving_fails, true) {
                    tracing::warn!(%error, rows = rows.len(), "requests not saved, tried again at the next tick");
                }

                let newer: Vec<RequestEntry> = state.unsaved.drain(..).collect();

                state.keep_unsaved(rows);
                state.keep_unsaved(newer);
            }
        }
    }
}

// ! SQLite is written here, off the async workers and outside the state's
// lock: a slow disk delays the stored rows, never a request.
fn write_when_woken(shared: Weak<Shared>, woken: Receiver<()>) {
    while woken.recv().is_ok() {
        let Some(shared) = shared.upgrade() else {
            return;
        };

        shared.save();
    }
}

#[derive(Clone)]
pub struct RequestLog {
    shared: Arc<Shared>,
}

impl Default for RequestLog {
    fn default() -> Self {
        Self::new(None)
    }
}

impl RequestLog {
    pub fn new(store: Option<Store>) -> Self {
        let (wake, woken) = match store {
            Some(_) => {
                let (wake, woken) = sync_channel(1);

                (Some(wake), Some(woken))
            }
            None => (None, None),
        };
        let shared = Arc::new(Shared {
            state: Mutex::new(State::default()),
            store,
            wake,
            writing: Mutex::new(()),
        });

        if let Some(woken) = woken {
            let writer = Arc::downgrade(&shared);
            let spawned = std::thread::Builder::new()
                .name("requests".into())
                .spawn(move || write_when_woken(writer, woken));

            if let Err(error) = spawned {
                tracing::warn!(%error, "requests are saved only when read");
            }
        }

        Self { shared }
    }

    fn state(&self) -> MutexGuard<'_, State> {
        self.shared.state()
    }

    fn keep(&self, state: &mut State, rows: Vec<RequestEntry>) -> bool {
        if self.shared.store.is_none() || rows.is_empty() {
            return false;
        }

        state.keep_unsaved(rows);

        true
    }

    pub fn start(&self, kind: RequestKind, target: impl Into<String>) -> Pending {
        Pending {
            log: self.clone(),
            account_id: None,
            kind,
            target: target.into(),
            socket: None,
            expected: &[],
            started: Instant::now(),
        }
    }

    fn record(&self, done: Done) {
        self.record_at(done, Utc::now());
    }

    fn record_at(&self, done: Done, now: DateTime<Utc>) {
        let target = redact(&done.target);
        let detail = done.detail.as_deref().map(clipped);
        let grouped = done.kind == RequestKind::Segment || done.outcome == RequestOutcome::Ok;
        let mut state = self.state();
        let closed = state.roll(now);
        let mut unsaved = self.keep(&mut state, closed);

        if grouped {
            let sum = state
                .groups
                .entry(GroupKey {
                    account_id: done.account_id.clone(),
                    kind: done.kind,
                    target: target.clone(),
                    socket: done.socket,
                    outcome: done.outcome,
                    status: done.status,
                })
                .or_default();

            sum.count = sum.count.saturating_add(1);
            sum.total_ms = sum.total_ms.saturating_add(done.duration_ms);
            sum.max_ms = sum.max_ms.max(done.duration_ms);
        }

        if done.kind == RequestKind::Segment {
            drop(state);

            if unsaved {
                self.shared.wake();
            }

            return;
        }

        let id = state.next_id;

        state.next_id += 1;

        let entry = RequestEntry {
            id,
            source: RequestSource::Live,
            at: now,
            account_id: done.account_id,
            kind: done.kind,
            target,
            socket: done.socket,
            outcome: done.outcome,
            status: done.status,
            detail,
            duration_ms: done.duration_ms,
            group: None,
        };

        if !grouped {
            unsaved |= self.keep(&mut state, vec![entry.clone()]);
            state.failures.push(entry.clone());
        }

        if state.live.len() == LIVE_CAPACITY {
            state.live.pop_front();
        }

        state.live.push_back(entry);
        drop(state);

        if unsaved {
            self.shared.wake();
        }
    }

    pub fn forget(&self, account_id: &str) {
        let mut state = self.state();
        let other = |entry: &RequestEntry| entry.account_id.as_deref() != Some(account_id);

        state.live.retain(other);
        state.failures.retain(other);
        state
            .groups
            .retain(|key, _| key.account_id.as_deref() != Some(account_id));
    }

    pub fn close_due(&self) {
        let mut state = self.state();
        let closed = state.roll(Utc::now());

        self.keep(&mut state, closed);

        let unsaved = !state.unsaved.is_empty();

        drop(state);

        if unsaved {
            self.shared.wake();
        }
    }

    pub fn flush(&self) {
        let mut state = self.state();

        if let Some(bucket) = state.bucket {
            let open = state.grouped(bucket);

            self.keep(&mut state, open);
        }

        drop(state);
        self.shared.save();
    }

    pub fn page(
        &self,
        filter: &RequestFilter,
        cursor: RequestCursor,
        limit: usize,
    ) -> Result<RequestPage> {
        self.shared.save();

        let (fresh, since) = {
            let state = self.state();
            let open = state.bucket.unwrap_or_else(|| bucket_of(Utc::now()));
            // ! A page that goes on down the live list stays in the bucket it
            // started in: one closed since then is on screen row by row already.
            let since = match cursor {
                RequestCursor {
                    live_id: Some(before),
                    stored_id: None,
                } => state.opened_with(before).unwrap_or(open),
                _ => open,
            };

            (state.fresh(since), since)
        };
        let mut page = RequestPage::default();

        if cursor.stored_id.is_none() {
            let mut live = fresh
                .into_iter()
                .filter(|entry| cursor.live_id.is_none_or(|before| entry.id < before))
                .filter(|entry| filter.keeps(entry));

            page.requests.extend(live.by_ref().take(limit));

            if page.requests.len() == limit {
                if live.next().is_some() {
                    page.next.live_id = page.requests.last().map(|entry| entry.id);
                } else {
                    page.next.stored_id = self
                        .stored(filter, since, None, 1)?
                        .0
                        .first()
                        .map(|entry| entry.id.saturating_add(1));
                }

                return Ok(page);
            }
        }

        let room = limit - page.requests.len();
        let (stored, next) = self.stored(filter, since, cursor.stored_id, room)?;

        page.requests.extend(stored);
        page.next.stored_id = next;

        Ok(page)
    }

    fn stored(
        &self,
        filter: &RequestFilter,
        until: DateTime<Utc>,
        before_id: Option<u64>,
        limit: usize,
    ) -> Result<(Vec<RequestEntry>, Option<u64>)> {
        match &self.shared.store {
            Some(store) => store.requests(filter, until, before_id, limit),
            None => Ok((Vec::new(), None)),
        }
    }

    pub fn report(
        &self,
        filter: &RequestFilter,
        logins: &HashMap<String, String>,
        limit: usize,
    ) -> Result<String> {
        let page = self.page(filter, RequestCursor::default(), limit)?;

        Ok(page
            .requests
            .iter()
            .rev()
            .map(|entry| {
                let login = entry
                    .account_id
                    .as_ref()
                    .and_then(|id| logins.get(id))
                    .map(String::as_str);

                entry.report_line(login)
            })
            .collect::<Vec<_>>()
            .join("\n"))
    }
}

struct Done {
    account_id: Option<String>,
    kind: RequestKind,
    target: String,
    socket: Option<SocketEvent>,
    outcome: RequestOutcome,
    status: Option<u16>,
    detail: Option<String>,
    duration_ms: u64,
}

#[must_use = "a request is recorded only once it is finished"]
pub struct Pending {
    log: RequestLog,
    account_id: Option<String>,
    kind: RequestKind,
    target: String,
    socket: Option<SocketEvent>,
    expected: &'static [StatusCode],
    started: Instant,
}

impl Pending {
    pub fn account(mut self, account_id: Option<&str>) -> Self {
        self.account_id = account_id.map(str::to_owned);

        self
    }

    pub fn socket(mut self, event: SocketEvent) -> Self {
        self.socket = Some(event);

        self
    }

    pub fn expecting(mut self, statuses: &'static [StatusCode]) -> Self {
        self.expected = statuses;

        self
    }

    fn finish(self, outcome: RequestOutcome, status: Option<u16>, detail: Option<String>) {
        let duration_ms = millis(self.started);

        self.log.record(Done {
            account_id: self.account_id,
            kind: self.kind,
            target: self.target,
            socket: self.socket,
            outcome,
            status,
            detail,
            duration_ms,
        });
    }

    pub fn succeeded(self, status: Option<StatusCode>) {
        self.finish(
            RequestOutcome::Ok,
            status.map(|status| status.as_u16()),
            None,
        );
    }

    pub fn answered(self, status: StatusCode) {
        let outcome = if status.is_success() || self.expected.contains(&status) {
            RequestOutcome::Ok
        } else {
            RequestOutcome::Status
        };

        self.finish(outcome, Some(status.as_u16()), None);
    }

    // ! Twitch adds an error next to the data when it could not read one field:
    // only an error that fails the call is a failure.
    pub fn graphql(self, status: StatusCode, body: &Value, failed: bool) {
        let outcome = if failed {
            RequestOutcome::Graphql
        } else {
            RequestOutcome::Ok
        };

        self.finish(outcome, Some(status.as_u16()), graphql_error(body));
    }

    pub fn closed(self, reason: &str) {
        self.finish(RequestOutcome::Closed, None, Some(reason.to_owned()));
    }

    pub fn failed(self, error: &Error) {
        match error {
            Error::Status(status) => self.answered(*status),
            Error::Unauthorized => self.answered(StatusCode::UNAUTHORIZED),
            Error::Transport(NetworkErrorKind::Timeout) => {
                self.finish(RequestOutcome::Timeout, None, None)
            }
            Error::Transport(NetworkErrorKind::Connect) => {
                self.finish(RequestOutcome::Connect, None, None)
            }
            Error::Transport(NetworkErrorKind::Other) => {
                self.finish(RequestOutcome::Network, None, None)
            }
            Error::Gql { code, .. } => {
                self.finish(RequestOutcome::Graphql, None, Some(code.clone()))
            }
            other => self.finish(RequestOutcome::Network, None, Some(other.to_string())),
        }
    }

    pub fn sent(self, sent: &reqwest::Result<reqwest::Response>) {
        match sent {
            Ok(response) => self.answered(response.status()),
            Err(error) => match error.status() {
                Some(status) => self.answered(status),
                None if error.is_timeout() => self.finish(RequestOutcome::Timeout, None, None),
                None if error.is_connect() => self.finish(RequestOutcome::Connect, None, None),
                None => self.finish(RequestOutcome::Network, None, None),
            },
        }
    }

    pub async fn send(self, request: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        let sent = request.send().await;

        self.sent(&sent);

        Ok(sent?)
    }
}
