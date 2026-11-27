// docs-keeper (review-tracks docs pass): moves STATUS's old "Last updated" line (line 5, after the first move) to the
// archive, byte for byte, right after the block docs-keeper-move.mjs wrote; STATUS keeps a new line written after.
// Run once from the worktree root:  node .checkpoints/review-tracks/docs-keeper-move2.mjs
import { readFileSync, writeFileSync } from 'node:fs';
const S = 'docs/STATUS.md', A = 'docs/STATUS_ARCHIVE.md';
const st = readFileSync(S, 'utf8').split('\n');
if (!st[4].startsWith('_Last updated: 2026-11-27 (ci-time: the #78 row added')) throw new Error('STATUS line 5 is not the old Last updated line');
const ar = readFileSync(A, 'utf8');
const anchor = '_STATUS\'s "Waiting on Mustafa" bullet for #75, moved the same way (merged since, as the header above says):_\n\n';
const at = ar.indexOf(anchor);
if (at < 0 || ar.indexOf(anchor, at + 1) >= 0) throw new Error('anchor not found once');
const end = ar.indexOf('\n\n', at + anchor.length) + 2; // after the moved bullet and its blank line
if (ar.includes('_STATUS\'s "Last updated" line as it stood before review-tracks')) throw new Error('already moved');
const add = `_STATUS's "Last updated" line as it stood before review-tracks' docs pass, moved the same way:_\n\n${st[4]}\n\n`;
writeFileSync(A, ar.slice(0, end) + add + ar.slice(end));
st[4] = '_Last updated: 2026-11-27 (review-tracks\' docs pass, on the lane branch with main b8f02b01 merged in: the change-set applied, its pull request next; #75, #76 and #78 merged, their rows and the old line moved to the archive by script; T10 done)._';
writeFileSync(S, st.join('\n'));
console.log('moved the old Last updated line; STATUS line 5 replaced');
