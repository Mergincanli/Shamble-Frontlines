// security-auditor pass 3 probe (kit-v3-3 A3, head 01b158c): hidden lines after a # comment that holds a quote.
// Real git repos under the given scratch dir. Prints row labels, the guard's decision kind, and what real bash did.
// No env values or payloads printed. usage: node sap3-probe.mjs <guard.mjs path> <scratch dir>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, readFileSync, realpathSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [guardPath, scratch] = process.argv.slice(2);
const { decide, realIo } = await import(pathToFileURL(guardPath).href);
const root = join(scratch, 'sap3-probe');
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
// controls: the switch to cs/evil alone is refused; a fetch alone is allowed
await row('ctl night switch cs/evil alone:', 'night-shift', 'git switch -q cs/evil', false);
await row('ctl night fetch alone:', 'night-shift', 'git fetch -q origin cs/ok', false);
// carriers holding a hidden second line
await row('H1 night fetch #-hidden switch:', 'night-shift', "git fetch -q origin cs/ok # '\ngit switch -q cs/evil # '"); resetMain();
await row('H2 night status #-hidden switch:', 'night-shift', "git status # '\ngit switch -q cs/evil # '"); resetMain();
await row('H3 night switch cs/ok #-hidden switch:', 'night-shift', "git switch -q cs/ok # '\ngit switch -q cs/evil # '"); resetMain();
await row('H4 night echo #-hidden write:', 'night-shift', "echo hi # '\necho x > CLAUDE.md # '"); resetMain();
await row('H5 night git log #-hidden write:', 'night-shift', "git log -1 --oneline # '\necho x > CLAUDE.md # '"); resetMain();
await row('H6 night status, double quote:', 'night-shift', 'git status # "\ngit switch -q cs/evil # "'); resetMain();
await row('H7 pr-advisor status #-hidden write:', 'pr-advisor', "git status # '\necho x > CLAUDE.md # '"); resetMain();
await row('H8 day status #-hidden main-folder switch:', '', "git status # '\ngit switch -q cs/evil # '"); resetMain();
await row('H9 night ls #-hidden:', 'night-shift', "ls # '\necho x > CLAUDE.md # '"); resetMain();
// plain comment without a quote: the next line is seen
await row('ctl night status # x then switch line:', 'night-shift', 'git status # x\ngit switch -q cs/evil', false);
out('git', gx(P, '--version').stdout.trim(), 'bash', sh(P, 'echo $BASH_VERSION').stdout.trim());
rmSync(root, { recursive: true, force: true });
