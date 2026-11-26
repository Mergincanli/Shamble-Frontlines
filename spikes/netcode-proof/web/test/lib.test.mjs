// lib.test.mjs: Block T2's pure logic (u7-brief.md rev 1.1 section 3 with C1, C4, C5): the link, the hash file, the
// framing, the WebSocket tag, the limits, the inbox queue, the records, the status texts.
import assert from "node:assert/strict";
import { test } from "node:test";
import { HASH_BYTES, hashFile } from "./fakes.mjs";
import { checkLine, load, token } from "./helpers.mjs";

const L = await load("lib.js");
const TOKEN = token();
const q = (o) => `?${Object.entries(o).map(([k, v]) => `${k}=${v}`).join("&")}`;
const base = { smoke: "1", suite: "smoke", room: "10", transport: "wt", attempt: "0" };

test("the link: U8's form parses, each field read", () => {
  const p = L.parseLink(q(base), `#t=${TOKEN}`);
  assert.equal(p.ok, true);
  assert.deepEqual(p.value, { suite: "smoke", room: "10", transport: "wt", attempt: 0, token: TOKEN, wrongHash: false, bite: null });
  for (const [suite, transport] of [["smoke", "ws"], ["cert-14d", "wt"], ["fallback", "wt"], ["mds", "wt"], ["wt-attempts", "wt"]]) {
    assert.equal(L.parseLink(q({ ...base, suite, transport, room: "00", attempt: "19" }), `#t=${TOKEN}`).ok, true, suite);
  }
  assert.equal(L.parseLink(q({ ...base, room: "31", attempt: "99" }), `#t=${TOKEN}`).value.room, "31");
});

test("the link: each bad field starts nothing, and no reason or status holds the token (P16)", () => {
  const bad = [
    [q({ ...base, smoke: "0" }), `#t=${TOKEN}`],
    [q({ ...base, suite: "load" }), `#t=${TOKEN}`],
    [q({ ...base, room: "32" }), `#t=${TOKEN}`],
    [q({ ...base, room: "7" }), `#t=${TOKEN}`],
    [q({ ...base, room: "007" }), `#t=${TOKEN}`],
    [q({ ...base, transport: "h3" }), `#t=${TOKEN}`],
    [q({ ...base, suite: "cert-14d", transport: "ws" }), `#t=${TOKEN}`],
    [q({ ...base, attempt: "01" }), `#t=${TOKEN}`],
    [q({ ...base, attempt: "100" }), `#t=${TOKEN}`],
    [q({ ...base, extra: "1" }), `#t=${TOKEN}`],
    [`${q(base)}&room=11`, `#t=${TOKEN}`],
    [q({ ...base, suite: "cert%2D14d" }), `#t=${TOKEN}`],
    [q(Object.fromEntries(Object.entries(base).filter(([k]) => k !== "attempt"))), `#t=${TOKEN}`],
    ["", `#t=${TOKEN}`],
    [q(base), ""],
    [q(base), `#x=${TOKEN}`],
    [q(base), `#t=${TOKEN.slice(5)}`],
    [q(base), `#t=${TOKEN.slice(0, -1)}`],
    [q(base), `#t=${TOKEN}A`],
    [q(base), `#t=${TOKEN.slice(0, -1)}!`],
    [q(base), `#t=${TOKEN}&t=${TOKEN}`],
    [q({ ...base, bite: "x" }), `#t=${TOKEN}`],
  ];
  for (const [search, hash] of bad) {
    const p = L.parseLink(search, hash);
    assert.equal(p.ok, false, `${search} ${hash.length}`);
    const status = L.STATUS.badLink(p.why);
    for (const s of [p.why, status]) {
      assert.equal(s.includes(TOKEN.slice(5, 15)), false);
      assert.equal(s.includes("nptj_"), false);
      assert.equal(s.includes("?smoke"), false);
    }
  }
});

