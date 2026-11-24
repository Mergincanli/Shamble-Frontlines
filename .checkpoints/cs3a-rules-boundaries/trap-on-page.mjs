// Evidence for plan step 6, not a test: what does the page say when the players' Wasm panics? The players' Wasm has
// no export that panics on demand, so a panic is planted in it by hand (never committed), the development page is
// built from that tree, and this script opens it:   node trap-on-page.mjs <the built page's folder> [port]
import { spawn } from "node:child_process";
import { chromium } from "@playwright/test";

const [dist, port = "4191"] = process.argv.slice(2);
const server = spawn("node", ["serve.mjs", port], { cwd: dist, stdio: "ignore" });
const browser = await chromium.launch({ executablePath: process.env["SHAMBLE_CHROMIUM"] ?? "/opt/pw-browsers/chromium" });
try {
  const page = await browser.newPage();
  const thrown = [];
  page.on("pageerror", (error) => thrown.push(error.message));
  await new Promise((resolve) => setTimeout(resolve, 500));
  await page.goto(`http://127.0.0.1:${port}/`);
  await page.locator("#notice").waitFor({ state: "visible", timeout: 60_000 });
  const report = await page.evaluate(() => {
    const data = window.__shambleReport();
    return { errors: data.errors, lastError: data.lastError, tick: data.tick, workerTicks: data.worker?.ticks };
  });
  console.log(JSON.stringify({ notice: await page.locator("#notice").textContent(), ...report, pageErrors: thrown, workers: page.workers().length }, null, 1));
} finally {
  await browser.close();
  server.kill();
}
