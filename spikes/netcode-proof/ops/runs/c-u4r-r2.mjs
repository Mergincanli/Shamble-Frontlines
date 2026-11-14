#!/usr/bin/env node
// U4r's contract test r2 (u4r-brief.md 2.4 r2), re-made from a commit: the advisor's U4r build ruling 3, point 6 (4).
// The builder's own r2 runs are builder evidence only (ruling 2, point 33); this script's run from the merge commit is
// the record, before -17's judged runs and before carry 62 (h)'s setup check is decided (7.1).
//   node spikes/netcode-proof/ops/runs/c-u4r-r2.mjs --date YYYY-MM-DD
// - Each suite's three lines exactly as the builder ran them (.gate/u4r/c3-r2-p0.log, c3-r2-suites.log and
//   c3-r2-bites.log, U4r's copy 3; then copy 5's runs of ruling 3), but for --date, --commit, --out and the copies of
//   marks.json and SERIES.md: `npt-lab scenario` (SS or ES, 48 clients, seed 1, a 10 s warm-up and 20 s measured unless
//   the suite names other lengths), then `npt-lab contract --logs <its folder> --series <SERIES.md's copy>`, then
//   `npt-judge --marks <marks.json's copy> --logs <its folder> --only <the run's marks> --out <suite>-verdict.json`.
// - The suites: the ten measured ones of copy 3 (p0, p2, p3, b1; -21's option-B stand-ins forged, clamp, fast-clock and
//   lag-switch; es, carry 59's; evstart, carry 75's); ruling 3's point 3 pair (the tick16 bite at tick_start 64,651, so
//   the 16-bit wrap falls at measured tick 585, 15 before the measured end; and its clean partner at 64,936, the wrap at
//   measured tick 300); point 4's bitten B1 run (PM-0001-18/no-fallback; since point 8 (A) the mirror keeps the frame
//   the room's stale copy codes against); and carry 62 (h)'s bite, PM-0001-17/whole-as-lost, on the p0 suite's
//   settings (advisor-u4r-r1, FP-5 (i): T-7; the builder's .gate/u4r/whole-as-lost/ is a labelled dry check only).
//   Copy 3's three earlier tick16 tries (tick_start 65,506) stay builder evidence and are not re-made.
// - Every command runs whatever the one before gave, each exit code recorded: a FAIL is a result here, never a stop and
//   never re-run (condition 5). Each command runs through check.mjs's run() with a 600 s limit, in its own process
//   group, which is killed whole at the limit: its exit line then reads "stopped at <s> s", a result, never a retry
//   (FP-5 (f): INF-U4R-11). Expected, by ruling 3: b1 fails PM-0001-2's everyone-in-view share, so its -17 and -18 are
//   INVALID and not counted (point 1: npt-room's, fixed in U4g); es fails ES's reliable-events band (point 2: U4h's);
//   b1-nofallback fails -18's max check and is INVALID by pre:PM-0001-2 while the room is unfixed (points 4 and 8 (A)).
//   Each suite's line in r2.txt states what is expected beside what came; the main session reads both.
// - Run it under the lane lock, exclusive (u4r-brief.md section 4 rule 2: it holds bite runs, and its outputs are a
//   record): `bash <runner> -x <copy> <log> node ops/runs/c-u4r-r2.mjs --date ...` or the main folder's own runner.
// - Its order (FP-5 (a): INF-U4R-2): the usage and git checks (no uncommitted change to a tracked file, and no untracked
//   file under spikes/netcode-proof/crates: every log names its commit); each suite's key checked as a name (one or more
//   of A-Z, a-z, 0-9 and -: FP-5 (h)); the projection; each release build (--release --locked --offline), with at least
//   4 GiB free before each (section 4 rule 6); the projection again; only then the output folder,
//   .gate/netcode-proof-test/r2-<commit12>/ (gitignored, never overwritten): one folder per suite (its three logs),
//   <suite>-verdict.json beside it, the copies of marks/marks.json and marks/SERIES.md (their sha256s in r2.txt's head:
//   FP-5 (e)), and r2.txt (the commit, the binaries' SHA-256s, then per suite each command, its exit line and its
//   output). The binaries go to .gate/netcode-proof-test/bin/r2-<commit12>/, which no exists-check reads, so a stop
//   before the first suite leaves nothing that blocks a re-run at that commit. Before r2.txt is written, HEAD and the
//   tracked files' status are checked against their start values (FP-5 (e)).
// - The projection (FP-5 (b), (c)): the builder's own 13 suites took about 145 MB (u4r-build.md, copy 5's state: the
//   r2 folder; their 13 folders sum to 115,746,371 B, du -sb in copy 6), plus whole-as-lost at the p0 suite's size
//   (24,813,722 B); it starts only when free space minus that total is at least 4 GiB, printing both; before each suite,
//   free space minus that suite's own measured size must be at least 4 GiB.
// - The binaries: npt-lab's measured and bites builds and npt-judge, each artifact's path read from cargo's
//   --message-format=json messages (check.mjs's builtArtifact) and copied out before the next build replaces it.
// - Its pure helpers are exported (the projection, the name check); main runs only when this file is the entry module.

