// ! Never through an account's proxy: a proxy of the environment
// (`HTTPS_PROXY`) applies, as it does to the desktop updater.

use std::time::{Duration, Instant};

use anyhow::{anyhow, bail};
use miner_core::error::ErrorCode;
use miner_core::requests::{address_target, RequestKind, RequestLog};
use miner_core::Manager;
use reqwest::redirect::Policy;
use reqwest::Url;
use semver::Version;
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::access::HTTPS;

const TIMEOUT: Duration = Duration::from_secs(30);
const MAX_BYTES: usize = 256 * 1024;
const ANSWER_FRESH_FOR: Duration = Duration::from_secs(60);
// ! The latest release's file is served by the tagged release, which sends
// it on to GitHub's file storage: nothing else is followed.
const MAX_REDIRECTS: usize = 5;
const GITHUB_HOST: &str = "github.com";
const GITHUB_FILES_HOST_SUFFIX: &str = ".githubusercontent.com";

#[derive(Deserialize)]
struct LatestRelease {
    version: String,
}

struct Answer {
    asked_at: Instant,
    newer: Result<Option<String>, ErrorCode>,
}

pub struct Releases {
    client: reqwest::Client,
    url: String,
    last_answer: Mutex<Option<Answer>>,
}

impl Releases {
    pub fn new() -> anyhow::Result<Self> {
        Self::with_url(&miner_core::updates::latest_release_url())
    }

    pub(crate) fn with_url(url: &str) -> anyhow::Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(format!(
                "TwitchDropsPointsMiner/{} (+{})",
                miner_core::VERSION,
                miner_core::REPOSITORY_URL
            ))
            .timeout(TIMEOUT)
            .redirect(Policy::custom(|attempt| {
                if attempt.previous().len() >= MAX_REDIRECTS {
                    attempt.error("too many redirects")
                } else if is_github(attempt.url()) {
                    attempt.follow()
                } else {
                    attempt.stop()
                }
            }))
            .build()?;

        Ok(Self {
            client,
            url: url.to_owned(),
            last_answer: Mutex::new(None),
        })
    }

    pub async fn check(&self, manager: &Manager) -> Result<Option<String>, ErrorCode> {
        self.check_at(manager, Instant::now()).await
    }

    pub(crate) async fn check_at(
        &self,
        manager: &Manager,
        now: Instant,
    ) -> Result<Option<String>, ErrorCode> {
        let mut last_answer = self.last_answer.lock().await;

        if let Some(answer) = last_answer
            .as_ref()
            .filter(|answer| now.saturating_duration_since(answer.asked_at) < ANSWER_FRESH_FOR)
        {
            return answer.newer.clone();
        }

        let newer = match self.latest(&manager.requests).await {
            Ok(latest) => {
                manager.update_checked();

                let newer = is_newer(&latest).then(|| latest.to_string());

                match &newer {
                    Some(version) => {
                        if let Err(error) = manager.announce_update(version) {
                            tracing::warn!(%error, "the new version was not announced");
                        }
                    }
                    None => manager.clear_update(),
                }

                Ok(newer)
            }
            Err(error) => {
                tracing::info!("the update check failed: {error:#}");

                Err(ErrorCode::NetworkError)
            }
        };

        *last_answer = Some(Answer {
            asked_at: now,
            newer: newer.clone(),
        });

        newer
    }

    pub async fn check_regularly(&self, manager: &Manager) {
        miner_core::updates::on_schedule(
            manager,
            || async move { self.check(manager).await.is_ok() },
        )
        .await;
    }

    async fn latest(&self, requests: &RequestLog) -> anyhow::Result<Version> {
        let pending = requests.start(RequestKind::UpdateCheck, address_target(&self.url));
        let sent = self
            .client
            .get(&self.url)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await;

        pending.sent(&sent);

        let mut response = sent.map_err(reqwest::Error::without_url)?;

        let status = response.status();

        if !status.is_success() {
            bail!("GitHub answered {status}");
        }

        if response
            .content_length()
            .is_some_and(|length| length > MAX_BYTES as u64)
        {
            bail!("the release manifest is too large");
        }

        let mut body = Vec::new();

        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(reqwest::Error::without_url)?
        {
            body.extend_from_slice(&chunk);

            if body.len() > MAX_BYTES {
                bail!("the release manifest is too large");
            }
        }

        let release: LatestRelease = serde_json::from_slice(&body)?;
        let version = release.version.trim();

        Version::parse(version.strip_prefix('v').unwrap_or(version))
            .map_err(|error| anyhow!("the release version {version:?} is not one: {error}"))
    }
}

fn is_github(url: &Url) -> bool {
    url.scheme() == HTTPS
        && url
            .host_str()
            .is_some_and(|host| host == GITHUB_HOST || host.ends_with(GITHUB_FILES_HOST_SUFFIX))
}

fn is_newer(latest: &Version) -> bool {
    Version::parse(miner_core::VERSION).is_ok_and(|current| *latest > current)
}
