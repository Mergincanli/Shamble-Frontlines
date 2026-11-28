#!/usr/bin/env node
// U4f's short pre-runs (u4f-brief.md 5.3; carry 86; U4R-Q4), on c-u4r-r2.mjs's model, re-made from a commit (the merge
// commit: r2's record rule, c-u4r-r2.mjs:2-4) before -20's, -21's and -47's judged sets:
//   node spikes/netcode-proof/ops/runs/c-u4f-r2.mjs --date YYYY-MM-DD
// - Each row of RUNS: `npt-lab scenario` (SS, 48 clients, seed 1, r2's 10 s warm-up and 20 s measured unless the row
//   names other lengths; a clock run holds one round of 48 connects), then `npt-lab contract --logs <its folder>
//   --series <SERIES.md's copy>`, then the judge in 5.4's form: `npt-judge --marks <marks.json's copy> --logs <its folder>
//   --only <the row's marks>`, where every mark that carries pre:PM-0001-2 (-20, -21) is named beside PM-0001-2
//   (P31's rule; ADV-CBH-1's lesson) and -47 alone. No twin runs here (TE31-2): a twin check reads a Gap, and the run is
//   read by name.
// - How a run is read (TE3-3): its verdict's checks by name, against the row's `expect`, which names the checks that
//   fail by length on these lengths (-47's 1,000 connects); r2.txt holds each command, its exit line and its output, and
//   the main session reads them (5.3's last paragraph: what comes to the advisor before its set).
// - Every command runs whatever the one before gave (a FAIL is a result, never a stop and never re-run: carry 86), each
//   through check.mjs's run() with a 600 s limit, in its own process group ("stopped at <s> s" is a result).
// - Its order, as c-u4r-r2.mjs's: the usage and git checks (no uncommitted change to a tracked file, no untracked file
//   under spikes/netcode-proof/crates: every log names its commit); the projection (ROW_BYTES a run, an estimate from
//   r2's 13 suites' 145 MB, about 10.6 MiB a run: 5.5); the release builds (npt-lab measured and bites, npt-judge), at
//   least 4 GiB free before each; the floor before each run; the output folder .gate/netcode-proof-test/u4f-r2-<commit12>/
//   (never overwritten): one folder per row, <key>-verdict.json beside it, the copies of marks.json and SERIES.md (their
//   sha256s in r2.txt's head), r2.txt; spikes/netcode-proof re-checked before r2.txt is written.
// - B6 holds 5.3's p5, p6, p10, p11, p12 and p21, B7 the clamp's p2, p8 and p15, B8 the lag switch's p4, p13, p18
//   and p19 and the fast clock's p3, p9, p16 and p20; U4f2 adds the forged suite's p1, p7, p14 and p17.
// - -21's rows (p1-p4, p7-p9, p13-p20) read as -21's 5.3 record only from U4f2's merge commit or a later one (carry
//   86; C-U4F2-4); from an earlier commit they are an early read of the suites built then (advisor-u4f-stop.md:90-94).
// - Each forged-suite row (p1, p7, p14, p17) also gets kind 7's line from its lab.jsonl (`kind7Line`: F-B-COUNT (a),
//   "report the number left out beside the check on every run"; FP-10). `--kind7 <run folder>...` prints that line for
//   each folder and runs nothing (a dry read).
// - Its helpers are exported (c-u4f-runs.mjs reads them); main runs only when this file is the entry module.

import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, readFileSync, statfsSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { builtArtifact, run } from "../../check.mjs";
import { roomFor, spikeChanged, spikeReading } from "./c-u4r-runs.mjs";

