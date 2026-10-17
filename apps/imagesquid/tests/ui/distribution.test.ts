/**
 * UI test: the FileSquid shell rendered as the ImageSquid distribution.
 */
import { beforeEach, expect, it } from "vitest";
import { defineElements, type FsAppShell } from "@filesquid/ui";
import { imagesquid } from "../../src/distribution";

beforeEach(() => defineElements());

it("renders ImageSquid brand and vocabulary in the shared shell", async () => {
  const shell = document.createElement("fs-app-shell") as FsAppShell;
  shell.configure({
    distribution: imagesquid,
    client: { health: async () => ({ schema_version: 1, status: "ok", product: "filesquid" }) },
  });
  document.body.replaceChildren(shell);
  await new Promise((resolve) => setTimeout(resolve, 0));
  const root = shell.shadowRoot;
  expect(root?.querySelector("h1")?.textContent).toBe("ImageSquid");
  expect(root?.querySelector("img")?.getAttribute("alt")).toBe("ImageSquid logo");
  expect([...(root?.querySelectorAll("nav li") ?? [])].map((li) => li.textContent)).toEqual([
    "Libraries",
    "Albums",
    "Galleries",
    "Exhibits",
    "Monitor",
  ]);
  expect(root?.querySelector('[role="status"]')?.textContent).toBe("Archives API: ok (filesquid)");
});
