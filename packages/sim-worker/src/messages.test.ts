import { expect, test } from "vitest";
import { isPageToWorker, isWorkerStats, isWorkerToPage, MODE_LOCAL, type PageToWorker, type WorkerStats, type WorkerToPage } from "./messages.ts";

test("local play is mode 0", () => {
  expect(MODE_LOCAL).toBe(0);
});

/** An input post with nothing held and no press; a test changes what it is about. */
const INPUT: PageToWorker = { type: "input", post: 1, held: 0, p0: 0, p1: 0, p2: 0, p3: 0, returned: [] };

test("every message the page may send is accepted", () => {
  const full = 0xffff_ffff;
  const good: PageToWorker[] = [
    { type: "start", padBytes: 0 },
    { type: "start", padBytes: 65536 },
    { ...INPUT, post: 0 },
    { type: "input", post: full, held: 0xffff, p0: full, p1: full, p2: full, p3: full, returned: [new ArrayBuffer(8), new ArrayBuffer(8)] },
    { type: "visibility", hidden: true },
    { type: "visibility", hidden: false },
    { type: "report-request" },
  ];
  for (const message of good) expect(isPageToWorker(message), JSON.stringify(message)).toBe(true);
});

test("anything else is refused", () => {
  const bad: unknown[] = [
    null,
    undefined,
    "start",
    42,
    [],
    {},
    { type: "stop" },
    { type: "start" },
    { type: "start", padBytes: -1 },
    { type: "start", padBytes: 1.5 },
    { type: "start", padBytes: 0x1_0000_0000 },
    { type: "start", padBytes: "8" },
    // The old shape, a post without one of its numbers, and each number outside its range.
    { type: "input", sequence: 1, returned: [] },
    { ...INPUT, returned: undefined },
    ...["post", "held", "p0", "p1", "p2", "p3"].flatMap((name) => [
      { ...INPUT, [name]: undefined },
      { ...INPUT, [name]: -1 },
      { ...INPUT, [name]: 1.5 },
      { ...INPUT, [name]: 0x1_0000_0000 },
      { ...INPUT, [name]: "1" },
    ]),
    { ...INPUT, held: 0x1_0000 },
    { ...INPUT, returned: [new Uint8Array(8)] },
    { ...INPUT, returned: "none" },
    { type: "visibility" },
    { type: "visibility", hidden: "yes" },
  ];
  for (const message of bad) expect(isPageToWorker(message), JSON.stringify(message)).toBe(false);
});

const STATS: WorkerStats = {
  ticks: 90,
  wakes: 91,
  maxTicksInOneWake: 1,
  drops: 0,
  droppedMs: 0,
  pauses: 0,
  stalePosts: 0,
  // No tick has started yet, or no block was copied: a reading without samples is NaN, and that is a valid report.
  latenessP50: Number.NaN,
  latenessP99: 1.5,
  latenessMax: 2,
  copyP50: 0.01,
  copyP99: 0.02,
  blockBytes: 8,
  poolMisses: 0,
  poolRefused: 0,
  undetached: 0,
  hardwareConcurrency: 8,
  crossOriginIsolated: false,
};

test("every message the Worker may send is accepted", () => {
  const good: WorkerToPage[] = [
    { type: "ready", blockBytes: 8, tickHz: 30 },
    { type: "block", buffer: new ArrayBuffer(8), startAt: 5_000_000, doneAt: 5_000_000.25 },
    { type: "error", message: "" },
    { type: "error", message: "panicked at crates/sim/src/world.rs:42:14:\nattempt to add with overflow" },
    { type: "report", stats: STATS },
  ];
  for (const message of good) expect(isWorkerToPage(message), message.type).toBe(true);
  expect(isWorkerStats(STATS)).toBe(true);
});

test("anything else from the Worker is refused", () => {
  const bad: unknown[] = [
    null,
    undefined,
    "ready",
    7,
    [],
    {},
    { type: "surprise" },
    { type: "ready" },
    { type: "ready", blockBytes: 8 },
    { type: "ready", blockBytes: "8", tickHz: 30 },
    { type: "ready", blockBytes: 8, tickHz: 29.97 },
    { type: "ready", blockBytes: -8, tickHz: 30 },
    { type: "block", buffer: new Uint8Array(8), startAt: 0, doneAt: 0 },
    { type: "block", buffer: new ArrayBuffer(8) },
    { type: "block", buffer: new ArrayBuffer(8), doneAt: 0 },
    { type: "block", buffer: new ArrayBuffer(8), startAt: 0 },
    { type: "block", buffer: new ArrayBuffer(8), startAt: 0, doneAt: Number.NaN },
    { type: "block", buffer: new ArrayBuffer(8), startAt: Number.POSITIVE_INFINITY, doneAt: 0 },
    { type: "block", buffer: new ArrayBuffer(8), startAt: "0", doneAt: 0 },
    { type: "error" },
    { type: "error", message: 42 },
    { type: "report" },
    { type: "report", stats: null },
    { type: "report", stats: { ticks: 5 } },
  ];
  for (const message of bad) expect(isWorkerToPage(message), JSON.stringify(message)).toBe(false);
});

test("the statistics are refused when one field is missing or of another kind, whichever field it is", () => {
  const names = Object.keys(STATS) as (keyof WorkerStats)[];
  expect(names).toHaveLength(18);
  for (const name of names) {
    const without: Record<string, unknown> = { ...STATS };
    delete without[name];
    expect(isWorkerStats(without), `without ${name}`).toBe(false);
    expect(isWorkerStats({ ...STATS, [name]: "1" }), `${name} as text`).toBe(false);
    expect(isWorkerStats({ ...STATS, [name]: typeof STATS[name] === "boolean" ? 1 : true }), `${name} as the other kind`).toBe(false);
  }
  // A field more than the page knows does not make a report invalid: the Worker may be one build ahead in what it counts.
  expect(isWorkerStats({ ...STATS, extra: 1 })).toBe(true);
});
