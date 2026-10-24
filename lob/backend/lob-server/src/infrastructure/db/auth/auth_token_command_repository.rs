//! CQRS command adapter: PostgreSQL write + Redis invalidate.

use uuid::Uuid;

use super::auth_token_store_pg::AuthTokenStorePg;
use crate::auth::enums::AuthEntityTypeEnum;
use crate::auth::errors::AuthTokenRepositoryError;
use crate::auth::models::AuthTokenModel;
use crate::auth::ports::AuthTokenCommandRepository;
use crate::infrastructure::redis::auth::AuthTokenRedisRepository;

/// Write-side auth token repository (Postgres source of truth + cache invalidate).
#[derive(Clone)]
pub struct AuthTokenCommandRepositoryImpl {
    pg: AuthTokenStorePg,
    redis: AuthTokenRedisRepository,
}

impl AuthTokenCommandRepositoryImpl {
    pub fn new(pg: AuthTokenStorePg, redis: AuthTokenRedisRepository) -> Self {
        Self { pg, redis }
    }

    async fn invalidate_value(&self, value: &str) {
        if let Err(e) = self.redis.delete(value).await {
            tracing::warn!(error = %e, "auth token redis invalidate failed");
        }
    }
}

impl AuthTokenCommandRepository for AuthTokenCommandRepositoryImpl {
    async fn create(&self, token: &AuthTokenModel) -> Result<(), AuthTokenRepositoryError> {
        self.pg.insert(token).await?;

        let now_ns = token.created_at;
        if let Err(e) = self.redis.set(token, now_ns).await {
            tracing::warn!(error = %e, "auth token redis warm after create failed");
        }
        Ok(())
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<(), AuthTokenRepositoryError> {
        let value = self.pg.delete_by_id(id).await?;
        match value {
            Some(v) => {
                self.invalidate_value(&v).await;
                Ok(())
            }
            None => Err(AuthTokenRepositoryError::NotFound),
        }
    }

    async fn delete_by_value(&self, value: &str) -> Result<(), AuthTokenRepositoryError> {
        let deleted = self.pg.delete_by_value(value).await?;
        if !deleted {
            return Err(AuthTokenRepositoryError::NotFound);
        }
        self.invalidate_value(value).await;
        Ok(())
    }

    async fn delete_by_entity(
        &self,
        entity_type: AuthEntityTypeEnum,
        entity_id: Uuid,
    ) -> Result<(), AuthTokenRepositoryError> {
        let values = self.pg.delete_by_entity(entity_type, entity_id).await?;
        for value in values {
            self.invalidate_value(&value).await;
        }
        Ok(())
    }
}
