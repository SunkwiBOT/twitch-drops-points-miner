#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod env;
mod lifecycle;
mod tray;
mod updates;

use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use miner_core::api::{
    AccountStatus, AppInfo, AppInfoResponse, AutostartResponse, CatalogRefreshResponse,
    CatalogView, ChannelBannersResponse, ChannelResponse, ChannelsLookupResponse, Command,
    CommandName, DesktopEvent, FollowsResponse, HistoryQuery, HistoryResponse, LoginFlowResponse,
    LogsQuery, LogsReportResponse, LogsResponse, NotificationTestResponse, ProxyTestResponse,
    RedeemRequest, RedeemResponse, RequestsQuery, RequestsResponse, SearchResponse, Shell,
    Snapshot, SnapshotResponse, SpamBalanceResponse, SpamChannelResponse, StartupProblem,
    StartupProblemResponse,
};
use miner_core::config::{CloseAction, GlobalSettings};
use miner_core::error::{ApiError, ErrorCode};
use miner_core::i18n;
use miner_core::notifications::desktop::DesktopNotifier;
use miner_core::store::VaultStatus;
use miner_core::Manager;
use tauri::{AppHandle, Emitter, Manager as _, RunEvent, WindowEvent};

use crate::env::DesktopEnv;
use crate::tray::TrayView;

pub struct AppState {
    pub manager: Arc<Manager>,
    pub stopped: AtomicBool,
}

pub struct DataFolder {
    pub path: PathBuf,
    pub problem: Option<String>,
}

type Answer<T> = Result<T, ApiError>;

// ! Locked, the window may only unlock, lock again or cancel a sign-in: it holds
// no settings worth saving (see `locked_snapshot`) and must not remove an account
// without the password.
const LOCKED_COMMANDS: [CommandName; 3] = [
    CommandName::Unlock,
    CommandName::Lock,
    CommandName::CancelLogin,
];

// ! Strict lock: while the app password is not typed, nothing the window asks
// for reads the accounts, their history or the log, changes the system or
// installs an update.
pub(crate) async fn unlocked(state: &AppState) -> Answer<()> {
    if state.manager.vault_status().await.unlocked {
        Ok(())
    } else {
        Err(ApiError::new(ErrorCode::VaultLocked))
    }
}

// ! A lock between the two reads leaves the full snapshot with a locked vault:
// it is reduced too.
async fn visible_snapshot(manager: &Manager) -> Answer<Snapshot> {
    if manager.vault_status().await.unlocked {
        let snapshot = manager.snapshot().await?;

        if snapshot.vault.unlocked {
            return Ok(snapshot);
        }
    }

    Ok(locked_snapshot(manager).await)
}

// ! Only what the lock screen needs: no account, event, campaign, proxy or
// notification secret.
async fn locked_snapshot(manager: &Manager) -> Snapshot {
    let settings = manager.global_settings().await;
    let vault = manager.vault_status().await;

    Snapshot {
        version: miner_core::VERSION.to_owned(),
        build: None,
        update_available: None,
        update_checked_at: None,
        accounts: Vec::new(),
        catalog: CatalogView::default(),
        settings: GlobalSettings {
            language: settings.language,
            theme: settings.theme,
            time_zone: settings.time_zone,
            date_format: settings.date_format,
            clock_format: settings.clock_format,
            ..GlobalSettings::default()
        },
        vault: VaultStatus {
            unlocked: false,
            ..vault
        },
        recent_events: Vec::new(),
        login: None,
    }
}

#[tauri::command]
async fn snapshot(state: tauri::State<'_, AppState>) -> Answer<SnapshotResponse> {
    Ok(SnapshotResponse {
        snapshot: visible_snapshot(&state.manager).await?,
    })
}

#[tauri::command]
async fn command(state: tauri::State<'_, AppState>, command: Command) -> Answer<()> {
    if !LOCKED_COMMANDS.contains(&command.name()) {
        unlocked(&state).await?;
    }

    Ok(state.manager.execute(command).await?)
}

