// perf-auditor, cs2 review 1, part 2: (a) after how many calls does a small method's allocation leave its own name in
// Chromium's sampling heap profile (inlined into its caller)? (b) does the profile see an off-heap typed array's
// backing store (4096 bytes) or only its wrapper? Same method as allocation.spec.ts. Default launch, no flags.
//   node .gate/perf-auditor-inline2.mjs
import { createRequire } from "node:module";
import http from "node:http";
import path from "node:path";

const WT = "/home/user/Shamble-Frontlines/.gate/cs2-plants";
const require = createRequire(path.join(WT, "package.json"));
const { chromium } = require("@playwright/test");

const pageFor = (extra) => `<!doctype html><title>inline2</title><script>
class Link {
  constructor() { this.returned = []; this.posts = 0; this.sink = null; this.scratch = null; }
  frame(seq, t) { const r = this.returned; this.returned = []; this.sink = { type: "input", seq, t, r }; this.posts += 1; ${extra} }
}
window.link = new Link();
const ch = new MessageChannel();
let left = 0, done = null;
function onFrame() { link.frame(left, performance.now()); if (--left > 0) ch.port2.postMessage(0); else done(); }
ch.port1.onmessage = onFrame;
window.frames = (n) => new Promise((resolve) => { left = n; done = resolve; ch.port2.postMessage(0); });
</script>`;
const variants = { clean: "", plant64: "this.scratch = new Uint8Array(64);", plant4096: "this.scratch = new Uint8Array(4096);" };
const server = http.createServer((req, res) => {
  const v = new URL(req.url, "http://x").searchParams.get("v") ?? "clean";
  res.writeHead(200, { "content-type": "text/html" });
  res.end(pageFor(variants[v] ?? ""));
});
const port = await new Promise((resolve) => server.listen(0, "127.0.0.1", () => resolve(server.address().port)));
const total = (n) => n.selfSize + (n.children ?? []).reduce((s, c) => s + total(c), 0);
const under = (n, name) => (n.callFrame.functionName === name ? total(n) : (n.children ?? []).reduce((s, c) => s + under(c, name), 0));
const self = (n, name) => (n.callFrame.functionName === name ? n.selfSize : 0) + (n.children ?? []).reduce((s, c) => s + self(c, name), 0);

const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium" });
console.log(`browser ${browser.version()} headless; ${new Date().toISOString()}`);
const run = async (variant, warm, n) => {
  const p = await browser.newPage();
  await p.goto(`http://127.0.0.1:${port}/?v=${variant}`);
  const cdp = await p.context().newCDPSession(p);
  await cdp.send("HeapProfiler.enable");
  if (warm > 0) await p.evaluate((k) => window.frames(k), warm);
  await cdp.send("HeapProfiler.startSampling", { samplingInterval: 64, includeObjectsCollectedByMajorGC: true, includeObjectsCollectedByMinorGC: true });
  await p.evaluate((k) => window.frames(k), n);
  const { profile } = await cdp.send("HeapProfiler.stopSampling");
  console.log(`${variant.padEnd(9)} warm-up ${String(warm).padStart(5)} calls, sampled ${n}: under "frame" ${(under(profile.head, "frame") / n).toFixed(1)} B per call; caller "onFrame" self ${(self(profile.head, "onFrame") / n).toFixed(1)} B per call`);
  await p.close();
};
for (const warm of [0, 300, 1000, 2000, 5000, 10000]) await run("clean", warm, 2000);
for (const warm of [0, 300, 1000, 2000, 5000, 10000]) await run("plant64", warm, 2000);
await run("plant4096", 0, 2000);
await run("plant4096", 10000, 2000);
await browser.close();
server.close();
