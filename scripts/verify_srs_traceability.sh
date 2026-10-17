#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Peter Van Garderen
#
# SPDX-License-Identifier: MIT
#
# PRD/SRS traceability validation.
#
# Checks three things, in order:
#   1. Every PRD requirement ID is a heading of the form "## F-001:", "## NFR-001:", "## TC-001:" or "## RC-001:".
#   2. Every "[PRD <ID>]" reference in the product SRS and the module SRS files resolves to one of those headings.
#   3. Every PRD functional requirement (F-), non-functional requirement (NFR-) and technical constraint (TC-)
#      has a matching "### SRS-<ID>:" heading in the product SRS.
# Exit code 0 when all three hold, 1 otherwise. The script never edits a file.

set -e

PRD_FILE="resources/prd.md"
SRS_FILE="resources/srs.md"

echo "Starting PRD/SRS Traceability Validation..."

if [ ! -f "$PRD_FILE" ] || [ ! -f "$SRS_FILE" ]; then
  echo "🚨 VALIDATION FAILED: $PRD_FILE or $SRS_FILE not found"
  exit 1
fi

SRS_FILES="$SRS_FILE $(find apps -name 'srs.md' 2>/dev/null | tr '\n' ' ')"

# 1. PRD requirement IDs, one per line.
PRD_IDS=$(grep -oE '^## (F|NFR|TC|RC)-[0-9]+:' "$PRD_FILE" | sed -E 's/^## //; s/:$//' | sort -u)

# 2. Every [PRD <ID>] reference must name an existing PRD heading. Any ID-shaped
#    reference is captured, so a mistyped prefix fails instead of being skipped.
SRS_REFS=$(grep -hoE '\[PRD [A-Z][A-Z0-9]*-[0-9]+\]' $SRS_FILES | sed -E 's/\[PRD //; s/\]//' | sort -u)
DEAD_LINKS=0
for REF in $SRS_REFS; do
  if ! grep -qxF "$REF" <<< "$PRD_IDS"; then
    echo "❌ DEAD LINK: an SRS file references PRD ID '$REF', which is not a heading in $PRD_FILE"
    DEAD_LINKS=1
  fi
done

# 3. Every PRD F-, NFR- and TC- requirement must have a product-SRS heading.
SRS_HEADINGS=$(grep -oE '^### SRS-(F|NFR|TC)-[0-9]+:' "$SRS_FILE" | sed -E 's/^### SRS-//; s/:$//' | sort -u)
ORPHANS=0
for ID in $PRD_IDS; do
  case "$ID" in RC-*) continue ;; esac
  if ! grep -qxF "$ID" <<< "$SRS_HEADINGS"; then
    echo "❌ ORPHAN REQUIREMENT: PRD '$ID' has no '### SRS-$ID:' heading in $SRS_FILE"
    ORPHANS=1
  fi
done

if [ "$DEAD_LINKS" -eq 1 ] || [ "$ORPHANS" -eq 1 ]; then
  echo ""
  echo "🚨 VALIDATION FAILED: every SRS reference must point to a PRD heading, and every PRD F-, NFR- and TC- requirement must have a product-SRS heading."
  exit 1
fi

echo "✅ Traceability Validation passed: $(echo "$PRD_IDS" | wc -l | tr -d ' ') PRD requirements, $(echo "$SRS_REFS" | wc -l | tr -d ' ') resolved SRS references."
exit 0
