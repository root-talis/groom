use color_eyre::eyre::{self, Result, eyre};

use tokio::{net::TcpListener, signal};
use axum::Router;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    run_server(
        String::from("127.0.0.1:3000"),
        groom_example_auth_middleware::make_axum_router()?,
        shutdown_signal()
    ).await
}

async fn run_server<Stop>(addr: String, router: Router, stop: Stop) -> Result<()> 
where
    Stop: Future<Output = ()> + Send + 'static
{
    let listener = TcpListener::bind(addr.clone())
        .await
        .map_err(|e| -> eyre::Report {
            eyre!("failed to listen on {}: {}", addr, e)
        })?;

    tracing::info!(
        addr=listener.local_addr().expect("failed to get local_addr from tcp listener").to_string(),
        "accepting connections"
    );

    axum::serve(listener, router)
        .with_graceful_shutdown(stop)
        .await?;

    Ok(())
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
