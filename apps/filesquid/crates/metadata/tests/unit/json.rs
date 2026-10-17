//! JSON sidecar interchange tests (MD-S-010).
//!
//! A sidecar is one UTF-8 JSON document per OCFL object that binds
//! `{namespace, term, value}` statements to the object identifier and the
//! SHA-256 digest of the described logical file.

use filesquid_metadata::json::{
    apply_json, parse_json, sha256_hex, to_json, JsonError, JsonTarget, DIGEST_ALGORITHM,
};
use filesquid_metadata::record::Record;
use filesquid_metadata::{MetadataError, MetadataNamespace};

/// Object identifier used by every test.
const OBJECT_ID: &str = "urn:example:object-1";

/// SHA-256 of the ASCII bytes `abc` (FIPS 180-2 test vector).
const ABC_SHA256: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

/// Return the target binding for the `abc` file of [`OBJECT_ID`].
fn abc_target() -> JsonTarget<'static> {
    JsonTarget {
        object_id: OBJECT_ID,
        digest: ABC_SHA256,
    }
}

/// Build a sidecar document by hand so tests can inject bad input.
fn sidecar(object_id: &str, algorithm: &str, digest: &str, statements: &str) -> String {
    format!(
        "{{\"objectId\":\"{object_id}\",\"digestAlgorithm\":\"{algorithm}\",\
         \"digest\":\"{digest}\",\"statements\":[{statements}]}}"
    )
}

/// Build one `{namespace, term, value}` statement as JSON text.
fn statement(namespace: &str, term: &str, value: &str) -> String {
    format!("{{\"namespace\":\"{namespace}\",\"term\":\"{term}\",\"value\":\"{value}\"}}")
}

/// Import a sidecar into an empty record bound to the `abc` file.
fn import(text: &str) -> Result<Record, JsonError> {
    let mut record = Record::default();
    apply_json(text, &abc_target(), &mut record)?;
    Ok(record)
}

/// Return a record with statements in all three built-in namespaces.
fn sample_record() -> Record {
    let mut record = Record::default();
    record
        .set(
            MetadataNamespace::Dcterms,
            "title",
            "Harbour, \"dawn\"\nline two",
        )
        .expect("title");
    record
        .set(MetadataNamespace::Dcterms, "creator", "Zoë Ångström")
        .expect("creator");
    record
        .set(MetadataNamespace::Exif, "Model", "X100V")
        .expect("model");
    record
        .set(MetadataNamespace::SchemaOrg, "album", "Coast")
        .expect("album");
    record
}

#[test]
fn sha256_matches_fips_vector() {
    assert_eq!(sha256_hex(b"abc"), ABC_SHA256);
}

#[test]
fn digest_algorithm_is_sha256() {
    assert_eq!(DIGEST_ALGORITHM, "sha256");
}

#[test]
fn export_binds_object_id_and_digest() {
    let doc = parse_json(&to_json(OBJECT_ID, ABC_SHA256, &sample_record())).expect("parse");
    assert_eq!(doc.object_id, OBJECT_ID);
    assert_eq!(doc.digest_algorithm, "sha256");
    assert_eq!(doc.digest, ABC_SHA256);
}

#[test]
fn export_writes_namespace_term_value_statements() {
    let text = to_json(OBJECT_ID, ABC_SHA256, &sample_record());
    assert!(text.contains("\"namespace\": \"dcterms\""));
    assert!(text.contains("\"term\": \"title\""));
    assert!(text.contains("\"value\": \"X100V\""));
    assert!(text.contains("\"namespace\": \"exif\""));
    assert!(text.contains("\"namespace\": \"schema\""));
}

#[test]
fn export_is_utf8_without_escaping_unicode() {
    let text = to_json(OBJECT_ID, ABC_SHA256, &sample_record());
    assert!(text.contains("Zoë Ångström"));
}

#[test]
fn export_then_import_reproduces_statements() {
    let original = sample_record();
    let imported = import(&to_json(OBJECT_ID, ABC_SHA256, &original)).expect("import");
    assert_eq!(imported.statements, original.statements);
}