export const SPIKE = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
export const ROOT = join(SPIKE, "..", "..");
export const GATE = join(ROOT, ".gate", "netcode-proof-test");
const STEP_LIMIT_MS = 600_000;
/** One row's projected log bytes: r2's 145 MB over its 13 suites, about 11.2 MB a run, rounded up (an estimate). */
const ROW_BYTES = 12_000_000;
const SHORT = "--scene SS --seed 1 --warmup-s 10 --measured-s 20";
const P20 = `${SHORT} --profile P3 --controller 1 --brawl-press-every 2 --suite brawl`;
const CLOCK = `${SHORT} --profile P2 --suite clock --connects 48`;
const M20 = "PM-0001-2,PM-0001-20";
const M21 = "PM-0001-2,PM-0001-21";
/** -21's suites at P0 (U4F-Q11 (ii) (a)), the honest clients on the open-loop steady script under the controller. */
const P21 = `${SHORT} --profile P0 --controller 1 --behaviour steady`;
/** The lag switch at P1, slot 0 holding its `Input`s 200 ms every 6 s (5 bursts in the run's 30 s: F5 (a)). */
const LAG = `${SHORT} --profile P1 --controller 1 --behaviour steady --suite lag-switch`;
/** The forged suite at P0: the 11 forgers (slots 0 to 10) at the judged interval, 37 honest on the steady script. */
const FORGED = `${P21} --suite forged --forge-every 12`;
const FORGED_SAYS = "'forged: 1,000 commands of every kind' fails by length (about 75 a kind in 30 s) and the twin check reads a Gap (no twin in 5.3); 'forged: every kind of part 5's set sent (11 kinds)' passes";
/** The fast clock at P0, 70 s measured: one whole 60 s episode from the measured start and 10 s (5.3; 2.6's schedule). */
const FAST = "--scene SS --seed 1 --warmup-s 10 --profile P0 --controller 1 --behaviour steady --suite fast-clock";
const FAST_SAYS = "'fast clock: 10 runs of 60 s' fails by length (1 episode); the episode's excess, lead-bound and not-newer refusals reported (event.fast_clock)";
const CLAMP_SAYS ="'clamp: 1,000 points beyond reach per weapon kind the sim has' fails by length and the twin check reads a Gap (no twin in 5.3)";

