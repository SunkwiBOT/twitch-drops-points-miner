pub mod auth;
pub mod channels;
pub mod chat;
pub mod drops;
pub mod operations;
pub mod playback;
pub mod points;
pub mod pubsub;
pub mod rewards;
pub mod search;
pub mod spade;

use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, AUTHORIZATION, ORIGIN, REFERER,
};
use reqwest::{Method, StatusCode};
use reqwest_websocket::Upgrade as _;
use serde_json::Value;
use tokio::sync::{RwLock, Semaphore};
use tokio_util::sync::CancellationToken;
use zeroize::Zeroizing;

use crate::error::{Error, NetworkErrorKind, Result};
use crate::requests::{address_target, RequestKind, RequestLog, SocketEvent};
use operations::Operation;

// ! Twitch for Android TV ("SMARTBOX"): its device code still works.
pub const CLIENT_ID: &str = "ue6666qo983tsx6so1t0vnawi233wa";
pub const CLIENT_ORIGIN: &str = "https://android.tv.twitch.tv";
pub const TV_USER_AGENT: &str = "Mozilla/5.0 (Linux; Android 7.1; Smart Box C1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36";
pub const WEB_USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36";

pub const GQL_URL: &str = "https://gql.twitch.tv/gql";
pub const OAUTH_URL: &str = "https://id.twitch.tv/oauth2";
pub const USHER_URL: &str = "https://usher.ttvnw.net/api/channel/hls/";
pub const WEB_URL: &str = "https://www.twitch.tv/";
pub const PUBSUB_URL: &str = "wss://pubsub-edge.twitch.tv/v1";
pub const CHAT_URL: &str = "wss://irc-ws.chat.twitch.tv:443";

const REQUEST_TIMEOUT: Duration = Duration::from_secs(20);
const MAX_CONCURRENT: usize = 6;
// ! Below MAX_CONCURRENT, so the slots and claims still get a request in
// while balances, directories or redemptions are on their way.
pub const POINTS_READS_AT_ONCE: usize = 4;
pub const DIRECTORY_READS_AT_ONCE: usize = 4;
const GQL_ATTEMPTS: u32 = 3;
const GQL_BACKOFF: Duration = Duration::from_millis(400);
const PROXY_TEST_TIMEOUT: Duration = Duration::from_secs(15);
pub const SOCKET_WARN_AFTER_TRIES: u32 = 3;
const SOCKET_CONNECT_TIMEOUT: Duration = Duration::from_secs(20);

pub type Socket = reqwest_websocket::WebSocket;

pub struct SocketPlace {
    pub kind: RequestKind,
    pub address: &'static str,
    pub name: &'static str,
}

pub struct TwitchHttp {
    pub client: reqwest::Client,
    pub device_id: String,
    pub proxied: bool,
    pub requests: RequestLog,
    concurrent: Semaphore,
}

impl TwitchHttp {
    pub fn new(device_id: String, proxy: Option<&str>, requests: RequestLog) -> Result<Self> {
        let mut headers = HeaderMap::new();

        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US"));

        let mut builder = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent(TV_USER_AGENT)
            .timeout(REQUEST_TIMEOUT)
            .gzip(true);
        let proxy = proxy.map(str::trim).filter(|p| !p.is_empty());

        if let Some(proxy) = proxy {
            builder = builder.proxy(parse_proxy(proxy)?);
        }

        let client = builder.build().map_err(|error| {
            tracing::warn!(%error, "the HTTP client could not be built");

            Error::Transport(NetworkErrorKind::Other)
        })?;

        Ok(Self {
            client,
            device_id,
            proxied: proxy.is_some(),
            requests,
            concurrent: Semaphore::new(MAX_CONCURRENT),
        })
    }

    pub async fn permit(&self) -> tokio::sync::SemaphorePermit<'_> {
        self.concurrent
            .acquire()
            .await
            .expect("semaphore is never closed")
    }

    pub fn request(&self, method: Method, url: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, url)
            .header("Client-Id", CLIENT_ID)
            .header("X-Device-Id", &self.device_id)
            .header(ORIGIN, CLIENT_ORIGIN)
            .header(REFERER, format!("{CLIENT_ORIGIN}/"))
    }

    pub async fn post_gql(
        &self,
        account_id: Option<&str>,
        operation: &str,
        token: Option<&str>,
        body: &Value,
    ) -> Result<Value> {
        self.post_gql_leaving(account_id, operation, token, body, || true)
            .await
    }

    // ! `leaving` runs once the request has its slot: from there it may reach
    // Twitch, before that it never left. `false` keeps it from leaving at all.
    pub async fn post_gql_leaving(
        &self,
        account_id: Option<&str>,
        operation: &str,
        token: Option<&str>,
        body: &Value,
        leaving: impl FnOnce() -> bool,
    ) -> Result<Value> {
        let _permit = self.permit().await;

        if !leaving() {
            return Err(Error::InvalidRequest("held back before it left".to_owned()));
        }

        let pending = self
            .requests
            .start(RequestKind::Gql, operation)
            .account(account_id);
        let mut request = self.request(Method::POST, GQL_URL).json(body);

        if let Some(token) = token {
            request = request.header(AUTHORIZATION, format!("OAuth {token}"));
        }

        let response = match request.send().await {
            Ok(response) => response,
            Err(error) => {
                let error = Error::from(error);

                pending.failed(&error);

                return Err(error);
            }
        };
        let status = response.status();

        if !status.is_success() {
            pending.answered(status);

            return Err(Error::Status(status));
        }

        match response.json::<Value>().await {
            Ok(answer) => {
                pending.graphql(status, &answer, gql_refusal(&answer).is_some());

                Ok(answer)
            }
            Err(error) => {
                let error = Error::from(error);

                pending.failed(&error);

                Err(error)
            }
        }
    }
}

