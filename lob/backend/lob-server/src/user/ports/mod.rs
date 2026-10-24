//! User repository ports (CQRS).

mod user_command_repository;
mod user_query_repository;

pub use user_command_repository::UserCommandRepository;
pub use user_query_repository::UserQueryRepository;
