// The method of PM-0002-7 ("the boundary makes no garbage on the page"): a sampling heap profile through the Chrome
// DevTools Protocol that also counts objects already collected, with allocations attributed by function. The mark
// itself is judged in change-set 5, over a 10-minute match on a named machine; here the method runs for a few
// seconds, prints what it attributes to the page's hand-off code (WorkerLink's `onMessage` and `frame`, with all
// they call), and must be able to see a small leak: the limits below sit between the clean page and one planted
// 64-byte typed array per frame or per block.
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
  await running(page, 300);
  const session = await page.context().newCDPSession(page);
  await session.send("HeapProfiler.enable");
  const before = await report(page);
  await session.send("HeapProfiler.startSampling", {
    samplingInterval: 64,
    includeObjectsCollectedByMajorGC: true,
    includeObjectsCollectedByMinorGC: true,
  });
  await page.waitForTimeout(8000);
  const { profile } = (await session.send("HeapProfiler.stopSampling")) as { profile: { head: Node } };
  const after = await report(page);
  const frames = after.frames - before.frames;
  const blocks = after.blocks - before.blocks;
  const perFrame = under(profile.head, "frame") / frames;
  const perBlock = under(profile.head, "onMessage") / blocks;
  const rate = (under(profile.head, "frame") + under(profile.head, "onMessage")) / 8;
  console.log(
    `allocation in the page's hand-off code: ${perFrame.toFixed(0)} bytes per frame posting the input, ` +
      `${perBlock.toFixed(0)} bytes per block received, ${rate.toFixed(0)} bytes a second ` +
      `(${frames} frames, ${blocks} blocks, ${total(profile.head)} bytes sampled on the whole page)`,
  );
  expect(frames).toBeGreaterThan(20);
  expect(blocks).toBeGreaterThan(200);
  // The clean page allocates the input message per frame and one view per block: well under these limits.
  expect(perFrame).toBeLessThan(170);
  expect(perBlock).toBeLessThan(170);
  // PM-0002-7's own rate, far above what this page does: under 0.1 MB a second.
  expect(rate).toBeLessThan(100_000);
});
