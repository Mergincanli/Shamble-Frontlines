// The verifier's F-V3: every version of every file ever under .checkpoints/foundation-2-3-docs* (the checkpoint and the
// notes folder), as pushed, checked for runs of 30 or more characters of his words (A1 to A4, whitespace-normalised).
// Prints, per file holding any: the items it holds, the most 30-character windows in one version, the first commit
// with a hit, and whether the current tree's version still holds any. Never prints his words.
//   node .checkpoints/foundation-2-3-docs/fp3-fragments-all.mjs
import { execFileSync } from 'node:child_process';
import { readFileSync, existsSync } from 'node:fs';
import { allItems } from './words.mjs';
const W = 30;
const items = allItems().map((it) => { const t = it.text.replace(/\s+/g, ' '); const ws = new Set(); for (let i = 0; i + W <= t.length; i++) ws.add(t.slice(i, i + W)); return [it.id, ws]; });
// Per item: the number of matching 30-character windows and the longest run (consecutive matching offsets + 29).
function hits(text) {
  const t = text.replace(/\s+/g, ' '); const out = {};
  for (const [id, ws] of items) {
    let n = 0, run = 0, longest = 0;
    for (let i = 0; i + W <= t.length; i++) { if (ws.has(t.slice(i, i + W))) { n++; run++; longest = Math.max(longest, run); } else run = 0; }
    if (n) out[id] = { n, longest: longest + W - 1 };
  }
  return out;
}
const log = execFileSync('git', ['log', '--format=@%h', '--name-only', '--', '.checkpoints/foundation-2-3-docs.md', '.checkpoints/foundation-2-3-docs/'], { encoding: 'utf8', maxBuffer: 1 << 26 });
const pairs = []; let commit = null;
for (const l of log.split('\n')) { if (l.startsWith('@')) commit = l.slice(1); else if (l.trim()) pairs.push([commit, l.trim()]); }
const byFile = new Map();
for (const [c, f] of pairs.reverse()) {
  let text; try { text = execFileSync('git', ['show', `${c}:${f}`], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'], maxBuffer: 1 << 26 }); } catch { continue; }
  const h = hits(text); if (!Object.keys(h).length) continue;
  const r = byFile.get(f) ?? { first: c, items: new Set(), max: 0, longest: 0, versions: 0 };
  r.versions++; for (const [id, v] of Object.entries(h)) { r.items.add(id); r.max = Math.max(r.max, v.n); r.longest = Math.max(r.longest, v.longest); }
  byFile.set(f, r);
}
console.log(`${pairs.length} file versions checked; ${byFile.size} files hold a run of ${W} characters or more of his words in some version`);
for (const [f, r] of [...byFile].sort()) {
  const now = existsSync(f) ? Object.keys(hits(readFileSync(f, 'utf8'))).join('+') || 'none' : 'deleted';
  console.log(`${f.replace('.checkpoints/foundation-2-3-docs/', '')}: ${[...r.items].sort().join('+')}; longest run ${r.longest}; most windows in one version ${r.max}; first ${r.first}; versions with hits ${r.versions}; now ${now}`);
}
