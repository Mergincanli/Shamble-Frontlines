// skeptic SD-6 probe (from verifier vd-probe2) (A3, ed3ea8c): auto-4 #9 (SA-6 reads the index only), #10 (a variable name built from an
// expansion), #11 (git clone value options). Day decisions on a scratch repo under <a3>/.gate/kit-v3-3/skeptic-sd6.
// Prints decisions only. Usage: node skeptic-sd6.mjs <a3 worktree>
import { mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { join } from 'node:path';
const A3 = process.argv[2];
const base = join(A3, '.gate/kit-v3-3/skeptic-sd6');
rmSync(base, { recursive: true, force: true });
Object.assign(process.env, { GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t',
  GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' });
delete process.env.SHAMBLE_ROLE;
const g = (cwd, ...a) => spawnSync('git', a, { cwd, encoding: 'utf8' });
const proj = join(base, 'proj');
mkdirSync(join(proj, '.claude'), { recursive: true }); mkdirSync(join(proj, 'docs/sub'), { recursive: true });
g(base, 'init', '-q', '-b', 'main', proj);
writeFileSync(join(proj, 'CLAUDE.md'), 'r\n'); writeFileSync(join(proj, '.claude/settings.json'), '{}\n'); writeFileSync(join(proj, 'docs/a.md'), 'a\n');
writeFileSync(join(proj, '.gitignore'), '.gate/\n');
g(proj, 'add', '-A'); g(proj, 'commit', '-q', '-m', 'b');
mkdirSync(join(proj, '.gate')); g(proj, 'worktree', 'add', '-q', join(proj, '.gate/wt'), '-b', 'cs/wt');
writeFileSync(join(proj, 'docs/sub/CLAUDE.md'), 'untracked\n'); // untracked nested CLAUDE.md
process.env.CLAUDE_PROJECT_DIR = proj;
const { decide, realIo } = await import(join(A3, '.claude/hooks/routine-guard.mjs'));
const io = realIo(); io.env = { ...process.env };
const run = async (label, cmd) => {
  const d = await decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command: cmd }, cwd: proj, session_id: 's' }, io);
  console.log(`${label}: ${d === null ? 'allow' : d.ask ? 'ASK - ' + d.ask.slice(0, 100) : 'DENY - ' + String(d.deny).slice(0, 100)}`);
};
await run('#9 control rm -rf .claude', 'rm -rf .claude');
await run('#9 control rm -rf .gate/wt (a worktree top)', 'rm -rf .gate/wt');
await run('#9 rm -rf .gate (holds a worktree with .claude)', 'rm -rf .gate');
await run('#9 rm -rf docs (untracked docs/sub/CLAUDE.md)', 'rm -rf docs');
await run('#9 rm -r .gate', 'rm -r .gate');
await run('#9 mv .gate /tmp/x', 'mv .gate ' + join(base, 'moved'));
await run('#9 abs rm -rf <proj>/.gate', 'rm -rf ' + join(proj, '.gate'));
await run('#9 control rm -rf .gate/wt/.claude', 'rm -rf .gate/wt/.claude');
rmSync(base, { recursive: true, force: true });
