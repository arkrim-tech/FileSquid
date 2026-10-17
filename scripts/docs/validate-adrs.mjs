/**
 * Validate the FileSquid stack ADRs (001-033) for ISO 42010 section presence,
 * the ADR header, the expected Status, relative linking, and key decision
 * vocabulary. Every ADR MUST use the FileSquid header.
 *
 * Usage (from repository root or this directory):
 *   node scripts/docs/validate-adrs.mjs
 */
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(here, "../..");
const adrDir = join(repoRoot, "resources/architecture");

export const REQUIRED_FILES = [
  "README.md",
  "001-shared-rust-core-with-tauri-and-axum-hosts.md",
  "002-tauri2-desktop-shell.md",
  "003-svelte-client-rendered-ui.md",
  "004-axum-hosted-ciphertext-mailbox.md",
  "005-sqlite-local-postgres-hosted.md",
  "006-libvips-native-stills-not-imagemagick.md",
  "007-durable-jobs-and-interactive-performance.md",
  "008-browser-wasm-scope.md",
  "009-cryptography-webcrypto-compatible-suite.md",
  "010-ocfl-1-1-dual-profile.md",
  "011-native-media-adapters-and-ffmpeg-license.md",
  "012-s3-compatible-ciphertext-storage.md",
  "013-cargo-svelte-workspaces.md",
  "014-stdout-observability.md",
  "015-prov-o-jsonld-and-c2pa-sdk.md",
  "016-rust-ocfl-library.md",
  "017-test-runners-and-reporting.md",
  "018-inline-docs-generators.md",
  "019-coding-style-and-lint.md",
  "020-distribution-web-and-tauri.md",
  "021-uuid-v4-identifiers.md",
  "022-dublin-core-descriptive-metadata.md",
  "023-filesquid-ocfl-engine-and-parquet.md",
  "024-filesquid-metadata-engine.md",
  "025-client-side-filesquid-engines.md",
  "026-filesquid-provenance-engine.md",
  "027-filesquid-metadata-pluggable-stores.md",
  "028-filesquid-ui-packages-and-panel-composition.md",
  "029-filesquid-unified-product-and-distributions.md",
  "030-filesquid-api-and-transports.md",
  "031-vanilla-typescript-ui-and-white-label-theming.md",
  "032-hosting-and-privacy-model.md",
  "033-activity-journal-and-live-health-monitoring.md",
];

/** ADRs whose Status is not plain Accepted, mapped to the allowed status words. */
export const STATUS_OVERRIDES = {
  "003-svelte-client-rendered-ui.md": ["Superseded"],
  "030-filesquid-api-and-transports.md": ["Proposed", "Accepted"],
};

/**
 * Return the ADR header line a given ADR file must start with.
 * @param {string} name ADR filename such as `029-...md`.
 * @returns {string[]} Accepted header lines (with trailing newline).
 */
export function allowedHeaders(name) {
  return ["# Architecture Decision Record - FileSquid\n"];
}

/**
 * Return the Status words allowed for an ADR file.
 * @param {string} name ADR filename.
 * @returns {string[]} Allowed first words of the Status section.
 */
export function allowedStatuses(name) {
  return STATUS_OVERRIDES[name] ?? ["Accepted"];
}

export const REQUIRED_SECTIONS = [
  "Title",
  "Status",
  "Context / Requirement Reference",
  "Decision",
  "Rationale",
  "Assumptions",
  "Alternatives Considered",
  "Consequences / Implications",
  "Related Decisions / Notes",
];

const ABSOLUTE_PATH = /(?:[A-Za-z]:\\|\/Users\/|\/home\/|file:\/\/)/;
const MARKDOWN_LINK = /\[[^\]]*\]\(([^)\s]+)\)/g;
const WINDOWS_DRIVE_LINK = /^[A-Za-z]:[\\/]/;

/**
 * @param {string} href
 * @returns {boolean}
 */
export function isProhibitedMarkdownHref(href) {
  const dest = href.trim();
  if (dest.startsWith("#") || dest.startsWith("./") || dest.startsWith("../")) {
    return false;
  }
  if (/^https?:\/\//i.test(dest) || dest.startsWith("mailto:")) {
    return false;
  }
  if (dest.startsWith("file:") || dest.startsWith("/") || WINDOWS_DRIVE_LINK.test(dest)) {
    return true;
  }
  return ABSOLUTE_PATH.test(dest);
}

/**
 * @param {string} name
 * @returns {string}
 */
function readAdr(name) {
  return readFileSync(join(adrDir, name), "utf8");
}

/**
 * @returns {{ ok: boolean, errors: string[], files: string[] }}
 */
