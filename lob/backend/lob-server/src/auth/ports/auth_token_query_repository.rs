//! Read-side port for the `auth_tokens` table.

use crate::auth::errors::AuthTokenRepositoryError;
use crate::auth::models::AuthTokenModel;

/// CQRS query repository for auth tokens (Redis → PostgreSQL fallback).
#[allow(async_fn_in_trait)]
pub trait AuthTokenQueryRepository {
    /// Load a token by opaque Bearer value (may be expired).
    async fn find_by_value(
        &self,
        value: &str,
    ) -> Result<Option<AuthTokenModel>, AuthTokenRepositoryError>;

    /// Load a token by value and reject rows with `valid_due < now_ns`.
    ///
    /// `valid_due = None` means no expiry.
    async fn find_valid_by_value(
        &self,
        value: &str,
        now_ns: i64,
    ) -> Result<Option<AuthTokenModel>, AuthTokenRepositoryError>;
}
