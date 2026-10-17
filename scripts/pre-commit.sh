#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Peter Van Garderen
#
# SPDX-License-Identifier: MIT
#
# Master pre-commit hook: runs every local verification gate in order and stops at the first failure.

echo "========================================"
echo "Running Pre-Commit Hooks..."
echo "========================================"

# 0. Validate Branch-per-Task Workflow
./scripts/verify_branch.sh
if [ $? -ne 0 ]; then
  exit 1
fi

# 1. Validate isolated ports
./scripts/verify_isolated_ports.sh
if [ $? -ne 0 ]; then
  echo "❌ pre-commit failed: Port Validation"
  exit 1
fi

# 2. Validate document sync
./scripts/verify_doc_sync.sh
if [ $? -ne 0 ]; then
  echo "❌ pre-commit failed: Document Sync Validation"
  exit 1
fi

# 3. Validate Bi-directional PRD/SRS Traceability
./scripts/verify_srs_traceability.sh
if [ $? -ne 0 ]; then
  echo "❌ pre-commit failed: Traceability Validation"
  exit 1
fi

# 4. Validate relative links
./scripts/verify_relative_links.sh
if [ $? -ne 0 ]; then

  echo "❌ pre-commit failed: Portable Documentation Validation"
  exit 1
fi

# 5. Validate workspace boundaries
./scripts/verify_workspace_boundaries.sh
if [ $? -ne 0 ]; then
  echo "❌ pre-commit failed: Workspace Boundary Validation"
  exit 1
fi

# 6. Validate centralized logging
./scripts/verify_centralized_logging.sh
if [ $? -ne 0 ]; then
  echo "❌ pre-commit failed: Centralized Logging Validation"
  exit 1
fi

# 7. Validate formatting (Rust, TypeScript, Elixir)
./scripts/verify_formatting.sh
if [ $? -ne 0 ]; then
  echo "❌ pre-commit failed: Formatting Validation"
  exit 1
fi

# 8. Validate documentation (ADR structure, PRD allocation, links, task cleanup)
./scripts/verify_docs.sh
if [ $? -ne 0 ]; then
  echo "❌ pre-commit failed: Documentation Validation"
  exit 1
fi

# 9. Lint and tests (Clippy, Cargo workspace and per-crate tests, rustdoc, UI lint/check/tests).
#    No hosted CI: this is the gate before a push. FILESQUID_SKIP_TESTS=1 skips it for WIP commits.
./scripts/verify_tests.sh
if [ $? -ne 0 ]; then
  echo "❌ pre-commit failed: Lint and Test Validation"
  exit 1
fi

echo "========================================"
echo "✅ All Pre-Commit Hooks Passed!"
echo "========================================"
exit 0
