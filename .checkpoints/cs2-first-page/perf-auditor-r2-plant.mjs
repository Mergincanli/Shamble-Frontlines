// perf-auditor, cs2 review round 2: how often allocation.spec's per-block check sees plant 7b (plants.py line 60-62:
// `this.scratch = new Uint8Array(64)` in WorkerLink.receive, a `scratch = null` field) on this container, against the
// clean page. Clean = the main folder's dist (built 08:15:18Z from ae47239, the d257bb6 code); plant = a copy of it in
// .gate/perf-auditor-r2-dist-7b with exactly that edit made in the bundle (no rebuild). Each run: a fresh context, the
// spec's flow (wait for 300 blocks, then an 8 s window with report() inside it), plus a cold window first (W0 from ~60
// blocks) to read the cost before the listener tiers up. Alternating clean / plant, RUNS times each.
// Own servers on free ports, Playwright's default launch, no flags of our own.   node .gate/perf-auditor-r2-plant.mjs
import { createRequire } from "node:module";
import path from "node:path";
import { pathToFileURL } from "node:url";

const ROOT = "/home/user/Shamble-Frontlines";
const RUNS = Number(process.env.RUNS ?? 3);
const require = createRequire(path.join(ROOT, "package.json"));
const { chromium } = require("@playwright/test");
const dists = { clean: path.join(ROOT, "packages/client/dist"), plant: path.join(ROOT, ".gate/perf-auditor-r2-dist-7b") };
const { createServer, listen } = await import(pathToFileURL(path.join(dists.clean, "serve.mjs")).href);
const ports = {};
const servers = [];
for (const [k, root] of Object.entries(dists)) {
  const s = createServer({ root });
  servers.push(s);
  ports[k] = await listen(s, 0);
}
const total = (n) => n.selfSize + (n.children ?? []).reduce((s, c) => s + total(c), 0);
const under = (n, name) => (n.callFrame.functionName === name ? total(n) : (n.children ?? []).reduce((s, c) => s + under(c, name), 0));
const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium" });
console.log(`browser ${browser.version()} headless; started ${new Date().toISOString()}; ports ${JSON.stringify(ports)}`);
for (let run = 1; run <= RUNS; run += 1) {
  for (const kind of ["clean", "plant"]) {
    const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
    const page = await context.newPage();
    await page.goto(`http://127.0.0.1:${ports[kind]}/?run=perf-r2-${kind}`);
    const report = () => page.evaluate(() => window.__shambleReport());
    const waitBlocks = (n) => page.waitForFunction((k) => { const d = window.__shambleReport?.(); return d !== undefined && d.worker !== null && d.blocks >= k; }, n, { timeout: 300000, polling: 250 });
    const cdp = await context.newCDPSession(page);
    await cdp.send("HeapProfiler.enable");
    const win = async (ms) => {
      await cdp.send("HeapProfiler.startSampling", { samplingInterval: 64, includeObjectsCollectedByMajorGC: true, includeObjectsCollectedByMinorGC: true });
      const b = await report();
      await page.waitForTimeout(ms);
      const a = await report();
      const { profile } = await cdp.send("HeapProfiler.stopSampling");
      return { frames: a.frames - b.frames, blocks: a.blocks - b.blocks, from: b.blocks, to: a.blocks, f: under(profile.head, "frame"), r: under(profile.head, "receive") };
    };
    await waitBlocks(60);
    const w0 = await win(6000);
    await waitBlocks(300);
    const w1 = await win(8000);
    const perFrame = w1.f / w1.frames, perBlock = w1.r / w1.blocks;
    const verdict = perFrame > 25 && perFrame < 170 && perBlock > 150 && perBlock < 390 ? "pass" : `FAIL (${[perFrame <= 25 && "frame floor", perFrame >= 170 && "frame limit", perBlock <= 150 && "block floor", perBlock >= 390 && "block limit"].filter(Boolean).join(", ")})`;
    console.log(`run ${run} ${kind}: W0 blocks ${w0.from}->${w0.to} ${(w0.r / w0.blocks).toFixed(1)} B/block, ${(w0.f / w0.frames).toFixed(1)} B/frame | ` +
      `W1 (the spec's) blocks ${w1.from}->${w1.to} ${perBlock.toFixed(1)} B/block, ${perFrame.toFixed(1)} B/frame (${w1.frames} frames) -> spec ${verdict}; ` +
      `visible fraction vs W0 ${(perBlock / (w0.r / w0.blocks)).toFixed(2)}`);
    await context.close();
  }
}
await browser.close();
for (const s of servers) s.close();
console.log(`finished ${new Date().toISOString()}`);
