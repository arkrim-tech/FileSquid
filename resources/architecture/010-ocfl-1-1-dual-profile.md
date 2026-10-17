# Architecture Decision Record - FileSquid

010-ocfl-1-1-dual-profile.md

## Title

OCFL 1.1 owner-readable local profile and separate ciphertext hosted profile

## Status

Accepted

Accepted: 2026-09-16T22:24:57Z. Revised: 2026-09-22T04:34:37Z.

## Context / Requirement Reference

Canonical local state MUST be owner-readable OCFL 1.1 files and snapshots, not SQLite alone ([FE-001](../product-vision.md#independent-library), [FE-033](../product-vision.md#portable-preservation), [FE-065](../product-vision.md#independent-library), [U-03](../product-vision.md#platform-ownership-and-access), [ADR 005](./005-sqlite-local-postgres-hosted.md)). Hosted roots MUST describe ciphertext. The vision already requires this; it lacked an ADR.

## Decision

Preservation uses **OCFL 1.1** with two profiles:

1. **Default local:** owner-readable media, documented sidecars, inventories and collection snapshots. Sensitive HTTP evidence stays separately encrypted. Owners can copy a completed OCFL snapshot with ordinary file tools and rebuild supported organization without the operational database ([FE-065](../product-vision.md#independent-library)). Changes MUST remain visibly pending until that snapshot is durable ([T-035](../product-vision.md#canonical-ocfl-state-and-evidence)). Direct inspection and copy MUST NOT authorize in-place inventory or signed-original edits ([T-036](../product-vision.md#canonical-ocfl-state-and-evidence)).
2. **Hosted / optional encrypted package:** inventories and digest sidecars remain structurally readable over **ciphertext**; names, media, metadata, plaintext digests and HTTP evidence are encrypted per [ADR 009](./009-cryptography-webcrypto-compatible-suite.md). One opaque stable user ID maps to one independently exportable root. Shared buckets with per-user prefixes are the default; dedicated buckets are justified exceptions.

Commit sequence remains intent → durable OCFL version/snapshot → catalog/job update from the canonical receipt ([ADR 005](./005-sqlite-local-postgres-hosted.md), [ADR 007](./007-durable-jobs-and-interactive-performance.md)). Exact on-disk object layout is specified when the first library writer is implemented; this ADR freezes the profiles and authority, not every path name.

## Rationale

OCFL is the portable, versioned, fixity-aware layout the product already promised. Two profiles keep local convenience without putting plaintext in the mailbox.

## Assumptions

- OCFL 1.1 remains the published standard at implementation.
- Standard OCFL validators check structure/ciphertext fixity; clients check plaintext recovery.

## Alternatives Considered

- **BagIt or ad-hoc folders only:** Weaker versioning/fixity. Rejected as the canonical layout.
- **Ciphertext-only local libraries by default:** Violates owner-readable MVP.
- **SQLite as canonical store:** Forbidden by [FE-001](../product-vision.md#independent-library) and [U-03](../product-vision.md#platform-ownership-and-access).

## Consequences / Implications

Easier: exit and rebuild stories. Harder: dual-profile testing; layout/path names remain for first implementation of the OCFL library ([ADR 016](./016-rust-ocfl-library.md) freezes ownership and read/write/browse scope, not every path). No named cloud vendor is implied.

## Related Decisions / Notes

- [ADR 005](./005-sqlite-local-postgres-hosted.md), [ADR 009](./009-cryptography-webcrypto-compatible-suite.md), [ADR 012](./012-s3-compatible-ciphertext-storage.md), [ADR 016](./016-rust-ocfl-library.md)
- [OCFL 1.1](https://ocfl.io/1.1/spec/)
