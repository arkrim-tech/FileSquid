//! Unit tests for collection, folder, membership, and virtual collection types.

use filesquid_core::{
    Collection, CollectionId, CollectionMembership, CollectionSort, DomainError, File, FileId,
    Folder, FolderId, LibraryId, RepositoryId, VirtualCollection, VirtualCollectionId,
    VirtualExhibit, VirtualExhibitId,
};

fn library() -> LibraryId {
    LibraryId::new("lib-1").expect("lib")
}

#[test]
fn collection_removal_retains_files() {
    let file = File::new(
        FileId::new("file-1").expect("file"),
        library(),
        CollectionId::new("col-home").expect("col"),
    );
    let files = vec![file];
    let retained = filesquid_core::retain_files_after_collection_removal(&files);
    assert_eq!(retained.len(), 1);
    assert_eq!(retained[0].id().as_str(), "file-1");
}

#[test]
fn nested_folders_record_parent() {
    let parent =
        Folder::new(FolderId::new("parent").expect("id"), library(), "Parent").expect("folder");
    let child = Folder::new(FolderId::new("child").expect("id"), library(), "Child")
        .expect("folder")
        .nest_under(parent.id().clone());
    assert_eq!(child.parent_id().map(FolderId::as_str), Some("parent"));
}

#[test]
fn ordered_membership_preserves_keys() {
    let collection = CollectionId::new("col-1").expect("col");
    let mut memberships = ["a", "b", "c"]
        .into_iter()
        .enumerate()
        .map(|(i, name)| {
            let file = File::new(
                FileId::new(name).expect("file"),
                library(),
                collection.clone(),
            );
            CollectionMembership::for_file(&file, i as i64)
        })
        .collect::<Vec<_>>();
    memberships.reverse();
    memberships.sort_by_key(CollectionMembership::order_key);
    let order: Vec<_> = memberships.iter().map(|m| m.file_id().as_str()).collect();
    let keys: Vec<_> = memberships
        .iter()
        .map(CollectionMembership::order_key)
        .collect();
    assert_eq!(order, ["a", "b", "c"]);
    assert_eq!(keys, [0, 1, 2]);
}

#[test]
fn physical_membership_rejects_a_second_home_collection() {
    let file = File::new(
        FileId::new("file-1").expect("file"),
        library(),
        CollectionId::new("col-home").expect("col"),
    );
    let err = CollectionMembership::try_for_collection(
        &file,
        CollectionId::new("col-other").expect("col"),
        0,
    )
    .unwrap_err();
    assert_eq!(err, DomainError::SecondHomeCollection);
}

#[test]
fn empty_collection_title_cannot_deserialize() {
    let json = r#"{"id":"col-1","library_id":"lib-1","title":""}"#;
    let parsed: Result<Collection, _> = serde_json::from_str(json);
    assert!(parsed.is_err());
}

#[test]
fn virtual_collection_does_not_store_bytes() {
    let vc = VirtualCollection::new(
        VirtualCollectionId::new("vc-1").expect("vc"),
        library(),
        "Extra",
    )
    .expect("vc");
    assert!(!vc.stores_bytes());
}

#[test]
fn collection_belongs_to_exactly_one_library() {
    let collection = Collection::new(
        CollectionId::new("col-1").expect("col"),
        library(),
        "Holidays",
    )
    .expect("collection");
    assert_eq!(collection.library_id().as_str(), "lib-1");
}

#[test]
fn collection_sort_and_cover_are_portable() {
    let collection = Collection::new(
        CollectionId::new("col-1").expect("col"),
        library(),
        "Holidays",
    )
    .expect("collection")
    .with_sort(CollectionSort::DateDescending)
    .with_cover(FileId::new("file-cover").expect("file"));
    assert_eq!(collection.sort(), CollectionSort::DateDescending);
    assert_eq!(
        collection.cover_file_id().map(FileId::as_str),
        Some("file-cover")
    );
}

#[test]
fn empty_collection_title_is_rejected() {
    let err = Collection::new(CollectionId::new("col-1").expect("col"), library(), "").unwrap_err();
    assert_eq!(err, filesquid_core::DomainError::EmptyName);
}

#[test]
fn virtual_exhibit_belongs_to_a_repository_and_is_public() {
    let repo = RepositoryId::new("repo-1").expect("repo");
    let exhibit = VirtualExhibit::new(
        VirtualExhibitId::new("ex-1").expect("ex"),
        repo.clone(),
        "Public feed",
    )
    .expect("exhibit");
    assert_eq!(exhibit.repository_id().as_str(), "repo-1");
    assert!(!exhibit.stores_bytes());
    assert!(exhibit.members_are_public());
}
