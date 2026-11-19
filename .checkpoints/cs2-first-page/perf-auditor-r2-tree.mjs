// perf-auditor, cs2 review round 2: where the per-block bytes sit in the profile tree on the real page, in three
// windows: W0 right after the Worker's first 60 blocks (cold), W1 after 300 blocks (allocation.spec's own point) and W1b
// right after it. For each: bytes under "receive" and its self part, under "onMessage" and how much of that is NOT
// below a "receive" node (receive inlined into the listener), the listener's own bytes, under "frame", and the counts.
// The main folder's dist (the fixes), own server on a free port, Playwright's default launch, no flags of our own.
//   node .gate/perf-auditor-r2-tree.mjs
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
const total = (n) => n.selfSize + (n.children ?? []).reduce((s, c) => s + total(c), 0);
const under = (n, name) => (n.callFrame.functionName === name ? total(n) : (n.children ?? []).reduce((s, c) => s + under(c, name), 0));
const selfOf = (n, name) => (n.callFrame.functionName === name ? n.selfSize : 0) + (n.children ?? []).reduce((s, c) => s + selfOf(c, name), 0);
// "onMessage" bytes whose stack has no "receive" above them.
const orphan = (n, name, guard, inGuard = false) => {
  const g = inGuard || n.callFrame.functionName === guard;
  if (n.callFrame.functionName === name) return g ? 0 : total(n);
  return (n.children ?? []).reduce((s, c) => s + orphan(c, name, guard, g), 0);
};
// The listener: the anonymous function in the page script whose children include receive or onMessage.
const listenerBytes = (n, acc = { self: 0, total: 0, key: "" }) => {
  const kids = n.children ?? [];
  if (n.callFrame.functionName === "" && n.callFrame.url.includes("index-") &&
      (kids.some((c) => c.callFrame.functionName === "receive" || c.callFrame.functionName === "onMessage") || `${n.callFrame.lineNumber}:${n.callFrame.columnNumber}` === "541:48700")) {
    acc.self += n.selfSize; acc.total += total(n); acc.key = `${n.callFrame.lineNumber}:${n.callFrame.columnNumber}`;
  }
  for (const c of kids) listenerBytes(c, acc);
  return acc;
};
const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium" });
console.log(`browser ${browser.version()} headless; started ${new Date().toISOString()}`);
const page = await browser.newPage({ viewport: { width: 1280, height: 720 } });
await page.goto(`http://127.0.0.1:${port}/?run=perf-r2-tree`);
const report = () => page.evaluate(() => window.__shambleReport());
const waitBlocks = (n) => page.waitForFunction((k) => { const d = window.__shambleReport?.(); return d !== undefined && d.worker !== null && d.blocks >= k; }, n, { timeout: 300000, polling: 250 });
const cdp = await page.context().newCDPSession(page);
await cdp.send("HeapProfiler.enable");
const win = async (label) => {
  await cdp.send("HeapProfiler.startSampling", { samplingInterval: 64, includeObjectsCollectedByMajorGC: true, includeObjectsCollectedByMinorGC: true });
  const b = await report();
  await page.waitForTimeout(8000);
  const a = await report();
  const { profile: p } = await cdp.send("HeapProfiler.stopSampling");
  const fr = a.frames - b.frames, bl = a.blocks - b.blocks;
  const L = listenerBytes(p.head);
  const per = (x) => (x / bl).toFixed(1);
  console.log(`${label}: frames ${b.frames}->${a.frames} (${fr}), blocks ${b.blocks}->${a.blocks} (${bl}); per block: under receive ${per(under(p.head, "receive"))} ` +
    `(self ${per(selfOf(p.head, "receive"))}), under onMessage ${per(under(p.head, "onMessage"))} (not below receive ${per(orphan(p.head, "onMessage", "receive"))}), ` +
    `listener ${L.key} self ${per(L.self)} total ${per(L.total)}; per frame under frame ${(under(p.head, "frame") / fr).toFixed(1)}`);
};
await waitBlocks(60);
await win("W0 after 60 blocks");
await waitBlocks(300);
await win("W1 after 300 blocks");
await win("W1b right after");
await browser.close();
server.close();
console.log(`finished ${new Date().toISOString()}`);
