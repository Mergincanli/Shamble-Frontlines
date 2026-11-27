import type { LinearMemory } from "./read-block.ts";

/**
 * How much text the hook's buffer holds (sim-wasm's panic.rs: 1,024 bytes less its two length words). The buffer says
 * so itself; a buffer that says anything else is not the hook's, and nothing of it is read: a damaged header must not
 * make the page show other memory. The boundary test reads this number from the real Wasm.
 */
export const PANIC_TEXT_ROOM = 1016;

/** What the Worker says before a panic's text. */
export const PANIC_LEAD = "the simulation stopped: ";

/**
 * The most UTF-16 code units of a thrown value's text that the Worker tells the page (anything but a panic's text,
 * whose buffer has a room of its own): the game's loader cuts its sentences to the same number of bytes, all ASCII
 * (content's `MESSAGE_BYTES`, which the boundary test reads from the Rust source), so a loader's refusal passes whole.
 * The page cuts the Worker's own error event to it too.
 */
export const THROWN_TEXT_UNITS = 512;

/**
 * The longest `error` text the page takes, in UTF-16 code units: a whole panic buffer after the lead (decoding the
 * room's 1,016 bytes gives at most 1,016 code units). Any longer text is not from this build's Worker.
 */
export const ERROR_TEXT_UNITS = PANIC_TEXT_ROOM + PANIC_LEAD.length;

/** Marks where a text was cut, inside the bound. */
const CUT_MARK = "...";

/**
 * A text of at most `THROWN_TEXT_UNITS` UTF-16 code units: the text itself when it fits, otherwise its start and the
 * mark, never half of a character (a high surrogate whose low half would be cut off goes too).
 */
export function cutText(text: string): string {
  if (text.length <= THROWN_TEXT_UNITS) return text;
  let keep = THROWN_TEXT_UNITS - CUT_MARK.length;
  const last = text.charCodeAt(keep - 1);
  if (last >= 0xd800 && last <= 0xdbff) keep -= 1;
  return `${text.slice(0, keep)}${CUT_MARK}`;
}

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
 * What the Worker tells the page after a call into the Wasm threw: the panic's own text when the hook kept one, whole
 * (its buffer bounds it), otherwise what was thrown, cut to `THROWN_TEXT_UNITS` (a refusal of the constructor, a trap
 * without a panic, a failed allocation, or an error that is not the Wasm's).
 */
export function trapMessage(memory: LinearMemory, ptr: number, thrown: unknown): string {
  const panic = readPanicMessage(memory, ptr);
  if (panic !== "") return `${PANIC_LEAD}${panic}`;
  return cutText(thrown instanceof Error ? thrown.message : String(thrown));
}
