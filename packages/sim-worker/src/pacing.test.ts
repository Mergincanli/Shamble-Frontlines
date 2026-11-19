import { expect, test } from "vitest";
import { MAX_TICKS_PER_WAKE, Pacer, type PacedSim } from "./pacing.ts";

const PERIOD = 1000 / 30;

/** A stand-in for the Wasm's schedule: an anchor and a count, as `LocalSchedule` keeps them in Rust. */
class FakeSim implements PacedSim {
  ticks = 0;
  rebases = 0;
  #anchor = 0;
  #sinceAnchor = 0;
  /** What one step costs on the clock, for the tests that need slow steps. */
  stepCostMs = 0;
  readonly #clock: { now: number };

  constructor(clock: { now: number }) {
    this.#clock = clock;
  }

  step(): number {
    this.ticks += 1;
    this.#sinceAnchor += 1;
    this.#clock.now += this.stepCostMs;
    return this.#anchor + (this.#sinceAnchor + 1) * PERIOD;
  }

  rebase(nowMs: number): number {
    this.rebases += 1;
    this.#anchor = nowMs;
    this.#sinceAnchor = 0;
    return nowMs + PERIOD;
  }
}

function setUp(startMs = 1000): { clock: { now: number }; sim: FakeSim; pacer: Pacer; blocks: number[] } {
  const clock = { now: startMs };
  const sim = new FakeSim(clock);
  const pacer = new Pacer(sim, () => clock.now);
  return { clock, sim, pacer, blocks: [] };
}

test("the cap is 7 ticks: a quarter of a second at 30 Hz", () => {
  expect(MAX_TICKS_PER_WAKE).toBe(7);
  expect(Math.floor(0.25 * 30)).toBe(MAX_TICKS_PER_WAKE);
});

test("the first tick is due one period after the start, and an early wake runs nothing", () => {
  const { clock, sim, pacer } = setUp();
  expect(pacer.wake(() => {})).toBeCloseTo(PERIOD, 9);
  expect(sim.ticks).toBe(0);
  clock.now += 10;
  expect(pacer.wake(() => {})).toBeCloseTo(PERIOD - 10, 9);
  expect(sim.ticks).toBe(0);
  expect(pacer.counters.wakes).toBe(2);
});

test("a wake on time runs one tick, tells the caller, and sleeps until the time the step gave back", () => {
  const { clock, sim, pacer } = setUp();
  let told = 0;
  for (let tick = 1; tick <= 90; tick += 1) {
    clock.now = 1000 + tick * PERIOD;
    const sleep = pacer.wake(() => {
      told += 1;
    });
    expect(sim.ticks).toBe(tick);
    expect(sleep).toBeCloseTo(PERIOD, 6);
  }
  expect(told).toBe(90);
  expect(pacer.counters).toMatchObject({ ticks: 90, wakes: 90, maxTicksInOneWake: 1, drops: 0, droppedMs: 0 });
  expect(pacer.lateness.count).toBe(90);
  expect(pacer.lateness.max()).toBeLessThan(1e-6);
  // Only the first rebase, at the start.
  expect(sim.rebases).toBe(1);
});

test("a late wake catches up within the cap and keeps the schedule", () => {
  const { clock, sim, pacer } = setUp();
  // 3.5 periods late: ticks 1 to 4 are due.
  clock.now = 1000 + 4.5 * PERIOD;
  const sleep = pacer.wake(() => {});
  expect(sim.ticks).toBe(4);
  expect(sleep).toBeCloseTo(0.5 * PERIOD, 6);
  expect(pacer.counters).toMatchObject({ ticks: 4, maxTicksInOneWake: 4, drops: 0 });
  expect(sim.rebases).toBe(1);
  // The first of them started 3.5 periods late, the last half a period late.
  expect(pacer.lateness.max()).toBeCloseTo(3.5 * PERIOD, 6);
});

test("a stall longer than the cap runs 7 ticks and drops the rest", () => {
  const { clock, sim, pacer } = setUp();
  // One second late: 30 ticks would be due.
  clock.now = 1000 + PERIOD + 1000;
  const sleep = pacer.wake(() => {});
  expect(sim.ticks).toBe(MAX_TICKS_PER_WAKE);
  expect(pacer.counters).toMatchObject({ ticks: 7, maxTicksInOneWake: 7, drops: 1 });
  // 31 periods had passed; 7 ticks ran and tick 8 was due 23 periods ago.
  expect(pacer.counters.droppedMs).toBeCloseTo(23 * PERIOD, 6);
  // After the drop the next tick is one period away.
  expect(sleep).toBeCloseTo(PERIOD, 6);
  expect(sim.rebases).toBe(2);
});

test("steps that take long are caught up in the same wake, never past the cap", () => {
  const { clock, sim, pacer } = setUp();
  sim.stepCostMs = 40; // each step takes longer than a period
  clock.now = 1000 + PERIOD;
  pacer.wake(() => {});
  expect(sim.ticks).toBe(MAX_TICKS_PER_WAKE);
  expect(pacer.counters.maxTicksInOneWake).toBe(MAX_TICKS_PER_WAKE);
  expect(pacer.counters.drops).toBe(1);
});

test("a pause runs no tick, and the match resumes where it stopped", () => {
  const { clock, sim, pacer } = setUp();
  for (let tick = 1; tick <= 30; tick += 1) {
    clock.now = 1000 + tick * PERIOD;
    pacer.wake(() => {});
  }
  expect(sim.ticks).toBe(30);
  pacer.pause();
  expect(pacer.paused).toBe(true);
  // The tab stays hidden for six minutes; a stray wake does nothing and asks for no timer.
  clock.now += 360_000;
  expect(pacer.wake(() => {})).toBeNull();
  expect(sim.ticks).toBe(30);
  const wakesBefore = pacer.counters.wakes;
  const sleep = pacer.resume();
  expect(pacer.paused).toBe(false);
  expect(sleep).toBeCloseTo(PERIOD, 6);
  expect(sim.ticks).toBe(30);
  // The hidden time is a pause, not a drop: nothing was owed.
  expect(pacer.counters).toMatchObject({ pauses: 1, drops: 0, droppedMs: 0, wakes: wakesBefore });
  clock.now += PERIOD;
  pacer.wake(() => {});
  expect(sim.ticks).toBe(31);
  expect(pacer.counters.maxTicksInOneWake).toBe(1);
});

test("resume without a pause changes nothing", () => {
  const { clock, sim, pacer } = setUp();
  clock.now += 10;
  expect(pacer.resume()).toBeCloseTo(PERIOD - 10, 9);
  expect(pacer.counters.pauses).toBe(0);
  expect(sim.rebases).toBe(1);
});
