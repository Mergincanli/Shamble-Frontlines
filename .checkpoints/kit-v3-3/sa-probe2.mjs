// security-auditor probe 2: the night shift's push check from a worktree under .gate/ (plan §10 follow-up scope).
// Usage: node sa-probe2.mjs <worktree>  (needs sa-probe.mjs run first: reuses its repos). Decisions only.
import { spawnSync } from 'node:child_process';
import { join } from 'node:path';
const WT = process.argv[2], D = join(WT, '.gate/kit-v3-3/sa-probe'), P = join(D, 'proj');
const ENV = { ...process.env, GIT_CONFIG_GLOBAL: '/dev/null' };
spawnSync('bash', ['-c', 'git worktree remove --force .gate/w 2>/dev/null; mkdir -p .gate && git worktree add -q .gate/w cs/evil'], { cwd: P, env: ENV });
process.env.CLAUDE_PROJECT_DIR = P;
for (const [k, f] of [['head', join(WT, '.claude/hooks/routine-guard.mjs')], ['base', join(D, 'base-guard.mjs')]]) {
  const m = await import(f);
  const io = { ...m.realIo(), projectDir: P, env: { SHAMBLE_ROLE: 'night-shift' } };
  const d = await m.decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command: 'git push origin cs/evil' }, cwd: join(P, '.gate/w') }, io);
  console.log(`${k}: night-shift push of cs/evil (changes .claude/settings.json) from .gate/w ->`, d === null ? 'null' : d.deny ? 'DENY' : 'ASK');
}
