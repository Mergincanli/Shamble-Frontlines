// boundary.test.mjs: C5's golden file against the page's packing (lib.js) and the fake core's own (fake-core.mjs).
// R3's Rust test reads the same file with include_str! against the real packing.
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { test } from "node:test";
import { fakePackOutbox, fakeUnpackInbox } from "./fake-core.mjs";
import { WEB, load } from "./helpers.mjs";

const L = await load("lib.js");
const G = JSON.parse(readFileSync(join(WEB, "test", "fixtures", "boundary.json"), "utf8"));
const hex = (b) => Buffer.from(b).toString("hex");
const bytes = (h) => Uint8Array.from(Buffer.from(h, "hex"));
const inbox = G.inbox.records.map((r) => ({ channel: r.channel, arrivalNs: BigInt(r.arrival_ns), bytes: bytes(r.bytes_hex) }));
const outbox = G.outbox.records.map((r) => ({ channel: r.channel, bytes: bytes(r.bytes_hex) }));

test("the golden file: one inbox record per channel, an arrival above 2^32, two outbox records", () => {
  assert.deepEqual(G.inbox.records.map((r) => r.channel), [0, 1, 2]);
  assert.ok(inbox.some((r) => r.arrivalNs > 0xffffffffn));
  assert.deepEqual(G.outbox.records.map((r) => r.channel), [0, 1]);
});

test("the page packs the golden inbox and reads the golden outbox", () => {
  assert.equal(hex(L.packInbox(inbox)), G.inbox.hex);
  assert.deepEqual(L.unpackOutbox(bytes(G.outbox.hex)).map((r) => ({ channel: r.channel, bytes_hex: hex(r.bytes) })),
    G.outbox.records);
  assert.equal(L.packInbox([]).length, 0);
  assert.deepEqual(L.unpackOutbox(new Uint8Array(0)), []);
});

test("the fake core reads the golden inbox and packs the golden outbox", () => {
  const u = fakeUnpackInbox(bytes(G.inbox.hex));
  assert.equal(u.refused, 0);
  assert.deepEqual(u.records.map((r) => ({ channel: r.channel, arrival_ns: String(r.arrivalNs), bytes_hex: hex(r.bytes) })),
    G.inbox.records);
  assert.equal(hex(fakePackOutbox(outbox)), G.outbox.hex);
});

test("a malformed outbox is a core fault: cut header, cut body, unknown channel", () => {
  const o = bytes(G.outbox.hex);
  assert.throws(() => L.unpackOutbox(o.subarray(0, 3)), /record header/);
  assert.throws(() => L.unpackOutbox(o.subarray(0, o.length - 1)), /inside a record/);
  const bad = Uint8Array.from(o);
  bad[0] = 2;
  assert.throws(() => L.unpackOutbox(bad), /unknown channel/);
});

test("the fake core refuses a truncated inbox record and the rest after it", () => {
  const b = bytes(G.inbox.hex);
  const u = fakeUnpackInbox(b.subarray(0, b.length - 1));
  assert.equal(u.refused, 1);
  assert.equal(u.records.length, 2);
});
