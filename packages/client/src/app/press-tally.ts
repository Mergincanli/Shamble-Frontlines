/**
 * The page's count of what the server echoed: presses and releases, as the blocks' `echo` section shows them. The
 * page counts its own key presses (`KeyboardInput`); when both counts agree once the keys have been quiet for a
 * moment, no press and no release was lost or doubled on the way to the server and back.
 */
export class EchoTally {
  #held = 0;
  readonly #counts = new Uint8Array(16);
  #presses = 0;
  #releases = 0;
  #pressed = 0;

  get presses(): number {
    return this.#presses;
  }

  get releases(): number {
    return this.#releases;
  }

  /**
   * Takes one block's echo: the held bits and the 16 press counters, four to a word. Each counter's step since the
   * last block, around 256, is that many presses. A button pressed `n` times that is held now as it was before was
   * also released `n` times, so the releases follow from the presses and the two held bits, and add up rightly
   * whatever blocks are skipped in between.
   */
  take(held: number, p0: number, p1: number, p2: number, p3: number): void {
    for (let button = 0; button < 16; button += 1) {
      const word = button < 4 ? p0 : button < 8 ? p1 : button < 12 ? p2 : p3;
      const count = (word >>> (8 * (button & 3))) & 0xff;
      const presses = (count - (this.#counts[button] ?? 0)) & 0xff;
      const bit = 1 << button;
      this.#presses += presses;
      this.#releases += presses + Number((this.#held & bit) !== 0) - Number((held & bit) !== 0);
      if (presses !== 0) this.#pressed |= bit;
      this.#counts[button] = count;
    }
    this.#held = held;
  }

  /** The buttons that were pressed in the echoes taken since the last call, one bit each; the call forgets them. */
  takePressed(): number {
    const pressed = this.#pressed;
    this.#pressed = 0;
    return pressed;
  }
}

/** How long the keys must have been quiet before a press that has not come back counts as lost: its echo takes a few ticks. */
export const SETTLE_MS = 500;

/**
 * How many of `made` did not come back in `echoed`, or came back too often (a negative number), once the last key
 * change is `SETTLE_MS` old; `null` until then, while the newest echo may still be on its way.
 */
export function unsettled(made: number, echoed: number, changedAtMs: number, nowMs: number): number | null {
  return nowMs - changedAtMs < SETTLE_MS ? null : made - echoed;
}
