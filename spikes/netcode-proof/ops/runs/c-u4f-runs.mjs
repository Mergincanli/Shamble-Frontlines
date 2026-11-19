#!/usr/bin/env node
// U4f's judged sets (u4f-brief.md 5.1, 5.4; U4F-Q2 (a)), on c-u4r-runs.mjs's model, one set per call:
//   node spikes/netcode-proof/ops/runs/c-u4f-runs.mjs 47|22|20 --date YYYY-MM-DD
// - How it is run (c-u4r-runs.mjs:6-17; 5.5 item 2; section 12 item 9): in the main folder, with CARGO_TARGET_DIR unset
//   (the judged runs build in the main folder's own cache; a set run with it set stops here), under the lane lock,
//   exclusive, from the repository root: `flock -x .gate/shared-target/.lane.lock node
//   spikes/netcode-proof/ops/runs/c-u4f-runs.mjs …`; one set at a time, in R2's order (advisor-u4f-stop.md:126-127:
//   -47, -20, -22, then -21's), after D2's reading; each set's projection against the 4 GiB floor first, and before each
//   run.
// - The sets (5.1; marks.json's parts): 47: L1 P2, suite clock, 1,000 connects (a 10 s warm-up and 120 s measured: about
//   21 rounds of 48), seeds 1 to 3, headers listing PM-0001-47; its bites newest-sample and tick-quantised at seed 1.
//   22: SS, 48 clients, P0 (U4F-Q11 (ii) (a)), U4f's controller (U4F-Q7 (i); NC-U4F-2), suite never-ack (slot 0 the
//   never-acker), 600 s measured, seeds 1 to 3, headers listing PM-0001-2 and PM-0001-22 (PM-0001-2 for 5.2's load
//   report only); its bite PM-0001-22/unbounded over a full S30 (1,800 s measured: marks.json:5218-5223), under the
//   controller too. 20: SS, 48, P3, U4f's controller, the brawl with
//   --brawl-press-every 2, 600 s measured, seeds 1 to 3 under each option (`--cmd-timing`), headers listing PM-0001-2 and
//   PM-0001-20; its bite redundancy-1 under option B (U4F-Q10 (a)) at seed 1. Every run at the room's event cap (2.8's
//   value, once B3 set it) and the frozen lead bound (LEAD-BOUND: the header's lead_bound_ms).
// - Each judge call in 5.4's form (P31's rule; ADV-CBH-1's lesson: a mark carrying pre:PM-0001-2 is never judged without
//   it): 20 `--only PM-0001-2,PM-0001-20`; 22 `--only PM-0001-22`, then 5.2's load report, `--only PM-0001-2` over the
//   same measured folder, its verdict never counted (the designed drop leaves 47 clients: marks.json:5227); 47 `--only
//   PM-0001-47`. A set with bites is judged in two calls, measured/ then bites/ (U4r-F2's split), and STOP_NOTE says so
//   at the head of runs.txt (TE-CBH-5 = NA-CBB-1's lesson: every set judged in two calls is named).
// - ADV-CBH-2's hardenings: npt-lab (both builds) and npt-judge built in this call; marks.json's and npt-judge's sha256s
//   read when the set starts and checked again before each judge call (a difference stops the set there); `--commit` the
//   full HEAD, with `git diff --quiet <HEAD> HEAD -- spikes/netcode-proof ':(exclude)spikes/netcode-proof/results'`
//   exiting 0 before each judge call (spikeChanged's re-check: c-u4r-runs.mjs, INF-R2-1); `--date` the day it runs; a
//   10 s warm-up for 48 clients.
// - The rest as c-u4r-runs.mjs: the git checks (c-u4f-r2.mjs's `committed`); raw logs under
//   .gate/netcode-proof-test/runs/u4f-<set>-<commit12>/ (measured/ and bites/), results under
//   results/u4f/<commit12>/pm-0001-<set>/ (runs.txt and each call's verdict file), neither ever overwritten; each npt-lab
//   call and the judge bounded at RUN_LIMIT_MS (a stopped call is a result, never retried); the judge's projected peak
//   per call (judgeRoom) printed before the builds, the set stopping there over the 10 GiB line.
// - The projection: BYTES_10MIN_OTHER (56 MB a 10-minute run: c-u4r-runs.mjs:149-154) scaled by the measured seconds,
//   and 1.5 times that for -20 (its per-command series: an estimate until its first run measures it, 5.5).

