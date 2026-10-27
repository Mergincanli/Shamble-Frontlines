// Tests for .claude/settings.json (kit-v3-1 plan, evidence C3a): node --test .claude/hooks/settings.test.mjs
// It checks the static layer, not the guard: the ask list is empty (an ask stalls a routine), effortLevel is high,
// the guard hook is registered on the three events with onFailure "block", and — through an emulation of Claude Code's
// documented Bash-rule matching — every command the two routines need is matched by an allow and by no deny, while
// every "never" is caught by a static deny (the routine-only nevers are the guard's job, tested in routine-guard.test).
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const HERE = dirname(fileURLToPath(import.meta.url));
const settings = JSON.parse(readFileSync(join(HERE, '..', 'settings.json'), 'utf8'));
const { allow = [], deny = [], ask = [] } = settings.permissions ?? {};

// ---- an emulation of the documented Bash-rule matching (prefix + glob; a trailing " *" also matches the bare form) ----
const bashRule = (rule) => { const m = /^Bash\((.*)\)$/.exec(rule); return m ? m[1] : null; };
const globToRe = (glob) => new RegExp('^' + glob.replace(/[.+?^${}()|[\]\\]/g, '\\$&').replace(/\*/g, '.*') + '$');
function ruleMatches(pattern, cmd) {
  const parts = cmd.split(/\s*(?:&&|\|\||;|\|)\s*/).map((s) => s.trim()).filter(Boolean);
  const res = [globToRe(pattern)];
  if (pattern.endsWith(' *')) res.push(globToRe(pattern.slice(0, -2))); // the bare form
  return [cmd, ...parts].some((c) => res.some((re) => re.test(c)));
}
const rulesOf = (list) => list.map(bashRule).filter((x) => x !== null);
const deniedBy = (cmd) => rulesOf(deny).some((p) => ruleMatches(p, cmd));
const allowedBy = (cmd) => rulesOf(allow).some((p) => ruleMatches(p, cmd));

// Command forms the two routines need (from the audit traces). None may be caught by a deny; each should match an allow.
const NEEDED = [
  'git push -u origin cs/x', 'git push origin cs/x', 'git fetch -q origin main', 'git switch -c cs/x origin/main',
  'git switch --detach origin/main', 'git commit -m "a message"', 'git worktree add .gate/base origin/main',
  'git worktree remove .gate/base', 'git rm -r .checkpoints/x.md', 'git reset -q HEAD -- .checkpoints',
  'git ls-remote --heads origin cs/x', 'git rev-parse HEAD', 'git ls-files', 'git grep -n foo',
  'node tools/gate.mjs origin/main', 'node tools/gate.mjs', 'node .claude/scripts/checkpoint.mjs save s -m x',
  'node tools/preview.js . eval tools/sim.js', 'node --expose-gc tools/preview.js . eval tools/perfbots.js',
  'node tools/smoke.mjs', 'node --test .claude/hooks/routine-guard.test.mjs', 'mkdir -p .checkpoints/x',
  'npm ci --prefix tools --ignore-scripts --no-audit --no-fund', 'gh pr view 27', 'gh pr diff 27',
  'gh api repos/Mergincanli/Shamble-Frontlines/pulls/27',
];
// Command forms the routines must never run that a STATIC deny catches (the routine-only nevers are the guard's job).
const NEVER = [
  'git push --force origin cs/x', 'git push -f origin cs/x', 'git push origin main', 'git push origin :cs/x',
  'git push', 'git reset --hard', 'git reset --soft HEAD~1', 'git rebase main', 'git commit --amend --no-edit',
  'git checkout main', 'git restore x', 'git branch -D cs/x', 'git switch -f cs/x', 'git worktree remove --force .gate/x',
  'git filter-repo --force', 'git update-ref refs/heads/main x', 'rm -rf node_modules', 'npx create-react-app x',
  'npm run build', 'bash -c "git push -f"', 'eval "git status"', 'gh pr merge 27', 'gh auth token',
  'gh api -X DELETE repos/x/git/refs/heads/y', 'git config --list', 'git config --get http.x.extraheader',
];

test('the ask list is empty (every ask would stall a routine)', () => assert.deepEqual(ask, []));
test('effortLevel is high (ROUTINE-EFFORT)', () => assert.equal(settings.effortLevel, 'high'));

test('the guard hook is registered on the three events with onFailure block', () => {
  for (const ev of ['PreToolUse', 'PermissionRequest', 'ConfigChange']) {
    const handlers = (settings.hooks?.[ev] ?? []).flatMap((g) => g.hooks ?? []);
    const h = handlers.find((x) => (x.args ?? []).some((a) => String(a).includes('guard.mjs')));
    assert.ok(h, `${ev} runs the guard`);
    assert.equal(h.command, 'node', `${ev} runs it with node`);
    assert.equal(h.onFailure, 'block', `${ev} fails closed`);
    assert.ok(h.args.some((a) => a.includes('${CLAUDE_PROJECT_DIR}')), `${ev} uses the project-dir path`);
  }
});
test('the syntax-gate hook is registered on PostToolUse', () => {
  const h = (settings.hooks?.PostToolUse ?? []).flatMap((g) => g.hooks ?? []).find((x) => (x.args ?? []).some((a) => String(a).includes('syntax-gate.mjs')));
  assert.ok(h && h.command === 'node', 'PostToolUse runs the syntax gate with node');
});

for (const cmd of NEEDED) {
  test(`needed, not denied: ${cmd}`, () => assert.equal(deniedBy(cmd), false, `a deny rule catches a needed command: ${cmd}`));
  test(`needed, matches an allow: ${cmd}`, () => assert.equal(allowedBy(cmd), true, `no allow rule matches a needed command: ${cmd}`));
}
for (const cmd of NEVER) {
  test(`never, denied by a static rule: ${cmd}`, () => assert.equal(deniedBy(cmd), true, `no static deny catches: ${cmd}`));
}