#[tauri::command]
async fn start_login(state: tauri::State<'_, AppState>) -> Answer<LoginFlowResponse> {
    unlocked(&state).await?;

    Ok(LoginFlowResponse {
        login: state.manager.start_login().await?,
    })
}

#[tauri::command]
async fn lookup_channel(
    state: tauri::State<'_, AppState>,
    login: String,
) -> Answer<ChannelResponse> {
    unlocked(&state).await?;

    Ok(ChannelResponse {
        channel: state.manager.lookup_channel(&login).await?,
    })
}

#[tauri::command]
async fn channel_banners(
    state: tauri::State<'_, AppState>,
    logins: Vec<String>,
) -> Answer<ChannelBannersResponse> {
    unlocked(&state).await?;

    Ok(state.manager.channel_banners(&logins).await?)
}

#[tauri::command]
async fn search(state: tauri::State<'_, AppState>, query: String) -> Answer<SearchResponse> {
    unlocked(&state).await?;

    Ok(state.manager.search(&query).await?)
}

#[tauri::command]
async fn lookup_channels(
    state: tauri::State<'_, AppState>,
    logins: Vec<String>,
) -> Answer<ChannelsLookupResponse> {
    unlocked(&state).await?;

    Ok(state.manager.lookup_channels(&logins).await?)
}

#[tauri::command]
async fn follows(state: tauri::State<'_, AppState>, account_id: String) -> Answer<FollowsResponse> {
    unlocked(&state).await?;

    Ok(FollowsResponse {
        channels: state.manager.followed_channels(&account_id).await?,
    })
}

#[tauri::command]
async fn spam_channel(
    state: tauri::State<'_, AppState>,
    account_id: String,
    login: String,
) -> Answer<SpamChannelResponse> {
    unlocked(&state).await?;

    Ok(SpamChannelResponse {
        channel: state.manager.spam_channel(&account_id, &login).await?,
    })
}

#[tauri::command]
async fn spam_balance(
    state: tauri::State<'_, AppState>,
    account_id: String,
    login: String,
) -> Answer<SpamBalanceResponse> {
    unlocked(&state).await?;

    Ok(SpamBalanceResponse {
        balance: state.manager.spam_balance(&account_id, &login).await?,
    })
}

#[tauri::command]
async fn redeem(
    state: tauri::State<'_, AppState>,
    request: RedeemRequest,
) -> Answer<RedeemResponse> {
    unlocked(&state).await?;

    Ok(RedeemResponse {
        burst_id: state.manager.redeem(request).await?,
    })
}

#[tauri::command]
async fn test_notifications(state: tauri::State<'_, AppState>) -> Answer<NotificationTestResponse> {
    unlocked(&state).await?;

    Ok(NotificationTestResponse {
        targets: state.manager.test_notification_targets().await,
    })
}

#[tauri::command]
async fn test_proxy(state: tauri::State<'_, AppState>, url: String) -> Answer<ProxyTestResponse> {
    unlocked(&state).await?;

    Ok(state.manager.test_proxy(&url).await?)
}

#[tauri::command]
async fn history(
    state: tauri::State<'_, AppState>,
    query: HistoryQuery,
) -> Answer<HistoryResponse> {
    unlocked(&state).await?;

    Ok(state.manager.history_page(&query)?)
}

#[tauri::command]
async fn refresh_catalog(state: tauri::State<'_, AppState>) -> Answer<CatalogRefreshResponse> {
    unlocked(&state).await?;

    Ok(CatalogRefreshResponse {
        refresh: state.manager.refresh_catalog().await?,
    })
}

#[tauri::command]
async fn logs(state: tauri::State<'_, AppState>, query: LogsQuery) -> Answer<LogsResponse> {
    unlocked(&state).await?;

    Ok(state.manager.logs_page(&query))
}

