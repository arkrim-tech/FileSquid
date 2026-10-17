/**
 * End-to-end: the BUILT ImageSquid distribution served by the real `filesquid-server`.
 * ImageSquid is FileSquid underneath: same API, ImageSquid brand, vocabulary and tokens.
 */
import { expect, test } from "@playwright/test";
import { ADMIN_TOKEN } from "./admin-token";

test("ImageSquid brand, album vocabulary and FileSquid API health", async ({ page }) => {
  const errors: string[] = [];
  page.on("console", (msg) => msg.type() === "error" && errors.push(msg.text()));
  page.on("pageerror", (err) => errors.push(err.message));
  await page.goto("/");
  await expect(page).toHaveTitle("ImageSquid");
  const shell = page.locator("fs-app-shell");
  await expect(shell.locator("h1")).toHaveText("ImageSquid");
  await expect(shell.locator("header img")).toHaveAttribute("src", "/imagesquid-logo.svg");
  await expect(shell.locator("nav li")).toHaveText([
    "Libraries",
    "Albums",
    "Galleries",
    "Exhibits",
    "Monitor",
  ]);
  await expect(shell.locator('[role="status"]')).toHaveText("Archives API: ok (filesquid)");
  expect(errors).toEqual([]);
});

test("ImageSquid tokens override FileSquid defaults in light and dark", async ({ page }) => {
  const primary = () =>
    page.evaluate(() =>
      getComputedStyle(document.documentElement).getPropertyValue("--fs-primary").trim(),
    );
  const sidebar = () =>
    page.evaluate(() => {
      const nav = document.querySelector("fs-app-shell")?.shadowRoot?.querySelector("nav");
      return nav ? getComputedStyle(nav).backgroundColor : "";
    });
  await page.emulateMedia({ colorScheme: "light" });
  await page.goto("/");
  expect(await primary()).toBe("#1d3f73");
  expect(await sidebar()).toBe("rgb(244, 245, 247)");
  await page.emulateMedia({ colorScheme: "dark" });
  expect(await primary()).toBe("#9cc0ff");
  expect(await sidebar()).toBe("rgb(27, 30, 36)");
});

test("ImageSquid reaches the FileSquid Monitor with its own vocabulary", async ({ page }) => {
  const errors: string[] = [];
  page.on("console", (msg) => msg.type() === "error" && errors.push(msg.text()));
  page.on("pageerror", (err) => errors.push(err.message));
  await page.addInitScript(
    (token) => sessionStorage.setItem("filesquid.adminToken", token),
    ADMIN_TOKEN,
  );
  await page.goto("/");
  await page.locator("fs-app-shell nav a", { hasText: "Monitor" }).click();
  const monitor = page.locator("fs-monitor");
  await expect(monitor.locator('[data-section="journal"] h2')).toHaveText("Activity history");
  await expect(monitor.locator('[data-section="storage"] h2')).toHaveText("Storage");
  await expect(monitor.locator('tr[data-process-id^="host-"]')).toHaveAttribute(
    "data-state",
    "running",
  );
  expect(errors).toEqual([]);
});
