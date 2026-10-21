//! Port for converting vendor market data into canonical events.

use crate::error::ImporterError;

/// Adapter that reads a vendor-specific payload and emits canonical market events.
///
/// Implementations (MOEX, Binance, generic CSV) live in this crate in later milestones.
pub trait ImportAdapter {
    /// Human-readable adapter name (e.g. `"generic-csv"`).
    fn name(&self) -> &'static str;

    /// Validate that the adapter can process the given source descriptor.
    ///
    /// M0: no real parsing; returns `Ok(())` for stubs.
    fn validate_source(&self, source: &str) -> Result<(), ImporterError>;
}
