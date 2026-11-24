import { expect, test } from "vitest";
import { readPanicMessage } from "./panic-message.ts";

/** A memory with a panic buffer at `ptr`: the length word, the room word, then the text's bytes. */
function memoryWith(ptr: number, text: Uint8Array, options: { length?: number; room?: number; bytes?: number } = {}) {
  const buffer = new ArrayBuffer(options.bytes ?? 4096);
  const view = new DataView(buffer);
  view.setUint32(ptr, options.length ?? text.length, true);
  view.setUint32(ptr + 4, options.room ?? 1016, true);
  new Uint8Array(buffer).set(text, ptr + 8);
  return { buffer };
}
const utf8 = (text: string): Uint8Array => new TextEncoder().encode(text);

test("the text the hook left is read whole, with its line break and any character", () => {
  const text = "panicked at crates/sim/src/world.rs:43:14:\nthe tick counter is at u32::MAX and cannot advance";
  expect(readPanicMessage(memoryWith(1024, utf8(text)), 1024)).toBe(text);
  expect(readPanicMessage(memoryWith(64, utf8("zähler übergelaufen ✓")), 64)).toBe("zähler übergelaufen ✓");
  // Only the length's bytes are read: what lies behind them in the buffer is an older, longer text.
  expect(readPanicMessage(memoryWith(64, utf8("newer text, older rest"), { length: 10 }), 64)).toBe("newer text");
});

test("no panic reads as no text", () => {
  expect(readPanicMessage(memoryWith(64, utf8("left over"), { length: 0 }), 64)).toBe("");
  expect(readPanicMessage({ buffer: new ArrayBuffer(4096) }, 64)).toBe("");
});

test("a buffer that makes no sense reads as no text and throws nothing", () => {
  const text = utf8("panicked");
  // A length above the room, a text that would end outside the memory, a place outside it, and no place at all.
  expect(readPanicMessage(memoryWith(64, text, { length: 1017 }), 64)).toBe("");
  expect(readPanicMessage(memoryWith(4080, text, { length: 9, bytes: 4096 }), 4080)).toBe("");
  expect(readPanicMessage(memoryWith(4088, new Uint8Array(0), { length: 1 }), 4088)).toBe("");
  for (const ptr of [4089, 4096, 1e9, -8, 1.5, Number.NaN, undefined as unknown as number]) {
    expect(readPanicMessage(memoryWith(64, text), ptr), String(ptr)).toBe("");
  }
  // A text cut inside a character is still read, with the mark for what is missing.
  expect(readPanicMessage(memoryWith(64, utf8("né").slice(0, 2)), 64)).toBe("n\ufffd");
});
