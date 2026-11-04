// ! Twitch credits watch time (and drop minutes) when the player fetches the
// HLS playlist and its segments. A HEAD request per segment is enough, so no
// video is downloaded.

use std::collections::VecDeque;
use std::time::Duration;

use reqwest::{Method, StatusCode};
use serde_json::json;
use url::Url;

use super::operations::Operation;
use super::{TwitchClient, USHER_URL, WEB_USER_AGENT};
use crate::error::{Error, NetworkErrorKind, Result};
use crate::requests::{host_target, url_target, RequestKind};

pub const POLL_INTERVAL: Duration = Duration::from_secs(15);
const POLL_TIMEOUT: Duration = Duration::from_secs(25);
const MAX_SEGMENTS: usize = 256;
const MAX_PLAYLIST: usize = 512 * 1024;
// ! A segment refused with one of these was signed by a playback token that
// expired: the next poll reads the playlist again with a new one. Only a
// playlist or a segment gone (`NOT_FOUND`) says the stream ended.
const SEGMENT_REFUSED: [StatusCode; 2] = [StatusCode::UNAUTHORIZED, StatusCode::FORBIDDEN];

pub struct Playback {
    pub channel_id: String,
    pub login: String,
    pub broadcast_id: String,
    playlist: Option<Url>,
    seen: VecDeque<Url>,
    pub segments_requested: u64,
}

impl Playback {
    pub fn new(channel_id: &str, login: &str, broadcast_id: &str) -> Self {
        Self {
            channel_id: channel_id.to_owned(),
            login: login.to_owned(),
            broadcast_id: broadcast_id.to_owned(),
            playlist: None,
            seen: VecDeque::new(),
            segments_requested: 0,
        }
    }

    fn remember(&mut self, segment: Url) {
        if self.seen.contains(&segment) {
            return;
        }

        if self.seen.len() == MAX_SEGMENTS {
            self.seen.pop_front();
        }

        self.seen.push_back(segment);
    }

    pub async fn poll(&mut self, client: &TwitchClient) -> Result<usize> {
        tokio::time::timeout(POLL_TIMEOUT, self.poll_inner(client))
            .await
            .unwrap_or(Err(Error::Transport(NetworkErrorKind::Timeout)))
    }

    async fn poll_inner(&mut self, client: &TwitchClient) -> Result<usize> {
        if self.playlist.is_none() {
            self.playlist = Some(self.resolve_playlist(client).await?);
        }

        let playlist = self.playlist.clone().expect("set above");
        let body = match fetch(client, Method::GET, playlist.clone()).await {
            Ok(body) => body,
            Err(error) => {
                self.playlist = None;

                return Err(error);
            }
        };
        let segments = match parse_media_playlist(&body, &playlist) {
            Ok(segments) => segments,
            Err(error) => {
                self.playlist = None;

                return Err(error);
            }
        };
        let mut requested = 0;

        for segment in segments {
            if self.seen.contains(&segment) {
                continue;
            }

            match fetch(client, Method::HEAD, segment.clone()).await {
                Ok(_) => {
                    requested += 1;
                    self.segments_requested += 1;
                    self.remember(segment);
                }
                Err(Error::Status(status))
                    if status == StatusCode::NOT_FOUND || SEGMENT_REFUSED.contains(&status) =>
                {
                    self.playlist = None;

                    return Err(Error::Status(status));
                }
                Err(error) => return Err(error),
            }
        }

        Ok(requested)
    }

