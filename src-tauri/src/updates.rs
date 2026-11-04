// ! The update a check found is kept: it is the one downloaded and installed, so
// only the version the user was shown and confirmed is ever installed.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use miner_core::error::{ApiError, ErrorCode};
use miner_core::requests::{address_target, url_target, Pending, RequestKind};
use miner_core::Manager;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager as _, State};
use tauri_plugin_updater::{Update, Updater, UpdaterExt};

pub const UPDATE_PROGRESS_EVENT: &str = "UPDATE_PROGRESS";
pub const UPDATE_AVAILABLE_EVENT: &str = "UPDATE_AVAILABLE";

const PROGRESS_STEP_BYTES: u64 = 1024 * 1024;
const CHECK_TIMEOUT: Duration = Duration::from_secs(30);
const STALL_TIMEOUT: Duration = Duration::from_secs(30);

type Answer<T> = Result<T, ApiError>;

#[derive(Clone, Serialize)]
pub struct UpdateInfoDto {
    pub version: String,
    pub current_version: String,
    pub notes: Option<String>,
}

impl UpdateInfoDto {
    fn of(update: &Update) -> Self {
        Self {
            version: update.version.clone(),
            current_version: update.current_version.clone(),
            notes: update.body.clone(),
        }
    }
}

#[derive(Default)]
pub struct FoundUpdate(Mutex<Option<Found>>);

struct Found {
    update: Update,
    first: bool,
}

#[derive(Clone, Serialize)]
pub struct FailedUpdateDto {
    pub version: String,
}

#[derive(Clone, Serialize)]
pub struct UpdateProgressDto {
    pub downloaded: u64,
    pub total: Option<u64>,
}

#[derive(Default)]
pub struct PendingUpdate(Mutex<Option<DownloadedUpdate>>);

struct DownloadedUpdate {
    update: Update,
    bytes: Vec<u8>,
}

// ! Installing stops the accounts and restarts the app: only someone who typed
// the app password may ask, and only once the core runs.
pub(crate) async fn may_update(state: Option<&crate::AppState>) -> Answer<()> {
    let state = state.ok_or_else(|| ApiError::new(ErrorCode::NotSupported))?;

    crate::unlocked(state).await
}

async fn unlocked(app: &AppHandle) -> Answer<()> {
    may_update(app.try_state::<crate::AppState>().as_deref()).await
}

fn updater(app: &AppHandle) -> Answer<Updater> {
    let handle = app.clone();

    app.updater_builder()
        .timeout(CHECK_TIMEOUT)
        .configure_client(|client| client.read_timeout(STALL_TIMEOUT))
        // ! Windows only: the plugin starts the installer, then ends the process
        // itself. The accounts stop here first, like on every other way out.
        .on_before_exit(move || {
            tauri::async_runtime::block_on(crate::lifecycle::stop_miner(&handle));
            handle.cleanup_before_exit();
        })
        .build()
        .map_err(|error| {
            tracing::warn!("the updater is unavailable: {error}");
            ApiError::new(ErrorCode::NotSupported)
        })
}

fn recorded(app: &AppHandle, kind: RequestKind, target: String) -> Option<Pending> {
    app.try_state::<crate::AppState>()
        .map(|state| state.manager.requests.start(kind, target))
}

fn finished<T>(request: Option<Pending>, outcome: &Result<T, tauri_plugin_updater::Error>) {
    match (request, outcome) {
        (Some(pending), Ok(_)) => pending.succeeded(None),
        (Some(pending), Err(error)) => {
            pending.failed(&miner_core::Error::Network(error.to_string()));
        }
        (None, _) => {}
    }
}

async fn find_update(app: &AppHandle) -> Answer<Option<Update>> {
    let updater = updater(app)?;
    let request = recorded(
        app,
        RequestKind::UpdateCheck,
        address_target(&miner_core::updates::latest_release_url()),
    );
    let found = updater.check().await;

    finished(request, &found);

    found.map_err(|error| {
        tracing::info!("the update check failed: {error}");
        ApiError::new(ErrorCode::NetworkError)
    })
}

fn record(app: &AppHandle, update: Option<Update>) -> Option<UpdateInfoDto> {
    let manager = app
        .try_state::<crate::AppState>()
        .map(|state| state.manager.clone());

    if let Some(manager) = &manager {
        manager.update_checked();
    }

    let found = app.state::<FoundUpdate>();
    let mut found = miner_core::util::locked(&found.0);

    let Some(update) = update else {
        *found = None;

        if let Some(manager) = &manager {
            manager.clear_update();
        }

        return None;
    };

    let announced = manager.is_some_and(|manager| {
        manager
            .announce_update(&update.version)
            .inspect_err(|error| tracing::warn!(%error, "the new version was not announced"))
            .unwrap_or(false)
    });
    let known_first = found
        .as_ref()
        .is_some_and(|known| known.first && known.update.version == update.version);
    let info = UpdateInfoDto::of(&update);

    *found = Some(Found {
        update,
        first: announced || known_first,
    });

    if !announced {
        return None;
    }

    tracing::info!(
        "version {} is available (installed: {})",
        info.version,
        info.current_version
    );

    Some(info)
}

#[tauri::command]
pub async fn check_for_update(app: AppHandle) -> Answer<Option<UpdateInfoDto>> {
    unlocked(&app).await?;

    let update = find_update(&app).await?;
    let info = update.as_ref().map(UpdateInfoDto::of);

    record(&app, update);

    Ok(info)
}

