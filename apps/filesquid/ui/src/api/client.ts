/**
 * FileSquid API client.
 *
 * Every UI reaches FileSquid only through this client (FileSquid U-16). It hides which
 * host answers: `filesquid-server` over HTTP and WebSocket, or the desktop host through
 * the `filesquid://` custom URI scheme and Tauri events (ADR 030).
 */
import type { components } from "./schema";

/** Process health payload, generated from the API contract. */
export type HealthStatus = components["schemas"]["HealthStatus"];

/** One push event from the API event channel. */
export type ApiEvent = components["schemas"]["ApiEvent"];

/** Callback for push events. */
export type EventListener = (event: ApiEvent) => void;

/** Monitor health report (`GET /api/v1/admin/health`). */
export type HealthReport = components["schemas"]["HealthReport"];
/** One live or recently finished process. */
export type ProcessView = components["schemas"]["ProcessView"];
/** One activity journal entry. */
export type JournalEntry = components["schemas"]["JournalEntry"];
/** One page of journal entries. */
export type JournalPage = components["schemas"]["JournalPage"];
/** Journal entry outcome. */
export type Outcome = components["schemas"]["Outcome"];
/** Journal entry level. */
export type Level = components["schemas"]["Level"];
/** Metric name to value (same values as Prometheus `/metrics`). */
export type MetricsSnapshot = Record<string, number>;

/** Filters for journal queries and exports; every field is optional. */
export interface JournalFilters {
  /** Earliest time, RFC 3339. */
  from?: string;
  /** Latest time, RFC 3339. */
  to?: string;
  /** Exact actor. */
  actor?: string;
  /** Exact operation, for example `ocfl.commit`. */
  operation?: string;
  /** Exact outcome. */
  outcome?: Outcome;
  /** Exact level. */
  level?: Level;
  /** Exact process id. */
  process?: string;
  /** Cursor from the previous page. */
  after?: number;
  /** Page size. */
  limit?: number;
  /** `desc` for newest first (the Monitor dashboard); `asc` (default) otherwise. */
  order?: "asc" | "desc";
}

/** Response header naming the cursor to continue a truncated journal export. */
const NEXT_CURSOR_HEADER = "X-Journal-Next-Cursor";
/** Export requests followed at most (each up to 1 000 000 entries on the server). */
const MAX_EXPORT_REQUESTS = 10;

/** Build a query string from defined, non-empty filter values (in declaration order). */
function queryString(filters: JournalFilters): string {
  const params = new URLSearchParams();
  for (const [key, value] of Object.entries(filters)) {
    if (value !== undefined && value !== "") params.set(key, String(value));
  }
  const text = params.toString();
  return text ? `?${text}` : "";
}

/** Facts about the host the UI is running in. */
export interface HostEnvironment {
  /** True inside a Tauri desktop webview. */
  isTauri: boolean;
  /** Operating system family; only Windows changes the scheme URL form. */
  platform: "windows" | "android" | "macos" | "linux" | "other";
}

/** Error raised when the API answers with a non-2xx status. */
export class ApiError extends Error {
  /**
   * @param status HTTP status code returned by the API.
   * @param path API path that failed.
   */
  constructor(
    public readonly status: number,
    public readonly path: string,
  ) {
    super(`FileSquid API ${path} failed with HTTP ${status}`);
    this.name = "ApiError";
  }
}

/** Options for {@link FileSquidClient}. Transports are injectable for tests. */
export interface ClientOptions {
  /** Prefix for API paths: "" (same origin), "filesquid://localhost" or "http://filesquid.localhost" (Windows, Android). */
  baseUrl: string;
  /** WebSocket URL of the event channel (server host only). */
  eventsUrl?: string;
  /** Subscribe function for desktop events (Tauri). When set, WebSocket is not used. */
  subscribeHost?: (listener: EventListener) => () => void;
  /** fetch implementation; defaults to the global fetch. */
  fetch?: typeof fetch;
  /** WebSocket constructor; defaults to the global WebSocket. */
  WebSocket?: typeof WebSocket;
  /** Admin token for the server's admin routes; not needed on the desktop host. */
  adminToken?: () => string | undefined;
}

/**
 * Return the API base URL for a host environment.
 *
 * @param env Host facts from {@link detectHost}.
 * Tauri serves custom schemes as `http://<scheme>.localhost` on Windows and Android,
 * and as `<scheme>://localhost` elsewhere.
 *
 * @returns "" for same-origin HTTP, or the custom-scheme origin inside the Tauri app.
 */
export function apiBaseUrl(env: HostEnvironment): string {
  if (!env.isTauri) return "";
  const httpForm = env.platform === "windows" || env.platform === "android";
  return httpForm ? "http://filesquid.localhost" : "filesquid://localhost";
}

/**
 * Return the WebSocket URL of the event channel for a page served by `filesquid-server`.
 *
 * @param location The page URL.
 */
