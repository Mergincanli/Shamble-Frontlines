// perf-auditor, cs2 review 1: allocation.spec.ts's exact method on the real page (worktree dist), once after the
// spec's own wait (300 blocks) and once after a long warm-up (WARM_S seconds), to see whether the bytes leave the names
// "frame" and "onMessage" once the page's callbacks tier up. Prints the top self-size functions too. Default launch.
//   node .gate/perf-auditor-alloc-warm.mjs
import { createRequire } from "node:module";
import path from "node:path";
import { pathToFileURL } from "node:url";

const WARM_S = 150;
const WT = "/home/user/Shamble-Frontlines/.gate/cs2-plants";
const require = createRequire(path.join(WT, "package.json"));
const { chromium } = require("@playwright/test");
const dist = path.join(WT, "packages/client/dist");
const { createServer, listen } = await import(pathToFileURL(path.join(dist, "serve.mjs")).href);
const server = createServer({ root: dist });
const port = await listen(server, 0);
const total = (n) => n.selfSize + (n.children ?? []).reduce((s, c) => s + total(c), 0);
const under = (n, name) => (n.callFrame.functionName === name ? total(n) : (n.children ?? []).reduce((s, c) => s + under(c, name), 0));
const selfBy = (n, map = new Map()) => {
  const key = `${n.callFrame.functionName || "(anonymous)"} ${n.callFrame.url.split("/").pop()}:${n.callFrame.lineNumber}:${n.callFrame.columnNumber}`;
  map.set(key, (map.get(key) ?? 0) + n.selfSize);
  for (const c of n.children ?? []) selfBy(c, map);
  return map;
};
const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium" });
console.log(`browser ${browser.version()} headless; ${new Date().toISOString()}`);
const page = await browser.newPage();
await page.goto(`http://127.0.0.1:${port}/?run=perf-alloc-warm`);
await page.waitForFunction(() => { const d = window.__shambleReport?.(); return d !== undefined && d.worker !== null && d.blocks >= 300; }, null, { timeout: 120000 });
const cdp = await page.context().newCDPSession(page);
await cdp.send("HeapProfiler.enable");
const window8s = async (label) => {
  const before = await page.evaluate(() => window.__shambleReport());
  await cdp.send("HeapProfiler.startSampling", { samplingInterval: 64, includeObjectsCollectedByMajorGC: true, includeObjectsCollectedByMinorGC: true });
  await page.waitForTimeout(8000);
  const { profile } = await cdp.send("HeapProfiler.stopSampling");
  const after = await page.evaluate(() => window.__shambleReport());
  const frames = after.frames - before.frames;
  const blocks = after.blocks - before.blocks;
  console.log(`${label}: frames so far ${after.frames}, blocks so far ${after.blocks}; window: ${frames} frames, ${blocks} blocks; under "frame" ${(under(profile.head, "frame") / frames).toFixed(0)} B per frame, under "onMessage" ${(under(profile.head, "onMessage") / blocks).toFixed(0)} B per block; whole page ${(total(profile.head) / 8).toFixed(0)} B/s`);
  const top = [...selfBy(profile.head)].filter(([, v]) => v > 0).sort((a, b) => b[1] - a[1]).slice(0, 8);
  for (const [k, v] of top) console.log(`    ${String(v).padStart(9)} B self  ${k}`);
};
await window8s("after the spec's wait (300 blocks)");
await page.waitForTimeout(WARM_S * 1000);
await window8s(`after ${WARM_S} s more`);
await browser.close();
server.close();
console.log(`finished ${new Date().toISOString()}`);
