#!/usr/bin/env python3
"""Check that FileSquid and ImageSquid documents do not link to each other.

Purpose:
    ImageSquid (`apps/imagesquid/`) MUST stay extractable to its own repository
    (ADR 029), so its documents and FileSquid's documents MUST stand on their own.
    A relative Markdown or HTML link from a file inside `apps/imagesquid/` to a
    file outside it, or the reverse, breaks when ImageSquid moves. Each side names
    the other's requirement IDs and documents in plain text instead.

    Task walkthroughs under `resources/tasks/` are historical evidence and
    are not checked. Links inside inline code spans are examples, not links.

Usage:
    python3 scripts/docs/check_distribution_links.py

Returns:
    Exit code 0 when no link crosses the boundary, 1 otherwise. Each crossing
    link is printed as `file:line: target`.
"""
import os
import re
import subprocess
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
DISTRIBUTION = "apps/imagesquid"
HISTORY = ("/tasks/",)
LINK = re.compile(r"\]\(([^)\s]+)\)|(?:href|src)=\"([^\"]+)\"")
INLINE_CODE = re.compile(r"(?<!\\)(`+)(?:(?!\1)[^\n])+?\1(?!`)")  # CommonMark code span: unescaped, equal-length backtick runs


def documents() -> list:
    """Return repository-relative paths of tracked and untracked Markdown and HTML files."""
    out = subprocess.run(["git", "ls-files", "--cached", "--others", "--exclude-standard", "*.md", "*.html"],
                         cwd=ROOT, capture_output=True, text=True, check=True).stdout
    return [p for p in out.splitlines()
            if os.path.exists(os.path.join(ROOT, p)) and not any(h in p for h in HISTORY)]


def inside(path: str) -> bool:
    """Return True when a repository-relative path is inside the distribution."""
    return path == DISTRIBUTION or path.startswith(DISTRIBUTION + "/")


def crossing_links(rel: str) -> list:
    """Return `file:line: target` messages for links in `rel` that cross the boundary."""
    found = []
    text = open(os.path.join(ROOT, rel), errors="replace").read()
    for n, line in enumerate(text.splitlines(), 1):
        for m in LINK.finditer(INLINE_CODE.sub("", line)):
            target = m.group(1) or m.group(2)
            path = target.split("#", 1)[0]
            if not path or re.match(r"^[a-z][a-z0-9+.-]*:", target) or path.startswith("/"):
                continue
            resolved = os.path.normpath(os.path.join(os.path.dirname(rel), path))
            if inside(resolved) != inside(rel):
                found.append(f"{rel}:{n}: {target}")
    return found


def main() -> int:
    """Check every document and print a summary; return the process exit code."""
    files = documents()
    problems = [msg for rel in files for msg in crossing_links(rel)]
    for msg in problems:
        print(msg)
    print(f"checked {len(files)} documents: {len(problems)} link(s) cross the FileSquid/ImageSquid boundary")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
