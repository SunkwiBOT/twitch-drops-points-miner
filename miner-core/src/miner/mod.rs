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

    announce(events, event);
}

// ! `false`: the account was removed meanwhile, and the line with it.
pub(crate) fn record_account_event(
    store: &Store,
    events: &broadcast::Sender<MinerEvent>,
    mut event: MinerEvent,
) -> bool {
    match store.push_account_history(&event) {
        Ok(Some(id)) => event.id = id,
        Ok(None) => return false,
        Err(error) => tracing::warn!(%error, "history write failed"),
    }

    announce(events, event);

    true
}

fn announce(events: &broadcast::Sender<MinerEvent>, event: MinerEvent) {
    let account = Some(event.account_login.as_str()).filter(|login| !login.is_empty());

    match event.kind {
        EventKind::Error => tracing::warn!(account, "{}", event.title),
        _ => tracing::info!(account, "{}", event.title),
    }

    let _ = events.send(event);
}
