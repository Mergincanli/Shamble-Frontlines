// Revision 2b (the advisor agent's ruling, item 7 and H-1): E-TASK's header in docs/DECISIONS.md is rewritten from
// decisions.mjs's new text; the old header is read from the file itself.
// The old header lines are the block from the E-TASK title line to the line before "  - His decision, word for word";
// they are replaced by the new header wrapped as decisions.mjs wraps it. Only that block changes.
import { readFileSync, writeFileSync } from 'node:fs';
import { ENTRIES } from './decisions.mjs';
const FILE = 'docs/DECISIONS.md';
function wrap(text, first, rest, width = 120) {
  const out = []; let line = first;
  for (const w of text.split(' ')) { const cand = line === first ? first + w : `${line} ${w}`;
    if (cand.length > width && line !== first) { out.push(line); line = rest + w; } else line = cand; }
  out.push(line); return out;
}
const lines = readFileSync(FILE, 'utf8').split('\n');
const e = ENTRIES.find((x) => x.key === 'E-TASK');
const start = lines.findIndex((l) => l.startsWith('- **Taken by Mustafa, 2026-11-13, 18:47:55 UTC: his decision on step 2.3'));
const end = lines.findIndex((l, i) => i > start && l === '  - His decision, word for word (A2, from inside its quote marks):');
if (start < 0 || end < 0 || end - start > 14) throw new Error('E-TASK header block not found');
const old = lines.slice(start, end).map((l) => l.trim()).join(' ');
if (!old.includes('by his words they are the rule there too')) throw new Error('the old header is not the one to fix');
lines.splice(start, end - start, ...wrap(`- ${e.head}`.slice(2), '- ', '  '));
writeFileSync(FILE, lines.join('\n'));
console.log(`E-TASK header rewritten (lines ${start + 1} to ${end})`);
