//! WebCrypto-compatible envelope parameter types.
//!
//! This module stores algorithm names, sizes, HKDF labels, and wrap layout. It
//! performs no key-storage I/O and links no cryptographic crates.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{DomainError, KeyId};

/// Content AEAD algorithm name.
pub const AES_256_GCM: &str = "AES-256-GCM";

/// AES-256-GCM nonce length (96 bits).
pub const NONCE_LEN_BYTES: usize = 12;

/// AES-GCM tag length.
pub const TAG_LEN_BYTES: usize = 16;

/// Wrap wire version byte placed before the nonce.
pub const WRAP_WIRE_VERSION: u8 = 0x01;

/// Argon2id memory parameter in KiB.
pub const ARGON2ID_MEMORY_KIB: u32 = 65_536;

/// Argon2id iterations.
pub const ARGON2ID_ITERATIONS: u32 = 3;

/// Argon2id parallelism.
pub const ARGON2ID_PARALLELISM: u32 = 1;

/// Argon2id output length in bytes (AEAD key).
pub const ARGON2ID_OUTPUT_LEN: usize = 32;

/// Argon2id salt length in bytes.
pub const ARGON2ID_SALT_LEN: usize = 16;

/// Ed25519 public key length in bytes.
pub const ED25519_KEY_LEN: usize = 32;

/// X25519 public key length in bytes.
pub const X25519_KEY_LEN: usize = 32;

/// HKDF info for library content keys.
pub const HKDF_INFO_CONTENT: &str = "filesquid/v1/content";

/// HKDF info for evidence keys.
pub const HKDF_INFO_EVIDENCE: &str = "filesquid/v1/evidence";

/// HKDF info for wrap keys.
pub const HKDF_INFO_WRAP: &str = "filesquid/v1/wrap";

/// HKDF info for device keys.
pub const HKDF_INFO_DEVICE: &str = "filesquid/v1/device";

/// Key purpose that selects an HKDF info label.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyPurpose {
    /// Library content encryption.
    Content,
    /// HTTP evidence encryption.
    Evidence,
    /// Data-key wrap.
    Wrap,
    /// Device enrollment keys.
    Device,
}

impl KeyPurpose {
    /// Return the domain-separated HKDF info label.
    pub const fn hkdf_info(self) -> &'static str {
        match self {
            Self::Content => HKDF_INFO_CONTENT,
            Self::Evidence => HKDF_INFO_EVIDENCE,
            Self::Wrap => HKDF_INFO_WRAP,
            Self::Device => HKDF_INFO_DEVICE,
        }
    }
}

/// Set of registered HKDF purposes. Labels MUST NOT be reused across purposes.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HkdfInfoSet {
    purposes: BTreeSet<KeyPurpose>,
}

impl HkdfInfoSet {
    /// Register a purpose once.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::DuplicateHkdfPurpose`] when `purpose` is already
    /// registered.
    pub fn register(&mut self, purpose: KeyPurpose) -> Result<(), DomainError> {
        if !self.purposes.insert(purpose) {
            return Err(DomainError::DuplicateHkdfPurpose);
        }
        Ok(())
    }

    /// Return the HKDF info for a registered purpose.
    pub fn info(&self, purpose: KeyPurpose) -> Option<&'static str> {
        self.purposes
            .contains(&purpose)
            .then_some(purpose.hkdf_info())
    }
}

/// AES-256-GCM content envelope parameters (ciphertext bytes are opaque).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentEnvelope {
    purpose: KeyPurpose,
    nonce: [u8; NONCE_LEN_BYTES],
    ciphertext: Vec<u8>,
}

impl ContentEnvelope {
    /// Construct an envelope for `purpose` with a 96-bit nonce.
    pub fn new(purpose: KeyPurpose, nonce: [u8; NONCE_LEN_BYTES], ciphertext: Vec<u8>) -> Self {
        Self {
            purpose,
            nonce,
            ciphertext,
        }
    }

    /// Return the AEAD algorithm name.
    pub fn algorithm(&self) -> &'static str {
        AES_256_GCM
    }

    /// Return the key purpose.
    pub fn purpose(&self) -> KeyPurpose {
        self.purpose
    }

    /// Return the nonce length in bytes.
    pub fn nonce_len(&self) -> usize {
        self.nonce.len()
    }

    /// Return the tag length in bytes.
    pub fn tag_len(&self) -> usize {
        TAG_LEN_BYTES
    }

    /// Borrow historical ciphertext. Rewrap MUST NOT rewrite this in place.
    pub fn historical_ciphertext(&self) -> &[u8] {
        &self.ciphertext
    }

    /// Plan a wrap to `device_key` without mutating historical ciphertext.
    ///
    /// This crate does not perform AEAD. The plan names the wrap wire layout
    /// hosts MUST use when they emit the new wrapped data-key payload.
    pub fn plan_rewrap(&self, device_key: KeyId) -> RewrapPlan {
        RewrapPlan {
            historical_ciphertext: self.ciphertext.clone(),
            target_key_id: device_key,
        }
    }
}

