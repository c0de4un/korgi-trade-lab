//! Limit order book and matching engine.
//!
//! No I/O and no async: pure deterministic domain logic.
//! CPU-bound matching must be offloaded by callers (`spawn_blocking` / OS thread pool).

mod error;

pub use error::LobCoreError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_is_stable() {
        let err = LobCoreError::InvariantViolated("crossed book".into());
        assert!(err.to_string().contains("crossed"));
    }
}
