// security-auditor pass 3 probe (kit-v3-3 A3, head 01b158c): hidden lines after a # comment that holds a quote.
// Real git repos under the given scratch dir. Prints row labels, the guard's decision kind, and what real bash did.
// No env values or payloads printed. usage: node sap3-probe.mjs <guard.mjs path> <scratch dir>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, readFileSync, realpathSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [guardPath, scratch] = process.argv.slice(2);
const { decide, realIo } = await import(pathToFileURL(guardPath).href);
const root = join(scratch, 'sap3-probe7');
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
// SD-2 commit check: pathspec forms (CLAUDE.md modified, not staged); real commit only when allowed
const dirty = () => writeFileSync(join(P, 'CLAUDE.md'), 'changed\n');
const committed = () => g('show', '--name-only', '--format=', 'HEAD').stdout.trim().split('\n').join(',');
const crow = async (label, line) => { dirty(); g('switch', '-q', '-c', 'cs/w', 'origin/main'); dirty(); const d = await dec('night-shift', line); out(label, JSON.stringify(line), '->', kind(d), why(d)); if (!d) { sh(P, line); out('  real commit holds:', committed()); } g('reset', '-q', '--hard', 'origin/main'); g('switch', '-q', 'main'); g('branch', '-q', '-D', 'cs/w'); g('checkout', '-q', '--', '.'); };
await crow('S2a commit pathspec, no --:', 'git commit -q -m m CLAUDE.md');
await crow('S2b commit -i pathspec:', 'git commit -q -m m -i CLAUDE.md');
await crow('S2c commit -o pathspec:', 'git commit -q -m m -o CLAUDE.md');
await crow('S2d commit -- pathspec (control):', 'git commit -q -m m -- CLAUDE.md');
await crow('S2e commit -a (control):', 'git commit -q -a -m m');
writeFileSync(join(R, 'ps.txt'), 'CLAUDE.md\n');
await crow('S2f commit --pathspec-from-file:', `git commit -q -m m --pathspec-from-file=${R}/ps.txt`);
await crow('S2g commit pathspec glob quoted:', "git commit -q -m m 'CLAUDE.*'");
await crow('S2h commit :(top) magic:', "git commit -q -m m ':(top)CLAUDE.md'");
await crow('S2i commit -p:', 'git commit -q -m m -p');
await crow('S2j commit -ma (message a):', 'git commit -q -ma');
out('git', gx(P, '--version').stdout.trim(), 'bash', sh(P, 'echo $BASH_VERSION').stdout.trim());
rmSync(root, { recursive: true, force: true });
