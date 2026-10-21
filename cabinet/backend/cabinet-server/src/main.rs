//! Trader cabinet server binary entrypoint.

use anyhow::Context;
use cabinet_server::app;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let addr = std::env::var("CABINET_SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8081".into());
    let listener = TcpListener::bind(&addr)
        .await
        .with_context(|| format!("failed to bind {addr}"))?;

    info!(%addr, "cabinet-server listening");
    axum::serve(listener, app())
        .await
        .context("cabinet-server exited with error")?;

    Ok(())
}
