// perf-auditor, cs2 review 1: does the pacer's fractional setTimeout delay (truncated by WebIDL `long`) plus the
// nested-timer clamp make ticks late? Part A reads the real page (worktree dist, pad 0): ticks vs wakes and lateness.
// Part B runs the pacer's own logic in two blob Workers on a blank page: delay as the head passes it (fractional) and
// Math.ceil(delay). Playwright's default launch, no flags of our own. Run: node .gate/perf-auditor-pacing.mjs
import { createRequire } from "node:module";
import http from "node:http";
import path from "node:path";
import { pathToFileURL } from "node:url";

const WT = "/home/user/Shamble-Frontlines/.gate/cs2-plants";
const require = createRequire(path.join(WT, "package.json"));
const { chromium } = require("@playwright/test");
const dist = path.join(WT, "packages/client/dist");
const { createServer, listen } = await import(pathToFileURL(path.join(dist, "serve.mjs")).href);
const server = createServer({ root: dist });
const port = await listen(server, 0);

const workerSource = `
const TICK = 1000 / 30;
let mode = "";
let anchor = 0, n = 0, nextDue = 0;
const late = [], shortAsked = [], shortGot = [];
let wakes = 0, ticks = 0, empty = 0, drops = 0, lastSet = 0, lastAsked = 0;
function wake() {
  const woke = performance.now();
  if (lastAsked < 4) { shortAsked.push(lastAsked); shortGot.push(woke - lastSet); }
  wakes++;
  let ran = 0; let now = woke;
  while (ran < 7 && nextDue <= now) {
    late.push(now - nextDue);
    n++; nextDue = anchor + (n + 1) * TICK;
    ran++; ticks++;
    now = performance.now();
  }
  if (ran === 0) empty++;
  if (nextDue <= now) { drops++; anchor = now; n = 0; nextDue = anchor + TICK; }
  let sleep = Math.max(0, nextDue - now);
  if (mode === "ceil") sleep = Math.ceil(sleep);
  if (ticks >= 600) { done(); return; }
  lastSet = performance.now(); lastAsked = sleep;
  setTimeout(wake, sleep);
}
function pct(a, p) { const s = [...a].sort((x, y) => x - y); return s.length ? s[Math.min(s.length, Math.max(1, Math.ceil(p / 100 * s.length))) - 1] : NaN; }
function done() {
  postMessage({ mode, ticks, wakes, empty, drops, p50: pct(late, 50), p99: pct(late, 99), max: Math.max(...late),
    shortSleeps: shortAsked.length, shortAskedP50: pct(shortAsked, 50), shortGotP50: pct(shortGot, 50), shortGotP99: pct(shortGot, 99) });
}
onmessage = (e) => { mode = e.data; anchor = performance.now(); nextDue = anchor + TICK; lastSet = anchor; lastAsked = 99; setTimeout(wake, TICK); };
`;
const blank = http.createServer((req, res) => {
  res.writeHead(200, { "content-type": "text/html" });
  res.end("<!doctype html><title>pacing</title><p>pacing</p>");
});
const blankPort = await new Promise((resolve) => blank.listen(0, "127.0.0.1", () => resolve(blank.address().port)));

const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium" });
console.log(`browser ${browser.version()} headless; started ${new Date().toISOString()}`);

// Part A: the real page.
{
  const page = await browser.newPage({ viewport: { width: 1280, height: 720 } });
  await page.goto(`http://127.0.0.1:${port}/?run=perf-pacing`);
  await page.waitForFunction(() => window.__shambleReport?.().blocks >= 600, null, { timeout: 120000 });
  await page.waitForTimeout(600);
  const d = await page.evaluate(() => window.__shambleReport());
  const w = d.worker;
  console.log(`A real page: ticks ${w.ticks}, wakes ${w.wakes} (${(w.wakes / w.ticks).toFixed(2)} per tick), most in a wake ${w.maxTicksInOneWake}, drops ${w.drops}, lateness p50 ${w.latenessP50.toFixed(2)} p99 ${w.latenessP99.toFixed(2)} max ${w.latenessMax.toFixed(2)} ms, pool misses ${w.poolMisses}, fps ${d.fps.toFixed(1)}, frame p50 ${d.frameP50.toFixed(1)} ms, delivery p50 ${d.deliveryP50.toFixed(1)} p95 ${d.deliveryP95.toFixed(1)} ms`);
  await page.close();
}

// Part B: the pacer's logic alone, two variants at once in two Workers.
for (let round = 1; round <= 2; round += 1) {
  const page = await browser.newPage();
  await page.goto(`http://127.0.0.1:${blankPort}/`);
  const results = await page.evaluate(async (src) => {
    const url = URL.createObjectURL(new Blob([src], { type: "text/javascript" }));
    const run = (mode) => new Promise((resolve) => { const w = new Worker(url); w.onmessage = (e) => { resolve(e.data); w.terminate(); }; w.postMessage(mode); });
    return Promise.all([run("as-head"), run("ceil")]);
  }, workerSource);
  for (const r of results) {
    console.log(`B round ${round} ${r.mode.padEnd(7)}: ticks ${r.ticks}, wakes ${r.wakes} (${(r.wakes / r.ticks).toFixed(2)} per tick; ${r.empty} with no tick due), drops ${r.drops}, lateness p50 ${r.p50.toFixed(2)} p99 ${r.p99.toFixed(2)} max ${r.max.toFixed(2)} ms; sleeps asked under 4 ms: ${r.shortSleeps} (asked p50 ${Number(r.shortAskedP50).toFixed(2)} ms, got p50 ${Number(r.shortGotP50).toFixed(2)} p99 ${Number(r.shortGotP99).toFixed(2)} ms)`);
  }
  await page.close();
}
await browser.close();
server.close();
blank.close();
console.log(`finished ${new Date().toISOString()}`);
