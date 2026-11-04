use std::fmt;
use std::marker::PhantomData;

use chrono::{DateTime, Utc};
use serde::de::{self, IntoDeserializer, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use ts_rs::TS;

use crate::config::{AccountSettings, GlobalSettings};
use crate::domain::{Campaign, Channel, PointsState};
use crate::error::{Error, ErrorCode};
use crate::events::{EventCategory, EventKind, MinerEvent};
use crate::logs::{LogLevel, LogLine};
use crate::requests::{RequestEntry, RequestKind};
use crate::scheduler::{QueueItem, SlotPurpose};
use crate::spam::{
    valid_burst_id, BurstProgress, SpamBalance, SpamChannelEntry, SpamChannelView, BURST_MAX,
    REDEEM_TEXT_MAX_CHARS, REWARD_ID_MAX_CHARS,
};
use crate::store::VaultStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum AccountStatus {
    #[default]
    Starting,
    Mining,
    Idle,
    Paused,
    LoginRequired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(rename = "DropProgressViewType")]
pub struct DropProgressView {
    pub campaign_id: String,
    pub campaign_name: String,
    pub game: String,
    pub drop_id: String,
    pub drop_name: String,
    pub current_minutes: u32,
    pub required_minutes: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(rename = "SlotViewType")]
pub struct SlotView {
    pub channel: Channel,
    pub purpose: SlotPurpose,
    pub since: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub last_beacon_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub segments_requested: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub drop: Option<DropProgressView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub points: Option<PointsState>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "StreamerViewType")]
pub struct StreamerView {
    pub login: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub channel: Option<Channel>,
    pub points: PointsState,
    #[serde(default)]
    pub watching: bool,
    #[serde(default)]
    pub streak_at_risk: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "AccountStatsType")]
pub struct AccountStats {
    pub drops_claimed: u64,
    pub points_earned: i64,
    pub minutes_watched: u64,
    pub bonuses_claimed: u64,
    pub predictions_won: u64,
    pub predictions_lost: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(rename = "AccountSnapshotType")]
pub struct AccountSnapshot {
    pub id: String,
    pub login: String,
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub avatar_url: Option<String>,
    pub settings: AccountSettings,
    pub status: AccountStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub problem: Option<ErrorCode>,
    pub slots: Vec<SlotView>,
    pub queue: Vec<QueueItem>,
    pub streamers: Vec<StreamerView>,
    pub campaigns: Vec<Campaign>,
    pub stats: AccountStats,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub spam_channels: Vec<SpamChannelEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "CatalogViewType")]
pub struct CatalogView {
    pub campaigns: Vec<Campaign>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub fetched_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub problem: Option<ErrorCode>,
    pub interval_seconds: u64,
    pub next_refresh_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(rename = "SnapshotType")]
pub struct Snapshot {
    pub version: String,
    // ! The dashboard's interface build, set by the server: a tab that loaded
    // another one reloads, even when a rebuild kept the version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub build: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub update_available: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub update_checked_at: Option<DateTime<Utc>>,
    pub accounts: Vec<AccountSnapshot>,
    pub catalog: CatalogView,
    pub settings: GlobalSettings,
    pub vault: VaultStatus,
    pub recent_events: Vec<MinerEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub login: Option<LoginFlowView>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "LoginFlowViewType")]
