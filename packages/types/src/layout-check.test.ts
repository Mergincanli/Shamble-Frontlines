import { expect, test } from "vitest";
import { BLOCK_LAYOUT, LAYOUT_HASH } from "./generated/block/layout.ts";
import { checkLayoutHash } from "./layout-check.ts";

// The hash sits at byte 4 of every block, whatever else the layout holds (the Rust test layout_rules_hold pins it).
const HASH_AT = 4;
const header = BLOCK_LAYOUT.header;

/** A block of the header's size with these four bytes where the hash sits. */
function blockWith(bytes: readonly [number, number, number, number], byteLength: number = header.stride): ArrayBuffer {
  const block = new ArrayBuffer(byteLength);
  new Uint8Array(block).set(bytes, HASH_AT);
  return block;
}

/** The hash's four bytes, lowest first, as the block writer writes them. */
function lowestFirst(hash: number): [number, number, number, number] {
  return [hash & 0xff, (hash >>> 8) & 0xff, (hash >>> 16) & 0xff, hash >>> 24];
}

test("a block that carries the generated hash passes", () => {
  expect(checkLayoutHash(blockWith(lowestFirst(LAYOUT_HASH)))).toEqual({ ok: true });
  expect(checkLayoutHash(blockWith(lowestFirst(LAYOUT_HASH), header.stride + 64))).toEqual({ ok: true });
});

test("a hash one bit off fails and names both values", () => {
  // The lowest bit, then the highest: a read as a signed number would report the second as a negative value.
  for (const bit of [0, 31]) {
    const found = (LAYOUT_HASH ^ (1 << bit)) >>> 0;
    expect(checkLayoutHash(blockWith(lowestFirst(found)))).toEqual({
      ok: false,
      reason: "layout-hash",
      expected: LAYOUT_HASH,
      found,
    });
  }
});

test("a block of zero bytes fails", () => {
  expect(checkLayoutHash(new ArrayBuffer(header.stride))).toEqual({
    ok: false,
    reason: "layout-hash",
    expected: LAYOUT_HASH,
    found: 0,
  });
});

test("a block shorter than the header fails and is not read", () => {
  for (const byteLength of [0, HASH_AT, header.stride - 1]) {
    expect(checkLayoutHash(new ArrayBuffer(byteLength))).toEqual({ ok: false, reason: "too-short", byteLength });
  }
});

test("the hash is read little-endian", () => {
  expect(checkLayoutHash(blockWith([0x01, 0x02, 0x03, 0x04]))).toEqual({
    ok: false,
    reason: "layout-hash",
    expected: LAYOUT_HASH,
    found: 0x04030201,
  });
  const highestFirst = lowestFirst(LAYOUT_HASH).reverse() as [number, number, number, number];
  expect(checkLayoutHash(blockWith(highestFirst)).ok).toBe(highestFirst.join() === lowestFirst(LAYOUT_HASH).join());
});
