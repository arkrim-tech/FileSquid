# Task ADR-035 Walkthrough: macOS-first desktop MVP

Documentation-only task. TDD-ENFORCE: Not applicable - documentation update.

## C-056: FileSquid MVP scope

Owner decisions (2026-10-17): the first desktop target is macOS; Linux and Windows follow the MVP; the MVP is desktop-only (no `filesquid-server`); C2PA is limited to read, validation and unsigned import records; search, version restore and labels, and drive views, sort and ZIP download stay in; the MVP installer is an unsigned early-access DMG.

Revision (owner, 2026-10-17): libvips media workers (DE-0.5) stay in the MVP inside Task 0.5. An earlier draft had moved them out of the MVP without owner approval; that change is reverted. The owner then put the ADR 011 installer licence and patent review on the MVP path (ADR 035 decision 5; Tasks FS-3.1 and FS-3.9; DE-F3.1; PRD MVP exit gate).

## Changes

- [Product vision](../product-vision.md): new [C-056](../product-vision.md#release-sequence); FE-017, FE-075, U-34, T-011 and C-025 now say macOS first and add the MVP.
- [PRD](../prd.md): new "FileSquid MVP" row in the release gates; release order, F-050, F-057 and the platform statements updated; C-056 allocated.
- [SRS](../srs.md): SRS-F-057 now says macOS first and allows the unsigned early-access installer.
- [ADR 035](../architecture/035-macos-first-desktop-mvp.md) (new); ADRs 002, 006, 013 and 020 and the [architecture](../architecture.md) note the amendment.
- [Implementation plan](../implementation-plan.md): Phase F3 is now the MVP, closed by Task FS-3.9. The new Phase F3R holds the deferred scope and the release-1 gate (Task FS-3.5). New tasks: FS-2.7 and FS-3.9 to FS-3.13.
- Module plans: new rows DE-F3.3 and API-F1.9; DE-F3.1, DE-F3.2 and API-F1.7 re-scoped; section headers updated.
- [README](../../README.md): release column.

## Verification

`bash scripts/pre-commit.sh` exit 0. Raw output (tail):

```text
✅ Document Sync Validation passed.
Starting PRD/SRS Traceability Validation...
✅ Traceability Validation passed: 95 PRD requirements, 81 resolved SRS references.
✅ Portable Documentation validation passed.
✅ Workspace Boundary validation passed.
✅ Logging rule validation passed (no relevant staged files).
✅ ./apps/filesquid is properly formatted.
✅ ./apps/filesquid/ui is properly formatted.
✅ ./apps/imagesquid is properly formatted.
✅ Format validation passed.
✅ ADR structure (ISO 42010) passed.
FileSquid: 259 vision IDs, 259 allocated, 129 allocation rows
ImageSquid: 29 vision IDs, 29 allocated, 19 allocation rows
OK: every vision ID is allocated
✅ PRD allocation passed.
OK: every requirement is accounted for in a plan task
✅ Plan traceability passed.
checked 139 Markdown files: 0 broken link(s)
✅ Relative links and anchors passed.
checked 146 documents: 0 link(s) cross the FileSquid/ImageSquid boundary
✅ FileSquid and ImageSquid document boundary passed.
✅ Task cleanup passed.
✅ Documentation validation passed.
✅ Clippy passed.
  Scenario: Publishing to Bluesky is explicit, policy-checked and journaled
✅ Workspace tests passed.
✅ Module crate alone: filesquid-core passed.
✅ Module crate alone: filesquid-ocfl passed.
✅ Module crate alone: filesquid-metadata passed.
  Scenario: Publishing to Bluesky is explicit, policy-checked and journaled
✅ Module crate alone: filesquid-api passed.
✅ Rustdoc passed.
✅ ESLint passed.
✅ TypeScript passed.
✅ Vitest passed.
✅ ESLint passed.
✅ TypeScript passed.
✅ Vitest passed.
✅ Lint and test validation passed.
✅ All Pre-Commit Hooks Passed!
```
