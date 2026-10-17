// @vitest-environment node
/**
 * Contract drift test (FileSquid T-048): the committed TypeScript types MUST equal a fresh
 * generation from the Rust-owned OpenAPI contract. Regenerate with `npm run contract:types`.
 */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import openapiTS, { astToString } from "openapi-typescript";
import { expect, it } from "vitest";

const contractUrl = new URL("../../../crates/api/openapi.json", import.meta.url);
const typesPath = fileURLToPath(new URL("../../src/api/schema.d.ts", import.meta.url));

it("src/api/schema.d.ts matches crates/api/openapi.json", async () => {
  const contract = JSON.parse(readFileSync(contractUrl, "utf8"));
  const generated = astToString(await openapiTS(contract));
  const committed = readFileSync(typesPath, "utf8");
  const body = (text: string) => text.slice(text.indexOf("export"));
  expect(body(committed).trim()).toBe(body(generated).trim());
});
