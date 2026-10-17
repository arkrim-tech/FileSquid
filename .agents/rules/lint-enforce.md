---
description: Ensure all code is strictly linted and formatted before pushing or testing.
copyright: Pieter Van Garderen (2026)
license: CC-BY
---

# Linting and Formatting Enforcement

**Purpose:** This file instructs the AI agent on exactly how to lint and format the codebase to satisfy the Test-Driven Development (TDD) linting mandate in `AGENTS.md`.

## Active Tools

Based on the accepted Architecture Decision Records in `resources/architecture/` for coding style ([ADR 019](../../resources/architecture/019-coding-style-and-lint.md)):

- **rustfmt** (`cargo fmt`) for Rust formatting.
- **Clippy** (`cargo clippy -D warnings`) for Rust static analysis.
- **ESLint** for TypeScript static analysis.
- **Prettier** for TypeScript (and CSS) formatting.

## Execution Requirements

Before running any tests, committing code, or completing an execution task, you MUST execute the following CLI commands inside the correct workspace.

### 1. Format each TypeScript UI package
Run in `apps/filesquid/ui/` and in `apps/imagesquid/`:
```bash
cd apps/filesquid/ui/
npx prettier --write .
cd ../../imagesquid/
npx prettier --write .
```

### 2. Lint each TypeScript UI package (and auto-fix)
Run in `apps/filesquid/ui/` and in `apps/imagesquid/`:
```bash
cd apps/filesquid/ui/
npx eslint . --fix
cd ../../imagesquid/
npx eslint . --fix
```

### 3. Format and lint the FileSquid Cargo workspace
Run once from the product-local workspace root. It covers every crate under `apps/filesquid/crates/`.
```bash
cd apps/filesquid/
cargo fmt --all
cargo clippy --workspace --all-targets --locked -- -D warnings
```

## Agent Resolution Mandate

If `eslint` or `cargo clippy` return a non-zero exit code, the agent **MUST NOT** proceed to testing or committing.

The agent MUST:

1. Review the CLI output.
2. Fix the violations.
3. Rerun the same commands until they exit 0.

Only when rustfmt, Clippy, Prettier, and ESLint all exit cleanly is the codebase valid for testing or PR submission.
