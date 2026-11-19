// perf-auditor, cs2 review 1: where does Chromium's sampling heap profiler put the bytes of a small method once its
// caller is optimized and inlines it? And where do a message's deserialized objects land: in the listener that reads
// event.data, or in the method it passes them to? Mirrors allocation.spec.ts's method (samplingInterval 64, collected
// objects included, sum under a function name). Playwright's default launch, no flags. Run:
//   node .gate/perf-auditor-inline.mjs
import { createRequire } from "node:module";
import http from "node:http";
import path from "node:path";

const WT = "/home/user/Shamble-Frontlines/.gate/cs2-plants";
const require = createRequire(path.join(WT, "package.json"));
const { chromium } = require("@playwright/test");

const page = `<!doctype html><title>inline</title><script>
class Link {
  constructor() { this.returned = []; this.posts = 0; this.sink = null; this.tick = 0; this.prev = null; }
  frame(seq, t) { const r = this.returned; this.returned = []; this.sink = { type: "input", seq, t, r }; this.posts += 1; }
  onMessage(data) {
    if (typeof data === "object" && data !== null && data.type === "block" && data.buffer instanceof ArrayBuffer) {
      this.tick = new DataView(data.buffer).getUint32(0, true);
      if (this.prev !== null) this.returned.push(this.prev);
      this.prev = data.buffer;
      if (this.returned.length > 64) this.returned = [];
    }
    return "";
  }
}
window.link = new Link();
const ch = new MessageChannel();
let left = 0, done = null;
ch.port1.onmessage = () => { link.frame(left, performance.now()); if (--left > 0) ch.port2.postMessage(0); else done(); };
window.frames = (n) => new Promise((resolve) => { left = n; done = resolve; ch.port2.postMessage(0); });
const src = "onmessage = (e) => { for (let i = 0; i < e.data; i += 1) { const b = new ArrayBuffer(8); new DataView(b).setUint32(0, i, true); postMessage({ type: 'block', buffer: b, doneAt: performance.timeOrigin + performance.now() }, [b]); } postMessage({ type: 'end' }); };";
const worker = new Worker(URL.createObjectURL(new Blob([src], { type: "text/javascript" })));
let blocksLeft = 0, blocksDone = null;
worker.onmessage = (e) => { const data = e.data; if (data.type === "end") { blocksDone(); return; } link.onMessage(data); };
window.blocks = (n) => new Promise((resolve) => { blocksDone = resolve; worker.postMessage(n); });
</script>`;
const server = http.createServer((req, res) => { res.writeHead(200, { "content-type": "text/html" }); res.end(page); });
const port = await new Promise((resolve) => server.listen(0, "127.0.0.1", () => resolve(server.address().port)));

const total = (n) => n.selfSize + (n.children ?? []).reduce((s, c) => s + total(c), 0);
const under = (n, name) => (n.callFrame.functionName === name ? total(n) : (n.children ?? []).reduce((s, c) => s + under(c, name), 0));
// Self bytes of anonymous functions defined in the page itself (the listeners), outside any named method.
const anonSelf = (n, url) => (n.callFrame.functionName === "" && n.callFrame.url === url ? n.selfSize : 0) + (n.children ?? []).reduce((s, c) => s + anonSelf(c, url), 0);

const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium" });
console.log(`browser ${browser.version()} headless; ${new Date().toISOString()}`);
for (let round = 1; round <= 2; round += 1) {
  const p = await browser.newPage();
  const url = `http://127.0.0.1:${port}/`;
  await p.goto(url);
  const cdp = await p.context().newCDPSession(p);
  await cdp.send("HeapProfiler.enable");
  const sample = async (what, n, warm) => {
    if (warm > 0) await p.evaluate(([w, k]) => window[w](k), [what, warm]);
    await cdp.send("HeapProfiler.startSampling", { samplingInterval: 64, includeObjectsCollectedByMajorGC: true, includeObjectsCollectedByMinorGC: true });
    await p.evaluate(([w, k]) => window[w](k), [what, n]);
    const { profile } = await cdp.send("HeapProfiler.stopSampling");
    const named = what === "frames" ? "frame" : "onMessage";
    console.log(`round ${round} ${what.padEnd(6)} warm-up ${String(warm).padStart(6)} calls, sampled ${n}: under "${named}" ${(under(profile.head, named) / n).toFixed(1)} B per call; page's anonymous listeners (self) ${(anonSelf(profile.head, url) / n).toFixed(1)} B per call`);
  };
  await sample("frames", 300, 0);
  await sample("frames", 20000, 60000);
  await sample("blocks", 300, 0);
  await sample("blocks", 20000, 60000);
  await p.close();
}
await browser.close();
server.close();
