use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use axum::extract::ws::Utf8Bytes;
use miner_core::api::ServerMessage;
use miner_core::Manager;
use tokio::sync::{watch, Mutex};

const SNAPSHOT_DEBOUNCE: Duration =
    Duration::from_millis(miner_core::api::SNAPSHOT_PUSH_DEBOUNCE_MS);

pub struct SnapshotFeed {
    manager: Arc<Manager>,
    latest: watch::Sender<Option<Utf8Bytes>>,
    stale: AtomicBool,
    building: Mutex<()>,
}

impl SnapshotFeed {
    pub fn new(manager: Arc<Manager>) -> Arc<Self> {
        Arc::new(Self {
            manager,
            latest: watch::Sender::new(None),
            stale: AtomicBool::new(true),
            building: Mutex::new(()),
        })
    }

    pub fn start(self: &Arc<Self>) {
        let this = self.clone();

        tokio::spawn(async move { this.run().await });
    }

    async fn run(self: Arc<Self>) {
        let mut debounce = tokio::time::interval(SNAPSHOT_DEBOUNCE);

        // ! One waiter kept armed between iterations, so a change signalled
        // while the loop builds is not lost.
        let changed = self.manager.changed.notified();
        tokio::pin!(changed);
        changed.as_mut().enable();

        loop {
            tokio::select! {
                _ = &mut changed => {
                    self.mark_stale();
                    changed.set(self.manager.changed.notified());
                    changed.as_mut().enable();
                }
                _ = debounce.tick() => {
                    if self.latest.receiver_count() > 0 {
                        self.publish().await;
                    }
                }
            }
        }
    }

    pub fn subscribe(&self) -> watch::Receiver<Option<Utf8Bytes>> {
        self.latest.subscribe()
    }

    pub fn mark_stale(&self) {
        self.stale.store(true, Ordering::SeqCst);
    }

    pub async fn publish(&self) {
        let _one_at_a_time = self.building.lock().await;

        if !self.stale.swap(false, Ordering::SeqCst) {
            return;
        }

        let snapshot = match self.manager.snapshot().await {
            Ok(snapshot) => snapshot,
            Err(error) => {
                tracing::warn!(%error, "snapshot not built, the clients keep the last one");

                return;
            }
        };
        let message = ServerMessage::Snapshot {
            snapshot: Box::new(snapshot),
        };

        match serde_json::to_string(&message) {
            Ok(text) => {
                self.latest.send_replace(Some(text.into()));
            }
            Err(error) => tracing::warn!(%error, "snapshot not serialised"),
        }
    }
}
