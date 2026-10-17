//! Integration tests for the desktop transport: the FileSquid API served through the
//! `filesquid://` custom URI scheme, without a TCP listener (ADR 030, FileSquid T-049).

use filesquid_api::ApiState;
use filesquid_desktop_lib::api_protocol::{handle, SCHEME};

/// Build a GET request the way the webview delivers it to the scheme handler.
fn get(uri: &str, origin: Option<&str>) -> http::Request<Vec<u8>> {
    let mut builder = http::Request::get(uri);
    if let Some(origin) = origin {
        builder = builder.header(http::header::ORIGIN, origin);
    }
    builder.body(Vec::new()).expect("request")
}

#[test]
fn scheme_name_is_filesquid() {
    assert_eq!(SCHEME, "filesquid");
}

#[tokio::test]
async fn health_is_served_through_the_custom_scheme() {
    let response = handle(
        ApiState::new(),
        get("filesquid://localhost/api/v1/health", None),
    )
    .await;
    assert_eq!(response.status(), http::StatusCode::OK);
    let json: serde_json::Value = serde_json::from_slice(response.body()).expect("json");
    assert_eq!(json["product"], "filesquid");
}

#[tokio::test]
async fn windows_style_scheme_urls_are_served_too() {
    let response = handle(
        ApiState::new(),
        get("http://filesquid.localhost/api/v1/health", None),
    )
    .await;
    assert_eq!(response.status(), http::StatusCode::OK);
}

#[tokio::test]
async fn app_origins_receive_cors_headers() {
    let response = handle(
        ApiState::new(),
        get(
            "filesquid://localhost/api/v1/health",
            Some("tauri://localhost"),
        ),
    )
    .await;
    assert_eq!(
        response.headers()[http::header::ACCESS_CONTROL_ALLOW_ORIGIN],
        "tauri://localhost"
    );
}

#[tokio::test]
async fn foreign_origins_receive_no_cors_headers() {
    let response = handle(
        ApiState::new(),
        get(
            "filesquid://localhost/api/v1/health",
            Some("https://evil.example"),
        ),
    )
    .await;
    assert!(response
        .headers()
        .get(http::header::ACCESS_CONTROL_ALLOW_ORIGIN)
        .is_none());
}

#[tokio::test]
async fn unknown_paths_are_not_found() {
    let response = handle(
        ApiState::new(),
        get("filesquid://localhost/api/v1/nope", None),
    )
    .await;
    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn the_desktop_owner_reaches_admin_routes_without_a_token() {
    use filesquid_api::admin::AdminAuth;
    use filesquid_api::journal::MemoryJournal;
    use filesquid_api::monitor::{Monitor, MonitorConfig};
    let state = ApiState::new();
    let monitor = Monitor::start(
        std::sync::Arc::new(MemoryJournal::new("desktop")),
        MonitorConfig::new("desktop", std::env::temp_dir(), AdminAuth::Owner),
        state.events.clone(),
    );
    state.install_monitor(monitor.clone());
    let response = handle(
        state,
        get(
            "filesquid://localhost/api/v1/admin/health",
            Some("tauri://localhost"),
        ),
    )
    .await;
    assert_eq!(response.status(), http::StatusCode::OK);
    let json: serde_json::Value = serde_json::from_slice(response.body()).expect("json");
    assert!(json["status"].is_string());
    monitor.shutdown();
}
