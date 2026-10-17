# Architecture Decision Record - FileSquid

018-inline-docs-generators.md

## Title

rustdoc for Rust crates; TSDoc comments in TypeScript built with Vite

## Status

Accepted

Accepted: 2026-09-22T07:00:00Z.

Paths updated 2026-10-15; see [ADR 031](./031-vanilla-typescript-ui-and-white-label-theming.md). Revised 2026-10-15 (Svelte dropped; TypeScript is built with Vite wherever a UI is needed): the JSDoc generator is retired and no separate TypeScript documentation generator is adopted.

## Context / Requirement Reference

AGENTS.md Inline Documentation Mandate. Python pdoc / Elixir ExDoc are the wrong generators for this stack.

## Decision

- Rust public items use rustdoc (`///`, `//!`). Generate with `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` from `apps/filesquid/`.
- Every UI is TypeScript (strict) built with **Vite**; there is no Svelte and no JavaScript-only source. TypeScript uses TSDoc-style `/** */` comments on every exported module, type, class and function in `apps/filesquid/ui/src/` and `apps/imagesquid/src/`. `tsc --noEmit` (`npm run check`) type-checks them and code review enforces their presence.
- No separate TypeScript documentation generator is adopted. The API reference for UI developers is the OpenAPI contract (`apps/filesquid/crates/api/openapi.json`) and the TypeScript types generated from it (`apps/filesquid/ui/src/api/schema.d.ts`), plus rustdoc for the Rust side. The JSDoc `"docs"` script was retired with the Svelte frontend.
- Product docs (vision, PRD, ADRs) stay under `resources/`. Generated user manuals stay under repository-root `docs/`.

## Rationale

Matches the language split in [ADR 001](./001-shared-rust-core-with-tauri-and-axum-hosts.md) and the TypeScript-and-Vite UI of [ADR 031](./031-vanilla-typescript-ui-and-white-label-theming.md) (which superseded [ADR 003](./003-svelte-client-rendered-ui.md)). Plain JSDoc tooling does not parse TypeScript, and the generated contract types already document the API surface UIs use.

## Assumptions

- Scaffold crates have a small public surface.

## Alternatives Considered

- **pdoc / ExDoc / phpDocumentor:** Wrong languages.
- **JSDoc generator:** Does not parse TypeScript. Retired.
- **TypeDoc:** Would add a generator dependency; not needed while the contract types and TSDoc in source serve UI developers. MAY be revisited if a published UI component reference is required.

## Consequences / Implications

Easier: `cargo doc` is built in; TypeScript needs no extra tool. Harder: TSDoc presence is enforced by review, not by a generator; a published UI component reference, if wanted later, needs its own decision.

## Related Decisions / Notes

- [docs-enforce.md](../../.agents/rules/docs-enforce.md)
