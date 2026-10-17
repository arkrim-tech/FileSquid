//! OCFL Git-guard unit tests.

use filesquid_ocfl::{reject_git_in_ocfl_root, OcflError};
use std::fs;

#[test]
fn git_directory_is_rejected() {
    let dir = std::env::temp_dir().join("filesquid-ocfl-git-guard");
    let git = dir.join(".git");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&git).expect("temp git dir");
    let err = reject_git_in_ocfl_root(&dir).expect_err("must reject");
    assert_eq!(err, OcflError::GitDirectoryForbidden);
    let _ = fs::remove_dir_all(&dir);
}
