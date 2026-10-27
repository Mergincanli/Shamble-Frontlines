// Tests for the routine guard (kit-v3-1): node --test .claude/hooks/routine-guard.test.mjs
// The guard is small on purpose (Advisor's WRITE-GUARD scope reset): in a routine the shell runs only git, gh, `npm ci`,
// node/python on main's tracked scripts, and date; reading and searching go through the Read/Grep/Glob tools. So these
// tests are about what the guard ALLOWS, with a representative set of denials — not a catalogue of attacks.
// 1. The case table: role x tool x input -> refused or not, through decide() with a fake io.
// 2. Real git on a throwaway repository: abbreviated options really do what the full ones do, and the guard refuses them.
// 3. The launcher (guard.mjs) in its error modes: broken rules, unreadable input; routines refuse, day sessions go on.
// 4. Mutation runs: each rule, switched off in a copy, makes at least one case of the table fail.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { copyFileSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { basename, dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { decide, REPO } from './routine-guard.mjs';

const HERE = dirname(fileURLToPath(import.meta.url));
const P = '/repo';
const SCRATCH = '/tmp/scratch';
const SHA = 'a'.repeat(40);
const OPTS = new Map();
const gitLongOptions = (sub) => {
  if (!OPTS.has(sub)) OPTS.set(sub, (spawnSync('git', [sub, '--git-completion-helper'], { encoding: 'utf8' }).stdout || '').split(/\s+/).filter((o) => o.startsWith('--')));
  return OPTS.get(sub);
};

// A fake io. Options change what git and gh answer.
function fakeIo(o = {}) {
  const {
    role = '', branch = 'cs/x', branchDiff = 'docs/a.md\n', staged = 'docs/a.md\n', unstaged = '',
    tracked = ['tools/gate.mjs', '.claude/scripts/checkpoint.mjs', 'tools/village.py', 'tools/preview.js', 'docs/a.md', 'changed.mjs'],
    changed = ['changed.mjs'], manifestSame = true, npmrc = [],
    headSha = SHA, prHeadSha = headSha, fork = false, checks = [['gate', 'completed', 'success'], ['smoke', 'completed', 'success']],
    prFiles = 'docs/a.md\n', ghFail = false,
  } = o;
  // prHeadSha is what the PR object reports as its head; headSha is the SHA the check-runs are keyed by (= the expected
  // head in the green case). They differ only in the "head moved" case, to isolate the head-equality check.
  const pr = (ref, n) => JSON.stringify({ number: n, head: { sha: prHeadSha, ref, repo: { full_name: fork ? 'someone/fork' : REPO } }, base: { repo: { full_name: REPO } } });
  const ghAnswers = {
    [`repos/${REPO}/pulls/27`]: pr('cs/x', 27),
    [`repos/${REPO}/pulls/5`]: pr('cs/other', 5),
    [`repos/${REPO}/issues/comments/111`]: JSON.stringify({ issue_url: `https://api.github.com/repos/${REPO}/issues/16` }),
    [`repos/${REPO}/issues/comments/222`]: JSON.stringify({ issue_url: `https://api.github.com/repos/${REPO}/issues/27` }),
    [`repos/${REPO}/commits/${headSha}/check-runs?per_page=100`]: JSON.stringify({ check_runs: checks.map(([name, status, conclusion]) => ({ name, status, conclusion })) }),
    [`repos/${REPO}/pulls/27/files?per_page=100`]: prFiles,
  };
  return {
    env: role ? { SHAMBLE_ROLE: role } : {},
    projectDir: P,
    home: '/home/u',
    gitLongOptions,
    git: (args) => {
      const a = args.join(' ');
      if (a === 'branch --show-current') return { code: 0, out: `${branch}\n` };
      if (a === 'rev-parse --show-toplevel') return { code: 0, out: `${P}\n` };
      if (a === 'diff --name-only origin/main...HEAD') return { code: 0, out: branchDiff };
      if (a === 'diff --cached --name-only') return { code: 0, out: staged };
      if (a === 'diff --name-only') return { code: 0, out: unstaged };
      if (args[0] === 'ls-files') return { code: tracked.includes(args.at(-1)) ? 0 : 1, out: '' };
      if (args[0] === 'diff' && args[1] === '--quiet') {
        const fs = args.slice(4);
        if (fs[0] === 'tools/package.json') return { code: manifestSame ? 0 : 1, out: '' };
        return { code: fs.some((f) => changed.includes(f)) ? 1 : 0, out: '' };
      }
      return { code: 0, out: '' };
    },
    gh: (args) => {
      if (ghFail) return { code: 1, out: '' }; // every gh api fails -> ghJson throws inside mergeCheck
      const path = args.find((x) => x.startsWith('repos/'));
      return path in ghAnswers ? { code: 0, out: ghAnswers[path] } : { code: 1, out: '' };
    },
    exists: (p) => (basename(p) === '.npmrc' ? npmrc.includes(p) : true),
    realpath: (p) => p,
    readFile: () => '{}',
  };
}
const ev = (tool_name, tool_input, extra = {}) => ({ hook_event_name: 'PreToolUse', tool_name, tool_input, cwd: P, scratchpad_dir: SCRATCH, ...extra });
const sh = (command) => ev('Bash', { command });
const merge = (extra = {}) => ev('mcp__github__merge_pull_request', { owner: 'Mergincanli', repo: 'Shamble-Frontlines', pullNumber: 27, expectedHeadSha: SHA, ...extra });
const gw = (name, input) => ev(`mcp__github__${name}`, { owner: 'Mergincanli', repo: 'Shamble-Frontlines', ...input });

