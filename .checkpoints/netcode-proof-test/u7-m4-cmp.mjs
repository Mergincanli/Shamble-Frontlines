// U7's merge, M3 and M4's file check (u7-merge-plan.md): each path wip/u7-build-rs.patch names is, in the main folder,
// byte-equal to its copy in U7's builder's worktree, and both files have mode 644. Read only: the paths come from the
// patch's "diff --git" lines; it runs no git and writes nothing. The index modes are a separate git call.
// Run from the main folder after M4: node .checkpoints/netcode-proof-test/u7-m4-cmp.mjs
import { readFileSync, statSync } from "node:fs";
import { join } from "node:path";

const M = "/home/user/Shamble-Frontlines";
const T = join(M, ".claude", "worktrees", "agent-a9873cdba07a3219e");
const PATCH = join(M, ".checkpoints", "netcode-proof-test", "wip", "u7-build-rs.patch");

const paths = [];
for (const line of readFileSync(PATCH, "utf8").split("\n")) {
  const m = /^diff --git a\/(\S+) b\/(\S+)$/.exec(line);
  if (!m) continue;
  if (m[1] !== m[2]) throw new Error(`m4: a rename or copy in the patch: ${line}`);
  paths.push(m[1]);
}
let bad = 0;
for (const p of paths) {
  const here = join(M, p);
  const there = join(T, p);
  const same = readFileSync(here).equals(readFileSync(there));
  const modeHere = (statSync(here).mode & 0o777).toString(8);
  const modeThere = (statSync(there).mode & 0o777).toString(8);
  if (!same || modeHere !== "644" || modeThere !== "644") {
    bad += 1;
    console.log(`m4: DIFFERS ${p} (bytes ${same ? "equal" : "differ"}; modes ${modeHere} and ${modeThere})`);
  }
}
console.log(`m4: ${paths.length} paths in the patch; ${paths.length - bad} byte-equal with mode 644 on both sides; ${bad} not`);
process.exit(paths.length === 43 && bad === 0 ? 0 : 1);
