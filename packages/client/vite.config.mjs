// The build and dev-server settings of the player's page (ADR 0002 parts 2 and 5).
import { closeSync, constants, fstatSync, mkdirSync, openSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

const folder = (relative) => fileURLToPath(new URL(relative, import.meta.url));

/** What the development server answers for `/content.json` when the content tool refused or anything failed. */
const CONTENT_REFUSED = "the content tool refused the content: its words are in the dev server's output\n";

/**
 * The development server's `/content.json` (engine-architect's block 1.9, its serving part only: no watch, no
 * reload). A built page carries the bundle beside it; the development server serves the sources, and would answer
 * the address with the page itself. So on each request the build's own function writes the bundle (scripts/build.mjs,
 * `writeContentBundle`: the same content tool, cargo call, environment and target folder as a build, so after a build
 * it compiles nothing) into the build's fixed folder for it, outside every folder a server serves, and the file is
 * handed out whole. `apply: "serve"` keeps it out of every build, and Vite calls `configureServer` for the development
 * server alone: the preview server serves a built folder and its own bundle.
 */
const devContent = {
  name: "shamble-dev-content",
  apply: "serve",
  configureServer(server) {
    // Added here, not in a returned function, so it runs before Vite's own handlers and their page-for-any-address.
    server.middlewares.use((request, response, next) => {
      if (request.method !== "GET" || new URL(request.url ?? "/", "http://127.0.0.1").pathname !== "/content.json") {
        next();
        return;
      }
      serveContent(response).catch(() => undefined);
    });
  },
};

async function serveContent(response) {
  try {
    const { DEV_CONTENT_DIR, writeContentBundle } = await import(new URL("./scripts/build.mjs", import.meta.url).href);
    mkdirSync(DEV_CONTENT_DIR, { recursive: true });
    writeContentBundle(DEV_CONTENT_DIR);
    // The one plain file the tool wrote, never through a link.
    const handle = openSync(path.join(DEV_CONTENT_DIR, "content.json"), constants.O_RDONLY | constants.O_NOFOLLOW);
    try {
      if (!fstatSync(handle).isFile()) throw new Error("content.json is not a plain file");
      const bytes = readFileSync(handle);
      response.writeHead(200, { "Content-Type": "application/json", "X-Content-Type-Options": "nosniff" });
      response.end(bytes);
    } finally {
      closeSync(handle);
    }
  } catch (error) {
    // The tool's own words went to this server's output already; the page gets the fixed sentence.
    console.error(`the dev server's /content.json: ${error instanceof Error ? error.message : String(error)}`);
    response.writeHead(500, { "Content-Type": "text/plain; charset=utf-8", "X-Content-Type-Options": "nosniff" });
    response.end(CONTENT_REFUSED);
  }
}

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
  plugins: [devContent],
  // Both servers answer on this computer only, never on the network (the dependency answer's condition), and the
  // dev server serves only the folders the page loads from, not the whole repository: its own, of the Worker's
  // package the sources and the players' Wasm, the shared types, and the one library. Not node_modules as a whole:
  // npm links every package of this repository into it, the Worker's whole folder among them. The test-only module
  // is built outside all of these (.gate/rules-module).
  server: {
    host: "127.0.0.1",
    headers: isolation,
    fs: { allow: [folder("."), folder("../sim-worker/src"), folder("../sim-worker/pkg"), folder("../types"), folder("../../node_modules/three")] },
  },
  preview: { host: "127.0.0.1", headers: isolation },
});
