// The isolation flag (ADR 0002 part 5; PM-0002-20): cross-origin isolation is on when a run asks for it and off
// otherwise, on the page and in the Worker, under Vite's dev server and its preview server. Chromium's 4 of the 12
// cases; Firefox and WebKit run in CI later.
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
        await page.goto(address ?? "");
        await running(page, 30);
        const data = await report(page);
        expect(await page.evaluate(() => crossOriginIsolated)).toBe(flag);
        expect(data.header.crossOriginIsolated).toBe(flag);
        expect(data.worker?.crossOriginIsolated).toBe(flag);
        expect(data.errors).toBe(0);
      } finally {
        await server?.close();
        restore("SHAMBLE_ISOLATION", before[0]);
        restore("VITE_SHAMBLE_DEVTOOLS", before[1]);
      }
    });
  }
}
