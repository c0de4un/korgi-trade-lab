//! Errors for the limit order book core.

use thiserror::Error;

/// Error type for LOB matching and book operations.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum LobCoreError {
    /// Operation rejected by book invariants.
    #[error("order book invariant violated: {0}")]
    InvariantViolated(String),
}
