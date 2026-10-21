//! Domain errors for market types.
//!
//! Prices and quantities must use scaled integers (`i64` / `u64`), never `f32`/`f64`.
//! Timestamps are UTC nanoseconds as `i64`.

use thiserror::Error;

/// Error type for market-types validation and construction failures.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum MarketTypesError {
    /// A value failed domain validation.
    #[error("invalid market value: {0}")]
    InvalidValue(String),
}
