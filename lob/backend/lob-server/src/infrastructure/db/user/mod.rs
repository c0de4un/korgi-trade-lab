//! PostgreSQL adapters for the user domain.

mod user_command_repository_pg;
mod user_query_repository_pg;
mod user_row;

pub use user_command_repository_pg::UserCommandRepositoryPg;
pub use user_query_repository_pg::UserQueryRepositoryPg;
