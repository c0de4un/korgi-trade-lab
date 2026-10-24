//! Database connection settings loaded from environment.

use std::env;
use std::time::Duration;

use anyhow::{Context, bail};

/// PostgreSQL pool configuration for lob-server.
#[derive(Debug, Clone)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
    pub ssl_mode: String,
    pub timeout: Duration,
    pub max_idle_conns: u32,
    pub max_open_conns: u32,
    pub conn_max_lifetime: Duration,
}

impl DbConfig {
    /// Load config from `DB_*` environment variables.
    pub fn from_env() -> anyhow::Result<Self> {
        let host = required("DB_HOST")?;
        let port = parse_u16("DB_PORT")?;
        let user = required("DB_USER")?;
        let password = required("DB_PASSWORD")?;
        let name = required("DB_NAME")?;
        let ssl_mode = required("DB_SSL_MODE")?;
        let timeout_secs = parse_u64("DB_TIMEOUT")?;
        let max_idle_conns = parse_u32("DB_MAX_IDLE_CONNS")?;
        let max_open_conns = parse_u32("DB_MAX_OPEN_CONNS")?;
        let lifetime_secs = parse_u64("DB_CONN_MAX_LIFETIME")?;

        if max_open_conns == 0 {
            bail!("DB_MAX_OPEN_CONNS must be >= 1");
        }
        if max_idle_conns > max_open_conns {
            bail!(
                "DB_MAX_IDLE_CONNS ({max_idle_conns}) must be <= DB_MAX_OPEN_CONNS ({max_open_conns})"
            );
        }

        Ok(Self {
            host,
            port,
            user,
            password,
            name,
            ssl_mode,
            timeout: Duration::from_secs(timeout_secs),
            max_idle_conns,
            max_open_conns,
            conn_max_lifetime: Duration::from_secs(lifetime_secs),
        })
    }

    /// Build a PostgreSQL connection URL for sqlx.
    pub fn database_url(&self) -> String {
        let user = urlencoding_encode(&self.user);
        let password = urlencoding_encode(&self.password);
        format!(
            "postgres://{user}:{password}@{host}:{port}/{name}?sslmode={ssl_mode}",
            host = self.host,
            port = self.port,
            name = self.name,
            ssl_mode = self.ssl_mode,
        )
    }
}

fn required(key: &str) -> anyhow::Result<String> {
    env::var(key).with_context(|| format!("missing required env var {key}"))
}

fn parse_u16(key: &str) -> anyhow::Result<u16> {
    required(key)?
        .parse()
        .with_context(|| format!("invalid u16 in env var {key}"))
}

fn parse_u32(key: &str) -> anyhow::Result<u32> {
    required(key)?
        .parse()
        .with_context(|| format!("invalid u32 in env var {key}"))
}

fn parse_u64(key: &str) -> anyhow::Result<u64> {
    required(key)?
        .parse()
        .with_context(|| format!("invalid u64 in env var {key}"))
}

/// Minimal URL-encode for user/password (percent-encode reserved chars).
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
        assert_eq!(urlencoding_encode("korgi_secret"), "korgi_secret");
    }
}
