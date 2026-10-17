/**
 * UI tests for the `#/monitor` hash route and the session token store: the Monitor is
 * mounted only on its route, removed when leaving it, and storage failures never throw.
 */
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  MONITOR_ROUTE,
  bindMonitorRoute,
  defineElements,
  sessionTokenStore,
  type MonitorClient,
} from "../../src/index";
import labels from "../../src/theme/labels.en.json";

const never = () => new Promise<never>(() => undefined);
const client: MonitorClient = {
  adminHealth: never,
  adminProcesses: never,
  adminJournal: never,
  adminMetrics: never,
  adminJournalExport: never,
  subscribe: () => () => undefined,
};

describe("bindMonitorRoute", () => {
  let unbind: () => void = () => undefined;
  beforeEach(() => defineElements());
  afterEach(() => {
    unbind();
    window.location.hash = "";
  });

  it("mounts <fs-monitor> on #/monitor and removes it on other hashes", () => {
    const shell = document.createElement("div");
    document.body.replaceChildren(shell);
    window.location.hash = "";
    unbind = bindMonitorRoute(shell, { labels, client, pollMs: 0 });
    expect(shell.querySelector("fs-monitor")).toBeNull();
    window.location.hash = MONITOR_ROUTE;
    window.dispatchEvent(new HashChangeEvent("hashchange"));
    expect(shell.querySelectorAll("fs-monitor")).toHaveLength(1);
    window.dispatchEvent(new HashChangeEvent("hashchange"));
    expect(shell.querySelectorAll("fs-monitor")).toHaveLength(1);
    window.location.hash = "#/libraries";
    window.dispatchEvent(new HashChangeEvent("hashchange"));
    expect(shell.querySelector("fs-monitor")).toBeNull();
  });
});

describe("sessionTokenStore", () => {
  it("round-trips a token through the given storage", () => {
    const store = sessionTokenStore(window.sessionStorage);
    store.set("abc");
    expect(store.get()).toBe("abc");
    window.sessionStorage.clear();
  });

  it("returns undefined and warns instead of throwing when storage is blocked", () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => undefined);
    const blocked = {
      getItem: () => {
        throw new Error("blocked");
      },
      setItem: () => {
        throw new Error("blocked");
      },
    } as unknown as Storage;
    const store = sessionTokenStore(blocked);
    expect(() => store.set("abc")).not.toThrow();
    expect(store.get()).toBeUndefined();
    expect(warn).toHaveBeenCalledTimes(2);
    warn.mockRestore();
  });
});
