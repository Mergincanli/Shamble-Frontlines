// docs-keeper (ci-after-3a): moves STATUS's "In flight" rows of the merged pull requests #67 and #69 to the archive's
// "In flight", byte for byte (his OLD-LINES of 2026-11-18: lines about merged pull requests move by script, never
// reworded). Run once from the worktree root: node .checkpoints/ci-after-3a/docs-keeper-move-rows.mjs
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const S = 'docs/STATUS.md', A = 'docs/STATUS_ARCHIVE.md';
const st = readFileSync(S, 'utf8').split('\n');
const ar = readFileSync(A, 'utf8').split('\n');
const pick = (p) => { const i = st.map((l, n) => (l.startsWith(p) ? n : -1)).filter((n) => n >= 0); if (i.length !== 1) throw new Error(p + ' found ' + i.length); return i[0]; };
const i67 = pick('| #67 |'), i69 = pick('| #69 |');
const rows = [st[i67], st[i69]];
const end = ar.indexOf("| Phase 0 setup | merged (2d30e4a) | PARTIAL | Mustafa's Windows run |");
if (end < 0 || ar[end + 1] !== '' || ar[end + 2] !== '## Targets') throw new Error('archive anchor not as expected');
if (ar.some((l) => l === rows[0] || l === rows[1])) throw new Error('already moved');
const block = ['',
  "_Moved from STATUS (2026-11-25, ci-after-3a's docs pass, under his OLD-LINES of 2026-11-18): STATUS's rows for two merged pull requests, moved by script, byte for byte, so their cells read as they stood when they left STATUS. Both merged on 2026-11-25 (read with `git log --first-parent` on main): #69 as 311c048, #67 as c1da1bd._",
  '', '| PR | Work | Branch | State | Waiting on |', '|---|---|---|---|---|', ...rows];
ar.splice(end + 1, 0, ...block);
for (const i of [i69, i67].sort((a, b) => b - a)) st.splice(i, 1);
writeFileSync(S, st.join('\n'));
writeFileSync(A, ar.join('\n'));
for (const [n, r] of [[i67 + 1, rows[0]], [i69 + 1, rows[1]]]) console.log(`moved STATUS line ${n}: ${Buffer.byteLength(r)} bytes, sha256 ${sha(r)}`);
const back = readFileSync(A, 'utf8').split('\n');
console.log('in the archive at lines', back.indexOf(rows[0]) + 1, back.indexOf(rows[1]) + 1, 'equal:', back.includes(rows[0]) && back.includes(rows[1]));
