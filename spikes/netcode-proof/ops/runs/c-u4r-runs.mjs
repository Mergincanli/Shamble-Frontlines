#!/usr/bin/env node
// U4r's judged runs and the -30 pre-check, in this container (u4r-brief.md 2.3 and section 7; c-u2-pm-0001-3.mjs's
// model). One script for every set, the set named first:
//   node spikes/netcode-proof/ops/runs/c-u4r-runs.mjs precheck --date YYYY-MM-DD
//   node spikes/netcode-proof/ops/runs/c-u4r-runs.mjs judged --profile P0|P2|P3|B1 --date YYYY-MM-DD
// - How it is run (advisor-u4r-r1, FP-5 (d): INF-U4R-8; advisor-u4r-f1-plan, Q5): in the main folder, with
//   CARGO_TARGET_DIR unset (plan §7.1 item 6: the judged runs build in the main folder's own cache), not through
//   main-run.sh, which sets CARGO_TARGET_DIR: this script refuses it, since a shared-target run comes to the advisor
//   first (u4r-brief.md section 12 item 8). The set runs under the lane lock, exclusive, from the repository root:
//   `flock -x .gate/shared-target/.lane.lock node spikes/netcode-proof/ops/runs/c-u4r-runs.mjs …`. If this script dies
//   while a call runs, the lane lock is freed at once (no npt-lab or npt-judge call holds it: advisor-u4r-f1-head,
//   INF-F1-1, measured), while the call, in its own session, runs on with no limit: to stop a set, end the call's group
//   first (`kill -9 -- -<pid>`, the pid from `pgrep -a npt-lab; pgrep -af npt-judge`), then the script. In a set with
//   bites (P0), ending the measured judge call's group starts the bite-runs call at once: pgrep again and end that
//   call's group too; the script then writes runs.txt and exits 1 on its own. Kill the script only when no npt-lab or
//   npt-judge call is left (U4r-F2's review, INF-F2-1). Each npt-lab call and the judge are bounded at RUN_LIMIT_MS
//   (advisor-u4r-r2, INF-R2-4); the builds are not.
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
//   marks (marks.json's lists, read from the set's copy) whose part has this profile: -17's and -57's on P0, and -17's
//   whole-as-lost and link-lost-overstated on P2 (C62H-BOUND; link-lost-overstated never on P0), each header listing
//   PM-0001-2 and the bite's own mark; then `npt-judge --only` on the set's folder. One profile per call keeps each
//   set's logs and the judge's memory to one profile's (the advisor's U4r build ruling, point 3: the main session plans
//   the disk and judges each set's folder apart). A run whose summary line counts an overflow resync above 0 and whose
//   header lists -17 or -18 is marked on its runs.txt line: it goes to the advisor before it counts (carry 58; FP-5
//   (g)).
// - A set with bites (P0 and P2: bitesOf) is judged in two calls (U4r-F2: advisor-u4r-sets-r1, Q2;
//   advisor-u4r-f2-plan): its measured runs write under <logs>/measured/<run> and its bite runs under
//   <logs>/bites/<run>; npt-judge reads measured/ into verdict.json, then bites/ into verdict-bites.json, each call at
//   most 10 GiB of judge memory where one call over the whole folder projected about 11.5 GiB before the set (the
//   four-point fit at F 1.504); at P0's measured bytes (3,248.3 MiB of logs), one call projects about 7.7 GiB by the
//   six-point fit (8.2 GiB by the four-point fit: advisor-u4r-sets-r2). No run's verdict reads another run for -2's,
//   -17's and -57's checks (no against threshold, no ws-practice rule, figures that read one run); a mark that compares
//   runs (a twin figure) needs one call. Both calls run whatever the first's end; a re-check stop before either call
//   ends the set there, with no results/. Each call runs after its own re-check; results/ is made only after both, each
//   file copied on its own exit 0. A set without bites keeps one call and its <logs>/<run> layout.
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
//   projection again; marks.json copied into the bin folder and its sha256 checked there; the first run's floor; only
//   then the set's folders (advisor-u4r-r2, INF-R2-3). The binaries go to .gate/netcode-proof-test/bin/<set>-<commit12>/,
//   which no exists-check reads, so a stop before the first run leaves nothing that blocks a re-run at that commit.
// - The projection (FP-5 (b), (c): INF-U4R-3): each run's log bytes from U4r's judged sets' largest measured runs
//   (BYTES_10MIN_57, BYTES_10MIN_OTHER: RATES-1, advisor-u4r-sets-r1) and the builder's PAIR_BYTES, the set's total
//   checked before the builds and after them: it starts only when free space minus the total is at least 4 GiB,
//   printing both; before each run, free space minus that run's projected bytes must be at least 4 GiB, else it stops
//   before that run.
// - The judge's projected peak (the advisor's P0 ruling, next step 3; advisor-u4r-f3-plan, condition 4): 302.2 +
//   2.3403 x the MiB of logs a call reads (JUDGE_FIT, the six-point fit over U4r's judge measurements apart:
//   advisor-u4r-sets-r2, residuals -90 to +70 MiB). Right after the disk projection before the builds, one line per
//   judge call from the plan's bytes for that call's runs; the set stops there, with nothing made, if any call projects
//   over the 10 GiB line (JUDGE_LINE_MIB: advisor-u4r-sets-r1, Q2). Before each call, after its re-check, the same line
//   from the bytes on disk, which only prints. The pre-check has no judge and prints none.
// - Under each judge call that exits 0, runs.txt prints each part and bite that holds a run of that call, with its
//   verdict or status, each run's validity and the part's why lines, copied from the verdict file (setParts; RUNS-LINE:
//   advisor-u4r-sets-r1): the judge's own summary gives each mark's overall verdict, which can hide the set's part.
//   Nothing is judged there. These lines and the on-disk projection go through one guard, so they can add a line
//   naming an error but never stop runs.txt being written (advisor-u4r-f3-plan, condition 3).
// - Raw logs go under .gate/netcode-proof-test/runs/<set>-<commit12>/ (one folder per run), results under
//   results/u4r/<commit12>/<profile>/ or results/u4r-precheck-30/<commit12>/; neither is ever overwritten. The bin
//   folder's copy of marks/marks.json (its sha256 in runs.txt's head) gives the bite list, and the same bytes, copied into
//   the raw-log folder, are the judge's. Before the judge (and the pre-check's runs.txt), spikes/netcode-proof's tree
//   hash and tracked status, read from the repository root, are checked against their start values, and the set stops
//   if either changed (FP-5 (e): INF-U4R-9; scoped by advisor-u4r-r2, INF-R2-1: a commit elsewhere, such as a
//   .checkpoints/ save, stops nothing; a commit under spikes/netcode-proof, an earlier set's results/ included, does).
//   runs.txt's head names the commit and the spike's tree, the target (the main folder's own cache) and the uptime and
//   load average at the start and at the end.
// - A stopped call (RUN_LIMIT_MS) is a result, never retried (condition 5): a judged run's or a pair's line says
//   "stopped at <s> s" and the set goes on; a stopped recording stops the set before the pairs; when the judge stops or
//   exits non-zero, runs.txt is written with the run lines and no verdict from that call, and the script exits 1
//   (advisor-u4r-f1-plan, condition 1).
// - Both npt-lab builds and npt-judge are built -v --release --locked --offline, each artifact's path read from cargo's
//   --message-format=json messages (check.mjs's builtArtifact, as the wasm step reads its builds) and copied out before
//   the next build replaces it; cargo's own "Compiling" or "Fresh" lines for the npt-* crates go to the runner log
//   (advisor-u4r-r2run, RUN-L1; -v makes cargo print Fresh: advisor-u4r-f1-plan, Q6).
// - Each npt-lab process is timed (the wall time around the call) and its seconds go on its runs.txt line beside its
//   ticks (npt-lab's summary line), for the ticks-per-second figure (2.4 r5, 7.3).
// - No run logs its clients' bytes (`scenario --bytes`): no part these sets judge reads a replay log (only an x86_64
//   replay log of an L2 Arm run is read: judge/src/figures.rs:222-233).
// - Its pure helpers are exported (the projection, the judge's projected peak, the name check, the summary line's flag,
//   the pairs' check, the verdict file's parts lines, and the spike's re-check, which c-u4r-r2.mjs imports); main runs
//   only when this file is the entry module.

