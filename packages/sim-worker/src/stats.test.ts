import { expect, test } from "vitest";
import { percentile, Ring } from "./stats.ts";

test("the nearest-rank percentile of a sorted list", () => {
  const ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  expect(percentile(ten, 50)).toBe(5);
  expect(percentile(ten, 90)).toBe(9);
  expect(percentile(ten, 91)).toBe(10);
  expect(percentile(ten, 99)).toBe(10);
  expect(percentile(ten, 100)).toBe(10);
  expect(percentile(ten, 0)).toBe(1);
  expect(percentile([7], 99)).toBe(7);
  expect(percentile([], 50)).toBeNaN();
  // 200 samples: p99 is rank 198, the third from the top.
  const many = Array.from({ length: 200 }, (_, i) => i + 1);
  expect(percentile(many, 99)).toBe(198);
  expect(percentile(many, 95)).toBe(190);
});

test("a ring keeps the newest samples and forgets the oldest", () => {
  const ring = new Ring(4);
  expect(ring.count).toBe(0);
  expect(ring.percentile(50)).toBeNaN();
  expect(ring.max()).toBeNaN();
  for (const value of [5, 1, 9]) ring.push(value);
  expect(ring.count).toBe(3);
  expect(ring.percentile(50)).toBe(5);
  expect(ring.max()).toBe(9);
  // Three more: 5, 1 and 9 are pushed out one by one; 9 leaves last.
  for (const value of [2, 3, 4]) ring.push(value);
  expect(ring.count).toBe(4);
  expect(ring.max()).toBe(9);
  expect(ring.percentile(100)).toBe(9);
  ring.push(0);
  expect(ring.max()).toBe(4);
  expect(ring.percentile(50)).toBe(2);
  expect(ring.percentile(1)).toBe(0);
});

test("reading a ring does not disturb it, and clear empties it", () => {
  const ring = new Ring(3);
  for (const value of [3, 1, 2]) ring.push(value);
  expect(ring.percentile(99)).toBe(3);
  expect(ring.percentile(99)).toBe(3);
  ring.push(10);
  // The oldest sample was 3, not the smallest: sorting for a reading must not reorder what is held.
  expect(ring.percentile(1)).toBe(1);
  expect(ring.max()).toBe(10);
  ring.clear();
  expect(ring.count).toBe(0);
  expect(ring.max()).toBeNaN();
});

test("a ring refuses a capacity that is not a whole number above 0", () => {
  for (const capacity of [0, -1, 1.5, Number.NaN]) {
    expect(() => new Ring(capacity)).toThrow(RangeError);
  }
});
