---
trigger: always_on
description: Permanently split GitHub Pages manuals (docs/) from product and agent engineering files (resources/).
copyright: Pieter Van Garderen (2026)
license: CC-BY
---

# docs/ vs resources/ split

FileSquid reserves **`docs/`** for user and developer **manuals** produced with documentation generators and hosted with conventions such as GitHub Pages.

**`resources/`** holds product-engineering material that used to live under `docs/`: vision, PRD, SRS, architecture document, ADRs, templates, references, reports, and UI mockups. Agent task artifacts live under `apps/*/resources/tasks/`, or under repository-root `resources/tasks/` for documentation-only tasks that own no app directory.

## MUST

- Put GitHub Pages / published manuals only under repository-root `docs/`.
- Put ADRs in `resources/architecture/`.
- Put the FileSquid vision, PRD, SRS, architecture narrative, templates, references and reports under `resources/`. A distribution's own vision, plan and SRS live under its app (for example `apps/imagesquid/resources/`).
- Put Plan / ToDos / Walkthrough files in the owning workspace's `resources/tasks/`: `apps/filesquid/crates/<crate>/resources/tasks/`, `apps/filesquid/ui/resources/tasks/`, `apps/imagesquid/resources/tasks/`, or repository-root `resources/tasks/` for documentation-only tasks (vision, PRD, SRS, ADRs, plans), which own no app directory.
- Put product-wide UI mockups in `resources/ui-mockups/` and an app's own UI mockups in `apps/<app>/resources/ui-mockups/` ([workspace-boundaries.md](./workspace-boundaries.md) rule 5).
- Keep the global implementation plan at `resources/implementation-plan.md`.
- Search `resources/architecture/` before claiming an ADR is missing.

## MUST NOT

- Write product vision, PRD, SRS, ADRs, or task walkthroughs under repository-root `docs/`.
- Treat `docs/` as the Open Agent Dev default dump for requirements.
- Reverse owner moves from `docs/` to `resources/` without asking ([no-silent-restore.md](./no-silent-restore.md)).

## Template skills

Init and writer skills MAY still mention `docs/` as a generic template. For **this** repository, remap those outputs to `resources/` (and `apps/*/resources/tasks/` for walkthroughs) unless the user is explicitly authoring GitHub Pages manuals.
