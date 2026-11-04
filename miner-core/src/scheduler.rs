use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::config::{AccountSettings, DropsOrder, DropsScope, Mode, StreamerSettings};
use crate::domain::{Campaign, Channel, PointsState};

// ! Declared in rank order: the derived `Ord` ranks the reasons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum PointsReason {
    StreakAtRisk,
    StreakBonus,
    Subscribed,
    ListOrder,
}

impl PointsReason {
    pub fn describe(self) -> &'static str {
        match self {
            Self::StreakAtRisk => "streak at risk",
            Self::StreakBonus => "streak bonus",
            Self::Subscribed => "subscribed",
            Self::ListOrder => "list order",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(rename = "SlotPurposeType")]
pub enum SlotPurpose {
    Drop {
        campaign_id: String,
        drop_id: String,
    },
    Points {
        reason: PointsReason,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum SlotPurposeKind {
    Drop,
    Points,
}

impl SlotPurpose {
    pub fn kind(&self) -> SlotPurposeKind {
        match self {
            Self::Drop { .. } => SlotPurposeKind::Drop,
            Self::Points { .. } => SlotPurposeKind::Points,
        }
    }

    pub fn same_watch(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Drop {
                    campaign_id: mine, ..
                },
                Self::Drop {
                    campaign_id: theirs,
                    ..
                },
            ) => mine == theirs,
            (Self::Points { reason: mine }, Self::Points { reason: theirs }) => mine == theirs,
            _ => false,
        }
    }

    pub fn keeps_stream(&self, next: &Self) -> bool {
        match (self, next) {
            (
                Self::Drop {
                    campaign_id: mine, ..
                },
                Self::Drop {
                    campaign_id: theirs,
                    ..
                },
            ) => mine == theirs,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Slot {
    pub channel: Channel,
    pub purpose: SlotPurpose,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum QueueReason {
    Ordered,
    Priority,
    EndingSoon,
    LowestRemaining,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum QueueBlock {
    NoLiveChannel,
    OneCampaignAtATime,
    NoDropAvailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "QueueItemType")]
pub struct QueueItem {
    pub campaign_id: String,
    pub reason: QueueReason,
    pub live_channels: usize,
    pub remaining_minutes: u32,
    pub ends_at: DateTime<Utc>,
    pub linked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub blocked: Option<QueueBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Plan {
    pub slots: Vec<Slot>,
    pub queue: Vec<QueueItem>,
}

// ! Twitch pays the watch streak bonus a few minutes into the watch of a new
// broadcast.
pub const STREAK_BONUS_WINDOW: chrono::Duration = chrono::Duration::minutes(10);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreakBonus {
    pub broadcast_id: String,
    pub watched_since: Option<DateTime<Utc>>,
    pub settled: bool,
}

impl StreakBonus {
    pub fn new(broadcast_id: impl Into<String>) -> Self {
        Self {
            broadcast_id: broadcast_id.into(),
            watched_since: None,
            settled: false,
        }
    }

    pub fn window_passed(&self, now: DateTime<Utc>) -> bool {
        self.watched_since
            .is_some_and(|since| now - since >= STREAK_BONUS_WINDOW)
    }

    pub fn pending(&self, now: DateTime<Utc>) -> bool {
        !self.settled && !self.window_passed(now)
    }
}

#[derive(Debug, Clone)]
pub struct StreamerState<'a> {
    pub settings: &'a StreamerSettings,
    pub channel: Option<&'a Channel>,
    pub points: &'a PointsState,
    pub streak_bonus: Option<&'a StreakBonus>,
}

fn contains_ci(haystack: &str, needle: &str) -> bool {
    haystack.to_lowercase().contains(&needle.to_lowercase())
}

fn game_matches(campaign: &Campaign, name: &str) -> bool {
    let name = name.trim();

    if name.is_empty() {
        return false;
    }

    // ! A name without Latin letters or digits ("原神") has no slug: two empty
    // slugs would make every such game match.
    let slug = crate::util::game_slug(name);

    campaign.game.name.eq_ignore_ascii_case(name)
        || campaign.game.id == name
        || (!slug.is_empty() && campaign.game.slug().eq_ignore_ascii_case(&slug))
}

pub fn excluded(settings: &AccountSettings, campaign: &Campaign) -> bool {
    let drops = &settings.drops;

    if drops
        .exclude_campaign_ids
        .iter()
        .any(|id| id == &campaign.id)
    {
        return true;
    }

    if drops
        .exclude_games
        .iter()
        .any(|g| game_matches(campaign, g))
    {
        return true;
    }

    drops.exclude_names.iter().any(|needle| {
        !needle.trim().is_empty()
            && (contains_ci(&campaign.name, needle)
                || campaign.drops.iter().any(|d| contains_ci(&d.name, needle)))
    })
}

fn priority_index(settings: &AccountSettings, campaign: &Campaign) -> Option<usize> {
    settings
        .drops
        .priority_games
        .iter()
        .position(|g| game_matches(campaign, g))
}

type RankKey = (u8, i64, u8, i64);

pub fn rank_campaigns<'a>(
    settings: &AccountSettings,
    campaigns: &'a [Campaign],
    now: DateTime<Utc>,
) -> Vec<(&'a Campaign, QueueReason)> {
    let drops = &settings.drops;
    let mut ranked: Vec<(&Campaign, QueueReason, RankKey)> = campaigns
        .iter()
        .filter(|c| {
            c.active_at(now) && !c.finished() && c.next_drop(now, &drops.exclude_drop_ids).is_some()
        })
        .filter(|c| !excluded(settings, c))
        .filter(|c| !drops.skip_unfinishable || c.can_finish(now, &drops.exclude_drop_ids))
        .filter_map(|c| {
            let pinned = drops.campaign_order.iter().position(|id| id == &c.id);
            let priority = priority_index(settings, c);

            let in_scope = match drops.scope {
                DropsScope::PriorityOnly => priority.is_some(),
                DropsScope::LinkedOnly => c.linked == Some(true),
                DropsScope::All => true,
            };

            if !in_scope && pinned.is_none() {
                return None;
            }

            let linked_rank = if drops.linked_first && c.linked != Some(true) {
                1
            } else {
                0
            };

            let (reason, tier, secondary) = match (pinned, priority) {
                (Some(index), _) => (QueueReason::Ordered, 0u8, index as i64),
                (None, Some(index)) => (QueueReason::Priority, 1, index as i64),
                (None, None) => match drops.order {
                    DropsOrder::EndingSoonest => {
                        (QueueReason::EndingSoon, 2, c.ends_at.timestamp())
                    }
                    DropsOrder::LowestRemaining => (
                        QueueReason::LowestRemaining,
                        2,
                        i64::from(c.remaining_minutes(&drops.exclude_drop_ids)),
                    ),
                },
            };

            let key = match tier {
                0 | 1 => (tier, secondary, 0, 0),
                _ => (tier, 0, linked_rank, secondary),
            };

            Some((c, reason, key))
        })
        .collect();

    ranked.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| a.0.id.cmp(&b.0.id)));

    ranked
        .into_iter()
        .map(|(c, reason, _)| (c, reason))
        .collect()
}

#[derive(Debug, Clone, Copy)]
pub struct CurrentDrop<'a> {
    pub campaign_id: &'a str,
    pub channel_id: &'a str,
}

pub fn best_channel<'a>(
    settings: &AccountSettings,
    campaign: &Campaign,
    live: &'a [Channel],
) -> Option<&'a Channel> {
    pick_channel(settings, campaign, live, None)
}

