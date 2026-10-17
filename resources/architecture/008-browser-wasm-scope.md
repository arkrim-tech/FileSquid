# Architecture Decision Record - FileSquid

008-browser-wasm-scope.md

## Title

Day-one WebAssembly scope: envelopes, hashes, replica queries — not codecs

## Status

Accepted

Accepted: 2026-09-16T01:01:07Z. Revised: 2026-09-22T04:34:37Z. Revised: 2026-10-15T03:34:43Z. Amended by [ADR 025](./025-client-side-filesquid-engines.md): `filesquid-ocfl` and `filesquid-metadata` run in browser WASM workers; codecs and the local filesystem backend stay out.

Amended 2026-10-15 by [ADR 032](./032-hosting-and-privacy-model.md): the encrypted browser host moves to the roadmap; the modules' `wasm32-unknown-unknown` build requirements stay in force so it remains reachable.

## Context / Requirement Reference

The independent browser app MUST decrypt and present authorized content without a running desktop ([FE-017](../product-vision.md#cross-platform-access), [T-002](../product-vision.md#client-processing-and-hosted-mailbox)). Shared Rust logic SHOULD not be rewritten in JavaScript where it is security-sensitive ([ADR 001](./001-shared-rust-core-with-tauri-and-axum-hosts.md)). Native filesystem APIs and some media libraries are unavailable in a browser.

## Decision

Compile **browser-safe** private-client, client-engine (`filesquid-ocfl`, `filesquid-metadata`) and portable-contract libraries to WebAssembly ([ADR 001](./001-shared-rust-core-with-tauri-and-axum-hosts.md), [ADR 025](./025-client-side-filesquid-engines.md)). Native processing (libvips, FFmpeg, LibRaw, the local-filesystem OCFL backend, PhotoKit) stays out of WASM.

**Replica bootstrap and search (required before browser implementation):** a fresh browser MUST be able to find accessible content without a running desktop. Encrypted catalog/index **shards**, signed snapshots and visible coverage are this ADR's bootstrap contract ([T-002](../product-vision.md#client-processing-and-hosted-mailbox), [T-022](../product-vision.md#caching)). Queries run over the decrypted replica after unlock. Incomplete index download, pending shards and unsupported semantic models MUST be visible. Cache eviction or storage loss produces recovery/regeneration states, not a silent empty library. Whole-collection search is search of **accessible, indexed** coverage, not a plaintext query to Axum.

**Browser MVP commands** (each MUST be assigned to shared Rust/WASM or a documented browser adapter; they MUST NOT silently require a desktop, and MUST NOT be reimplemented as a second TypeScript domain):

| Operation | Owner |
|---|---|
| Unlock, envelopes, hash/verify | WASM and/or WebCrypto with **shared test vectors** |
| Replica query, favorites, captions, collections | Shared private-client WASM |
| Upload ciphertext / resume | WASM `filesquid-ocfl` with the S3-compatible backend after client-side encryption; Axum issues presigned URLs and fences heads ([ADR 025](./025-client-side-filesquid-engines.md)) |
| Provenance/C2PA read, validate, mint and display | WASM `filesquid-c2pa` ([ADR 026](./026-filesquid-provenance-engine.md)); prepared derivatives from desktop/enrolled workers when pixel encoding is required |
| Supported metadata extract/inspect/correct in browser | WASM `filesquid-metadata`, committing `metadata.md` through WASM `filesquid-ocfl`; native write/scrub of original bytes follows capability matrix |
| Durable mutation publication | An OCFL version committed to the encrypted hosted root by WASM `filesquid-ocfl` (fenced head CAS); the mailbox relays signed notifications to other devices ([ADR 004](./004-axum-hosted-ciphertext-mailbox.md), [ADR 025](./025-client-side-filesquid-engines.md)) |
| RAW/libvips/FFmpeg | Not in browser; prepared derivatives or explicit waiting/unavailable |

**WebCrypto vs WASM:** WebCrypto MAY implement approved envelope primitives when it matches the crypto ADR. WASM owns hashing, replica queries, the OCFL and metadata engines, and any envelope logic WebCrypto cannot share. Expensive WASM MUST run on **workers**, not the UI thread. Large files MUST be processed incrementally. WASM does not by itself keep work off the main thread.

The browser displays prepared, browser-decodable derivatives when native RAW/HDR/video exceeds the declared runtime. Unavailable work shows an explicit state; it MUST NOT send plaintext to Axum to compensate.

Widening WASM codec scope requires measurement and a later ADR. It is not implied by "we have Rust." Desktop libvips format support does not imply browser decode of those formats.

## Rationale

WASM reuses the crypto and type logic that must not drift between desktop and web. Codecs in the browser via WASM would duplicate native builds, blow download size, and still lack disk/PhotoKit. Prepared derivatives plus honest capability states match the vision ([T-004](../product-vision.md#client-processing-and-hosted-mailbox)).

## Assumptions

- wasm-bindgen (or equivalent) packaging fits the Svelte client without a second UI.
- Envelope algorithms chosen in the crypto ADR will have a WASM-capable implementation.
- Browser storage quotas and eviction produce visible recovery states (already in the vision).

## Alternatives Considered

- **JavaScript-only crypto:** Drift risk versus desktop Rust. Rejected for envelopes/hashes.
- **Full media stack in WASM on day one:** Too large; not a desktop replacement. Rejected.
- **Hosted decode of private originals:** Forbidden.
- **No browser replica; every view hits Axum plaintext:** Forbidden.

## Consequences / Implications

Easier: one implementation of envelopes and replica rules. Harder: WASM builds exclude the native-processing library entirely but include the client engines ([ADR 001](./001-shared-rust-core-with-tauri-and-axum-hosts.md), [ADR 025](./025-client-side-filesquid-engines.md)); bootstrap/search and mutation publication must exist before calling the browser MVP complete. Browser performance still depends on replica paging, workers and caches ([ADR 007](./007-durable-jobs-and-interactive-performance.md)).

## Related Decisions / Notes

- [ADR 001](./001-shared-rust-core-with-tauri-and-axum-hosts.md), [ADR 003](./003-svelte-client-rendered-ui.md), [ADR 004](./004-axum-hosted-ciphertext-mailbox.md), [ADR 006](./006-libvips-native-stills-not-imagemagick.md)
