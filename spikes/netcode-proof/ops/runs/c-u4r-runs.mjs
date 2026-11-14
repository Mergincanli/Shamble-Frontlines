#!/usr/bin/env node
// U4r's judged runs and the -30 pre-check, in this container (u4r-brief.md 2.3 and section 7; c-u2-pm-0001-3.mjs's
// model). One script for every set, the set named first:
//   node spikes/netcode-proof/ops/runs/c-u4r-runs.mjs precheck --date YYYY-MM-DD
//   node spikes/netcode-proof/ops/runs/c-u4r-runs.mjs judged --profile P0|P2|P3|B1 --date YYYY-MM-DD
// - How it is run (advisor-u4r-r1, FP-5 (d): INF-U4R-8): in the main folder, with CARGO_TARGET_DIR unset (plan §7.1
//   item 6: the judged runs build in the main folder's own cache), not through main-run.sh; it refuses a set
//   CARGO_TARGET_DIR, since a shared-target run comes to the advisor first (u4r-brief.md section 12 item 8). The main
//   session's notes say whether it held the lane lock (`flock -x` on .gate/shared-target/.lane.lock) for the set.
// - precheck (set u4r-precheck-30; 7.2): SS P0 seed 4's recording made again by `npt-lab scenario --record` at its
//   crates/lab/precheck-recordings.json entry's settings (the set stops if that run's rewinds refused are above 0: 7.1),
//   then three pairs by `npt-lab precheck`, each checking the file against that entry first: clean (the measured
//   build), PM-0001-30/wallclock and PM-0001-30/late-command (the bites build), each bite on the L1 leg. runs.txt holds
//   every pair's lines whole (both legs' records, the comparer's output and the result) and no verdict: the judge
//   never reads these runs. The late-command pair is as expected only when its whole result line is
//   `result: VOID (L1 late_count 1)` (precheck's own test asserts that line: FP-5 (g)).
// - judged (set u4r-<profile>; 7.1): the profile's three measured runs (seeds 1 to 3; SS, 48 clients, L1, a 10 s
//   warm-up, 600 s measured, tick_start 63,736; B1 with 10 bursts, every 60 s from 30 s into the measured phase), their
//   headers listing PM-0001-2 (every L1 part's pre:PM-0001-2), PM-0001-17 and PM-0001-57 (-57's parts are on -17's
//   runs), and PM-0001-18 on B1; then, in the bites build at seed 1 with the same settings, one run per bite of those
//   marks (marks.json's lists, read from the set's copy) whose part has this profile: -17's and -57's on P0, each header
//   listing PM-0001-2 and the bite's own mark; then `npt-judge --only` on the set's folder. One profile per call keeps
//   each set's logs and the judge's memory to one profile's (the advisor's U4r build ruling, point 3: the main session
//   plans the disk and judges each set's folder apart). A run whose summary line counts an overflow resync above 0 and
//   whose header lists -17 or -18 is marked on its runs.txt line: it goes to the advisor before it counts (carry 58;
//   FP-5 (g)).
// - The tick16 bite run (the advisor's U4r build ruling 3, point 3): its tick_start is computed from its own lengths,
//   65,551 - (warm-up ticks + measured ticks) (47,251 for 300 + 18,000), so the 16-bit wrap falls 15 ticks before the
//   measured phase's end and the measured window ends before the bitten room evicts; every other -17 run, clean and the
//   client-rounding bite run, starts at 63,736 (U4R-Q7 (a) as that ruling reads it).
// - No -18 bite run at U4r (the advisor's U4r build ruling 3, point 1): -18's bitten judged run (PM-0001-18/no-fallback)
//   moves to U4g with the room's fix; ws-budget-x2 is L2's. The three B1 measured runs stay in this set for -57's L1 B1
//   part, which has no precondition; their -17 and -18 verdicts are recorded INVALID (pre:PM-0001-2's everyone-in-view
//   share, npt-room's recovery after each burst) and never counted.
// - Its order (FP-5 (a): INF-U4R-2): the usage and git checks (no uncommitted change to a tracked file, and no untracked
//   file under spikes/netcode-proof/crates: every log names its commit); the CARGO_TARGET_DIR refusal; the names (the
//   pre-check entry's name and each bite id's tail, each one or more of A-Z, a-z, 0-9 and -, before any use: FP-5 (h));
//   the projection; each release build, with at least 4 GiB free before each (u4r-brief.md section 4 rule 6); the
//   projection again; only then the set's folders. The binaries go to .gate/netcode-proof-test/bin/<set>-<commit12>/,
//   which no exists-check reads, so a stop before the first run leaves nothing that blocks a re-run at that commit.
// - The projection (FP-5 (b), (c): INF-U4R-3): each run's log bytes from the builder's measured rates (BYTES_10MIN_57,
//   BYTES_10MIN_OTHER, PAIR_BYTES), the set's total checked before the builds and after them: it starts only when free
//   space minus the total is at least 4 GiB, printing both; before each run, free space minus that run's projected
//   bytes must be at least 4 GiB, else it stops before that run.
// - Raw logs go under .gate/netcode-proof-test/runs/<set>-<commit12>/ (one folder per run), results under
//   results/u4r/<commit12>/<profile>/ or results/u4r-precheck-30/<commit12>/; neither is ever overwritten. Right after
//   the raw-log folder is made, marks/marks.json is copied into it (its sha256 in runs.txt's head); the bite list is read
//   from that copy and the judge is given it. Before the judge, HEAD and the tracked files' status are checked against
//   their start values, and the set stops if either changed (FP-5 (e): INF-U4R-9). runs.txt's head names the target
//   (the main folder's own cache) and the uptime and load average at the start and at the end.
// - Both npt-lab builds and npt-judge are built --release --locked --offline, each artifact's path read from cargo's
//   --message-format=json messages (check.mjs's builtArtifact, as the wasm step reads its builds) and copied out before
//   the next build replaces it.
// - Each npt-lab process is timed (the wall time around the call) and its seconds go on its runs.txt line beside its
//   ticks (npt-lab's summary line), for the ticks-per-second figure (2.4 r5, 7.3).
// - No run logs its clients' bytes (`scenario --bytes`): no part these sets judge reads a replay log (only an x86_64
//   replay log of an L2 Arm run is read: judge/src/figures.rs:222-233).
// - Its pure helpers are exported (the projection, the name check, the summary line's flag, the pairs' check); main
//   runs only when this file is the entry module.

