//! CQRS query adapter: Redis cache-aside with PostgreSQL fallback.

use super::auth_token_store_pg::AuthTokenStorePg;
use crate::auth::errors::AuthTokenRepositoryError;
use crate::auth::models::AuthTokenModel;
use crate::auth::ports::AuthTokenQueryRepository;
use crate::infrastructure::redis::auth::AuthTokenRedisRepository;

/// Read-side auth token repository (Redis → PostgreSQL → warm cache).
#[derive(Clone)]
pub struct AuthTokenQueryRepositoryImpl {
    pg: AuthTokenStorePg,
    redis: AuthTokenRedisRepository,
}

impl AuthTokenQueryRepositoryImpl {
    pub fn new(pg: AuthTokenStorePg, redis: AuthTokenRedisRepository) -> Self {
        Self { pg, redis }
    }

    fn is_valid(token: &AuthTokenModel, now_ns: i64) -> bool {
        match token.valid_due {
            None => true,
            Some(due) => due >= now_ns,
        }
    }
}

fn wall_now_ns() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| i64::try_from(d.as_nanos()).unwrap_or(i64::MAX))
        .unwrap_or(0)
}

impl AuthTokenQueryRepository for AuthTokenQueryRepositoryImpl {
    async fn find_by_value(
        &self,
        value: &str,
    ) -> Result<Option<AuthTokenModel>, AuthTokenRepositoryError> {
        match self.redis.get(value).await {
            Ok(Some(token)) => return Ok(Some(token)),
            Ok(None) => {}
            Err(e) => {
                tracing::warn!(error = %e, "auth token redis get failed; falling back to postgres");
            }
        }

        let token = self.pg.find_by_value(value).await?;
        if let Some(ref t) = token {
            if let Err(e) = self.redis.set(t, wall_now_ns()).await {
                tracing::warn!(error = %e, "auth token redis warm after pg miss failed");
            }
        }
        Ok(token)
    }

    async fn find_valid_by_value(
        &self,
        value: &str,
        now_ns: i64,
    ) -> Result<Option<AuthTokenModel>, AuthTokenRepositoryError> {
        let token = self.find_by_value(value).await?;
        Ok(token.filter(|t| Self::is_valid(t, now_ns)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::enums::AuthEntityTypeEnum;
    use uuid::Uuid;

    fn sample(valid_due: Option<i64>) -> AuthTokenModel {
        AuthTokenModel {
            id: Uuid::nil(),
            entity_type: AuthEntityTypeEnum::User,
            entity_id: Uuid::nil(),
            value: "tok".to_owned(),
            created_at: 0,
            valid_due,
        }
    }

    #[test]
    fn valid_when_no_expiry() {
        assert!(AuthTokenQueryRepositoryImpl::is_valid(&sample(None), 999));
    }

    #[test]
    fn valid_when_due_in_future() {
        assert!(AuthTokenQueryRepositoryImpl::is_valid(&sample(Some(100)), 50));
    }

    #[test]
    fn invalid_when_due_in_past() {
        assert!(!AuthTokenQueryRepositoryImpl::is_valid(&sample(Some(10)), 50));
    }

    #[test]
    fn valid_when_due_equals_now() {
        assert!(AuthTokenQueryRepositoryImpl::is_valid(&sample(Some(50)), 50));
    }
}
