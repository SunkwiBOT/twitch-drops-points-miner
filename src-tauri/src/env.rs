use std::path::{Path, PathBuf};

use anyhow::Context;

const APP_DIR: &str = "twitch-miner";
// ! The folder of the versions named "Twitch Drops & Points Miner".
const OLD_APP_DIR: &str = "twitch-drops-points-miner";

pub struct DesktopEnv {
    pub data_dir: PathBuf,
}

impl DesktopEnv {
    pub fn from_env() -> anyhow::Result<Self> {
        let set = |name: &str| std::env::var(name).ok().filter(|value| !value.is_empty());

        let data_dir = match set("DATA_DIR") {
            Some(dir) => PathBuf::from(dir),
            None => data_dir_in(
                &dirs::data_dir().context("the platform has no data directory; set DATA_DIR")?,
            ),
        };

        Ok(Self { data_dir })
    }
}

// ! The old folder moves once, as it is; if it cannot, it is used where it
// is: the accounts and the vault are never left behind.
fn data_dir_in(base: &Path) -> PathBuf {
    let dir = base.join(APP_DIR);
    let old = base.join(OLD_APP_DIR);

    if dir.exists() || !old.is_dir() {
        return dir;
    }

    match std::fs::rename(&old, &dir) {
        Ok(()) => dir,
        Err(_) => old,
    }
}
