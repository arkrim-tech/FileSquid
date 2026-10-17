---
title: "Desktop Software Requirements Specification"
parent: "../../../../../resources/prd.md"
---

# Desktop SRS (scaffold)

`filesquid-desktop`, the Tauri 2 host for FileSquid and its distributions. Native file access stays out of the webview. Parent: [SRS-F-057](../../../../../resources/srs.md#srs-f-057-desktop-and-server-hosts).

| Local requirement | Parent |
|---|---|
| DE-S-001 The desktop serves the FileSquid API in-process through a Tauri custom URI scheme with no TCP listener (the API health route replaces `health_ping`). Any remaining Tauri command MUST be typed and explicitly authorized by a deny-by-default Tauri capability. | [PRD F-037](../../../../../resources/prd.md#f-037-setup-updates-and-diagnostics), [FE-064](../../../../../resources/product-vision.md#desktop-host-isolation) |
| DE-S-002 Capabilities are deny-by-default; no fs/shell/sql plugins at scaffold. | [PRD F-037](../../../../../resources/prd.md#f-037-setup-updates-and-diagnostics) |
| DE-S-003 Logs are stdout JSON. | [PRD NFR-009](../../../../../resources/prd.md#nfr-009-observability-and-capacity-economics), [SRS-NFR-009](../../../../../resources/srs.md#srs-nfr-009-observability-and-capacity-economics), [PRD F-061](../../../../../resources/prd.md#f-061-activity-journal-and-live-health-monitoring), [ADR 014](../../../../../resources/architecture/014-stdout-observability.md) |
| DE-S-004 Desktop links `filesquid-api`, which links `filesquid_ocfl`, `filesquid_metadata` and `filesquid_c2pa`, and injects the local filesystem and S3-compatible storage backends; the F5 browser host links the same crates; the ciphertext-only hosted service MUST NOT ([ADR 025](../../../../../resources/architecture/025-client-side-filesquid-engines.md), [ADR 032](../../../../../resources/architecture/032-hosting-and-privacy-model.md)). | [PRD TC-005](../../../../../resources/prd.md#tc-005-shared-rust-graph-hosted-mailbox-isolation-and-ocfl-writer) |
| DE-S-005 Every API request through the scheme MUST carry a per-launch token, and the event channel MUST sit behind the same client interface as the server's. | [PRD F-057](../../../../../resources/prd.md#f-057-desktop-and-server-hosts), [SRS-F-057](../../../../../resources/srs.md#srs-f-057-desktop-and-server-hosts), [PRD F-056](../../../../../resources/prd.md#f-056-single-filesquid-api), [FileSquid U-18](../../../../../resources/product-vision.md#one-api-hosts-and-distributions), [T-049](../../../../../resources/product-vision.md#api-and-hosts), [ADR 030](../../../../../resources/architecture/030-filesquid-api-and-transports.md) |
| DE-S-006 The webview MUST receive opaque handles and API responses, never raw filesystem paths or SQL. | [PRD F-057](../../../../../resources/prd.md#f-057-desktop-and-server-hosts), [SRS-F-057](../../../../../resources/srs.md#srs-f-057-desktop-and-server-hosts), [PRD F-056](../../../../../resources/prd.md#f-056-single-filesquid-api), [FileSquid U-18](../../../../../resources/product-vision.md#one-api-hosts-and-distributions), [ADR 002](../../../../../resources/architecture/002-tauri2-desktop-shell.md) |
| DE-S-007 One desktop binary MUST serve every distribution through a Tauri config overlay (product name, identifier, icons, UI directory). | [F-058](../../../../../resources/prd.md#f-058-white-label-distributions), [FileSquid U-34](../../../../../resources/product-vision.md#one-api-hosts-and-distributions), [ADR 029](../../../../../resources/architecture/029-filesquid-unified-product-and-distributions.md) |
