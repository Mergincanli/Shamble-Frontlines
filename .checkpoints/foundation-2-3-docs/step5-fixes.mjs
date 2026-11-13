// Step 5's fixes from the advisor agent's ruling on step 4 (plan revision 2b): STATUS's "Last merged" with the archive
// pointer restored (its exact two lines), the REPO_MAP claim dropped from STATUS's #52 row and ROADMAP's #52 entry, and
// the optional Low in ROADMAP's #48 entry ("his go asked, then given with the merge"). Exact replacements, each found
// once; ROADMAP's paragraphs re-wrapped only from the edited line until the old line breaks resume (fixes.mjs's apply).
//   node .checkpoints/foundation-2-3-docs/step5-fixes.mjs
import { readFileSync, writeFileSync } from 'node:fs';
import { apply } from './fixes.mjs';

function once(text, old, neu, what) {
  const n = text.split(old).length - 1;
  if (n !== 1) throw new Error(`${what}: found ${n} times`);
  return text.replace(old, neu);
}
const LAST_OLD = '- **Last merged:** foundation 2.2, the toolchain (PR #48, 4d21c1a, 2026-11-13 16:20:12 UTC, by merge commit, on his\n' +
  "  go, quoted in the merge commit's message, after the advisor agent's APPROVE of 9b20b67). Every merge: ROADMAP.";
const LAST_NEW = '- **Last merged:** foundation 2.2, the toolchain (PR #48, 4d21c1a, 2026-11-13 16:20:12 UTC, by merge commit, on his go,\n' +
  '  quoted in its message, after the advisor agent\'s APPROVE of 9b20b67). Every merge: ROADMAP; the archive\'s "In flight".';

let st = readFileSync('docs/STATUS.md', 'utf8');
st = once(st, LAST_OLD, LAST_NEW, 'STATUS Last merged');
st = once(st, 'STATUS, ROADMAP, the archive and REPO_MAP true for main 4d21c1a)', 'STATUS, ROADMAP and the archive true for main 4d21c1a)', 'STATUS #52 row');
writeFileSync('docs/STATUS.md', st);

let rm = readFileSync('docs/ROADMAP.md', 'utf8');
for (const e of [
  { id: 'ROADMAP #52 entry', kind: 'reflow', start: 'two pointing clauses; STATUS, ROADMAP, the archive and REPO_MAP made true for main', stop: (l) => l === '', old: 'STATUS, ROADMAP, the archive and REPO_MAP made true for main 4d21c1a.', repl: 'STATUS, ROADMAP and the archive made true for main 4d21c1a.', cont: '' },
  { id: 'ROADMAP #48 entry (L-x)', kind: 'reflow', start: "crate on the advisor agent's ruling (RR-1, High; his go asked, OVERFLOW-ALL)", stop: (l) => l === '', old: 'his go asked, OVERFLOW-ALL', repl: 'his go asked, then given with the merge, OVERFLOW-ALL', cont: '' },
]) { const r = apply(rm, e); rm = r.text; console.log(`${e.id}: ${r.done}`); }
writeFileSync('docs/ROADMAP.md', rm);
const widths = LAST_NEW.split('\n').map((l) => l.length);
console.log(`OK: STATUS Last merged (${widths.join(' and ')} columns) and #52 row; ROADMAP #52 and #48 entries`);
