//! Opaque string identifiers for portable FileSquid entities.
//!
//! Identifiers are stable opaque strings. They MUST NOT require a capture date
//! or filesystem path as their format ([architecture 4.1](../../../resources/architecture.md#41-data-architecture)).

use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};

use crate::InvalidOpaqueId;

/// Borrow an opaque identifier as text.
pub trait OpaqueId {
    /// Return the identifier string.
    fn as_str(&self) -> &str;
}

macro_rules! opaque_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
        pub struct $name(String);

        impl $name {
            /// Construct an identifier from a non-empty opaque string.
            ///
            /// # Errors
            ///
            /// Returns [`InvalidOpaqueId::Empty`] when `value` is empty.
            pub fn new(value: impl Into<String>) -> Result<Self, InvalidOpaqueId> {
                let value = value.into();
                if value.is_empty() {
                    return Err(InvalidOpaqueId::Empty);
                }
                Ok(Self(value))
            }

            /// Borrow the opaque identifier as text.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl OpaqueId for $name {
            fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value = String::deserialize(deserializer)?;
                Self::new(value).map_err(de::Error::custom)
            }
        }
    };
}

opaque_id!(
    /// Opaque identifier for a FileSquid **repository** (one installation).
    ///
    /// Distinct from the Git source repository.
    RepositoryId
);

opaque_id!(
    /// Opaque identifier for a product **library**.
    LibraryId
);

opaque_id!(
    /// Opaque identifier for a physical **collection** (ImageSquid UI: album).
    CollectionId
);

opaque_id!(
    /// Opaque identifier for a **virtual collection** (ImageSquid UI: gallery).
    VirtualCollectionId
);

opaque_id!(
    /// Opaque identifier for a **virtual exhibit** (ImageSquid UI: exhibit).
    VirtualExhibitId
);

opaque_id!(
    /// Opaque identifier for a stored **file** (the item type).
    FileId
);

opaque_id!(
    /// Opaque identifier for a **file version** (recipe-backed, non-destructive).
    FileVersionId
);

opaque_id!(
    /// Opaque identifier for a **file resource** (original, sidecar, or derivative bytes).
    FileResourceId
);

opaque_id!(
    /// Opaque identifier for a **folder** in a collection hierarchy.
    FolderId
);

opaque_id!(
    /// Opaque identifier for a command, query, or job **operation** (idempotency key).
    OperationId
);

opaque_id!(
    /// Opaque identifier for a wrapped content, evidence, wrap, or device **key**.
    KeyId
);
