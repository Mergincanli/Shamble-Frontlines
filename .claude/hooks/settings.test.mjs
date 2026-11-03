// Tests for .claude/settings.json (kit-v3-1 plan, evidence C3a): node --test .claude/hooks/settings.test.mjs
// It checks the static layer, not the guard: the ask list is empty (an ask stalls a routine), effortLevel is high,
// the guard hook is registered on the three events with onFailure "block", and — through an emulation of Claude Code's
// documented Bash-rule matching — every command the two routines need is matched by an allow and by no deny, while
// every "never" is caught by a static deny (the routine-only nevers are the guard's job, tested in routine-guard.test).
// GUARD-FILE (Mustafa, 2026-11-03): it also runs the guard's hook command the way Claude Code runs it, with guard.mjs
// removed, broken or present: a day session goes on when guard.mjs can't load, a routine is blocked.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const HERE = dirname(fileURLToPath(import.meta.url));
const settings = JSON.parse(readFileSync(join(HERE, '..', 'settings.json'), 'utf8'));
const { allow = [], deny = [], ask = [] } = settings.permissions ?? {};
const GUARD_PATH = '${CLAUDE_PROJECT_DIR}/.claude/hooks/guard.mjs';

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
    // GUARD-FILE: node -e <bootstrap> <guard path>; the bootstrap holds no placeholder and nothing a shell would act on
    assert.equal(h.args.length, 3, `${ev} runs node -e <bootstrap> <guard path>`);
    assert.equal(h.args[0], '-e', `${ev} starts the guard through the bootstrap`);
    assert.equal(h.args[2], GUARD_PATH, `${ev} passes the guard's path last`);
    assert.doesNotMatch(h.args[1], /[$"`\\&|<>^%]/, `${ev}: the bootstrap holds no placeholder or shell-special character`);
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

// ---- GUARD-FILE: the guard's hook command, run the way Claude Code runs it ----
// A session keeps the hook settings it loaded while a branch switch changes the files on disk, so guard.mjs can go
// missing under a live hook. Exec form, per the hooks reference: ${CLAUDE_PROJECT_DIR} is substituted into each args
// element as a plain string, and the command starts with no shell. Exit 0 with no output is "no decision"; any exit
// other than 0 or 2 is a hook failure, which "onFailure": "block" turns into a block (a deny on PermissionRequest).
const GUARD_EVENTS = {
  PreToolUse: { hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command: 'git status' }, cwd: '/' },
  PermissionRequest: { hook_event_name: 'PermissionRequest', tool_name: 'Bash', tool_input: { command: 'git status' } },
  ConfigChange: { hook_event_name: 'ConfigChange', source: 'project_settings' },
};
const ROUTINES = ['night-shift', 'pr-advisor', 'not-a-role']; // any SHAMBLE_ROLE at all counts as a routine here
const guardHandler = (ev) => (settings.hooks?.[ev] ?? []).flatMap((g) => g.hooks ?? [])
  .find((x) => (x.args ?? []).some((a) => String(a).includes('guard.mjs')));
function runGuardHook(ev, projectDir, input, role) {
  const args = guardHandler(ev).args.map((a) => String(a).split('${CLAUDE_PROJECT_DIR}').join(projectDir));
  const env = { ...process.env, CLAUDE_PROJECT_DIR: projectDir };
  delete env.SHAMBLE_ROLE;
  if (role) env.SHAMBLE_ROLE = role;
  return spawnSync(process.execPath, args, { input: JSON.stringify(input), env, encoding: 'utf8', timeout: 30000 });
}
// A throwaway project with a copy of the guard's two files; change(guardFile) then removes or breaks guard.mjs.
function withProject(change, fn) {
  const dir = mkdtempSync(join(tmpdir(), 'guard-file-'));
  try {
    const hooks = join(dir, '.claude', 'hooks');
    mkdirSync(hooks, { recursive: true });
    for (const f of ['guard.mjs', 'routine-guard.mjs']) copyFileSync(join(HERE, f), join(hooks, f));
    change(join(hooks, 'guard.mjs'));
    return fn(dir);
  } finally { rmSync(dir, { recursive: true, force: true }); }
}
const removed = (fn) => withProject((g) => rmSync(g), fn);

test('the three guard handlers are identical (a fix cannot reach one event and miss another)', () => {
  const [pre, perm, cfg] = ['PreToolUse', 'PermissionRequest', 'ConfigChange'].map(guardHandler);
  assert.equal(JSON.stringify(perm), JSON.stringify(pre));
  assert.equal(JSON.stringify(cfg), JSON.stringify(pre));
});
for (const ev of Object.keys(GUARD_EVENTS)) {
  test(`guard.mjs removed, ${ev}: a day session goes on (exit 0, no decision)`, () => removed((dir) => {
    const r = runGuardHook(ev, dir, GUARD_EVENTS[ev], '');
    assert.equal(r.status, 0, r.stderr);
    assert.equal(r.stdout, '');
  }));
  for (const role of ROUTINES) {
    test(`guard.mjs removed, ${ev}: a routine (${role}) is blocked (exit 1, a hook failure)`, () => removed((dir) => {
      const r = runGuardHook(ev, dir, GUARD_EVENTS[ev], role);
      assert.equal(r.status, 1, r.stderr);
      assert.equal(r.stdout, '');
    }));
  }
}
test('guard.mjs removed: the routine\'s refusal says why (stderr)', () => removed((dir) => {
  assert.match(runGuardHook('PreToolUse', dir, GUARD_EVENTS.PreToolUse, 'night-shift').stderr, /could not load.*fails closed/);
}));
test('guard.mjs with a syntax error: a day session goes on, a routine is blocked', () =>
  withProject((g) => writeFileSync(g, 'export const x = ;\n'), (dir) => {
    const day = runGuardHook('PreToolUse', dir, GUARD_EVENTS.PreToolUse, '');
    assert.equal(day.status, 0, day.stderr);
    assert.equal(day.stdout, '');
    assert.equal(runGuardHook('PreToolUse', dir, GUARD_EVENTS.PreToolUse, 'night-shift').status, 1);
  }));
test('guard.mjs present: the real guard runs through the bootstrap', () => {
  const root = join(HERE, '..', '..');
  const bash = (command) => ({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd: root });
  const decision = (r) => JSON.parse(r.stdout).hookSpecificOutput.permissionDecision;
  const checkout = runGuardHook('PreToolUse', root, bash('git checkout main'), '');
  assert.equal(checkout.status, 0, checkout.stderr);
  assert.equal(decision(checkout), 'deny', 'a day session is still refused git checkout');
  const curl = runGuardHook('PreToolUse', root, bash('curl https://example.com'), 'night-shift');
  assert.equal(curl.status, 0, curl.stderr);
  assert.equal(decision(curl), 'deny', 'a routine is still refused curl');
  const cfg = runGuardHook('ConfigChange', root, GUARD_EVENTS.ConfigChange, 'night-shift');
  assert.equal(cfg.status, 0, cfg.stderr);
  assert.equal(JSON.parse(cfg.stdout).decision, 'block', 'a routine still keeps its settings');
  const status = runGuardHook('PreToolUse', root, bash('git status'), '');
  assert.equal(status.status, 0, status.stderr);
  assert.equal(status.stdout, '', 'git status in a day session: no decision');
});
