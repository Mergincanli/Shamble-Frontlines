// kit-v3-3 step 4b, S-1 with real git: in a throwaway repo under .gate/kit-v3-3/s1-probe-<n>/, run the base guard
// (f22c4df, copied to .gate/kit-v3-3/mut-step4b-1/) and the current guard on `git switch -- --output=<path>` and report
// whether a file was written. Usage (worktree root): node .checkpoints/kit-v3-3/s1-probe.mjs
import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, realpathSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
for (const [label, file] of [['base f22c4df', '.gate/kit-v3-3/mut-step4b-1/.claude/hooks/routine-guard.mjs'], ['step 4b', '.claude/hooks/routine-guard.mjs']]) {
  const dir = resolve('.gate/kit-v3-3', `s1-probe-${label === 'step 4b' ? 'new' : 'base'}-${Date.now()}`);
  mkdirSync(dir, { recursive: true });
  const g = (...a) => spawnSync('git', a, { cwd: dir, env, encoding: 'utf8' });
  g('init', '-q', '-b', 'main'); g('commit', '-q', '--allow-empty', '-m', 'one'); g('update-ref', 'refs/remotes/origin/main', 'HEAD');
  process.env.CLAUDE_PROJECT_DIR = realpathSync(dir);
  const m = await import(pathToFileURL(resolve(file)).href);
  const io = { ...m.realIo(), env: {} };
  const out = join(dir, 'leak.txt');
  const d = await m.decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command: `git switch -- --output=${out}` }, cwd: realpathSync(dir) }, io);
  console.log(`${label}: decision ${JSON.stringify(d)?.slice(0, 90) ?? 'null'}; file written: ${existsSync(out)}`);
}
