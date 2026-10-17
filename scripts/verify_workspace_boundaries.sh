#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Peter Van Garderen
#
# SPDX-License-Identifier: MIT
#
# Workspace boundary validation: no language manifests at the repository root and no Rust crates inside UI-only distributions.

set -e

echo "Starting Workspace Boundary Validation..."

# Language manifests that MUST NOT sit at the repository root.
FORBIDDEN_FILES=(
  "mix.exs"
  "package.json"
  "requirements.txt"
  "Gemfile"
  "composer.json"
  "Cargo.toml"
  "Cargo.lock"
  "rust-toolchain.toml"
)

# White-label UI distributions: UI only, no Rust crates.
DISTRIBUTION_DIRS=(
  "apps/imagesquid"
)

FAILED=0

for FILE in "${FORBIDDEN_FILES[@]}"; do
  if [ -f "$FILE" ]; then
    echo "❌ ERROR: Forbidden language tooling '$FILE' found at global repository root!"
    FAILED=1
  fi
done

for DIST in "${DISTRIBUTION_DIRS[@]}"; do
  if [ -d "$DIST" ] && [ -n "$(find "$DIST" -name node_modules -prune -o -name Cargo.toml -print)" ]; then
    echo "❌ ERROR: '$DIST' is a UI-only distribution but contains a Cargo.toml. Backend code belongs in apps/filesquid/."
    FAILED=1
  fi
done

if [ "$FAILED" -ne 0 ]; then
  echo ""
  echo "🚨 VALIDATION FAILED: Workspace Boundaries rule violated."
  echo "All language and dependency manifests MUST be placed inside individual modules in the 'apps/' directory, not at the root!"
  exit 1
fi

echo "✅ Workspace Boundary validation passed."
exit 0
