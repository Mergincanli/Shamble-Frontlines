// perf-auditor, cs2 review round 2: the pacer's logic (pacing.ts wake loop + host.ts's timer call) in blob Workers on
// a blank page, to split each tick's lateness into the part the whole-millisecond rounding adds by design
// (Math.ceil(sleep) - sleep, 0 to 1 ms) and the timer's own overshoot (wake - (set time + delay)), and to count wakes
// that find no tick due (the early-wake risk the ceil leaves: the clock's 0.1 ms coarsening). "ceil" = host.ts:139 at
// d257bb6, run for N_CEIL ticks; "head" = 30a5a13 (fractional delay), 600 ticks at the same time, as the control.
// Own server on a free port, Playwright's default launch, no flags of our own.   node .gate/perf-auditor-r2-pacer.mjs
import { createRequire } from "node:module";
import http from "node:http";
import path from "node:path";

const ROOT = "/home/user/Shamble-Frontlines";
const N_CEIL = Number(process.env.N_CEIL ?? 3000);
const require = createRequire(path.join(ROOT, "package.json"));
const { chromium } = require("@playwright/test");

const workerSource = `
const PERIOD = 1000 / 30;
let mode = "", target = 0;
let anchor = 0, since = 0, nextDue = 0;
const late = [], margin = [], over = [], emptyInfo = [];
let wakes = 0, ticks = 0, empty = 0, drops = 0, maxRan = 0, setAt = 0, delay = 0, exact = 0, fromTimer = false;
// The schedule LocalSchedule keeps (host.test.ts's stand-in): step() returns when the next tick is due.
const step = () => { since += 1; return anchor + (since + 1) * PERIOD; };
const rebase = (now) => { anchor = now; since = 0; return now + PERIOD; };
function wake() {
  const woke = performance.now();
  wakes++;
  if (fromTimer) over.push(woke - setAt - delay);
  let ran = 0, now = woke;
  while (ran < 7 && nextDue <= now) {          // pacing.ts:92-98
    late.push(now - nextDue);
    nextDue = step();
    ran++;
    now = performance.now();
  }
  ticks += ran;
  if (ran > maxRan) maxRan = ran;
  if (ran === 0 && fromTimer) { empty++; emptyInfo.push({ asked: exact, delay, slept: woke - setAt, dueIn: nextDue - woke }); }
  if (nextDue <= now) { drops++; nextDue = rebase(now); }   // pacing.ts:101-105
  const sleep = Math.max(0, nextDue - now);                // pacing.ts:106
  if (ticks >= target) { done(); return; }
  exact = sleep;
  delay = mode === "ceil" ? Math.ceil(sleep) : sleep;      // host.ts:139 (d257bb6) or 30a5a13
  margin.push(delay - sleep);
  fromTimer = true;
  setAt = performance.now();
  setTimeout(wake, delay);
}
const pct = (a, p) => { const s = Float64Array.from(a).sort(); return s.length ? s[Math.min(s.length, Math.max(1, Math.ceil(p / 100 * s.length))) - 1] : NaN; };
const sum = (a) => { const s = Float64Array.from(a); return { p50: pct(s, 50), p90: pct(s, 90), p99: pct(s, 99), p999: pct(s, 99.9), max: Math.max(...s), mean: s.reduce((x, y) => x + y, 0) / s.length }; };
function done() {
  const lateOver4 = late.filter((x) => x > 4).length;
  postMessage({ mode, ticks, wakes, empty, drops, maxRan, late: sum(late), margin: sum(margin), over: sum(over), lateOver4, emptyInfo: emptyInfo.slice(0, 10) });
}
onmessage = (e) => { mode = e.data.mode; target = e.data.target; nextDue = rebase(performance.now()); wake(); };  // host.start(): a wake at once
`;
const blank = http.createServer((req, res) => {
  res.writeHead(200, { "content-type": "text/html" });
  res.end("<!doctype html><title>pacing</title><p>pacing</p>");
});
const blankPort = await new Promise((resolve) => blank.listen(0, "127.0.0.1", () => resolve(blank.address().port)));
const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium" });
console.log(`browser ${browser.version()} headless; started ${new Date().toISOString()}; ceil ${N_CEIL} ticks, head 600 ticks alongside`);
const page = await browser.newPage();
await page.goto(`http://127.0.0.1:${blankPort}/`);
const results = await page.evaluate(async ({ src, nCeil }) => {
  const url = URL.createObjectURL(new Blob([src], { type: "text/javascript" }));
  const run = (mode, target) => new Promise((resolve) => { const w = new Worker(url); w.onmessage = (e) => { resolve(e.data); w.terminate(); }; w.postMessage({ mode, target }); });
  return Promise.all([run("head", 600), run("ceil", nCeil)]);
}, { src: workerSource, nCeil: N_CEIL });
const f = (s) => `p50 ${s.p50.toFixed(2)} p90 ${s.p90.toFixed(2)} p99 ${s.p99.toFixed(2)} p99.9 ${s.p999.toFixed(2)} max ${s.max.toFixed(2)} mean ${s.mean.toFixed(2)}`;
for (const r of results) {
  console.log(`${r.mode}: ticks ${r.ticks}, wakes ${r.wakes} (${(r.wakes / r.ticks).toFixed(3)} per tick; ${r.empty} timer wakes found no tick due), drops ${r.drops}, most in a wake ${r.maxRan}`);
  console.log(`   lateness ms  ${f(r.late)}; ticks later than 4 ms: ${r.lateOver4} (${((100 * r.lateOver4) / r.ticks).toFixed(2)} %)`);
  console.log(`   rounding ms  ${f(r.margin)}  (delay given minus the exact sleep)`);
  console.log(`   overshoot ms ${f(r.over)}  (wake minus set time minus delay given)`);
  if (r.emptyInfo.length) console.log(`   first empty wakes: ${JSON.stringify(r.emptyInfo.map((x) => Object.fromEntries(Object.entries(x).map(([k, v]) => [k, Number(v.toFixed(3))]))))}`);
}
await browser.close();
blank.close();
console.log(`finished ${new Date().toISOString()}`);
