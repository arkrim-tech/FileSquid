import assert from "node:assert/strict";
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import {
  REQUIRED_FILES,
  REQUIRED_SECTIONS,
  allowedHeaders,
  allowedStatuses,
  isProhibitedMarkdownHref,
  validateAdrs,
} from "./validate-adrs.mjs";

const adrDir = resolve(dirname(fileURLToPath(import.meta.url)), "../../resources/architecture");

test("stack ADRs exist with ISO 42010 sections and accepted vocabulary", () => {
  const onDisk = readdirSync(adrDir);
  for (const name of REQUIRED_FILES) {
    assert.ok(onDisk.includes(name), `missing ${name}`);
  }

  for (const name of REQUIRED_FILES) {
    if (name === "README.md") {
      continue;
    }
    const body = readFileSync(join(adrDir, name), "utf8");
    assert.ok(
      allowedHeaders(name).some((header) => body.startsWith(header)),
      `${name} missing ADR header`,
    );
    for (const section of REQUIRED_SECTIONS) {
      assert.ok(body.includes(`\n## ${section}\n`), `${name} missing ${section}`);
    }
  }

  const result = validateAdrs();
  assert.equal(result.ok, true, result.errors.join("\n"));
});

test("ADR markdown hrefs reject absolute destinations", () => {
  assert.equal(isProhibitedMarkdownHref("./001-shared-rust-core-with-tauri-and-axum-hosts.md"), false);
  assert.equal(isProhibitedMarkdownHref("../prd.md#f-001"), false);
  assert.equal(isProhibitedMarkdownHref("https://v2.tauri.app/"), false);
  assert.equal(isProhibitedMarkdownHref("/docs/architecture/001.md"), true);
  assert.equal(isProhibitedMarkdownHref("file:///tmp/adr.md"), true);
  assert.equal(isProhibitedMarkdownHref("C:\\\\temp\\\\adr.md"), true);
});

test("FileSquid ADRs 029-032 are indexed with the FileSquid header and expected status", () => {
  for (const number of ["029", "030", "031", "032"]) {
    assert.ok(
      REQUIRED_FILES.some((name) => name.startsWith(`${number}-`)),
      `REQUIRED_FILES must list ADR ${number}`,
    );
  }
  assert.deepEqual(allowedHeaders("029-filesquid-unified-product-and-distributions.md"), [
    "# Architecture Decision Record - FileSquid\n",
  ]);
  assert.deepEqual(allowedHeaders("001-shared-rust-core-with-tauri-and-axum-hosts.md"), [
    "# Architecture Decision Record - FileSquid\n",
  ]);
  assert.deepEqual(allowedStatuses("030-filesquid-api-and-transports.md"), ["Proposed", "Accepted"]);
  assert.deepEqual(allowedStatuses("003-svelte-client-rendered-ui.md"), ["Superseded"]);
  assert.deepEqual(allowedStatuses("032-hosting-and-privacy-model.md"), ["Accepted"]);
});
