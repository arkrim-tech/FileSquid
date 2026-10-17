# Architecture Decision Record - FileSquid

028-filesquid-ui-packages-and-panel-composition.md

## Title

Each filesquid engine ships a view API and a web UI package; filesquid-ocfl provides a Drive-style file browser with sidebar panel slots that filesquid-metadata and filesquid-c2pa fill

## Status

Accepted

Accepted: 2026-10-15T06:14:23Z. Owner directed that the `filesquid-ocfl` API supports an HTML/CSS/JS file browser that looks like Google Drive or Nextcloud, with a clear demarcation that all other file-related metadata belongs to `filesquid-metadata`, whose API supplies a sidebar panel the browser can show. Builds on [ADR 003](./003-svelte-client-rendered-ui.md), [ADR 025](./025-client-side-filesquid-engines.md), [ADR 026](./026-filesquid-provenance-engine.md) and [ADR 027](./027-filesquid-metadata-pluggable-stores.md).

Superseded in part 2026-10-15 by [ADR 030](./030-filesquid-api-and-transports.md) (one FileSquid API replaces the per-engine view APIs and client adapters) and [ADR 031](./031-vanilla-typescript-ui-and-white-label-theming.md) (one `@filesquid/ui` package of vanilla TypeScript custom elements replaces the per-engine Svelte packages). The file-fact demarcation and the named panel slots remain in force. `filesquid-c2pa` is now `filesquid-c2pa`; `apps/client` and `apps/frontend` no longer exist.

## Context / Requirement Reference

