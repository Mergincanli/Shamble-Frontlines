// pass 4 probe F1: a wrapper or reserved word given by a path (or quoted) is stripped by unwrapRole; bash runs the file.
// Real git + bash 5.2 in a scratch repo. Prints labels, guard decision kind, and what real bash did. No env values.
// usage: node sap4-wrap.mjs <guard.mjs path> <scratch dir>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, readFileSync, realpathSync, chmodSync, existsSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [guardPath, scratch] = process.argv.slice(2);
const { decide, realIo } = await import(pathToFileURL(guardPath).href);
const root = join(scratch, 'sap4-wrap');
rmSync(root, { recursive: true, force: true });
mkdirSync(root, { recursive: true });
const R = realpathSync(root);
const O = join(R, 'origin.git'), P = join(R, 'proj');
const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
delete env.CDPATH;
const gx = (cwd, ...a) => spawnSync('git', a, { cwd, env, encoding: 'utf8' });
const g = (...a) => gx(P, ...a);
const sh = (cwd, line) => spawnSync('bash', ['--norc', '--noprofile', '-c', line], { cwd, env, encoding: 'utf8' });
spawnSync('git', ['init', '-q', '--bare', '-b', 'main', O], { env });
mkdirSync(join(P, '.claude', 'hooks'), { recursive: true });
mkdirSync(join(P, 'docs'), { recursive: true });
writeFileSync(join(P, 'CLAUDE.md'), 'rules\n');
writeFileSync(join(P, '.claude', 'settings.json'), '{"a":1}\n');
writeFileSync(join(P, 'docs', 'a.md'), 'a\n');
writeFileSync(join(P, '.gitignore'), '.gate/\n');
g('init', '-q', '-b', 'main'); g('add', '-A'); g('commit', '-q', '-m', 'base');
g('remote', 'add', 'origin', O); g('push', '-q', 'origin', 'main'); g('fetch', '-q', 'origin');
const SCRIPT = '#!/bin/sh\necho pwned > CLAUDE.md\n';
const mk = (name, edit) => { g('switch', '-q', '-c', name, 'origin/main'); edit(); g('add', '-A'); g('commit', '-q', '-m', name); g('push', '-q', 'origin', name); g('switch', '-q', 'main'); };
mk('cs/x', () => { for (const f of ['do', 'timeout', 'env', 'then']) { writeFileSync(join(P, 'docs', f), SCRIPT); chmodSync(join(P, 'docs', f), 0o755); } });
g('fetch', '-q', 'origin');
g('branch', '-q', '-D', 'cs/x');
process.env.CLAUDE_PROJECT_DIR = P;
const kind = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : 'null');
const why = (d) => (d ? String(d.deny ?? d.ask ?? '').slice(0, 120) : '');
const dec = async (role, command, cwd = P) => decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd, scratchpad_dir: join(R, 'scratch') }, { ...realIo(), env: role ? { SHAMBLE_ROLE: role } : {} });
const out = (...a) => console.log(...a);
const claude = () => readFileSync(join(P, 'CLAUDE.md'), 'utf8').trim();
const row = async (label, role, line, real = true) => {
  writeFileSync(join(P, 'CLAUDE.md'), 'rules\n');
  const d = await dec(role, line);
  out(label, JSON.stringify(line), '->', kind(d), why(d));
  if (real && !d) { const r = sh(P, line); out('  real bash exit', r.status, 'branch', g('branch', '--show-current').stdout.trim() || '(detached)', 'CLAUDE.md:', claude()); }
};
// step 1: the night switch to cs/x (kit files equal main's) is allowed
await row('S1 night switch to cs/x (kit files = main):', 'night-shift', 'git switch -q cs/x');
out('  mode of docs/do on disk:', existsSync(join(P, 'docs', 'do')) ? (spawnSync('stat', ['-c', '%a', join(P, 'docs', 'do')], { encoding: 'utf8' }).stdout.trim()) : 'absent');
await row('W1 night docs/do git status:', 'night-shift', 'docs/do git status');
await row('W2 night ./docs/timeout 5 git status:', 'night-shift', './docs/timeout 5 git status');
await row('W3 night docs/env git status:', 'night-shift', 'docs/env git status');
await row('W4 night docs/then git status:', 'night-shift', 'docs/then git status');
await row('W5 pr-advisor docs/do git log -1:', 'pr-advisor', 'docs/do git log -1 --oneline');
await row('W6 day docs/do git status (day: not R-PLAIN):', '', 'docs/do git status', false);
await row('ctl night docs/x git status (not a wrapper name):', 'night-shift', 'docs/x git status', false);
// quoted reserved word at command position: bash runs a program named then/do (not reserved when quoted)
await row('Q1 night quoted reserved word:', 'night-shift', "'do' git status", false);
// plainForm atCmd cleared by `if`: "TZ=./x" after if
mkdirSync(join(P, 'TZ=.'), { recursive: true }); writeFileSync(join(P, 'TZ=.', 'x'), SCRIPT); chmodSync(join(P, 'TZ=.', 'x'), 0o755);
await row('T1 night if "TZ=./x" date; then date; fi:', 'night-shift', 'if "TZ=./x" date; then date; fi');
await row('T1c night "TZ=./x" date (control, R-PLAIN narrowing):', 'night-shift', '"TZ=./x" date', false);
await row('T2 night >&2 "TZ=./x" date (after a dup redirect):', 'night-shift', '>&2 "TZ=./x" date');
// routine-only route to an executable: git add --chmod and a worktree
g('switch', '-q', 'main');
writeFileSync(join(P, 'docs', 'run'), SCRIPT);
await row('C1 night git add --chmod=+x docs/run:', 'night-shift', 'git add --chmod=+x docs/run');
await row('C2 night git commit -q -m x:', 'night-shift', 'git commit -q -m x');
await row('C3 night git worktree add .gate/w HEAD:', 'night-shift', 'git worktree add -q .gate/w HEAD');
out('  mode of .gate/w/docs/run:', existsSync(join(P, '.gate', 'w', 'docs', 'run')) ? spawnSync('stat', ['-c', '%a', join(P, '.gate/w/docs/run')], { encoding: 'utf8' }).stdout.trim() : 'absent');
out('git', gx(P, '--version').stdout.trim(), 'bash', sh(P, 'echo $BASH_VERSION').stdout.trim());
