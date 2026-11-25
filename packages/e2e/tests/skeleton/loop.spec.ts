// The loop in a real browser, on the built development page: a one-minute local match (PM-0002-64's Chromium
// half). The page's keys go to the Worker as posts, the Worker's endpoint and its local server exchange the real
// messages over the loopback, and what the server applied comes back in the block and lights the key diagram. The
// page itself makes no call into WebAssembly and asks nothing of any other address.
import { expect, test } from "@playwright/test";
import { needsBuild, open, report, wasmCalls } from "./report.ts";

test.beforeAll(() => void needsBuild());

test("a one-minute local match: keys go to the local server through the real messages and come back", async ({ page }) => {
  test.setTimeout(150_000);
  const outside = await open(page, "");
  const start = await report(page);
  const started = Date.now();
  const box = (button: number) => page.locator("#key-diagram span").nth(button);
  await expect(page.locator("#key-diagram span")).toHaveText(["W", "A", "S", "D", "Space", "Shift"]);

  // W held: its box lights from the server's echo and stays lit; let go, it goes dark.
  await page.keyboard.down("KeyW");
  await expect(box(0)).toHaveClass("lit");
  await page.waitForTimeout(500);
  await expect(box(0)).toHaveClass("lit");
  expect((await report(page)).presses).toMatchObject({ made: 1, echoed: 1 });
  await page.keyboard.up("KeyW");
  await expect(box(0)).not.toHaveClass("lit");
  // Ten quick taps of Space, each down and up at once: every one is echoed.
  for (let tap = 0; tap < 10; tap += 1) await page.keyboard.press("Space");
  // Then a key every quarter of a second for the rest of the minute: A, D and Shift in turn, held for 60 ms.
  let taps = 0;
  while (Date.now() - started < 60_000) {
    const code = ["KeyA", "KeyD", "ShiftLeft"][taps % 3] ?? "KeyA";
    await page.keyboard.down(code);
    await page.waitForTimeout(60);
    await page.keyboard.up(code);
    await page.waitForTimeout(190);
    taps += 1;
  }
  // The keys have been quiet long enough for the last echo to be back.
  await page.waitForFunction(() => (window as unknown as { __shambleReport(): { presses: { lost: number | null } } }).__shambleReport().presses.lost !== null);
  const end = await report(page);
  const seconds = (Date.now() - started) / 1000;
  const ticks = end.tick - start.tick;
  // The readings, before anything is judged. The key-to-tick time is PM-0002-13's first reading: printed, not judged.
  console.log(
    `the loop: ${ticks} ticks in ${seconds.toFixed(1)} s; messages ${JSON.stringify(end.messages)}; presses ${JSON.stringify(end.presses)}; ` +
      `releases ${JSON.stringify(end.releases)}; posts ${end.posts}, post used ${end.postUsed}, frames ${end.frames}; ` +
      `key to tick: median ${end.keyToTickP50.toFixed(1)} ms, 95th percentile ${end.keyToTickP95.toFixed(1)} ms over ${end.keyToTickCount} key changes; ` +
      `worker ${JSON.stringify(end.worker)}`,
  );
  // Every press and every release came back exactly once.
  expect(end.presses).toEqual({ made: 11 + taps, echoed: 11 + taps, lost: 0 });
  expect(end.releases).toEqual({ made: 11 + taps, echoed: 11 + taps, doubled: 0 });
  expect(end.olderPosts).toBe(0);
  // A key change is timed when a frame posts it; on a slow page a key's down and up share one frame.
  expect(end.keyToTickCount).toBeGreaterThan(taps / 2);
  // The match ran for the minute at its rate: a tick never runs early, and no more than a second's worth is missing.
  expect(ticks).toBeLessThanOrEqual(Math.ceil(30 * seconds) + 1);
  expect(ticks).toBeGreaterThan(30 * (seconds - 1) - (end.worker?.drops ?? 0) * 30);
  expect(Math.abs(end.blocks - end.tick)).toBeLessThanOrEqual(1);
  // One snapshot received per tick and one command sent per tick but the first (the join's), and none refused.
  expect(end.messages.received - start.messages.received).toBe(ticks);
  expect(end.messages.sent - start.messages.sent).toBe(ticks);
  expect(end.messages).toMatchObject({ received: end.tick, sent: end.tick - 1, refused: 0 });
  // Every tick ran on the newest post: the last command was built from one of the last few posts, never a later one.
  expect(end.postUsed).toBeLessThanOrEqual(end.posts);
  expect(end.posts - end.postUsed).toBeLessThanOrEqual(8);
  expect(end.worker).toMatchObject({ stalePosts: 0, undetached: 0 });
  expect(end.errors).toBe(0);
  // The page made no call into WebAssembly, runs one Worker, and asked nothing of any other address.
  expect(await wasmCalls(page)).toBe(0);
  expect(page.workers()).toHaveLength(1);
  expect(outside).toEqual([]);
});
