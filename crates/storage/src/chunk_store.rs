//! Port for filesystem-backed historical market data chunks.
//!
//! Chunks live on disk (compressed blobs). PostgreSQL stores only metadata.

use crate::error::StorageError;

/// Identifier of a stored market-data chunk (opaque to callers in M0).
pub type ChunkId = String;

/// Port for reading and writing market-data chunks on the filesystem.
pub trait ChunkStore {
    /// Persist chunk bytes and return an assigned identifier.
    fn put(&self, bytes: &[u8]) -> Result<ChunkId, StorageError>;

    /// Load chunk bytes by identifier.
    fn get(&self, id: &str) -> Result<Vec<u8>, StorageError>;
}
