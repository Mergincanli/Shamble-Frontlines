/** What `copyBlock` needs of a `WebAssembly.Memory`: its buffer, asked for again on every call. */
export interface LinearMemory {
  readonly buffer: ArrayBuffer;
}

/**
 * Copies the block out of the Wasm's linear memory into `into`: the one copy per tick (ADR 0002 part 5). It reads
 * through a fresh view of `memory.buffer` on every call, never a kept one: when the memory grows, the old buffer
 * is detached and a kept view would read nothing.
 *
 * Throws a `RangeError` when `into` is not exactly `len` bytes, or when `ptr` and `len` do not lie inside the memory.
 */
export function copyBlock(memory: LinearMemory, ptr: number, len: number, into: ArrayBuffer): void {
  if (into.byteLength !== len) {
    throw new RangeError(`the block is ${len} bytes and the buffer for it is ${into.byteLength}`);
  }
  new Uint8Array(into).set(new Uint8Array(memory.buffer, ptr, len));
}
