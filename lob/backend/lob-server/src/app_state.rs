//! Shared Axum application state (composition root / DI container).

use std::sync::Arc;

use redis::aio::ConnectionManager;
use sqlx::PgPool;

use crate::infrastructure::db::auth::{
    AuthTokenCommandRepositoryImpl, AuthTokenQueryRepositoryImpl, AuthTokenStorePg,
};
use crate::infrastructure::db::user::{UserCommandRepositoryPg, UserQueryRepositoryPg};
use crate::infrastructure::redis::auth::AuthTokenRedisRepository;

/// Runtime state shared across HTTP handlers.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub redis: ConnectionManager,
    pub user_query_repo: Arc<UserQueryRepositoryPg>,
    pub user_command_repo: Arc<UserCommandRepositoryPg>,
    pub auth_token_store: Arc<AuthTokenStorePg>,
    pub auth_token_query_repo: Arc<AuthTokenQueryRepositoryImpl>,
    pub auth_token_command_repo: Arc<AuthTokenCommandRepositoryImpl>,
}

impl AppState {
    /// Build the DI graph on the main thread after Postgres and Redis are connected.
    pub fn new(pool: PgPool, redis: ConnectionManager) -> Self {
        let user_query_repo = Arc::new(UserQueryRepositoryPg::new(pool.clone()));
        let user_command_repo = Arc::new(UserCommandRepositoryPg::new(pool.clone()));

        let auth_token_store = AuthTokenStorePg::new(pool.clone());
        let auth_token_redis = AuthTokenRedisRepository::new(redis.clone());

        let auth_token_query_repo = Arc::new(AuthTokenQueryRepositoryImpl::new(
            auth_token_store.clone(),
            auth_token_redis.clone(),
        ));
        let auth_token_command_repo = Arc::new(AuthTokenCommandRepositoryImpl::new(
            auth_token_store.clone(),
            auth_token_redis,
        ));

        Self {
            pool,
            redis,
            user_query_repo,
            user_command_repo,
            auth_token_store: Arc::new(auth_token_store),
            auth_token_query_repo,
            auth_token_command_repo,
        }
    }
}
