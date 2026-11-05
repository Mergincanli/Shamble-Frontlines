// verifier E5: the real guard through settings.json's own bootstrap (args read from the file), project dir = <worktree>.
// Prints each case's exit code and stdout verbatim (decisions only). Usage: node vr-e5.mjs <worktree>
import { spawnSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
const WT = process.argv[2];
const s = JSON.parse(readFileSync(join(WT, '.claude/settings.json'), 'utf8'));
const boot = (ev) => s.hooks[ev][0].hooks[0];
const run = (ev, input, role) => {
  const h = boot(ev);
  const args = h.args.map((a) => a.replace('${CLAUDE_PROJECT_DIR}', WT));
  const env = { ...process.env, CLAUDE_PROJECT_DIR: WT }; delete env.SHAMBLE_ROLE; if (role) env.SHAMBLE_ROLE = role;
  const r = spawnSync(h.command, args, { input: JSON.stringify({ hook_event_name: ev, cwd: WT, ...input }), env, encoding: 'utf8' });
  return `exit ${r.status} stdout ${r.stdout || '(none)'}`;
};
const ed = (p, extra = {}) => ({ tool_name: 'Edit', tool_input: { file_path: join(WT, p), old_string: 'a', new_string: 'b' }, ...extra });
const sh = (c, extra = {}) => ({ tool_name: 'Bash', tool_input: { command: c }, ...extra });
const cases = [
  ['S2 day Edit .claude/settings.json', 'PreToolUse', ed('.claude/settings.json'), ''],
  ['S2b day Write docs/x/CLAUDE.md', 'PreToolUse', { tool_name: 'Write', tool_input: { file_path: join(WT, 'docs/x/CLAUDE.md'), content: 'x' } }, ''],
  ['S2c day Edit shamble-game.js', 'PreToolUse', ed('shamble-game.js'), ''],
  ['S2d day subagent Edit .claude/settings.json', 'PreToolUse', ed('.claude/settings.json', { agent_id: 'a1', agent_type: 'advisor' }), ''],
  ['S3 night Edit .claude/settings.json', 'PreToolUse', ed('.claude/settings.json'), 'night-shift'],
  ['S3b pr-advisor Edit .claude/settings.json', 'PreToolUse', ed('.claude/settings.json'), 'pr-advisor'],
  ['S4 day git switch cs/x (project root)', 'PreToolUse', sh('git switch cs/x'), ''],
  ['S4b day echo > .claude/settings.json', 'PreToolUse', sh('echo x > .claude/settings.json'), ''],
  ['S5 day git worktree add .gate/x HEAD', 'PreToolUse', sh('git worktree add .gate/x HEAD'), ''],
  ['S6 day git rev-list --output=x HEAD', 'PreToolUse', sh('git rev-list --output=x HEAD'), ''],
  ['A37a day PermissionRequest Edit kit', 'PermissionRequest', ed('.claude/settings.json'), ''],
];
for (const [n, ev, inp, role] of cases) console.log(`${n} [${role || 'day'}]: ${run(ev, inp, role)}`);
