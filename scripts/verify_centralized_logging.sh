#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Peter Van Garderen
#
# SPDX-License-Identifier: MIT
#
# Logging rule validation: fails when staged Rust, TypeScript or Cargo manifests introduce text log files.
#
# Detects:
# file appenders (tracing-appender, log4rs, flexi_logger, simplelog file targets),
# Tauri's log plugin (Rust crate or npm package), or code that opens *.log files. Operational events go to stdout
# as JSON; complete activity is recorded in FileSquid's activity journal (product data
# in the operational store), never in log files.
set -e

echo "Starting logging rule validation..."

STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACMR | grep -E '\.(rs|ts|js)$|Cargo\.toml$|package\.json$' || true)

if [ -z "$STAGED_FILES" ]; then
  echo "✅ Logging rule validation passed (no relevant staged files)."
  exit 0
fi

FAILED=0
PATTERN='tracing[-_]appender|RollingFileAppender|log4rs|flexi_logger|simplelog|tauri[-_]plugin[-_]log|@tauri-apps/plugin-log|\.log"|\.log'"'"
# Check the staged blob (what will be committed), not the working-tree file.
for FILE in $STAGED_FILES; do
  if git show ":$FILE" | grep -nE "$PATTERN" >/dev/null; then
    echo "❌ ERROR: file-logging destination detected in staged $FILE:"
    git show ":$FILE" | grep -nE "$PATTERN" | head -5
    FAILED=1
  fi
done

if [ "$FAILED" -ne 0 ]; then
  echo ""
  echo "🚨 VALIDATION FAILED: applications MUST NOT write log files."
  echo "Write JSON events to stdout and record activity in the FileSquid activity journal (ADR 033)."
  exit 1
fi

echo "✅ Logging rule validation passed."
exit 0
