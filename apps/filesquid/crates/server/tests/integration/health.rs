//! Integration tests: the FileSquid API health route through the server host.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use filesquid_api::ApiState;
use filesquid_server::{app, ServerConfig};
use tower::ServiceExt;

#[tokio::test]
async fn api_health_returns_filesquid_json() {
    let response = app(&ServerConfig::default(), ApiState::new())
        .oneshot(Request::get("/api/v1/health").body(Body::empty()).unwrap())
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), 64 * 1024)
        .await
        .expect("body");
    let body = String::from_utf8(bytes.to_vec()).expect("utf8");
    assert!(body.contains("\"status\":\"ok\""));
    assert!(body.contains("\"product\":\"filesquid\""));
}

#[tokio::test]
async fn metrics_are_exposed_for_prometheus() {
    let response = app(&ServerConfig::default(), ApiState::new())
        .oneshot(Request::get("/metrics").body(Body::empty()).unwrap())
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), 64 * 1024)
        .await
        .expect("body");
    assert!(String::from_utf8_lossy(&bytes).contains("filesquid_up 1"));
}