import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, readFileSync, statfsSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { builtArtifact, run } from "../../check.mjs";

const SPIKE = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const ROOT = join(SPIKE, "..", "..");
const GATE = join(ROOT, ".gate", "netcode-proof-test");
/** The free space a build and a suite must leave, bytes (section 4 rule 6's 4 GB floor, as 4 GiB). */
export const FLOOR = 4 * 1024 ** 3;
/** Each command's limit, ms (FP-5 (f)): the longest suite command the builder ran took well under it. */
const STEP_LIMIT_MS = 600_000;
const MARKS_17 = "PM-0001-2,PM-0001-17,PM-0001-57";
const B1 = "--profile B1 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 30 --bursts 2 --burst-first-s 5 --burst-every-s 15";
const P0 = "--scene SS --profile P0 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 20";
/** The builder's 13 suites' bytes, about 145 MB (the module text), and the p0 suite's own (whole-as-lost's projection). */
export const SUITES_13_BYTES = 145_000_000;
const P0_SUITE_BYTES = 24_813_722;
/** A suite key, a folder name (FP-5 (h)): one or more of A-Z, a-z, 0-9 and -. */
const NAME = /^[A-Za-z0-9-]+$/;

/**
 * The suites, in the order the builder ran them: `build` names the npt-lab build (`lab` measured, `bites`), `run` the
 * run id, `args` the scenario's settings after --out, `marks` its header's marks (the judge's --only), `expect` ruling
 * 3's or copy 3's outcome, `bytes` the builder's folder of that suite (du -sb, copy 6; whole-as-lost: the p0 suite's).
 */
