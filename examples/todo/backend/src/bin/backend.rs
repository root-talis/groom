use std::env;
use color_eyre::eyre::{Result, eyre};

use groom_example_todo_backend::{CorsOrigin, make_router};
use serde::Serialize;
use tokio::signal;

use groom_example_todo_backend::{bootstrap::Bootstrap, server::run_server};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let cors_origin: CorsOrigin = env::var("CORS_ORIGIN").unwrap_or_else(|_| "any".to_owned()).into();
    let serve_spec = match env::var("SERVE_SPEC").unwrap_or_else(|_| "false".to_owned()).to_lowercase().as_str() {
        "true"  => Ok(true),
        "false" => Ok(false),
        v       => Err(eyre!("invalid value `{}` for SERVE_SPEC, only `true` and `false` are allowed", v)),
    }?;

    info!(?cors_origin, "CORS_ORIGIN is set");
    info!(?serve_spec, "SERVE_SPEC is set");

    let app = Bootstrap::new();

    run_server(
        String::from("127.0.0.1:8888"),
        make_router(app, serve_spec, cors_origin)?,
        shutdown_signal()
    ).await
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    let signal = tokio::select! {
        _ = ctrl_c => "ctrl+c",
        _ = terminate => "termination signal",
    };

    tracing::info!(reason=signal, "shutting down")
}
