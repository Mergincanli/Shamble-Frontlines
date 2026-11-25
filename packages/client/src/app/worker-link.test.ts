import { LAYOUT_HASH } from "@shamble/types";
import { expect, test } from "vitest";
import { type KeySource, WorkerLink } from "./worker-link.ts";
import type { PageToWorker, WorkerStats } from "@shamble/sim-worker";

/** A block as the Worker sends it: the header, then the echo (held, four words of counts) and the link's counts. */
function block(tick: number, hash = LAYOUT_HASH, bytes = 44, rest: readonly number[] = []): ArrayBuffer {
  const buffer = new ArrayBuffer(bytes);
  const view = new DataView(buffer);
  [tick, hash, ...rest].forEach((word, at) => view.setUint32(4 * at, word, true));
  return buffer;
}

/** The keys as the link reads them: these held, each word of counts its own number, last changed at `changedAt`. */
const keys = (held = 0, changedAt = 0): KeySource => ({ held, changedAt, word: (index) => 10 + index });
const NO_KEYS = keys();

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
  const ready = () => link.onMessage({ type: "ready", blockBytes: 44, tickHz: 30 });
  const deliver = (tick: number) => link.onMessage({ type: "block", buffer: block(tick), startAt: 0, doneAt: clock.now - 2 });
  return { link, sent, clock, port, ready, deliver };
}

test("it starts the Worker, and posts no input before the Worker is ready", () => {
  const { link, sent, ready } = setUp();
  link.start(4096);
  link.frame(NO_KEYS);
  expect(sent.map((entry) => entry.message)).toEqual([{ type: "start", padBytes: 4096 }]);
  expect(ready()).toBe("");
  expect(link).toMatchObject({ ready: true, blockBytes: 44, tickHz: 30 });
});

