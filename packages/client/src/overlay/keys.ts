/**
 * The key diagram: one small box per key, lit from the server's echo, never from the key event itself. So a lit box
 * says the press went to the local server through the real messages and came back. Development builds only, like
 * the readout beside it; it is plain HTML and no part of the picture.
 */

/** How long a box stays lit after an echoed press, so that a tap shorter than a frame still shows. */
export const FLASH_MS = 150;

/**
 * Which boxes are lit: the buttons the echo holds as held, and those pressed in an echo within the last `FLASH_MS`.
 * Times are kept as whole milliseconds: read once per frame, a whole number costs no allocation where a fraction
 * would, before the browser has optimised the code.
 */
export class KeyLights {
  readonly #until = new Int32Array(16);

  /** Buttons that an echo showed as pressed, one bit each, light up from `nowMs`. */
  pressed(buttons: number, nowMs: number): void {
    if (buttons === 0) return;
    const until = Math.trunc(nowMs) + FLASH_MS;
    for (let button = 0; button < 16; button += 1) {
      if ((buttons & (1 << button)) !== 0) this.#until[button] = until;
    }
  }

  /** The buttons lit at `nowMs`, one bit each, given the buttons the newest echo holds as held. */
  lit(held: number, nowMs: number): number {
    const now = Math.trunc(nowMs);
    let lit = held;
    for (let button = 0; button < 16; button += 1) {
      if ((this.#until[button] ?? 0) > now) lit |= 1 << button;
    }
    return lit;
  }
}

export class KeyDiagram {
  readonly #boxes: HTMLSpanElement[];
  readonly #lights: KeyLights;
  #lit = 0;

  /** Makes the boxes once, one per name, in the buttons' order. */
  constructor(parent: HTMLElement, names: readonly string[], lights: KeyLights) {
    this.#lights = lights;
    const document = parent.ownerDocument;
    const row = document.createElement("div");
    row.id = "key-diagram";
    this.#boxes = names.map((name) => {
      const box = document.createElement("span");
      box.textContent = name;
      return box;
    });
    row.append(...this.#boxes);
    parent.append(row);
  }

  /**
   * Once per drawn frame: `pressed` are the buttons the echoes since the last frame showed as pressed, `held` the
   * buttons the newest echo holds. Only a box whose light changes is touched, and nothing is allocated.
   */
  follow(pressed: number, held: number, nowMs: number): void {
    this.#lights.pressed(pressed, nowMs);
    const lit = this.#lights.lit(held, nowMs);
    const changed = lit ^ this.#lit;
    if (changed === 0) return;
    this.#lit = lit;
    for (let button = 0; button < this.#boxes.length; button += 1) {
      if ((changed & (1 << button)) !== 0) this.#boxes[button]?.classList.toggle("lit", (lit & (1 << button)) !== 0);
    }
  }
}
