// Checks that STATUS.md quotes his item 12 b and c byte for byte from his saved message, and DECISIONS.md the same.
// From anywhere: node /home/user/Shamble-Frontlines/.gate/cs4b-2/.checkpoints/cs4c-soldier/quote-check-12bc.mjs
// (first run 2026-11-28 from the scratchpad, after 4c-1's docs pass; both exact). It calls no git.
import { readFileSync } from "node:fs";

const root = "/home/user/Shamble-Frontlines/.gate/cs4b-2";
const saved = readFileSync(`${root}/.checkpoints/cs4-walk/mustafa-2026-11-28-answers.md`);
const status = readFileSync(`${root}/docs/STATUS.md`);
const decisions = readFileSync(`${root}/docs/DECISIONS.md`);

const lines = saved.toString("utf8").split("\n");
const items = { b: lines[24], c: lines[25] };
let failed = 0;
for (const [name, line] of Object.entries(items)) {
  const prefix = `   ${name}. `;
  if (!line.startsWith(prefix)) {
    console.log(`item 12 ${name}: saved line ${name === "b" ? 25 : 26} does not start with ${JSON.stringify(prefix)}`);
    failed += 1;
    continue;
  }
  const words = Buffer.from(line.slice(prefix.length), "utf8");
  const quoted = Buffer.concat([Buffer.from('"'), words, Buffer.from('"')]);
  const inStatus = status.indexOf(quoted) !== -1;
  const inDecisions = decisions.indexOf(words) !== -1;
  console.log(`item 12 ${name}: ${words.length} bytes; quoted whole in STATUS.md: ${inStatus}; in DECISIONS.md: ${inDecisions}`);
  if (!inStatus || !inDecisions) failed += 1;
}
process.exit(failed === 0 ? 0 : 1);
