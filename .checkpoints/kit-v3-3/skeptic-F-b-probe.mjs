// skeptic F-b: day decisions for git worktree add / git clone into kit folders (real io, project = worktree)
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const WT = process.argv[2];
const { decide, realIo, kitPath, protectedRel } = await import(pathToFileURL(join(WT, '.claude/hooks/routine-guard.mjs')).href);
const io = realIo(); io.projectDir = WT; io.env = { ...process.env }; delete io.env.SHAMBLE_ROLE;
const cmds = ['git worktree add .claude/rules/wt 4def8d2', 'git worktree add .claude/skills/x 4def8d2', 'git worktree add tools/x 4def8d2',
  'git clone https://example.invalid/x .claude/skills/x', 'git worktree add .gate/x 4def8d2', 'cp a .claude/rules/wt', 'mkdir .claude/rules/wt'];
for (const c of cmds) {
  for (const agent of [false, true]) {
    const input = { hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command: c }, cwd: WT };
    if (agent) input.agent_id = 'a1';
    const d = await decide(input, io);
    console.log(`${agent ? 'agent' : 'main '} ${c}: ${d ? (d.deny !== undefined ? 'deny: ' + d.deny.slice(0, 80) : 'ask: ' + d.ask.slice(0, 80)) : 'null'}`);
  }
}
for (const p of ['.claude/rules/wt', '.claude/rules/wt/x.md', '.claude/skills/x/SKILL.md']) console.log(p, protectedRel(p), kitPath(join(WT, p), WT, io));
