// skeptic SD-3 probe: day main-folder line `git fetch && git switch --detach origin/cs/x`, guard decision + real git result.
// usage: node skeptic-sd3-probe.mjs <guard.mjs> <scratch dir>. Prints only labels, decision kinds and toy file contents.
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, readFileSync, realpathSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [guardPath, scratch] = process.argv.slice(2);
const { decide, realIo } = await import(pathToFileURL(guardPath).href);
const root = join(scratch, 'p'); rmSync(root, { recursive: true, force: true }); mkdirSync(root, { recursive: true });
const R = realpathSync(root), O = join(R, 'o.git'), P = join(R, 'proj'), Q = join(R, 'q');
const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
const gx = (cwd, ...a) => spawnSync('git', a, { cwd, env, encoding: 'utf8' }); const g = (...a) => gx(P, ...a);
const sh = (line) => spawnSync('bash', ['--norc', '--noprofile', '-c', line], { cwd: P, env, encoding: 'utf8' });
spawnSync('git', ['init', '-q', '--bare', '-b', 'main', O], { env });
mkdirSync(join(P, '.claude'), { recursive: true }); mkdirSync(join(P, 'docs'), { recursive: true });
writeFileSync(join(P, 'CLAUDE.md'), 'r\n'); writeFileSync(join(P, '.claude', 'settings.json'), '{"a":1}\n'); writeFileSync(join(P, 'docs', 'a.md'), 'a\n');
g('init', '-q', '-b', 'main'); g('add', '-A'); g('commit', '-q', '-m', 'b'); g('remote', 'add', 'origin', O); g('push', '-q', 'origin', 'main');
g('switch', '-q', '-c', 'cs/x'); writeFileSync(join(P, 'docs', 'a.md'), 'x\n'); g('commit', '-q', '-am', 'x'); g('push', '-q', 'origin', 'cs/x'); g('switch', '-q', 'main'); g('branch', '-q', '-D', 'cs/x'); g('fetch', '-q', 'origin');
spawnSync('git', ['clone', '-q', O, Q], { env }); gx(Q, 'switch', '-q', 'cs/x'); writeFileSync(join(Q, '.claude', 'settings.json'), '{"a":3}\n'); gx(Q, 'commit', '-q', '-am', 'x2'); gx(Q, 'push', '-q', 'origin', 'cs/x');
process.env.CLAUDE_PROJECT_DIR = P;
const kind = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : 'null');
const run = async (command) => kind(await decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd: P }, { ...realIo(), env: {} }));
const st = () => readFileSync(join(P, '.claude', 'settings.json'), 'utf8').trim();
const lines = ['git fetch -q origin cs/x && git switch --detach origin/cs/x', 'git fetch -q origin cs/x; git switch --detach origin/cs/x', 'git fetch -q origin cs/x && git switch -c cs/y origin/cs/x', 'git fetch -q origin cs/x && git switch cs/x'];
for (const l of lines) {
  console.log('line:', l, '-> guard', await run(l));
  sh(l); console.log('  real: settings', st(), 'branch', g('branch', '--show-current').stdout.trim() || '(detached)');
  g('switch', '-q', 'main'); g('branch', '-q', '-D', 'cs/x', 'cs/y'); g('update-ref', 'refs/remotes/origin/cs/x', gx(Q, 'rev-parse', 'HEAD~1').stdout.trim());
}
g('fetch', '-q', 'origin', 'cs/x'); console.log('control, alone after fetch: git switch --detach origin/cs/x ->', await run('git switch --detach origin/cs/x'));
console.log('control, WT-UPDATE: git fetch origin main && git merge --ff-only origin/main ->', await run('git fetch -q origin main && git merge --ff-only origin/main'));
