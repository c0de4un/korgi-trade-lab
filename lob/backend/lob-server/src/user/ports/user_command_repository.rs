//! Write-side port for the `users` table.

use uuid::Uuid;

use crate::user::errors::UserRepositoryError;
use crate::user::models::UserModel;

/// CQRS command repository for users (PostgreSQL writes).
#[allow(async_fn_in_trait)]
pub trait UserCommandRepository {
    /// Insert a new user row.
    async fn create(&self, user: &UserModel) -> Result<(), UserRepositoryError>;

    /// Replace mutable fields of an existing user.
    async fn update(&self, user: &UserModel) -> Result<(), UserRepositoryError>;

    /// Soft-delete: set status to Deleted and `deleted_at`.
    async fn soft_delete(&self, id: Uuid, deleted_at: i64) -> Result<(), UserRepositoryError>;
}
