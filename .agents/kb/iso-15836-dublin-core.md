---
type: Reference
title: ISO 15836 / DCMI Metadata Terms — Dublin Core /terms/ vocabulary
description: Agent-facing synthesis of ISO 15836-2:2019 and DCMI Metadata Terms in the /terms/ namespace for FileSquid descriptive metadata.
resource: https://www.dublincore.org/specifications/dublin-core/dcmi-terms/
tags:
- dublin-core
- iso-15836
- metadata
- dcterms
- schema.org
status: stable
generated:
  by: process:kb-synthesizer
  at: '2026-09-25T18:44:20Z'
sources:
- id: dcmi-terms
  resource: https://www.dublincore.org/specifications/dublin-core/dcmi-terms/
  title: DCMI Metadata Terms (Recommendation 2020-01-20)
  author: process:dcmi-usage-board
- id: iso-15836-2
  resource: https://www.iso.org/standard/71341.html
  title: ISO 15836-2:2019 Information and documentation — The Dublin Core metadata element set — Part 2 DCMI Properties and classes
  author: process:iso
- id: iso-15836-1
  resource: https://www.iso.org/standard/71339.html
  title: ISO 15836-1:2017 The Dublin Core metadata element set — Part 1 Core elements
  author: process:iso
- id: dc-schema-map
  resource: https://hackmd.io/_yBWOsCTQOiNxPzH-N3tpQ
  title: DCMI working notes — DC mappings to Schema.org
  author: process:dcmi
kb-title: ISO 15836 / DCMI Metadata Terms — Dublin Core /terms/ vocabulary
source-url: https://www.dublincore.org/specifications/dublin-core/dcmi-terms/
spec-version: '2020-01-20'
last-updated: '2026-09-25T18:44:20Z'
domain: Descriptive metadata, Interoperability, Photo libraries
---

# ISO 15836 / DCMI Metadata Terms

**Purpose:** Agent-facing synthesis of Dublin Core for FileSquid. Descriptive metadata keys for files are the properties in the `/terms/` namespace (`http://purl.org/dc/terms/`). That namespace is ISO 15836-2:2019. Load the [DCMI Recommendation](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/) for full comments, encoding schemes, and classes.[^dcmi-terms][^iso-15836-2]

**TL;DR:** FileSquid **descriptive** metadata is a set of `dcterms:` key/value pairs. Technical camera/file facts stay in `exif:` (and native IPTC/XMP/video packets as extracted observations). When a user wants a descriptive fact that has no `/terms/` property, suggest the closest **Schema.org** term and store it as `schema:`. Show every populated field with its namespace prefix. Do **not** dump empty keys.

---

## Novice Mental Model

| Everyday idea | Standard equivalent |
|---------------|---------------------|
| “What is this photo about?” | **Descriptive metadata** — `/terms/` properties (ISO 15836-2) |
| Title, creator, caption, rights, subject | `dcterms:title`, `dcterms:creator`, `dcterms:description`, `dcterms:rights`, `dcterms:subject` |
| Camera GPS, aperture, serial | **Technical metadata** — EXIF / native packets (`exif:`), not Dublin Core |
| The 15 original Dublin Core elements | ISO 15836-1 `/elements/1.1/` (`dc:`) — legacy; FileSquid authors `/terms/` |
| Qualified / refined Dublin Core | `/terms/` refinements (`created`, `spatial`, `license`, …) |
| A fact Dublin Core cannot name | Schema.org property (`schema:certification`, `schema:keywords`, …) |
| Stable file identity | FileSquid `FileId` (UUID v4) — not a Dublin Core key |

**Key insight:** Dublin Core names **resource description** (who made it, what it is about, rights, relations). EXIF names **capture/device facts**. Schema.org fills gaps Dublin Core does not cover. FileSquid keeps those three namespaces visible and separate.

---

## Namespaces and ISO parts

