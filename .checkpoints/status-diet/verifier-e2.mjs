// Verifier's probe of the E2 nets: which lines of his that sit outside DECISIONS.md would the nets miss?
// Usage: node verifier-e2.mjs   (repository root)
import { execFileSync } from 'node:child_process';
import { RA, RB, RC, RM, MANUAL } from './split.mjs';
const git = (...a) => execFileSync('git', a, { encoding: 'utf8', maxBuffer: 1 << 27 });
const old = git('show', '2d22b69:docs/STATUS.md').replace(/\n$/, '').split('\n');
const dec = new Set(git('show', '8ead8f1:docs/DECISIONS.md').split('\n'));
const sta = new Set(git('show', '8ead8f1:docs/STATUS.md').split('\n'));
// Old lines that are outside DECISIONS (archive or STATUS) and look like his words or decisions.
const HIS = /\bhis (?:words|answer|answers|go|decision|choice|direction|item|items|addition|ruling|list|note|reconfirm)|\bMustafa\b|"merge #\d+"|in his words|word for word/;
let n = 0, notRM = [];
for (let i = 0; i < old.length; i++) {
  const t = old[i];
  if (dec.has(t) && !sta.has(t)) continue; // in DECISIONS (approximate by text)
  if (!HIS.test(t)) continue;
  n++;
  const caught = RM.test(t) || MANUAL[i + 1] || RA.test(t);
  RA.lastIndex = 0;
  if (!caught) notRM.push(i + 1);
}
console.log(`lines outside DECISIONS that mention Mustafa or his words/answers/go: ${n}; not caught by RM, MANUAL or net a: ${notRM.length}`);
for (const i of notRM) console.log(`  ${i}: ${old[i - 1].slice(0, 150)}`);
// Net soundness: a decision code without a hyphen and without an option letter that does not lead a list entry.
const bare = new Map();
for (let i = 0; i < old.length; i++) for (const m of old[i].matchAll(/\b[A-Z][A-Z0-9]{3,}\b/g)) {
  const w = m[0]; if (/-/.test(w)) continue; bare.set(w, (bare.get(w) || 0) + 1);
}
console.log('bare capital words (4+ letters, no hyphen), top 40:', [...bare].sort((a, b) => b[1] - a[1]).slice(0, 40).map(x => x.join(':')).join(' '));
