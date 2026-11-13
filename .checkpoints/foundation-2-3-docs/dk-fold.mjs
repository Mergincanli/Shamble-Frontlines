// docs-keeper scratch: fold ROADMAP's oldest Recent-work entry (Mustafa's design walk, PR #36) into Build history.
import { readFileSync, writeFileSync } from 'node:fs';
const f = '/home/user/Shamble-Frontlines/docs/ROADMAP.md';
const lines = readFileSync(f, 'utf8').split('\n');
const old = lines.map((l, i) => (l.startsWith("**Mustafa's design walk [2026-11-04, MERGED]**") ? i : -1)).filter((i) => i >= 0);
if (old.length !== 1 || lines[old[0] - 1] !== '') throw new Error(`entry: ${old}`);
const tail = lines.slice(old[0] + 1).filter((l) => l.trim() !== '');
if (tail.length) throw new Error('the entry is not the last block');
const anchor = lines.findIndex((l) => l.includes('OQ-0002-9 and OQ-0006-23; report `docs/reports/2026-11-04-docs-after-kit-v3-1.md`.'));
if (anchor < 0 || !lines[anchor + 1].startsWith('- **UI theme step 1')) throw new Error(`anchor ${anchor}`);
const add = [
  "- **Mustafa's design walk B3 (2026-11-04):** b3-design-walk (PR #36, 8bcc65e, by merge commit, on Mustafa's go after",
  "  the advisor's APPROVE of a32e4c0; docs only): his 2026-11-04 decisions and ideas recorded word for word, tagged",
  "  `[DECIDED]`, `[IDEA]` (new tag) or `[OPEN]`, in LOOP_V2 (new §13), UNITS.md §0 and DESIGN_NOTES, with \"bears on\"",
  "  clauses in the ADR register (no ADR edited); report `docs/reports/2026-11-04-b3-design-walk.md`.",
];
const removed = lines.splice(old[0] - 1);
const keepEnd = removed.slice(2); // whatever followed the entry (only empty lines / trailing newline)
lines.splice(anchor + 1, 0, ...add);
writeFileSync(f, lines.concat(keepEnd).join('\n'));
console.log(`folded entry at ${old[0] + 1}; added ${add.length} lines after ${anchor + 1}`);
