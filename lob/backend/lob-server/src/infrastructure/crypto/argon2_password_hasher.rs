//! Argon2id adapter for [`crate::user::ports::PasswordHasher`].

use argon2::password_hash::{PasswordHash, PasswordHasher as _, PasswordVerifier as _, SaltString};
use argon2::Argon2;
use argon2::password_hash::rand_core::OsRng;

use crate::user::errors::PasswordHasherError;
use crate::user::ports::PasswordHasher;

/// Production password hasher using Argon2id (default `argon2` crate algorithm).
#[derive(Debug, Clone, Default)]
pub struct Argon2PasswordHasher;

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordHasher for Argon2PasswordHasher {
    async fn hash(&self, password: &str) -> Result<String, PasswordHasherError> {
        let password = password.to_owned();
        tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            argon2
                .hash_password(password.as_bytes(), &salt)
                .map(|h| h.to_string())
                .map_err(|e| PasswordHasherError::Hash(e.to_string()))
        })
        .await
        .map_err(|e| PasswordHasherError::Hash(format!("join error: {e}")))?
    }

    async fn verify(&self, password: &str, password_hash: &str) -> Result<bool, PasswordHasherError> {
        let password = password.to_owned();
        let password_hash = password_hash.to_owned();
        tokio::task::spawn_blocking(move || {
            let parsed = PasswordHash::new(&password_hash)
                .map_err(|e| PasswordHasherError::Verify(e.to_string()))?;
            match Argon2::default().verify_password(password.as_bytes(), &parsed) {
                Ok(()) => Ok(true),
                Err(argon2::password_hash::Error::Password) => Ok(false),
                Err(e) => Err(PasswordHasherError::Verify(e.to_string())),
            }
        })
        .await
        .map_err(|e| PasswordHasherError::Verify(format!("join error: {e}")))?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn hash_then_verify_roundtrip() {
        let hasher = Argon2PasswordHasher::new();
        let hash = hasher
            .hash("correct-horse-battery")
            .await
            .expect("hash should succeed");

        assert!(
            hasher
                .verify("correct-horse-battery", &hash)
                .await
                .expect("verify should succeed")
        );
        assert!(
            !hasher
                .verify("wrong-password", &hash)
                .await
                .expect("verify mismatch should succeed")
        );
    }
}
