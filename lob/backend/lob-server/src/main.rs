//! LOB engine server binary entrypoint.

use anyhow::Context;
use lob_server::app;
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

    let addr = std::env::var("LOB_SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".into());
    let listener = TcpListener::bind(&addr)
        .await
        .with_context(|| format!("failed to bind {addr}"))?;

    info!(%addr, "lob-server listening");
    axum::serve(listener, app())
        .await
        .context("lob-server exited with error")?;

    Ok(())
}