#[tauri::command]
async fn logs_report(
    state: tauri::State<'_, AppState>,
    query: LogsQuery,
) -> Answer<LogsReportResponse> {
    unlocked(&state).await?;

    Ok(state.manager.logs_report(&query))
}

#[tauri::command]
async fn requests(
    state: tauri::State<'_, AppState>,
    query: RequestsQuery,
) -> Answer<RequestsResponse> {
    unlocked(&state).await?;

    Ok(state.manager.requests_page(&query)?)
}

#[tauri::command]
async fn requests_report(
    state: tauri::State<'_, AppState>,
    query: RequestsQuery,
) -> Answer<LogsReportResponse> {
    unlocked(&state).await?;

    Ok(state.manager.requests_report(&query)?)
}

#[tauri::command]
async fn close_window(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    action: CloseAction,
) -> Answer<()> {
    let action = if action == CloseAction::Tray {
        CloseAction::Tray
    } else {
        CloseAction::Quit
    };

    let mut settings = state.manager.global_settings().await;
    settings.close_action = action;
    state.manager.set_global_settings(settings).await?;

    if action == CloseAction::Tray {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.hide();
        }
    } else {
        lifecycle::quit(app).await;
    }

    Ok(())
}

#[tauri::command]
fn app_info(folder: tauri::State<'_, DataFolder>) -> AppInfoResponse {
    AppInfoResponse {
        app: AppInfo {
            name: miner_core::APP_NAME.to_owned(),
            version: miner_core::VERSION.to_owned(),
            platform: std::env::consts::OS.to_owned(),
            data_dir: folder.path.to_string_lossy().into_owned(),
        },
    }
}

#[tauri::command]
fn startup_problem(folder: tauri::State<'_, DataFolder>) -> StartupProblemResponse {
    StartupProblemResponse {
        problem: folder.problem.as_ref().map(|detail| StartupProblem {
            detail: detail.clone(),
            data_dir: folder.path.to_string_lossy().into_owned(),
        }),
    }
}

#[tauri::command]
async fn quit(app: AppHandle) {
    lifecycle::quit(app).await;
}

fn open_directory(app: &AppHandle, dir: &Path) -> Answer<()> {
    tauri_plugin_opener::OpenerExt::opener(app)
        .open_path(dir.to_string_lossy().to_string(), None::<&str>)
        .map_err(|error| {
            tracing::warn!("opening a folder failed: {error}");
            ApiError::new(ErrorCode::StorageError)
        })
}

#[tauri::command]
async fn open_logs_directory(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    folder: tauri::State<'_, DataFolder>,
) -> Answer<()> {
    unlocked(&state).await?;

    open_directory(&app, &folder.path.join("logs"))
}

#[tauri::command]
fn open_data_directory(app: AppHandle, folder: tauri::State<'_, DataFolder>) -> Answer<()> {
    open_directory(&app, &folder.path)
}

#[tauri::command]
fn autostart_status(app: AppHandle) -> AutostartResponse {
    use tauri_plugin_autostart::ManagerExt;

    AutostartResponse {
        enabled: app.autolaunch().is_enabled().unwrap_or(false),
    }
}

#[tauri::command]
async fn set_autostart(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    enabled: bool,
) -> Answer<()> {
    use tauri_plugin_autostart::ManagerExt;

    unlocked(&state).await?;

    let launcher = app.autolaunch();
    let result = if enabled {
        launcher.enable()
    } else {
        launcher.disable()
    };

    result.map_err(|error| {
        tracing::warn!("autostart change failed: {error}");
        ApiError::new(ErrorCode::StorageError)
    })
}

pub fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

