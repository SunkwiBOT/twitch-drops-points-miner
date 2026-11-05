use miner_core::i18n;
use tauri::menu::{Menu, MenuBuilder, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Wry};

pub const TRAY_ID: &str = "twitch-miner-tray";

const MENU_OPEN: &str = "open";
const MENU_QUIT: &str = "quit";

const TEXT_OPEN: &str = "tray_open";
const TEXT_QUIT: &str = "tray_quit";
const TEXT_SIGN_IN_REQUIRED: &str = "tray_sign_in_required";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrayView {
    pub language: &'static str,
    pub sign_in_required: bool,
}

pub fn install(app: &AppHandle, language: &'static str) -> tauri::Result<()> {
    let mut builder = TrayIconBuilder::with_id(TRAY_ID)
        .tooltip(tooltip(language, false))
        .menu(&menu(app, language)?)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            MENU_OPEN => crate::show_main_window(app),
            MENU_QUIT => {
                tauri::async_runtime::spawn(crate::lifecycle::quit(app.clone()));
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                crate::show_main_window(tray.app_handle());
            }
        });

    if let Some(icon) = app.default_window_icon().cloned() {
        builder = builder.icon(icon);
    }

    builder.build(app)?;

    Ok(())
}

pub fn show(app: &AppHandle, before: Option<TrayView>, view: TrayView) {
    let Some(tray) = app.tray_by_id(TRAY_ID) else {
        return;
    };

    if before.map(|before| before.language) != Some(view.language) {
        match menu(app, view.language) {
            Ok(menu) => {
                let _ = tray.set_menu(Some(menu));
            }
            Err(error) => tracing::warn!("the tray menu could not be rebuilt: {error}"),
        }
    }

    let _ = tray.set_tooltip(Some(tooltip(view.language, view.sign_in_required)));
}

fn menu(app: &AppHandle, language: &str) -> tauri::Result<Menu<Wry>> {
    let open = i18n::text(language, TEXT_OPEN, &[]);
    let quit = i18n::text(language, TEXT_QUIT, &[]);

    MenuBuilder::new(app)
        .item(&MenuItem::with_id(
            app,
            MENU_OPEN,
            open,
            true,
            None::<&str>,
        )?)
        .separator()
        .item(&MenuItem::with_id(
            app,
            MENU_QUIT,
            quit,
            true,
            None::<&str>,
        )?)
        .build()
}

fn tooltip(language: &str, sign_in_required: bool) -> String {
    if !sign_in_required {
        return miner_core::APP_NAME.to_owned();
    }

    format!(
        "{}: {}",
        miner_core::APP_NAME,
        i18n::text(language, TEXT_SIGN_IN_REQUIRED, &[])
    )
}
