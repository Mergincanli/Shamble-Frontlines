// Run track.mjs --files on the last merged PRs (paths from git diff --name-only M^1 M) and on <merge>^2 (head mode).
import { execFileSync, spawnSync } from 'node:child_process';
const T = process.argv[2], R = '/home/user/Shamble-Frontlines';
const M = [['#76', '2671595f'], ['#74', '3b949c1a'], ['#73', '4a62873e'], ['#70', '2483e9a7'], ['#71', '07d673b0'], ['#67', 'c1da1bd2'],
  ['#69', '311c0484'], ['#68', '3ab9eaec'], ['#66', '09faefa7'], ['#65', '28254a73'], ['#77', '47bb79e9'], ['#75', 'b9aad019']];
for (const [pr, m] of M) {
  const names = execFileSync('git', ['diff', '--name-only', '--no-renames', `${m}^1`, m], { cwd: R, encoding: 'utf8' }).split('\n').filter(Boolean);
  const r = spawnSync(process.execPath, [T, '--files', ...names], { cwd: R, encoding: 'utf8' });
  const o = JSON.parse(r.stdout);
  const fullWhy = [...new Set(o.why.filter((w) => !w.includes('light (fixed')).map((w) => w.replace(/^.*: full \((.*)\)$/, '$1')))];
  const h = spawnSync(process.execPath, [T, `${m}^2`], { cwd: R, encoding: 'utf8' });
  console.log(`${pr} files=${names.length} ${o.track} [${fullWhy.join('; ')}] floors=${[...new Set(o.goFloors.map((f) => f.replace(/^[^:]*: /, '')))].join('; ') || 'none'} eyes=${o.eyesPaths.length} tool=${o.toolSource.length} rev=${o.reviewers.picks.join(',')} | head-mode exit ${h.status} ${h.stderr.trim().slice(0, 60)}`);
}
