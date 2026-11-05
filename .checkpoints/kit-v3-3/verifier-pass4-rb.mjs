// Verifier pass 4: the RB rows on real git and bash, decided by the REAL guard at 01b158c and by the head guard, then
// run by real bash. Routine rows (night-shift, pr-advisor), the night commit rows (P3-5), P3-6, and the day rows.
// usage: node verifier-pass4-rb.mjs <a3 root> <scratch dir>. Prints no environment values.
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, readFileSync, realpathSync, existsSync } from 'node:fs';
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
const env = { PATH: process.env.PATH, HOME: R, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
const g = (...a) => spawnSync('git', a, { cwd: P, env, encoding: 'utf8' });
const sh = (line, cwd = P) => spawnSync('bash', ['--norc', '--noprofile', '-c', line], { cwd, env, encoding: 'utf8', input: '' });
spawnSync('git', ['init', '-q', '--bare', '-b', 'main', O], { env });
mkdirSync(join(P, '.claude'), { recursive: true }); mkdirSync(join(P, 'docs'), { recursive: true });
writeFileSync(join(P, 'CLAUDE.md'), 'rules\n'); writeFileSync(join(P, '.claude', 'settings.json'), '{"a":1}\n');
writeFileSync(join(P, 'docs', 'a.md'), 'a\n');
g('init', '-q', '-b', 'main'); g('add', '-A'); g('commit', '-q', '-m', 'base'); g('remote', 'add', 'origin', O); g('push', '-q', 'origin', 'main');
g('switch', '-q', '-c', 'cs/ok'); writeFileSync(join(P, 'docs', 'a.md'), 'ok\n'); g('commit', '-q', '-am', 'ok'); g('push', '-q', 'origin', 'cs/ok');
g('switch', '-q', 'main');
g('switch', '-q', '-c', 'cs/evil'); writeFileSync(join(P, '.claude', 'settings.json'), '{"a":2}\n'); g('commit', '-q', '-am', 'evil'); g('push', '-q', 'origin', 'cs/evil');
g('switch', '-q', 'main'); g('branch', '-q', '-D', 'cs/evil', 'cs/ok');
// P3-6: cs/sub adds a gitlink at .claude/sub with .gitmodules ignore = all; cs/gm adds only that .gitmodules
const SUB = '[submodule "s"]\n\tpath = .claude/sub\n\turl = ./x\n\tignore = all\n';
const base = g('rev-parse', 'HEAD').stdout.trim();
g('switch', '-q', '-c', 'cs/sub'); writeFileSync(join(P, '.gitmodules'), SUB);
g('update-index', '--add', '--cacheinfo', `160000,${base},.claude/sub`); g('add', '.gitmodules'); g('commit', '-q', '-m', 'sub'); g('push', '-q', 'origin', 'cs/sub');
g('switch', '-q', 'main'); g('switch', '-q', '-c', 'cs/gm', 'main'); writeFileSync(join(P, '.gitmodules'), SUB); g('add', '.gitmodules'); g('commit', '-q', '-m', 'gm');
g('switch', '-q', 'main'); g('fetch', '-q', 'origin');
process.env.CLAUDE_PROJECT_DIR = P;
const kind = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : 'null');
const ev = (command, cwd, extra = {}) => ({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd, scratchpad_dir: join(R, 's'), ...extra });
const rd = (f) => (existsSync(join(P, f)) ? readFileSync(join(P, f), 'utf8') : '(none)');
const state = () => `branch=${g('branch', '--show-current').stdout.trim()} settings=${JSON.stringify(rd('.claude/settings.json').slice(0, 12))} CLAUDE.md=${JSON.stringify(rd('CLAUDE.md').slice(0, 12))} head-files=[${g('show', '--name-only', '--format=', 'HEAD').stdout.trim().split('\n').join(',')}] sub=${existsSync(join(P, '.claude/sub')) ? 'yes' : 'no'}`;
const reset = () => {
  g('reset', '-q', '--hard'); g('switch', '-q', '-f', 'main'); g('checkout', '-q', '--', '.');
  for (const b of ['cs/evil', 'cs/ok', 'cs/w', 'cs/sub', 'cs/sub3']) g('branch', '-q', '-D', b);
  rmSync(join(P, '.gitmodules'), { force: true }); rmSync(join(P, '.claude', 'sub'), { recursive: true, force: true });
  g('fetch', '-q', 'origin');
};
const scratchBranch = (more) => () => { g('switch', '-q', '-c', 'cs/w', 'origin/main'); writeFileSync(join(P, 'docs', 'b.md'), 'b\n'); g('add', 'docs/b.md'); more?.(); };
const dirtyRules = scratchBranch(() => writeFileSync(join(P, 'CLAUDE.md'), 'changed\n'));
const N = 'night-shift', A = 'pr-advisor';
// [label, role (null: day main session), line, cwd, setup, agent too]
const rows = [
  ['A1', N, "git status  # what's changed\ngit switch -q cs/evil", P],
  ['H1', N, "git fetch -q origin cs/ok # '\ngit switch -q cs/evil # '", P],
  ['D1', N, 'git log -1 --oneline <<E\\ O\nE O\ngit switch -q cs/evil\nE', P],
  ['D4', A, 'git log -1 <<E\\ O\nE O\necho x > CLAUDE.md\nE', P],
  ['Q6', N, "date +$'\\''x\ngit switch -q cs/evil\ndate +'%s'", P],
  ['Q7', A, "date +$'\\''x\necho x > CLAUDE.md\ndate +'%s'", P],
  ['R1', N, 'git status >&CLAUDE.md', P],
  ['R2', A, 'git log -1 --oneline >&CLAUDE.md', P],
  ['R3', N, 'git status >&.claude/settings.json', P],
  ['S2a', N, 'git commit -q -m m CLAUDE.md', P, dirtyRules],
  ['-- .', N, 'git commit -q -m m -- .', P, dirtyRules],
  ['-i', N, 'git commit -q -m m -i CLAUDE.md', P, dirtyRules],
  ['-o', N, 'git commit -q -m m -o CLAUDE.md', P, dirtyRules],
  [':(top)', N, "git commit -q -m m ':(top)CLAUDE.md'", P, dirtyRules],
  ['N3', N, 'git commit -q -m m -- settings.json', join(P, '.claude'), scratchBranch(() => writeFileSync(join(P, '.claude', 'settings.json'), '{"a":9}\n'))],
  ['P3-6 switch', N, 'git switch -q cs/sub', P, () => g('switch', '-q', 'cs/gm')],
  ['P3-6 push', N, 'git push -q origin cs/sub3', P, () => g('switch', '-q', '-c', 'cs/sub3', 'origin/cs/sub')],
  ['H8 day', null, "git status # '\ngit switch -q cs/evil # '", P, null, true],
  ['A1 day', null, "git status  # what's changed\ngit switch -q cs/evil", P, null, true],
  ['D5 day', null, "git status \\\n# '\ngit switch -q cs/evil # '", P, null, true],
  ['AC day', null, "printf $'a\\'b'; git switch -q cs/evil", P, null, true],
  ['HQ day', null, "cat <<'E O'\nE O\ngit switch -q cs/evil\nE", P, null, true],
  ['R4 day', null, 'git status >&CLAUDE.md', P, null, true],
  ['ctl night 2>&1', N, 'git status 2>&1', P],
  ['ctl night commit', N, 'git commit -q -m "fix #16: it\'s done"', P, scratchBranch()],
  ['ctl pra step0', A, 'git fetch -q origin main && git switch --detach origin/main', P],
  ['ctl day heredoc', null, 'cat <<EOF\nbody\nEOF', P, null, true],
];
for (const [label, role, line, cwd, setup, agentToo] of rows) {
  setup?.();
  const res = [];
  for (const v of ['old', 'new']) {
    const io = { ...G[v].realIo(), env: role ? { SHAMBLE_ROLE: role } : {} };
    const d = await G[v].decide(ev(line, cwd), io);
    let s = `${v}: ${kind(d)}`;
    if (agentToo) s += ` agent ${kind(await G[v].decide(ev(line, cwd, { agent_id: 'a1' }), io))}`;
    if (v === 'new' && d) s += ` (${String(d.ask ?? d.deny).slice(0, 70)})`;
    res.push(s);
  }
  const r = sh(line, cwd);
  console.log(`${label} [${role ?? 'day'}] | ${res.join(' | ')} | bash exit ${r.status} ${state()}${label === 'P3-6 push' ? ' origin cs/sub3=' + (g('ls-remote', 'origin', 'cs/sub3').stdout ? 'yes' : 'no') : ''}`);
  reset();
}
console.log('git', g('--version').stdout.trim(), '| bash', sh('echo $BASH_VERSION').stdout.trim());
rmSync(root, { recursive: true, force: true });