pub struct LoginFlowView {
    pub flow_id: String,
    pub user_code: String,
    pub verification_uri: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub status: LoginFlowStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub problem: Option<ErrorCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub account_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum LoginFlowStatus {
    Pending,
    Completed,
    Expired,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    Desktop,
    Server,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(
    tag = "command",
    rename_all = "SCREAMING_SNAKE_CASE",
    deny_unknown_fields
)]
#[ts(rename = "CommandType")]
// ! Variants without fields are written `{}`: serde ignores unknown keys on
// unit variants of a tagged enum, but rejects them on struct variants.
pub enum Command {
    CancelLogin {},
    RemoveAccount {
        account_id: String,
    },
    SetAccountSettings {
        account_id: String,
        #[serde(deserialize_with = "crate::config::strict_account_settings")]
        settings: AccountSettings,
    },
    ReorderAccounts {
        account_ids: Vec<String>,
    },
    SetGlobalSettings {
        #[serde(deserialize_with = "crate::config::strict_global_settings")]
        settings: GlobalSettings,
    },
    RefreshAccount {
        account_id: String,
    },
    ClaimDrop {
        account_id: String,
        drop_instance_id: String,
    },
    SetPassphrase {
        current: Option<String>,
        next: Option<String>,
    },
    Unlock {
        passphrase: String,
    },
    Lock {},
    OrderCampaign {
        account_id: String,
        campaign_id: String,
        position: Option<u32>,
    },
    AddSpamChannel {
        account_id: String,
        login: String,
    },
    ForgetSpamChannel {
        account_id: String,
        login: String,
    },
    CancelBurst {
        burst_id: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum CommandName {
    CancelLogin,
    RemoveAccount,
    SetAccountSettings,
    ReorderAccounts,
    SetGlobalSettings,
    RefreshAccount,
    ClaimDrop,
    SetPassphrase,
    Unlock,
    Lock,
    OrderCampaign,
    AddSpamChannel,
    ForgetSpamChannel,
    CancelBurst,
}

// ! Written by hand so a passphrase, a webhook or a proxy password never
// reaches a log line through `{:?}`: the log redaction only catches the
// secrets it knows the shape of.
impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CancelLogin {} => f.write_str("CancelLogin"),
            Self::RemoveAccount { account_id } => f
                .debug_struct("RemoveAccount")
                .field("account_id", account_id)
                .finish(),
            Self::SetAccountSettings { account_id, .. } => f
                .debug_struct("SetAccountSettings")
                .field("account_id", account_id)
                .finish_non_exhaustive(),
            Self::ReorderAccounts { account_ids } => f
                .debug_struct("ReorderAccounts")
                .field("account_ids", account_ids)
                .finish(),
            Self::SetGlobalSettings { .. } => {
                f.debug_struct("SetGlobalSettings").finish_non_exhaustive()
            }
            Self::RefreshAccount { account_id } => f
                .debug_struct("RefreshAccount")
                .field("account_id", account_id)
                .finish(),
            Self::ClaimDrop {
                account_id,
                drop_instance_id,
            } => f
                .debug_struct("ClaimDrop")
                .field("account_id", account_id)
                .field("drop_instance_id", drop_instance_id)
                .finish(),
            Self::SetPassphrase { .. } => f.debug_struct("SetPassphrase").finish_non_exhaustive(),
            Self::Unlock { .. } => f.debug_struct("Unlock").finish_non_exhaustive(),
            Self::Lock {} => f.write_str("Lock"),
            Self::OrderCampaign {
                account_id,
                campaign_id,
                position,
            } => f
                .debug_struct("OrderCampaign")
                .field("account_id", account_id)
                .field("campaign_id", campaign_id)
                .field("position", position)
                .finish(),
            Self::AddSpamChannel { account_id, login } => f
                .debug_struct("AddSpamChannel")
                .field("account_id", account_id)
                .field("login", login)
                .finish(),
            Self::ForgetSpamChannel { account_id, login } => f
                .debug_struct("ForgetSpamChannel")
                .field("account_id", account_id)
                .field("login", login)
                .finish(),
            Self::CancelBurst { burst_id } => f
                .debug_struct("CancelBurst")
                .field("burst_id", burst_id)
                .finish(),
        }
    }
}