fn pick_channel<'a>(
    settings: &AccountSettings,
    campaign: &Campaign,
    live: &'a [Channel],
    watching: Option<&str>,
) -> Option<&'a Channel> {
    let preferred = |channel: &Channel| {
        settings.drops.streamers_first
            && (channel.drops_enabled || !campaign.allowed_channels.is_empty())
            && settings
                .points
                .streamers
                .iter()
                .any(|s| s.enabled && s.login == channel.login)
    };

    let eligible: Vec<&Channel> = live
        .iter()
        .filter(|c| c.live && campaign.accepts_channel(&c.id))
        .filter(|c| {
            campaign.allowed_channels.is_empty()
                || campaign.allowed_channels.iter().any(|a| a.id == c.id)
        })
        .filter(|c| {
            // ! Twitch credits drop minutes only on a stream of the campaign's
            // game, also for a channel of its allow list.
            c.game.as_ref().is_some_and(|g| {
                g.id == campaign.game.id || g.name.eq_ignore_ascii_case(&campaign.game.name)
            })
        })
        .collect();

    let kind = |c: &Channel| (preferred(c), c.drops_enabled);
    let best = eligible
        .iter()
        .copied()
        .max_by_key(|c| (kind(c), c.viewers.unwrap_or(0)))?;

    watching
        .and_then(|id| eligible.iter().copied().find(|c| c.id == id))
        .filter(|current| kind(current) == kind(best))
        .or(Some(best))
}

