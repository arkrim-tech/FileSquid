//! Collection, folder, membership, and virtual-collection types.
//!
//! UI labels (album, gallery) stay out of this module. A collection belongs to
//! exactly one library. A file's bytes live in exactly one [`Collection`]. Extra
//! memberships use [`VirtualCollection`].

use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};

use crate::{
    CollectionId, DomainError, File, FileId, FolderId, LibraryId, RepositoryId,
    VirtualCollectionId, VirtualExhibitId,
};

/// Sort order stored with a collection.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionSort {
    /// Owner-defined membership order.
    #[default]
    Manual,
    /// Title A→Z.
    TitleAscending,
    /// Title Z→A.
    TitleDescending,
    /// Capture/addition date oldest first.
    DateAscending,
    /// Capture/addition date newest first.
    DateDescending,
}

/// Physical grouping that stores file bytes.
///
/// A collection belongs to exactly one library.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Collection {
    id: CollectionId,
    library_id: LibraryId,
    folder_id: Option<FolderId>,
    title: String,
    sort: CollectionSort,
    cover_file_id: Option<FileId>,
    order_key: i64,
}

impl Collection {
    /// Construct a collection with manual sort and no cover.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::EmptyName`] when `title` is empty.
    pub fn new(
        id: CollectionId,
        library_id: LibraryId,
        title: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let title = title.into();
        if title.is_empty() {
            return Err(DomainError::EmptyName);
        }
        Ok(Self {
            id,
            library_id,
            folder_id: None,
            title,
            sort: CollectionSort::Manual,
            cover_file_id: None,
            order_key: 0,
        })
    }

    /// Set the collection sort.
    pub fn with_sort(mut self, sort: CollectionSort) -> Self {
        self.sort = sort;
        self
    }

    /// Set the cover file.
    pub fn with_cover(mut self, cover_file_id: FileId) -> Self {
        self.cover_file_id = Some(cover_file_id);
        self
    }

    /// Place the collection in a folder.
    pub fn with_folder(mut self, folder_id: FolderId) -> Self {
        self.folder_id = Some(folder_id);
        self
    }

    /// Set the sidebar/manual order key.
    pub fn with_order_key(mut self, order_key: i64) -> Self {
        self.order_key = order_key;
        self
    }

    /// Return the collection identifier.
    pub fn id(&self) -> &CollectionId {
        &self.id
    }

    /// Return the owning library identifier.
    pub fn library_id(&self) -> &LibraryId {
        &self.library_id
    }

    /// Return the parent folder identifier, if any.
    pub fn folder_id(&self) -> Option<&FolderId> {
        self.folder_id.as_ref()
    }

    /// Return the collection title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Return the sort.
    pub fn sort(&self) -> CollectionSort {
        self.sort
    }

    /// Return the cover file identifier, if any.
    pub fn cover_file_id(&self) -> Option<&FileId> {
        self.cover_file_id.as_ref()
    }

    /// Return the order key.
    pub fn order_key(&self) -> i64 {
        self.order_key
    }
}

/// Return file records unchanged after a collection is removed.
///
/// Collection deletion MUST retain files. Library deletion of a file is a
/// distinct action.
pub fn retain_files_after_collection_removal(files: &[File]) -> &[File] {
    files
}

/// Nested folder in a library's collection hierarchy.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Folder {
    id: FolderId,
    library_id: LibraryId,
    parent_id: Option<FolderId>,
    name: String,
    order_key: i64,
}

impl Folder {
    /// Construct a root folder.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::EmptyName`] when `name` is empty.
    pub fn new(
        id: FolderId,
        library_id: LibraryId,
        name: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let name = name.into();
        if name.is_empty() {
            return Err(DomainError::EmptyName);
        }
        Ok(Self {
            id,
            library_id,
            parent_id: None,
            name,
            order_key: 0,
        })
    }

    /// Nest this folder under `parent_id`.
    pub fn nest_under(mut self, parent_id: FolderId) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// Return the folder identifier.
    pub fn id(&self) -> &FolderId {
        &self.id
    }

    /// Return the owning library identifier.
    pub fn library_id(&self) -> &LibraryId {
        &self.library_id
    }

    /// Return the parent folder identifier, if any.
    pub fn parent_id(&self) -> Option<&FolderId> {
        self.parent_id.as_ref()
    }

    /// Return the folder name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the order key.
    pub fn order_key(&self) -> i64 {
        self.order_key
    }
}

/// Ordered membership of a file in a physical collection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionMembership {
    collection_id: CollectionId,
    file_id: FileId,
    order_key: i64,
}

impl CollectionMembership {
    /// Construct membership of `file` in its single home collection.
    pub fn for_file(file: &File, order_key: i64) -> Self {
        Self {
            collection_id: file.home_collection_id().clone(),
            file_id: file.id().clone(),
            order_key,
        }
    }