test("the link: hash=wrong only with wt-attempts (C4, P19); bite only when the dist names it", () => {
  const w = L.parseLink(q({ ...base, suite: "wt-attempts", hash: "wrong" }), `#t=${TOKEN}`);
  assert.equal(w.ok, true);
  assert.equal(w.value.wrongHash, true);
  for (const suite of ["cert-14d", "smoke", "fallback", "mds"]) {
    assert.equal(L.parseLink(q({ ...base, suite, hash: "wrong" }), `#t=${TOKEN}`).ok, false, suite);
  }
  assert.equal(L.parseLink(q({ ...base, suite: "wt-attempts", hash: "right" }), `#t=${TOKEN}`).ok, false);
  const ids = ["one/id", "two/id"];
  assert.equal(L.parseLink(q({ ...base, bite: "one/id" }), `#t=${TOKEN}`).ok, false);
  assert.equal(L.parseLink(q({ ...base, bite: "one/id" }), `#t=${TOKEN}`, ids).value.bite, "one/id");
  assert.equal(L.parseLink(q({ ...base, bite: "three/id" }), `#t=${TOKEN}`, ids).ok, false);
});

test("the hash file: U6b's form gives its 32 bytes in order (P8); anything else is refused", () => {
  const good = L.parseHashFile(hashFile());
  assert.equal(good.ok, true);
  assert.deepEqual([...good.value], [...HASH_BYTES]);
  const hex = Buffer.from(HASH_BYTES).toString("hex");
  const bad = [
    JSON.stringify({ algorithm: "sha-384", value_hex: hex, validity_ns: 1 }),
    JSON.stringify({ algorithm: "sha-256", value_hex: hex.toUpperCase(), validity_ns: 1 }),
    JSON.stringify({ algorithm: "sha-256", value_hex: hex.slice(2), validity_ns: 1 }),
    JSON.stringify({ algorithm: "sha-256", value_hex: `${hex}00`, validity_ns: 1 }),
    JSON.stringify({ algorithm: "sha-256", value_hex: hex }),
    JSON.stringify({ algorithm: "sha-256", value_hex: hex, validity_ns: -1 }),
    JSON.stringify({ algorithm: "sha-256", value_hex: hex, validity_ns: 1, next: hex }),
    JSON.stringify([hex]),
    "not json",
    hashFile().padEnd(1025, " "),
  ];
  for (const t of bad) assert.equal(L.parseHashFile(t).ok, false, t.slice(0, 40));
  assert.deepEqual([...L.wrongHash(HASH_BYTES)], [...HASH_BYTES].map((b) => b ^ 0xff));
  assert.equal(L.hashPath("07"), "/npt/cert-hash-07.json");
});

test("the addresses: the page's own host, UDP 30001 + NN and TCP 31001 + NN", () => {
  assert.equal(L.wtUrl("rs.example", "05"), "https://rs.example:30006/npt");
  assert.equal(L.wsUrl("rs.example", "31"), "wss://rs.example:31032/");
});

const LIM = { control: 256, fullState: 65541, datagram: 1150, tags: [7, 8, 9] };
const control = (body) => Uint8Array.from([body.length & 0xff, body.length >> 8, ...body]);
const full = (body) => {
  const h = Buffer.alloc(4);
  h.writeUInt32LE(body.length);
  return Uint8Array.from([...h, ...body]);
};

function splits(framer, bytes, cut) {
  return [...framer.push(bytes.subarray(0, cut)), ...framer.push(bytes.subarray(cut))];
}

