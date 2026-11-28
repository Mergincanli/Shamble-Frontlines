// A one-off reading for the scene step (the advisor tool's point 2 before the build): the page's render counts must not
// depend on whether the map is drawn in before or after the first frame. Two runs of the built development page in
// Chromium: (late map) the content's response held back 4 s, so the Worker's `ready` comes after the frames begin;
// (late frames) the development chunks held back 4 s, so the frame loop (which boot starts after them) begins after
// `ready`. Each run prints the order seen and the counts. From the worktree's root:
//   node .checkpoints/cs4c-soldier/counts-order-4c.mjs <built development folder, a full path>
import { spawn } from "node:child_process";
import { createRequire } from "node:module";
import path from "node:path";

const require = createRequire(import.meta.url);
const { chromium } = require("playwright");
const [dist] = process.argv.slice(2);
if (typeof dist !== "string" || !path.isAbsolute(dist)) throw new Error("usage: counts-order-4c.mjs <built folder, a full path>");
const ADDRESS = "http://127.0.0.1:4173";

const server = spawn(process.execPath, ["serve.mjs"], { cwd: dist, stdio: ["ignore", "pipe", "inherit"] });
await new Promise((resolve, reject) => {
  const timer = setTimeout(() => reject(new Error("the page's server did not start in 20 s")), 20_000);
  server.stdout.on("data", (chunk) => {
    if (String(chunk).includes("127.0.0.1:4173")) { clearTimeout(timer); resolve(); }
  });
});
const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium" });
const hold = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
try {
  // The Worker answers `ready` about 5 s after the page opens in this container, so the frames are held 10 s.
  for (const [name, pattern, ms] of [["late map", "**/content.json", 4000], ["late frames", "**/assets/{overlay,report,keys}-*.js", 10_000]]) {
    const page = await browser.newPage({ viewport: { width: 1280, height: 720 }, deviceScaleFactor: 1 });
    await page.route(pattern, async (route) => {
      await hold(ms);
      await route.continue();
    });
    // When the first frame is drawn, on the page's own clock: the hold sets the order (the content held, the map comes
    // after the frames begin; the chunks held, the frames begin after the map).
    // Both moments are taken inside the page, when they happen: the polling of a test is late by up to a frame, and
    // a frame takes about half a second on this software renderer. The Worker's `ready` is the map's hand-off (the
    // page reads and draws it in the same message's handler); three's first animation frame is the loop's start.
    await page.addInitScript(() => {
      const marks = { firstFrame: null, ready: null };
      window.__orderMarks = marks;
      const raf = window.requestAnimationFrame.bind(window);
      window.requestAnimationFrame = (callback) => raf((time) => {
        if (marks.firstFrame === null) marks.firstFrame = performance.now();
        callback(time);
      });
      const PageWorker = window.Worker;
      window.Worker = class extends PageWorker {
        constructor(...args) {
          super(...args);
          this.addEventListener("message", (event) => {
            if (marks.ready === null && event.data?.type === "ready") marks.ready = performance.now();
          });
        }
      };
    });
    await page.goto(`${ADDRESS}/`);
    // The development block (with the readout's hook) is set up right before boot starts the frame loop, so the hook's
    // first moment is the loop's start; the scene's first moment is the map's delivery, which the hook can only show
    // once it exists.
    const at = (test) => page.waitForFunction(test, undefined, { timeout: 120_000, polling: 25 }).then(() => page.evaluate(() => performance.now()));
    const loopStart = await at(() => typeof window.__shambleReport === "function");
    const sceneSeen = await at(() => window.__shambleReport?.().scene !== null);
    await page.waitForFunction(() => {
      const data = window.__shambleReport?.();
      return data !== undefined && data.worker !== null && data.scene !== null && data.frames >= 30;
    }, undefined, { timeout: 120_000 });
    const data = await page.evaluate(() => window.__shambleReport());
    const marks = await page.evaluate(() => window.__orderMarks);
    const order = marks.ready < marks.firstFrame ? "the map before the first frame" : "the map after the first frame";
    console.log(`${name}: ready at ${Math.round(marks.ready)} ms, the first frame at ${Math.round(marks.firstFrame)} ms (${order}); seen by polling: the loop ${Math.round(loopStart)} ms, the scene ${Math.round(sceneSeen)} ms; frames ${data.frames}, scene ${JSON.stringify(data.scene)}, errors ${data.errors}, counts ${JSON.stringify(data.counters)}`);
    await page.close();
  }
} finally {
  await browser.close();
  server.kill();
}
