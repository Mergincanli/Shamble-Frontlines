// docs-keeper-cs2: proves RECORD-FILES held in change-set 2's docs pass (PR #61). For every line removed or changed in
// the four record files (the working tree against main), it blames main's line: the line must be this lane's (written
// by a commit of #56, #57, #58 or #60), reappear byte for byte as an added line (a move, OLD-LINES), or be one of the
// map rows this pass corrected under REC-OLD (a) (Mustafa, 2026-11-18, 20:20:09 UTC: a line that describes code that
// has changed, also another lane's; each listed with its old and new text in docs-keeper-moved-lines.md, the old text
// kept in docs-keeper-repomap-old.json). Run from the repository root:
//   node .checkpoints/cs2-first-page/docs-keeper-blame-check.mjs [base, default origin/main]
// Exit 1 on any other line. Adapted from
// git show f167f061a9b0b694836c272e397c90c815d7829f:.checkpoints/kit-hold-ci/docs-keeper-blame-check.mjs.
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';

const base = process.argv[2] ?? 'origin/main';
const git = (...a) => execFileSync('git', a, { encoding: 'utf8', maxBuffer: 64 << 20 });
const files = ['docs/STATUS.md', 'docs/STATUS_ARCHIVE.md', 'docs/ROADMAP.md', 'docs/REPO_MAP.md'];
// This lane's commits: #56 (merge d7dad97: c94c81b..cd61b28), #57 (merge 22c6afa: d7dad97..f26ac5b), #58 (merge
// 9b4b518: 22c6afa..1bfe462) and #60 (merge b763ba3: 9b4b518..e5ca735).
const own = new Set([
  ...git('rev-list', 'c94c81b..cd61b28').split('\n').filter(Boolean),
  ...git('rev-list', 'd7dad97..f26ac5b').split('\n').filter(Boolean),
  ...git('rev-list', '22c6afa..1bfe462').split('\n').filter(Boolean),
  ...git('rev-list', '9b4b518..e5ca735').split('\n').filter(Boolean),
]);
const recOld = new Set(Object.values(JSON.parse(readFileSync('.checkpoints/cs2-first-page/docs-keeper-repomap-old.json', 'utf8'))));
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
const tally = new Map();
for (const r of removed) {
  const sha = git('blame', '--porcelain', '-L', `${r.n},${r.n}`, base, '--', r.f).split(' ')[0];
  const moved = r.text !== '' && (addedCount.get(r.text) ?? 0) > 0;
  if (moved) addedCount.set(r.text, addedCount.get(r.text) - 1);
  const verdict = r.text === '' ? 'blank separator' : moved ? 'moved byte for byte' : own.has(sha)
    ? 'own line (this lane)' : r.f === 'docs/REPO_MAP.md' && recOld.has(r.text) ? 'REC-OLD correction (listed)'
      : 'NOT THIS LANE AND NOT MOVED';
  if (verdict.startsWith('NOT')) bad++;
  tally.set(verdict, (tally.get(verdict) ?? 0) + 1);
  process.stdout.write(`${r.f}:${r.n} ${sha.slice(0, 7)} ${verdict} | ${r.text.slice(0, 60)}\n`);
}
process.stdout.write(`${removed.length} removed lines checked; ${[...tally].map(([k, v]) => `${k}: ${v}`).join('; ')}\n`);
process.stdout.write(`${bad} not this lane's, not moved and not a listed REC-OLD correction\n`);
process.exit(bad ? 1 : 0);
