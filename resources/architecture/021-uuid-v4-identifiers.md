# Architecture Decision Record - FileSquid

021-uuid-v4-identifiers.md

## Title

RFC 4122 UUID version 4 for newly generated FileSquid identifiers

## Status

Accepted

Accepted: 2026-09-23T04:12:17Z.

## Context / Requirement Reference

Every stored FileSquid **file** needs a durable logical identity that survives rename, move, recatalog, and OCFL rebuild. [Architecture 4.1](../architecture.md#41-data-architecture) already requires stable opaque random identifiers that do not encode capture dates or filesystem paths. [TC-001](../prd.md#tc-001-immutable-originals-and-explicit-identity) requires logical asset IDs, source IDs, storage hashes, versions, relationships, and resource roles to remain distinct. [F-001](../prd.md#f-001-independent-local-library-and-ownership) and [F-049](../prd.md#f-049-product-component-terminology-and-item-type) make the file the item type in a portable library.

Those records did not pin a concrete identifier format. Unspecified “opaque string” leaves path-like names, type prefixes, timestamps, and content hashes available as accidental identities. OCFL 1.1 inventory `id` values are expected to be URIs ([OCFL inventory](https://ocfl.io/1.1/spec/#inventory)). FileSquid therefore needs one generation rule for domain IDs and a matching OCFL object identifier.

## Decision

Newly generated FileSquid identifiers for **repository**, **library**, **collection**, **virtual collection**, **virtual exhibit**, **file**, **file version**, **file resource**, and **folder** MUST be UUID version 4 as specified in [RFC 9562 §5.4](https://www.rfc-editor.org/rfc/rfc9562.html#name-uuid-version-4) (the random UUID previously specified in [RFC 4122](https://www.rfc-editor.org/rfc/rfc4122)).

| Rule | Requirement |
|---|---|
| Version | UUID version 4 (random). The version nibble MUST be `4`. |
| Variant | RFC 9562 / RFC 4122 variant. |
| Canonical stored string | Lowercase 8-4-4-4-12 hexadecimal with hyphens (`xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx`). |
| Randomness | CSPRNG. Native hosts use the OS CSPRNG; browser/WASM uses `crypto.getRandomValues`. MUST NOT use a non-crypto PRNG. |
| Lifetime | Minted once at first persist. The identifier MUST NOT change because of rename, recatalog, or export. |
| Import | A FileSquid package that already carries UUID v4 identifiers MUST keep them. FileSquid MUST NOT remint IDs on reimport of its own objects. |
| Distinct from digest | SHA-256 remains the storage digest ([ADR 009](./009-cryptography-webcrypto-compatible-suite.md)). Identical bytes from different sources MAY share a digest and MUST still receive distinct file IDs. |
| Distinct from source IDs | Provider/export identifiers stay on `SourceRecord`. They MUST NOT become the FileSquid file ID. |

**OCFL object identifiers.** The OCFL inventory `id` for a file object MUST be `urn:uuid:` followed by that file’s canonical UUID. Collection-snapshot objects MUST use their own UUID v4 in the same `urn:uuid:` form. Watermark or resolver identifiers MUST NOT be these private OCFL object IDs ([architecture 4.1 publication](../architecture.md#41-data-architecture)).

**Portable contracts.** Identifier types in `apps/filesquid/crates/core` (`filesquid-core`) remain opaque string newtypes at the API boundary. Newly generated values MUST satisfy this UUID v4 canonical form. Empty strings remain invalid.

## Rationale

UUID version 4 is random, widely implemented on desktop, WASM, and hosted hosts, and already has a URI form (`urn:uuid:`) that OCFL inventories can carry. It does not encode capture time, generation time, filesystem path, or item type, so identity stays a separate field from chronology, storage layout, and media kind. Keeping the digest as SHA-256 preserves TC-001: two imports of the same bytes remain two files.

## Assumptions

- Collision risk of UUID version 4 is acceptable for library-scale and hosted-tenant-scale identifier sets.
- Each library remains a private namespace; UUIDs still provide merge-safe identity when packages move between repositories.
- CSPRNG APIs are available in the Tauri host, the browser replica, and the hosted mailbox process.
- Exact on-disk OCFL path layout remains a later writer concern ([ADR 010](./010-ocfl-1-1-dual-profile.md), [ADR 016](./016-rust-ocfl-library.md)); this record only pins the inventory `id` URI and the logical File ID.

## Alternatives Considered

- **Unspecified opaque strings:** Rejected. Tests and implementations can then store paths or `file-1` labels as if they were product identity.
- **UUID version 7:** Rejected. Version 7 encodes a Unix timestamp of generation. Architecture 4.1 requires identifiers that do not encode dates.
- **UUID version 1:** Rejected. Version 1 encodes a timestamp and a node identifier.
- **ULID / KSUID / type-prefixed strings (`file-…`):** Rejected. Time-sortable or type-encoding IDs leak ordering or item kind that belongs in other fields.
- **Monotonic integers:** Rejected. They leak counts, collide across devices, and are a poor OCFL URI.
- **Content-hash identifiers:** Rejected by [TC-001](../prd.md#tc-001-immutable-originals-and-explicit-identity). The SHA-256 digest identifies bytes, not the logical file.

## Consequences / Implications

- CO-0.1 and later import/persist paths MUST mint UUID version 4 for new files and the other named domain entities.
- Catalog rows, sidecars, and OCFL inventories MUST round-trip the same canonical lowercase string (or `urn:uuid:` form at the OCFL `id`).
- Operational SQLite/PostgreSQL MAY store 16-byte UUID columns if the portable contract still exposes the canonical string.
- Existing scaffold fixtures that use non-UUID placeholder strings are not production identifiers; implementation of generation follows this ADR.
- Hosted tenant directory names that currently appear as `<opaque-user-id>` SHOULD use the same UUID v4 user identifier when those IDs are minted.

## Related Decisions / Notes

- [Architecture 4.1](../architecture.md#41-data-architecture)
- [TC-001](../prd.md#tc-001-immutable-originals-and-explicit-identity), [F-001](../prd.md#f-001-independent-local-library-and-ownership), [F-049](../prd.md#f-049-product-component-terminology-and-item-type)
- [ADR 009](./009-cryptography-webcrypto-compatible-suite.md), [ADR 010](./010-ocfl-1-1-dual-profile.md), [ADR 016](./016-rust-ocfl-library.md)
- [RFC 9562](https://www.rfc-editor.org/rfc/rfc9562.html), [RFC 9562 KB](../../.agents/kb/ietf-9562-uuid.md), [RFC 4122](https://www.rfc-editor.org/rfc/rfc4122), [OCFL 1.1 inventory](https://ocfl.io/1.1/spec/#inventory)
