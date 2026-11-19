import { LAYOUT_HASH } from "@shamble/types";
import { expect, test } from "vitest";
import { WorkerLink } from "./worker-link.ts";
import type { PageToWorker } from "@shamble/sim-worker";

function block(tick: number, hash = LAYOUT_HASH, bytes = 8): ArrayBuffer {
  const buffer = new ArrayBuffer(bytes);
  const view = new DataView(buffer);
  view.setUint32(0, tick, true);
  view.setUint32(4, hash, true);
  return buffer;
}

function setUp() {
  const sent: { message: PageToWorker; transfer: ArrayBuffer[] }[] = [];
  const clock = { now: 10_000 };
  const link = new WorkerLink({ postMessage: (message, transfer) => sent.push({ message, transfer }) }, () => clock.now);
  const ready = () => link.onMessage({ type: "ready", blockBytes: 8, tickHz: 30 });
  const deliver = (tick: number) => link.onMessage({ type: "block", buffer: block(tick), doneAt: clock.now - 2 });
  return { link, sent, clock, ready, deliver };
}

test("it starts the Worker, and posts no input before the Worker is ready", () => {
  const { link, sent, ready } = setUp();
  link.start(4096);
  link.frame(1, 0);
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
  link.frame(7, 123.5);
  expect(sent.at(-1)).toEqual({
    message: { type: "input", sequence: 7, viewTime: 123.5, returned: [blocks[0], blocks[1]] },
    transfer: [blocks[0], blocks[1]],
  });
  // One message per frame, and a returned buffer goes back once.
  link.frame(8, 140);
  expect(sent.at(-1)?.message).toEqual({ type: "input", sequence: 8, viewTime: 140, returned: [] });
  expect(link.posts).toBe(2);
  expect(link.delivery.count).toBe(4);
  expect(link.delivery.max()).toBe(2);
});

test("a first block of another layout is refused, and so is every block after it", () => {
  const { link, sent, ready } = setUp();
  ready();
  const reason = link.onMessage({ type: "block", buffer: block(1, LAYOUT_HASH ^ 1), doneAt: 0 });
  expect(reason).toContain("layout-hash");
  expect(link).toMatchObject({ blocks: 0, newest: null, failure: reason });
  link.frame(1, 0);
  expect(sent).toHaveLength(0);
  // A new Worker start is checked again.
  ready();
  expect(link.onMessage({ type: "block", buffer: block(1, 0), doneAt: 0 })).toContain("layout-hash");
});

test("a block of the wrong length, an error from the Worker and an unknown message each stop the match", () => {
  for (const message of [
    { type: "block", buffer: block(1, LAYOUT_HASH, 16), doneAt: 0 },
    { type: "error", message: "unreachable executed" },
    { type: "surprise" },
    null,
  ]) {
    const { link, ready } = setUp();
    ready();
    expect(link.onMessage(message), JSON.stringify(message)).not.toBe("");
    expect(link.failure).not.toBe("");
  }
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
  expect(link.onMessage({ type: "report", stats: { ticks: 5 } })).toBe("");
  expect(link.stats).toEqual({ ticks: 5 });
});
