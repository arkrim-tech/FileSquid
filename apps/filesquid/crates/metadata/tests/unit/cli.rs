//! CLI help and validate.

use filesquid_metadata::cli::{run, USAGE};

#[test]
fn help_prints_commands() {
    let code = run(&["help".into()]).expect("help");
    assert_eq!(code, 0);
    assert!(USAGE.contains("extract"));
    assert!(USAGE.contains("validate"));
}

#[test]
fn validate_title_ok() {
    let code = run(&["validate".into(), "dcterms:title".into()]).expect("ok");
    assert_eq!(code, 0);
}

#[test]
fn validate_name_fails() {
    let code = run(&["validate".into(), "dcterms:name".into()]).expect("fail");
    assert_eq!(code, 1);
}

#[test]
fn unknown_command_errors() {
    let err = run(&["mods".into()]).expect_err("mods");
    assert!(err.contains("unknown command"));
}

#[test]
fn set_and_get_round_trip() {
    let dir = std::env::temp_dir().join("filesquid-metadata-cli-set");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let object = dir.to_string_lossy().into_owned();
    let code = run(&[
        "set".into(),
        "dcterms:title".into(),
        "Harbour".into(),
        "--object".into(),
        object.clone(),
    ])
    .expect("set");
    assert_eq!(code, 0);
    let code = run(&[
        "get".into(),
        "dcterms:title".into(),
        "--object".into(),
        object,
    ])
    .expect("get");
    assert_eq!(code, 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn export_csv_contains_header() {
    let dir = std::env::temp_dir().join("filesquid-metadata-cli-csv");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let object = dir.to_string_lossy().into_owned();
    run(&[
        "set".into(),
        "dcterms:creator".into(),
        "George".into(),
        "--object".into(),
        object.clone(),
    ])
    .unwrap();
    let csv_path = dir.join("out.csv");
    let code = run(&[
        "export".into(),
        "csv".into(),
        csv_path.to_string_lossy().into_owned(),
        "--object".into(),
        object,
    ])
    .expect("export");
    assert_eq!(code, 0);
    let body = std::fs::read_to_string(&csv_path).unwrap();
    assert!(body.contains("dcterms:creator"));
    assert!(body.contains("George"));
    let _ = std::fs::remove_dir_all(&dir);
}
