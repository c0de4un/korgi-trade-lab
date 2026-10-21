//! FIX protocol codec stub (post-MVP).

mod error;

pub use error::FixCodecError;

/// Placeholder encode entry point until FIX is implemented.
pub fn encode_stub(_payload: &[u8]) -> Result<Vec<u8>, FixCodecError> {
    Err(FixCodecError::NotImplemented)
}

/// Placeholder decode entry point until FIX is implemented.
pub fn decode_stub(_payload: &[u8]) -> Result<Vec<u8>, FixCodecError> {
    Err(FixCodecError::NotImplemented)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_stub_returns_not_implemented() {
        assert_eq!(encode_stub(b"x"), Err(FixCodecError::NotImplemented));
    }

    #[test]
    fn decode_stub_returns_not_implemented() {
        assert_eq!(decode_stub(b"x"), Err(FixCodecError::NotImplemented));
    }
}
