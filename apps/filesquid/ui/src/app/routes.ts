/**
 * Hash routing shared by every distribution until the full navigation model (FS-2.1):
 * `#/monitor` shows the Monitor dashboard inside the shell; any other hash shows the shell
 * alone. Also provides the session-scoped admin token store used on the server host.
 */
import type { MonitorOptions, TokenStore } from "../components/fs-monitor";

/** Hash of the Monitor route. */
export const MONITOR_ROUTE = "#/monitor";

const TOKEN_KEY = "filesquid.adminToken";

/**
 * Keep the admin token in `sessionStorage` (cleared when the tab closes). Storage access
 * can throw in private windows; failures are logged and the token is simply not kept.
 *
 * @param storage Storage to use; defaults to `window.sessionStorage`.
 */
export function sessionTokenStore(storage?: Storage): TokenStore {
  const target = (): Storage | undefined => storage ?? globalThis.sessionStorage;
  return {
    get() {
      try {
        return target()?.getItem(TOKEN_KEY) ?? undefined;
      } catch (error) {
        console.warn("FileSquid could not read the admin token from session storage", error);
        return undefined;
      }
    },
    set(token: string) {
      try {
        target()?.setItem(TOKEN_KEY, token);
      } catch (error) {
        console.warn("FileSquid could not keep the admin token in session storage", error);
      }
    },
  };
}

/**
 * Show or hide an `<fs-monitor>` child of `shell` to match the current hash, now and on
 * every `hashchange`. Side effect: adds a window listener and mutates the shell's children.
 *
 * @param shell Element the Monitor is slotted into (the app shell).
 * @param options Monitor configuration.
 * @param win Window whose hash is followed.
 * @returns A function that removes the listener.
 */
export function bindMonitorRoute(
  shell: HTMLElement,
  options: MonitorOptions,
  win: Window = window,
): () => void {
  const apply = () => applyMonitorRoute(shell, options, win.location.hash);
  apply();
  win.addEventListener("hashchange", apply);
  return () => win.removeEventListener("hashchange", apply);
}

/** Mount the Monitor when `hash` is the Monitor route, otherwise remove it. */
function applyMonitorRoute(shell: HTMLElement, options: MonitorOptions, hash: string): void {
  const existing = shell.querySelector("fs-monitor");
  if (hash !== MONITOR_ROUTE) {
    existing?.remove();
    return;
  }
  if (existing) return;
  const monitor = document.createElement("fs-monitor") as HTMLElement & {
    configure(options: MonitorOptions): void;
  };
  monitor.configure(options);
  shell.append(monitor);
}
