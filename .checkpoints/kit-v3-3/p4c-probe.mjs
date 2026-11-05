// kit-v3-3 step 4c real-git and real-bash probe, in throwaway folders under .gate/kit-v3-3/. Finding 6: a branch and a
// tag named dup at different commits; only the branch's commit differs from origin/main in a kit file
// (.claude/settings.json). The base guard (HEAD) and the step-4c guard decide `git switch dup` there. Finding 5: real
// bash shows where `CDPATH=... cd w2` lands. Usage (from the worktree root): node .checkpoints/kit-v3-3/p4c-probe.mjs
import { spawnSync } from 'node:child_process';
import { mkdirSync, realpathSync, rmSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
const G = resolve('.gate/kit-v3-3');
const repo = join(G, 'p4c-probe2');
rmSync(repo, { recursive: true, force: true });
mkdirSync(join(repo, '.claude'), { recursive: true });
const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t',
  GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
const g = (...a) => spawnSync('git', a, { cwd: repo, env, encoding: 'utf8' });
g('init', '-q', '-b', 'main');
writeFileSync(join(repo, '.claude', 'settings.json'), 'a\n');
g('add', '-A'); g('commit', '-q', '-m', 'one');
g('remote', 'add', 'origin', repo); g('fetch', '-q', 'origin'); // origin/main = commit one
g('tag', 'dup');
g('switch', '-q', '-c', 'dup');
writeFileSync(join(repo, '.claude', 'settings.json'), 'b\n');
g('commit', '-q', '-am', 'two'); g('switch', '-q', 'main');
const rev = (r) => g('rev-parse', '--verify', '--end-of-options', `${r}^{commit}`).stdout.trim().slice(0, 7);
console.log(`git ${g('--version').stdout.trim()}; origin/main ${rev('refs/remotes/origin/main')}, tag dup ${rev('refs/tags/dup')}, ` +
  `branch dup ${rev('refs/heads/dup')}, short name dup reads ${rev('dup')}`);
const base = join(G, 'p4c-base-guard.mjs');
writeFileSync(base, spawnSync('git', ['show', 'HEAD:.claude/hooks/routine-guard.mjs'], { encoding: 'utf8' }).stdout);
process.env.CLAUDE_PROJECT_DIR = realpathSync(repo);
const input = (command) => ({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd: realpathSync(repo) });
for (const [label, file] of [['base 546d5a7', base], ['step 4c', resolve('.claude/hooks/routine-guard.mjs')]]) {
  const m = await import(pathToFileURL(file).href);
  const io = { ...m.realIo(), env: {} };
  for (const c of ['git switch dup', 'git switch --detach dup', 'git switch main']) {
    const d = await m.decide(input(c), io);
    console.log(`${label}: ${c} -> ${d ? (d.ask ? 'ask' : 'deny') + ': ' + (d.ask ?? d.deny).slice(0, 120) : 'goes on'}`);
  }
}
g('switch', '-q', 'dup');
console.log(`git switch dup lands on ${rev('HEAD')} (the branch is ${rev('refs/heads/dup')})`);
// finding 5, real bash
const sh = join(G, 'p4c-cdpath');
rmSync(sh, { recursive: true, force: true });
mkdirSync(join(sh, 'here', 'w2'), { recursive: true });
mkdirSync(join(sh, 'elsewhere', 'w2'), { recursive: true });
for (const line of ['cd w2 && pwd', `CDPATH=${join(sh, 'elsewhere')} cd w2 >/dev/null && pwd`, `export CDPATH=${join(sh, 'elsewhere')}; cd w2 >/dev/null && pwd`,
  `export CDPATH=${join(sh, 'elsewhere')}; cd ./w2 && pwd`]) {
  const r = spawnSync('bash', ['--norc', '--noprofile', '-c', line], { cwd: join(sh, 'here'), env: { PATH: process.env.PATH }, encoding: 'utf8' });
  console.log(`bash in here/: ${line.replace(sh, '<p>')} -> ${r.stdout.trim().replace(sh, '<p>')}`);
}
