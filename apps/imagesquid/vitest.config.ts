/**
 * Unit and UI tests (jsdom). JUnit results go to tests/allure-results for the report.
 */
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    environment: "jsdom",
    include: ["tests/unit/**/*.test.ts", "tests/ui/**/*.test.ts"],
    reporters: ["default", "junit"],
    outputFile: { junit: "tests/allure-results/vitest-junit.xml" },
    cache: false,
  },
});