fn init_logging(
    dir: &Path,
    log_buffer: miner_core::logs::LogBuffer,
) -> tracing_appender::non_blocking::WorkerGuard {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    let logs = dir.join("logs");
    let _ = std::fs::create_dir_all(&logs);

    let file = miner_core::logs::daily_files(&logs)
        .unwrap_or_else(|_| tracing_appender::rolling::daily(&logs, "miner.log"));
    let (file, guard) = tracing_appender::non_blocking(file);
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,hyper=warn,reqwest=warn"));

    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_writer(miner_core::logs::Redacted(std::io::stdout)),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(miner_core::logs::Redacted(file)),
        )
        .with(miner_core::logs::LogLayer { buffer: log_buffer })
        .try_init();

    guard
}

fn configure_window(window: &tauri::WebviewWindow) {
    if let Ok(icon) = tauri::image::Image::from_bytes(include_bytes!("../icons/128x128.png")) {
        let _ = window.set_icon(icon);
    }

    #[cfg(target_os = "linux")]
    gtk::Window::set_default_icon_name("twitchminer");
}

#[cfg(target_os = "linux")]
fn prepare_linux_display() {
    // ! The GTK program name becomes the Wayland app id; KWin looks up
    // "<app id>.desktop" for the icon, so it must match the product name.
    glib::set_prgname(Some("Twitch Drops & Points Miner"));
    glib::set_application_name("Twitch Drops & Points Miner");

    if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
        // ! Avoids blank windows on NVIDIA drivers.
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }
}

#[cfg(not(target_os = "linux"))]
fn prepare_linux_display() {}