import { copyFileSync, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { run } from "../../check.mjs";
import { BYTES_10MIN_OTHER, judgeRoom, logBytes, RUN_LIMIT_MS, setParts, spikeChanged } from "./c-u4r-runs.mjs";
import { build, committed, floor, GATE, ROOT, sha256, SPIKE } from "./c-u4f-r2.mjs";

const SEEDS = [1, 2, 3];
/** The line above a set's two judge blocks (5.4: U4f's stop note). */
export const STOP_NOTE = "the set is judged in two calls (U4r-F2's split): verdict.json holds the measured runs' verdicts (read the set's parts; its bites read 'no run yet' by design); verdict-bites.json holds the bite runs' verdicts (read its bites; every part reads '0 of n valid runs' by design)";

/** 5.1's sets: the marks each header lists, each judge call's --only, the settings, the options and the bites. */
export const SETS = {
  47: { listed: ["PM-0001-47"], only: ["PM-0001-47"], args: ["--profile", "P2", "--suite", "clock", "--connects", "1000"], measuredS: 120, options: [null], bites: [["PM-0001-47/newest-sample", null], ["PM-0001-47/tick-quantised", null]] },
  22: { listed: ["PM-0001-2", "PM-0001-22"], only: ["PM-0001-22"], report: ["PM-0001-2"], args: ["--profile", "P0", "--controller", "1", "--suite", "never-ack"], measuredS: 600, options: [null], bites: [["PM-0001-22/unbounded", null, 1_800]] },
  20: { listed: ["PM-0001-2", "PM-0001-20"], only: ["PM-0001-2", "PM-0001-20"], args: ["--profile", "P3", "--controller", "1", "--brawl-press-every", "2", "--suite", "brawl"], measuredS: 600, options: ["A", "B"], bites: [["PM-0001-20/redundancy-1", "B"]] },
};

/** One set's runs in order, each with its projected bytes and group (measured or bites). */
export function plan(key) {
  const set = SETS[key];
  const bytes = (s) => Math.ceil(((BYTES_10MIN_OTHER * s) / 600) * (key === "20" ? 1.5 : 1));
  const opt = (o) => (o ? ["--cmd-timing", o] : []);
  const runs = set.options.flatMap((o) =>
    SEEDS.map((seed) => ({ run: `l1-${key}${o ? `-${o.toLowerCase()}` : ""}-s${seed}`, exe: "lab", seed, group: "measured", extra: [...opt(o), "--role", "measured"], measuredS: set.measuredS })),
  );
  for (const [id, o, s] of set.bites) runs.push({ run: `bite-${id.split("/")[1]}`, exe: "labBites", seed: 1, group: "bites", extra: [...opt(o), "--role", "bite", "--bite", id], measuredS: s ?? set.measuredS });
  return runs.map((r) => ({ ...r, bytes: bytes(r.measuredS) }));
}

function main(argv) {
  const [key, flag, date] = argv;
  if (!SETS[key] || flag !== "--date" || !/^\d{4}-\d{2}-\d{2}$/.test(date ?? "") || argv.length !== 3) throw new Error("usage: c-u4f-runs.mjs 47|22|20 --date YYYY-MM-DD");
  if (process.env.CARGO_TARGET_DIR !== undefined) throw new Error("CARGO_TARGET_DIR is set: the judged sets build in the main folder's own cache (plan §7.1 item 6)");
  const set = SETS[key];
  const { commit, reading, start } = committed();
  const short = commit.slice(0, 12);
  const logs = join(GATE, "runs", `u4f-${key}-${short}`);
  const results = join(SPIKE, "results", "u4f", short, `pm-0001-${key}`);
  for (const d of [logs, results]) if (existsSync(d)) throw new Error(`${d} exists: a run set is never overwritten`);
  const runs = plan(key);
  const total = runs.reduce((s, r) => s + r.bytes, 0);
  const head = [`commit ${commit}, spikes/netcode-proof tree ${start.tree}; target: the main folder's own cache`, `projection: ${floor(`u4f ${key}: the set's ${runs.length} runs, before the builds`, total)}`];
  for (const g of ["measured", "bites"]) {
    const j = judgeRoom(runs.filter((r) => r.group === g).reduce((s, r) => s + r.bytes, 0), `the judge (${g} runs), from the plan's bytes`);
    head.push(`projection: ${j.line}`);
    if (!j.ok) throw new Error(j.line);
  }
  const bin = join(GATE, "bin", `u4f-${key}-${short}`);
  mkdirSync(bin, { recursive: true });
  const exe = { lab: build(["-p", "npt-lab"], "npt-lab", join(bin, "npt-lab")), labBites: build(["-p", "npt-lab", "--features", "bites"], "npt-lab", join(bin, "npt-lab-bites")) };
  const judge = build(["-p", "npt-judge"], "npt-judge", join(bin, "npt-judge"));
  mkdirSync(logs, { recursive: true });
  const marks = join(logs, "marks.json");
  copyFileSync(join(SPIKE, "marks", "marks.json"), marks);
  const sums = { marks: sha256(marks), judge: sha256(judge) };
  head.push(`npt-lab sha256 ${sha256(exe.lab)}; npt-lab (bites) sha256 ${sha256(exe.labBites)}; npt-judge sha256 ${sums.judge}; marks.json sha256 ${sums.marks}`);
  const lines = [];
  for (const r of runs) {
    floor(r.run, r.bytes);
    const args = ["scenario", "--run", r.run, "--set", `u4f-${key}`, "--date", date, "--commit", commit, "--out", join(logs, r.group, r.run), "--scene", "SS", "--seed", String(r.seed), "--warmup-s", "10", "--measured-s", String(r.measuredS), ...set.args, ...r.extra, "--marks", (r.group === "bites" ? [...set.listed.filter((m) => m === "PM-0001-2"), r.extra.at(-1).split("/")[0]] : set.listed).join(",")];
    const t0 = Date.now();
    const x = run(exe[r.exe], args, ROOT, process.env, RUN_LIMIT_MS);
    const secs = ((Date.now() - t0) / 1000).toFixed(1);
    const said = x.timedOut ? `${r.run}: stopped at ${secs} s (a result, never retried)` : x.status === 0 ? `${x.stdout.trim()} (wall ${secs} s)` : `${r.run}: exit ${x.status} (wall ${secs} s): ${x.out.trim()}`;
    console.log(said);
    lines.push(said);
  }
  const call = (label, dir, only, file) => {
    const why = spikeChanged(start, reading());
    if (why) throw new Error(`spikes/netcode-proof changed since the set started (${why}): stopped before ${label}`);
    if (sha256(marks) !== sums.marks || sha256(judge) !== sums.judge) throw new Error(`marks.json or npt-judge changed since the set started: stopped before ${label}`);
    const block = [judgeRoom(logBytes(dir), `${label}, from its logs on disk`, true).line];
    const out = join(logs, file);
    const j = run(judge, ["--marks", marks, "--logs", dir, "--only", only.join(","), "--out", out], ROOT, process.env, RUN_LIMIT_MS);
    const ok = !j.timedOut && j.status === 0;
    block.push(`${label} (--only ${only.join(",")}): ${ok ? j.stdout.trim() : j.timedOut ? "stopped (no verdict)" : `exit ${j.status} (no verdict): ${j.out.trim()}`}`);
    if (ok) block.push(...setParts(JSON.parse(readFileSync(out, "utf8"))));
    console.log(block.join("\n"));
    return { ok, out, file, block };
  };
  const calls = [call("the judge (measured runs)", join(logs, "measured"), set.only, "verdict.json"), call("the judge (bite runs)", join(logs, "bites"), set.only, "verdict-bites.json")];
  if (set.report) calls.push(call("5.2's load report (never counted)", join(logs, "measured"), set.report, "verdict-load.json"));
  mkdirSync(results, { recursive: true });
  for (const c of calls) if (c.ok) copyFileSync(c.out, join(results, c.file));
  const top = [`# u4f-${key}: PM-0001-${key}'s L1 set, run on ${date} (u4f-brief.md 5.1, 5.4)`, ...head, STOP_NOTE, ...calls.flatMap((c) => c.block), "", "# one line per run (npt-lab's summary line and its wall time)"];
  writeFileSync(join(results, "runs.txt"), `${[...top, ...lines].join("\n")}\n`);
  if (calls.some((c) => !c.ok)) throw new Error("a judge call gave no verdict: runs.txt written (to the advisor)");
  console.log(`written: ${join("spikes", "netcode-proof", "results", "u4f", short, `pm-0001-${key}`)}/`);
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    main(process.argv.slice(2));
  } catch (e) {
    console.error(`c-u4f-runs: ${e.message}`);
    process.exit(1);
  }
}
