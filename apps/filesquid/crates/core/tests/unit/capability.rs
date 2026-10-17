//! Unit tests for capability register types.

use filesquid_core::{
    CapabilityInventory, CapabilityRegister, CapabilityRow, CapabilityStatus, DomainError,
};

fn inventory() -> CapabilityInventory {
    CapabilityInventory::new("linux-desktop", "jpeg", "import", "F-040", "unit").expect("inv")
}

#[test]
fn supported_row_is_nonempty_and_equivalent() {
    let row = CapabilityRow::new(
        "jpeg-import",
        CapabilityStatus::Equivalent,
        None,
        inventory(),
    )
    .expect("row");
    assert_eq!(row.platform(), "linux-desktop");
    assert_eq!(row.format(), "jpeg");
    assert_eq!(row.outcome(), "import");
    assert_eq!(row.requirement_id(), "F-040");
    assert_eq!(row.evidence(), "unit");
    let register = CapabilityRegister::new(vec![row]).expect("register");
    assert!(register.contains("jpeg-import"));
    assert_eq!(
        register.row("jpeg-import").expect("row").status(),
        CapabilityStatus::Equivalent
    );
    assert!(register.supported_set_nonempty());
}

#[test]
fn preservation_only_is_not_editable_parity() {
    assert!(!CapabilityStatus::PreservationOnly.counts_as_editable_parity());
    assert!(CapabilityStatus::Equivalent.counts_as_editable_parity());
    assert!(CapabilityStatus::AdapterEquivalent.counts_as_editable_parity());
}

#[test]
fn limited_is_distinct_from_equivalent() {
    let row = CapabilityRow::new("hdr-display", CapabilityStatus::Limited, None, inventory())
        .expect("row");
    assert_eq!(row.status(), CapabilityStatus::Limited);
    assert_eq!(row.status().as_str(), "limited");
}

#[test]
fn unavailable_child_remains_visible() {
    let parent = CapabilityRow::new(
        "jpeg-import",
        CapabilityStatus::Equivalent,
        None,
        inventory(),
    )
    .expect("parent");
    let child = CapabilityRow::new(
        "heic-edit",
        CapabilityStatus::Unavailable,
        Some("jpeg-import".to_string()),
        CapabilityInventory::new("linux-desktop", "heic", "edit", "F-040", "unit").expect("inv"),
    )
    .expect("child");
    let register = CapabilityRegister::new(vec![parent, child]).expect("register");
    assert!(register.exposes_unavailable_child("heic-edit"));
}

#[test]
fn empty_register_is_rejected() {
    assert_eq!(
        CapabilityRegister::new(vec![]).unwrap_err(),
        DomainError::EmptyCapabilitySet
    );
}

#[test]
fn only_unsupported_rows_are_rejected() {
    let row = CapabilityRow::new(
        "heic-edit",
        CapabilityStatus::Unavailable,
        None,
        inventory(),
    )
    .expect("row");
    assert_eq!(
        CapabilityRegister::new(vec![row]).unwrap_err(),
        DomainError::EmptySupportedSet
    );
}

#[test]
fn deserialize_rejects_empty_inventory_fields() {
    let parsed: Result<CapabilityInventory, _> = serde_json::from_str(
        r#"{"platform":"","format":"jpeg","outcome":"import","requirement_id":"F-040","evidence":"unit"}"#,
    );
    assert!(parsed.is_err());
}

#[test]
fn deserialize_rejects_empty_and_all_unavailable_registers() {
    let empty: Result<CapabilityRegister, _> = serde_json::from_str(r#"{"rows":[]}"#);
    assert!(empty.is_err());
    let all_unavailable = r#"{
        "rows": [{
            "id": "heic-edit",
            "platform": "linux-desktop",
            "format": "heic",
            "outcome": "edit",
            "requirement_id": "F-040",
            "evidence": "unit",
            "status": "unavailable",
            "parent_id": null
        }]
    }"#;
    let parsed: Result<CapabilityRegister, _> = serde_json::from_str(all_unavailable);
    assert!(parsed.is_err());
}
