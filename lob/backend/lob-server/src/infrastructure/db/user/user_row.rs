//! PostgreSQL row shape for the `users` table (sqlx `query_as!`).

use uuid::Uuid;

use crate::user::enums::{UserRoleEnum, UserStatusEnum};
use crate::user::errors::UserRepositoryError;
use crate::user::models::UserModel;

/// Flat DB row before mapping into [`UserModel`].
pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: i16,
    pub status: i16,
    pub created_at: i64,
    pub deleted_at: Option<i64>,
}

impl UserRow {
    pub fn into_model(self) -> Result<UserModel, UserRepositoryError> {
        let role_u8 = u8::try_from(self.role).map_err(|_| {
            UserRepositoryError::Mapping(format!("role out of u8 range: {}", self.role))
        })?;
        let status_u8 = u8::try_from(self.status).map_err(|_| {
            UserRepositoryError::Mapping(format!("status out of u8 range: {}", self.status))
        })?;

        let role = UserRoleEnum::try_from(role_u8)
            .map_err(|e| UserRepositoryError::Mapping(e.to_string()))?;
        let status = UserStatusEnum::try_from(status_u8)
            .map_err(|e| UserRepositoryError::Mapping(e.to_string()))?;

        Ok(UserModel {
            id: self.id,
            email: self.email,
            password_hash: self.password_hash,
            role,
            status,
            created_at: self.created_at,
            deleted_at: self.deleted_at,
        })
    }
}
