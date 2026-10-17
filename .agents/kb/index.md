---
okf_version: "0.2"
---

# ATPix Agent Knowledge Bundle

Progressive disclosure listing for `.agents/kb/` — an [OKF v0.2](./okf.md) Knowledge Bundle. Open a concept only when the task needs that domain.

# Bundle orientation

* [Knowledge Base Bundle Guide](./README.md) - Purpose of this directory vs `docs/references/` and OKF conventions.
* [Open Knowledge Format (OKF) 0.2](./okf.md) - How to produce and consume OKF concepts (required reading for kb-synthesizer).

# Protocols & product platforms

* [C2PA (Content Credentials)](./c2pa.md) - Manifest lifecycle, validation states, and UX obligations.
* [ProofMode × FileSquid playbook](./proofmode.md) - Human explainer and agent playbook for a later FileSquid milestone (ordered by the owner with Arkrim and IIIF/GLAM after the ImageSquid release) that verifies ProofMode capture credentials, stores them in OCFL, and chains FileSquid enhancements on share and export.
* [Oxford Common File Layout (OCFL) 1.1](./ocfl.md) - Versioned object layout for FS and S3-compatible stores.

# Project standards

* [Agent Skills 1.0](./agent-skills-1.0.md) - Progressive disclosure, vendor neutrality, multi-tool support.
* [Conventional Commits 1.0.0](./conventional-commits.md) - Required Git commit message format.
* [RFC 2119 requirement keywords](./rfc-2119-req-terms.md) - MUST / SHOULD / MAY for PRDs and SRS.
* [RFC 3339 date and time](./rfc-3339-date-and-time.md) - Internet timestamps; UTC default.
* [RFC 9562 UUIDs](./ietf-9562-uuid.md) - 128-bit UUID versions, variant bits, hex-and-dash text, and CSPRNG rules. Obsoletes RFC 4122.
* [ISO/IEC/IEEE 29148 SRS](./iso-29148-srs.md) - Unambiguous, complete, verifiable, consistent, traceable requirements.
* [ISO/IEC/IEEE 42010 ADRs](./iso-42010-adr.md) - Nine required information items for architecture decisions.
* [ISO 15836 / DCMI Metadata Terms](./iso-15836-dublin-core.md) - Dublin Core `/terms/` vocabulary (ISO 15836-2) for FileSquid descriptive metadata, with Schema.org 1:1 map.

# Implementation playbooks

* [Tauri 2](./tauri2.md) - Rust desktop host: capabilities, IPC, plugins, testing, and distribution (stable major 2).
