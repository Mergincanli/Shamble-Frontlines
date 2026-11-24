import { expect, test } from "vitest";
import { type HostEnv, type HostSim, SimHost } from "./host.ts";
import type { WorkerToPage } from "./messages.ts";

const PERIOD = 1000 / 30;

/** A stand-in for the Wasm: a "linear memory" whose block holds the tick, and the schedule `LocalSchedule` keeps. */
function setUp(options: { transfer?: boolean; blockBytes?: number; copyMs?: number } = {}) {
  const blockBytes = options.blockBytes ?? 8;
  const bytes = new ArrayBuffer(1024);
  const clock = { now: 1000 };
  // The host reads the memory's buffer when it copies the block: a test can let that copy take time on the clock.
  const memory = {
    get buffer(): ArrayBuffer {
      clock.now += options.copyMs ?? 0;
      return bytes;
    },
  };
  const calls = { input: 0, step: 0, rebase: 0, block_ptr: 0, block_len: 0 };
  let tick = 0;
  let anchor = 0;
  let since = 0;
  let newest = -1;
  const sim: HostSim & { trapNext: boolean; newest(): number } = {
    trapNext: false,
    newest: () => newest,
    input(sequence) {
      calls.input += 1;
      newest = sequence;
    },
    step() {
      calls.step += 1;
      if (this.trapNext) throw new Error("unreachable executed");
      tick += 1;
      since += 1;
      new DataView(bytes).setUint32(64, tick, true);
      return anchor + (since + 1) * PERIOD;
    },
    rebase(nowMs) {
      calls.rebase += 1;
      anchor = nowMs;
      since = 0;
      return nowMs + PERIOD;
    },
    block_ptr: () => ((calls.block_ptr += 1), 64),
    block_len: () => ((calls.block_len += 1), blockBytes),
  };
  const posts: WorkerToPage[] = [];
  let timer: { run: () => void; at: number } | undefined;
  const delays: number[] = [];
  const env: HostEnv = {
    memory,
    now: () => clock.now,
    timeOrigin: 5_000_000,
    post(message, transfer) {
      posts.push(options.transfer === false ? message : structuredClone(message, { transfer }));
    },
    setTimer(run, delayMs) {
      delays.push(delayMs);
      timer = { run, at: clock.now + delayMs };
      return 1;
    },
    clearTimer() {
      timer = undefined;
    },
    hardwareConcurrency: 8,
    crossOriginIsolated: false,
  };
  const host = new SimHost(sim, env, 30);
  /** Lets the clock run to the pending timer and fires it; false when no timer is set. */
  const runTimer = (): boolean => {
    if (timer === undefined) return false;
    const due = timer;
    timer = undefined;
    clock.now = Math.max(clock.now, due.at);
    due.run();
    return true;
  };
  const blocks = () => posts.flatMap((post) => (post.type === "block" ? [post] : []));
  const tickOf = (buffer: ArrayBuffer) => new DataView(buffer).getUint32(0, true);
  return { host, sim, calls, posts, clock, delays, runTimer, blocks, tickOf, hasTimer: () => timer !== undefined };
}

test("it says ready, then posts one transferred block per tick with two calls per tick", () => {
  const { host, calls, posts, delays, runTimer, blocks, tickOf } = setUp();
  host.start();
  expect(posts[0]).toEqual({ type: "ready", blockBytes: 8, tickHz: 30 });
  for (let i = 0; i < 90; i += 1) expect(runTimer()).toBe(true);
  // Every sleep is a whole number of milliseconds, so no timer fires before its tick is due: one wake, one tick.
  expect(delays.every((delay) => Number.isInteger(delay) && delay >= 33)).toBe(true);
  expect(host.stats().wakes).toBe(91);
  expect(blocks().map((block) => tickOf(block.buffer))).toEqual(Array.from({ length: 90 }, (_, i) => i + 1));
  // Exactly one input call and one step call per tick; the block's place and length were asked once, and the
  // schedule was set once, at the start (no pause, no drop).
  expect(calls).toEqual({ input: 90, step: 90, rebase: 1, block_ptr: 1, block_len: 1 });
  expect(host.stats()).toMatchObject({ ticks: 90, maxTicksInOneWake: 1, drops: 0, undetached: 0, blockBytes: 8 });
  // The time is on the clock the page shares: the first wake came 34 ms after the start (33.33 rounded up).
  expect(blocks()[0]?.doneAt).toBe(5_000_000 + 1000 + Math.ceil(PERIOD));
});

