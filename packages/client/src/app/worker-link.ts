import { isWorkerToPage, Ring, type PageToWorker, type WorkerStats } from "@shamble/sim-worker";
import { BLOCK_LAYOUT, checkLayoutHash } from "@shamble/types";

/** How the link reaches the Worker and stops it: the Worker itself, or a stand-in in tests. */
export interface WorkerPort {
  postMessage(message: PageToWorker, transfer: ArrayBuffer[]): void;
  terminate(): void;
}

/** One pause of the match while the tab was hidden. */
export interface PauseRecord {
  readonly hiddenMs: number;
  /** Ticks the page saw arrive between hiding and showing: 0, or the few that were already on their way. */
  readonly ticksWhileHidden: number;
}

const DELIVERY_SAMPLES = 600;

/**
 * The page's side of the hand-off (ADR 0002 part 5). It keeps the two newest blocks, sends the older ones back to
 * the Worker's pool with the next frame's input (one message per frame), and checks the layout hash of the first
 * block of every Worker start before any offset of a block is trusted. The page never touches the Wasm.
 *
 * It is also the one owner of "the match is over" (`end`): whatever ends the match, a message that makes no sense,
 * an error the Worker reports or an error event of the Worker itself, ends it here, once.
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
  #ready = false;
  #blockBytes = 0;
  #tickHz = 0;
  #blocks = 0;
  #tick = 0;
  #posts = 0;
  #failure = "";
  #stats: WorkerStats | null = null;
  #lastPause: PauseRecord | null = null;
  #pauses = 0;

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

  /** The Worker has loaded the Wasm and the match runs. */
  get ready(): boolean {
    return this.#ready;
  }

  get blockBytes(): number {
    return this.#blockBytes;
  }

  get tickHz(): number {
    return this.#tickHz;
  }

  /** Blocks received so far. */
  get blocks(): number {
    return this.#blocks;
  }

  /** The tick of the newest block. */
  get tick(): number {
    return this.#tick;
  }

  /** Input messages posted so far: one per drawn frame. */
  get posts(): number {
    return this.#posts;
  }

  /** Why the match stopped, in plain words; empty while it runs. */
  get failure(): string {
    return this.#failure;
  }

  /** The Worker's newest report, once one has arrived. */
  get stats(): WorkerStats | null {
    return this.#stats;
  }

  get lastPause(): PauseRecord | null {
    return this.#lastPause;
  }

  get pauses(): number {
    return this.#pauses;
  }

  start(padBytes: number): void {
    this.#port.postMessage({ type: "start", padBytes }, []);
  }

  /**
   * Ends the match. The first call keeps the reason, stops the Worker (it would run on and fill memory with blocks
   * nobody takes) and returns true; every later call changes nothing and returns false, so one cause is one error.
   */
  end(reason: string): boolean {
    if (this.#failure !== "") return false;
    this.#failure = reason === "" ? "no reason was given" : reason;
    this.#port.terminate();
    return true;
  }

  /** The Worker's message event. Unpacking it is part of the hand-off's cost, so it happens in here, by name. */
  receive(event: { readonly data: unknown }): string {
    return this.onMessage(event.data);
  }

  /**
   * A message from the Worker. Returns the reason when this message ended the match (the page counts and shows it),
   * or an empty string. After the end whatever still arrives is ignored.
   */
  onMessage(data: unknown): string {
    if (this.#failure !== "") return "";
    if (!isWorkerToPage(data)) return this.#fail("the Worker sent a message the page does not know");
    switch (data.type) {
      case "ready":
        this.#ready = true;
        this.#blockBytes = data.blockBytes;
        this.#tickHz = data.tickHz;
        this.#layoutChecked = false;
        return "";
      case "block":
        return this.#takeBlock(data.buffer, data.doneAt);
      case "report":
        this.#stats = data.stats;
        return "";
      case "error":
        return this.#fail(data.message);
      default: {
        // A kind added to `WorkerToPage` does not compile until it has its case above.
        const unhandled: never = data;
        return unhandled;
      }
    }
  }

  #takeBlock(buffer: ArrayBuffer, doneAt: number): string {
    if (!this.#layoutChecked) {
      const check = checkLayoutHash(buffer);
      if (!check.ok) return this.#fail(`the block's layout is not the page's (${check.reason}): the page and the Wasm are from different builds`);
      this.#layoutChecked = true;
    }
    if (buffer.byteLength !== this.#blockBytes) return this.#fail(`a block of ${buffer.byteLength} bytes, not ${this.#blockBytes}`);
    this.#delivery.push(this.#now() - doneAt);
    if (this.#previous !== null) this.#returned.push(this.#previous);
    this.#previous = this.#newest;
    this.#newest = buffer;
    this.#blocks += 1;
    this.#tick = new DataView(buffer).getUint32(BLOCK_LAYOUT.header.fields.tick.offset, true);
    return "";
  }

  #fail(reason: string): string {
    this.end(reason);
    return this.#failure;
  }

  /**
   * Once per drawn frame: the newest input (the buttons held, and the running press counts four to a word), with
   * the buffers the page is done with riding back to the pool. The post's number is the count of posts so far,
   * around the 32-bit circle.
   */
  frame(held: number, p0: number, p1: number, p2: number, p3: number): void {
    if (!this.#ready || this.#failure !== "") return;
    const returned = this.#returned;
    this.#returned = [];
    this.#posts += 1;
    this.#port.postMessage({ type: "input", post: this.#posts >>> 0, held, p0, p1, p2, p3, returned }, returned);
  }

  /** The tab was hidden or shown: the Worker pauses the match, and the page records what the pause was. */
  visibility(hidden: boolean): void {
    if (this.#failure !== "") return;
    this.#port.postMessage({ type: "visibility", hidden }, []);
    if (hidden) {
      this.#hiddenAt = { timeMs: this.#now(), tick: this.#tick };
    } else if (this.#hiddenAt !== null) {
      this.#pauses += 1;
      this.#lastPause = { hiddenMs: this.#now() - this.#hiddenAt.timeMs, ticksWhileHidden: this.#tick - this.#hiddenAt.tick };
      this.#hiddenAt = null;
    }
  }

  requestReport(): void {
    if (this.#ready && this.#failure === "") this.#port.postMessage({ type: "report-request" }, []);
  }
}