/** 5.3's rows: `build` the npt-lab build (`lab` measured, `bites`), `args` the scenario's settings, `marks` its header's. */
export const RUNS = [
  { key: "p1", build: "lab", args: `${FORGED} --cmd-timing A`, marks: M21, expect: `every PM-0001-2 check and the shot band pass; 'forged: every forged command rejected and counted' passes (event.forged_kind, each kind's unrejected_count 0 and refusals_dropped_count 0); ${FORGED_SAYS}` },
  { key: "p7", build: "bites", args: `${FORGED} --cmd-timing A --role bite --bite PM-0001-21/no-lead-bound`, marks: M21, expect: `'forged: every forged command rejected and counted' FAILS (kind 1's items run; it passes on p1); ${FORGED_SAYS}` },
  { key: "p14", build: "lab", args: `${FORGED} --cmd-timing A`, marks: M21, expect: "p1 again at seed 1 (TE-24, D11): its lab log's fate_log sha256 and its room log's per-tick hashes equal p1's" },
  { key: "p17", build: "lab", args: `${FORGED} --cmd-timing B`, marks: M21, expect: "as p1, under option B (U4F-Q11 (iv))" },
  { key: "p2", build: "lab", args: `${P21} --cmd-timing A --suite clamp`, marks: M21, expect: `every PM-0001-2 check and the shot band pass; 'clamp: never refused or counted' passes; the clamp reruns 0 (event.clamp_kind.rerun_reports_count; SA-10); ${CLAMP_SAYS}` },
  { key: "p8", build: "bites", args: `${P21} --cmd-timing A --suite clamp --role bite --bite PM-0001-21/refuse-not-clamp`, marks: M21, expect: `'clamp: never refused or counted' FAILS (it passes on p2); ${CLAMP_SAYS}` },
  { key: "p15", build: "lab", args: `${P21} --cmd-timing B --suite clamp`, marks: M21, expect: "as p2, under option B (U4F-Q11 (iv))" },
  { key: "p4", build: "lab", args: `${LAG} --cmd-timing A`, marks: M21, expect: "every PM-0001-2 check and the shot band pass; 'lag switch: no tick moves further than the catch-up cap allows' passes; 'lag switch: 100 bursts' fails by length (its window is the whole run: 5 bursts in 30 s, F5 (a))" },
  { key: "p13", build: "bites", args: `${LAG} --cmd-timing A --role bite --bite PM-0001-21/no-catchup-cap`, marks: M21, expect: "'lag switch: no tick moves further than the catch-up cap allows' still PASSES (TE-7: it cannot fail at 200 ms); tick.v.lagswitch_cmds_count's max reported (at most 7 expected: D13)" },
  { key: "p18", build: "lab", args: `${LAG} --cmd-timing B`, marks: M21, expect: "as p4, under option B (U4F-Q11 (iv))" },
  { key: "p19", build: "lab", args: `${LAG} --cmd-timing A --lag-hold-ms 400`, marks: M21, expect: "unjudged (U4F-Q15 (c); C-U4F2-6): a 400 ms hold, reported with no fixed counts: the commands W drops late (late_count), the bucket's limited datagrams (pump_input_limited_count), the per-tick cap's kind-10 refusals under option A (buffer_full_count on slot 0) and tick.v.lagswitch_cmds_count's max (at most 8)" },
  { key: "p3", build: "lab", args: `${FAST} --measured-s 70 --cmd-timing A`, marks: M21, expect: `every PM-0001-2 check and the shot band pass; 'fast clock: at most the server's ticks plus the lead bound of commands' and 'fast clock: moves no further than the honest client plus that many ticks of movement' pass; ${FAST_SAYS}` },
  { key: "p9", build: "bites", args: `${FAST} --measured-s 70 --cmd-timing A --role bite --bite PM-0001-21/not-older-than`, marks: M21, expect: `'fast clock: at most the server's ticks plus the lead bound of commands' FAILS (excess above 0; it passes on p3), the movement check may fail too; ${FAST_SAYS}` },
  { key: "p16", build: "lab", args: `${FAST} --measured-s 70 --cmd-timing B`, marks: M21, expect: "as p3, under option B (U4F-Q11 (iv))" },
  { key: "p20", build: "lab", args: `${FAST} --measured-s 130 --cmd-timing A --fast-clock-idle-s 60`, marks: M21, expect: "unjudged (SA3-1; U4F-Q18 (a)): no command for the measured window's first 60 s, then one 60 s episode at 1.5x; reported: the two episodes' excess commands and movement (at most 0), the commands dropped late, the lead-bound refusals (F1's every 10th stamp past the bound closes the bank within 10 commands)" },
  { key: "p5", build: "lab", args: `${P20} --cmd-timing A`, marks: M20, expect: "every PM-0001-2 check and the shot band pass; -20's 'at least 99.9 % of commands reach the server before their tick runs' and 'missing inputs at most 0.1 % of player-ticks' pass by name (carry 86; TE3-1); the reorder rule holds (D3)" },
  { key: "p6", build: "lab", args: `${P20} --cmd-timing B`, marks: M20, expect: "as p5, under option B (TE-3)" },
  { key: "p10", build: "bites", args: `${CLOCK} --role bite --bite PM-0001-47/newest-sample`, marks: "PM-0001-47", expect: "'offset error at most 5 ms after 2 s of samples' FAILS; '1,000 connects' fails by length; it passes on p21" },
  { key: "p11", build: "bites", args: `${CLOCK} --role bite --bite PM-0001-47/tick-quantised`, marks: "PM-0001-47", expect: "as p10 (D4: about 11.9 ms against the mean's 2.35 ms)" },
  { key: "p12", build: "bites", args: `${P20} --cmd-timing B --role bite --bite PM-0001-20/redundancy-1`, marks: M20, expect: "'at least 99.9 % of commands reach the server before their tick runs' FAILS (89.5 % on U4f's early read: F5 (b)), and '0 presses lost' fails too, reached by the bite (F5 (c)), every PM-0001-2 check passing; it passes on p6" },
  { key: "p21", build: "lab", args: CLOCK, marks: "PM-0001-47", expect: "the p99 check passes (p10's and p11's clean partner); '1,000 connects' fails by length" },
];
// FP-8 (TE-U4F2-10): every -21 row also expects its summary's "in a building: 0" (u4f2-brief.md:110).
for (const r of RUNS) if (r.marks === M21) r.expect += "; the summary's 'in a building: 0 slot-ticks'";

