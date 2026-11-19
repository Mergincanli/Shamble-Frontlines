// perf-auditor, cs2 review round 2: the console messages of the real page (main folder's dist) over ~12 s, with their
// text and source, to see whether three's console hook (three-picture.ts:27-30) is on any per-frame path and what the
// one console error is. Own server on a free port, Playwright's default launch.   node .gate/perf-auditor-r2-console.mjs
import { createRequire } from "node:module";
import path from "node:path";
import { pathToFileURL } from "node:url";

const ROOT = "/home/user/Shamble-Frontlines";
const require = createRequire(path.join(ROOT, "package.json"));
const { chromium } = require("@playwright/test");
const dist = path.join(ROOT, "packages/client/dist");
const { createServer, listen } = await import(pathToFileURL(path.join(dist, "serve.mjs")).href);
const server = createServer({ root: dist });
const port = await listen(server, 0);
const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium" });
const page = await browser.newPage({ viewport: { width: 1280, height: 720 } });
const t0 = Date.now();
page.on("console", (m) => console.log(`[${((Date.now() - t0) / 1000).toFixed(1)} s] console.${m.type()}: ${m.text().slice(0, 220)} @ ${m.location().url.split("/").pop()}:${m.location().lineNumber}`));
page.on("requestfailed", (r) => console.log(`request failed: ${r.url()} ${r.failure()?.errorText}`));
page.on("response", (r) => { if (r.status() >= 400) console.log(`response ${r.status()}: ${r.url()}`); });
await page.goto(`http://127.0.0.1:${port}/?run=perf-r2-console`);
await page.waitForTimeout(12000);
const d = await page.evaluate(() => window.__shambleReport());
console.log(`after 12 s: frames ${d.frames}, blocks ${d.blocks}, errors ${d.errors}, last error "${d.lastError}"`);
await browser.close();
server.close();
