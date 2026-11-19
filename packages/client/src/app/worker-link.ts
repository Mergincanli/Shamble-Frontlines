import { Ring, type PageToWorker, type WorkerStats } from "@shamble/sim-worker";
import { checkLayoutHash } from "@shamble/types";

/** How the link reaches the Worker: the Worker's own `postMessage`, or a stand-in in tests. */
export interface WorkerPort {
  postMessage(message: PageToWorker, transfer: ArrayBuffer[]): void;
}

/** One pause of the match while the tab was hidden. */
export interface PauseRecord {
  readonly hiddenMs: number;
  /** Ticks that ran between hiding and showing; 0 when the match paused as it should. */
  readonly ticksWhileHidden: number;
}

const DELIVERY_SAMPLES = 600;

/**
 * The page's side of the hand-off (ADR 0002 part 5). It keeps the two newest blocks, sends the older ones back to
 * the Worker's pool with the next frame's input (one message per frame), and checks the layout hash of the first
 * block of every Worker start before any offset of a block is trusted. The page never touches the Wasm.
 */
export class WorkerLink {
  readonly #port: WorkerPort;
  readonly #now: () => number;
  readonly #delivery = new Ring(DELIVERY_SAMPLES);
  #newest: ArrayBuffer | null = null;
  #previous: ArrayBuffer | null = null;
  #returned: ArrayBuffer[] = [];
  #layoutChecked = false;
  #hiddenAt: { timeMs: number; tick: number } | null = null;
  ready = false;
  blockBytes = 0;
  tickHz = 0;
  blocks = 0;
  tick = 0;
  posts = 0;
  /** Why the match stopped, in plain words; empty while it runs. */
  failure = "";
  stats: WorkerStats | null = null;
  lastPause: PauseRecord | null = null;
  pauses = 0;

  /** `now` is the clock both sides share: `performance.timeOrigin + performance.now()`. */
  constructor(port: WorkerPort, now: () => number) {
    this.#port = port;
    this.#now = now;
  }

  get newest(): ArrayBuffer | null {
    return this.#newest;
  }

  get previous(): ArrayBuffer | null {
    return this.#previous;
  }

  /** From the Worker's finished step to the page holding the block, in milliseconds. */
  get delivery(): Ring {
    return this.#delivery;
  }

  start(padBytes: number): void {
    this.#port.postMessage({ type: "start", padBytes }, []);
  }

  /** A message from the Worker. Returns the text of an error the page should count, or an empty string. */
  onMessage(data: unknown): string {
    if (typeof data !== "object" || data === null || !("type" in data)) return this.#fail("the Worker sent no message");
    const message = data as { type: unknown; [key: string]: unknown };
    if (message.type === "ready" && typeof message["blockBytes"] === "number" && typeof message["tickHz"] === "number") {
      this.ready = true;
      this.blockBytes = message["blockBytes"];
      this.tickHz = message["tickHz"];
      this.#layoutChecked = false;
      return "";
    }
    if (message.type === "block" && message["buffer"] instanceof ArrayBuffer && typeof message["doneAt"] === "number") {
      return this.#takeBlock(message["buffer"], message["doneAt"]);
    }
    if (message.type === "report" && typeof message["stats"] === "object" && message["stats"] !== null) {
      this.stats = message["stats"] as WorkerStats;
      return "";
    }
    if (message.type === "error" && typeof message["message"] === "string") return this.#fail(message["message"]);
    return this.#fail("the Worker sent a message the page does not know");
  }

  #takeBlock(buffer: ArrayBuffer, doneAt: number): string {
    if (!this.#layoutChecked) {
      const check = checkLayoutHash(buffer);
      if (!check.ok) return this.#fail(`the block's layout is not the page's (${check.reason}): the page and the Wasm are from different builds`);
      this.#layoutChecked = true;
    }
    if (buffer.byteLength !== this.blockBytes) return this.#fail(`a block of ${buffer.byteLength} bytes, not ${this.blockBytes}`);
    this.#delivery.push(this.#now() - doneAt);
    if (this.#previous !== null) this.#returned.push(this.#previous);
    this.#previous = this.#newest;
    this.#newest = buffer;
    this.blocks += 1;
    this.tick = new DataView(buffer).getUint32(0, true);
    return "";
  }

  #fail(reason: string): string {
    this.failure = reason;
    return reason;
  }

  /** Once per drawn frame: the newest input, with the buffers the page is done with riding back to the pool. */
  frame(sequence: number, viewTime: number): void {
    if (!this.ready || this.failure !== "") return;
    const returned = this.#returned;
    this.#returned = [];
    this.#port.postMessage({ type: "input", sequence, viewTime, returned }, returned);
    this.posts += 1;
  }

  /** The tab was hidden or shown: the Worker pauses the match, and the page records what the pause was. */
  visibility(hidden: boolean): void {
    this.#port.postMessage({ type: "visibility", hidden }, []);
    if (hidden) {
      this.#hiddenAt = { timeMs: this.#now(), tick: this.tick };
    } else if (this.#hiddenAt !== null) {
      this.pauses += 1;
      this.lastPause = { hiddenMs: this.#now() - this.#hiddenAt.timeMs, ticksWhileHidden: this.tick - this.#hiddenAt.tick };
      this.#hiddenAt = null;
    }
  }

  requestReport(): void {
    if (this.ready) this.#port.postMessage({ type: "report-request" }, []);
  }
}
