// R-WORDS scratch (read-only): every straight-quoted span on the lines this PR adds to STATUS, ROADMAP, the archive
// and REPO_MAP, with where it is found (A2, A3, DECISIONS.md at the head, the file's base text) and 90 characters of
// context before it, so a person can judge whether the span is attributed rightly. Prints only; writes nothing.
//   node .checkpoints/foundation-2-3-docs/rw-spans.mjs
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { messageText, ITEMS, go48 } from './words.mjs';

const BASE = '4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300';
const FILES = ['docs/STATUS.md', 'docs/ROADMAP.md', 'docs/STATUS_ARCHIVE.md', 'docs/REPO_MAP.md'];
const norm = (s) => s.replace(/\s+/g, ' ').trim();
const git = (a) => execFileSync('git', a, { encoding: 'utf8', maxBuffer: 1 << 26 });
const src = { A2: norm(messageText(ITEMS[1].uuid)), A3: norm(go48()), DECISIONS: norm(readFileSync('docs/DECISIONS.md', 'utf8')) };
let n = 0;
for (const f of FILES) {
  const base = norm(git(['show', `${BASE}:${f}`]));
  const d = git(['diff', '--no-color', '-U0', BASE, '--', f]).split('\n');
  const blocks = [];
  let cur = null;
  for (const l of d) {
    if (l.startsWith('@@')) { if (cur) blocks.push(cur); cur = { at: l.split('@@')[1].trim(), lines: [] }; continue; }
    if (cur && l.startsWith('+') && !l.startsWith('+++')) cur.lines.push(l.slice(1));
  }
  if (cur) blocks.push(cur);
  for (const b of blocks.filter((x) => x.lines.length)) {
    const text = norm(b.lines.join(' '));
    for (const m of text.matchAll(/"([^"]+)"/g)) {
      n++;
      const span = norm(m[1]);
      const where = Object.entries({ ...src, base }).filter(([, t]) => t.includes(span)).map(([k]) => k);
      const ctx = text.slice(Math.max(0, m.index - 90), m.index);
      console.log(`#${n} ${f} [${b.at}] in: ${where.join(',') || 'NONE'} | ...${ctx}| "${span.slice(0, 160)}${span.length > 160 ? '...' : ''}"`);
    }
  }
}
