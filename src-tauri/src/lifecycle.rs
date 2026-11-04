use std::sync::atomic::Ordering;

use tauri::{AppHandle, Manager};

use crate::AppState;

// ! The core cannot start again in the same process, so the flag lets a failed
// update restart the app.
pub async fn stop_miner(app: &AppHandle) {
    let Some(state) = app.try_state::<AppState>() else {
        return;
    };

    state.stopped.store(true, Ordering::SeqCst);

    let manager = state.manager.clone();
    manager.shutdown().await;
}

pub fn miner_stopped(app: &AppHandle) -> bool {
    app.try_state::<AppState>()
        .is_some_and(|state| state.stopped.load(Ordering::SeqCst))
}

pub async fn quit(app: AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }

    stop_miner(&app).await;
    app.exit(0);
}
