// The method of PM-0002-7 ("the boundary makes no garbage on the page"): a sampling heap profile through the Chrome
// DevTools Protocol that also counts objects already collected, with allocations attributed by function. The mark
// itself is judged in change-set 5, over a 10-minute match on a named machine; here the method runs for a few
// seconds, prints what it attributes to the page's hand-off code (WorkerLink's `receive`, which unpacks the
// message event, and `frame`, with all they call), and must be able to see a small leak: the upper limits below sit between the clean page and one
// planted 64-byte typed array per frame or per block.
//
// What this method cannot do, found by perf-auditor: once the browser optimises the page's code it charges an
// inlined function to its caller, and the sums by name fall towards 0. The message listener reaches that after
// about 470 blocks (the same count on every run in the container), a frame callback after some hundreds of
// frames. So the window is taken early, before that point, and each sum also has a floor: a run in which the
// names were lost fails instead of passing blind. Change-set 5 measures the mark itself another way (the whole
// page's bytes with the hand-off on and off, and memory outside the heap).
import { expect, test } from "@playwright/test";
import { needsBuild, report, running } from "./report.ts";

interface Node {
  callFrame: { functionName: string };
  selfSize: number;
  children?: Node[];
}
const total = (node: Node): number => node.selfSize + (node.children ?? []).reduce((sum, child) => sum + total(child), 0);
/** The bytes allocated inside every call of a function of this name, with everything it calls. */
const under = (node: Node, name: string): number =>
  node.callFrame.functionName === name ? total(node) : (node.children ?? []).reduce((sum, child) => sum + under(child, name), 0);

test("the sampling heap profile attributes the hand-off's allocations by function", async ({ page }) => {
  needsBuild();
  await page.goto("/");
  await running(page); // from block 90: the 8 s window ends near block 340, before the listener is optimised
  const session = await page.context().newCDPSession(page);
  await session.send("HeapProfiler.enable");
  await session.send("HeapProfiler.startSampling", {
    samplingInterval: 64,
    includeObjectsCollectedByMajorGC: true,
    includeObjectsCollectedByMinorGC: true,
  });
  // The counts are read inside the sampled window, as close to its two ends as a call can be.
  const before = await report(page);
  await page.waitForTimeout(8000);
  const after = await report(page);
  const { profile } = (await session.send("HeapProfiler.stopSampling")) as { profile: { head: Node } };
  const frames = after.frames - before.frames;
  const blocks = after.blocks - before.blocks;
  const perFrame = under(profile.head, "frame") / frames;
  const perBlock = under(profile.head, "receive") / blocks;
  const rate = (under(profile.head, "frame") + under(profile.head, "receive")) / 8;
  console.log(
    `allocation in the page's hand-off code: ${perFrame.toFixed(0)} bytes per frame posting the input, ` +
      `${perBlock.toFixed(0)} bytes per block received, ${rate.toFixed(0)} bytes a second ` +
      `(${frames} frames, blocks ${before.blocks} to ${after.blocks}, ${total(profile.head)} bytes sampled on the whole page)`,
  );
  // The window must end before the browser optimises the listener (about block 470), or the sums below mean less.
  expect(after.blocks).toBeLessThan(420);
  expect(frames).toBeGreaterThan(20);
  expect(blocks).toBeGreaterThan(200);
  // The clean page allocates the input message per frame (about 65 bytes) and, per block, the message event's own
  // data and one view (about 505 bytes before the listener is optimised). Each limit sits between that and one
  // planted 64-byte typed array more (about 170 bytes more per frame, about 215 per block); each floor tells a run
  // that lost the names to inlining, in part or whole, from a clean one.
  expect(perFrame).toBeGreaterThan(25);
  expect(perFrame).toBeLessThan(170);
  expect(perBlock).toBeGreaterThan(430);
  expect(perBlock).toBeLessThan(610);
  // PM-0002-7's own rate, far above what this page does: under 0.1 MB a second.
  expect(rate).toBeLessThan(100_000);
});