| Prefix | Base URI | ISO | FileSquid use |
|--------|----------|-----|----------------|
| `dcterms` | `http://purl.org/dc/terms/` | ISO 15836-2:2019 | **Authoring namespace** for descriptive metadata |
| `dc` | `http://purl.org/dc/elements/1.1/` | ISO 15836-1:2017 | Read-only alias for the 15 core terms when importing legacy packets; map to `dcterms:` on write |
| `dctype` | `http://purl.org/dc/dcmitype/` | outside ISO 15836-2 | Type vocabulary (`StillImage`, `MovingImage`); optional value for `dcterms:type` |
| `schema` | `https://schema.org/` | — | Overflow descriptive terms with no `/terms/` equivalent |
| `exif` | EXIF tag namespaces / TIFF IFDs | — | Technical metadata (latitude, f-number, serial, …) |

DCMI created `/terms/` in 2001 for refinements, then in 2008 mirrored the original fifteen elements there with formal ranges. There is both `dc:date` (no range) and `dcterms:date` (range Literal). Treat the fifteen parallel properties as equivalent for non-RDF storage; **author `dcterms:`**. DCMI encourages `/terms/` while keeping `/elements/1.1/` indefinitely. (Source: DCMI Terms §1)[^dcmi-terms]

ISO 15836-2 includes `/terms/` **properties and classes**. It does **not** include: `/elements/1.1/` terms, vocabulary encoding schemes, syntax encoding schemes, the DCMI Type vocabulary, or DCAM vocabulary-description terms. (Source: ISO 15836-2 scope)[^iso-15836-2]

Non-RDF implementations (JSON, sidecars, SQL) MAY ignore RDF identifier machinery and treat domain/range/subproperty as usage hints. (Source: DCMI Terms §1)

---

## Display and identity

Show populated fields as compact namespaced pairs:

```
exif:GPSLatitude "49.3"
dcterms:creator "George"
schema:certification "WERT level1"
```

| Rule | Detail |
|------|--------|
| Prefix | `exif:`, `dcterms:`, `schema:` (CURIE). Do **not** label `/terms/` fields `dc:` on write. |
| There is no `dc:name` / `dcterms:name` | A name **of the resource** is `dcterms:title` (Schema.org closest: `schema:name`). A **person** is `dcterms:creator` or `dcterms:contributor`. |
| FileId | Logical identity per ADR 021. `dcterms:identifier` MAY hold additional identifiers (DOI, URN). Do not replace FileId with a Dublin Core field. |
| Values | Literals as quoted strings; dates as RFC 3339 / W3CDTF (`YYYY`, `YYYY-MM`, `YYYY-MM-DD`, or `start/end` periods). Recommended practice for `dcterms:date` is ISO 8601-1 / W3CDTF / EDTF. (Source: `date` comment) |

---

## Properties in `/terms/` (authoring keys)

55 properties. URI for each is `http://purl.org/dc/terms/{name}`. **FileSquid file descriptive metadata MAY use any of these except the three Collection-domain accrual properties**, which belong on collection/library records.

### Core fifteen (also in ISO 15836-1 as `dc:`)

| Term | Label | Definition |
|------|-------|------------|
| `contributor` | Contributor | An entity responsible for making contributions to the resource. |
| `coverage` | Coverage | Spatial or temporal topic, spatial applicability, or jurisdiction. Prefer `spatial` / `temporal`. |
| `creator` | Creator | An entity responsible for making the resource. Prefer a URI; a literal MAY identify the creator. |
| `date` | Date | A point or period of time associated with a lifecycle event. Prefer a date subproperty. |
| `description` | Description | An account of the resource (abstract, TOC, graphical representation, or free text). |
| `format` | Format | File format, physical medium, or dimensions. Prefer IMT/MIME for file format. |
| `identifier` | Identifier | An unambiguous reference within a given context (ISBN, DOI, URN, HTTP URI). |
| `language` | Language | A language of the resource. Prefer ISO 639 or BCP 47. |
| `publisher` | Publisher | An entity responsible for making the resource available. |
| `relation` | Relation | A related resource. Prefer a URI. Prefer a specific relation subproperty. |
| `rights` | Rights | Information about rights held in and over the resource. Prefer a URI. |
| `source` | Source | A related resource from which the described resource is derived. |
| `subject` | Subject | A topic of the resource. Prefer a URI / controlled vocabulary. |
| `title` | Title | A name given to the resource. Range: Literal. |
| `type` | Type | Nature or genre. Prefer DCMI Type (`StillImage`, `MovingImage`). Use `format` for file format. |

