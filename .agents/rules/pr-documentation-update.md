---
description: Ensure all core documentation is updated before a Pull Request is prepared.
copyright: Pieter Van Garderen (2026)
license: CC-BY
---

# Pull Request Documentation Updates

**Purpose:** Guarantee that the permanent project documentation stays strictly synchronized with the codebase whenever new features, architecture changes, or schema updates are introduced in a branch.

## Pre-PR Prompt Execution

Every time a Pull Request (PR) is being prepared by the AI agent, the agent MUST implicitly execute the following instruction as a required step before opening the PR:

> "update @resources/product-vision.md @resources/prd.md @resources/srs.md the ADRs and the BDD feature scenarios based on the changes introduced in this branch."

## Execution Requirements

1. **Review Current Branch Changes:** The agent MUST review all commits, code changes, schema alterations, and new features introduced in the current working branch.
2. **Update Core Documents:** The agent MUST verify and update (`resources/product-vision.md`, `resources/prd.md`, `resources/srs.md`, and the module `resources/{plan,srs}.md` under `apps/filesquid/crates/<module>/`, `apps/filesquid/ui/`, or `apps/imagesquid/` that the branch touched) to reflect the new state of the application. Changes to the ImageSquid distribution also update its own `apps/imagesquid/resources/product-vision.md`, `prd.md` and `srs.md`.
3. **Architecture Decision Records:** The agent MUST ensure any architectural shifts (e.g. database schema changes, new core libraries, API structural changes) are formally documented as a new ADR in `resources/architecture/`.
4. **Behavioral Tests (BDD Scenarios):** The agent MUST verify that the automated feature scenarios accurately reflect the updated acceptance criteria and system behaviors. The agent MUST invoke the [@feature-writer](../skills/feature-writer/SKILL-feature-writer.md) skill to generate or verify at least one scenario for each unique PRD and SRS requirement, so no requirement is left without a scenario.
   - For a **Python/JS Stack**, this means updating the Gherkin `.feature` files located idiomatic to the boundary (e.g. `apps/web/tests/features/`).
   - For a **BEAM/Elixir Stack**, this means updating the native Elixir `Wallaby.Feature` macros located idiomatic to the boundary (e.g. `apps/my_app_web/test/features/`).
   - For a **PHP/JS Stack**, this means updating Behat (or the chosen single Gherkin runner) under each plugin (e.g. `apps/c2papress-audit/tests/features/`).
   - For a **Rust/JS Stack**, this means updating the Rust `cucumber` features under each crate (e.g. `apps/filesquid/crates/server/tests/features/`, `apps/filesquid/crates/ocfl/tests/features/`) and UI Gherkin under `apps/filesquid/ui/tests/features/` or `apps/imagesquid/tests/features/` when that runner owns the flow.
5. **Commit Updates:** The agent MUST commit these documentation updates to the current branch *before* completing the Pull Request creation process.
