import type { LinearMemory } from "./read-block.ts";

/**
 * How much text the hook's buffer holds (sim-wasm's panic.rs: 1,024 bytes less its two length words). The buffer says
 * so itself; a buffer that says anything else is not the hook's, and nothing of it is read: a damaged header must not
 * make the page show other memory. The boundary test reads this number from the real Wasm.
 */
export const PANIC_TEXT_ROOM = 1016;

/**
 * The text the Wasm's panic hook left in its buffer, or an empty string when nothing has panicked (sim-wasm's
 * panic.rs gives the layout: the text's length and the room for it as two little-endian `u32`, then the text).
 * It reads through a fresh view of the memory and calls nothing in the Wasm: after a trap no export may be called.
 * A buffer that makes no sense (it does not lie inside the memory, its room is not the hook's, or its length is
 * above its room) reads as no text, never as an error of its own.
 */
export function readPanicMessage(memory: LinearMemory, ptr: number): string {
  const bytes = memory.buffer.byteLength;
  if (!Number.isInteger(ptr) || ptr < 0 || ptr + 8 > bytes) return "";
  const header = new DataView(memory.buffer, ptr, 8);
  const [length, room] = [header.getUint32(0, true), header.getUint32(4, true)];
  if (length === 0 || room !== PANIC_TEXT_ROOM || length > room || ptr + 8 + length > bytes) return "";
  // Not fatal: a text cut inside a character still reads, with a mark in its place.
  return new TextDecoder().decode(new Uint8Array(memory.buffer, ptr + 8, length));
}

/**
 * What the Worker tells the page after a call into the Wasm threw: the panic's own text when the hook kept one,
 * otherwise what was thrown (a trap without a panic, a failed allocation, or an error that is not the Wasm's).
 */
export function trapMessage(memory: LinearMemory, ptr: number, thrown: unknown): string {
  const panic = readPanicMessage(memory, ptr);
  if (panic !== "") return `the simulation stopped: ${panic}`;
  return thrown instanceof Error ? thrown.message : String(thrown);
}
