//! Errors for the FIX codec stub.

use thiserror::Error;

/// Error type for FIX encode/decode (stub until M8+).
#[derive(Debug, Error, PartialEq, Eq)]
pub enum FixCodecError {
    /// FIX support is not implemented yet.
    #[error("FIX codec is not implemented")]
    NotImplemented,
}
