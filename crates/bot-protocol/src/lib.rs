//! Bot REST push/pull DTOs and message signing helpers.
//!
//! gRPC, WebSocket, and FIX channels are stubs at the server layer (post-MVP).

mod error;

pub use error::BotProtocolError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_is_stable() {
        let err = BotProtocolError::InvalidMessage("bad signature".into());
        assert!(err.to_string().contains("signature"));
    }
}
