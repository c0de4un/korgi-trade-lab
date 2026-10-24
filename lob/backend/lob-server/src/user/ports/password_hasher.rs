//! Port for password hashing and verification (Argon2id in production).

use crate::user::errors::PasswordHasherError;

/// Hashes and verifies passwords without exposing a concrete crypto library.
#[allow(async_fn_in_trait)]
pub trait PasswordHasher {
    /// Produce a PHC-encoded password hash suitable for storage.
    async fn hash(&self, password: &str) -> Result<String, PasswordHasherError>;

    /// Check a plaintext password against a stored hash.
    ///
    /// Returns `Ok(true)` on match, `Ok(false)` on mismatch, and `Err` on
    /// malformed hash or internal crypto failures.
    async fn verify(&self, password: &str, password_hash: &str) -> Result<bool, PasswordHasherError>;
}
