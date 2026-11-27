// docs-keeper (review-tracks docs pass): moves STATUS's lines for three merged pull requests to the end of the archive's
// "In flight" by line ranges, byte for byte (his STATUS-DIET: "Move text with a script by line ranges, never by
// retyping"). Each line's start is asserted first. Run once from the worktree root, before any other STATUS edit:
//   node .checkpoints/review-tracks/docs-keeper-move.mjs
import { readFileSync, writeFileSync } from 'node:fs';
const S = 'docs/STATUS.md', A = 'docs/STATUS_ARCHIVE.md';
const st = readFileSync(S, 'utf8').split('\n');
const want = { 34: '| none yet | MAIN-WALL (his words', 35: '| #75 (draft) | change-set 4b-1', 36: '| #78 (draft) | CI-TIME', 49: '- 4b-1 (#75), reviewed (its report): his go.' };
for (const [n, p] of Object.entries(want)) if (!st[n - 1].startsWith(p)) throw new Error(`STATUS line ${n} does not start with ${p}`);
const rows = [st[33], st[34], st[35]], bullet = st[48];
const block = [
  '_Moved from STATUS (2026-11-27, review-tracks\' docs pass, under his OLD-LINES of 2026-11-18): STATUS\'s rows for three merged pull requests, moved by script, byte for byte, so their cells read as they stood when they left STATUS: MAIN-WALL\'s docs pull request (PR #76, merge commit 2671595f, 2026-11-26 18:00:25 UTC), change-set 4b-1 (PR #75, merge commit b9aad019, 2026-11-26 18:53:13 UTC) and CI-TIME (PR #78, merge commit b8f02b01, 2026-11-27 13:55:47 UTC), each by its merge commit\'s date read with git:_',
  '',
  '| PR | Work | Branch | State | Waiting on |',
  '|---|---|---|---|---|',
  ...rows,
  '',
  '_STATUS\'s "Waiting on Mustafa" bullet for #75, moved the same way (merged since, as the header above says):_',
  '',
  bullet,
  '',
];
const ar = readFileSync(A, 'utf8');
const at = ar.indexOf('\n## Targets\n');
if (at < 0 || ar.indexOf('\n## Targets\n', at + 1) >= 0) throw new Error('archive: "## Targets" not found once');
if (ar.includes('review-tracks\' docs pass, under his OLD-LINES')) throw new Error('already moved');
writeFileSync(A, ar.slice(0, at + 1) + block.join('\n') + '\n' + ar.slice(at + 1));
const keep = st.filter((_, i) => ![33, 34, 35, 48].includes(i));
writeFileSync(S, keep.join('\n'));
console.log(`moved 3 rows and 1 bullet; STATUS ${st.length} -> ${keep.length} lines`);
