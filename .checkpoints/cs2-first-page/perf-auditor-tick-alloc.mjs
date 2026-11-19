// perf-auditor, cs2 review 1: bytes allocated per tick on the Worker's tick path, with the real host.ts, pacing.ts,
// block-pool.ts, read-block.ts and the built Wasm (worktree 30a5a13), in Node's V8 (in-process sampling heap profiler,
// samplingInterval 32, collected objects included). The env stub allocates nothing itself: post() keeps the buffer and
// the "page" gives it back through a reused input message. Two windows: the first 300 ticks (cold) and 20,000 ticks
// after 100,000 (hot). Also: the input message path (isPageToWorker + give) per page frame.
//   node .gate/perf-auditor-tick-alloc.mjs
import { readFileSync } from "node:fs";
import { Session } from "node:inspector/promises";
import path from "node:path";
import { pathToFileURL } from "node:url";

const WT = "/home/user/Shamble-Frontlines/.gate/cs2-plants";
const glue = await import(pathToFileURL(path.join(WT, "packages/sim-worker/pkg/sim_wasm.js")).href);
const wasm = glue.initSync({ module: readFileSync(path.join(WT, "packages/sim-worker/pkg/sim_wasm_bg.wasm")) });
const { SimHost } = await import(pathToFileURL(path.join(WT, "packages/sim-worker/src/host.ts")).href);

let clock = 1000;
let timer = null;
const held = [];
const env = {
  memory: wasm.memory,
  now: () => clock,
  timeOrigin: 1.7e12,
  post: (message, transfer) => { if (message.type === "block") held.push(message.buffer); },
  setTimer: (run, delay) => { timer = run; return 1; },
  clearTimer: () => {},
  hardwareConcurrency: 4,
  crossOriginIsolated: false,
};
const sim = new glue.Sim(new Uint8Array(32), 0, 0, clock);
const host = new SimHost(sim, env, glue.tick_hz());
const input = { type: "input", sequence: 0, viewTime: 0, returned: [] };
const PERIOD = 1000 / 30;
host.start();
function ticks(n, withInput) {
  for (let i = 0; i < n; i += 1) {
    clock += PERIOD + 0.01;
    timer();
    if (held.length > 2) {
      // The page gives back what it no longer keeps, on its next frame's input post.
      input.returned.length = 0;
      while (held.length > 2) input.returned.push(held.shift());
      host.onMessage(input);
    }
  }
}
const session = new Session();
session.connect();
await session.post("HeapProfiler.enable");
const total = (n) => n.selfSize + (n.children ?? []).reduce((s, c) => s + total(c), 0);
const selfBy = (n, map = new Map()) => {
  const key = n.callFrame.functionName || "(anonymous)";
  map.set(key, (map.get(key) ?? 0) + n.selfSize);
  for (const c of n.children ?? []) selfBy(c, map);
  return map;
};
const empty = { type: "input", sequence: 0, viewTime: 0, returned: [] };
function inputs(n) { for (let i = 0; i < n; i += 1) host.onMessage(empty); }
async function measureInputs(label, n) {
  await session.post("HeapProfiler.startSampling", { samplingInterval: 32, includeObjectsCollectedByMajorGC: true, includeObjectsCollectedByMinorGC: true });
  inputs(n);
  const { profile } = await session.post("HeapProfiler.stopSampling");
  const top = [...selfBy(profile.head)].filter(([, v]) => v > 0).sort((a, b) => b[1] - a[1]).slice(0, 7);
  console.log(`${label}: ${n} input messages (no buffer returned), ${(total(profile.head) / n).toFixed(1)} B per message; by function (self, B per message): ${top.map(([k, v]) => `${k} ${(v / n).toFixed(1)}`).join(", ")}`);
}
async function measure(label, n, withInput) {
  await session.post("HeapProfiler.startSampling", { samplingInterval: 32, includeObjectsCollectedByMajorGC: true, includeObjectsCollectedByMinorGC: true });
  ticks(n, withInput);
  const { profile } = await session.post("HeapProfiler.stopSampling");
  const top = [...selfBy(profile.head)].filter(([, v]) => v > 0).sort((a, b) => b[1] - a[1]).slice(0, 7);
  console.log(`${label}: ${n} ticks, ${(total(profile.head) / n).toFixed(1)} B per tick in all; by function (self, B per tick): ${top.map(([k, v]) => `${k} ${(v / n).toFixed(1)}`).join(", ")}`);
}
await measure("cold, ticks each followed by one input message returning one buffer", 300, true);
ticks(100000, true);
await measure("hot,  ticks each followed by one input message returning one buffer", 20000, true);
await measureInputs("cold input path", 300);
inputs(100000);
await measureInputs("hot input path", 20000);
const s = host.stats();
console.log(`host stats: ticks ${s.ticks}, wakes ${s.wakes}, pool misses ${s.poolMisses}, refused ${s.poolRefused}, undetached ${s.undetached} (no transfer in this stub)`);
