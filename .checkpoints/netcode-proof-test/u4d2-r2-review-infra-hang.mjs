// The infra seat's U4d2 round-2 harness (FX-7, by count): how many of the wasm step's calls each wait out the 300 s
// limit when a hang sits in npt-sim's replay (every leg and every compare that replays ticks hangs; listing, h2's
// header-only vectors and the builds do not). It imports wasmStep from the copy given and passes a fake runner and a
// fake io (no cargo, no child process, no file written); it prints to stdout only.
//   node u4d2-r2-review-infra-hang.mjs <spikes/netcode-proof folder of a copy>
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const here = process.argv[2];
const { wasmStep, WASM_BUILDS, WASM_BITES } = await import(pathToFileURL(join(here, "check.mjs")).href);

const calls = [];
const artifact = (b) =>
  JSON.stringify({
    reason: "compiler-artifact",
    target: { name: b.target, kind: [b.kind] },
    executable: b.kind === "bin" ? `/fake/${b.copy}` : null,
    filenames: b.kind === "bin" ? [] : [`/fake/${b.copy}`],
  });
const vectorsLine = "wrote the header-only recordings vector-SS, vector-ES, vector-CO, vector-TC (seed 1, first tick 0, no tick) for h2";
const stopped = (stdout = "") => ({ ok: false, status: null, out: stdout, stdout, timedOut: true });
const done = (stdout = "") => ({ ok: true, status: 0, out: stdout, stdout, timedOut: false });

function runner(cmd, args, cwd, env, limit) {
  let r;
  let what;
  if (cmd === "cargo") {
    const b = WASM_BUILDS.find((x) => JSON.stringify(x.args) === JSON.stringify(args));
    what = "build";
    r = done(`${artifact(b)}\n`);
  } else if (cmd === process.execPath) {
    const rec = args[args.indexOf("--name") + 1];
    what = rec.startsWith("vector-") ? "loader, header-only" : "loader, replays ticks";
    r = rec.startsWith("vector-") ? done("") : stopped("");
  } else {
    const mode = args[1];
    if (mode === "bites") {
      what = "npt-lab hash bites";
      r = done(`1 ${WASM_BITES.hashMap}\n2 ${WASM_BITES.stdSin}\n`);
    } else if (mode === "vectors") {
      what = "npt-lab hash vectors";
      r = done("h2: CO's wasm32 dump is the committed vector\n");
    } else if (mode === "compare") {
      const replays = args.includes("--digest-field") || args.includes("--ours-dump") || args.includes("--theirs-dump");
      what = replays ? "npt-lab hash compare, replays (replay_to)" : "npt-lab hash compare, lists only";
      r = replays ? stopped("") : done("");
    } else {
      what = "npt-lab hash legs";
      r = stopped(`${vectorsLine}\n`);
    }
  }
  calls.push({ what, limit, stopped: r.timedOut });
  return r;
}

const io = {
  reset: () => {},
  mkdir: () => {},
  copy: () => {},
  exists: () => true,
  read: (p) => readFileSync(join(here, p), "utf8"),
  libc: () => "glibc (harness)",
};
let clock = 0;
const result = wasmStep(runner, io, () => (clock += 1));
const waits = calls.filter((c) => c.stopped);
console.log(`calls: ${calls.length}; with the 300 s limit: ${calls.filter((c) => c.limit === 300_000).length}; with none: ${calls.filter((c) => c.limit === 0).length}`);
console.log(`calls that would each wait out 300 s: ${waits.length} (${(waits.length * 300) / 60} min of the exclusive lock)`);
const by = new Map();
for (const c of waits) by.set(c.what, (by.get(c.what) ?? 0) + 1);
for (const [k, n] of by) console.log(`  ${n} x ${k}`);
console.log(`step ok: ${result.ok}; problems naming the limit: ${result.out.split("\n").filter((l) => l.includes("still running after 300 s")).length}`);
