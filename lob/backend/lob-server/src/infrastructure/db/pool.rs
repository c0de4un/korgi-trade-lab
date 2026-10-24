//! PostgreSQL pool creation and migration runner.

use std::path::{Path, PathBuf};

use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::info;

use super::config::DbConfig;

/// Load `lob/backend/.env` if present (repo root or crate-relative).
pub fn load_env_file() {
    let candidates = [
        PathBuf::from("lob/backend/.env"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.env"),
    ];

    for path in candidates {
        if path.is_file() {
            // Tracing may not be initialized yet; keep this silent on success.
            if dotenvy::from_path(&path).is_ok() {
                return;
            }
        }
    }
}

/// Create a PgPool from [`DbConfig`].
pub async fn create_pool(config: &DbConfig) -> anyhow::Result<PgPool> {
    let url = config.database_url();
    let min_connections = config.max_idle_conns.min(config.max_open_conns);

    let pool = PgPoolOptions::new()
        .max_connections(config.max_open_conns)
        .min_connections(min_connections)
        .acquire_timeout(config.timeout)
        .max_lifetime(config.conn_max_lifetime)
        .connect(&url)
        .await
        .context("failed to connect to PostgreSQL")?;

    info!(
        max_open = config.max_open_conns,
        min = min_connections,
        "PostgreSQL pool ready"
    );

    Ok(pool)
}

/// Apply sqlx migrations from the lob-server `migrations/` directory.
pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    let migrations_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    info!(path = %migrations_dir.display(), "running database migrations");

    sqlx::migrate::Migrator::new(migrations_dir)
        .await
        .context("failed to load migrations")?
        .run(pool)
        .await
        .context("failed to apply migrations")?;

    info!("database migrations applied");
    Ok(())
}
