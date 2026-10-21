//! Errors for market data importers.

use thiserror::Error;

/// Error type for import adapters.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ImporterError {
    /// Source data could not be parsed or validated.
    #[error("import failed: {0}")]
    Failed(String),
}
