// The quote-span check (the advisor tool's step-4 item): every span in straight double quotes on the lines this change
// adds to STATUS, ROADMAP, the archive and REPO_MAP (against the base 4d21c1a) must be found, whitespace-normalised, in
// his words (A2, A3), in the file's own base text, or in DECISIONS.md as it stands (entry titles, his older words).
// Spans found nowhere are listed for a person to judge; an odd number of quote marks in a block is listed too.
//   node .checkpoints/foundation-2-3-docs/quotes.mjs
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { messageText, ITEMS, go48 } from './words.mjs';

const BASE = '4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300';
const FILES = ['docs/STATUS.md', 'docs/ROADMAP.md', 'docs/STATUS_ARCHIVE.md', 'docs/REPO_MAP.md'];
const norm = (s) => s.replace(/\s+/g, ' ').trim();

function addedBlocks(file) {
  const d = execFileSync('git', ['diff', '--no-color', '-U0', BASE, '--', file], { encoding: 'utf8' }).split('\n');
  const blocks = [];
  let cur = null;
  for (const l of d) {
    if (l.startsWith('@@')) { if (cur) blocks.push(cur); cur = { at: l, lines: [] }; continue; }
    if (cur && l.startsWith('+') && !l.startsWith('+++')) cur.lines.push(l.slice(1));
  }
  if (cur) blocks.push(cur);
  return blocks.filter((b) => b.lines.length);
}

const sources = {
  A2: norm(messageText(ITEMS[1].uuid)),
  A3: norm(go48()),
  DECISIONS: norm(readFileSync('docs/DECISIONS.md', 'utf8')),
};
let unmatched = 0;
let total = 0;
for (const f of FILES) {
  const base = norm(execFileSync('git', ['show', `${BASE}:${f}`], { encoding: 'utf8' }));
  for (const b of addedBlocks(f)) {
    const text = norm(b.lines.join(' '));
    const marks = (text.match(/"/g) || []).length;
    if (marks % 2) console.log(`${f} ${b.at}: an odd number of quote marks (${marks}); judge by hand`);
    for (const m of text.matchAll(/"([^"]+)"/g)) {
      total++;
      const span = norm(m[1]);
      const where = Object.entries({ ...sources, base }).filter(([, t]) => t.includes(span)).map(([k]) => k);
      if (!where.length) { unmatched++; console.log(`${f} ${b.at}: NOT FOUND: "${span.slice(0, 140)}${span.length > 140 ? '...' : ''}"`); }
    }
  }
}
console.log(`${unmatched ? 'CHECK' : 'OK'}: ${total} quoted spans on the added lines; ${unmatched} found in none of A2, A3, the base file or DECISIONS.md`);
process.exit(unmatched ? 1 : 0);
