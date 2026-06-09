use color_eyre::eyre::Result;

use clap::Parser;

use groom_example_todo_backend::{CorsOrigin, make_router};
use tokio::signal;

use groom_example_todo_backend::{bootstrap::Bootstrap, server::run_server};
use tracing::info;


#[derive(Parser)]
struct Args {
    /// Whether to serve OpenAPI spec and Swagger UI. This is useful for development and testing, but should be disabled in production.
    #[arg(long, env = "SERVE_SPEC", default_value = "false")]
    serve_spec: bool,

    /// Which origins should be allowed for CORS. Can be either `any` or a comma-separated list of origins.
    #[arg(long, env = "CORS_ORIGIN")]
    cors_origin: Option<CorsOrigin>,

    #[arg(long, env = "LISTEN_ADDR", default_value = "127.0.0.1:8888")]
    listen_addr: String,
}


#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let args = Args::parse();

    info!(?args.cors_origin, "CORS_ORIGIN is set");
    info!(?args.serve_spec, "SERVE_SPEC is set");
    info!(?args.listen_addr, "LISTEN_ADDR is set");

    let app = Bootstrap::new();

    run_server(
        args.listen_addr,
        make_router(app, args.serve_spec, args.cors_origin)?,
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
