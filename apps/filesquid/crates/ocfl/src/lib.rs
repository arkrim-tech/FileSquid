//! Stand-alone FOSS OCFL 1.1 engine and its own tool (`filesquid-ocfl`,
//! import `filesquid_ocfl`).
//!
//! Default public API is generic OCFL. FileSquid is one caller. Digital
//! preservation programmes can install this crate without FileSquid. The crate
//! remains extractable to its own Git repository
//! ([ADR 016](../../../resources/architecture/016-rust-ocfl-library.md),
//! [ADR 023](../../../resources/architecture/023-filesquid-ocfl-engine-and-parquet.md)).
//! Scaffold only: write/read of objects is later work. This module encodes the
//! Git prohibition.

use std::path::Path;

use thiserror::Error;

/// Errors from OCFL root inspection.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum OcflError {
    /// A `.git` directory was found inside an OCFL object or storage root.
    #[error("a .git directory MUST NOT appear in an OCFL object root or storage root")]
    GitDirectoryForbidden,
}

/// Errors from writing a logical file into an object directory.
#[derive(Debug, Error)]
pub enum WriteError {
    /// Git is forbidden in the object root.
    #[error(transparent)]
    Git(#[from] OcflError),
    /// Filesystem write failed.
    #[error("write failed: {0}")]
    Io(#[from] std::io::Error),
    /// Relative path was empty or tried to escape the object root.
    #[error("logical path must be a relative file name inside the object")]
    InvalidPath,
}

/// Return an error when `root` contains a `.git` directory.
pub fn reject_git_in_ocfl_root(root: &Path) -> Result<(), OcflError> {
    if root.join(".git").exists() {
        return Err(OcflError::GitDirectoryForbidden);
    }
    Ok(())
}

/// Write `bytes` to `logical_path` under `object_root` after the Git guard.
///
/// Inventory/`vN` publication remains later OCFL work. Callers use this to
/// persist a sidecar such as `metadata.md`.
pub fn write_logical_file(
    object_root: &Path,
    logical_path: &str,
    bytes: &[u8],
) -> Result<(), WriteError> {
    reject_git_in_ocfl_root(object_root)?;
    if logical_path.is_empty()
        || logical_path.contains("..")
        || logical_path.starts_with('/')
        || Path::new(logical_path).is_absolute()
    {
        return Err(WriteError::InvalidPath);
    }
    let dest = object_root.join(logical_path);
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(dest, bytes)?;
    Ok(())
}

/// Read a logical file from `object_root` after the Git guard.
pub fn read_logical_file(object_root: &Path, logical_path: &str) -> Result<Vec<u8>, WriteError> {
    reject_git_in_ocfl_root(object_root)?;
    if logical_path.is_empty()
        || logical_path.contains("..")
        || logical_path.starts_with('/')
        || Path::new(logical_path).is_absolute()
    {
        return Err(WriteError::InvalidPath);
    }
    Ok(std::fs::read(object_root.join(logical_path))?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn missing_git_is_ok() {
        let root = PathBuf::from("/tmp/filesquid-ocfl-no-git-placeholder");
        assert_eq!(reject_git_in_ocfl_root(&root), Ok(()));
    }
}
