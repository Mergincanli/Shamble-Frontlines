import { LAYOUT_HASH } from "@shamble/types";
import { expect, test } from "vitest";
import { WorkerLink } from "./worker-link.ts";
import type { PageToWorker, WorkerStats } from "@shamble/sim-worker";

function block(tick: number, hash = LAYOUT_HASH, bytes = 8): ArrayBuffer {
  const buffer = new ArrayBuffer(bytes);
  const view = new DataView(buffer);
  view.setUint32(0, tick, true);
  view.setUint32(4, hash, true);
  return buffer;
}

const STATS: WorkerStats = {
  ticks: 5,
  wakes: 6,
  maxTicksInOneWake: 1,
  drops: 0,
  droppedMs: 0,
  pauses: 0,
  stalePosts: 0,
  latenessP50: 0.5,
  latenessP99: 1,
  latenessMax: 1,
  copyP50: Number.NaN,
  copyP99: Number.NaN,
  blockBytes: 8,
  poolMisses: 0,
  poolRefused: 0,
  undetached: 0,
  hardwareConcurrency: 8,
  crossOriginIsolated: false,
};

function setUp() {
  const sent: { message: PageToWorker; transfer: ArrayBuffer[] }[] = [];
  const clock = { now: 10_000 };
  const port = { terminated: 0 };
  const link = new WorkerLink(
    { postMessage: (message, transfer) => sent.push({ message, transfer }), terminate: () => (port.terminated += 1) },
    () => clock.now,
  );
  const ready = () => link.onMessage({ type: "ready", blockBytes: 8, tickHz: 30 });
  const deliver = (tick: number) => link.onMessage({ type: "block", buffer: block(tick), startAt: 0, doneAt: clock.now - 2 });
  return { link, sent, clock, port, ready, deliver };
}

test("it starts the Worker, and posts no input before the Worker is ready", () => {
  const { link, sent, ready } = setUp();
  link.start(4096);
  link.frame(0, 0, 0, 0, 0);
  expect(sent.map((entry) => entry.message)).toEqual([{ type: "start", padBytes: 4096 }]);
  expect(ready()).toBe("");
  expect(link).toMatchObject({ ready: true, blockBytes: 8, tickHz: 30 });
});

test("it keeps the two newest blocks and sends the older ones back with the next frame's input", () => {
  const { link, sent, ready, deliver } = setUp();
  ready();
  const blocks = [1, 2, 3, 4].map((tick) => (deliver(tick), link.newest));
  expect(link).toMatchObject({ blocks: 4, tick: 4 });
  expect(link.newest).toBe(blocks[3]);
  expect(link.previous).toBe(blocks[2]);
  link.frame(0x8001, 1, 2, 3, 4);
  expect(sent.at(-1)).toEqual({
    message: { type: "input", post: 1, held: 0x8001, p0: 1, p1: 2, p2: 3, p3: 4, returned: [blocks[0], blocks[1]] },
    transfer: [blocks[0], blocks[1]],
  });
  // One message per frame, each with the next post number, and a returned buffer goes back once.
  link.frame(0, 1, 2, 3, 5);
  expect(sent.at(-1)?.message).toEqual({ type: "input", post: 2, held: 0, p0: 1, p1: 2, p2: 3, p3: 5, returned: [] });
  expect(link.posts).toBe(2);
  expect(link.delivery.count).toBe(4);
  expect(link.delivery.max()).toBe(2);
});

test("the first block of every Worker start is checked, also after blocks were accepted", () => {
  const { link, ready, deliver } = setUp();
  ready();
  deliver(1);
  // The Worker starts again (a reload of the Worker, later a rejoin): its first block is checked anew.
  ready();
  expect(link.onMessage({ type: "block", buffer: block(1, 0), startAt: 0, doneAt: 0 })).toContain("layout-hash");
  expect(link.blocks).toBe(1);
});

test("a first block of another layout is refused, and the failure is said once", () => {
  const { link, sent, ready } = setUp();
  ready();
  const reason = link.onMessage({ type: "block", buffer: block(1, LAYOUT_HASH ^ 1), startAt: 0, doneAt: 0 });
  expect(reason).toContain("layout-hash");
  expect(link).toMatchObject({ blocks: 0, newest: null, failure: reason });
  link.frame(0, 0, 0, 0, 0);
  expect(sent).toHaveLength(0);
  // One cause is one error: what still arrives after the failure is ignored, a good block too.
  expect(link.onMessage({ type: "block", buffer: block(2, LAYOUT_HASH ^ 1), startAt: 0, doneAt: 0 })).toBe("");
  expect(link.onMessage({ type: "block", buffer: block(3), startAt: 0, doneAt: 0 })).toBe("");
  expect(link).toMatchObject({ blocks: 0, newest: null, failure: reason });
});

