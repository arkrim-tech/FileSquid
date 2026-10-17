//! File, resource, version, recipe, and derivative types.
//!
//! Originals are immutable. Versions reference recipes. This module performs
//! no codec I/O.

use std::collections::BTreeMap;

use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};

use crate::{CollectionId, DomainError, FileId, FileResourceId, FileVersionId, LibraryId};

/// SHA-256 digest stored as 64 lowercase hexadecimal characters.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Sha256Digest(String);

impl Sha256Digest {
    /// Parse a SHA-256 hex digest.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::InvalidDigest`] when `hex` is not 64 hex digits.
    pub fn parse(hex: impl AsRef<str>) -> Result<Self, DomainError> {
        let hex = hex.as_ref();
        if hex.len() != 64 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(DomainError::InvalidDigest);
        }
        Ok(Self(hex.to_ascii_lowercase()))
    }

    /// Borrow the hexadecimal digest.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Sha256Digest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(de::Error::custom)
    }
}

/// Role of a [`FileResource`] relative to its parent file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceRole {
    /// Immutable original bytes.
    Original,
    /// Rendered output of a [`FileVersion`] recipe.
    Derivative,
    /// Companion sidecar (metadata, evidence) linked to the file.
    Sidecar,
}

/// Integrity state of stored bytes. This crate does not compute hashes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrityState {
    /// Digest has been verified by a host.
    Verified,
    /// Digest is recorded but not yet verified.
    Unverified,
}

/// Lifecycle status of a file record.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileStatus {
    /// The file is present in the library.
    Active,
}

/// A logical file whose bytes live in exactly one home collection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct File {
    id: FileId,
    library_id: LibraryId,
    home_collection_id: CollectionId,
    status: FileStatus,
}

impl File {
    /// Construct a file with a single home collection for its bytes.
    pub fn new(id: FileId, library_id: LibraryId, home_collection_id: CollectionId) -> Self {
        Self {
            id,
            library_id,
            home_collection_id,
            status: FileStatus::Active,
        }
    }

    /// Return the file identifier.
    pub fn id(&self) -> &FileId {
        &self.id
    }

    /// Return the owning library identifier.
    pub fn library_id(&self) -> &LibraryId {
        &self.library_id
    }

    /// Return the single collection that stores this file's bytes.
    pub fn home_collection_id(&self) -> &CollectionId {
        &self.home_collection_id
    }

    /// Return the file status.
    pub fn status(&self) -> FileStatus {
        self.status
    }

    /// Assign a home collection.
    ///
    /// Succeeds when `collection_id` matches the existing home collection.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::SecondHomeCollection`] when the file already
    /// lives in a different collection. Extra memberships MUST use a virtual
    /// collection.
    pub fn assign_home_collection(&self, collection_id: CollectionId) -> Result<Self, DomainError> {
        if self.home_collection_id.as_str() == collection_id.as_str() {
            return Ok(self.clone());
        }
        Err(DomainError::SecondHomeCollection)
    }
}

/// Byte payload associated with a file (original, sidecar, or derivative).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileResource {
    id: FileResourceId,
    file_id: FileId,
    role: ResourceRole,
    byte_length: u64,
    digest: Sha256Digest,
    integrity: IntegrityState,
}

impl FileResource {
    /// Construct an immutable original resource.
    pub fn original(
        id: FileResourceId,
        file_id: FileId,
        byte_length: u64,
        digest: Sha256Digest,
    ) -> Self {
        Self {
            id,
            file_id,
            role: ResourceRole::Original,
            byte_length,
            digest,
            integrity: IntegrityState::Verified,
        }
    }

    /// Construct a derivative resource produced by a recipe render.
    pub fn derivative(
        id: FileResourceId,
        file_id: FileId,
        byte_length: u64,
        digest: Sha256Digest,
    ) -> Self {
        Self {
            id,
            file_id,
            role: ResourceRole::Derivative,
            byte_length,
            digest,
            integrity: IntegrityState::Unverified,
        }
    }

    /// Construct a sidecar resource.
    pub fn sidecar(
        id: FileResourceId,
        file_id: FileId,
        byte_length: u64,
        digest: Sha256Digest,
    ) -> Self {
        Self {
            id,
            file_id,
            role: ResourceRole::Sidecar,
            byte_length,
            digest,
            integrity: IntegrityState::Verified,
        }
    }

    /// Return the resource identifier.
    pub fn id(&self) -> &FileResourceId {
        &self.id
    }