export function validateAdrs() {
  const errors = [];
  let files = [];
  try {
    files = readdirSync(adrDir);
  } catch (err) {
    return {
      ok: false,
      errors: [`ADR directory missing at resources/architecture: ${err.message}`],
      files: [],
    };
  }

  for (const name of REQUIRED_FILES) {
    if (!files.includes(name)) {
      errors.push(`missing ${name}`);
    }
  }

  for (const name of REQUIRED_FILES) {
    if (!files.includes(name)) {
      continue;
    }
    const body = readAdr(name);
    if (ABSOLUTE_PATH.test(body)) {
      errors.push(`${name} contains a prohibited absolute path`);
    }
    MARKDOWN_LINK.lastIndex = 0;
    let linkMatch = MARKDOWN_LINK.exec(body);
    while (linkMatch) {
      if (isProhibitedMarkdownHref(linkMatch[1])) {
        errors.push(`${name} contains a prohibited absolute Markdown link: ${linkMatch[1]}`);
      }
      linkMatch = MARKDOWN_LINK.exec(body);
    }
    if (name === "README.md") {
      for (const adrName of REQUIRED_FILES) {
        if (adrName === "README.md") {
          continue;
        }
        if (!body.includes(adrName)) {
          errors.push(`README.md must index ${adrName}`);
        }
      }
      continue;
    }
    if (!allowedHeaders(name).some((header) => body.startsWith(header))) {
      errors.push(`${name} must start with Architecture Decision Record header`);
    }
    if (!body.includes(`\n${name}\n`)) {
      errors.push(`${name} must include its filename on its own line`);
    }
    for (const section of REQUIRED_SECTIONS) {
      if (!body.includes(`\n## ${section}\n`)) {
        errors.push(`${name} missing section ${section}`);
      }
    }
    const statuses = allowedStatuses(name);
    if (!statuses.some((status) => body.includes(`## Status\n\n${status}\n`))) {
      errors.push(`${name} Status must be one of: ${statuses.join(", ")}`);
    }
  }

  if (files.includes("001-shared-rust-core-with-tauri-and-axum-hosts.md")) {
    const body = readAdr("001-shared-rust-core-with-tauri-and-axum-hosts.md");
    for (const term of ["Tauri", "Axum", "WebAssembly", "Svelte", "envelopes", "libvips", "Portable contracts"]) {
      if (!body.includes(term)) {
        errors.push(`ADR 001 must mention ${term}`);
      }
    }
  }

  if (files.includes("003-svelte-client-rendered-ui.md")) {
    const body = readAdr("003-svelte-client-rendered-ui.md");
    if (!body.includes("React")) {
      errors.push("ADR 003 must reject React");
    }
  }

  if (files.includes("004-axum-hosted-ciphertext-mailbox.md")) {
    const body = readAdr("004-axum-hosted-ciphertext-mailbox.md");
    if (!body.includes("plaintext")) {
      errors.push("ADR 004 must state the plaintext prohibition");
    }
    if (!body.includes("static files") || !body.includes("server-render")) {
      errors.push("ADR 004 must distinguish static file hosting from server-rendered private pages");
    }
  }

  if (files.includes("006-libvips-native-stills-not-imagemagick.md")) {
    const body = readAdr("006-libvips-native-stills-not-imagemagick.md");
    if (!body.includes("ImageMagick") || !body.includes("MUST NOT")) {
      errors.push("ADR 006 must forbid co-installing ImageMagick");
    }
  }

  if (files.includes("005-sqlite-local-postgres-hosted.md")) {
    const body = readAdr("005-sqlite-local-postgres-hosted.md");
    if (!body.includes("canonical receipt")) {
      errors.push("ADR 005 must adopt the canonical-receipt commit sequence");
    }
  }

  if (files.includes("007-durable-jobs-and-interactive-performance.md")) {
    const body = readAdr("007-durable-jobs-and-interactive-performance.md");
    if (!body.includes("100 ms") || !body.includes("30 s")) {
      errors.push("ADR 007 must record the 100 ms and 30 s budgets");
    }
    if (!body.includes("canonical receipt")) {
      errors.push("ADR 007 must reconcile jobs to a canonical receipt");
    }
  }

  if (files.includes("008-browser-wasm-scope.md")) {
    const body = readAdr("008-browser-wasm-scope.md");
    if (!body.includes("shards")) {
      errors.push("ADR 008 must define replica bootstrap via index shards");
    }
  }

  if (files.includes("021-uuid-v4-identifiers.md")) {
    const body = readAdr("021-uuid-v4-identifiers.md");
    if (!body.includes("UUID version 4")) {
      errors.push("ADR 021 must specify UUID version 4");
    }
    if (!body.includes("RFC 4122") && !body.includes("RFC 9562")) {
      errors.push("ADR 021 must cite RFC 4122 or RFC 9562");
    }
    if (
      !body.includes("capture time") &&
      !body.includes("capture date") &&
      !body.includes("capture dates")
    ) {
      errors.push("ADR 021 must forbid encoding capture time or capture date");
    }
    if (!body.includes("urn:uuid:")) {
      errors.push("ADR 021 must specify urn:uuid OCFL object identifiers");
    }
    if (!body.includes("SHA-256")) {
      errors.push("ADR 021 must keep SHA-256 digests distinct from logical IDs");
    }
  }

  if (files.includes("016-rust-ocfl-library.md")) {
    const body = readAdr("016-rust-ocfl-library.md");
    if (!body.includes("Git MUST NOT")) {
      errors.push("ADR 016 must state Git MUST NOT be the OCFL library");
    }
    if (!body.includes("client-engine family") || !body.includes("never in the ciphertext-only")) {
      errors.push("ADR 016 must place the OCFL library in the client-engine family, never in the ciphertext-only hosted service (ADR 025, ADR 032)");
    }
    if (!body.includes("browse") || !body.includes("read")) {
      errors.push("ADR 016 must require read and browse, not write only");
    }
    if (!body.includes("authors its own")) {
      errors.push("ADR 016 must state that FileSquid authors the OCFL library");
    }
    if (!body.includes("rocfl")) {
      errors.push("ADR 016 must name rocfl as a community OCFL program");
    }
    if (!body.includes("MUST NOT be FileSquid's library")) {
      errors.push("ADR 016 must state community OCFL tools MUST NOT be FileSquid's library");
    }
    if (!body.includes("filesystem") || !body.includes("S3-compatible")) {
      errors.push("ADR 016 must require local filesystem and S3-compatible backends");
    }
    if (!body.includes("filesquid-ocfl")) {
      errors.push("ADR 016 must freeze the crate name filesquid-ocfl");
    }
  }

  if (files.includes("023-filesquid-ocfl-engine-and-parquet.md")) {
    const body = readAdr("023-filesquid-ocfl-engine-and-parquet.md");
    if (!body.includes("inventory-driven")) {
      errors.push("ADR 023 must require inventory-driven logical state");
    }
    if (!body.includes("parquet")) {
      errors.push("ADR 023 must name the later parquet Cargo feature");
    }
    if (!body.includes("FileSquid application profile")) {
      errors.push("ADR 023 must keep the FileSquid profile as a caller");
    }
    if (!body.includes("stand-alone FOSS")) {
      errors.push("ADR 023 must state filesquid-ocfl is a stand-alone FOSS product");
    }
  }

  // ADR 025: both filesquid engines run on every client and never on the server.
  if (files.includes("025-client-side-filesquid-engines.md")) {
    const body = readAdr("025-client-side-filesquid-engines.md");
    for (const term of ["wasm32-unknown-unknown", "service MUST NOT depend on either crate", "async and runtime-agnostic"]) {
      if (!body.includes(term)) {
        errors.push(`ADR 025 must state: ${term}`);
      }
    }
    if (!body.includes("cache and staging area only")) {
      errors.push("ADR 025 must keep browser storage as a cache, never canonical");
    }
  }

  // ADR 026: filesquid-provenance is a from-scratch engine; the official crate is only an oracle.
  if (files.includes("026-filesquid-provenance-engine.md")) {
    const body = readAdr("026-filesquid-provenance-engine.md");
    for (const term of ["from scratch", "MUST NOT hand-roll cryptographic primitives", "MUST NOT be the implementation", "wasm32-unknown-unknown"]) {
      if (!body.includes(term)) {
        errors.push(`ADR 026 must state: ${term}`);
      }
    }
  }

  errors.push(...checkFileSquidAdrs(files));

  return { ok: errors.length === 0, errors, files };
}

