# Former private-client library (`apps/client`)

The ADR 001 **private-client** family (catalog, search, replica and recipe **meaning**) was planned as a separate library crate at `apps/client`. It was never scaffolded. That logic is FileSquid's **application layer** inside [`filesquid-api`](./plan.md), the single FileSquid API that every host mounts.

- The planned `CL` task IDs are unchanged and live in the [api plan](./plan.md); global Task 0.10 dispatches CL-0.1–CL-0.3.
- `filesquid-api` links `filesquid-core`, `filesquid-ocfl`, `filesquid-metadata` and `filesquid-c2pa`, and builds the FileSquid OCFL application profile (file objects, collection snapshots, dual-profile encryption) against storage backends injected by the host ([ADR 025](../../../../../resources/architecture/025-client-side-filesquid-engines.md)).
- It MUST NOT link Tauri host APIs, libvips, FFmpeg or a SQLite driver, and MUST NOT do storage I/O itself (`std::fs`, network clients) ([API-S-006](./srs.md)).
- Native I/O (local filesystem backend, SQLite, codecs) stays in the hosts and the native-processing library.
- The ciphertext-only hosted service at `*.filesquid.tech` MUST NOT link its read paths over private plaintext ([ADR 032](../../../../../resources/architecture/032-hosting-and-privacy-model.md)).

Global sequence: [implementation-plan.md](../../../../../resources/implementation-plan.md).
