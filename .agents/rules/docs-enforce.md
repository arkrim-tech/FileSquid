---
description: Ensure all code is comprehensively documented using rustdoc and TSDoc-style comments.
copyright: Pieter Van Garderen (2026)
license: CC-BY
---

# Inline Documentation Enforcement

**Purpose:** This rule dictates how an AI agent MUST document the codebase to satisfy the Inline Documentation Mandate in `AGENTS.md`.

## Active Enforcement

Before declaring any coding task "complete" or writing tests, the agent MUST verify that the following components include verbose inline documentation that clearly explains their function and intended usage:

1.  **ALL** Modules / crates
2.  **ALL** Public types (structs, enums, traits)
3.  **ALL** Public functions / methods
4.  **ALL** Complex or non-obvious code blocks

### Rust (every crate under `apps/filesquid/crates/`)
The agent MUST use rustdoc (`///` on items, `//!` on modules) on all public modules, types, and functions. These comments MUST be parseable by `cargo doc` (run `cd apps/filesquid && cargo doc --workspace --no-deps`).

### TypeScript (`apps/filesquid/ui`, `apps/imagesquid`)
The agent MUST use TSDoc-style comments (`/** ... */` with `@param`, `@returns`, `@throws` where they apply) on all exported modules, custom elements, the `FileSquidClient` API client and its transport adapters, view modules, theme and label loaders, and utilities. Types come from TypeScript, so do not repeat them in braces inside the comment.

*Failure to document code according to these standards violates the core AGENTS.md manifesto.*
