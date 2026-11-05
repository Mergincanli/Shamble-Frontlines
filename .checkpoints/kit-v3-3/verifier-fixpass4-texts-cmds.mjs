// verifier: every command in NIGHT_SHIFT.md, CHECKPOINTS.md and main's advisor.md (placeholders filled) through head's
// plainForm (R-PLAIN with R-PROG) and through head's and 60ae124's decide for its role (lite io)
import { pathToFileURL } from 'node:url';
const [headG, oldG] = process.argv.slice(2);
const H = await import(pathToFileURL(headG).href), O = await import(pathToFileURL(oldG).href);
const L = [
  ['night-shift', 'NIGHT_SHIFT.md', 'TZ=Europe/Amsterdam date'], ['night-shift', 'NIGHT_SHIFT.md', 'date'],
  ['night-shift', 'NIGHT_SHIFT.md', 'git add docs/a.md docs/b.md'], ['night-shift', 'NIGHT_SHIFT.md', 'git commit -m "fix #16: it\'s done"'],
  ['night-shift', 'NIGHT_SHIFT.md', 'npm ci --prefix tools --ignore-scripts --no-audit --no-fund'],
  ['night-shift', 'CHECKPOINTS.md', 'git log --oneline -5'], ['night-shift', 'CHECKPOINTS.md', 'git ls-tree -r --name-only HEAD -- .checkpoints'],
  ['night-shift', 'CHECKPOINTS.md', 'git worktree add .gate/kit-v3-3-wt cs/x'], ['night-shift', 'CHECKPOINTS.md', 'git worktree prune'],
  ['night-shift', 'CHECKPOINTS.md', 'node .claude/scripts/checkpoint.mjs save kit-v3-3 -m "step 2: done" --trailer "Claude-Session: https://claude.ai/code/session_x"'],
  ['night-shift', 'CHECKPOINTS.md', 'node .claude/scripts/checkpoint.mjs watch kit-v3-3 --trailer "Claude-Session: https://claude.ai/code/session_x"'],
  ['pr-advisor', 'advisor.md (main)', 'git fetch -q origin main && git switch --detach origin/main'],
  ['pr-advisor', 'advisor.md (main)', 'git diff --name-only origin/main...' + 'a'.repeat(40)],
  ['pr-advisor', 'advisor.md (main)', 'node tools/gate.mjs'], ['night-shift', 'NIGHT_SHIFT.md (gate)', 'node tools/gate.mjs'],
];
const lite = (role) => ({ env: { SHAMBLE_ROLE: role }, projectDir: '/repo', home: '/home/u', gitLongOptions: () => [],
  git: (args) => (args.join(' ') === 'rev-parse --show-toplevel' ? { code: 0, out: '/repo\n' } : { code: 0, out: '' }),
  gh: () => ({ code: 1, out: '' }), exists: () => false, realpath: (p) => p, readFile: () => '{}' });
const ev = (command) => ({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd: '/repo', scratchpad_dir: '/tmp/s' });
const k = (d) => (d ? 'deny: ' + String(d.deny ?? d.ask).slice(0, 90) : 'null');
let bad = 0;
for (const [role, src, line] of L) {
  const pf = H.plainForm(line), a = k(await O.decide(ev(line), lite(role))), b = k(await H.decide(ev(line), lite(role)));
  if (pf !== null) bad++;
  console.log(`${pf === null ? 'PLAIN' : 'REFUSED ' + JSON.stringify(pf)} | ${role} | ${src} | ${JSON.stringify(line)} | 60ae124 ${a} | head ${b}`);
}
console.log(`commands ${L.length}, refused by head plainForm ${bad}`);
