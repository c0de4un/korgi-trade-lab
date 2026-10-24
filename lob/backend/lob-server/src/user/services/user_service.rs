//! Application service for user identity operations.

use crate::user::enums::UserStatusEnum;
use crate::user::errors::UserServiceError;
use crate::user::models::UserModel;
use crate::user::ports::{PasswordHasher, UserQueryRepository};

/// Use-case layer for users (depends only on ports).
pub struct UserService<Q, H> {
    query_repo: Q,
    password_hasher: H,
}

impl<Q, H> UserService<Q, H>
where
    Q: UserQueryRepository,
    H: PasswordHasher,
{
    pub fn new(query_repo: Q, password_hasher: H) -> Self {
        Self {
            query_repo,
            password_hasher,
        }
    }

    /// Look up a user by email and verify the plaintext password.
    ///
    /// Returns the user model on success, or a typed error when the account is
    /// missing, inactive, or the password does not match.
    pub async fn authenticate(
        &self,
        email: &str,
        password: &str,
    ) -> Result<UserModel, UserServiceError> {
        let user = self
            .query_repo
            .find_by_email(email)
            .await?
            .ok_or(UserServiceError::NotFound)?;

        if user.status != UserStatusEnum::Active {
            return Err(UserServiceError::Inactive);
        }

        let matches = self
            .password_hasher
            .verify(password, &user.password_hash)
            .await?;

        if !matches {
            return Err(UserServiceError::InvalidPassword);
        }

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::memory::InMemoryUserQueryRepository;
    use crate::user::enums::UserRoleEnum;
    use crate::user::errors::PasswordHasherError;
    use crate::user::ports::PasswordHasher;
    use uuid::Uuid;

    /// Deterministic stub: treats `"ok"` as the only valid password for any hash.
    struct StubPasswordHasher;

    impl PasswordHasher for StubPasswordHasher {
        async fn hash(&self, password: &str) -> Result<String, PasswordHasherError> {
            Ok(format!("stub:{password}"))
        }

        async fn verify(
            &self,
            password: &str,
            _password_hash: &str,
        ) -> Result<bool, PasswordHasherError> {
            Ok(password == "ok")
        }
    }

    fn active_user(email: &str) -> UserModel {
        UserModel {
            id: Uuid::nil(),
            email: email.to_owned(),
            password_hash: "stub:ok".to_owned(),
            role: UserRoleEnum::Trader,
            status: UserStatusEnum::Active,
            created_at: 1,
            deleted_at: None,
        }
    }

    #[tokio::test]
    async fn authenticate_returns_user_when_found_and_password_matches() {
        let repo = InMemoryUserQueryRepository::new();
        let expected = active_user("trader@example.com");
        repo.insert(expected.clone());

        let service = UserService::new(repo, StubPasswordHasher);
        let found = service
            .authenticate("trader@example.com", "ok")
            .await
            .expect("user should be authenticated");

        assert_eq!(found, expected);
    }

    #[tokio::test]
    async fn authenticate_returns_not_found_when_email_missing() {
        let repo = InMemoryUserQueryRepository::new();
        let service = UserService::new(repo, StubPasswordHasher);

        let err = service
            .authenticate("missing@example.com", "ok")
            .await
            .expect_err("missing user must fail");

        assert!(matches!(err, UserServiceError::NotFound));
    }

    #[tokio::test]
    async fn authenticate_returns_invalid_password_when_verify_fails() {
        let repo = InMemoryUserQueryRepository::new();
        repo.insert(active_user("trader@example.com"));

        let service = UserService::new(repo, StubPasswordHasher);
        let err = service
            .authenticate("trader@example.com", "wrong")
            .await
            .expect_err("bad password must fail");

        assert!(matches!(err, UserServiceError::InvalidPassword));
    }

    #[tokio::test]
    async fn authenticate_returns_inactive_when_user_blocked() {
        let repo = InMemoryUserQueryRepository::new();
        let mut user = active_user("blocked@example.com");
        user.status = UserStatusEnum::Blocked;
        repo.insert(user);

        let service = UserService::new(repo, StubPasswordHasher);
        let err = service
            .authenticate("blocked@example.com", "ok")
            .await
            .expect_err("blocked user must fail");

        assert!(matches!(err, UserServiceError::Inactive));
    }

    #[tokio::test]
    async fn authenticate_returns_inactive_when_user_deleted() {
        let repo = InMemoryUserQueryRepository::new();
        let mut user = active_user("deleted@example.com");
        user.status = UserStatusEnum::Deleted;
        user.deleted_at = Some(2);
        repo.insert(user);

        let service = UserService::new(repo, StubPasswordHasher);
        let err = service
            .authenticate("deleted@example.com", "ok")
            .await
            .expect_err("deleted user must fail");

        assert!(matches!(err, UserServiceError::Inactive));
    }
}
