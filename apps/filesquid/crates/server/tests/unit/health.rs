//! Unit tests for `filesquid-server` composition.

use filesquid_api::ApiState;
use filesquid_server::{app, ServerConfig};

#[test]
fn app_builds_without_a_ui_directory() {
    let _router = app(&ServerConfig::default(), ApiState::new());
}

#[test]
fn config_reads_bind_and_ui_dir_from_lookup() {
    let config = ServerConfig::from_lookup(|key| match key {
        "FILESQUID_BIND" => Some("0.0.0.0:9000".to_string()),
        "FILESQUID_UI_DIR" => Some("/srv/ui".to_string()),
        "FILESQUID_DATA_DIR" => Some("/srv/data".to_string()),
        "FILESQUID_ADMIN_TOKEN" => Some("abc".to_string()),
        _ => None,
    })
    .expect("valid config");
    assert_eq!(config.bind.to_string(), "0.0.0.0:9000");
    assert_eq!(
        config.ui_dir.as_deref(),
        Some(std::path::Path::new("/srv/ui"))
    );
    assert_eq!(config.data_dir, std::path::PathBuf::from("/srv/data"));
    assert_eq!(config.admin_token.as_deref(), Some("abc"));
}

#[test]
fn config_debug_output_redacts_the_admin_token() {
    let config = ServerConfig {
        admin_token: Some("s3cret-token".into()),
        ..ServerConfig::default()
    };
    let text = format!("{config:?}");
    assert!(!text.contains("s3cret-token"), "{text}");
    assert!(text.contains("[redacted]"), "{text}");
}

#[test]
fn config_defaults_to_loopback_8083() {
    let config = ServerConfig::from_lookup(|_| None).expect("defaults");
    assert_eq!(config.bind.to_string(), "127.0.0.1:8083");
    assert!(config.ui_dir.is_none());
    assert_eq!(config.data_dir, std::path::PathBuf::from("filesquid-data"));
    assert!(config.admin_token.is_none());
}

#[test]
fn config_rejects_an_invalid_bind_address() {
    let err =
        ServerConfig::from_lookup(|key| (key == "FILESQUID_BIND").then(|| "nope".to_string()))
            .expect_err("invalid bind");
    assert!(err.contains("FILESQUID_BIND"));
}