test("the hand-off's time is read when the step returned, before the copy", () => {
  const { host, runTimer, blocks } = setUp({ copyMs: 3 });
  host.start();
  runTimer();
  // The wake came 34 ms after the start; the copy then took 3 ms, and the block's time holds none of them
  // (PM-0002-8's method starts the hand-off "when its step returns, before the copy").
  expect(blocks()[0]?.doneAt).toBe(5_000_000 + 1000 + Math.ceil(PERIOD));
  expect(host.stats().copyP50).toBe(3);
});

test("the newest input reaches the simulation, and returned buffers are used again", () => {
  const { host, sim, runTimer, blocks } = setUp();
  host.start();
  for (let i = 0; i < 4; i += 1) runTimer();
  expect(host.stats().poolMisses).toBe(0);
  runTimer();
  // Five blocks out and none returned: the fifth had to be allocated.
  expect(host.stats().poolMisses).toBe(1);
  host.onMessage({ type: "input", sequence: 41, returned: blocks().slice(0, 3).map((block) => block.buffer) });
  host.onMessage({ type: "input", sequence: 42, returned: [] });
  for (let i = 0; i < 3; i += 1) runTimer();
  expect(sim.newest()).toBe(42);
  expect(host.stats()).toMatchObject({ poolMisses: 1, poolRefused: 0 });
});

test("a block that was posted without its transfer list is counted", () => {
  const { host, runTimer } = setUp({ transfer: false });
  host.start();
  for (let i = 0; i < 3; i += 1) runTimer();
  expect(host.stats().undetached).toBe(3);
});

test("a hidden tab runs no tick and sets no timer; shown again, the match goes on from its tick", () => {
  const { host, clock, runTimer, blocks, tickOf, hasTimer } = setUp();
  host.start();
  for (let i = 0; i < 10; i += 1) runTimer();
  host.onMessage({ type: "visibility", hidden: true });
  expect(hasTimer()).toBe(false);
  clock.now += 360_000;
  expect(runTimer()).toBe(false);
  host.onMessage({ type: "visibility", hidden: false });
  host.onMessage({ type: "visibility", hidden: false });
  for (let i = 0; i < 5; i += 1) runTimer();
  expect(blocks().map((block) => tickOf(block.buffer))).toEqual(Array.from({ length: 15 }, (_, i) => i + 1));
  expect(host.stats()).toMatchObject({ ticks: 15, pauses: 1, drops: 0, maxTicksInOneWake: 1 });
});

test("a report is sent when asked, and a message that makes no sense gets an error", () => {
  const { host, posts } = setUp();
  host.start();
  host.onMessage({ type: "report-request" });
  expect(posts.at(-1)).toMatchObject({ type: "report", stats: { hardwareConcurrency: 8, crossOriginIsolated: false } });
  for (const bad of [null, { type: "stop" }, { type: "start", padBytes: 0 }, { type: "input", sequence: -1 }]) {
    host.onMessage(bad);
    expect(posts.at(-1)).toEqual({ type: "error", message: "the Worker got a message it does not know" });
  }
});

test("a trap stops the match and tells the page once", () => {
  const { host, sim, posts, runTimer, hasTimer } = setUp();
  host.start();
  runTimer();
  sim.trapNext = true;
  runTimer();
  expect(posts.at(-1)).toEqual({ type: "error", message: "unreachable executed" });
  expect(hasTimer()).toBe(false);
  expect(posts.filter((post) => post.type === "error")).toHaveLength(1);
});
