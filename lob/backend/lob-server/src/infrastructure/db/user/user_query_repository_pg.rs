//! PostgreSQL adapter for [`UserQueryRepository`].

use sqlx::PgPool;
use uuid::Uuid;

use super::user_row::UserRow;
use crate::user::errors::UserRepositoryError;
use crate::user::models::UserModel;
use crate::user::ports::UserQueryRepository;

/// Read-side user repository backed by PostgreSQL.
#[derive(Clone)]
pub struct UserQueryRepositoryPg {
    pool: PgPool,
}

impl UserQueryRepositoryPg {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserQueryRepository for UserQueryRepositoryPg {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserModel>, UserRepositoryError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT id, email, password_hash, role, status, created_at, deleted_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(UserRepositoryError::Db)?;

        row.map(UserRow::into_model).transpose()
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<UserModel>, UserRepositoryError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT id, email, password_hash, role, status, created_at, deleted_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(UserRepositoryError::Db)?;

        row.map(UserRow::into_model).transpose()
    }

    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<UserModel>, UserRepositoryError> {
        let rows = sqlx::query_as!(
            UserRow,
            r#"
            SELECT id, email, password_hash, role, status, created_at, deleted_at
            FROM users
            WHERE deleted_at IS NULL
            ORDER BY created_at
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(UserRepositoryError::Db)?;

        rows.into_iter().map(UserRow::into_model).collect()
    }
}
