#!/usr/bin/env python3
"""Check that each product PRD's Vision allocation covers every requirement row of its own vision.

Purpose:
    Each product's PRD MUST allocate every requirement row of its own vision to
    at least one PRD item: the FileSquid PRD (`resources/prd.md`) allocates the
    FileSquid vision (`resources/product-vision.md`), and the ImageSquid PRD
    (`apps/imagesquid/resources/prd.md`) allocates the ImageSquid vision
    (`apps/imagesquid/resources/product-vision.md`). This script extracts the row
    IDs (U-, FE-, C- and T- IDs in the first column of each vision table) and
    confirms that each one appears in the first column of the allocation table
    under `## Vision allocation` in that product's PRD (under a
    `### <name> vision` subsection when the PRD has one). It also confirms that
    every allocation cell links to at least one PRD heading that exists, and
    reports allocation IDs that no longer exist in the vision.

Usage:
    python3 scripts/docs/check_prd_allocation.py

Returns:
    Exit code 0 when every vision ID is allocated and every allocation target
    exists, 1 otherwise. A summary line is printed per vision, followed by any
    problems.
"""
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
VISIONS = {
    "FileSquid": (ROOT / "resources" / "product-vision.md", ROOT / "resources" / "prd.md"),
    "ImageSquid": (ROOT / "apps" / "imagesquid" / "resources" / "product-vision.md",
                   ROOT / "apps" / "imagesquid" / "resources" / "prd.md"),
}
ROW_ID = re.compile(r"^\|\s*((?:U|FE|C|T)-\d+)\s*\|")
ANY_ID = re.compile(r"\b((?:U|FE|C|T)-\d+)\b")
ANCHOR_LINK = re.compile(r"\]\(#([^)]+)\)")


def vision_ids(path: pathlib.Path) -> set:
    """Return the set of requirement-row IDs declared in a vision file.

    Args:
        path: Path to a product-vision Markdown file.

    Returns:
        IDs found in the first column of a Markdown table row.
    """
    return {m.group(1) for line in path.read_text().splitlines() if (m := ROW_ID.match(line))}


def slugify(heading: str) -> str:
    """Return the GitHub anchor slug for a Markdown heading text.

    Args:
        heading: The heading text without leading hashes.

    Returns:
        The lower-case, hyphenated anchor GitHub generates for it.
    """
    text = re.sub(r"[`*_~]", "", heading).strip().lower()
    text = re.sub(r"[^\w\- ]", "", text)
    return text.replace(" ", "-")


def prd_anchors(text: str) -> set:
    """Return all heading anchors in the PRD text.

    Args:
        text: Full PRD Markdown text.

    Returns:
        Set of anchor slugs (duplicates are not suffixed; the PRD has none).
    """
    return {slugify(m.group(1)) for m in re.finditer(r"(?m)^#{1,6}\s+(.*?)\s*$", text)}


def allocation_rows(text: str, vision: str) -> list:
    """Return (first-column IDs, allocation cell) pairs for one vision table.

    Args:
        text: Full PRD Markdown text.
        vision: "FileSquid" or "ImageSquid"; selects the `### <vision> vision`
            subsection when the PRD has one, otherwise the whole section.

    Returns:
        List of tuples (set of IDs in column 1, text of column 2).
    """
    section = text.split("## Vision allocation", 1)[1].split("\n## ", 1)[0]
    marker = f"### {vision} vision"
    block = section.split(marker, 1)[1].split("\n### ", 1)[0] if marker in section else section
    rows = []
    for line in block.splitlines():
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) < 2 or not line.startswith("|") or cells[0] in ("Vision ID", "---"):
            continue
        rows.append((set(ANY_ID.findall(cells[0])), cells[1]))
    return rows


def check_vision(text: str, vision: str, path: pathlib.Path, anchors: set) -> list:
    """Check one vision against its PRD allocation table.

    Args:
        text: Full PRD Markdown text.
        vision: Vision name used in the PRD subsection heading.
        path: Path of the vision file.
        anchors: Valid PRD heading anchors.

    Returns:
        List of problem messages (empty when the vision is fully allocated).
    """
    declared = vision_ids(path)
    rows = allocation_rows(text, vision)
    allocated = set().union(*(ids for ids, _ in rows)) if rows else set()
    problems = [f"{vision}: {i} not allocated" for i in sorted(declared - allocated)]
    problems += [f"{vision}: {i} allocated but not in vision" for i in sorted(allocated - declared)]
    for ids, cell in rows:
        targets = ANCHOR_LINK.findall(cell)
        if not targets:
            problems.append(f"{vision}: row {sorted(ids)} has no PRD link")
        problems += [f"{vision}: row {sorted(ids)} links missing #{a}" for a in targets if a not in anchors]
    print(f"{vision}: {len(declared)} vision IDs, {len(declared & allocated)} allocated, {len(rows)} allocation rows")
    return problems


def main() -> int:
    """Run the allocation check for each vision against its own PRD and print the result.

    Returns:
        Process exit code: 0 on success, 1 when any problem is found.
    """
    problems = []
    for vision, (path, prd) in VISIONS.items():
        text = prd.read_text()
        problems += check_vision(text, vision, path, prd_anchors(text))
    for p in problems:
        print(p)
    print("OK: every vision ID is allocated" if not problems else f"FAIL: {len(problems)} problem(s)")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
