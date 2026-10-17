#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Peter Van Garderen
#
# SPDX-License-Identifier: MIT
#
# Port isolation validation: checks that every local development port declared by an app under apps/ is unique.
#
# Checks that every local development port declared by an app under apps/ is
# unique (workspace-boundaries rule 4). It reads:
#   - Elixir apps: config/dev.exs and config/runtime.exs (dev and runtime ports must match)
#   - Vite UI packages: `port:` values in vite.config.{ts,js,mjs} (dev server and preview)
#   - filesquid-server: the default `127.0.0.1:<port>` bind address in its Rust source
# Missing config files are skipped, so the script also runs on a partly built tree.

set -e

echo "Starting Port Isolation Validation..."

APPS_DIR="./apps"
SERVER_SRC_DIR="$APPS_DIR/filesquid/crates/server/src"
ALL_PORTS_STRING=" "
FAILED=0

# Record one port for one owner and fail if another owner already holds it.
# Arguments: $1 port number, $2 owner label (app or file name).
claim_port() {
  local PORT="$1"
  local OWNER="$2"
  if [[ "$ALL_PORTS_STRING" == *" $PORT:"* ]]; then
    echo "❌ ERROR ($OWNER): Port $PORT is already in use by another app!"
    FAILED=1
    return 0
  fi
  ALL_PORTS_STRING="$ALL_PORTS_STRING$PORT:$OWNER "
  echo "✅ $OWNER uniquely bound to $PORT"
}

# Validate one Elixir app: dev.exs and runtime.exs must declare the same port.
# Arguments: $1 app directory.
check_elixir_app() {
  local APP_PATH="$1"
  local APP_NAME
  APP_NAME=$(basename "$APP_PATH")
  local DEV_EXS="$APP_PATH/config/dev.exs"
  local RUNTIME_EXS="$APP_PATH/config/runtime.exs"
  if [ ! -f "$DEV_EXS" ] || [ ! -f "$RUNTIME_EXS" ]; then
    return 0
  fi
  echo "🔍 Validating $APP_NAME (Elixir)..."
  local DEV_PORT RUNTIME_PORT
  DEV_PORT=$(grep -oE 'port:\s*[0-9]+' "$DEV_EXS" | grep -oE '[0-9]+' | head -n 1 || true)
  RUNTIME_PORT=$(grep -oE 'System\.get_env\("PORT",\s*"[0-9]+"\)' "$RUNTIME_EXS" | grep -oE '[0-9]+' | head -n 1 || true)
  if [ -z "$DEV_PORT" ] || [ -z "$RUNTIME_PORT" ]; then
    echo "❌ ERROR ($APP_NAME): Could not find the port in dev.exs or runtime.exs"
    FAILED=1
    return 0
  fi
  if [ "$DEV_PORT" != "$RUNTIME_PORT" ]; then
    echo "❌ ERROR ($APP_NAME): Port mismatch! dev.exs ($DEV_PORT) != runtime.exs ($RUNTIME_PORT)"
    FAILED=1
  fi
  claim_port "$DEV_PORT" "$APP_NAME"
}

# Validate one Vite config: every `port: N` it declares must be unique.
# Arguments: $1 path to the vite config file.
check_vite_config() {
  local CONFIG="$1"
  local OWNER
  OWNER=$(dirname "$CONFIG")
  echo "🔍 Validating $OWNER (Vite)..."
  local PORTS
  PORTS=$(grep -oE 'port:[[:space:]]*[0-9]+' "$CONFIG" | grep -oE '[0-9]+' || true)
  if [ -z "$PORTS" ]; then
    echo "❌ ERROR ($OWNER): No hardcoded 'port:' found in $CONFIG"
    FAILED=1
    return 0
  fi
  local PORT
  for PORT in $PORTS; do
    claim_port "$PORT" "$OWNER"
  done
}

# Validate the filesquid-server default bind port, if the crate exists.
check_server_bind() {
  if [ ! -d "$SERVER_SRC_DIR" ]; then
    return 0
  fi
  echo "🔍 Validating filesquid-server (default bind)..."
  local PORT
  PORT=$(grep -rhoE '127\.0\.0\.1:[0-9]+' "$SERVER_SRC_DIR" | grep -oE '[0-9]+$' | sort -u | head -n 1 || true)
  if [ -z "$PORT" ]; then
    echo "⚠️  [SKIPPING] filesquid-server: no default 127.0.0.1:<port> bind found"
    return 0
  fi
  claim_port "$PORT" "filesquid-server"
}

for APP_PATH in "$APPS_DIR"/*; do
  if [ -d "$APP_PATH" ]; then
    check_elixir_app "$APP_PATH"
  fi
done

VITE_CONFIGS=$(find "$APPS_DIR" -maxdepth 3 -name node_modules -prune -o -type f \( -name 'vite.config.ts' -o -name 'vite.config.js' -o -name 'vite.config.mjs' \) -print | sort)
for CONFIG in $VITE_CONFIGS; do
  check_vite_config "$CONFIG"
done

check_server_bind

if [ "$FAILED" -ne 0 ]; then
  echo ""
  echo "🚨 VALIDATION FAILED: Port isolation constraints violated. See errors above."
  exit 1
fi

echo ""
echo "🚀 VALIDATION PASSED: All modules maintain strict port isolation."
exit 0
