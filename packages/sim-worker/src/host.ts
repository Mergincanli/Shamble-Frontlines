import { BlockPool } from "./block-pool.ts";
import { isPageToWorker, type WorkerStats, type WorkerToPage } from "./messages.ts";
import { Pacer } from "./pacing.ts";
import { trapMessage } from "./panic-message.ts";
import { copyBlock, type LinearMemory } from "./read-block.ts";
import { Ring } from "./stats.ts";

/** The control calls the host makes on the simulation: the Wasm's `Sim`, or a stand-in in tests. */
export interface HostSim {
  input(sequence: number): void;
  step(): number;
  rebase(nowMs: number): number;
  block_ptr(): number;
  block_len(): number;
}

/** Everything the host needs from the Worker around it, passed in so that a test can stand in for each part. */
export interface HostEnv {
  readonly memory: LinearMemory;
  /** Where the Wasm's panic hook keeps a trap's message (`panic_buffer()`, asked once at the start). */
  readonly panicBuffer: number;
  /** The Worker's clock, `performance.now()`. */
  now(): number;
  /** `performance.timeOrigin`: with `now()` it gives the clock the page shares. */
  readonly timeOrigin: number;
  post(message: WorkerToPage, transfer: ArrayBuffer[]): void;
  setTimer(run: () => void, delayMs: number): number;
  clearTimer(id: number): void;
  readonly hardwareConcurrency: number;
  readonly crossOriginIsolated: boolean;
}

/**
 * Buffers in the pool: the page keeps the two newest blocks, one is on its way and one is being filled. Enough while
 * frames come at least as often as ticks; a slower page or a catch-up wake takes more, and each miss is counted.
 */
const POOL_SIZE = 4;
const COPY_SAMPLES = 600;

/**
 * The Worker's host (ADR 0002 part 5): it paces the tick, makes the two calls per tick (`input`, then `step`),
 * copies the block once into a pooled buffer and transfers it to the page.
 */
export class SimHost {
  readonly #sim: HostSim;
  readonly #env: HostEnv;
  readonly #pacer: Pacer;
  readonly #pool: BlockPool;
  readonly #copyMs = new Ring(COPY_SAMPLES);
  readonly #blockPtr: number;
  readonly #blockBytes: number;
  readonly #tickHz: number;
  #newestSequence = 0;
  #timer: number | undefined;
  #undetached = 0;
  #failed = false;

  constructor(sim: HostSim, env: HostEnv, tickHz: number) {
    this.#sim = sim;
    this.#env = env;
    this.#tickHz = tickHz;
    // Read once: the block's buffer never moves and never changes its length.
    this.#blockPtr = sim.block_ptr();
    this.#blockBytes = sim.block_len();
    this.#pool = new BlockPool(this.#blockBytes, POOL_SIZE);
    this.#pacer = new Pacer(
      {
        step: () => {
          sim.input(this.#newestSequence);
          return sim.step();
        },
        rebase: (nowMs) => sim.rebase(nowMs),
      },
      () => env.now(),
    );
  }

  /** Tells the page the match runs and sets the first timer. */
  start(): void {
    this.#env.post({ type: "ready", blockBytes: this.#blockBytes, tickHz: this.#tickHz }, []);
    this.#wake();
  }

  /** A message from the page; one that makes no sense is answered with an error and changes nothing. */
  onMessage(data: unknown): void {
    if (!isPageToWorker(data) || data.type === "start") {
      this.#env.post({ type: "error", message: "the Worker got a message it does not know" }, []);
      return;
    }
    switch (data.type) {
      case "input":
        this.#newestSequence = data.sequence;
        for (const buffer of data.returned) this.#pool.give(buffer);
        return;
      case "visibility":
        if (data.hidden) {
          this.#pacer.pause();
          this.#clearTimer();
        } else if (this.#pacer.paused && this.#call(this.#resume) !== undefined) {
          this.#wake();
        }
        return;
      case "report-request":
        this.#env.post({ type: "report", stats: this.stats() }, []);
        return;
    }
  }

  stats(): WorkerStats {
    const lateness = this.#pacer.lateness;
    return {
      ...this.#pacer.counters,
      latenessP50: lateness.percentile(50),
      latenessP99: lateness.percentile(99),
      latenessMax: lateness.max(),
      copyP50: this.#copyMs.percentile(50),
      copyP99: this.#copyMs.percentile(99),
      blockBytes: this.#blockBytes,
      poolMisses: this.#pool.misses,
      poolRefused: this.#pool.refused,
      undetached: this.#undetached,
      hardwareConcurrency: this.#env.hardwareConcurrency,
      crossOriginIsolated: this.#env.crossOriginIsolated,
    };
  }

  /**
   * The one way into the Wasm after the start: every call the host makes goes through here. A throw is a trap in
   * the Wasm, or a block that does not fit: the match cannot go on. The page is told why, once, with the panic's
   * own text when the hook kept one; the timer is cleared, and no call into the Wasm is made again (a trapped
   * instance may hold half-written state). Returns what `run` returned, or `undefined` when the match is over.
   */
  #call<T>(run: () => T): T | undefined {
    if (this.#failed) return undefined;
    try {
      return run();
    } catch (error) {
      this.#failed = true;
      this.#clearTimer();
      this.#env.post({ type: "error", message: trapMessage(this.#env.memory, this.#env.panicBuffer, error) }, []);
      return undefined;
    }
  }

  // The two calls that reach the Wasm, made once: a wake allocates no function of its own.
  readonly #runTicks = (): number | null => this.#pacer.wake(this.#afterTick);
  readonly #resume = (): number => this.#pacer.resume();

  readonly #wake = (): void => {
    this.#timer = undefined;
    const sleep = this.#call(this.#runTicks);
    // A whole number of milliseconds, rounded up: a timer cuts a fraction off, fires before the tick is due, and
    // the short second sleep that follows is stretched to about 4 ms by the browser, so the tick starts late.
    if (sleep !== undefined && sleep !== null) this.#timer = this.#env.setTimer(this.#wake, Math.ceil(sleep));
  };

  readonly #afterTick = (): void => {
    const buffer = this.#pool.take();
    const before = this.#env.now();
    copyBlock(this.#env.memory, this.#blockPtr, this.#blockBytes, buffer);
    this.#copyMs.push(this.#env.now() - before);
    // The hand-off's clock starts when the step returned, before the copy.
    this.#env.post({ type: "block", buffer, doneAt: this.#env.timeOrigin + before }, [buffer]);
    // A transferred buffer is detached, so its length is 0 here; anything else means it was copied (PM-0002-6).
    if (buffer.byteLength !== 0) this.#undetached += 1;
  };

  #clearTimer(): void {
    if (this.#timer !== undefined) {
      this.#env.clearTimer(this.#timer);
      this.#timer = undefined;
    }
  }
}
