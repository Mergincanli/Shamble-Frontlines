// perf-auditor, cs2 review 1: what one readout update costs in JS on the real page (worktree dist): the time of the
// page's own report function (__shambleReport = boot.ts's data(): 5 percentile sorts, runHeader, counters), and a bare
// copy-and-sort of 600 doubles as Ring.percentile does it, in Chromium's V8. Default launch.
//   node .gate/perf-auditor-readout.mjs
import { createRequire } from "node:module";
import path from "node:path";
import { pathToFileURL } from "node:url";

const WT = "/home/user/Shamble-Frontlines/.gate/cs2-plants";
const require = createRequire(path.join(WT, "package.json"));
const { chromium } = require("@playwright/test");
const dist = path.join(WT, "packages/client/dist");
const { createServer, listen } = await import(pathToFileURL(path.join(dist, "serve.mjs")).href);
const server = createServer({ root: dist });
const port = await listen(server, 0);
const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium" });
console.log(`browser ${browser.version()} headless; ${new Date().toISOString()}`);
const page = await browser.newPage();
await page.goto(`http://127.0.0.1:${port}/?run=perf-readout`);
await page.waitForFunction(() => { const d = window.__shambleReport?.(); return d !== undefined && d.worker !== null && d.frames >= 60 && d.blocks >= 600; }, null, { timeout: 180000 });
const r = await page.evaluate(() => {
  const time = (n, f) => { const runs = []; for (let k = 0; k < 7; k += 1) { const t0 = performance.now(); for (let i = 0; i < n; i += 1) f(); runs.push(((performance.now() - t0) * 1000) / n); } runs.sort((a, b) => a - b); return runs; };
  for (let i = 0; i < 50; i += 1) window.__shambleReport();
  const report = time(100, () => window.__shambleReport());
  const values = new Float64Array(600), sorted = new Float64Array(600);
  for (let i = 0; i < 600; i += 1) values[i] = 16.6 + Math.random();
  let sink = 0;
  const sort = time(2000, () => { const held = sorted.subarray(0, 600); held.set(values.subarray(0, 600)); held.sort(); sink += held[594]; });
  const d = window.__shambleReport();
  return { report, sort, sink: sink > 0, frames: d.frames, fps: d.fps };
});
const f = (runs) => `median ${runs[3].toFixed(1)} us (min ${runs[0].toFixed(1)}, max ${runs[6].toFixed(1)})`;
console.log(`__shambleReport() (boot.ts data()): ${f(r.report)} per call; copy+sort of 600 doubles: ${f(r.sort)}; page at ${r.fps.toFixed(1)} fps after ${r.frames} frames`);
await browser.close();
server.close();
