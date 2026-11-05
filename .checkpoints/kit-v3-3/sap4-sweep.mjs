// pass 4 sweep: P3-4, P3-9, R-PLAIN edges and the night commit allowlist on the head guard (decisions only; real git
// repo with CLAUDE.md staged or modified). usage: node sap4-sweep.mjs <guard.mjs> <scratch dir>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, realpathSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [guardPath, scratch] = process.argv.slice(2);
const { decide, realIo } = await import(pathToFileURL(guardPath).href);
const root = join(scratch, 'sap4-sweep');
rmSync(root, { recursive: true, force: true }); mkdirSync(root, { recursive: true });
const R = realpathSync(root), O = join(R, 'origin.git'), P = join(R, 'proj');
const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
const g = (...a) => spawnSync('git', a, { cwd: P, env, encoding: 'utf8' });
spawnSync('git', ['init', '-q', '--bare', '-b', 'main', O], { env });
mkdirSync(join(P, 'docs'), { recursive: true });
writeFileSync(join(P, 'CLAUDE.md'), 'rules\n'); writeFileSync(join(P, 'docs', 'a.md'), 'a\n');
g('init', '-q', '-b', 'main'); g('add', '-A'); g('commit', '-q', '-m', 'base');
g('remote', 'add', 'origin', O); g('push', '-q', 'origin', 'main'); g('fetch', '-q', 'origin'); g('switch', '-q', '-c', 'cs/n');
process.env.CLAUDE_PROJECT_DIR = P;
const kind = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : 'null');
const row = async (role, line, bg = false) => {
  const ti = { command: line }; if (bg) ti.run_in_background = true;
  const d = await decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: ti, cwd: P, scratchpad_dir: join(R, 's') }, { ...realIo(), env: { SHAMBLE_ROLE: role } });
  console.log(role, bg ? '[bg]' : '', JSON.stringify(line), '->', kind(d), d ? String(d.deny ?? d.ask).slice(0, 95) : '');
};
const N = 'night-shift', A = 'pr-advisor';
console.log('-- P3-4');
await row(N, 'git fetch -q origin main &');
await row(N, 'git fetch -q origin main', true);
await row(A, 'git fetch -q origin main', true);
await row(N, 'git fetch -q origin main & date');
await row(N, 'date &\ngit fetch -q origin main');
await row(N, 'git status 2>&1');
await row(N, 'git status |& git log -1');
console.log('-- P3-9 / R-PLAIN redirect edges');
for (const l of ['git status >&CLAUDE.md', 'git status >& CLAUDE.md', 'git status 1>&CLAUDE.md', 'git status >&2CLAUDE.md', 'git status >&-CLAUDE.md', 'git status >&2-x', 'git status <&0', 'git status >>&CLAUDE.md', 'git status &>CLAUDE.md', 'git status 2>CLAUDE.md', 'git status >|CLAUDE.md', 'git status &>>CLAUDE.md', 'git status >&2', 'git status >&-', 'git status 2>&1-']) await row(N, l);
console.log('-- commit allowlist, CLAUDE.md modified (unstaged), docs/a.md staged');
writeFileSync(join(P, 'docs', 'a.md'), 'b\n'); g('add', 'docs/a.md'); writeFileSync(join(P, 'CLAUDE.md'), 'evil\n');
for (const l of ['git commit -q -m x', 'git commit -qam x', 'git commit -sa -m x', 'git commit --all -m x', 'git commit -m x -a', 'git commit -m -a', 'git commit -m x CLAUDE.md', 'git commit -mx -- CLAUDE.md', 'git commit --message=x --only', 'git commit -m x --al', 'git commit -m x --allow-empty', 'git commit -m=x', 'git commit -m', 'git commit -m x -S', 'git commit -m x --no-verify']) await row(N, l);