import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, readdirSync, readFileSync, statfsSync, statSync, writeFileSync } from "node:fs";
import { loadavg, uptime } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { builtArtifact, run } from "../../check.mjs";

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
 * from U4r's judged sets (RATES-1: advisor-u4r-sets-r1; advisor-u4r-f3-plan, condition 7): the largest whole run of
 * each kind, du -sb of its kept folder on 2026-11-14, rounded up to 0.1 MB (MB = 10^6 B). The five run folders:
 * .gate/netcode-proof-test/runs/u4r-P3-0f066338dd8d, u4r-B1-64f3307d5c7f, u4r-P2-ab10f058a828,
 * u4r-P0-051995db9cea/measured and u4r-P0-051995db9cea/bites. A run whose header lists PM-0001-57 writes -57's three
 * series (the advisor's U4r build ruling, point 3), which grow through a run, so the builder's rates from 20 to 30 s
 * runs understated them:
 * - P0 603.7 MB: the bite run pos-4cm, the largest of P0's six -57 runs, which also sets the rate for P0's measured
 *   runs (514.0 to 520.3 MB; the other two -57 bites 518.8 and 567.5 MB);
 * - P2 272.9 MB (its runs 271.2 to 272.8 MB), P3 183.3 MB (178.9 to 183.3), B1 291.4 MB (283.1 to 291.3).
 * Any other run: 56 MB, the builder's figure (about 3,050 B a measured tick), kept: P0's three -17 bite runs, the only
 * such runs, came to 55.6 to 55.84 MB. The cross-check: each set's measured bytes over its old projection were P3 1.44,
 * B1 1.504, P2 1.238 and P0 1.057, and at these rates each set's projection covers its measured bytes.
 */
export const BYTES_10MIN_57 = { P0: 603_700_000, P2: 272_900_000, P3: 183_300_000, B1: 291_400_000 };
export const BYTES_10MIN_OTHER = 56_000_000;
/**
 * One -30 pre-check pair's output (its lines, kept for runs.txt), at most 25,164 B in the builder's pre-check runs
 * (.gate/u4r/c4-b12-3.log:18-365: the clean, wallclock and late-command pairs, 24,840, 25,164 and 25,093 B); the
 * recording itself is its manifest entry's bytes.
 */
export const PAIR_BYTES = 25_164;
/** The late-command pair's one expected result line (precheck.rs, the late-command bite's test). */
export const LATE_COMMAND_LINE = "result: VOID (L1 late_count 1)";
/**
 * The one fixed line above a set's two judge blocks in runs.txt when the set has bites (U4r-F2: advisor-u4r-f2-plan,
 * condition 3 (b); its place: advisor-u4r-f3-plan, condition 5); it never parses the judge's output.
 */
export const SPLIT_LINE = "the set is judged in two calls, each at most 10 GiB (U4r-F2); verdict.json holds the measured runs' verdicts (read the set's parts; its bites read 'no run yet' by design); verdict-bites.json holds the bite runs' verdicts (read its bites; every part reads '0 of n valid runs' by design)";
/**
 * Each npt-lab call's and the judge's limit, ms (advisor-u4r-r2, INF-R2-4): about 12 times r5's slowest judged run
 * (18,300 ticks at 60.1 ticks/s, about 304 s); re-checked against the judge's measured time before the P0 set.
 */
export const RUN_LIMIT_MS = 3_600_000;
/**
 * The judge's projected peak memory, MiB, from the MiB of logs a call reads: a + b x MiB (advisor-u4r-sets-r2, RR3-L6's
 * figures: the six-point fit over U4r's judge measurements apart, peak.py's ru_maxrss; residuals -90 to +70 MiB).
 */
export const JUDGE_FIT = { a: 302.2, b: 2.3403 };
/** The judge's memory line, MiB (advisor-u4r-sets-r1, Q2: a call projected over 10 GiB waits for the advisor). */
export const JUDGE_LINE_MIB = 10_240;

/**
 * What changed in spikes/netcode-proof since the set started (advisor-u4r-r2, INF-R2-1): its tree hash and its tracked
 * status (spikeReading's two readings), "" when both are equal. A commit or an edit elsewhere (a .checkpoints/ save)
 * changes neither; a commit under spikes/netcode-proof changes the tree, an uncommitted edit there the status.
 */
export function spikeChanged(start, now) {
  const why = [];
  if (now.tree !== start.tree) why.push(`tree ${start.tree.slice(0, 12)} is now ${now.tree.slice(0, 12)}`);
  if (now.spike !== start.spike) why.push(`its tracked status now reads: ${now.spike ? now.spike.split("\n").join("; ") : "clean"}`);
  return why.join("; ");
}

/** spikeChanged's two readings, through `git(args)`: a git call that must run from the repository root (the pathspec). */
export const spikeReading = (git) => ({
  tree: git(["rev-parse", "HEAD:spikes/netcode-proof"]),
  spike: git(["status", "--porcelain", "--untracked-files=no", "--", "spikes/netcode-proof"]),
});

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

/** The judge's projected peak, MiB, for a call that reads `bytes` of logs (JUDGE_FIT). */
export const judgePeakMiB = (bytes) => JUDGE_FIT.a + JUDGE_FIT.b * (bytes / 1024 ** 2);

/**
 * Whether a judge call that reads `bytes` of logs stays at or under JUDGE_LINE_MIB, with the line that names the call,
 * its log MiB, the formula, the projected peak and the line (advisor-u4r-f3-plan, condition 4). Over the line, the
 * start check stops the set before the builds; the line printed before a call from its logs on disk (`onDisk`) only
 * prints.
 */
export function judgeRoom(bytes, what, onDisk = false) {
  const peak = judgePeakMiB(bytes);
  const ok = peak <= JUDGE_LINE_MIB;
  const over = onDisk ? "over the line, printed only (advisor-u4r-sets-r1, Q2: to the advisor)" : "over the line: stopped before the builds (advisor-u4r-sets-r1, Q2: to the advisor)";
  return {
    ok,
    line: `${what}: ${(bytes / 1024 ** 2).toFixed(1)} MiB of logs, so a projected peak of ${peak.toFixed(0)} MiB by ${JUDGE_FIT.a} + ${JUDGE_FIT.b} x MiB (the six-point fit: advisor-u4r-sets-r2) against the ${JUDGE_LINE_MIB} MiB line: ${ok ? "room" : over}`,
  };
}

/** The bytes of every file under `dir`, at any depth (the judge reads its .jsonl files at any depth). */
export function logBytes(dir) {
  let sum = 0;
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, e.name);
    if (e.isDirectory()) sum += logBytes(p);
    else if (e.isFile()) sum += statSync(p).size;
  }
  return sum;
}