#[derive(Clone)]
pub struct TwitchClient {
    pub http: Arc<TwitchHttp>,
    pub user_id: String,
    pub login: String,
    token: Arc<RwLock<Zeroizing<String>>>,
}

#[derive(Debug, PartialEq, Eq)]
enum StatusOutcome {
    Retry(Error),
    Fail(Error),
}

impl TwitchClient {
    pub fn new(
        http: Arc<TwitchHttp>,
        user_id: String,
        login: String,
        access_token: String,
    ) -> Self {
        Self {
            http,
            user_id,
            login,
            token: Arc::new(RwLock::new(Zeroizing::new(access_token))),
        }
    }

    pub async fn token(&self) -> String {
        self.token.read().await.as_str().to_owned()
    }

    pub async fn set_token(&self, token: String) {
        *self.token.write().await = Zeroizing::new(token);
    }

    pub async fn gql(&self, operation: Operation, variables: Value) -> Result<Value> {
        self.send_gql(operation, &operation.request(variables))
            .await
    }

    // ! A mutation that spends points goes once: sent again after a lost
    // answer, Twitch could take it twice.
    pub async fn gql_once(
        &self,
        operation: Operation,
        variables: Value,
        leaving: impl FnOnce() -> bool,
    ) -> Result<Value> {
        let token = self.token.read().await.clone();
        let sent = self
            .http
            .post_gql_leaving(
                Some(&self.user_id),
                operation.name(),
                Some(&token),
                &operation.request(variables),
                leaving,
            )
            .await;

        match sent {
            Ok(answer) => check_gql_errors(operation, answer),
            Err(Error::Status(status)) => match status_outcome(status) {
                StatusOutcome::Retry(error) | StatusOutcome::Fail(error) => Err(error),
            },
            Err(error) => Err(error),
        }
    }

    pub async fn gql_batch(
        &self,
        operation: Operation,
        variables: impl IntoIterator<Item = Value>,
    ) -> Result<Value> {
        let batch: Vec<Value> = variables
            .into_iter()
            .map(|variables| operation.request(variables))
            .collect();

        self.send_gql(operation, &Value::Array(batch)).await
    }

    async fn send_gql(&self, operation: Operation, body: &Value) -> Result<Value> {
        let token = self.token.read().await.clone();
        let mut attempt = 1;

        loop {
            // ! One permit per attempt, given back before the pause: an account
            // backing off must not sit on the slots the others share.
            let sent = self
                .http
                .post_gql(Some(&self.user_id), operation.name(), Some(&token), body)
                .await;
            let retried = match sent {
                Ok(answer) => match check_gql_errors(operation, answer) {
                    Err(error) if busy(&error) => error,
                    answer => return answer,
                },
                Err(Error::Status(status)) => match status_outcome(status) {
                    StatusOutcome::Retry(error) => error,
                    StatusOutcome::Fail(error) => return Err(error),
                },
                Err(error) => error,
            };

            if attempt == GQL_ATTEMPTS {
                return Err(retried);
            }

            tokio::time::sleep(GQL_BACKOFF * attempt).await;
            attempt += 1;
        }
    }
}

// ! Only 401 says the token is dead. Twitch, a proxy or Cloudflare also answer
// 403 for an integrity check, a proxy address or a disguised rate limit: that
// passes, so it is retried like a busy server.
fn status_outcome(status: StatusCode) -> StatusOutcome {
    if status == StatusCode::UNAUTHORIZED {
        return StatusOutcome::Fail(Error::Unauthorized);
    }

    if retried_status(status) {
        return StatusOutcome::Retry(Error::Status(status));
    }

    StatusOutcome::Fail(Error::Status(status))
}

const SERVICE_ERRORS: [&str; 2] = ["service error", "service timeout"];

// ! GraphQL errors arrive with status 200. Persisted-query misses mean a
// hash is outdated; integrity failures mean the operation needs the web client.
fn gql_refusal(body: &Value) -> Option<&'static str> {
    let errors: Vec<&Value> = match body {
        Value::Array(items) => items.iter().filter_map(|v| v.get("errors")).collect(),
        _ => body.get("errors").into_iter().collect(),
    };

    errors
        .into_iter()
        .filter_map(Value::as_array)
        .flatten()
        .find_map(|error| {
            let message = error["message"].as_str().unwrap_or("");

            if message.contains("PersistedQueryNotFound") {
                return Some("persisted query outdated");
            }

            if message.contains("integrity") {
                return Some("integrity check failed");
            }

            SERVICE_ERRORS
                .iter()
                .find(|service| message.contains(*service))
                .copied()
        })
}

