use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, LazyLock, Once};
use std::time::Duration;

use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::ws::{close_code, CloseFrame, Message, Utf8Bytes, WebSocket, WebSocketUpgrade};
use axum::extract::{ConnectInfo, FromRequest, FromRequestParts, Request, State};
use axum::http::request::Parts;
use axum::http::{header, HeaderMap, HeaderName, HeaderValue, StatusCode, Uri};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use miner_core::api::{
    ApiRoute, AuthResponse, AuthStatus, CatalogRefreshResponse, ChannelBannersRequest,
    ChannelBannersResponse, ChannelQuery, ChannelResponse, ChannelsLookupRequest,
    ChannelsLookupResponse, Command, DashboardLoginRequest, FollowsQuery, FollowsResponse,
    HistoryQuery, HistoryResponse, LoginFlowResponse, LogsQuery, LogsReportResponse, LogsResponse,
    NotificationTestResponse, ProxyTestRequest, ProxyTestResponse, RedeemRequest, RedeemResponse,
    RequestsQuery, RequestsResponse, SearchQuery, SearchResponse, ServerMessage, SnapshotResponse,
    SpamBalanceResponse, SpamChannelQuery, SpamChannelResponse, UpdateCheck, UpdateCheckResponse,
};
use miner_core::error::{ApiError, ErrorCode};
use miner_core::Manager;
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use tower_http::set_header::SetResponseHeaderLayer;

use crate::access::Access;
use crate::attempts::{SignIn, SignInAttempts};
use crate::feed::SnapshotFeed;
use crate::sessions::{SessionLease, Sessions, SESSION_MAX_AGE};
use crate::updates::Releases;

const BODY_LIMIT_BYTES: usize = 256 * 1024;
const WRONG_PASSWORD_DELAY: Duration = Duration::from_millis(500);
// ! The desktop window's policy (`tauri.conf.json`) without its `ipc:` sources,
// plus the push channel's websocket: the two change together.
const CONTENT_SECURITY_POLICY: &str = "default-src 'self'; img-src 'self' data: https://static-cdn.jtvnw.net https://*.jtvnw.net; style-src 'self' 'unsafe-inline'; font-src 'self'; connect-src 'self' {connect}; object-src 'none'; base-uri 'self'; form-action 'self'; frame-ancestors 'none'";
// ! A local dashboard is reached by whatever address the network gives it.
const LOCAL_WEBSOCKETS: &str = "ws: wss:";
const SECURE_WEBSOCKET_SCHEME: &str = "wss";
const STRICT_TRANSPORT_SECURITY: &str = "max-age=31536000";
const API_CACHE_CONTROL: &str = "no-store";
const PERMISSIONS_POLICY: &str = "camera=(), microphone=(), geolocation=(), payment=(), usb=()";
const PERMISSIONS_POLICY_HEADER: HeaderName = HeaderName::from_static("permissions-policy");
const SESSION_ENDED_REASON: &str = "session ended";
// ! A client that stops reading would otherwise hold its push channel, and
// the end of its session, open.
const SEND_TIMEOUT: Duration = Duration::from_secs(10);
const INCOMING_MESSAGE_MAX_BYTES: usize = 4 * 1024;

static PLAIN_HTTP_WARNING: Once = Once::new();
static UNKNOWN_VISITOR_WARNING: Once = Once::new();
static DIRECT_VISITOR_WARNING: Once = Once::new();
static CROSS_SITE_NOTICE: Once = Once::new();

#[derive(rust_embed::Embed)]
#[folder = "$CARGO_MANIFEST_DIR/../ui/dist/"]
struct Assets;

static UI_BUILD: LazyLock<Option<String>> = LazyLock::new(|| {
    Assets::get("index.html").map(|file| hex::encode(&file.metadata.sha256_hash()[..8]))
});

pub fn ui_build() -> Option<String> {
    UI_BUILD.clone()
}

pub struct AppState {
    pub manager: Arc<Manager>,
    pub snapshots: Arc<SnapshotFeed>,
    pub access: Access,
    pub sessions: Sessions,
    pub attempts: SignInAttempts,
    pub releases: Releases,
}

pub type Shared = Arc<AppState>;

fn content_security_policy(access: &Access) -> HeaderValue {
    let connect = match access {
        Access::Local { .. } => LOCAL_WEBSOCKETS.to_owned(),
        Access::Public { url, .. } => {
            let mut websocket = url.clone();
            let _ = websocket.set_scheme(SECURE_WEBSOCKET_SCHEME);

            websocket.origin().ascii_serialization()
        }
    };

    HeaderValue::from_str(&CONTENT_SECURITY_POLICY.replace("{connect}", &connect))
        .expect("the public address is a valid header value")
}