const DAY = '', NIGHT = 'night-shift', PRA = 'pr-advisor';
// [name, role, input, refused?, fake io options]
export const CASES = [
  // --- every session: git/gh option forms and the merge check (day sessions bound too) ---
  ['git --version', DAY, sh('git --version'), false],
  ['git -C elsewhere', DAY, sh('git -C /tmp status'), true],
  ['git -c option', DAY, sh('git -c core.pager=x log'), true],
  ['push a cs branch', DAY, sh('git push -u origin cs/x'), false],
  ['push -f', DAY, sh('git push -f origin cs/x'), true],
  ['push --force-with-lease', DAY, sh('git push --force-with-lease origin cs/x'), true],
  ['push --del (abbreviated)', DAY, sh('git push origin cs/x --del'), true],
  ['push +refspec', DAY, sh('git push origin +cs/x'), true],
  ['push :delete', DAY, sh('git push origin :cs/x'), true],
  ['push main', DAY, sh('git push origin main'), true],
  ['push HEAD:main', DAY, sh('git push origin HEAD:main'), true],
  ['bare push', DAY, sh('git push'), true],
  ['branch -D', DAY, sh('git branch -D cs/x'), true],
  ['branch --del (abbreviated)', DAY, sh('git branch --del cs/x'), true],
  ['commit --amend', DAY, sh('git commit --amend --no-edit'), true],
  ['commit --am (abbreviated)', DAY, sh('git commit --am'), true],
  ['reset --hard', DAY, sh('git reset --hard'), true],
  ['reset -q HEAD', DAY, sh('git reset -q HEAD -- .checkpoints'), false],
  ['reset another commit', DAY, sh('git reset HEAD~1'), true],
  ['brace group hides force push', DAY, sh('{ git push -f origin cs/x; }'), true],
  ['checkout', DAY, sh('git checkout main'), true],
  ['rebase', DAY, sh('git rebase main'), true],
  ['merge in a day session', DAY, sh('git merge origin/main'), false],
  ['config --list', DAY, sh('git config --list'), true],
  ['timeout wrapper hides force push', DAY, sh('timeout 30 git push -f origin cs/x'), true],
  ['env wrapper hides force push', DAY, sh('env GIT_TRACE=1 git push --force origin cs/x'), true],
  ['subshell hides force push', DAY, sh('(git push -f origin cs/x)'), true],
  ['absolute git path', DAY, sh('/usr/bin/git push -f origin cs/x'), true],
  ['gh api read', DAY, sh(`gh api repos/${REPO}/pulls/1`), false],
  ['gh api -X POST', DAY, sh(`gh api -X POST repos/${REPO}/issues`), true],
  ['gh pr merge', DAY, sh('gh pr merge 27'), true],
  ['gh pr view', DAY, sh('gh pr view 27'), false],
  ['merge, green', DAY, merge(), false],
  ['merge without a SHA', DAY, merge({ expectedHeadSha: undefined }), true],
  ['merge with a short SHA that is the head', DAY, merge({ expectedHeadSha: 'abc1234' }), true, { headSha: 'abc1234', prHeadSha: 'abc1234' }],
  ['merge, head moved', DAY, merge(), true, { prHeadSha: 'b'.repeat(40) }],
  ['merge, fork head', DAY, merge(), true, { fork: true }],
  ['merge, no smoke', DAY, merge(), true, { checks: [['gate', 'completed', 'success']] }],
  ['merge, gate failed', DAY, merge(), true, { checks: [['gate', 'completed', 'failure'], ['smoke', 'completed', 'success']] }],
  ['merge, the check errors -> fails closed', DAY, merge(), true, { ghFail: true }],
  ['auto-merge', DAY, gw('enable_pr_auto_merge', { pullNumber: 27 }), true],
  // day sessions keep their freedom (no role: only git and gh are checked)
  ['day: edit CLAUDE.md', DAY, ev('Edit', { file_path: `${P}/CLAUDE.md` }), false],
  ['day: a file tool', DAY, sh('cat docs/a.md'), false],
  ['day: curl', DAY, sh('curl https://x'), false],
  ['day: node -e', DAY, sh('node -e "1"'), false],
  ['day: WebFetch', DAY, ev('WebFetch', { url: 'https://x' }), false],
  ['day: AskUserQuestion', DAY, ev('AskUserQuestion', {}), false],
  ['day: PermissionRequest', DAY, { hook_event_name: 'PermissionRequest', tool_name: 'Bash', tool_input: {} }, false],
  ['day: ConfigChange', DAY, { hook_event_name: 'ConfigChange', source: 'project_settings' }, false],

  // --- night shift: the fixed shell set is ALLOWED, reading goes through the tools ---
  ['night: push its branch', NIGHT, sh('git push -u origin cs/x'), false],
  ['night: fetch origin', NIGHT, sh('git fetch -q origin main'), false],
  ['night: switch -c', NIGHT, sh('git switch -c cs/x origin/main'), false],
  ['night: add explicit paths', NIGHT, sh('git add docs/a.md docs/b.md'), false],
  ['night: commit', NIGHT, sh('git commit -m "a message"'), false],
  ['night: worktree add', NIGHT, sh('git worktree add .gate/base origin/main'), false],
  ['night: worktree remove', NIGHT, sh('git worktree remove .gate/base'), false],
  ['night: git rm the checkpoint', NIGHT, sh('git rm -r .checkpoints/x.md .checkpoints/x'), false],
  ['night: reset -q HEAD', NIGHT, sh('git reset -q HEAD -- .checkpoints'), false],
  ['night: ls-remote the handoff', NIGHT, sh('git ls-remote --heads origin cs/x'), false],
  ['night: git config read', NIGHT, sh('git config user.email'), false],
  ['night: the gate (main copy)', NIGHT, sh('node tools/gate.mjs origin/main'), false],
  ['night: checkpoint save', NIGHT, sh('node .claude/scripts/checkpoint.mjs save s -m x'), false],
  ['night: village.py (main copy)', NIGHT, sh('python3 tools/village.py'), false],
  ['night: npm ci, exact', NIGHT, sh('npm ci --prefix tools --ignore-scripts --no-audit --no-fund'), false],
  ['night: the clock', NIGHT, sh('date'), false],
  ['night: TZ clock', NIGHT, sh('TZ=Europe/Amsterdam date'), false],
  ['night: the clock with a format', NIGHT, sh('date -u +%H:%M'), false],
  ['night: read a project file', NIGHT, ev('Read', { file_path: `${P}/docs/a.md` }), false],
  ['night: grep the tree', NIGHT, ev('Grep', { pattern: 'foo', path: `${P}/docs` }), false],
  ['night: glob the tree', NIGHT, ev('Glob', { pattern: '**/*.md' }), false],
  ['night: edit a doc', NIGHT, ev('Edit', { file_path: `${P}/docs/a.md` }), false],
  ['night: write to the scratchpad', NIGHT, ev('Write', { file_path: `${SCRATCH}/n.md` }), false],
  ['night: agent memory (local)', NIGHT, ev('Write', { file_path: `${P}/.claude/agent-memory-local/a/m.md` }), false],
  ['night: a redirect into docs', NIGHT, sh('node tools/gate.mjs > docs/out.txt'), false],

  // --- night shift: everything else is DENIED (the fixed set) ---
  ['night: cat is off the shell', NIGHT, sh('cat docs/a.md'), true],
  ['night: ls is off the shell', NIGHT, sh('ls docs'), true],
  ['night: find is off the shell', NIGHT, sh('find . -name x'), true],
  ['night: grep is off the shell', NIGHT, sh('grep foo docs'), true],
  ['night: sort is off the shell', NIGHT, sh('sort x'), true],
  ['night: cp is off the shell', NIGHT, sh('cp a b'), true],
  ['night: mv is off the shell', NIGHT, sh('mv a b'), true],
  ['night: rm is off the shell', NIGHT, sh('rm x'), true],
  ['night: sed is off the shell', NIGHT, sh('sed -i s/a/b/ docs/a.md'), true],
  ['night: mkdir is off the shell', NIGHT, sh('mkdir .checkpoints/x'), true],
  ['night: tee is off the shell', NIGHT, sh('node tools/gate.mjs | tee out.txt'), true],
  ['night: a program by a path', NIGHT, sh('/usr/bin/node tools/gate.mjs'), true],
  ['night: date -s sets the clock', NIGHT, sh('date -s "2020-01-01"'), true],
  ['night: date -f reads a file', NIGHT, sh('date -f dates.txt'), true],
  ['night: curl', NIGHT, sh('curl https://x'), true],
  ['night: wget', NIGHT, sh('wget https://x'), true],
  ['night: npx', NIGHT, sh('npx --no -- playwright install chromium'), true],
  ['night: pip install', NIGHT, sh('pip install x'), true],
  ['night: cargo', NIGHT, sh('cargo build'), true],
  ['night: sudo', NIGHT, sh('sudo ls'), true],
  ['night: a shell', NIGHT, sh('bash -c "git status"'), true],
  ['night: eval', NIGHT, sh('eval "git status"'), true],
  ['night: command wrapper hides a command', NIGHT, sh('command git status'), true],
  ['night: exec wrapper', NIGHT, sh('exec node x'), true],
  ['night: gh browse', NIGHT, sh('gh browse'), true],
  ['night: npm, other form', NIGHT, sh('npm ci --prefix tools'), true],
  ['night: npm install', NIGHT, sh('npm install left-pad'), true],
  ['night: npm ci, .npmrc present', NIGHT, sh('npm ci --prefix tools --ignore-scripts --no-audit --no-fund'), true, { npmrc: ['/home/u/.npmrc'] }],
  ['night: node -e', NIGHT, sh('node -e "1"'), true],
  ['night: node a .gate harness', NIGHT, sh('node .gate/h.mjs'), true],
  ['night: node an untracked script', NIGHT, sh('node new.mjs'), true],
  ['night: node a changed script', NIGHT, sh('node changed.mjs'), true],
  ['night: node preview.js', NIGHT, sh('node tools/preview.js . eval tools/sim.js'), true],
  ['night: a bad env prefix', NIGHT, sh('PATH=.gate node tools/gate.mjs'), true],
  ['night: env with a bad var', NIGHT, sh('env NODE_OPTIONS=--require=./x node tools/gate.mjs'), true],
  ['night: bare env', NIGHT, sh('env'), true],
  ['night: printenv', NIGHT, sh('printenv'), true],
  ['night: a substitution in a node arg', NIGHT, sh('node tools/gate.mjs $(date)'), true],
  ['night: an unquoted glob (git add *)', NIGHT, sh('git add *'), true],
  ['night: an unterminated here-document', NIGHT, sh("node tools/gate.mjs <<'EOF'\nx"), true],
  ['night: push main', NIGHT, sh('git push origin main'), true],
  ['night: push force', NIGHT, sh('git push -f origin cs/x'), true],
  ['night: push another branch', NIGHT, sh('git push origin cs/other'), true],
  ['night: push, rules file in the branch', NIGHT, sh('git push origin cs/x'), true, { branchDiff: 'CLAUDE.md\n' }],
  ['night: checkout', NIGHT, sh('git checkout main'), true],
  ['night: rebase', NIGHT, sh('git rebase main'), true],
  ['night: merge', NIGHT, sh('git merge origin/main'), true],
  ['night: commit a staged rules file', NIGHT, sh('git commit -m x'), true, { staged: 'docs/a.md\n.claude/agents/x.md\n' }],
  ['night: fetch a PR ref', NIGHT, sh('git fetch origin refs/pull/1/head'), true],
  ['night: git config write', NIGHT, sh('git config user.name x'), true],
  ['night: git config, a key not on the list', NIGHT, sh('git config core.editor'), true],
  ['night: read a credential', NIGHT, ev('Read', { file_path: `${P}/.env` }), true],
  ['night: read .git/config', NIGHT, ev('Read', { file_path: `${P}/.git/config` }), true],
  ['night: read ~/.ssh', NIGHT, ev('Read', { file_path: '/home/u/.ssh/id_rsa' }), true],
  ['night: grep a credential dir', NIGHT, ev('Grep', { pattern: 'x', path: `${P}/.git` }), true],
  ['night: glob a credential dir', NIGHT, ev('Glob', { pattern: '**', path: '/home/u/.ssh' }), true],
  ['night: a < redirect from a credential', NIGHT, sh('node tools/gate.mjs < .git/config'), true],
  ['night: edit CLAUDE.md', NIGHT, ev('Edit', { file_path: `${P}/CLAUDE.md` }), true],
  ['night: edit .claude', NIGHT, ev('Edit', { file_path: `${P}/.claude/settings.json` }), true],
  ['night: edit .github', NIGHT, ev('Write', { file_path: `${P}/.github/workflows/gate.yml` }), true],
  ['night: edit a gate file', NIGHT, ev('Edit', { file_path: `${P}/tools/gate.mjs` }), true],
  ['night: write .mcp.json', NIGHT, ev('Write', { file_path: `${P}/.mcp.json` }), true],
  ['night: a redirect into .claude', NIGHT, sh('node tools/gate.mjs > .claude/x'), true],
  ['night: a redirect outside the project', NIGHT, sh('node tools/gate.mjs > /etc/x'), true],
  ['night: WebFetch', NIGHT, ev('WebFetch', { url: 'https://doc.rust-lang.org/book/' }), true],
  ['night: WebSearch', NIGHT, ev('WebSearch', { query: 'rust' }), true],
  ['night: AskUserQuestion', NIGHT, ev('AskUserQuestion', {}), true],
  ['night: plan mode', NIGHT, ev('EnterPlanMode', {}), true],
  ['night: PermissionRequest', NIGHT, { hook_event_name: 'PermissionRequest', tool_name: 'Bash', tool_input: {} }, true],
  ['night: ConfigChange project', NIGHT, { hook_event_name: 'ConfigChange', source: 'project_settings' }, true],
  ['night: ConfigChange policy (cannot be blocked)', NIGHT, { hook_event_name: 'ConfigChange', source: 'policy_settings' }, false],
  // night GitHub MCP and connectors
  ['night: the merge tool', NIGHT, merge(), true],
  ['night: comment on #16', NIGHT, gw('add_issue_comment', { issue_number: 16, body: 'x' }), false],
  ['night: comment on its own PR', NIGHT, gw('add_issue_comment', { issue_number: 27, body: 'x' }), false],
  ['night: comment on another PR', NIGHT, gw('add_issue_comment', { issue_number: 5, body: 'x' }), true],
  ['night: update #16', NIGHT, gw('issue_write', { method: 'update', issue_number: 16, body: 'x' }), false],
  ['night: open a PR from its branch', NIGHT, gw('create_pull_request', { head: 'cs/x', base: 'main', title: 'x' }), false],
  ['night: change a PR base', NIGHT, gw('update_pull_request', { pullNumber: 27, base: 'x' }), true],
  ['night: push files by API', NIGHT, gw('push_files', { branch: 'main' }), true],
  ['night: read a PR', NIGHT, gw('pull_request_read', { method: 'get', pullNumber: 27 }), false],
  ['night: write to another repo', NIGHT, ev('mcp__github__add_issue_comment', { owner: 'x', repo: 'y', issue_number: 1, body: 'x' }), true],
  ['night: create a routine', NIGHT, ev('mcp__claude-code-remote__create_trigger', {}), true],
  ['night: list routines', NIGHT, ev('mcp__claude-code-remote__list_triggers', {}), false],
  ['night: a docs connector', NIGHT, ev('mcp__Claude_Docs__create', {}), true],

  // --- PR Advisor: read-only, no program, notes only ---
  ['advisor: git status', PRA, sh('git status'), false],
  ['advisor: fetch origin main', PRA, sh('git fetch -q origin main'), false],
  ['advisor: switch to main', PRA, sh('git switch --detach origin/main'), false],
  ['advisor: add', PRA, sh('git add x'), true],
  ['advisor: commit', PRA, sh('git commit -m x'), true],
  ['advisor: push', PRA, sh('git push origin cs/x'), true],
  ['advisor: node', PRA, sh('node tools/gate.mjs'), true],
  ['advisor: npm ci', PRA, sh('npm ci --prefix tools --ignore-scripts --no-audit --no-fund'), true],
  ['advisor: the clock', PRA, sh('date'), false],
  ['advisor: gh pr view', PRA, sh('gh pr view 27'), false],
  ['advisor: read a file', PRA, ev('Read', { file_path: `${P}/docs/a.md` }), false],
  ['advisor: write notes', PRA, ev('Write', { file_path: `${P}/.checkpoints/n.md` }), false],
  ['advisor: edit docs', PRA, ev('Edit', { file_path: `${P}/docs/a.md` }), true],
  ['advisor: comment on a PR', PRA, gw('add_issue_comment', { issue_number: 27, body: 'x' }), false],
  ['advisor: comment on #16', PRA, gw('add_issue_comment', { issue_number: 16, body: 'x' }), true],
  ['advisor: label a PR', PRA, gw('issue_write', { method: 'update', issue_number: 27, labels: ['ready-to-merge'] }), false],
  ['advisor: merge a green docs PR', PRA, merge(), false],
  ['advisor: merge a rules PR', PRA, merge(), true, { prFiles: 'docs/a.md\n.claude/settings.json\n' }],

  // --- an unknown role is refused outright ---
  ['unknown role', 'nightshift', sh('git status'), true],
];

