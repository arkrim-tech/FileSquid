# Architecture Decision Record - FileSquid

025-client-side-filesquid-engines.md

## Title

filesquid-ocfl and filesquid-metadata run on every client (native desktop and browser WebAssembly), never on the ciphertext-only hosted service

## Status

Accepted

Accepted: 2026-10-15T03:34:43Z. Owner directed one client architecture for desktop and the independent web app/PWA: the same `filesquid-ocfl` and `filesquid-metadata` crates run in the Tauri process and, compiled to WebAssembly, in browser workers. Amends [ADR 001](./001-shared-rust-core-with-tauri-and-axum-hosts.md), [ADR 008](./008-browser-wasm-scope.md), [ADR 023](./023-filesquid-ocfl-engine-and-parquet.md) and [ADR 024](./024-filesquid-metadata-engine.md).

Amended 2026-10-15 by [ADR 032](./032-hosting-and-privacy-model.md): the modules MAY also run natively in the owner-operated `filesquid-server`; the encrypted browser host moves to the roadmap while the WASM build rules stay; the ciphertext-only `*.filesquid.tech` service still never links or runs them. Per [ADR 029](./029-filesquid-unified-product-and-distributions.md), `filesquid-provenance` is now `filesquid-c2pa`, and the private-client layer planned as `apps/client` is now planned in `filesquid-api` ([filesquid-api plan](../../apps/filesquid/crates/api/resources/plan.md)).

## Context / Requirement Reference

[ADR 001](./001-shared-rust-core-with-tauri-and-axum-hosts.md) placed OCFL in the native-processing library family (Tauri only). [ADR 008](./008-browser-wasm-scope.md) kept "OCFL disk" out of WebAssembly, and [ADR 024](./024-filesquid-metadata-engine.md) stated that WASM MUST NOT link OCFL I/O. Together these gave the browser a second metadata and preservation path: replica fields edited by `imagesquid_client`, with no `filesquid-metadata` extract/CRUD and no `filesquid-ocfl` write. That is two architectures for one product, which the no-redundancy rule in ADR 001 forbids for application backends and which the owner rejected for clients.

Two constraints were conflated:

