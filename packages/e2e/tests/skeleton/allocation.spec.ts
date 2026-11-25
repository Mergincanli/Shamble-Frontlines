// The method of PM-0002-7 ("the boundary makes no garbage on the page"): a sampling heap profile through the Chrome
// DevTools Protocol that also counts objects already collected, with allocations attributed by function. The mark
// itself is judged in change-set 5, over a 10-minute match on a named machine; here the method runs for a few
// seconds, prints what it attributes to the page's hand-off code, and must be able to see a small leak: the upper
// limits below sit between the clean page and one planted 64-byte typed array per frame or per block. Per block
// that code is the message listener's whole body (`blockArrived`, which calls WorkerLink's `receive`: it unpacks
// the message event and reads the tick, the echo and the link's counts). Per frame it is WorkerLink's `frame`,
// which reads the keys and posts the input, and the key diagram's `follow`, each with all they call.
//
// What this method cannot do, found by perf-auditor: once the browser optimises the page's code it charges an
// inlined function to its caller, and the sums by name fall towards 0. The message listener reaches that after
// about 470 blocks (the same count on every run in the container), a frame callback after some hundreds of
// frames. So the window is taken early, before that point, and each sum also has a floor: a run in which the
// names were lost fails instead of passing blind. Change-set 5 measures the mark itself another way (the whole
// page's bytes with the hand-off on and off, and memory outside the heap).
//
// And, found by test-engineer: what this method charges to a block depends on how fast the page draws. Measured
// at six viewport sizes in the container (2026-11-25): 478 bytes per block at 0.9 frames a second, 567 at 5.9,
// 591 at 10.1, 765 at 23.2, 747 at 38.7, 629 at 60.0. The cause, found by perf-auditor: the browser's optimising
// compilers finish their background work on the drawing code, which warms up at the frame rate, and installing
// that code is charged to whatever JavaScript runs at the next check, usually the listener's own frame, whose
// code allocates nothing (with those compilers switched off the listener's total is flat at 545 to 600). The
// hand-off's own work, the sum under WorkerLink's `receive`, is 523 to 568 bytes per block at every rate; it is
// printed below. The per-block limit is right only for a slow page, as the container (about 6 frames a second, 5
// blocks per frame) and GitHub's runner (about 9, 3.3 per frame) are, where that noise is 0 to 41 bytes: a run
// with fewer than 2.5 blocks per frame is refused by name instead of being judged against a limit that was
// never measured for it. Change-set 5's method must not fall into the same trap.
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
  const started = Date.now();
  await page.waitForTimeout(8000);
  const after = await report(page);
  const windowMs = Date.now() - started;
  const { profile } = (await session.send("HeapProfiler.stopSampling")) as { profile: { head: Node } };
  const frames = after.frames - before.frames;
  const blocks = after.blocks - before.blocks;
  const inFrames = under(profile.head, "frame") + under(profile.head, "follow");
  const perFrame = inFrames / frames;
  const perBlock = under(profile.head, "blockArrived") / blocks;
  const rate = (inFrames + under(profile.head, "blockArrived")) / 8;
  console.log(
    `allocation in the page's hand-off code: ${perFrame.toFixed(0)} bytes per frame posting the input and lighting the keys ` +
      `(${under(profile.head, "follow")} bytes of it in the key diagram, over the whole window), ` +
      `${perBlock.toFixed(0)} bytes per block received (${(under(profile.head, "receive") / blocks).toFixed(0)} of them in the link's own work), ` +
      `${rate.toFixed(0)} bytes a second ` +
      `(${frames} frames in ${windowMs} ms, blocks ${before.blocks} to ${after.blocks}, ${after.worker?.drops} drops, ` +
      `${total(profile.head)} bytes sampled on the whole page)`,
  );
  // The window must end before the browser optimises the listener (about block 470), or the sums below mean less.
  expect(after.blocks).toBeLessThan(420);
  expect(frames).toBeGreaterThan(20);
  expect(blocks).toBeGreaterThan(200);
  const blocksPerFrame = blocks / frames;
  expect(blocksPerFrame, "the page draws faster than the per-block limit was measured for: measure again").toBeGreaterThan(2.5);
  // The clean page allocates, per frame, the input message and its list of returned buffers (about 75 bytes and 8
  // more per buffer; the key diagram none) and, per block, the message event's own data and one view (520 to 610
  // bytes before the listener is optimised). Each limit sits between that and one planted 64-byte typed array
  // more (about 170 bytes more per frame, about 215 per block); each floor tells a run that lost the names to
  // inlining, in part or whole, from a clean one.
  expect(perFrame).toBeGreaterThan(25);
  expect(perFrame).toBeLessThan(130 + 8 * blocksPerFrame);
  expect(perBlock).toBeGreaterThan(430);
  expect(perBlock).toBeLessThan(680);
  // PM-0002-7's own rate, far above what this page does: under 0.1 MB a second.
  expect(rate).toBeLessThan(100_000);
});
