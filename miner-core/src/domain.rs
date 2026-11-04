use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "GameType")]
pub struct Game {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub box_art_url: Option<String>,
}

impl Game {
    pub fn slug(&self) -> String {
        self.slug
            .clone()
            .unwrap_or_else(|| crate::util::game_slug(&self.name))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "ChannelRefType")]
pub struct ChannelRef {
    pub id: String,
    pub login: String,
    #[serde(default)]
    pub display_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum BenefitKind {
    Badge,
    InGameItem,
    #[default]
    Other,
}

impl BenefitKind {
    pub fn from_twitch(distribution_type: Option<&str>) -> Self {
        match distribution_type {
            Some("BADGE") => Self::Badge,
            Some("DIRECT_ENTITLEMENT") => Self::InGameItem,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "BenefitType")]
pub struct Benefit {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub kind: BenefitKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "TimedDropType")]
pub struct TimedDrop {
    pub id: String,
    pub name: String,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub required_minutes: u32,
    #[serde(default)]
    pub required_subs: u32,
    #[serde(default)]
    pub current_minutes: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub instance_id: Option<String>,
    #[serde(default)]
    pub claimed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub preconditions_met: Option<bool>,
    #[serde(default)]
    pub precondition_drop_ids: Vec<String>,
    #[serde(default)]
    pub benefits: Vec<Benefit>,
}

impl TimedDrop {
    pub fn complete(&self) -> bool {
        self.claimed || self.current_minutes >= self.required_minutes
    }

    pub fn claimable(&self) -> bool {
        !self.claimed && self.instance_id.is_some()
    }

    pub fn remaining_minutes(&self) -> u32 {
        self.required_minutes.saturating_sub(self.current_minutes)
    }

    pub fn active_at(&self, now: DateTime<Utc>) -> bool {
        self.starts_at <= now && now < self.ends_at
    }

    pub fn minable(&self, excluded: &[String]) -> bool {
        self.required_subs == 0 && !excluded.iter().any(|id| id == &self.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(repr(enum = name))]
pub enum CampaignStatus {
    #[default]
    Active,
    Upcoming,
    Expired,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "CampaignType")]
pub struct Campaign {
    pub id: String,
    pub name: String,
    pub game: Game,
    pub status: CampaignStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub linked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub link_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub image_url: Option<String>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    #[serde(default)]
    pub allowed_channels: Vec<ChannelRef>,
    #[serde(default)]
    pub drops: Vec<TimedDrop>,
    #[serde(default)]
    pub in_inventory: bool,
}

impl Campaign {
    // ! Twitch answers a claim on a game the account has not linked as if it
    // took it, and gives nothing: the drop waits for the link.
    pub fn claims_delivered(&self) -> bool {
        self.linked != Some(false)
    }

    pub fn active_at(&self, now: DateTime<Utc>) -> bool {
        self.status == CampaignStatus::Active && self.starts_at <= now && now < self.ends_at
    }

    pub fn finished(&self) -> bool {
        !self.drops.is_empty() && self.drops.iter().all(TimedDrop::complete)
    }

    pub fn remaining_minutes(&self, excluded: &[String]) -> u32 {
        self.drops
            .iter()
            .filter(|d| !d.complete() && d.minable(excluded))
            .map(|d| d.remaining_minutes())
            .max()
            .unwrap_or(0)
    }

    pub fn can_finish(&self, now: DateTime<Utc>, excluded: &[String]) -> bool {
        self.drops
            .iter()
            .filter(|d| !d.complete() && d.minable(excluded) && d.ends_at > now)
            .all(|d| {
                let window = d.ends_at.min(self.ends_at) - d.starts_at.max(now);

                i64::from(d.remaining_minutes()) <= window.num_minutes()
            })
    }

    pub fn next_drop(&self, now: DateTime<Utc>, excluded: &[String]) -> Option<&TimedDrop> {
        let completed: Vec<&str> = self
            .drops
            .iter()
            .filter(|d| d.complete())
            .map(|d| d.id.as_str())
            .collect();

        self.drops
            .iter()
            .filter(|d| !d.complete() && d.active_at(now) && d.minable(excluded))
            .filter(|d| {
                d.preconditions_met.unwrap_or(true)
                    && d.precondition_drop_ids
                        .iter()
                        .all(|id| completed.contains(&id.as_str()))
            })
            .min_by_key(|d| (d.required_minutes, d.id.clone()))
    }

    pub fn accepts_channel(&self, channel_id: &str) -> bool {
        self.allowed_channels.is_empty() || self.allowed_channels.iter().any(|c| c.id == channel_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "SearchGameType")]
pub struct SearchGame {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub box_art_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "SearchChannelType")]
pub struct SearchChannel {
    pub login: String,
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub avatar_url: Option<String>,
    pub live: bool,
    pub verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "ChannelType")]
pub struct Channel {
    pub id: String,
    pub login: String,
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub live: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub broadcast_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub viewers: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub game: Option<Game>,
    #[serde(default)]
    pub drops_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "PointsStateType")]
pub struct PointsState {
    pub balance: i64,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub earned_session: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub streak: Option<WatchStreak>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub last_bonus_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub subscribed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "WatchStreakType")]
pub struct WatchStreak {
    pub count: u32,
    #[serde(default)]
    pub missed_streams: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub expires_at: Option<DateTime<Utc>>,
}

const STREAK_RISK_WINDOW: chrono::Duration = chrono::Duration::hours(6);

impl WatchStreak {
    pub fn at_risk(&self, now: DateTime<Utc>) -> bool {
        self.missed_streams > 0
            || self
                .expires_at
                .is_some_and(|expires| expires > now && expires - now < STREAK_RISK_WINDOW)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionOutcome {
    pub id: String,
    pub title: String,
    pub color: String,
    pub total_users: i64,
    pub total_points: i64,
    pub top_points: i64,
    #[serde(default)]
    pub percentage_users: f64,
    #[serde(default)]
    pub odds: f64,
    #[serde(default)]
    pub odds_percentage: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionEvent {
    pub id: String,
    pub channel_id: String,
    pub title: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub window_seconds: i64,
    pub outcomes: Vec<PredictionOutcome>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub winning_outcome_id: Option<String>,
}

impl PredictionEvent {
    pub fn update_outcomes(&mut self) {
        let total_users: i64 = self.outcomes.iter().map(|o| o.total_users).sum();
        let total_points: i64 = self.outcomes.iter().map(|o| o.total_points).sum();

        for outcome in &mut self.outcomes {
            outcome.percentage_users = if total_users > 0 {
                outcome.total_users as f64 * 100.0 / total_users as f64
            } else {
                0.0
            };

            outcome.odds = if outcome.total_points > 0 {
                total_points as f64 / outcome.total_points as f64
            } else {
                0.0
            };

            outcome.odds_percentage = if outcome.odds > 0.0 {
                100.0 / outcome.odds
            } else {
                0.0
            };
        }
    }

    pub fn closes_at(&self) -> DateTime<Utc> {
        chrono::Duration::try_seconds(self.window_seconds)
            .and_then(|window| self.created_at.checked_add_signed(window))
            .unwrap_or(self.created_at)
    }
}
