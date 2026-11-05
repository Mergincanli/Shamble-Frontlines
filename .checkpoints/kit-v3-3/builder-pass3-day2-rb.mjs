// Day RB rows on real git and bash (builder pass 3, day side 2): the guard at 01b158c and the guard at this head
// decide each row from the main folder (main session and an agent); then real bash runs the row. No env values printed.
// usage: node builder-pass3-day2-rb.mjs <a3 root> <scratch dir>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, readFileSync, realpathSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [a3, scratch] = process.argv.slice(2);
mkdirSync(scratch, { recursive: true });
const old = spawnSync('git', ['show', '01b158c:.claude/hooks/routine-guard.mjs'], { cwd: a3, encoding: 'utf8' });
writeFileSync(join(scratch, 'old.mjs'), old.stdout);
writeFileSync(join(scratch, 'new.mjs'), readFileSync(join(a3, '.claude/hooks/routine-guard.mjs')));
const G = { old: await import(pathToFileURL(join(scratch, 'old.mjs')).href), new: await import(pathToFileURL(join(scratch, 'new.mjs')).href) };
const root = join(scratch, 'repo'); rmSync(root, { recursive: true, force: true }); mkdirSync(root, { recursive: true });
const R = realpathSync(root), O = join(R, 'origin.git'), P = join(R, 'proj');
const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
delete env.CDPATH;
const g = (...a) => spawnSync('git', a, { cwd: P, env, encoding: 'utf8' });
const sh = (line) => spawnSync('bash', ['--norc', '--noprofile', '-c', line], { cwd: P, env, encoding: 'utf8' });
spawnSync('git', ['init', '-q', '--bare', '-b', 'main', O], { env });
mkdirSync(join(P, '.claude'), { recursive: true });
writeFileSync(join(P, 'CLAUDE.md'), 'rules\n'); writeFileSync(join(P, '.claude', 'settings.json'), '{"a":1}\n');
g('init', '-q', '-b', 'main'); g('add', '-A'); g('commit', '-q', '-m', 'base'); g('remote', 'add', 'origin', O); g('push', '-q', 'origin', 'main');
g('switch', '-q', '-c', 'cs/evil'); writeFileSync(join(P, '.claude', 'settings.json'), '{"a":2}\n'); g('commit', '-q', '-am', 'evil'); g('push', '-q', 'origin', 'cs/evil');
g('switch', '-q', 'main'); g('branch', '-q', '-D', 'cs/evil'); g('fetch', '-q', 'origin');
process.env.CLAUDE_PROJECT_DIR = P;
const kind = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : 'null');
const ev = (command, extra = {}) => ({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd: P, scratchpad_dir: join(R, 's'), ...extra });
const state = () => `branch ${g('branch', '--show-current').stdout.trim()} settings ${readFileSync(join(P, '.claude', 'settings.json'), 'utf8').trim()} CLAUDE.md ${JSON.stringify(readFileSync(join(P, 'CLAUDE.md'), 'utf8').slice(0, 30))}`;
const reset = () => { g('switch', '-q', '-f', 'main'); g('checkout', '-q', '--', '.'); g('branch', '-q', '-D', 'cs/evil'); };
const rows = [
  ['H8', "git status # '\ngit switch -q cs/evil # '"],
  ['A1', "git status  # what's changed\ngit switch -q cs/evil"],
  ['D5', "git status \\\n# '\ngit switch -q cs/evil # '"],
  ['AC', "printf $'a\\'b'; git switch -q cs/evil"],
  ['HQ', "cat <<'E O'\nE O\ngit switch -q cs/evil\nE"],
  ['R4', 'git status >&CLAUDE.md'],
  ['ctl 2>&1', 'git status 2>&1'], ['ctl >&2', 'git status >&2'], ['ctl heredoc', 'cat <<EOF\nbody\nEOF'],
];
for (const [label, line] of rows) {
  const res = [];
  for (const v of ['old', 'new']) {
    const d = await G[v].decide(ev(line), { ...G[v].realIo(), env: {} });
    const a = await G[v].decide(ev(line, { agent_id: 'a1' }), { ...G[v].realIo(), env: {} });
    res.push(`${v}: main ${kind(d)} agent ${kind(a)}${d ? ' (' + String(d.ask ?? d.deny).slice(0, 90) + ')' : ''}`);
  }
  const r = sh(line);
  console.log(label, JSON.stringify(line), '|', res.join(' | '), '| real bash exit', r.status, state());
  reset();
}
console.log('git', g('--version').stdout.trim(), 'bash', sh('echo $BASH_VERSION').stdout.trim());
rmSync(root, { recursive: true, force: true });
