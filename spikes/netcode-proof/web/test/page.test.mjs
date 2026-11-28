// page.test.mjs: Block T3's page state (C1): the record cap and its drop count, the clock step, the counters, the
// start message with the measured hooks' timings.
import assert from "node:assert/strict";
import { test } from "node:test";
import { load, token } from "./helpers.mjs";

const M = await load("main.js");
const L = await load("lib.js");

test("the page keeps at most 4,096 records and counts the rest", () => {
  const s = new M.PageState();
  s.apply({ kind: "records", lines: Array.from({ length: 4000 }, (_, i) => `a${i}`) });
  s.apply({ kind: "records", lines: Array.from({ length: 100 }, (_, i) => `b${i}`) });
  assert.equal(s.records.length, L.RECORDS_MAX);
  assert.equal(s.records.length, 4096);
  assert.equal(s.dropped, 4);
  assert.equal(s.records.at(-1), "b95");
});

test("status, clock step and counters from the Worker", () => {
  const s = new M.PageState();
  assert.equal(s.status, "Starting");
  s.apply({ kind: "status", text: "WebSocket · Playing" });
  s.apply({ kind: "clock", resNs: 100000 });
  s.apply({ kind: "counters", names: ["joined", "steps"], values: [1, 42] });
  assert.equal(s.status, "WebSocket · Playing");
  assert.equal(s.clockResNs, 100000);
  assert.deepEqual(s.counters, [["joined", 1], ["steps", 42]]);
});

test("the drawn page_csp_violations is the Worker's count plus the page's own (U7R-NC-7)", () => {
  const s = new M.PageState();
  s.csp = 2;
  assert.deepEqual(s.rows(), [], "no row before the Worker's first counters");
  s.apply({ kind: "counters", names: ["joined", "page_csp_violations"], values: [1, 3] });
  assert.deepEqual(s.rows(), [["joined", 1], ["page_csp_violations", 5]]);
});

test("the start message carries the link and the measured hooks' timings; the measured dist arms no bite", () => {
  const link = { suite: "smoke", room: "10", transport: "wt", attempt: 0, token: token(), wrongHash: false, bite: null };
  assert.deepEqual(M.startMessage(link, "127.0.0.1"),
    { kind: "start", link, host: "127.0.0.1", wtTimeoutMs: 1500, mdsPeriodMs: 10000 });
  assert.deepEqual(M.BITE_IDS, []);
});
