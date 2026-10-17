/**
 * Vite build for the ImageSquid distribution. Ports: 5174 (dev) and 4174 (preview).
 * In dev, `/api` is proxied to `filesquid-server` (FILESQUID_API_URL, default
 * http://127.0.0.1:8083), WebSocket included.
 */
import { defineConfig } from "vite";

const api = process.env.FILESQUID_API_URL ?? "http://127.0.0.1:8083";

export default defineConfig({
  server: {
    host: "127.0.0.1",
    port: 5174,
    strictPort: true,
    proxy: { "/api": { target: api, ws: true } },
  },
  preview: { host: "127.0.0.1", port: 4174, strictPort: true },
  build: { outDir: "dist", emptyOutDir: true },
});
