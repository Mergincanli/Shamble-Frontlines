// kit-v3-3 step 5 mutation runs (Actions reads): each mutant in a scratch copy under .gate/kit-v3-3/, never the real
// file; prints the failing test names. Usage (from the worktree root): node .checkpoints/kit-v3-3/mut-step5.mjs
import { spawnSync } from 'node:child_process';
import { copyFileSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
const H = '.claude/hooks';
const rg = readFileSync(join(H, 'routine-guard.mjs'), 'utf8');
const gd = readFileSync(join(H, 'guard.mjs'), 'utf8');
const m = (from, to, s = rg) => (s.split(from).length !== 2 ? null : s.replace(from, to));
// the guard as step 4 left it: the step-5 block and dispatch line removed
const blockStart = rg.indexOf('// kit-v3-3 (C7)'), blockEnd = rg.indexOf('\nasync function mcp(');
const pre5 = blockStart < 0 || blockEnd < 0 ? null
  : m('    if (role && Object.hasOwn(ACTIONS_READ, name)) return actionsRead(name, ti, role);\n', '',
    rg.slice(0, blockStart) + rg.slice(blockEnd + 1));
const runs = [
  ['guard before step 5 + new tests', pre5],
  ['Actions method check off (plan row)', m('if (ok.includes(ti.method)) return null;', 'return null;')],
  ['Actions owner/repo check off (plan row)', m('return deny(`routine (${role}): GitHub reads only on ${REPO}`);', 'return null;')],
  ['night refusal off', m("if (role !== 'pr-advisor') return deny(", 'if (false) return deny(')],
  ['logs URL added to the list', m("'get_workflow_run_usage'],", "'get_workflow_run_usage', 'get_workflow_run_logs_url'],")],
  ['artifact download added to the list', m("'get_workflow_run_usage'],", "'get_workflow_run_usage', 'download_workflow_run_artifact'],")],
  ['owner/repo as one joined string', m("if (String(ti.owner ?? '').toLowerCase() !== owner || String(ti.repo ?? '').toLowerCase() !== repo) {",
    "if (`${ti.owner}/${ti.repo}`.toLowerCase() !== REPO.toLowerCase()) {")],
  ['dispatch for day sessions too', m('if (role && Object.hasOwn(ACTIONS_READ, name))', 'if (Object.hasOwn(ACTIONS_READ, name))')],
];
for (const [i, [label, src]] of runs.entries()) {
  if (!src) { console.log(`${label}: TARGET NOT FOUND`); continue; }
  const dir = join('.gate', 'kit-v3-3', `mut-step5-${i}`);
  rmSync(dir, { recursive: true, force: true }); mkdirSync(join(dir, '.claude', 'hooks'), { recursive: true });
  const hd = join(dir, '.claude', 'hooks');
  writeFileSync(join(hd, 'routine-guard.mjs'), src);
  writeFileSync(join(hd, 'guard.mjs'), gd);
  for (const f of ['routine-guard.test.mjs', 'settings.test.mjs']) copyFileSync(join(H, f), join(hd, f));
  copyFileSync('.claude/settings.json', join(dir, '.claude', 'settings.json'));
  const r = spawnSync(process.execPath, ['--test', join(hd, 'routine-guard.test.mjs')], { encoding: 'utf8' });
  const fails = r.stdout.split('\n').filter((l) => /^not ok/.test(l)).map((l) => l.replace(/^not ok \d+ - /, ''));
  const count = (k) => (r.stdout.match(new RegExp(`^# ${k} (\\d+)`, 'm')) || [])[1];
  console.log(`${label}: pass ${count('pass')}/${count('tests')}, failing: ${fails.join(' | ') || 'none'}`);
}
