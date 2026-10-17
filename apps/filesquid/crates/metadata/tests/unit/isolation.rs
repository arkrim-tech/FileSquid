//! Isolation: filesquid-metadata may depend on filesquid-ocfl only among I/O crates.

use std::fs;
use std::path::PathBuf;

fn cargo_toml() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    fs::read_to_string(path).expect("Cargo.toml")
}

fn library_dependency_package_names(manifest: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut in_deps = false;
    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_deps = trimmed == "[dependencies]";
            continue;
        }
        if !in_deps || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let key = trimmed.split([' ', '=']).next().unwrap_or("");
        if !key.is_empty() {
            names.push(key.to_string());
        }
    }
    names.sort();
    names.dedup();
    names
}

const FORBIDDEN: &[&str] = &[
    "axum",
    "tauri",
    "libvips",
    "ffmpeg-sys",
    "ffmpeg",
    "filesquid-core",
    "filesquid_core",
    "imagesquid_core",
    "rusqlite",
    "sqlx",
];

#[test]
fn depends_on_filesquid_ocfl() {
    let names = library_dependency_package_names(&cargo_toml());
    assert!(
        names.iter().any(|n| n == "filesquid-ocfl"),
        "filesquid-metadata MUST depend on filesquid-ocfl"
    );
}

#[test]
fn excludes_hosts_codecs_and_filesquid_core() {
    let names = library_dependency_package_names(&cargo_toml());
    for forbidden in FORBIDDEN {
        assert!(
            !names.iter().any(|name| name == *forbidden),
            "{forbidden} must be absent from [dependencies]"
        );
    }
}
