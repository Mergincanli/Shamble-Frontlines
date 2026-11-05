// skeptic SD-4 probe: day lines with a cd bash skips (&& after a failed cd; ! cd ... && cd); guard decision + real bash result.
// usage: node skeptic-sd4-probe.mjs <routine-guard.mjs> <scratch dir>. Prints only labels, decision kinds, branch names, toy file contents.
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, readFileSync, realpathSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [guardPath, scratch] = process.argv.slice(2);
const { decide, realIo } = await import(pathToFileURL(guardPath).href);
const root = join(scratch, 'sd4'); rmSync(root, { recursive: true, force: true }); mkdirSync(root, { recursive: true });
const R = realpathSync(root), O = join(R, 'o.git'), P = join(R, 'proj'), N = join(R, 'nonrepo'), M = join(R, 'missing');
const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
const g = (...a) => spawnSync('git', a, { cwd: P, env, encoding: 'utf8' });
const sh = (line) => spawnSync('bash', ['--norc', '--noprofile', '-c', line], { cwd: P, env, encoding: 'utf8' });
spawnSync('git', ['init', '-q', '--bare', '-b', 'main', O], { env });
mkdirSync(join(P, '.claude'), { recursive: true }); mkdirSync(N);
writeFileSync(join(P, 'CLAUDE.md'), 'r\n'); writeFileSync(join(P, '.claude', 'settings.json'), '{"a":1}\n');
g('init', '-q', '-b', 'main'); g('add', '-A'); g('commit', '-q', '-m', 'b'); g('remote', 'add', 'origin', O); g('push', '-q', 'origin', 'main');
g('switch', '-q', '-c', 'cs/evil'); writeFileSync(join(P, '.claude', 'settings.json'), '{"a":2}\n'); g('commit', '-q', '-am', 'e'); g('push', '-q', 'origin', 'cs/evil'); g('switch', '-q', 'main'); g('branch', '-q', '-D', 'cs/evil');
const W = join(P, '.gate', 'wt'); g('worktree', 'add', '-q', '--detach', W, 'main');
process.env.CLAUDE_PROJECT_DIR = P;
const kind = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : 'null');
const run = async (command) => kind(await decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd: P }, { ...realIo(), env: {} }));
const state = () => `branch=${g('branch', '--show-current').stdout.trim()} CLAUDE.md=${readFileSync(join(P, 'CLAUDE.md'), 'utf8').trim()}`;
const reset = () => { g('switch', '-q', 'main'); g('branch', '-q', '-D', 'cs/c1', 'cs/c2', 'cs/c0', 'cs/c3'); writeFileSync(join(P, 'CLAUDE.md'), 'r\n'); };
const lines = {
  C1: `cd ${M} && cd ${W}; git switch -q -c cs/c1 origin/cs/evil`,
  C1c: `cd ${M} && cd ${N}; echo x > CLAUDE.md`,
  C2: `! cd ${P} && cd ./.gate/wt; git switch -q -c cs/c2 origin/cs/evil`,
  C3_bare_and_after_known: `cd ${P} && cd ${M} && cd ${W}; git switch -q -c cs/c3 origin/cs/evil`,
  control_semicolon: `cd ${M}; cd ${W}; git switch -q -c cs/c0 origin/cs/evil`,
  control_plain: `cd ${W} && git switch -q -c cs/c0 origin/cs/evil`,
  control_main: `git switch -q -c cs/c0 origin/cs/evil`,
};
for (const [k, l] of Object.entries(lines)) {
  const d = await run(l); sh(l); console.log(k, '-> guard', d, '| real main folder:', state()); reset();
}
