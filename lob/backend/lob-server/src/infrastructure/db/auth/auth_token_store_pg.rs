//! Low-level PostgreSQL access for `auth_tokens` (no Redis).

use sqlx::PgPool;
use uuid::Uuid;

use super::auth_token_row::AuthTokenRow;
use crate::auth::enums::AuthEntityTypeEnum;
use crate::auth::errors::AuthTokenRepositoryError;
use crate::auth::models::AuthTokenModel;

/// Direct PostgreSQL driver for auth tokens.
#[derive(Clone)]
pub struct AuthTokenStorePg {
    pool: PgPool,
}

impl AuthTokenStorePg {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, token: &AuthTokenModel) -> Result<(), AuthTokenRepositoryError> {
        let entity_type = i16::from(token.entity_type.as_u8());

        let result = sqlx::query!(
            r#"
            INSERT INTO auth_tokens (id, entity_type, entity_id, value, created_at, valid_due)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            token.id,
            entity_type,
            token.entity_id,
            token.value,
            token.created_at,
            token.valid_due
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) if is_value_unique_violation(&e) => Err(AuthTokenRepositoryError::Conflict(
                "auth token value already exists".to_owned(),
            )),
            Err(e) => Err(AuthTokenRepositoryError::Db(e)),
        }
    }

    pub async fn find_by_value(
        &self,
        value: &str,
    ) -> Result<Option<AuthTokenModel>, AuthTokenRepositoryError> {
        let row = sqlx::query_as!(
            AuthTokenRow,
            r#"
            SELECT id, entity_type, entity_id, value, created_at, valid_due
            FROM auth_tokens
            WHERE value = $1
            "#,
            value
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AuthTokenRepositoryError::Db)?;

        row.map(AuthTokenRow::into_model).transpose()
    }

    /// Delete by id; returns the deleted opaque value for cache invalidation.
    pub async fn delete_by_id(&self, id: Uuid) -> Result<Option<String>, AuthTokenRepositoryError> {
        let row = sqlx::query!(
            r#"
            DELETE FROM auth_tokens
            WHERE id = $1
            RETURNING value
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AuthTokenRepositoryError::Db)?;

        Ok(row.map(|r| r.value))
    }

    /// Delete by opaque value; returns whether a row was deleted.
    pub async fn delete_by_value(&self, value: &str) -> Result<bool, AuthTokenRepositoryError> {
        let done = sqlx::query!(
            r#"
            DELETE FROM auth_tokens
            WHERE value = $1
            "#,
            value
        )
        .execute(&self.pool)
        .await
        .map_err(AuthTokenRepositoryError::Db)?;

        Ok(done.rows_affected() > 0)
    }

    /// Delete all tokens for an entity; returns deleted opaque values.
    pub async fn delete_by_entity(
        &self,
        entity_type: AuthEntityTypeEnum,
        entity_id: Uuid,
    ) -> Result<Vec<String>, AuthTokenRepositoryError> {
        let entity_type_i16 = i16::from(entity_type.as_u8());

        let rows = sqlx::query!(
            r#"
            DELETE FROM auth_tokens
            WHERE entity_type = $1 AND entity_id = $2
            RETURNING value
            "#,
            entity_type_i16,
            entity_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AuthTokenRepositoryError::Db)?;

        Ok(rows.into_iter().map(|r| r.value).collect())
    }
}

fn is_value_unique_violation(err: &sqlx::Error) -> bool {
    match err {
        sqlx::Error::Database(db) => db.is_unique_violation(),
        _ => false,
    }
}
