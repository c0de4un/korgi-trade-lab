//! Trader cabinet HTTP server library (Axum adapters).

use axum::{Router, routing::get};

// Workspace wiring for later milestones.
const _: fn() = || {
    let _ = std::any::type_name::<bot_protocol::BotProtocolError>();
    let _ = std::any::type_name::<market_types::MarketTypesError>();
    let _ = std::any::type_name::<storage::StorageError>();
};

/// Build the HTTP router for the cabinet server.
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