#[test]
fn import_accepts_uppercase_digest_hex() {
    let text = sidecar(
        OBJECT_ID,
        "sha256",
        &ABC_SHA256.to_uppercase(),
        &statement("dcterms", "title", "Harbour"),
    );
    let record = import(&text).expect("import");
    assert_eq!(record.get("dcterms:title").expect("title").value, "Harbour");
}

#[test]
fn import_rejects_digest_mismatch() {
    let other = sha256_hex(b"abd");
    let text = sidecar(
        OBJECT_ID,
        "sha256",
        &other,
        &statement("dcterms", "title", "x"),
    );
    assert_eq!(import(&text), Err(JsonError::DigestMismatch));
}

#[test]
fn import_rejects_other_digest_algorithm() {
    let text = sidecar(OBJECT_ID, "sha512", ABC_SHA256, "");
    assert_eq!(
        import(&text),
        Err(JsonError::UnsupportedDigestAlgorithm("sha512".into()))
    );
}

#[test]
fn import_rejects_other_object_id() {
    let text = sidecar("urn:example:object-2", "sha256", ABC_SHA256, "");
    assert_eq!(import(&text), Err(JsonError::ObjectIdMismatch));
}

#[test]
fn import_rejects_unknown_dcterms_term() {
    let text = sidecar(
        OBJECT_ID,
        "sha256",
        ABC_SHA256,
        &statement("dcterms", "name", "x"),
    );
    assert_eq!(
        import(&text),
        Err(JsonError::Register(MetadataError::UnknownDctermsTerm))
    );
}

#[test]
fn import_rejects_accrual_term_on_file() {
    let text = sidecar(
        OBJECT_ID,
        "sha256",
        ABC_SHA256,
        &statement("dcterms", "accrualPolicy", "x"),
    );
    assert_eq!(
        import(&text),
        Err(JsonError::Register(MetadataError::AccrualOnFile))
    );
}

#[test]
fn import_maps_dc_onto_dcterms() {
    let text = sidecar(
        OBJECT_ID,
        "sha256",
        ABC_SHA256,
        &statement("dc", "creator", "George"),
    );
    let record = import(&text).expect("import");
    assert_eq!(
        record.get("dcterms:creator").expect("creator").value,
        "George"
    );
}

#[test]
fn import_rejects_unregistered_namespace_by_name() {
    let text = sidecar(
        OBJECT_ID,
        "sha256",
        ABC_SHA256,
        &statement("rico", "title", "x"),
    );
    let err = import(&text).expect_err("rico is not registered");
    assert_eq!(err, JsonError::UnregisteredNamespace("rico".into()));
    assert!(err.to_string().contains("rico"));
}

#[test]
fn rejected_import_leaves_record_unchanged() {
    let mut record = Record::default();
    record
        .set(MetadataNamespace::Dcterms, "title", "Before")
        .expect("title");
    let statements = format!(
        "{},{}",
        statement("dcterms", "title", "After"),
        statement("dcterms", "name", "x")
    );
    let text = sidecar(OBJECT_ID, "sha256", ABC_SHA256, &statements);
    assert!(apply_json(&text, &abc_target(), &mut record).is_err());
    assert_eq!(record.get("dcterms:title").expect("title").value, "Before");
}

#[test]
fn import_merges_into_existing_record() {
    let mut record = Record::default();
    record
        .set(MetadataNamespace::Exif, "Model", "X100V")
        .expect("model");
    let text = sidecar(
        OBJECT_ID,
        "sha256",
        ABC_SHA256,
        &statement("dcterms", "title", "Harbour"),
    );
    apply_json(&text, &abc_target(), &mut record).expect("import");
    assert_eq!(record.statements.len(), 2);
}

#[test]
fn malformed_json_is_rejected() {
    assert!(matches!(import("{not json"), Err(JsonError::Malformed(_))));
}

#[test]
fn missing_digest_is_rejected() {
    let text = format!("{{\"objectId\":\"{OBJECT_ID}\",\"statements\":[]}}");
    assert!(matches!(import(&text), Err(JsonError::Malformed(_))));
}

#[test]
fn array_of_objects_is_rejected() {
    let one = sidecar(OBJECT_ID, "sha256", ABC_SHA256, "");
    assert!(matches!(
        import(&format!("[{one},{one}]")),
        Err(JsonError::Malformed(_))
    ));
}
