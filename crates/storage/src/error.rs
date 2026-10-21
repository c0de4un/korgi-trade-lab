//! Errors for storage ports and adapters.

use thiserror::Error;

/// Error type for metadata and chunk storage.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum StorageError {
    /// Chunk read/write on the filesystem failed.
    #[error("chunk store error: {0}")]
    Chunk(String),

    /// Metadata repository operation failed.
    #[error("meta repository error: {0}")]
    Meta(String),
}
