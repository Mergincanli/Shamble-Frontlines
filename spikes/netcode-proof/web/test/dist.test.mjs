// dist.test.mjs: Block T4 (u7-brief.md rev 1.1 sections 1 and 3; C11, C13): the measured dist from tsc's output holds
// no bite id and no innerHTML (P14, P15); its hooks.js holds only the two timings; the bites dist arms each bite by
// its query, and each armed bite does what marks.json says (read once; no WebTransport timeout).
import assert from "node:assert/strict";
import { mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { test } from "node:test";
import { pathToFileURL } from "node:url";
import { DIST_NAMES, assemble } from "../build.mjs";
import { makeCore } from "./fake-core.mjs";
import { FakeClock, FakeWs, FakeWt, connectDeps } from "./fakes.mjs";
import { OUT, WEB, load, scratch, token } from "./helpers.mjs";

const L = await load("lib.js");
const W = await load("worker.js");
const OURS = ["index.html", "main.js", "worker.js", "lib.js", "hooks.js"];

/** A dist from tsc's output and stand-ins for wasm-bindgen's two files (the real ones come with R3). */
function dist(tag, hooks = null) {
  const dir = scratch(tag);
  const bindgenDir = join(dir, "bindgen");
  mkdirSync(bindgenDir);
  writeFileSync(join(bindgenDir, "npt_client_wasm.js"), "export default async function init() {}\n");
  writeFileSync(join(bindgenDir, "npt_client_wasm_bg.wasm"), Buffer.from([0, 0x61, 0x73, 0x6d, 1, 0, 0, 0]));
  const out = join(dir, "dist");
  assemble({ out, index: join(WEB, "index.html"), jsDir: OUT, bindgenDir, hooks });
  return out;
}

test("the measured dist: the seven names, no bite id, no innerHTML, no eval or Function, no Date.now (P14, P15)", () => {
  const out = dist("measured");
  assert.deepEqual(readdirSync(out).sort(), [...DIST_NAMES].sort());
  for (const n of DIST_NAMES) {
    const text = readFileSync(join(out, n), "latin1");
    assert.equal(text.includes("PM-0001-"), false, `${n} holds a bite id`);
    assert.equal(text.includes("innerHTML"), false, `${n} uses innerHTML`);
  }
  for (const n of OURS) {
    const text = readFileSync(join(out, n), "utf8");
    assert.doesNotMatch(text, /\beval\s*\(|\bnew Function\b|\bFunction\s*\(|Date\.now|outerHTML|insertAdjacentHTML|document\.write/, n);
  }
});

test("the measured hooks.js holds only the 1,500 ms timeout and the 10,000 ms read period", async () => {
  const h = await load("hooks.js");
  assert.deepEqual(Object.keys(h).sort(), ["MDS_READ_PERIOD_MS", "WT_TIMEOUT_MS"]);
  assert.equal(h.WT_TIMEOUT_MS, 1500);
  assert.equal(h.MDS_READ_PERIOD_MS, 10000);
});

let n = 0;
/** The bites dist's hooks.js, evaluated afresh under a page whose query is `search`. */
async function bitesHooks(out, search) {
  globalThis.location = { search };
  try {
    return await import(`${pathToFileURL(join(out, "hooks.js")).href}?case=${++n}`);
  } finally {
    delete globalThis.location;
  }
}

test("the bites dist (C13): its hooks.js arms each bite by the link's query, and the page takes only its ids", async () => {
  const out = dist("bites", join(WEB, "bites", "hooks.js"));
  assert.deepEqual(readdirSync(out).sort(), [...DIST_NAMES].sort());
  const link = "?smoke=1&suite=mds&room=19&transport=wt&attempt=0";
  const none = await bitesHooks(out, link);
  const once = await bitesHooks(out, `${link}&bite=PM-0001-36/read-once`);
  const hang = await bitesHooks(out, `${link}&bite=PM-0001-29/no-wt-timeout`);
  assert.deepEqual([none.WT_TIMEOUT_MS, none.MDS_READ_PERIOD_MS], [1500, 10000]);
  assert.deepEqual([once.WT_TIMEOUT_MS, once.MDS_READ_PERIOD_MS], [1500, null]);
  assert.deepEqual([hang.WT_TIMEOUT_MS, hang.MDS_READ_PERIOD_MS], [null, 10000]);
  const ids = once.BITE_IDS;
  const frag = `#t=${token()}`;
  assert.equal(L.parseLink(`${link}&bite=PM-0001-36/read-once`, frag, ids).value.bite, "PM-0001-36/read-once");
  assert.equal(L.parseLink(`${link}&bite=PM-0001-36/read-once`, frag).ok, false, "the measured page refuses a bite");
  assert.equal(L.parseLink(`${link}&bite=PM-0001-99/other`, frag, ids).ok, false);
});

function engine(link, hooks) {
  const c = new FakeClock();
  const cd = connectDeps(link, c);
  const posts = [];
  const e = new W.Engine({
    now: c.now, setTimer: c.setTimer, clearTimer: c.clearTimer, fetchText: cd.fetchText, webTransport: cd.webTransport,
    webSocket: cd.webSocket, randomBytes: (k) => new Uint8Array(k), loadCore: async () => makeCore(), post: (m) => posts.push(m),
  });
  void e.start({ kind: "start", link, host: "127.0.0.1", wtTimeoutMs: hooks.WT_TIMEOUT_MS, mdsPeriodMs: hooks.MDS_READ_PERIOD_MS });
  const events = (ev) => posts.filter((p) => p.kind === "records").flatMap((p) => p.lines).map((l) => JSON.parse(l))
    .filter((x) => x.ev === ev);
  return { c, events };
}

test("the bites armed: read-once gives 1 read in 12 s; no-wt-timeout gives no fallback event in 10 s", async () => {
  const out = dist("bites-run", join(WEB, "bites", "hooks.js"));
  const base = { room: "19", transport: "wt", attempt: 0, token: token(), wrongHash: false };
  const once = engine({ ...base, suite: "mds", bite: "PM-0001-36/read-once" },
    await bitesHooks(out, "?bite=PM-0001-36/read-once"));
  await once.c.advance(100);
  FakeWt.made[0].open();
  await once.c.advance(12000);
  assert.equal(once.events("mds_read").length, 1);
  const hang = engine({ ...base, suite: "fallback", bite: "PM-0001-29/no-wt-timeout" },
    await bitesHooks(out, "?bite=PM-0001-29/no-wt-timeout"));
  await hang.c.advance(10000);
  assert.equal(FakeWs.made.length, 0);
  assert.equal(hang.events("fallback").length, 0);
  assert.deepEqual(hang.events("attempt").map((x) => x.f.ok), [0], "the hung attempt reads 0 at its deadline");
});
