use std::collections::{BTreeMap, HashMap};
use std::sync::Mutex;
use std::time::Duration;

use tokio::time::Instant;

use crate::util::locked;

// ! A channel without a banner is kept too, so it is not asked again at each
// page; it may get one, so it is asked again after a while like the others.
const KEPT_FOR: Duration = Duration::from_secs(60 * 60);
const KEPT_MAX: usize = 2_000;

#[derive(Default)]
pub(super) struct Banners(Mutex<HashMap<String, (Option<String>, Instant)>>);

impl Banners {
    pub(super) fn missing(&self, wanted: &[String]) -> Vec<String> {
        let now = Instant::now();
        let known = locked(&self.0);

        wanted
            .iter()
            .filter(|login| {
                known
                    .get(*login)
                    .is_none_or(|(_, at)| now.duration_since(*at) >= KEPT_FOR)
            })
            .cloned()
            .collect()
    }

    pub(super) fn keep(&self, answered: Vec<(String, Option<String>)>) {
        let now = Instant::now();
        let mut known = locked(&self.0);

        known.retain(|_, (_, at)| now.duration_since(*at) < KEPT_FOR);
        known.extend(
            answered
                .into_iter()
                .map(|(login, banner)| (login, (banner, now))),
        );

        let over = known.len().saturating_sub(KEPT_MAX);

        if over > 0 {
            let mut oldest: Vec<(Instant, String)> = known
                .iter()
                .map(|(login, (_, at))| (*at, login.clone()))
                .collect();

            oldest.sort();

            for (_, login) in oldest.into_iter().take(over) {
                known.remove(&login);
            }
        }
    }

    pub(super) fn found(&self, wanted: Vec<String>) -> BTreeMap<String, String> {
        let known = locked(&self.0);

        wanted
            .into_iter()
            .filter_map(|login| {
                let banner = known.get(&login)?.0.clone()?;

                Some((login, banner))
            })
            .collect()
    }
}
