// docs-keeper (ci-after-3a): ROADMAP's "Recent work" gets ci-after-3a's entry on top, and its oldest entry
// (docs-after-54) moves byte for byte into "Build history", after the newest entry folded there before it.
// Run once from the worktree root: node .checkpoints/ci-after-3a/docs-keeper-roadmap.mjs
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const F = 'docs/ROADMAP.md';
const L = readFileSync(F, 'utf8').split('\n');
const one = (pred, what) => { const i = L.map((l, n) => (pred(l) ? n : -1)).filter((n) => n >= 0); if (i.length !== 1) throw new Error(what + ' found ' + i.length); return i[0]; };
// 1. The oldest entry: from its title line to the end of the file (it is the last entry).
const s = one((l) => l.startsWith("**Main's docs after #52, #53 and #54 [2026-11-17, MERGED]**"), 'oldest entry');
let e = L.length; while (e > s && L[e - 1] === '') e--;
if (L[s - 1] !== '') throw new Error('no blank line above the oldest entry');
const moved = L.slice(s, e);
L.splice(s - 1, e - s + 1); // the blank line above it and the entry
// 2. Into "Build history": right after the night-shift-stints paragraph, before the KEPT-LOGS bullet.
const k = one((l) => l.startsWith('- **Kit: KEPT-LOGS (2026-11-15):**'), 'KEPT-LOGS bullet');
if (L[k - 1] !== '') throw new Error('no blank line above KEPT-LOGS');
L.splice(k - 1, 0, '', ...moved);
// 3. The new entry on top of "Recent work".
const r = one((l) => l === '*(Newest first, a few lines each. Older items fold into "Build history" above.)*', 'Recent work note');
if (L[r + 1] !== '') throw new Error('no blank line under the note');
const entry = [
  '**The small CI pull request after change-set 3a [2026-11-24 to 11-25, waiting for his go]** — ci-after-3a, by the',
  "chat he calls CI Builder (risk tier 2; CI and the test configuration only): the `toolchain` job runs every crate's",
  'release tests (E1), checks the committed vector files against their two generators (3A-GEN-CHECK), builds `devtools`',
  "for wasm32 on a line of its own (3A-RULES-STEP's CI half) and runs the rule tests in the runner image's Mozilla",
  'Firefox 160.0 (FIREFOX-TESTS (b), his FIREFOX-SOURCE (c)). The first GitHub run was red as he had foreseen (the',
  "runner's Firefox gives the page no WebGL), so by his rule decided in advance Firefox runs the rule tests only and its",
  'drawing tests are a follow-up (FF-DRAW). Green on code head 832268e (Chromium 16 passed, Firefox 2). Report',
  "`docs/reports/2026-11-25-ci-after-3a.md`; waits for the advisor agent's review of the final head and his go.",
  '',
];
L.splice(r + 2, 0, ...entry);
writeFileSync(F, L.join('\n'));
const back = readFileSync(F, 'utf8');
console.log(`moved ${moved.length} lines (sha256 ${sha(moved.join('\n'))}); found again whole: ${back.includes(moved.join('\n'))}`);
