//! PostgreSQL row shape for the `auth_tokens` table (sqlx `query_as!`).

use uuid::Uuid;

use crate::auth::enums::AuthEntityTypeEnum;
use crate::auth::errors::AuthTokenRepositoryError;
use crate::auth::models::AuthTokenModel;

/// Flat DB row before mapping into [`AuthTokenModel`].
pub struct AuthTokenRow {
    pub id: Uuid,
    pub entity_type: i16,
    pub entity_id: Uuid,
    pub value: String,
    pub created_at: i64,
    pub valid_due: Option<i64>,
}

impl AuthTokenRow {
    pub fn into_model(self) -> Result<AuthTokenModel, AuthTokenRepositoryError> {
        let entity_type_u8 = u8::try_from(self.entity_type).map_err(|_| {
            AuthTokenRepositoryError::Mapping(format!(
                "entity_type out of u8 range: {}",
                self.entity_type
            ))
        })?;
        let entity_type = AuthEntityTypeEnum::try_from(entity_type_u8)
            .map_err(|e| AuthTokenRepositoryError::Mapping(e.to_string()))?;

        Ok(AuthTokenModel {
            id: self.id,
            entity_type,
            entity_id: self.entity_id,
            value: self.value,
            created_at: self.created_at,
            valid_due: self.valid_due,
        })
    }
}
