// worker.test.mjs: Block T3's Worker engine over a fake clock, fake transports and the fake core (u7-brief.md rev 1.1
// sections 1 and 3; C-U7-3; C1, C9): the join on one transport, Playing, the records, stamping, the bounded queue, the
// timer from next_due_ns, the trap, the caps.
import assert from "node:assert/strict";
import { test } from "node:test";
import { TICK_NS, TYPE, makeCore } from "./fake-core.mjs";
import { FakeClock, FakeWs, FakeWt, connectDeps, flush, hookedWritable } from "./fakes.mjs";
import { checkLine, load, token } from "./helpers.mjs";

const W = await load("worker.js");
const TOKEN = token();
const link = (o = {}) => ({ suite: "smoke", room: "10", transport: "wt", attempt: 0, token: TOKEN, wrongHash: false, bite: null, ...o });
const JOIN_ACCEPTED = [1, 0, TYPE.joinAccepted];
const PONG = [TYPE.pong, 1];
const FULL_STATE = [1, 0, 0, 0, TYPE.fullState];

function rig(o = {}) {
  const c = new FakeClock();
  const core = o.core ?? makeCore(o.coreOpts);
  const cd = connectDeps(link(o.link), c, { timeoutMs: o.timeoutMs === undefined ? 1500 : o.timeoutMs });
  const posts = [];
  const e = new W.Engine({
    now: c.now, setTimer: c.setTimer, clearTimer: c.clearTimer, fetchText: cd.fetchText, webTransport: cd.webTransport,
    webSocket: cd.webSocket, randomBytes: (n) => new Uint8Array(n).fill(7),
    loadCore: () => (o.coreDelayMs ? new Promise((ok) => c.setTimer(() => ok(core), o.coreDelayMs)) : Promise.resolve(core)),
    post: (m) => posts.push(m),
  });
  const r = {
    c, core, posts, e,
    start: () => e.start({ kind: "start", link: link(o.link), host: "127.0.0.1", wtTimeoutMs: o.timeoutMs === undefined ? 1500 : o.timeoutMs,
      mdsPeriodMs: o.mdsPeriodMs === undefined ? 10000 : o.mdsPeriodMs }),
    lines: () => posts.filter((p) => p.kind === "records").flatMap((p) => p.lines),
    events: (ev) => r.lines().map((l) => JSON.parse(l)).filter((x) => x.k === "event" && x.ev === ev),
    statuses: () => posts.filter((p) => p.kind === "status").map((p) => p.text),
    counters: () => posts.filter((p) => p.kind === "counters").at(-1),
  };
  return r;
}

/** Starts a smoke session over WebTransport, opened at 100 ms; returns the rig and the fake session. */
async function wtSession(o = {}) {
  const r = rig(o);
  void r.start();
  await r.c.advance(100);
  const wt = FakeWt.made[0];
  o.before?.(wt, r.c);
  wt.open();
  await r.c.advance(0);
  return { r, wt };
}

/** The counter `name` in the last counters post (undefined when absent). */
const counter = (r, name) => {
  const k = r.counters();
  return k?.values[k.names.indexOf(name)];
};

test("WebTransport: the Join on the control stream, datagrams on the writer, then Playing and attempt.ok 1", async () => {
  const { r, wt } = await wtSession();
  assert.equal(r.core.log.clients.length, 1);
  assert.equal(r.core.log.clients[0].token, TOKEN);
  assert.equal(r.core.log.clients[0].bind.length, 16);
  assert.equal(wt.bidis.length, 1);
  assert.deepEqual([...wt.bidis[0].written[0]], [3, 0, TYPE.join, 0xa1, 0xa2]);
  assert.ok(wt.dg.written.length >= 1);
  assert.deepEqual(r.statuses(), ["WebTransport · Connecting"]);
  wt.bidis[0].push(JOIN_ACCEPTED);
  wt.dg.push(PONG);
  wt.pushUni([FULL_STATE]).end();
  await r.c.advance(40);
  assert.equal(r.statuses().at(-1), "WebTransport · Playing");
  assert.deepEqual(r.events("attempt").map((x) => x.f), [{ ok: 1 }]);
  assert.equal(r.events("fallback").length, 0);
  assert.equal(FakeWs.made.length, 0);
  for (const l of r.lines()) checkLine(l, TOKEN);
  assert.equal(JSON.stringify(r.posts).includes(TOKEN.slice(5)), false, "no post holds the token");
});

