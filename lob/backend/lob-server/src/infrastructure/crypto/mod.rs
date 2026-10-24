//! Cryptographic adapters (password hashing, etc.).

mod argon2_password_hasher;

pub use argon2_password_hasher::Argon2PasswordHasher;
