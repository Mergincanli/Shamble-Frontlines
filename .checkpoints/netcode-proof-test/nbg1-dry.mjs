// nbg1's dry run (nbg1-plan.md): applies the five in-place lines to a DRY-RUN WORKTREE only, never the main tree.
// Usage: node nbg1-dry.mjs <worktree under .gate/> <shape>, shape one of:
//   sh     r1.sh:34's LOCATION=fsn1 -> nbg1 (alone: r1sh.test.mjs:199 must turn red)
//   pin    r1sh.test.mjs:199's location "fsn1" -> "nbg1"
//   plan   R1.md:63, :114 and :135 (fsn1 -> nbg1, four words on three lines)
//   undo   every change above reversed (the worktree back to X's bytes)
//   show   prints `git diff --numstat HEAD -- spikes/netcode-proof` and the sha256 of `git diff HEAD -- spikes/netcode-proof`
// Each replacement must match exactly once, else it stops with exit 1. git runs with cwd = the worktree (no -C).
import { readFileSync, writeFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { join } from "node:path";

const [wt, shape] = process.argv.slice(2);
const SHAPES = ["sh", "pin", "plan", "undo", "show"];
if (!wt || !wt.includes("/.gate/") || !SHAPES.includes(shape)) {
  console.error(`usage: node nbg1-dry.mjs <a worktree under .gate/> ${SHAPES.join("|")}`);
  process.exit(1);
}
const OPS = join(wt, "spikes", "netcode-proof", "ops");
const EDITS = {
  sh: [[join(OPS, "runs", "r1.sh"),
    "readonly TICK=3 TICK_HZ=30 LOCATION=fsn1 # LOCATION: a Stop B fill, fsn1 or nbg1 (INF-SB-3)\n",
    "readonly TICK=3 TICK_HZ=30 LOCATION=nbg1 # LOCATION: a Stop B fill, fsn1 or nbg1 (INF-SB-3)\n"]],
  pin: [[join(OPS, "tests", "r1sh.test.mjs"),
    '    assert.deepEqual(wt.machine, { role: "LG", name: "LG1", plan: "CAX21", location: "fsn1", address: ADDR.lg });\n',
    '    assert.deepEqual(wt.machine, { role: "LG", name: "LG1", plan: "CAX21", location: "nbg1", address: ADDR.lg });\n']],
  plan: [
    [join(OPS, "runs", "R1.md"),
      "the location is a fill (r1.sh's LOCATION, this plan's `fsn1`).\n",
      "the location is a fill (r1.sh's LOCATION, this plan's `nbg1`).\n"],
    [join(OPS, "runs", "R1.md"),
      "| 09:15 | Create LG (CAX21, `fsn1`, Ubuntu 24.04,",
      "| 09:15 | Create LG (CAX21, `nbg1`, Ubuntu 24.04,"],
    [join(OPS, "runs", "R1.md"),
      "Hetzner `fsn1` refuses the VMs: R1 waits (the kept addresses are bound to `fsn1`).",
      "Hetzner `nbg1` refuses the VMs: R1 waits (the kept addresses are bound to `nbg1`)."],
  ],
};
function replaceOnce(file, from, to) {
  const text = readFileSync(file, "utf8");
  const n = text.split(from).length - 1;
  if (n !== 1) {
    console.error(`${file}: expected exactly one match, found ${n}`);
    process.exit(1);
  }
  writeFileSync(file, text.replace(from, to));
  console.log(`${file}: ${JSON.stringify(from).slice(0, 70)} -> ${JSON.stringify(to).slice(0, 70)}`);
}
if (shape === "show") {
  const git = (args) => execFileSync("git", args, { cwd: wt, encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
  console.log(`HEAD ${git(["rev-parse", "HEAD"]).trim()}`);
  console.log(git(["diff", "--numstat", "HEAD", "--", "spikes/netcode-proof"]).trimEnd() || "(no diff)");
  const diff = execFileSync("git", ["diff", "HEAD", "--", "spikes/netcode-proof"], { cwd: wt, maxBuffer: 64 * 1024 * 1024 });
  console.log(`diff sha256 ${createHash("sha256").update(diff).digest("hex")} (${diff.length} bytes)`);
  console.log(`status:\n${git(["status", "--short", "--untracked-files=all", "--", "spikes/netcode-proof"]).trimEnd() || "(clean)"}`);
} else if (shape === "undo") {
  for (const s of ["plan", "pin", "sh"]) {
    for (const [file, from, to] of EDITS[s]) {
      if (readFileSync(file, "utf8").includes(to)) replaceOnce(file, to, from);
    }
  }
} else {
  for (const [file, from, to] of EDITS[shape]) replaceOnce(file, from, to);
}
