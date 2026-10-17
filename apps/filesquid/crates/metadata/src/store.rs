//! Persist `metadata.md` through filesquid-ocfl logical-file helpers.

use std::path::Path;

use filesquid_ocfl::{read_logical_file, write_logical_file, WriteError};

use crate::markdown::{from_markdown, to_markdown};
use crate::record::Record;

/// Canonical logical path inside an OCFL object.
pub const METADATA_MD: &str = "metadata.md";

/// Read `metadata.md` from an object directory. Missing file is an empty record.
pub fn load(object_root: &Path) -> Result<Record, WriteError> {
    match read_logical_file(object_root, METADATA_MD) {
        Ok(bytes) => {
            let text = String::from_utf8_lossy(&bytes);
            Ok(from_markdown(&text))
        }
        Err(WriteError::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => {
            Ok(Record::default())
        }
        Err(err) => Err(err),
    }
}

/// Write `metadata.md` into an object directory (Git-guarded).
pub fn save(object_root: &Path, record: &Record) -> Result<(), WriteError> {
    let bytes = to_markdown(record).into_bytes();
    write_logical_file(object_root, METADATA_MD, &bytes)
}
