//! Errors returned by password hashing / verification.

/// Failure while hashing or verifying a password.
#[derive(Debug, thiserror::Error)]
pub enum PasswordHasherError {
    #[error("password hash failed: {0}")]
    Hash(String),

    #[error("password verify failed: {0}")]
    Verify(String),
}