for (const [name, role, input, refused, opts] of CASES) {
  test(`${refused ? 'refused' : 'goes on'}: ${name}`, async () => {
    const d = await decide(input, fakeIo({ role, ...opts }));
    assert.equal(!!d, refused, `${name}: ${JSON.stringify(d)}`);
  });
}

test('a merge whose check cannot complete fails closed, even for a day session', async () => {
  const d = await decide(merge(), fakeIo({ role: '', ghFail: true }));
  assert.ok(d && /could not complete|refused/i.test(d.deny), `expected a closed failure, got ${JSON.stringify(d)}`);
});

test('a refusal names what to use instead', async () => {
  const msg = async (command) => (await decide(sh(command), fakeIo({ role: NIGHT })))?.deny ?? '';
  assert.match(await msg('cat docs/a.md'), /Read, Grep/);
  assert.match(await msg('curl https://example.com'), /web|daytime/);
  assert.match(await msg('git -C /tmp status'), /without -C/);
  assert.match(await msg('git checkout main'), /git switch/);
});

test('PermissionRequest in a routine denies and interrupts', async () => {
  const d = await decide({ hook_event_name: 'PermissionRequest', tool_name: 'Bash', tool_input: {} }, fakeIo({ role: NIGHT }));
  assert.equal(d.interrupt, true);
});