export const SUITES = [
  { key: "p0", build: "lab", run: "u4r-r2-p0-1", args: P0, marks: MARKS_17, suite: "r2-p0", bytes: P0_SUITE_BYTES, expect: "contract PASS; PM-0001-2's SS checks, -17 (0 mismatches) and -57 pass on the run" },
  { key: "p2", build: "lab", run: "u4r-r2-p2-1", args: "--scene SS --profile P2 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 20", marks: MARKS_17, suite: "r2-p2", bytes: 11_483_900, expect: "contract PASS; PM-0001-2, -17 and -57 pass" },
  { key: "p3", build: "lab", run: "u4r-r2-p3-1", args: "--scene SS --profile P3 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 20", marks: MARKS_17, suite: "r2-p3", bytes: 6_509_038, expect: "contract PASS; PM-0001-2, -17 and -57 pass (carry 62 (g): P3, the reorder step on)" },
  { key: "b1", build: "lab", run: "u4r-r2-b1-1", args: `--scene SS ${B1}`, marks: `${MARKS_17},PM-0001-18`, suite: "r2-b1", bytes: 13_900_772, expect: "contract PASS; PM-0001-2 FAIL (everyone in view 0.273333, needs >= 0.99), so -17 and -18 L1 B1 INVALID by pre:PM-0001-2, recorded and not counted; -57 passes (ruling 3, point 1: npt-room's, fixed in U4g)" },
  { key: "forged", build: "lab", run: "u4r-r2-forged-1", args: "--scene SS --profile P0 --seed 1 --warmup-s 10 --measured-s 20 --behaviour presses", marks: "PM-0001-2", suite: "forged", bytes: 2_761_291, expect: "contract PASS; PM-0001-2 passes" },
  { key: "clamp", build: "lab", run: "u4r-r2-clamp-1", args: "--scene SS --profile P0 --seed 1 --warmup-s 10 --measured-s 20 --behaviour clamp", marks: "PM-0001-2", suite: "clamp", bytes: 2_734_499, expect: "contract PASS; PM-0001-2 passes" },
  { key: "fast-clock", build: "lab", run: "u4r-r2-fast-clock-1", args: "--scene SS --profile P0 --seed 1 --warmup-s 10 --measured-s 20 --behaviour steady", marks: "PM-0001-2", suite: "fast-clock", bytes: 2_761_213, expect: "contract PASS; PM-0001-2 passes" },
  { key: "lag-switch", build: "lab", run: "u4r-r2-lag-switch-1", args: "--scene SS --profile P1 --seed 1 --warmup-s 10 --measured-s 20 --behaviour steady", marks: "PM-0001-2", suite: "lag-switch", bytes: 2_878_953, expect: "contract PASS; PM-0001-2 passes" },
  { key: "es", build: "lab", run: "u4r-r2-es-1", args: "--scene ES --profile P0 --seed 1 --warmup-s 10 --measured-s 20", marks: "PM-0001-2", suite: "r2-es", bytes: 2_737_959, expect: "contract PASS; PM-0001-2 FAIL by ES's reliable events (50.05 a second against 9.5-10.5), the other 11 ES checks pass; no ES run counts at U4r (ruling 3, point 2: U4h's)" },
  { key: "evstart", build: "lab", run: "u4r-r2-evstart-1", args: "--scene SS --profile P0 --seed 1 --tick-start 63736 --event-start 65533 --warmup-s 10 --measured-s 20", marks: "PM-0001-2,PM-0001-17", suite: "r2-evstart", bytes: 2_766_566, expect: "contract PASS; PM-0001-2 and -17 (0 mismatches) pass (carry 75)" },
  { key: "tick16", build: "bites", run: "u4r-r2-tick16-4", args: "--scene SS --profile P0 --seed 1 --tick-start 64651 --warmup-s 10 --measured-s 20 --after-max-s 5 --role bite --bite PM-0001-17/tick16", marks: "PM-0001-2,PM-0001-17", suite: "r2-tick16", bytes: 2_937_440, expect: "contract PASS (0 mirror breaches); PM-0001-2 passes; -17's bite run FAILs by decoded_hash_mismatch_count with at least 48 mismatches; after_capped as it comes (false when the room's overflow resync resolves the measured ticks before the cap, true when the cap path of ruling 3, point 7 (a) ends the run); carry 62 (h)'s measured sum is the bite's own (tick16d: 720 = 48 x 15 post-wrap measured ticks) (ruling 3, point 3, as ruling 4, point 1 reads it; carry 73)" },
  { key: "tick16-clean", build: "lab", run: "u4r-r2-tick16-clean-1", args: "--scene SS --profile P0 --seed 1 --tick-start 64936 --warmup-s 10 --measured-s 20 --after-max-s 5", marks: MARKS_17, suite: "r2-tick16-clean", bytes: 24_813_880, expect: "contract PASS (0 mirror breaches); PM-0001-2, -17 (0 mismatches) and -57 pass; the after phase resolves without the cap (ruling 3, point 3)" },
  { key: "b1-nofallback", build: "bites", run: "u4r-r2-b1-nofallback-1", args: `--scene SS ${B1} --role bite --bite PM-0001-18/no-fallback`, marks: `${MARKS_17},PM-0001-18`, suite: "r2-b1", bytes: 14_647_138, expect: "contract PASS (0 mirror breaches, ruling 3, point 8 (A)); -18's max check FAILs (a heal later than 500 ms); INVALID by pre:PM-0001-2 is expected while npt-room is unfixed (ruling 3, points 4 and 8 (A))" },
  { key: "whole-as-lost", build: "bites", run: "u4r-r2-whole-as-lost-1", args: `${P0} --role bite --bite PM-0001-17/whole-as-lost`, marks: "PM-0001-17", suite: "r2-whole-as-lost", bytes: P0_SUITE_BYTES, expect: "carry 62 (h)'s bite: measured incomplete equal to the client count (48), as in the builder's .gate/u4r/whole-as-lost/ dry check; -17's verdict as it comes until marks.json holds ruling 4's plain check" },
];

