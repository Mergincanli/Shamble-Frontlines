import { expect, test } from "vitest";
import { EchoTally, SETTLE_MS, unsettled } from "./press-tally.ts";

/** The four words of 16 counters. */
function words(counts: Partial<Record<number, number>>): [number, number, number, number] {
  const bytes = new Uint8Array(16);
  for (const [button, count] of Object.entries(counts)) bytes[Number(button)] = count ?? 0;
  const [p0 = 0, p1 = 0, p2 = 0, p3 = 0] = new Uint32Array(bytes.buffer);
  return [p0, p1, p2, p3];
}

test("presses are the counters' steps, and releases follow from them and the held bits", () => {
  const tally = new EchoTally();
  // Button 0 pressed and held; button 15 tapped (pressed and released inside one tick).
  tally.take(1, ...words({ 0: 1, 15: 1 }));
  expect([tally.presses, tally.releases]).toEqual([2, 1]);
  expect(tally.takePressed()).toBe(0x8001);
  // Nothing new: the same echo again.
  tally.take(1, ...words({ 0: 1, 15: 1 }));
  expect([tally.presses, tally.releases, tally.takePressed()]).toEqual([2, 1, 0]);
  // Button 0 let go; then pressed twice more and held again, with a block skipped in between.
  tally.take(0, ...words({ 0: 1, 15: 1 }));
  expect([tally.presses, tally.releases]).toEqual([2, 2]);
  tally.take(1, ...words({ 0: 3, 15: 1 }));
  expect([tally.presses, tally.releases]).toEqual([4, 3]);
  // What was pressed since the last asking is kept over several echoes, then forgotten.
  tally.take(1 | (1 << 7), ...words({ 0: 3, 7: 1, 15: 1 }));
  expect(tally.takePressed()).toBe(0x0081);
  expect(tally.takePressed()).toBe(0);
});

test("a button held again with no press is counted apart: the server took up an older state of the keys", () => {
  const tally = new EchoTally();
  // Pressed and held, let go, pressed and held again: every hold has its press.
  tally.take(1, ...words({ 0: 1 }));
  tally.take(0, ...words({ 0: 1 }));
  tally.take(1, ...words({ 0: 2 }));
  // A tap of another button while button 0 stays held changes nothing for button 0.
  tally.take(1, ...words({ 0: 2, 3: 1 }));
  expect(tally.heldWithoutPress).toBe(0);
  // Let go, then held again with the counter where it was: no press was made. The releases do not show it.
  tally.take(0, ...words({ 0: 2, 3: 1 }));
  tally.take(1, ...words({ 0: 2, 3: 1 }));
  tally.take(0, ...words({ 0: 2, 3: 1 }));
  expect([tally.heldWithoutPress, tally.presses, tally.releases]).toEqual([1, 3, 3]);
});

test("a counter's wrap at 256 is one step, and every button has its own byte", () => {
  const tally = new EchoTally();
  for (let button = 0; button < 16; button += 1) {
    tally.take(0, ...words({ [button]: 255 }));
    tally.take(0, ...words({ [button]: 1 }));
    tally.take(0, ...words({}));
  }
  // Per button: 255, then 2 more across the wrap, then 255 more back to 0 (a counter never runs backwards).
  expect([tally.presses, tally.releases]).toEqual([16 * 512, 16 * 512]);
});

test("a press counts as lost only once the keys have been quiet for the settle time", () => {
  expect(unsettled(10, 9, 1000, 1000 + SETTLE_MS - 1)).toBeNull();
  expect(unsettled(10, 9, 1000, 1000 + SETTLE_MS)).toBe(1);
  expect(unsettled(10, 10, 1000, 5000)).toBe(0);
  // More echoed than made is a doubled press: a negative number.
  expect(unsettled(10, 12, 1000, 5000)).toBe(-2);
});
