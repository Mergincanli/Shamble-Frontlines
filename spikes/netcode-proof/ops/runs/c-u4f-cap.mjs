#!/usr/bin/env node
// U4f's event-cap measurement (u4f-brief.md 2.8 items 3-5; U4F-Q12 (b); carries 57, 58 and 50), on c-u4r-r2.mjs's
// model: nine short runs, SS, 48 clients, the brawl with U4f's tick-lead controller, at P2, P3 and B1 (2 bursts), seeds 1
// to 3, each with the room's queue cap opened to CAP_OPEN (`--event-cap`), so no overflow resync clears a queue and the
// high-water marks show the need, not the provisional cap. Per run, then over all nine:
// - the rate: the room's reliable gameplay events a second (`second.v.reliable_events_count`), mean and max per 30 s bin
//   of the measured phase, and each client's mean queued rate over the run (`event.client_counts.events_queued_count`
//   over the run's seconds, from `event.run_lengths`);
// - the queue's high-water mark per client: the most of `second.s.event_queue_max_count` in each 30 s bin, and per burst
//   (`event.burst_heal.queue_max_count`, B1);
// - the value (U4F-Q12 (b)): the largest per-client high-water mark seen times MARGIN, rounded up to a multiple of 64,
//   checked below carry 50's upper bound (NA-12): the lowest client's mean queued rate x 255 / 30 (an event older than
//   255 ticks expires: room/src/events.rs:19-20), so a never-acker reaches the cap before its events expire. The value
//   and its figures go to the advisor before any -22 run (carry 57); plan §4.7's text is the main session's.
// Run it under the lane runner, exclusive (u4f-brief.md section 4 rule 2), from the spike:
//   bash <u4f-run.sh> -x <worktree> <log> node ops/runs/c-u4f-cap.mjs --date YYYY-MM-DD --out <new folder>
// The builder's runs hold uncommitted edits (2.8 item 3: builder evidence): every log names HEAD, and cap.txt's head
// adds `git diff HEAD`'s sha256. One release build of npt-lab, at least 4 GiB free before it and before each run
// (section 4 rule 6), copied to <out>/bin; each run's logs under <out>/<profile>-s<seed>/, cap.txt beside them. Each
// command runs through check.mjs's run() with a 600 s limit; a stopped run is a result, never retried.

import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, readFileSync, statfsSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { builtArtifact, run } from "../../check.mjs";
import { roomFor } from "./c-u4r-runs.mjs";

const SPIKE = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
/** The opened cap for the measurement: above carry 50's bound at any rate seen, so no queue meets it. */
export const CAP_OPEN = 8_192;
/** The stated margin over the largest per-client high-water mark (U4F-Q12 (b)). */
export const MARGIN = 1.5;
/** An event's age bound, ticks (the event section's 8 bits: carry 50), and the tick rate. */
const AGE_TICKS = 255;
const TICK_HZ = 30;
const STEP_LIMIT_MS = 600_000;
const LENGTHS = ["--warmup-s", "10", "--measured-s", "60"];
const B1 = ["--bursts", "2", "--burst-first-s", "10", "--burst-every-s", "30"];
export const RUNS = ["P2", "P3", "B1"].flatMap((p) => [1, 2, 3].map((seed) => ({ profile: p, seed })));

/** One log's records by kind (`k`: header, tick, second, event, footer). */
export function records(path) {
  const out = { second: [], event: [] };
  for (const line of readFileSync(path, "utf8").split("\n")) {
    if (!line) continue;
    const r = JSON.parse(line);
    out[r.k]?.push(r);
  }
  return out;
}

/** One run's figures from its room and lab logs (the module text). */
export function figures(room, lab) {
  const measured = room.second.filter((s) => s.ph === "measured");
  const first = measured[0]?.sec ?? 0;
  const bins = [];
  for (const s of measured) {
    const b = (bins[Math.floor((s.sec - first) / 30)] ??= { secs: 0, events: 0, rateMax: 0, queueMax: 0 });
    const r = s.v?.reliable_events_count ?? 0;
    b.secs += 1;
    b.events += r;
    b.rateMax = Math.max(b.rateMax, r);
    b.queueMax = Math.max(b.queueMax, ...(s.s?.event_queue_max_count ?? [0]));
  }
  const bursts = {};
  for (const e of lab.event.filter((x) => x.ev === "burst_heal")) bursts[e.f.burst] = Math.max(bursts[e.f.burst] ?? 0, e.f.queue_max_count);
  const len = lab.event.find((e) => e.ev === "run_lengths")?.f ?? {};
  const secs = (len.warmup_ticks_count + len.measured_ticks_count + len.after_ticks_count) / TICK_HZ;
  const rates = lab.event.filter((e) => e.ev === "client_counts").map((e) => e.f.events_queued_count / secs);
  return { bins, bursts, rateLow: Math.min(...rates), rateHigh: Math.max(...rates), high: Math.max(0, ...bins.map((b) => b.queueMax), ...Object.values(bursts)) };
}