test("framing: a control, a full state and a two-message chunk, split at every byte offset (P10)", () => {
  const c = control([8, 1, 2, 3, 4, 5, 6, 7]);
  const f = full(Array.from({ length: 300 }, (_, i) => i & 0xff));
  const two = Uint8Array.from([...control([8, 9]), ...control([11, 12, 13])]);
  for (let cut = 0; cut <= c.length; cut++) assert.deepEqual(splits(L.controlFramer(LIM), c, cut), [c], `control ${cut}`);
  for (let cut = 0; cut <= f.length; cut++) assert.deepEqual(splits(L.fullStateFramer(LIM), f, cut), [f], `full ${cut}`);
  for (let cut = 0; cut <= two.length; cut++) {
    assert.deepEqual(splits(L.controlFramer(LIM), two, cut), [two.subarray(0, 4), two.subarray(4)], `two ${cut}`);
  }
  const fr = L.controlFramer(LIM);
  const parts = [...c].flatMap((b) => fr.push(Uint8Array.of(b)));
  assert.deepEqual(parts, [c]);
  assert.equal(fr.partial, false);
  fr.push(c.subarray(0, 3));
  assert.equal(fr.partial, true);
});

test("framing: a message over its cap closes, counting its length bytes (P11)", () => {
  const atC = control(Array(254).fill(1));
  assert.equal(L.controlFramer(LIM).push(atC).length, 1);
  assert.throws(() => L.controlFramer(LIM).push(control(Array(255).fill(1))), L.FrameError);
  const head = (n) => {
    const h = Buffer.alloc(4);
    h.writeUInt32LE(n);
    return Uint8Array.from(h);
  };
  assert.deepEqual(L.fullStateFramer(LIM).push(head(65537)), []);
  assert.throws(() => L.fullStateFramer(LIM).push(head(65538)), L.FrameError);
  const atF = full(Array(65537).fill(2));
  assert.equal(atF.length, 65541);
  assert.deepEqual(L.fullStateFramer(LIM).push(atF), [atF]);
});

test("the WebSocket tag both ways, from the limits' table; unknown tags and over-cap messages", () => {
  const body = Uint8Array.of(8, 1, 2);
  for (const ch of [0, 1, 2]) {
    const w = L.wsWrap(LIM.tags[ch], body);
    assert.deepEqual([...w], [LIM.tags[ch], 8, 1, 2]);
    const u = L.wsUnwrap(w, LIM);
    assert.equal(u.ok, true);
    assert.equal(u.channel, ch);
    assert.deepEqual([...u.bytes], [...body]);
  }
  assert.deepEqual(L.wsUnwrap(Uint8Array.of(0, 1), LIM), { ok: false, close: true, why: "an unknown WebSocket tag" });
  assert.equal(L.wsUnwrap(new Uint8Array(0), LIM).close, true);
  assert.equal(L.wsUnwrap(L.wsWrap(7, new Uint8Array(1150)), LIM).ok, true);
  assert.deepEqual(L.wsUnwrap(L.wsWrap(7, new Uint8Array(1151)), LIM), { ok: false, close: false, why: "a message over its cap" });
  assert.equal(L.wsUnwrap(L.wsWrap(8, new Uint8Array(257)), LIM).close, true);
  assert.equal(L.wsUnwrap(L.wsWrap(9, new Uint8Array(65542)), LIM).close, true);
});

test("the limits: six numbers in C5's order; anything else is refused", () => {
  assert.deepEqual(L.readLimits([256, 65541, 1150, 7, 8, 9]), { ok: true, value: LIM });
  for (const v of [[256, 65541, 1150, 7, 8], [256, 65541, 1150, 7, 8, 9, 10], [256, 65541, 1150, 7, 7, 9],
    [256, 65541, 1150, 7, 8, 256], [2, 65541, 1150, 7, 8, 9], [256, 4, 1150, 7, 8, 9], [256, 65541, 0, 7, 8, 9]]) {
    assert.equal(L.readLimits(v).ok, false, v.join(","));
  }
});

test("the inbox queue holds at most 512, counting the rest", () => {
  const qq = new L.InboxQueue();
  for (let i = 0; i < 520; i++) qq.push({ channel: 0, arrivalNs: BigInt(i), bytes: Uint8Array.of(i & 0xff) });
  assert.equal(qq.length, 512);
  assert.equal(qq.dropped, 8);
  assert.equal(qq.drain().length, 512);
  assert.equal(qq.length, 0);
});