import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, readFileSync, statfsSync, writeFileSync } from "node:fs";
import { loadavg, uptime } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { builtArtifact } from "../../check.mjs";

const SPIKE = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const ROOT = join(SPIKE, "..", "..");
const GATE = join(ROOT, ".gate", "netcode-proof-test");
const SEEDS = [1, 2, 3];
const PROFILES = ["P0", "P2", "P3", "B1"];
/** -17's L1 runs start here, clean and the client-rounding bite run (U4R-Q7 (a), as ruling 3, point 3 reads it). */
const TICK_START = 63_736;
/** Each judged run's warm-up and measured lengths, s (plan :767: 10 minutes measured). */
const WARMUP_S = 10;
const MEASURED_S = 600;
/** npt_sim::TICK_HZ: ticks per second. */
const TICK_HZ = 30;
/**
 * The tick16 bite run's start (the advisor's U4r build ruling 3, point 3): from its own lengths, so the wrap (65,536)
 * falls 15 ticks before its measured phase's end: 65,551 - (warm-up ticks + measured ticks).
 */
const TICK16 = "PM-0001-17/tick16";
const TICK16_START = 65_536 + 15 - (WARMUP_S + MEASURED_S) * TICK_HZ;
/** The bites of these marks that only L2 plants (marks.json: -18's ws-budget-x2 is the L2 B1 ws part's). */
const L2_ONLY = ["PM-0001-18/ws-budget-x2"];
/** The bites whose judged run is not U4r's (the advisor's U4r build ruling 3, point 1: -18's no-fallback, to U4g). */
const NOT_AT_U4R = ["PM-0001-18/no-fallback"];
/** The free space a build and a run must leave, bytes (section 4 rule 6's 4 GB floor, as 4 GiB). */
export const FLOOR = 4 * 1024 ** 3;
/** precheck-recordings.json's scene ids (Scene::by_id). */
const SCENES = { 1: "SS", 2: "ES", 3: "CO", 4: "TC" };
/** A name that reaches a file path or a run id (FP-5 (h): SEC-U4R-7): one or more of A-Z, a-z, 0-9 and -. */
const NAME = /^[A-Za-z0-9-]+$/;
/**
 * One judged run's projected log bytes over a 10-minute measured phase (18,300 ticks with its warm-up and after phase),
 * from the builder's measured bytes per measured tick (u4r-build.md:739-751; MB = 10^6 B): a run whose header lists
 * PM-0001-57 writes -57's three series (the advisor's U4r build ruling, point 3), P0 509.1 MB, P2 219.8 MB, P3 125.8
 * MB, B1 190.6 MB; any other run about 3,050 B a measured tick at every profile, 56 MB.
 */
