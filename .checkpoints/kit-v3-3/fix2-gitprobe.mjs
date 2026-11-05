// builder-fix2: how real git behaves for the forms the SA-2 routine rules read. Usage: node fix2-gitprobe.mjs <worktree>
// Throwaway repos under <worktree>/.gate/kit-v3-3/fix2-git/. Prints exit codes and ref names only.
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
const D = join(process.argv[2], '.gate/kit-v3-3/fix2-git');
rmSync(D, { recursive: true, force: true }); mkdirSync(D, { recursive: true });
const env = { ...process.env, GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1', GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t' };
const R = join(D, 'r');
const g = (...a) => { const r = spawnSync('git', a, { cwd: R, env, encoding: 'utf8' }); return `${r.status} ${(r.stdout || '').trim().replace(/[0-9a-f]{40}/g, '<sha>').slice(0, 200)} ${(r.stderr || '').trim().split('\n')[0].slice(0, 120)}`; };
mkdirSync(R);
console.log('init', g('init', '-q', '-b', 'main'), g('commit', '-q', '--allow-empty', '-m', 'one'));
const heads = () => g('for-each-ref', '--format=%(refname)');
console.log('branch -v foo:', g('branch', '-v', 'foo'), '|', heads());
console.log('branch -vv bar:', g('branch', '-vv', 'bar'), '|', heads());
console.log('branch -a baz:', g('branch', '-a', 'baz'), '|', heads());
console.log('branch -r qux:', g('branch', '-r', 'qux'), '|', heads());
console.log('branch --contains main:', g('branch', '--contains', 'main'));
console.log('branch HEAD:', g('branch', 'HEAD'));
console.log('symbolic-ref write:', g('symbolic-ref', 'refs/remotes/origin/main', 'refs/heads/foo'), '| full ref ->', g('rev-parse', '--verify', 'refs/remotes/origin/main'));
console.log('symbolic-ref read:', g('symbolic-ref', 'HEAD'), g('symbolic-ref', '--short', 'HEAD'), g('symbolic-ref', '-q', 'HEAD'));
const cfg = () => readFileSync(join(R, '.git/config'), 'utf8').split('\n').filter((l) => /^\[|url|fetch/.test(l)).join(' ');
console.log('remote add:', g('remote', 'add', 'origin', '/nonexistent'), '|', cfg());
console.log('remote set-url:', g('remote', 'set-url', 'origin', '/other'), '|', cfg());
console.log('remote set-branches:', g('remote', 'set-branches', 'origin', 'x'), '|', cfg());
console.log('remote -v:', g('remote', '-v'), 'get-url:', g('remote', 'get-url', 'origin'));
console.log('switch --orphan:', g('switch', '-q', '--orphan', 'orph'), '|', heads(), g('switch', '-q', 'main'));
console.log('worktree add DWIM:', g('worktree', 'add', '-q', join(D, 'wt', 'main')), '|', heads());
console.log('git version', g('version'));
