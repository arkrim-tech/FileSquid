//! Unit tests for file, resource, version, recipe, and derivative types.

use filesquid_core::{
    CollectionId, Derivative, DomainError, File, FileId, FileResource, FileResourceId, FileVersion,
    FileVersionId, IntegrityState, LibraryId, RecipeOperation, ResourceRole, Sha256Digest,
    VirtualCollectionId, VirtualMembership,
};

fn digest() -> Sha256Digest {
    Sha256Digest::parse("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")
        .expect("digest")
}

fn file() -> File {
    File::new(
        FileId::new("file-1").expect("file"),
        LibraryId::new("lib-1").expect("lib"),
        CollectionId::new("col-home").expect("col"),
    )
}

#[test]
fn file_rejects_a_second_home_collection() {
    let file = file();
    let err = file
        .assign_home_collection(CollectionId::new("col-other").expect("col"))
        .expect_err("second home");
    assert_eq!(err, DomainError::SecondHomeCollection);
    assert_eq!(file.home_collection_id().as_str(), "col-home");
}

#[test]
fn extra_membership_is_virtual() {
    let file = file();
    let membership = VirtualMembership::new(
        VirtualCollectionId::new("vc-extra").expect("vc"),
        file.id().clone(),
        0,
    );
    assert_eq!(membership.virtual_collection_id().as_str(), "vc-extra");
    assert_eq!(file.home_collection_id().as_str(), "col-home");
}

#[test]
fn original_bytes_cannot_be_replaced() {
    let original = FileResource::original(
        FileResourceId::new("res-1").expect("res"),
        FileId::new("file-1").expect("file"),
        1024,
        digest(),
    );
    assert_eq!(original.role(), ResourceRole::Original);
    let err = original
        .with_replaced_bytes(2048, digest())
        .expect_err("immutable");
    assert_eq!(err, DomainError::OriginalImmutable);
    assert_eq!(original.digest(), &digest());
}

#[test]
fn file_version_references_recipe_without_mutating_original() {
    let original = FileResource::original(
        FileResourceId::new("res-1").expect("res"),
        FileId::new("file-1").expect("file"),
        1024,
        digest(),
    );
    let op = RecipeOperation::new("exposure", "engine-1").with_parameter("ev", "0.5");
    let version = FileVersion::new(
        FileVersionId::new("ver-1").expect("ver"),
        FileId::new("file-1").expect("file"),
        original.id().clone(),
        vec![op],
    );
    assert_eq!(version.operations()[0].name(), "exposure");
    assert_eq!(
        version.operations()[0]
            .parameters()
            .get("ev")
            .map(String::as_str),
        Some("0.5")
    );
    assert!(version.operations()[0].enabled());
    assert_eq!(version.operations()[0].engine_version(), "engine-1");
    assert_eq!(original.digest(), &digest());
    let derivative = Derivative::new(
        version.id().clone(),
        FileResourceId::new("res-der").expect("res"),
        true,
    );
    assert!(derivative.approved_appearance());
}

#[test]
fn sha256_digest_rejects_wrong_length() {
    assert_eq!(
        Sha256Digest::parse("deadbeef").unwrap_err(),
        DomainError::InvalidDigest
    );
}

#[test]
fn invalid_digest_cannot_deserialize() {
    let parsed: Result<Sha256Digest, _> = serde_json::from_str("\"deadbeef\"");
    assert!(parsed.is_err());
}

#[test]
fn replacing_sidecar_bytes_marks_integrity_unverified() {
    let sidecar = FileResource::sidecar(
        FileResourceId::new("res-side").expect("res"),
        FileId::new("file-1").expect("file"),
        512,
        digest(),
    );
    assert_eq!(sidecar.integrity(), IntegrityState::Verified);
    let next = sidecar.with_replaced_bytes(256, digest()).expect("sidecar");
    assert_eq!(next.integrity(), IntegrityState::Unverified);
    assert_eq!(next.byte_length(), 256);
}
