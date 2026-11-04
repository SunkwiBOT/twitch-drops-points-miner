// ! The cookie carries a random value; the server keeps only its SHA-256, so a
// memory dump never yields a usable cookie.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use base64::Engine;
use miner_core::Manager;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use tokio::sync::{Notify, RwLock};
use tokio_util::sync::CancellationToken;

const MAX_SESSIONS: usize = 50;
const SESSION_BYTES: usize = 32;

pub const SESSION_COOKIE: &str = "tdpm_session";
// ! `__Host-` makes the browser refuse the cookie unless it is `Secure`, on
// `Path=/` and without `Domain`: a page served over plain HTTP on the same
// host can neither read, plant nor overwrite it.
pub const PUBLIC_SESSION_COOKIE: &str = "__Host-tdpm_session";
// ! The cookie's `Max-Age`, enforced here too: a stolen cookie dies with it.
pub const SESSION_MAX_AGE: Duration = Duration::from_secs(30 * 24 * 60 * 60);

type ActiveAt = Arc<Mutex<Instant>>;

struct Session {
    hash: [u8; 32],
    created_at: Instant,
    active_at: ActiveAt,
    revoked: CancellationToken,
}

fn read(active_at: &ActiveAt) -> Instant {
    *miner_core::util::locked(active_at)
}

impl Session {
    fn expires_at(&self) -> Instant {
        self.created_at + SESSION_MAX_AGE
    }

    fn idle_at(&self, idle_limit: Option<Duration>) -> Option<Instant> {
        idle_limit.map(|limit| read(&self.active_at) + limit)
    }

    fn alive_at(&self, now: Instant, idle_limit: Option<Duration>) -> bool {
        now < self.expires_at() && self.idle_at(idle_limit).is_none_or(|idle_at| now < idle_at)
    }
}

#[derive(Clone)]
pub struct SessionLease {
    revoked: CancellationToken,
    expires_at: Instant,
    active_at: ActiveAt,
}

impl SessionLease {
    pub async fn ended(&self) {
        tokio::select! {
            _ = self.revoked.cancelled() => {}
            _ = tokio::time::sleep_until(self.expires_at.into()) => {}
        }
    }

    // ! Only the activity route calls it: what the interface asks on its own (the
    // logs every five seconds, a snapshot) keeps no session open.
    pub fn touch(&self) {
        *miner_core::util::locked(&self.active_at) = Instant::now();
    }
}

#[derive(Default)]
pub struct Sessions {
    entries: RwLock<Vec<Session>>,
    created: Notify,
}

fn hash(session: &str) -> [u8; 32] {
    Sha256::digest(session.as_bytes()).into()
}

impl Sessions {
    pub async fn create(&self) -> String {
        let mut raw = [0u8; SESSION_BYTES];
        rand::RngCore::fill_bytes(&mut rand::rng(), &mut raw);
        let session = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(raw);
        let now = Instant::now();

        let mut entries = self.entries.write().await;

        entries.retain(|entry| {
            let alive = now < entry.expires_at();

            if !alive {
                entry.revoked.cancel();
            }

            alive
        });

        entries.push(Session {
            hash: hash(&session),
            created_at: now,
            active_at: Arc::new(Mutex::new(now)),
            revoked: CancellationToken::new(),
        });

        if entries.len() > MAX_SESSIONS {
            entries.remove(0).revoked.cancel();
        }

        drop(entries);
        self.created.notify_waiters();

        session
    }

    pub async fn find(&self, session: &str, idle_limit: Option<Duration>) -> Option<SessionLease> {
        self.find_at(session, Instant::now(), idle_limit).await
    }

    async fn find_at(
        &self,
        session: &str,
        now: Instant,
        idle_limit: Option<Duration>,
    ) -> Option<SessionLease> {
        let wanted = hash(session);

        self.entries
            .read()
            .await
            .iter()
            .find(|entry| bool::from(entry.hash.ct_eq(&wanted)))
            .filter(|entry| entry.alive_at(now, idle_limit))
            .map(|entry| SessionLease {
                revoked: entry.revoked.clone(),
                expires_at: entry.expires_at(),
                active_at: entry.active_at.clone(),
            })
    }

    async fn end_idle_at(&self, now: Instant, idle_limit: Option<Duration>) -> Option<Instant> {
        idle_limit?;

        let idle = |entry: &Session| entry.idle_at(idle_limit).is_some_and(|at| at <= now);

        if self.entries.read().await.iter().any(idle) {
            self.entries.write().await.retain(|entry| {
                let ended = idle(entry);

                if ended {
                    entry.revoked.cancel();
                }

                !ended
            });
        }

        self.entries
            .read()
            .await
            .iter()
            .filter_map(|entry| entry.idle_at(idle_limit))
            .min()
    }

    pub async fn end_idle(&self, manager: &Manager) {
        let changed = manager.changed.notified();
        tokio::pin!(changed);
        changed.as_mut().enable();

        let created = self.created.notified();
        tokio::pin!(created);
        created.as_mut().enable();

        loop {
            let idle_limit = manager.global_settings().await.auto_lock.idle_limit();
            let next = self.end_idle_at(Instant::now(), idle_limit).await;
            let deadline = async {
                match next {
                    Some(at) => tokio::time::sleep_until(at.into()).await,
                    None => std::future::pending().await,
                }
            };

            tokio::select! {
                _ = deadline => {}
                _ = &mut changed => {
                    changed.set(manager.changed.notified());
                    changed.as_mut().enable();
                }
                _ = &mut created => {
                    created.set(self.created.notified());
                    created.as_mut().enable();
                }
            }
        }
    }

    pub async fn revoke(&self, session: &str) {
        let wanted = hash(session);

        self.entries.write().await.retain(|entry| {
            let kept = entry.hash != wanted;

            if !kept {
                entry.revoked.cancel();
            }

            kept
        });
    }
}
