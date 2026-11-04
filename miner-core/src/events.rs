use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::error::ErrorCode;
use crate::scheduler::PointsReason;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, Hash, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum EventKind {
    #[default]
    Info,
    Error,
    WatchStarted,
    WatchStopped,
    DropProgress,
    DropClaimed,
    CampaignFinished,
    PointsEarned,
    PointsSpent,
    BonusClaimed,
    PredictionPlaced,
    PredictionResult,
    StreamOnline,
    StreamOffline,
    RaidJoined,
    MomentClaimed,
    StreakRecovered,
    GoalContributed,
    ChatMention,
    SessionExpired,
    LoginCompleted,
    UpdateAvailable,
    RewardRedeemed,
}

impl EventKind {
    pub const ALL: [EventKind; 23] = [
        Self::Info,
        Self::Error,
        Self::WatchStarted,
        Self::WatchStopped,
        Self::DropProgress,
        Self::DropClaimed,
        Self::CampaignFinished,
        Self::PointsEarned,
        Self::PointsSpent,
        Self::BonusClaimed,
        Self::PredictionPlaced,
        Self::PredictionResult,
        Self::StreamOnline,
        Self::StreamOffline,
        Self::RaidJoined,
        Self::MomentClaimed,
        Self::StreakRecovered,
        Self::GoalContributed,
        Self::ChatMention,
        Self::SessionExpired,
        Self::LoginCompleted,
        Self::UpdateAvailable,
        Self::RewardRedeemed,
    ];

    pub fn miner(self) -> Option<MinerKind> {
        match self {
            Self::DropProgress | Self::DropClaimed | Self::CampaignFinished => {
                Some(MinerKind::Drops)
            }
            Self::PointsEarned
            | Self::PointsSpent
            | Self::BonusClaimed
            | Self::PredictionPlaced
            | Self::PredictionResult
            | Self::StreamOnline
            | Self::StreamOffline
            | Self::RaidJoined
            | Self::MomentClaimed
            | Self::StreakRecovered
            | Self::GoalContributed
            | Self::ChatMention => Some(MinerKind::Points),
            Self::Info
            | Self::Error
            | Self::WatchStarted
            | Self::WatchStopped
            | Self::SessionExpired
            | Self::LoginCompleted
            | Self::UpdateAvailable
            | Self::RewardRedeemed => None,
        }
    }

    pub fn follows_miner(self) -> bool {
        matches!(self, Self::WatchStarted | Self::WatchStopped)
    }

    // ! A Spam Points burst is the user's own spend: no miner acts, but it is
    // a channel points event.
    pub fn category(self, miner: Option<MinerKind>) -> EventCategory {
        if self == Self::RewardRedeemed {
            return EventCategory::Points;
        }

        let miner = if self.follows_miner() {
            miner
        } else {
            self.miner()
        };

        match miner {
            Some(MinerKind::Drops) => EventCategory::Drops,
            Some(MinerKind::Points) => EventCategory::Points,
            None => EventCategory::Other,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "INFO",
            Self::Error => "ERROR",
            Self::WatchStarted => "WATCH_STARTED",
            Self::WatchStopped => "WATCH_STOPPED",
            Self::DropProgress => "DROP_PROGRESS",
            Self::DropClaimed => "DROP_CLAIMED",
            Self::CampaignFinished => "CAMPAIGN_FINISHED",
            Self::PointsEarned => "POINTS_EARNED",
            Self::PointsSpent => "POINTS_SPENT",
            Self::BonusClaimed => "BONUS_CLAIMED",
            Self::PredictionPlaced => "PREDICTION_PLACED",
            Self::PredictionResult => "PREDICTION_RESULT",
            Self::StreamOnline => "STREAM_ONLINE",
            Self::StreamOffline => "STREAM_OFFLINE",
            Self::RaidJoined => "RAID_JOINED",
            Self::MomentClaimed => "MOMENT_CLAIMED",
            Self::StreakRecovered => "STREAK_RECOVERED",
            Self::GoalContributed => "GOAL_CONTRIBUTED",
            Self::ChatMention => "CHAT_MENTION",
            Self::SessionExpired => "SESSION_EXPIRED",
            Self::LoginCompleted => "LOGIN_COMPLETED",
            Self::UpdateAvailable => "UPDATE_AVAILABLE",
            Self::RewardRedeemed => "REWARD_REDEEMED",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.as_str() == name)
    }

    pub fn in_recent(self) -> bool {
        !matches!(
            self,
            Self::DropProgress | Self::PointsEarned | Self::PointsSpent
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum MinerKind {
    Drops,
    Points,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, Hash, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum EventCategory {
    Drops,
    Points,
    #[default]
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(repr(enum = name))]
pub enum BetResult {
    Win,
    Lose,
    Refund,
}

impl BetResult {
    pub fn from_twitch(result: &str) -> Self {
        match result {
            "WIN" => Self::Win,
            "LOSE" => Self::Lose,
            _ => Self::Refund,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(default)]
#[ts(rename = "EventContextType", optional_fields)]
pub struct EventContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miner: Option<MinerKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub art: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_reason: Option<PointsReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earned: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spent: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chatter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bet_result: Option<BetResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points_won: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staked: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusals: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<ErrorCode>>,
}

impl EventContext {
    pub fn channel(login: impl Into<String>) -> Self {
        Self {
            channel: Some(login.into()),
            ..Self::default()
        }
    }

    pub fn campaign(campaign_id: impl Into<String>) -> Self {
        Self {
            campaign_id: Some(campaign_id.into()),
            ..Self::default()
        }
    }

    pub fn drop(campaign_id: impl Into<String>, drop_id: impl Into<String>) -> Self {
        Self {
            campaign_id: Some(campaign_id.into()),
            drop_id: Some(drop_id.into()),
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "MinerEventType")]
pub struct MinerEvent {
    #[serde(default)]
    pub id: u64,
    pub at: DateTime<Utc>,
    pub account_id: String,
    #[serde(default)]
    pub account_login: String,
    pub kind: EventKind,
    #[serde(default)]
    pub category: EventCategory,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub detail: Option<String>,
    #[serde(default)]
    pub context: EventContext,
}

impl MinerEvent {
    pub fn new(
        account_id: &str,
        account_login: &str,
        kind: EventKind,
        title: impl Into<String>,
    ) -> Self {
        Self {
            id: 0,
            at: Utc::now(),
            account_id: account_id.to_owned(),
            account_login: account_login.to_owned(),
            kind,
            category: kind.category(None),
            title: title.into(),
            detail: None,
            context: EventContext::default(),
        }
    }

    pub fn restore_miner(&mut self) {
        if !self.kind.follows_miner() && self.context.miner.is_none() {
            self.context.miner = self.kind.miner();
        }
    }

    pub fn categorize(&mut self) {
        self.category = self.kind.category(self.context.miner);
    }

    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn context(mut self, context: EventContext) -> Self {
        self.context = context;
        self
    }

    pub fn stored(&self) -> MinerEvent {
        let mut event = self.clone();
        let context = &mut event.context;

        context.account_name = None;
        context.account_avatar = None;
        context.channel_name = None;
        context.avatar = None;

        if !event.kind.follows_miner() && context.miner == event.kind.miner() {
            context.miner = None;
        }

        if event.kind == EventKind::DropProgress {
            event.title = String::new();
            context.campaign_name = None;
            context.game = None;
            context.game_slug = None;
            context.art = None;
            context.drop_name = None;
            context.reward = None;
        }

        event
    }
}
