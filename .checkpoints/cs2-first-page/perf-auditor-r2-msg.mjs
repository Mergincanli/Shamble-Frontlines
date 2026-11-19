// perf-auditor, cs2 review round 2: what one Worker message costs the page under the name "receive", by message kind,
// to see how much of allocation.spec's "per block" figure is the readout's report messages (about 3 to 4 a second,
// frame-rate dependent) rather than blocks. A mirror of WorkerLink.receive/onMessage/#takeBlock (worker-link.ts at
// d257bb6) on a blank page; the Worker posts one kind at 30 Hz: blocks ({type, buffer (8 B, transferred), doneAt}) or
// reports (host.ts stats()'s 17 fields, same order and kinds of value). Cold: 8 s windows (~240 calls).
// Own server on a free port, Playwright's default launch, no flags of our own.   node .gate/perf-auditor-r2-msg.mjs
import { createRequire } from "node:module";
import http from "node:http";
import path from "node:path";

const ROOT = "/home/user/Shamble-Frontlines";
const require = createRequire(path.join(ROOT, "package.json"));
const { chromium } = require("@playwright/test");
const workerSource = `
let n = 0;
onmessage = (e) => {
  const kind = e.data;
  setInterval(() => {
    n += 1;
    if (kind === "block") {
      const buffer = new ArrayBuffer(8);
      new DataView(buffer).setUint32(0, n, true);
      postMessage({ type: "block", buffer, doneAt: performance.timeOrigin + performance.now() }, [buffer]);
    } else {
      const counters = { ticks: 1000 + n, wakes: 1001 + n, maxTicksInOneWake: 1, drops: 0, droppedMs: 0, pauses: 0 };
      postMessage({ type: "report", stats: { ...counters, latenessP50: 0.8 + (n % 7) / 10, latenessP99: 4.3 + (n % 3) / 10,
        latenessMax: 8.4, copyP50: 0, copyP99: 0.1, blockBytes: 8, poolMisses: 353 + n, poolRefused: 0, undetached: 0,
        hardwareConcurrency: 4, crossOriginIsolated: false } }, []);
    }
  }, 1000 / 30);
};
`;
const pageScript = `
class Ring { constructor(c) { this.v = new Float64Array(c); this.i = 0; this.count = 0; } push(x) { this.v[this.i] = x; this.i = (this.i + 1) % this.v.length; this.count += 1; } }
class Link {
  #delivery = new Ring(600); #newest = null; #previous = null; #returned = []; #layoutChecked = true;
  ready = true; blockBytes = 8; tickHz = 30; blocks = 0; tick = 0; failure = ""; stats = null; messages = 0;
  receive(event) { this.messages += 1; return this.onMessage(event.data); }
  onMessage(data) {
    if (this.failure !== "") return "";
    if (typeof data !== "object" || data === null || !("type" in data)) return this.#fail("no message");
    const message = data;
    if (message.type === "block" && message["buffer"] instanceof ArrayBuffer && typeof message["doneAt"] === "number") return this.#takeBlock(message["buffer"], message["doneAt"]);
    if (message.type === "report" && typeof message["stats"] === "object" && message["stats"] !== null) { this.stats = message["stats"]; return ""; }
    return this.#fail("unknown");
  }
  #takeBlock(buffer, doneAt) {
    if (buffer.byteLength !== this.blockBytes) return this.#fail("length");
    this.#delivery.push(performance.timeOrigin + performance.now() - doneAt);
    if (this.#previous !== null) this.#returned.push(this.#previous);
    this.#previous = this.#newest; this.#newest = buffer; this.blocks += 1;
    this.tick = new DataView(buffer).getUint32(0, true);
    if (this.#returned.length > 2) this.#returned.length = 0;
    return "";
  }
  #fail(r) { this.failure = r; return r; }
}
window.__start = (src, kind) => {
  const link = new Link();
  window.__link = link;
  const w = new Worker(URL.createObjectURL(new Blob([src], { type: "text/javascript" })));
  w.addEventListener("message", (event) => { const p = link.receive(event); if (p !== "") console.log(p); });
  w.postMessage(kind);
};
`;
const total = (n) => n.selfSize + (n.children ?? []).reduce((s, c) => s + total(c), 0);
const under = (n, name) => (n.callFrame.functionName === name ? total(n) : (n.children ?? []).reduce((s, c) => s + under(c, name), 0));
const selfOf = (n, name) => (n.callFrame.functionName === name ? n.selfSize : 0) + (n.children ?? []).reduce((s, c) => s + selfOf(c, name), 0);
const blank = http.createServer((req, res) => {
  res.writeHead(200, { "content-type": "text/html" });
  res.end(`<!doctype html><title>msg</title><script>${pageScript}</script><p>msg</p>`);
});
const port = await new Promise((resolve) => blank.listen(0, "127.0.0.1", () => resolve(blank.address().port)));
const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium" });
console.log(`browser ${browser.version()} headless; started ${new Date().toISOString()}`);
for (const round of [1, 2]) {
  for (const kind of ["block", "report"]) {
    const page = await browser.newPage();
    await page.goto(`http://127.0.0.1:${port}/`);
    await page.evaluate(([src, k]) => window.__start(src, k), [workerSource, kind]);
    await page.waitForTimeout(1000);
    const cdp = await page.context().newCDPSession(page);
    await cdp.send("HeapProfiler.enable");
    await cdp.send("HeapProfiler.startSampling", { samplingInterval: 64, includeObjectsCollectedByMajorGC: true, includeObjectsCollectedByMinorGC: true });
    const m0 = await page.evaluate(() => window.__link.messages);
    await page.waitForTimeout(8000);
    const m1 = await page.evaluate(() => window.__link.messages);
    const { profile } = await cdp.send("HeapProfiler.stopSampling");
    const msgs = m1 - m0;
    console.log(`round ${round} ${kind.padEnd(6)}: ${msgs} messages; under "receive" ${(under(profile.head, "receive") / msgs).toFixed(1)} B per message ` +
      `(receive self, i.e. event.data's deserialization: ${(selfOf(profile.head, "receive") / msgs).toFixed(1)}; under onMessage ${(under(profile.head, "onMessage") / msgs).toFixed(1)})`);
    await page.close();
  }
}
await browser.close();
blank.close();
console.log(`finished ${new Date().toISOString()}`);
