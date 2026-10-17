// @vitest-environment node
/**
 * The ImageSquid distribution MUST supply every label and design token FileSquid
 * requires (ADR 031), and MUST use the ImageSquid vocabulary (ImageSquid U-05).
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { REQUIRED_LABEL_KEYS, missingLabels, missingTokens } from "@filesquid/ui/theme";
import labels from "../../src/theme/labels.en.json";

const tokensCss = readFileSync(new URL("../../src/theme/tokens.css", import.meta.url), "utf8");

describe("ImageSquid label table", () => {
  it("defines every required key", () => {
    expect(missingLabels(labels)).toEqual([]);
  });
  it("uses repo, album, gallery and exhibit for the internal nouns", () => {
    expect(labels.repository).toBe("Repo");
    expect(labels.collections).toBe("Albums");
    expect(labels.virtualCollections).toBe("Galleries");
    expect(labels.virtualExhibits).toBe("Exhibits");
    const nounKeys = REQUIRED_LABEL_KEYS.filter((key) => !key.startsWith("api"));
    const nouns = nounKeys
      .map((key) => labels[key])
      .join(" ")
      .toLowerCase();
    for (const internal of ["repository", "virtual collection", "virtual exhibit", "collection"]) {
      expect(nouns).not.toContain(internal);
    }
  });
  it("carries the screenshot wording (IS-S-004)", () => {
    expect(labels.automaticGalleriesHeading).toBe("Collections");
    expect(labels.searchPlaceholder).toBe("Search photos, collections, dates…");
  });
});

describe("ImageSquid design tokens", () => {
  it("define every required token for light and dark", () => {
    expect(missingTokens(tokensCss)).toEqual({ light: [], dark: [] });
  });
});
