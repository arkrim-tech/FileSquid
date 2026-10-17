#!/usr/bin/env python3
"""Relative-link checker for the repository's Markdown documentation.

Purpose:
    Verify that every relative Markdown link (`[text](path#anchor)`) in tracked
    Markdown files points at a file that exists and, for `.md` targets with an
    anchor, at a heading that exists (GitHub slug rules).

Usage:
    python3 scripts/docs/check_links.py [--include-history]

    By default, historical evidence (task walkthroughs and
    `resources/tasks/`) is checked for missing
    files only, not for anchors, because their headings refer to past document
    states.

Returns:
    Exit code 0 when no broken links are found, 1 otherwise. Each broken link is
    printed as `file:line: target (reason)`.
"""
import functools
import pathlib
import re
import subprocess
import sys
import urllib.parse

ROOT = pathlib.Path(__file__).resolve().parents[2]
LINK = re.compile(r"(?<!!)\[[^\]]*\]\(([^)\s]+)(?:\s+\"[^\"]*\")?\)|!\[[^\]]*\]\(([^)\s]+)\)")
HISTORY = ("/tasks/",)
INLINE_CODE = re.compile(r"(?<!\\)(`+)(?:(?!\1)[^\n])+?\1(?!`)")  # CommonMark code span: unescaped, equal-length backtick runs


def tracked_markdown() -> list:
    """Return repository-relative paths of tracked and untracked (not ignored) Markdown files."""
    out = subprocess.run(["git", "ls-files", "--cached", "--others", "--exclude-standard", "*.md"],
                         cwd=ROOT, capture_output=True, text=True, check=True).stdout
    return [p for p in out.splitlines() if (ROOT / p).exists()]


def slugify(heading: str) -> str:
    """Return the GitHub anchor slug for a Markdown heading text."""
    text = re.sub(r"<[^>]+>", "", heading).strip().lower()
    text = re.sub(r"[`*_~]|\[([^\]]*)\]\([^)]*\)", lambda m: m.group(1) or "", text)
    text = re.sub(r"[^\w\- ]", "", text)
    return text.replace(" ", "-")


@functools.lru_cache(maxsize=None)
def anchors(path: pathlib.Path) -> frozenset:
    """Return all heading anchors in a Markdown file, with GitHub duplicate suffixes."""
    seen, result, in_code = {}, set(), False
    for line in path.read_text(errors="replace").splitlines():
        if line.startswith("```"):
            in_code = not in_code
        if in_code:
            continue
        m = re.match(r"^#{1,6}\s+(.*?)\s*#*\s*$", line)
        if not m:
            continue
        slug = slugify(m.group(1))
        n = seen.get(slug, 0)
        result.add(slug if n == 0 else f"{slug}-{n}")
        seen[slug] = n + 1
    for m in re.finditer(r'<a\s+(?:name|id)="([^"]+)"', path.read_text(errors="replace")):
        result.add(m.group(1))
    return frozenset(result)


def check_file(rel: str, history: bool) -> list:
    """Return broken-link messages for one Markdown file."""
    src = ROOT / rel
    errors, in_code = [], False
    for n, line in enumerate(src.read_text(errors="replace").splitlines(), 1):
        if line.lstrip().startswith("```"):
            in_code = not in_code
        if in_code:
            continue
        # Link syntax inside inline code (`[PRD](../prd.md)`) is an example, not a link.
        for m in LINK.finditer(INLINE_CODE.sub("", line)):
            target = m.group(1) or m.group(2)
            if re.match(r"^[a-z][a-z0-9+.-]*:", target) or target.startswith("<"):
                continue
            path_part, _, anchor = target.partition("#")
            path_part = urllib.parse.unquote(path_part)
            dest = src if not path_part else (src.parent / path_part).resolve()
            if not dest.exists():
                errors.append(f"{rel}:{n}: {target} (missing file)")
                continue
            if anchor and not history and dest.suffix == ".md" and anchor not in anchors(dest):
                errors.append(f"{rel}:{n}: {target} (missing anchor)")
    return errors


def main() -> int:
    """Check every Markdown file and print a summary; return the exit code."""
    include_history = "--include-history" in sys.argv
    errors = []
    files = tracked_markdown()
    for rel in files:
        is_history = any(h in rel for h in HISTORY) and not include_history
        errors.extend(check_file(rel, is_history))
    for e in errors:
        print(e)
    print(f"checked {len(files)} Markdown files: {len(errors)} broken link(s)")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