1. **The server must not see private plaintext** ([T-001](../product-vision.md#client-processing-and-hosted-mailbox), [ADR 004](./004-axum-hosted-ciphertext-mailbox.md)). `filesquid-metadata` reads file bytes to extract EXIF, ID3 and XMP. Running it in Axum would require decryption on the server. This constraint is sound and is kept.
2. **A browser has no ordinary filesystem.** This is a storage-backend limit, not a reason to exclude the engines. [ADR 023](./023-filesquid-ocfl-engine-and-parquet.md) already requires `filesquid-ocfl` to expose a storage backend trait with local filesystem and S3-compatible backends. The current crate (`apps/filesquid/crates/ocfl/src/lib.rs`) still calls `std::fs` directly, so the trait does not exist yet. `filesquid-metadata` extraction (`src/extract.rs`) also opens source files with `std::fs` instead of reading logical files through `filesquid-ocfl`, and its CLI import/export reads and writes interchange files with `std::fs`.

[FE-069](../product-vision.md#independent-library) and [FE-070](../product-vision.md#independent-library) require FileSquid to preserve through `filesquid-ocfl` and to divert all metadata handling to `filesquid-metadata`. Those requirements make no exception for the browser.

## Decision

**1. Placement.** `filesquid-ocfl` and `filesquid-metadata` are **client engines**. They run:

| Host | Build | Storage backend it receives |
|---|---|---|
| Tauri desktop | native Rust, linked into the Tauri process | local filesystem (owner-readable profile); S3-compatible (encrypted hosted profile) |
| Independent web app / PWA | `wasm32-unknown-unknown`, in a dedicated web worker | S3-compatible over `fetch` (encrypted hosted profile); browser cache backend (OPFS/IndexedDB) for staging only |
| Axum hosted mailbox (`*.filesquid.tech`) | **not linked** | none |

The ciphertext-only `*.filesquid.tech` service MUST NOT depend on either crate. CI MUST keep failing its build if they appear in its dependency graph. The owner-operated `filesquid-server` MAY link them ([ADR 032](./032-hosting-and-privacy-model.md)) ([ADR 001](./001-shared-rust-core-with-tauri-and-axum-hosts.md)).

**2. Storage backend interface (filesquid-ocfl).** The ADR 023 storage backend trait is the only path to bytes. The engine core MUST NOT call `std::fs`, `tokio::fs` or any network client directly.

- Operations: put, get (ranged/streamed), head, list, exists, delete-for-staging, multipart where the backend needs it, and compare-and-swap on canonical heads ([ADR 012](./012-s3-compatible-ciphertext-storage.md)).
- The trait is **async and runtime-agnostic**. The engine MUST NOT depend on a specific executor. `Send` bounds MUST NOT be required on `wasm32`.
- Backends shipped by the crate: local filesystem (native only, behind a default-on `fs` feature), S3-compatible (native and WASM; a request-signing or presigned-URL adapter supplied by the caller), and an in-memory backend for tests. The browser cache backend MAY live in the caller (FileSquid) rather than the crate.
- Logical files MUST be streamed in bounded chunks. No API may require a whole object or a whole large file in memory (WebAssembly linear memory limit).

**3. WebAssembly compatibility.** Both crates MUST build for `wasm32-unknown-unknown` with default features off plus their portable features. Native-only parts (local filesystem backend, CLI binaries) MUST sit behind Cargo features or `cfg(not(target_arch = "wasm32"))`. `filesquid-metadata` extract adapters MUST take caller-supplied bytes or a reader plus a `FileRef` (not a filesystem path; the caller chooses where the bytes come from, independently of the configured store, [ADR 027](./027-filesquid-metadata-pluggable-stores.md)), and MUST be pure Rust that compiles to WASM, or be feature-gated and reported as unsupported in the browser. File-path convenience APIs MAY remain for the native CLI only. CI MUST build both crates for `wasm32-unknown-unknown`.

**4. Canonical store per host.**

- Desktop: the owner-readable local OCFL root stays canonical ([ADR 010](./010-ocfl-1-1-dual-profile.md)).
- Web app / PWA: the **encrypted hosted OCFL profile** in S3-compatible storage is canonical. The client encrypts before any byte leaves the device. Browser storage (OPFS, IndexedDB) is a cache and staging area only. Eviction MUST produce visible recovery states, never silent loss ([T-022](../product-vision.md#caching)).
- A browser-only user without a hosted root has no durable OCFL store. The UI MUST say so before accepting work that would need preservation.

**5. Server role stays ciphertext-only.** For browser and desktop writes to the hosted profile, Axum MAY issue scoped presigned object URLs and MUST fence head publication (native conditional writes or the PostgreSQL compare-and-swap record from ADR 012). Axum MUST NOT receive plaintext, keys, or decrypted metadata. Axum MUST NOT parse OCFL: it checks object lengths, checksums and receipts only. OCFL structural validation of the hosted profile runs in clients.

Desktop and browser clients MAY write the same hosted root. Each writer MUST first obtain a fenced `vN` reservation authorized before any content upload for that version, and head publication then uses compare-and-swap; head CAS alone is not sufficient ([ADR 016](./016-rust-ocfl-library.md), [ADR 012](./012-s3-compatible-ciphertext-storage.md)). A writer that loses the race MUST re-read the new head and rebase or report a conflict; it MUST NOT overwrite. Browser uploads therefore **are** OCFL commits once the fenced head publication succeeds. Until then they are visibly pending, and the mailbox relay only notifies other devices.

**6. FileSquid application profile moves to the shared client layer.** ADR 023 assigned profile construction (one OCFL object per file, collection snapshot objects, dual-profile encryption transform, receipt-driven catalog update) to `imagesquid_native`. That construction moves to the private-client layer in `filesquid-api` (planned as `apps/client` when this ADR was accepted), which links `filesquid-ocfl` and `filesquid-metadata` against an injected storage backend. `filesquid-api` still MUST NOT implement storage I/O itself (no `std::fs`, no network client); it only receives backends from its host. Each host injects its backend: Tauri passes local/S3 backends, the browser worker passes the S3-over-`fetch` and cache backends. `imagesquid_native` keeps libvips, FFmpeg, SQLite, LibRaw, PhotoKit and OS adapters.

**7. Unchanged.** libvips, FFmpeg, LibRaw and other codecs stay desktop-only ([ADR 008](./008-browser-wasm-scope.md)). The browser still shows prepared derivatives. C2PA processing is the third client engine, `filesquid-c2pa`, under the same placement rules ([ADR 026](./026-filesquid-provenance-engine.md)), with its manifests and sidecars stored through `filesquid-ocfl` by `filesquid-api`. Both crates stay independently buildable, testable and publishable FOSS modules ([ADR 029](./029-filesquid-unified-product-and-distributions.md) Decision 4).

## Rationale

One engine pair in every client means one extract/CRUD path, one OCFL writer, one set of receipts, and one test suite for desktop and web. The privacy guarantee comes from where decryption happens (the client), not from which client type is running. Running the engines in the browser keeps the server exactly as blind as before.

ADR 023 already demanded backend-agnostic storage, so this decision makes the browser one more backend user instead of a special case. Doing it while `filesquid-ocfl` is under 100 lines is far cheaper than retrofitting it after callers depend on `std::fs` paths.

## Assumptions

- Async functions in traits (stable since Rust 1.75) are sufficient for the backend trait without an executor dependency. A small blocking adapter (for example `pollster`) is acceptable in native CLI binaries.
- S3-compatible providers chosen under ADR 012 support CORS and presigned URLs, or a conditional-write fence via PostgreSQL, so a browser can upload multipart ciphertext directly.
- `kamadak-exif` and `id3` compile to `wasm32-unknown-unknown` through their reader APIs. This is verified by the first implementation task, not assumed in production.
- Browser workers can hold enough memory for chunked streaming of large originals. Exact chunk sizes are set by measurement under [ADR 007](./007-durable-jobs-and-interactive-performance.md).

## Alternatives Considered

- **Keep the engines desktop-only; browser edits replica fields (previous ADR 008/024 position):** Two metadata and preservation architectures. Rejected.
- **Run the engines in Axum for web users:** The server would decrypt private files to extract and write metadata. Violates T-001 and ADR 004. Rejected.
- **Browser defers all writes to an online desktop:** Breaks "works when every desktop is off" ([FE-017](../product-vision.md#cross-platform-access)). Rejected.
- **Make browser storage (OPFS) a canonical OCFL root:** Browser storage can be evicted and is not owner-copyable. Rejected as canonical; kept as a cache.
- **File System Access API to a user-chosen folder as a readable local root:** Chromium-only today. Deferred; MAY be added later as an additional backend without changing this decision.
- **Separate WASM-only OCFL implementation:** Second writer, divergent receipts. Rejected; one engine.

## Consequences / Implications

Easier: desktop and web share the same engine calls, tests and receipts; the inspector behaves the same in both hosts; institutions get a WASM-capable OCFL and metadata engine. Harder: `filesquid-ocfl` needs the async backend trait before more callers land; all three client engines (`filesquid-ocfl`, `filesquid-metadata` and, per [ADR 026](./026-filesquid-provenance-engine.md), `filesquid-provenance`) need a `wasm32` CI build; `apps/client` grows the application-profile layer and a dependency on all three engines; Axum gains presigned-URL issuance and head fencing for client writes; web users need a hosted root for durable preservation. The browser download grows by the size of the three engines plus extract adapters and the C2PA crypto backend, and MUST be measured.

## Related Decisions / Notes

- Amends: [ADR 001](./001-shared-rust-core-with-tauri-and-axum-hosts.md) (OCFL leaves the Tauri-only family), [ADR 008](./008-browser-wasm-scope.md) (OCFL and metadata engines enter WASM; codecs stay out), [ADR 023](./023-filesquid-ocfl-engine-and-parquet.md) (async, WASM-capable backend trait; profile construction moves to `apps/client`), [ADR 024](./024-filesquid-metadata-engine.md) (WASM build; browser is a caller). Wording aligned in [ADR 003](./003-svelte-client-rendered-ui.md), [ADR 004](./004-axum-hosted-ciphertext-mailbox.md), [ADR 013](./013-cargo-svelte-workspaces.md), [ADR 016](./016-rust-ocfl-library.md) and [ADR 022](./022-dublin-core-descriptive-metadata.md).
- Related: [ADR 009](./009-cryptography-webcrypto-compatible-suite.md), [ADR 010](./010-ocfl-1-1-dual-profile.md), [ADR 012](./012-s3-compatible-ciphertext-storage.md), [ADR 015](./015-prov-o-jsonld-and-c2pa-sdk.md).
- Component diagram: [component-diagram.html](./component-diagram.html) shows this decision against the code status.
- Requirements: [PRD F-051](../prd.md#f-051-ocfl-module), [PRD F-052](../prd.md#f-052-metadata-module), [FE-069](../product-vision.md#independent-library), [FE-070](../product-vision.md#independent-library).
- First implementation task: storage backend trait plus `wasm32` build for `filesquid-ocfl`, then the `wasm32` build for `filesquid-metadata` (see [implementation plan](../implementation-plan.md)).