pub fn router(state: Shared) -> Router {
    let policy = content_security_policy(&state.access);
    let public = state.access.is_public();
    let protected = Router::new()
        .route(ApiRoute::Activity.path(), post(activity))
        .route(ApiRoute::Snapshot.path(), get(snapshot))
        .route(ApiRoute::Command.path(), post(command))
        .route(ApiRoute::CatalogRefresh.path(), post(refresh_catalog))
        .route(ApiRoute::UpdatesCheck.path(), post(check_for_update))
        .route(ApiRoute::LoginFlow.path(), post(start_login))
        .route(ApiRoute::Channel.path(), get(lookup_channel))
        .route(ApiRoute::ChannelsLookup.path(), post(lookup_channels))
        .route(ApiRoute::Search.path(), get(search))
        .route(ApiRoute::ChannelBanners.path(), post(channel_banners))
        .route(ApiRoute::Follows.path(), get(follows))
        .route(ApiRoute::NotificationsTest.path(), post(test_notifications))
        .route(ApiRoute::ProxyTest.path(), post(test_proxy))
        .route(ApiRoute::History.path(), get(history))
        .route(ApiRoute::Logs.path(), get(logs))
        .route(ApiRoute::LogsReport.path(), get(logs_report))
        .route(ApiRoute::Requests.path(), get(requests))
        .route(ApiRoute::RequestsReport.path(), get(requests_report))
        .route(ApiRoute::Events.path(), get(events))
        .route(ApiRoute::SpamChannel.path(), get(spam_channel))
        .route(ApiRoute::SpamBalance.path(), get(spam_balance))
        .route(ApiRoute::SpamRedeem.path(), post(redeem))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_session,
        ));

    let router = Router::new()
        .route(
            ApiRoute::Auth.path(),
            get(auth_status).post(login).delete(logout),
        )
        .merge(protected)
        .route_layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static(API_CACHE_CONTROL),
        ))
        .fallback(static_file)
        .layer(middleware::from_fn_with_state(state.clone(), admit))
        // ! Added after the admission check, which only wraps the routes above:
        // Docker's health check calls it over plain HTTP from inside the container.
        .route(
            ApiRoute::Health.path(),
            get(|| async { StatusCode::NO_CONTENT }),
        )
        .layer(axum::extract::DefaultBodyLimit::max(BODY_LIMIT_BYTES))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::REFERRER_POLICY,
            HeaderValue::from_static("no-referrer"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-robots-tag"),
            HeaderValue::from_static("noindex, nofollow"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::CONTENT_SECURITY_POLICY,
            policy,
        ))
        .layer(SetResponseHeaderLayer::overriding(
            PERMISSIONS_POLICY_HEADER,
            HeaderValue::from_static(PERMISSIONS_POLICY),
        ))
        .layer(tower_http::compression::CompressionLayer::new())
        .with_state(state);

    if public {
        return router.layer(SetResponseHeaderLayer::overriding(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static(STRICT_TRANSPORT_SECURITY),
        ));
    }

    router
}

