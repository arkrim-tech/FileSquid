//! Repository and library types.
//!
//! A user or group MAY own more than one library on a repository.

use serde::{Deserialize, Serialize};

use crate::{LibraryId, OpaqueId, RepositoryId};

/// One FileSquid installation. Distinct from the Git source repository.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Repository {
    id: RepositoryId,
}

impl Repository {
    /// Construct a repository from its opaque identifier.
    pub fn new(id: RepositoryId) -> Self {
        Self { id }
    }

    /// Return the repository identifier.
    pub fn id(&self) -> &RepositoryId {
        &self.id
    }
}

/// A named set of files owned by a user or group, hosted on a repository.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Library {
    id: LibraryId,
    repository_id: RepositoryId,
}

impl Library {
    /// Construct a library belonging to `repository_id`.
    pub fn new(id: LibraryId, repository_id: RepositoryId) -> Self {
        Self { id, repository_id }
    }

    /// Return the library identifier.
    pub fn id(&self) -> &LibraryId {
        &self.id
    }

    /// Return the hosting repository identifier.
    pub fn repository_id(&self) -> &RepositoryId {
        &self.repository_id
    }
}

impl OpaqueId for Repository {
    fn as_str(&self) -> &str {
        self.id.as_str()
    }
}
