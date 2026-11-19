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

test("a pause between two frames is skipped, not counted as one long frame", () => {
  const times = new FrameTimes();
  times.push(0);
  times.push(16);
  times.skip();
  times.push(60_000);
  times.push(60_016);
  expect(times.percentile(100)).toBe(16);
});
