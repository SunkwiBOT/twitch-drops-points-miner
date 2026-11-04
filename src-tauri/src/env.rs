use std::path::PathBuf;

use anyhow::Context;

const APP_DIR: &str = "twitch-drops-points-miner";

pub struct DesktopEnv {
    pub data_dir: PathBuf,
}

impl DesktopEnv {
    pub fn from_env() -> anyhow::Result<Self> {
        let set = |name: &str| std::env::var(name).ok().filter(|value| !value.is_empty());

        let data_dir = match set("DATA_DIR") {
            Some(dir) => PathBuf::from(dir),
            None => dirs::data_dir()
                .context("the platform has no data directory; set DATA_DIR")?
                .join(APP_DIR),
        };

        Ok(Self { data_dir })
    }
}