    /// Return the parent file identifier.
    pub fn file_id(&self) -> &FileId {
        &self.file_id
    }

    /// Return the resource role.
    pub fn role(&self) -> ResourceRole {
        self.role
    }

    /// Return whether this resource is an original.
    pub fn is_original(&self) -> bool {
        self.role == ResourceRole::Original
    }

    /// Return the recorded byte length.
    pub fn byte_length(&self) -> u64 {
        self.byte_length
    }

    /// Return the SHA-256 digest.
    pub fn digest(&self) -> &Sha256Digest {
        &self.digest
    }

    /// Return the integrity state.
    pub fn integrity(&self) -> IntegrityState {
        self.integrity
    }

    /// Attempt to replace stored bytes.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::OriginalImmutable`] when this resource is an
    /// original. Optional conversions MUST add a linked rendition instead.
    pub fn with_replaced_bytes(
        &self,
        byte_length: u64,
        digest: Sha256Digest,
    ) -> Result<Self, DomainError> {
        if self.is_original() {
            return Err(DomainError::OriginalImmutable);
        }
        let mut next = self.clone();
        next.byte_length = byte_length;
        next.digest = digest;
        next.integrity = IntegrityState::Unverified;
        Ok(next)
    }
}

/// One ordered operation in a non-destructive recipe.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeOperation {
    name: String,
    parameters: BTreeMap<String, String>,
    enabled: bool,
    engine_version: String,
}

impl RecipeOperation {
    /// Construct an enabled operation with no parameters.
    pub fn new(name: impl Into<String>, engine_version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parameters: BTreeMap::new(),
            enabled: true,
            engine_version: engine_version.into(),
        }
    }

    /// Add a parameter and return the operation.
    pub fn with_parameter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.parameters.insert(key.into(), value.into());
        self
    }

    /// Return the operation name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the operation parameters.
    pub fn parameters(&self) -> &BTreeMap<String, String> {
        &self.parameters
    }

    /// Return whether the operation is enabled.
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Return the rendering-engine / profile version that produced this step.
    pub fn engine_version(&self) -> &str {
        &self.engine_version
    }
}

/// A non-destructive file version: selected original plus ordered recipe.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileVersion {
    id: FileVersionId,
    file_id: FileId,
    parent_id: Option<FileVersionId>,
    original_resource_id: FileResourceId,
    operations: Vec<RecipeOperation>,
}

impl FileVersion {
    /// Construct a version that references `original_resource_id` and `operations`.
    pub fn new(
        id: FileVersionId,
        file_id: FileId,
        original_resource_id: FileResourceId,
        operations: Vec<RecipeOperation>,
    ) -> Self {
        Self {
            id,
            file_id,
            parent_id: None,
            original_resource_id,
            operations,
        }
    }

    /// Set the parent version identifier.
    pub fn with_parent(mut self, parent_id: FileVersionId) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// Return the version identifier.
    pub fn id(&self) -> &FileVersionId {
        &self.id
    }

    /// Return the parent file identifier.
    pub fn file_id(&self) -> &FileId {
        &self.file_id
    }

    /// Return the parent version identifier, if any.
    pub fn parent_id(&self) -> Option<&FileVersionId> {
        self.parent_id.as_ref()
    }

    /// Return the original resource this version develops.
    pub fn original_resource_id(&self) -> &FileResourceId {
        &self.original_resource_id
    }

    /// Return the ordered recipe operations.
    pub fn operations(&self) -> &[RecipeOperation] {
        &self.operations
    }
}

/// Rendered output of a [`FileVersion`] with an approved-appearance flag.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Derivative {
    version_id: FileVersionId,
    resource_id: FileResourceId,
    approved_appearance: bool,
}

impl Derivative {
    /// Construct a derivative binding a version to an output resource.
    pub fn new(
        version_id: FileVersionId,
        resource_id: FileResourceId,
        approved_appearance: bool,
    ) -> Self {
        Self {
            version_id,
            resource_id,
            approved_appearance,
        }
    }

    /// Return the source version identifier.
    pub fn version_id(&self) -> &FileVersionId {
        &self.version_id
    }

    /// Return the output resource identifier.
    pub fn resource_id(&self) -> &FileResourceId {
        &self.resource_id
    }

    /// Return whether this render is the approved appearance.
    pub fn approved_appearance(&self) -> bool {
        self.approved_appearance
    }
}
