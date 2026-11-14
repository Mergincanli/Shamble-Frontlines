// The infra seat's U4d2 round-2 probe (FX-7): does spawnSync's `timeout`, as check.mjs's run() uses it, leave a
// grandchild running that still holds a flock fd inherited from the runner (u4d2-run.sh's fd 9)? No cargo, no shared
// lock: the lock is this file's own, opened read-only by the caller (`exec 9<this-file; flock -x 9`), as the runner
// opens .lane.lock. It writes no file; it prints to stdout only.
//   node u4d2-r2-review-infra-probe.mjs <plain|group> <tag>
// plain: run()'s form (spawnSync with `timeout`). group: the same with `detached: true` and, on ETIMEDOUT, a SIGKILL
// to the child's whole process group (a candidate fix). The child is `sh -c 'sleep <tag> & wait'`: the grandchild
// (sleep) stands for a native leg (npt-lab hash's Leg child) stuck in compute, which neither reads nor writes a pipe.
import { spawnSync } from "node:child_process";
import { readdirSync, readFileSync, readlinkSync } from "node:fs";
import { fileURLToPath } from "node:url";

const self = fileURLToPath(import.meta.url);
const [mode = "plain", tag = "41.3"] = process.argv.slice(2);
const pause = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);

/** Every process but this one with an fd on `path`: "pid comm fd pgid ppid". */
function holders(path) {
  const out = [];
  for (const p of readdirSync("/proc")) {
    if (!/^\d+$/.test(p) || Number(p) === process.pid) continue;
    let fds;
    try {
      fds = readdirSync(`/proc/${p}/fd`);
    } catch {
      continue;
    }
    for (const fd of fds) {
      let to;
      try {
        to = readlinkSync(`/proc/${p}/fd/${fd}`);
      } catch {
        continue;
      }
      if (to !== path) continue;
      let comm = "?";
      let pgid = "?";
      let ppid = "?";
      try {
        comm = readFileSync(`/proc/${p}/comm`, "utf8").trim();
        const stat = readFileSync(`/proc/${p}/stat`, "utf8");
        const f = stat.slice(stat.lastIndexOf(")") + 2).split(" ");
        ppid = f[1];
        pgid = f[2];
      } catch {}
      out.push(`pid ${p} ${comm} fd ${fd} pgid ${pgid} ppid ${ppid}`);
    }
  }
  return out;
}

console.log(`probe ${mode}: node ${process.version}, pid ${process.pid}, holders before: ${JSON.stringify(holders(self))}`);
const t0 = Date.now();
const r = spawnSync("sh", ["-c", `sleep ${tag} & wait`], {
  encoding: "utf8",
  maxBuffer: 1 << 28,
  timeout: 1500,
  ...(mode === "group" ? { detached: true } : {}),
});
const ms = Date.now() - t0;
console.log(`probe ${mode}: returned after ${ms} ms: status ${r.status}, signal ${r.signal}, error.code ${r.error?.code}, pid ${r.pid}`);
if (mode === "group" && r.error?.code === "ETIMEDOUT") {
  try {
    process.kill(-r.pid, "SIGKILL");
    console.log(`probe ${mode}: SIGKILL sent to process group ${r.pid}`);
  } catch (e) {
    console.log(`probe ${mode}: the group kill failed: ${e.code}`);
  }
}
pause(400);
console.log(`probe ${mode}: holders 400 ms after the call: ${JSON.stringify(holders(self))}`);
