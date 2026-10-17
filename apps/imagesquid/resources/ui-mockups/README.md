# UI mockups

Design artifacts used when generating the [ImageSquid SRS](../srs.md) for the surface they show: wireframes, mockups, flows, and look-and-feel examples. FileSquid's own surfaces and mockups are recorded in FileSquid's SRS, not here.

These files are hard look-and-feel requirements for the surfaces they show. Outcome-level product needs stay in the [ImageSquid PRD](../prd.md). The SRS writer records what each mockup shows as ISO/IEC/IEEE 29148 requirements that are unambiguous, verifiable, consistent, and traced. A pixel or two of browser rendering difference is acceptable.

**Active suite:** the two light-mode screenshots in this folder:

| File | Shows |
|---|---|
| [`screen1-light-mode.png`](./screen1-light-mode.png) | Library grid with the C2PA floatover |
| [`screen2-light-mode.png`](./screen2-light-mode.png) | Viewer with the Metadata / C2PA / Archive inspector and the `metadata.md` front matter |

**The screenshots are canonical**: where a screenshot and PRD, vision or SRS wording disagree about a surface it shows, the screenshot governs and the written requirements are aligned to it. Only the active suite is a requirement source. The earlier suites (`imagesquid-ui-mockups-v1` to `-v4` and the Stitch unpacks) are no longer included.

## What goes here

- Wireframes and mid- or high-fidelity mockups (PNG, SVG, PDF, or HTML)
- Interaction flows and information-architecture diagrams
- Look-and-feel examples: visual hierarchy, layout regions, density, typography, color roles, empty/error/loading/offline states
- Optional notes beside a mockup that name the intended surface, states, and PRD feature IDs

Do **not** put mockups in `prd.md` or `docs/`.

## How SRS generation uses these files

When generating or updating `srs.md`, the agent MUST:

1. Read every mockup and accompanying note in the active suite. Do not read archived suites as requirements.
2. Convert **UI features** into testable requirements: required surfaces, information shown, primary actions, navigation, and states (empty, loading, error, offline, permission-denied).
3. Record the composition a person sees: regions and what sits in each, what is visible without scrolling, grouping, hierarchy, density, typography scale, colour roles, control shape, and which status text is shown or withheld. Use RFC 2119 (`MUST` / `SHOULD` / `MAY`). Region presence, a minimum font size, and a contrast ratio do not by themselves satisfy the mockup.
4. Cite each source mockup in the requirement `Source` block, with the related PRD `F-` / `NFR-` / `TC-` ID when one exists.
5. Map look-and-feel and presentation requirements to UI tests; map behavioral requirements to BDD scenarios.

The SRS text records the mockup. A pixel or two of difference from browser rounding, font rasterization, and scrollbars is acceptable. Do **not** require a screenshot hash or a one-pixel coordinate match. Do **not** treat that tolerance as permission to omit the mockup's layout, default view, grouping, or designed controls. Do **not** write that the running page need not match the mockup. Do **not** call the mockup an illustration. Do **not** use subjective words such as “clean” or “modern” as the requirement; describe what the mockup shows.

If a mockup contradicts the PRD, halt and report the conflict. If a mockup shows a surface or state with no PRD ID, still specify it, cite the mockup, and record the PRD gap in the SRS traceability matrix.
