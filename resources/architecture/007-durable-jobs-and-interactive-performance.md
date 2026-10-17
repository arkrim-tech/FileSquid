# Architecture Decision Record - FileSquid

007-durable-jobs-and-interactive-performance.md

## Title

Durable Rust jobs and client-side interactive media performance

## Status

Accepted

Accepted: 2026-09-16T01:01:07Z. Revised: 2026-09-22T04:34:37Z.

## Context / Requirement Reference

Interrupted imports, uploads, migrations and scans MUST resume without throwing away completed work ([T-027](../product-vision.md#durable-jobs), [T-028](../product-vision.md#durable-jobs), [U-27](../product-vision.md#one-api-hosts-and-distributions)). Everyday browsing MUST stay near-instant while that work runs ([U-11](../product-vision.md#platform-ownership-and-access), [T-010](../product-vision.md#performance-acceptance), ImageSquid high-performance UX). LiveView async tasks and UI component state were incorrectly hoped to provide resumption.

## Decision

**Jobs are durable work with identity, not UI processes.** Persisted rows are necessary and **not sufficient** for recovery.

Local jobs persist in SQLite; hosted operational jobs persist in PostgreSQL ([ADR 005](./005-sqlite-local-postgres-hosted.md)). The Svelte UI only observes progress. Exact job-table columns are specified when that schema is implemented; this ADR freezes the recovery contract, not the CREATE TABLE text.

This ADR **adopts** the recovery rules before ingestion ships ([T-005](../product-vision.md#canonical-ocfl-state-and-evidence), [T-027](../product-vision.md#durable-jobs), [T-035](../product-vision.md#canonical-ocfl-state-and-evidence)):

- every mutation/job step has an idempotent **operation ID**
- durable collection completion follows the **canonical receipt**, not the job row
- resume revalidates source identity and permissions
- workers take **exclusive leases**; an expired worker MUST NOT commit after a replacement owns the lease
- crash after an OCFL version and before the job-complete row MUST reconcile to the receipt without duplicating history or repeating completed media work
- ordinary restart uses bounded pending-job/checkpoint reconciliation, not a full-media rescan

**Window versus quit:** closing the last window MUST NOT destroy job checkpoints. Whether workers **keep running** after the last window closes, or **checkpoint and resume on next launch**, is a product setting that MUST be explicit in the UI. Durability (survive crash) and continuous execution (keep going with no window) are different promises. Default until that setting exists: checkpoint and resume on next launch; do not silently keep heavy decode/upload work after the user quit. Job **checkpoints** are durable in-progress job state (import, upload, migration). They are not OCFL **collection snapshots**.

Interactive media MUST stay local and immediate:

- scrolling, selection, zoom, video scrubbing and slideshow timing MUST NOT wait on Axum or on a full-library scan
- collection and virtual-collection views use indexed, paged queries and a bounded visible window
- previews are appropriately sized, cached, and prefetched nearby within explicit limits
- expensive decode/hash/OCFL/FFmpeg/libvips work runs in bounded workers, separate from rendering
- obsolete speculative work is cancelled after jumps; queues stay finite

The following numbers are **selected highlights** from [U-11](../product-vision.md#platform-ownership-and-access), [U-28](../product-vision.md#one-api-hosts-and-distributions), [U-27](../product-vision.md#one-api-hosts-and-distributions) and [T-010](../product-vision.md#performance-acceptance). They do not replace those requirements. Frame timing, browser/mobile profiles, retry caps, soak/endurance and other T-009–T-032 clauses remain binding. The SRS MAY refine them; it MUST NOT silently drop them:

- declared 4-core/16 GB/SSD desktop: warm interaction-to-visible-feedback p95 ≤100 ms; indexed metadata search p95 ≤2 s; first useful local viewport ≤3 s
- prepared thumbnail/viewer/next-slide correct paint p95 ≤100 ms; placeholders and spinners do not pass
- default staging cap 10 GiB with reserve max(2 GiB, 5% destination capacity)
- default active transfers ≤4; CPU-heavy workers ≤2
- proposed migration process memory ≤2 GiB excluding separately budgeted ML models
- restart reaches a resumable job state within 30 s without rescanning completed media

Rust enables this design. It does not make an unbounded queue or an unbounded collection view fast. **Accepted** here is implementation direction. FileSquid U-11 is not met until these budgets (and the rest of T-009–T-032) are measured on packaged desktop builds and the independent browser app, Linux/web first.

## Rationale

Resumption is a persistence problem. Interactivity is a client-and-worker scheduling problem. Mixing them into a LiveView socket or a REST round-trip for every scroll would fail both [T-010](../product-vision.md#performance-acceptance) and [T-027](../product-vision.md#durable-jobs).

## Assumptions

- Packaged-artifact UI and performance tests will exist before claiming budgets.
- Browser long-running work remains limited by the browser; desktop or enrolled workers take over as already specified in the vision ([C-001](../product-vision.md#platforms-originals-and-service-limits), [T-004](../product-vision.md#client-processing-and-hosted-mailbox)).
- Cache eviction never deletes originals or authoritative OCFL content.

## Alternatives Considered

- **LiveView/handle_async as the job system:** Dies with the view. Rejected.
- **Only in-memory queues:** Lost on crash. Rejected.
- **Celery/Oban beside Rust:** Second job stack. Rejected.
- **Server-rendered collection pages:** Violates privacy and the 100 ms budget. Rejected.

## Consequences / Implications

Easier: UI stays dumb about durability. Harder: crash recovery is **reconcile intent, canonical receipts and leases**, not `SELECT * FROM jobs WHERE status != done`. Worker fairness, prefetch cancellation and cache keys must be designed. Job SQL is a follow-up at coding time and MUST implement this contract.

## Related Decisions / Notes

- [ADR 001](./001-shared-rust-core-with-tauri-and-axum-hosts.md), [ADR 003](./003-svelte-client-rendered-ui.md), [ADR 004](./004-axum-hosted-ciphertext-mailbox.md), [ADR 005](./005-sqlite-local-postgres-hosted.md), [ADR 016](./016-rust-ocfl-library.md)
