// The infra seat's U4d2 round-2 sampler (FX-7, live): every 500 ms, which processes hold an fd on the lane lock, and
// which of the step's child processes (cargo, rustc, npt-lab, npt-lab-bites, node) run without it. Read-only /proc
// reads; it writes no file and prints its summary to stdout when it gets SIGTERM.
//   node u4d2-r2-review-infra-holders.mjs <lock path>
import { readdirSync, readFileSync, readlinkSync } from "node:fs";

const lock = process.argv[2];
const holding = new Map();
const without = new Map();
let samples = 0;
const bump = (m, k) => m.set(k, (m.get(k) ?? 0) + 1);
const watched = /^(cargo|rustc|npt-lab|npt-lab-bites|node|bash|sh|find|sleep|touch)$/;

function sample() {
  samples++;
  for (const p of readdirSync("/proc")) {
    if (!/^\d+$/.test(p) || Number(p) === process.pid) continue;
    let comm;
    try {
      comm = readFileSync(`/proc/${p}/comm`, "utf8").trim();
    } catch {
      continue;
    }
    if (!watched.test(comm)) continue;
    let held = false;
    let fds = [];
    try {
      fds = readdirSync(`/proc/${p}/fd`);
    } catch {
      continue;
    }
    for (const fd of fds) {
      try {
        if (readlinkSync(`/proc/${p}/fd/${fd}`) === lock) held = true;
      } catch {}
    }
    let args = comm;
    try {
      args = readFileSync(`/proc/${p}/cmdline`, "utf8").split("\0").filter(Boolean).slice(0, 3).join(" ").slice(0, 90);
    } catch {}
    bump(held ? holding : without, `${comm} (${args})`);
  }
}

const timer = setInterval(sample, 500);
process.on("SIGTERM", () => {
  clearInterval(timer);
  console.log(`samples: ${samples}`);
  console.log("holding the lock (process, samples seen):");
  for (const [k, n] of [...holding].sort()) console.log(`  ${n}  ${k}`);
  console.log("watched processes without it:");
  for (const [k, n] of [...without].sort()) console.log(`  ${n}  ${k}`);
  process.exit(0);
});
