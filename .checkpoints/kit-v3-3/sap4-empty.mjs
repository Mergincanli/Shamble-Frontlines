// pass 4 probe F3: a routine command whose argv is empty after unwrapRole (a bare redirect, an assignment, a stripped
// reserved word) skips the redirect checks. Real bash. usage: node sap4-empty.mjs <guard.mjs> <scratch dir>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, readFileSync, realpathSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [guardPath, scratch] = process.argv.slice(2);
const { decide, realIo } = await import(pathToFileURL(guardPath).href);
const root = join(scratch, 'sap4-empty');
rmSync(root, { recursive: true, force: true }); mkdirSync(root, { recursive: true });
const R = realpathSync(root), P = join(R, 'proj');
const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
delete env.CDPATH;
const g = (...a) => spawnSync('git', a, { cwd: P, env, encoding: 'utf8' });
const sh = (line) => spawnSync('bash', ['--norc', '--noprofile', '-c', line], { cwd: P, env, encoding: 'utf8' });
mkdirSync(join(P, '.claude'), { recursive: true });
const reset = () => { writeFileSync(join(P, 'CLAUDE.md'), 'rules\n'); writeFileSync(join(P, '.claude', 'settings.json'), '{"a":1}\n'); };
reset(); g('init', '-q', '-b', 'main'); g('add', '-A'); g('commit', '-q', '-m', 'base');
process.env.CLAUDE_PROJECT_DIR = P;
const kind = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : 'null');
const row = async (role, line) => {
  reset();
  const d = await decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command: line }, cwd: P, scratchpad_dir: join(R, 'scratch') }, { ...realIo(), env: role ? { SHAMBLE_ROLE: role } : {} });
  let eff = '';
  if (!d) { const r = sh(line); eff = `bash exit ${r.status}; CLAUDE.md ${JSON.stringify(readFileSync(join(P, 'CLAUDE.md'), 'utf8'))}; settings.json ${JSON.stringify(readFileSync(join(P, '.claude/settings.json'), 'utf8'))}`; }
  console.log(role || 'day', JSON.stringify(line), '->', kind(d), d ? String(d.deny ?? d.ask).slice(0, 90) : eff);
};
for (const role of ['night-shift', 'pr-advisor']) {
  await row(role, '>CLAUDE.md');
  await row(role, 'git status; >.claude/settings.json');
  await row(role, 'TZ=x >CLAUDE.md');
  await row(role, 'TZ=x >.claude/settings.json');
  await row(role, 'git status && then >CLAUDE.md');
  await row(role, 'git status > CLAUDE.md');
}
await row('', '>CLAUDE.md');
await row('', 'TZ=x >CLAUDE.md');
