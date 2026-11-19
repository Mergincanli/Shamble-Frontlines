// perf-auditor, cs2 review 1: the cost of Ring.percentile (stats.ts at 30a5a13: copy the held samples, sort the copy)
// at the ring sizes in use (600) and at a full-run size (18,000), with realistic frame-time and lateness samples.
// Node's V8; 7 batches after a warm-up; median, min, max per call.   node .gate/perf-auditor-ring.mjs
import path from "node:path";
import { pathToFileURL } from "node:url";

const WT = "/home/user/Shamble-Frontlines/.gate/cs2-plants";
const { Ring } = await import(pathToFileURL(path.join(WT, "packages/sim-worker/src/stats.ts")).href);
let seed = 12345;
const rand = () => ((seed = (seed * 1103515245 + 12345) >>> 0) / 2 ** 32);
for (const [capacity, calls] of [[600, 20000], [18000, 400]]) {
  const ring = new Ring(capacity);
  for (let i = 0; i < capacity; i += 1) ring.push(16.6 + (rand() - 0.5) * 2 + (rand() < 0.01 ? 20 : 0));
  let sink = 0;
  for (let i = 0; i < calls / 4; i += 1) { ring.push(16.6 + rand()); sink += ring.percentile(99); }
  const runs = [];
  for (let r = 0; r < 7; r += 1) {
    const t0 = process.hrtime.bigint();
    for (let i = 0; i < calls; i += 1) { ring.push(16.6 + rand()); sink += ring.percentile(i % 2 ? 50 : 99); }
    runs.push(Number(process.hrtime.bigint() - t0) / calls / 1000);
  }
  runs.sort((a, b) => a - b);
  console.log(`Ring(${capacity}).percentile: median ${runs[3].toFixed(2)} us per call (min ${runs[0].toFixed(2)}, max ${runs[6].toFixed(2)}; 7 batches of ${calls}); sink ${sink > 0}`);
}
// What the readout costs per update in sorts: 5 on the page (fps, frame p50 and p99, delivery p50 and p95) and 4 in the
// Worker (lateness p50 and p99, copy p50 and p99), each over 600 samples, 4 times a second.