### Date refinements (subproperties of `date`; range Literal)

| Term | Label |
|------|-------|
| `available` | Date Available |
| `created` | Date Created |
| `dateAccepted` | Date Accepted |
| `dateCopyrighted` | Date Copyrighted |
| `dateSubmitted` | Date Submitted |
| `issued` | Date Issued |
| `modified` | Date Modified |
| `valid` | Date Valid |

Photo capture time maps to `dcterms:created` (descriptive). EXIF DateTimeOriginal remains the technical observation.

### Coverage, description, format, rights, title refinements

| Term | Label | Notes |
|------|-------|-------|
| `spatial` | Spatial Coverage | Subproperty of `coverage`. Range includes Location. |
| `temporal` | Temporal Coverage | Subproperty of `coverage`. Range includes PeriodOfTime. |
| `abstract` | Abstract | Subproperty of `description`. |
| `tableOfContents` | Table Of Contents | Subproperty of `description`. |
| `extent` | Extent | Size or duration. File size in megabytes; duration ISO 8601. |
| `medium` | Medium | Material or physical carrier. Domain includes PhysicalResource. |
| `accessRights` | Access Rights | Who may access the resource, or security status. Subproperty of `rights`. |
| `license` | License | Legal document giving permission. Prefer a URI. |
| `rightsHolder` | Rights Holder | Person or organization owning or managing rights. |
| `alternative` | Alternative Title | Alternative name. Range Literal. Subproperty of `title`. |

### Relation refinements (non-literal related resources)

`conformsTo`, `hasFormat`, `hasPart`, `hasVersion`, `isFormatOf`, `isPartOf`, `isReferencedBy`, `isReplacedBy`, `isRequiredBy`, `isVersionOf`, `references`, `replaces`, `requires`.

Inverse pairs: `hasFormat`/`isFormatOf`, `hasPart`/`isPartOf`, `hasVersion`/`isVersionOf`, `references`/`isReferencedBy`, `replaces`/`isReplacedBy`, `requires`/`isRequiredBy`.

### Audience, instruction, identifier, provenance

| Term | Label | Notes |
|------|-------|-------|
| `audience` | Audience | Class of agents for whom the resource is intended. |
| `educationLevel` | Audience Education Level | Subproperty of `audience`. |
| `mediator` | Mediator | Entity that mediates access (parent, teacher, …). Subproperty of `audience`. |
| `instructionalMethod` | Instructional Method | Instructional process the resource supports. |
| `bibliographicCitation` | Bibliographic Citation | Subproperty of `identifier`. Range Literal. |
| `provenance` | Provenance | Custody/ownership changes significant for authenticity. **Descriptive statement.** Do not treat as C2PA or PROV-O. |

### Collection-domain only (do not author on a file)

| Term | Label | Domain |
|------|-------|--------|
| `accrualMethod` | Accrual Method | `dctype:Collection` |
| `accrualPeriodicity` | Accrual Periodicity | `dctype:Collection` |
| `accrualPolicy` | Accrual Policy | `dctype:Collection` |

---

## Classes in `/terms/` (ISO 15836-2)

Value-type hints, not extra keys: `Agent`, `AgentClass`, `BibliographicResource`, `FileFormat`, `Frequency`, `Jurisdiction`, `LicenseDocument`, `LinguisticSystem`, `Location`, `LocationPeriodOrJurisdiction`, `MediaType`, `MediaTypeOrExtent`, `MethodOfAccrual`, `MethodOfInstruction`, `PeriodOfTime`, `PhysicalMedium`, `PhysicalResource`, `Policy`, `ProvenanceStatement`, `RightsStatement`, `SizeOrDuration`, `Standard`.

