//! Market data import adapters.

mod error;
mod import_adapter;

pub use error::ImporterError;
pub use import_adapter::ImportAdapter;

#[cfg(test)]
mod tests {
    use super::*;

    struct StubAdapter;

    impl ImportAdapter for StubAdapter {
        fn name(&self) -> &'static str {
            "stub"
        }

        fn validate_source(&self, _source: &str) -> Result<(), ImporterError> {
            Ok(())
        }
    }

    #[test]
    fn stub_adapter_validates() {
        let adapter = StubAdapter;
        assert_eq!(adapter.name(), "stub");
        assert!(adapter.validate_source("fixture.csv").is_ok());
    }
}