    /// Construct physical membership only when `collection_id` is the file's home collection.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::SecondHomeCollection`] when `collection_id` is not
    /// the file's home collection. Extra memberships MUST use a virtual collection.
    pub fn try_for_collection(
        file: &File,
        collection_id: CollectionId,
        order_key: i64,
    ) -> Result<Self, DomainError> {
        if file.home_collection_id().as_str() != collection_id.as_str() {
            return Err(DomainError::SecondHomeCollection);
        }
        Ok(Self::for_file(file, order_key))
    }

    /// Return the collection identifier.
    pub fn collection_id(&self) -> &CollectionId {
        &self.collection_id
    }

    /// Return the file identifier.
    pub fn file_id(&self) -> &FileId {
        &self.file_id
    }

    /// Return the order key.
    pub fn order_key(&self) -> i64 {
        self.order_key
    }
}

/// Non-storage membership grouping (ImageSquid UI: gallery / extra album).
///
/// A virtual collection MUST NOT store file bytes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct VirtualCollection {
    id: VirtualCollectionId,
    library_id: LibraryId,
    title: String,
    expression: Option<String>,
    cover_file_id: Option<FileId>,
    order_key: i64,
}

impl VirtualCollection {
    /// Construct a virtual collection that never stores bytes.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::EmptyName`] when `title` is empty.
    pub fn new(
        id: VirtualCollectionId,
        library_id: LibraryId,
        title: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let title = title.into();
        if title.is_empty() {
            return Err(DomainError::EmptyName);
        }
        Ok(Self {
            id,
            library_id,
            title,
            expression: None,
            cover_file_id: None,
            order_key: 0,
        })
    }

    /// Attach a versioned Any/All query expression.
    pub fn with_expression(mut self, expression: impl Into<String>) -> Self {
        self.expression = Some(expression.into());
        self
    }

    /// Return the virtual collection identifier.
    pub fn id(&self) -> &VirtualCollectionId {
        &self.id
    }

    /// Return the owning library identifier.
    pub fn library_id(&self) -> &LibraryId {
        &self.library_id
    }

    /// Return the title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Return the query expression, if any.
    pub fn expression(&self) -> Option<&str> {
        self.expression.as_deref()
    }

    /// Return the cover file identifier, if any.
    pub fn cover_file_id(&self) -> Option<&FileId> {
        self.cover_file_id.as_ref()
    }

    /// Return the order key.
    pub fn order_key(&self) -> i64 {
        self.order_key
    }

    /// Virtual collections never store bytes.
    pub fn stores_bytes(&self) -> bool {
        false
    }
}

/// Ordered membership of a file in a virtual collection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VirtualMembership {
    virtual_collection_id: VirtualCollectionId,
    file_id: FileId,
    order_key: i64,
}

impl VirtualMembership {
    /// Construct a virtual membership.
    pub fn new(
        virtual_collection_id: VirtualCollectionId,
        file_id: FileId,
        order_key: i64,
    ) -> Self {
        Self {
            virtual_collection_id,
            file_id,
            order_key,
        }
    }

    /// Return the virtual collection identifier.
    pub fn virtual_collection_id(&self) -> &VirtualCollectionId {
        &self.virtual_collection_id
    }

    /// Return the file identifier.
    pub fn file_id(&self) -> &FileId {
        &self.file_id
    }

    /// Return the order key.
    pub fn order_key(&self) -> i64 {
        self.order_key
    }
}

/// Public curated query (ImageSquid UI: exhibit).
///
/// A virtual exhibit belongs to a user on a repository. It MAY span collections
/// in different libraries and on other repositories. Every member MUST be
/// public. A virtual exhibit MUST NOT store file bytes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct VirtualExhibit {
    id: VirtualExhibitId,
    repository_id: RepositoryId,
    title: String,
    cover_file_id: Option<FileId>,
    order_key: i64,
}

impl VirtualExhibit {
    /// Construct a public virtual exhibit that never stores bytes.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::EmptyName`] when `title` is empty.
    pub fn new(
        id: VirtualExhibitId,
        repository_id: RepositoryId,
        title: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let title = title.into();
        if title.is_empty() {
            return Err(DomainError::EmptyName);
        }
        Ok(Self {
            id,
            repository_id,
            title,
            cover_file_id: None,
            order_key: 0,
        })
    }

    /// Return the virtual exhibit identifier.
    pub fn id(&self) -> &VirtualExhibitId {
        &self.id
    }

    /// Return the hosting repository identifier.
    pub fn repository_id(&self) -> &RepositoryId {
        &self.repository_id
    }

    /// Return the title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Return the cover file identifier, if any.
    pub fn cover_file_id(&self) -> Option<&FileId> {
        self.cover_file_id.as_ref()
    }

