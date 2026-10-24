//! Polymorphic owner type for auth tokens.

use std::fmt;
use std::str::FromStr;

use super::parse_auth_entity_type_error::ParseAuthEntityTypeError;

/// Entity kind referenced by [`crate::auth::models::AuthTokenModel`].
///
/// DB stores as `SMALLINT`: `0` user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthEntityTypeEnum {
    User,
}

impl AuthEntityTypeEnum {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
        }
    }

    /// Stable DB / wire-independent discriminant (`SMALLINT` / `u8`).
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::User => 0,
        }
    }
}

impl Default for AuthEntityTypeEnum {
    fn default() -> Self {
        Self::User
    }
}

impl AsRef<str> for AuthEntityTypeEnum {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AuthEntityTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for AuthEntityTypeEnum {
    type Err = ParseAuthEntityTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Self::User),
            other => Err(ParseAuthEntityTypeError(other.to_owned())),
        }
    }
}

impl TryFrom<&str> for AuthEntityTypeEnum {
    type Error = ParseAuthEntityTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<u8> for AuthEntityTypeEnum {
    type Error = ParseAuthEntityTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::User),
            other => Err(ParseAuthEntityTypeError(other.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_user() {
        assert_eq!(AuthEntityTypeEnum::default(), AuthEntityTypeEnum::User);
        assert_eq!(
            "user".parse::<AuthEntityTypeEnum>().unwrap(),
            AuthEntityTypeEnum::User
        );
    }

    #[test]
    fn roundtrip_entity_type_u8() {
        assert_eq!(AuthEntityTypeEnum::User.as_u8(), 0);
        assert_eq!(
            AuthEntityTypeEnum::try_from(0u8).unwrap(),
            AuthEntityTypeEnum::User
        );
        assert!(AuthEntityTypeEnum::try_from(1u8).is_err());
    }
}
