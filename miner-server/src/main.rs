mod access;
mod attempts;
mod env;
mod feed;
mod routes;
mod sessions;
mod updates;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use miner_core::api::{ApiRoute, Shell};
use miner_core::Manager;
use tracing_subscriber::EnvFilter;

use crate::access::Access;
use crate::attempts::SignInAttempts;
use crate::env::{ServerEnv, PASSWORD_MIN_CHARS};
use crate::feed::SnapshotFeed;
use crate::routes::AppState;
use crate::sessions::Sessions;
use crate::updates::Releases;

const HEALTHCHECK_TIMEOUT: Duration = Duration::from_secs(4);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = ServerEnv::from_env()?;

    if std::env::args().nth(1).as_deref() == Some("healthcheck") {
        let host = match env.address.ip() {
            IpAddr::V4(ip) if ip.is_unspecified() => IpAddr::V4(Ipv4Addr::LOCALHOST),
            IpAddr::V6(ip) if ip.is_unspecified() => IpAddr::V6(Ipv6Addr::LOCALHOST),
            ip => ip,
        };

        let url = format!(
            "http://{}{}",
            SocketAddr::new(host, env.address.port()),
            ApiRoute::Health.path()
        );
        let healthy = reqwest::Client::new()
            .get(url)
            .timeout(HEALTHCHECK_TIMEOUT)
            .send()
            .await
            .is_ok_and(|response| response.status().is_success());

        std::process::exit(if healthy { 0 } else { 1 });
    }

    std::fs::create_dir_all(&env.data_dir)?;
    std::fs::create_dir_all(&env.log_dir)?;

    let file = miner_core::logs::daily_files(&env.log_dir)?;
    let (file, guard) = tracing_appender::non_blocking(file);
    let _file_writer = miner_core::logs::keep_file_writer(guard);
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,hyper=warn,reqwest=warn"));

    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    let log_buffer = miner_core::logs::LogBuffer::new();

    tracing_subscriber::registry()
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
        .with(miner_core::logs::LogLayer {
            buffer: log_buffer.clone(),
        })
        .init();

    tracing::info!(
        version = miner_core::VERSION,
        "starting {}",
        miner_core::APP_NAME
    );

    let manager = Manager::open(&env.data_dir, log_buffer, Shell::Server)?;

    if let Some(passphrase) = &env.vault_passphrase {
        if let Err(error) = manager.unlock(passphrase).await {
            tracing::warn!(%error, "vault passphrase from the environment was refused");
        }
    }

    manager.start().await;

    if env.short_password() {
        tracing::warn!(
            "MINER_PASSWORD has fewer than {} characters: choose a longer one, the sign-in lockout only slows a guess down",
            PASSWORD_MIN_CHARS
        );
    }

    let snapshots = SnapshotFeed::new(manager.clone());

    snapshots.start();

    let state = Arc::new(AppState {
        manager: manager.clone(),
        snapshots,
        access: env.access,
        sessions: Sessions::default(),
        attempts: SignInAttempts::default(),
        releases: Releases::new()?,
    });

    let watched = state.clone();

    tokio::spawn(async move { watched.sessions.end_idle(&watched.manager).await });

    let checking = state.clone();

    tokio::spawn(async move { checking.releases.check_regularly(&checking.manager).await });

    match &state.access {
        Access::Local { .. } => tracing::info!(
            "dashboard on http://{}, over plain HTTP: keep it on your own network",
            env.address
        ),
        Access::Public { url, .. } => tracing::info!(
            "dashboard behind HTTPS at {}, answering only what the proxy forwards to {}",
            url.origin().ascii_serialization(),
            env.address
        ),
    }

    let app = routes::router(state);
    let listener = tokio::net::TcpListener::bind(env.address).await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    manager.shutdown().await;

    Ok(())
}

// ! `docker stop` sends SIGTERM: as process 1 of a container the server would
// otherwise ignore it and be killed without stopping its accounts.
async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};

        match signal(SignalKind::terminate()) {
            Ok(mut terminate) => {
                tokio::select! {
                    _ = tokio::signal::ctrl_c() => {}
                    _ = terminate.recv() => {}
                }
            }
            Err(_) => {
                let _ = tokio::signal::ctrl_c().await;
            }
        }
    }

    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }

    tracing::info!("shutting down");
}
