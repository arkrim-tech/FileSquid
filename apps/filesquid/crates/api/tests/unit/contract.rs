//! Contract tests: the checked-in OpenAPI document and the router MUST agree (T-048).

use axum::body::Body;
use axum::http::{Request, StatusCode};
use filesquid_api::{router, ApiState, API_VERSION, CONTRACT};
use tower::ServiceExt;

/// Parse the embedded OpenAPI contract.
fn contract() -> serde_json::Value {
    serde_json::from_str(CONTRACT).expect("openapi.json is valid JSON")
}

#[test]
fn contract_is_openapi_3_1_with_the_crate_api_version() {
    let doc = contract();
    assert_eq!(doc["openapi"], "3.1.0");
    assert_eq!(doc["info"]["version"], API_VERSION);
}

#[tokio::test]
async fn every_documented_get_path_is_routed() {
    let doc = contract();
    let paths = doc["paths"].as_object().expect("paths");
    assert!(!paths.is_empty(), "contract documents at least one path");
    for (path, item) in paths {
        if item.get("get").is_none() || path.ends_with("/events") {
            continue;
        }
        let response = router(ApiState::new())
            .oneshot(
                Request::get(path.as_str())
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("router answers");
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "{path} is documented but not routed"
        );
    }
}

/// JSON Schema type name of a serialized value.
fn json_type(value: &serde_json::Value) -> &'static str {
    match value {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "boolean",
        serde_json::Value::Number(n) if n.is_i64() || n.is_u64() => "integer",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}

#[test]
fn health_schema_matches_the_core_payload() {
    let doc = contract();
    let schema = &doc["components"]["schemas"]["HealthStatus"];
    let props = schema["properties"]
        .as_object()
        .expect("HealthStatus schema");
    let payload = serde_json::to_value(filesquid_core::health()).expect("serialize");
    let fields = payload.as_object().expect("object");
    let mut documented: Vec<_> = props.keys().cloned().collect();
    let mut actual: Vec<_> = fields.keys().cloned().collect();
    documented.sort();
    actual.sort();
    assert_eq!(documented, actual);

    for (name, prop) in props {
        let value = &fields[name];
        assert_eq!(prop["type"], json_type(value), "{name}: documented type");
        if let Some(constant) = prop.get("const") {
            assert_eq!(constant, value, "{name}: documented const");
        }
    }

    // Every field of HealthStatus is non-optional, so every field must be required.
    let mut required: Vec<String> = schema["required"]
        .as_array()
        .expect("required list")
        .iter()
        .map(|v| v.as_str().expect("required entry").to_string())
        .collect();
    required.sort();
    assert_eq!(required, actual, "required list");
}

#[tokio::test]
async fn router_serves_its_own_contract() {
    let response = router(ApiState::new())
        .oneshot(
            Request::get("/api/v1/openapi.json")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("router answers");
    assert_eq!(response.status(), StatusCode::OK);
}
