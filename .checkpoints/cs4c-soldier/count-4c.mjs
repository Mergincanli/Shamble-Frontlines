// 4c's official count, by 4b-2's method (the advisor agent's ruling of 2026-11-27, readable at 59f6cc7 as
// `.checkpoints/cs4b-content/advisor-count-rule.md`; 4b-2's script `count-4b2.mjs` beside it), against 4c's base,
// main dae6d107fdbe942567104bcf1c9a2cbc6b3a1e0f. It reads the saved output of
//   git diff --numstat dae6d107fdbe942567104bcf1c9a2cbc6b3a1e0f -- crates packages
// (new files marked with `git add -N` first: numstat does not see untracked files). Added lines; left out:
// crates/devtools/vectors/ but for its .py files, and what make_recordings writes into crates/devtools/recordings/
// (.rec, .hashes, manifest.txt, *.content.json). Everything else counts, packages/types/src/generated/ included (the
// ruling held it counted to 4b-2's end; for 4c the advisor agent rules again, and this line follows its ruling). The
// raw total is printed beside it.
// Usage: node .checkpoints/cs4c-soldier/count-4c.mjs <the saved numstat file, a full path>
import { readFileSync } from "node:fs";

const [file] = process.argv.slice(2);
if (file === undefined) throw new Error("usage: node count-4c.mjs <the saved numstat file>");
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
  // The raw total is the official one with the recordings in; vectors/ data stays out of both.
  if (!vectors) raw += lines;
  if (vectors || recordings) left.push(`${lines} ${name}`);
  else official += lines;
}
console.log(`official ${official} (raw, the recordings in: ${raw})`);
for (const entry of left.filter((entry) => entry.includes("/recordings/"))) console.log(`  left out: ${entry}`);
