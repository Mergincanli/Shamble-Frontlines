/**
 * The pool of buffers the block travels in (ADR 0002 part 5: the block is "copied once into a pooled `ArrayBuffer`
 * and transferred to the page", which later sends the buffers it is done with back). Every buffer has the block's
 * size. An empty pool never drops a tick: `take` makes a new buffer and counts a miss.
 */
export class BlockPool {
  readonly #blockBytes: number;
  readonly #size: number;
  readonly #free: ArrayBuffer[] = [];
  #misses = 0;
  #refused = 0;

  constructor(blockBytes: number, size: number) {
    if (!Number.isInteger(blockBytes) || blockBytes < 1 || !Number.isInteger(size) || size < 1) {
      throw new RangeError(`a pool needs a block size and a count, both whole numbers above 0: ${blockBytes}, ${size}`);
    }
    this.#blockBytes = blockBytes;
    this.#size = size;
    for (let i = 0; i < size; i += 1) this.#free.push(new ArrayBuffer(blockBytes));
  }

  /** How often `take` found the pool empty and had to allocate. */
  get misses(): number {
    return this.#misses;
  }

  /** How many returned buffers were not taken back: of another size, or detached. */
  get refused(): number {
    return this.#refused;
  }

  /** A buffer of the block's size: a pooled one, or a new one when none is left. */
  take(): ArrayBuffer {
    const pooled = this.#free.pop();
    if (pooled !== undefined) return pooled;
    this.#misses += 1;
    return new ArrayBuffer(this.#blockBytes);
  }

  /**
   * Takes a buffer back from the page. One of another size is refused and counted; a detached buffer has length 0
   * and is refused the same way. A good buffer that finds the pool full is let go, which is not an error: it was
   * made on a miss.
   */
  give(buffer: ArrayBuffer): void {
    if (buffer.byteLength !== this.#blockBytes) {
      this.#refused += 1;
      return;
    }
    if (this.#free.length < this.#size) this.#free.push(buffer);
  }
}