pub fn rank_streamers<'a>(
    settings: &AccountSettings,
    streamers: &[StreamerState<'a>],
    now: DateTime<Utc>,
) -> Vec<(&'a Channel, PointsReason)> {
    let points = &settings.points;
    let mut live: Vec<(PointsReason, bool, usize, &Channel)> = streamers
        .iter()
        .enumerate()
        .filter(|(_, s)| s.settings.enabled)
        .filter_map(|(index, s)| {
            let channel = s.channel.filter(|c| c.live)?;
            let streaks = points.streak_priority && s.settings.watch_streak;
            let bonus = s.streak_bonus.filter(|bonus| {
                channel.broadcast_id.as_deref() == Some(bonus.broadcast_id.as_str())
                    && bonus.pending(now)
            });
            let at_risk = s
                .points
                .streak
                .as_ref()
                .is_some_and(|streak| streak.at_risk(now));

            let reason = match bonus {
                _ if streaks && at_risk => PointsReason::StreakAtRisk,
                Some(_) if streaks => PointsReason::StreakBonus,
                _ if points.subscriber_priority && s.points.subscribed => PointsReason::Subscribed,
                _ => PointsReason::ListOrder,
            };

            let waiting = reason == PointsReason::StreakBonus
                && bonus.is_some_and(|bonus| bonus.watched_since.is_none());

            Some((reason, waiting, index, channel))
        })
        .collect();

    live.sort_by_key(|(reason, waiting, index, _)| (*reason, *waiting, *index));

    live.into_iter()
        .map(|(reason, _, _, channel)| (channel, reason))
        .collect()
}

pub fn plan(
    settings: &AccountSettings,
    campaigns: &[Campaign],
    live_by_campaign: &HashMap<String, Vec<Channel>>,
    streamers: &[StreamerState<'_>],
    current: Option<CurrentDrop<'_>>,
    now: DateTime<Utc>,
) -> Plan {
    let slots = settings.slots();
    let mut plan = Plan::default();
    let mut used: Vec<String> = Vec::new();

    if settings.mode != Mode::Points {
        let mut ranked = rank_campaigns(settings, campaigns, now);

        let watching = |campaign: &Campaign| {
            current
                .filter(|current| current.campaign_id == campaign.id)
                .map(|current| current.channel_id)
        };
        let minable = |campaign: &Campaign| {
            let live = live_by_campaign
                .get(&campaign.id)
                .map(Vec::as_slice)
                .unwrap_or(&[]);

            campaign
                .next_drop(now, &settings.drops.exclude_drop_ids)
                .is_some()
                && pick_channel(settings, campaign, live, None).is_some()
        };

        if let Some(current) = current {
            let viable = ranked
                .iter()
                .position(|(c, _)| c.id == current.campaign_id)
                .filter(|&index| {
                    let (campaign, reason) = &ranked[index];
                    let live = live_by_campaign
                        .get(&campaign.id)
                        .map(Vec::as_slice)
                        .unwrap_or(&[]);
                    let above: Vec<QueueReason> = ranked[..index]
                        .iter()
                        .filter(|(c, _)| minable(c))
                        .map(|(_, r)| *r)
                        .collect();

                    pick_channel(settings, campaign, live, watching(campaign)).is_some()
                        && !above.contains(&QueueReason::Ordered)
                        && (*reason == QueueReason::Priority
                            || *reason == QueueReason::Ordered
                            || !above.contains(&QueueReason::Priority))
                });

            if let Some(index) = viable {
                let item = ranked.remove(index);
                ranked.insert(0, item);
            }
        }

        let mut mining = false;

        for (campaign, reason) in ranked {
            let live = live_by_campaign
                .get(&campaign.id)
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            let chosen = pick_channel(settings, campaign, live, watching(campaign));
            let mut item = QueueItem {
                campaign_id: campaign.id.clone(),
                reason,
                live_channels: live.iter().filter(|c| c.live).count(),
                remaining_minutes: campaign.remaining_minutes(&settings.drops.exclude_drop_ids),
                ends_at: campaign.ends_at,
                linked: campaign.linked,
                blocked: None,
            };

            match (
                mining,
                chosen,
                campaign.next_drop(now, &settings.drops.exclude_drop_ids),
            ) {
                (false, Some(channel), Some(drop)) => {
                    mining = true;
                    used.push(channel.id.clone());
                    plan.slots.push(Slot {
                        channel: channel.clone(),
                        purpose: SlotPurpose::Drop {
                            campaign_id: campaign.id.clone(),
                            drop_id: drop.id.clone(),
                        },
                    });
                }
                (false, None, _) => item.blocked = Some(QueueBlock::NoLiveChannel),
                (true, _, _) => item.blocked = Some(QueueBlock::OneCampaignAtATime),
                (false, Some(_), None) => item.blocked = Some(QueueBlock::NoDropAvailable),
            }

            plan.queue.push(item);
        }
    }

    if settings.mode != Mode::Drops {
        for (channel, reason) in rank_streamers(settings, streamers, now) {
            if plan.slots.len() >= slots {
                break;
            }

            if used.contains(&channel.id) {
                continue;
            }

            used.push(channel.id.clone());
            plan.slots.push(Slot {
                channel: channel.clone(),
                purpose: SlotPurpose::Points { reason },
            });
        }
    }

    plan
}
