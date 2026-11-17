// The verifier's comparison (cs0-verifier) of ADR 0005's pass-mark section, as committed at a ref, against
// test-engineer's file (sections 1 and 3) and against the base. Prints only verdicts and the differing words.
//   node .checkpoints/cs0-model-rules/verifier-marks-compare.mjs <ref> <baseRef>
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
const [, , REF = 'HEAD', BASE = 'c94c81b'] = process.argv;
const A5 = 'docs/adr/0005-content-abilities-assets.md';
const show = (r) => execFileSync('git', ['show', `${r}:${A5}`], { maxBuffer: 1 << 26 }).toString('utf8');
const head = show(REF), base = show(BASE);
const draft = readFileSync('.checkpoints/cs0-model-rules/adr0005-marks.draft.md', 'utf8').split('\n');
const flat = (s) => s.replace(/\s+/g, ' ').trim();
// section 1: the group row and ten rows
const s1 = draft.filter((l) => l.startsWith('| **The export checks') || /^\| PM-0005-(4[7-9]|5[0-6]) \|/.test(l));
const hl = head.split('\n');
let rowsOk = 0;
for (const r of s1) {
  const id = (r.match(/^\| (PM-0005-\d+|\*\*The export checks)/) || [])[1];
  const got = hl.filter((l) => l.startsWith(`| ${id}`));
  const eq = got.length === 1 && got[0] === r;
  if (eq) rowsOk++; else console.log('ROW DIFFERS:', id, 'found', got.length);
}
console.log('section 1 rows equal byte for byte:', rowsOk, 'of', s1.length);
// section 3: quoted blocks (a), (b), (c) first and third insert, (d)
const sec3 = draft.slice(draft.findIndex((l) => l.startsWith('## 3.')), draft.findIndex((l) => l.startsWith('## 4.')));
const blocks = []; let cur = null;
for (const l of sec3) {
  if (l.startsWith('> ')) { cur = cur ?? []; cur.push(l.slice(2)); } else if (cur) { blocks.push(flat(cur.join(' '))); cur = null; }
}
if (cur) blocks.push(flat(cur.join(' ')));
const H = flat(head);
const wordsDiff = (a, b) => { // words of a missing from b, in order (LCS-free greedy scan) and extra words
  const A = a.split(' '), B = b.split(' '); let i = 0, j = 0; const miss = [], extra = [];
  while (i < A.length && j < B.length) {
    if (A[i] === B[j]) { i++; j++; continue; }
    const k = B.indexOf(A[i], j);
    if (k >= 0 && k - j < 12) { extra.push(B.slice(j, k).join(' ')); j = k; } else { miss.push(A[i]); i++; }
  }
  return { miss: miss.concat(A.slice(i)), extra: extra.concat(B.slice(j).length ? [B.slice(j).join(' ')] : []) };
};
blocks.forEach((b, n) => {
  const inHead = H.includes(b);
  console.log(`section 3 quoted block ${n + 1} (${b.slice(0, 40)}...): ${inHead ? 'present verbatim' : 'NOT verbatim'}`);
  if (!inHead) { // find the closest window in head by its first 8 words and show the word difference
    const first = b.split(' ').slice(0, 8).join(' '); const at = H.indexOf(first);
    if (at >= 0) { const win = H.slice(at, at + b.length + 200); const d = wordsDiff(b, win.slice(0, b.length + 80));
      console.log('   words of the draft missing at the head:', JSON.stringify(d.miss.slice(0, 20)), '| extra words at the head:', JSON.stringify(d.extra.filter(Boolean).slice(0, 5))); }
    else console.log('   its first words are not found at the head');
  }
});
// (c) second change: the bracket
const bracket = "graphics-engineer for the GPU section's parts and the export rules (AER), engine-architect for the joints (JL), and the model's author when a file breaks a rule that stands)";
console.log('(c) bracket extension present:', H.includes(flat(bracket)));
// nothing else in the section changed: every base word of the section stays, in order, at the head
const start = (t) => t.indexOf('Written by test-engineer (plan step 4).');
const end = (t) => t.indexOf('## ', start(t) + 10);
const bS = flat(base.slice(start(base), end(base))), hS = flat(head.slice(start(head), end(head)));
const d = wordsDiff(bS, hS);
console.log('pass-mark section: base words missing at the head:', d.miss.length, JSON.stringify(d.miss.slice(0, 30)));
