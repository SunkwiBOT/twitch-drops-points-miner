// ! Twitch does not show the campaign list to the TV client, so campaigns
// come from this public catalogue.

use std::collections::HashSet;
use std::sync::Mutex;
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::Shell;
use crate::domain::Campaign;
use crate::error::{Error, ErrorCode, Result};
use crate::requests::{address_target, RequestKind, RequestLog};
use crate::twitch::drops::parse_campaign;
use crate::twitch::note_left_out;
use crate::util::locked;

const CATALOG_URL: &str = "https://drops.sunkwi.com/api/twitch/drops";

// ! The public catalogue allows one anonymous request every 5 minutes.
pub const REFRESH_INTERVAL: Duration = Duration::from_secs(600);

const MAX_CAMPAIGNS: usize = 3000;

const MAX_BYTES: usize = 16 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Catalog {
    pub campaigns: Vec<Campaign>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fetched_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub problem: Option<ErrorCode>,
}

pub struct CatalogClient {
    client: reqwest::Client,
    url: String,
    requests: RequestLog,
    // ! An answer "not modified" (304) costs no request of the quota: the last
    // catalogue read is kept with the tag that names it.
    last: Mutex<Option<(String, Catalog)>>,
}

pub struct Fetch {
    pub result: Result<Catalog>,
    pub retry_after: Duration,
}

impl CatalogClient {
    pub fn new(requests: RequestLog, shell: Shell) -> Result<Self> {
        Self::with_url(CATALOG_URL, requests, &crate::user_agent(shell))
    }

    pub(crate) fn with_url(url: &str, requests: RequestLog, agent: &str) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(agent)
            .timeout(Duration::from_secs(30))
            .gzip(true)
            .build()
            .map_err(|e| Error::Network(e.to_string()))?;

        Ok(Self {
            client,
            url: url.to_owned(),
            requests,
            last: Mutex::new(None),
        })
    }

    pub fn interval() -> Duration {
        REFRESH_INTERVAL
    }

    pub async fn fetch(&self) -> Fetch {
        let mut retry_after = Duration::ZERO;
        let result = async {
            let tag = locked(&self.last).as_ref().map(|(tag, _)| tag.clone());
            let mut request = self
                .client
                .get(&self.url)
                .header("Accept", "application/json");

            if let Some(tag) = &tag {
                request = request.header("If-None-Match", tag);
            }

            let response = self
                .requests
                .start(RequestKind::Catalog, address_target(&self.url))
                .send(request)
                .await?;

            for name in ["ratelimit-reset", "retry-after"] {
                if let Some(seconds) = response
                    .headers()
                    .get(name)
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.trim().parse::<u64>().ok())
                {
                    retry_after = retry_after.max(Duration::from_secs(seconds.min(3600)));
                }
            }

            let status = response.status();

            if status == reqwest::StatusCode::NOT_MODIFIED {
                if let Some((_, catalog)) = locked(&self.last).as_ref() {
                    return Ok(Catalog {
                        fetched_at: Some(Utc::now()),
                        ..catalog.clone()
                    });
                }
            }

            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                return Err(Error::CatalogRateLimited);
            }

            if !status.is_success() {
                return Err(Error::Status(status));
            }

            let tag = response
                .headers()
                .get(reqwest::header::ETAG)
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned);
            let body = read_bounded(response).await?;
            let payload: Value = serde_json::from_slice(&body)?;
            let catalog = parse_catalog(&payload)?;

            *locked(&self.last) = tag.map(|tag| (tag, catalog.clone()));

            Ok(catalog)
        }
        .await;

        Fetch {
            result,
            retry_after,
        }
    }
}

// ! The gzip body is unpacked while it streams, so the bound also holds for a
// small download that unpacks into a huge one.
async fn read_bounded(mut response: reqwest::Response) -> Result<Vec<u8>> {
    if response
        .content_length()
        .is_some_and(|length| length > MAX_BYTES as u64)
    {
        return Err(too_large());
    }

    let mut body = Vec::new();

    while let Some(chunk) = response.chunk().await? {
        if body.len() + chunk.len() > MAX_BYTES {
            return Err(too_large());
        }

        body.extend_from_slice(&chunk);
    }

    Ok(body)
}

fn too_large() -> Error {
    Error::InvalidResponse("catalogue too large".into())
}

pub fn parse_catalog(payload: &Value) -> Result<Catalog> {
    let records = payload["games"]
        .as_array()
        .ok_or_else(|| Error::InvalidResponse("catalogue games must be a list".into()))?;

    if records.len() > MAX_CAMPAIGNS {
        return Err(too_large());
    }

    let mut campaigns = Vec::new();

    for game in records {
        let box_art = game["gameBoxArtURL"].as_str().map(str::to_owned);
        let rewards = game["rewards"].as_array();
        let items: Vec<&Value> = match rewards {
            Some(list) => list.iter().collect(),
            None => vec![game],
        };

        let before = campaigns.len();
        let total = items.len();

        for record in items {
            if let Some(mut campaign) = parse_campaign(record, false) {
                if campaign.game.box_art_url.is_none() {
                    campaign.game.box_art_url = box_art.clone();
                }

                campaigns.push(campaign);
            }
        }

        note_left_out("catalogue campaigns", total, campaigns.len() - before);
    }

    // ! The catalogue can list a campaign more than once: the interface keys its
    // lists by id.
    let mut seen: HashSet<String> = HashSet::new();

    campaigns.retain(|campaign| seen.insert(campaign.id.clone()));
    campaigns.sort_by(|a, b| a.ends_at.cmp(&b.ends_at).then_with(|| a.id.cmp(&b.id)));

    Ok(Catalog {
        campaigns,
        updated_at: payload["refreshedAt"]
            .as_str()
            .and_then(|v| v.parse::<DateTime<Utc>>().ok()),
        fetched_at: Some(Utc::now()),
        problem: None,
    })
}
