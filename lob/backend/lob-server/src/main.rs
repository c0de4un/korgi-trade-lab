//! LOB engine server binary entrypoint.

use anyhow::Context;
use lob_server::{AppState, DbConfig, app, create_pool, load_env_file, run_migrations};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env before tracing so RUST_LOG from the file is visible to EnvFilter.
    load_env_file();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let db_config = DbConfig::from_env().context("invalid database configuration")?;
    let pool = create_pool(&db_config).await?;
    run_migrations(&pool).await?;

    let state = AppState { pool };
    let addr = std::env::var("LOB_SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".into());
    let listener = TcpListener::bind(&addr)
        .await
        .with_context(|| format!("failed to bind {addr}"))?;

    info!(%addr, "lob-server listening");
    axum::serve(listener, app(state))
        .await
        .context("lob-server exited with error")?;

    Ok(())
}
