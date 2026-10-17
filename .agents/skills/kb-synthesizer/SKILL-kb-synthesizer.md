---
name: Knowledge Base Synthesizer
description: Synthesizes resources/references into OKF v0.2-conformant concept files under .agents/kb/
allowed-tools:
  - code_interpreter
copyright: Pieter Van Garderen (2026)
license: CC-BY
---

# Knowledge Base (KB) Synthesizer Skill

## Purpose

This skill reads raw technical documents in `resources/references/` (PDFs, whitepapers, specs) and synthesizes them into **high-signal, token-efficient Markdown concept files** in `.agents/kb/`.

**Mandatory output format:** Every concept file MUST be a conformant **Open Knowledge Format (OKF) v0.2** document. The project's OKF rules live in [`.agents/kb/okf.md`](../../kb/okf.md). Agents MUST load that concept (or this skill) before writing or revising KB entries.

`.agents/kb/` is treated as an **OKF Knowledge Bundle**: a directory of concept `.md` files, optional `index.md` / `log.md`, and progressive disclosure toward heavier sources under `resources/references/`.

## When to Use This Skill

- Explicit user request (e.g., "Synthesize the ERC-3643 spec into a KB file").
- After `vision-writer` when the user confirms new reference files should be synthesized.
- When revising existing `.agents/kb/` entries for OKF compliance.

## Prerequisites

Before writing any KB file, the agent MUST:

1. Read [`.agents/kb/okf.md`](../../kb/okf.md) (OKF v0.2 synthesis).
2. Confirm the target reference exists under `resources/references/` (or is an internal project standard without a PDF).
3. Never use reserved OKF filenames for concepts: **`index.md`** and **`log.md`** only.

---

## Workflow

### Step 1: Target Identification

Identify which file(s) in `resources/references/` (or which project standard) to synthesize. Confirm they exist and are readable.

### Step 2: Boilerplate Stripping

Read the source. Ignore tables of contents, legal preambles, marketing fluff, and non-engineering context.

### Step 3: Extraction & Synthesis

Extract actionable content only:

- **Constraints / laws** — MUST / MUST NOT rules
- **Data models** — schemas, required fields, payloads
- **Workflows** — mandatory sequences
- **Security / privacy** — crypto, audit, trust parameters

Rewrite as concise Markdown (headings, lists, tables). Do **not** paste long paragraphs. Prefer structural markdown (OKF §4.2).

### Step 4: Write as an OKF Concept (REQUIRED)

Every non-reserved `.md` file under `.agents/kb/` MUST satisfy OKF v0.2 conformance ([`.agents/kb/okf.md`](../../kb/okf.md) §11):

1. Parseable YAML frontmatter (`---` … `---`).
2. Non-empty **`type`** field.
3. If `index.md` / `log.md` are present at any level, they follow OKF §8 / §9 (not concept frontmatter, except root `index.md` MAY have `okf_version`).

#### Required frontmatter

| Key | Rule |
|-----|------|
| `type` | **REQUIRED.** Descriptive concept kind. Examples for this bundle: `Reference`, `Project Standard`, `Protocol`, `Playbook`. Not centrally registered; unknown types MUST be tolerated by consumers. |

#### Recommended frontmatter (SHOULD)

| Key | Rule |
|-----|------|
| `title` | Human display name |
| `description` | One-line summary (for `index.md` / search) |
| `resource` | Canonical URI of the underlying standard/spec when one exists |
| `tags` | YAML list of short categorization strings |

#### Provenance, trust, lifecycle (SHOULD for synthesized specs)

| Family | Keys | Rules (see okf.md §5) |
|--------|------|------------------------|
| Provenance | `sources` | List of entries; each entry **REQUIRES** `resource` (URL, bundle-relative `/path`, or scope descriptor). Use stable `id` when body footnotes cite the source. Optional credibility: `author`, `usage_count`, `last_modified`; optional sibling `usage_window`. |
| Trust | `generated`, `verified` | `generated: { by, at }` — `by` is an **actor** (§7); `at` is ISO 8601. `verified` is a list of `{ by, at }` (bare mapping ≡ one-element list). |
| Lifecycle | `status`, `stale_after` | `status`: `draft` \| `stable` \| `deprecated` (absent ⇒ `stable`). `stale_after`: absolute `YYYY-MM-DD` only. |

**Actor convention** (`generated.by`, `verified[].by`, `sources[].author`):

- Agents/tools: `<producer>/<version>` (e.g. `kb-synthesizer/1.0`)
- People: `human:<id>`
- Processes: `process:<id>` (e.g. `process:kb-synthesizer`)

Default for machine-written KB entries:

```yaml
generated:
  by: process:kb-synthesizer
  at: 2026-07-26T00:00:00Z   # use actual UTC write time (RFC 3339 / ISO 8601)
status: stable
```

#### Project extension keys (MAY — MUST NOT be rejected by consumers)

