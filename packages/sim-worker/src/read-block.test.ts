import { expect, test } from "vitest";
import { copyBlock } from "./read-block.ts";

function memoryWith(bytes: number[], at: number): WebAssembly.Memory {
  const memory = new WebAssembly.Memory({ initial: 1 });
  new Uint8Array(memory.buffer).set(bytes, at);
  return memory;
}

test("the block is copied out byte for byte", () => {
  const memory = memoryWith([1, 2, 3, 4, 5, 6, 7, 8], 1024);
  const into = new ArrayBuffer(8);
  copyBlock(memory, 1024, 8, into);
  expect([...new Uint8Array(into)]).toEqual([1, 2, 3, 4, 5, 6, 7, 8]);
  // A copy, not a view: a later write in the memory does not reach it.
  new Uint8Array(memory.buffer)[1024] = 99;
  expect(new Uint8Array(into)[0]).toBe(1);
});

test("after the memory grows, the next copy still reads the block", () => {
  const memory = memoryWith([9, 8, 7, 6], 64);
  const before = memory.buffer;
  memory.grow(1);
  // The old buffer is gone: a host that kept a view of it would read nothing.
  expect(before.byteLength).toBe(0);
  const into = new ArrayBuffer(4);
  copyBlock(memory, 64, 4, into);
  expect([...new Uint8Array(into)]).toEqual([9, 8, 7, 6]);
});

test("a buffer of the wrong size, or a block outside the memory, is refused", () => {
  const memory = memoryWith([1, 2, 3, 4], 0);
  expect(() => copyBlock(memory, 0, 4, new ArrayBuffer(8))).toThrow(RangeError);
  expect(() => copyBlock(memory, 0, 4, new ArrayBuffer(0))).toThrow(RangeError);
  expect(() => copyBlock(memory, 65536 - 2, 4, new ArrayBuffer(4))).toThrow(RangeError);
});