test("the records: attempt once, fallback once from the connect start, mds reads, one second record a second", () => {
  const lines = [];
  const r = new L.Recorder((l) => lines.push(l), 2e8);
  r.attempt(1.5e9, 1);
  r.attempt(1.6e9, 0);
  r.fallback(2.9e9);
  r.fallback(3e9);
  r.mdsRead(3.1e9, 1024);
  r.datagram(1100, 1024);
  r.datagram(1000, 1024);
  r.datagram(1200, 1024);
  r.second(1.2e9);
  r.second(1.9e9);
  r.datagram(1300, 1024);
  r.second(3.05e9);
  assert.deepEqual(lines, [
    '{"k":"event","ev":"attempt","ph":"measured","ns":1500000000,"f":{"ok":1}}',
    '{"k":"event","ev":"fallback","ph":"measured","ns":2900000000,"f":{"playing_ns":2700000000}}',
    '{"k":"event","ev":"mds_read","ph":"measured","ns":3100000000,"f":{"size_bytes":1024}}',
    '{"k":"second","sec":0,"ph":"measured","v":{"recv_over_max_count":2}}',
    '{"k":"second","sec":2,"ph":"measured","v":{"recv_over_max_count":1}}',
  ]);
  assert.equal(r.attemptWritten, true);
  for (const l of lines) checkLine(l, TOKEN);
});

test("the status texts, byte for byte (U+00B7 as U8's PAGE reads it)", () => {
  assert.equal(Buffer.from(L.STATUS.playing("wt")).toString("hex"), Buffer.from("WebTransport · Playing").toString("hex"));
  assert.equal(L.STATUS.playing("wt"), "WebTransport · Playing");
  assert.equal(L.STATUS.playing("ws"), "WebSocket · Playing");
  assert.equal(L.STATUS.connecting("wt"), "WebTransport · Connecting");
  assert.equal(L.STATUS.joined("ws"), "WebSocket · Joined");
  assert.equal(L.STATUS.ended("x"), "Ended: x");
  assert.equal(L.STATUS.badLink("x"), "Bad link: x");
  assert.equal(L.STATUS.starting, "Starting");
});

test("the clock step, the counters by name, the version text", () => {
  const seq = [0, 0, 0, 0.1, 0.1, 0.3, 0.3, 0.35];
  let i = 0;
  assert.equal(L.clockResNs(() => seq[Math.min(i++, seq.length - 1)], seq.length), 50000);
  assert.equal(L.clockResNs(() => 5, 10), 0);
  assert.deepEqual(L.counterIndex(["x", "own_soldier", "joined", "clock_samples"]),
    { ok: true, value: { joined: 2, clock_samples: 3, own_soldier: 1 } });
  assert.equal(L.counterIndex(["joined", "clock_samples"]).ok, false);
  assert.equal(L.versionText({ userAgent: "UA", userAgentData: { brands: [{ brand: "Not)A;Brand", version: "8" },
    { brand: "Chromium", version: "143" }] } }), "Chromium 143");
  assert.equal(L.versionText({ userAgent: "Mozilla/5.0 UA" }), "Mozilla/5.0 UA");
});

test("countCsp counts securitypolicyviolation events at its target, and nothing else (U7R-NC-7)", () => {
  const t = new EventTarget();
  let n = 0;
  L.countCsp(t, () => n++);
  t.dispatchEvent(new Event("securitypolicyviolation"));
  t.dispatchEvent(new Event("error"));
  t.dispatchEvent(new Event("securitypolicyviolation"));
  assert.equal(n, 2);
});

test("forgetFragment replaces the address with its path and query: the token's fragment leaves history (U7R-NC-5)", () => {
  const calls = [];
  L.forgetFragment({ pathname: "/", search: "?smoke=1&room=10", hash: `#t=${TOKEN}` }, { replaceState: (...a) => calls.push(a) });
  assert.deepEqual(calls, [[null, "", "/?smoke=1&room=10"]]);
});
