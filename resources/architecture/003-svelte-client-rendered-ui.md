# Architecture Decision Record - FileSquid

003-svelte-client-rendered-ui.md

## Title

Svelte as the single client-rendered user interface

## Status

Superseded

Accepted: 2026-09-16T01:01:07Z. Revised: 2026-09-22T04:34:37Z. Revised: 2026-10-15T03:34:43Z. Amended by [ADR 025](./025-client-side-filesquid-engines.md): the browser's canonical store is the encrypted hosted OCFL profile; browser storage is a cache. Revised: 2026-10-15T06:14:23Z. Amended by [ADR 028](./028-filesquid-ui-packages-and-panel-composition.md): each filesquid engine also ships a Svelte UI package (with a Web Components build) that ImageSquid composes.

Superseded 2026-10-15 by [ADR 031](./031-vanilla-typescript-ui-and-white-label-theming.md): UIs are vanilla TypeScript custom elements built with Vite, with no UI framework. The Svelte scaffold (`apps/frontend`) is removed. This record is kept as history.

## Context / Requirement Reference

Desktop, independent browser and reduced-capability mobile web MUST share one interface ([FE-017](../product-vision.md#cross-platform-access), [T-002](../product-vision.md#client-processing-and-hosted-mailbox), [WCAG 2.2 AA](../product-vision.md#standards-laws-and-regulations)). Private file metadata MUST NOT be rendered into HTML by a hosted server ([T-001](../product-vision.md#client-processing-and-hosted-mailbox), [ADR 004](./004-axum-hosted-ciphertext-mailbox.md)). The owner rejected React and does not want LiveView. They preferred Svelte on the understanding that it compiles to ordinary HTML/CSS/JS without a Svelte server.

## Decision

The only UI library is **Svelte** (current major line at writing: Svelte 5), compiled as a **client-rendered single-page application**.

The same Svelte app runs:

- inside the Tauri webview, calling Rust through IPC
- in an ordinary browser, calling Axum for operational/ciphertext APIs and decrypting privately in the client ([ADR 004](./004-axum-hosted-ciphertext-mailbox.md), [ADR 008](./008-browser-wasm-scope.md))

Rules:

- After build, the private UI is static assets plus client JavaScript. It MUST NOT require a Svelte, Node, Phoenix or FastAPI process to render private library state.
- SvelteKit server-side rendering, `+page.server` loaders and SvelteKit API routes MUST NOT own private file data. If SvelteKit is used, it is static/SPA adapter mode only. The default implementation MAY be Svelte plus Vite without a SvelteKit server.
- Marketing/public pages MAY later be prerendered as static HTML. That is a separate surface, not a second UI library.
- Reduced-capability mobile is layout and capability checks in this client, not a CSS hide of desktop-only work.
- Components talk to a **common application interface**. Presentation state (focus, selection, viewport, playhead, unsaved controls) stays in Svelte. Durable library state and jobs stay in Rust/WASM/replica. Tauri IPC and browser/WASM adapters MUST sit behind that interface, not scatter through the component tree ([ADR 001](./001-shared-rust-core-with-tauri-and-axum-hosts.md), [ADR 002](./002-tauri2-desktop-shell.md)).
- The independent browser product MUST define a PWA lifecycle: installable manifest, cached application assets, update flow, offline bootstrap of the replica, lock/logout cleanup of decrypted memory, and compatibility between UI version, WASM engine version, stored cache data and the hosted OCFL root. Browser storage (OPFS, IndexedDB) is a cache, never canonical; the canonical store is the encrypted hosted OCFL profile ([ADR 025](./025-client-side-filesquid-engines.md)). Cache/storage loss produces honest recovery states ([T-002](../product-vision.md#client-processing-and-hosted-mailbox), [T-022](../product-vision.md#caching)).

React, Solid, Vue, LiveView and hand-rolled vanilla application frameworks MUST NOT be added as parallel UI stacks.

## Rationale

Svelte compiles components to ordinary DOM-updating JavaScript. That matches the owner's goal: no React runtime, no UI server callbacks. A photo and video manager still needs client JavaScript (virtualized grid, selection, slideshow). Svelte is the authoring model; the user receives HTML, CSS and JS.

LiveView was investigated and rejected for the private workspace because hosted LiveView would need plaintext the server MUST NOT see, and local LiveView would add a BEAM sidecar.

## Assumptions

- Svelte 5 remains suitable for a long-lived SPA with accessibility and virtualization.
- Packaged UI tests against production artifacts remain mandatory ([AGENTS UI test mandate](../../AGENTS.md)).
- "No server callbacks" means no Svelte server, not "the app never calls Tauri or Axum."

## Alternatives Considered

- **React:** Already a documentation candidate. Owner rejected it. Not used.
- **Solid:** Closest compiler-style alternative. Not chosen because Svelte meets the requirement.
- **Vanilla HTML/CSS/JS:** Becomes an ad-hoc framework at Photos-class complexity. Rejected.
- **Phoenix LiveView:** Server-owned HTML. Incompatible with hosted privacy and desktop IPC. Rejected.
- **SvelteKit SSR as the product UI:** Would recreate a server-rendered private app. Rejected.

## Consequences / Implications

Easier: one component tree; compile-time UI updates without a React runtime. That is an expected authoring benefit, **not** proof of collection-browse performance; [U-11](../product-vision.md#platform-ownership-and-access) and [T-010](../product-vision.md#performance-acceptance) still require packaged measurements ([ADR 007](./007-durable-jobs-and-interactive-performance.md)). Harder: collection virtualization, keyboard and screen-reader contracts must be built as Svelte components with rigorous DOM tests against production artifacts; SvelteKit SSR is a footgun and is forbidden for private data. Public-site SEO, if any, is a later static-page decision.

## Related Decisions / Notes

- [ADR 001](./001-shared-rust-core-with-tauri-and-axum-hosts.md), [ADR 002](./002-tauri2-desktop-shell.md), [ADR 004](./004-axum-hosted-ciphertext-mailbox.md)
- npm `svelte` current at writing: 5.x. Pin at implementation.
