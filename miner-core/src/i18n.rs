use crate::events::{BetResult, EventKind, MinerEvent};
use crate::scheduler::PointsReason;

include!(concat!(env!("OUT_DIR"), "/texts.rs"));

pub const AUTO: &str = "AUTO";
pub const FALLBACK: &str = "en";

pub fn resolve(setting: &str) -> &'static str {
    if setting != AUTO {
        if let Some(tag) = known(setting) {
            return tag;
        }
    }

    sys_locale::get_locales()
        .find_map(|candidate| matched(&candidate))
        .unwrap_or(FALLBACK)
}

pub fn is_known(tag: &str) -> bool {
    known(tag).is_some()
}

fn known(tag: &str) -> Option<&'static str> {
    TEXTS
        .iter()
        .map(|(known, _)| *known)
        .find(|known| known.eq_ignore_ascii_case(tag))
}

fn matched(candidate: &str) -> Option<&'static str> {
    let lower = candidate
        .split(['.', '@'])
        .next()?
        .replace('_', "-")
        .to_ascii_lowercase();

    if lower.is_empty() || lower == "c" || lower == "posix" {
        return None;
    }

    if let Some(tag) = known(&lower) {
        return Some(tag);
    }

    if lower.starts_with("es") {
        return known(if lower.contains("es-es") {
            "es-ES"
        } else {
            "es-419"
        });
    }

    if lower.starts_with("pt") {
        return known(if lower.contains("br") {
            "pt-BR"
        } else {
            "pt-PT"
        });
    }

    if lower.starts_with("zh") {
        let traditional = ["tw", "hant", "hk"].iter().any(|part| lower.contains(part));

        return known(if traditional { "zh-TW" } else { "zh-CN" });
    }

    if lower.starts_with("no") || lower.starts_with("nn") {
        return known("nb");
    }

    known(lower.split('-').next()?)
}

fn lookup(language: &str, key: &str) -> Option<&'static str> {
    TEXTS
        .iter()
        .find(|(tag, _)| *tag == language)?
        .1
        .iter()
        .find(|(name, _)| *name == key)
        .map(|(_, text)| *text)
}

// ! One pass: a value is never read as a placeholder.
pub fn text(language: &str, key: &str, args: &[(&str, &str)]) -> String {
    let template = lookup(language, key)
        .or_else(|| lookup(FALLBACK, key))
        .unwrap_or(key);
    let mut out = String::with_capacity(template.len());
    let mut rest = template;

    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let value = after.find('}').and_then(|close| {
            let name = &after[..close];
            args.iter()
                .find(|(arg, _)| *arg == name)
                .map(|(_, value)| (*value, close))
        });

        match value {
            Some((value, close)) => {
                out.push_str(value);
                rest = &after[close + 1..];
            }
            None => {
                out.push('{');
                rest = after;
            }
        }
    }

    out.push_str(rest);

    out
}

pub fn number(language: &str, value: i64) -> String {
    let separator = match language {
        "fr" => "\u{202f}",
        "pt-PT" | "cs" | "pl" | "ru" | "uk" | "bg" | "hu" | "sv" | "nb" | "fi" => "\u{a0}",
        "de" | "nl" | "it" | "es-ES" | "pt-BR" | "id" | "tr" | "el" | "ro" | "da" | "vi" => ".",
        _ => ",",
    };

    // ! "1500", not "1 500": these languages group from five digits.
    let min_grouped = match language {
        "es-ES" | "pt-PT" | "it" | "pl" | "hu" | "bg" => 5,
        _ => 4,
    };

    let minus = match language {
        "sv" | "nb" | "fi" => "\u{2212}",
        _ => "-",
    };

    let digits = value.unsigned_abs().to_string();
    let grouped = digits.len() >= min_grouped;
    let mut out = String::new();

    if value < 0 {
        out.push_str(minus);
    }

    for (index, digit) in digits.chars().enumerate() {
        if grouped && index > 0 && (digits.len() - index).is_multiple_of(3) {
            out.push_str(separator);
        }

        out.push(digit);
    }

    out
}

pub fn kind_label(language: &str, kind: EventKind) -> String {
    let key = match kind {
        EventKind::Info => "event_kind_info",
        EventKind::Error => "event_kind_error",
        EventKind::WatchStarted => "event_kind_watch_started",
        EventKind::WatchStopped => "event_kind_watch_stopped",
        EventKind::DropProgress => "event_kind_drop_progress",
        EventKind::DropClaimed => "event_kind_drop_claimed",
        EventKind::CampaignFinished => "event_kind_campaign_finished",
        EventKind::PointsEarned => "event_kind_points_earned",
        EventKind::PointsSpent => "event_kind_points_spent",
        EventKind::BonusClaimed => "event_kind_bonus_claimed",
        EventKind::PredictionPlaced => "event_kind_prediction_placed",
        EventKind::PredictionResult => "event_kind_prediction_result",
        EventKind::StreamOnline => "event_kind_stream_online",
        EventKind::StreamOffline => "event_kind_stream_offline",
        EventKind::RaidJoined => "event_kind_raid_joined",
        EventKind::MomentClaimed => "event_kind_moment_claimed",
        EventKind::StreakRecovered => "event_kind_streak_recovered",
        EventKind::GoalContributed => "event_kind_goal_contributed",
        EventKind::ChatMention => "event_kind_chat_mention",
        EventKind::SessionExpired => "event_kind_session_expired",
        EventKind::LoginCompleted => "event_kind_login_completed",
        EventKind::UpdateAvailable => "event_kind_update_available",
        EventKind::RewardRedeemed => "event_kind_reward_redeemed",
    };

    text(language, key, &[])
}