/** Required phrases for ADRs 029-032, keyed by filename. */
export const FILESQUID_TERMS = {
  "029-filesquid-unified-product-and-distributions.md": [
    "apps/filesquid/crates/",
    "MUST NOT take a default dependency on another module crate",
    "MUST NOT contain Rust",
    "filesquid-c2pa",
    "MUST be extractable",
  ],
  "030-filesquid-api-and-transports.md": [
    "OpenAPI 3.1",
    "register_asynchronous_uri_scheme_protocol",
    "No TCP port is opened",
    "tauri::ipc::Channel",
    "Service Workers cannot intercept",
    "per-launch token",
    "spike S1",
  ],
  "031-vanilla-typescript-ui-and-white-label-theming.md": [
    "custom elements",
    "No UI framework",
    "--fs-",
    "label table",
    "tauri build --config",
    "built artifacts",
  ],
  "032-hosting-and-privacy-model.md": [
    "*.filesquid.tech",
    "MUST NOT hold library keys",
    "MUST NOT parse OCFL",
    "That operator can read them",
    "The UI shows the host",
  ],
};

/**
 * Check that ADRs 029-032 state their key decisions.
 * @param {string[]} files Filenames present in the ADR directory.
 * @returns {string[]} Error messages; empty when every phrase is present.
 */
export function checkFileSquidAdrs(files) {
  const errors = [];
  for (const [name, terms] of Object.entries(FILESQUID_TERMS)) {
    if (!files.includes(name)) {
      continue;
    }
    const body = readAdr(name);
    for (const term of terms) {
      if (!body.includes(term)) {
        errors.push(`${name} must state: ${term}`);
      }
    }
  }
  return errors;
}

const isMain = process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url);
if (isMain) {
  const result = validateAdrs();
  if (!result.ok) {
    console.error("ADR validation failed:");
    for (const error of result.errors) {
      console.error(`- ${error}`);
    }
    process.exit(1);
  }
  console.log(`ADR validation passed (${REQUIRED_FILES.length} required files).`);
}
