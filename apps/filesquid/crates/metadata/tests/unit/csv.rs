//! CSV header tests.

use filesquid_metadata::csv::{parse_csv_header, parse_csv_header_line, CsvError};
use filesquid_metadata::MetadataError;

#[test]
fn dcterms_title_header() {
    let h = parse_csv_header("dcterms:title").expect("header");
    assert_eq!(h.term, "title");
}

#[test]
fn unknown_dcterms_header_rejected() {
    let err = parse_csv_header("dcterms:name").expect_err("name");
    assert_eq!(err, CsvError::Dcterms(MetadataError::UnknownDctermsTerm));
}

#[test]
fn line_skips_id_column() {
    let headers = parse_csv_header_line("id,dcterms:title,dcterms:creator").expect("line");
    assert_eq!(headers.len(), 2);
    assert_eq!(headers[0].term, "title");
    assert_eq!(headers[1].term, "creator");
}

use filesquid_metadata::csv::{apply_csv, to_csv};
use filesquid_metadata::record::Record;
use filesquid_metadata::MetadataNamespace;

/// Export one statement, import the CSV into a fresh record, return the value.
fn round_trip(value: &str) -> String {
    let mut record = Record::default();
    record
        .set(MetadataNamespace::Dcterms, "creator", value)
        .expect("set");
    let mut imported = Record::default();
    apply_csv(&to_csv("obj-1", &record), &mut imported).expect("import");
    imported
        .get("dcterms:creator")
        .expect("creator")
        .value
        .clone()
}

#[test]
fn round_trip_keeps_commas() {
    assert_eq!(round_trip("Smith, J."), "Smith, J.");
}

#[test]
fn round_trip_keeps_quotes() {
    assert_eq!(round_trip("He said \"hi\""), "He said \"hi\"");
}

#[test]
fn round_trip_keeps_newlines() {
    assert_eq!(round_trip("line one\nline two"), "line one\nline two");
}

#[test]
fn quoted_cell_maps_to_its_own_column() {
    let text = "id,dcterms:creator,dcterms:title\nobj-1,\"Smith, J.\",Harbour\n";
    let mut record = Record::default();
    apply_csv(text, &mut record).expect("import");
    assert_eq!(record.get("dcterms:creator").unwrap().value, "Smith, J.");
    assert_eq!(record.get("dcterms:title").unwrap().value, "Harbour");
}

#[test]
fn unterminated_quote_is_rejected() {
    let text = "id,dcterms:title\nobj-1,\"Harbour\n";
    let mut record = Record::default();
    assert_eq!(
        apply_csv(text, &mut record),
        Err(CsvError::UnterminatedQuote)
    );
}

#[test]
fn quoted_whitespace_value_is_kept() {
    let mut record = Record::default();
    apply_csv("dcterms:title\n\" \"\n", &mut record).expect("import");
    assert_eq!(record.get("dcterms:title").unwrap().value, " ");
}

#[test]
fn physically_blank_lines_are_skipped() {
    let mut record = Record::default();
    apply_csv("id,dcterms:title\n\n\r\nobj-1,Harbour\n", &mut record).expect("import");
    assert_eq!(record.get("dcterms:title").unwrap().value, "Harbour");
}
