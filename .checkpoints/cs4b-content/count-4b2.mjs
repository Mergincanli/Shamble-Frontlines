// 4b-2's official count (the advisor agent's ruling of 2026-11-27, cs4b-content/advisor-count-rule.md), read from the
// saved output of `git diff --numstat b9aad01954be666340a7ec5c6e9fc9353b11abf1 -- crates packages` (new files marked
// with `git add -N` first: numstat does not see untracked files). Added lines; left out: crates/devtools/vectors/ but
// for its .py files, and what make_recordings writes into crates/devtools/recordings/ (.rec, .hashes, manifest.txt,
// *.content.json). Everything else counts, packages/types/src/generated/ included. The raw total is printed beside it.
// Usage: node count-4b2.mjs <the saved numstat file>
import { readFileSync } from "node:fs";

const [file] = process.argv.slice(2);
if (file === undefined) throw new Error("usage: node count-4b2.mjs <the saved numstat file>");
let official = 0;
let raw = 0;
const left = [];
for (const line of readFileSync(file, "utf8").split("\n")) {
  if (line === "") continue;
  const [added, , name] = line.split("\t");
  // A binary file (the .rec) has no lines.
  if (added === "-" || name === undefined) continue;
  const lines = Number(added);
  if (!Number.isInteger(lines)) throw new Error(`not a numstat line: ${line}`);
  const vectors = name.startsWith("crates/devtools/vectors/") && !name.endsWith(".py");
  const recordings =
    name.startsWith("crates/devtools/recordings/") &&
    (name.endsWith(".rec") || name.endsWith(".hashes") || name.endsWith("/manifest.txt") || name.endsWith(".content.json"));
  // The raw total is the official one with the recordings in (the ruling's B3); vectors/ data stays out of both.
  if (!vectors) raw += lines;
  if (vectors || recordings) left.push(`${lines} ${name}`);
  else official += lines;
}
console.log(`official ${official} (raw, the recordings in: ${raw})`);
for (const entry of left.filter((entry) => entry.includes("/recordings/"))) console.log(`  left out: ${entry}`);
