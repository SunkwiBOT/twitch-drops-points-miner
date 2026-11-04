use chrono::{DateTime, Duration, Utc};

use crate::config::{BetFilter, BetTiming, FilterBy, FilterOp, PredictionSettings, Strategy};
use crate::domain::{PredictionEvent, PredictionOutcome};

pub const MAX_PREDICTION_POINTS: i64 = 250_000;

const TIMING_PERCENT_MIN: u8 = 1;
const TIMING_PERCENT_MAX: u8 = 99;

pub fn bet_time(event: &PredictionEvent, settings: &PredictionSettings) -> DateTime<Utc> {
    match settings.timing {
        BetTiming::AtOpen => event.created_at,
        BetTiming::BeforeClose => event
            .closes_at()
            .checked_sub_signed(Duration::seconds(i64::from(settings.delay_seconds)))
            .unwrap_or(event.created_at),
        BetTiming::Percent => {
            let percent = i64::from(
                settings
                    .timing_percent
                    .clamp(TIMING_PERCENT_MIN, TIMING_PERCENT_MAX),
            );

            let share = event.window_seconds.saturating_mul(10 * percent);

            Duration::try_milliseconds(share)
                .and_then(|share| event.created_at.checked_add_signed(share))
                .unwrap_or_else(|| event.closes_at())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    pub outcome_id: String,
    pub outcome_index: usize,
    pub points: i64,
}

fn max_by_f64(
    outcomes: &[PredictionOutcome],
    score: impl Fn(&PredictionOutcome) -> f64,
) -> Option<usize> {
    let mut best: Option<(usize, f64)> = None;

    for (index, outcome) in outcomes.iter().enumerate() {
        let value = score(outcome);

        if best.is_none_or(|(_, b)| value > b) {
            best = Some((index, value));
        }
    }

    best.map(|(index, _)| index)
}

fn max_by_i64(
    outcomes: &[PredictionOutcome],
    key: impl Fn(&PredictionOutcome) -> i64,
) -> Option<usize> {
    let mut best: Option<(usize, i64)> = None;

    for (index, outcome) in outcomes.iter().enumerate() {
        let value = key(outcome);

        if best.is_none_or(|(_, b)| value > b) {
            best = Some((index, value));
        }
    }

    best.map(|(index, _)| index)
}

pub fn select_outcome(
    outcomes: &[PredictionOutcome],
    settings: &PredictionSettings,
) -> Option<usize> {
    if outcomes.is_empty() {
        return None;
    }

    let numbered = |index: usize| {
        if index < outcomes.len() {
            Some(index)
        } else {
            max_by_f64(outcomes, |o| o.odds)
        }
    };

    match settings.strategy {
        Strategy::MostVoted => max_by_i64(outcomes, |o| o.total_users),
        Strategy::HighOdds => max_by_f64(outcomes, |o| o.odds),
        Strategy::Percentage => max_by_f64(outcomes, |o| o.odds_percentage),
        Strategy::SmartMoney => max_by_i64(outcomes, |o| o.top_points),
        Strategy::Number1 => numbered(0),
        Strategy::Number2 => numbered(1),
        Strategy::Number3 => numbered(2),
        Strategy::Number4 => numbered(3),
        Strategy::Smart => {
            let gap = f64::from(settings.percentage_gap);
            let mut sorted: Vec<f64> = outcomes.iter().map(|o| o.percentage_users).collect();

            sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

            if sorted.len() >= 2 && (sorted[0] - sorted[1]).abs() < gap {
                max_by_f64(outcomes, |o| o.odds)
            } else {
                max_by_i64(outcomes, |o| o.total_users)
            }
        }
    }
}

fn passes_filter(filter: &BetFilter, event: &PredictionEvent, chosen: &PredictionOutcome) -> bool {
    let value = match filter.by {
        FilterBy::TotalUsers => event.outcomes.iter().map(|o| o.total_users).sum::<i64>() as f64,
        FilterBy::TotalPoints => event.outcomes.iter().map(|o| o.total_points).sum::<i64>() as f64,
        FilterBy::Odds => chosen.odds,
        FilterBy::OddsPercentage => chosen.odds_percentage,
        FilterBy::PercentageUsers => chosen.percentage_users,
        FilterBy::DecisionUsers => chosen.total_users as f64,
        FilterBy::DecisionPoints => chosen.total_points as f64,
    };

    match filter.op {
        FilterOp::Gt => value > filter.value,
        FilterOp::Lt => value < filter.value,
        FilterOp::Gte => value >= filter.value,
        FilterOp::Lte => value <= filter.value,
    }
}

pub fn decide(
    event: &PredictionEvent,
    settings: &PredictionSettings,
    balance: i64,
) -> Option<Decision> {
    if !settings.enabled || balance <= 0 || balance <= i64::from(settings.min_points) {
        return None;
    }

    let index = select_outcome(&event.outcomes, settings)?;
    let chosen = &event.outcomes[index];

    if let Some(filter) = &settings.filter {
        if !passes_filter(filter, event, chosen) {
            return None;
        }
    }

    let mut points = balance * i64::from(settings.percentage.clamp(1, 100)) / 100;

    points = points
        .min(i64::from(settings.max_points))
        .min(MAX_PREDICTION_POINTS)
        .min(balance);

    if settings.stealth && chosen.top_points > 0 && points >= chosen.top_points {
        let offset = 1 + (rand::random::<u8>() % 5) as i64;
        points = (chosen.top_points - offset).max(0);
    }

    if points <= 0 {
        return None;
    }

    Some(Decision {
        outcome_id: chosen.id.clone(),
        outcome_index: index,
        points,
    })
}