fn main() {
    prepare_linux_display();

    let env = match DesktopEnv::from_env() {
        Ok(env) => env,
        Err(error) => {
            eprintln!("invalid environment: {error:#}");
            std::process::exit(2);
        }
    };

    let dir = env.data_dir.clone();
    let _ = std::fs::create_dir_all(&dir);
    let log_buffer = miner_core::logs::LogBuffer::new();
    let _file_writer = miner_core::logs::keep_file_writer(init_logging(&dir, log_buffer.clone()));

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            show_main_window(app)
        }))
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(updates::PendingUpdate::default())
        .manage(updates::FoundUpdate::default())
        .invoke_handler(tauri::generate_handler![
            snapshot,
            command,
            refresh_catalog,
            start_login,
            lookup_channel,
            lookup_channels,
            search,
            channel_banners,
            follows,
            spam_channel,
            spam_balance,
            redeem,
            test_notifications,
            test_proxy,
            history,
            logs,
            logs_report,
            requests,
            requests_report,
            close_window,
            app_info,
            startup_problem,
            quit,
            open_logs_directory,
            open_data_directory,
            autostart_status,
            set_autostart,
            updates::check_for_update,
            updates::found_update,
            updates::take_failed_update,
            updates::download_update,
            updates::apply_update
        ])
        .setup(move |app| {
            let opened = Manager::open(&dir, log_buffer.clone(), Shell::Desktop);

            app.manage(DataFolder {
                path: dir.clone(),
                problem: opened
                    .as_ref()
                    .err()
                    .map(|error| miner_core::logs::redact(&error.to_string())),
            });

            // ! An error out of `setup` would end the app before any window
            // opens (nothing at all on Windows): the window says why instead.
            let manager = match opened {
                Ok(manager) => manager,
                Err(error) => {
                    tracing::error!("the core could not start: {error}");

                    if let Some(window) = app.get_webview_window("main") {
                        configure_window(&window);
                        let _ = window.show();
                    }

                    return Ok(());
                }
            };

            app.manage(AppState {
                manager: manager.clone(),
                stopped: AtomicBool::new(false),
            });

            // ! The window is created hidden (tauri.conf.json) so it never flashes on screen
            // first. A locked app password still shows it: nothing mines before the
            // password is typed, so hiding would only leave the app idle.
            let (settings, unlocked) = tauri::async_runtime::block_on(async {
                (
                    manager.global_settings().await,
                    manager.vault_status().await.unlocked,
                )
            });
            let hidden = unlocked
                && (settings.start_minimized || std::env::args().any(|a| a == "--minimized"));
            let handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                manager.start().await;
                tauri::async_runtime::spawn(updates::check_regularly(
                    handle.clone(),
                    manager.clone(),
                ));
                forward_events(handle, manager).await;
            });

            tray::install(app.handle(), i18n::resolve(&settings.language))?;

            if let Some(window) = app.get_webview_window("main") {
                configure_window(&window);

                if !hidden {
                    let _ = window.show();
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let app = window.app_handle();

                let Some(state) = app.try_state::<AppState>() else {
                    return;
                };

                let manager = state.manager.clone();
                let action = tauri::async_runtime::block_on(async {
                    manager.global_settings().await.close_action
                });

                match action {
                    CloseAction::Tray => {
                        api.prevent_close();
                        let _ = window.hide();
                    }
                    CloseAction::Quit => {
                        api.prevent_close();
                        tauri::async_runtime::spawn(lifecycle::quit(app.clone()));
                    }
                    CloseAction::Ask => {
                        api.prevent_close();
                        let _ = window.emit(DesktopEvent::CloseRequested.name(), ());
                    }
                }
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building the application")
        .run(|app, event| {
            // ! Only an exit the app did not start itself: its own exits carry a code and
            // have already been through `quit`.
            if let RunEvent::ExitRequested {
                code: None, api, ..
            } = event
            {
                api.prevent_exit();
                tauri::async_runtime::spawn(lifecycle::quit(app.clone()));
            }
        });
}

async fn forward_events(app: AppHandle, manager: Arc<Manager>) {
    let data_dir = app.state::<DataFolder>().path.clone();
    let desktop = DesktopNotifier::open(
        &data_dir,
        &app.config().identifier,
        tauri::is_dev(),
        include_bytes!("../icons/128x128.png"),
        manager.requests.clone(),
    )
    .await;

    let mut receiver = manager.subscribe();
    let mut bursts = manager.subscribe_bursts();
    let mut dirty = false;
    let mut tray_view: Option<TrayView> = None;
    let mut debounce = tokio::time::interval(std::time::Duration::from_millis(
        miner_core::api::SNAPSHOT_PUSH_DEBOUNCE_MS,
    ));

    // ! One waiter kept armed between iterations, so a change signalled while
    // the loop is busy is not lost.
    let changed = manager.changed.notified();
    tokio::pin!(changed);
    changed.as_mut().enable();

    loop {
        tokio::select! {
            _ = &mut changed => {
                dirty = true;
                changed.set(manager.changed.notified());
                changed.as_mut().enable();
            }
            _ = debounce.tick() => {
                if dirty {
                    dirty = false;

                    let Ok(snapshot) = visible_snapshot(&manager).await else {
                        continue;
                    };
                    let view = TrayView {
                        language: i18n::resolve(&snapshot.settings.language),
                        sign_in_required: snapshot
                            .accounts
                            .iter()
                            .any(|account| account.status == AccountStatus::LoginRequired),
                    };

                    if tray_view != Some(view) {
                        tray::show(&app, tray_view, view);
                        tray_view = Some(view);
                    }

                    let _ = app.emit(DesktopEvent::Snapshot.name(), snapshot);
                }
            }
            event = receiver.recv() => match event {
                Ok(event) => {
                    let settings = manager.global_settings().await;

                    if settings.notifications.system && miner_core::notifications::wanted(&settings.notifications.events, event.kind) {
                        desktop.notify(event.clone(), &settings.language);
                    }

                    if manager.vault_status().await.unlocked {
                        let _ = app.emit(DesktopEvent::MinerEvent.name(), event);
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => { dirty = true; }
                Err(_) => break,
            },
            progress = bursts.recv() => match progress {
                Ok(progress) => {
                    if manager.vault_status().await.unlocked {
                        let _ = app.emit(DesktopEvent::BurstProgress.name(), progress);
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {}
                Err(_) => break,
            }
        }
    }
}
