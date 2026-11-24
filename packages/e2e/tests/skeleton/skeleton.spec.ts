// The first page in a real browser, on the built development page (playwright.config.ts says how to run it).
import { readFileSync } from "node:fs";
import { expect, test, type Page } from "@playwright/test";
import { ADDRESS } from "../../playwright.config.ts";
import { drawn, needsBuild, report, running, type Report } from "./report.ts";

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
    // Every function and constructor the name holds: the first five run code or check it, the rest make its parts.
    const names = ["instantiate", "instantiateStreaming", "compile", "compileStreaming", "validate", "Module", "Instance", "Memory"];
    for (const name of [...names, "Table", "Global", "Tag", "Exception", "Suspending", "promising"]) {
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
    // The readings, before anything is judged: a red run on another machine comes with its numbers.
    console.log(`the first page on ${name}: ${JSON.stringify(data)}`);
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
    const started = Date.now();
    await page.waitForTimeout(2000);
    const later = await report(page);
    const [posts, frames] = [later.posts - data.posts, later.frames - data.frames];
    console.log(`the first page on ${name}: ${frames} frames and ${posts} posts in ${Date.now() - started} ms`);
    expect(frames).toBeGreaterThan(3);
    expect(posts).toBeLessThanOrEqual(frames);
    expect(frames - posts).toBeLessThanOrEqual(1);
    expect(later.errors).toBe(0);
    // The page made no call into WebAssembly and no instantiation (PM-0002-2).
    expect(await page.evaluate(() => (window as unknown as { __wasmCalls: number }).__wasmCalls)).toBe(0);
    // And the Wasm runs in one Worker: no second one was started beside it.
    expect(page.workers()).toHaveLength(1);
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
  const atHide = await report(page);
  await setHidden(true);
  // The Worker's own count is what is judged: the page asks it for a report four times a second, and blocks that
  // were already on their way when the tab was hidden do not move it. Four more frames carry a request and its
  // answer on a page too slow to do that within the second.
  await page.waitForTimeout(1000);
  await drawn(page, atHide.frames + 4);
  const atRead = await report(page);
  const hidden = atRead.worker?.ticks ?? -1;
  await page.waitForTimeout(2000);
  const stillHidden = await report(page);
  console.log(
    `a hidden tab: ${hidden} ticks at the read after hiding, ${stillHidden.worker?.ticks} two seconds later; ` +
      `the newest block's tick ${atRead.tick}, then ${stillHidden.tick}`,
  );
  // Not one tick in two seconds; a match that ran on would have made 60.
  expect(stillHidden.worker?.ticks).toBe(hidden);
  // The Worker's count reaches the page in a report, and two reads of one old report are equal too. The newest
  // block's tick is the page's own: a match that ran on would have moved it, whatever the reports did.
  expect(stillHidden.tick).toBe(atRead.tick);
  const shown = performance.now();
  await setHidden(false);
  await page.waitForTimeout(1500);
  await drawn(page, stillHidden.frames + 4);
  const after = await report(page);
  // A tick never runs before it is due, so the time since showing bounds the ticks: 30 a second, and two for the ends.
  const most = Math.floor((30 * (performance.now() - shown)) / 1000) + 2;
  console.log(`a hidden tab: ${(after.worker?.ticks ?? 0) - hidden} ticks since showing (at most ${most}), pauses ${after.worker?.pauses}, drops ${after.worker?.drops}`);
  // It goes on from where it stopped: one pause, nothing dropped, and no more ticks than the time since.
  expect(after.worker?.pauses).toBe(1);
  expect(after.worker?.drops).toBe(0);
  expect(after.worker?.ticks ?? 0).toBeGreaterThan(hidden);
  expect((after.worker?.ticks ?? 999) - hidden).toBeLessThanOrEqual(most);
  expect(after.pauses).toBe(1);
  expect(after.errors).toBe(0);
});

const broken = [["cannot be loaded", null, "the Worker failed"], ["throws as it starts", 'throw new Error("boom");', "boom"]] as const;
for (const [name, body, said] of broken) {
  test(`a Worker that ${name} stops the match once and says so on the page`, async ({ page }) => {
    const thrown: string[] = [];
    page.on("pageerror", (error) => thrown.push(error.message));
    await page.route("**/assets/worker-*.js", (route) =>
      body === null ? route.fulfill({ status: 404, body: "" }) : route.fulfill({ contentType: "text/javascript", body }),
    );
    const started = Date.now();
    await page.goto("/");
    // The notice follows the picture's start, which is slow on a machine with no graphics card.
    await expect(page.locator("#notice")).toHaveText(new RegExp(`^The match stopped: .*${said}`), { timeout: 20_000 });
    console.log(`a Worker that ${name}: the notice stood ${Date.now() - started} ms after the page was asked for`);
    await page.waitForFunction(() => "__shambleReport" in window);
    expect((await report(page)).errors).toBe(1);
    expect(thrown).toEqual([]);
  });
}
