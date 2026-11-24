/**
 * The nearest-rank percentile of a list that is already sorted from small to large: the value at rank
 * `ceil(p / 100 * n)`, counted from 1 (how the drafts define p50, p95 and p99). `NaN` for an empty list.
 */
export function percentile(sorted: ArrayLike<number>, p: number): number {
  const n = sorted.length;
  if (n === 0) return Number.NaN;
  const rank = Math.min(n, Math.max(1, Math.ceil((p / 100) * n)));
  return sorted[rank - 1] ?? Number.NaN;
}

/**
 * The newest `capacity` samples of one measurement. Both buffers are allocated once, so `push` allocates nothing;
 * the readings (`percentile`, `mean`, `max`) are for the readout and the report, a few times a second at most.
 */
export class Ring {
  readonly #values: Float64Array;
  readonly #sorted: Float64Array;
  #next = 0;
  #count = 0;

  constructor(capacity: number) {
    if (!Number.isInteger(capacity) || capacity < 1) {
      throw new RangeError(`a ring holds a whole number of samples, 1 or more: ${capacity}`);
    }
    this.#values = new Float64Array(capacity);
    this.#sorted = new Float64Array(capacity);
  }

  /** How many samples it holds now: at most its capacity. */
  get count(): number {
    return this.#count;
  }

  /** Adds a sample; once the ring is full the oldest one goes. */
  push(value: number): void {
    this.#values[this.#next] = value;
    this.#next = (this.#next + 1) % this.#values.length;
    if (this.#count < this.#values.length) this.#count += 1;
  }

  /** The nearest-rank percentile of the samples held; `NaN` when there are none. */
  percentile(p: number): number {
    const held = this.#sorted.subarray(0, this.#count);
    held.set(this.#values.subarray(0, this.#count));
    held.sort();
    return percentile(held, p);
  }

  /** The mean of the samples held; `NaN` when there are none. */
  mean(): number {
    let sum = 0;
    for (let i = 0; i < this.#count; i += 1) sum += this.#values[i] ?? Number.NaN;
    return this.#count === 0 ? Number.NaN : sum / this.#count;
  }

  /** The largest sample held; `NaN` when there are none. */
  max(): number {
    let max = Number.NaN;
    for (let i = 0; i < this.#count; i += 1) {
      const value = this.#values[i] ?? Number.NaN;
      if (!(value <= max)) max = value;
    }
    return max;
  }
}
