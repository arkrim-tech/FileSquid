/**
 * End-to-end: the BUILT FileSquid UI served by the real `filesquid-server`, calling the
 * real FileSquid API. Strict DOM assertions; any console error fails the test.
 */
import { expect, test, type Page } from "@playwright/test";
import { ADMIN_TOKEN } from "./admin-token";

/** Collect console errors and uncaught page errors for the whole test. */
function trackErrors(page: Page): string[] {
  const errors: string[] = [];
  page.on("console", (msg) => {
    if (msg.type() === "error") errors.push(msg.text());
  });
  page.on("pageerror", (err) => errors.push(err.message));
  return errors;
}

test("shell mounts with FileSquid brand, internal vocabulary and live API health", async ({
  page,
}) => {
  const errors = trackErrors(page);
  await page.goto("/");
  await expect(page).toHaveTitle("FileSquid");
  expect(await page.evaluate(() => customElements.get("fs-app-shell") !== undefined)).toBe(true);

  const shell = page.locator("fs-app-shell");
  await expect(shell.locator("h1")).toHaveText("FileSquid");
  await expect(shell.locator("header img")).toHaveAttribute("alt", "FileSquid logo");
  await expect(shell.locator("nav li")).toHaveText([
    "Libraries",
    "Collections",
    "Virtual collections",
    "Virtual exhibits",
    "Monitor",
  ]);
  await expect(shell.locator('[role="status"]')).toHaveText("API: ok (filesquid)");
  await expect(shell.locator('[role="alert"]')).toHaveCount(0);
  expect(errors).toEqual([]);
});

test("design tokens switch between light and dark", async ({ page }) => {
  const background = () => page.evaluate(() => getComputedStyle(document.body).backgroundColor);
  await page.emulateMedia({ colorScheme: "light" });
  await page.goto("/");
  expect(await background()).toBe("rgb(255, 255, 255)");
  await page.emulateMedia({ colorScheme: "dark" });
  expect(await background()).toBe("rgb(16, 19, 27)");
  await page.evaluate(() => document.documentElement.setAttribute("data-theme", "light"));
  expect(await background()).toBe("rgb(255, 255, 255)");
});

test("client-side routes fall back to the distribution index", async ({ page }) => {
  const response = await page.goto("/library/some/deep/link");
  expect(response?.status()).toBe(200);
  await expect(page.locator("fs-app-shell h1")).toHaveText("FileSquid");
});

test("Monitor asks for the admin token, then shows live health, processes and the journal", async ({
  page,
}) => {
  await page.goto("/#/monitor");
  const monitor = page.locator("fs-app-shell fs-monitor");
  const form = monitor.locator("form[data-form='token']");
  await expect(form.locator("label")).toHaveText("Admin token");
  await expect(form.locator("input")).toHaveAttribute("type", "password");
  await form.locator("input").fill(ADMIN_TOKEN);
  await form.locator("button").click();

  await expect(monitor.locator('[data-section="health"] h2')).toHaveText("Health");
  await expect(monitor.locator('[data-section="health"]')).toHaveAttribute(
    "data-status",
    /^(ok|degraded|failing)$/,
  );
  const host = monitor.locator('tr[data-process-id^="host-"]');
  await expect(host.locator("td")).toHaveText(["host", "running", "0", "–", "–"]);
  await expect(monitor.locator('[data-section="storage"] dd').first()).toHaveText(/ GiB$/);
  // The data dir persists between runs, so earlier host starts may be listed too.
  await expect(
    monitor
      .locator('[data-section="journal"] tbody tr td:nth-child(2)', { hasText: "host.started" })
      .first(),
  ).toBeVisible();
});

test("Monitor with a stored token mounts without console errors and leaves on navigation", async ({
  page,
}) => {
  const errors = trackErrors(page);
  await page.addInitScript(
    (token) => sessionStorage.setItem("filesquid.adminToken", token),
    ADMIN_TOKEN,
  );
  await page.goto("/");
  await expect(page.locator("fs-monitor")).toHaveCount(0);
  await page.locator("fs-app-shell nav a").click();
  await expect(page).toHaveURL(/#\/monitor$/);
  await expect(page.locator('fs-monitor [data-section="processes"] h2')).toHaveText("Processes");
  await expect(page.locator("fs-monitor form[data-form='token']")).toHaveCount(0);
  await page.evaluate(() => (location.hash = ""));
  await expect(page.locator("fs-monitor")).toHaveCount(0);
  expect(errors).toEqual([]);
});
