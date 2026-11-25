/**
 * The keyboard, by physical key (`KeyboardEvent.code`): the key's place on the board, whatever the layout prints on
 * it. It keeps what the page posts once per drawn frame: one bit per button held, and a running count of presses per
 * button that wraps at 256. A count never goes back, so a tap shorter than a frame is still in the next post, and a
 * post that is skipped loses nothing. It listens to nothing itself: the app hands it the events.
 */

/** The keys so far, and the button each one is. Either Shift is the one button 5. */
const KEYS: readonly (readonly [string, number])[] = [
  ["KeyW", 0],
  ["KeyA", 1],
  ["KeyS", 2],
  ["KeyD", 3],
  ["Space", 4],
  ["ShiftLeft", 5],
  ["ShiftRight", 5],
];

/** The buttons' names for the readout and the key diagram, in the buttons' order. */
export const BUTTON_NAMES = ["W", "A", "S", "D", "Space", "Shift"] as const;

export class KeyboardInput {
  /** Which of `KEYS` are down, one bit per key. */
  #down = 0;
  #held = 0;
  readonly #counts = new Uint8Array(16);
  readonly #words = new DataView(this.#counts.buffer);
  readonly #made = new Int32Array(16);
  #presses = 0;
  #releases = 0;
  #changedAt = 0;

  /** One bit per button held. */
  get held(): number {
    return this.#held;
  }

  /** The running press counts of buttons `4 * index` to `4 * index + 3`, one per byte, the first in the lowest. */
  word(index: 0 | 1 | 2 | 3): number {
    return this.#words.getUint32(4 * index, true);
  }

  /** Presses and releases so far, over all buttons, for the readout's tally; and when the last of them was. */
  get presses(): number {
    return this.#presses;
  }

  get releases(): number {
    return this.#releases;
  }

  /** The presses of each button so far, not wrapped: the tally compares them with the echo's, button by button. */
  get pressesByButton(): Int32Array {
    return this.#made;
  }

  get changedAt(): number {
    return this.#changedAt;
  }

  /**
   * A key went down or up at `atMs`. Returns whether the game took it, so the caller can keep the browser from also
   * scrolling on Space. A key held down repeats its key-down: a repeat counts as nothing.
   *
   * `typing` says the focus is in a text box. A key that goes down there is the text box's and counts as nothing.
   * A key that comes up counts wherever it lands: a key held when the focus moved into the text box is let go
   * there, and would stay held for good otherwise. The text box keeps the event either way.
   */
  key(code: string, down: boolean, repeat: boolean, atMs: number, typing = false): boolean {
    const key = KEYS.findIndex((entry) => entry[0] === code);
    if (key < 0 || (typing && down)) return false;
    if (!repeat) this.#settle(down ? this.#down | (1 << key) : this.#down & ~(1 << key), atMs);
    return !typing;
  }

  /** No key is held any more: the window lost the focus or the tab was hidden, and no key-up will come. */
  clear(atMs: number): void {
    this.#settle(0, atMs);
  }

  #settle(down: number, atMs: number): void {
    this.#down = down;
    let held = 0;
    for (let key = 0; key < KEYS.length; key += 1) {
      if ((down & (1 << key)) !== 0) held |= 1 << (KEYS[key]?.[1] ?? 0);
    }
    const changed = held ^ this.#held;
    if (changed === 0) return;
    for (let button = 0; button < 16; button += 1) {
      if ((changed & (1 << button)) === 0) continue;
      if ((held & (1 << button)) !== 0) {
        // A typed array's element wraps at 256 by itself.
        this.#counts[button] = (this.#counts[button] ?? 0) + 1;
        this.#made[button] = (this.#made[button] ?? 0) + 1;
        this.#presses += 1;
      } else {
        this.#releases += 1;
      }
    }
    this.#held = held;
    this.#changedAt = atMs;
  }
}
