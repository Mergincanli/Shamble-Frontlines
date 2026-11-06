#!/usr/bin/env node
// PM-0001-3's L1 half in this container (plan §6.2: "C (L1, U2)"; §6.4: three seeds per profile).
//   node spikes/netcode-proof/ops/runs/c-u2-pm-0001-3.mjs --date YYYY-MM-DD
// - Every L1 part of the mark has its own twin check (marks.json), so each measured run gets a twin: the same profile
//   and seed, in a separate process.
// - Each of the four L1 bites runs in the bites build at P2, seed 1, with its own twin from the same build, so a bite is
//   shown only by the check it plants a defect for (the clock-seed bite's twin differs; the others' twins match).
// - It refuses uncommitted changes to tracked files: every log names the commit it was built from.
// - Raw logs go under .gate/netcode-proof-test/runs/ (gitignored, never committed: plan §10.6). The judge's verdict
//   on PM-0001-3 and one line per run go to results/c-u2-pm-0001-3/<commit>/. Neither is ever overwritten.

import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const SPIKE = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const ROOT = join(SPIKE, "..", "..");
const GATE = join(ROOT, ".gate", "netcode-proof-test");
const SET = "c-u2-pm-0001-3";
const PROFILES = ["P1", "P2", "P3", "P4", "B1"];
const SEEDS = [1, 2, 3];
const BITES = [
  "PM-0001-3/rtt-each-way",
  "PM-0001-3/loss-per-round-trip",
  "PM-0001-3/normal-jitter",
  "PM-0001-3/clock-seed",
];

function sh(cmd, args, cwd = ROOT) {
  const r = spawnSync(cmd, args, { cwd, encoding: "utf8", maxBuffer: 1 << 28 });
  if (r.status !== 0) throw new Error(`${cmd} ${args.join(" ")}: exit ${r.status}\n${r.stdout}${r.stderr}`);
  return r.stdout.trim();
}

const sha256 = (path) => createHash("sha256").update(readFileSync(path)).digest("hex");

function main(argv) {
  const at = argv.indexOf("--date");
  const date = at >= 0 ? argv[at + 1] : undefined;
  if (!/^\d{4}-\d{2}-\d{2}$/.test(date ?? "")) throw new Error("usage: c-u2-pm-0001-3.mjs --date YYYY-MM-DD");
  if (sh("git", ["status", "--porcelain", "--untracked-files=no"])) {
    throw new Error("tracked files have uncommitted changes: commit first, since every log names its commit");
  }
  const commit = sh("git", ["rev-parse", "HEAD"]);
  const short = commit.slice(0, 12);
  const logs = join(GATE, "runs", `${SET}-${short}`);
  const results = join(SPIKE, "results", SET, short);
  for (const d of [logs, results]) if (existsSync(d)) throw new Error(`${d} exists: a run set is never overwritten`);

  // Both builds of npt-lab, each copied out before the next build replaces it, and the judge.
  const bin = join(GATE, "bin", short);
  mkdirSync(bin, { recursive: true });
  const built = join(GATE, "target", "release", "npt-lab");
  sh("cargo", ["build", "--release", "--locked", "-p", "npt-lab"], SPIKE);
  copyFileSync(built, join(bin, "npt-lab"));
  sh("cargo", ["build", "--release", "--locked", "-p", "npt-lab", "--features", "bites"], SPIKE);
  copyFileSync(built, join(bin, "npt-lab-bites"));
  sh("cargo", ["build", "--release", "--locked", "-p", "npt-judge"], SPIKE);
  const judge = join(GATE, "target", "release", "npt-judge");

  mkdirSync(logs, { recursive: true });
  const lines = [];
  const probe = (exe, run, role, profile, seed, extra) => {
    const out = sh(join(bin, exe), [
      "probe", "--run", run, "--set", SET, "--role", role, "--profile", profile, "--seed", String(seed),
      "--date", date, "--commit", commit, "--out", join(logs, `${run}.jsonl`), ...extra,
    ]);
    console.log(out);
    lines.push(out);
  };
  for (const p of PROFILES) {
    for (const s of SEEDS) {
      const run = `l1-${p.toLowerCase()}-s${s}`;
      probe("npt-lab", run, "measured", p, s, ["--twin", `${run}-twin`]);
      probe("npt-lab", `${run}-twin`, "twin", p, s, []);
    }
  }
  for (const b of BITES) {
    const run = `bite-${b.split("/")[1]}`;
    probe("npt-lab-bites", run, "bite", "P2", 1, ["--bite", b, "--twin", `${run}-twin`]);
    probe("npt-lab-bites", `${run}-twin`, "twin", "P2", 1, ["--bite", b]);
  }

  const verdict = join(logs, "verdict.json");
  const judged = sh(judge, [
    "--marks", join(SPIKE, "marks", "marks.json"), "--logs", logs, "--only", "PM-0001-3", "--out", verdict,
  ]);
  console.log(judged);
  mkdirSync(results, { recursive: true });
  copyFileSync(verdict, join(results, "verdict.json"));
  const head = [
    `# ${SET}: PM-0001-3's L1 half, run in this container (machine role C) on ${date}`,
    `commit ${commit}`,
    `npt-lab (measured build) sha256 ${sha256(join(bin, "npt-lab"))}`,
    `npt-lab (bites build) sha256 ${sha256(join(bin, "npt-lab-bites"))}`,
    `npt-judge sha256 ${sha256(judge)}`,
    `raw logs: .gate/netcode-proof-test/runs/${SET}-${short}/ (not committed: plan §10.6)`,
    `judge: ${judged}`,
    "",
    "# one line per run (npt-lab's own summary; the judge reads the logs, never these lines)",
  ];
  writeFileSync(join(results, "runs.txt"), `${[...head, ...lines].join("\n")}\n`);
  console.log(`written: ${join("spikes", "netcode-proof", "results", SET, short)}/verdict.json and runs.txt`);
}

try {
  main(process.argv.slice(2));
} catch (e) {
  console.error(`c-u2-pm-0001-3: ${e.message}`);
  process.exit(1);
}
