# Architecture Decision Record - FileSquid

017-test-runners-and-reporting.md

## Title

cargo test, Vitest, Playwright, and Allure under each app

## Status

Accepted

Accepted: 2026-09-22T07:00:00Z.

## Context / Requirement Reference

AGENTS.md requires unit, UI, integration, performance, and BDD tests with structured reports. Scaffolding must not dump reports at the repository root ([artifact-isolation.md](../../.agents/rules/artifact-isolation.md)).

## Decision

- Rust crates use `cargo test` (optionally nextest later) with `[[test]]` paths under `tests/unit/` and `tests/integration/`.
- Frontend uses Vitest (jsdom) for unit/UI and Playwright against `npm run build` + preview for e2e.
- Desktop native e2e is WebdriverIO embedded (`@wdio/tauri-service`) under `apps/filesquid/crates/desktop/tests/e2e/` when first window tests ship; not required to compile the scaffold.
- Gherkin lives under each app `tests/features/`. The first runner is `cargo test` / Playwright; the `cucumber` crate is added when the first Rust Gherkin scenario is implemented.
- Allure/JUnit output stays in each app’s `tests/allure-results/`.
- Criterion benches and k6 against Axum are later performance work ([ADR 007](./007-durable-jobs-and-interactive-performance.md)).

## Rationale

Matches the test-folder taxonomy of the TDD rule ([tdd-enforce.md](../../.agents/rules/tdd-enforce.md)) without claiming FileSquid U-11 measurements.

## Assumptions

- No application features beyond health/ping exist yet.
- Playwright browsers are installed per developer machine (`npx playwright install`).

## Alternatives Considered

- **Jest + Locust:** Rejected; Vitest matches Vite; k6 is the HTTP load candidate.
- **Root-level Allure:** Violates artifact isolation.

## Consequences / Implications

Easier: each app owns its reports. Harder: CI must `cd` into each crate.

## Related Decisions / Notes

- [ADR 013](./013-cargo-svelte-workspaces.md), AGENTS.md Test-driven development.
