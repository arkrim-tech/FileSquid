//! Unit tests for WebCrypto-compatible envelope types.

use filesquid_core::{
    Argon2idParams, ContentEnvelope, DomainError, HkdfInfoSet, KeyId, KeyPurpose, RewrapPlan,
    WrapAssociatedData, WrapWireLayout, AES_256_GCM, ARGON2ID_ITERATIONS, ARGON2ID_MEMORY_KIB,
    ARGON2ID_OUTPUT_LEN, ARGON2ID_PARALLELISM, ARGON2ID_SALT_LEN, HKDF_INFO_CONTENT,
    HKDF_INFO_DEVICE, HKDF_INFO_EVIDENCE, HKDF_INFO_WRAP, NONCE_LEN_BYTES, TAG_LEN_BYTES,
    WRAP_WIRE_VERSION,
};

#[test]
fn aes_gcm_nonce_and_tag_lengths() {
    let envelope = ContentEnvelope::new(KeyPurpose::Content, [7u8; 12], vec![1, 2, 3]);
    assert_eq!(envelope.algorithm(), AES_256_GCM);
    assert_eq!(envelope.nonce_len(), NONCE_LEN_BYTES);
    assert_eq!(envelope.tag_len(), TAG_LEN_BYTES);
    assert_eq!(NONCE_LEN_BYTES, 12);
    assert_eq!(TAG_LEN_BYTES, 16);
}

#[test]
fn wrap_wire_is_version_then_nonce() {
    let layout = WrapWireLayout;
    assert_eq!(layout.version(), WRAP_WIRE_VERSION);
    assert_eq!(layout.version(), 1);
    assert_eq!(layout.nonce_offset(), 1);
    assert_eq!(layout.nonce_len(), 12);
}

#[test]
fn hkdf_labels_are_domain_separated() {
    let mut set = HkdfInfoSet::default();
    set.register(KeyPurpose::Content).expect("content");
    set.register(KeyPurpose::Evidence).expect("evidence");
    set.register(KeyPurpose::Wrap).expect("wrap");
    set.register(KeyPurpose::Device).expect("device");
    assert_eq!(set.info(KeyPurpose::Content), Some(HKDF_INFO_CONTENT));
    assert_eq!(set.info(KeyPurpose::Evidence), Some(HKDF_INFO_EVIDENCE));
    assert_eq!(set.info(KeyPurpose::Wrap), Some(HKDF_INFO_WRAP));
    assert_eq!(set.info(KeyPurpose::Device), Some(HKDF_INFO_DEVICE));
    assert_eq!(HKDF_INFO_CONTENT, "filesquid/v1/content");
    assert_eq!(HKDF_INFO_EVIDENCE, "filesquid/v1/evidence");
    assert_eq!(HKDF_INFO_WRAP, "filesquid/v1/wrap");
    assert_eq!(HKDF_INFO_DEVICE, "filesquid/v1/device");
}

#[test]
fn duplicate_hkdf_purpose_is_rejected() {
    let mut set = HkdfInfoSet::default();
    set.register(KeyPurpose::Content).expect("content");
    assert_eq!(
        set.register(KeyPurpose::Content).unwrap_err(),
        DomainError::DuplicateHkdfPurpose
    );
}

#[test]
fn argon2id_recovery_parameters() {
    let params = Argon2idParams::recovery_default();
    assert_eq!(params.memory_kib(), ARGON2ID_MEMORY_KIB);
    assert_eq!(params.memory_kib(), 65_536);
    assert_eq!(params.iterations(), ARGON2ID_ITERATIONS);
    assert_eq!(params.parallelism(), ARGON2ID_PARALLELISM);
    assert_eq!(params.output_len(), ARGON2ID_OUTPUT_LEN);
    assert_eq!(params.salt_len(), ARGON2ID_SALT_LEN);
}

#[test]
fn associated_data_binds_version_key_and_purpose() {
    let key = KeyId::new("key-1").expect("key");
    let aad = WrapAssociatedData::new(key.clone(), KeyPurpose::Wrap);
    assert!(aad.binds(WRAP_WIRE_VERSION, &key, KeyPurpose::Wrap));
    assert!(!aad.binds(WRAP_WIRE_VERSION, &key, KeyPurpose::Content));
}

#[test]
fn rewrap_does_not_rewrite_historical_ciphertext() {
    let envelope = ContentEnvelope::new(KeyPurpose::Content, [0u8; 12], vec![9, 8, 7]);
    let original = envelope.historical_ciphertext().to_vec();
    let plan = envelope.plan_rewrap(KeyId::new("device-2").expect("key"));
    assert_eq!(plan.historical_ciphertext(), original);
    assert_eq!(envelope.historical_ciphertext(), original);
    assert_eq!(plan.target_key_id().as_str(), "device-2");
    assert_eq!(plan.wrap_wire_version(), WRAP_WIRE_VERSION);
    assert_eq!(plan.nonce_len(), 12);
    assert_eq!(plan.tag_len(), 16);
    let json = serde_json::to_string(&plan).expect("json");
    let restored: RewrapPlan = serde_json::from_str(&json).expect("plan");
    assert_eq!(restored, plan);
    assert_eq!(restored.historical_ciphertext(), original);
    assert_eq!(restored.target_key_id().as_str(), "device-2");
    assert_eq!(restored.wrap_wire_version(), WRAP_WIRE_VERSION);
    assert_eq!(restored.nonce_len(), 12);
    assert_eq!(restored.tag_len(), 16);
}

#[test]
fn key_purposes_remain_distinct() {
    let labels = [
        KeyPurpose::Content.hkdf_info(),
        KeyPurpose::Evidence.hkdf_info(),
        KeyPurpose::Wrap.hkdf_info(),
        KeyPurpose::Device.hkdf_info(),
    ];
    let mut unique = labels.to_vec();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), 4);
}
