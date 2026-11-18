// docs-keeper-khc: proves RECORD-FILES held in the kit hold's docs pass (PR #60). For every line removed or changed in
// the four record files (the working tree against main), it blames main's line: the line must be this lane's (written
// by a commit of #56, #57 or #58) or reappear byte for byte as an added line (a move, OLD-LINES). Run from the repository
// root: node .checkpoints/kit-hold-ci/docs-keeper-blame-check.mjs [base, default origin/main]. Exit 1 on any other line.
import { execFileSync } from 'node:child_process';

const base = process.argv[2] ?? 'origin/main';
const git = (...a) => execFileSync('git', a, { encoding: 'utf8', maxBuffer: 64 << 20 });
const files = ['docs/STATUS.md', 'docs/STATUS_ARCHIVE.md', 'docs/ROADMAP.md', 'docs/REPO_MAP.md'];
// This lane's commits: #56 (merge d7dad97: c94c81b..cd61b28), #57 (merge 22c6afa: d7dad97..f26ac5b) and #58 (merge
// 9b4b518: 22c6afa..1bfe462). Adapted from git show 76df3f3:.checkpoints/cs1-types/docs-keeper-blame-check.mjs.
const own = new Set([
  ...git('rev-list', 'c94c81b..cd61b28').split('\n').filter(Boolean),
  ...git('rev-list', 'd7dad97..f26ac5b').split('\n').filter(Boolean),
  ...git('rev-list', '22c6afa..1bfe462').split('\n').filter(Boolean),
]);
const removed = [], added = [];
for (const f of files) {
  let old = 0;
  for (const l of git('diff', '-U0', base, '--', f).split('\n')) {
    const h = /^@@ -(\d+)(?:,\d+)? \+\d+(?:,\d+)? @@/.exec(l);
    if (h) { old = Number(h[1]); continue; }
    if (l.startsWith('---') || l.startsWith('+++')) continue;
    if (l.startsWith('-')) removed.push({ f, n: old++, text: l.slice(1) });
    else if (l.startsWith('+')) added.push(l.slice(1));
  }
}
const addedCount = new Map();
for (const t of added) addedCount.set(t, (addedCount.get(t) ?? 0) + 1);
let bad = 0;
for (const r of removed) {
  const sha = git('blame', '--porcelain', '-L', `${r.n},${r.n}`, base, '--', r.f).split(' ')[0];
  const moved = r.text !== '' && (addedCount.get(r.text) ?? 0) > 0;
  if (moved) addedCount.set(r.text, addedCount.get(r.text) - 1);
  const verdict = r.text === '' ? 'blank separator' : own.has(sha) ? 'own line (this lane)' : moved
    ? 'moved byte for byte' : 'NOT THIS LANE AND NOT MOVED';
  if (verdict.startsWith('NOT')) bad++;
  process.stdout.write(`${r.f}:${r.n} ${sha.slice(0, 7)} ${verdict}${own.has(sha) && moved ? ' (and moved)' : ''}` +
    ` | ${r.text.slice(0, 60)}\n`);
}
process.stdout.write(`${removed.length} removed lines checked; ${bad} not this lane's and not moved\n`);
process.exit(bad ? 1 : 0);
