/**
 * Vite build for the FileSquid default UI. Dev and preview ports are fixed so
 * workspaces never clash: 5173 (dev) and 4173 (preview). In dev, `/api` is proxied to
 * `filesquid-server` (FILESQUID_API_URL, default http://127.0.0.1:8083), WebSocket included.
 */
import { defineConfig } from "vite";

const api = process.env.FILESQUID_API_URL ?? "http://127.0.0.1:8083";

export default defineConfig({
  server: {
    host: "127.0.0.1",
    port: 5173,
    strictPort: true,
    proxy: { "/api": { target: api, ws: true } },
  },
  preview: { host: "127.0.0.1", port: 4173, strictPort: true },
  build: { outDir: "dist", emptyOutDir: true },
});
