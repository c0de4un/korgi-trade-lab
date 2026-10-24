//! Error for invalid user status strings or SMALLINT codes.

use thiserror::Error;

/// Invalid status string (API) or `u8` code (DB).
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("invalid user status: {0}")]
pub struct ParseUserStatusError(pub String);