test("a block of the wrong length, an error from the Worker and an unknown message each end the match and stop the Worker", () => {
  for (const [message, said] of [
    [{ type: "block", buffer: block(1, LAYOUT_HASH, 16), startAt: 0, doneAt: 0 }, "a block of 16 bytes, not 8"],
    [{ type: "error", message: "unreachable executed" }, "unreachable executed"],
    [{ type: "surprise" }, "the Worker sent a message the page does not know"],
    [{ type: "report", stats: { ticks: 5 } }, "the Worker sent a message the page does not know"],
    [null, "the Worker sent a message the page does not know"],
  ] as const) {
    const { link, port, ready } = setUp();
    ready();
    expect(link.onMessage(message), JSON.stringify(message)).toBe(said);
    expect(link.failure).toBe(said);
    expect(port.terminated).toBe(1);
  }
});

test("the match ends once, whoever ends it first, and nothing is posted to the Worker after that", () => {
  // The Worker's own error event (boot.ts) ends the match through the link, so the link knows too.
  const { link, sent, port, ready, deliver } = setUp();
  ready();
  deliver(1);
  expect(link.end("the Worker failed")).toBe(true);
  expect(link.failure).toBe("the Worker failed");
  expect(port.terminated).toBe(1);
  // A second cause is not a second error: the page counts and shows only the first.
  expect(link.end("another reason")).toBe(false);
  expect(link.onMessage({ type: "error", message: "late" })).toBe("");
  expect(link.failure).toBe("the Worker failed");
  expect(port.terminated).toBe(1);
  const before = sent.length;
  link.frame(0, 0, 0, 0, 0);
  link.requestReport();
  link.visibility(true);
  link.visibility(false);
  expect(sent).toHaveLength(before);
  expect(link).toMatchObject({ posts: 0, pauses: 0, lastPause: null });
  // An end without a reason still is an end.
  const other = setUp();
  expect(other.link.end("")).toBe(true);
  expect(other.link.failure).not.toBe("");
  expect(other.link.end("again")).toBe(false);
});

test("what the link knows can be read and not written from outside", () => {
  const { link, ready, deliver } = setUp();
  ready();
  deliver(3);
  expect(link).toMatchObject({ ready: true, blockBytes: 8, tickHz: 30, blocks: 1, tick: 3, posts: 0, failure: "", stats: null, pauses: 0 });
  const open = link as unknown as Record<string, unknown>;
  for (const name of ["ready", "blockBytes", "tickHz", "blocks", "tick", "posts", "failure", "stats", "lastPause", "pauses"]) {
    expect(() => (open[name] = 99), name).toThrow(TypeError);
  }
  expect(link).toMatchObject({ ready: true, blocks: 1, tick: 3, failure: "" });
});

test("a pause is recorded with the ticks that ran while the tab was hidden", () => {
  const { link, sent, clock, ready, deliver } = setUp();
  ready();
  deliver(30);
  link.visibility(true);
  clock.now += 61_000;
  link.visibility(false);
  expect(link.lastPause).toEqual({ hiddenMs: 61_000, ticksWhileHidden: 0 });
  expect(sent.map((entry) => entry.message)).toEqual([{ type: "visibility", hidden: true }, { type: "visibility", hidden: false }]);
  // A Worker that ran on while hidden shows up here.
  link.visibility(true);
  deliver(1830);
  link.visibility(false);
  expect(link.lastPause?.ticksWhileHidden).toBe(1800);
  expect(link.pauses).toBe(2);
});

test("the Worker's report is kept", () => {
  const { link, sent, ready } = setUp();
  link.requestReport();
  expect(sent).toHaveLength(0);
  ready();
  link.requestReport();
  expect(sent.at(-1)?.message).toEqual({ type: "report-request" });
  expect(link.onMessage({ type: "report", stats: STATS })).toBe("");
  expect(link.stats).toEqual(STATS);
});
