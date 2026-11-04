// ! Only pictures on Twitch's CDN are fetched, so an event never makes the app
// download an arbitrary URL.

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use reqwest::header::CONTENT_TYPE;
use sha2::{Digest, Sha256};
use url::Url;

use crate::error::{Error, Result};
use crate::requests::{url_target, RequestKind, RequestLog};

const TIMEOUT: Duration = Duration::from_secs(5);
const MAX_BYTES: usize = 2 * 1024 * 1024;
const MAX_AGE: Duration = Duration::from_secs(30 * 24 * 60 * 60);
const MAX_REDIRECTS: usize = 3;
const EXTENSIONS: [&str; 5] = ["png", "jpg", "jpeg", "gif", "webp"];

pub fn is_twitch_cdn(url: &str) -> bool {
    Url::parse(url).is_ok_and(|url| trusted(&url))
}

fn trusted(url: &Url) -> bool {
    url.scheme() == "https"
        && url.username().is_empty()
        && url.password().is_none()
        && url.port().is_none()
        && url
            .domain()
            .is_some_and(|host| host.ends_with(".jtvnw.net") || host.ends_with(".twitchcdn.net"))
}

#[derive(Clone)]
pub struct ImageCache {
    dir: PathBuf,
    client: reqwest::Client,
    requests: RequestLog,
}

impl ImageCache {
    pub async fn open(data_dir: &Path, requests: RequestLog) -> Result<Self> {
        let dir = data_dir.join("cache").join("images");

        tokio::fs::create_dir_all(&dir).await.map_err(disk)?;

        let redirects = reqwest::redirect::Policy::custom(|attempt| {
            if attempt.previous().len() < MAX_REDIRECTS && trusted(attempt.url()) {
                attempt.follow()
            } else {
                attempt.stop()
            }
        });
        let client = reqwest::Client::builder()
            .timeout(TIMEOUT)
            .redirect(redirects)
            .user_agent(format!("twitch-drops-points-miner/{}", crate::VERSION))
            .build()
            .map_err(network)?;
        let cache = Self {
            dir,
            client,
            requests,
        };

        cache.prune(MAX_AGE).await;

        Ok(cache)
    }

    pub async fn fetch(&self, url: &str) -> Result<PathBuf> {
        let url = Url::parse(url)
            .ok()
            .filter(trusted)
            .ok_or_else(|| Error::Notification("picture is not on Twitch's CDN".into()))?;
        let path = self.path_for(&url);

        if tokio::fs::try_exists(&path).await.unwrap_or(false) {
            return Ok(path);
        }

        let bytes = self.download(url).await?;

        store(&path, &bytes).await?;

        Ok(path)
    }

    fn path_for(&self, url: &Url) -> PathBuf {
        let hash = hex::encode(Sha256::digest(url.as_str().as_bytes()));

        self.dir.join(format!("{hash}.{}", extension(url)))
    }

    async fn download(&self, url: Url) -> Result<Vec<u8>> {
        let pending = self.requests.start(RequestKind::Picture, url_target(&url));
        let sent = self.client.get(url).send().await;

        pending.sent(&sent);

        let mut response = sent.map_err(network)?;

        if !response.status().is_success() {
            return Err(Error::Notification(format!(
                "picture answered {}",
                response.status().as_u16()
            )));
        }

        let image = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| value.starts_with("image/"));

        if !image {
            return Err(Error::Notification("picture is not an image".into()));
        }

        if response
            .content_length()
            .is_some_and(|length| length > MAX_BYTES as u64)
        {
            return Err(too_large());
        }

        let mut body = Vec::new();

        while let Some(chunk) = response.chunk().await.map_err(network)? {
            if body.len() + chunk.len() > MAX_BYTES {
                return Err(too_large());
            }

            body.extend_from_slice(&chunk);
        }

        Ok(body)
    }

    async fn prune(&self, max_age: Duration) {
        let Ok(mut entries) = tokio::fs::read_dir(&self.dir).await else {
            return;
        };
        let now = SystemTime::now();

        while let Ok(Some(entry)) = entries.next_entry().await {
            let expired = entry
                .metadata()
                .await
                .ok()
                .filter(|metadata| metadata.is_file())
                .and_then(|metadata| metadata.modified().ok())
                .and_then(|modified| now.duration_since(modified).ok())
                .is_some_and(|age| age > max_age);

            if expired {
                let _ = tokio::fs::remove_file(entry.path()).await;
            }
        }
    }
}

fn extension(url: &Url) -> &'static str {
    let name = url
        .path_segments()
        .and_then(|mut segments| segments.next_back())
        .unwrap_or_default();
    let found = name
        .rsplit_once('.')
        .map(|(_, extension)| extension.to_ascii_lowercase());

    EXTENSIONS
        .into_iter()
        .find(|known| found.as_deref() == Some(*known))
        .unwrap_or("img")
}

async fn store(path: &Path, bytes: &[u8]) -> Result<()> {
    let partial = path.with_extension(format!("{}.part", crate::util::random_hex(4)));

    tokio::fs::write(&partial, bytes).await.map_err(disk)?;

    if let Err(error) = tokio::fs::rename(&partial, path).await {
        let _ = tokio::fs::remove_file(&partial).await;

        return Err(disk(error));
    }

    Ok(())
}

fn too_large() -> Error {
    Error::Notification(format!("picture is larger than {MAX_BYTES} bytes"))
}

fn network(error: reqwest::Error) -> Error {
    Error::Notification(format!("picture download failed: {}", Error::from(error)))
}

fn disk(error: std::io::Error) -> Error {
    Error::Notification(format!("picture cache: {error}"))
}
