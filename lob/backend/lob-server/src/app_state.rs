//! Shared Axum application state.

use sqlx::PgPool;

/// Runtime state shared across HTTP handlers.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}
