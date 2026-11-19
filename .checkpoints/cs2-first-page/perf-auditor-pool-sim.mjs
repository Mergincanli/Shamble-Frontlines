// perf-auditor, cs2 review 1: pool misses as a function of the page's frame rate, with the real BlockPool (worktree
// 30a5a13, size 4) and the page's real keep-two / return-on-next-frame rule (worker-link.ts). A discrete-event model:
// ticks every 1000/30 ms; the page's main thread is busy for `busy` of each frame interval, a block that arrives while
// it is busy is taken at the end of the busy part; returned buffers ride the next frame's input post and reach the
// pool 0.2 ms later. Prints misses per 600 ticks.   node .gate/perf-auditor-pool-sim.mjs
import path from "node:path";
import { pathToFileURL } from "node:url";

const WT = "/home/user/Shamble-Frontlines/.gate/cs2-plants";
const { BlockPool } = await import(pathToFileURL(path.join(WT, "packages/sim-worker/src/block-pool.ts")).href);

function run({ fps, busy, hitchEveryMs = 0, hitchMs = 0, size = 4, ticks = 6000 }) {
  const pool = new BlockPool(8, size);
  const tickMs = 1000 / 30;
  const frameMs = 1000 / fps;
  const events = [];
  const at = (t, kind, data) => events.push({ t, kind, data });
  for (let k = 1; k <= ticks; k += 1) at(k * tickMs, "tick");
  // Frames: start times; a hitch stretches one frame.
  let t = 0, next = hitchEveryMs || Infinity;
  const frames = [];
  while (t < ticks * tickMs + 1000) {
    let len = frameMs;
    if (t >= next) { len = hitchMs; next += hitchEveryMs; }
    frames.push({ start: t, busyEnd: t + len * busy, len });
    at(t, "frame");
    t += len;
  }
  let fi = 0;
  const busyEndAt = (time) => {
    while (fi + 1 < frames.length && frames[fi + 1].start <= time) fi += 1;
    const f = frames[fi];
    return time < f.busyEnd ? f.busyEnd : time;
  };
  let newest = null, previous = null, returned = [];
  events.sort((a, b) => a.t - b.t || (a.kind === "give" ? -1 : 1));
  for (let i = 0; i < events.length; i += 1) {
    const e = events[i];
    if (e.kind === "tick") {
      const buffer = pool.take();
      const arrive = busyEndAt(e.t + 0.2);
      insert({ t: arrive, kind: "block", data: buffer });
    } else if (e.kind === "block") {
      if (previous !== null) returned.push(previous);
      previous = newest; newest = e.data;
    } else if (e.kind === "frame") {
      if (returned.length > 0) { insert({ t: e.t + 0.2, kind: "give", data: returned }); returned = []; }
    } else if (e.kind === "give") {
      for (const b of e.data) pool.give(b);
    }
  }
  function insert(ev) {
    let j = events.length;
    events.push(ev);
    while (j > 0 && events[j - 1].t > ev.t) { events[j] = events[j - 1]; j -= 1; }
    events[j] = ev;
  }
  return (pool.misses / ticks) * 600;
}
for (const busy of [0.95, 0.3]) {
  const row = [5.5, 8.6, 12, 15, 20, 30, 60, 144].map((fps) => `${fps} fps ${run({ fps, busy }).toFixed(1)}`);
  console.log(`busy ${busy}: misses per 600 ticks: ${row.join(" | ")}`);
}
console.log(`30 fps, busy 0.95, one 250 ms hitch every 5 s: ${run({ fps: 30, busy: 0.95, hitchEveryMs: 5000, hitchMs: 250 }).toFixed(1)} per 600 ticks; with a pool of 8: ${run({ fps: 30, busy: 0.95, hitchEveryMs: 5000, hitchMs: 250, size: 8 }).toFixed(1)}`);
console.log(`60 fps, busy 0.95, one 250 ms hitch every 5 s: ${run({ fps: 60, busy: 0.95, hitchEveryMs: 5000, hitchMs: 250 }).toFixed(1)} per 600 ticks; with a pool of 8: ${run({ fps: 60, busy: 0.95, hitchEveryMs: 5000, hitchMs: 250, size: 8 }).toFixed(1)}`);
console.log(`8.6 fps, busy 0.95, pool of 8: ${run({ fps: 8.6, busy: 0.95, size: 8 }).toFixed(1)} per 600 ticks`);
