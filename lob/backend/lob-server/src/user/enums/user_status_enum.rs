//! Lifecycle status of a user account.

use std::fmt;
use std::str::FromStr;

use super::parse_user_status_error::ParseUserStatusError;

/// Account status for [`crate::user::models::UserModel`].
///
/// DB stores as `SMALLINT`: `0` active, `1` blocked, `2` deleted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatusEnum {
    Active,
    Blocked,
    Deleted,
}

impl UserStatusEnum {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Blocked => "blocked",
            Self::Deleted => "deleted",
        }
    }

    /// Stable DB / wire-independent discriminant (`SMALLINT` / `u8`).
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Active => 0,
            Self::Blocked => 1,
            Self::Deleted => 2,
        }
    }
}

impl AsRef<str> for UserStatusEnum {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for UserStatusEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for UserStatusEnum {
    type Err = ParseUserStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "blocked" => Ok(Self::Blocked),
            "deleted" => Ok(Self::Deleted),
            other => Err(ParseUserStatusError(other.to_owned())),
        }
    }
}

impl TryFrom<&str> for UserStatusEnum {
    type Error = ParseUserStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<u8> for UserStatusEnum {
    type Error = ParseUserStatusError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Active),
            1 => Ok(Self::Blocked),
            2 => Ok(Self::Deleted),
            other => Err(ParseUserStatusError(other.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_status_strings() {
        assert_eq!(UserStatusEnum::Deleted.as_str(), "deleted");
        assert_eq!(
            "blocked".parse::<UserStatusEnum>().unwrap(),
            UserStatusEnum::Blocked
        );
        assert!("pending".parse::<UserStatusEnum>().is_err());
    }

    #[test]
    fn roundtrip_status_u8() {
        assert_eq!(UserStatusEnum::Active.as_u8(), 0);
        assert_eq!(UserStatusEnum::Blocked.as_u8(), 1);
        assert_eq!(UserStatusEnum::Deleted.as_u8(), 2);
        assert_eq!(UserStatusEnum::try_from(0u8).unwrap(), UserStatusEnum::Active);
        assert_eq!(UserStatusEnum::try_from(1u8).unwrap(), UserStatusEnum::Blocked);
        assert_eq!(UserStatusEnum::try_from(2u8).unwrap(), UserStatusEnum::Deleted);
        assert!(UserStatusEnum::try_from(3u8).is_err());
    }
}
