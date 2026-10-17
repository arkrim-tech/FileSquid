---
trigger: always_on
description: Ensure the repository explicitly defines workspace boundaries (apps/) to maintain a language-agnostic root and proper separation of concerns.
copyright: Pieter Van Garderen (2026)
license: CC-BY
---

# Root Workspace Project Boundaries

The Open Agent Dev repository enforces strict organizational boundaries based on the initialization sequence chosen.

## 1. Python/Javascript Stack (Monorepo)
If initialized with the Python/JS stack, the application is structured as a polyglot Monorepo containing `/apps` exclusively.
- **`/apps`**: Houses all independent, deployable applications, regardless of whether they are client-side or server-side (e.g., the JavaScript frontend in `apps/frontend/` and the Python API in `apps/backend/`).

## 2. BEAM/Elixir Stack (Umbrella Project)
If initialized with the BEAM/Elixir stack, the application is structured as a standard Phoenix Umbrella project.
- **`/apps`**: In an Umbrella project, *both* backend logic and frontend web interfaces are stored as separate OTP applications inside the `/apps` directory (e.g., `apps/my_app/` and `apps/my_app_web/`).
- **Targeted Tooling**: The root contains the global umbrella configuration (`mix.exs`, `config/`), but all substantive code must execute within the scoped `apps/` folders.

## 3. PHP/Javascript Stack (WordPress plugins)
If initialized with the PHP/JS stack, the application is structured as a polyglot Monorepo containing `/apps` exclusively.
- **`/apps`**: Houses zip-ready WordPress plugins (e.g., `apps/c2papress-audit/` and `apps/c2papress-pro/`) and an optional shared Composer library (e.g., `apps/c2papress-shared/`). Each plugin is a standalone deployable. Do not generate `apps/frontend` + `apps/backend` (FastAPI/SPA) or Phoenix OTP apps for this stack.

## 4. Rust/Javascript Stack (Web + Tauri desktop)
If initialized with the Rust/JS stack, the application is structured as a polyglot Monorepo containing `/apps` exclusively.
- **`/apps`**: Houses one directory per product or distribution. A product MAY be a **product-local Cargo workspace** (e.g., `apps/filesquid/Cargo.toml` with member crates under `apps/filesquid/crates/*/`) plus its TypeScript UI package (e.g., `apps/filesquid/ui/`). A white-label distribution is a UI-only package (e.g., `apps/imagesquid/`) with no Rust or server code. The Axum server host (`apps/filesquid/crates/server/`) and the Tauri 2 desktop host (`apps/filesquid/crates/desktop/`) are separate deployables. Do not add a root `Cargo.toml` workspace.

## Structure & Tooling Isolation

Each application inside the apps directory must be a self-contained project with its own tooling and source code based on its stack.

**Python/JS Stack Example:**
- `/apps/frontend/`
  - `/apps/frontend/package.json`
  - `/apps/frontend/src/`
- `/apps/backend/`
  - `/apps/backend/requirements.txt`
  - `/apps/backend/app/`

**BEAM/Elixir Stack Example:**
- `/apps/my_app/`
  - `/apps/my_app/mix.exs`
  - `/apps/my_app/lib/`
  - `/apps/my_app/test/`

**PHP/JS Stack Example:**
- `/apps/c2papress-audit/`
  - `/apps/c2papress-audit/composer.json`
  - `/apps/c2papress-audit/package.json`
  - `/apps/c2papress-audit/src/`
  - `/apps/c2papress-audit/assets/`
- `/apps/c2papress-pro/`
  - `/apps/c2papress-pro/composer.json`
  - `/apps/c2papress-pro/src/`

**Rust/JS Stack Example:**
- `/apps/filesquid/`
  - `/apps/filesquid/Cargo.toml` (product-local `[workspace]`)
  - `/apps/filesquid/Cargo.lock`
  - `/apps/filesquid/rust-toolchain.toml`
  - `/apps/filesquid/crates/core/Cargo.toml`
  - `/apps/filesquid/crates/core/src/`
  - `/apps/filesquid/crates/server/Cargo.toml`
  - `/apps/filesquid/crates/server/src/`
  - `/apps/filesquid/crates/desktop/Cargo.toml`
  - `/apps/filesquid/crates/desktop/src/`
  - `/apps/filesquid/crates/desktop/tauri.conf.json`
  - `/apps/filesquid/ui/package.json`
  - `/apps/filesquid/ui/src/`
- `/apps/imagesquid/`
  - `/apps/imagesquid/package.json`
  - `/apps/imagesquid/src/`

1. **No Language Tooling at Root:** You MUST NOT place language-specific dependency manifests (e.g., `package.json`, `mix.exs`, `requirements.txt`, `composer.json`, `Cargo.toml`, `rust-toolchain.toml`) or framework configuration files in the global repository root.
2. **Strict Scaffolding:** When generating or scaffolding a new project (e.g., running `mix phx.new`, `npx create-next-app`, `cargo new`, Composer plugin scaffolds, or `npm create tauri-app`), you MUST execute the command to place the output inside `apps/`. NEVER run a scaffolding command targeting the repository root. If a scaffolder writes `package.json` next to `src-tauri/` (Tauri), relocate the UI into `apps/filesquid/ui/` and the Rust host into `apps/filesquid/crates/desktop/`.
3. **Path Resolution:** When adding CI pipelines, executing tests, or running scripts, always execute language commands (like `npm run dev`, `pytest`, `mix test`, `phpunit`, `cargo test`, or `npx tauri`) from within the specific workspace sub-directory, rather than the root. For a product-local Cargo workspace, run Cargo from the workspace directory (e.g., `cd apps/filesquid && cargo test -p filesquid-ocfl`). For Tauri, the Cargo crate root is `apps/filesquid/crates/desktop/`.
4. **Unique Port Assignments:** When scaffolding any new service or web module within the `/apps` directory, you MUST explicitly hardcode a unique HTTP port for its development server (e.g. `config/dev.exs`, `vite.config.ts`, Axum bind address, or Tauri `build.devUrl`) by scanning the repository's existing applications and incrementing exactly +1 to ensure no two workspaces ever clash on the same port locally. Current assignments: `apps/filesquid/ui` dev 5173 / preview 4173, `apps/imagesquid` dev 5174 / preview 4174, `filesquid-server` `127.0.0.1:8083` (`FILESQUID_BIND`).
5. **UI Mockups Folders:** The repository root MUST keep `resources/ui-mockups/` with a `README.md` for product-wide mockups. Every app under `/apps` MUST have `apps/<app>/resources/ui-mockups/` with a `README.md` that names its active suite (or says there is none yet). When scaffolding a new app, you MUST create this folder and README in the same change. Mockups for surfaces owned by one app go in that app's folder, never in the root folder.