/// Planned wrap of an existing envelope to a new device key.
///
/// Historical ciphertext stays unchanged. Hosts emit wire bytes
/// `version || nonce || ciphertext||tag` using the ADR 009 sizes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewrapPlan {
    historical_ciphertext: Vec<u8>,
    target_key_id: KeyId,
}

impl RewrapPlan {
    /// Borrow the unchanged historical ciphertext.
    pub fn historical_ciphertext(&self) -> &[u8] {
        &self.historical_ciphertext
    }

    /// Return the target wrap key id.
    pub fn target_key_id(&self) -> &KeyId {
        &self.target_key_id
    }

    /// Return the wrap wire version byte.
    pub fn wrap_wire_version(&self) -> u8 {
        WRAP_WIRE_VERSION
    }

    /// Return the wrap nonce length.
    pub fn nonce_len(&self) -> usize {
        NONCE_LEN_BYTES
    }

    /// Return the wrap tag length.
    pub fn tag_len(&self) -> usize {
        TAG_LEN_BYTES
    }
}

/// Associated data for wrap AEAD: version, key id, and purpose label.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WrapAssociatedData {
    version: u8,
    key_id: KeyId,
    purpose: KeyPurpose,
}

impl WrapAssociatedData {
    /// Bind wrap AEAD additional data.
    pub fn new(key_id: KeyId, purpose: KeyPurpose) -> Self {
        Self {
            version: WRAP_WIRE_VERSION,
            key_id,
            purpose,
        }
    }

    /// Return whether AAD binds `version`, `key_id`, and `purpose`.
    pub fn binds(&self, version: u8, key_id: &KeyId, purpose: KeyPurpose) -> bool {
        self.version == version && &self.key_id == key_id && self.purpose == purpose
    }

    /// Return the wrap wire version.
    pub fn version(&self) -> u8 {
        self.version
    }
}

/// Layout of wrap wire bytes: version, then 12-byte nonce, then ciphertext||tag.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WrapWireLayout;

impl WrapWireLayout {
    /// Return the version byte.
    pub fn version(self) -> u8 {
        WRAP_WIRE_VERSION
    }

    /// Return the nonce offset (immediately after the version byte).
    pub fn nonce_offset(self) -> usize {
        1
    }

    /// Return the nonce length.
    pub fn nonce_len(self) -> usize {
        NONCE_LEN_BYTES
    }
}

/// Argon2id recovery-wrap parameters stored on the envelope.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Argon2idParams {
    memory_kib: u32,
    iterations: u32,
    parallelism: u32,
    output_len: usize,
    salt_len: usize,
}

impl Argon2idParams {
    /// Parameters from ADR 009. Stored so they can be raised later.
    pub fn recovery_default() -> Self {
        Self {
            memory_kib: ARGON2ID_MEMORY_KIB,
            iterations: ARGON2ID_ITERATIONS,
            parallelism: ARGON2ID_PARALLELISM,
            output_len: ARGON2ID_OUTPUT_LEN,
            salt_len: ARGON2ID_SALT_LEN,
        }
    }

    /// Return memory in KiB.
    pub fn memory_kib(self) -> u32 {
        self.memory_kib
    }

    /// Return iterations.
    pub fn iterations(self) -> u32 {
        self.iterations
    }

    /// Return parallelism.
    pub fn parallelism(self) -> u32 {
        self.parallelism
    }

    /// Return output length in bytes.
    pub fn output_len(self) -> usize {
        self.output_len
    }

    /// Return salt length in bytes.
    pub fn salt_len(self) -> usize {
        self.salt_len
    }
}

/// Ed25519 public key (32 bytes). This crate does not generate keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ed25519PublicKey([u8; ED25519_KEY_LEN]);

impl Ed25519PublicKey {
    /// Wrap 32 raw public-key bytes.
    pub fn from_bytes(bytes: [u8; ED25519_KEY_LEN]) -> Self {
        Self(bytes)
    }

    /// Borrow the public-key bytes.
    pub fn as_bytes(&self) -> &[u8; ED25519_KEY_LEN] {
        &self.0
    }
}

/// X25519 public key (32 bytes). This crate does not generate keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct X25519PublicKey([u8; X25519_KEY_LEN]);

impl X25519PublicKey {
    /// Wrap 32 raw public-key bytes.
    pub fn from_bytes(bytes: [u8; X25519_KEY_LEN]) -> Self {
        Self(bytes)
    }

    /// Borrow the public-key bytes.
    pub fn as_bytes(&self) -> &[u8; X25519_KEY_LEN] {
        &self.0
    }
}
