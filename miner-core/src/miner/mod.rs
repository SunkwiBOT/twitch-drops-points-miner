pub mod account;
mod banners;
pub mod manager;
mod spam_points;
mod twitch_api;

use tokio::sync::broadcast;

use crate::events::{EventKind, MinerEvent};
use crate::store::Store;

pub(crate) fn record_event(
    store: &Store,
    events: &broadcast::Sender<MinerEvent>,
    mut event: MinerEvent,
) {
    match store.push_history(&event) {
        Ok(id) => event.id = id,
        Err(error) => tracing::warn!(%error, "history write failed"),
    }

    let account = Some(event.account_login.as_str()).filter(|login| !login.is_empty());

    match event.kind {
        EventKind::Error => tracing::warn!(account, "{}", event.title),
        _ => tracing::info!(account, "{}", event.title),
    }

    let _ = events.send(event);
}
