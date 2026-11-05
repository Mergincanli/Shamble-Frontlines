// verifier pass 3 probe: real git 2.x, the guard at 01b158c (the worktree's source). Prints kind per row, no env values.
import { spawnSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, writeFileSync, realpathSync } from 'node:fs';
import { join } from 'node:path';
const G = process.argv[2]; // path to routine-guard.mjs
const base = process.argv[3]; // scratch folder
const { decide, realIo } = await import(G);
const root = realpathSync(mkdtempSync(join(base, 'p3-')));
const O = join(root, 'o.git'), P = join(root, 'p');
const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
const g = (...a) => spawnSync('git', a, { cwd: P, env, encoding: 'utf8' });
spawnSync('git', ['init', '-q', '--bare', '-b', 'main', O], { env });
mkdirSync(join(P, '.claude'), { recursive: true }); mkdirSync(join(P, 'docs'));
writeFileSync(join(P, 'CLAUDE.md'), 'r\n'); writeFileSync(join(P, '.claude', 'settings.json'), '{"a":1}\n'); writeFileSync(join(P, 'docs', 'a.md'), 'a\n');
g('init', '-q', '-b', 'main'); g('add', '-A'); g('commit', '-q', '-m', 'b'); g('remote', 'add', 'origin', O); g('push', '-q', 'origin', 'main'); g('fetch', '-q', 'origin');
g('switch', '-q', '-c', 'cs/ok'); writeFileSync(join(P, 'docs', 'a.md'), 'ok\n'); g('commit', '-q', '-am', 'ok'); g('push', '-q', 'origin', 'cs/ok'); g('switch', '-q', 'main');
process.env.CLAUDE_PROJECT_DIR = P;
const kind = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : null);
const io = (role) => ({ ...realIo(), env: role ? { SHAMBLE_ROLE: role } : {} });
const run = async (label, role, command, cwd = P) => {
  const d = await decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd }, io(role));
  console.log(`${label}\t${role || 'day'}\t${kind(d)}\t${(d?.deny ?? d?.ask ?? '').slice(0, 140)}`);
};
const N = 'night-shift';
// commit check: dirty unstaged kit file, nothing staged, on a scratch branch
g('switch', '-q', '-c', 'cs/c'); writeFileSync(join(P, '.claude', 'settings.json'), '{"a":9}\n');
writeFileSync(join(P, 'docs', 'list.txt'), '.claude/settings.json\n');
await run('N1 commit pathspec without --', N, 'git commit -m m .claude/settings.json');
await run('N2 commit -- pathspec', N, 'git commit -m m -- .claude/settings.json');
await run('N3 commit -- relative from .claude', N, 'git commit -m m -- settings.json', join(P, '.claude'));
await run('N4 commit --pathspec-from-file', N, 'git commit -m m --pathspec-from-file=docs/list.txt');
await run('N5 commit -i pathspec', N, 'git commit -i -m m .claude/settings.json');
await run('N5b commit -o pathspec', N, 'git commit -o -m m .claude/settings.json');
g('checkout', '--', '.claude/settings.json'); g('switch', '-q', 'main'); g('branch', '-q', '-D', 'cs/c');
// line shapes
await run('N6 switch with redirect into .git', N, 'git switch cs/ok > .git/refs/remotes/origin/cs/ok');
await run('N7 switch 2>&1 alone', N, 'git switch cs/ok 2>&1');
await run('N8 switch then comment', N, 'git switch cs/ok # ; git fetch');
await run('N9 VAR prefix', N, 'GIT_DIR=.git git switch cs/ok');
await run('N10 timeout wrapper', N, 'timeout 60 git switch cs/ok');
await run('N11 switch @{-1}', N, 'git switch @{-1}');
await run('N12 switch @', N, 'git switch @');
await run('N13 commit heredoc -F -', N, "git commit -F - <<'X'\nmsg\nX");
await run('N14 push alone current branch main', N, 'git push origin main');
await run('N15 switch newline fetch', N, 'git switch cs/ok\ngit fetch origin cs/ok');
await run('N16 xargs git switch', N, 'echo cs/ok | xargs git switch');
await run('N17 switch --detach origin/main then', N, 'git switch --detach origin/main');
await run('P1 pr-advisor step 0', 'pr-advisor', 'git fetch -q origin main && git switch --detach origin/main');
await run('P2 pr-advisor switch cs/ok', 'pr-advisor', 'git switch cs/ok');
await run('P3 pr-advisor fetch && switch cs', 'pr-advisor', 'git fetch -q origin cs/ok && git switch --detach origin/cs/ok');
// day controls in the main folder
await run('D1 WT-UPDATE', '', 'git fetch origin main && git merge --ff-only origin/main');
await run('D2 new branch from origin/main', '', 'git fetch origin main && git switch -c cs/n origin/main');
await run('D3 add && commit', '', 'git add -A && git commit -m x');
await run('D4 rm -rf scratch folder outside', '', `rm -rf ${join(root, 'nothing')}`);
await run('D5 git status && git log', '', 'git status && git log -1');
await run('D6 cd docs && cd .. && echo x > docs/b.md', '', 'cd docs && cd .. && echo x > docs/b.md');
console.log('ROOT', root.replace(base, '<scratch>'));
