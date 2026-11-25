// docs-keeper (ci-after-3a): moves STATUS's "Waiting on Mustafa" bullet about the merged change-set 3b (#68, 3ab9eae)
// to the archive, byte for byte, under the "Moved from STATUS" note this docs pass added to the archive's "In flight"
// (his OLD-LINES of 2026-11-18). Run once from the worktree root: node .checkpoints/ci-after-3a/docs-keeper-move-wait.mjs
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const S = 'docs/STATUS.md', A = 'docs/STATUS_ARCHIVE.md';
const st = readFileSync(S, 'utf8').split('\n');
const ar = readFileSync(A, 'utf8').split('\n');
const s = st.findIndex((l) => l.startsWith('- Change-set 3b: his go for the merge of exactly its last commit'));
if (s < 0 || st.filter((l) => l.startsWith('- Change-set 3b: his go')).length !== 1) throw new Error('bullet not found once');
let e = s + 1; while (st[e].startsWith('  ')) e++;
const moved = st.slice(s, e);
const rowLast = ar.findIndex((l) => l.startsWith('| #69 |'));
if (rowLast < 0 || ar[rowLast + 1] !== '' || ar[rowLast + 2] !== '## Targets') throw new Error('archive anchor not as expected');
if (ar.includes(moved[0])) throw new Error('already moved');
ar.splice(rowLast + 1, 0, '',
  "_And STATUS's \"Waiting on Mustafa\" bullet for change-set 3b's merge (#68, merged 2026-11-25 as 3ab9eae), moved by the same docs pass and rule, byte for byte:_",
  '', ...moved);
st.splice(s, e - s);
writeFileSync(S, st.join('\n'));
writeFileSync(A, ar.join('\n'));
console.log(`moved STATUS lines ${s + 1}-${e} (${moved.length} lines, sha256 ${sha(moved.join('\n'))}); found again whole: ${readFileSync(A, 'utf8').includes(moved.join('\n'))}`);