// ! The window asks when it opens: the check at start may end before it listens
// to `UPDATE_AVAILABLE_EVENT`. Allowed while locked: the lock screen only says a
// version exists, with nothing to install.
#[tauri::command]
pub fn found_update(found: State<'_, FoundUpdate>) -> Option<UpdateInfoDto> {
    miner_core::util::locked(&found.0)
        .as_ref()
        .filter(|found| found.first)
        .map(|found| UpdateInfoDto::of(&found.update))
}

#[tauri::command]
pub fn take_failed_update(app: AppHandle) -> Option<FailedUpdateDto> {
    let state = app.try_state::<crate::AppState>()?;

    state
        .manager
        .take_failed_update()
        .inspect_err(|error| tracing::warn!(%error, "the failed update was not read"))
        .ok()
        .flatten()
        .map(|version| FailedUpdateDto { version })
}

async fn check_in_background(app: &AppHandle) -> bool {
    let Ok(update) = find_update(app).await else {
        return false;
    };

    if let Some(info) = record(app, update) {
        let _ = app.emit(UPDATE_AVAILABLE_EVENT, info);
    }

    true
}

pub async fn check_regularly(app: AppHandle, manager: Arc<Manager>) {
    miner_core::updates::on_schedule(&manager, || check_in_background(&app)).await;
}

fn signature_refused(error: &tauri_plugin_updater::Error) -> bool {
    use tauri_plugin_updater::Error;

    matches!(
        error,
        Error::Minisign(_)
            | Error::Base64(_)
            | Error::SignatureUtf8(_)
            | Error::SignedVersionMismatch { .. }
            | Error::MissingSignedVersion
    )
}

#[tauri::command]
pub async fn download_update(
    app: AppHandle,
    found: State<'_, FoundUpdate>,
    pending: State<'_, PendingUpdate>,
) -> Answer<String> {
    unlocked(&app).await?;

    let update = miner_core::util::locked(&found.0)
        .as_ref()
        .map(|found| found.update.clone())
        .ok_or_else(|| ApiError::new(ErrorCode::NotFound))?;
    let version = update.version.clone();

    tracing::info!("downloading version {version}");

    let emitter = app.clone();
    let mut downloaded: u64 = 0;
    let mut pushed: Option<u64> = None;
    let request = recorded(
        &app,
        RequestKind::UpdateDownload,
        url_target(&update.download_url),
    );
    let bytes = update
        .download(
            move |chunk, total| {
                downloaded += chunk as u64;

                let step = match total {
                    Some(total) if total > 0 => downloaded * 100 / total,
                    _ => downloaded / PROGRESS_STEP_BYTES,
                };

                if pushed != Some(step) {
                    pushed = Some(step);
                    let _ = emitter.emit(
                        UPDATE_PROGRESS_EVENT,
                        UpdateProgressDto { downloaded, total },
                    );
                }
            },
            || {},
        )
        .await;

    finished(request, &bytes);

    let bytes = bytes.map_err(|error| {
        if signature_refused(&error) {
            tracing::error!(
                "the download of version {version} does not match its signature: {error}"
            );

            return ApiError::new(ErrorCode::UpdateSignatureRefused);
        }

        tracing::warn!("the update download failed: {error}");
        ApiError::new(ErrorCode::NetworkError)
    })?;

    *miner_core::util::locked(&pending.0) = Some(DownloadedUpdate { update, bytes });

    Ok(version)
}

#[tauri::command]
pub async fn apply_update(
    app: AppHandle,
    pending: State<'_, PendingUpdate>,
    version: String,
) -> Answer<()> {
    unlocked(&app).await?;

    let downloaded = {
        let mut slot = miner_core::util::locked(&pending.0);

        match slot.as_ref() {
            Some(candidate) if candidate.update.version == version => slot.take(),
            _ => None,
        }
    }
    .ok_or_else(|| ApiError::new(ErrorCode::NotFound))?;

    tracing::info!("installing version {version}");

    // ! Off the async workers: the Windows hook above blocks on stopping the
    // accounts, which an async worker thread is not allowed to do.
    let installed = match tauri::async_runtime::spawn_blocking(move || {
        downloaded.update.install(&downloaded.bytes)
    })
    .await
    {
        Ok(result) => result.map_err(|error| {
            if signature_refused(&error) {
                tracing::error!("version {version} does not match its signature: {error}");

                return ErrorCode::UpdateSignatureRefused;
            }

            tracing::warn!("the update could not be installed: {error}");
            ErrorCode::UpdateInstallFailed
        }),
        Err(error) => {
            tracing::warn!("the update could not be installed: {error}");
            Err(ErrorCode::UpdateInstallFailed)
        }
    };

    if let Err(code) = installed {
        // ! The Windows hook may have stopped the accounts before the installer
        // failed to start; restarting on the current version brings them back.
        if crate::lifecycle::miner_stopped(&app) {
            if let Some(state) = app.try_state::<crate::AppState>() {
                if let Err(error) = state.manager.note_failed_update(&version) {
                    tracing::warn!(%error, "the failed update was not noted");
                }
            }

            app.restart();
        }

        return Err(ApiError::new(code));
    }

    // ! Elsewhere the new files are in place and this process still runs: the
    // accounts stop now, then the new version starts.
    crate::lifecycle::stop_miner(&app).await;
    tracing::info!("restarting on version {version}");
    app.restart();
}
