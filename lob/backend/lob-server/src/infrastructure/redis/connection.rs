//! Redis connection helpers.

use ::redis::aio::ConnectionManager;
use ::redis::Client;
use tracing::info;

use super::config::RedisConfig;

/// Open a multiplexed Redis connection manager from config and verify with PING.
pub async fn connect(config: &RedisConfig) -> anyhow::Result<ConnectionManager> {
    let client = Client::open(config.redis_url())
        .map_err(|e| anyhow::anyhow!("redis client open failed: {e}"))?;

    let mut manager = tokio::time::timeout(config.timeout, ConnectionManager::new(client))
        .await
        .map_err(|_| anyhow::anyhow!("redis connect timed out after {:?}", config.timeout))?
        .map_err(|e| anyhow::anyhow!("redis connection manager failed: {e}"))?;

    let pong: String = tokio::time::timeout(
        config.timeout,
        ::redis::cmd("PING").query_async(&mut manager),
    )
    .await
    .map_err(|_| anyhow::anyhow!("redis PING timed out after {:?}", config.timeout))?
    .map_err(|e| anyhow::anyhow!("redis PING failed: {e}"))?;

    if !pong.eq_ignore_ascii_case("PONG") {
        anyhow::bail!("redis PING unexpected reply: {pong}");
    }

    info!(addr = %config.addr, db = config.db, "Redis connection ready");
    Ok(manager)
}