// --- 2. real git on a throwaway repository: the abbreviations work, and the guard refuses them ---
test('real git accepts the abbreviations the guard refuses (throwaway repo, no network)', async () => {
  const dir = mkdtempSync(join(tmpdir(), 'guard-git-'));
  const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
  const g = (...a) => spawnSync('git', a, { cwd: dir, env, encoding: 'utf8' });
  try {
    g('init', '-q', '-b', 'main');
    g('commit', '-q', '--allow-empty', '-m', 'one');
    g('branch', 'cs/a');
    assert.equal(g('branch', '--del', 'cs/a').status, 0);
    assert.equal(g('branch', '--list', 'cs/a').stdout.trim(), '', 'git branch --del deleted the branch');
    const before = g('rev-parse', 'HEAD').stdout.trim();
    assert.equal(g('commit', '-q', '--am', '--allow-empty', '-m', 'two').status, 0);
    assert.notEqual(g('rev-parse', 'HEAD').stdout.trim(), before, 'git commit --am amended');
    for (const c of ['git branch --del cs/a', 'git commit --am --allow-empty -m two', 'git reset --ha']) {
      assert.ok(await decide(sh(c), fakeIo()), `the guard refuses: ${c}`);
    }
  } finally { rmSync(dir, { recursive: true, force: true }); }
});

