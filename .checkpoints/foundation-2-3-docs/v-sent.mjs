// Verifier (read-only): which files under .checkpoints/foundation-2-3-docs* hold, in some pushed version, a WHOLE
// sentence of his words (A1 to A4, held in memory only; a sentence = a piece of an item split at ". ", "? ", "! ", ": "
// ends or line breaks, at least 20 characters)? Prints per file: the number of distinct whole sentences found in its
// best version, the longest one's length, the items, and the first commit holding one. Never prints his words.
//   node .checkpoints/foundation-2-3-docs/v-sent.mjs
import { execFileSync } from 'node:child_process';
import { allItems } from './words.mjs';
const sentences = [];
for (const it of allItems()) {
  for (const line of it.text.split('\n')) {
    // Pieces: split at sentence ends, and also at quote marks and parentheses, so a quoted sentence counts on its own.
    for (const s of line.split(/(?<=[.!?])\s+|["()]/)) {
      const t = s.replace(/^[-\d.\s]+(?=[A-Z])/, '').trim();
      if (t.length >= 20 && /^[A-Z]/.test(t)) sentences.push([it.id, t.replace(/\s+/g, ' ')]);
    }
  }
}
const uniq = new Map(sentences.map(([id, s]) => [s, id]));
sentences.length = 0;
for (const [s, id] of uniq) sentences.push([id, s]);
const log = execFileSync('git', ['log', '--format=@%h', '--name-only', '--', '.checkpoints/foundation-2-3-docs.md', '.checkpoints/foundation-2-3-docs/'], { encoding: 'utf8', maxBuffer: 1 << 26 });
const pairs = []; let commit = null;
for (const l of log.split('\n')) { if (l.startsWith('@')) commit = l.slice(1); else if (l.trim()) pairs.push([commit, l.trim()]); }
const byFile = new Map();
for (const [c, f] of pairs.reverse()) {
  let text; try { text = execFileSync('git', ['show', `${c}:${f}`], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'], maxBuffer: 1 << 26 }).replace(/\s+/g, ' '); } catch { continue; }
  const found = sentences.filter(([, s]) => text.includes(s));
  if (!found.length) continue;
  const r = byFile.get(f) ?? { first: c, best: 0, longest: 0, items: new Set() };
  r.best = Math.max(r.best, found.length);
  for (const [id, s] of found) { r.longest = Math.max(r.longest, s.length); r.items.add(id); }
  byFile.set(f, r);
}
console.log(`${sentences.length} sentences of his (>= 20 chars); ${pairs.length} file versions; ${byFile.size} files hold a whole sentence in some version`);
for (const [f, r] of [...byFile].sort()) console.log(`${f.replace('.checkpoints/foundation-2-3-docs/', '')}: whole sentences in best version ${r.best}; longest ${r.longest}; items ${[...r.items].sort().join('+')}; first ${r.first}`);
