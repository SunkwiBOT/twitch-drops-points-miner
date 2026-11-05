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

// ! What the app tells the services it calls besides Twitch (to Twitch it
// stays the TV client): the kind of install, the system and the processor. The
// Docker image sets MINER_CONTAINER; the same server run by hand is "server".
pub fn user_agent(shell: api::Shell) -> String {
    let kind = match shell {
        api::Shell::Desktop => "desktop",
        api::Shell::Server if std::env::var_os("MINER_CONTAINER").is_some() => "docker",
        api::Shell::Server => "server",
    };

    format!(
        "TwitchMiner/{VERSION} ({kind}; {}; {}; +{REPOSITORY_URL})",
        std::env::consts::OS,
        std::env::consts::ARCH
    )
}
