//! Register unit tests for the `/terms/` vocabulary.

use filesquid_metadata::{map_dc_to_dcterms, reject_file_dcterms_term, MetadataError};

#[test]
fn dc_title_maps_to_dcterms_title() {
    assert_eq!(map_dc_to_dcterms("title"), Some("title"));
}

#[test]
fn unknown_dc_name_does_not_map() {
    assert_eq!(map_dc_to_dcterms("name"), None);
}

#[test]
fn creator_is_accepted() {
    assert_eq!(reject_file_dcterms_term("creator"), Ok(()));
}

#[test]
fn unknown_term_is_rejected() {
    assert_eq!(
        reject_file_dcterms_term("name"),
        Err(MetadataError::UnknownDctermsTerm)
    );
}

use filesquid_metadata::{
    dcterms_to_schema, is_dcterms_property, schema_to_dcterms, DCTERMS_SCHEMA_1TO1,
};

#[test]
fn title_maps_to_schema_name_both_ways() {
    assert_eq!(dcterms_to_schema("title"), Some("name"));
    assert_eq!(schema_to_dcterms("name"), Some("title"));
}

#[test]
fn renamed_equivalents_map() {
    assert_eq!(dcterms_to_schema("subject"), Some("about"));
    assert_eq!(dcterms_to_schema("rightsHolder"), Some("copyrightHolder"));
    assert_eq!(schema_to_dcterms("datePublished"), Some("issued"));
}

#[test]
fn close_and_unmatched_terms_do_not_alias() {
    assert_eq!(dcterms_to_schema("rights"), None);
    assert_eq!(dcterms_to_schema("provenance"), None);
    assert_eq!(schema_to_dcterms("certification"), None);
}

/// The 1:1 rows from ADR 022's DCMI-to-Schema.org working map, as `(dcterms, schema)`.
const EXPECTED_1TO1: [(&str, &str); 22] = [
    ("creator", "creator"),
    ("contributor", "contributor"),
    ("publisher", "publisher"),
    ("title", "name"),
    ("alternative", "alternateName"),
    ("description", "description"),
    ("abstract", "abstract"),
    ("identifier", "identifier"),
    ("language", "inLanguage"),
    ("license", "license"),
    ("subject", "about"),
    ("audience", "audience"),
    ("educationLevel", "educationalLevel"),
    ("spatial", "spatialCoverage"),
    ("temporal", "temporalCoverage"),
    ("created", "dateCreated"),
    ("modified", "dateModified"),
    ("issued", "datePublished"),
    ("hasPart", "hasPart"),
    ("isPartOf", "isPartOf"),
    ("references", "citation"),
    ("rightsHolder", "copyrightHolder"),
];

#[test]
fn one_to_one_map_matches_expected_table() {
    assert_eq!(DCTERMS_SCHEMA_1TO1, EXPECTED_1TO1.as_slice());
    for (dcterms, schema) in EXPECTED_1TO1 {
        assert_eq!(dcterms_to_schema(dcterms), Some(schema), "{dcterms}");
        assert_eq!(schema_to_dcterms(schema), Some(dcterms), "{schema}");
    }
}

#[test]
fn one_to_one_map_has_22_rows_of_registered_terms() {
    assert_eq!(DCTERMS_SCHEMA_1TO1.len(), 22);
    for (dcterms, _) in DCTERMS_SCHEMA_1TO1 {
        assert!(is_dcterms_property(dcterms), "{dcterms} not in register");
    }
}

#[test]
fn one_to_one_map_is_injective_both_ways() {
    let mut left: Vec<&str> = DCTERMS_SCHEMA_1TO1.iter().map(|(d, _)| *d).collect();
    let mut right: Vec<&str> = DCTERMS_SCHEMA_1TO1.iter().map(|(_, s)| *s).collect();
    left.sort_unstable();
    left.dedup();
    right.sort_unstable();
    right.dedup();
    assert_eq!(left.len(), DCTERMS_SCHEMA_1TO1.len());
    assert_eq!(right.len(), DCTERMS_SCHEMA_1TO1.len());
}
