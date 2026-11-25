import { isWorkerToPage, Ring, type PageToWorker, type WorkerStats } from "@shamble/sim-worker";
import { BLOCK_LAYOUT, checkLayoutHash } from "@shamble/types";
import { EchoTally } from "./press-tally.ts";

// Where the page reads in a block, from the generated layout: a section's start plus the field's place in its row.
const { header: HEADER, echo: ECHO, link: LINK } = BLOCK_LAYOUT;
const TICK = HEADER.offset + HEADER.fields.tick.offset;
const HELD = ECHO.offset + ECHO.fields.held.offset;
const PRESSES_0 = ECHO.offset + ECHO.fields.presses_0_3.offset;
const PRESSES_1 = ECHO.offset + ECHO.fields.presses_4_7.offset;
const PRESSES_2 = ECHO.offset + ECHO.fields.presses_8_11.offset;
const PRESSES_3 = ECHO.offset + ECHO.fields.presses_12_15.offset;
const SENT = LINK.offset + LINK.fields.sent.offset;
const RECEIVED = LINK.offset + LINK.fields.received.offset;
const REFUSED = LINK.offset + LINK.fields.refused.offset;
const POST_USED = LINK.offset + LINK.fields.post_used.offset;
/** The length of the block's sections together: a block is never shorter. */
const BLOCK_END = LINK.offset + LINK.stride;

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

/** What the link posts of the keys each frame: `KeyboardInput`, or a stand-in in tests. */
export interface KeySource {
  readonly held: number;
  word(index: 0 | 1 | 2 | 3): number;
  /** When a key last went down or up, on the clock both sides share. */
  readonly changedAt: number;
}

const DELIVERY_SAMPLES = 600;

/**
 * The page's side of the hand-off (ADR 0002 part 5). It keeps the two newest blocks, sends the older ones back to
 * the Worker's pool with the next frame's input (one message per frame), and checks the layout hash of the first
 * block of every Worker start before any offset of a block is trusted. From every block it reads the tick, the
 * server's echo of the player's input and the link's counts. The page never touches the Wasm.
 *
 * It is also the one owner of "the match is over" (`end`): whatever ends the match, a message that makes no sense,
 * an error the Worker reports or an error event of the Worker itself, ends it here, once.
 */
export class WorkerLink {
  readonly #port: WorkerPort;
  readonly #now: () => number;
  readonly #delivery = new Ring(DELIVERY_SAMPLES);
  readonly #keyToTick = new Ring(DELIVERY_SAMPLES);
  // The one key change being timed: the post that first carried it and when the key changed (NaN: none), and the
  // newest change that has been timed or is being timed.
  #keyPost = 0;
  #keyAt = Number.NaN;
  #keyChange = 0;
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
  readonly #echo = new EchoTally();
  #echoHeld = 0;
  #sent = 0;
  #received = 0;
  #refused = 0;
  #postUsed = 0;
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

  /**
   * From a key going down or up to the start of the first tick whose command was built from a post that held the
   * change, in milliseconds: one key change is timed at a time. A press that waits its turn behind an earlier
   * press of the same button is applied a tick later than this says.
   */
  get keyToTick(): Ring {
    return this.#keyToTick;
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

  /** What the server echoed so far, counted from every block: presses and releases. */
  get echo(): EchoTally {
    return this.#echo;
  }

  /** The buttons the newest block's echo holds as held, one bit each. */
  get echoHeld(): number {
    return this.#echoHeld;
  }

  /** The newest block's count of messages the endpoint sent and received, and of messages either side refused. */
  get sent(): number {
    return this.#sent;
  }

  get received(): number {
    return this.#received;
  }

  get refused(): number {
    return this.#refused;
  }

  /** The number of the post from which the last command the server ran was built. */
  get postUsed(): number {
    return this.#postUsed;
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
        return this.#takeBlock(data.buffer, data.startAt, data.doneAt);
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

  #takeBlock(buffer: ArrayBuffer, startAt: number, doneAt: number): string {
    if (!this.#layoutChecked) {
      const check = checkLayoutHash(buffer);
      if (!check.ok) return this.#fail(`the block's layout is not the page's (${check.reason}): the page and the Wasm are from different builds`);
      this.#layoutChecked = true;
    }
    if (buffer.byteLength !== this.#blockBytes) return this.#fail(`a block of ${buffer.byteLength} bytes, not ${this.#blockBytes}`);
    // The hash says the layouts agree; the length is still checked before any place past the header is read.
    if (buffer.byteLength < BLOCK_END) return this.#fail(`a block of ${buffer.byteLength} bytes is shorter than its ${BLOCK_END} bytes of sections`);
    this.#delivery.push(this.#now() - doneAt);
    if (this.#previous !== null) this.#returned.push(this.#previous);
    this.#previous = this.#newest;
    this.#newest = buffer;
    this.#blocks += 1;
    const view = new DataView(buffer);
    this.#tick = view.getUint32(TICK, true);
    this.#echoHeld = view.getUint32(HELD, true);
    this.#echo.take(
      this.#echoHeld,
      view.getUint32(PRESSES_0, true),
      view.getUint32(PRESSES_1, true),
      view.getUint32(PRESSES_2, true),
      view.getUint32(PRESSES_3, true),
    );
    this.#sent = view.getUint32(SENT, true);
    this.#received = view.getUint32(RECEIVED, true);
    this.#refused = view.getUint32(REFUSED, true);
    this.#postUsed = view.getUint32(POST_USED, true);
    // The post this tick used is the timed one or a later one, around the 32-bit circle: the tick took the change up.
    if (!Number.isNaN(this.#keyAt) && (this.#postUsed - this.#keyPost) >>> 0 < 0x8000_0000) {
      this.#keyToTick.push(startAt - this.#keyAt);
      this.#keyAt = Number.NaN;
    }
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
  frame(keys: KeySource): void {
    if (!this.#ready || this.#failure !== "") return;
    const returned = this.#returned;
    this.#returned = [];
    this.#posts += 1;
    const post = this.#posts >>> 0;
    if (keys.changedAt !== this.#keyChange && Number.isNaN(this.#keyAt)) {
      this.#keyChange = keys.changedAt;
      this.#keyAt = keys.changedAt;
      this.#keyPost = post;
    }
    const message = { type: "input", post, held: keys.held, p0: keys.word(0), p1: keys.word(1), p2: keys.word(2), p3: keys.word(3), returned } as const;
    this.#port.postMessage(message, returned);
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