/** The value and carry 50's bound from all runs' figures: [value, bound, the line]. */
export function value(all) {
  const high = Math.max(...all.map((f) => f.high));
  const v = Math.ceil((high * MARGIN) / 64) * 64;
  const bound = Math.floor((Math.min(...all.map((f) => f.rateLow)) * AGE_TICKS) / TICK_HZ);
  const ok = v < bound;
  return [v, bound, `value: the largest per-client high-water mark ${high} x the margin ${MARGIN}, rounded up to 64: ${v}; carry 50's bound (the lowest client's mean queued rate x ${AGE_TICKS} / ${TICK_HZ}): ${bound}; ${ok ? "below it" : "NOT below it: to the advisor"}`];
}

function sh(cmd, args) {
  const r = spawnSync(cmd, args, { cwd: SPIKE, encoding: "utf8", maxBuffer: 1 << 28 });
  if (r.status !== 0) throw new Error(`${cmd} ${args.join(" ")}: exit ${r.status}\n${r.stdout}${r.stderr}`);
  return r.stdout;
}

function floor(what) {
  const s = statfsSync(SPIKE);
  const r = roomFor(s.bavail * s.bsize, 0, what);
  if (!r.ok) throw new Error(r.line);
}

function main(argv) {
  const opt = (k) => (argv.includes(k) ? argv[argv.indexOf(k) + 1] : undefined);
  const [date, out] = [opt("--date"), opt("--out")];
  if (!/^\d{4}-\d{2}-\d{2}$/.test(date ?? "") || !out || argv.length !== 4) throw new Error("usage: c-u4f-cap.mjs --date YYYY-MM-DD --out <new folder>");
  if (existsSync(out)) throw new Error(`${out} exists: a measurement is never overwritten`);
  const commit = sh("git", ["rev-parse", "HEAD"]).trim();
  const diff = createHash("sha256").update(sh("git", ["diff", "HEAD", "--", "."])).digest("hex");
  floor("the release build of npt-lab");
  const b = spawnSync("cargo", ["build", "-v", "--release", "--locked", "--offline", "-p", "npt-lab", "--message-format=json"], { cwd: SPIKE, encoding: "utf8", maxBuffer: 1 << 28 });
  if (b.status !== 0) throw new Error(`the release build: exit ${b.status}\n${b.stderr}`);
  for (const l of b.stderr.split("\n")) if (/^\s*(Compiling|Fresh) npt-/.test(l)) console.log(l);
  mkdirSync(join(out, "bin"), { recursive: true });
  const lab = join(out, "bin", "npt-lab");
  copyFileSync(builtArtifact(b.stdout.trim(), { target: "npt-lab", kind: "bin" }), lab);
  const lines = [`# U4f's cap measurement (2.8), ${date}: commit ${commit}, git diff HEAD sha256 ${diff}; cap opened to ${CAP_OPEN}`];
  const all = [];
  for (const { profile, seed } of RUNS) {
    floor(`run ${profile} seed ${seed}`);
    const dir = join(out, `${profile}-s${seed}`);
    const args = ["scenario", "--run", `cap-${profile}-s${seed}`, "--set", "u4f-cap", "--date", date, "--commit", commit, "--out", dir, "--scene", "SS", "--profile", profile, "--seed", String(seed), ...LENGTHS, ...(profile === "B1" ? B1 : []), "--controller", "1", "--event-cap", String(CAP_OPEN), "--suite", "brawl"];
    const t0 = Date.now();
    const r = run(lab, args, SPIKE, process.env, STEP_LIMIT_MS);
    const head = `## ${profile} seed ${seed}: ${r.timedOut ? "stopped" : `exit ${r.status}`} (wall ${((Date.now() - t0) / 1000).toFixed(1)} s): ${r.stdout.trim()}`;
    lines.push(head);
    console.log(head);
    if (r.timedOut || r.status !== 0) continue;
    const f = figures(records(join(dir, "room.jsonl")), records(join(dir, "lab.jsonl")));
    all.push(f);
    for (const [i, x] of f.bins.entries()) lines.push(`  bin ${i} (${x.secs} s): reliable events a second mean ${(x.events / x.secs).toFixed(1)}, max ${x.rateMax}; per-client queue high-water ${x.queueMax}`);
    for (const [k, q] of Object.entries(f.bursts)) lines.push(`  burst ${k}: per-client queue high-water from its start to its heal ${q}`);
    lines.push(`  clients' mean queued rate a second: lowest ${f.rateLow.toFixed(1)}, highest ${f.rateHigh.toFixed(1)}; the run's high-water ${f.high}`);
  }
  if (all.length) lines.push(value(all)[2]);
  console.log(lines.slice(-1)[0]);
  writeFileSync(join(out, "cap.txt"), `${lines.join("\n")}\n`);
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    main(process.argv.slice(2));
  } catch (e) {
    console.error(`c-u4f-cap: ${e.message}`);
    process.exit(1);
  }
}
