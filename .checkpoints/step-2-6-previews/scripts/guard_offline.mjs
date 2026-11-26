// Asks main's guard rules (routine-guard.mjs decide(), the same function the PreToolUse hook calls) what they decide for
// a Bash command in the main folder, without running the command. Read-only: the guard's own git lookups and file reads.
// Usage (from the repository root): node .checkpoints/step-2-6-previews/scripts/guard_offline.mjs
import { pathToFileURL } from 'node:url';
import { join } from 'node:path';

const root = process.cwd();
const { decide, realIo } = await import(pathToFileURL(join(root, '.claude/hooks/routine-guard.mjs')).href);
const target = 'origin/claude/optimistic-albattani-mdoonz';
const cases = [
  ['A bare merge of the branch', `git merge --ff-only ${target}`],
  ['B the exact line of 2026-11-25 21:00:05 UTC', `cd ${root} && git status --short --branch | head -2 && git merge --ff-only ${target} 2>&1 | tail -2 && git log --oneline -1 && git status --short --branch | head -1`],
  ['C cd in front only', `cd ${root} && git merge --ff-only ${target}`],
  ['D git status in front, pipes after', `git status --short --branch | head -2 && git merge --ff-only ${target} 2>&1 | tail -2`],
  ['E control: the allowed form', 'git merge --ff-only origin/main'],
];
for (const [name, command] of cases) {
  const input = { hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd: root, session_id: 'offline-check' };
  const d = await decide(input, realIo());
  const verdict = d == null ? 'ALLOW (no decision)' : d.ask !== undefined && d.deny === undefined ? `ASK: ${d.ask}` : `DENY: ${d.deny}`;
  console.log(`${name}\n  -> ${verdict.slice(0, 260)}`);
}