    async fn resolve_playlist(&self, client: &TwitchClient) -> Result<Url> {
        let response = client
            .gql(Operation::PlaybackAccessToken, json!({"login": self.login}))
            .await?;
        let token = &response["data"]["streamPlaybackAccessToken"];
        let value = token["value"]
            .as_str()
            .filter(|v| !v.is_empty())
            .ok_or_else(|| Error::InvalidResponse("playback token missing".into()))?;
        let signature = token["signature"]
            .as_str()
            .filter(|v| !v.is_empty())
            .ok_or_else(|| Error::InvalidResponse("playback signature missing".into()))?;
        let mut master = Url::parse(&format!("{USHER_URL}{}.m3u8", self.login))
            .map_err(|_| Error::InvalidResponse("usher url".into()))?;

        master
            .query_pairs_mut()
            .append_pair("sig", signature)
            .append_pair("token", value)
            .append_pair("allow_source", "true")
            .append_pair("player_backend", "mediaplayer")
            .append_pair("playlist_include_framerate", "true")
            .append_pair("fast_bread", "true")
            .append_pair(
                "p",
                &format!("{}", rand::random::<u32>() % 9_000_000 + 1_000_000),
            );

        let body = fetch(client, Method::GET, master.clone()).await?;
        let variants = parse_master_playlist(&body, &master)?;

        // ! Twitch lists the variants best quality first: the last is the lowest.
        variants
            .into_iter()
            .last()
            .ok_or_else(|| Error::InvalidResponse("master playlist has no variant".into()))
    }
}

fn trusted(url: &Url) -> bool {
    url.scheme() == "https"
        && url.username().is_empty()
        && url.password().is_none()
        && url.host_str().is_some_and(|host| {
            ["ttvnw.net", "twitch.tv", "jtvnw.net"]
                .iter()
                .any(|suffix| host == *suffix || host.ends_with(&format!(".{suffix}")))
        })
}

async fn fetch(client: &TwitchClient, method: Method, url: Url) -> Result<String> {
    if !trusted(&url) {
        return Err(Error::InvalidResponse("untrusted playback url".into()));
    }

    let head = method == Method::HEAD;
    let _permit = client.http.permit().await;
    let pending = if head {
        client
            .http
            .requests
            .start(RequestKind::Segment, host_target(&url))
    } else {
        client
            .http
            .requests
            .start(RequestKind::Playlist, url_target(&url))
    };
    let response = pending
        .account(Some(&client.user_id))
        .send(
            client
                .http
                .client
                .request(method, url)
                .header(reqwest::header::USER_AGENT, WEB_USER_AGENT)
                .header(reqwest::header::ACCEPT, "*/*")
                .timeout(Duration::from_secs(if head { 5 } else { 8 })),
        )
        .await?;
    let status = response.status();

    if !status.is_success() {
        return Err(Error::Status(status));
    }

    if head {
        return Ok(String::new());
    }

    if response
        .content_length()
        .is_some_and(|len| len > MAX_PLAYLIST as u64)
    {
        return Err(Error::InvalidResponse("playlist too large".into()));
    }

    let body = response.text().await?;

    if body.len() > MAX_PLAYLIST {
        return Err(Error::InvalidResponse("playlist too large".into()));
    }

    Ok(body)
}

pub fn parse_master_playlist(body: &str, base: &Url) -> Result<Vec<Url>> {
    if !body.trim_start().starts_with("#EXTM3U") {
        return Err(Error::InvalidResponse("not an m3u8".into()));
    }

    let mut urls = Vec::new();
    let mut expect_url = false;

    for line in body.lines().map(str::trim) {
        if line.starts_with("#EXT-X-STREAM-INF") {
            expect_url = true;
        } else if expect_url && !line.is_empty() && !line.starts_with('#') {
            expect_url = false;

            if let Ok(url) = base.join(line) {
                if trusted(&url) {
                    urls.push(url);
                }
            }
        }
    }

    Ok(urls)
}

pub fn parse_media_playlist(body: &str, base: &Url) -> Result<Vec<Url>> {
    if !body.trim_start().starts_with("#EXTM3U") {
        return Err(Error::InvalidResponse("not an m3u8".into()));
    }

    let mut urls = Vec::new();
    let mut expect_url = false;

    for line in body.lines().map(str::trim) {
        if line.starts_with("#EXTINF") {
            expect_url = true;
        } else if expect_url && !line.is_empty() && !line.starts_with('#') {
            expect_url = false;

            if let Ok(url) = base.join(line) {
                if trusted(&url) {
                    urls.push(url);
                }
            }
        }
    }

    Ok(urls)
}
