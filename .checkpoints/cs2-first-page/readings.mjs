// The readings of change-set 2 in the container's Chromium (plan section 9): the hand-off as a curve over block
// sizes, the Worker's pacing, the frame times. Reported, never judged. A scratch run, not a test:
//   node .checkpoints/cs2-first-page/readings.mjs <built development page's folder>
import { createRequire } from "node:module";
import path from "node:path";
import { pathToFileURL } from "node:url";
const require = createRequire(path.resolve("package.json"));
const { chromium } = require("@playwright/test");
const dist = path.resolve(process.argv[2]);
const { createServer, listen } = await import(pathToFileURL(path.join(dist, "serve.mjs")).href);
const server = createServer({ root: dist });
const port = await listen(server, 0);
const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium" });
console.log(`browser: ${browser.version()} (headless), viewport 1280x720`);
console.log("pad bytes | block bytes | backend | ticks | copy p50 / p99 ms | delivery p50 / p95 ms | lateness p50 / p99 / max ms | most ticks in a wake | drops | pool misses | fps | frame p50 / p99 ms");
for (const pad of [0, 1024, 4096, 16384, 65536]) {
  const page = await browser.newPage({ viewport: { width: 1280, height: 720 } });
  await page.goto(`http://127.0.0.1:${port}/?pad=${pad}&run=container-pad-${pad}`);
  await page.waitForFunction(() => window.__shambleReport?.().blocks >= 600, null, { timeout: 120000 });
  await page.waitForTimeout(600);
  const d = await page.evaluate(() => window.__shambleReport());
  const w = d.worker;
  const f = (v) => (Number.isFinite(v) ? v.toFixed(3) : "n/a");
  console.log([pad, d.blockBytes, d.picture.backend, d.tick, `${f(w.copyP50)} / ${f(w.copyP99)}`, `${f(d.deliveryP50)} / ${f(d.deliveryP95)}`,
    `${f(w.latenessP50)} / ${f(w.latenessP99)} / ${f(w.latenessMax)}`, w.maxTicksInOneWake, w.drops, w.poolMisses, d.fps.toFixed(1), `${f(d.frameP50)} / ${f(d.frameP99)}`].join(" | "));
  if (pad === 0) console.log(`  run header: ${JSON.stringify(d.header)}; hardwareConcurrency in the Worker ${w.hardwareConcurrency}; timestampQuery ${d.picture.timestampQuery}; counters ${JSON.stringify(d.counters)}`);
  await page.close();
}
await browser.close();
server.close();