fn status_of(code: ErrorCode) -> StatusCode {
    match code {
        ErrorCode::Unauthorized | ErrorCode::WrongPassword => StatusCode::UNAUTHORIZED,
        ErrorCode::WrongPassphrase
        | ErrorCode::HttpsRequired
        | ErrorCode::ForwardedForRequired
        | ErrorCode::ProxyNotLocal
        | ErrorCode::CrossSiteRequest
        | ErrorCode::UpdateSignatureRefused => StatusCode::FORBIDDEN,
        ErrorCode::InvalidRequest => StatusCode::BAD_REQUEST,
        ErrorCode::NotFound
        | ErrorCode::UnknownAccount
        | ErrorCode::UnknownChannel
        | ErrorCode::UnknownReward => StatusCode::NOT_FOUND,
        ErrorCode::VaultLocked => StatusCode::LOCKED,
        ErrorCode::PassphraseRequired
        | ErrorCode::RewardNotRedeemable
        | ErrorCode::BurstRunning
        | ErrorCode::DeviceCodePending
        | ErrorCode::DeviceCodeExpired => StatusCode::CONFLICT,
        ErrorCode::InvalidProxy | ErrorCode::InvalidStreamerLogin => {
            StatusCode::UNPROCESSABLE_ENTITY
        }
        ErrorCode::NetworkError
        | ErrorCode::TwitchUnavailable
        | ErrorCode::TwitchRejected
        | ErrorCode::InvalidResponse
        | ErrorCode::SessionExpired
        | ErrorCode::NotificationFailed
        | ErrorCode::BeaconBlocked => StatusCode::BAD_GATEWAY,
        ErrorCode::CatalogRateLimited | ErrorCode::TooManyAttempts => StatusCode::TOO_MANY_REQUESTS,
        ErrorCode::NotSupported => StatusCode::NOT_IMPLEMENTED,
        ErrorCode::StorageError
        | ErrorCode::InvalidVault
        | ErrorCode::UpdateInstallFailed
        | ErrorCode::ServerUnreachable
        | ErrorCode::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub struct Failure(ApiError);

impl From<ErrorCode> for Failure {
    fn from(code: ErrorCode) -> Self {
        Self(ApiError::new(code))
    }
}

impl From<miner_core::Error> for Failure {
    fn from(error: miner_core::Error) -> Self {
        Self(ApiError::from(error))
    }
}

impl IntoResponse for Failure {
    fn into_response(self) -> Response {
        (status_of(self.0.code), Json(self.0)).into_response()
    }
}

pub struct Body<T>(pub T);

impl<S, T> FromRequest<S> for Body<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Failure;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(request, state).await {
            Ok(Json(value)) => Ok(Self(value)),
            Err(rejection) => Err(invalid_request(&rejection)),
        }
    }
}

pub struct Params<T>(pub T);

impl<S, T> FromRequestParts<S> for Params<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Failure;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Query::<T>::from_request_parts(parts, state).await {
            Ok(axum::extract::Query(value)) => Ok(Self(value)),
            Err(rejection) => Err(invalid_query(&rejection)),
        }
    }
}

fn invalid_request(rejection: &JsonRejection) -> Failure {
    tracing::debug!("request body refused: {rejection}");
    ErrorCode::InvalidRequest.into()
}

fn invalid_query(rejection: &QueryRejection) -> Failure {
    tracing::debug!("query refused: {rejection}");
    ErrorCode::InvalidRequest.into()
}

#[derive(Clone, Copy)]
struct ClientAddress(IpAddr);

fn cookie_session(headers: &HeaderMap, name: &str) -> Option<String> {
    let prefix = format!("{name}=");
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(';'))
        .map(str::trim)
        .find_map(|pair| pair.strip_prefix(&prefix).map(str::to_owned))
}

fn bearer(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
}

// ! Hashed first: two values of one length, so the time taken tells nothing,
// not even the password's length.
fn password_matches(expected: &str, given: &str) -> bool {
    let expected: [u8; 32] = Sha256::digest(expected.as_bytes()).into();
    let given: [u8; 32] = Sha256::digest(given.as_bytes()).into();

    expected.ct_eq(&given).into()
}

enum Verdict {
    Allowed(Option<SessionLease>),
    Refused,
    LockedOut(Duration),
}

async fn authorize(state: &AppState, headers: &HeaderMap, client: IpAddr) -> Verdict {
    // ! A bearer password is a sign-in try like the form's, or scripts would
    // be a way around the lockout.
    if let Some(given) = bearer(headers) {
        let matches = password_matches(state.access.password(), given);

        return match state.attempts.attempt(client, matches) {
            SignIn::Accepted => Verdict::Allowed(None),
            SignIn::Refused => {
                tokio::time::sleep(WRONG_PASSWORD_DELAY).await;

                Verdict::Refused
            }
            SignIn::LockedOut { retry_after } => Verdict::LockedOut(retry_after),
        };
    }

    let Some(session) = cookie_session(headers, state.access.cookie_name()) else {
        return Verdict::Refused;
    };

    let idle_limit = state.manager.global_settings().await.auto_lock.idle_limit();

    match state.sessions.find(&session, idle_limit).await {
        Some(lease) => Verdict::Allowed(Some(lease)),
        None => Verdict::Refused,
    }
}

fn locked_out(retry_after: Duration) -> Response {
    let seconds = retry_after.as_millis().div_ceil(1000);

    (
        StatusCode::TOO_MANY_REQUESTS,
        [(header::RETRY_AFTER, seconds.to_string())],
        Json(ApiError::new(ErrorCode::TooManyAttempts)),
    )
        .into_response()
}

