// A picture of the built development page in Chromium, for the scene step's before and after (no part of the game):
// it starts the page's own small server (`serve.mjs`, in the built folder), opens the page at 1280 x 720, waits until
// the match runs and the page has drawn its frames, saves one screenshot, and prints the page's render counts and
// backend from its own report. From the worktree's root:
//   node .checkpoints/cs4c-soldier/shot-4c.mjs <built folder, a full path> <out .png, a full path> [scale]
// `scale` is the device pixel ratio: 2 for a picture a person looks at, 1 for one only the chat reads.
import { spawn } from "node:child_process";
import { createRequire } from "node:module";
import path from "node:path";

const require = createRequire(import.meta.url);
const { chromium } = require("playwright");
const [dist, out, scaleText = "2"] = process.argv.slice(2);
const full = (p) => typeof p === "string" && path.isAbsolute(p) && !p.split("/").includes("..");
if (!full(dist) || !full(out) || !out.endsWith(".png")) throw new Error("usage: shot-4c.mjs <built folder> <out .png> [scale], both full paths");
const scale = Number(scaleText);
const ADDRESS = "http://127.0.0.1:4173";

const server = spawn(process.execPath, ["serve.mjs"], { cwd: dist, stdio: ["ignore", "pipe", "inherit"] });
try {
  // The server says its address once it listens.
  await new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error("the page's server did not start in 20 s")), 20_000);
    server.stdout.on("data", (chunk) => {
      if (String(chunk).includes("127.0.0.1:4173")) { clearTimeout(timer); resolve(); }
    });
    server.on("exit", (code) => reject(new Error(`the page's server ended (${code})`)));
  });
  const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium" });
  try {
    const page = await browser.newPage({ viewport: { width: 1280, height: 720 }, deviceScaleFactor: scale });
    const said = [];
    page.on("console", (message) => said.push(`${message.type()}: ${message.text()}`));
    page.on("pageerror", (error) => said.push(`pageerror: ${error.message}`));
    await page.goto(`${ADDRESS}/`);
    try {
      await page.waitForFunction(() => {
        const data = window.__shambleReport?.();
        // Headless Chromium draws with its software renderer here: about 2 frames a second at a pixel ratio of 2.
        return data !== undefined && data.worker !== null && data.blocks >= 60 && data.frames >= 20;
      }, undefined, { timeout: 90_000 });
    } catch (error) {
      // What the page said and showed, so that a page that does not run says why.
      const state = await page.evaluate(() => {
        const data = window.__shambleReport?.();
        return {
          notice: document.getElementById("notice")?.textContent ?? null,
          report: data === undefined ? null : { blocks: data.blocks, frames: data.frames, worker: data.worker !== null, errors: data.errors, lastError: data.lastError },
        };
      });
      console.error(JSON.stringify(state), "\n", said.slice(0, 20).join("\n"));
      throw error;
    }
    const data = await page.evaluate(() => window.__shambleReport());
    await page.screenshot({ path: out });
    console.log(`${out}: ${data.picture.backend}, frames ${data.frames}, errors ${data.errors}, counts ${JSON.stringify(data.counters)}`);
  } finally {
    await browser.close();
  }
} finally {
  server.kill();
}
