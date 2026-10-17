//! Canonical metadata.md round-trip.

use filesquid_metadata::markdown::{from_markdown, to_markdown};
use filesquid_metadata::record::Record;
use filesquid_metadata::MetadataNamespace;

#[test]
fn yaml_front_matter_round_trip() {
    let mut record = Record::default();
    record
        .set(MetadataNamespace::Dcterms, "title", "Harbour")
        .unwrap();
    record
        .set(MetadataNamespace::Exif, "GPSLatitude", "49.3")
        .unwrap();
    record.body = "Evening light.".into();
    let md = to_markdown(&record);
    assert!(md.starts_with("---\n"));
    assert!(md.contains("dcterms:\n  title: Harbour"));
    let back = from_markdown(&md);
    assert_eq!(back.get("dcterms:title").unwrap().value, "Harbour");
    assert_eq!(back.get("exif:GPSLatitude").unwrap().value, "49.3");
    assert_eq!(back.body, "Evening light.");
}

/// Write one dcterms:description value to metadata.md text and read it back.
fn markdown_round_trip(value: &str) -> String {
    let mut record = Record::default();
    record
        .set(MetadataNamespace::Dcterms, "description", value)
        .unwrap();
    let back = from_markdown(&to_markdown(&record));
    back.get("dcterms:description").unwrap().value.clone()
}

#[test]
fn line_breaks_survive_metadata_md() {
    assert_eq!(
        markdown_round_trip("line one\nline two"),
        "line one\nline two"
    );
    assert_eq!(markdown_round_trip("a\r\nb"), "a\r\nb");
}

#[test]
fn quotes_and_backslashes_survive_metadata_md() {
    assert_eq!(markdown_round_trip("say \"hi\" \\n"), "say \"hi\" \\n");
}
