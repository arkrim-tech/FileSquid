//! Desktop transport for the FileSquid API.
//!
//! The webview reaches the API through the `filesquid://` custom URI scheme
//! (`http://filesquid.localhost` on Windows and Android). Requests are answered in-process
//! by [`filesquid_api::router`]; no TCP port is opened (ADR 030, FileSquid T-049).

use axum::body::Body;
use filesquid_api::ApiState;
use http::header::{ACCESS_CONTROL_ALLOW_ORIGIN, ORIGIN, VARY};
use http::{HeaderValue, Request, Response, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

/// Custom URI scheme name registered with Tauri.
pub const SCHEME: &str = "filesquid";

/// Page origins of the app's own webviews that may call the API cross-origin.
/// The scheme is only reachable from this app's webviews; the allowlist keeps
/// other origins (for example a page navigated to by mistake) from reading responses.
const APP_ORIGINS: &[&str] = &[
    "tauri://localhost",
    "http://tauri.localhost",
    "https://tauri.localhost",
    "http://127.0.0.1:5173",
    "http://127.0.0.1:5174",
];

/// Answer one webview request with the FileSquid API router.
///
/// Returns the router's response with its body collected, plus CORS headers when the
/// request comes from an allowed app origin. Never panics: internal failures become `500`.
pub async fn handle(state: ApiState, request: Request<Vec<u8>>) -> Response<Vec<u8>> {
    let origin = request.headers().get(ORIGIN).cloned();
    let mut response = route(state, request).await;
    allow_app_origin(&mut response, origin.as_ref());
    response
}

/// Rebase the scheme URL onto a path-only URI and run it through the router.
async fn route(state: ApiState, request: Request<Vec<u8>>) -> Response<Vec<u8>> {
    let (mut parts, body) = request.into_parts();
    let path = parts
        .uri
        .path_and_query()
        .map(|pq| pq.as_str().to_string())
        .unwrap_or_else(|| "/".to_string());
    parts.uri = match path.parse() {
        Ok(uri) => uri,
        Err(_) => return error_response(StatusCode::BAD_REQUEST),
    };
    let routed = filesquid_api::router(state)
        .oneshot(Request::from_parts(parts, Body::from(body)))
        .await;
    let Ok(response) = routed;
    let (parts, body) = response.into_parts();
    match body.collect().await {
        Ok(collected) => Response::from_parts(parts, collected.to_bytes().to_vec()),
        Err(err) => {
            tracing::error!(%err, "API response body failed");
            error_response(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Echo the request origin in `Access-Control-Allow-Origin` when it is an app origin.
fn allow_app_origin(response: &mut Response<Vec<u8>>, origin: Option<&HeaderValue>) {
    let allowed = origin.filter(|value| {
        value
            .to_str()
            .map(|text| APP_ORIGINS.contains(&text))
            .unwrap_or(false)
    });
    if let Some(value) = allowed {
        response
            .headers_mut()
            .insert(ACCESS_CONTROL_ALLOW_ORIGIN, value.clone());
        response
            .headers_mut()
            .insert(VARY, HeaderValue::from_static("Origin"));
    }
}

/// Build an empty response with the given status.
fn error_response(status: StatusCode) -> Response<Vec<u8>> {
    let mut response = Response::new(Vec::new());
    *response.status_mut() = status;
    response
}
