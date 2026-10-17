---
copyright: Pieter Van Garderen (2026)
license: CC-BY
---

# Decoupled Execution Timelines

**Purpose:** Guarantee architectural specs can survive aggressive timeline refactoring without requiring massive document rewrites or excessive token consumption.

## Active Enforcement

1. **Agnostic Requirements:** The PRD (`resources/prd.md`) and SRS (`resources/srs.md` and the module `resources/srs.md` files) MUST strictly define the 'What' and the 'How' of a system without ever dictating the 'When' (execution timelines, phases, or task assignments).
2. **Execution Isolation:** All timeline serialization, execution phasing, and task chronologies MUST be exclusively isolated to the Implementation Blueprint (`resources/implementation-plan.md` and the module `resources/plan.md` files).
3. **Refactoring Resilience:** If Phase priorities change, you MUST ONLY change the implementation plan framework and MUST NOT inject execution timelines retroactively into the foundational requirements.
