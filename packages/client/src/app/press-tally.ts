/**
 * The page's count of what the server echoed: presses and releases, as the blocks' `echo` section shows them. The
 * page counts its own key presses (`KeyboardInput`); when both counts agree once the keys have been quiet for a
 * moment, no press and no release was lost on the way to the server and back.
 *
 * The presses are counted: each is a step of a counter, kept per button. The releases are worked out, because the
 * echo has no counter for them: a button pressed `n` times that is held now as it was before was also released `n`
 * times. So at the end of a run a lost release shows as a release that never came back (the button stays held);
 * in the middle of a run it only comes late, at that button's next press, and no count here moves. A doubled one
 * shows as a button held again with no press (`heldWithoutPress`). Which keys the server holds at each tick,
 * early, late or rolled back, is the link's check against the post that tick used (`WorkerLink.wrongKeys`).
 */
export class EchoTally {
  #held = 0;
  readonly #counts = new Uint8Array(16);
  readonly #echoed = new Int32Array(16);
  #presses = 0;
  #releases = 0;
  #heldWithoutPress = 0;
  #pressed = 0;

  get presses(): number {
    return this.#presses;
  }

  get releases(): number {
    return this.#releases;
  }

  /** The presses of each button the echo has shown so far, not wrapped. */
  get pressesByButton(): Int32Array {
    return this.#echoed;
  }

  /**
   * Times a button showed as held, after not being held, with no press in between: the server took up an older
   * state of the keys. On a page whose posts reach the server in order this stays 0.
   */
  get heldWithoutPress(): number {
    return this.#heldWithoutPress;
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
      this.#echoed[button] = (this.#echoed[button] ?? 0) + presses;
      this.#releases += presses + Number((this.#held & bit) !== 0) - Number((held & bit) !== 0);
      if (presses === 0 && (held & bit) !== 0 && (this.#held & bit) === 0) this.#heldWithoutPress += 1;
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
 * Once the last key change is `SETTLE_MS` old: how many presses did not come back (`lost`) and how many came back
 * that were never made (`doubled`), button by button. So a press that the server applied to another button is one
 * lost and one doubled, never nothing. `null` until then, while the newest echo may still be on its way.
 */
export function lostAndDoubled(
  made: ArrayLike<number>,
  echoed: ArrayLike<number>,
  changedAtMs: number,
  nowMs: number,
): { lost: number; doubled: number } | null {
  if (nowMs - changedAtMs < SETTLE_MS) return null;
  let lost = 0;
  let doubled = 0;
  for (let button = 0; button < 16; button += 1) {
    const missing = (made[button] ?? 0) - (echoed[button] ?? 0);
    if (missing > 0) lost += missing;
    else doubled -= missing;
  }
  return { lost, doubled };
}
