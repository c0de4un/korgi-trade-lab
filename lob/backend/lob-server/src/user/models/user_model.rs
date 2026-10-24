//! User persistence model (maps to `users` table).

use uuid::Uuid;

use crate::user::enums::{UserRoleEnum, UserStatusEnum};

/// LOB user account row.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: UserRoleEnum,
    pub status: UserStatusEnum,
    /// UTC timestamp in nanoseconds.
    pub created_at: i64,
    /// UTC timestamp in nanoseconds; set on soft-delete.
    pub deleted_at: Option<i64>,
}
