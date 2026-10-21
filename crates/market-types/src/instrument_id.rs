//! Stable identifier for a tradeable instrument.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Instrument identifier (UUID v7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InstrumentId(Uuid);

impl InstrumentId {
    /// Create a new time-ordered instrument id.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Wrap an existing UUID (must be v7 in production data paths).
    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    /// Borrow the underlying UUID.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for InstrumentId {
    fn default() -> Self {
        Self::new()
    }
}
