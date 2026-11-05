// kit-v3-3 step 3 mutation runs: each mutant (and the base guard) in a scratch copy under .gate/kit-v3-3/, never the real file.
// Usage: node .checkpoints/kit-v3-3/mut-step3.mjs   (from the worktree root)
import { spawnSync } from 'node:child_process';
import { copyFileSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
const H = '.claude/hooks';
const src = readFileSync(join(H, 'routine-guard.mjs'), 'utf8');
const base = spawnSync('git', ['show', 'HEAD:.claude/hooks/routine-guard.mjs'], { encoding: 'utf8' }).stdout;
const runs = [
  ['base (HEAD, before step 3)', base],
  ["rev-list's --output removed", src.replace("  'rev-list': ['--output'],\n", '')],
  ["diff's --output removed", src.replace("  diff: ['--output'],\n", '')],
  ["log's --output removed", src.replace("  log: ['--output'],\n", '')],
  ["show's --output removed", src.replace("  show: ['--output'],\n", '')],
  ['rev-list off GIT_ROUTINE', src.replace("'symbolic-ref',\n  'rev-list']);", "'symbolic-ref']);")],
  ['rev-list off GIT_READ', src.replace("'symbolic-ref', 'rev-list']);", "'symbolic-ref']);")],
];
for (const [i, [label, code]] of runs.entries()) {
  if (i > 0 && code === src) { console.log(`${label}: TARGET NOT FOUND`); continue; }
  const dir = join('.gate', 'kit-v3-3', `mut-step3-${i}`);
  rmSync(dir, { recursive: true, force: true }); mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, 'routine-guard.mjs'), code);
  for (const f of ['routine-guard.test.mjs', 'guard.mjs']) copyFileSync(join(H, f), join(dir, f));
  const r = spawnSync(process.execPath, ['--test', join(dir, 'routine-guard.test.mjs')], { encoding: 'utf8' });
  const fails = r.stdout.split('\n').filter((l) => /^not ok/.test(l)).map((l) => l.replace(/^not ok \d+ - /, ''));
  const count = (k) => (r.stdout.match(new RegExp(`^# ${k} (\\d+)`, 'm')) || [])[1];
  console.log(`${label}: pass ${count('pass')}/${count('tests')}, failing: ${fails.join(' | ') || 'none'}`);
}
