//! Errors for the simulation engine.

use thiserror::Error;

/// Error type for replay and simulation gateway operations.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SimEngineError {
    /// Simulation could not complete.
    #[error("simulation failed: {0}")]
    Failed(String),
}
