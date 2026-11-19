import { Ring } from "@shamble/sim-worker";

/** How many frames the reading keeps: 10 seconds at 60 frames a second. */
const FRAME_SAMPLES = 600;

/** The measurement harness's CPU side: the time between drawn frames. `push` allocates nothing. */
export class FrameTimes {
  readonly #deltas = new Ring(FRAME_SAMPLES);
  #last: number | null = null;
  frames = 0;

  /** Call once per drawn frame with `performance.now()`. */
  push(nowMs: number): void {
    if (this.#last !== null) this.#deltas.push(nowMs - this.#last);
    this.#last = nowMs;
    this.frames += 1;
  }

  /** After a pause the next frame's distance from the last one is no frame time. */
  skip(): void {
    this.#last = null;
  }

  /** Frames per second over the frames kept; 0 before the second frame. */
  fps(): number {
    const median = this.#deltas.percentile(50);
    return median > 0 ? 1000 / median : 0;
  }

  /** The frame time at a percentile, in milliseconds. */
  percentile(p: number): number {
    return this.#deltas.percentile(p);
  }
}
