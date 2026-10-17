# Task mvp-milestone - Walkthrough

## Summary

Added an owner-edited "Current milestone" section to [pr-review-loop.md](../../.agents/rules/pr-review-loop.md). It names the milestone the agent merge permission covers: the FileSquid MVP, phases F0 to F3 of the [implementation plan](../implementation-plan.md), gated by Task FS-3.9 ([ADR 035](../architecture/035-macos-first-desktop-mvp.md)). Agents do not start tasks outside the scope on their own. After the gate task's PR merges, they stop and report for the owner's manual testing review. Work resumes when the owner sets the next milestone. [AGENTS.md](../../AGENTS.md) §1 points at the section instead of naming the milestone itself.

TDD-ENFORCE: Not applicable - documentation-only change.

## Validation

`bash scripts/pre-commit.sh` exited 0 (all gates 0 to 9, including Clippy, Cargo, rustdoc, ESLint, tsc and Vitest).

## Mocked Environment Variables

None.
