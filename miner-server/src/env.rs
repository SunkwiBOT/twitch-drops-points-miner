use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context};
use ipnet::IpNet;
use url::Url;

use crate::access::{Access, HTTPS};

const DEFAULT_DATA_DIR: &str = "data";
const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 8080;
// ! A shorter password is said at start, not refused: a running server keeps
// starting.
pub const PASSWORD_MIN_CHARS: usize = 12;
const FILE_SUFFIX: &str = "_FILE";
const PASSWORD: &str = "MINER_PASSWORD";
// ! The value docker-compose.yml and the README show: anyone who read them knows it.
const PASSWORD_EXAMPLE: &str = "change-me";
const PUBLIC_URL: &str = "MINER_PUBLIC_URL";
const VAULT_PASSPHRASE: &str = "MINER_VAULT_PASSPHRASE";
const TRUSTED_HOPS: &str = "MINER_TRUSTED_HOPS";
const TRUSTED_HOPS_DEFAULT: usize = 1;
const TRUSTED_HOPS_MAX: usize = 10;
const PROXY_NETWORKS: &str = "MINER_PROXY_NETWORKS";

pub struct ServerEnv {
    pub data_dir: PathBuf,
    pub log_dir: PathBuf,
    pub address: SocketAddr,
    pub access: Access,
    pub vault_passphrase: Option<String>,
}

impl ServerEnv {
    pub fn from_env() -> anyhow::Result<Self> {
        Self::from_lookup(
            |name| std::env::var(name).ok(),
            |path| std::fs::read_to_string(path),
        )
    }

    fn from_lookup(
        lookup: impl Fn(&str) -> Option<String>,
        read: impl Fn(&Path) -> std::io::Result<String>,
    ) -> anyhow::Result<Self> {
        let set = |name: &str| lookup(name).filter(|value| !value.is_empty());
        let secret = |name: &str| read_secret(name, &set, &read);
        let data_dir = PathBuf::from(set("DATA_DIR").unwrap_or_else(|| DEFAULT_DATA_DIR.into()));
        let log_dir = set("LOG_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| data_dir.join("logs"));
        let host: IpAddr = set("HOST")
            .unwrap_or_else(|| DEFAULT_HOST.into())
            .parse()
            .context("HOST must be an IP address")?;

        let port = match set("PORT") {
            Some(port) => parse_port(&port)?,
            None => DEFAULT_PORT,
        };

        let password = secret(PASSWORD)?;
        let trusted_hops = match set(TRUSTED_HOPS) {
            Some(hops) => parse_trusted_hops(&hops)?,
            None => TRUSTED_HOPS_DEFAULT,
        };
        let public_url = set(PUBLIC_URL)
            .map(|url| parse_public_url(&url).context(PUBLIC_URL))
            .transpose()?;
        let proxy_networks = match set(PROXY_NETWORKS) {
            Some(networks) => parse_proxy_networks(&networks)?,
            None => Vec::new(),
        };

        let Some(password) = password else {
            bail!("{PASSWORD} is required (or {PASSWORD}{FILE_SUFFIX}): the dashboard never runs without a password");
        };

        if password.trim() == PASSWORD_EXAMPLE {
            bail!(
                "{PASSWORD} is still the example \"{PASSWORD_EXAMPLE}\": choose your own password"
            );
        }

        let access = match public_url {
            None => Access::Local { password },
            Some(url) => Access::Public {
                url,
                password,
                trusted_hops,
                proxy_networks,
            },
        };

        Ok(Self {
            data_dir,
            log_dir,
            address: SocketAddr::new(host, port),
            access,
            vault_passphrase: secret(VAULT_PASSPHRASE)?,
        })
    }

    pub fn short_password(&self) -> bool {
        self.access.password().chars().count() < PASSWORD_MIN_CHARS
    }
}

fn read_secret(
    name: &str,
    set: &impl Fn(&str) -> Option<String>,
    read: &impl Fn(&Path) -> std::io::Result<String>,
) -> anyhow::Result<Option<String>> {
    let file_variable = format!("{name}{FILE_SUFFIX}");

    match (set(name), set(&file_variable)) {
        (Some(_), Some(_)) => bail!("set {name} or {file_variable}, not both"),
        (value, None) => Ok(value),
        (None, Some(path)) => {
            let content = read(Path::new(&path))
                .with_context(|| format!("{file_variable}: {path} could not be read"))?;
            let value = content.trim_end_matches(['\r', '\n']);

            if value.is_empty() {
                bail!("{file_variable}: {path} is empty");
            }

            Ok(Some(value.to_owned()))
        }
    }
}

fn parse_port(value: &str) -> anyhow::Result<u16> {
    match value.parse::<u16>() {
        Ok(port) if port > 0 => Ok(port),
        _ => bail!("PORT must be an integer from 1 to 65535, got {value:?}"),
    }
}

fn parse_trusted_hops(value: &str) -> anyhow::Result<usize> {
    match value.trim().parse::<usize>() {
        Ok(hops) if (1..=TRUSTED_HOPS_MAX).contains(&hops) => Ok(hops),
        _ => bail!(
            "{TRUSTED_HOPS} must be a whole number from 1 to {TRUSTED_HOPS_MAX}, got {value:?}"
        ),
    }
}

// ! A bare address is a network of one.
fn parse_proxy_networks(value: &str) -> anyhow::Result<Vec<IpNet>> {
    value
        .split([',', ' ', '\n', '\t'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            part.parse::<IpNet>()
                .or_else(|_| part.parse::<IpAddr>().map(IpNet::from))
                .map_err(|_| {
                    anyhow::anyhow!(
                        "{PROXY_NETWORKS}: {part:?} is not an address or a network like 192.0.2.0/24 or 2001:db8::/64"
                    )
                })
        })
        .collect()
}

fn parse_public_url(value: &str) -> anyhow::Result<Url> {
    let url = Url::parse(value).context("not a URL")?;

    if url.scheme() != HTTPS || url.host().is_none() {
        bail!("must be an https:// address, like https://miner.example.com");
    }

    let bare = url.path() == "/"
        && url.query().is_none()
        && url.fragment().is_none()
        && url.username().is_empty()
        && url.password().is_none();

    if !bare {
        bail!("must be the address alone, without a path, like https://miner.example.com");
    }

    Ok(url)
}
