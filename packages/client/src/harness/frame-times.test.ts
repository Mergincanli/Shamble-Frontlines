import { expect, test } from "vitest";
import { FrameTimes } from "./frame-times.ts";

test("frame times are the distances between pushes", () => {
  const times = new FrameTimes();
  expect(times.fps()).toBe(0);
  for (let frame = 0; frame <= 120; frame += 1) times.push(1000 + frame * 20);
  expect(times.frames).toBe(121);
  expect(times.fps()).toBeCloseTo(50, 9);
  expect(times.percentile(99)).toBeCloseTo(20, 9);
});

test("frames a second is a count over time, not the usual gap turned upside down", () => {
  // Three frames 4 ms apart, then one 20 ms later, over and over: 4 frames every 32 ms is 125 a second. The usual
  // gap is 4 ms, which would read as 250 (Firefox's readout said 239.2 where the run's average was 179.9).
  const times = new FrameTimes();
  let now = 0;
  times.push(now);
  for (let frame = 1; frame <= 400; frame += 1) {
    now += frame % 4 === 0 ? 20 : 4;
    times.push(now);
  }
  expect(times.percentile(50)).toBe(4);
  expect(times.fps()).toBeCloseTo(125, 9);
  // A pause is left out of the time as it is left out of the frame times.
  times.skip();
  times.push(now + 60_000);
  expect(times.fps()).toBeCloseTo(125, 9);
});

test("a pause between two frames is skipped, not counted as one long frame", () => {
  const times = new FrameTimes();
  times.push(0);
  times.push(16);
  times.skip();
  times.push(60_000);
  times.push(60_016);
  expect(times.percentile(100)).toBe(16);
});
