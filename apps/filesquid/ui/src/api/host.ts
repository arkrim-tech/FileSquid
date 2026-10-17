/**
 * Build a {@link FileSquidClient} for the host this page runs in.
 * Inside the desktop app, events arrive as Tauri events named "filesquid-event".
 */
import { listen } from "@tauri-apps/api/event";
import {
  FileSquidClient,
  apiBaseUrl,
  detectHost,
  type ApiEvent,
  type EventListener,
} from "./client";

/** Tauri event name used by `filesquid-desktop` to forward API events. */
export const DESKTOP_EVENT = "filesquid-event";

/**
 * Subscribe to desktop events through Tauri. Side effect: registers a Tauri listener.
 *
 * @param listener Called once per event.
 * @returns Unsubscribe function.
 */
function subscribeDesktop(listener: EventListener): () => void {
  const pending = listen<ApiEvent>(DESKTOP_EVENT, (event) => listener(event.payload));
  return () => {
    pending.then((unlisten) => unlisten()).catch((error) => console.error(error));
  };
}

/**
 * Create the client for the current window.
 *
 * @param win Window the UI runs in.
 * @param adminToken Supplies the server host's admin token (not needed on the desktop host).
 */
export function createHostClient(
  win: Window = window,
  adminToken?: () => string | undefined,
): FileSquidClient {
  const env = detectHost(win);
  return new FileSquidClient({
    baseUrl: apiBaseUrl(env),
    subscribeHost: env.isTauri ? subscribeDesktop : undefined,
    adminToken,
  });
}
