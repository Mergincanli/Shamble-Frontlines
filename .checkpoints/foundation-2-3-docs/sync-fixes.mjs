// Step 4's fixes on top of docs-keeper's sync: STATUS back within 120 lines once the placeholders are filled (the kit
// bullet as pointers, the old-sections table's two "under the same names" rows as one), the archive's 2.2 block made
// true for the merge (its opening re-wrapped; three "his go asked and still waiting" lines), and REPO_MAP's new row
// taken out (the map lists only four reports; its docs/reports/ row covers this one). Exact replacements, each found once.
//   node .checkpoints/foundation-2-3-docs/sync-fixes.mjs
import { readFileSync, writeFileSync } from 'node:fs';
import { wrap, apply } from './fixes.mjs';

const KIT_OLD = [
  '- **The kit** (KIT-FREEZE-2, and what he meant by it, FREEZE-FDN:',
  '  `docs/DECISIONS.md`, "Queued"). His exceptions: a broken main, STATUS-DIET and LANE-RULES (carried out by',
  '  status-diet, #49, merged); and by FREEZE-FDN it "does not stop the smallest edits the foundation needs to keep the',
  '  kit true after its own moves" ("Each such edit still asks me and merges on my go"). His two additions of',
  '  2026-11-11 for the foundation chat: `docs/DECISIONS.md`. His words of 2026-11-13 for step 2.3 (`docs/DECISIONS.md`):',
  '  {{Q-KIT}}',
].join('\n');
const KIT_NEW = [
  ...wrap('**The kit:** KIT-FREEZE-2, what he meant by it (FREEZE-FDN) and his exceptions: `docs/DECISIONS.md`, "Queued"; ' +
    'his two additions of 2026-11-11 for the foundation chat: `docs/DECISIONS.md`. His words of 2026-11-13 for step 2.3:', '- ', '  '),
  '  {{Q-KIT}}',
].join('\n');
const ROW1 = '| The header (2-27), Phase (30-139), In flight (142-185), Targets (188-195) | `docs/STATUS_ARCHIVE.md`, under the same names |';
const ROW6 = '| Routines (949-1008), Open bugs and risks (1011-1217), Health (1220-1233) | `docs/STATUS_ARCHIVE.md`, under the same names |';
const ROW16 = '| The header (2-27), Phase (30-139), In flight (142-185), Targets (188-195), Routines (949-1008), Open bugs and risks (1011-1217), Health (1220-1233) | `docs/STATUS_ARCHIVE.md`, under the same names |';
const REPO_ROW = "| `docs/reports/2026-11-13-foundation-2-3-docs.md` | The advisor report of step 2.3's first docs pass (PR #52); its Appendix A holds Mustafa's words for it in full (A1-A3), the source its `docs/DECISIONS.md` entries are copied from |\n";

function once(text, old, neu, what) {
  const n = text.split(old).length - 1;
  if (n !== 1) throw new Error(`${what}: found ${n} times`);
  return text.replace(old, neu);
}
// Replace inside a paragraph (joined with single spaces) and re-wrap it from its first line, at the given indents.
function reflowAt(text, startNeedle, old, neu, first, cont, stop) {
  const lines = text.split('\n');
  const s = lines.findIndex((l) => l.includes(startNeedle));
  if (s < 0 || lines.filter((l) => l.includes(startNeedle)).length !== 1) throw new Error(`not once: ${startNeedle}`);
  let e = s + 1;
  while (e < lines.length && !stop(lines[e])) e++;
  const joined = lines.slice(s, e).map((l) => l.trim()).join(' ');
  if (joined.split(old).length !== 2) throw new Error(`the old text is not once in lines ${s + 1}-${e}: ${old.slice(0, 50)}`);
  lines.splice(s, e - s, ...wrap(joined.replace(old, neu), first, cont));
  return lines.join('\n');
}

let st = readFileSync('docs/STATUS.md', 'utf8');
st = once(st, KIT_OLD, KIT_NEW, 'STATUS kit bullet');
st = once(st, `${ROW1}\n`, `${ROW16}\n`, 'STATUS table row 1');
st = once(st, `${ROW6}\n`, '', 'STATUS table row 6');
writeFileSync('docs/STATUS.md', st);

let ar = readFileSync('docs/STATUS_ARCHIVE.md', 'utf8');
const ARCH = [
  { id: 'G0: his go came', kind: 'reflow', start: "asked in the report's \"Decisions needed\" (ADR-INDEX), is to come with PR #48's merge", stop: (l) => l.startsWith('    - '), old: "is to come with PR #48's merge and is still waiting;", repl: "came with PR #48's merge (his ADR-INDEX (a), 2026-11-13: `docs/DECISIONS.md`);", cont: '      ' },
  { id: 'OVERFLOW-XCRATE-CI: his go given', kind: 'reflow', start: 'register row OVERFLOW-ALL, his go asked and still waiting), so every workspace crate', stop: (l) => l.startsWith('    - '), old: 'his go asked and still waiting', repl: "his go given with PR #48's merge: his OVERFLOW-ALL (a), 2026-11-13", cont: '      ' },
  { id: 'OVERFLOW-PER-CRATE: his go given', kind: 'reflow', start: 'reopens it (the register row OVERFLOW-ALL, his go', stop: (l) => l.startsWith('    - ') || l === '', old: 'his go asked and still waiting', repl: "his go given with PR #48's merge: his OVERFLOW-ALL (a), 2026-11-13", cont: '      ' },
];
for (const e of ARCH) { const r = apply(ar, e); ar = r.text; console.log(`archive, ${e.id}: ${r.done}`); }
// The 2.2 block's opening (140 characters after docs-keeper's edit): split once at the last space within 120, so the
// paragraph's other lines stay as they are.
{
  const lines = ar.split('\n');
  const i = lines.findIndex((l) => l.startsWith('- **From foundation 2.2** (PR #48, merged as 4d21c1a on 2026-11-13:'));
  if (i < 0 || lines[i].length <= 120) throw new Error('the 2.2 opening is not found, or already short');
  const cut = lines[i].lastIndexOf(' ', 120);
  lines.splice(i, 1, lines[i].slice(0, cut), `  ${lines[i].slice(cut + 1)}`);
  ar = lines.join('\n');
  console.log(`archive, the 2.2 opening split at column ${cut}`);
}
writeFileSync('docs/STATUS_ARCHIVE.md', ar);

let rm = readFileSync('docs/REPO_MAP.md', 'utf8');
rm = once(rm, REPO_ROW, '', 'REPO_MAP row');
writeFileSync('docs/REPO_MAP.md', rm);
console.log('OK: STATUS kit bullet and table; the archive\'s 2.2 opening and three lines; REPO_MAP row out');
