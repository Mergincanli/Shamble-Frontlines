// kit-v3-3 step 4c mutation runs: each mutant in a scratch copy under .gate/kit-v3-3/, never the real files; prints
// the failing test names. Usage (from the worktree root): node .checkpoints/kit-v3-3/mut-step4c.mjs
import { spawnSync } from 'node:child_process';
import { copyFileSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
const H = '.claude/hooks';
const rg = readFileSync(join(H, 'routine-guard.mjs'), 'utf8');
const base = (f) => spawnSync('git', ['show', `HEAD:${f}`], { encoding: 'utf8' }).stdout;
const m = (from, to, s = rg) => (s.split(from).length !== 2 ? null : s.replace(from, to));
const m2 = (pairs) => pairs.reduce((s, [a, b]) => (s === null ? null : m(a, b, s)), rg);
const F5 = 'if (!absolute && dirs.cdpath && !/^\\.\\.?(\\/|$)/.test(t)) return lost();';
const F6 = 'const sha = detach ? commitOf(pos[0], vcwd, io) : branchCommit(pos[0], vcwd, io);';
const runs = [
  ['unmutated copy', {}],
  ['base guard (HEAD = 546d5a7, before step 4c) + new tests', { rg: base(`${H}/routine-guard.mjs`) }],
  ['F5 fix off (the line flag not checked in dayCd)', { rg: m(F5, '') }],
  ['F5 flag never set', { rg: m('if (!role) dirs.cdpath = lineSetsCdpath(command, cmds);', 'if (!role) dirs.cdpath = false;') }],
  ['F6 fix off (switch takes the general resolve)', { rg: m(F6, 'const sha = commitOf(pos[0], vcwd, io);') }],
  ['F6 ambiguity check off (refs/heads only)', { rg: m('full.code === 0 && full.out.trim() === `refs/heads/${name}` ?', 'true ?') }],
  ['F5 and F6 both off', { rg: m2([[F5, ''], [F6, 'const sha = commitOf(pos[0], vcwd, io);']]) }],
];
for (const [i, [label, files]] of runs.entries()) {
  if (Object.values(files).some((v) => v === null || v === undefined || v === '')) { console.log(`${label}: TARGET NOT FOUND`); continue; }
  const dir = join('.gate', 'kit-v3-3', `mut-step4c-${i}`);
  rmSync(dir, { recursive: true, force: true });
  const hd = join(dir, '.claude', 'hooks');
  mkdirSync(hd, { recursive: true });
  writeFileSync(join(hd, 'routine-guard.mjs'), files.rg ?? rg);
  copyFileSync(join(H, 'guard.mjs'), join(hd, 'guard.mjs'));
  copyFileSync(join(H, 'routine-guard.test.mjs'), join(hd, 'routine-guard.test.mjs'));
  const r = spawnSync(process.execPath, ['--test', join(hd, 'routine-guard.test.mjs')], { encoding: 'utf8', maxBuffer: 64 * 1024 * 1024 });
  const fails = r.stdout.split('\n').filter((l) => /^not ok/.test(l)).map((l) => l.replace(/^not ok \d+ - /, ''));
  const count = (k) => (r.stdout.match(new RegExp(`^# ${k} (\\d+)`, 'm')) || [])[1];
  console.log(`[${i}] ${label}: routine-guard.test.mjs ${count('pass')}/${count('tests')}${fails.length ? ' failing: ' + fails.join(' | ') : ''}`);
}
