// Fix-pass evidence: head mode on real history. In a shared local clone (.gate/rt-hist, gitignored; made beforehand with
// git clone --shared), origin/main is set to each merge's first parent, then track.mjs runs on its second parent: the
// PR's own diff through the real git layer. Compared with --files on the same paths. Run from the repository root:
//   node .checkpoints/review-tracks/fix-hist.mjs
import { execFileSync, spawnSync } from 'node:child_process';
const C = '/home/user/Shamble-Frontlines/.gate/rt-hist', T = '/home/user/Shamble-Frontlines/.gate/review-tracks/build/.claude/scripts/track.mjs';
const g = (a) => execFileSync('git', a, { cwd: C, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] });
const M = [['#76', '2671595f'], ['#74', '3b949c1a'], ['#73', '4a62873e'], ['#70', '2483e9a7'], ['#71', '07d673b0'], ['#67', 'c1da1bd2'],
  ['#69', '311c0484'], ['#68', '3ab9eaec'], ['#66', '09faefa7'], ['#65', '28254a73'], ['#77', '47bb79e9'], ['#75', 'b9aad019']];
for (const [pr, m] of M) {
  g(['update-ref', 'refs/remotes/origin/main', `${m}^1`]);
  const h = spawnSync(process.execPath, [T, `${m}^2`], { cwd: C, encoding: 'utf8' });
  if (h.status !== 0) { console.log(`${pr} head mode exit ${h.status}: ${h.stderr.trim()}`); continue; }
  const o = JSON.parse(h.stdout);
  const names = g(['diff', '--name-only', '--no-renames', o.base, o.head]).split('\n').filter(Boolean);
  const f = JSON.parse(spawnSync(process.execPath, [T, '--files', ...names], { cwd: C, encoding: 'utf8' }).stdout);
  const floors = [...new Set(o.goFloors.map((x) => x.replace(/^.*?: /, '')))];
  const words = Array.isArray(o.wordsRecord) ? o.wordsRecord.map((w) => w.result).reduce((t, r) => ({ ...t, [r]: (t[r] || 0) + 1 }), {}) : o.wordsRecord;
  console.log(`${pr} head=${o.track} files-mode=${f.track} same=${o.track === f.track} n=${names.length} picks=${o.reviewers.picks.join(',')} ` +
    `floors=${floors.join('; ') || 'none'} mergeCheck=${o.forMergeCheck.length} words=${JSON.stringify(words)}`);
}
