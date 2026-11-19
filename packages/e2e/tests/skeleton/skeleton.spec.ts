// The first page in a real browser, on the built development page (playwright.config.ts says how to run it).
import { readFileSync } from "node:fs";
import { expect, test, type Page } from "@playwright/test";
import { ADDRESS } from "../../playwright.config.ts";

interface Report {
  picture: { backend: string };
  counters: Record<string, number>;
  tick: number;
  blocks: number;
  posts: number;
  frames: number;
  errors: number;
  pauses: number;
  lastPause: { ticksWhileHidden: number } | null;
  worker: { ticks: number; undetached: number; maxTicksInOneWake: number } | null;
}
const baseline = JSON.parse(readFileSync(new URL("./render-counts.json", import.meta.url), "utf8")) as Record<string, unknown>;
const report = (page: Page): Promise<Report> =>
  page.evaluate(() => (window as unknown as { __shambleReport(): Report }).__shambleReport());

test.beforeAll(() => {
  if (process.env["SHAMBLE_DIST"] === undefined) throw new Error("set SHAMBLE_DIST to the built page's folder");
});

/** Opens the page with every WebAssembly entry point on the page replaced by one that counts and throws. */
async function open(page: Page, query: string): Promise<string[]> {
  const outside: string[] = [];
  page.on("request", (request) => {
    if (!request.url().startsWith(`${ADDRESS}/`)) outside.push(request.url());
  });
  // Init scripts run in the page and not in its Worker, so the Worker still loads the Wasm.
  await page.addInitScript(() => {
    const count = window as unknown as { __wasmCalls: number };
    count.__wasmCalls = 0;
    const wasm = WebAssembly as unknown as Record<string, unknown>;
    for (const name of ["instantiate", "instantiateStreaming", "compile", "compileStreaming", "validate", "Module", "Instance", "Memory"]) {
      wasm[name] = function () {
        count.__wasmCalls += 1;
        throw new Error(`the page used WebAssembly.${name}`);
      };
    }
  });
  await page.goto(`/${query}`);
  await page.waitForFunction(() => (window as unknown as { __shambleReport?(): Report }).__shambleReport?.().worker != null);
  await page.waitForFunction(() => (window as unknown as { __shambleReport(): Report }).__shambleReport().blocks >= 90);
  return outside;
}

for (const [name, query] of [["the default backend", ""], ["the forced WebGL 2 fallback", "?backend=webgl"]] as const) {
  test(`the first page on ${name}`, async ({ page }, info) => {
    const outside = await open(page, query);
    const data = await report(page);
    const backend = data.picture.backend;
    // The backend is named: on the page itself, in the readout, and so in the screenshot.
    expect(["WebGPU", "WebGL 2 fallback"]).toContain(backend);
    if (query !== "") expect(backend).toBe("WebGL 2 fallback");
    await expect(page.locator("html")).toHaveAttribute("data-backend", backend);
    await expect(page.locator("#overlay pre")).toContainText(`backend: ${backend}`);
    await page.screenshot({ path: info.outputPath(`first-page-${backend.replaceAll(" ", "-")}.png`) });
    // One block per tick, each transferred; at most one input post per drawn frame; no error.
    expect(Math.abs(data.blocks - data.tick)).toBeLessThanOrEqual(1);
    expect(Math.abs((data.worker?.ticks ?? -9) - data.tick)).toBeLessThanOrEqual(8);
    expect(data.worker?.undetached).toBe(0);
    expect(data.posts).toBeLessThanOrEqual(data.frames);
    expect(data.posts).toBeGreaterThan(0);
    expect(data.errors).toBe(0);
    // The page made no call into WebAssembly and no instantiation (PM-0002-2).
    expect(await page.evaluate(() => (window as unknown as { __wasmCalls: number }).__wasmCalls)).toBe(0);
    // Nothing was asked of any address but the small server's.
    expect(outside).toEqual([]);
    // The empty scene's render counts are the committed ones for this backend.
    if (baseline[backend] !== null) expect(data.counters).toEqual(baseline[backend]);
  });
}

test("a hidden tab pauses the match, and it resumes where it stopped", async ({ page }) => {
  await open(page, "");
  // A headless tab cannot really be hidden: the page's own handler is driven instead. Brave checks the real thing.
  const setHidden = (hidden: boolean): Promise<void> =>
    page.evaluate((value) => {
      Object.defineProperty(document, "hidden", { configurable: true, get: () => value });
      document.dispatchEvent(new Event("visibilitychange"));
    }, hidden);
  await setHidden(true);
  const before = (await report(page)).tick;
  await page.waitForTimeout(2000);
  const during = (await report(page)).tick;
  // At most the one tick that was already on its way; two seconds would be 60.
  expect(during - before).toBeLessThanOrEqual(1);
  await setHidden(false);
  await page.waitForFunction((tick) => (window as unknown as { __shambleReport(): Report }).__shambleReport().tick >= tick + 30, during);
  const after = await report(page);
  expect(after.pauses).toBe(1);
  // Blocks already on their way when the tab was hidden still arrive: at most one wake's worth (7) and one.
  expect(after.lastPause?.ticksWhileHidden).toBeLessThanOrEqual(8);
  expect(after.worker?.maxTicksInOneWake).toBeLessThanOrEqual(7);
  expect(after.errors).toBe(0);
});
