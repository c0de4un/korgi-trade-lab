//! Errors returned by user command/query repositories.

/// Failure while persisting or loading a user.
#[derive(Debug, thiserror::Error)]
pub enum UserRepositoryError {
    #[error("user not found")]
    NotFound,

    #[error("user conflict: {0}")]
    Conflict(String),

    #[error("database error: {0}")]
    Db(#[source] sqlx::Error),

    #[error("row mapping error: {0}")]
    Mapping(String),
}
