//! Errors returned by auth-token command/query repositories.

/// Failure while persisting or loading an auth token.
#[derive(Debug, thiserror::Error)]
pub enum AuthTokenRepositoryError {
    #[error("auth token not found")]
    NotFound,

    #[error("auth token conflict: {0}")]
    Conflict(String),

    #[error("database error: {0}")]
    Db(#[source] sqlx::Error),

    #[error("cache error: {0}")]
    Cache(String),

    #[error("row mapping error: {0}")]
    Mapping(String),
}
