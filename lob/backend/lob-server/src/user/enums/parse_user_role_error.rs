//! Error for invalid user role strings or SMALLINT codes.

use thiserror::Error;

/// Invalid role string (API) or `u8` code (DB).
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("invalid user role: {0}")]
pub struct ParseUserRoleError(pub String);