    /// Return the order key.
    pub fn order_key(&self) -> i64 {
        self.order_key
    }

    /// Virtual exhibits never store bytes.
    pub fn stores_bytes(&self) -> bool {
        false
    }

    /// Every exhibit member is public.
    pub fn members_are_public(&self) -> bool {
        true
    }
}

/// Ordered membership of a public file in a virtual exhibit.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VirtualExhibitMembership {
    virtual_exhibit_id: VirtualExhibitId,
    file_id: FileId,
    order_key: i64,
}

impl VirtualExhibitMembership {
    /// Construct an exhibit membership. The file MUST already be public.
    pub fn new(virtual_exhibit_id: VirtualExhibitId, file_id: FileId, order_key: i64) -> Self {
        Self {
            virtual_exhibit_id,
            file_id,
            order_key,
        }
    }

    /// Return the virtual exhibit identifier.
    pub fn virtual_exhibit_id(&self) -> &VirtualExhibitId {
        &self.virtual_exhibit_id
    }

    /// Return the file identifier.
    pub fn file_id(&self) -> &FileId {
        &self.file_id
    }

    /// Return the order key.
    pub fn order_key(&self) -> i64 {
        self.order_key
    }
}

#[derive(Deserialize)]
struct CollectionWire {
    id: CollectionId,
    library_id: LibraryId,
    #[serde(default)]
    folder_id: Option<FolderId>,
    title: String,
    #[serde(default)]
    sort: CollectionSort,
    #[serde(default)]
    cover_file_id: Option<FileId>,
    #[serde(default)]
    order_key: i64,
}

impl TryFrom<CollectionWire> for Collection {
    type Error = DomainError;

    fn try_from(wire: CollectionWire) -> Result<Self, Self::Error> {
        let mut collection = Self::new(wire.id, wire.library_id, wire.title)?;
        collection.folder_id = wire.folder_id;
        collection.sort = wire.sort;
        collection.cover_file_id = wire.cover_file_id;
        collection.order_key = wire.order_key;
        Ok(collection)
    }
}

impl<'de> Deserialize<'de> for Collection {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        CollectionWire::deserialize(deserializer)?
            .try_into()
            .map_err(de::Error::custom)
    }
}

#[derive(Deserialize)]
struct FolderWire {
    id: FolderId,
    library_id: LibraryId,
    #[serde(default)]
    parent_id: Option<FolderId>,
    name: String,
    #[serde(default)]
    order_key: i64,
}

impl TryFrom<FolderWire> for Folder {
    type Error = DomainError;

    fn try_from(wire: FolderWire) -> Result<Self, Self::Error> {
        let mut folder = Self::new(wire.id, wire.library_id, wire.name)?;
        folder.parent_id = wire.parent_id;
        folder.order_key = wire.order_key;
        Ok(folder)
    }
}

impl<'de> Deserialize<'de> for Folder {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        FolderWire::deserialize(deserializer)?
            .try_into()
            .map_err(de::Error::custom)
    }
}

#[derive(Deserialize)]
struct VirtualCollectionWire {
    id: VirtualCollectionId,
    library_id: LibraryId,
    title: String,
    #[serde(default)]
    expression: Option<String>,
    #[serde(default)]
    cover_file_id: Option<FileId>,
    #[serde(default)]
    order_key: i64,
}

impl TryFrom<VirtualCollectionWire> for VirtualCollection {
    type Error = DomainError;

    fn try_from(wire: VirtualCollectionWire) -> Result<Self, Self::Error> {
        let mut collection = Self::new(wire.id, wire.library_id, wire.title)?;
        collection.expression = wire.expression;
        collection.cover_file_id = wire.cover_file_id;
        collection.order_key = wire.order_key;
        Ok(collection)
    }
}

impl<'de> Deserialize<'de> for VirtualCollection {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        VirtualCollectionWire::deserialize(deserializer)?
            .try_into()
            .map_err(de::Error::custom)
    }
}

#[derive(Deserialize)]
struct VirtualExhibitWire {
    id: VirtualExhibitId,
    repository_id: RepositoryId,
    title: String,
    #[serde(default)]
    cover_file_id: Option<FileId>,
    #[serde(default)]
    order_key: i64,
}

impl TryFrom<VirtualExhibitWire> for VirtualExhibit {
    type Error = DomainError;

    fn try_from(wire: VirtualExhibitWire) -> Result<Self, Self::Error> {
        let mut exhibit = Self::new(wire.id, wire.repository_id, wire.title)?;
        exhibit.cover_file_id = wire.cover_file_id;
        exhibit.order_key = wire.order_key;
        Ok(exhibit)
    }
}

impl<'de> Deserialize<'de> for VirtualExhibit {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        VirtualExhibitWire::deserialize(deserializer)?
            .try_into()
            .map_err(de::Error::custom)
    }
}
