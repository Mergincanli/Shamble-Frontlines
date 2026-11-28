// Copies ADR 0004's rows PM-0004-20 and PM-0004-35, byte for byte, into the 4c-2 wording draft's "old" section.
import { readFileSync, writeFileSync } from "node:fs";

const adr = "/home/user/Shamble-Frontlines/.gate/cs4b-2/docs/adr/0004-simulation-core.md";
const out = "/home/user/Shamble-Frontlines/.gate/cs4b-2/.checkpoints/cs4c-soldier/pm-wording-4c2.md";
const lines = readFileSync(adr, "utf8").split("\n");
const row = (id) => {
  const found = lines.map((text, i) => ({ text, n: i + 1 })).filter(({ text }) => text.startsWith(`| ${id} |`));
  if (found.length !== 1) throw new Error(`${id}: ${found.length} rows`);
  return found[0];
};
const a = row("PM-0004-20");
const b = row("PM-0004-35");
const headerAt = lines.findIndex((text) => text.startsWith("| mark | what it proves"));
if (headerAt < 0 || !lines[headerAt + 1].startsWith("|---")) throw new Error("the table's header row is not found");
const text = [
  "# PM-0004-20 and PM-0004-35: the old and the new wording, side by side (4c-2's draft, his DEPART-4C (a))",
  "",
  "His words (2026-11-28, item 12 a, saved in `.checkpoints/cs4-walk/mustafa-2026-11-28-answers.md`): the report shows",
  "the old and the new wording of the two marks side by side, and the new wording asks no less than the old about what",
  "the players' file does. ADR 0004's rows are not edited in 4c-2 (a decision doc; the advisor agent's F2): the new",
  "wording goes into 4c-2's report and a dated pointer in STATUS, as DEPART-4 did in 4b.",
  "",
  `## Old, copied by script (\`pm-old-rows.mjs\`) from \`docs/adr/0004-simulation-core.md\`, lines ${a.n} and ${b.n}`,
  "",
  lines[headerAt],
  lines[headerAt + 1],
  a.text,
  b.text,
  "",
];
writeFileSync(out, text.join("\n"));
console.log(`wrote ${out}: rows at lines ${a.n} and ${b.n}, ${Buffer.byteLength(a.text)} and ${Buffer.byteLength(b.text)} bytes`);