fn check_gql_errors(operation: Operation, body: Value) -> Result<Value> {
    match gql_refusal(&body) {
        Some(code) => Err(Error::Gql {
            operation: operation.name(),
            code: code.to_owned(),
        }),
        None => Ok(body),
    }
}

fn retried_status(status: StatusCode) -> bool {
    status == StatusCode::FORBIDDEN
        || status == StatusCode::TOO_MANY_REQUESTS
        || status.is_server_error()
}

fn busy(error: &Error) -> bool {
    matches!(error, Error::Gql { code, .. } if SERVICE_ERRORS.contains(&code.as_str()))
}

pub fn transient(error: &Error) -> bool {
    match error {
        Error::Status(status) => retried_status(*status),
        Error::Transport(kind) => *kind == NetworkErrorKind::Timeout,
        other => busy(other),
    }
}

// ! The messages never quote the URL: it can carry the proxy password.
pub fn parse_proxy(url: &str) -> Result<reqwest::Proxy> {
    let parsed =
        url::Url::parse(url).map_err(|_| Error::InvalidProxy("the proxy is not a URL".into()))?;

    if !matches!(
        parsed.scheme(),
        "http" | "https" | "socks5" | "socks5h" | "socks4" | "socks4a"
    ) {
        return Err(Error::InvalidProxy(format!(
            "unsupported proxy scheme: {}",
            parsed.scheme()
        )));
    }

    if parsed.host_str().is_none() {
        return Err(Error::InvalidProxy("proxy URL has no host".into()));
    }

    reqwest::Proxy::all(parsed.as_str())
        .map_err(|_| Error::InvalidProxy("the HTTP client refused the proxy".into()))
}

pub async fn test_proxy(proxy: &str, requests: RequestLog) -> Result<Duration> {
    let http = TwitchHttp::new(crate::util::random_device_id(), Some(proxy), requests)?;
    let address = format!("{OAUTH_URL}/validate");
    let started = std::time::Instant::now();
    let response = http
        .requests
        .start(RequestKind::ProxyTest, address_target(&address))
        .expecting(&[StatusCode::UNAUTHORIZED])
        .send(http.client.get(&address).timeout(PROXY_TEST_TIMEOUT))
        .await?;
    let status = response.status();

    // ! 401 is the expected answer without a token: the proxy reached Twitch.
    if status != StatusCode::UNAUTHORIZED && !status.is_success() {
        return Err(Error::Status(status));
    }

    Ok(started.elapsed())
}

pub async fn open_socket(
    client: &TwitchClient,
    place: &SocketPlace,
    event: SocketEvent,
) -> Result<Socket> {
    let name = place.name;
    let pending = client
        .http
        .requests
        .start(place.kind, address_target(place.address))
        .account(Some(&client.user_id))
        .socket(event);
    let sent = tokio::time::timeout(
        SOCKET_CONNECT_TIMEOUT,
        client.http.client.get(place.address).upgrade().send(),
    )
    .await;
    let response = match sent {
        Ok(Ok(response)) => response,
        Ok(Err(error)) => {
            let error = Error::Network(format!("{name} connect: {error}"));

            pending.failed(&error);

            return Err(error);
        }
        Err(_) => {
            pending.failed(&Error::Transport(NetworkErrorKind::Timeout));

            return Err(Error::Network(format!("{name} connect timeout")));
        }
    };
    let status = response.status();

    match response.into_websocket().await {
        Ok(socket) => {
            pending.succeeded(Some(status));

            Ok(socket)
        }
        Err(error) => {
            let error = Error::Network(format!("{name} upgrade: {error}"));

            if status == StatusCode::SWITCHING_PROTOCOLS {
                pending.failed(&error);
            } else {
                pending.answered(status);
            }

            Err(error)
        }
    }
}

pub async fn hold_socket(
    client: &TwitchClient,
    place: &SocketPlace,
    cancel: &CancellationToken,
    session: impl Future<Output = Result<()>>,
) -> Option<Result<()>> {
    let close = client
        .http
        .requests
        .start(place.kind, address_target(place.address))
        .account(Some(&client.user_id))
        .socket(SocketEvent::Close);
    let result = tokio::select! { biased;
        _ = cancel.cancelled() => None,
        result = session => Some(result),
    };

    match &result {
        Some(Err(Error::Network(reason))) => close.closed(reason),
        Some(Err(error)) => close.closed(&error.to_string()),
        _ => close.succeeded(None),
    }

    result
}

pub fn values(value: &Value) -> impl Iterator<Item = &Value> {
    value.as_array().into_iter().flatten()
}

pub(crate) fn note_left_out(list: &'static str, total: usize, kept: usize) {
    if kept < total {
        tracing::debug!(list, left_out = total - kept, "items of an answer left out");
    }
}

pub fn text(value: &Value) -> Option<String> {
    value.as_str().filter(|v| !v.is_empty()).map(str::to_owned)
}
