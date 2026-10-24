//! Redis connection helpers.

use ::redis::aio::ConnectionManager;
use ::redis::Client;

use super::config::RedisConfig;

/// Open a multiplexed Redis connection manager from config.
///
/// Not wired into DI/`main` yet — call sites come in a later task.
pub async fn connect(config: &RedisConfig) -> anyhow::Result<ConnectionManager> {
    let client = Client::open(config.redis_url())
        .map_err(|e| anyhow::anyhow!("redis client open failed: {e}"))?;

    let manager = tokio::time::timeout(config.timeout, ConnectionManager::new(client))
        .await
        .map_err(|_| anyhow::anyhow!("redis connect timed out after {:?}", config.timeout))?
        .map_err(|e| anyhow::anyhow!("redis connection manager failed: {e}"))?;

    Ok(manager)
}
