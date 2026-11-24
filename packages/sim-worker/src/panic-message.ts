import type { LinearMemory } from "./read-block.ts";

/**
 * The text the Wasm's panic hook left in its buffer, or an empty string when nothing has panicked (sim-wasm's
 * panic.rs gives the layout: the text's length and the room for it as two little-endian `u32`, then the text).
 * It reads through a fresh view of the memory and calls nothing in the Wasm: after a trap no export may be called.
 * A buffer that makes no sense (it does not lie inside the memory, or its length is above its room) reads as no
 * text, never as an error of its own.
 */
export function readPanicMessage(memory: LinearMemory, ptr: number): string {
  const bytes = memory.buffer.byteLength;
  if (!Number.isInteger(ptr) || ptr < 0 || ptr + 8 > bytes) return "";
  const header = new DataView(memory.buffer, ptr, 8);
  const [length, room] = [header.getUint32(0, true), header.getUint32(4, true)];
  if (length === 0 || length > room || ptr + 8 + length > bytes) return "";
  // Not fatal: a text cut inside a character still reads, with a mark in its place.
  return new TextDecoder().decode(new Uint8Array(memory.buffer, ptr + 8, length));
}
