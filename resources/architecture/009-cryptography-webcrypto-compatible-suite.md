# Architecture Decision Record - FileSquid

009-cryptography-webcrypto-compatible-suite.md

## Title

WebCrypto-compatible AEAD, HKDF, SHA-256 and Ed25519/X25519 envelopes

## Status

Accepted

Accepted: 2026-09-16T22:24:57Z. Revised: 2026-09-22T04:34:37Z.

Amended 2026-10-15 per [ADR 029](./029-filesquid-unified-product-and-distributions.md): the HKDF info labels are renamed from `imagesquid/v1/*` to `filesquid/v1/*`. The rename happened before any envelope was produced, so no ciphertext exists under the old labels.

## Context / Requirement Reference

Hosted preservation MUST keep ordinary private plaintext and keys off the operator ([T-001](../product-vision.md#client-processing-and-hosted-mailbox), [FE-033](../product-vision.md#portable-preservation), [ADR 004](./004-axum-hosted-ciphertext-mailbox.md)). Browser unlock MUST work without a desktop ([ADR 008](./008-browser-wasm-scope.md), [U-22](../product-vision.md#one-api-hosts-and-distributions)). The vision still listed crypto algorithms as undecided. This ADR supplies the reviewed suite that works with [Web Cryptography](https://www.w3.org/TR/WebCryptoAPI/).

## Decision

The FileSquid content-crypto suite is:

| Role | Algorithm |
|---|---|
| Content AEAD | AES-256-GCM, 96-bit random nonce, never reuse key+nonce |
| Fixity / OCFL digest | SHA-256 |
| Key derivation | HKDF-SHA-256 with domain-separated info labels |
| Device/operation signatures | Ed25519 |
| Data-key wrap to devices | X25519 + AEAD wrap |
| Recovery passphrase wrap | Argon2id then AEAD (WASM/Rust; not a hosted KDF) |

Library keys, evidence keys, and publication/BYO disclosure keys remain separate. Rewrap MUST NOT rewrite historical ciphertext in place.

**Implementation contract (the audit verifies this; it does not replace the audit):**

- **Wrap AEAD:** AES-256-GCM, same as content. Wire version `0x01` then 12-byte nonce then ciphertext||16-byte tag. Associated data MUST bind version, key id, and purpose label.
- **HKDF:** SHA-256; IKM is the parent secret; salt is empty unless a documented envelope field supplies one. Info labels are UTF-8 and MUST NOT be reused across purposes: `filesquid/v1/content`, `filesquid/v1/evidence`, `filesquid/v1/wrap`, `filesquid/v1/device`.
- **Nonce:** 96-bit CSPRNG per encryption; never reuse with the same key; abort if the RNG fails. Do not use counters unless a later ADR proves uniqueness.
- **Argon2id (recovery passphrase):** 16-byte random salt stored with the envelope; memory 65536 KiB; iterations 3; parallelism 1; 32-byte output used as AEAD key. Parameters are stored in the envelope so they can be raised later without rewriting history.

Implementations: portable Rust (`aes-gcm`, `hkdf`, `sha2`, `ed25519-dalek`, `x25519-dalek`, `argon2`) in the contracts/crypto crate. Browsers MAY use WebCrypto for AES-GCM, HKDF and SHA-256 when test vectors match; Ed25519/X25519/Argon2id MAY run in WASM if WebCrypto is missing. Shared test vectors are mandatory.

Independent cryptographic review is a **future gate**, not completed by this ADR. This record is implementation direction, not an audit. The gate is **before a hosted preservation pilot** (when other people's ciphertext sits on FileSquid storage). Local-only desktop work MAY implement this suite first.

## Rationale

AES-GCM + SHA-256 + HKDF is the overlap of WebCrypto, OCFL, and Rust. Ed25519/X25519 are compact for device enrollment; WASM covers browsers that lack them. Argon2id is for human-held recovery, not for the server. A cloud KMS would put keys on the operator.

## Assumptions

- Reviewers will confirm nonce, AAD, and domain-separation labels before hosted launch.
- C2PA signing for paid publication may use a provider-chosen algorithm; that is [FE-042](../product-vision.md#paid-c2pa-publication-watermarking-and-ai-disclosure), not this envelope suite.

## Alternatives Considered

- **ChaCha20-Poly1305 only:** Excellent in software; weaker WebCrypto availability. Rejected as the sole AEAD.
- **P-256 only:** WebCrypto-native; rejected as the sole signature suite because Ed25519 is the device-key default; P-256 MAY be added if a review requires it.
- **PBKDF2 only for recovery:** WebCrypto-native but weaker than Argon2id. Rejected for passphrase wrap.
- **Operator-held KMS:** Violates the mailbox model.

## Consequences / Implications

Easier: one suite across desktop, WASM and WebCrypto. Harder: the audit below must still happen. SPA-asset signing is a **separate** release-integrity ADR, not this content-crypto suite.

**Future work — crypto audit (before hosted preservation pilot).** Someone independent of the people who implement the suite MUST check the **protocol**, not the brand names of the algorithms:

- How nonces and additional authenticated data are built (reuse of AES-GCM key+nonce is catastrophic).
- HKDF “info” labels so library keys, evidence keys, and wrap keys cannot be mixed up.
- Test vectors that match across Rust, WASM, and WebCrypto.
- Recovery envelopes: a passphrase can unlock the library without FileSquid’s servers holding keys.
- That the hosted mailbox still never sees plaintext or keys.

## Related Decisions / Notes

- [ADR 001](./001-shared-rust-core-with-tauri-and-axum-hosts.md), [ADR 004](./004-axum-hosted-ciphertext-mailbox.md), [ADR 008](./008-browser-wasm-scope.md), [ADR 010](./010-ocfl-1-1-dual-profile.md)
- [U-22](../product-vision.md#one-api-hosts-and-distributions), [T-001](../product-vision.md#client-processing-and-hosted-mailbox)
