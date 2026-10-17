//! Unit tests for portable identifiers.

use filesquid_core::{
    CollectionId, FileId, FileResourceId, FileVersionId, FolderId, InvalidOpaqueId, LibraryId,
    RepositoryId, VirtualCollectionId,
};

#[test]
fn library_id_round_trips() {
    let id = LibraryId::new("lib-opaque-1").expect("non-empty id");
    assert_eq!(id.as_str(), "lib-opaque-1");
}

#[test]
fn file_id_is_not_a_photo_type() {
    let id = FileId::new("file-opaque-1").expect("non-empty id");
    assert_eq!(id.as_str(), "file-opaque-1");
}

#[test]
fn repository_collection_virtual_and_version_ids_round_trip() {
    assert_eq!(
        RepositoryId::new("repo-opaque-1").expect("id").as_str(),
        "repo-opaque-1"
    );
    assert_eq!(
        CollectionId::new("col-opaque-1").expect("id").as_str(),
        "col-opaque-1"
    );
    assert_eq!(
        VirtualCollectionId::new("vc-opaque-1")
            .expect("id")
            .as_str(),
        "vc-opaque-1"
    );
    assert_eq!(
        FileVersionId::new("ver-opaque-1").expect("id").as_str(),
        "ver-opaque-1"
    );
    assert_eq!(
        FileResourceId::new("res-opaque-1").expect("id").as_str(),
        "res-opaque-1"
    );
    assert_eq!(
        FolderId::new("folder-opaque-1").expect("id").as_str(),
        "folder-opaque-1"
    );
}

#[test]
fn empty_identifier_is_rejected() {
    assert_eq!(LibraryId::new("").unwrap_err(), InvalidOpaqueId::Empty);
    assert_eq!(FileId::new("").unwrap_err(), InvalidOpaqueId::Empty);
    assert_eq!(RepositoryId::new("").unwrap_err(), InvalidOpaqueId::Empty);
}

#[test]
fn identifiers_accept_strings_that_look_like_paths_without_interpreting_them() {
    let id = FileId::new("/Photos/2020/IMG_0001.JPG").expect("opaque");
    assert_eq!(id.as_str(), "/Photos/2020/IMG_0001.JPG");
}

#[test]
fn empty_identifier_cannot_deserialize() {
    let parsed: Result<LibraryId, _> = serde_json::from_str("\"\"");
    assert!(parsed.is_err());
}