The three filesquid engines are separate products that composite products (FileSquid and its distributions) combine through public APIs ([product visions](../product-vision.md#modules)). Until now each engine exposed a Rust API and a CLI only. Developers and institutions adopting `filesquid-ocfl` on its own need a usable graphical file browser, and FileSquid's inspector already shows storage, metadata and C2PA information side by side (ImageSquid SRS-UI-002).

Without a rule for who owns which file facts, each UI would grow its own metadata handling, which [C-048](../product-vision.md#module-rules) forbids ("FileSquid MUST NOT keep a parallel metadata extract or write path").

## Decision

**1. Demarcation of file facts.**

| Owner | Facts it owns |
|---|---|
| `filesquid-ocfl` | Storage facts only: object identifier, logical path and the folders implied by path prefixes, versions, version `created` timestamp, version `user` (name, address) and `message`, digests and fixity status, size (from the storage backend), content location. A file-type label derived from the extension is a display hint only. |
| `filesquid-metadata` | Descriptive and technical facts about a file: embedded technical metadata (EXIF, ID3, IPTC, XMP), format identification beyond the extension hint, original creation or capture dates, descriptive metadata (Dublin Core, Schema.org), titles, captions, keywords, rights. |
| `filesquid-c2pa` | Content Credentials: manifests, validation results, signers, ingredients and actions. |
| Composite product (FileSquid's `filesquid-api`, and a distribution UI such as `apps/imagesquid`) | Application meaning: albums, collections, people, favorites, search indexes, recipes, and product-specific views. |

**2. View APIs.** Each engine exposes a UI-oriented view API of serializable request and response types, in addition to its core API:

- `filesquid-ocfl`: list a folder (logical-path prefix) at a pinned version with paging, file details, per-file version history, object versions, size lookup, fixity check, ranged content read for preview and download, and write operations that each produce a new OCFL version (add or upload, create folder, rename or move, remove, restore a previous version). OCFL cannot store an empty folder, so creating a folder means adding a file beneath the new path prefix; the UI MAY show a pending empty folder until then, but it is not persisted.
- `filesquid-metadata`: the properties API from [ADR 024](./024-filesquid-metadata-engine.md) (populated rows, prefixes, Add property search, set, remove, extracted versus owner-asserted values) for a `FileRef`.
- `filesquid-c2pa`: credential summary and validation report for a file.

**3. UI packages.** Each engine ships a web UI package inside its own directory (`apps/filesquid-*/ui/`, npm package `@filesquid/ocfl-ui`, `@filesquid/metadata-ui`, `@filesquid/provenance-ui`) so it moves with the crate to its own repository. Components are written in Svelte 5 ([ADR 003](./003-svelte-client-rendered-ui.md)) and also built as standard Web Components (custom elements) so non-Svelte applications can embed them. UI packages hold presentation only; all logic stays in the engine behind the view API.

**4. Transport-agnostic clients.** Each UI package calls its view API through a small client interface. Hosts supply the adapter: an in-browser WebAssembly worker running the engine, Tauri IPC on the desktop, or HTTP for a third party that deploys the engine on its own server. The ciphertext-only `*.filesquid.tech` service never runs the engines, so no client may use an HTTP adapter against it ([ADR 025](./025-client-side-filesquid-engines.md), [ADR 032](./032-hosting-and-privacy-model.md)); `filesquid-server` serves the FileSquid API over HTTP ([ADR 030](./030-filesquid-api-and-transports.md)).

**5. The filesquid-ocfl file browser.** `@filesquid/ocfl-ui` provides a Drive-style browser: navigation pane, breadcrumb, list and grid views (name, size, type hint, modified, version), preview area, and a right sidebar with a **Details** panel (storage facts) and a **Versions** panel (history and restore). The browser shows objects and their logical paths as folders and files. A host MAY supply a presentation adapter for an application profile (for example FileSquid's one-object-per-file profile, or decrypted names for an encrypted root). A reference stand-alone app (static web app or PWA) composes the browser with the WebAssembly engine.

**6. Sidebar panel slots.** The browser exposes named extension points (sidebar tabs, and preview renderers). Panels receive the selected `FileRef` and host-supplied byte access. `@filesquid/metadata-ui` provides a **Metadata** panel and `@filesquid/provenance-ui` provides a **Content Credentials** panel. No UI package depends on another; the host registers the panels it wants.

**7. Theming and wording.** Components read design tokens from CSS custom properties (light and dark), meet the project's accessibility contracts, and take all visible labels from replaceable string tables so products can apply their own vocabulary (for example ImageSquid's repo, library, album wording, ImageSquid U-05).

**8. ImageSquid composition.** ImageSquid builds its photo-specific views (Days, Months, Years, albums, viewer, studio) itself and composes the engine panels in its inspector:

| ImageSquid inspector area (ImageSquid SRS-UI-002) | Supplied by |
|---|---|
| Overview and Descriptive (file facts, `exif:`, `dcterms:`, `schema:` rows, Add property) | `@filesquid/metadata-ui` Metadata panel |
| C2PA (credentials, assertions, validation) | `@filesquid/provenance-ui` Content Credentials panel |
| Archive (OCFL object, versions, digests) | `@filesquid/ocfl-ui` Details and Versions panels |
| Source (F-041 URLs, locked HTTP evidence) | ImageSquid's own component |

ImageSquid's Archives view MAY embed the full `@filesquid/ocfl-ui` browser. The FileSquid default UI is the generic composition: the browser plus all three panels with internal names.

## Rationale

Engines that ship their own UI are far easier to adopt: an institution can browse an OCFL root without building a front end. Slots keep the products independent while letting them combine, which is the composition model the owner chose. The demarcation table gives every file fact one owner, so neither the browser nor a distribution grows a second metadata path. Svelte matches the existing UI stack, and Web Components remove the framework lock-in for outside developers.

## Assumptions

- Svelte 5's custom-element build is adequate for the panel components; any gaps are handled by a thin wrapper.
- The Drive-style browser targets one user and one storage root at a time. Multi-user accounts, sharing and server-side permissions (a Nextcloud-style server) are out of scope here and need their own decision.
- File-type hints by extension are acceptable in the browser; formal format identification is a `filesquid-metadata` concern.

## Alternatives Considered

- **Engines ship APIs only; every product builds its own UI:** Duplicates browser and panel work and slows independent adoption. Rejected.
- **One combined UI package for all three engines:** Couples the products and forces OCFL on metadata-only users. Rejected.
- **filesquid-ocfl's browser reads embedded metadata itself:** Breaks the demarcation and creates a parallel metadata path. Rejected.
- **React or framework-free UI:** Diverges from the accepted Svelte stack ([ADR 003](./003-svelte-client-rendered-ui.md)). Rejected; Web Components cover non-Svelte hosts.
- **A filesquid-ocfl HTTP server inside FileSquid's Axum:** Violates [ADR 025](./025-client-side-filesquid-engines.md). Rejected.

## Consequences / Implications

Easier: stand-alone adoption with a working GUI; FileSquid's inspector reuses tested panels; distributions become mostly composition. Harder: three npm packages with their own builds, UI tests against production artifacts, versioning and releases; view APIs must stay stable for outside hosts; each package needs a unique dev-server port and its own Playwright tests; the reference app needs hosting as static files only.

## Related Decisions / Notes

- Related: [ADR 003](./003-svelte-client-rendered-ui.md), [ADR 013](./013-cargo-svelte-workspaces.md), [ADR 023](./023-filesquid-ocfl-engine-and-parquet.md), [ADR 024](./024-filesquid-metadata-engine.md), [ADR 025](./025-client-side-filesquid-engines.md), [ADR 026](./026-filesquid-provenance-engine.md), [ADR 027](./027-filesquid-metadata-pluggable-stores.md).
- Requirements: [PRD F-055](../prd.md#f-055-default-ui-and-panel-composition), [FE-073](../product-vision.md#independent-library), [FileSquid U-01](../product-vision.md#platform-ownership-and-access), [C-055](../product-vision.md#module-rules), ImageSquid SRS-UI-002.
