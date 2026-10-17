# SRS-F-029: Client encryption and key custody
# SRS-TC-002: Endpoint processing and hosted encryption
# ADR 009: WebCrypto-compatible AEAD, HKDF, SHA-256 and Ed25519/X25519 envelopes
Feature: Cryptographic envelope types
  As a host crate
  I want WebCrypto-compatible envelope parameter types
  So that desktop, WASM, and WebCrypto share one suite without storing keys in this crate

  Scenario: AES-256-GCM uses a 96-bit nonce and 16-byte tag
    When I construct a content envelope for purpose "content"
    Then the envelope algorithm should be "AES-256-GCM"
    And the nonce length should be 12 bytes
    And the tag length should be 16 bytes

  Scenario: Wrap wire layout is version, nonce, then ciphertext and tag
    When I inspect wrap wire parameters
    Then the wrap wire version should be 1
    And the wrap wire should start with version then a 12-byte nonce

  Scenario: HKDF info labels are domain-separated
    When I register HKDF info labels for content, evidence, wrap, and device
    Then the content HKDF info should be "filesquid/v1/content"
    And the evidence HKDF info should be "filesquid/v1/evidence"
    And the wrap HKDF info should be "filesquid/v1/wrap"
    And the device HKDF info should be "filesquid/v1/device"

  Scenario: Reusing an HKDF purpose label fails
    When I register HKDF info labels for content, evidence, wrap, and device
    And I reuse the content HKDF purpose
    Then HKDF purpose registration should fail

  Scenario: Argon2id wrap parameters are stored on the envelope
    When I construct Argon2id wrap parameters
    Then Argon2id memory should be 65536 KiB
    And Argon2id iterations should be 3
    And Argon2id parallelism should be 1
    And Argon2id output should be 32 bytes
    And Argon2id salt should be 16 bytes

  Scenario: Associated data binds version, key id, and purpose
    When I construct wrap associated data for key "key-1" and purpose "wrap"
    Then associated data should bind version, key id "key-1", and purpose "wrap"

  Scenario: Rewrap does not rewrite historical ciphertext in place
    When I construct a content envelope for purpose "content"
    And I rewrap the envelope for a new device
    Then the historical ciphertext should be unchanged
    And a new wrap record should exist

  Scenario: Library, evidence, wrap, and device purposes remain separate
    When I inspect key purposes
    Then library content, evidence, wrap, and device purposes should be distinct

  # Source
  # - resources/srs.md SRS-F-029: Client encryption and key custody
  # - resources/prd.md F-029: Client encryption and key custody
  # - resources/prd.md TC-002: Endpoint processing and hosted encryption
  # - resources/architecture/009-cryptography-webcrypto-compatible-suite.md
