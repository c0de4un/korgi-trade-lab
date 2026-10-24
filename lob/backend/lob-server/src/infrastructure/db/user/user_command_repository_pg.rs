//! PostgreSQL adapter for [`UserCommandRepository`].

use sqlx::PgPool;
use uuid::Uuid;

use crate::user::enums::UserStatusEnum;
use crate::user::errors::UserRepositoryError;
use crate::user::models::UserModel;
use crate::user::ports::UserCommandRepository;

/// Write-side user repository backed by PostgreSQL.
#[derive(Clone)]
pub struct UserCommandRepositoryPg {
    pool: PgPool,
}

impl UserCommandRepositoryPg {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserCommandRepository for UserCommandRepositoryPg {
    async fn create(&self, user: &UserModel) -> Result<(), UserRepositoryError> {
        let role = i16::from(user.role.as_u8());
        let status = i16::from(user.status.as_u8());

        let result = sqlx::query!(
            r#"
            INSERT INTO users (id, email, password_hash, role, status, created_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            user.id,
            user.email,
            user.password_hash,
            role,
            status,
            user.created_at,
            user.deleted_at
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) if is_unique_violation(&e) => {
                Err(UserRepositoryError::Conflict(format!("email {}", user.email)))
            }
            Err(e) => Err(UserRepositoryError::Db(e)),
        }
    }

    async fn update(&self, user: &UserModel) -> Result<(), UserRepositoryError> {
        let role = i16::from(user.role.as_u8());
        let status = i16::from(user.status.as_u8());

        let result = sqlx::query!(
            r#"
            UPDATE users
            SET email = $2,
                password_hash = $3,
                role = $4,
                status = $5,
                deleted_at = $6
            WHERE id = $1
            "#,
            user.id,
            user.email,
            user.password_hash,
            role,
            status,
            user.deleted_at
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(done) if done.rows_affected() == 0 => Err(UserRepositoryError::NotFound),
            Ok(_) => Ok(()),
            Err(e) if is_unique_violation(&e) => {
                Err(UserRepositoryError::Conflict(format!("email {}", user.email)))
            }
            Err(e) => Err(UserRepositoryError::Db(e)),
        }
    }

    async fn soft_delete(&self, id: Uuid, deleted_at: i64) -> Result<(), UserRepositoryError> {
        let status = i16::from(UserStatusEnum::Deleted.as_u8());

        let done = sqlx::query!(
            r#"
            UPDATE users
            SET status = $2, deleted_at = $3
            WHERE id = $1
            "#,
            id,
            status,
            deleted_at
        )
        .execute(&self.pool)
        .await
        .map_err(UserRepositoryError::Db)?;

        if done.rows_affected() == 0 {
            return Err(UserRepositoryError::NotFound);
        }
        Ok(())
    }
}

fn is_unique_violation(err: &sqlx::Error) -> bool {
    match err {
        sqlx::Error::Database(db) => db.is_unique_violation(),
        _ => false,
    }
}