---

## FileSquid application rules

These rules are **product policy** on top of DCMI/ISO. They are not DCMI requirements.

### Vocabulary boundary

1. Owner-authored **descriptive** metadata for a file MUST use `/terms/` properties (`dcterms:`).
2. Extracted **technical** metadata (EXIF, IPTC, XMP, native video packets) remains an observation in its native namespace. Correction of those observations is the existing EXIF/native editor.
3. IPTC/XMP descriptive facts (title, creator, caption, keywords, copyright) MUST be mapped onto `dcterms:` for the owner-facing descriptive record. Preserve the raw extraction separately (FE-055).
4. Free-form “custom field” namespaces MUST NOT be the consumer descriptive vocabulary. Later F-047 professional profiles remain a named, validated overlay.
5. Keywords/tags that are topics map to `dcterms:subject`. `schema:keywords` is the Schema.org closest match when the user insists on a keyword list rather than a subject.

### CRUD and progressive disclosure

6. The product MUST create, read, update, and delete descriptive elements for a file, with the same class of single/batch editing as EXIF technical metadata.
7. The descriptive editor is a compact Markdown-style key/value surface.
8. The default view lists **populated** keys only (progressive disclosure). Empty `/terms/` properties MUST NOT appear as a wall of blank fields.
9. Adding a field offers `/terms/` choices (search/filter by label and definition). After a key is chosen, the row stays visible while the user supplies a value; deleting the value removes the row.

### Schema.org overflow

10. If the user’s intended meaning matches a `/terms/` property, use that property. Do not invent a parallel Schema.org field for a 1:1 equivalent.
11. If it does not match, suggest the next-closest Schema.org property (prefer `CreativeWork` / `ImageObject` / `MediaObject` / `Thing`). The user confirms before write.
12. Store overflow as `schema:{term}`. Show the `schema:` prefix in the UI.
13. Maintain an internal 1:1 map for the rows marked **1:1** below. Bidirectional: writing `dcterms:title` exposes `schema:name` for export; writing `schema:name` when the user meant a resource title SHOULD be offered as `dcterms:title`.

### Layers that stay distinct

14. `dcterms:provenance` is a descriptive custody statement. C2PA manifests and PROV-O JSON-LD remain the authenticity/history layers.
15. `dcterms:identifier` does not replace FileId / OCFL `urn:uuid:`.
16. `dcterms:created` is descriptive date created. EXIF DateTimeOriginal / GPS remain technical observations; conflicts stay visible (unknown / conflicting states already required by F-046).

---

## dcterms ↔ Schema.org map

FileSquid stores **1:1** rows as the same descriptive fact in two CURIEs. **Close** rows are suggestion targets only. **None** means suggest a Schema.org term from search, or keep `dcterms:` only. Working notes: DCMI DC→Schema.org mapping.[^dc-schema-map]

### 1:1 equivalents (maintain in code)

| dcterms | schema.org | Notes |
|---------|------------|-------|
| `creator` | `creator` | Agreed. |
| `contributor` | `contributor` | Schema.org: secondary contributor. |
| `publisher` | `publisher` | |
| `title` | `name` | Schema.org `name` is Thing-level; this is the resource title mapping. |
| `alternative` | `alternateName` | |
| `description` | `description` | Agreed. |
| `abstract` | `abstract` | |
| `identifier` | `identifier` | Additional IDs only; not FileId. |
| `language` | `inLanguage` | BCP 47. |
| `license` | `license` | Prefer URI. |
| `subject` | `about` | Topic of the content. |
| `audience` | `audience` | |
| `educationLevel` | `educationalLevel` | |
| `spatial` | `spatialCoverage` | |
| `temporal` | `temporalCoverage` | |
| `created` | `dateCreated` | |
| `modified` | `dateModified` | |
| `issued` | `datePublished` | |
| `hasPart` | `hasPart` | |
| `isPartOf` | `isPartOf` | |
| `references` | `citation` | |
| `rightsHolder` | `copyrightHolder` | |

