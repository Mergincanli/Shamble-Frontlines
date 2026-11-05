// security-auditor pass 3 probe (kit-v3-3 A3, head 01b158c): hidden lines after a # comment that holds a quote.
// Real git repos under the given scratch dir. Prints row labels, the guard's decision kind, and what real bash did.
// No env values or payloads printed. usage: node sap3-probe.mjs <guard.mjs path> <scratch dir>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, readFileSync, realpathSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [guardPath, scratch] = process.argv.slice(2);
const { decide, realIo } = await import(pathToFileURL(guardPath).href);
const root = join(scratch, 'sap3-probe6');
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
// V-3: unusual names; real switch only when the guard allows
g('pack-refs', '--all');
for (const n of ['cs/ok', 'cs/evil', 'cs/ok^', 'cs/ok@{0}', 'cs/ok@{u}', 'cs/x..y', 'cs/ok.lock', 'CS/OK', 'refs/heads/cs/ok', 'heads/cs/ok', 'origin/cs/ok', '@', '@{-1}', 'cs/ok~0']) {
  await row(`V3 night packed refs, git switch -q ${n}:`, 'night-shift', `git switch -q ${n}`); resetMain();
}
// symbolic ref under refs/heads pointing at a clean branch
g('branch', '-q', 'cs/ok', 'origin/cs/ok'); g('symbolic-ref', 'refs/heads/cs/sym', 'refs/heads/cs/ok');
await row('V3 night symbolic refs/heads/cs/sym -> cs/ok:', 'night-shift', 'git switch -q cs/sym');
g('switch', '-q', '-f', 'main'); g('symbolic-ref', '--delete', 'refs/heads/cs/sym'); resetMain();
// upstream to a local branch: cs/up tracks local cs/evil (name with @{u} must not reach it)
g('branch', '-q', 'cs/evil', 'origin/cs/evil'); g('branch', '-q', 'cs/up', 'origin/cs/ok'); g('config', 'branch.cs/up.remote', '.'); g('config', 'branch.cs/up.merge', 'refs/heads/cs/evil');
await row('V3 night git switch cs/up@{u} (upstream local cs/evil):', 'night-shift', 'git switch -q cs/up@{u}');
resetMain(); g('branch', '-q', '-D', 'cs/up');
out('git', gx(P, '--version').stdout.trim(), 'bash', sh(P, 'echo $BASH_VERSION').stdout.trim());
rmSync(root, { recursive: true, force: true });