impl Command {
    pub fn name(&self) -> CommandName {
        match self {
            Self::CancelLogin {} => CommandName::CancelLogin,
            Self::RemoveAccount { .. } => CommandName::RemoveAccount,
            Self::SetAccountSettings { .. } => CommandName::SetAccountSettings,
            Self::ReorderAccounts { .. } => CommandName::ReorderAccounts,
            Self::SetGlobalSettings { .. } => CommandName::SetGlobalSettings,
            Self::RefreshAccount { .. } => CommandName::RefreshAccount,
            Self::ClaimDrop { .. } => CommandName::ClaimDrop,
            Self::SetPassphrase { .. } => CommandName::SetPassphrase,
            Self::Unlock { .. } => CommandName::Unlock,
            Self::Lock {} => CommandName::Lock,
            Self::OrderCampaign { .. } => CommandName::OrderCampaign,
            Self::AddSpamChannel { .. } => CommandName::AddSpamChannel,
            Self::ForgetSpamChannel { .. } => CommandName::ForgetSpamChannel,
            Self::CancelBurst { .. } => CommandName::CancelBurst,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(rename = "SnapshotResponseType")]
pub struct SnapshotResponse {
    pub snapshot: Snapshot,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(rename = "HistoryResponseType")]
pub struct HistoryResponse {
    pub events: Vec<MinerEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub next_before_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "LogsResponseType")]
pub struct LogsResponse {
    pub lines: Vec<LogLine>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub next_before_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "RequestsResponseType")]
pub struct RequestsResponse {
    pub requests: Vec<RequestEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub next_live_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub next_stored_id: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "CatalogRefreshType")]
pub struct CatalogRefresh {
    pub campaign_count: u32,
    pub next_refresh_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "CatalogRefreshResponseType")]
pub struct CatalogRefreshResponse {
    pub refresh: CatalogRefresh,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "UpdateCheckType")]
pub struct UpdateCheck {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub available: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "UpdateCheckResponseType")]
pub struct UpdateCheckResponse {
    pub check: UpdateCheck,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "LogsReportResponseType")]
pub struct LogsReportResponse {
    pub report: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "LoginFlowResponseType")]
pub struct LoginFlowResponse {
    pub login: LoginFlowView,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "ChannelResponseType")]
pub struct ChannelResponse {
    pub channel: Channel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "SearchResponseType")]
pub struct SearchResponse {
    pub games: Vec<crate::domain::SearchGame>,
    pub channels: Vec<crate::domain::SearchChannel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "ChannelBannersRequestType")]
pub struct ChannelBannersRequest {
    pub logins: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "ChannelBannersResponseType")]
pub struct ChannelBannersResponse {
    pub banners: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "ChannelsLookupRequestType")]
pub struct ChannelsLookupRequest {
    pub logins: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "ChannelsLookupResponseType")]
pub struct ChannelsLookupResponse {
    pub channels: Vec<Channel>,
    pub missing: Vec<String>,
    pub unchecked: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "FollowsQueryType")]
pub struct FollowsQuery {
    pub account_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "FollowsResponseType")]
pub struct FollowsResponse {
    pub channels: Vec<Channel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum NotificationTarget {
    Discord,
    Telegram,
    Webhook,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "NotificationTestType")]
pub struct NotificationTest {
    pub target: NotificationTarget,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub problem: Option<ErrorCode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "NotificationTestResponseType")]
pub struct NotificationTestResponse {
    pub targets: Vec<NotificationTest>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "ProxyTestResponseType")]
pub struct ProxyTestResponse {
    pub latency_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "AuthStatusType")]
pub struct AuthStatus {
    pub authorized: bool,
    pub public: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "AuthResponseType")]
pub struct AuthResponse {
    pub auth: AuthStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "AppInfoType")]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub platform: String,
    pub data_dir: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "AppInfoResponseType")]
pub struct AppInfoResponse {
    pub app: AppInfo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "AutostartResponseType")]
pub struct AutostartResponse {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "StartupProblemType")]
pub struct StartupProblem {
    pub detail: String,
    pub data_dir: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "StartupProblemResponseType")]
pub struct StartupProblemResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub problem: Option<StartupProblem>,
}