/** The set's projected bytes (FP-5 (b)): the 13 suites' figure plus whole-as-lost's, or the suites' own sum if larger. */
export function r2Projected(suites = SUITES) {
  const sum = suites.reduce((s, x) => s + x.bytes, 0);
  return Math.max(SUITES_13_BYTES + P0_SUITE_BYTES, sum);
}

/** Each suite key that is not a name, as a sentence (FP-5 (h)). */
export const keyProblems = (suites = SUITES) =>
  suites.filter((s) => !NAME.test(s.key)).map((s) => `suite key ${JSON.stringify(s.key)} is not one or more of A-Z, a-z, 0-9 and -: refused before any use`);

/** Whether `free` bytes leave the 4 GiB floor after `projected` more, with the line that prints both. */
export function roomFor(free, projected, what) {
  const ok = free - projected >= FLOOR;
  const mib = (b) => (b / 1024 ** 2).toFixed(0);
  return {
    ok,
    line: `${what}: free ${mib(free)} MiB, projected ${mib(projected)} MiB, so ${mib(free - projected)} MiB after it against the 4 GiB floor (4096 MiB): ${ok ? "room" : "no room, stopped before it"}`,
  };
}

function sh(cmd, args, cwd = SPIKE) {
  const r = spawnSync(cmd, args, { cwd, encoding: "utf8", maxBuffer: 1 << 28 });
  if (r.status !== 0) throw new Error(`${cmd} ${args.join(" ")}: exit ${r.status}\n${r.stdout}${r.stderr}`);
  return r.stdout.trim();
}

const sha256 = (path) => createHash("sha256").update(readFileSync(path)).digest("hex");

function freeBytes() {
  const s = statfsSync(ROOT);
  return s.bavail * s.bsize;
}

/** Stops unless free space minus `projected` bytes leaves the floor (0: a build's own check, 4 GiB free). */
function floor(what, projected = 0) {
  const r = roomFor(freeBytes(), projected, what);
  if (!r.ok) throw new Error(r.line);
}

/** One release build, at least 4 GiB free before it, its artifact's path read from cargo's JSON messages and copied to `to`. */
function build(args, target, to) {
  floor(`the release build of ${target}${args.includes("bites") ? " (bites)" : ""}`);
  const out = sh("cargo", ["build", "--release", "--locked", "--offline", ...args, "--message-format=json"]);
  const path = builtArtifact(out, { target, kind: "bin" });
  if (!path) throw new Error(`cargo build ${args.join(" ")} names no ${target} executable in its JSON messages`);
  copyFileSync(path, to);
  return to;
}

/**
 * One command, run from the spike whatever it gives, through check.mjs's run() with STEP_LIMIT_MS (its own process
 * group, killed whole at the limit: FP-5 (f)): its line, its exit line ("stopped at <s> s" for a stopped one) and output.
 */
function step(exe, args) {
  const t0 = Date.now();
  const r = run(exe, args, SPIKE, process.env, STEP_LIMIT_MS);
  const secs = ((Date.now() - t0) / 1000).toFixed(1);
  const exit = r.timedOut ? `stopped at ${secs} s` : `exit ${r.status ?? "by a signal"} (wall ${secs} s)`;
  return [`$ ${[exe, ...args].join(" ")}`, exit, r.out.trimEnd()];
}

