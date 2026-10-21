//! Canonical market events and instrument specifications.
//!
//! # Conventions
//! - Prices and quantities: exact integers scaled by tick/lot size (never `f32`/`f64`).
//! - Timestamps: UTC nanoseconds as `i64`.
//! - Identifiers: UUID v7.

mod error;
mod instrument_id;

pub use error::MarketTypesError;
pub use instrument_id::InstrumentId;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_is_stable() {
        let err = MarketTypesError::InvalidValue("tick".into());
        assert!(err.to_string().contains("tick"));
    }

    #[test]
    fn instrument_id_is_uuid_v7() {
        let id = InstrumentId::new();
        assert_eq!(id.as_uuid().get_version_num(), 7);
    }
}
