import { BLOCK_LAYOUT, LAYOUT_HASH } from "./generated/block/layout.ts";

/** The answer of `checkLayoutHash`: the block has the layout these types describe, or why it does not. */
export type LayoutCheck =
  | { readonly ok: true }
  | { readonly ok: false; readonly reason: "too-short"; readonly byteLength: number }
  | { readonly ok: false; readonly reason: "layout-hash"; readonly expected: number; readonly found: number };

/**
 * Compares the layout hash in a block's header with the hash these types were generated with (ADR 0002 part 5).
 * The page calls it before it reads any other offset of a block, and refuses the block unless the answer is `ok`.
 * The hash is a `u32`, little-endian, as the block writer writes every number.
 *
 * It finds drift between the writer and the page; it is not a security check. A block that passes still has every
 * count and offset checked against its length by whoever reads it. Given an `ArrayBuffer` it never throws; the
 * caller makes sure it has one.
 */
export function checkLayoutHash(block: ArrayBuffer): LayoutCheck {
  const header = BLOCK_LAYOUT.header;
  // When the header grows (its counts), read the hash once its 4 bytes are there, before asking for the whole
  // header: a block in an older layout is then refused as "layout-hash", not as "too-short".
  if (block.byteLength < header.stride) {
    return { ok: false, reason: "too-short", byteLength: block.byteLength };
  }
  const found = new DataView(block).getUint32(header.fields.layout_hash.offset, true);
  if (found !== LAYOUT_HASH) {
    return { ok: false, reason: "layout-hash", expected: LAYOUT_HASH, found };
  }
  return { ok: true };
}
