// perf-auditor, cs2 review 1: (a) do the page's and a dedicated Worker's `performance.timeOrigin + performance.now()`
// agree in Chromium without cross-origin isolation (no offset), and what step does each clock have? (b) the copy's
// real cost per block size, timed over a batch (the per-copy reading steps by 0.1 ms here and cannot resolve it).
// Blank page served here; Playwright's default launch, no flags.   node .gate/perf-auditor-clock.mjs
import { createRequire } from "node:module";
import http from "node:http";
import path from "node:path";

const WT = "/home/user/Shamble-Frontlines/.gate/cs2-plants";
const require = createRequire(path.join(WT, "package.json"));
const { chromium } = require("@playwright/test");

const workerSource = `
function step() { let a = performance.now(), b = a, min = Infinity; for (let i = 0; i < 200000; i += 1) { b = performance.now(); if (b > a) { if (b - a < min) min = b - a; a = b; } } return min; }
onmessage = async (e) => {
  if (e.data === "stamps") {
    for (let i = 0; i < 400; i += 1) { await new Promise((r) => setTimeout(r, 5)); postMessage({ type: "stamp", at: performance.timeOrigin + performance.now() }); }
    postMessage({ type: "stamps-done", workerStep: step(), isolated: crossOriginIsolated });
    return;
  }
  const memory = new WebAssembly.Memory({ initial: 2 });
  const out = [];
  for (const len of [8, 1032, 4104, 16392, 65544]) {
    const pool = [new ArrayBuffer(len), new ArrayBuffer(len), new ArrayBuffer(len), new ArrayBuffer(len)];
    const n = len > 10000 ? 20000 : 100000;
    for (let w = 0; w < 2000; w += 1) new Uint8Array(pool[w & 3]).set(new Uint8Array(memory.buffer, 64, len));
    const runs = [];
    for (let r = 0; r < 5; r += 1) {
      const t0 = performance.now();
      for (let i = 0; i < n; i += 1) new Uint8Array(pool[i & 3]).set(new Uint8Array(memory.buffer, 64, len));
      runs.push(((performance.now() - t0) * 1e6) / n);
    }
    runs.sort((x, y) => x - y);
    out.push({ len, median: runs[2], min: runs[0], max: runs[4] });
  }
  postMessage({ type: "copy", out });
};`;
const blank = http.createServer((req, res) => { res.writeHead(200, { "content-type": "text/html" }); res.end("<!doctype html><title>clock</title>"); });
const port = await new Promise((resolve) => blank.listen(0, "127.0.0.1", () => resolve(blank.address().port)));
const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium" });
console.log(`browser ${browser.version()} headless; ${new Date().toISOString()}`);
const page = await browser.newPage();
await page.goto(`http://127.0.0.1:${port}/`);
const result = await page.evaluate(async (src) => {
  const url = URL.createObjectURL(new Blob([src], { type: "text/javascript" }));
  const w = new Worker(url);
  const deltas = [];
  const stamps = await new Promise((resolve) => {
    w.onmessage = (e) => {
      if (e.data.type === "stamp") deltas.push(performance.timeOrigin + performance.now() - e.data.at);
      else if (e.data.type === "stamps-done") resolve(e.data);
    };
    w.postMessage("stamps");
  });
  let a = performance.now(), b = a, pageStep = Infinity;
  for (let i = 0; i < 200000; i += 1) { b = performance.now(); if (b > a) { if (b - a < pageStep) pageStep = b - a; a = b; } }
  const copy = await new Promise((resolve) => { w.onmessage = (e) => resolve(e.data.out); w.postMessage("copy"); });
  deltas.sort((x, y) => x - y);
  const q = (p) => deltas[Math.min(deltas.length, Math.max(1, Math.ceil((p / 100) * deltas.length))) - 1];
  return { n: deltas.length, min: deltas[0], p50: q(50), p95: q(95), max: deltas[deltas.length - 1], negatives: deltas.filter((d) => d < 0).length,
    workerStep: stamps.workerStep, pageStep, isolated: stamps.isolated, pageIsolated: crossOriginIsolated, copy };
}, workerSource);
console.log(`(a) idle page, ${result.n} stamps 5 ms apart: delivery min ${result.min.toFixed(3)}, p50 ${result.p50.toFixed(3)}, p95 ${result.p95.toFixed(3)}, max ${result.max.toFixed(3)} ms; negative ${result.negatives}; smallest clock step: Worker ${result.workerStep.toFixed(3)} ms, page ${result.pageStep.toFixed(3)} ms; crossOriginIsolated page ${result.pageIsolated}, Worker ${result.isolated}`);
for (const c of result.copy) console.log(`(b) copy of ${String(c.len).padStart(5)} B with both views: median ${c.median.toFixed(0)} ns (min ${c.min.toFixed(0)}, max ${c.max.toFixed(0)}; 5 batches)`);
await browser.close();
blank.close();
