//! In-memory adapter for [`UserQueryRepository`].
//!
//! Intended for unit tests and local demos — not a production substitute for PostgreSQL.

use std::collections::HashMap;
use std::sync::Mutex;

use uuid::Uuid;

use crate::user::errors::UserRepositoryError;
use crate::user::models::UserModel;
use crate::user::ports::UserQueryRepository;

/// `HashMap`-backed read repository for users.
#[derive(Debug, Default)]
pub struct InMemoryUserQueryRepository {
    users: Mutex<HashMap<Uuid, UserModel>>,
}

impl InMemoryUserQueryRepository {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace a user fixture (keyed by `user.id`).
    pub fn insert(&self, user: UserModel) {
        self.users
            .lock()
            .expect("in-memory user map lock")
            .insert(user.id, user);
    }
}

impl UserQueryRepository for InMemoryUserQueryRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserModel>, UserRepositoryError> {
        let guard = self.users.lock().expect("in-memory user map lock");
        Ok(guard.get(&id).cloned())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<UserModel>, UserRepositoryError> {
        let guard = self.users.lock().expect("in-memory user map lock");
        Ok(guard.values().find(|u| u.email == email).cloned())
    }

    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<UserModel>, UserRepositoryError> {
        let guard = self.users.lock().expect("in-memory user map lock");
        let mut users: Vec<UserModel> = guard
            .values()
            .filter(|u| u.deleted_at.is_none())
            .cloned()
            .collect();
        users.sort_by_key(|u| u.created_at);

        let offset = offset.max(0) as usize;
        let limit = limit.max(0) as usize;
        Ok(users.into_iter().skip(offset).take(limit).collect())
    }
}