test("stamps: each message at its own read, in ns; prev_send_ns 0 first, then the last wake's send time (C-U7-3)", async () => {
  const { r, wt } = await wtSession();
  await r.c.advance(5);
  const at = r.c.t;
  wt.dg.push(PONG);
  wt.bidis[0].push(JOIN_ACCEPTED);
  await r.c.advance(50);
  const steps = r.core.log.steps;
  assert.equal(steps[0].prevSendNs, 0n);
  const seen = steps.flatMap((s) => s.records);
  assert.deepEqual(seen.map((x) => x.arrivalNs), [BigInt(at * 1e6), BigInt(at * 1e6)]);
  assert.deepEqual(seen.map((x) => x.channel), [0, 1]);
  for (let i = 1; i < steps.length; i++) assert.equal(steps[i].prevSendNs, steps[i - 1].nowNs, `step ${i}`);
});

test("the send time is stamped after the wake's sends: each write taking 1 ms moves prev_send_ns on by it (C-U7-3, U7R-TE-2 (b))", async () => {
  const { r } = await wtSession({ before: (wt, c) => wt.useHook({ onWrite: () => void (c.t += 1) }) });
  await r.c.advance(300);
  const steps = r.core.log.steps;
  assert.ok(steps.filter((s) => s.sent > 0).length >= 8, "wakes that sent");
  for (let i = 1; i < steps.length; i++) {
    assert.equal(steps[i].prevSendNs, steps[i - 1].nowNs + BigInt(steps[i - 1].sent) * 1_000_000n, `step ${i}`);
  }
});

test("JoinAccepted alone, with no clock sample, writes no attempt.ok (PM-0006-15's one echo; U7R-TE-8)", async () => {
  const { r, wt } = await wtSession();
  wt.bidis[0].push(JOIN_ACCEPTED);
  await r.c.advance(2000);
  assert.equal(r.statuses().at(-1), "WebTransport · Joined");
  assert.equal(r.events("attempt").length, 0);
});

test("a stuck datagram writer: 64 writes in flight, every later one dropped and counted (U7R-TE-8)", async () => {
  const { r, wt } = await wtSession({ before: (w) => w.useHook({ stuck: true }) });
  await r.c.advance(3000);
  const sent = r.core.log.steps.reduce((n, s) => n + s.sent, 0) - 1;
  assert.ok(sent > 100, String(sent));
  assert.equal(wt.dg.written.length, 64, "64 in flight, none after");
  const dropped = counter(r, "page_datagram_send_dropped");
  assert.ok(dropped > 0 && dropped <= sent - 64, `${dropped} of ${sent - 64}`);
});

test("a 5th full-state stream open at once is refused and counted; the four keep reading (U7R-TE-8)", async () => {
  const { r, wt } = await wtSession();
  for (let i = 0; i < 5; i++) wt.pushUni([FULL_STATE]);
  await r.c.advance(1100);
  assert.equal(counter(r, "page_uni_refused"), 1);
  assert.equal(r.core.log.steps.flatMap((s) => s.records).filter((x) => x.channel === 2).length, 4);
});

test("WebSocket over its 1 MiB buffer: datagrams dropped and counted; a control message ends the session (U7R-TE-8)", async () => {
  for (const at of ["after the Join", "before the Join"]) {
    const r = rig({ link: { transport: "ws" } });
    void r.start();
    await r.c.advance(100);
    const ws = FakeWs.made[0];
    if (at === "before the Join") ws.bufferedAmount = 1_048_577;
    ws.open();
    await r.c.advance(0);
    const before = ws.sent.length;
    ws.bufferedAmount = 1_048_577;
    await r.c.advance(1100);
    if (at === "after the Join") {
      assert.ok(before >= 1 && ws.sent.length === before, "nothing sent once the buffer is over");
      assert.ok(counter(r, "page_datagram_send_dropped") > 0);
      assert.equal(r.statuses().at(-1), "WebSocket · Connecting");
    } else {
      assert.equal(ws.sent.length, 0);
      assert.equal(r.statuses().at(-1), "Ended: the WebSocket's send buffer is full");
    }
  }
});

test("a session whose datagrams offer createWritable() writes through it (Firefox's path; U7R-TE-8)", async () => {
  const via = [];
  const { wt } = await wtSession({ before: (w) => void (w.datagrams.createWritable = () => hookedWritable(via)) });
  assert.ok(via.length >= 1);
  assert.equal(wt.dg.written.length, 0, "not the plain writable");
});

test("securitypolicyviolation in the Worker counts in page_csp_violations (U7R-NC-7)", async () => {
  const { r } = await wtSession();
  assert.equal(counter(r, "page_csp_violations"), undefined, "no counters post yet");
  r.e.cspViolation();
  await r.c.advance(1000);
  assert.equal(counter(r, "page_csp_violations"), 1);
});