fn watch_points_key(reason: PointsReason) -> &'static str {
    match reason {
        PointsReason::StreakBonus => "ev_watch_points_streak_bonus",
        PointsReason::StreakAtRisk => "ev_watch_points_streak_at_risk",
        PointsReason::Subscribed => "ev_watch_points_subscribed",
        PointsReason::ListOrder => "ev_watch_points_list_order",
    }
}

pub fn event_text(language: &str, event: &MinerEvent) -> String {
    let context = &event.context;
    let channel = context
        .channel_name
        .as_deref()
        .or(context.channel.as_deref())
        .filter(|name| !name.is_empty());
    let drop = context.drop_name.as_deref();
    let campaign = context.campaign_name.as_deref();
    let count = |value: i64| number(language, value);
    let say = |key: &str, args: &[(&str, &str)]| Some(text(language, key, args));

    let sentence = match event.kind {
        EventKind::WatchStarted => match (channel, campaign, context.watch_reason) {
            (Some(channel), Some(campaign), _) => say(
                "ev_watch_drop",
                &[("channel", channel), ("campaign", campaign)],
            ),
            (Some(channel), None, Some(reason)) => {
                say(watch_points_key(reason), &[("channel", channel)])
            }
            _ => None,
        },
        EventKind::WatchStopped => {
            channel.and_then(|channel| say("ev_watch_stopped", &[("channel", channel)]))
        }
        EventKind::DropClaimed => match (drop, campaign) {
            (Some(drop), Some(campaign)) => {
                say("ev_drop_claimed", &[("drop", drop), ("campaign", campaign)])
            }
            _ => None,
        },
        EventKind::CampaignFinished => {
            campaign.and_then(|campaign| say("ev_campaign_finished", &[("campaign", campaign)]))
        }
        EventKind::DropProgress => match (drop, context.current, context.required) {
            (Some(drop), Some(current), Some(required)) => say(
                "ev_drop_progress",
                &[
                    ("drop", drop),
                    ("current", &count(i64::from(current))),
                    ("required", &count(i64::from(required))),
                ],
            ),
            (None, Some(current), Some(required)) => say(
                "ev_drop_progress_unknown",
                &[
                    ("current", &count(i64::from(current))),
                    ("required", &count(i64::from(required))),
                ],
            ),
            _ => None,
        },
        EventKind::BonusClaimed => {
            channel.and_then(|channel| say("ev_bonus", &[("channel", channel)]))
        }
        EventKind::PointsEarned => match (channel, context.earned) {
            (Some(channel), Some(earned)) => say(
                "ev_points",
                &[("channel", channel), ("points", &count(earned))],
            ),
            _ => None,
        },
        EventKind::PointsSpent => match (channel, context.spent, context.balance) {
            (Some(channel), Some(spent), _) => say(
                "ev_points_spent",
                &[("channel", channel), ("points", &count(spent))],
            ),
            (Some(channel), None, Some(balance)) => say(
                "ev_points_spent_balance",
                &[("channel", channel), ("balance", &count(balance))],
            ),
            _ => None,
        },
        EventKind::StreamOnline => {
            channel.and_then(|channel| say("ev_online", &[("channel", channel)]))
        }
        EventKind::StreamOffline => {
            channel.and_then(|channel| say("ev_offline", &[("channel", channel)]))
        }
        EventKind::RaidJoined => {
            channel.and_then(|channel| say("ev_raid", &[("channel", channel)]))
        }
        EventKind::MomentClaimed => {
            channel.and_then(|channel| say("ev_moment", &[("channel", channel)]))
        }
        EventKind::PredictionPlaced => match (channel, context.points) {
            (Some(channel), Some(points)) => say(
                "ev_bet_placed",
                &[
                    ("channel", channel),
                    ("points", &count(points)),
                    ("outcome", context.outcome.as_deref().unwrap_or_default()),
                ],
            ),
            _ => None,
        },
        EventKind::PredictionResult => match context.bet_result {
            Some(BetResult::Win) => say(
                "ev_bet_won",
                &[("points", &count(context.points_won.unwrap_or_default()))],
            ),
            Some(BetResult::Lose) => say(
                "ev_bet_lost",
                &[("points", &count(context.staked.unwrap_or_default()))],
            ),
            Some(BetResult::Refund) => say("ev_bet_refund", &[]),
            None => None,
        },
        EventKind::LoginCompleted => {
            let login = context
                .account_name
                .as_deref()
                .unwrap_or(event.account_login.as_str());

            say("ev_login", &[("login", login)])
        }
        EventKind::SessionExpired => say("ev_session_expired", &[]),
        EventKind::UpdateAvailable => context
            .version
            .as_deref()
            .and_then(|version| say("ev_update_available", &[("version", version)])),
        EventKind::StreakRecovered => {
            channel.and_then(|channel| say("ev_streak_recovered", &[("channel", channel)]))
        }
        EventKind::GoalContributed => match (channel, context.points) {
            (Some(channel), Some(points)) => say(
                "ev_goal_contributed",
                &[
                    ("channel", channel),
                    ("points", &count(points)),
                    ("goal", context.goal.as_deref().unwrap_or_default()),
                ],
            ),
            _ => None,
        },
        EventKind::ChatMention => match (channel, context.chatter.as_deref()) {
            (Some(channel), Some(chatter)) => say(
                "ev_chat_mention",
                &[("channel", channel), ("chatter", chatter)],
            ),
            _ => None,
        },
        EventKind::Info | EventKind::Error | EventKind::RewardRedeemed => None,
    };

    sentence.unwrap_or_else(|| event.title.clone())
}
