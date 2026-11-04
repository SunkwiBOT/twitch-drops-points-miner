// ! Shown with `notify-rust` directly: the Tauri notification plugin only
// forwards an icon name, so it cannot show a picture on Windows or macOS.

use std::path::{Path, PathBuf};

use super::images::ImageCache;
use super::{account_name, channel_name, picture};
use crate::error::{Error, Result};
use crate::events::{EventContext, MinerEvent};
use crate::requests::RequestLog;

#[derive(Clone)]
pub struct DesktopNotifier {
    images: Option<ImageCache>,
    // ! The app's own icon as a file: the icon theme only knows it once the
    // app is installed, and a dev run would show a "missing icon" mark.
    icon: Option<String>,
    #[cfg(windows)]
    app_id: Option<String>,
}

impl DesktopNotifier {
    pub async fn open(
        data_dir: &Path,
        identifier: &str,
        dev: bool,
        icon: &[u8],
        requests: RequestLog,
    ) -> Self {
        register(identifier, dev);

        let icon_path = data_dir.join("cache").join("app-icon.png");
        let icon = match tokio::fs::create_dir_all(data_dir.join("cache")).await {
            Ok(()) => tokio::fs::write(&icon_path, icon)
                .await
                .ok()
                .and_then(|()| icon_path.to_str().map(str::to_owned)),
            Err(_) => None,
        };
        let images = match ImageCache::open(data_dir, requests).await {
            Ok(images) => Some(images),
            Err(error) => {
                tracing::warn!(%error, "notification pictures are unavailable");
                None
            }
        };

        Self {
            images,
            icon,
            #[cfg(windows)]
            app_id: installed().then(|| identifier.to_owned()),
        }
    }

    pub fn notify(&self, event: MinerEvent, language: &str) {
        let event = super::localized(event, crate::i18n::resolve(language));
        let this = self.clone();

        tokio::spawn(async move {
            let image = this.image(&event).await;
            let shown =
                tokio::task::spawn_blocking(move || this.show(&event, image.as_deref())).await;

            if let Ok(Err(error)) = shown {
                tracing::warn!(%error, "desktop notification failed");
            }
        });
    }

    pub fn show(&self, event: &MinerEvent, image: Option<&Path>) -> Result<()> {
        let mut notification = notify_rust::Notification::new();

        notification
            .appname(crate::APP_NAME)
            .summary(&crate::logs::redact(&event.title))
            .body(&crate::logs::redact(&body(event)));

        match &self.icon {
            Some(icon) => notification.icon(icon),
            None => notification.auto_icon(),
        };

        if let Some(image) = image.and_then(image_path) {
            notification.image_path(&image);
        }

        #[cfg(windows)]
        if let Some(app_id) = &self.app_id {
            notification.app_id(app_id);
        }

        deliver(&notification)
    }

    async fn image(&self, event: &MinerEvent) -> Option<PathBuf> {
        let url = picture(&event.context)?;

        match self.images.as_ref()?.fetch(url).await {
            Ok(path) => Some(path),
            Err(error) => {
                tracing::debug!(%error, "notification shown without its picture");
                None
            }
        }
    }
}

fn body(event: &MinerEvent) -> String {
    [
        event.detail.clone(),
        place(&event.context),
        account_name(event).map(|name| format!("Twitch: {name}")),
    ]
    .into_iter()
    .flatten()
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>()
    .join("\n")
}

fn place(context: &EventContext) -> Option<String> {
    let parts: Vec<&str> = [context.game.as_deref(), channel_name(context)]
        .into_iter()
        .flatten()
        .filter(|part| !part.is_empty())
        .collect();

    (!parts.is_empty()).then(|| parts.join(" · "))
}

// ! What `image_path` takes: a `file://` URI for the Linux hint, as the
// notification specification asks, a plain path on Windows and macOS.
fn image_path(path: &Path) -> Option<String> {
    if cfg!(all(unix, not(target_os = "macos"))) {
        url::Url::from_file_path(path).ok().map(String::from)
    } else {
        path.to_str().map(str::to_owned)
    }
}

// ! Linux and the BSDs send on one long-lived bus connection, since GNOME
// closes a notification once the connection that sent it goes away.
fn deliver(notification: &notify_rust::Notification) -> Result<()> {
    #[cfg(all(unix, not(target_os = "macos"), feature = "zbus"))]
    if bus::send(notification) {
        return Ok(());
    }

    notification
        .show()
        .map(drop)
        .map_err(|error| Error::Notification(format!("desktop notification: {error}")))
}

// ! macOS shows notifications under one application, chosen once. A dev run
// borrows Terminal's identity, as the Tauri plugin does.
#[cfg(target_os = "macos")]
fn register(identifier: &str, dev: bool) {
    static ONCE: std::sync::Once = std::sync::Once::new();

    ONCE.call_once(|| {
        let _ = notify_rust::set_application(if dev {
            "com.apple.Terminal"
        } else {
            identifier
        });
    });
}

#[cfg(not(target_os = "macos"))]
fn register(_identifier: &str, _dev: bool) {}

// ! Windows: a toast carries the app's identity only for the installed app; a
// build run from `target\debug` or `target\release` has none registered.
#[cfg(windows)]
fn installed() -> bool {
    let Some(dir) = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf))
    else {
        return false;
    };

    !(dir.ends_with(Path::new("target").join("debug"))
        || dir.ends_with(Path::new("target").join("release")))
}

#[cfg(all(unix, not(target_os = "macos"), feature = "zbus"))]
mod bus {
    use std::collections::HashMap;
    use std::sync::OnceLock;

    use notify_rust::Hint;
    use zbus::zvariant::Value;

    const NAME: &str = "org.freedesktop.Notifications";
    const PATH: &str = "/org/freedesktop/Notifications";

    fn connection() -> Option<&'static zbus::blocking::Connection> {
        static CONNECTION: OnceLock<Option<zbus::blocking::Connection>> = OnceLock::new();

        CONNECTION
            .get_or_init(|| zbus::blocking::Connection::session().ok())
            .as_ref()
    }

    pub fn send(notification: &notify_rust::Notification) -> bool {
        let Some(connection) = connection() else {
            return false;
        };
        let hints: HashMap<&str, Value<'_>> = notification
            .hints
            .iter()
            .filter_map(|hint| match hint {
                Hint::ImagePath(path) => Some(("image-path", Value::from(path.as_str()))),
                _ => None,
            })
            .collect();
        let sent = connection.call_method(
            Some(NAME),
            PATH,
            Some(NAME),
            "Notify",
            &(
                &notification.appname,
                0u32,
                &notification.icon,
                &notification.summary,
                &notification.body,
                &notification.actions,
                hints,
                -1i32,
            ),
        );

        if let Err(error) = &sent {
            tracing::debug!(%error, "notification bus refused, retrying on a new connection");
        }

        sent.is_ok()
    }
}
