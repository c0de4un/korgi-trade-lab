//! LOB engine HTTP server library (Axum adapters).

use axum::{Router, routing::get};

pub mod auth;
pub mod infrastructure;
pub mod user;

mod app_state;

pub use app_state::AppState;
pub use infrastructure::db::{DbConfig, create_pool, load_env_file, run_migrations};
pub use infrastructure::redis::{RedisConfig, connect as connect_redis};

// Workspace wiring for later milestones (REST adapters, stubs).
const _: fn() = || {
    let _ = std::any::type_name::<bot_protocol::BotProtocolError>();
    let _ = std::any::type_name::<fix_codec::FixCodecError>();
    let _ = std::any::type_name::<importers::ImporterError>();
    let _ = std::any::type_name::<sim_engine::SimEngineError>();
    let _ = std::any::type_name::<storage::StorageError>();
};

/// Build the HTTP router for the LOB server.
pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .with_state(state)
}

async fn healthz() -> &'static str {
    "ok"
}

#[cfg(test)]
mod tests {
    use super::healthz;

    #[tokio::test]
    async fn healthz_returns_ok() {
        assert_eq!(healthz().await, "ok");
    }
}
