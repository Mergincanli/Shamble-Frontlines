import { expect, test } from "vitest";
import { BlockPool } from "./block-pool.ts";

test("a pool hands out its own buffers, each of the block's size", () => {
  const pool = new BlockPool(8, 3);
  expect(pool.free).toBe(3);
  const taken = [pool.take(), pool.take(), pool.take()];
  expect(new Set(taken).size).toBe(3);
  for (const buffer of taken) expect(buffer.byteLength).toBe(8);
  expect(pool.free).toBe(0);
  expect(pool.misses).toBe(0);
});

test("an empty pool makes a new buffer and counts the miss", () => {
  const pool = new BlockPool(16, 1);
  const first = pool.take();
  const second = pool.take();
  expect(second).not.toBe(first);
  expect(second.byteLength).toBe(16);
  expect(pool.misses).toBe(1);
  pool.take();
  expect(pool.misses).toBe(2);
});

test("a returned buffer is used again", () => {
  const pool = new BlockPool(8, 2);
  const buffer = pool.take();
  pool.take();
  pool.give(buffer);
  expect(pool.free).toBe(1);
  expect(pool.take()).toBe(buffer);
  expect(pool.misses).toBe(0);
});

test("a buffer of another size, or a detached one, is refused and counted", () => {
  const pool = new BlockPool(8, 2);
  pool.take();
  pool.give(new ArrayBuffer(16));
  expect(pool.refused).toBe(1);
  expect(pool.free).toBe(1);
  // A buffer that was transferred away has length 0.
  const sent = pool.take();
  structuredClone(sent, { transfer: [sent] });
  expect(sent.byteLength).toBe(0);
  pool.give(sent);
  expect(pool.refused).toBe(2);
  expect(pool.free).toBe(0);
});

test("the pool never grows past its size", () => {
  const pool = new BlockPool(8, 1);
  const first = pool.take();
  const extra = pool.take();
  pool.give(first);
  pool.give(extra);
  expect(pool.free).toBe(1);
  expect(pool.refused).toBe(0);
});

test("a pool refuses sizes that are not whole numbers above 0", () => {
  for (const [bytes, size] of [[0, 1], [8, 0], [1.5, 1], [8, -1]] as const) {
    expect(() => new BlockPool(bytes, size)).toThrow(RangeError);
  }
});
