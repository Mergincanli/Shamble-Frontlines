// perf-auditor scratch: does the built development page hang when its window becomes 0 CSS pixels wide?
// node .checkpoints/cs4c-soldier/perf-s-zero-width.mjs <built development folder, a full path>
import { spawn } from "node:child_process";
import { createRequire } from "node:module";
import path from "node:path";

const require = createRequire(path.resolve("tools/package.json"));
const { chromium } = require("playwright");
const [dist] = process.argv.slice(2);
console.log("serving", dist);
const server = spawn(process.execPath, ["serve.mjs", "4179"], { cwd: dist, stdio: ["ignore", "pipe", "inherit"] });
await new Promise((resolve) => server.stdout.on("data", (c) => String(c).includes("4179") && resolve()));
console.log("server up");
const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium" });
const withTimeout = (p, ms) => Promise.race([p, new Promise((r) => setTimeout(() => r("TIMEOUT"), ms))]);
const ready = (page) =>
  page.waitForFunction(() => typeof window.__shambleReport === "function" && window.__shambleReport().scene !== null && window.__shambleReport().frames >= 5, undefined, { timeout: 120_000 });
try {
  const page = await browser.newPage({ viewport: { width: 640, height: 360 }, deviceScaleFactor: 1 });
  await page.goto("http://127.0.0.1:4179/");
  await ready(page);
  console.log("before", JSON.stringify(await page.evaluate(() => ({ w: innerWidth, h: innerHeight, frames: window.__shambleReport().frames }))));
  try {
    await page.setViewportSize({ width: 0, height: 360 });
    const seen = await withTimeout(page.evaluate(() => ({ w: innerWidth, h: innerHeight, frames: window.__shambleReport().frames })), 15_000);
    console.log("after setViewportSize width 0:", JSON.stringify(seen));
  } catch (e) {
    console.log("setViewportSize 0 refused:", String(e).split("\n")[0]);
  }
  // Second route: the page's own resize handler, run with innerWidth reading 0 (what a 0-wide window reports).
  const page2 = await browser.newPage({ viewport: { width: 640, height: 360 }, deviceScaleFactor: 1 });
  await page2.goto("http://127.0.0.1:4179/");
  await ready(page2);
  const f0 = await page2.evaluate(() => window.__shambleReport().frames);
  await page2.evaluate(() => {
    Object.defineProperty(window, "innerWidth", { configurable: true, get: () => 0 });
    setTimeout(() => window.dispatchEvent(new Event("resize")), 0);
  });
  await new Promise((r) => setTimeout(r, 1000));
  const seen2 = await withTimeout(page2.evaluate(() => window.__shambleReport().frames), 15_000);
  console.log(`innerWidth 0 + resize event: frames before ${f0}, page answers ${JSON.stringify(seen2)} within 15 s`);
  // Control: the same with innerWidth 1.
  const page3 = await browser.newPage({ viewport: { width: 640, height: 360 }, deviceScaleFactor: 1 });
  await page3.goto("http://127.0.0.1:4179/");
  await ready(page3);
  await page3.evaluate(() => {
    Object.defineProperty(window, "innerWidth", { configurable: true, get: () => 1 });
    setTimeout(() => window.dispatchEvent(new Event("resize")), 0);
  });
  await new Promise((r) => setTimeout(r, 1000));
  const seen3 = await withTimeout(page3.evaluate(() => window.__shambleReport().frames), 15_000);
  console.log(`control innerWidth 1 + resize event: page answers ${JSON.stringify(seen3)}`);
} catch (e) {
  console.log("ERROR", String(e).slice(0, 500));
} finally {
  await withTimeout(browser.close(), 10_000);
  server.kill();
  process.exit(0);
}
