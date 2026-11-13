// docs-keeper scratch: replace STATUS's #48 In-flight row (the line starting "| #48 |") with #52's row.
import { readFileSync, writeFileSync } from 'node:fs';
const f = '/home/user/Shamble-Frontlines/docs/STATUS.md';
const lines = readFileSync(f, 'utf8').split('\n');
const at = lines.map((l, i) => (l.startsWith('| #48 |') ? i : -1)).filter((i) => i >= 0);
if (at.length !== 1) throw new Error(`#48 rows: ${at.length}`);
lines[at[0]] = "| #52 | the foundation, 2.3: its first docs pass (his words for #48 and of 2026-11-13 in `docs/DECISIONS.md`, by script; dated pointers in SPIKE_PLAN, ADR 0002, DECISIONS (SLICE-ORDER condition 3), ROADMAP and STATUS; the 2.2 report's fixes M1, L1, L2 (worded as the CI logs read), L3 and L5, two more archive lines and two pointing clauses; STATUS, ROADMAP, the archive and REPO_MAP true for main 4d21c1a) | `claude/sweet-mayer-grsukw` (his personal account; for `cs/foundation-2-3-docs`) | draft; built | review round 1 (two reviewers), the verifier, the advisor agent's review of its final head; then his go (risk tier 1, decision docs) |";
writeFileSync(f, lines.join('\n'));
console.log('replaced line', at[0] + 1);