fn listed<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct Listed<T>(PhantomData<T>);

    impl<'de, T: Deserialize<'de>> Visitor<'de> for Listed<T> {
        type Value = Option<Vec<T>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a list, or its items joined by commas")
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
            value
                .split(',')
                .filter(|item| !item.is_empty())
                .map(|item| T::deserialize(IntoDeserializer::<E>::into_deserializer(item)))
                .collect::<Result<_, _>>()
                .map(Some)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut items: A) -> Result<Self::Value, A::Error> {
            let mut list = Vec::new();

            while let Some(item) = items.next_element()? {
                list.push(item);
            }

            Ok(Some(list))
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    deserializer.deserialize_any(Listed(PhantomData))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "HistoryQueryType")]
pub struct HistoryQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub account_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub category: Option<EventCategory>,
    #[serde(
        default,
        deserialize_with = "listed",
        skip_serializing_if = "Option::is_none"
    )]
    #[ts(optional)]
    pub kinds: Option<Vec<EventKind>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub before_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "LogsQueryType")]
pub struct LogsQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub account_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "listed",
        skip_serializing_if = "Option::is_none"
    )]
    #[ts(optional)]
    pub levels: Option<Vec<LogLevel>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub before_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub limit: Option<u32>,
}

// ! The live requests come first, then the stored ones: a page carries on
// with the cursor of the part it ended in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "RequestsQueryType")]
pub struct RequestsQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub account_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "listed",
        skip_serializing_if = "Option::is_none"
    )]
    #[ts(optional)]
    pub kinds: Option<Vec<RequestKind>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub errors_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub before_live_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub before_stored_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "ChannelQueryType")]
pub struct ChannelQuery {
    pub login: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "SearchQueryType")]
pub struct SearchQuery {
    pub query: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "SpamChannelQueryType")]
pub struct SpamChannelQuery {
    pub account_id: String,
    pub login: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "SpamChannelResponseType")]
pub struct SpamChannelResponse {
    pub channel: SpamChannelView,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "SpamBalanceResponseType")]
pub struct SpamBalanceResponse {
    pub balance: SpamBalance,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "RedeemRequestType")]
pub struct RedeemRequest {
    pub account_id: String,
    pub login: String,
    pub burst_id: String,
    pub reward_id: String,
    pub count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub text: Option<String>,
}

// ! The text is the viewer's own message to the streamer: it stays out of the logs.
impl std::fmt::Debug for RedeemRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedeemRequest")
            .field("account_id", &self.account_id)
            .field("login", &self.login)
            .field("burst_id", &self.burst_id)
            .field("reward_id", &self.reward_id)
            .field("count", &self.count)
            .finish_non_exhaustive()
    }
}

impl RedeemRequest {
    pub fn message(&self) -> Option<&str> {
        self.text
            .as_deref()
            .map(str::trim)
            .filter(|text| !text.is_empty())
    }

    pub fn validate(&self) -> crate::error::Result<()> {
        if !(1..=BURST_MAX).contains(&self.count) {
            return Err(Error::InvalidRequest(format!(
                "{} redemptions, 1 to {BURST_MAX}",
                self.count
            )));
        }

        if !valid_burst_id(&self.burst_id) {
            return Err(Error::InvalidRequest("malformed burst id".to_owned()));
        }

        if self.reward_id.is_empty() || self.reward_id.chars().count() > REWARD_ID_MAX_CHARS {
            return Err(Error::InvalidRequest("malformed reward id".to_owned()));
        }

        if self
            .message()
            .is_some_and(|text| text.chars().count() > REDEEM_TEXT_MAX_CHARS)
        {
            return Err(Error::InvalidRequest("message too long".to_owned()));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "RedeemResponseType")]
pub struct RedeemResponse {
    pub burst_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "ProxyTestRequestType")]
pub struct ProxyTestRequest {
    pub url: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "DashboardLoginRequestType")]
pub struct DashboardLoginRequest {
    pub password: String,
}

impl std::fmt::Debug for DashboardLoginRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DashboardLoginRequest { .. }")
    }
}

pub const SNAPSHOT_PUSH_DEBOUNCE_MS: u64 = 700;

pub const CHANNELS_LOOKUP_MAX: usize = 500;

pub const FOLLOWS_MAX: usize = 1_000;

pub const HISTORY_LIMIT_MAX: u32 = 500;
pub const HISTORY_LIMIT_DEFAULT: u32 = 100;

pub const LOGS_LIMIT_DEFAULT: u32 = 100;
pub const LOGS_LIMIT_MAX: u32 = 100;