export const BYTES_10MIN_57 = { P0: 509_100_000, P2: 219_800_000, P3: 125_800_000, B1: 190_600_000 };
export const BYTES_10MIN_OTHER = 56_000_000;
/**
 * One -30 pre-check pair's output (its lines, kept for runs.txt), at most 25,164 B in the builder's pre-check runs
 * (.gate/u4r/c4-b12-3.log:18-365: the clean, wallclock and late-command pairs, 24,840, 25,164 and 25,093 B); the
 * recording itself is its manifest entry's bytes.
 */
export const PAIR_BYTES = 25_164;
/** The late-command pair's one expected result line (precheck.rs, the late-command bite's test). */
export const LATE_COMMAND_LINE = "result: VOID (L1 late_count 1)";

/** Whether `s` is a name (NAME). */
export const nameOk = (s) => typeof s === "string" && NAME.test(s);

/** Each [what, value] whose value is not a name, as a sentence naming both (FP-5 (h)). */
export function nameProblems(named) {
  return named
    .filter(([, v]) => !nameOk(v))
    .map(([what, v]) => `${what} ${JSON.stringify(v)} is not one or more of A-Z, a-z, 0-9 and -: refused before any use`);
}

/**
 * One run's projected log bytes (FP-5 (b)): its profile's figure when its header lists PM-0001-57 (the largest, P0's,
 * for a profile the table lacks), else BYTES_10MIN_OTHER, scaled by its measured seconds over 600.
 */
export function runBytes({ profile, marks, measuredS }) {
  const per10 = marks.includes("PM-0001-57") ? (BYTES_10MIN_57[profile] ?? BYTES_10MIN_57.P0) : BYTES_10MIN_OTHER;
  return Math.ceil((per10 * measuredS) / 600);
}

/** The set's projected bytes: its runs' sum. */
export const projectedBytes = (plan) => plan.reduce((s, r) => s + r.bytes, 0);

/**
 * Whether `free` bytes leave the 4 GiB floor after `projected` more (FP-5 (b), (c)), with the line that prints both.
 */
export function roomFor(free, projected, what) {
  const ok = free - projected >= FLOOR;
  const mib = (b) => (b / 1024 ** 2).toFixed(0);
  return {
    ok,
    line: `${what}: free ${mib(free)} MiB, projected ${mib(projected)} MiB, so ${mib(free - projected)} MiB after it against the 4 GiB floor (4096 MiB): ${ok ? "room" : "no room, stopped before it"}`,
  };
}

/**
 * The mark a run's summary line takes (FP-5 (g): NC-U4R-6): its "overflow resyncs N" above 0 on a run whose header
 * lists PM-0001-17 or PM-0001-18 goes to the advisor before the run counts (carry 58); "" otherwise.
 */
export function overflowFlag(line, marks) {
  const n = Number(line.match(/overflow resyncs (\d+)/)?.[1] ?? 0);
  return n > 0 && marks.some((m) => m === "PM-0001-17" || m === "PM-0001-18") ? " (overflow resyncs: to the advisor before it counts, carry 58)" : "";
}

/**
 * Whether a pair's output is as expected (FP-5 (g)): the late-command pair only when its whole result line is
 * LATE_COMMAND_LINE; the clean and wallclock pairs by the result's first word.
 */
export function pairAsExpected(pair, expected, out) {
  const line = out.match(/^result: .*$/m)?.[0] ?? "";
  return pair === "late-command" ? line === LATE_COMMAND_LINE : line.match(/^result: (\S+)/)?.[1] === expected;
}

/** The marks a judged set's headers list (the module text). */
const listedOf = (profile) => ["PM-0001-2", "PM-0001-17", "PM-0001-57", ...(profile === "B1" ? ["PM-0001-18"] : [])];

/**
 * The bites a judged set runs, from marks.json's lists: one run each at seed 1 on P0, the profile of a part that reads
 * it (-17's and -57's); -18's none at U4r (ruling 3, point 1: to U4g), so a B1 set runs no bite.
 */
