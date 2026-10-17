// @vitest-environment node
/**
 * The default distribution supplies every required label and design token
 * (light and dark). Other distributions run the same checks in their own tests.
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import labels from "../../src/theme/labels.en.json";
import { REQUIRED_LABEL_KEYS, missingLabels } from "../../src/theme/labels";
import { REQUIRED_TOKENS, missingTokens } from "../../src/theme/tokens";

const tokensCss = readFileSync(new URL("../../src/theme/tokens.css", import.meta.url), "utf8");

describe("default label table", () => {
  it("covers every internal name", () => {
    expect(missingLabels(labels)).toEqual([]);
  });
  it("uses the internal names as FileSquid nouns", () => {
    expect(labels.collection).toBe("Collection");
    expect(labels.virtualCollection).toBe("Virtual collection");
  });
  it("lists required keys for repository through file", () => {
    expect(REQUIRED_LABEL_KEYS).toContain("repository");
    expect(REQUIRED_LABEL_KEYS).toContain("file");
  });
});

describe("default design tokens", () => {
  it("defines every required token for light and dark", () => {
    expect(missingTokens(tokensCss)).toEqual({ light: [], dark: [] });
  });
  it("reports tokens a distribution forgot", () => {
    expect(missingTokens(":root { --fs-surface: #fff; }").light).toContain("--fs-primary");
    expect(REQUIRED_TOKENS.length).toBeGreaterThan(5);
  });
  it("does not count commented-out declarations", () => {
    const css = tokensCss.replace(/(--fs-primary\s*:[^;]*;)/g, "/* $1 */");
    expect(missingTokens(css).light).toContain("--fs-primary");
    expect(missingTokens(css).dark).toContain("--fs-primary");
  });
});