pub const REQUESTS_LIMIT_DEFAULT: u32 = 100;
pub const REQUESTS_LIMIT_MAX: u32 = 500;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(rename = "ServerMessageType")]
pub enum ServerMessage {
    Snapshot { snapshot: Box<Snapshot> },
    Event { event: Box<MinerEvent> },
    Burst { progress: BurstProgress },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum ServerMessageKind {
    Snapshot,
    Event,
    Burst,
}

impl ServerMessage {
    pub fn kind(&self) -> ServerMessageKind {
        match self {
            Self::Snapshot { .. } => ServerMessageKind::Snapshot,
            Self::Event { .. } => ServerMessageKind::Event,
            Self::Burst { .. } => ServerMessageKind::Burst,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum DesktopEvent {
    Snapshot,
    MinerEvent,
    CloseRequested,
    BurstProgress,
}

impl DesktopEvent {
    pub fn name(self) -> &'static str {
        match self {
            Self::Snapshot => "SNAPSHOT",
            Self::MinerEvent => "MINER_EVENT",
            Self::CloseRequested => "CLOSE_REQUESTED",
            Self::BurstProgress => "BURST_PROGRESS",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "RoutesApi", repr(enum = name))]
pub enum ApiRoute {
    #[serde(rename = "/api/health")]
    Health,
    #[serde(rename = "/api/auth")]
    Auth,
    #[serde(rename = "/api/activity")]
    Activity,
    #[serde(rename = "/api/snapshot")]
    Snapshot,
    #[serde(rename = "/api/command")]
    Command,
    #[serde(rename = "/api/catalog/refresh")]
    CatalogRefresh,
    #[serde(rename = "/api/updates/check")]
    UpdatesCheck,
    #[serde(rename = "/api/login-flow")]
    LoginFlow,
    #[serde(rename = "/api/channel")]
    Channel,
    #[serde(rename = "/api/channels/lookup")]
    ChannelsLookup,
    #[serde(rename = "/api/search")]
    Search,
    #[serde(rename = "/api/channels/banners")]
    ChannelBanners,
    #[serde(rename = "/api/follows")]
    Follows,
    #[serde(rename = "/api/notifications/test")]
    NotificationsTest,
    #[serde(rename = "/api/proxy/test")]
    ProxyTest,
    #[serde(rename = "/api/history")]
    History,
    #[serde(rename = "/api/logs")]
    Logs,
    #[serde(rename = "/api/logs/report")]
    LogsReport,
    #[serde(rename = "/api/requests")]
    Requests,
    #[serde(rename = "/api/requests/report")]
    RequestsReport,
    #[serde(rename = "/api/events")]
    Events,
    #[serde(rename = "/api/spam/channel")]
    SpamChannel,
    #[serde(rename = "/api/spam/balance")]
    SpamBalance,
    #[serde(rename = "/api/spam/redeem")]
    SpamRedeem,
}

impl ApiRoute {
    pub fn path(self) -> &'static str {
        match self {
            Self::Health => "/api/health",
            Self::Auth => "/api/auth",
            Self::Activity => "/api/activity",
            Self::Snapshot => "/api/snapshot",
            Self::Command => "/api/command",
            Self::CatalogRefresh => "/api/catalog/refresh",
            Self::UpdatesCheck => "/api/updates/check",
            Self::LoginFlow => "/api/login-flow",
            Self::Channel => "/api/channel",
            Self::ChannelsLookup => "/api/channels/lookup",
            Self::Search => "/api/search",
            Self::ChannelBanners => "/api/channels/banners",
            Self::Follows => "/api/follows",
            Self::NotificationsTest => "/api/notifications/test",
            Self::ProxyTest => "/api/proxy/test",
            Self::History => "/api/history",
            Self::Logs => "/api/logs",
            Self::LogsReport => "/api/logs/report",
            Self::Requests => "/api/requests",
            Self::RequestsReport => "/api/requests/report",
            Self::Events => "/api/events",
            Self::SpamChannel => "/api/spam/channel",
            Self::SpamBalance => "/api/spam/balance",
            Self::SpamRedeem => "/api/spam/redeem",
        }
    }
}