Preserve ATPix/project metadata as OKF extensions (§4.1):

| Key | Purpose |
|-----|---------|
| `kb-title` | Alias of title for older tooling (SHOULD match `title`) |
| `source-reference` | Primary path under `resources/references/` |
| `source-url` | Canonical web URL of the specification |
| `spec-version` | Version string of the source standard |
| `domain` | Short domain label(s) |
| `last-updated` | ISO 8601 when this concept was last revised |

#### Per-claim attribution

When a body claim derives from a source, use a markdown footnote whose label equals `sources[].id`:

```markdown
OCFL digests SHOULD be sha512.[^ocfl-3.4]

[^ocfl-3.4]: OCFL 1.1 §3.4 Digests
```

Also allow inline `(Source: §X)` citations for dense constraint lists.

#### Body structure

- No required body sections.
- Prefer: purpose/TL;DR, mental model tables, constraints, data models, workflows, agent rules, decision tables.
- Conventional headings when applicable: `# Schema`, `# Examples`, `# Computation` (Attested Computation only).
- Cross-links: prefer bundle-relative `/concept-id` form only when documenting full OKF bundles; within `.agents/kb/` use relative links such as `[OKF](./okf.md)` (portable, repo-friendly).
- Consumers MUST tolerate broken links; still prefer correct relative paths.

#### Filename rules

- Concept ID = path within the bundle **without** `.md`.
- Use intuitive kebab-case names: `ocfl.md`, `c2pa.md`, `rfc-2119-req-terms.md`.
- **MUST NOT** name a concept `index.md` or `log.md`.

#### Template (copy and fill)

```markdown
---
type: Reference
title: <Human title>
description: <One-line summary for agents and index.md>
resource: <https://… canonical URL or omit>
tags: [<tag>, <tag>]
status: stable
generated:
  by: process:kb-synthesizer
  at: <RFC3339 UTC>
sources:
  - id: primary
    resource: resources/references/<filename>
    title: <Source title>
    author: process:<publisher-or-org>
kb-title: <same as title>
source-reference: resources/references/<filename>
source-url: <https://…>
spec-version: "<version>"
last-updated: <RFC3339 UTC>
domain: <Domain labels>
---

# <Title>

**Purpose:** …

**TL;DR:** …

## Key Constraints
- … (Source: §…)

[^primary]: <Source title>
```

### Step 5: Bundle maintenance

After adding or revising concepts under `.agents/kb/`:

1. **Update `index.md`** (OKF §8) at the bundle root: no concept frontmatter except optional `okf_version: "0.2"`. List concepts with title + description from frontmatter. Group under section headings.
2. **Optionally append `log.md`** (OKF §9): newest-first date headings (`YYYY-MM-DD`), prose entries.
3. **Do not** put concept frontmatter on `index.md` / `log.md` beyond root `okf_version`.
4. `README.md` MAY exist as human orientation; if kept, either give it OKF concept frontmatter (`type`) or treat it as a non-concept sidecar documented in `index.md`. Prefer pointing humans at `index.md` for the OKF listing.

### Step 6: Finalization & validation

1. Save to `.agents/kb/<concept-id>.md`.
2. Validate:
   - YAML frontmatter parses.
   - `type` is non-empty.
   - Filename is not reserved.
   - `sources[].resource` present when `sources` is used.
   - `generated.by` uses actor convention when `generated` is present.
   - Footnote labels match `sources[].id` when footnotes are used.
3. Present the file path and a short summary to the user.

---

## What this skill MUST NOT do

- Emit legacy-only frontmatter without `type` (pre-OKF `kb-title` alone is **non-conformant**).
- Reject or omit unknown extension keys when round-tripping existing concepts.
- Store attestation **receipts** in the bundle (runtime only; OKF §10).
- Invent normative requirements not present in the source reference.
- Use absolute host filesystem paths in permanent docs (project links-enforce rule).

---

## Success Criteria

- ✅ **OKF conformance:** Every concept has parseable frontmatter + non-empty `type`; reserved files follow §8/§9.
- ✅ **High signal-to-noise:** Actionable rules only; fluff stripped.
- ✅ **Traceability:** Rules point to source sections; `sources` / footnotes used for primary references.
- ✅ **Provenance & trust:** Synthesized specs include `sources` and `generated` (and `status`).
- ✅ **Location:** File lives in `.agents/kb/`; `index.md` lists it.
- ✅ **Portable links:** Relative Markdown links only for in-repo paths.
- ✅ **No reserved-name collisions:** Concepts never named `index.md` or `log.md`.

## Reference

- Full OKF agent reference: [`.agents/kb/okf.md`](../../kb/okf.md)
- Upstream OKF v0.2: `resources/references/GoogleOpenKnowledgeFormat_v0.2.pdf`
- Bundle index: [`.agents/kb/index.md`](../../kb/index.md)
