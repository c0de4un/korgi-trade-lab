//! Database infrastructure (config, pool, migrations, domain adapters).

mod config;
mod pool;
pub mod auth;
pub mod user;

pub use config::DbConfig;
pub use pool::{create_pool, load_env_file, run_migrations};
