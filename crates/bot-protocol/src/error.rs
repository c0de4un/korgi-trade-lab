//! Errors for bot protocol DTOs and signing.

use thiserror::Error;

/// Error type for bot protocol operations.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum BotProtocolError {
    /// Message failed validation or signature check.
    #[error("bot protocol error: {0}")]
    InvalidMessage(String),
}
