//! Read-side port for the `users` table.

use uuid::Uuid;

use crate::user::errors::UserRepositoryError;
use crate::user::models::UserModel;

/// CQRS query repository for users (PostgreSQL reads).
#[allow(async_fn_in_trait)]
pub trait UserQueryRepository {
    /// Load a user by primary key.
    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserModel>, UserRepositoryError>;

    /// Load a user by unique email.
    async fn find_by_email(&self, email: &str) -> Result<Option<UserModel>, UserRepositoryError>;

    /// List non-deleted users with pagination.
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<UserModel>, UserRepositoryError>;
}
