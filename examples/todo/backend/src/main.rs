use color_eyre::eyre::Result;

use tower_http::trace::TraceLayer;
use tokio::signal;
use axum::{Extension, Router};

use crate::{bootstrap::Bootstrap, server::run_server};

mod bootstrap;
mod server;

mod controller;
mod service;
mod repository;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Bootstrap::new();

    run_server(
        String::from("127.0.0.1:8888"),
        make_router(app)?,
        shutdown_signal()
    ).await
}

fn make_router(app: Bootstrap) -> Result<Router> {
    Ok(
        controller::setup_router(Router::new(), true)?
            .layer(Extension(app.task_service))
            .layer(TraceLayer::new_for_http())
    )
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
