// The production flavour holds none of the development-only code (build plan, Part 0: each development tool "is
// kept out of the production build"). A scan of every built file for markers that only the readout, the copy
// report, the run header and the test hook have, with the development build as the control, and one look at the
// production page in Chromium.   node .checkpoints/cs2-first-page/production-check.mjs <dev dist> <prod dist>
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { createRequire } from "node:module";
import path from "node:path";
import { pathToFileURL } from "node:url";
const [dev, prod] = process.argv.slice(2).map((p) => path.resolve(p));
const MARKERS = ["Copy report", "__shambleReport", "ticks while hidden", "first page: report", "frame time", "colorDepth"];
const files = (dir) => readdirSync(dir).flatMap((n) => (statSync(path.join(dir, n)).isDirectory() ? files(path.join(dir, n)) : [path.join(dir, n)]));
const scan = (dir) => Object.fromEntries(MARKERS.map((m) => [m, files(dir).filter((f) => readFileSync(f).includes(m)).map((f) => path.relative(dir, f))]));
let bad = 0;
for (const [name, dir, wanted] of [["development (the control)", dev, true], ["production", prod, false]]) {
  console.log(`${name}: ${files(dir).length} files; serve.mjs ${existsSync(path.join(dir, "serve.mjs")) ? "present" : "absent"}`);
  if (existsSync(path.join(dir, "serve.mjs")) !== wanted) bad += 1;
  for (const [marker, found] of Object.entries(scan(dir))) {
    console.log(`  ${JSON.stringify(marker)}: ${found.length === 0 ? "not found" : `in ${found.join(", ")}`}`);
    if ((found.length > 0) !== wanted) bad += 1;
  }
}
const require = createRequire(path.resolve("package.json"));
const { chromium } = require("@playwright/test");
const { createServer, listen } = await import(pathToFileURL(path.resolve("packages/client/serve.mjs")).href);
const server = createServer({ root: prod });
const port = await listen(server, 0);
const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium" });
const page = await browser.newPage();
const problems = [];
page.on("pageerror", (e) => problems.push(e.message));
page.on("console", (m) => { if (m.type() === "error" && !m.text().includes("404")) problems.push(m.text()); });
await page.goto(`http://127.0.0.1:${port}/`);
await page.waitForFunction(() => document.documentElement.dataset.backend !== undefined, null, { timeout: 30000 });
await page.waitForTimeout(1500);
const seen = await page.evaluate(() => ({ backend: document.documentElement.dataset.backend, commit: document.documentElement.dataset.commit,
  overlay: document.querySelectorAll("#overlay").length, hook: typeof window.__shambleReport, noticeHidden: document.getElementById("notice")?.hidden }));
console.log(`the production page in Chromium: ${JSON.stringify(seen)}; Workers: ${page.workers().length}; page errors: ${problems.length}`);
if (seen.overlay !== 0 || seen.hook !== "undefined" || page.workers().length !== 1 || problems.length > 0 || seen.noticeHidden !== true) bad += 1;
await browser.close(); server.close();
console.log(bad === 0 ? "PASS: the production build holds no development marker, and the control finds each" : `FAIL: ${bad} problems`);
process.exit(bad === 0 ? 0 : 1);
