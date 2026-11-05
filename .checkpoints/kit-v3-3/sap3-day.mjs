// security-auditor pass 3 probe (kit-v3-3 A3, head 01b158c): hidden lines after a # comment that holds a quote.
// Real git repos under the given scratch dir. Prints row labels, the guard's decision kind, and what real bash did.
// No env values or payloads printed. usage: node sap3-day.mjs <head guard.mjs> <scratch dir> <base guard.mjs>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, readFileSync, realpathSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [guardPath, scratch] = process.argv.slice(2);
const { decide, realIo } = await import(pathToFileURL(guardPath).href);
const root = join(scratch, 'sap3-day');
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
g('init', '-q', '-b', 'main'); g('add', '-A'); g('commit', '-q', '-m', 'base');
g('remote', 'add', 'origin', O); g('push', '-q', 'origin', 'main'); g('fetch', '-q', 'origin');
const mk = (name, edit) => { g('switch', '-q', '-c', name, 'origin/main'); edit(); g('add', '-A'); g('commit', '-q', '-m', name); g('push', '-q', 'origin', name); g('switch', '-q', 'main'); };
mk('cs/ok', () => writeFileSync(join(P, 'docs', 'a.md'), 'ok\n'));
mk('cs/evil', () => writeFileSync(join(P, '.claude', 'settings.json'), '{"a":2}\n'));
g('fetch', '-q', 'origin');
g('branch', '-q', '-D', 'cs/ok', 'cs/evil');
process.env.CLAUDE_PROJECT_DIR = P;
const kind = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : 'null');
const why = (d) => (d ? String(d.deny ?? d.ask ?? '').slice(0, 110) : '');
const dec = async (role, command, cwd = P) => decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd, scratchpad_dir: join(R, 'scratch') }, { ...realIo(), env: role ? { SHAMBLE_ROLE: role } : {} });
const out = (...a) => console.log(...a);
const settings = () => readFileSync(join(P, '.claude', 'settings.json'), 'utf8').trim();
const row = async (label, role, line, real = true) => {
  const d = await dec(role, line);
  out(label, JSON.stringify(line), '->', kind(d), why(d));
  if (real && !d) { const r = sh(P, line); out('  real bash exit', r.status, 'branch', g('branch', '--show-current').stdout.trim() || '(detached)', 'settings', settings(), 'CLAUDE.md', readFileSync(join(P, 'CLAUDE.md'), 'utf8').trim()); }
};
const resetMain = () => { g('switch', '-q', '-f', 'main'); g('checkout', '-q', '--', '.'); for (const b of ['cs/ok', 'cs/evil']) g('branch', '-q', '-D', b); };
// day controls (ordinary commands) on head and base; no real runs
const base = await import(pathToFileURL(process.argv[4]).href);
const decB = async (role, command, cwd = P) => base.decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd, scratchpad_dir: join(R, 'scratch') }, { ...base.realIo(), env: role ? { SHAMBLE_ROLE: role } : {} });
mkdirSync(join(P, 'tools'), { recursive: true }); writeFileSync(join(P, 'tools', 'gate.mjs'), 'x\n'); mkdirSync(join(P, 'build', 'x'), { recursive: true });
g('add', 'tools/gate.mjs'); g('commit', '-q', '-m', 'tools'); g('push', '-q', 'origin', 'main'); g('fetch', '-q', 'origin');
writeFileSync(join(P, '.git', 'info', 'exclude'), '.gate/\nbuild/\n');
g('worktree', 'add', '-q', '.gate/wt', '-b', 'cs/wt', 'origin/main');
mkdirSync(join(P, '.gate', 'empty'), { recursive: true }); mkdirSync(join(P, '.gate', 'build'), { recursive: true });
const day = ['git fetch origin main', 'git merge --ff-only origin/main', 'git fetch origin main && git merge --ff-only origin/main',
  'git fetch -q origin main && git switch --detach origin/main', 'git worktree add .gate/slug -b cs/slug origin/main',
  'git worktree add .gate/slug2 origin/cs/ok', 'rm -rf build', 'rm -rf build/x', 'rm -rf .gate/build', 'rm -rf .gate/empty', 'rm -rf .gate/wt/node_modules',
  `git clone ${O} .gate/clone`, `git clone -q ${O} .gate/clone2`, `git clone ${P} .gate/self`, 'git clone https://example.com/a/three.js.git .gate/three',
  'git clone https://example.com/a/tools.git .gate/t', `git clone ${O} .gate/tools`, `git clone --depth 1 ${O} .gate/d`, 'git clone https://example.com/a/b.git',
  'git status && git log -1', 'git fetch origin && git status', 'git pull --ff-only', 'git switch -q cs/ok', 'git fetch -q origin cs/ok && git switch -q cs/ok',
  'git stash && git switch -q main', 'cd .gate/wt && git switch -q -c cs/z origin/cs/evil', 'cd .gate/wt && git fetch -q origin && git switch -q -c cs/z2 origin/cs/evil',
  'cd .gate/wt; git log -1', 'rm -rf .gate', 'mv .gate .gate-old', 'npm --prefix tools ci', 'grep function docs/a.md', 'grep -rn trap docs'];
for (const l of day) { const h = kind(await dec('', l)), b = kind(await decB('', l)); out(h === b ? 'same ' : 'MOVED', 'head', h.padEnd(4), 'base', b.padEnd(4), JSON.stringify(l.replaceAll(R, '<R>'))); }
out('git', gx(P, '--version').stdout.trim(), 'bash', sh(P, 'echo $BASH_VERSION').stdout.trim());
rmSync(root, { recursive: true, force: true });
