//! Port for PostgreSQL-backed metadata (users, jobs, dataset descriptors).

use crate::error::StorageError;

/// Opaque dataset metadata key used until concrete models land.
pub type DatasetKey = String;

/// Port for application metadata stored in PostgreSQL.
pub trait MetaRepository {
    /// Check whether a dataset metadata record exists.
    fn dataset_exists(&self, key: &str) -> Result<bool, StorageError>;
}
