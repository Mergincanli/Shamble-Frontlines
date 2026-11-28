// NPT-L2-ONEWAY: adds one line to PM-0001-3's notes in spikes/netcode-proof/marks/marks.json, and nothing else.
//   node .checkpoints/netcode-judge/oneway-note.mjs [--write]      (from the repository root; without --write: a dry run)
// The line is two quotes of Mustafa's, read from where they are saved, never retyped (the advisor agent's plan
// review, condition 1): the bracket of item 4 of his message to Netcode Builder 2 (mustafa-2026-11-25.md) and his
// answer (a) in the netcode chat (.checkpoints/netcode-proof-test/history.md, item 2 of his message of that day).
// The file is exactly JSON.stringify(x, null, 2) + "\n" (checked), so no other byte moves. It refuses a second run.
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";

const FILE = "spikes/netcode-proof/marks/marks.json";
const sha = (t) => createHash("sha256").update(t, "utf8").digest("hex");
const fail = (m) => {
  console.error(`oneway-note: ${m}`);
  process.exit(1);
};

const mine = readFileSync(".checkpoints/netcode-judge/mustafa-2026-11-25.md", "utf8");
const bracket = mine.match(/^4\. Next, only after the netcode chat has merged your branch: NPT-L2-ONEWAY \((the probe's one-way delay checks on the real network read against the profile's one-way delay minus half the measured base round trip; no tolerance changes)\)\./m);
const theirs = readFileSync(".checkpoints/netcode-proof-test/history.md", "utf8");
const answer = theirs.match(/^2\. NPT-L2-ONEWAY \(the one-way delay checks on the real network\): \(a\)\. (No tolerance changes, the round-trip checks stay as they are, and the base round trip is the leg's own measured value\.) Netcode Builder 2 builds it/m);
if (!bracket || !answer) fail("his two sentences are not where this script reads them");

const text = readFileSync(FILE, "utf8");
const data = JSON.parse(text);
if (`${JSON.stringify(data, null, 2)}\n` !== text) fail("marks.json is not JSON.stringify(x, null, 2): refusing to rewrite it");
const at = data.marks.map((m, i) => (m.id === "PM-0001-3" ? i : -1)).filter((i) => i >= 0);
if (at.length !== 1) fail(`${at.length} marks are PM-0001-3, not 1`);
const mark = data.marks[at[0]];
if (mark.notes.includes("NPT-L2-ONEWAY")) fail("PM-0001-3's notes already hold the NPT-L2-ONEWAY line");
const line =
  `NPT-L2-ONEWAY (Mustafa, 2026-11-25): on L2, "${bracket[1]}" (his message to Netcode Builder 2, item 4); ` +
  `"${answer[1]}" (his answer (a) in the netcode chat). The judge carries the rule: marks/README.md 2.3.`;
mark.notes = `${mark.notes} ${line}`;

const out = `${JSON.stringify(data, null, 2)}\n`;
console.log(`the line: ${line}`);
console.log(`sha256 before ${sha(text)}`);
console.log(`sha256 after  ${sha(out)}`);
if (process.argv.includes("--write")) {
  writeFileSync(FILE, out);
  console.log(`written: ${FILE}`);
} else {
  console.log("dry run: nothing written (pass --write)");
}
