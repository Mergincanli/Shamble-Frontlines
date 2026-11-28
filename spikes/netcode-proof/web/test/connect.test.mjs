// connect.test.mjs: the transport chooser (u7-brief.md rev 1.1 section 3; C4, C9) over a fake clock and fake
// transports: wt ready, wt failing, wt silent, ws, the no-fallback suites, the hash file first, the late-ready attempt.
import assert from "node:assert/strict";
import { test } from "node:test";
import { FakeClock, FakeWs, FakeWt, HASH_BYTES, connectDeps, flush } from "./fakes.mjs";
import { load, token } from "./helpers.mjs";

const L = await load("lib.js");
const link = (o = {}) => ({ suite: "smoke", room: "10", transport: "wt", attempt: 0, token: token(), wrongHash: false, bite: null, ...o });

/** Starts connect() and resolves `out.v` once it settles. */
function start(d) {
  const out = { v: undefined };
  L.connect(d).then((v) => (out.v = v));
  return out;
}

test("wt ready: WebTransport, with the pinned hash, no pooling, unreliable required, the room's port", async () => {
  const c = new FakeClock();
  const d = connectDeps(link(), c);
  const r = start(d);
  await flush();
  assert.deepEqual(d.fetched, ["/npt/cert-hash-10.json"]);
  assert.equal(FakeWt.made.length, 1);
  const wt = FakeWt.made[0];
  assert.equal(wt.url, "https://127.0.0.1:30011/npt");
  assert.equal(wt.options.allowPooling, false);
  assert.equal(wt.options.requireUnreliable, true);
  assert.equal(wt.options.serverCertificateHashes.length, 1);
  assert.equal(wt.options.serverCertificateHashes[0].algorithm, "sha-256");
  assert.deepEqual([...wt.options.serverCertificateHashes[0].value], [...HASH_BYTES]);
  await c.advance(200);
  wt.open();
  await flush();
  assert.equal(r.v.kind, "wt");
  assert.equal(r.v.wt, wt);
  assert.equal(c.timers.size, 0, "the 1.5 s timer is cleared");
  await c.advance(5000);
  assert.equal(FakeWs.made.length, 0);
  assert.equal(wt.closeCalls, 0);
});

test("wt failing: WebSocket at once, not after 1.5 s", async () => {
  const c = new FakeClock();
  const d = connectDeps(link(), c);
  const r = start(d);
  await c.advance(100);
  FakeWt.made[0].fail();
  await flush();
  assert.equal(FakeWs.made.length, 1);
  assert.equal(FakeWs.made[0].url, "wss://127.0.0.1:31011/");
  assert.equal(FakeWs.made[0].binaryType, "arraybuffer");
  FakeWs.made[0].open();
  await flush();
  assert.equal(r.v.kind, "ws");
  assert.equal(r.v.fellBack, true);
  assert.equal(c.t, 100);
});

test("wt silent: WebSocket at 1.5 s, not before (P9)", async () => {
  const c = new FakeClock();
  const d = connectDeps(link(), c);
  const r = start(d);
  await c.advance(1499);
  assert.equal(FakeWs.made.length, 0);
  await c.advance(1);
  assert.equal(FakeWs.made.length, 1);
  assert.equal(FakeWt.made[0].closeCalls, 1, "the silent attempt is closed");
  FakeWs.made[0].open();
  await flush();
  assert.equal(r.v.kind, "ws");
  assert.deepEqual(d.statuses, ["WebTransport · Connecting", "WebSocket · Connecting"]);
});

test("C9: a WebTransport ready at 1.6 s, after the fallback began, is closed again and never used", async () => {
  const c = new FakeClock();
  const d = connectDeps(link(), c);
  const r = start(d);
  await c.advance(1600);
  const wt = FakeWt.made[0];
  wt.open();
  await flush();
  assert.equal(wt.closeCalls, 2);
  assert.equal(wt.bidis.length, 0, "no control stream, so no Join, on the abandoned session");
  FakeWs.made[0].open();
  await flush();
  assert.equal(r.v.kind, "ws");
});

test("ws: never WebTransport, no hash file", async () => {
  const c = new FakeClock();
  const d = connectDeps(link({ transport: "ws" }), c);
  const r = start(d);
  await flush();
  FakeWs.made[0].open();
  await flush();
  assert.equal(r.v.kind, "ws");
  assert.equal(r.v.fellBack, false);
  assert.equal(FakeWt.made.length, 0);
  assert.deepEqual(d.fetched, []);
});

test("cert-14d and wt-attempts: no fallback and no timer; a failed session fails at once (P13)", async () => {
  for (const suite of ["cert-14d", "wt-attempts"]) {
    const c = new FakeClock();
    const d = connectDeps(link({ suite }), c);
    const r = start(d);
    await c.advance(5000);
    assert.equal(r.v, undefined, `${suite}: still waiting at 5 s`);
    assert.equal(FakeWs.made.length, 0);
    FakeWt.made[0].fail();
    await flush();
    assert.deepEqual(r.v, { kind: "failed", why: "the WebTransport session did not open" });
    await c.advance(5000);
    assert.equal(FakeWs.made.length, 0, `${suite}: never WebSocket`);
  }
});

test("hash=wrong pins each byte XOR 0xff (C4)", async () => {
  const c = new FakeClock();
  const d = connectDeps(link({ suite: "wt-attempts", wrongHash: true }), c);
  start(d);
  await flush();
  assert.deepEqual([...FakeWt.made[0].options.serverCertificateHashes[0].value], [...HASH_BYTES].map((b) => b ^ 0xff));
});

test("the hash file comes first: a fetch failure or a bad file connects nothing", async () => {
  for (const o of [{ fetchFails: true }, { hashText: "{}" }]) {
    const c = new FakeClock();
    const d = connectDeps(link(), c, o);
    const r = start(d);
    await c.advance(3000);
    assert.equal(r.v.kind, "failed");
    assert.equal(FakeWt.made.length + FakeWs.made.length, 0);
  }
});

test("a WebSocket that never opens fails; no timeout (null) waits on WebTransport for ever", async () => {
  const c = new FakeClock();
  const d = connectDeps(link({ transport: "ws" }), c);
  const r = start(d);
  await flush();
  FakeWs.made[0].fail();
  await flush();
  assert.deepEqual(r.v, { kind: "failed", why: "the WebSocket did not open" });
  const c2 = new FakeClock();
  const d2 = connectDeps(link(), c2, { timeoutMs: null });
  const r2 = start(d2);
  await c2.advance(30000);
  assert.equal(r2.v, undefined);
  assert.equal(FakeWs.made.length, 0);
});
