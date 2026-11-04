use std::fmt;

use chrono::{DateTime, Duration, Utc};
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use zeroize::{Zeroize, ZeroizeOnDrop};

use super::{TwitchHttp, CLIENT_ID, OAUTH_URL};
use crate::error::{Error, Result};
use crate::requests::{address_target, RequestKind};

const SCOPES: &str =
    "channel_read chat:read user_blocks_edit user_blocks_read user_follows_edit user_read";

const MASKED: &str = "[redacted]";

#[derive(Clone, Serialize, Deserialize)]
pub struct DeviceCode {
    #[serde(skip_serializing)]
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub interval_seconds: u64,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

// ! The secret half stays out of `{:?}`: the log redaction only catches it
// when it looks like a key.
impl fmt::Debug for DeviceCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeviceCode")
            .field("device_code", &MASKED)
            .field("user_code", &self.user_code)
            .field("verification_uri", &self.verification_uri)
            .field("interval_seconds", &self.interval_seconds)
            .field("issued_at", &self.issued_at)
            .field("expires_at", &self.expires_at)
            .finish()
    }
}

#[derive(Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct TokenSet {
    pub access_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

impl fmt::Debug for TokenSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TokenSet")
            .field("access_token", &MASKED)
            .field(
                "refresh_token",
                &self.refresh_token.as_ref().map(|_| MASKED),
            )
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Validation {
    pub user_id: String,
    pub login: String,
}

pub enum Poll {
    Pending,
    Token(TokenSet),
}

async fn oauth(
    http: &TwitchHttp,
    account_id: Option<&str>,
    path: &str,
    form: &[(&str, &str)],
    expected: &'static [StatusCode],
) -> Result<(StatusCode, Value)> {
    let address = format!("{OAUTH_URL}/{path}");
    let response = http
        .requests
        .start(RequestKind::Auth, address_target(&address))
        .account(account_id)
        .expecting(expected)
        .send(http.request(Method::POST, &address).form(form))
        .await?;
    let status = response.status();
    let body: Value = response.json().await.unwrap_or(Value::Null);

    Ok((status, body))
}

pub async fn start_device_flow(http: &TwitchHttp) -> Result<DeviceCode> {
    let (status, body) = oauth(
        http,
        None,
        "device",
        &[("client_id", CLIENT_ID), ("scopes", SCOPES)],
        &[],
    )
    .await?;

    if status != StatusCode::OK {
        let message = body["message"].as_str().unwrap_or("device code refused");

        return Err(Error::InvalidResponse(format!("{status}: {message}")));
    }

    let device_code = body["device_code"]
        .as_str()
        .filter(|v| !v.is_empty())
        .ok_or_else(|| Error::InvalidResponse("device_code missing".into()))?;
    let user_code = body["user_code"]
        .as_str()
        .filter(|v| !v.is_empty())
        .ok_or_else(|| Error::InvalidResponse("user_code missing".into()))?;
    let expires_in = body["expires_in"].as_i64().unwrap_or(1800).clamp(60, 3600);
    let issued_at = Utc::now();

    Ok(DeviceCode {
        device_code: device_code.to_owned(),
        user_code: user_code.to_owned(),
        verification_uri: activation_link(
            body["verification_uri"]
                .as_str()
                .unwrap_or("https://www.twitch.tv/activate"),
            user_code,
        ),
        interval_seconds: body["interval"].as_u64().unwrap_or(5).clamp(1, 60),
        issued_at,
        expires_at: issued_at + Duration::seconds(expires_in),
    })
}

pub fn activation_link(uri: &str, user_code: &str) -> String {
    if uri.contains("device-code=") {
        return uri.to_owned();
    }

    let separator = if uri.contains('?') { '&' } else { '?' };

    format!("{uri}{separator}device-code={user_code}")
}

pub async fn poll_device_flow(http: &TwitchHttp, code: &DeviceCode) -> Result<Poll> {
    if Utc::now() >= code.expires_at {
        return Err(Error::DeviceCodeExpired);
    }

    // ! Twitch answers 400 while the code waits for the user: not a failure.
    let (status, body) = oauth(
        http,
        None,
        "token",
        &[
            ("client_id", CLIENT_ID),
            ("device_code", &code.device_code),
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
        ],
        &[StatusCode::BAD_REQUEST],
    )
    .await?;

    if status == StatusCode::OK {
        return Ok(Poll::Token(parse_tokens(&body)?));
    }

    if status != StatusCode::BAD_REQUEST {
        return Err(Error::Status(status));
    }

    let message = body["message"].as_str().unwrap_or("");

    if message.contains("expired") {
        Err(Error::DeviceCodeExpired)
    } else {
        Ok(Poll::Pending)
    }
}

fn parse_tokens(body: &Value) -> Result<TokenSet> {
    let access_token = body["access_token"]
        .as_str()
        .filter(|v| !v.is_empty())
        .ok_or_else(|| Error::InvalidResponse("access_token missing".into()))?;

    Ok(TokenSet {
        access_token: access_token.to_owned(),
        refresh_token: body["refresh_token"]
            .as_str()
            .filter(|v| !v.is_empty())
            .map(str::to_owned),
    })
}

pub async fn validate(
    http: &TwitchHttp,
    account_id: Option<&str>,
    access_token: &str,
) -> Result<Option<Validation>> {
    let address = format!("{OAUTH_URL}/validate");
    let response = http
        .requests
        .start(RequestKind::Auth, address_target(&address))
        .account(account_id)
        .send(
            http.request(Method::GET, &address)
                .header("Authorization", format!("OAuth {access_token}")),
        )
        .await?;
    let status = response.status();

    if status == StatusCode::UNAUTHORIZED {
        return Ok(None);
    }

    if status != StatusCode::OK {
        return Err(Error::Status(status));
    }

    let body: Value = response.json().await?;

    if body["client_id"].as_str() != Some(CLIENT_ID) {
        return Err(Error::Unauthorized);
    }

    Ok(Some(Validation {
        user_id: body["user_id"]
            .as_str()
            .ok_or_else(|| Error::InvalidResponse("user_id missing".into()))?
            .to_owned(),
        login: body["login"].as_str().unwrap_or_default().to_owned(),
    }))
}

pub async fn refresh(
    http: &TwitchHttp,
    account_id: Option<&str>,
    refresh_token: &str,
) -> Result<TokenSet> {
    let (status, body) = oauth(
        http,
        account_id,
        "token",
        &[
            ("client_id", CLIENT_ID),
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ],
        &[],
    )
    .await?;

    if status == StatusCode::OK {
        return parse_tokens(&body);
    }

    if status == StatusCode::BAD_REQUEST || status == StatusCode::UNAUTHORIZED {
        return Err(Error::Unauthorized);
    }

    Err(Error::Status(status))
}

pub async fn revoke(http: &TwitchHttp, account_id: Option<&str>, access_token: &str) {
    let _ = oauth(
        http,
        account_id,
        "revoke",
        &[("client_id", CLIENT_ID), ("token", access_token)],
        &[],
    )
    .await;
}