/**
 * The set's own parts and bites in a parsed verdict file (RUNS-LINE: advisor-u4r-sets-r1): each part and each bite that
 * holds a run of this call, with its verdict or status, each run's validity and the part's why lines, all copied from
 * the file's fields; nothing is judged here. The file must be a list of marks (anything else throws, for the caller's
 * guard to name); a missing parts, bites, runs or why array reads as empty (advisor-u4r-f3-plan, condition 3).
 */
export function setParts(verdict) {
  if (!Array.isArray(verdict)) throw new Error(`the file holds ${verdict === null ? "null" : typeof verdict}, not a list of marks`);
  const list = (x) => (Array.isArray(x) ? x : []);
  const validity = (r) => (r?.valid === true ? "valid" : r?.valid === false ? "invalid" : `valid ${JSON.stringify(r?.valid) ?? "missing"}`);
  const runsOf = (runs) => list(runs).map((r) => `${r?.run} ${validity(r)}`).join(", ");
  const out = [];
  for (const m of verdict) {
    for (const p of list(m?.parts)) {
      if (!list(p?.runs).length) continue;
      out.push(`  ${m.mark} ${p.name}: ${p.verdict} (${runsOf(p.runs)})`);
      for (const w of list(p.why)) out.push(`    ${w}`);
    }
    for (const b of list(m?.bites)) if (list(b?.runs).length) out.push(`  ${b.id}: ${b.status} (${runsOf(b.runs)})`);
  }
  return out;
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
 * The bites a judged set runs, from marks.json's lists: one run each at seed 1 on the profiles its part reads. -17's
 * whole-as-lost runs on P0 and P2 and link-lost-overstated on P2 only, never P0 (C62H-BOUND); every other -17 and -57
 * bite on P0, as before; -18's none at U4r (ruling 3, point 1: to U4g), so a B1 set runs no bite.
 */
export function bitesOf(profile, marks) {
  const listed = listedOf(profile);
  const on = { "PM-0001-17/whole-as-lost": ["P0", "P2"], "PM-0001-17/link-lost-overstated": ["P2"] };
  const bitesOn = (mark, bite) => (mark === "PM-0001-18" ? [] : (on[bite] ?? ["P0"]));
  const out = [];
  for (const m of marks.marks.filter((x) => listed.includes(x.id) && x.id !== "PM-0001-2")) {
    for (const b of (m.bites ?? []).filter((x) => !L2_ONLY.includes(x.id) && !NOT_AT_U4R.includes(x.id) && bitesOn(m.id, x.id).includes(profile))) out.push({ id: b.id, mark: m.id });
  }
  return out;
}

/**
 * The judged set's runs, in order, each with its projected bytes (the module text; FP-5 (b)) and its group, measured or
 * bites (a set with bites is judged in two calls: U4r-F2).
 */
export function planJudged(profile, marks) {
  const listed = listedOf(profile);
  const runs = SEEDS.map((s) => ({ run: `l1-${profile.toLowerCase()}-s${s}`, exe: "lab", seed: s, marks: listed, extra: ["--role", "measured", "--marks", listed.join(",")], tickStart: TICK_START, group: "measured" }));
  for (const b of bitesOf(profile, marks)) {
    const marksOf = ["PM-0001-2", b.mark];
    runs.push({ run: `bite-${b.id.split("/")[1]}`, exe: "labBites", seed: 1, marks: marksOf, extra: ["--role", "bite", "--bite", b.id, "--marks", marksOf.join(",")], tickStart: b.id === TICK16 ? TICK16_START : TICK_START, group: "bites" });
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

function sh(cmd, args, cwd = ROOT, keepLead = false) {
  const r = spawnSync(cmd, args, { cwd, encoding: "utf8", maxBuffer: 1 << 28 });
  if (r.status !== 0) throw new Error(`${cmd} ${args.join(" ")}: exit ${r.status}\n${r.stdout}${r.stderr}`);
  // keepLead: the re-check's readings keep the status's first column (INF-F1-5: " M" is an unstaged change).
  return keepLead ? r.stdout.trimEnd() : r.stdout.trim();
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

/**
 * One release build, at least 4 GiB free before it, its artifact's path read from cargo's JSON messages and copied to
 * `to`; cargo's "Compiling" or "Fresh" lines for the npt-* crates are printed as cargo wrote them (RUN-L1).
 */
function build(args, target, to) {
  floor(`the release build of ${target}${args.includes("bites") ? " (bites)" : ""}`);
  const cargoArgs = ["build", "-v", "--release", "--locked", "--offline", ...args, "--message-format=json"];
  const r = spawnSync("cargo", cargoArgs, { cwd: SPIKE, encoding: "utf8", maxBuffer: 1 << 28 });
  if (r.status !== 0) throw new Error(`cargo ${cargoArgs.join(" ")}: exit ${r.status}\n${r.stdout}${r.stderr}`);
  for (const l of r.stderr.split("\n")) if (/^\s*(Compiling|Fresh) npt-/.test(l)) console.log(l);
  const path = builtArtifact(r.stdout.trim(), { target, kind: "bin" });
  if (!path) throw new Error(`cargo build ${args.join(" ")} names no ${target} executable in its JSON messages`);
  copyFileSync(path, to);
  return to;
}

/**
 * One npt-lab process, timed and bounded (INF-R2-4): through check.mjs's run(), in its own process group, killed whole
 * at RUN_LIMIT_MS; its summary line (stdout, as sh() gave it), the wall seconds and whether it was stopped. A non-zero
 * exit that is not a stop throws sh()'s message: the tool's own refusal (advisor-u4r-f1-plan, Q2).
 */
function timed(exe, args) {
  const t0 = Date.now();
  const r = run(exe, args, ROOT, process.env, RUN_LIMIT_MS);
  const secs = ((Date.now() - t0) / 1000).toFixed(1);
  if (!r.timedOut && r.status !== 0) throw new Error(`${exe} ${args.join(" ")}: exit ${r.status}\n${r.out}`);
  return { out: r.stdout.trim(), secs, stopped: r.timedOut };
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
  const reading = () => spikeReading((args) => sh("git", args, ROOT, true));
  const start = { commit, ...reading(), machine: machineNow() };
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
  // The judge's projected peak for each call, from the plan's bytes for that call's runs (advisor-u4r-f3-plan,
  // condition 4): the set stops here, before the builds and with nothing made, if any call projects over the line.
  const judgeGroups = set !== "judged" ? [] : plan.some((r) => r.group === "bites") ? [["the judge (measured runs)", "measured"], ["the judge (bite runs)", "bites"]] : [["the judge", null]];
  const judgeAhead = judgeGroups.map(([what, group]) => judgeRoom(projectedBytes(plan.filter((r) => group === null || r.group === group)), `${name}: ${what}, from the plan's bytes before the builds`));
  for (const j of judgeAhead) console.log(j.line);
  const overLine = judgeAhead.find((j) => !j.ok);
  if (overLine) throw new Error(overLine.line);

  // Both builds of npt-lab, each copied out before the next build replaces it, and the judge.
  const bin = join(GATE, "bin", `${name}-${short}`);
  mkdirSync(bin, { recursive: true });
  const lab = build(["-p", "npt-lab"], "npt-lab", join(bin, "npt-lab"));
  const labBites = build(["-p", "npt-lab", "--features", "bites"], "npt-lab", join(bin, "npt-lab-bites"));
  const judge = set === "judged" ? build(["-p", "npt-judge"], "npt-judge", join(bin, "npt-judge")) : null;
  const after = room("after the builds");

  // marks.json into the bin folder, checked there, and the bite list read from it; the first run's floor; only then the
  // set's folder, with the same bytes as the judge's copy (FP-5 (e); INF-R2-3: a stop before the first run leaves no
  // set folder).
  const binMarks = join(bin, "marks.json");
  copyFileSync(join(SPIKE, "marks", "marks.json"), binMarks);
  if (sha256(binMarks) !== marksSha) throw new Error("marks/marks.json changed since the set started: stopped before the set's folder");
  const runs = planOf(JSON.parse(readFileSync(binMarks, "utf8")));
  const split = runs.some((r) => r.group === "bites");
  floor(runs[0].what, runs[0].bytes);
  mkdirSync(logs, { recursive: true });
  const marksCopy = join(logs, "marks.json");
  copyFileSync(binMarks, marksCopy);
  const head = [
    `commit ${commit}, spikes/netcode-proof tree ${start.tree}`,
    "target: the main folder's own cache (CARGO_TARGET_DIR unset: plan §7.1 item 6)",
    `npt-lab (measured build) sha256 ${sha256(lab)}`,
    `npt-lab (bites build) sha256 ${sha256(labBites)}`,
    ...(judge ? [`npt-judge sha256 ${sha256(judge)}`] : []),
    `marks.json sha256 ${marksSha} (copied into the bin folder and checked there, then into the raw-log folder; the bite list and the judge read those copies)`,
    `raw logs: .gate/netcode-proof-test/runs/${name}-${short}/ (${split ? "measured/ and bites/; " : ""}not committed: plan §7.2)`,
    `projection: ${before}`,
    `projection: ${after}`,
    ...judgeAhead.map((j) => `projection: ${j.line}`),
    `start: ${start.machine}`,
  ];
  const common = ["--set", name, "--date", date, "--commit", commit];
  const lines = [];
  const unchanged = (what) => {
    const why = spikeChanged(start, reading());
    if (why) throw new Error(`spikes/netcode-proof changed since the set started (${why}): stopped before ${what} (INF-R2-1)`);
  };

  if (set === "precheck") {
    const rec = join(logs, `${entry.name}.rec`);
    floor(runs[0].what, runs[0].bytes);
    const made = timed(lab, [
      "scenario", "--run", `record-${entry.name}`, ...common, "--out", join(logs, `record-${entry.name}`), "--scene", SCENES[entry.scene],
      "--profile", entry.profile, "--seed", String(entry.seed), "--clients", String(entry.clients), "--tick-start", String(entry.first_tick),
      "--warmup-s", String(entry.warmup_s), "--measured-s", String(entry.measured_s), "--logs", "0", "--suite", "record", "--record", rec,
    ]);
    if (made.stopped) throw new Error(`${entry.name}'s recording run stopped at ${made.secs} s: the set stops before the pre-check (INF-R2-4; as carry 40's stop)`);
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
      const expected = p.pair === "late-command" ? `${p.expected}, the whole line "${LATE_COMMAND_LINE}"` : p.expected;
      if (r.stopped) {
        const said = `## pair ${p.pair}: stopped at ${r.secs} s, expected ${expected} (NOT as expected: to the advisor)`;
        console.log(said);
        lines.push(said);
        continue;
      }
      const result = r.out.match(/^result: (\S+)/m)?.[1] ?? "none";
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
    const t = timed(exe, ["scenario", "--run", r.run, ...common, "--out", join(logs, split ? r.group : "", r.run), "--seed", String(r.seed), ...settings(r.tickStart), ...r.extra]);
    const said = t.stopped ? `${r.run}: stopped at ${t.secs} s (a result, never retried: condition 5)` : `${t.out} (wall ${t.secs} s)${overflowFlag(t.out, r.marks)}`;
    console.log(said);
    lines.push(said);
  }
  // The judge, bounded too (INF-R2-4). Whatever its end, runs.txt is written; a verdict file only on its call's exit 0:
  // a stopped run's log is cut (written through a buffer flushed at its footer), and the judge refuses a cut log
  // (advisor-u4r-f1-plan, condition 1). A set with bites is judged in two calls, measured/ then bites/, each after its
  // own re-check; both calls run whatever the first's end; a re-check stop before either call ends the set there, with
  // no results/; and results/ is made only after both (U4r-F2: advisor-u4r-f2-plan, conditions 2, 3). Each call's block
  // in runs.txt: its projection from its logs on disk, its judge line, then (on exit 0 only) its parts and bites
  // (advisor-u4r-f3-plan, condition 5).
  const judgeCall = (label, what, dir, file) => {
    unchanged(what);
    const out = join(logs, file);
    const block = [];
    // U4r-F3's lines go through this one guard: an error adds one line naming it, and nothing here can throw out of
    // judgeCall or stop runs.txt being written (advisor-u4r-f3-plan, condition 3).
    const guarded = (heading, make) => {
      try {
        for (const l of make()) {
          console.log(l);
          block.push(l);
        }
      } catch (e) {
        const l = `${heading}: not printed (${e?.message ?? e})`;
        console.log(l);
        block.push(l);
      }
    };
    guarded(`${label}, projected from its logs on disk`, () => [judgeRoom(logBytes(dir), `${label}, projected from its logs on disk`, true).line]);
    const t0 = Date.now();
    const j = run(judge, ["--marks", marksCopy, "--logs", dir, "--only", listedOf(profile).join(","), "--out", out], ROOT, process.env, RUN_LIMIT_MS);
    const secs = ((Date.now() - t0) / 1000).toFixed(1);
    const ok = !j.timedOut && j.status === 0;
    const line = ok ? j.stdout.trim() : j.timedOut ? `stopped at ${secs} s (no verdict)` : `exit ${j.status ?? "by a signal"} (no verdict): ${j.out.trim()}`;
    console.log(line);
    block.push(`${label}: ${line}`);
    if (ok) {
      guarded(`${label}, the set's parts and bites (${file})`, () => [
        `${label}, the set's parts and bites (${file}: each part and bite that holds a run of this call, as the file gives them):`,
        ...setParts(JSON.parse(readFileSync(out, "utf8"))),
      ]);
    }
    return { what, out, file, ok, line, block, end: j.timedOut ? `stopped at ${secs} s` : `exited ${j.status ?? "by a signal"}` };
  };
  const calls = split
    ? [judgeCall("judge (measured runs)", "the judge (measured runs)", join(logs, "measured"), "verdict.json"), judgeCall("judge (bite runs)", "the judge (bite runs)", join(logs, "bites"), "verdict-bites.json")]
    : [judgeCall("judge", "the judge", logs, "verdict.json")];
  mkdirSync(results, { recursive: true });
  for (const c of calls) if (c.ok) copyFileSync(c.out, join(results, c.file));
  const judgeLines = split ? [SPLIT_LINE, ...calls[0].block, ...calls[1].block] : calls[0].block;
  const top = [
    `# ${name}: -17's, -57's${profile === "B1" ? " and -18's" : ""} L1 ${profile} part(s), run in this container on ${date} (u4r-brief.md 7.1)`,
    ...head,
    `end: ${machineNow()}`,
    ...judgeLines,
    "",
    "# one line per run (npt-lab's own summary line and its wall time; the judge reads the logs, never these lines)",
  ];
  writeFileSync(join(results, "runs.txt"), `${[...top, ...lines].join("\n")}\n`);
  const failed = calls.filter((c) => !c.ok);
  if (failed.length && !split) throw new Error(`the judge ${failed[0].end}: runs.txt written with no verdict (to the advisor)`);
  if (failed.length) throw new Error(`${failed.map((c) => `${c.what} ${c.end}`).join("; ")}: runs.txt written, with no verdict from ${failed.length === 1 ? "that call" : "those calls"} (to the advisor)`);
  console.log(`written: ${join("spikes", "netcode-proof", "results", "u4r", short, profile)}/${split ? "verdict.json, verdict-bites.json" : "verdict.json"} and runs.txt`);
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    main(process.argv.slice(2));
  } catch (e) {
    console.error(`c-u4r-runs: ${e.message}`);
    process.exit(1);
  }
}
