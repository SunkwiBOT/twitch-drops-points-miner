pub mod api;
pub mod catalog;
pub mod config;
pub mod domain;
pub mod error;
pub mod events;
pub mod i18n;
pub mod logs;
pub mod miner;
pub mod notifications;
pub mod predictions;
pub mod requests;
pub mod scheduler;
pub mod spam;
pub mod store;
pub mod twitch;
pub mod updates;
pub mod util;

pub use error::{Error, Result};
pub use miner::manager::Manager;

pub const APP_NAME: &str = "Twitch Miner";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const REPOSITORY_URL: &str = env!("CARGO_PKG_REPOSITORY");
