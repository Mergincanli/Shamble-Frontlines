// perf-auditor, cs2 review round 2: the Worker's tick path, bytes per tick before and after the fix pass, in Node's
// V8 (in-process sampling heap profiler, interval 32, collected objects included). ARM=a runs host.ts and friends as at
// 30a5a13 (.gate/perf-auditor-r2-src-a), ARM=b as at d257bb6 (.gate/perf-auditor-r2-src-b); both with the same built
// Wasm (the main folder's packages/sim-worker/pkg), one process per arm. The env stub allocates nothing itself; the
// "page" gives buffers back through a reused input message (arm a's message has viewTime, as its validation needs).
// Windows: the first 300 ticks (cold) and 20,000 ticks after 100,000 (hot); the input path alone the same way.
//   ARM=a node .gate/perf-auditor-r2-tick-alloc.mjs ; ARM=b node .gate/perf-auditor-r2-tick-alloc.mjs
import { readFileSync } from "node:fs";
import { Session } from "node:inspector/promises";
import path from "node:path";
import { pathToFileURL } from "node:url";

const ROOT = "/home/user/Shamble-Frontlines";
const ARM = process.env.ARM ?? "b";
const SRC = path.join(ROOT, `.gate/perf-auditor-r2-src-${ARM}`);
const glue = await import(pathToFileURL(path.join(ROOT, "packages/sim-worker/pkg/sim_wasm.js")).href);
const wasm = glue.initSync({ module: readFileSync(path.join(ROOT, "packages/sim-worker/pkg/sim_wasm_bg.wasm")) });
const { SimHost } = await import(pathToFileURL(path.join(SRC, "host.ts")).href);

let clock = 1000;
let timer = null;
const delays = [];
const held = [];
const env = {
  memory: wasm.memory,
  now: () => clock,
  timeOrigin: 1.7e12,
  post: (message) => { if (message.type === "block") held.push(message.buffer); },
  setTimer: (run, delay) => { timer = run; if (delays.length < 5) delays.push(delay); return 1; },
  clearTimer: () => {},
  hardwareConcurrency: 4,
  crossOriginIsolated: false,
};
const sim = new glue.Sim(new Uint8Array(32), 0, 0, clock);
const host = new SimHost(sim, env, glue.tick_hz());
const input = ARM === "a" ? { type: "input", sequence: 0, viewTime: 0, returned: [] } : { type: "input", sequence: 0, returned: [] };
const empty = ARM === "a" ? { type: "input", sequence: 0, viewTime: 0, returned: [] } : { type: "input", sequence: 0, returned: [] };
const PERIOD = 1000 / 30;
host.start();
function ticks(n) {
  for (let i = 0; i < n; i += 1) {
    clock += PERIOD + 0.01;
    timer();
    if (held.length > 2) {
      input.returned.length = 0;
      while (held.length > 2) input.returned.push(held.shift());
      host.onMessage(input);
    }
  }
}
function inputs(n) { for (let i = 0; i < n; i += 1) host.onMessage(empty); }
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
async function measure(label, n, run) {
  await session.post("HeapProfiler.startSampling", { samplingInterval: 32, includeObjectsCollectedByMajorGC: true, includeObjectsCollectedByMinorGC: true });
  run(n);
  const { profile } = await session.post("HeapProfiler.stopSampling");
  const top = [...selfBy(profile.head)].filter(([, v]) => v > 0).sort((a, b) => b[1] - a[1]).slice(0, 6);
  console.log(`arm ${ARM} ${label}: ${n}, ${(total(profile.head) / n).toFixed(1)} B each; top self (B each): ${top.map(([k, v]) => `${k} ${(v / n).toFixed(1)}`).join(", ")}`);
}
console.log(`arm ${ARM} (${SRC}); Node ${process.version}; first timer delays ${JSON.stringify(delays)}`);
await measure("cold ticks (each with one input returning one buffer)", 300, ticks);
ticks(100000);
await measure("hot ticks  (each with one input returning one buffer)", 20000, ticks);
await measure("cold input messages (no buffer)", 300, inputs);
inputs(100000);
await measure("hot input messages  (no buffer)", 20000, inputs);
const s = host.stats();
console.log(`arm ${ARM} stats: ticks ${s.ticks}, wakes ${s.wakes}, pool misses ${s.poolMisses}; delays seen ${JSON.stringify(delays)}`);
