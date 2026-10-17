# Architecture Decision Record - FileSquid

022-dublin-core-descriptive-metadata.md

## Title

ISO 15836-2 Dublin Core `/terms/` for owner-authored descriptive metadata

## Status

Accepted

Accepted: 2026-09-25T18:44:20Z. Revised: 2026-10-15T02:45:00Z. The Sidecar JSON row follows [ADR 024](./024-filesquid-metadata-engine.md): JSON sidecars are interchange bound to the OCFL object identifier and SHA-256 digest, and `metadata.md` is canonical. Revised: 2026-10-15T03:34:43Z. Amended by [ADR 025](./025-client-side-filesquid-engines.md): the browser uses the same `filesquid-metadata` engine as the desktop. Revised: 2026-10-17T04:10:11Z. Owner-authored descriptive keys MAY also use a registered extension namespace (for example RiC-O, or an owner-declared custom namespace for free key:value pairs); `dcterms:` stays closed and undeclared prefixes stay rejected ([C-043](../product-vision.md#provenance-metadata-and-disclosure) amended).

## Context / Requirement Reference

FileSquid already extracts EXIF, IPTC, XMP and native video packets and lets owners correct chronology, tags and location ([F-046](../prd.md#f-046-media-metadata-inspection-correction-and-privacy-exports), [FE-054](../product-vision.md#metadata-ownership-and-sharing-privacy)). Earlier product text treated owner-authored descriptive facts as an open “namespaced custom field” vocabulary. That left search, sidecars, XMP export and later F-047 professional profiles without a closed, interoperable key set.

[U-55](../product-vision.md#metadata-module) and [FE-068](../product-vision.md#metadata-ownership-and-sharing-privacy) require owner-authored descriptive metadata to use ISO 15836-2 Dublin Core `/terms/` properties, Schema.org only when `/terms/` has no match, namespace prefixes on every displayed pair, and progressive disclosure of populated keys. [C-043](../product-vision.md#provenance-metadata-and-disclosure) forbids descriptive keys in an undeclared namespace and admits registered extension namespaces, including an owner-declared custom namespace, so that archivists can add schemas such as RiC-O ([Product Vision](../product-vision.md#product-vision)). [T-038](../product-vision.md#descriptive-metadata) requires `filesquid-metadata` to ship the `/terms/` register and the 1:1 `dcterms:`↔Schema.org map ([ADR 024](./024-filesquid-metadata-engine.md)).

Technical camera/file facts remain EXIF/native observations. C2PA and PROV-O remain authenticity/history layers, distinct from `dcterms:provenance`.

## Decision

Owner-authored **descriptive** metadata for a FileSquid **file** MUST use properties from the DCMI `/terms/` namespace, confirmed Schema.org overflow, or a registered extension namespace:

| Rule | Requirement |
|---|---|
| Authoring namespace | `http://purl.org/dc/terms/` (prefix `dcterms:`). ISO 15836-2:2019. Canonical term list: [DCMI Metadata Terms](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/). |
| Legacy read alias | `http://purl.org/dc/elements/1.1/` (prefix `dc:`, ISO 15836-1) MAY appear on import. Writes MUST map the fifteen core terms onto the matching `dcterms:` property. |
| Technical metadata | EXIF and native IPTC/XMP/video packets remain observations with prefix `exif:` (and native tag identifiers). They are not `/terms/` keys. |
| Schema.org overflow | When the owner’s intended meaning has no `/terms/` property, the product MUST suggest the closest Schema.org term (`https://schema.org/`, prefix `schema:`) and store it only after confirmation. |
| 1:1 map | `filesquid-metadata` MUST maintain the 1:1 `dcterms:`↔Schema.org equivalents listed in the agent synthesis [iso-15836-dublin-core.md](../../.agents/kb/iso-15836-dublin-core.md). Close matches MUST NOT auto-alias. |
| Closed register | Unknown `dcterms:` local names MUST be rejected. Collection-domain `accrualMethod`, `accrualPeriodicity` and `accrualPolicy` MUST NOT be authored on a file. |
| Extension namespaces | `filesquid-metadata` MUST keep a namespace registry. An owner, distribution or module MAY register an extension namespace (prefix, IRI, optional term list; for example RiC-O), and an owner MAY declare a custom namespace whose keys are free key:value pairs. A registration MUST NOT redefine `dcterms:`, `dc:`, `schema:` or `exif:`. An extension prefix MUST be unique within a repository and map to exactly one IRI; re-registering an existing prefix with a different IRI MUST be rejected. A namespace with a term list MUST reject unknown terms; a statement whose prefix is not registered MUST be rejected. Extension statements live in their own YAML map in `metadata.md`, round-trip through CSV, JSON and XMP, are covered by the outgoing disclosure policy exactly like `exif:`, `dcterms:` and `schema:` statements (retained-field selection, scrubbing and disclosure preview), and are never mapped onto `dcterms:`. Registrations persist in canonical OCFL state. |
| Display | Every populated pair MUST show its prefix: `exif:GPSLatitude "49.3"`, `dcterms:creator "George"`, `schema:certification "WERT level1"`. |
| Progressive disclosure | The default editor MUST list populated keys only. Empty `/terms/` properties MUST NOT appear as a wall of blank fields. |
| CRUD | Create, read, update and delete of descriptive statements MUST be available as a first-class inspector flow, matching the EXIF technical editor (single and batch). |
| IPTC/XMP mapping | Extracted IPTC/XMP title, creator, caption, keywords and copyright MUST map onto `dcterms:title`, `dcterms:creator`, `dcterms:description`, `dcterms:subject` and `dcterms:rights` for the owner-facing record. Raw extraction remains a separate FE-055 observation. |
| Sidecar JSON | A JSON sidecar is interchange, not a canonical store ([ADR 024](./024-filesquid-metadata-engine.md)). It MUST bind `{namespace, term, value}` statements to the OCFL object identifier and the SHA-256 digest of the described logical file. Canonical statements live in `metadata.md` in the OCFL object. Metadata-only changes to `metadata.md` remain F-042 versions. |
| FileId | Logical identity remains UUID v4 ([ADR 021](./021-uuid-v4-identifiers.md)). `dcterms:identifier` MAY hold additional identifiers. It MUST NOT replace FileId. |
| Layers | `dcterms:provenance` is a descriptive custody statement. C2PA manifests and PROV-O JSON-LD remain the authenticity/history layers ([ADR 015](./015-prov-o-jsonld-and-c2pa-sdk.md)). |
| Later profiles | F-047 professional metadata profiles remain a named, validated overlay. Arbitrary JSON is not a professional profile. A registered extension namespace (for example RiC-O) stores a profile's terms; it is not profile conformance on its own. |

**Engine.** Identifier-bound statement types and the `/terms/` register live in `apps/filesquid/crates/metadata` (`filesquid_metadata`). Persistence uses `filesquid-ocfl`. `filesquid-metadata` extracts embedded packets and maps IPTC/XMP descriptive facts onto `dcterms:`. Hosts search and render; they MUST NOT invent descriptive namespaces outside the `filesquid-metadata` registry or a parallel extract path ([ADR 024](./024-filesquid-metadata-engine.md)).

## Rationale

Dublin Core `/terms/` is the ISO vocabulary for cross-domain resource description and is already the interchange baseline for GLAM and OCFL-adjacent collections. Closing the consumer key set to `/terms/` plus confirmed Schema.org overflow gives search, Smart Albums, XMP export and later F-047 mappings one register instead of an unbounded custom-field table. Prefix display keeps technical EXIF, descriptive Dublin Core and Schema.org overflow distinguishable on one inspector surface. A registry rather than open fields lets owners and archivists add schemas (RiC-O) or their own keys while every statement still carries a declared prefix that search, interchange and privacy policies can address.

## Assumptions

- Owners can describe photographs and videos with `/terms/` properties plus a small Schema.org overflow set (CreativeWork / ImageObject / MediaObject / Thing).
- A 1:1 map is maintainable for the agreed equivalents; remaining `/terms/` properties stay `dcterms:`-only until a later mapping revision.
- `filesquid-metadata` extractors run on the desktop and, compiled to WASM, in the browser; both commit `metadata.md` through `filesquid-ocfl` without rewriting original bytes ([ADR 025](./025-client-side-filesquid-engines.md)).
- DCMI Recommendation 2020-01-20 remains the living term list; ISO 15836-2:2019 is the ISO publication of `/terms/` properties and classes.

## Alternatives Considered

- **Undeclared namespaced custom fields:** Rejected by [C-043](../product-vision.md#provenance-metadata-and-disclosure). Search, export and F-047 cannot validate keys with no declared namespace. Owner-declared custom namespaces are accepted instead because each is registered, prefixed and round-trips through interchange.
- **ISO 15836-1 `/elements/1.1/` only:** Rejected. The product needs date, rights and relation refinements (`created`, `license`, `spatial`, …) that live in `/terms/`.
- **Schema.org as the primary vocabulary:** Rejected. Schema.org is the overflow when Dublin Core has no match; the owner requirement is ISO 15836 `/terms/`.
- **IPTC Photo Metadata / XMP as the owner-facing key set:** Rejected for authoring. Those packets remain extraction/export encodings mapped onto `dcterms:`.
- **Blank form of all 55 `/terms/` properties:** Rejected. Progressive disclosure lists populated keys only.

## Consequences / Implications

- `filesquid-metadata` MUST add a closed `DctermsProperty` register, `MetadataNamespace`, `DescriptiveStatement`, and the 1:1 Schema.org map, with tests that reject unknown `dcterms:` names and file-scoped `accrual*` terms. Persistence MUST call `filesquid-ocfl`.
- `filesquid-metadata` MUST add the namespace registry (MD-S-012) with tests for registration, refused redefinitions, term-list checks, unregistered prefixes and interchange round-trips.
- F-005 title/caption/keyword UI writes `dcterms:title` / `dcterms:description` / `dcterms:subject`.
- F-006 / RC-013 predicates use namespaced keys.
- D-21 metadata adapters own extract, map IPTC/XMP → `dcterms:`, apply corrections, suggest Schema.org overflow, and verify privacy exports.
- The ImageSquid inspector UI (ImageSquid SRS-UI-002) shows one Properties card of populated rows, grouped `dcterms:` first, then `exif:`, then `schema:`, with an add/suggest flow.
- Product docs cite the DCMI URL and ISO 15836-2. The agent KB is not a PRD/architecture source citation.

## Related Decisions / Notes

- [Architecture 4.1](../architecture.md#41-data-architecture), metadata adapters in [3.2](../architecture.md#32-frontend-architecture)
- [F-005](../prd.md#f-005-favorites-metadata-and-keywords), [F-006](../prd.md#f-006-virtual-collections-and-metadata-search), [F-046](../prd.md#f-046-media-metadata-inspection-correction-and-privacy-exports), [F-047](../prd.md#f-047-institutional-dam-and-iiif-interoperability), [RC-013](../prd.md#rc-013-metadata-fidelity-and-configurable-sharing-privacy)
- [U-66](../product-vision.md#metadata-module), [U-55](../product-vision.md#metadata-module), [FE-068](../product-vision.md#metadata-ownership-and-sharing-privacy), [C-043](../product-vision.md#provenance-metadata-and-disclosure), [T-038](../product-vision.md#descriptive-metadata)
- [ADR 015](./015-prov-o-jsonld-and-c2pa-sdk.md), [ADR 021](./021-uuid-v4-identifiers.md)
- [DCMI Metadata Terms](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/), [ISO 15836-2:2019](https://www.iso.org/standard/71341.html), [ISO 15836 KB](../../.agents/kb/iso-15836-dublin-core.md)
