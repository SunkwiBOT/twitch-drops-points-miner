// ! The `minute-watched` beacon the Twitch site sends every minute is what
// credits channel points; its endpoint lives in the site's settings script.

use base64::Engine;
use chrono::Utc;
use regex::Regex;
use reqwest::Method;
use serde_json::json;
use std::net::IpAddr;
use std::sync::OnceLock;
use url::Url;

use super::{TwitchClient, WEB_URL, WEB_USER_AGENT};
use crate::domain::Channel;
use crate::error::{Error, Result};
use crate::requests::{url_target, RequestKind};

fn settings_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();

    RE.get_or_init(|| {
        Regex::new(r#"(https://(?:static\.twitchcdn\.net|assets\.twitch\.tv)/config/settings\.[0-9a-f]+\.js)"#)
            .expect("valid regex")
    })
}

fn spade_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();

    RE.get_or_init(|| Regex::new(r#""spade_url":"([^"]+)""#).expect("valid regex"))
}

fn trusted(url: &Url) -> bool {
    url.scheme() == "https"
        && url.username().is_empty()
        && url.password().is_none()
        && url.host_str().is_some_and(|host| {
            ["twitch.tv", "jtvnw.net", "ttvnw.net", "twitchcdn.net"]
                .iter()
                .any(|suffix| host == *suffix || host.ends_with(&format!(".{suffix}")))
        })
}

async fn page(client: &TwitchClient, url: Url) -> Result<String> {
    if !trusted(&url) {
        return Err(Error::InvalidResponse("untrusted page url".into()));
    }

    let _permit = client.http.permit().await;
    let response = client
        .http
        .requests
        .start(RequestKind::ChannelPage, url_target(&url))
        .account(Some(&client.user_id))
        .send(
            client
                .http
                .client
                .request(Method::GET, url)
                .header(reqwest::header::USER_AGENT, WEB_USER_AGENT)
                .header(
                    reqwest::header::ACCEPT,
                    "text/html,application/xhtml+xml,*/*",
                ),
        )
        .await?;

    if !response.status().is_success() {
        return Err(Error::Status(response.status()));
    }

    Ok(response.text().await?)
}

pub async fn resolve_beacon(client: &TwitchClient, login: &str) -> Result<Url> {
    let url = Url::parse(WEB_URL)
        .and_then(|u| u.join(login))
        .map_err(|_| Error::InvalidResponse("channel url".into()))?;
    let html = page(client, url).await?;

    // ! Only a Twitch address is used, whether it was found in the page or in
    // its settings script.
    let beacon = match spade_regex().captures(&html) {
        Some(found) => {
            Url::parse(&found[1]).map_err(|_| Error::InvalidResponse("spade url".into()))?
        }
        None => beacon_from_settings(client, &html).await?,
    };

    if !trusted(&beacon) {
        return Err(Error::InvalidResponse("untrusted beacon".into()));
    }

    Ok(beacon)
}

async fn beacon_from_settings(client: &TwitchClient, html: &str) -> Result<Url> {
    let settings = settings_regex()
        .captures(html)
        .ok_or_else(|| Error::InvalidResponse("settings script not found".into()))?;
    let script_url =
        Url::parse(&settings[1]).map_err(|_| Error::InvalidResponse("settings url".into()))?;
    let script = page(client, script_url).await?;
    let found = spade_regex()
        .captures(&script)
        .ok_or_else(|| Error::InvalidResponse("spade_url not found".into()))?;

    Url::parse(&found[1]).map_err(|_| Error::InvalidResponse("spade url".into()))
}

pub async fn send_minute_watched(
    client: &TwitchClient,
    beacon: &Url,
    channel: &Channel,
) -> Result<bool> {
    let Some(broadcast_id) = &channel.broadcast_id else {
        return Ok(false);
    };
    let payload = json!([{
        "event": "minute-watched",
        "properties": {
            "broadcast_id": broadcast_id,
            "channel_id": channel.id,
            "channel": channel.login,
            "client_time": Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Micros, true),
            "game": channel.game.as_ref().map_or("", |g| g.name.as_str()),
            "game_id": channel.game.as_ref().map_or("", |g| g.id.as_str()),
            "hidden": false, "is_live": true, "live": true, "location": "channel",
            "logged_in": true, "minutes_logged": 1, "muted": false, "player": "site",
            "user_id": client.user_id.parse::<u64>().ok(),
        }
    }]);
    let data = base64::engine::general_purpose::STANDARD.encode(payload.to_string());

    let _permit = client.http.permit().await;
    let response = client
        .http
        .requests
        .start(RequestKind::Beacon, url_target(beacon))
        .account(Some(&client.user_id))
        .send(
            client
                .http
                .client
                .request(Method::POST, beacon.clone())
                .header(reqwest::header::USER_AGENT, WEB_USER_AGENT)
                .form(&[("data", data)]),
        )
        .await?;

    Ok(response.status() == reqwest::StatusCode::NO_CONTENT)
}

// ! DNS blockers answer with `0.0.0.0`, `::` or a loopback: when every
// address found is one, the name is blocked, not just down.
pub fn blocking_address(addresses: &[IpAddr]) -> Option<IpAddr> {
    let first = *addresses.first()?;

    addresses
        .iter()
        .all(|ip| ip.is_unspecified() || ip.is_loopback())
        .then_some(first)
}

pub async fn dns_block(beacon: &Url) -> Option<IpAddr> {
    let host = beacon.host_str()?;
    let found = tokio::net::lookup_host((host, 443)).await.ok()?;

    blocking_address(&found.map(|address| address.ip()).collect::<Vec<_>>())
}