### Close matches (suggest; do not auto-alias)

| dcterms | Closest schema.org | Why not 1:1 |
|---------|--------------------|-------------|
| `coverage` | `contentLocation` / `spatialCoverage` / `temporalCoverage` | Split in Schema.org; DCMI prefers `spatial`/`temporal`. |
| `date` | `dateCreated` / `datePublished` / `dateModified` | Too generic; pick a date subproperty. |
| `format` | `encodingFormat` (`fileFormat` synonym) | Schema.org splits media type vs encoding object. |
| `extent` | `contentSize` or `duration` | Size and duration are separate Schema.org properties. |
| `source` | `isBasedOn` | Common linked-data practice; not DCMI-official EQ. |
| `type` | `@type` or `genre` | DCMI type vs Schema.org class vs genre. |
| `rights` | `license` / `copyrightHolder` / `conditionsOfAccess` | Several narrower Schema.org properties. |
| `accessRights` | `conditionsOfAccess` | Natural-language access conditions. |
| `dateCopyrighted` | `copyrightYear` | Schema.org is typically a year. |
| `relation` | `isRelatedTo` | Schema.org property is product-oriented. |

### No clear Schema.org equivalent

`accrualMethod`, `accrualPeriodicity`, `accrualPolicy`, `available`, `bibliographicCitation`, `conformsTo`, `dateAccepted`, `dateSubmitted`, `hasFormat`, `hasVersion`, `instructionalMethod`, `isFormatOf`, `isReferencedBy`, `isReplacedBy`, `isRequiredBy`, `isVersionOf`, `mediator`, `medium`, `provenance`, `replaces`, `requires`, `tableOfContents`, `valid`.

### Schema.org-only examples (overflow)

No `/terms/` property: `schema:certification`, `schema:hasCredential`, `schema:keywords` (as a keyword list), `schema:exifData` (technical — prefer `exif:`), `schema:contentLocation` when the user means depicted place and refuses `dcterms:spatial`.

---

## Agent rules (when implementing)

- Load this concept before adding descriptive-metadata types, sidecars, inspector UI, or search predicates.
- Keys are `/terms/` local names with prefix `dcterms`. Reject unknown `dcterms:` local names.
- `dc:` on write is a bug; map the 15 core terms to `dcterms:` .
- Do not author `accrualMethod` / `accrualPeriodicity` / `accrualPolicy` on a file.
- Do not present all 55 properties as empty form rows.
- Do not call Schema.org overflow “Dublin Core”.
- Do not call EXIF GPS `dcterms:spatial` without an owner-facing descriptive assertion; keep `exif:` as the observation.
- Product docs (`resources/prd.md`, `resources/architecture.md`) cite https://www.dublincore.org/specifications/dublin-core/dcmi-terms/ and ISO 15836-2, not this KB file.
- Related FileSquid layers: EXIF/native extraction (F-046), OCFL sidecars (F-042), C2PA (F-043), PROV-O (ADR 015).

---

## Progressive disclosure (spec vs this file)

- Full comments, encoding schemes (`W3CDTF`, `URI`, `Point`, `Box`, …), DCMI Type classes, and RDF schemas: [DCMI Metadata Terms](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/).
- ISO purchase texts: [ISO 15836-1:2017](https://www.iso.org/standard/71339.html), [ISO 15836-2:2019](https://www.iso.org/standard/71341.html).
- Schema.org types for overflow search: [CreativeWork](https://schema.org/CreativeWork), [ImageObject](https://schema.org/ImageObject), [MediaObject](https://schema.org/MediaObject).

[^dcmi-terms]: DCMI Metadata Terms, Recommendation 2020-01-20
[^iso-15836-2]: ISO 15836-2:2019 DCMI Properties and classes
[^iso-15836-1]: ISO 15836-1:2017 Core elements
[^dc-schema-map]: DCMI working notes, DC mappings to Schema.org