export function bitesOf(profile, marks) {
  const listed = listedOf(profile);
  const bitesOn = (mark) => (mark === "PM-0001-18" ? null : "P0");
  const out = [];
  for (const m of marks.marks.filter((x) => listed.includes(x.id) && x.id !== "PM-0001-2" && bitesOn(x.id) === profile)) {
    for (const b of (m.bites ?? []).filter((x) => !L2_ONLY.includes(x.id) && !NOT_AT_U4R.includes(x.id))) out.push({ id: b.id, mark: m.id });
  }
  return out;
}

/** The judged set's runs, in order, each with its projected bytes (the module text; FP-5 (b)). */
export function planJudged(profile, marks) {
  const listed = listedOf(profile);
  const runs = SEEDS.map((s) => ({ run: `l1-${profile.toLowerCase()}-s${s}`, exe: "lab", seed: s, marks: listed, extra: ["--role", "measured", "--marks", listed.join(",")], tickStart: TICK_START }));
  for (const b of bitesOf(profile, marks)) {
    const marksOf = ["PM-0001-2", b.mark];
    runs.push({ run: `bite-${b.id.split("/")[1]}`, exe: "labBites", seed: 1, marks: marksOf, extra: ["--role", "bite", "--bite", b.id, "--marks", marksOf.join(",")], tickStart: b.id === TICK16 ? TICK16_START : TICK_START });
  }
  return runs.map((r) => ({ ...r, what: r.run, bytes: runBytes({ profile, marks: r.marks, measuredS: MEASURED_S }) }));
}

/** The pre-check set's outputs, each with its projected bytes (FP-5 (b)): the recording, then the three pairs. */
export function planPrecheck(entry) {
  return [
    { what: `the recording ${entry.name}`, bytes: entry.bytes },
    ...["clean", "wallclock", "late-command"].map((p) => ({ what: `the ${p} pair`, bytes: PAIR_BYTES })),
  ];
}

function sh(cmd, args, cwd = ROOT) {
  const r = spawnSync(cmd, args, { cwd, encoding: "utf8", maxBuffer: 1 << 28 });
  if (r.status !== 0) throw new Error(`${cmd} ${args.join(" ")}: exit ${r.status}\n${r.stdout}${r.stderr}`);
  return r.stdout.trim();
}

const sha256 = (path) => createHash("sha256").update(readFileSync(path)).digest("hex");

/** The free bytes where the logs go. */
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
  const out = sh("cargo", ["build", "--release", "--locked", "--offline", ...args, "--message-format=json"], SPIKE);
  const path = builtArtifact(out, { target, kind: "bin" });
  if (!path) throw new Error(`cargo build ${args.join(" ")} names no ${target} executable in its JSON messages`);
  copyFileSync(path, to);
  return to;
}

/** One npt-lab process, timed: its summary line with the wall seconds. */
function timed(exe, args) {
  const t0 = Date.now();
  const out = sh(exe, args);
  const secs = ((Date.now() - t0) / 1000).toFixed(1);
  return { out, secs };
}

/** The machine's uptime and load average now (runs.txt's head: FP-5 (d)). */
const machineNow = () => `uptime ${uptime().toFixed(0)} s, load average ${loadavg().map((x) => x.toFixed(2)).join(" ")}`;