function main(argv) {
  const date = argv.includes("--date") ? argv[argv.indexOf("--date") + 1] : undefined;
  if (!/^\d{4}-\d{2}-\d{2}$/.test(date ?? "") || argv.length !== 2) throw new Error("usage: c-u4r-r2.mjs --date YYYY-MM-DD");
  const tracked = () => sh("git", ["status", "--porcelain", "--untracked-files=no"]);
  if (tracked()) throw new Error("tracked files have uncommitted changes: commit first, since every log names its commit");
  const untracked = sh("git", ["status", "--porcelain", "--untracked-files=all", "--", "crates"]);
  if (untracked) throw new Error(`files under spikes/netcode-proof/crates are not committed (${untracked.split("\n").join("; ")}): commit first, since every log names its commit`);
  const commit = sh("git", ["rev-parse", "HEAD"]);
  const start = { commit, status: tracked() };
  const out = join(GATE, `r2-${commit.slice(0, 12)}`);
  if (existsSync(out)) throw new Error(`${out} exists: a run set is never overwritten`);
  const bad = keyProblems();
  if (bad.length) throw new Error(bad.join("; "));
  const total = r2Projected();
  const room = (when) => {
    const r = roomFor(freeBytes(), total, `r2: the ${SUITES.length} suites' projected logs, ${when}`);
    console.log(r.line);
    if (!r.ok) throw new Error(r.line);
    return r.line;
  };
  const before = room("before the builds");
  const bin = join(GATE, "bin", `r2-${commit.slice(0, 12)}`);
  mkdirSync(bin, { recursive: true });
  const exe = {
    lab: build(["-p", "npt-lab"], "npt-lab", join(bin, "npt-lab")),
    bites: build(["-p", "npt-lab", "--features", "bites"], "npt-lab", join(bin, "npt-lab-bites")),
  };
  const judge = build(["-p", "npt-judge"], "npt-judge", join(bin, "npt-judge"));
  const after = room("after the builds");
  mkdirSync(out, { recursive: true });
  const copies = {};
  for (const f of ["marks.json", "SERIES.md"]) {
    copies[f] = join(out, f);
    copyFileSync(join(SPIKE, "marks", f), copies[f]);
  }
  const lines = [
    `# U4r's r2, re-made from commit ${commit} on ${date} (the advisor's U4r build ruling 3, point 6 (4))`,
    `npt-lab (measured build) sha256 ${sha256(exe.lab)}`,
    `npt-lab (bites build) sha256 ${sha256(exe.bites)}`,
    `npt-judge sha256 ${sha256(judge)}`,
    `marks.json sha256 ${sha256(copies["marks.json"])}, SERIES.md sha256 ${sha256(copies["SERIES.md"])} (copied into the output folder at the start; contract and the judge read the copies)`,
    `projection: ${before}`,
    `projection: ${after}`,
    "",
  ];
  for (const s of SUITES) {
    floor(`suite ${s.key}`, s.bytes);
    const logs = join(out, s.key);
    const said = [
      `## suite ${s.key} (the ${s.build === "lab" ? "measured" : "bites"} build); expected: ${s.expect}`,
      ...step(exe[s.build], ["scenario", "--run", s.run, "--set", "u4r-r2", "--date", date, "--commit", commit, "--out", logs, ...s.args.split(" "), "--marks", s.marks, "--suite", s.suite]),
      ...step(exe[s.build], ["contract", "--logs", logs, "--series", copies["SERIES.md"]]),
      ...step(judge, ["--marks", copies["marks.json"], "--logs", logs, "--only", s.marks, "--out", join(out, `${s.key}-verdict.json`)]),
      "",
    ];
    console.log(said.join("\n"));
    lines.push(...said);
  }
  if (sh("git", ["rev-parse", "HEAD"]) !== start.commit || tracked() !== start.status) {
    throw new Error("HEAD or the tracked files changed since the set started: stopped before r2.txt is written (FP-5 (e))");
  }
  writeFileSync(join(out, "r2.txt"), `${lines.join("\n")}\n`);
  console.log(`written: ${join(".gate", "netcode-proof-test", `r2-${commit.slice(0, 12)}`, "r2.txt")}`);
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    main(process.argv.slice(2));
  } catch (e) {
    console.error(`c-u4r-r2: ${e.message}`);
    process.exit(1);
  }
}
