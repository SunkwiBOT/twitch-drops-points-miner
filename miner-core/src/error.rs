use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use ts_rs::TS;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkErrorKind {
    Connect,
    Timeout,
    Other,
}

impl NetworkErrorKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Connect => "connection failed",
            Self::Timeout => "timed out",
            Self::Other => "request failed",
        }
    }
}

impl fmt::Display for NetworkErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("network error: {0}")]
    Network(String),
    #[error("network error: {0}")]
    Transport(NetworkErrorKind),
    #[error("Twitch answered with status {0}")]
    Status(reqwest::StatusCode),
    #[error("Twitch rejected the session")]
    Unauthorized,
    #[error("Twitch answered with an unexpected payload: {0}")]
    InvalidResponse(String),
    #[error("GraphQL error in {operation}: {code}")]
    Gql {
        operation: &'static str,
        code: String,
    },
    #[error("Twitch refused the claim ({0})")]
    ClaimRefused(String),
    #[error("the device code expired before it was entered")]
    DeviceCodeExpired,
    #[error("the device code is not confirmed yet")]
    DeviceCodePending,
    #[error("storage error: {0}")]
    Storage(String),
    #[error("the vault is locked")]
    VaultLocked,
    #[error("wrong passphrase")]
    WrongPassphrase,
    #[error("vault data is corrupted")]
    InvalidVault,
    #[error("unknown account {0}")]
    UnknownAccount(String),
    #[error("no Twitch channel named {0}")]
    UnknownChannel(String),
    #[error("invalid proxy: {0}")]
    InvalidProxy(String),
    #[error("invalid streamer login: {0}")]
    InvalidStreamerLogin(String),
    #[error("set a password before locking the app")]
    PassphraseRequired,
    #[error("the catalogue asked to wait before the next refresh")]
    CatalogRateLimited,
    #[error("too many wrong passphrases in a row")]
    TooManyAttempts,
    #[error("not available in this version of the app")]
    NotSupported,
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    #[error("notification error: {0}")]
    Notification(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(rename = "ErrorApiMessage", repr(enum = name))]
pub enum ErrorCode {
    NetworkError,
    TwitchUnavailable,
    SessionExpired,
    InvalidResponse,
    TwitchRejected,
    DeviceCodeExpired,
    DeviceCodePending,
    StorageError,
    VaultLocked,
    WrongPassphrase,
    InvalidVault,
    UnknownAccount,
    UnknownChannel,
    InvalidProxy,
    InvalidStreamerLogin,
    PassphraseRequired,
    NotificationFailed,
    BeaconBlocked,
    CatalogRateLimited,
    NotSupported,
    Unauthorized,
    WrongPassword,
    TooManyAttempts,
    HttpsRequired,
    ForwardedForRequired,
    CrossSiteRequest,
    UpdateSignatureRefused,
    UpdateInstallFailed,
    InvalidRequest,
    NotFound,
    ServerUnreachable,
    UnknownError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "ApiErrorType")]
pub struct ApiError {
    pub code: ErrorCode,
}

impl ApiError {
    pub fn new(code: ErrorCode) -> Self {
        Self { code }
    }
}

impl Error {
    pub fn code(&self) -> ErrorCode {
        match self {
            Error::Network(_) | Error::Transport(_) => ErrorCode::NetworkError,
            Error::Status(_) => ErrorCode::TwitchUnavailable,
            Error::Unauthorized => ErrorCode::SessionExpired,
            Error::InvalidResponse(_) => ErrorCode::InvalidResponse,
            Error::Gql { .. } | Error::ClaimRefused(_) => ErrorCode::TwitchRejected,
            Error::DeviceCodeExpired => ErrorCode::DeviceCodeExpired,
            Error::DeviceCodePending => ErrorCode::DeviceCodePending,
            Error::Storage(_) => ErrorCode::StorageError,
            Error::VaultLocked => ErrorCode::VaultLocked,
            Error::WrongPassphrase => ErrorCode::WrongPassphrase,
            Error::InvalidVault => ErrorCode::InvalidVault,
            Error::UnknownAccount(_) => ErrorCode::UnknownAccount,
            Error::UnknownChannel(_) => ErrorCode::UnknownChannel,
            Error::InvalidRequest(_) => ErrorCode::InvalidRequest,
            Error::InvalidProxy(_) => ErrorCode::InvalidProxy,
            Error::InvalidStreamerLogin(_) => ErrorCode::InvalidStreamerLogin,
            Error::PassphraseRequired => ErrorCode::PassphraseRequired,
            Error::CatalogRateLimited => ErrorCode::CatalogRateLimited,
            Error::TooManyAttempts => ErrorCode::TooManyAttempts,
            Error::NotSupported => ErrorCode::NotSupported,
            Error::Notification(_) => ErrorCode::NotificationFailed,
        }
    }

    pub fn refusal(&self) -> bool {
        matches!(
            self,
            Error::ClaimRefused(_)
                | Error::DeviceCodeExpired
                | Error::DeviceCodePending
                | Error::VaultLocked
                | Error::WrongPassphrase
                | Error::UnknownAccount(_)
                | Error::UnknownChannel(_)
                | Error::InvalidProxy(_)
                | Error::InvalidStreamerLogin(_)
                | Error::PassphraseRequired
                | Error::CatalogRateLimited
                | Error::TooManyAttempts
                | Error::NotSupported
                | Error::InvalidRequest(_)
        )
    }
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        let code = error.code();

        if error.refusal() {
            tracing::info!(code = ?code, "{error}");
        } else {
            tracing::warn!(code = ?code, "{error}");
        }

        Self { code }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        if let Some(status) = error.status() {
            return Error::Status(status);
        }

        // ! Never keep the URL: signed playback URLs and tokens can appear in it.
        let kind = if error.is_timeout() {
            NetworkErrorKind::Timeout
        } else if error.is_connect() {
            NetworkErrorKind::Connect
        } else {
            NetworkErrorKind::Other
        };

        Error::Transport(kind)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        Error::Storage(error.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::InvalidResponse(error.to_string())
    }
}