function main(argv) {
  const set = argv[0];
  const opt = (k) => (argv.includes(k) ? argv[argv.indexOf(k) + 1] : undefined);
  const date = opt("--date");
  const profile = opt("--profile");
  const usage = "usage: c-u4r-runs.mjs precheck --date YYYY-MM-DD | judged --profile P0|P2|P3|B1 --date YYYY-MM-DD";
  if (!/^\d{4}-\d{2}-\d{2}$/.test(date ?? "") || !["precheck", "judged"].includes(set)) throw new Error(usage);
  if (set === "judged" && !PROFILES.includes(profile)) throw new Error(usage);
  const tracked = () => sh("git", ["status", "--porcelain", "--untracked-files=no"]);
  if (tracked()) throw new Error("tracked files have uncommitted changes: commit first, since every log names its commit");
  const untracked = sh("git", ["status", "--porcelain", "--untracked-files=all", "--", "crates"], SPIKE);
  if (untracked) throw new Error(`files under spikes/netcode-proof/crates are not committed (${untracked.split("\n").join("; ")}): commit first, since every log names its commit`);
  if (process.env.CARGO_TARGET_DIR !== undefined) {
    throw new Error(
      `CARGO_TARGET_DIR is set (${JSON.stringify(process.env.CARGO_TARGET_DIR)}): this script runs in the main folder with its own cache (plan §7.1 item 6); a shared-target run comes to the advisor first (u4r-brief.md section 12 item 8)`,
    );
  }
  const commit = sh("git", ["rev-parse", "HEAD"]);
  const start = { commit, status: tracked(), machine: machineNow() };
  const short = commit.slice(0, 12);
  const name = set === "precheck" ? "u4r-precheck-30" : `u4r-${profile}`;
  const logs = join(GATE, "runs", `${name}-${short}`);
  const results = set === "precheck" ? join(SPIKE, "results", "u4r-precheck-30", short) : join(SPIKE, "results", "u4r", short, profile);
  for (const d of [logs, results]) if (existsSync(d)) throw new Error(`${d} exists: a run set is never overwritten`);
  const marksText = readFileSync(join(SPIKE, "marks", "marks.json"), "utf8");
  const marksSha = createHash("sha256").update(marksText).digest("hex");
  const manifest = join(SPIKE, "crates", "lab", "precheck-recordings.json");
  const entry = set === "precheck" ? JSON.parse(readFileSync(manifest, "utf8")).recordings[0] : null;

  // The names, before any use (FP-5 (h)).
  const named = set === "precheck" ? [["the pre-check entry's name", entry?.name]] : bitesOf(profile, JSON.parse(marksText)).map((b) => [`bite ${JSON.stringify(b.id)}'s tail`, b.id.split("/")[1]]);
  const bad = nameProblems(named);
  if (bad.length) throw new Error(bad.join("; "));

  // The projection, before the builds and after them (FP-5 (b)).
  const planOf = (marks) => (set === "precheck" ? planPrecheck(entry) : planJudged(profile, marks));
  const plan = planOf(JSON.parse(marksText));
  const total = projectedBytes(plan);
  const room = (when) => {
    const r = roomFor(freeBytes(), total, `${name}: the set's ${plan.length} runs' projected logs, ${when}`);
    console.log(r.line);
    if (!r.ok) throw new Error(r.line);
    return r.line;
  };
  const before = room("before the builds");

  // Both builds of npt-lab, each copied out before the next build replaces it, and the judge.
  const bin = join(GATE, "bin", `${name}-${short}`);
  mkdirSync(bin, { recursive: true });
  const lab = build(["-p", "npt-lab"], "npt-lab", join(bin, "npt-lab"));
  const labBites = build(["-p", "npt-lab", "--features", "bites"], "npt-lab", join(bin, "npt-lab-bites"));
  const judge = set === "judged" ? build(["-p", "npt-judge"], "npt-judge", join(bin, "npt-judge")) : null;
  const after = room("after the builds");

  // The set's folder, and its copy of marks.json, which the bite list and the judge read (FP-5 (e)).
  mkdirSync(logs, { recursive: true });
  const marksCopy = join(logs, "marks.json");
  copyFileSync(join(SPIKE, "marks", "marks.json"), marksCopy);
  if (sha256(marksCopy) !== marksSha) throw new Error("marks/marks.json changed since the set started: stopped before the first run");
  const runs = planOf(JSON.parse(readFileSync(marksCopy, "utf8")));
  const head = [
    `commit ${commit}`,
    "target: the main folder's own cache (CARGO_TARGET_DIR unset: plan §7.1 item 6)",
    `npt-lab (measured build) sha256 ${sha256(lab)}`,
    `npt-lab (bites build) sha256 ${sha256(labBites)}`,
    ...(judge ? [`npt-judge sha256 ${sha256(judge)}`] : []),
    `marks.json sha256 ${marksSha} (copied into the raw-log folder at the set's start; the bite list and the judge read that copy)`,
    `raw logs: .gate/netcode-proof-test/runs/${name}-${short}/ (not committed: plan §7.2)`,
    `projection: ${before}`,
    `projection: ${after}`,
    `start: ${start.machine}`,
  ];
  const common = ["--set", name, "--date", date, "--commit", commit];
  const lines = [];
  const unchanged = (what) => {
    if (sh("git", ["rev-parse", "HEAD"]) !== start.commit || tracked() !== start.status) {
      throw new Error(`HEAD or the tracked files changed since the set started: stopped before ${what} (FP-5 (e))`);
    }
  };

  if (set === "precheck") {
    const rec = join(logs, `${entry.name}.rec`);
    floor(runs[0].what, runs[0].bytes);
    const made = timed(lab, [
      "scenario", "--run", `record-${entry.name}`, ...common, "--out", join(logs, `record-${entry.name}`), "--scene", SCENES[entry.scene],
      "--profile", entry.profile, "--seed", String(entry.seed), "--clients", String(entry.clients), "--tick-start", String(entry.first_tick),
      "--warmup-s", String(entry.warmup_s), "--measured-s", String(entry.measured_s), "--logs", "0", "--suite", "record", "--record", rec,
    ]);
    console.log(made.out);
    lines.push(`# the recording (npt-lab scenario --record at the manifest entry's settings; wall ${made.secs} s)`, made.out, `${entry.name} sha256 ${sha256(rec)}`);
    const refused = Number(made.out.match(/rewinds refused (\d+)/)?.[1] ?? Number.NaN);
    if (refused !== 0) throw new Error(`${entry.name}'s run refused ${refused} rewinds: the set stops before the pre-check (7.1; carry 40)`);
    const pairs = [
      { pair: "clean", exe: lab, extra: [], expected: "PASS" },
      { pair: "wallclock", exe: labBites, extra: ["--bite", "PM-0001-30/wallclock", "--bite-leg", "L1"], expected: "FAIL" },
      { pair: "late-command", exe: labBites, extra: ["--bite", "PM-0001-30/late-command", "--bite-leg", "L1"], expected: "VOID" },
    ];
    for (const [i, p] of pairs.entries()) {
      floor(runs[i + 1].what, runs[i + 1].bytes);
      const r = timed(p.exe, ["precheck", "--manifest", manifest, "--recording", rec, "--commit", commit, ...p.extra]);
      const result = r.out.match(/^result: (\S+)/m)?.[1] ?? "none";
      const expected = p.pair === "late-command" ? `${p.expected}, the whole line "${LATE_COMMAND_LINE}"` : p.expected;
      const said = `## pair ${p.pair} (wall ${r.secs} s): result ${result}, expected ${expected}${pairAsExpected(p.pair, p.expected, r.out) ? ", as expected" : " (NOT as expected: to the advisor)"}`;
      console.log(said);
      lines.push(said, r.out);
    }
    unchanged("runs.txt is written");
    mkdirSync(results, { recursive: true });
    const top = [`# u4r-precheck-30: the -30 pre-check (u4r-brief.md 7.2), run in this container on ${date}; no verdict (no judged id)`];
    writeFileSync(join(results, "runs.txt"), `${[...top, ...head, `end: ${machineNow()}`, "", ...lines].join("\n")}\n`);
    console.log(`written: ${join("spikes", "netcode-proof", "results", "u4r-precheck-30", short)}/runs.txt`);
    return;
  }

  const settings = (tickStart) => ["--scene", "SS", "--profile", profile, "--tick-start", String(tickStart), "--warmup-s", String(WARMUP_S), "--measured-s", String(MEASURED_S), "--suite", name];
  for (const r of runs) {
    floor(r.what, r.bytes);
    const exe = r.exe === "lab" ? lab : labBites;
    const t = timed(exe, ["scenario", "--run", r.run, ...common, "--out", join(logs, r.run), "--seed", String(r.seed), ...settings(r.tickStart), ...r.extra]);
    const said = `${t.out} (wall ${t.secs} s)${overflowFlag(t.out, r.marks)}`;
    console.log(said);
    lines.push(said);
  }
  unchanged("the judge");
  const verdict = join(logs, "verdict.json");
  const judged = sh(judge, ["--marks", marksCopy, "--logs", logs, "--only", listedOf(profile).join(","), "--out", verdict]);
  console.log(judged);
  mkdirSync(results, { recursive: true });
  copyFileSync(verdict, join(results, "verdict.json"));
  const top = [
    `# ${name}: -17's, -57's${profile === "B1" ? " and -18's" : ""} L1 ${profile} part(s), run in this container on ${date} (u4r-brief.md 7.1)`,
    ...head,
    `end: ${machineNow()}`,
    `judge: ${judged}`,
    "",
    "# one line per run (npt-lab's own summary line and its wall time; the judge reads the logs, never these lines)",
  ];
  writeFileSync(join(results, "runs.txt"), `${[...top, ...lines].join("\n")}\n`);
  console.log(`written: ${join("spikes", "netcode-proof", "results", "u4r", short, profile)}/verdict.json and runs.txt`);
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    main(process.argv.slice(2));
  } catch (e) {
    console.error(`c-u4r-runs: ${e.message}`);
    process.exit(1);
  }
}
