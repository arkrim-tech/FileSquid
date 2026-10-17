//! Unit tests for C2PA and PROV-O record types.

use filesquid_core::{
    C2paAction, C2paImportRecord, C2paValidationStatus, DomainError, FileResourceId, ProvOSidecar,
    Rfc3339Utc, Sha256Digest, PROV_O_CONTEXT, PROV_O_PROFILE_VERSION,
};

fn digest() -> Sha256Digest {
    Sha256Digest::parse("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")
        .expect("digest")
}

fn record() -> C2paImportRecord {
    C2paImportRecord::initialize(
        FileResourceId::new("res-1").expect("res"),
        digest(),
        Rfc3339Utc::parse("2026-09-23T12:00:00Z").expect("time"),
    )
}

#[test]
fn import_record_is_bound_to_resource_and_sha256() {
    let record = record();
    assert_eq!(record.resource_id().as_str(), "res-1");
    assert_eq!(record.digest(), &digest());
    assert_eq!(record.digest_algorithm().as_str(), "sha-256");
    assert_eq!(record.imported_at().as_str(), "2026-09-23T12:00:00Z");
    assert_eq!(record.status(), C2paValidationStatus::Unsigned);
    assert!(!record.status().is_valid_signed_manifest());
    assert!(!record.invents_capture());
}

#[test]
fn unsigned_is_not_a_valid_signed_manifest() {
    assert!(!C2paValidationStatus::Unsigned.is_valid_signed_manifest());
    assert!(C2paValidationStatus::Signed.is_valid_signed_manifest());
    assert!(!C2paValidationStatus::Invalid.is_valid_signed_manifest());
    assert!(!C2paValidationStatus::Unavailable.is_valid_signed_manifest());
}

#[test]
fn catalog_only_action_does_not_claim_pixel_change() {
    assert!(!C2paAction::catalog_only().claims_pixel_change());
}

#[test]
fn ephemeral_cache_is_not_permanent_history() {
    let time = Rfc3339Utc::parse("2026-09-23T12:00:00Z").expect("time");
    let sidecar = ProvOSidecar::ephemeral_cache(time);
    assert!(!sidecar.is_permanent_history());
}

#[test]
fn signed_manifest_cannot_be_mutated_in_place() {
    let mut record = record();
    record.set_status(C2paValidationStatus::Signed);
    assert_eq!(
        record.mutate_signed_in_place().unwrap_err(),
        DomainError::SignedManifestImmutable
    );
}

#[test]
fn provo_sidecar_is_json_ld_utc() {
    let sidecar = ProvOSidecar::retained(Rfc3339Utc::parse("2026-09-23T12:00:00Z").expect("time"));
    assert_eq!(sidecar.time().as_str(), "2026-09-23T12:00:00Z");
    assert!(sidecar.is_json_ld());
    assert_eq!(sidecar.context(), PROV_O_CONTEXT);
    assert_eq!(sidecar.profile_version(), PROV_O_PROFILE_VERSION);
    assert_eq!(sidecar.activity(), "prov:Activity");
    assert!(sidecar.is_permanent_history());
}

#[test]
fn signed_manifest_stays_immutable_after_later_status() {
    let mut record = record();
    record.set_status(C2paValidationStatus::Signed);
    record.set_status(C2paValidationStatus::Invalid);
    assert_eq!(record.status(), C2paValidationStatus::Invalid);
    assert_eq!(
        record.mutate_signed_in_place().unwrap_err(),
        DomainError::SignedManifestImmutable
    );
}

#[test]
fn invalid_timestamp_is_rejected() {
    assert_eq!(
        Rfc3339Utc::parse("not-a-time").unwrap_err(),
        DomainError::InvalidTimestamp
    );
    assert_eq!(
        Rfc3339Utc::parse("2026-99-99T99:99:99Z").unwrap_err(),
        DomainError::InvalidTimestamp
    );
    let parsed: Result<Rfc3339Utc, _> = serde_json::from_str("\"2026-99-99T99:99:99Z\"");
    assert!(parsed.is_err());
}
