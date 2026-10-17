# Architecture Decision Record - FileSquid

012-s3-compatible-ciphertext-storage.md

## Title

S3-compatible object storage and resumable multipart for ciphertext

## Status

Accepted

Accepted: 2026-09-16T22:24:57Z. Revised: 2026-09-22T04:34:37Z.

## Context / Requirement Reference

Hosted roots need object storage for ciphertext ([FE-033](../product-vision.md#portable-preservation), [FE-034](../product-vision.md#portable-preservation), [ADR 005](./005-sqlite-local-postgres-hosted.md)). The vision listed hosting provider/geography and cloud frameworks as undecided. Naming a single vendor would lock the stack before operations review.

## Decision

Hosted ciphertext uses an **S3-compatible** object API (put/get/head, listing as required, **multipart upload**, checksums). The application MUST NOT require Amazon-only APIs.

**Conflict guard (required):** completing or replacing an object that is an OCFL head or other canonical pointer MUST use a compare-and-swap. Prefer native conditional writes (`If-Match` / `If-None-Match` / equivalent). If a selected provider lacks them, the application MUST still fence the completion (for example a compare-and-swap record in PostgreSQL that authorizes the object key/generation before the new head is published). Competing multipart completions MUST NOT silently overwrite a canonical head. A provider that cannot support this MUST NOT be used for hosted OCFL.

Resumable uploads are S3 multipart (or an equivalent part protocol on that API). tus MAY be used as a front-door if it terminates into the same object parts; it is not a second media-identity model.

Provider, region, replication class and residency remain an **operations ADR** before a hosted pilot. The technical stack is the API shape, not “AWS” or “OVH.”

Default layout: shared bucket, enforced per-user prefix / OCFL root ([ADR 010](./010-ocfl-1-1-dual-profile.md)).

## Rationale

S3 is the de facto portable object API. Keeping the vendor out of the stack preserves mailbox portability.

## Assumptions

- At least one S3-compatible provider will pass multipart, checksum, and conditional-write (or equivalent fence) tests before hosted beta.
- PostgreSQL remains the operational catalog ([ADR 005](./005-sqlite-local-postgres-hosted.md)).

## Alternatives Considered

- **Named AWS-only stack:** Rejected as premature vendor lock-in.
- **Azure Blob or GCS native APIs as the only interface:** Rejected; adapters MAY exist later, S3-compatible is the contract.
- **tus as the identity of a file:** Rejected; tus is transport.

## Consequences / Implications

Easier: MinIO locally, any compatible host in production. Harder: test matrix across providers; geography still unset. The FileSquid OCFL library is the client that reads and writes OCFL on this API ([ADR 016](./016-rust-ocfl-library.md)); S3 is not a second, non-OCFL blob store.

## Related Decisions / Notes

- [ADR 004](./004-axum-hosted-ciphertext-mailbox.md), [ADR 005](./005-sqlite-local-postgres-hosted.md), [ADR 010](./010-ocfl-1-1-dual-profile.md), [ADR 016](./016-rust-ocfl-library.md)
- [tus](https://github.com/tus/tus-resumable-upload-protocol/blob/main/protocol.md) optional front-door only
