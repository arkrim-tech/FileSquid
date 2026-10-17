//! Unit tests for the versioned health payload.

use filesquid_core::{health, HEALTH_SCHEMA_VERSION};

#[test]
fn health_reports_ok_with_schema_version() {
    let h = health();
    assert_eq!(h.status, "ok");
    assert_eq!(h.product, "filesquid");
    assert_eq!(h.schema_version, HEALTH_SCHEMA_VERSION);
    assert_eq!(h.schema_version, 1);
}

#[test]
fn health_json_includes_version_status_and_product() {
    let json = serde_json::to_string(&health()).expect("json");
    assert!(json.contains("\"status\":\"ok\""));
    assert!(json.contains("\"product\":\"filesquid\""));
    assert!(json.contains("\"schema_version\":1"));
}
