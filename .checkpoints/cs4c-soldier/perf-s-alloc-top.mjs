// perf-auditor scratch: the page's main-thread allocation by function (V8 sampling heap profiler over CDP, minor and
// major collections included), 8 s after the scene is drawn, on a built development page.
// node .checkpoints/cs4c-soldier/perf-s-alloc-top.mjs <built development folder, a full path> <port>
import { spawn } from "node:child_process";
import { createRequire } from "node:module";
import path from "node:path";

const require = createRequire(path.resolve("tools/package.json"));
const { chromium } = require("playwright");
const [dist, port] = process.argv.slice(2);
const server = spawn(process.execPath, ["serve.mjs", port], { cwd: dist, stdio: ["ignore", "pipe", "inherit"] });
await new Promise((resolve) => server.stdout.on("data", (c) => String(c).includes(port) && resolve()));
const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium" });
try {
  const page = await browser.newPage({ viewport: { width: 1280, height: 720 }, deviceScaleFactor: 1 });
  await page.goto(`http://127.0.0.1:${port}/`);
  await page.waitForFunction(() => typeof window.__shambleReport === "function" && window.__shambleReport().frames >= 20, undefined, { timeout: 120_000 });
  const cdp = await page.context().newCDPSession(page);
  await cdp.send("HeapProfiler.enable");
  const f0 = await page.evaluate(() => window.__shambleReport().frames);
  await cdp.send("HeapProfiler.startSampling", { samplingInterval: 512, includeObjectsCollectedByMajorGC: true, includeObjectsCollectedByMinorGC: true });
  await new Promise((r) => setTimeout(r, 8000));
  const { profile } = await cdp.send("HeapProfiler.stopSampling");
  const f1 = await page.evaluate(() => window.__shambleReport().frames);
  const frames = f1 - f0;
  const by = new Map();
  let total = 0;
  const walk = (node) => {
    const self = node.selfSize;
    total += self;
    const cf = node.callFrame;
    const key = `${cf.functionName || "(anonymous)"} ${path.basename(cf.url)}:${cf.lineNumber + 1}`;
    by.set(key, (by.get(key) ?? 0) + self);
    for (const child of node.children) walk(child);
  };
  walk(profile.head);
  console.log(`${path.basename(dist)}: ${frames} frames, ${total} bytes sampled, ${(total / frames).toFixed(0)} B a frame`);
  for (const [key, bytes] of [...by.entries()].sort((a, b) => b[1] - a[1]).slice(0, 12)) console.log(`  ${(bytes / frames).toFixed(0).padStart(7)} B/frame  ${key}`);
} finally {
  await browser.close();
  server.kill();
}