export function sh(cmd, args, cwd = SPIKE, keepLead = false) {
  const r = spawnSync(cmd, args, { cwd, encoding: "utf8", maxBuffer: 1 << 28 });
  if (r.status !== 0) throw new Error(`${cmd} ${args.join(" ")}: exit ${r.status}\n${r.stdout}${r.stderr}`);
  return keepLead ? r.stdout.trimEnd() : r.stdout.trim();
}

export const sha256 = (path) => createHash("sha256").update(readFileSync(path)).digest("hex");

/** Kind 7's line for a forged-suite run (F-B-COUNT (a), Mustafa 2026-11-21: "report the number left out beside the
 * check on every run"; FP-10, TE-U4F2-R2-1): its `forged_kind` record's left-out, sent and unrejected counts, read from
 * the run's lab.jsonl; a twin has no such record and says so. */
export function kind7Line(dir) {
  const path = join(dir, "lab.jsonl");
  const recs = existsSync(path) ? readFileSync(path, "utf8").split("\n").filter(Boolean).map((l) => JSON.parse(l)) : [];
  const f = recs.find((r) => r.k === "event" && r.ev === "forged_kind" && r.f?.kind === 7)?.f;
  if (!f) return `${basename(dir)}: kind 7: no forged_kind record (a twin forges nothing; its twin check covers every press, the left-out ones included)`;
  return `${basename(dir)}: kind 7 beside 'every forged command rejected and counted': dead_left_out_count ${f.dead_left_out_count}, sent_count ${f.sent_count}, unrejected_count ${f.unrejected_count}`;
}

/** Stops unless free space minus `projected` bytes leaves the 4 GiB floor (section 4 rule 6). */
export function floor(what, projected = 0) {
  const s = statfsSync(ROOT);
  const r = roomFor(s.bavail * s.bsize, projected, what);
  if (!r.ok) throw new Error(r.line);
  return r.line;
}

/** One release build (-v --release --locked --offline), its artifact copied to `to`; cargo's npt-* lines printed. */
export function build(args, target, to) {
  floor(`the release build of ${target}${args.includes("bites") ? " (bites)" : ""}`);
  const cargoArgs = ["build", "-v", "--release", "--locked", "--offline", ...args, "--message-format=json"];
  const r = spawnSync("cargo", cargoArgs, { cwd: SPIKE, encoding: "utf8", maxBuffer: 1 << 28 });
  if (r.status !== 0) throw new Error(`cargo ${cargoArgs.join(" ")}: exit ${r.status}\n${r.stderr}`);
  for (const l of r.stderr.split("\n")) if (/^\s*(Compiling|Fresh) npt-/.test(l)) console.log(l);
  const path = builtArtifact(r.stdout.trim(), { target, kind: "bin" });
  if (!path) throw new Error(`cargo build ${args.join(" ")} names no ${target} executable`);
  copyFileSync(path, to);
  return to;
}

/** One command whatever it gives, bounded: its line, its exit line and its output. */
export function step(exe, args, limitMs = STEP_LIMIT_MS) {
  const t0 = Date.now();
  const r = run(exe, args, SPIKE, process.env, limitMs);
  const secs = ((Date.now() - t0) / 1000).toFixed(1);
  const exit = r.timedOut ? `stopped at ${secs} s` : `exit ${r.status ?? "by a signal"} (wall ${secs} s)`;
  return [`$ ${[exe, ...args].join(" ")}`, exit, r.out.trimEnd()];
}

/** The usage's git checks (every log names its commit): the commit and the spike's start reading. */
export function committed() {
  if (sh("git", ["status", "--porcelain", "--untracked-files=no"])) throw new Error("tracked files have uncommitted changes: commit first, since every log names its commit");
  const untracked = sh("git", ["status", "--porcelain", "--untracked-files=all", "--", "crates"]);
  if (untracked) throw new Error(`files under spikes/netcode-proof/crates are not committed (${untracked.split("\n").join("; ")})`);
  const commit = sh("git", ["rev-parse", "HEAD"]);
  const reading = () => spikeReading((args) => sh("git", args, ROOT, true));
  return { commit, reading, start: reading() };
}

