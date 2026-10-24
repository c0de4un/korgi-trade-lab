//! Redis driver for caching auth tokens by opaque Bearer value.

use ::redis::aio::ConnectionManager;
use ::redis::AsyncCommands;

use crate::auth::models::AuthTokenModel;

/// Default TTL when `valid_due` is `None` (24 hours).
pub const DEFAULT_TOKEN_CACHE_TTL_SECS: u64 = 24 * 60 * 60;

const KEY_PREFIX: &str = "auth:token:";

/// Low-level Redis store for [`AuthTokenModel`] (not a domain port).
#[derive(Clone)]
pub struct AuthTokenRedisRepository {
    conn: ConnectionManager,
}

impl AuthTokenRedisRepository {
    pub fn new(conn: ConnectionManager) -> Self {
        Self { conn }
    }

    fn key(value: &str) -> String {
        format!("{KEY_PREFIX}{value}")
    }

    /// Compute cache TTL in seconds from token expiry and current time (ns).
    pub fn ttl_secs(valid_due: Option<i64>, now_ns: i64) -> u64 {
        match valid_due {
            Some(due) if due > now_ns => {
                let secs = (due - now_ns) / 1_000_000_000;
                u64::try_from(secs).unwrap_or(1).max(1)
            }
            Some(_) => 1,
            None => DEFAULT_TOKEN_CACHE_TTL_SECS,
        }
    }

    /// Load a token from cache; `Ok(None)` on miss.
    pub async fn get(&self, value: &str) -> Result<Option<AuthTokenModel>, String> {
        let key = Self::key(value);
        let mut conn = self.conn.clone();
        let payload: Option<String> = conn
            .get(&key)
            .await
            .map_err(|e| format!("redis GET {key}: {e}"))?;

        match payload {
            None => Ok(None),
            Some(json) => serde_json::from_str(&json)
                .map(Some)
                .map_err(|e| format!("redis JSON decode {key}: {e}")),
        }
    }

    /// Store a token with TTL derived from `valid_due` / `now_ns`.
    pub async fn set(
        &self,
        token: &AuthTokenModel,
        now_ns: i64,
    ) -> Result<(), String> {
        let key = Self::key(&token.value);
        let json =
            serde_json::to_string(token).map_err(|e| format!("redis JSON encode {key}: {e}"))?;
        let ttl = Self::ttl_secs(token.valid_due, now_ns);
        let mut conn = self.conn.clone();
        conn.set_ex::<_, _, ()>(&key, json, ttl)
            .await
            .map_err(|e| format!("redis SETEX {key}: {e}"))?;
        Ok(())
    }

    /// Remove a cached token by opaque value.
    pub async fn delete(&self, value: &str) -> Result<(), String> {
        let key = Self::key(value);
        let mut conn = self.conn.clone();
        let _: () = conn
            .del(&key)
            .await
            .map_err(|e| format!("redis DEL {key}: {e}"))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ttl_uses_valid_due_when_future() {
        let now = 1_000_000_000_000;
        let due = now + 5_000_000_000; // 5 seconds
        assert_eq!(AuthTokenRedisRepository::ttl_secs(Some(due), now), 5);
    }

    #[test]
    fn ttl_defaults_when_no_expiry() {
        assert_eq!(
            AuthTokenRedisRepository::ttl_secs(None, 0),
            DEFAULT_TOKEN_CACHE_TTL_SECS
        );
    }

    #[test]
    fn ttl_min_one_when_already_expired() {
        let now = 10_000_000_000;
        assert_eq!(AuthTokenRedisRepository::ttl_secs(Some(1), now), 1);
    }
}
