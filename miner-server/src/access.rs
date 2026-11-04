// ! A browser never sends a `Secure` cookie over plain HTTP: the local mode
// cannot set one, and the public mode must.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::Duration;

use axum::http::{header, HeaderMap, HeaderName, HeaderValue, Method};
use ipnet::IpNet;
use miner_core::error::ErrorCode;
use url::Url;

use crate::sessions::{PUBLIC_SESSION_COOKIE, SESSION_COOKIE};

pub const HTTPS: &str = "https";

const X_FORWARDED_FOR: HeaderName = HeaderName::from_static("x-forwarded-for");
const X_FORWARDED_PROTO: HeaderName = HeaderName::from_static("x-forwarded-proto");
const SEC_FETCH_SITE: HeaderName = HeaderName::from_static("sec-fetch-site");
const SAME_ORIGIN: &str = "same-origin";

pub enum Access {
    Local {
        password: String,
    },
    Public {
        url: Url,
        password: String,
        trusted_hops: usize,
        proxy_networks: Vec<IpNet>,
    },
}

impl Access {
    pub fn password(&self) -> &str {
        match self {
            Self::Local { password } | Self::Public { password, .. } => password,
        }
    }

    pub fn is_public(&self) -> bool {
        matches!(self, Self::Public { .. })
    }

    pub fn cookie_name(&self) -> &'static str {
        match self {
            Self::Local { .. } => SESSION_COOKIE,
            Self::Public { .. } => PUBLIC_SESSION_COOKIE,
        }
    }

    pub fn session_cookie(&self, value: &str, max_age: Duration) -> HeaderValue {
        let name = self.cookie_name();
        let secure = if self.is_public() { "; Secure" } else { "" };
        let cookie = format!(
            "{name}={value}; Path=/; HttpOnly; SameSite=Strict; Max-Age={}{secure}",
            max_age.as_secs()
        );

        HeaderValue::from_str(&cookie).expect("the session value is URL-safe base64")
    }

    // ! A request without `Origin` does not come from a browser page.
    pub fn allows_origin(&self, headers: &HeaderMap) -> bool {
        let Some(origin) = headers.get(header::ORIGIN).and_then(|v| v.to_str().ok()) else {
            return true;
        };

        match self {
            Self::Local { .. } => origin_matches_host(origin, headers),
            // ! The proxy may rewrite `Host` to an internal name; the public
            // address is the one the browser shows.
            Self::Public { url, .. } => {
                Url::parse(origin).is_ok_and(|origin| origin.origin() == url.origin())
            }
        }
    }

    // ! Another port of this host is the same site to the browser, so
    // `SameSite=Strict` still sends the cookie: only reads are let through from
    // another site or port. An older browser names the source in `Origin` only, not
    // in `Sec-Fetch-Site`.
    pub fn allows_write(&self, method: &Method, headers: &HeaderMap) -> bool {
        if method.is_safe() {
            return true;
        }

        match headers.get(SEC_FETCH_SITE).and_then(|v| v.to_str().ok()) {
            Some(site) => site.eq_ignore_ascii_case(SAME_ORIGIN),
            None => self.allows_origin(headers),
        }
    }

    // ! Behind the proxy every connection comes from the proxy itself: a request
    // without what the proxy adds reached the published port directly, or the proxy
    // is set up wrong, and the password and the cookie must not travel over it.
    pub fn admit(&self, headers: &HeaderMap, socket: IpAddr) -> Result<IpAddr, ErrorCode> {
        match self {
            Self::Local { .. } => Ok(socket),
            Self::Public {
                trusted_hops,
                proxy_networks,
                ..
            } => {
                let proxy = socket.to_canonical();

                if !from_local_network(proxy)
                    && !proxy_networks
                        .iter()
                        .any(|network| network.contains(&proxy))
                {
                    return Err(ErrorCode::ProxyNotLocal);
                }

                if !over_https(headers, *trusted_hops) {
                    return Err(ErrorCode::HttpsRequired);
                }

                forwarded_for(headers, *trusted_hops).ok_or(ErrorCode::ForwardedForRequired)
            }
        }
    }
}

// ! The proxy runs on this machine or its local network (Docker's bridge
// included): a public address reached the published port directly, and the
// `X-Forwarded-*` it sends are its own words.
fn from_local_network(socket: IpAddr) -> bool {
    match socket.to_canonical() {
        IpAddr::V4(ip) => {
            ip.is_loopback() || ip.is_private() || ip.is_link_local() || shared_space(ip)
        }
        IpAddr::V6(ip) => ip.is_loopback() || unique_local(ip) || link_local(ip),
    }
}

// ! 100.64.0.0/10: carrier-grade NAT, and the addresses Tailscale hands out.
fn shared_space(ip: Ipv4Addr) -> bool {
    let [first, second, ..] = ip.octets();

    first == 100 && second & 0xc0 == 0x40
}

fn unique_local(ip: Ipv6Addr) -> bool {
    ip.segments()[0] & 0xfe00 == 0xfc00
}

fn link_local(ip: Ipv6Addr) -> bool {
    ip.segments()[0] & 0xffc0 == 0xfe80
}

fn entries<'a>(headers: &'a HeaderMap, name: &HeaderName) -> Option<Vec<&'a str>> {
    let mut entries = Vec::new();

    for value in headers.get_all(name) {
        entries.extend(value.to_str().ok()?.split(',').map(str::trim));
    }

    Some(entries)
}

// ! Proxies usually replace `X-Forwarded-Proto` instead of adding to it: a list
// shorter than the trusted hops is read from its first entry.
fn over_https(headers: &HeaderMap, trusted_hops: usize) -> bool {
    entries(headers, &X_FORWARDED_PROTO).is_some_and(|protocols| {
        protocols
            .get(protocols.len().saturating_sub(trusted_hops))
            .is_some_and(|protocol| protocol.eq_ignore_ascii_case(HTTPS))
    })
}

fn origin_matches_host(origin: &str, headers: &HeaderMap) -> bool {
    let host = headers.get(header::HOST).and_then(|v| v.to_str().ok());
    let origin_host = Url::parse(origin).ok().and_then(|url| {
        let host = url.host_str()?.to_owned();

        Some(match url.port() {
            Some(port) => format!("{host}:{port}"),
            None => host,
        })
    });

    matches!((origin_host, host), (Some(a), Some(b)) if a.eq_ignore_ascii_case(b))
}

// ! Each proxy appends the address it received the request from; whatever
// stands before the trusted hops was written by the client and can be forged.
fn forwarded_for(headers: &HeaderMap, trusted_hops: usize) -> Option<IpAddr> {
    let addresses = entries(headers, &X_FORWARDED_FOR)?;

    addresses
        .get(addresses.len().checked_sub(trusted_hops)?)?
        .parse()
        .ok()
}