// --- 3. the launcher's error modes ---
function launcher(rules, stdin, role) {
  const dir = mkdtempSync(join(tmpdir(), 'guard-launch-'));
  try {
    copyFileSync(join(HERE, 'guard.mjs'), join(dir, 'guard.mjs'));
    writeFileSync(join(dir, 'routine-guard.mjs'), rules);
    const env = { ...process.env, CLAUDE_PROJECT_DIR: dir };
    if (role) env.SHAMBLE_ROLE = role; else delete env.SHAMBLE_ROLE;
    return spawnSync(process.execPath, [join(dir, 'guard.mjs')], { input: stdin, env, encoding: 'utf8' });
  } finally { rmSync(dir, { recursive: true, force: true }); }
}
const preTool = JSON.stringify({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command: 'ls' }, cwd: '/' });
const throwing = 'export const decide = () => { throw new Error("boom"); }; export const realIo = () => ({});';
const broken = 'export const decide = (;';
for (const [label, rules] of [['rules that throw', throwing], ['rules with a syntax error', broken]]) {
  test(`launcher, ${label}: a routine is refused`, () => {
    const r = launcher(rules, preTool, NIGHT);
    assert.equal(r.status, 0);
    assert.equal(JSON.parse(r.stdout).hookSpecificOutput.permissionDecision, 'deny');
  });
  test(`launcher, ${label}: a day session goes on`, () => {
    const r = launcher(rules, preTool, '');
    assert.equal(r.status, 0);
    assert.equal(r.stdout, '');
  });
}
test('launcher, unreadable input: a routine is refused, a day session goes on', () => {
  assert.equal(JSON.parse(launcher(throwing, 'not json', NIGHT).stdout).hookSpecificOutput.permissionDecision, 'deny');
  assert.equal(launcher(throwing, 'not json', '').stdout, '');
});
test('launcher, PermissionRequest in a routine: deny with interrupt', () => {
  const r = launcher(readFileSync(join(HERE, 'routine-guard.mjs'), 'utf8'), JSON.stringify({ hook_event_name: 'PermissionRequest', tool_name: 'Bash', tool_input: {} }), NIGHT);
  const d = JSON.parse(r.stdout).hookSpecificOutput.decision;
  assert.deepEqual([d.behavior, d.interrupt], ['deny', true]);
});
test('launcher, ConfigChange in a routine: block', () => {
  const r = launcher(readFileSync(join(HERE, 'routine-guard.mjs'), 'utf8'), JSON.stringify({ hook_event_name: 'ConfigChange', source: 'project_settings' }), NIGHT);
  assert.equal(JSON.parse(r.stdout).decision, 'block');
});
test('launcher, real rules: curl in a routine is refused', () => {
  const r = launcher(readFileSync(join(HERE, 'routine-guard.mjs'), 'utf8'), JSON.stringify({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command: 'curl https://example.com' }, cwd: '/' }), NIGHT);
  assert.equal(JSON.parse(r.stdout).hookSpecificOutput.permissionDecision, 'deny');
});