function main(argv) {
  if (argv[0] === "--kind7" && argv.length > 1) {
    for (const d of argv.slice(1)) console.log(kind7Line(d));
    return;
  }
  const date = argv[0] === "--date" ? argv[1] : undefined;
  if (!/^\d{4}-\d{2}-\d{2}$/.test(date ?? "") || argv.length !== 2) throw new Error("usage: c-u4f-r2.mjs --date YYYY-MM-DD | --kind7 <run folder>...");
  const { commit, reading, start } = committed();
  const out = join(GATE, `u4f-r2-${commit.slice(0, 12)}`);
  if (existsSync(out)) throw new Error(`${out} exists: a run set is never overwritten`);
  const before = floor(`u4f-r2: the ${RUNS.length} runs' projected logs, before the builds`, RUNS.length * ROW_BYTES);
  const bin = join(GATE, "bin", `u4f-r2-${commit.slice(0, 12)}`);
  mkdirSync(bin, { recursive: true });
  const exe = {
    lab: build(["-p", "npt-lab"], "npt-lab", join(bin, "npt-lab")),
    bites: build(["-p", "npt-lab", "--features", "bites"], "npt-lab", join(bin, "npt-lab-bites")),
  };
  const judge = build(["-p", "npt-judge"], "npt-judge", join(bin, "npt-judge"));
  const after = floor("u4f-r2: after the builds", RUNS.length * ROW_BYTES);
  mkdirSync(out, { recursive: true });
  const copy = (f) => (copyFileSync(join(SPIKE, "marks", f), join(out, f)), join(out, f));
  const [marks, series] = [copy("marks.json"), copy("SERIES.md")];
  const lines = [
    `# U4f's 5.3 pre-runs from commit ${commit}, spikes/netcode-proof tree ${start.tree}, on ${date}`,
    "-21's rows (p1-p4, p7-p9, p13-p20) are -21's 5.3 record only from U4f2's merge commit or a later one (carry 86; C-U4F2-4); from an earlier commit, an early read of the suites built then (advisor-u4f-stop.md:90-94)",
    `npt-lab sha256 ${sha256(exe.lab)}; npt-lab (bites) sha256 ${sha256(exe.bites)}; npt-judge sha256 ${sha256(judge)}`,
    `marks.json sha256 ${sha256(marks)}, SERIES.md sha256 ${sha256(series)} (the copies contract and the judge read)`,
    `projection: ${before}`,
    `projection: ${after}`,
    "",
  ];
  for (const r of RUNS) {
    floor(`run ${r.key}`, ROW_BYTES);
    const logs = join(out, r.key);
    const said = [
      `## ${r.key} (the ${r.build === "lab" ? "measured" : "bites"} build); expected: ${r.expect}`,
      ...step(exe[r.build], ["scenario", "--run", `u4f-r2-${r.key}-1`, "--set", "u4f-r2", "--date", date, "--commit", commit, "--out", logs, ...r.args.split(" "), "--marks", r.marks]),
      ...step(exe[r.build], ["contract", "--logs", logs, "--series", series]),
      ...step(judge, ["--marks", marks, "--logs", logs, "--only", r.marks, "--out", join(out, `${r.key}-verdict.json`)]),
      ...(r.args.includes("--suite forged") ? [kind7Line(logs)] : []),
      "",
    ];
    console.log(said.join("\n"));
    lines.push(...said);
  }
  const why = spikeChanged(start, reading());
  if (why) throw new Error(`spikes/netcode-proof changed since the set started (${why}): stopped before r2.txt is written`);
  writeFileSync(join(out, "r2.txt"), `${lines.join("\n")}\n`);
  console.log(`written: ${join(".gate", "netcode-proof-test", `u4f-r2-${commit.slice(0, 12)}`, "r2.txt")}`);
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    main(process.argv.slice(2));
  } catch (e) {
    console.error(`c-u4f-r2: ${e.message}`);
    process.exit(1);
  }
}
