//! User role within the LOB admin / trading context.

use std::fmt;
use std::str::FromStr;

use super::parse_user_role_error::ParseUserRoleError;

/// Role assigned to a [`crate::user::models::UserModel`].
///
/// DB stores as `SMALLINT`: `0` trader, `1` admin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRoleEnum {
    Trader,
    Admin,
}

impl UserRoleEnum {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trader => "trader",
            Self::Admin => "admin",
        }
    }

    /// Stable DB / wire-independent discriminant (`SMALLINT` / `u8`).
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Trader => 0,
            Self::Admin => 1,
        }
    }
}

impl AsRef<str> for UserRoleEnum {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for UserRoleEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for UserRoleEnum {
    type Err = ParseUserRoleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "trader" => Ok(Self::Trader),
            "admin" => Ok(Self::Admin),
            other => Err(ParseUserRoleError(other.to_owned())),
        }
    }
}

impl TryFrom<&str> for UserRoleEnum {
    type Error = ParseUserRoleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<u8> for UserRoleEnum {
    type Error = ParseUserRoleError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Trader),
            1 => Ok(Self::Admin),
            other => Err(ParseUserRoleError(other.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_role_strings() {
        assert_eq!(UserRoleEnum::Trader.as_str(), "trader");
        assert_eq!("admin".parse::<UserRoleEnum>().unwrap(), UserRoleEnum::Admin);
        assert!("viewer".parse::<UserRoleEnum>().is_err());
    }

    #[test]
    fn roundtrip_role_u8() {
        assert_eq!(UserRoleEnum::Trader.as_u8(), 0);
        assert_eq!(UserRoleEnum::Admin.as_u8(), 1);
        assert_eq!(UserRoleEnum::try_from(0u8).unwrap(), UserRoleEnum::Trader);
        assert_eq!(UserRoleEnum::try_from(1u8).unwrap(), UserRoleEnum::Admin);
        assert!(UserRoleEnum::try_from(2u8).is_err());
    }
}
