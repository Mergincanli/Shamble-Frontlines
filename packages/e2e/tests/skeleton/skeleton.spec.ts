// The first page in a real browser, on the built development page (playwright.config.ts says how to run it).
import { readFileSync } from "node:fs";
import { expect, test, type Page } from "@playwright/test";
import { ADDRESS } from "../../playwright.config.ts";
import { needsBuild, report, running, type Report } from "./report.ts";

const baseline = JSON.parse(readFileSync(new URL("./render-counts.json", import.meta.url), "utf8")) as Record<string, unknown>;

test.beforeAll(() => void needsBuild());

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
  await running(page);
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
    // One block per tick, each transferred; no error.
    expect(Math.abs(data.blocks - data.tick)).toBeLessThanOrEqual(1);
    expect(data.worker?.undetached).toBe(0);
    // A timer that fires before its tick is due costs a second, late wake: about one wake per tick is right.
    expect(data.worker?.wakes ?? 0).toBeLessThanOrEqual(1.1 * (data.worker?.ticks ?? 0) + 2);
    // The newest input is posted once per drawn frame: over two seconds, as many posts as frames.
    await page.waitForTimeout(2000);
    const later = await report(page);
    const [posts, frames] = [later.posts - data.posts, later.frames - data.frames];
    expect(frames).toBeGreaterThan(3);
    expect(posts).toBeLessThanOrEqual(frames);
    expect(frames - posts).toBeLessThanOrEqual(1);
    expect(later.errors).toBe(0);
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
  // The Worker's own count is what is judged: the page asks it for a report four times a second, and blocks that
  // were already on their way when the tab was hidden do not move it.
  await page.waitForTimeout(1000);
  const hidden = (await report(page)).worker?.ticks ?? -1;
  await page.waitForTimeout(2000);
  const stillHidden = await report(page);
  // Not one tick in two seconds; a match that ran on would have made 60.
  expect(stillHidden.worker?.ticks).toBe(hidden);
  await setHidden(false);
  await page.waitForTimeout(1500);
  const after = await report(page);
  // It goes on from where it stopped: one pause, nothing dropped, and no more ticks than the time since.
  expect(after.worker?.pauses).toBe(1);
  expect(after.worker?.drops).toBe(0);
  expect(after.worker?.ticks ?? 0).toBeGreaterThan(hidden);
  expect((after.worker?.ticks ?? 999) - hidden).toBeLessThanOrEqual(60);
  expect(after.pauses).toBe(1);
  expect(after.errors).toBe(0);
});
