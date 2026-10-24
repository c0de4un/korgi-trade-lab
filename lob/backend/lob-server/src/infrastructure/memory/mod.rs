//! In-memory infrastructure adapters.
//!
//! Used by unit tests (and optionally demos). Not a replacement for PostgreSQL in production.

pub mod user;

pub use user::InMemoryUserQueryRepository;
