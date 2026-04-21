use axum::Router;
use color_eyre::eyre::{self, Result, eyre};
use tokio::net::TcpListener;

/// Runs HTTP server until stop signal is received.
pub async fn run_server<Stop>(addr: String, router: Router, stop: Stop) -> Result<()> 
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
