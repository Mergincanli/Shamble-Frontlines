// E3 (condition 5): tag counts in docs/EDITOR_V2.md, against counts written BEFORE the build.
// Read-only. Run from the repository root: node .checkpoints/b1-plan-view/e3-tags.mjs [file]
// Expected, written before any edit (2026-11-05):
//   before (main af8393e): [DECIDED] 2, [DEFAULT] 4, [OPEN] 1, [LATER] 2, [IDEA] 0
//   after (the build adds 9 tagged items: his 8 lines of 2026-11-05 + the plan view of 2026-11-03):
//         [DECIDED] 11, [DEFAULT] 4, [OPEN] 1, [LATER] 2, no other tag.
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
const KNOWN = ['DECIDED', 'DEFAULT', 'OPEN', 'LATER', 'IDEA'];
const BEFORE = { DECIDED: 2, DEFAULT: 4, OPEN: 1, LATER: 2, IDEA: 0 };
const AFTER = { DECIDED: 11, DEFAULT: 4, OPEN: 1, LATER: 2, IDEA: 0 };
function count(text) {
  const c = {}; const other = {};
  for (const m of text.matchAll(/\[([A-Z][A-Z-]+)\]/g)) {
    if (KNOWN.includes(m[1])) c[m[1]] = (c[m[1]] || 0) + 1; else other[m[1]] = (other[m[1]] || 0) + 1;
  }
  for (const k of KNOWN) c[k] = c[k] || 0;
  return { c, other };
}
function check(label, text, exp) {
  const { c, other } = count(text); let fail = 0;
  for (const k of KNOWN) { const ok = c[k] === exp[k]; if (!ok) fail++; console.log(`${label} [${k}] ${c[k]} expected ${exp[k]} ${ok ? 'ok' : 'FAIL'}`); }
  const o = Object.keys(other);
  if (o.length) { fail++; console.log(`${label} other tags: ${o.map(k => `[${k}] ${other[k]}`).join(', ')} FAIL`); }
  else console.log(`${label} other tags: none ok`);
  return fail;
}
const file = process.argv[2] || 'docs/EDITOR_V2.md';
const base = execFileSync('git', ['show', `af8393e:${file}`], { encoding: 'utf8' });
let fail = check('before(af8393e)', base, BEFORE);
fail += check('after(worktree)', readFileSync(file, 'utf8'), AFTER);
console.log(fail ? `E3 FAIL (${fail})` : 'E3 PASS');
process.exit(fail ? 1 : 0);
