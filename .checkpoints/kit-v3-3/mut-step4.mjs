// kit-v3-3 step 4 mutation runs: each mutant in a scratch copy under .gate/kit-v3-3/, never the real file; prints the
// failing test names. Usage (from the worktree root): node .checkpoints/kit-v3-3/mut-step4.mjs
import { spawnSync } from 'node:child_process';
import { copyFileSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
const H = '.claude/hooks';
const rg = readFileSync(join(H, 'routine-guard.mjs'), 'utf8');
const gd = readFileSync(join(H, 'guard.mjs'), 'utf8');
const base = (f) => spawnSync('git', ['show', `HEAD:${H}/${f}`], { encoding: 'utf8' }).stdout;
const m = (from, to, file = 'rg') => { const s = file === 'rg' ? rg : gd; if (s.split(from).length !== 2) return null; return s.replace(from, to); };
const runs = [
  ['base guard (HEAD, before step 4) + new tests', { rg: base('routine-guard.mjs'), gd: base('guard.mjs') }],
  ['day ask off (shell writes)', { rg: m('first ??= ask(`kit and gate files ask first in a day session: ${rel} (${what})`);', '') }],
  ['agent deny keyed on agent_type', { rg: m('const agent = !!input.agent_id;', 'const agent = !!input.agent_type;') }],
  ['kitPath: projectRel first', { rg: m('const g = gitPlace(abs, io);\n  if (!projectCommon', '{ const pr0 = projectRel(p, cwd, io); if (pr0 !== null) return protectedRel(pr0) ? pr0 : null; }\n  const g = gitPlace(abs, io);\n  if (!projectCommon') }],
  ['stale main: name exemption for main', { rg: m("const h = protectedHit(['origin/main', pos[0], '--'], vcwd, io);", "const h = pos[0] === 'main' ? null : protectedHit(['origin/main', pos[0], '--'], vcwd, io);") }],
  ["guard.mjs fail-closed ask off (asks with a role)", { gd: m("print(d.ask, false, event === 'PreToolUse' && !role)", "print(d.ask, false, event === 'PreToolUse')", 'gd') }],
  ['main folder check off', { rg: m('if (!sameDir(gitTop(vcwd, io), io.projectDir, io)) return null;', '') }],
  ['no -- after the ref (path reading)', { rg: m("const h = protectedHit(['origin/main', pos[0], '--'], vcwd, io);", "const h = protectedHit(['origin/main', pos[0]], vcwd, io);") }],
];
for (const [i, [label, files]] of runs.entries()) {
  if (Object.values(files).some((v) => v === null || v === undefined)) { console.log(`${label}: TARGET NOT FOUND`); continue; }
  const dir = join('.gate', 'kit-v3-3', `mut-step4-${i}`);
  rmSync(dir, { recursive: true, force: true }); mkdirSync(join(dir, '.claude', 'hooks'), { recursive: true });
  const hd = join(dir, '.claude', 'hooks');
  writeFileSync(join(hd, 'routine-guard.mjs'), files.rg ?? rg);
  writeFileSync(join(hd, 'guard.mjs'), files.gd ?? gd);
  for (const f of ['routine-guard.test.mjs', 'settings.test.mjs']) copyFileSync(join(H, f), join(hd, f));
  copyFileSync('.claude/settings.json', join(dir, '.claude', 'settings.json'));
  const r = spawnSync(process.execPath, ['--test', join(hd, 'routine-guard.test.mjs')], { encoding: 'utf8' });
  const fails = r.stdout.split('\n').filter((l) => /^not ok/.test(l)).map((l) => l.replace(/^not ok \d+ - /, ''));
  const count = (k) => (r.stdout.match(new RegExp(`^# ${k} (\\d+)`, 'm')) || [])[1];
  console.log(`${label}: pass ${count('pass')}/${count('tests')}, failing: ${fails.join(' | ') || 'none'}`);
}
