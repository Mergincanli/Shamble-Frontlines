import { Ring } from "./stats.ts";

/**
 * The most ticks one wake of the Worker may run: 0.25 s of game time, which at 30 Hz is 7 ticks (ADR 0004 part 5,
 * S18). A longer gap is dropped, never caught up.
 */
export const MAX_TICKS_PER_WAKE = 7;

/** How many tick starts the lateness reading keeps: 20 seconds at 30 Hz. */
const LATENESS_SAMPLES = 600;

/** What the pacer drives: the simulation's step and its schedule (the Wasm's control calls, or a stand-in in tests). */
export interface PacedSim {
  /** Runs one tick and returns the time, in the host's milliseconds, at which the next one is due. */
  step(): number;
  /** Drops the time up to `nowMs` and returns when the next tick is due, one tick period later. */
  rebase(nowMs: number): number;
}

/** The pacer's counts since it was made. */
export interface PacerCounters {
  /** Ticks run. */
  readonly ticks: number;
  /** Wakes, whether or not a tick was due. */
  readonly wakes: number;
  /** The most ticks any one wake ran; never above `MAX_TICKS_PER_WAKE`. */
  readonly maxTicksInOneWake: number;
  /** How often a gap was dropped because one wake could not catch it up. */
  readonly drops: number;
  /** The time dropped that way, in milliseconds. */
  readonly droppedMs: number;
  /** How often the match was paused and resumed. */
  readonly pauses: number;
}

/**
 * Paces the tick in the Worker (ADR 0004 part 5): it runs the ticks that are due, sleeps until the time the step
 * gave back, and never sets a timer from a count of its own. It owns no timer and reads no clock but the one it is
 * given, so a test drives it with a made-up clock. After a pause, or a stall longer than one wake may catch up, the
 * gap is dropped and counted: the match pauses, it does not fast-forward.
 */
export class Pacer {
  readonly #sim: PacedSim;
  readonly #now: () => number;
  readonly #lateness = new Ring(LATENESS_SAMPLES);
  #nextDueMs: number;
  #paused = false;
  #ticks = 0;
  #wakes = 0;
  #maxTicksInOneWake = 0;
  #drops = 0;
  #droppedMs = 0;
  #pauses = 0;

  /** Starts the schedule at the clock's time now: the first tick is due one tick period later. */
  constructor(sim: PacedSim, now: () => number) {
    this.#sim = sim;
    this.#now = now;
    this.#nextDueMs = sim.rebase(now());
  }

  get counters(): PacerCounters {
    return {
      ticks: this.#ticks,
      wakes: this.#wakes,
      maxTicksInOneWake: this.#maxTicksInOneWake,
      drops: this.#drops,
      droppedMs: this.#droppedMs,
      pauses: this.#pauses,
    };
  }

  /** How late each of the newest tick starts was against its due time, in milliseconds. */
  get lateness(): Ring {
    return this.#lateness;
  }

  get paused(): boolean {
    return this.#paused;
  }

  /**
   * One wake: runs every tick that is due, calling `afterTick` after each, up to `MAX_TICKS_PER_WAKE`; if ticks are
   * still due then, drops the rest of the gap. Returns how long to sleep before the next wake, in milliseconds, or
   * `null` while paused (the caller sets no timer).
   */
  wake(afterTick: () => void): number | null {
    if (this.#paused) return null;
    this.#wakes += 1;
    let ran = 0;
    let now = this.#now();
    while (ran < MAX_TICKS_PER_WAKE && this.#nextDueMs <= now) {
      this.#lateness.push(now - this.#nextDueMs);
      this.#nextDueMs = this.#sim.step();
      afterTick();
      ran += 1;
      now = this.#now();
    }
    this.#ticks += ran;
    if (ran > this.#maxTicksInOneWake) this.#maxTicksInOneWake = ran;
    if (this.#nextDueMs <= now) {
      this.#drops += 1;
      this.#droppedMs += now - this.#nextDueMs;
      this.#nextDueMs = this.#sim.rebase(now);
    }
    return Math.max(0, this.#nextDueMs - now);
  }

  /** Pauses the match (the tab is hidden): no tick runs until `resume`. */
  pause(): void {
    this.#paused = true;
  }

  /**
   * Resumes after a pause: the time spent paused is dropped, and the next tick is due one tick period from now.
   * Returns how long to sleep before the next wake. Without a pause before it, it changes nothing.
   */
  resume(): number {
    const now = this.#now();
    if (this.#paused) {
      this.#paused = false;
      this.#pauses += 1;
      this.#nextDueMs = this.#sim.rebase(now);
    }
    return Math.max(0, this.#nextDueMs - now);
  }
}
