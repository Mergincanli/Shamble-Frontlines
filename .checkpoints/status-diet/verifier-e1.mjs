// Verifier's independent E1 check (does not import split.mjs or read the templates).
// Usage: node verifier-e1.mjs <rev>   run from the repository root
import { execFileSync } from 'node:child_process';
const git = (...a) => execFileSync('git', a, { encoding: 'utf8', maxBuffer: 1 << 27 });
const rev = process.argv[2] || '8ead8f1';
const old = git('show', '2d22b69:docs/STATUS.md');
const oldL = old.replace(/\n$/, '').split('\n');
const FILES = ['STATUS.md', 'DECISIONS.md', 'STATUS_ARCHIVE.md'];
const newL = [];
const perFile = {};
for (const f of FILES) {
  const t = git('show', `${rev}:docs/${f}`);
  if (!t.endsWith('\n')) console.log(`note: ${f} has no final newline`);
  if (t.includes('\r')) console.log(`note: ${f} contains CR`);
  const L = t.replace(/\n$/, '').split('\n');
  perFile[f] = L.length;
  for (const x of L) newL.push(x);
}
const count = (arr) => { const m = new Map(); for (const x of arr) m.set(x, (m.get(x) || 0) + 1); return m; };
const co = count(oldL), cn = count(newL);
const STRUCT = [1, 28, 29, 140, 141, 186, 187, 196, 197, 947, 948, 1009, 1010, 1218, 1219];
// Expected: for each distinct old line text, new count >= old count, except structure lines.
const structText = count(STRUCT.map(i => oldL[i - 1]));
let short = [];
for (const [t, c] of co) {
  const need = c - (structText.get(t) || 0);
  const have = cn.get(t) || 0;
  if (have < need) short.push([t, need, have]);
}
// Lines in new files that are not old lines (new text), and old texts present more often than in old (duplicates)
let newOnly = 0, extra = [];
for (const [t, c] of cn) {
  const o = co.get(t) || 0;
  if (!o) newOnly += c;
  else if (c > o - (structText.get(t) || 0) && t !== '') extra.push([t.slice(0, 90), o, c]);
}
console.log(`rev ${rev}: old ${oldL.length} lines (${Buffer.byteLength(old)} bytes); new ${newL.length} lines (${FILES.map(f => f + ' ' + perFile[f]).join(', ')})`);
console.log(`structure texts: ${[...structText].map(([t, c]) => (t || '(blank)') + ' x' + c).join(' | ')}`);
console.log(`old line texts short in new files: ${short.length}`);
for (const [t, n, h] of short.slice(0, 20)) console.log(`  SHORT need ${n} have ${h}: ${t.slice(0, 120)}`);
console.log(`new lines whose text never occurs in old: ${newOnly}`);
console.log(`non-blank old texts occurring more often in new than moved (new text repeating an old line, or a duplicate move): ${extra.length}`);
for (const [t, o, c] of extra.slice(0, 20)) console.log(`  old ${o} new ${c}: ${t}`);
// Byte-level: total bytes of moved non-structure lines
const movedBytes = oldL.filter((_, i) => !STRUCT.includes(i + 1)).reduce((a, t) => a + Buffer.byteLength(t) + 1, 0);
console.log(`bytes of the 1218 moved lines incl. newline: ${movedBytes}`);
// Order check: within each new file, the moved old lines appear as runs; report the runs (old line ranges) per file
for (const f of FILES) {
  const L = git('show', `${rev}:docs/${f}`).replace(/\n$/, '').split('\n');
  // greedy: map each new line to the next old index matching contiguous run
  const runs = []; let i = 0;
  while (i < L.length) {
    let best = null;
    // find old index k with oldL[k]===L[i] and longest run
    for (let k = 0; k < oldL.length; k++) if (oldL[k] === L[i] && L[i] !== '') {
      let n = 0; while (i + n < L.length && k + n < oldL.length && oldL[k + n] === L[i + n]) n++;
      if (!best || n > best[1]) best = [k, n];
    }
    if (best && best[1] >= 1) { runs.push(`${best[0] + 1}-${best[0] + best[1]}`); i += best[1]; } else i++;
  }
  console.log(`${f} old runs: ${runs.join(' ')}`);
}