export function eventsUrl(location: URL): string {
  const scheme = location.protocol === "https:" ? "wss:" : "ws:";
  return `${scheme}//${location.host}/api/v1/events`;
}

/**
 * Detect the host environment from a window object.
 *
 * @param win The browser window (or a stand-in in tests).
 */
export function detectHost(win: Window): HostEnvironment {
  const isTauri = "__TAURI_INTERNALS__" in win;
  const agent = win.navigator.userAgent.toLowerCase();
  // Android user agents also contain "linux", so test for Android first.
  const platform = agent.includes("windows")
    ? "windows"
    : agent.includes("android")
      ? "android"
      : agent.includes("mac os")
        ? "macos"
        : agent.includes("linux")
          ? "linux"
          : "other";
  return { isTauri, platform };
}

/** Typed client for the FileSquid API. */
export class FileSquidClient {
  private readonly options: ClientOptions;

  /** @param options Base URL and injectable transports. */
  constructor(options: ClientOptions) {
    this.options = options;
  }

  /** Read process health (`GET /api/v1/health`). Network I/O. */
  async health(): Promise<HealthStatus> {
    return this.getJson<HealthStatus>("/api/v1/health");
  }

  /**
   * Subscribe to push events. Opens a WebSocket on the server host, or uses the
   * host-supplied subscription on the desktop host.
   *
   * @param listener Called once per event.
   * @returns A function that ends the subscription and releases its connection.
   */
  subscribe(listener: EventListener): () => void {
    if (this.options.subscribeHost) return this.options.subscribeHost(listener);
    const Socket = this.options.WebSocket ?? globalThis.WebSocket;
    const url = this.options.eventsUrl ?? eventsUrl(new URL(globalThis.location.href));
    const socket = new Socket(url);
    socket.onmessage = (message: MessageEvent<string>) => {
      try {
        listener(JSON.parse(message.data) as ApiEvent);
      } catch (error) {
        console.error("FileSquid event was not valid JSON", error);
      }
    };
    return () => socket.close();
  }

  /** Overall health with reasons (`GET /api/v1/admin/health`). Network I/O. */
  async adminHealth(): Promise<HealthReport> {
    return this.getJson<HealthReport>("/api/v1/admin/health");
  }

  /** Live and recently finished processes (`GET /api/v1/admin/processes`). Network I/O. */
  async adminProcesses(): Promise<ProcessView[]> {
    return this.getJson<ProcessView[]>("/api/v1/admin/processes");
  }

  /** One page of the activity journal (`GET /api/v1/admin/journal`). Network I/O. */
  async adminJournal(filters: JournalFilters = {}): Promise<JournalPage> {
    return this.getJson<JournalPage>(`/api/v1/admin/journal${queryString(filters)}`);
  }

  /** Counters and gauges (`GET /api/v1/admin/metrics`). Network I/O. */
  async adminMetrics(): Promise<MetricsSnapshot> {
    return this.getJson<MetricsSnapshot>("/api/v1/admin/metrics");
  }

  /**
   * All matching journal entries as JSON Lines text (`GET /api/v1/admin/journal/export`),
   * oldest first. A response cut at the server's size cap carries `X-Journal-Next-Cursor`;
   * this follows it for at most {@link MAX_EXPORT_REQUESTS} requests and throws if entries
   * still remain, rather than returning an incomplete export. Network I/O.
   */
  async adminJournalExport(
    filters: Omit<JournalFilters, "after" | "limit" | "order"> = {},
  ): Promise<string> {
    let text = "";
    let after: number | undefined;
    for (let i = 0; i < MAX_EXPORT_REQUESTS; i++) {
      const path = `/api/v1/admin/journal/export${queryString({ ...filters, after })}`;
      const response = await this.request(path);
      text += await response.text();
      const next = response.headers.get(NEXT_CURSOR_HEADER);
      if (next === null) return text;
      after = Number(next);
    }
    throw new Error(`journal export exceeds ${MAX_EXPORT_REQUESTS} requests; narrow the filters`);
  }

  /** GET a JSON resource under the base URL; throws {@link ApiError} on non-2xx. */
  private async getJson<T>(path: string): Promise<T> {
    const response = await this.request(path);
    return (await response.json()) as T;
  }

  /**
   * GET `path`; throws {@link ApiError} on non-2xx. The admin token, when set, is sent only
   * to `/api/v1/admin/*`, never to other routes.
   */
  private async request(path: string): Promise<Response> {
    const doFetch = this.options.fetch ?? globalThis.fetch.bind(globalThis);
    const token = path.startsWith("/api/v1/admin/") ? this.options.adminToken?.() : undefined;
    const init: RequestInit | undefined = token
      ? { headers: { Authorization: `Bearer ${token}` } }
      : undefined;
    const response = await doFetch(`${this.options.baseUrl}${path}`, init);
    if (!response.ok) throw new ApiError(response.status, path);
    return response;
  }
}
