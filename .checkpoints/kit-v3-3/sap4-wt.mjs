// pass 4 probe F4: a night routine works in a worktree under .gate/; the kit paths there are read as
// .gate/<wt>/.claude/... (not protected), so the Write tool, the commit check and the push check pass a kit change.
// Real git. usage: node sap4-wt.mjs <guard.mjs> <scratch dir>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, realpathSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [guardPath, scratch] = process.argv.slice(2);
const { decide, realIo } = await import(pathToFileURL(guardPath).href);
const root = join(scratch, 'sap4-wt');
rmSync(root, { recursive: true, force: true }); mkdirSync(root, { recursive: true });
const R = realpathSync(root), O = join(R, 'origin.git'), P = join(R, 'proj');
const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
delete env.CDPATH;
const gx = (cwd, ...a) => spawnSync('git', a, { cwd, env, encoding: 'utf8' });
const g = (...a) => gx(P, ...a);
const sh = (cwd, line) => spawnSync('bash', ['--norc', '--noprofile', '-c', line], { cwd, env, encoding: 'utf8' });
spawnSync('git', ['init', '-q', '--bare', '-b', 'main', O], { env });
mkdirSync(join(P, '.claude'), { recursive: true }); mkdirSync(join(P, '.github', 'workflows'), { recursive: true });
writeFileSync(join(P, 'CLAUDE.md'), 'rules\n'); writeFileSync(join(P, '.claude', 'settings.json'), '{"a":1}\n');
writeFileSync(join(P, '.github', 'workflows', 'ci.yml'), 'on: push\n'); writeFileSync(join(P, '.gitignore'), '.gate/\n');
g('init', '-q', '-b', 'main'); g('add', '-A'); g('commit', '-q', '-m', 'base');
g('remote', 'add', 'origin', O); g('push', '-q', 'origin', 'main'); g('fetch', '-q', 'origin');
process.env.CLAUDE_PROJECT_DIR = P;
const kind = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : 'null');
const call = async (role, tool, ti, cwd) => decide({ hook_event_name: 'PreToolUse', tool_name: tool, tool_input: ti, cwd, scratchpad_dir: join(R, 'scratch') }, { ...realIo(), env: { SHAMBLE_ROLE: role } });
const run = async (cwd, line) => {
  const d = await call('night-shift', 'Bash', { command: line }, cwd);
  const r = d ? null : sh(cwd, line);
  console.log(`[${cwd === P ? 'proj' : '.gate/w'}]`, JSON.stringify(line), '->', kind(d), d ? String(d.deny ?? d.ask).slice(0, 110) : `bash exit ${r.status}${r.status ? ' ' + r.stderr.trim().slice(0, 80) : ''}`);
};
const WT = join(P, '.gate', 'w');
await run(P, 'git worktree add -q -b cs/wt .gate/w origin/main');
for (const f of ['.claude/settings.json', '.github/workflows/ci.yml', 'CLAUDE.md']) {
  const d = await call('night-shift', 'Write', { file_path: join(WT, f), content: 'x' }, WT);
  console.log('Write', '.gate/w/' + f, '->', kind(d), d ? String(d.deny).slice(0, 90) : '');
  if (!d) writeFileSync(join(WT, f), f.endsWith('.json') ? '{"a":2}\n' : 'evil: true\n');
}
await run(WT, 'git add .claude/settings.json .github/workflows/ci.yml');
await run(WT, 'git commit -q -m "notes"');
await run(WT, 'git push -q origin cs/wt');
const remote = gx(P, 'ls-remote', O, 'cs/wt').stdout.trim();
console.log('origin has cs/wt:', remote ? 'yes' : 'no');
if (remote) console.log('origin cs/wt changes vs main:', gx(O, 'diff', '--name-only', 'main', 'cs/wt').stdout.trim().split('\n').join(', '));
// the same from the main folder (the control)
g('switch', '-q', '-c', 'cs/ctl', 'origin/main'); writeFileSync(join(P, '.github', 'workflows', 'ci.yml'), 'evil: true\n');
await run(P, 'git add .github/workflows/ci.yml');
await run(P, 'git commit -q -m "notes"');
