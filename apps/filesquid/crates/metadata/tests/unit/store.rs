//! Persist metadata.md through filesquid-ocfl helpers.

use filesquid_metadata::record::Record;
use filesquid_metadata::store::{load, save, METADATA_MD};
use filesquid_metadata::MetadataNamespace;

#[test]
fn save_and_load_object_dir() {
    let dir = std::env::temp_dir().join("filesquid-metadata-store-test");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let mut record = Record::default();
    record
        .set(MetadataNamespace::Dcterms, "creator", "George")
        .unwrap();
    save(&dir, &record).unwrap();
    assert!(dir.join(METADATA_MD).is_file());
    let loaded = load(&dir).unwrap();
    assert_eq!(loaded.get("dcterms:creator").unwrap().value, "George");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn csv_line_break_survives_save_and_load() {
    let dir = std::env::temp_dir().join(format!(
        "filesquid-metadata-store-newline-{}",
        std::process::id()
    ));
    if let Err(err) = std::fs::remove_dir_all(&dir) {
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound, "{err}");
    }
    std::fs::create_dir_all(&dir).unwrap();
    let mut record = Record::default();
    filesquid_metadata::csv::apply_csv(
        "dcterms:description\n\"line one\nline two\"\n",
        &mut record,
    )
    .unwrap();
    save(&dir, &record).unwrap();
    let loaded = load(&dir).unwrap();
    assert_eq!(
        loaded.get("dcterms:description").unwrap().value,
        "line one\nline two"
    );
    std::fs::remove_dir_all(&dir).unwrap();
}
