// The build and dev-server settings of the player's page (ADR 0002 parts 2 and 5).
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

const folder = (relative) => fileURLToPath(new URL(relative, import.meta.url));

// The isolation flag (ADR 0002 part 5): cross-origin isolation only when a profiling run asks for it, and only on
// the dev and preview servers. Both headers, or neither.
const isolation =
  process.env["SHAMBLE_ISOLATION"] === "1"
    ? { "Cross-Origin-Opener-Policy": "same-origin", "Cross-Origin-Embedder-Policy": "require-corp" }
    : {};

export default defineConfig({
  // Relative addresses, so the built page works from any folder a server serves.
  base: "./",
  // The build's two constants, written into the code as literals (scripts/build.mjs sets the two variables). A
  // literal `false` lets the bundler cut the development-only code out of a production build.
  define: {
    __SHAMBLE_DEVTOOLS__: JSON.stringify(process.env["VITE_SHAMBLE_DEVTOOLS"] === "1"),
    __SHAMBLE_COMMIT__: JSON.stringify(process.env["VITE_SHAMBLE_COMMIT"] ?? "unknown"),
  },
  build: {
    // No file is ever inlined: the Wasm and the Worker load as files, never from data: URLs (CSP-ready).
    assetsInlineLimit: 0,
  },
  // A module Worker (Vite's default would be a classic script).
  worker: { format: "es" },
  // Both servers answer on this computer only, never on the network (the dependency answer's condition), and the
  // dev server serves only the folders the page loads from, not the whole repository.
  server: {
    host: "127.0.0.1",
    headers: isolation,
    fs: { allow: [folder("."), folder("../sim-worker"), folder("../types"), folder("../../node_modules")] },
  },
  preview: { host: "127.0.0.1", headers: isolation },
});
