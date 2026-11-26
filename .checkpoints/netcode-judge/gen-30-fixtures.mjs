// Part b, block 3 (C-U6-9): writes PM-0001-30's C-part fixtures, the made-up logs tests/fixtures.rs judges with the
// real marks.json. Run once from the repository root: node .checkpoints/netcode-judge/gen-30-fixtures.mjs
// base/: six measured room logs on the container (3 recordings x wt and ws), each naming its other-transport run in
// params.twin, 3 measured ticks with the same state hash on both transports and late_count 0. Each case folder holds
// only the file it changes (a case file replaces the base file of the same name: fixtures/README.md).
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const DIR = "spikes/netcode-proof/crates/judge/fixtures/pm-0001-30";
const other = (t) => (t === "wt" ? "ws" : "wt");
const id = (rec, t) => `c-rec${rec}-${t}`;

/** One log's lines. `o` overrides: run, marks, transport, twin, and `h2` (tick 2's hash). */
function log(rec, t, o = {}) {
  const header = {
    k: "header",
    schema: 1,
    run: o.run ?? id(rec, t),
    set: "c-both",
    role: "measured",
    marks: o.marks ?? ["PM-0001-30"],
    machine: {
      role: "C", name: "C", plan: "container", arch: "x86_64", cpu: "fixture", vcpus: 4, mem_mb: 16384,
      location: "fixture", kernel: "fixture", address: "", threads_per_core: 1, smt: [[0], [1], [2], [3]], clock_res_ns: 1,
    },
    build: { commit: "0".repeat(40), binary_sha256: "0".repeat(64), features: [], profile: "release" },
    bite: null,
    layer: "L2",
    profile: "P0",
    seed: 30 + rec,
    tick_hz: 30,
    clients: 48,
    scene: "SS",
    process: "room",
    slot: 0,
    date: "2026-11-24",
    params: { suite: "both-transports", recording: rec, transport: o.transport ?? t, twin: o.twin ?? id(rec, other(t)) },
  };
  const hash = (tick) => (tick === 2 && o.h2 ? o.h2 : `00000000000${rec}000${tick}`);
  const ticks = [1, 2, 3].map((tick) => ({ k: "tick", t: tick, ph: "measured", v: { work_ns: 9000000, late_count: 0 }, h: hash(tick) }));
  return [header, ...ticks, { k: "footer", records: 4, ended: "clean" }].map((r) => JSON.stringify(r)).join("\n") + "\n";
}

const files = { base: {} };
for (const rec of [1, 2, 3]) for (const t of ["wt", "ws"]) files.base[`${id(rec, t)}.jsonl`] = log(rec, t);
files["twin-differs"] = { "c-rec2-ws.jsonl": log(2, "ws", { h2: "00000000000bad02" }) };
files["same-transport"] = { "c-rec3-ws.jsonl": log(3, "ws", { transport: "wt" }) };
files["not-mutual"] = { "c-rec1-ws.jsonl": log(1, "ws", { twin: "c-rec1-zz" }) };
// The twin's file holds another run that serves no mark, so c-rec1-wt's twin is not in the set yet.
files["twin-missing"] = { "c-rec1-ws.jsonl": log(1, "ws", { run: "c-rec1-later", marks: [] }) };

let n = 0;
for (const [folder, logs] of Object.entries(files)) {
  mkdirSync(join(DIR, folder), { recursive: true });
  for (const [name, text] of Object.entries(logs)) {
    writeFileSync(join(DIR, folder, name), text);
    n++;
  }
}
console.log(`written: ${n} fixture logs under ${DIR}`);