test("it keeps the two newest blocks and sends the older ones back with the next frame's input", () => {
  const { link, sent, ready, deliver } = setUp();
  ready();
  const blocks = [1, 2, 3, 4].map((tick) => (deliver(tick), link.newest));
  expect(link).toMatchObject({ blocks: 4, tick: 4 });
  expect(link.newest).toBe(blocks[3]);
  expect(link.previous).toBe(blocks[2]);
  link.frame(keys(0x8001));
  expect(sent.at(-1)).toEqual({
    message: { type: "input", post: 1, held: 0x8001, p0: 10, p1: 11, p2: 12, p3: 13, returned: [blocks[0], blocks[1]] },
    transfer: [blocks[0], blocks[1]],
  });
  // One message per frame, each with the next post number, and a returned buffer goes back once.
  link.frame(NO_KEYS);
  expect(sent.at(-1)?.message).toEqual({ type: "input", post: 2, held: 0, p0: 10, p1: 11, p2: 12, p3: 13, returned: [] });
  expect(link.posts).toBe(2);
  expect(link.delivery.count).toBe(4);
  expect(link.delivery.max()).toBe(2);
  // Each post has a list of its own, exactly as long as it needs, and a list given away is never touched again.
  const given = sent.slice(-2).map((entry) => entry.transfer);
  deliver(5);
  link.frame(NO_KEYS);
  expect(sent.at(-1)?.transfer).toEqual([blocks[2]]);
  expect(given).toEqual([[blocks[0], blocks[1]], []]);
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

test("a Worker that starts again has used no post yet: its first blocks are not behind the old one's last", () => {
  const { link, ready } = setUp();
  ready();
  const tick = (postUsed: number) =>
    link.onMessage({ type: "block", buffer: block(1, LAYOUT_HASH, 44, [0, 0, 0, 0, 0, 0, 0, 0, postUsed]), startAt: 0, doneAt: 0 });
  tick(9);
  ready();
  tick(0);
  tick(12);
  expect([link.olderPosts, link.postUsed]).toEqual([0, 12]);
});

test("a first block of another layout is refused, and the failure is said once", () => {
  const { link, sent, ready } = setUp();
  ready();
  const reason = link.onMessage({ type: "block", buffer: block(1, LAYOUT_HASH ^ 1), startAt: 0, doneAt: 0 });
  expect(reason).toContain("layout-hash");
  expect(link).toMatchObject({ blocks: 0, newest: null, failure: reason });
  link.frame(NO_KEYS);
  expect(sent).toHaveLength(0);
  // One cause is one error: what still arrives after the failure is ignored, a good block too.
  expect(link.onMessage({ type: "block", buffer: block(2, LAYOUT_HASH ^ 1), startAt: 0, doneAt: 0 })).toBe("");
  expect(link.onMessage({ type: "block", buffer: block(3), startAt: 0, doneAt: 0 })).toBe("");
  expect(link).toMatchObject({ blocks: 0, newest: null, failure: reason });
});

test("a block of the wrong length, an error from the Worker and an unknown message each end the match and stop the Worker", () => {
  for (const [message, said] of [
    [{ type: "block", buffer: block(1, LAYOUT_HASH, 48), startAt: 0, doneAt: 0 }, "a block of 48 bytes, not 44"],
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
  link.frame(NO_KEYS);
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
  expect(link).toMatchObject({ ready: true, blockBytes: 44, tickHz: 30, blocks: 1, tick: 3, posts: 0, failure: "", stats: null, pauses: 0 });
  const open = link as unknown as Record<string, unknown>;
  const read = ["ready", "blockBytes", "tickHz", "blocks", "tick", "posts", "failure", "stats", "lastPause", "pauses"];
  for (const name of [...read, "echo", "echoHeld", "sent", "received", "refused", "postUsed", "keyToTick"]) {
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

test("the echo and the link's counts are read from every block, and a block too short for them ends the match", () => {
  const { link, ready } = setUp();
  ready();
  // Held: buttons 0 and 5. Counts: button 0 once, button 5 twice. Then sent, received, refused, the post used.
  const first = block(1, LAYOUT_HASH, 44, [0x21, 1, 0x0200, 0, 0, 7, 8, 0, 6]);
  expect(link.onMessage({ type: "block", buffer: first, startAt: 0, doneAt: 0 })).toBe("");
  expect(link).toMatchObject({ echoHeld: 0x21, sent: 7, received: 8, refused: 0, postUsed: 6, olderPosts: 0 });
  expect([link.echo.presses, link.echo.releases]).toEqual([3, 1]);
  // Every block counts, also one the frame loop never looks at: two more presses of button 0, and both let go.
  for (const count of [2, 3]) link.onMessage({ type: "block", buffer: block(count, LAYOUT_HASH, 44, [0, count, 0x0200, 0, 0, 9, 9, 2, 8]), startAt: 0, doneAt: 0 });
  expect(link).toMatchObject({ echoHeld: 0, refused: 2, postUsed: 8, olderPosts: 0 });
  expect([link.echo.presses, link.echo.releases, link.echo.takePressed()]).toEqual([5, 5, 0x21]);
  // A tick that used post 7 after a tick used post 8 ran the keys out of order, and so does one across the wrap;
  // the same post again, or a later one, did not. Button 0 held again with no press is counted apart.
  for (const [postUsed, held] of [[7, 1], [7, 1], [0xffff_fffe, 0], [1, 0], [0xffff_ffff, 0]] as const) {
    link.onMessage({ type: "block", buffer: block(4, LAYOUT_HASH, 44, [held, 3, 0x0200, 0, 0, 9, 9, 2, postUsed]), startAt: 0, doneAt: 0 });
  }
  expect([link.olderPosts, link.echo.heldWithoutPress, link.echo.presses]).toEqual([3, 1, 5]);

  // A Worker whose blocks pass the hash and are shorter than the layout's sections: refused before any read past the header.
  const short = setUp();
  short.link.onMessage({ type: "ready", blockBytes: 8, tickHz: 30 });
  const reason = short.link.onMessage({ type: "block", buffer: block(1, LAYOUT_HASH, 8), startAt: 0, doneAt: 0 });
  expect(reason).toBe("a block of 8 bytes is shorter than its 44 bytes of sections");
  expect(short.port.terminated).toBe(1);
});

test("a key change is timed to the start of the first tick whose command was built from a post that held it", () => {
  const { link, ready } = setUp();
  ready();
  const tick = (postUsed: number, startAt: number) =>
    link.onMessage({ type: "block", buffer: block(1, LAYOUT_HASH, 44, [0, 0, 0, 0, 0, 0, 0, 0, postUsed]), startAt, doneAt: startAt });
  link.frame(keys(0, 0));
  link.frame(keys(1, 1000)); // post 2 is the first to hold the key that went down at 1000
  tick(1, 1010); // a tick that still ran post 1: not yet
  expect(link.keyToTick.count).toBe(0);
  link.frame(keys(0, 1012)); // a second change while the first is being timed is not timed
  tick(3, 1030); // this tick ran post 3, which held the change: 30 ms after it
  expect([link.keyToTick.count, link.keyToTick.max()]).toEqual([1, 30]);
  tick(3, 1063);
  expect(link.keyToTick.count).toBe(1);
  // The next change is timed again, from its own post on.
  link.frame(keys(1, 2000));
  tick(3, 2001);
  tick(4, 2040);
  expect([link.keyToTick.count, link.keyToTick.max()]).toEqual([2, 40]);
});
