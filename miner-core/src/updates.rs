use std::future::Future;
use std::time::Duration;

use crate::Manager;

// ! Under the repository: the same file as the `updater` endpoint of
// src-tauri/tauri.conf.json.
pub const LATEST_RELEASE_PATH: &str = "/releases/latest/download/latest.json";

pub fn latest_release_url() -> String {
    format!("{}{LATEST_RELEASE_PATH}", crate::REPOSITORY_URL)
}

// ! A sleeping computer stops the monotonic clock: the wait is cut in slices
// that also look at the wall clock, so a check due during the sleep runs soon
// after the wake.
const WALL_CLOCK_SLICE: Duration = Duration::from_secs(5 * 60);
const RETRY_DELAYS: [Duration; 3] = [
    Duration::from_secs(5 * 60),
    Duration::from_secs(15 * 60),
    Duration::from_secs(30 * 60),
];
const FAILURES_BEFORE_WARNING: usize = 3;

pub async fn on_schedule<F: Future<Output = bool>>(
    manager: &Manager,
    mut check: impl FnMut() -> F,
) {
    let changed = manager.changed.notified();
    tokio::pin!(changed);
    changed.as_mut().enable();

    let mut failures: usize = 0;

    loop {
        if check().await {
            failures = 0;
        } else {
            failures += 1;

            if failures == FAILURES_BEFORE_WARNING {
                tracing::warn!(
                    "{failures} update checks in a row got no answer; the next ones still run"
                );
            }
        }

        let checked = tokio::time::Instant::now();
        let checked_wall = std::time::SystemTime::now();
        let retry = failures
            .checked_sub(1)
            .map(|index| RETRY_DELAYS[index.min(RETRY_DELAYS.len() - 1)]);

        loop {
            let interval = manager.global_settings().await.update_interval.period();
            let wait = match (interval, retry) {
                (Some(interval), Some(retry)) => Some(interval.min(retry)),
                (interval, retry) => interval.or(retry),
            };
            let due = async {
                let Some(wait) = wait else {
                    return std::future::pending().await;
                };

                loop {
                    let slept_through = checked_wall.elapsed().is_ok_and(|elapsed| elapsed >= wait);

                    if slept_through || checked.elapsed() >= wait {
                        return;
                    }

                    tokio::time::sleep_until(
                        (checked + wait).min(tokio::time::Instant::now() + WALL_CLOCK_SLICE),
                    )
                    .await;
                }
            };

            tokio::select! {
                _ = due => break,
                _ = &mut changed => {
                    changed.set(manager.changed.notified());
                    changed.as_mut().enable();
                }
            }
        }
    }
}
