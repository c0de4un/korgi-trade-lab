//! Redis connection settings loaded from environment.

use std::env;
use std::time::Duration;

use anyhow::{Context, bail};

/// Redis client configuration for lob-server.
#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub addr: String,
    pub password: Option<String>,
    pub db: u8,
    pub timeout: Duration,
}

impl RedisConfig {
    /// Load config from `REDIS_*` environment variables.
    pub fn from_env() -> anyhow::Result<Self> {
        let addr = required("REDIS_ADDR")?;
        let password_raw = env::var("REDIS_PASSWORD").unwrap_or_default();
        let password = if password_raw.is_empty() {
            None
        } else {
            Some(password_raw)
        };
        let db = parse_u8("REDIS_DB")?;
        let timeout_secs = parse_u64("REDIS_TIMEOUT")?;

        if addr.trim().is_empty() {
            bail!("REDIS_ADDR must not be empty");
        }

        Ok(Self {
            addr,
            password,
            db,
            timeout: Duration::from_secs(timeout_secs),
        })
    }

    /// Build a Redis connection URL for the `redis` crate.
    pub fn redis_url(&self) -> String {
        match &self.password {
            Some(password) if !password.is_empty() => {
                let encoded = urlencoding_encode(password);
                format!(
                    "redis://:{encoded}@{addr}/{db}",
                    addr = self.addr,
                    db = self.db
                )
            }
            _ => format!("redis://{addr}/{db}", addr = self.addr, db = self.db),
        }
    }
}

fn required(key: &str) -> anyhow::Result<String> {
    env::var(key).with_context(|| format!("missing required env var {key}"))
}

fn parse_u8(key: &str) -> anyhow::Result<u8> {
    required(key)?
        .parse()
        .with_context(|| format!("invalid u8 in env var {key}"))
}

fn parse_u64(key: &str) -> anyhow::Result<u64> {
    required(key)?
        .parse()
        .with_context(|| format!("invalid u64 in env var {key}"))
}

/// Minimal URL-encode for password (percent-encode reserved chars).
fn urlencoding_encode(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for b in raw.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            _ => {
                use std::fmt::Write as _;
                let _ = write!(out, "%{b:02X}");
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::urlencoding_encode;

    #[test]
    fn encodes_password_special_chars() {
        assert_eq!(urlencoding_encode("a b!"), "a%20b%21");
        assert_eq!(urlencoding_encode("secret"), "secret");
    }
}
