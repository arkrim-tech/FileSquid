# Architecture Decision Record - FileSquid

019-coding-style-and-lint.md

## Title

rustfmt, Clippy, ESLint, Prettier, cargo-deny/audit

## Status

Accepted

Accepted: 2026-09-22T07:00:00Z.

Paths updated 2026-10-15; see [ADR 031](./031-vanilla-typescript-ui-and-white-label-theming.md).

## Context / Requirement Reference

AGENTS.md linting mandate. [U-38](../product-vision.md#one-api-hosts-and-distributions) requires `cargo audit` / `cargo deny` for crates.io and bundled C libraries.

## Decision

- Format Rust with `cargo fmt` inside each crate.
- Lint Rust with `cargo clippy --all-targets --locked -- -D warnings`.
- Format/lint the UI with Prettier and ESLint inside `apps/filesquid/ui/` and `apps/imagesquid/`.
- CI MUST run `cargo deny` and/or `cargo audit` once a `deny.toml` exists; the scaffold documents the commands. First `deny.toml` is later work.
- Ruff, Mix format, and PHPCS are not this product’s linters.

## Rationale

Standard tools for stable Rust and TypeScript built with Vite ([ADR 031](./031-vanilla-typescript-ui-and-white-label-theming.md)).

## Assumptions

- Each crate has its own `Cargo.lock`.
- No root `package.json`.

## Alternatives Considered

- **Root workspace rustfmt:** Violates no-language-tooling-at-root.

## Consequences / Implications

Agents MUST lint inside each app before tests ([lint-enforce.md](../../.agents/rules/lint-enforce.md)).

## Related Decisions / Notes

- [ADR 013](./013-cargo-svelte-workspaces.md)
