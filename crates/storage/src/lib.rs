//! Storage ports: filesystem chunks + PostgreSQL metadata.
//!
//! Adapters (sqlx, FS) are implemented in later milestones.

mod chunk_store;
mod error;
mod meta_repository;

pub use chunk_store::{ChunkId, ChunkStore};
pub use error::StorageError;
pub use meta_repository::{DatasetKey, MetaRepository};

#[cfg(test)]
mod tests {
    use super::*;

    struct MemoryChunkStore;

    impl ChunkStore for MemoryChunkStore {
        fn put(&self, bytes: &[u8]) -> Result<ChunkId, StorageError> {
            let _ = bytes;
            Ok("chunk-0".into())
        }

        fn get(&self, id: &str) -> Result<Vec<u8>, StorageError> {
            if id == "chunk-0" {
                Ok(vec![1, 2, 3])
            } else {
                Err(StorageError::Chunk(format!("missing {id}")))
            }
        }
    }

    #[test]
    fn memory_chunk_store_roundtrip_shape() {
        let store = MemoryChunkStore;
        let id = store.put(&[9]).expect("put");
        assert_eq!(id, "chunk-0");
        assert_eq!(store.get(&id).expect("get"), vec![1, 2, 3]);
    }
}
