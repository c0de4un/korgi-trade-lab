//! Error for invalid auth entity_type strings or SMALLINT codes.

use thiserror::Error;

/// Invalid entity_type string (API) or `u8` code (DB).
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("invalid auth entity type: {0}")]
pub struct ParseAuthEntityTypeError(pub String);
