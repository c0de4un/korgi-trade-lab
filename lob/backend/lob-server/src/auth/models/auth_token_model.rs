//! Auth token persistence model (maps to `auth_tokens` table).

use uuid::Uuid;

use crate::auth::enums::AuthEntityTypeEnum;

/// Opaque Bearer token bound to an entity (typically a user).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AuthTokenModel {
    pub id: Uuid,
    pub entity_type: AuthEntityTypeEnum,
    pub entity_id: Uuid,
    /// Opaque token value sent as `Authorization: Bearer …`.
    pub value: String,
    /// UTC timestamp in nanoseconds.
    pub created_at: i64,
    /// UTC expiry in nanoseconds; `None` means no expiry.
    pub valid_due: Option<i64>,
}
