// verifier: head guard's day decisions on other commands that move the main folder's HEAD or files (real io, project = <worktree>)
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const WT = process.argv[2];
const { decide, realIo } = await import(pathToFileURL(join(WT, '.claude/hooks/routine-guard.mjs')).href);
const io = realIo(); io.projectDir = WT; io.env = { ...process.env }; delete io.env.SHAMBLE_ROLE;
const cmds = ['git reset --keep origin/kit-v3-2', 'git reset --merge 4def8d2', 'git reset 4def8d2', 'git reset --hard 4def8d2',
  'git symbolic-ref HEAD refs/heads/kit-v3-2', 'git checkout 4def8d2', 'git restore --source=4def8d2 .claude',
  'git stash pop', 'git cherry-pick 4def8d2', 'git switch kit-v3-2', 'git worktree add .gate/vr-x 4def8d2',
  'git -C . switch kit-v3-2', 'git branch -f kit-v3-3 4def8d2', 'git merge --ff-only origin/main', 'git pull --ff-only origin main'];
for (const c of cmds) {
  const d = await decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command: c }, cwd: WT }, io);
  console.log(`${c}: ${d ? (d.deny !== undefined ? 'deny: ' + d.deny.slice(0, 90) : 'ask: ' + d.ask.slice(0, 90)) : 'null'}`);
}