test("the partial second is written at the end (U7R-NC-6)", async () => {
  const { r, wt } = await wtSession();
  wt.datagrams.maxDatagramSize = 1000;
  await r.c.advance(1500);
  wt.dg.push(new Array(1100).fill(2));
  await r.c.advance(10);
  wt.close();
  await r.c.advance(10);
  const secs = r.lines().map((l) => JSON.parse(l)).filter((x) => x.k === "second");
  assert.deepEqual(secs.map((x) => [x.sec, x.v.recv_over_max_count]), [[0, 0], [1, 1]]);
});

test("second stays on the Worker clock's grid when its timer runs late: no sec skipped over 300 s (U7R-NC-6)", async () => {
  const { r } = await wtSession({ before: (_, c) => void (c.lateMs = 7) });
  await r.c.advance(300_000);
  const secs = r.lines().map((l) => JSON.parse(l)).filter((x) => x.k === "second").map((x) => x.sec);
  assert.ok(secs.length >= 298, String(secs.length));
  assert.deepEqual(secs, secs.map((_, i) => i));
});

test("the timer follows next_due_ns: about 30 wakes a second", async () => {
  const { r } = await wtSession();
  const before = r.core.log.steps.length;
  await r.c.advance(1000);
  const n = r.core.log.steps.length - before;
  assert.ok(n >= 29 && n <= 31, `${n} wakes`);
  assert.equal(TICK_NS, 33_333_333n);
});

test("the inbox holds at most 512 between wakes; the rest is counted on the page", async () => {
  const { r, wt } = await wtSession();
  await r.c.advance(5);
  for (let i = 0; i < 600; i++) wt.dg.push([TYPE.snapshot, i & 0xff]);
  await r.c.advance(30);
  assert.equal(Math.max(...r.core.log.steps.map((s) => s.records.length)), 512);
  await r.c.advance(1000);
  const k = r.counters();
  assert.equal(k.values[k.names.indexOf("page_inbox_dropped")], 88);
  assert.ok(k.names.includes("joined") && k.names.includes("steps"));
});

test("mds_read at session ready and every 10 s (P12); a second record each second with recv_over_max_count", async () => {
  const { r, wt } = await wtSession();
  wt.datagrams.maxDatagramSize = 1000;
  wt.dg.push(new Array(1100).fill(2));
  wt.dg.push(new Array(1200).fill(2));
  wt.dg.push(new Array(1151).fill(2));
  await r.c.advance(21000);
  const reads = r.events("mds_read");
  assert.equal(reads.length, 3);
  assert.deepEqual(reads.map((x) => x.f.size_bytes), [1024, 1000, 1000]);
  assert.deepEqual(reads.map((x) => Math.round(x.ns / 1e8)), [1, 101, 201]);
  const secs = r.lines().map((l) => JSON.parse(l)).filter((x) => x.k === "second");
  assert.ok(secs.length >= 19);
  assert.deepEqual(secs.map((x) => x.sec), secs.map((_, i) => i));
  assert.equal(secs[0].v.recv_over_max_count, 3);
  assert.equal(secs.slice(1).every((x) => x.v.recv_over_max_count === 0), true);
  const k = r.counters();
  assert.equal(k.values[k.names.indexOf("page_datagram_over_cap")], 2, "1,200 B and 1,151 B are over the 1,150 B cap");
  for (const l of r.lines()) checkLine(l, TOKEN);
});

test("fallback: WebSocket at 1.5 s with the tags from limits(); Join on it only (C9); fallback.playing_ns once", async () => {
  const r = rig({ coreOpts: { limits: [256, 65541, 1150, 7, 8, 9] } });
  void r.start();
  await r.c.advance(1500);
  const ws = FakeWs.made[0];
  await r.c.advance(100);
  ws.open();
  await r.c.advance(0);
  assert.deepEqual([...ws.sent[0]], [8, 3, 0, TYPE.join, 0xa1, 0xa2]);
  assert.ok(ws.sent.slice(1).every((m) => m[0] === 7), "datagrams behind tag 7");
  FakeWt.made[0].open();
  await r.c.advance(100);
  assert.equal(FakeWt.made[0].bidis.length, 0, "no control stream, so no Join, on the late WebTransport");
  assert.equal(FakeWt.made[0].closeCalls, 2);
  ws.message([8, ...JOIN_ACCEPTED]);
  ws.message([7, ...PONG]);
  ws.message([9, ...FULL_STATE]);
  await r.c.advance(40);
  assert.equal(r.statuses().at(-1), "WebSocket · Playing");
  const fb = r.events("fallback");
  assert.equal(fb.length, 1);
  assert.ok(fb[0].f.playing_ns >= 1.5e9 && fb[0].f.playing_ns <= 3e9, String(fb[0].f.playing_ns));
  assert.equal(fb[0].f.playing_ns, fb[0].ns);
  assert.deepEqual(r.events("attempt").map((x) => x.f), [{ ok: 1 }]);
  assert.equal(r.events("mds_read").length, 0, "no maxDatagramSize over WebSocket");
});

