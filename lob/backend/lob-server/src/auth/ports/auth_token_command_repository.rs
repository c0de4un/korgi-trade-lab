//! Write-side port for the `auth_tokens` table.

use uuid::Uuid;

use crate::auth::enums::AuthEntityTypeEnum;
use crate::auth::errors::AuthTokenRepositoryError;
use crate::auth::models::AuthTokenModel;

/// CQRS command repository for auth tokens (PostgreSQL writes + cache invalidate).
#[allow(async_fn_in_trait)]
pub trait AuthTokenCommandRepository {
    /// Insert a new auth token.
    async fn create(&self, token: &AuthTokenModel) -> Result<(), AuthTokenRepositoryError>;

    /// Delete a token by primary key.
    async fn delete_by_id(&self, id: Uuid) -> Result<(), AuthTokenRepositoryError>;

    /// Delete a token by opaque Bearer value.
    async fn delete_by_value(&self, value: &str) -> Result<(), AuthTokenRepositoryError>;

    /// Delete all tokens bound to an entity (e.g. logout everywhere).
    async fn delete_by_entity(
        &self,
        entity_type: AuthEntityTypeEnum,
        entity_id: Uuid,
    ) -> Result<(), AuthTokenRepositoryError>;
}
