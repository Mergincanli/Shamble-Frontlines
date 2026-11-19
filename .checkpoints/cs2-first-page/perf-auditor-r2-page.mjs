// perf-auditor, cs2 review round 2: the real page after the fix pass (the main folder's dist, built 07:50:27Z from
// 81e916d + the d257bb6 changes: Math.ceil in the host, receive(), no viewTime). Its own server on a free port (never
// 4173), Playwright's default launch, no flags of our own. It reads:
//  W1: allocation.spec's exact method (report() inside the sampled window) right after 300 blocks;
//  P1: the Worker's pacing after 1,200 blocks (wakes per tick, lateness from the Worker's 600-tick ring);
//  W2: the same allocation window after a long warm-up: do the floors (25 B per frame, 150 B per block) trip once the
//      callers tier up, or does the method pass blind?
//  plus console messages over the run (three's console hook, three-picture.ts:27-30).
//   node .gate/perf-auditor-r2-page.mjs
import { createRequire } from "node:module";
import path from "node:path";
import { pathToFileURL } from "node:url";

const ROOT = "/home/user/Shamble-Frontlines";
const WARM_TOTAL_S = Number(process.env.WARM_TOTAL_S ?? 170);
const require = createRequire(path.join(ROOT, "package.json"));
const { chromium } = require("@playwright/test");
const dist = path.join(ROOT, "packages/client/dist");
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
const t0 = Date.now();
const el = () => ((Date.now() - t0) / 1000).toFixed(0);
console.log(`browser ${browser.version()} headless; port ${port}; started ${new Date().toISOString()}`);
const page = await browser.newPage({ viewport: { width: 1280, height: 720 } });
const consoleByType = new Map();
page.on("console", (m) => consoleByType.set(m.type(), (consoleByType.get(m.type()) ?? 0) + 1));
let pageErrors = 0;
page.on("pageerror", () => (pageErrors += 1));
await page.goto(`http://127.0.0.1:${port}/?run=perf-r2`);
const report = () => page.evaluate(() => window.__shambleReport());
const waitBlocks = (n, timeout = 300000) =>
  page.waitForFunction((k) => { const d = window.__shambleReport?.(); return d !== undefined && d.worker !== null && d.blocks >= k; }, n, { timeout, polling: 500 });
const cdp = await page.context().newCDPSession(page);
await cdp.send("HeapProfiler.enable");
const allocWindow = async (label) => {
  await cdp.send("HeapProfiler.startSampling", { samplingInterval: 64, includeObjectsCollectedByMajorGC: true, includeObjectsCollectedByMinorGC: true });
  const before = await report();
  await page.waitForTimeout(8000);
  const after = await report();
  const { profile } = await cdp.send("HeapProfiler.stopSampling");
  const frames = after.frames - before.frames;
  const blocks = after.blocks - before.blocks;
  const f = under(profile.head, "frame");
  const r = under(profile.head, "receive");
  const o = under(profile.head, "onMessage");
  console.log(`${label} [t=${el()} s]: frames so far ${after.frames}, blocks so far ${after.blocks}; window ${frames} frames, ${blocks} blocks; ` +
    `under "frame" ${(f / frames).toFixed(1)} B/frame (floor 25, limit 170), under "receive" ${(r / blocks).toFixed(1)} B/block (floor 150, limit 390), ` +
    `under "onMessage" ${(o / blocks).toFixed(1)} B/block; whole page ${(total(profile.head) / 8).toFixed(0)} B/s; ` +
    `spec verdict: ${f / frames > 25 && f / frames < 170 && r / blocks > 150 && r / blocks < 390 ? "pass" : "FAIL"}`);
  const top = [...selfBy(profile.head)].filter(([, v]) => v > 0).sort((a, b) => b[1] - a[1]).slice(0, 10);
  for (const [k, v] of top) console.log(`    ${String(v).padStart(9)} B self  ${k}`);
};
const pacing = async (label) => {
  const d = await report();
  const w = d.worker;
  console.log(`${label} [t=${el()} s]: ticks ${w.ticks}, wakes ${w.wakes} (${(w.wakes / w.ticks).toFixed(3)} per tick; ${w.wakes - w.ticks} more wakes than ticks), ` +
    `most in a wake ${w.maxTicksInOneWake}, drops ${w.drops}, pauses ${w.pauses}; lateness (last 600 ticks) p50 ${w.latenessP50.toFixed(2)} p99 ${w.latenessP99.toFixed(2)} max ${w.latenessMax.toFixed(2)} ms; ` +
    `copy p50 ${w.copyP50.toFixed(3)} p99 ${w.copyP99.toFixed(3)} ms; pool misses ${w.poolMisses}; fps ${d.fps.toFixed(1)}, frame p50 ${d.frameP50.toFixed(1)} p99 ${d.frameP99.toFixed(1)} ms; ` +
    `delivery p50 ${d.deliveryP50.toFixed(1)} p95 ${d.deliveryP95.toFixed(1)} ms; posts ${d.posts}, frames ${d.frames}; errors ${d.errors}`);
};
await waitBlocks(300);
await allocWindow("W1 after 300 blocks");
await waitBlocks(1200);
await pacing("P1 after 1,200 blocks");
const restMs = WARM_TOTAL_S * 1000 - (Date.now() - t0);
if (restMs > 0) await page.waitForTimeout(restMs);
await allocWindow(`W2 after ~${WARM_TOTAL_S} s`);
await pacing("P2 at the end");
console.log(`console messages by type over the run: ${JSON.stringify(Object.fromEntries(consoleByType))}; page errors ${pageErrors}`);
await browser.close();
server.close();
console.log(`finished ${new Date().toISOString()}`);