async fn admit(
    State(state): State<Shared>,
    ConnectInfo(socket): ConnectInfo<SocketAddr>,
    mut request: Request,
    next: Next,
) -> Response {
    match state.access.admit(request.headers(), socket.ip()) {
        Ok(_)
            if !state
                .access
                .allows_write(request.method(), request.headers()) =>
        {
            CROSS_SITE_NOTICE.call_once(|| {
                tracing::info!("refused a write sent by a page of another site or port");
            });

            Failure::from(ErrorCode::CrossSiteRequest).into_response()
        }
        Ok(client) => {
            request.extensions_mut().insert(ClientAddress(client));

            next.run(request).await
        }
        Err(code) => {
            warn_once(code);

            Failure::from(code).into_response()
        }
    }
}

fn warn_once(refusal: ErrorCode) {
    match refusal {
        ErrorCode::HttpsRequired => PLAIN_HTTP_WARNING.call_once(|| {
            tracing::warn!(
                "refused a request that did not arrive over HTTPS: open the dashboard at MINER_PUBLIC_URL, through a proxy that sends X-Forwarded-Proto"
            );
        }),
        ErrorCode::ProxyNotLocal => DIRECT_VISITOR_WARNING.call_once(|| {
            tracing::warn!(
                "refused a request from a public address that reached the port directly: with MINER_PUBLIC_URL, only the HTTPS proxy on this machine or its local network may connect"
            );
        }),
        _ => UNKNOWN_VISITOR_WARNING.call_once(|| {
            tracing::warn!(
                "refused a request without the visitor's address: the HTTPS proxy must send X-Forwarded-For (and MINER_TRUSTED_HOPS must not exceed the proxies that add to it)"
            );
        }),
    }
}

// ! Runs before any body is read, so an anonymous request never gets parsed.
async fn require_session(
    State(state): State<Shared>,
    Extension(ClientAddress(client)): Extension<ClientAddress>,
    mut request: Request,
    next: Next,
) -> Response {
    match authorize(&state, request.headers(), client).await {
        Verdict::Allowed(lease) => {
            if let Some(lease) = lease {
                request.extensions_mut().insert(lease);
            }

            next.run(request).await
        }
        Verdict::Refused => Failure::from(ErrorCode::Unauthorized).into_response(),
        Verdict::LockedOut(retry_after) => locked_out(retry_after),
    }
}

async fn auth_status(
    State(state): State<Shared>,
    Extension(ClientAddress(client)): Extension<ClientAddress>,
    headers: HeaderMap,
) -> Json<AuthResponse> {
    let verdict = authorize(&state, &headers, client).await;

    Json(AuthResponse {
        auth: AuthStatus {
            authorized: matches!(verdict, Verdict::Allowed(_)),
            public: state.access.is_public(),
        },
    })
}

async fn login(
    State(state): State<Shared>,
    Extension(ClientAddress(client)): Extension<ClientAddress>,
    headers: HeaderMap,
    Body(body): Body<DashboardLoginRequest>,
) -> Result<Response, Failure> {
    let matches = password_matches(state.access.password(), &body.password);

    match state.attempts.attempt(client, matches) {
        SignIn::Accepted => {}
        SignIn::Refused => {
            tokio::time::sleep(WRONG_PASSWORD_DELAY).await;

            return Err(ErrorCode::WrongPassword.into());
        }
        SignIn::LockedOut { retry_after } => return Ok(locked_out(retry_after)),
    }

    if let Some(previous) = cookie_session(&headers, state.access.cookie_name()) {
        state.sessions.revoke(&previous).await;
    }

    let session = state.sessions.create().await;
    let cookie = state.access.session_cookie(&session, SESSION_MAX_AGE);

    Ok((StatusCode::NO_CONTENT, [(header::SET_COOKIE, cookie)]).into_response())
}

async fn logout(State(state): State<Shared>, headers: HeaderMap) -> Response {
    if let Some(session) = cookie_session(&headers, state.access.cookie_name()) {
        state.sessions.revoke(&session).await;
    }

    let cleared = state.access.session_cookie("", Duration::ZERO);

    (StatusCode::NO_CONTENT, [(header::SET_COOKIE, cleared)]).into_response()
}

async fn activity(lease: Option<Extension<SessionLease>>) -> StatusCode {
    if let Some(Extension(lease)) = lease {
        lease.touch();
    }

    StatusCode::NO_CONTENT
}

async fn snapshot(State(state): State<Shared>) -> Result<Json<SnapshotResponse>, Failure> {
    let mut snapshot = state.manager.snapshot().await?;

    snapshot.build = ui_build();

    Ok(Json(SnapshotResponse { snapshot }))
}

