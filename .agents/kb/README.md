---
type: Project Standard
title: Knowledge Base Bundle Guide
description: Purpose of .agents/kb as an OKF v0.2 knowledge bundle versus docs/references progressive
  disclosure.
tags:
- okf
- knowledge-base
- agents
status: stable
generated:
  by: process:kb-synthesizer
  at: '2026-07-26T18:00:00Z'
sources:
- id: okf-kb
  resource: .agents/kb/okf.md
  title: OKF v0.2 project synthesis
kb-title: Knowledge Base Bundle Guide
last-updated: '2026-07-26T18:00:00Z'
domain: Agent Framework, Documentation
---

# Knowledge Base (`.agents/kb`)


**OKF bundle:** This directory is an [Open Knowledge Format (OKF) v0.2](./okf.md) Knowledge Bundle. Browse concepts via [index.md](./index.md). Every concept file (except reserved `index.md` / `log.md`) MUST have YAML frontmatter with a non-empty `type`. New entries MUST be written using the [kb-synthesizer](../skills/kb-synthesizer/SKILL-kb-synthesizer.md) skill.

This directory contains **synthesized domain knowledge** optimized for AI agent conceptual understanding and project conventions.

## Purpose

Files in `.agents/kb` are intended to be explicitly referenced by users or automatically applied to impart established patterns, rules, and high-level summaries. They give the AI the conceptual grounding necessary to write consistent code and adhere to project standards.

## vs. References (`docs/references`)

| Directory | Purpose | Use Case | When Loaded |
|-----------|---------|----------|-------------|
| `.agents/kb/` | Synthesized AI knowledge, domain concepts, project conventions. | "How do we do things here?" | Loaded when conceptually relevant or explicitly referenced (e.g., `@.agents/kb/beam-testing.md`). |
| `docs/references/` | Raw technical specifications, whitepapers, third-party docs. | "What are the exact technical details of this protocol/API?" | Loaded via **Progressive Disclosure** only when a specific AI Skill explicitly requests the file. |

**Rule of Thumb:**
- Use **`.agents/kb`** for high-level concepts and project vocabulary you want the AI to learn.
- Use **`docs/references`** for dense specification documents, meant for human readears, that would waste context window tokens unless explicitly required for a highly specific task.

## What Goes Here

- **Best Practices:** e.g., `beam-testing.md`, `python-testing.md`.
- **Project Standards:** e.g., `conventional-commits.md`.
- **Glossary/Concepts:** Definitions of domain-specific terms.

## What Does NOT Go Here

- Full API documentation for third party services (put in `docs/references/`).
- 50-page protocol specifications (put in `docs/references/`).