// --- 4. mutation runs: switch one rule off in a copy; some case of the table must notice ---
const MUTANTS = [
  ['abbreviated git long options', 'if (!exact && known.some((k) => k.startsWith(name))) return', 'if (false) return'],
  ['dangerous git long options', 'if (danger.includes(name)) return', 'if (false) return'],
  ['dangerous git short flags', 'if (shortDanger.includes(t[k])) return', 'if (false) return'],
  ['options before the git subcommand', "if (argv[1]?.startsWith('-')) return deny(", 'if (false) return deny('],
  ['git subcommands never allowed', 'if (GIT_NEVER[sub]) return deny(', 'if (false) return deny('],
  ['force, delete and main refspecs', "if (r.startsWith('+') || r.startsWith(':') || /(^|:)(refs\\/heads\\/)?(main|master|HEAD)$/.test(r) || r === 'HEAD') return deny(", 'if (false) return deny('],
  ['reset only unstages', "if (!(before.length === 0 || (before.length === 1 && before[0] === 'HEAD'))) {", 'if (false) {'],
  ['git config reads only (routines)', 'if (!GIT_CONFIG_READ.has(key.toLowerCase())) return deny(', 'if (false) return deny('],
  ['own-branch pushes (night)', 'if (refs.length !== 1 || refs[0] !== cur) return deny(', 'if (false) return deny('],
  ['no merge at night', "if (role === 'night-shift') return deny('routine (night-shift): the night shift never merges');", ''],
  ['the merge SHA', "if (!/^[0-9a-f]{40}$/.test(sha)) return deny(", 'if (false) return deny('],
  ['the merge head', 'if (pr.head?.sha !== sha) return deny(', 'if (false) return deny('],
  ['the merge checks', "if (rs.some((r) => !['success', 'skipped', 'neutral'].includes(r.conclusion)) || !rs.some((r) => r.conclusion === 'success')) return deny(", 'if (false) return deny('],
  ['merge fails closed', 'catch (e) { return deny(`merge: the check could not complete', 'catch (e) { return null && deny(`merge: the check could not complete'],
  ['protected files in a merge (PR Advisor)', 'if (hit) return deny(`routine (pr-advisor): the PR changes', 'if (false) return deny(`routine (pr-advisor): the PR changes'],
  ['protected writes (Edit/Write tool)', 'if (protectedRel(rel)) return deny(`routine (${role}): ${rel} is a rules, CI or gate file; those change', 'if (false) return deny(`routine (${role}): ${rel} is a rules, CI or gate file; those change'],
  ["the PR Advisor's notes-only writes", "if (role === 'pr-advisor' && !/^(\\.checkpoints|\\.gate|\\.claude\\/agent-memory-local)(\\/|$)/.test(rel)) {", 'if (false) {'],
  ['protected redirect targets', 'if (protectedRel(rel)) return deny(`routine (${role}): ${what} on ${rel}', 'if (false) return deny(`routine (${role}): ${what} on ${rel}'],
  ['a write outside the project', 'return deny(`routine (${role}): ${what} must write inside the project or the scratchpad', 'return null && deny(`routine (${role}): ${what} must write inside the project or the scratchpad'],
  ['the fixed shell set (catch-all)', 'return deny(`routine (${role}): the shell runs only git, gh, npm ci, node', 'return null && deny(`routine (${role}): the shell runs only git, gh, npm ci, node'],
  ['a program by a path', 'if (rawProg !== prog) return deny(`routine (${role}): run ${prog} by name', 'if (false) return deny(`routine (${role}): run ${prog} by name'],
  ['date reads the clock only', 'return deny(`routine (${role}): date reads the clock only', 'return null && deny(`routine (${role}): date reads the clock only'],
  ['tracked scripts only', "if (io.git(['ls-files', '--error-unmatch', '--', rel], root).code !== 0) return deny(", 'if (false) return deny('],
  ["main's copy only", "if (io.git(['diff', '--quiet', 'origin/main', '--', rel], root).code !== 0) return deny(", 'if (false) return deny('],
  ['.gate and preview scripts denied', "if (DENIED_SCRIPTS.has(rel) || rel.startsWith('.gate/')) return deny(", 'if (false) return deny('],
  ['no interpreter for the PR Advisor', "if (role === 'pr-advisor') return deny(\"routine (pr-advisor): the PR Advisor runs no program", "if (false) return deny(\"routine (pr-advisor): the PR Advisor runs no program"],
  ['the exact npm form', "if (role === 'night-shift' && argv.join(' ') === NIGHT_NPM) {", "if (role === 'night-shift') {"],
  ['environment set on a command', 'if (!ROLE_ENV_OK.has(name)) return { deny: `routine (${role}): only ${okList} may be set', 'if (false) return { deny: `routine (${role}): only ${okList} may be set'],
  ['re-exec wrappers', "if (b === 'command' || b === 'builtin' || b === 'exec' || b === 'time' || b === 'nohup') return { deny:", 'if (false) return { deny:'],
  ['no command substitution', 'if (role && subs) return deny(', 'if (false) return deny('],
  ['unquoted expansion (git add *)', 'if (role && c.expand) return deny(', 'if (false) return deny('],
  ['an unterminated here-document', 'if (role && unterminated) return deny(', 'if (false) return deny('],
  ['read-redirect targets are checked', 'for (const r of c.reads) {', 'for (const r of []) {'],
  ['credentials off the read tools', 'return cred ? deny(`routine (${role}): ${p} may hold credentials', 'return (false) ? deny(`routine (${role}): ${p} may hold credentials'],
  ['the web is off for routines', "if (tool === 'WebFetch' || tool === 'WebSearch') return deny(", 'if (false) return deny('],
  ['gh deny-by-default', "if ((sub === 'pr' || sub === 'issue') && /^(view|list|status|diff|checks)$/.test(sub2)) return null;\n    return deny(`routine (${role}): gh", "if (true) return null;\n    return deny(`routine (${role}): gh"],
  ['shell keywords', "'!', '{', '}', 'if', 'then', 'else',", "'!', 'else',"],
  ['parentheses separate commands', "if (c === ';' || c === '(' || c === ')') { endCmd(); i++; continue; }", "if (c === ';') { endCmd(); i++; continue; }"],
  ['PermissionRequest', "return role ? deny(`routine (${rawRole}): this would have waited", "return false ? deny(`routine (${rawRole}): this would have waited"],
  ['ConfigChange', "return role && ['project_settings', 'local_settings', 'skills', 'user_settings'].includes(input.source)", 'return false'],
];
test('mutation runs: every rule is needed by some case', async () => {
  const src = readFileSync(join(HERE, 'routine-guard.mjs'), 'utf8');
  const dir = mkdtempSync(join(tmpdir(), 'guard-mut-'));
  const survivors = [];
  // A case the real rules get wrong would "catch" every mutant, so the table must pass unmutated first.
  for (const [name, role, input, refused, opts] of CASES) {
    assert.equal(!!(await decide(input, fakeIo({ role, ...opts }))), refused, `unmutated rules: ${name}`);
  }
  try {
    for (const [i, [label, from, to]] of MUTANTS.entries()) {
      assert.equal(src.split(from).length - 1, 1, `mutant "${label}": its target text occurs exactly once`);
      const file = join(dir, `m${i}.mjs`);
      writeFileSync(file, src.replace(from, to));
      const m = await import(pathToFileURL(file).href);
      let caught = false;
      for (const [, role, input, refused, opts] of CASES) {
        let d;
        try { d = await m.decide(input, fakeIo({ role, ...opts })); } catch { d = 'threw'; }
        if (!!d !== refused) { caught = true; break; }
      }
      if (!caught) survivors.push(label);
    }
  } finally { rmSync(dir, { recursive: true, force: true }); }
  assert.deepEqual(survivors, [], `mutants no case catches: ${survivors.join('; ')}`);
});