test("cert-14d: a failed session writes attempt.ok 0 at once and never falls back (P13)", async () => {
  const r = rig({ link: { suite: "cert-14d" } });
  void r.start();
  await r.c.advance(300);
  FakeWt.made[0].fail();
  await r.c.advance(0);
  assert.deepEqual(r.events("attempt").map((x) => [x.f.ok, x.ns]), [[0, 3e8]]);
  assert.equal(r.statuses().at(-1), "Ended: the WebTransport session did not open");
  await r.c.advance(15000);
  assert.equal(FakeWs.made.length, 0);
  assert.equal(r.events("attempt").length, 1);
});

test("the attempt deadline: no JoinAccepted in 10 s writes attempt.ok 0, once", async () => {
  const { r } = await wtSession();
  await r.c.advance(9800);
  assert.equal(r.events("attempt").length, 0);
  await r.c.advance(200);
  assert.deepEqual(r.events("attempt").map((x) => [x.f.ok, x.ns]), [[0, 1e10]]);
  await r.c.advance(5000);
  assert.equal(r.events("attempt").length, 1);
});

test("a trap ends the session (Ended), no restart, no new client", async () => {
  const { r, wt } = await wtSession({ coreOpts: { trapAt: 3 } });
  await r.c.advance(500);
  assert.equal(r.statuses().at(-1), "Ended: the client trapped");
  assert.equal(r.core.log.clients.length, 1);
  const n = r.core.log.steps.length;
  await r.c.advance(2000);
  assert.equal(r.core.log.steps.length, n);
  assert.ok(wt.closeCalls >= 1);
  assert.deepEqual(r.events("attempt").map((x) => x.f.ok), [0]);
});

test("caps: a control message over 256 B ends the session; a full-state stream cut short ends it", async () => {
  const a = await wtSession();
  a.wt.bidis[0].push([255, 0, TYPE.joinAccepted]);
  await a.r.c.advance(40);
  assert.equal(a.r.statuses().at(-1), "Ended: a stream message over its cap");
  const b = await wtSession();
  b.wt.pushUni([[5, 0, 0, 0, 3]]).end();
  await b.r.c.advance(40);
  assert.equal(b.r.statuses().at(-1), "Ended: a stream ended inside a message");
});

test("a client without the needed counters, or with malformed limits, starts nothing", async () => {
  const a = rig({ coreOpts: { names: ["joined", "steps"] } });
  void a.start();
  await a.c.advance(100);
  FakeWt.made[0].open();
  await a.c.advance(0);
  assert.equal(a.statuses().at(-1), "Ended: the client has no counter clock_samples");
  assert.equal(a.core.log.clients.length, 0);
  const b = rig({ transport: "ws", coreOpts: { limits: [256, 65541, 1150, 1, 1, 2] }, link: { transport: "ws" } });
  void b.start();
  await b.c.advance(0);
  FakeWs.made[0].open();
  await b.c.advance(0);
  assert.equal(b.statuses().at(-1), "Ended: the client's limits are malformed");
});

test("a WebSocket that closes while the client still loads ends the session; no client is made", async () => {
  const r = rig({ link: { transport: "ws" }, coreDelayMs: 50 });
  void r.start();
  await r.c.advance(0);
  FakeWs.made[0].open();
  await r.c.advance(10);
  FakeWs.made[0].fail();
  await r.c.advance(100);
  assert.equal(r.statuses().at(-1), "Ended: the WebSocket closed");
  assert.equal(r.core.log.clients.length, 0);
  assert.deepEqual(r.events("attempt").map((x) => x.f.ok), [0]);
});

test("the start message is checked; the clock step is posted first", () => {
  assert.equal(W.readStart({ kind: "start", host: "h", link: { token: TOKEN }, wtTimeoutMs: 1500, mdsPeriodMs: null }) !== null, true);
  assert.equal(W.readStart({ kind: "start", host: "h", link: { token: TOKEN }, wtTimeoutMs: 0, mdsPeriodMs: 1 }), null);
  assert.equal(W.readStart({ kind: "go" }), null);
  assert.equal(W.readStart(null), null);
  const r = rig();
  assert.deepEqual(r.posts[0], { kind: "clock", resNs: 0 });
});
