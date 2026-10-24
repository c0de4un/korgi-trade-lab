//! Errors returned by [`crate::user::services::UserService`].

use super::password_hasher_error::PasswordHasherError;
use super::user_repository_error::UserRepositoryError;

/// Failure while authenticating or managing a user via the application service.
#[derive(Debug, thiserror::Error)]
pub enum UserServiceError {
    #[error("user not found")]
    NotFound,

    #[error("invalid password")]
    InvalidPassword,

    #[error("user is inactive")]
    Inactive,

    #[error(transparent)]
    Repository(#[from] UserRepositoryError),

    #[error(transparent)]
    Password(#[from] PasswordHasherError),
}
