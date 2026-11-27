// docs-keeper, 4b-2: a pointer to his VISION-RULE addition (2026-11-27, 13:57:21 UTC, item 9) at the end of three
// rows of docs/adr/README.md. Appends only; no word of a row changes. Run once from the root.
import { readFileSync, writeFileSync } from "node:fs";
const F = "docs/adr/README.md";
const r = readFileSync(F, "utf8").split("\n");
const add = ". See also his addition to version 2's point 2c (`docs/DECISIONS.md`, the entry of 2026-11-27, 13:57:21 UTC, his item 9; no code now; what it leaves open, in his words: the paragraph that names VISION-RULE, above)";
for (const id of ["R2-INT-OQ1", "R4-LOS-OQ1", "R4-LOS-OQ2"]) {
  const at = r.findIndex((l) => l.startsWith(`| ${id} | `));
  if (at < 0 || !r[at].endsWith(" |") || r[at].includes("13:57:21")) throw new Error(id);
  r[at] = r[at].slice(0, -2) + add + " |";
  console.log(id, "line", at + 1);
}
writeFileSync(F, r.join("\n"));
