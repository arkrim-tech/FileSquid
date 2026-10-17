---
description: Ensure that shared infrastructure like Docker Compose remains centralized at the repository root for all workspaces to utilize.
copyright: Pieter Van Garderen (2026)
license: CC-BY
---

# Centralized Infrastructure

Whether the Open Agent Dev repository is structured as a Polyglot Monorepo (Python/JS, PHP/JS, Rust/JS) or an Umbrella Project (BEAM/Elixir), application code is rigorously isolated within the `/apps` directory. However, underlying shared infrastructure MUST be centralized.

## Docker Compose

You MUST place the primary `docker-compose.yml` file at the repository root, **not** inside individual workspace directories (e.g., do not put it in `apps/web/`).

**CRITICAL DISTINCTION FOR BEAM/ELIXIR PROJECTS:** 
If the project is a BEAM/Elixir Umbrella app, this centralized `docker-compose.yml` is *only* to be used for spinning up **backing services** (PostgreSQL, Redis, RabbitMQ, etc.) for local development. 
- You MUST NOT use this file to Dockerize the actual Elixir application code itself. BEAM/Elixir applications are deployed using native OTP releases on the BEAM VM. They should run natively during local development (via `mix phx.server`) while connecting to the Dockerized backing services.
- If the project is a Polyglot Monorepo (e.g., Python/JS), it is acceptable and expected to provision the application code itself as a container within the root `docker-compose.yml` for distribution.

**CRITICAL DISTINCTION FOR PHP/JS (WORDPRESS PLUGIN) PROJECTS:**
If the project is a PHP/JS WordPress plugin monorepo, this centralized `docker-compose.yml` runs the **local** WordPress runtime (WordPress + MySQL/MariaDB) and the observability stack.
- Plugins under `apps/` are bind-mounted into `wp-content/plugins/`. They are not the Compose “app image” in production.
- You MUST NOT treat Compose as the production artifact. Production is a zip installed on a merchant WordPress host.
- The observability stack is for developers. It MUST NOT be bundled into plugin zips.

**CRITICAL DISTINCTION FOR RUST/JS (AXUM + TAURI) PROJECTS:**
If the project is a Rust/JS monorepo, this centralized `docker-compose.yml` runs backing services (e.g., PostgreSQL) and, under the opt-in `observability` profile, the optional VictoriaLogs and VictoriaMetrics sinks. It MAY also containerize the Axum server host (`filesquid-server`, which also serves the selected UI's static files).
- You MUST NOT Dockerize the Tauri desktop host (`apps/filesquid/crates/desktop`). Tauri uses OS webviews and MUST run natively during local development (`npx tauri dev`) and as a packaged desktop binary for release.
- Desktop process stdout is attached in the developer terminal. FileSquid's activity journal and Monitor dashboard provide logging and health on every host that runs `filesquid-api` ([ADR 033](../../resources/architecture/033-activity-journal-and-live-health-monitoring.md)); the desktop host never runs a telemetry server.

### Rationale

1. **Zero-Friction Setup:** A developer should be able to clone the repository and run `docker compose up -d` exactly once at the root to instantly provision all databases, message queues, and caches required for the entire project.
2. **Shared Services across Workspaces:** If the repository contains a backend API in `apps/backend/` and a worker service in `apps/worker/`, they will often need to talk to the exact same Postgres database or Redis cache. Even within a pure Elixir Umbrella project containing `apps/my_app` and `apps/my_app_web`, both OTP apps connect to the same backing database. PHP plugins share one local WordPress/MySQL instance. In `apps/filesquid/`, `filesquid-server` uses the Compose-backed stores; `filesquid-desktop` keeps its own owner-local SQLite/OCFL store and is not a Compose consumer. Add Compose services only for hosts that need them. Centralizing the `docker-compose.yml` prevents duplicate databases from being spun up.
3. **Language Agnosticism:** While `package.json`, `mix.exs`, `composer.json`, or `Cargo.toml` are specific to a single tech stack and belong inside their workspaces, a Docker container running PostgreSQL (or local WordPress) is entirely language-agnostic and serves as horizontal infrastructure for the whole repo.

## Implementation

When provisioning new databases or infrastructure dependencies during a task:
1. Append the new service to the root `docker-compose.yml`.
2. Ensure the individual application workspaces (e.g., `apps/web/`) connect to these localized Docker services via `.env` variables.
