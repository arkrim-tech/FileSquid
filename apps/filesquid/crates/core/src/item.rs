//! Canonical item type for portable contracts.

use serde::{Deserialize, Serialize};

/// Item-level type used in data and APIs.
///
/// MUST be **file**. Photo, Image, and Video are not domain types. ImageSquid
/// UIs MAY still label a given file photo, image, or video.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    /// The only item type. A file may contain still, motion, or mixed resources.
    File,
}

impl ItemType {
    /// Return the canonical item-type token (`file`).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
        }
    }
}