async fn command(
    State(state): State<Shared>,
    Body(command): Body<Command>,
) -> Result<StatusCode, Failure> {
    state.manager.execute(command).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn start_login(State(state): State<Shared>) -> Result<Json<LoginFlowResponse>, Failure> {
    Ok(Json(LoginFlowResponse {
        login: state.manager.start_login().await?,
    }))
}

async fn lookup_channel(
    State(state): State<Shared>,
    Params(query): Params<ChannelQuery>,
) -> Result<Json<ChannelResponse>, Failure> {
    Ok(Json(ChannelResponse {
        channel: state.manager.lookup_channel(&query.login).await?,
    }))
}

async fn channel_banners(
    State(state): State<Shared>,
    Body(request): Body<ChannelBannersRequest>,
) -> Result<Json<ChannelBannersResponse>, Failure> {
    Ok(Json(state.manager.channel_banners(&request.logins).await?))
}

async fn search(
    State(state): State<Shared>,
    Params(query): Params<SearchQuery>,
) -> Result<Json<SearchResponse>, Failure> {
    Ok(Json(state.manager.search(&query.query).await?))
}

async fn lookup_channels(
    State(state): State<Shared>,
    Body(request): Body<ChannelsLookupRequest>,
) -> Result<Json<ChannelsLookupResponse>, Failure> {
    Ok(Json(state.manager.lookup_channels(&request.logins).await?))
}

async fn follows(
    State(state): State<Shared>,
    Params(query): Params<FollowsQuery>,
) -> Result<Json<FollowsResponse>, Failure> {
    Ok(Json(FollowsResponse {
        channels: state.manager.followed_channels(&query.account_id).await?,
    }))
}

async fn spam_channel(
    State(state): State<Shared>,
    Params(query): Params<SpamChannelQuery>,
) -> Result<Json<SpamChannelResponse>, Failure> {
    Ok(Json(SpamChannelResponse {
        channel: state
            .manager
            .spam_channel(&query.account_id, &query.login)
            .await?,
    }))
}

async fn spam_balance(
    State(state): State<Shared>,
    Params(query): Params<SpamChannelQuery>,
) -> Result<Json<SpamBalanceResponse>, Failure> {
    Ok(Json(SpamBalanceResponse {
        balance: state
            .manager
            .spam_balance(&query.account_id, &query.login)
            .await?,
    }))
}

// ! Answered once the burst is accepted: its result comes over the push
// channel, as a burst can outlast a proxy's read timeout.
async fn redeem(
    State(state): State<Shared>,
    Body(request): Body<RedeemRequest>,
) -> Result<(StatusCode, Json<RedeemResponse>), Failure> {
    Ok((
        StatusCode::ACCEPTED,
        Json(RedeemResponse {
            burst_id: state.manager.redeem(request).await?,
        }),
    ))
}

async fn test_notifications(State(state): State<Shared>) -> Json<NotificationTestResponse> {
    Json(NotificationTestResponse {
        targets: state.manager.test_notification_targets().await,
    })
}

async fn test_proxy(
    State(state): State<Shared>,
    Body(request): Body<ProxyTestRequest>,
) -> Result<Json<ProxyTestResponse>, Failure> {
    Ok(Json(state.manager.test_proxy(&request.url).await?))
}

async fn history(
    State(state): State<Shared>,
    Params(query): Params<HistoryQuery>,
) -> Result<Json<HistoryResponse>, Failure> {
    Ok(Json(state.manager.history_page(&query)?))
}

async fn refresh_catalog(
    State(state): State<Shared>,
) -> Result<Json<CatalogRefreshResponse>, Failure> {
    Ok(Json(CatalogRefreshResponse {
        refresh: state.manager.refresh_catalog().await?,
    }))
}

async fn check_for_update(
    State(state): State<Shared>,
) -> Result<Json<UpdateCheckResponse>, Failure> {
    let available = state.releases.check(&state.manager).await?;

    Ok(Json(UpdateCheckResponse {
        check: UpdateCheck { available },
    }))
}

async fn logs(State(state): State<Shared>, Params(query): Params<LogsQuery>) -> Json<LogsResponse> {
    Json(state.manager.logs_page(&query))
}

async fn logs_report(
    State(state): State<Shared>,
    Params(query): Params<LogsQuery>,
) -> Json<LogsReportResponse> {
    Json(state.manager.logs_report(&query))
}

async fn requests(
    State(state): State<Shared>,
    Params(query): Params<RequestsQuery>,
) -> Result<Json<RequestsResponse>, Failure> {
    Ok(Json(state.manager.requests_page(&query)?))
}

async fn requests_report(
    State(state): State<Shared>,
    Params(query): Params<RequestsQuery>,
) -> Result<Json<LogsReportResponse>, Failure> {
    Ok(Json(state.manager.requests_report(&query)?))
}

// ! Only for a page of this same dashboard: a websocket skips the browser's
// CORS checks, so another site could otherwise read the snapshot (webhook
// URLs, proxies) through it.
async fn events(
    State(state): State<Shared>,
    lease: Option<Extension<SessionLease>>,
    headers: HeaderMap,
    upgrade: WebSocketUpgrade,
) -> Response {
    if !state.access.allows_origin(&headers) {
        return Failure::from(ErrorCode::Unauthorized).into_response();
    }

    let lease = lease.map(|Extension(lease)| lease);

    upgrade
        .max_message_size(INCOMING_MESSAGE_MAX_BYTES)
        .max_frame_size(INCOMING_MESSAGE_MAX_BYTES)
        .on_upgrade(move |socket| stream_events(state, socket, lease))
}

async fn send_message(socket: &mut WebSocket, message: Message) -> bool {
    matches!(
        tokio::time::timeout(SEND_TIMEOUT, socket.send(message)).await,
        Ok(Ok(()))
    )
}

async fn send(socket: &mut WebSocket, message: &ServerMessage) -> bool {
    let Ok(text) = serde_json::to_string(message) else {
        return false;
    };

    send_message(socket, Message::Text(text.into())).await
}

async fn send_snapshot(socket: &mut WebSocket, text: Option<Utf8Bytes>) -> bool {
    match text {
        Some(text) => send_message(socket, Message::Text(text)).await,
        None => true,
    }
}

async fn stream_events(state: Shared, mut socket: WebSocket, lease: Option<SessionLease>) {
    let mut receiver = state.manager.subscribe();
    let mut bursts = state.manager.subscribe_bursts();
    let mut snapshots = state.snapshots.subscribe();

    state.snapshots.publish().await;

    let first = snapshots.borrow_and_update().clone();

    if !send_snapshot(&mut socket, first).await {
        return;
    }

    let session_ended = async {
        match &lease {
            Some(lease) => lease.ended().await,
            None => std::future::pending().await,
        }
    };
    tokio::pin!(session_ended);

    // ! Biased: once the session ends, not one more message goes out.
    loop {
        tokio::select! {
            biased;

            _ = &mut session_ended => {
                let close = CloseFrame {
                    code: close_code::POLICY,
                    reason: SESSION_ENDED_REASON.into(),
                };
                send_message(&mut socket, Message::Close(Some(close))).await;

                return;
            }
            published = snapshots.changed() => {
                if published.is_err() {
                    return;
                }

                let text = snapshots.borrow_and_update().clone();

                if !send_snapshot(&mut socket, text).await {
                    return;
                }
            }
            event = receiver.recv() => match event {
                Ok(event) => {
                    if !send(&mut socket, &ServerMessage::Event { event: Box::new(event) }).await {
                        return;
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => state.snapshots.mark_stale(),
                Err(_) => return,
            },
            progress = bursts.recv() => match progress {
                Ok(progress) => {
                    if !send(&mut socket, &ServerMessage::Burst { progress }).await {
                        return;
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {}
                Err(_) => return,
            },
            incoming = socket.recv() => match incoming {
                Some(Ok(Message::Close(_))) | None | Some(Err(_)) => return,
                _ => {}
            }
        }
    }
}

async fn static_file(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');

    if path.starts_with("api/") {
        return Failure::from(ErrorCode::NotFound).into_response();
    }

    let path = if path.is_empty() { "index.html" } else { path };

    let (path, content) = match Assets::get(path) {
        Some(content) => (path, content),
        None => match Assets::get("index.html") {
            Some(content) => ("index.html", content),
            None => return Failure::from(ErrorCode::NotFound).into_response(),
        },
    };

    let mime = mime_guess::from_path(path).first_or_octet_stream();
    let cache = if path.starts_with("assets/") {
        "public, max-age=31536000, immutable"
    } else {
        "no-cache"
    };

    (
        [
            (header::CONTENT_TYPE, mime.as_ref().to_owned()),
            (header::CACHE_CONTROL, cache.to_owned()),
        ],
        content.data.into_owned(),
    )
        .into_response()
}
