// Scratch, print only (reads git and files, writes nothing): would `git merge --ff-only <to>` in a builder's worktree
// at <from> collide with its local files? ADDED paths: any file there blocks (git refuses even an identical untracked
// file). MODIFIED and DELETED paths: the worktree's copy must equal <from>'s blob (else git refuses a local change).
// Not read: the worktree's index (a staged change would also refuse; the brief's stop rule covers it).
// Run from the main folder (never cd into the worktree). First run 2026-11-25 13:5x UTC for brief 3 (launch-0850.md).
//   node ffcheck.mjs <worktree> <from> <to>
import { execFileSync } from "node:child_process";
import { lstatSync } from "node:fs";
import { join } from "node:path";

const [wt, from, to] = process.argv.slice(2);
const git = (args) => execFileSync("git", args, { encoding: "utf8", maxBuffer: 1 << 28 });
const parts = git(["diff", "--name-status", "--no-renames", "-z", from, to]).split("\0").filter(Boolean);
const rows = [];
for (let i = 0; i < parts.length; i += 2) rows.push({ st: parts[i], path: parts[i + 1] });
const exists = (p) => { try { lstatSync(join(wt, p)); return true; } catch { return false; } };
const blobs = new Map();
for (const e of git(["ls-tree", "-r", "-z", from]).split("\0").filter(Boolean)) {
  const tab = e.indexOf("\t");
  blobs.set(e.slice(tab + 1), e.slice(0, tab).split(" ")[2]);
}
const added = rows.filter((r) => r.st === "A");
const changed = rows.filter((r) => r.st !== "A");
const addedThere = added.filter((r) => exists(r.path));
const present = changed.filter((r) => exists(r.path));
const ids = present.length ? git(["hash-object", "--", ...present.map((r) => join(wt, r.path))]).trim().split("\n") : [];
const differ = present.filter((r, i) => ids[i] !== blobs.get(r.path));
const counts = {};
for (const r of rows) counts[r.st] = (counts[r.st] || 0) + 1;
console.log(`worktree ${wt}`);
console.log(`merge ${from}..${to}: ${rows.length} paths ${JSON.stringify(counts)}`);
console.log(`ADDED paths present in the worktree: ${addedThere.length} ${JSON.stringify(addedThere.map((r) => r.path))}`);
console.log(`MODIFIED/DELETED paths present: ${present.length}, absent: ${changed.length - present.length}`);
console.log(`  of those present, differing from ${from}'s blob: ${differ.length} ${JSON.stringify(differ.map((r) => r.path))}`);
console.log(`hash-object ids read: ${ids.length} (want ${present.length})`);
