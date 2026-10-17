#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Peter Van Garderen
#
# SPDX-License-Identifier: MIT
#
# Format validation: checks Rust, TypeScript and Elixir formatting under apps/ without changing files (pre-commit).
#
# Checks formatting for every stack present under apps/ without changing files:
#   - Rust: `cargo fmt --all --check` in each product-local Cargo workspace
#     (a directory under apps/ that holds a Cargo.toml).
#   - TypeScript: `prettier --check .` in each npm package (a directory under
#     apps/ or apps/*/ that holds a package.json). Requires `npm ci` to have
#     been run in that package; the check fails with that instruction otherwise.
#   - Elixir: `mix format --check-formatted` in each app with a mix.exs.
#
# Usage: ./scripts/verify_formatting.sh   (from the repository root)

echo "Starting Format Validation..."

APPS_DIR="./apps"
FAILED=0

check_rust_workspace() {
  # $1 = workspace directory holding Cargo.toml
  local dir="$1"
  echo "🔍 Validating Rust formatting in $dir..."
  if (cd "$dir" && cargo fmt --all --check >/dev/null 2>&1); then
    echo "✅ $dir is properly formatted."
  else
    echo "❌ ERROR ($dir): Rust format violation. Run 'cargo fmt --all' in $dir."
    FAILED=1
  fi
}

check_npm_package() {
  # $1 = package directory holding package.json
  local dir="$1"
  echo "🔍 Validating Prettier formatting in $dir..."
  if [ ! -d "$dir/node_modules" ]; then
    echo "❌ ERROR ($dir): node_modules missing. Run 'npm ci' in $dir first."
    FAILED=1
    return
  fi
  if (cd "$dir" && npx prettier --check . >/dev/null 2>&1); then
    echo "✅ $dir is properly formatted."
  else
    echo "❌ ERROR ($dir): Prettier format violation. Run 'npx prettier --write .' in $dir."
    FAILED=1
  fi
}

check_elixir_app() {
  # $1 = app directory holding mix.exs
  local dir="$1"
  echo "🔍 Validating Elixir formatting in $dir..."
  if (cd "$dir" && mix format --check-formatted >/dev/null 2>&1); then
    echo "✅ $dir is properly formatted."
  else
    echo "❌ ERROR ($dir): Elixir format violation. Run 'mix format' in $dir."
    FAILED=1
  fi
}

for APP_PATH in "$APPS_DIR"/*; do
  [ -d "$APP_PATH" ] || continue
  [ -f "$APP_PATH/Cargo.toml" ] && check_rust_workspace "$APP_PATH"
  [ -f "$APP_PATH/package.json" ] && check_npm_package "$APP_PATH"
  [ -f "$APP_PATH/mix.exs" ] && check_elixir_app "$APP_PATH"
  for SUB_PATH in "$APP_PATH"/*; do
    [ -d "$SUB_PATH" ] && [ -f "$SUB_PATH/package.json" ] && check_npm_package "$SUB_PATH"
  done
done

if [ "$FAILED" -ne 0 ]; then
  echo ""
  echo "🚨 VALIDATION FAILED: Test-Driven Development formatting rule violated."
  echo "Format each listed workspace before committing."
  exit 1
fi

echo "✅ Format validation passed."
exit 0
