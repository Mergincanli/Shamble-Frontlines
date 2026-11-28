// The isolation flag (ADR 0002 part 5; PM-0002-20): cross-origin isolation is on when a run asks for it and off
// otherwise, on the page and in the Worker, under Vite's dev server and its preview server. Chromium's 4 of the 12
// cases; Firefox and WebKit run in CI later.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { expect, test } from "@playwright/test";
import { createServer, preview } from "vite";
import { needsBuild, report, running } from "./report.ts";

const client = fileURLToPath(new URL("../../../client", import.meta.url));
const settings = { root: client, configFile: path.join(client, "vite.config.mjs"), logLevel: "silent" } as const;

/** Puts a variable of the test process back as it was: a spec that runs later reads its own environment, not this one's. */
function restore(name: string, value: string | undefined): void {
  if (value === undefined) delete process.env[name];
  else process.env[name] = value;
}

for (const kind of ["dev server", "preview server"] as const) {
  for (const flag of [true, false]) {
    test(`with the flag ${flag ? "on" : "off"} under Vite's ${kind}, crossOriginIsolated is ${flag} on the page and in the Worker`, async ({ page }) => {
      const dist = needsBuild();
      const before = [process.env["SHAMBLE_ISOLATION"], process.env["VITE_SHAMBLE_DEVTOOLS"]] as const;
      // The config reads the flag when it is loaded; the development page is what has the report.
      process.env["SHAMBLE_ISOLATION"] = flag ? "1" : "0";
      process.env["VITE_SHAMBLE_DEVTOOLS"] = "1";
      let server: Awaited<ReturnType<typeof preview>> | Awaited<ReturnType<typeof createServer>> | undefined;
      try {
        server =
          kind === "dev server"
            ? await (await createServer(settings)).listen()
            : await preview({ ...settings, build: { outDir: dist } });
        const address = server.resolvedUrls?.local[0];
        expect(address).toContain("127.0.0.1");
        if (kind === "dev server") {
          // First the game's content, which the page fetches before its Worker starts: the bundle the build's own
          // content tool writes (vite.config.mjs's plugin), byte for byte the built page's. (This server answers an
          // address it has no file for with the page itself, so the type is asked too.)
          const bundle = await page.request.get(`${address ?? ""}content.json`);
          expect([bundle.status(), bundle.headers()["content-type"]]).toEqual([200, "application/json"]);
          expect(new Uint8Array(await bundle.body())).toEqual(new Uint8Array(readFileSync(path.join(dist, "content.json"))));
          // Another site's ask, as a browser marks it, starts no content tool: refused in the server's own words, from
          // another site and from another server of this machine (`same-site`) alike.
          for (const from of ["cross-site", "same-site"]) {
            const asked = await page.request.get(`${address ?? ""}content.json`, { headers: { "Sec-Fetch-Site": from } });
            expect([asked.status(), await asked.text()], from).toEqual([403, "the content is served to this page alone\n"]);
          }
          // An address that reads as no path goes on to Vite's own handlers, never to an error page of the plugin's.
          expect((await page.request.get(`${address ?? ""}/`)).status()).not.toBe(500);
        }
        await page.goto(address ?? "");
        await running(page, 30);
        const data = await report(page);
        expect(await page.evaluate(() => crossOriginIsolated)).toBe(flag);
        expect(data.header.crossOriginIsolated).toBe(flag);
        expect(data.worker?.crossOriginIsolated).toBe(flag);
        expect(data.errors).toBe(0);
        if (kind === "dev server") {
          // The dev server reads files outside the page's folder by their full path. It hands out the players' Wasm,
          // which the Worker loads. It refuses the test-only module where the build puts it, outside every package.
          // And it refuses the Worker's package reached through npm's link in node_modules: that link would open
          // the package's whole folder, whatever is built into it.
          const at = (file: string): string => `${address ?? ""}@fs${path.join(client, "../..", file)}`;
          // (For a file that is not there this server answers 200 with the page itself, so the file's type is asked too.)
          const players = await page.request.get(at("packages/sim-worker/pkg/sim_wasm_bg.wasm"));
          expect([players.status(), players.headers()["content-type"]]).toEqual([200, "application/wasm"]);
          // The folder the content plugin writes its bundle into is not served.
          expect((await page.request.get(at(".gate/dev-content/content.json"))).status()).toBe(403);
          expect((await page.request.get(at(".gate/rules-module/release/devtools_bg.wasm"))).status()).toBe(403);
          expect((await page.request.get(at(".gate/rules-module/dev/devtools.js"))).status()).toBe(403);
          expect((await page.request.get(at("node_modules/@shamble/sim-worker/package.json"))).status()).toBe(403);
          expect((await page.request.get(at("node_modules/@shamble/sim-worker/pkg/sim_wasm_bg.wasm"))).status()).toBe(403);
          // Where earlier builds put the module: a copy may still lie there on a developer's machine. With or without
          // one, what comes back is no WebAssembly: a refusal, or the page itself for a file that is not there.
          const old = await page.request.get(at("packages/sim-worker/pkg-rules/release/devtools_bg.wasm"));
          expect(old.headers()["content-type"] ?? "").not.toContain("wasm");
        }
      } finally {
        await server?.close();
        restore("SHAMBLE_ISOLATION", before[0]);
        restore("VITE_SHAMBLE_DEVTOOLS", before[1]);
      }
    });
  }
}
