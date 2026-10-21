//! LOB engine HTTP server library (Axum adapters).

use axum::{Router, routing::get};

// Workspace wiring for later milestones (REST adapters, stubs).
const _: fn() = || {
    let _ = std::any::type_name::<bot_protocol::BotProtocolError>();
    let _ = std::any::type_name::<fix_codec::FixCodecError>();
    let _ = std::any::type_name::<importers::ImporterError>();
    let _ = std::any::type_name::<sim_engine::SimEngineError>();
    let _ = std::any::type_name::<storage::StorageError>();
};

/// Build the HTTP router for the LOB server.
pub fn app() -> Router {
    Router::new().route("/healthz", get(healthz))
}

async fn healthz() -> &'static str {
    "ok"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn healthz_returns_ok() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/healthz")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status(), StatusCode::OK);
    }
}
