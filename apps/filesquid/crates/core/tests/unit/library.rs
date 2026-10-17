//! Unit tests for repository and library types.

use filesquid_core::{Library, LibraryId, Repository, RepositoryId};

#[test]
fn repository_may_host_more_than_one_library() {
    let repo = Repository::new(RepositoryId::new("repo-1").expect("repo"));
    let a = Library::new(LibraryId::new("lib-a").expect("id"), repo.id().clone());
    let b = Library::new(LibraryId::new("lib-b").expect("id"), repo.id().clone());
    assert_eq!(a.repository_id().as_str(), "repo-1");
    assert_eq!(b.repository_id().as_str(), "repo-1");
    assert_ne!(a.id().as_str(), b.id().as_str());
}
