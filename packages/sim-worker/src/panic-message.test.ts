import { expect, test } from "vitest";
import { cutText, ERROR_TEXT_UNITS, PANIC_TEXT_ROOM, readPanicMessage, THROWN_TEXT_UNITS, trapMessage } from "./panic-message.ts";

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

test("a header whose room is not the hook's is not read at all", () => {
  // A damaged header could otherwise make the page decode whatever lies behind the buffer.
  expect(PANIC_TEXT_ROOM).toBe(1016);
  const text = utf8("panicked");
  for (const room of [0, 8, 1015, 1017, 4000, 0xffff_ffff]) {
    expect(readPanicMessage(memoryWith(64, text, { room }), 64), String(room)).toBe("");
  }
  expect(readPanicMessage(memoryWith(64, text, { length: 4000, room: 4000 }), 64)).toBe("");
  expect(readPanicMessage(memoryWith(64, text, { room: 1016 }), 64)).toBe("panicked");
});

test("the Worker's sentence after a throw: the panic's own text, or else what was thrown", () => {
  const panicked = memoryWith(64, utf8("panicked at crates/sim/src/world.rs:43:14:\nthe tick counter is at u32::MAX and cannot advance"));
  expect(trapMessage(panicked, 64, new Error("unreachable executed"))).toBe(
    "the simulation stopped: panicked at crates/sim/src/world.rs:43:14:\nthe tick counter is at u32::MAX and cannot advance",
  );
  // No panic was recorded (a trap without one, a stop for want of memory, an error that is not the Wasm's).
  const quiet = { buffer: new ArrayBuffer(4096) };
  expect(trapMessage(quiet, 64, new Error("unreachable executed"))).toBe("unreachable executed");
  expect(trapMessage(quiet, 64, "a thrown text")).toBe("a thrown text");
  expect(trapMessage({ buffer: new ArrayBuffer(0) }, 0, new RangeError("the block is 8 bytes and the buffer for it is 16"))).toBe(
    "the block is 8 bytes and the buffer for it is 16",
  );
});

test("a thrown text is cut to its bound with a mark, in code units and never inside a character; a panic's text is not", () => {
  const quiet = { buffer: new ArrayBuffer(4096) };
  expect(THROWN_TEXT_UNITS).toBe(512);
  // 1 MiB of text: its start and the mark, 512 code units in all.
  const cut = trapMessage(quiet, 64, new Error("k".repeat(1 << 20)));
  expect([cut.length, cut.endsWith("..."), cut.startsWith("kkk")]).toEqual([512, true, true]);
  // A text at the bound passes whole (a loader's longest sentence); one unit more is cut.
  expect(trapMessage(quiet, 64, new Error("a".repeat(512)))).toBe("a".repeat(512));
  expect(trapMessage(quiet, 64, new Error("a".repeat(513)))).toBe(`${"a".repeat(509)}...`);
  // A character of two code units across the cut goes whole: no lone half is left.
  const pair = cutText(`${"a".repeat(508)}😀${"b".repeat(10)}`);
  expect(pair).toBe(`${"a".repeat(508)}...`);
  expect(cutText(`${"a".repeat(507)}😀${"b".repeat(10)}`)).toBe(`${"a".repeat(507)}😀...`);
  // The panic's text is bounded by its buffer and told whole, the longest a whole room's text.
  const full = memoryWith(64, utf8("p".repeat(PANIC_TEXT_ROOM)));
  const told = trapMessage(full, 64, new Error("unreachable executed"));
  expect(told).toBe(`the simulation stopped: ${"p".repeat(PANIC_TEXT_ROOM)}`);
  expect([told.length, ERROR_TEXT_UNITS]).toEqual([1040, 1040]);
  // A room of characters that take two bytes each decodes to fewer code units than bytes: the bound holds for every text.
  const wide = trapMessage(memoryWith(64, utf8("é".repeat(PANIC_TEXT_ROOM / 2))), 64, null);
  expect(wide.length).toBeLessThanOrEqual(ERROR_TEXT_UNITS);
});
