//! Replay engine, SimGateway, latency and fee models.

mod error;

pub use error::SimEngineError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_is_stable() {
        let err = SimEngineError::Failed("no dataset".into());
        assert!(err.to_string().contains("dataset"));
    }
}
