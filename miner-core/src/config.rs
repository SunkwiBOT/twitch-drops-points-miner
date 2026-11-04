use std::ops::RangeInclusive;

use serde::{Deserialize, Deserializer, Serialize};
use ts_rs::TS;

use crate::error::{Error, Result};
use crate::predictions::MAX_PREDICTION_POINTS;

const SLOTS: RangeInclusive<u8> = 1..=2;
const BET_PERCENTAGE: RangeInclusive<u8> = 1..=100;
const BET_TIMING_PERCENT: RangeInclusive<u8> = 1..=99;
const BET_GAP_PERCENT: RangeInclusive<u8> = 0..=100;
const BET_DELAY_SECONDS_MAX: u32 = 300;
const TEXT_MAX_CHARS: usize = 2048;
const TIME_ZONE_MAX_CHARS: usize = 64;
pub const SPAM_DELAY_MAX_MS: u32 = 5_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum Mode {
    #[default]
    Auto,
    Drops,
    Points,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum DropsScope {
    PriorityOnly,
    LinkedOnly,
    #[default]
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum DropsOrder {
    #[default]
    EndingSoonest,
    LowestRemaining,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(rename = "DropsSettingsType")]
pub struct DropsSettings {
    pub scope: DropsScope,
    pub campaign_order: Vec<String>,
    pub priority_games: Vec<String>,
    pub linked_first: bool,
    pub order: DropsOrder,
    pub exclude_games: Vec<String>,
    pub exclude_names: Vec<String>,
    pub exclude_campaign_ids: Vec<String>,
    pub exclude_drop_ids: Vec<String>,
    pub auto_claim: bool,
    pub skip_unfinishable: bool,
    pub mine_points: bool,
    pub points_followed_only: bool,
    pub streamers_first: bool,
}

impl Default for DropsSettings {
    fn default() -> Self {
        Self {
            scope: DropsScope::default(),
            campaign_order: Vec::new(),
            priority_games: Vec::new(),
            linked_first: true,
            order: DropsOrder::default(),
            exclude_games: Vec::new(),
            exclude_names: Vec::new(),
            exclude_campaign_ids: Vec::new(),
            exclude_drop_ids: Vec::new(),
            auto_claim: false,
            skip_unfinishable: true,
            mine_points: true,
            points_followed_only: false,
            streamers_first: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum Strategy {
    #[default]
    Smart,
    MostVoted,
    HighOdds,
    Percentage,
    SmartMoney,
    Number1,
    Number2,
    Number3,
    Number4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum FilterBy {
    TotalUsers,
    TotalPoints,
    Odds,
    OddsPercentage,
    PercentageUsers,
    DecisionUsers,
    DecisionPoints,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum FilterOp {
    Gt,
    Lt,
    Gte,
    Lte,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "BetFilterType")]
pub struct BetFilter {
    pub by: FilterBy,
    pub op: FilterOp,
    pub value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum BetTiming {
    AtOpen,
    #[default]
    BeforeClose,
    Percent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(rename = "PredictionSettingsType")]
pub struct PredictionSettings {
    pub enabled: bool,
    pub strategy: Strategy,
    pub percentage: u8,
    pub max_points: u32,
    pub min_points: u32,
    pub timing: BetTiming,
    pub delay_seconds: u32,
    pub timing_percent: u8,
    pub stealth: bool,
    pub percentage_gap: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub filter: Option<BetFilter>,
}

impl Default for PredictionSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: Strategy::Smart,
            percentage: 5,
            max_points: 50_000,
            min_points: 10,
            timing: BetTiming::BeforeClose,
            delay_seconds: 6,
            timing_percent: 80,
            stealth: false,
            percentage_gap: 20,
            filter: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum ChatPresence {
    Never,
    #[default]
    Online,
    Offline,
    Always,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(rename = "StreamerSettingsType")]
pub struct StreamerSettings {
    pub login: String,
    pub enabled: bool,
    pub claim_bonus: bool,
    pub follow_raids: bool,
    pub claim_moments: bool,
    pub watch_streak: bool,
    pub community_goals: bool,
    pub chat: ChatPresence,
    pub predictions: PredictionSettings,
}

impl Default for StreamerSettings {
    fn default() -> Self {
        Self {
            login: String::new(),
            enabled: true,
            claim_bonus: true,
            follow_raids: true,
            claim_moments: true,
            watch_streak: true,
            community_goals: false,
            chat: ChatPresence::default(),
            predictions: PredictionSettings::default(),
        }
    }
}

impl StreamerSettings {
    pub fn new(login: impl Into<String>) -> Self {
        Self {
            login: login.into(),
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(rename = "PointsSettingsType")]
pub struct PointsSettings {
    pub streamers: Vec<StreamerSettings>,
    pub streak_priority: bool,
    pub subscriber_priority: bool,
    pub streamer_defaults: StreamerSettings,
}

impl Default for PointsSettings {
    fn default() -> Self {
        Self {
            streamers: Vec::new(),
            streak_priority: true,
            subscriber_priority: true,
            streamer_defaults: StreamerSettings::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(rename = "AccountSettingsType")]
pub struct AccountSettings {
    pub enabled: bool,
    pub mode: Mode,
    pub drops: DropsSettings,
    pub points: PointsSettings,
    // ! Twitch credits at most two channels per account at a time.
    pub slots: u8,
    pub proxy: String,
}

impl Default for AccountSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: Mode::Auto,
            drops: DropsSettings::default(),
            points: PointsSettings::default(),
            slots: 2,
            proxy: String::new(),
        }
    }
}

impl AccountSettings {
    pub fn slots(&self) -> usize {
        self.slots.clamp(*SLOTS.start(), *SLOTS.end()) as usize
    }

    pub fn validate(&self) -> Result<()> {
        within("slots", self.slots, &SLOTS)?;
        short("proxy", &self.proxy)?;

        let drops = &self.drops;

        for (name, list) in [
            ("campaign_order", &drops.campaign_order),
            ("priority_games", &drops.priority_games),
            ("exclude_games", &drops.exclude_games),
            ("exclude_names", &drops.exclude_names),
            ("exclude_campaign_ids", &drops.exclude_campaign_ids),
            ("exclude_drop_ids", &drops.exclude_drop_ids),
        ] {
            list.iter().try_for_each(|entry| short(name, entry))?;
        }

        self.points
            .streamers
            .iter()
            .chain(std::iter::once(&self.points.streamer_defaults))
            .try_for_each(|streamer| {
                short("login", &streamer.login)?;
                streamer.predictions.validate()
            })
    }
}

impl PredictionSettings {
    fn validate(&self) -> Result<()> {
        within("percentage", self.percentage, &BET_PERCENTAGE)?;
        within("timing_percent", self.timing_percent, &BET_TIMING_PERCENT)?;
        within("percentage_gap", self.percentage_gap, &BET_GAP_PERCENT)?;
        within(
            "delay_seconds",
            self.delay_seconds,
            &(0..=BET_DELAY_SECONDS_MAX),
        )?;
        within("min_points", self.min_points, &(1..=u32::MAX))?;
        within(
            "max_points",
            i64::from(self.max_points),
            &(1..=MAX_PREDICTION_POINTS),
        )?;

        match self.filter {
            Some(filter) if !filter.value.is_finite() => invalid("filter.value"),
            _ => Ok(()),
        }
    }
}

impl GlobalSettings {
    pub fn validate(&self) -> Result<()> {
        if self.language != crate::i18n::AUTO && !crate::i18n::is_known(&self.language) {
            return invalid("language");
        }

        let time_zone = self.time_zone.chars().count() <= TIME_ZONE_MAX_CHARS
            && self
                .time_zone
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "/_+-".contains(c));

        if self.time_zone.is_empty() || !time_zone {
            return invalid("time_zone");
        }

        within(
            "spam_points.delay_ms",
            self.spam_points.delay_ms,
            &(0..=SPAM_DELAY_MAX_MS),
        )?;

        let notifications = &self.notifications;

        [
            ("default_proxy", &self.default_proxy),
            ("discord.webhook_url", &notifications.discord.webhook_url),
            ("telegram.bot_token", &notifications.telegram.bot_token),
            ("telegram.chat_id", &notifications.telegram.chat_id),
            ("webhook.url", &notifications.webhook.url),
            (
                "webhook.authorization",
                &notifications.webhook.authorization,
            ),
        ]
        .into_iter()
        .try_for_each(|(name, text)| short(name, text))
    }
}

fn invalid(field: &str) -> Result<()> {
    Err(Error::InvalidRequest(format!("{field} is out of bounds")))
}

fn within<T: PartialOrd>(field: &str, value: T, bounds: &RangeInclusive<T>) -> Result<()> {
    if bounds.contains(&value) {
        return Ok(());
    }

    invalid(field)
}

fn short(field: &str, text: &str) -> Result<()> {
    if text.chars().count() > TEXT_MAX_CHARS {
        return invalid(field);
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(default)]
#[ts(rename = "DiscordSettingsType")]
pub struct DiscordSettings {
    pub enabled: bool,
    pub webhook_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(default)]
#[ts(rename = "TelegramSettingsType")]
pub struct TelegramSettings {
    pub enabled: bool,
    pub bot_token: String,
    pub chat_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(default)]
#[ts(rename = "WebhookSettingsType")]
pub struct WebhookSettings {
    pub enabled: bool,
    pub url: String,
    pub authorization: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(rename = "NotificationEventsType")]
pub struct NotificationEvents {
    pub drop_claimed: bool,
    pub drop_progress: bool,
    pub bonus_claimed: bool,
    pub prediction_placed: bool,
    pub prediction_result: bool,
    pub stream_online: bool,
    pub stream_offline: bool,
    pub raid: bool,
    pub moment: bool,
    pub chat_mention: bool,
    pub session_expired: bool,
    pub errors: bool,
    pub update_available: bool,
}

impl Default for NotificationEvents {
    fn default() -> Self {
        Self {
            drop_claimed: true,
            drop_progress: false,
            bonus_claimed: false,
            prediction_placed: true,
            prediction_result: true,
            stream_online: false,
            stream_offline: false,
            raid: false,
            moment: true,
            chat_mention: true,
            session_expired: true,
            errors: true,
            update_available: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(default)]
#[ts(rename = "NotificationSettingsType")]
pub struct NotificationSettings {
    pub discord: DiscordSettings,
    pub telegram: TelegramSettings,
    pub webhook: WebhookSettings,
    pub system: bool,
    pub events: NotificationEvents,
}

pub const AUTO_TIME_ZONE: &str = "AUTO";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(rename = "GlobalSettingsType")]
pub struct GlobalSettings {
    pub language: String,
    pub default_proxy: String,
    pub notifications: NotificationSettings,
    pub close_action: CloseAction,
    pub start_minimized: bool,
    pub update_interval: UpdateInterval,
    pub auto_lock: AutoLock,
    pub theme: Theme,
    pub time_zone: String,
    pub date_format: DateFormat,
    pub clock_format: ClockFormat,
    pub history_retention: HistoryRetention,
    pub spam_points: SpamPointsSettings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(rename = "SpamPointsSettingsType")]
pub struct SpamPointsSettings {
    pub show_disabled: bool,
    pub show_out_of_stock: bool,
    pub delay_ms: u32,
}

impl Default for SpamPointsSettings {
    fn default() -> Self {
        Self {
            show_disabled: true,
            show_out_of_stock: true,
            delay_ms: 50,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum DateFormat {
    #[default]
    Auto,
    DayMonthYear,
    MonthDayYear,
    YearMonthDay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum ClockFormat {
    #[default]
    Auto,
    H24,
    H12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum HistoryRetention {
    Days30,
    #[default]
    Days90,
    Year1,
    Unlimited,
}

impl HistoryRetention {
    pub fn max_age(self) -> Option<chrono::Duration> {
        match self {
            Self::Days30 => Some(chrono::Duration::days(30)),
            Self::Days90 => Some(chrono::Duration::days(90)),
            Self::Year1 => Some(chrono::Duration::days(365)),
            Self::Unlimited => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum UpdateInterval {
    AtStart,
    Minutes30,
    #[default]
    Hours1,
    Hours3,
    Hours6,
    Hours12,
    Hours24,
}

impl UpdateInterval {
    pub fn period(self) -> Option<std::time::Duration> {
        let minutes = match self {
            Self::AtStart => return None,
            Self::Minutes30 => 30,
            Self::Hours1 => 60,
            Self::Hours3 => 3 * 60,
            Self::Hours6 => 6 * 60,
            Self::Hours12 => 12 * 60,
            Self::Hours24 => 24 * 60,
        };

        Some(std::time::Duration::from_secs(minutes * 60))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum AutoLock {
    Never,
    Minutes1,
    #[default]
    Minutes5,
    Minutes10,
    Minutes15,
    Minutes30,
}

impl AutoLock {
    pub fn idle_limit(self) -> Option<std::time::Duration> {
        let minutes = match self {
            Self::Never => return None,
            Self::Minutes1 => 1,
            Self::Minutes5 => 5,
            Self::Minutes10 => 10,
            Self::Minutes15 => 15,
            Self::Minutes30 => 30,
        };

        Some(std::time::Duration::from_secs(minutes * 60))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum CloseAction {
    #[default]
    Ask,
    Quit,
    Tray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            language: crate::i18n::AUTO.into(),
            default_proxy: String::new(),
            notifications: NotificationSettings::default(),
            close_action: CloseAction::Ask,
            start_minimized: false,
            update_interval: UpdateInterval::Hours1,
            auto_lock: AutoLock::Minutes5,
            theme: Theme::Dark,
            time_zone: AUTO_TIME_ZONE.into(),
            date_format: DateFormat::Auto,
            clock_format: ClockFormat::Auto,
            history_retention: HistoryRetention::Days30,
            spam_points: SpamPointsSettings::default(),
        }
    }
}

// ! The stored settings stay lenient (`serde(default)`): a build reads what
// another one wrote. The API takes the exact shape, so a misspelt or missing key
// is refused instead of silently resetting a setting.
macro_rules! strict_input {
    ($input:ident => $stored:ident { $($field:ident: $ty:ty),+ $(,)? }) => {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct $input {
            $($field: $ty),+
        }

        impl From<$input> for $stored {
            fn from(input: $input) -> Self {
                Self {
                    $($field: input.$field.into()),+
                }
            }
        }
    };
}

strict_input!(DropsSettingsInput => DropsSettings {
    scope: DropsScope,
    campaign_order: Vec<String>,
    priority_games: Vec<String>,
    linked_first: bool,
    order: DropsOrder,
    exclude_games: Vec<String>,
    exclude_names: Vec<String>,
    exclude_campaign_ids: Vec<String>,
    exclude_drop_ids: Vec<String>,
    auto_claim: bool,
    skip_unfinishable: bool,
    mine_points: bool,
    points_followed_only: bool,
    streamers_first: bool,
});

strict_input!(PredictionSettingsInput => PredictionSettings {
    enabled: bool,
    strategy: Strategy,
    percentage: u8,
    max_points: u32,
    min_points: u32,
    timing: BetTiming,
    delay_seconds: u32,
    timing_percent: u8,
    stealth: bool,
    percentage_gap: u8,
    filter: Option<BetFilter>,
});

strict_input!(StreamerSettingsInput => StreamerSettings {
    login: String,
    enabled: bool,
    claim_bonus: bool,
    follow_raids: bool,
    claim_moments: bool,
    watch_streak: bool,
    community_goals: bool,
    chat: ChatPresence,
    predictions: PredictionSettingsInput,
});

#[derive(Deserialize)]
#[serde(transparent)]
struct StreamersInput(Vec<StreamerSettingsInput>);

impl From<StreamersInput> for Vec<StreamerSettings> {
    fn from(input: StreamersInput) -> Self {
        input.0.into_iter().map(Into::into).collect()
    }
}

strict_input!(PointsSettingsInput => PointsSettings {
    streamers: StreamersInput,
    streak_priority: bool,
    subscriber_priority: bool,
    streamer_defaults: StreamerSettingsInput,
});

strict_input!(AccountSettingsInput => AccountSettings {
    enabled: bool,
    mode: Mode,
    drops: DropsSettingsInput,
    points: PointsSettingsInput,
    slots: u8,
    proxy: String,
});

strict_input!(DiscordSettingsInput => DiscordSettings {
    enabled: bool,
    webhook_url: String,
});

strict_input!(TelegramSettingsInput => TelegramSettings {
    enabled: bool,
    bot_token: String,
    chat_id: String,
});

strict_input!(WebhookSettingsInput => WebhookSettings {
    enabled: bool,
    url: String,
    authorization: String,
});

strict_input!(NotificationEventsInput => NotificationEvents {
    drop_claimed: bool,
    drop_progress: bool,
    bonus_claimed: bool,
    prediction_placed: bool,
    prediction_result: bool,
    stream_online: bool,
    stream_offline: bool,
    raid: bool,
    moment: bool,
    chat_mention: bool,
    session_expired: bool,
    errors: bool,
    update_available: bool,
});

strict_input!(NotificationSettingsInput => NotificationSettings {
    discord: DiscordSettingsInput,
    telegram: TelegramSettingsInput,
    webhook: WebhookSettingsInput,
    system: bool,
    events: NotificationEventsInput,
});

strict_input!(SpamPointsSettingsInput => SpamPointsSettings {
    show_disabled: bool,
    show_out_of_stock: bool,
    delay_ms: u32,
});

strict_input!(GlobalSettingsInput => GlobalSettings {
    language: String,
    default_proxy: String,
    notifications: NotificationSettingsInput,
    close_action: CloseAction,
    start_minimized: bool,
    update_interval: UpdateInterval,
    auto_lock: AutoLock,
    theme: Theme,
    time_zone: String,
    date_format: DateFormat,
    clock_format: ClockFormat,
    history_retention: HistoryRetention,
    spam_points: SpamPointsSettingsInput,
});

pub fn strict_account_settings<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<AccountSettings, D::Error> {
    AccountSettingsInput::deserialize(deserializer).map(Into::into)
}

pub fn strict_global_settings<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<GlobalSettings, D::Error> {
    GlobalSettingsInput::deserialize(deserializer).map(Into::into)
}
