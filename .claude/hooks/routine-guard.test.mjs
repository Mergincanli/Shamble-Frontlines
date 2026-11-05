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
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from 'node:fs';
import { existsSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { basename, dirname, join } from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { decide, realIo, REPO } from './routine-guard.mjs';

const HERE = dirname(fileURLToPath(import.meta.url));
const P = '/repo';
const SCRATCH = '/tmp/scratch';
const SHA = 'a'.repeat(40);
const OPTS = new Map();
const gitLongOptions = (sub) => {
  if (!OPTS.has(sub)) OPTS.set(sub, (spawnSync('git', [sub, '--git-completion-helper'], { encoding: 'utf8' }).stdout || '').split(/\s+/).filter((o) => o.startsWith('--')));
  return OPTS.get(sub);
};

// fix pass 2 (SD-2): the guard lists changed files with these flags; the fakes take newline lists and answer NUL lists
// (fix pass 3, P3-6: --ignore-submodules=none added, a named scaffold edit)
const NAMES_Z = '--name-only -z --no-renames --no-relative --ignore-submodules=none';
const nul = (s) => s.split('\n').filter(Boolean).map((f) => `${f}\0`).join('');
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
      if (a === `diff ${NAMES_Z} origin/main...HEAD`) return { code: 0, out: nul(branchDiff) }; // fix pass 2 (SD-2) keys
      // kit-v3-3 step 4b (S-2): origin/main resolves to a commit, as real git answers it
      if (/^rev-parse --verify --end-of-options (refs\/remotes\/)?origin\/main\^\{commit\}$/.test(a)) return { code: 0, out: `${SHA}\n` };
      if (a === `diff --cached ${NAMES_Z}`) return { code: 0, out: nul(staged) };
      if (a === `diff ${NAMES_Z}`) return { code: 0, out: nul(unstaged) };
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
  // day sessions keep their freedom outside kit and gate files (kit-v3-3 item 1: those ask; the shapes are DAY_CASES)
  ['day: edit CLAUDE.md asks (kit-v3-3 item 1)', DAY, ev('Edit', { file_path: `${P}/CLAUDE.md` }), true],
  ['day: edit a game file', DAY, ev('Edit', { file_path: `${P}/shamble-game.js` }), false],
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
  ['night: a redirect with a leading tilde', NIGHT, sh('node tools/gate.mjs > ~/x'), true],
  ['night: a redirect with a brace', NIGHT, sh('node tools/gate.mjs > {a,b}'), true],
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

  // --- rev-list and --output (kit-v3-3): rev-list is a read; --output=<file> writes a file, refused in every session ---
  ['advisor: rev-list --count', PRA, sh('git rev-list --count origin/main..HEAD'), false],
  ['night: rev-list --count', NIGHT, sh('git rev-list --count origin/main..HEAD'), false],
  ['advisor: rev-list --output', PRA, sh('git rev-list --output=x HEAD'), true],
  ['advisor: rev-list --outp (abbreviated)', PRA, sh('git rev-list --outp=x HEAD'), true],
  ['advisor: log --output', PRA, sh('git log --output=x -1'), true],
  ['advisor: diff --output', PRA, sh('git diff --output=x'), true],
  ['advisor: show --output', PRA, sh('git show --output=x'), true],
  ['night: rev-list --output', NIGHT, sh('git rev-list --output=x HEAD'), true],
  ['night: log --outp (abbreviated)', NIGHT, sh('git log --outp=x -1'), true],
  ['day: rev-list --output', DAY, sh('git rev-list --output=x HEAD'), true],
  ['day: log --output', DAY, sh('git log --output=x -1'), true],
  ['day: log --outp (abbreviated)', DAY, sh('git log --outp=x -1'), true],
  ['day: diff --output', DAY, sh('git diff --output=x'), true],
  ['day: show --output', DAY, sh('git show --output=x'), true],
  ['day: show --outp (abbreviated)', DAY, sh('git show --outp=x'), true],
  ['day: diff --output-indicator-new is not --output', DAY, sh('git diff --output-indicator-new=+'), false],

  // --- Actions reads (kit-v3-3, item 3, C7): the PR Advisor reads CI on this repository; the listed methods only ---
  ['advisor: actions_get get_workflow', PRA, gw('actions_get', { method: 'get_workflow', resource_id: 'gate.yml' }), false],
  ['advisor: actions_get get_workflow_run', PRA, gw('actions_get', { method: 'get_workflow_run', resource_id: '1' }), false],
  ['advisor: actions_get get_workflow_job', PRA, gw('actions_get', { method: 'get_workflow_job', resource_id: '1' }), false],
  ['advisor: actions_get get_workflow_run_usage', PRA,
    gw('actions_get', { method: 'get_workflow_run_usage', resource_id: '1' }), false],
  ['advisor: actions_list list_workflows', PRA, gw('actions_list', { method: 'list_workflows' }), false],
  ['advisor: actions_list list_workflow_runs', PRA, gw('actions_list', { method: 'list_workflow_runs' }), false],
  ['advisor: actions_list list_workflow_jobs', PRA,
    gw('actions_list', { method: 'list_workflow_jobs', resource_id: '1' }), false],
  ['advisor: actions_list list_workflow_run_artifacts', PRA,
    gw('actions_list', { method: 'list_workflow_run_artifacts', resource_id: '1' }), false],
  ['advisor: actions_get download_workflow_run_artifact', PRA,
    gw('actions_get', { method: 'download_workflow_run_artifact', resource_id: '1' }), true],
  ['advisor: actions_get get_workflow_run_logs_url', PRA,
    gw('actions_get', { method: 'get_workflow_run_logs_url', resource_id: '1' }), true],
  ['advisor: actions_get unknown method', PRA, gw('actions_get', { method: 'x', resource_id: '1' }), true],
  ['advisor: actions_list unknown method', PRA, gw('actions_list', { method: 'x' }), true],
  ['advisor: actions_get no method', PRA, gw('actions_get', { resource_id: '1' }), true],
  ['advisor: actions_list no method', PRA, gw('actions_list', {}), true],
  ['advisor: actions_get a list method (wrong tool)', PRA, gw('actions_get', { method: 'list_workflows' }), true],
  ['advisor: actions_get another repo', PRA,
    ev('mcp__github__actions_get', { owner: 'x', repo: 'y', method: 'get_workflow_run', resource_id: '1' }), true],
  ['advisor: actions_list another repo', PRA,
    ev('mcp__github__actions_list', { owner: 'x', repo: 'y', method: 'list_workflow_runs' }), true],
  ['advisor: actions_get another repo of the same owner', PRA,
    gw('actions_get', { repo: 'other', method: 'get_workflow_run', resource_id: '1' }), true],
  ['advisor: actions_get owner holding the slash', PRA,
    gw('actions_get', { owner: 'Mergincanli/Shamble-Frontlines', repo: '', method: 'get_workflow_run', resource_id: '1' }), true],
  ['advisor: actions_get no owner or repo', PRA,
    ev('mcp__github__actions_get', { method: 'get_workflow_run', resource_id: '1' }), true],
  ['advisor: actions_get owner in another case', PRA,
    gw('actions_get', { owner: 'mergincanli', method: 'get_workflow_run', resource_id: '1' }), false],
  ['advisor: actions_run_trigger run_workflow', PRA,
    gw('actions_run_trigger', { method: 'run_workflow', workflow_id: 'gate.yml', ref: 'main' }), true],
  ['advisor: actions_run_trigger rerun_workflow_run', PRA, gw('actions_run_trigger', { method: 'rerun_workflow_run', run_id: 1 }), true],
  ['advisor: actions_run_trigger rerun_failed_jobs', PRA, gw('actions_run_trigger', { method: 'rerun_failed_jobs', run_id: 1 }), true],
  ['advisor: actions_run_trigger cancel_workflow_run', PRA,
    gw('actions_run_trigger', { method: 'cancel_workflow_run', run_id: 1 }), true],
  ['advisor: actions_run_trigger delete_workflow_run_logs', PRA,
    gw('actions_run_trigger', { method: 'delete_workflow_run_logs', run_id: 1 }), true],
  ['night: actions_get get_workflow_run', NIGHT, gw('actions_get', { method: 'get_workflow_run', resource_id: '1' }), true],
  ['night: actions_list list_workflow_runs', NIGHT, gw('actions_list', { method: 'list_workflow_runs' }), true],
  ['night: actions_run_trigger run_workflow', NIGHT,
    gw('actions_run_trigger', { method: 'run_workflow', workflow_id: 'gate.yml', ref: 'main' }), true],
  ['day: actions_run_trigger run_workflow (unchanged: day sessions ask)', DAY,
    gw('actions_run_trigger', { method: 'run_workflow', workflow_id: 'gate.yml', ref: 'main' }), false],
  ['day: actions_get download_workflow_run_artifact (unchanged)', DAY,
    gw('actions_get', { method: 'download_workflow_run_artifact', resource_id: '1' }), false],
  ['day: actions_get another repo (unchanged)', DAY,
    ev('mcp__github__actions_get', { owner: 'x', repo: 'y', method: 'get_workflow_run', resource_id: '1' }), false],

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

test('a refusal names python beside node (kit-v3-3: routines run python on main\'s tracked scripts)', async () => {
  const msg = async (command) => (await decide(sh(command), fakeIo({ role: NIGHT })))?.deny ?? '';
  assert.match(await msg('cat docs/a.md'), /node or python \(main's tracked scripts\)/);
  assert.match(await msg('bash -c "echo x"'), /node or python on main's tracked scripts/);
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
// --output=<file> really writes a file (rev-list, log, show, diff), and the guard refuses it in every session. The
// abbreviation --outp is refused too; git 2.43 itself rejects it (ambiguous or unrecognised), which may change, so
// git's answer to --outp is not asserted.
test('real git writes a file with --output (rev-list, log, show, diff); the guard refuses it and --outp', async () => {
  const dir = mkdtempSync(join(tmpdir(), 'guard-git-out-'));
  const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
  const g = (...a) => spawnSync('git', a, { cwd: dir, env, encoding: 'utf8' });
  try {
    g('init', '-q', '-b', 'main');
    g('commit', '-q', '--allow-empty', '-m', 'one');
    const forms = [['rev-list', 'HEAD'], ['log', '-1'], ['show'], ['diff', 'HEAD', 'HEAD']];
    for (const [i, [sub, ...rest]] of forms.entries()) {
      const full = [sub, `--output=o${i}`, ...rest];
      assert.equal(g(...full).status, 0, `git ${full.join(' ')} runs`);
      assert.ok(existsSync(join(dir, `o${i}`)), `git ${full.join(' ')} wrote a file`);
      for (const role of [DAY, NIGHT, PRA]) {
        for (const a of [full, [sub, `--outp=p${i}`, ...rest]]) {
          assert.ok(await decide(sh(`git ${a.join(' ')}`), fakeIo({ role })), `the guard refuses (${role || 'day'}): git ${a.join(' ')}`);
        }
      }
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
  // fix pass 3: 'no expansion in a redirect target' moved to PLAIN_BEHIND_MUTANTS (R-PLAIN refuses its rows first)
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
  // fix pass 3: 'no command substitution', 'unquoted expansion (git add *)' and 'an unterminated here-document' moved
  // to PLAIN_BEHIND_MUTANTS (R-PLAIN refuses their rows first)
  ['read-redirect targets are checked', 'for (const r of c.reads) {', 'for (const r of []) {'],
  ['credentials off the read tools', 'return cred ? deny(`routine (${role}): ${p} may hold credentials', 'return (false) ? deny(`routine (${role}): ${p} may hold credentials'],
  ['the web is off for routines', "if (tool === 'WebFetch' || tool === 'WebSearch') return deny(", 'if (false) return deny('],
  ['gh deny-by-default', "if ((sub === 'pr' || sub === 'issue') && /^(view|list|status|diff|checks)$/.test(sub2)) return null;\n    return deny(`routine (${role}): gh", "if (true) return null;\n    return deny(`routine (${role}): gh"],
  ['shell keywords', "'!', '{', '}', 'if', 'then', 'else',", "'!', 'else',"],
  ['parentheses separate commands', "if (c === ';' || c === '(' || c === ')') { endCmd(); i++; continue; }", "if (c === ';') { endCmd(); i++; continue; }"],
  ['PermissionRequest', "return role ? deny(`routine (${rawRole}): this would have waited", "return false ? deny(`routine (${rawRole}): this would have waited"],
  ['ConfigChange', "return role && ['project_settings', 'local_settings', 'skills', 'user_settings'].includes(input.source)", 'return false'],
  ["rev-list's --output", "  'rev-list': ['--output'],\n", ''],
  ["diff's --output", "  diff: ['--output'],\n", ''],
  ["log's --output", "  log: ['--output'],\n", ''],
  ["show's --output", "  show: ['--output'],\n", ''],
  ['the Actions reads reach their check', 'if (role && Object.hasOwn(ACTIONS_READ, name)) return actionsRead(name, ti, role);', ''],
  ['the Actions reads are for routines only', 'if (role && Object.hasOwn(ACTIONS_READ, name))', 'if (Object.hasOwn(ACTIONS_READ, name))'],
  ['the Actions owner/repo check', 'return deny(`routine (${role}): GitHub reads only on ${REPO}`);', 'return null;'],
  ['no Actions reads at night', "if (role !== 'pr-advisor') return deny(`routine (${role}): ${name} is not on the night", 'if (false) return deny(`'],
  ['the Actions method check', 'if (ok.includes(ti.method)) return null;', 'return null;'],
  ['the Actions method list (logs URL)', "'get_workflow_run_usage'],", "'get_workflow_run_usage', 'get_workflow_run_logs_url'],"],
];
test('Actions reads: a refusal names the repository or the listed methods', async () => {
  const io = fakeIo({ role: PRA });
  const other = await decide(ev('mcp__github__actions_get', { owner: 'x', repo: 'y', method: 'get_workflow_run' }), io);
  assert.match(other.deny, /GitHub reads only on Mergincanli\/Shamble-Frontlines/);
  const art = await decide(gw('actions_get', { method: 'download_workflow_run_artifact', resource_id: '1' }), io);
  assert.match(art.deny, /get_workflow, get_workflow_run, get_workflow_job, get_workflow_run_usage/);
  const none = await decide(gw('actions_list', {}), io);
  assert.match(none.deny, /list_workflows, list_workflow_runs, list_workflow_jobs, list_workflow_run_artifacts \(got none\)/);
  const night = await decide(gw('actions_get', { method: 'get_workflow_run' }), fakeIo({ role: NIGHT }));
  assert.match(night.deny, /not on the night shift's GitHub list/);
});
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

// --- 5. day sessions (kit-v3-3): kit writes ask first (item 1) and the main folder stays on main's rules (item 4) ---
// A fake io with a project /p, a worktree /w of the same repository, worktrees under /p/.gate/x and
// /p/.claude/worktrees/a1, and another repository /o. diffs answers `git diff --name-only origin/main <ref>`; a ref it
// does not name fails, as git does for a ref it can't resolve. Step 4b (S-1, S-2): a ref resolves through
// `git rev-parse --verify --end-of-options <ref>^{commit}` (a ref diffs names, HEAD, origin/main and
// refs/remotes/origin/main; shadow: true makes origin/main a local branch at another commit), and the diff then gets
// refs/remotes/origin/main and that SHA. threeDot answers `diff --name-only refs/remotes/origin/main...HEAD` (R2;
// null: git fails). A diff given --output answers as git does (it writes the file and prints nothing). calls records
// every git call.
const shaOf = (name) => createHash('sha1').update(name).digest('hex');
const WT = { '/p': '/p/.git', '/w': '/p/.git', '/p/.gate/x': '/p/.git', '/p/.claude/worktrees/a1': '/p/.git', '/o': '/o/.git' };
const CLAUDE_MD = new Set(['/p/CLAUDE.md', '/w/CLAUDE.md', '/p/.gate/x/CLAUDE.md', '/p/.claude/worktrees/a1/CLAUDE.md',
  '/p/docs/sub/CLAUDE.md']);
function dayIo(o = {}) {
  const { role = '', diffs = {}, platform, threeDot = '', shadow = false, calls, cdpath, tags = [], missing = [] } = o;
  // fix round 2 (SA-6, SA-2): lsFiles answers `git ls-files -z -- <dir>` by dir (lsFail: git fails); branch answers
  // `git branch --show-current`; npmrc false: no .npmrc anywhere; noRemoteMain: neither origin/main nor
  // refs/remotes/origin/main resolves. Unset, each answers as before.
  const { lsFiles = {}, lsFail = false, branch, npmrc = true, noRemoteMain = false } = o;
  // fix pass 2 (V-3), added: `git show-ref --verify --hash <full ref>` answers only a ref by its exact name (a branch
  // refs/heads/<name> of diffs, a full ref diffs names, refs/remotes/origin/main), and a SHA these answer peels to
  // itself; dwim maps a full ref to the name whose commit rev-parse's guessing answers for it (a tag such as
  // refs/tags/refs/heads/<name>), while show-ref still does not find it; peel maps a full ref to the name whose commit
  // an annotated tag object it points at peels to. Unset, each answers as before.
  const { dwim = {}, peel = {} } = o;
  // fix pass 2 (SD-6), added (a named scaffold edit): `git worktree list --porcelain -z` answers the worktrees of /p's
  // repository (WT's tops whose common dir is /p/.git, or worktrees when given), as git prints them; wtFail: git fails
  const { worktrees = Object.keys(WT).filter((t) => WT[t] === '/p/.git'), wtFail = false } = o;
  const tagObj = (r) => shaOf(`tag object ${r}`);
  const peeled = new Map(Object.entries(peel).map(([r, n]) => [tagObj(r), n]));
  const names = new Map(Object.keys(diffs).map((n) => [shaOf(n), n]));
  // step 4c (finding 6): a ref named in diffs is a branch refs/heads/<name>; tags names tags, each at its own commit
  // with no protected difference. A name both a branch and a tag is ambiguous: rev-parse reads the tag first, and
  // --symbolic-full-name prints nothing (exit 0), as git 2.43 does.
  const tagSha = (n) => shaOf(`tag ${n}`);
  const tagShas = new Set(tags.map(tagSha));
  const links = { '/p/link': '/p/.claude' };
  const topOf = (cwd) => Object.keys(WT).filter((t) => cwd === t || cwd.startsWith(t + '/')).sort((a, b) => b.length - a.length)[0];
  return {
    env: { ...(role ? { SHAMBLE_ROLE: role } : {}), ...(cdpath ? { CDPATH: cdpath } : {}) },
    projectDir: '/p',
    home: '/home/u',
    platform,
    gitLongOptions,
    git: (args, cwd) => {
      calls?.push(args);
      const a = args.join(' '), top = topOf(cwd ?? '/p');
      if (a === 'rev-parse --show-toplevel') return top ? { code: 0, out: `${top}\n` } : { code: 128, out: '' };
      if (a === 'rev-parse --git-common-dir') return top ? { code: 0, out: `${WT[top]}\n` } : { code: 128, out: '' };
      if (args[0] === 'ls-files' && args[1] === '-z') return lsFail ? { code: 128, out: '' } : { code: 0, out: lsFiles[args[3]] ?? '' };
      if (a === 'worktree list --porcelain -z') { // from the project folder only
        return wtFail || (cwd ?? '/p') !== '/p' ? { code: 128, out: '' } : { code: 0, out: worktrees.map((w) => `worktree ${w}\0HEAD ${SHA}\0detached\0\0`).join('') };
      }
      if (branch !== undefined && a === 'branch --show-current') return { code: 0, out: `${branch}\n` };
      if (noRemoteMain && /^rev-parse --verify --end-of-options (refs\/remotes\/)?origin\/main\^\{commit\}$/.test(a)) return { code: 128, out: '' };
      if (args[0] === 'diff' && args.some((x) => x.startsWith('--output'))) return { code: 0, out: '' };
      if (args.length === 4 && args.slice(0, 3).join(' ') === 'show-ref --verify --hash') {
        const r = args[3], n = r.startsWith('refs/heads/') ? r.slice(11) : null;
        if (noRemoteMain && r === 'refs/remotes/origin/main') return { code: 128, out: '' };
        if (r in peel) return { code: 0, out: `${tagObj(r)}\n` };
        const sha = n !== null && n in diffs && n !== 'HEAD' ? shaOf(n) : r === 'refs/remotes/origin/main' ? shaOf('origin/main')
          : r.startsWith('refs/') && r in diffs ? shaOf(r) : null;
        return sha ? { code: 0, out: `${sha}\n` } : { code: 128, out: '' };
      }
      if (args.length === 4 && args.slice(0, 3).join(' ') === 'rev-parse --verify --end-of-options' && args[3].endsWith('^{commit}')) {
        const r = args[3].slice(0, -'^{commit}'.length);
        if (r in dwim) return { code: 0, out: `${shaOf(dwim[r])}\n` };
        if (peeled.has(r)) return { code: 0, out: `${shaOf(peeled.get(r))}\n` };
        if (/^[0-9a-f]{40}$/.test(r) && (names.has(r) || r === shaOf('origin/main'))) return { code: 0, out: `${r}\n` };
      }
      if (args.length === 5 && args.slice(0, 4).join(' ') === 'rev-parse --verify --symbolic-full-name --end-of-options') {
        const n = args[4], branch = n in diffs && n !== 'HEAD';
        if (branch && tags.includes(n)) return { code: 0, out: '' };
        return branch ? { code: 0, out: `refs/heads/${n}\n` } : tags.includes(n) ? { code: 0, out: `refs/tags/${n}\n` } : { code: 128, out: '' };
      }
      if (args.length === 4 && args.slice(0, 3).join(' ') === 'rev-parse --verify --end-of-options' && args[3].endsWith('^{commit}')) {
        const r = args[3].slice(0, -'^{commit}'.length);
        if (r.startsWith('refs/heads/') && r !== 'refs/heads/HEAD') {
          return r.slice(11) in diffs ? { code: 0, out: `${shaOf(r.slice(11))}\n` } : { code: 128, out: '' };
        }
        if (tags.includes(r) || (r.startsWith('refs/tags/') && tags.includes(r.slice(10)))) return { code: 0, out: `${tagSha(r.replace(/^refs\/tags\//, ''))}\n` };
      }
      if (args.slice(0, 7).join(' ') === `diff ${NAMES_Z} refs/remotes/origin/main` && tagShas.has(args[7])) return { code: 0, out: '' };
      if (args.length === 4 && args.slice(0, 3).join(' ') === 'rev-parse --verify --end-of-options' && args[3].endsWith('^{commit}')) {
        const r = args[3].slice(0, -'^{commit}'.length);
        if (r === 'refs/remotes/origin/main') return { code: 0, out: `${shaOf('origin/main')}\n` };
        if (r === 'origin/main') return { code: 0, out: `${shaOf(shadow ? 'a local origin/main' : 'origin/main')}\n` };
        return r in diffs || r === 'HEAD' ? { code: 0, out: `${shaOf(r)}\n` } : { code: 128, out: '' };
      }
      if (a === `diff ${NAMES_Z} refs/remotes/origin/main...HEAD`) return threeDot === null ? { code: 128, out: '' } : { code: 0, out: nul(threeDot) };
      if (args.slice(0, 7).join(' ') === `diff ${NAMES_Z} refs/remotes/origin/main` && args[8] === '--' && args.length === 9) {
        const n = args[7] === 'HEAD' ? 'HEAD' : names.get(args[7]);
        return n !== undefined && n in diffs ? { code: 0, out: nul(diffs[n]) } : { code: 128, out: '' };
      }
      return { code: 0, out: '' };
    },
    gh: () => ({ code: 1, out: '' }),
    exists: (p) => (basename(p) === 'CLAUDE.md' ? CLAUDE_MD.has(p) : basename(p) === '.npmrc' ? npmrc : true),
    isDir: (p) => !missing.includes(p), // fix round R-b: a folder a cd can enter (missing: folders that are not there)
    realpath: (p) => { for (const [l, t] of Object.entries(links)) if (p === l || p.startsWith(l + '/')) return t + p.slice(l.length); return p; },
    readFile: () => '{}',
  };
}
const dsh = (command, extra = {}) => ({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd: '/p', scratchpad_dir: SCRATCH, ...extra });
// the file tools (step 4b, R1)
const ded = (tool, path, extra = {}) => ({ hook_event_name: 'PreToolUse', tool_name: tool,
  tool_input: tool === 'NotebookEdit' ? { notebook_path: path } : { file_path: path }, cwd: '/p', scratchpad_dir: SCRATCH, ...extra });
const kindOf = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : null);
const ASK = 'ask', DENY = 'deny';
const OTHER = { 'cs/other': 'tools/gate.mjs\n' };
// [name, input, expected kind, dayIo options, reason pattern]
export const DAY_CASES = [
  // group "day ask-first (item 1)", through the shell
  ['A4s docs/STATUS.md', dsh('echo x > docs/STATUS.md'), null],
  ['A5s agent memory', dsh('echo x > .claude/agent-memory-local/x'), null],
  ['A6s .checkpoints', dsh('echo x > .checkpoints/x.md'), null],
  ['A7s worktree /w .claude/settings.json', dsh('echo x > /w/.claude/settings.json'), ASK, {}, /\.claude\/settings\.json/],
  ['A8s worktree /w game file', dsh('echo x > /w/shamble-game.js'), null],
  ['A9s .claude/worktrees/a1/CLAUDE.md', dsh('echo x > /p/.claude/worktrees/a1/CLAUDE.md'), ASK],
  ['A9b-s .gate/x worktree .claude/settings.json', dsh('echo x > /p/.gate/x/.claude/settings.json'), ASK],
  ['A9c-s .gate/x worktree tools/gate.mjs', dsh('cp /tmp/x /p/.gate/x/tools/gate.mjs'), ASK],
  ['A9d-s .gate/x worktree game file', dsh('echo x > /p/.gate/x/shamble-core.js'), null],
  ['A10s a symlink to .claude', dsh('echo x > link/settings.json'), ASK],
  ["A11s another repository's .claude", dsh('echo x > /o/.claude/settings.json'), null],
  ['A12 redirect to .claude/settings.json', dsh('echo x > .claude/settings.json'), ASK, {}, /ask first.*\.claude\/settings\.json/],
  ['A13 sed -i CLAUDE.md', dsh('sed -i s/a/b/ CLAUDE.md'), ASK],
  ['A14 cp onto tools/gate.mjs', dsh('cp /tmp/x tools/gate.mjs'), ASK],
  ['A15 mv a workflow away', dsh('mv .github/workflows/gate.yml /tmp/'), ASK],
  ['A16 tee -a .claude/rules/x.md', dsh('tee -a .claude/rules/x.md'), ASK],
  ['A17 ln -s into .claude/hooks', dsh('ln -s /tmp/x .claude/hooks/x.mjs'), ASK],
  ['A18 cat CLAUDE.md', dsh('cat CLAUDE.md'), null],
  ['A19 git show into CLAUDE.md', dsh('git show origin/main:CLAUDE.md > CLAUDE.md'), ASK],
  ['A20 a redirect to $F', dsh('echo x > $F'), ASK, {}, /cannot resolve/],
  ['A21 an ask, then a force push: deny', dsh('echo x > .claude/a && git push -f origin cs/x'), DENY],
  ['A22s a subagent writes a kit file: deny', dsh('echo x > .claude/settings.json', { agent_id: 'a1' }), DENY, {}, /main session/],
  ['A22b-s agent_type without agent_id: ask, not deny', dsh('echo x > .claude/settings.json', { agent_type: 'advisor' }), ASK],
  ['A27 perl -pi -e on CLAUDE.md', dsh('perl -pi -e s/a/b/ CLAUDE.md'), ASK],
  ['A28 sed -i.bak', dsh('sed -i.bak s/a/b/ CLAUDE.md'), ASK],
  ['A28 sed --in-place=.b', dsh('sed --in-place=.b s/a/b/ CLAUDE.md'), ASK],
  ['A28 sed without -i only reads', dsh('sed -n p CLAUDE.md'), null],
  ['A29 git rm a hook', dsh('git rm .claude/hooks/guard.mjs'), ASK],
  ['A29 git mv the gate', dsh('git mv tools/gate.mjs x'), ASK],
  ['A30 rm -r tools', dsh('rm -r tools'), ASK],
  ['A30 mv .claude away', dsh('mv .claude x'), ASK],
  ['A30 cp -r into the top', dsh('cp -r /tmp/x/. .'), ASK],
  ['A30 rm -r docs/research', dsh('rm -r docs/research'), null],
  ['A30b rm -r a folder holding a CLAUDE.md', dsh('rm -r docs/sub'), ASK],
  ['A33 GIT_DIR= prefix (git switch main)', dsh('GIT_DIR=/x git switch main'), ASK],
  ['A33b GIT_DIR= prefix (git status)', dsh('GIT_DIR=/x git status'), ASK, {}, /GIT_DIR/],
  ['A35 deny beats ask inside git', dsh('git rm CLAUDE.md && git push --force origin cs/x'), DENY],
  ['A36s win32: claude.MD is CLAUDE.md', dsh('echo x > claude.MD'), ASK, { platform: 'win32' }],
  ['A36s linux: claude.MD is another file', dsh('echo x > claude.MD'), null, { platform: 'linux' }],
  ['/dev/null is no target', dsh('git status > /dev/null 2>&1'), null],
  // group "main-folder moves (item 4)"
  ['T1 git switch cs/other in the main folder', dsh('git switch cs/other'), ASK, { diffs: OTHER }, /worktree add/],
  ['T2 git switch --detach an old commit', dsh('git switch --detach 4def8d2'), ASK, { diffs: { '4def8d2': 'CLAUDE.md\n' } }],
  ['T3 gh pr checkout', dsh('gh pr checkout 38'), ASK],
  ['T3b gh co (the built-in alias)', dsh('gh co 38'), ASK],
  ['T4 git merge cs/other', dsh('git merge cs/other'), ASK],
  ['T5 git merge --ff-only origin/main', dsh('git merge --ff-only origin/main'), null],
  ['T5b git merge origin/main (as the existing day case)', dsh('git merge origin/main'), null],
  ['T6 git switch -c cs/new origin/main', dsh('git switch -c cs/new origin/main'), null],
  ['T6b git switch -c cs/new cs/other', dsh('git switch -c cs/new cs/other'), ASK],
  ['T7 git switch -c cs/new, HEAD equals origin/main', dsh('git switch -c cs/new'), null, { diffs: { HEAD: 'shamble-game.js\n' } }],
  ['T8 git switch cs/other in a worktree', dsh('git switch cs/other', { cwd: '/w' }), null, { diffs: OTHER }],
  // fix round 2 (F-b): moved from null to ask. The add's folder is now a write target, and in this fake .gate/x is
  // already a worktree top (a kit folder); real git refuses to add a worktree onto an existing one. T9n is the control.
  ['T9 git worktree add', dsh('git worktree add .gate/x 4def8d2'), ASK],
  ['T9n git worktree add into a new folder', dsh('git worktree add .gate/new 4def8d2'), null],
  ['T10 cd into the main folder, then switch', dsh('cd /p && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['T11 git pull', dsh('git pull'), ASK],
  ['T12 a subagent switches the main folder: deny', dsh('git switch cs/other', { agent_id: 'a1' }), DENY, { diffs: OTHER }],
  ['T13 git checkout main (unchanged)', dsh('git checkout main'), DENY],
  ['T14 git show into a worktree file', dsh('git show origin/main:CLAUDE.md > /w/x.md'), null],
  ['T16d git switch to a branch that changes only game files', dsh('git switch cs/other'), null, { diffs: { 'cs/other': 'shamble-game.js\n' } }],
  ['T18 git switch to a stale main', dsh('git switch main'), ASK, { diffs: { main: '.claude/hooks/guard.mjs\n' } }, /merge --ff-only origin\/main/],
  ['T19 git switch main, no protected difference', dsh('git switch main'), null, { diffs: { main: '' } }],
  ['T22 gh pr checkout in a worktree', dsh('gh pr checkout 38', { cwd: '/w' }), null],
  ['T23 git pull --ff-only origin main', dsh('git pull --ff-only origin main'), null],
  ['T23 git pull origin cs/x', dsh('git pull origin cs/x'), ASK],
  ['T24 git switch -c cs/new, HEAD differs in protected files', dsh('git switch -c cs/new'), ASK, { diffs: { HEAD: '.claude/hooks/guard.mjs\n' } }],
  ['T25 git bisect start', dsh('git bisect start'), ASK],
  ['T26 git switch - (cannot tell)', dsh('git switch -'), ASK],
  ['T26 git switch --orphan', dsh('git switch --orphan x'), ASK],
  ['T27 git switch, compare fails: ask', dsh('git switch cs/gone'), ASK, {}, /can't compare/],
  // step 4b, R1: the file tools (Edit, Write, MultiEdit, NotebookEdit) in a day session
  ['A1 Edit .claude/hooks/guard.mjs', ded('Edit', '/p/.claude/hooks/guard.mjs'), ASK, {}, /ask first in a day session: \.claude\/hooks\/guard\.mjs$/],
  ['A2 Write CLAUDE.md: the ask names the file', ded('Write', '/p/CLAUDE.md'), ASK, {}, /^kit and gate files ask first in a day session: CLAUDE\.md$/],
  ['A3 Edit tools/gate.mjs', ded('Edit', '/p/tools/gate.mjs'), ASK],
  ['A4 Edit docs/STATUS.md', ded('Edit', '/p/docs/STATUS.md'), null],
  ['A5 Write agent memory', ded('Write', '/p/.claude/agent-memory-local/x'), null],
  ['A6 Write .checkpoints', ded('Write', '/p/.checkpoints/x.md'), null],
  ['A8 Edit a game file in worktree /w', ded('Edit', '/w/shamble-game.js'), null],
  ['A9 Edit .claude/worktrees/a1/CLAUDE.md', ded('Edit', '/p/.claude/worktrees/a1/CLAUDE.md'), ASK],
  ['A9b Edit .gate/x worktree .claude/settings.json', ded('Edit', '/p/.gate/x/.claude/settings.json'), ASK, {}, /\.claude\/settings\.json/],
  ['A9c Write .gate/x worktree tools/gate.mjs', ded('Write', '/p/.gate/x/tools/gate.mjs'), ASK],
  ['A9d Edit .gate/x worktree game file', ded('Edit', '/p/.gate/x/shamble-core.js'), null],
  ['A10 Edit through a symlink to .claude', ded('Edit', '/p/link/settings.json'), ASK],
  ["A11 Edit another repository's .claude", ded('Edit', '/o/.claude/settings.json'), null],
  ['A22 a subagent Edit of a kit file: deny', ded('Edit', '/p/.claude/settings.json', { agent_id: 'a1' }), DENY, {}, /main session/],
  ['A22 a subagent Edit of a game file', ded('Edit', '/p/shamble-game.js', { agent_id: 'a1' }), null],
  ['A22b agent_type without agent_id, Edit: ask, not deny', ded('Edit', '/p/.claude/settings.json', { agent_type: 'advisor' }), ASK],
  ['A25 MultiEdit a kit file', ded('MultiEdit', '/p/.claude/rules/x.md'), ASK],
  ['A25 NotebookEdit a kit file', ded('NotebookEdit', '/p/.claude/x.ipynb'), ASK],
  ['A36 win32: Edit claude.MD is CLAUDE.md', ded('Edit', '/p/claude.MD'), ASK, { platform: 'win32' }],
  ['A36 linux: Edit claude.MD is another file', ded('Edit', '/p/claude.MD'), null, { platform: 'linux' }],
  // step 4b, S-3: darwin folds case too
  ['S-3 darwin: Edit claude.MD is CLAUDE.md', ded('Edit', '/p/claude.MD'), ASK, { platform: 'darwin' }],
  ['S-3 darwin: Edit .Claude/Settings.json', ded('Edit', '/p/.Claude/Settings.json'), ASK, { platform: 'darwin' }],
  ['S-3 darwin: echo > claude.MD', dsh('echo x > claude.MD'), ASK, { platform: 'darwin' }],
  // step 4b, R2: a plain merge of origin/main only when the branch changed no kit file since it left main
  ['T5c git merge origin/main, the branch changed a kit file: ask', dsh('git merge origin/main'), ASK,
    { threeDot: '.claude/settings.json\n' }, /mix with main/],
  ['T5d git merge origin/main, the compare fails: ask', dsh('git merge origin/main'), ASK, { threeDot: null }, /can't compare the branch/],
  ['T5e a subagent, plain merge onto kit changes: deny', dsh('git merge origin/main', { agent_id: 'a1' }), DENY,
    { threeDot: '.claude/settings.json\n' }],
  ['T5f --ff-only with kit changes on the branch', dsh('git merge --ff-only origin/main'), null, { threeDot: '.claude/settings.json\n' }],
  ['T5g plain merge, the branch changed only game files', dsh('git merge origin/main'), null, { threeDot: 'shamble-game.js\n' }],
  // step 4b, R4 (b): an agent's write to a target the guard cannot resolve is refused
  ['A20a an agent writes to $F: deny', dsh('echo x > $F', { agent_id: 'a1' }), DENY, {}, /spells its write targets out/],
  ['A20b an agent tees a glob: deny', dsh('tee .gate/*.md', { agent_id: 'a1' }), DENY],
  ['A20c an agent writes a literal notes path', dsh('echo x > .checkpoints/x.md', { agent_id: 'a1' }), null],
  // step 4b, S-1: a switch target that starts with '-' never reaches a guard-run git command
  ['S-1 git switch -- --output=<path>: ask', dsh('git switch -- --output=/tmp/x'), ASK, { diffs: OTHER }],
  ['S-1 git switch -c n -- --output=<path>: ask', dsh('git switch -c n -- --output=/tmp/x'), ASK, { diffs: OTHER }],
  ['S-1 git switch --detach -- --output=<path>: ask', dsh('git switch --detach -- --output=/tmp/x'), ASK, { diffs: OTHER }],
  // step 4b, S-2: origin/main by commit; a local branch named origin/main at another commit is not main
  ['S-2 git merge origin/main, a local origin/main elsewhere: ask', dsh('git merge origin/main'), ASK, { shadow: true }, /by commit/],
  ['S-2 git merge --ff-only origin/main, the same: ask', dsh('git merge --ff-only origin/main'), ASK, { shadow: true }],
  ['S-2 git switch -c from that origin/main: ask', dsh('git switch -c cs/new origin/main'), ASK, { shadow: true }],
  ['S-2 git merge --ff-only refs/remotes/origin/main, shadowed or not', dsh('git merge --ff-only refs/remotes/origin/main'), null,
    { shadow: true }],
  // step 4b, S-4: pushd and popd move the folder; a move the guard cannot follow counts as the project folder
  ['S-4 pushd into the main folder, then switch: ask', dsh('pushd /p && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['S-4 pushd into a worktree, then switch', dsh('pushd /w && git switch cs/other'), null, { diffs: OTHER }],
  ['S-4 pushd away, popd back, then switch: ask', dsh('pushd /w && popd && git switch cs/other'), ASK, { diffs: OTHER }],
  ['S-4 popd with an empty stack: ask', dsh('popd && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['S-4 pushd with no folder and an empty stack: ask', dsh('pushd && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['S-4 pushd +1: ask', dsh('pushd /w && pushd +1 && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['S-4 cd $D (untracked): ask', dsh('cd $D && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['S-4 cd - (untracked): ask', dsh('cd - && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['S-4 a relative cd after an untracked one: ask', dsh('cd $D && cd .gate/x && git switch cs/other', { cwd: '/w' }), ASK,
    { diffs: OTHER }],
  ['S-4 a bare relative cd under CDPATH: ask', dsh('cd w2 && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER, cdpath: '/' }],
  ['S-4 a cd in a subshell does not carry over: ask', dsh('(cd /w) && git switch cs/other'), ASK, { diffs: OTHER }],
  ['S-4 a cd in a substitution: ask', dsh('echo $(cd /w) && git switch cs/other'), ASK, { diffs: OTHER }],
  ['S-4 a cd before a pipe: ask', dsh('cd /w | true; git switch cs/other'), ASK, { diffs: OTHER }],
  ['S-4 cd to a worktree is still followed', dsh('cd /w && git switch cs/other'), null, { diffs: OTHER }],
  // fix round R-b: these two moved from followed to unknown (-P is an option other than -L; .gate/x is a bare name)
  ['S-4 cd -P to a worktree: unknown since the fix round (R-b), ask', dsh('cd -P /w && git switch cs/other'), ASK, { diffs: OTHER }],
  ['S-4 a bare relative cd into a worktree: unknown since the fix round (R-b), ask', dsh('cd .gate/x && git switch cs/other'), ASK,
    { diffs: OTHER }],
  // step 4c, finding 5: CDPATH set in the line itself; a bare relative cd then counts as lost (the project folder)
  ['F5 a CDPATH= prefix on the cd: ask', dsh('CDPATH=/ cd w2 && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['F5 a CDPATH= prefix on pushd: ask', dsh('CDPATH=/ pushd w2 && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['F5 export CDPATH earlier in the line: ask', dsh('export CDPATH=/; cd w2 && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['F5 declare -x CDPATH: ask', dsh('declare -x CDPATH=/ && cd w2 && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['F5 a bare CDPATH= assignment: ask', dsh('CDPATH=/; cd w2 && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['F5 ${CDPATH:=...}: ask', dsh(': ${CDPATH:=/}; cd w2 && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['F5 an indirect ${!V:=...}: ask', dsh(': ${!V:=/}; cd w2 && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['F5 export with a quoted name: ask', dsh('export CD"PATH"=/; cd w2 && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['F5 eval: ask', dsh('eval "$S"; cd w2 && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['F5 source (.): ask', dsh('. ./env.sh; cd w2 && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['F5 printf -v: ask', dsh('printf -v "$V" /; cd w2 && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['F5 a command named by a variable: ask', dsh('$SET CD$P=/; cd w2 && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['F5 a later relative move after the lost one: ask', dsh('export CDPATH=/; cd w2 && cd ../w && git switch cs/other', { cwd: '/w' }),
    ASK, { diffs: OTHER }],
  // fix round: these five moved to unknown (R-c: a cd after another command; R-b: a bare name), and the last one's
  // reason now says the folder is unknown (R-a)
  ['F5 ./ after another command: unknown since the fix round (R-c), ask', dsh('export CDPATH=/; cd ./.gate/x && git switch cs/other'),
    ASK, { diffs: OTHER }],
  ['F5 an absolute target after another command: unknown since the fix round (R-c), ask', dsh('export CDPATH=/; cd /w && git switch cs/other'),
    ASK, { diffs: OTHER }],
  ['F5 printf, then a bare cd: unknown since the fix round (R-b, R-c), ask', dsh('printf x; cd .gate/x && git switch cs/other'), ASK,
    { diffs: OTHER }],
  ['F5 cd docs (a bare name), then a relative write: asks since the fix round (R-a, R-b)', dsh('cd docs && echo x > notes.md'), ASK, {},
    /notes\.md after a cd the guard cannot follow.*absolute path, or put the cd first/],
  ['F5 cd .claude (a bare name), then a kit write still asks (R-a reason since the fix round)', dsh('cd .claude && echo x > settings.json'),
    ASK, {}, /settings\.json after a cd the guard cannot follow/],
  // step 4c, finding 6: git switch <name> moves to the branch refs/heads/<name>; a name a tag shares asks
  ['F6 git switch dup, a same-name tag at a clean commit: ask', dsh('git switch dup'), ASK,
    { diffs: { dup: '.claude/settings.json\n' }, tags: ['dup'] }, /refs\/heads\/dup/],
  ['F6 git switch dup, ambiguous even when the branch is clean: ask', dsh('git switch dup'), ASK, { diffs: { dup: '' }, tags: ['dup'] }],
  ['F6 git switch v1, a tag and no branch: ask', dsh('git switch v1'), ASK, { tags: ['v1'] }],
  ['F6 git switch --detach dup: the tag, as git reads it', dsh('git switch --detach dup'), null,
    { diffs: { dup: '.claude/settings.json\n' }, tags: ['dup'] }],
  ['F6 git switch -d dup: the same', dsh('git switch -d dup'), null, { diffs: { dup: '.claude/settings.json\n' }, tags: ['dup'] }],
  ['F6 git switch --detach --no-detach dup: ask', dsh('git switch --detach --no-detach dup'), ASK,
    { diffs: { dup: '.claude/settings.json\n' }, tags: ['dup'] }],
  ['F6 a subagent switches to an ambiguous name: deny', dsh('git switch dup', { agent_id: 'a1' }), DENY, { diffs: { dup: '' }, tags: ['dup'] }],
  // fix round (step 8 reviews: SA-1, automated 7 and 8, A-1), R-a..R-e: an unknown folder is a state; both checks ask
  // R-a + R-d, SA-1: a relative write after a cd in a forked line asks (Ff1-Ff3 and the skeptic's seven forms)
  ['Ff1 (cd .claude && sed -i ... settings.json): ask', dsh('(cd .claude && sed -i s/a/b/ settings.json)'), ASK, {}, /cannot follow/],
  ['Ff2 cd .claude && sed -i ... && git status | cat: ask', dsh('cd .claude && sed -i s/a/b/ settings.json && git status | cat'), ASK],
  ['Ff3 cd .claude && echo > settings.json; ls | head: ask', dsh('cd .claude && echo x > settings.json; ls | head'), ASK],
  ['SK4 cd .claude/hooks && sed -i ... g.mjs && node --version | tail -1: ask',
    dsh('cd .claude/hooks && sed -i s/a/b/ g.mjs && node --version | tail -1'), ASK],
  ['SK5 cd .claude/hooks && sed -i ... g.mjs &: ask', dsh('cd .claude/hooks && sed -i s/a/b/ g.mjs &'), ASK],
  ['SK6 cd .claude && sed -i ... && echo $(date): ask', dsh('cd .claude && sed -i s/a/b/ settings.json && echo $(date)'), ASK],
  ['SK7 git status | cat; cd .claude && rm settings.json: ask', dsh('git status | cat; cd .claude && rm settings.json'), ASK],
  ['R-d (cd ./.claude && sed -i ... settings.json): ask', dsh('(cd ./.claude && sed -i s/a/b/ settings.json)'), ASK],
  ['R-d cd /p/.claude && echo > settings.json | cat, from a worktree: ask', dsh('cd /p/.claude && echo x > settings.json | cat', { cwd: '/w' }), ASK],
  ['R-a an agent: a relative write after an unknown cd: deny', dsh('(cd ./.claude && sed -i s/a/b/ settings.json)', { agent_id: 'a1' }),
    DENY, {}, /cannot follow.*absolute path/],
  ['R-a an absolute write after an unknown cd is checked as before', dsh('cd - && echo x > /p/docs/x.md'), null],
  ['R-a an absolute kit write after an unknown cd asks as before', dsh('cd - && echo x > /p/.claude/settings.json'), ASK, {}, /\.claude\/settings\.json/],
  // R-b + R-c, automated 7 and 8
  ["A7 builtin export CD''PATH=/p ; cd tools && git switch: ask", dsh("builtin export CD''PATH=/p ; cd tools && git switch cs/other", { cwd: '/w' }),
    ASK, { diffs: OTHER }],
  ["A7b command -p export CD''PATH=/p; cd tools && git switch: ask", dsh("command -p export CD''PATH=/p; cd tools && git switch cs/other", { cwd: '/w' }),
    ASK, { diffs: OTHER }],
  ['A8 shopt -s cdable_vars; x=/p; cd x && git switch: ask', dsh('shopt -s cdable_vars; x=/p; cd x && git switch cs/other', { cwd: '/w' }),
    ASK, { diffs: OTHER }],
  ['A8b cd x (a bare name) first in the line: ask', dsh('cd x && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  // R-b: a cd that may fail, and the target forms
  ['R-b a failed cd: cd /nonexistent; git switch: ask', dsh('cd /nonexistent; git switch cs/other'), ASK, { diffs: OTHER, missing: ['/nonexistent'] }],
  ['R-b a failed cd into a worktree path: cd /w/gone; git switch: ask', dsh('cd /w/gone; git switch cs/other'), ASK,
    { diffs: OTHER, missing: ['/w/gone'] }],
  ['R-b cd ~/x (HOME can be set): ask', dsh('cd ~/x && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['R-b cd with no target (HOME): ask', dsh('cd && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['R-b cd -e (another option): ask', dsh('cd -e /p && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['R-b cd -L to a worktree is followed', dsh('cd -L /w && git switch cs/other'), null, { diffs: OTHER }],
  ['R-b cd -- to a worktree is followed', dsh('cd -- /w && git switch cs/other'), null, { diffs: OTHER }],
  ['R-b cd ../ from a worktree into the main folder: ask', dsh('cd ../p && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['R-b cd /w/$X (a variable): ask', dsh('cd /w/$X && git switch cs/other'), ASK, { diffs: OTHER }],
  ['R-b cd /w/* (a glob): ask', dsh('cd /w/* && git switch cs/other'), ASK, { diffs: OTHER }],
  ['R-b cd with two targets (bash refuses it): ask', dsh('cd /w /p && git switch cs/other'), ASK, { diffs: OTHER }],
  ['R-b a ./ cd after an unknown one: ask', dsh('cd - && cd ./w && git switch cs/other'), ASK, { diffs: OTHER }],
  // R-c: skipped cds, wrappers, path spellings, prefixes and later cds
  ['R-c true || cd /w; git switch: ask', dsh('true || cd /w; git switch cs/other'), ASK, { diffs: OTHER }],
  ['R-c if false; then cd /w; fi; git switch: ask', dsh('if false; then cd /w; fi; git switch cs/other'), ASK, { diffs: OTHER }],
  ['R-c timeout 5 cd /w && git switch: ask', dsh('timeout 5 cd /w && git switch cs/other'), ASK, { diffs: OTHER }],
  ['R-c command -p cd /p && git switch, from a worktree: ask', dsh('command -p cd /p && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['R-c command -v cd /w (it does not run): ask', dsh('command -v cd /w && git switch cs/other'), ASK, { diffs: OTHER }],
  ['R-c env cd /w: ask', dsh('env cd /w && git switch cs/other'), ASK, { diffs: OTHER }],
  ['R-c /usr/bin/cd /w (a program, not the shell): ask', dsh('/usr/bin/cd /w && git switch cs/other'), ASK, { diffs: OTHER }],
  ['R-c HOME=/w cd: ask', dsh('HOME=/w cd && git switch cs/other'), ASK, { diffs: OTHER }],
  ['R-c a VAR= prefix on an absolute cd: ask', dsh('X=1 cd /w && git switch cs/other'), ASK, { diffs: OTHER }],
  ['R-c git status && cd /w; git switch: ask', dsh('git status && cd /w; git switch cs/other'), ASK, { diffs: OTHER }],
  ['R-c cd /p/.claude || cd /w; a relative write: ask', dsh('cd /p/.claude || cd /w; echo x > settings.json'), ASK, {}, /cannot follow/],
  ['R-c cd /w || exit 1; git switch (the cd cannot fail): followed', dsh('cd /w || exit 1; git switch cs/other'), null, { diffs: OTHER }],
  ['R-c if cd /w; then ...; else cd /p; fi: ask', dsh('if cd /w; then :; else cd /p; fi; git switch cs/other'), ASK, { diffs: OTHER }],
  ['R-c while cd .. (a loop repeats it): ask', dsh('while cd ..; do :; done; git switch cs/other', { cwd: '/p/.gate/x/sub' }), ASK,
    { diffs: OTHER }],
  ['R-c builtin cd /w is followed', dsh('builtin cd /w && git switch cs/other'), null, { diffs: OTHER }],
  ['R-c command -p cd /w is followed', dsh('command -p cd /w && git switch cs/other'), null, { diffs: OTHER }],
  ['R-c { cd /w; } && git switch is followed', dsh('{ cd /w; } && git switch cs/other'), null, { diffs: OTHER }],
  ['R-c { cd /p; }; cd ./.gate/x (a bare } runs nothing) is followed', dsh('{ cd /p; }; cd ./.gate/x && git switch cs/other', { cwd: '/w' }),
    null, { diffs: OTHER }],
  ['R-c popd back to an unknown folder: ask', dsh('cd -; pushd /w; popd; echo x > notes.md'), ASK],
  ['R-c popd with an empty stack, then a relative write: ask', dsh('popd; echo x > notes.md'), ASK],
  ['R-c if cd /w; then git switch; fi is followed', dsh('if cd /w; then git switch cs/other; fi'), null, { diffs: OTHER }],
  ['R-c cd /p && cd ./.gate/x (a leading run) is followed', dsh('cd /p && cd ./.gate/x && git switch cs/other', { cwd: '/w' }), null, { diffs: OTHER }],
  // pushd and popd on a known stack
  ['R-c pushd, pushd, pushd (swap) is followed', dsh('pushd /w && pushd /p/.gate/x && pushd && git switch cs/other'), null, { diffs: OTHER }],
  ['R-c a pushd to an unknown target makes the stack unknown: ask', dsh('pushd /p/.claude; pushd w2; popd; echo x > settings.json', { cwd: '/w' }),
    ASK],
  ['R-c pushd behind a wrapper makes the stack unknown: ask', dsh('pushd /w; nohup pushd /p; popd; git switch cs/other', { cwd: '/w' }), ASK,
    { diffs: OTHER }],
  // R-e: commands that can move the folder without the word cd
  ['R-e eval cd /p && git switch, from a worktree: ask', dsh('eval cd /p && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['R-e . ./go.sh && git switch, from a worktree: ask', dsh('. ./go.sh && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['R-e source go.sh; a relative write: ask', dsh('source go.sh; echo x > notes.md'), ASK],
  ['R-e $CD /p && git switch, from a worktree: ask', dsh('$CD /p && git switch cs/other', { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['R-e time -p cd /p (prog -p) && git switch, from a worktree: ask', dsh('time -p cd /p && git switch cs/other', { cwd: '/w' }), ASK,
    { diffs: OTHER }],
  ['R-e exec 3>&1 (no command) leaves the folder known', dsh('exec 3>&1; echo x > notes.md'), null],
  ['R-e exec bash; a relative write: ask', dsh('exec bash; echo x > notes.md'), ASK],
  // the ruling's quiet controls stay quiet
  ['QC1 cd <abs wt> && git switch cs/x', dsh('cd /w && git switch cs/other'), null, { diffs: OTHER }],
  ['QC2 cd ./.gate/wt && git switch cs/x', dsh('cd ./.gate/x && git switch cs/other'), null, { diffs: OTHER }],
  ['QC3 cd <abs wt> && echo x > notes.md', dsh('cd /w && echo x > notes.md'), null],
  ['QC4 cd <abs>/tools && npm test 2>&1 | tail', dsh('cd /p/tools && npm test 2>&1 | tail'), null],
  // fix round 2 (step 8 reviews). SA-8: a line that names GIT_DIR, GIT_WORK_TREE, GIT_COMMON_DIR or GIT_INDEX_FILE
  ['SA-8 export GIT_DIR=, then git status: ask', dsh('export GIT_DIR=/x; git status'), ASK, {}, /GIT_DIR/],
  ["SA-8 GIT''_DIR= (quotes split the name): ask", dsh("GIT''_DIR=/x git status"), ASK],
  ['SA-8 a GIT_COMMON_DIR= prefix: ask', dsh('GIT_COMMON_DIR=/x git status'), ASK],
  ['SA-8 export GIT_INDEX_FILE=: ask', dsh('export GIT_INDEX_FILE=/tmp/i && git add docs/a.md'), ASK],
  ['SA-8 GIT_WORK_TREE in a here-document body (raw text): ask', dsh('cat <<EOF > notes.md\nGIT_WORK_TREE=/x\nEOF'), ASK],
  ['SA-8 a later force push still denies', dsh('export GIT_DIR=/x; git push -f origin cs/x'), DENY],
  ['SA-8 control: GIT_TRACE is not one of them', dsh('GIT_TRACE=1 git status'), null],
  // SA-5: any prefix of --in-place from --i on, with or without =suffix; and every positional word is a target
  ['SA-5 sed --i: ask', dsh('sed --i s/a/b/ CLAUDE.md'), ASK],
  ['SA-5 sed --in: ask', dsh('sed --in s/a/b/ CLAUDE.md'), ASK],
  ['SA-5 sed --in-pl: ask', dsh('sed --in-pl s/a/b/ CLAUDE.md'), ASK],
  ['SA-5 sed --in-place=.bak: ask', dsh('sed --in-place=.bak s/a/b/ CLAUDE.md'), ASK],
  ['SA-5 sed --in-p=.bak: ask', dsh('sed --in-p=.bak s/a/b/ CLAUDE.md'), ASK],
  ['SA-5 sed -i -es/a/b/ (attached script): ask', dsh('sed -i -es/a/b/ CLAUDE.md'), ASK],
  ['SA-5 sed --i --expr=s/a/b/ (abbreviated --expression=): ask', dsh('sed --i --expr=s/a/b/ CLAUDE.md'), ASK],
  ['SA-5 perl -pi -es/a/b/ (attached script): ask', dsh('perl -pi -es/a/b/ CLAUDE.md'), ASK],
  ['SA-5 control: sed --quiet only reads', dsh('sed --quiet p CLAUDE.md'), null],
  ['SA-5 control: sed -i on a game file', dsh('sed -i s/a/b/ shamble-game.js'), null],
  // F-b: git worktree add <path> and git clone <url> [<dir>] write their folder
  ['F-b git worktree add into .claude/rules: ask', dsh('git worktree add .claude/rules/wt 4def8d2'), ASK, {}, /\.claude\/rules\/wt \(git worktree\)/],
  ['F-b git worktree add -b cs/x into .claude/skills: ask', dsh('git worktree add -b cs/x .claude/skills/wt'), ASK],
  ['F-b git worktree add --lock --reason why into .github: ask', dsh('git worktree add --lock --reason why .github/wt'), ASK],
  ['F-b an agent: git worktree add into .claude/rules: deny', dsh('git worktree add .claude/rules/wt 4def8d2', { agent_id: 'a1' }), DENY],
  ['F-b git clone <url> .claude/skills/x: ask', dsh('git clone https://example.invalid/o/r.git .claude/skills/x'), ASK, {}, /git clone/],
  ['F-b an agent: git clone <url> .claude/skills/x: deny', dsh('git clone https://example.invalid/o/r.git .claude/skills/x', { agent_id: 'a1' }), DENY],
  ['F-b git clone <url> with no folder: the URL\'s last part (.claude): ask', dsh('git clone https://example.invalid/o/.claude.git'), ASK],
  ['F-b git clone --depth 1 <url ending CLAUDE.md.git>: ask', dsh('git clone --depth 1 https://example.invalid/o/CLAUDE.md.git/'), ASK],
  ['F-b git clone --separate-git-dir into .claude: ask', dsh('git clone --separate-git-dir .claude/g https://example.invalid/o/r.git /tmp/r'), ASK],
  ['F-b control: git clone into /tmp/r', dsh('git clone https://example.invalid/o/r.git /tmp/r'), null],
  ['F-b control: git clone <url> with no folder (r)', dsh('git clone https://example.invalid/o/r.git'), null],
  // SA-6: a folder target that holds the project folder, or whose tracked files name a kit path; git failing asks
  ['SA-6 Fc1 rm -rf docs, a tracked docs/sub/x/CLAUDE.md: ask', dsh('rm -rf docs'), ASK, { lsFiles: { docs: 'docs/a.md\0docs/sub/x/CLAUDE.md\0' } },
    /docs \(it holds docs\/sub\/x\/CLAUDE\.md\)/],
  ['SA-6 Fc2 rm -rf / (an ancestor of the project folder): ask', dsh('rm -rf /'), ASK, {}, /holds the project folder/],
  ['SA-6 rm -rf .. from the project folder: ask', dsh('rm -rf ..'), ASK],
  ['SA-6 mv a folder holding a nested .mcp.json: ask', dsh('mv kitbits /tmp/'), ASK, { lsFiles: { kitbits: 'kitbits/a.md\0kitbits/x/.mcp.json\0' } }],
  ['SA-6 git cannot list the folder: ask', dsh('rm -r docs/research'), ASK, { lsFail: true }, /could not list/],
  ['SA-6 an agent: rm -rf docs holding a kit path: deny', dsh('rm -rf docs', { agent_id: 'a1' }), DENY, { lsFiles: { docs: 'docs/sub/x/CLAUDE.md\0' } }],
  ['SA-6 control: rm -rf docs, no kit path tracked there', dsh('rm -rf docs'), null, { lsFiles: { docs: 'docs/a.md\0docs/b.md\0' } }],
  ['SA-6 control: a file target is not listed', dsh('echo x > docs/a.md'), null, { lsFail: true, missing: ['/p/docs/a.md'] }],
  // --- fix pass 2, the day paths (the seatbelt): SD-3, SD-4, SD-5, SD-6, V-1 ---
  // SD-3: in the main folder, a switch or merge after an earlier git command in the line asks, unless its target word
  // is origin/main (the guard reads the target before the line runs; the fetch can move it). origin/cs/x is clean now.
  ['SD-3 C3d git fetch -q origin cs/x && git switch --detach origin/cs/x, in the main folder: ask',
    dsh('git fetch -q origin cs/x && git switch --detach origin/cs/x'), ASK, { diffs: { 'origin/cs/x': 'docs/a.md\n' } },
    /\(an earlier git command in this line could move origin\/cs\/x after the guard checked it: run git fetch in its own call first, then the switch alone\)/],
  ['SD-3 an agent: C3d: deny', dsh('git fetch -q origin cs/x && git switch --detach origin/cs/x', { agent_id: 'a1' }), DENY,
    { diffs: { 'origin/cs/x': 'docs/a.md\n' } }, /A subagent never moves the main folder/],
  ['SD-3 control: the same switch alone (origin/cs/x clean) goes on', dsh('git switch --detach origin/cs/x'), null, { diffs: { 'origin/cs/x': 'docs/a.md\n' } }],
  ['SD-3 git fetch -q origin cs/x; git switch -q cs/x (a ;, cs/x clean now): ask', dsh('git fetch -q origin cs/x; git switch -q cs/x'), ASK,
    { diffs: { 'cs/x': 'docs/a.md\n' } }, /could move cs\/x after/],
  ['SD-3 git status && git switch -c cs/new (no start point: HEAD): ask', dsh('git status && git switch -c cs/new'), ASK,
    { diffs: { HEAD: 'docs/a.md\n' } }, /could move HEAD after/],
  ['SD-3 git fetch -q origin main && git switch -c cs/new main (main at origin/main now; only the word origin/main): ask',
    dsh('git fetch -q origin main && git switch -c cs/new main'), ASK, { dwim: { main: 'origin/main' } }, /could move main after/],
  ['SD-3 git fetch -q origin main && git merge --ff-only main (main at origin/main now): ask', dsh('git fetch -q origin main && git merge --ff-only main'),
    ASK, { dwim: { main: 'origin/main' } }, /could move main after the guard checked it: run git fetch in its own call first, then the merge alone/],
  ['SD-3 control: git merge --ff-only main alone (main at origin/main) goes on', dsh('git merge --ff-only main'), null, { dwim: { main: 'origin/main' } }],
  ['SD-3 control: git fetch origin main && git merge --ff-only origin/main (WT-UPDATE) goes on', dsh('git fetch origin main && git merge --ff-only origin/main'), null],
  ['SD-3 control: git fetch -q origin main && git switch --detach origin/main (the advisor\'s step 0) goes on',
    dsh('git fetch -q origin main && git switch --detach origin/main'), null, { diffs: { 'origin/main': '' } }],
  ['SD-3 control: git fetch -q origin main && git switch -c cs/new origin/main goes on', dsh('git fetch -q origin main && git switch -c cs/new origin/main'), null],
  ['SD-3 control: C3d in a worktree goes on', dsh('git fetch -q origin cs/x && git switch --detach origin/cs/x', { cwd: '/w' }), null,
    { diffs: { 'origin/cs/x': 'docs/a.md\n' } }],
  // SD-4: a cd, pushd or popd joined by && while the folder is unknown stays unknown (bash skips it after a failed
  // cd); ! is no longer a way to reach a followed cd. /m is a missing folder, /n a folder outside any repository.
  ['SD-4 C1 cd /m && cd /w; git switch -q -c cs/c1 origin/cs/evil: ask', dsh('cd /m && cd /w; git switch -q -c cs/c1 origin/cs/evil'), ASK,
    { missing: ['/m'] }],
  ['SD-4 C1c cd /m && cd /n; echo x > CLAUDE.md: ask', dsh('cd /m && cd /n; echo x > CLAUDE.md'), ASK, { missing: ['/m'] }, /cannot follow/],
  ['SD-4 an agent: C1c: deny', dsh('cd /m && cd /n; echo x > CLAUDE.md', { agent_id: 'a1' }), DENY, { missing: ['/m'] }, /cannot follow/],
  ['SD-4 C2 ! cd /p && cd ./.gate/x; git switch -q -c cs/c2 origin/cs/evil: ask', dsh('! cd /p && cd ./.gate/x; git switch -q -c cs/c2 origin/cs/evil'),
    ASK],
  ['SD-4 C3 cd /p && cd /m && cd /w; git switch -q -c cs/c3 origin/cs/evil: ask', dsh('cd /p && cd /m && cd /w; git switch -q -c cs/c3 origin/cs/evil'),
    ASK, { missing: ['/m'] }],
  ['SD-4 control: cd /m; cd /w; git switch -q -c cs/c0 origin/cs/evil (a ;: the cd runs) goes on', dsh('cd /m; cd /w; git switch -q -c cs/c0 origin/cs/evil'),
    null, { missing: ['/m'] }],
  ['SD-4 a pushd after && while unknown leaves the stack unknown: ask', dsh('pushd /p && pushd /w; cd ~ && pushd /p/.gate/x; popd; popd; git switch cs/other',
    { cwd: '/w' }), ASK, { diffs: OTHER }],
  ['R-b a ./ cd after an unknown one, joined by ; (SD-4 does not apply): ask', dsh('cd -; cd ./w && git switch cs/other'), ASK, { diffs: OTHER }],
  // V-1: function, trap and alias as command words make the folder unknown (they can run a cd in this shell)
  ['V-1 P1a cd /w; function f { cd /p; }; f; git switch cs/other: ask', dsh('cd /w; function f { cd /p; }; f; git switch cs/other'), ASK, { diffs: OTHER }],
  ['V-1 P1b cd /w; trap "cd /p" DEBUG; git switch cs/other: ask', dsh('cd /w; trap "cd /p" DEBUG; git switch cs/other'), ASK, { diffs: OTHER }],
  ['V-1 P1c cd /w; shopt -s expand_aliases; alias c=cd (newline) c /p (newline) git switch: ask',
    dsh('cd /w; shopt -s expand_aliases; alias c=cd\nc /p\ngit switch cs/other'), ASK, { diffs: OTHER }],
  ['V-1 P1d cd /; function f { cd /p; }; f; echo x > CLAUDE.md: ask', dsh('cd /; function f { cd /p; }; f; echo x > CLAUDE.md'), ASK, {}, /cannot follow/],
  ['V-1 P1e cd /; trap "cd /p" DEBUG; echo x > CLAUDE.md: ask', dsh('cd /; trap "cd /p" DEBUG; echo x > CLAUDE.md'), ASK, {}, /cannot follow/],
  ['V-1 control: grep function x goes on', dsh('grep function x'), null],
  ['V-1 control: cd /w; grep -e function -e trap -e alias x; git switch (the words, not the commands) goes on',
    dsh('cd /w; grep -e function -e trap -e alias x; git switch cs/other'), null, { diffs: OTHER }],
  // SD-5: git clone's targets are every positional word after the first, and the folder git makes from each one
  ['SD-5 git clone --revision main <url> .claude/sub: ask', dsh('git clone --revision main https://example.invalid/o/r.git .claude/sub'), ASK, {},
    /\.claude\/sub \(git clone\)/],
  ['SD-5 git clone --revision main <url> .claude: ask', dsh('git clone --revision main https://example.invalid/o/r.git .claude'), ASK],
  ['SD-5 git clone --revision main <url> CLAUDE.md: ask', dsh('git clone --revision main https://example.invalid/o/r.git CLAUDE.md'), ASK],
  ['SD-5 an agent: git clone --revision main <url> .claude/sub: deny', dsh('git clone --revision main https://example.invalid/o/r.git .claude/sub',
    { agent_id: 'a1' }), DENY],
  ['SD-5 an agent: git clone --revision main <url> CLAUDE.md: deny', dsh('git clone --revision main https://example.invalid/o/r.git CLAUDE.md',
    { agent_id: 'a1' }), DENY],
  ['SD-5 an option the guard does not know, with a value: git clone --x v <url> .claude/sub: ask',
    dsh('git clone --x v https://example.invalid/o/r.git .claude/sub'), ASK],
  ['SD-5 git clone --x v <url ending tools.git> (no folder; git makes tools): ask', dsh('git clone --x v https://example.invalid/o/tools.git'), ASK],
  ['SD-5 control: git clone <project> <outside> goes on', dsh('git clone /p /tmp/copy'), null],
  ['SD-5 control: git clone -b main <project> <outside> goes on', dsh('git clone -b main /p /tmp/copy'), null],
  ['SD-5 control: git clone --revision main <project> <outside> goes on', dsh('git clone --revision main /p /tmp/copy'), null],
  ['SD-5 control: git clone . <outside> (. names no new folder) goes on', dsh('git clone . /tmp/copy'), null],
  ['SD-5 control: git clone .. <outside> from docs (.. names no new folder) goes on', dsh('git clone .. /tmp/copy', { cwd: '/p/docs' }), null],
  // added by fix pass 3 (P3-7): with a folder given, no folder is made from the URL, so . and .. matter only alone
  ['SD-5 control: git clone . (no folder given; . names no new folder) goes on', dsh('git clone .'), null],
  // SD-6: a folder target that holds a worktree of this repository asks (git worktree list, from the project folder)
  ['SD-6 rm -rf .gate (it holds the worktree /p/.gate/x): ask', dsh('rm -rf .gate'), ASK, {}, /\.gate \(it holds the worktree \/p\/\.gate\/x\)/],
  ['SD-6 rm -r .gate: ask', dsh('rm -r .gate'), ASK],
  ['SD-6 rm -rf /p/.gate (absolute): ask', dsh('rm -rf /p/.gate'), ASK],
  ['SD-6 mv .gate x: ask', dsh('mv .gate x'), ASK],
  ['SD-6 an agent: rm -rf .gate: deny', dsh('rm -rf .gate', { agent_id: 'a1' }), DENY],
  ['SD-6 rm -rf /home/u/wts (outside any repository) holding a worktree of this one: ask', dsh('rm -rf /home/u/wts'), ASK,
    { worktrees: ['/p', '/home/u/wts/a'] }, /it holds the worktree \/home\/u\/wts\/a/],
  ['SD-6 git cannot list the worktrees: ask', dsh('rm -rf docs/research'), ASK, { wtFail: true }, /could not list this repository's worktrees/],
  ['SD-6 control: rm -rf .gate/empty (no worktree in it) goes on', dsh('rm -rf .gate/empty'), null],
  ['SA-6 rm -rf / with no worktree listed (the ancestor check alone): ask', dsh('rm -rf /'), ASK, { worktrees: [] }, /holds the project folder/],
];
for (const [name, input, want, opts, re] of DAY_CASES) {
  test(`day (kit-v3-3) ${want ?? 'goes on'}: ${name}`, async () => {
    const d = await decide(input, dayIo(opts));
    assert.equal(kindOf(d), want, `${name}: ${JSON.stringify(d)}`);
    if (re) assert.match(d.ask ?? d.deny, re);
  });
}
test('T17 pr-advisor git switch --detach origin/main goes on (unchanged)', async () => {
  assert.equal(await decide(dsh('git switch --detach origin/main'), dayIo({ role: PRA })), null);
});
test("A23 every role, A1-A3 (Edit/Write a kit file): today's deny reasons", async () => {
  for (const role of [NIGHT, PRA]) {
    for (const [tool, f] of [['Edit', '/p/.claude/hooks/guard.mjs'], ['Write', '/p/CLAUDE.md'], ['Edit', '/p/tools/gate.mjs']]) {
      const d = await decide({ hook_event_name: 'PreToolUse', tool_name: tool, tool_input: { file_path: f }, cwd: '/p' }, dayIo({ role }));
      assert.match(d?.deny ?? '', /is a rules, CI or gate file; those change only in a daytime kit change-set/, `${role} ${tool} ${f}`);
    }
  }
});
test('A24 PermissionRequest in a day session: no decision', async () => {
  assert.equal(await decide({ hook_event_name: 'PermissionRequest', tool_name: 'Bash', tool_input: {} }, dayIo()), null);
});
test('A26 with a role set, decide() never returns an ask (every case of both tables, each role)', async () => {
  for (const role of [NIGHT, PRA, 'nightshift']) {
    for (const [name, input, , opts] of DAY_CASES) {
      const d = await decide(input, dayIo({ ...opts, role }));
      assert.equal(d?.ask, undefined, `${role}: ${name}`);
    }
    for (const [name, , input, , opts] of CASES) {
      const d = await decide(input, fakeIo({ ...opts, role }));
      assert.equal(d?.ask, undefined, `${role}: ${name}`);
    }
  }
});

// A37 (C5): guard.mjs prints an ask only on PreToolUse with no role. Seam: a copy of the real guard.mjs beside a fake
// routine-guard.mjs whose decide() always answers { ask } (the launcher() helper of section 3).
const ASKING = 'export const decide = () => ({ ask: "fake ask" }); export const realIo = () => ({});';
const A37_CHECKS = (run) => {
  const day = JSON.parse(run(ASKING, preTool, '').stdout).hookSpecificOutput;
  assert.deepEqual([day.permissionDecision, day.permissionDecisionReason], ['ask', 'fake ask'], 'day PreToolUse: an ask');
  for (const role of [PRA, NIGHT]) {
    const r = JSON.parse(run(ASKING, preTool, role).stdout).hookSpecificOutput;
    assert.deepEqual([r.permissionDecision, r.permissionDecisionReason], ['deny', 'fake ask'], `${role}: an ask fails closed as a deny`);
  }
  const cfg = JSON.parse(run(ASKING, JSON.stringify({ hook_event_name: 'ConfigChange', source: 'project_settings' }), '').stdout);
  assert.deepEqual([cfg.decision, cfg.reason], ['block', 'fake ask'], 'ConfigChange, no role: not an ask');
  const perm = JSON.parse(run(ASKING, JSON.stringify({ hook_event_name: 'PermissionRequest', tool_name: 'Bash', tool_input: {} }), '').stdout);
  assert.equal(perm.hookSpecificOutput.decision.behavior, 'deny', 'PermissionRequest, no role: not an ask');
};
test('A37 launcher: an ask prints as an ask only on PreToolUse with no role; otherwise a deny (C5)', () => A37_CHECKS(launcher));
function launcherWith(guardSrc, rules, stdin, role) {
  const dir = mkdtempSync(join(tmpdir(), 'guard-launch-m-'));
  try {
    writeFileSync(join(dir, 'guard.mjs'), guardSrc);
    writeFileSync(join(dir, 'routine-guard.mjs'), rules);
    const env = { ...process.env, CLAUDE_PROJECT_DIR: dir };
    if (role) env.SHAMBLE_ROLE = role; else delete env.SHAMBLE_ROLE;
    return spawnSync(process.execPath, [join(dir, 'guard.mjs')], { input: stdin, env, encoding: 'utf8' });
  } finally { rmSync(dir, { recursive: true, force: true }); }
}
test("mutation: guard.mjs's fail-closed ask is needed by A37", () => {
  const src = readFileSync(join(HERE, 'guard.mjs'), 'utf8');
  const from = "print(d.ask, false, event === 'PreToolUse' && !role)";
  assert.equal(src.split(from).length - 1, 1, 'the target text occurs exactly once');
  const mutated = src.replace(from, "print(d.ask, false, event === 'PreToolUse')");
  assert.throws(() => A37_CHECKS((rules, stdin, role) => launcherWith(mutated, rules, stdin, role)), /fails closed/);
});

// Mutation runs for section 5: each day rule, switched off in a copy, makes at least one day case fail.
const DAY_MUTANTS = [
  ['the day ask (shell writes)', 'first ??= ask(`kit and gate files ask first in a day session: ${rel} (${what})`);', ''],
  ['the agent deny keys on agent_id (agent_type instead)', 'const agent = !!input.agent_id;', 'const agent = !!input.agent_type;'],
  ["kitPath: the path's own git top first (projectRel first instead)", 'const g = gitPlace(abs, io);\n  if (!projectCommon',
    '{ const pr0 = projectRel(p, cwd, io); if (pr0 !== null) return protectedRel(pr0) ? pr0 : null; }\n  const g = gitPlace(abs, io);\n  if (!projectCommon'],
  ['folder targets', "if (rel === '.' || rel === '.claude' || rel === '.github' || rel === 'tools') return rel;", ''],
  ['unresolvable targets ask', "if (/[$`*?[{]/.test(raw)) {", 'if (false) {'],
  // fix round 2 (SA-8) replaced the prefix-only test this row switched off: the line check, its raw-text and word halves
  ['SA-8 a line naming GIT_DIR and the like asks', 'if (!role && (GIT_PLACE_VARS.test(command) ||', 'if (!role && false && (GIT_PLACE_VARS.test(command) ||'],
  ['SA-8 the raw text half', 'if (!role && (GIT_PLACE_VARS.test(command) ||', 'if (!role && (false ||'],
  ['SA-8 the word half', 'cmds.some((c) => [...c.argv, ...c.redirects, ...c.reads].some((w) => GIT_PLACE_VARS.test(w))))) {', 'false)) {'],
  ['SA-8 GIT_COMMON_DIR and GIT_INDEX_FILE', 'const GIT_PLACE_VARS = /GIT_(DIR|WORK_TREE|COMMON_DIR|INDEX_FILE)/;', 'const GIT_PLACE_VARS = /GIT_(DIR|WORK_TREE)=/;'],
  ['the scan goes on after an ask', 'dayAsk ??= w;', 'if (w) return w;'],
  ['win32 folds case', "const foldsCase = (io) => ['win32', 'darwin'].includes(io.platform ?? process.platform);",
    'const foldsCase = () => false;'],
  ['the main folder only', 'if (!sameDir(gitTop(vcwd, io), io.projectDir, io)) return null;', ''],
  ['switch: no name exempt (a stale main)', "const h = sha ? protectedHit([ORIGIN_MAIN, sha, '--'], vcwd, io) : { failed: true };",
    "const h = pos[0] === 'main' ? null : sha ? protectedHit([ORIGIN_MAIN, sha, '--'], vcwd, io) : { failed: true };"],
  ['switch -c: the start point', 'if (pos[0] && !isOriginMain(pos[0], vcwd, io)) {', 'if (false) {'],
  ["switch -c: HEAD's protected files", "const h = protectedHit([ORIGIN_MAIN, 'HEAD', '--'], vcwd, io);", 'const h = null;'],
  ['merge forms', "const t = args.length === 2 && args[0] === '--ff-only' ? args[1] : args.length === 1 ? args[0] : null;",
    "const t = 'origin/main';"],
  ['pull form', "if (args.join(' ') !== '--ff-only origin main') why =", 'if (false) why ='],
  ['gh pr checkout', "if ((sub === 'pr' && sub2 === 'checkout') || sub === 'co') return mainFolderMove(", 'if (false) return mainFolderMove('],
  ['the agent deny on a move', 'return input.agent_id ? deny(', 'return false ? deny('],
  // step 4b
  ['R1 the day ask (file tools)', 'return ask(`kit and gate files ask first in a day session: ${rel}`);', 'return null;'],
  ['R1 dayEdit is wired into decide()', 'if (!role) return dayEdit(tool, ti, cwd, io, input);', 'if (!role) return null;'],
  ['R1 the agent deny on a file tool', 'if (input.agent_id) return deny(`${KIT_AGENT}: ${rel}`);', ''],
  ['R2 plain merge unguarded', '} else if (args.length === 1) {', '} else if (false) {'],
  ['R4 agent ask on an unresolvable target', 'if (agent) return deny(`${KIT_AGENT}; an agent spells', 'if (false) return deny(`${KIT_AGENT}; an agent spells'],
  ['S-1 the switch target reaches git as a word, not its SHA', "const h = sha ? protectedHit([ORIGIN_MAIN, sha, '--'], vcwd, io) : { failed: true };",
    "const h = protectedHit([ORIGIN_MAIN, pos[0], '--'], vcwd, io);"],
  ['S-2 merge: origin/main by name', 'if (t === null || !isOriginMain(t, vcwd, io)) {', "if (t !== 'origin/main') {"],
  ['S-2 switch -c: the start point by name', 'if (pos[0] && !isOriginMain(pos[0], vcwd, io)) {', "if (pos[0] && pos[0] !== 'origin/main') {"],
  ['S-2 origin/main resolved by its short name', 'const o = commitOf(ORIGIN_MAIN, vcwd, io);', "const o = commitOf('origin/main', vcwd, io);"],
  ['S-3 darwin folds case', "['win32', 'darwin'].includes(io.platform ?? process.platform)", "['win32'].includes(io.platform ?? process.platform)"],
  // fix round R-a..R-e (replacing the S-4 and F5 rows whose code the ruling deleted or rewrote)
  ['S-4 pushd and popd are followed', "if (prog !== 'cd' && prog !== 'pushd' && prog !== 'popd') {", "if (prog !== 'cd') {"],
  ['S-4 the forks flag', "(c === '&' && s[i + 1] !== '>')) forks = true;", "(c === '&' && s[i + 1] !== '>')) forks = false;"],
  ['R-a a relative write while the folder is unknown asks', 'if (vcwd === null && !isAbsolute(t)) {', 'if (false) {'],
  ['R-a the unknown state reaches the write check', 'const w = dayShellWrites(c, argv, dirs.known ? vcwd : null, io, input);',
    'const w = dayShellWrites(c, argv, vcwd, io, input);'],
  ['R-a the main-folder check reads an unknown folder as the project folder', 'if (!dirs.known) vcwd = io.projectDir;', ''],
  ['R-a an agent is refused', "if (agent) return deny(`${KIT_AGENT}; after a cd", "if (false) return deny(`${KIT_AGENT}; after a cd"],
  ['R-b only absolute, ./ and ../ targets', '(isAbsolute(t) || /^\\.\\.?(\\/|$)/.test(t))', 'true'],
  ['R-b no variable or glob', "&& !/[$`*?[{]/.test(t);", ';'],
  ['R-b one target', 'const fixed = a.length === 1 &&', 'const fixed = a.length >= 1 &&'],
  ['R-b the folder exists and can be entered', '!(io.isDir ? io.isDir(to) : io.exists(to))', 'false'],
  ['R-b no option but -L', "if (prog === 'cd') while (/^-L+$/.test(a[0] ?? '')) a = a.slice(1);",
    "if (prog === 'cd') while (/^-[LPe@]+$/.test(a[0] ?? '')) a = a.slice(1);"],
  ['R-b a relative target needs a known folder', '(here !== null || isAbsolute(t))', 'true'],
  ['R-c the leading run', "if (!dirs.lead || forked || c.sep === '||' ||", "if (forked || c.sep === '||' ||"],
  ['R-c a cd after ||', "forked || c.sep === '||' ||", 'forked ||'],
  ['R-c the || separator is recorded', "endCmd('||')", 'endCmd()'],
  ['R-c the bare word, not a path', "argv[0] !== prog || !prefix", '!prefix'],
  ['R-c only the listed words reach a cd', '!prefix.every((w) => CD_REACH.has(w))', 'false'],
  ['R-c command -v does not run the cd', "'command', '-p', '--']);", "'command', '-p', '-v', '--']);"],
  ['R-c a loop keyword does not reach a cd', "new Set(['if', '{',", "new Set(['while', 'if', '{',"],
  ["R-c unwrap strips command's options", "while (/^-[pvV]+$|^--$/.test(a[0] ?? '')) a = a.slice(1);", ''],
  ['R-c a bare keyword runs nothing', 'prefix.every((w) => WRAPPERS.has(w))) return vcwd;', 'false) return vcwd;'],
  ['R-c a pushd to an unknown target makes the stack unknown', "if (prog === 'pushd') dirs.stack = null;", ''],
  ['R-c popd with an empty stack', 'if (a.length || !dirs.stack?.length) {', 'if (a.length || !dirs.stack) {'],
  ['R-c popd to an unknown entry', 'if (top === null) return unknown();', 'if (top === null) return vcwd;'],
  ['R-d a move in a forked line is unknown', "if (!dirs.lead || forked ||", 'if (!dirs.lead ||'],
  ['R-e exec', "if (prefix.includes('exec') ||", 'if ('],
  ['R-e source, . and eval', "['source', '.', 'eval'].includes(prog) ||", ''],
  ['R-e a program word that starts with - or holds $', "|| /^-|[$`]/.test(argv[0] ?? '')) return unknown();", ') return unknown();'],
  ['F6 switch resolves refs/heads/<name>', 'const sha = detach ? commitOf(pos[0], vcwd, io) : branchCommit(pos[0], vcwd, io);',
    'const sha = commitOf(pos[0], vcwd, io);'],
  ['F6 --detach keeps the general resolve', 'const sha = detach ? commitOf(pos[0], vcwd, io) : branchCommit(pos[0], vcwd, io);',
    'const sha = branchCommit(pos[0], vcwd, io);'],
  ['F6 the short name must name the branch alone', 'full.code === 0 && full.out.trim() === `refs/heads/${name}` ?', 'true ?'],
  ['F6 --detach and -d', "if (t === '--detach' || t === '-d') { detach = true; continue; }", ''],
  ['F6 --no-detach cannot be told', "if (t.startsWith('--no-d')) { odd = true; continue; }", ''],
  // fix round 2: SA-5, the sed and perl targets, F-b, SA-6
  ['SA-5 any prefix of --in-place from --i on', "(w.split('=')[0].length >= 3 && '--in-place'.startsWith(w.split('=')[0]))", '/^--in-place(=|$)/.test(w)'],
  ['SA-5 the shortest prefix is --i', "w.split('=')[0].length >= 3 &&", "w.split('=')[0].length >= 5 &&"],
  ['sed/perl: every positional word is a target', 'return edits ? pos : [];', 'return edits ? pos.slice(1) : [];'],
  ['F-b git worktree add writes its folder', "if (prog === 'git' && a[0] === 'worktree' && a[1] === 'add') {", 'if (false) {'],
  ['F-b worktree add: the values of -b, -B and --reason', "if (w === '-b' || w === '-B' || w === '--reason') { i++; continue; }", ''],
  ['F-b git clone writes its folder', "if (prog === 'git' && a[0] === 'clone') {", 'if (false) {'],
  ['F-b clone: the values of its options', 'if (CLONE_VALUE.has(w)) { i++; continue; }', ''],
  // fix pass 2 (SD-5) rewrote the line this row switched off (the folder made from the URL): retargeted to its new form
  // retargeted by fix pass 3 (P3-7), which rewrote the made line: the mutant still drops every folder git would make
  ["F-b clone: no folder given, the URL's last part",
    'const made = [...(pos.length === 1 ? [pos[0]] : []), ...(unknown && pos.length > 1 ? [pos[1]] : [])]', 'const made = []'],
  ['F-b clone: --separate-git-dir', "if (w === '--separate-git-dir') { gitDir.push(a[++i] ?? ''); continue; }", ''],
  ['SA-6 an ancestor of the project folder', "if (relIn(fold(abs, io), fold(realOf(io.projectDir, io), io)) !== null) return", 'if (false) return'],
  ['SA-6 tracked kit paths in the folder', 'return hit ? `${p} (it holds ${hit})` : null;', 'return null;'],
  ['SA-6 git failing asks', 'if (ls.code !== 0) return `${p} (the guard could not list its files)`;', 'if (ls.code !== 0) return null;'],
  ['SA-6 only folders are listed', 'if (!(io.isDir ? io.isDir(abs) : io.exists(abs))) return null;', ''],
  // fix pass 2, the day paths
  ['SD-3 a switch after an earlier git command asks', "else if (line.gitBefore && pos[0] !== 'origin/main') {", 'else if (false) {'],
  ['SD-3 a merge after an earlier git command asks', "if (t !== null && line.gitBefore && t !== 'origin/main') {", 'if (false) {'],
  ['SD-3 bash() records an earlier git command', 'line.gitBefore = true; }', '}'],
  ['SD-3 git() passes the line to mainFolderMove', 'return mainFolderMove(sub, args, vcwd, io, input, line);', 'return mainFolderMove(sub, args, vcwd, io, input);'],
  ['SD-3 switch: only the word origin/main goes on', "pos[0] !== 'origin/main') {", "!isOriginMain(pos[0] ?? 'HEAD', vcwd, io)) {"],
  ['SD-3 merge: only the word origin/main goes on', "t !== 'origin/main') {", '!isOriginMain(t, vcwd, io)) {'],
  ['SD-4 a cd after && while the folder is unknown', "if (c.sep === '&&' && !dirs.known) {", 'if (false) {'],
  ['SD-4 a skipped pushd or popd makes the stack unknown', "if (prog !== 'cd') dirs.stack = null;", ''],
  ['SD-4 ! does not reach a cd', "const CD_REACH = new Set(['if', '{', 'time',", "const CD_REACH = new Set(['if', '{', '!', 'time',"],
  ['V-1 function', "if (['function', 'trap', 'alias'].includes(prog)) return unknown();", "if (['trap', 'alias'].includes(prog)) return unknown();"],
  ['V-1 trap', "if (['function', 'trap', 'alias'].includes(prog)) return unknown();", "if (['function', 'alias'].includes(prog)) return unknown();"],
  ['V-1 alias', "if (['function', 'trap', 'alias'].includes(prog)) return unknown();", "if (['function', 'trap'].includes(prog)) return unknown();"],
  ['V-1 command words only (not any word)', "if (['function', 'trap', 'alias'].includes(prog)) return unknown();",
    "if (argv.some((w) => ['function', 'trap', 'alias'].includes(w))) return unknown();"],
  ['SD-5 every positional word after the first', 'return [...pos.slice(1), ...made, ...gitDir];', 'return [...pos.slice(1, 2), ...made, ...gitDir];'],
  // retargeted by fix pass 3 (P3-7), which keeps of this rule the folder made from the second positional word after an
  // option the guard does not know (the shift SD-5 guards against)
  ['SD-5 the folder made from a later positional word (P3-7: the second, after an unknown option)',
    '...(unknown && pos.length > 1 ? [pos[1]] : [])', '...[]'],
  ['SD-5 . and .. name no new folder', "filter((d) => d && d !== '.' && d !== '..')", 'filter(Boolean)'],
  ['SD-5 --revision takes a value', "'--ref-format', '--revision']);", "'--ref-format']);"],
  ['SD-6 a folder holding a worktree of this repository', 'if (wt !== undefined) return', 'if (false) return'],
  ['SD-6 at or under the target', 'relIn(fold(abs, io), fold(realOf(w, io), io)) !== null', "relIn(fold(abs, io), fold(realOf(w, io), io)) === '.'"],
  ['SD-6 git failing to list the worktrees asks', "if (wts.code !== 0) return `${p} (the guard could not list this repository's worktrees)`;",
    'if (wts.code !== 0) return null;'],
  ['SD-6 before the not-in-a-repository return', "  const wts = io.git(['worktree',", "  if (!g) return null;\n  const wts = io.git(['worktree',"],
  ['SD-6 run from the project folder', "io.git(['worktree', 'list', '--porcelain', '-z'], io.projectDir);", "io.git(['worktree', 'list', '--porcelain', '-z'], abs);"],
];
test('mutation runs (kit-v3-3 day rules): every rule is needed by some day case', async () => {
  const src = readFileSync(join(HERE, 'routine-guard.mjs'), 'utf8');
  const dir = mkdtempSync(join(tmpdir(), 'guard-mut-day-'));
  const survivors = [];
  for (const [name, input, want, opts] of DAY_CASES) assert.equal(kindOf(await decide(input, dayIo(opts))), want, `unmutated rules: ${name}`);
  try {
    for (const [i, [label, from, to]] of DAY_MUTANTS.entries()) {
      assert.equal(src.split(from).length - 1, 1, `mutant "${label}": its target text occurs exactly once`);
      const file = join(dir, `d${i}.mjs`);
      writeFileSync(file, src.replace(from, to));
      const m = await import(pathToFileURL(file).href);
      let caught = false;
      for (const [, input, want, opts] of DAY_CASES) {
        let d;
        try { d = await m.decide(input, dayIo(opts)); } catch { d = 'threw'; }
        if ((d === 'threw' ? 'threw' : kindOf(d)) !== want) { caught = true; break; }
      }
      if (!caught) survivors.push(label);
    }
  } finally { rmSync(dir, { recursive: true, force: true }); }
  assert.deepEqual(survivors, [], `mutants no day case catches: ${survivors.join('; ')}`);
});

// --- fix round 2 (step 8 reviews, SA-2's routine half): origin/main and main cannot be repointed by a routine ---
// [name, role, input, expected kind, dayIo options, reason pattern]. dayIo's shadow: origin/main resolves to another
// commit than refs/remotes/origin/main (a local branch or tag of that name), as git reads it.
const SHADOWED = /origin\/main does not name refs\/remotes\/origin\/main's commit/;
const NIGHT_NPM_CMD = 'npm ci --prefix tools --ignore-scripts --no-audit --no-fund';
export const ROUTINE_FIX_CASES = [
  // the four sites: each first checks that origin/main is refs/remotes/origin/main's commit
  ['Rn4 night: node tools/sim.js, origin/main shadowed', NIGHT, dsh('node tools/sim.js'), DENY, { shadow: true }, SHADOWED],
  ['Rn4 control: node tools/sim.js', NIGHT, dsh('node tools/sim.js'), null],
  ['night: node tools/sim.js, refs/remotes/origin/main does not resolve', NIGHT, dsh('node tools/sim.js'), DENY, { noRemoteMain: true }, /can't resolve refs\/remotes\/origin\/main/],
  ['night: npm ci, origin/main shadowed', NIGHT, dsh(NIGHT_NPM_CMD), DENY, { shadow: true, npmrc: false }, SHADOWED],
  ['night: npm ci control', NIGHT, dsh(NIGHT_NPM_CMD), null, { npmrc: false }],
  ['Rp4 night: push its branch, origin/main shadowed', NIGHT, dsh('git push origin cs/x'), DENY, { shadow: true, branch: 'cs/x' }, SHADOWED],
  ['Rp4 control: push its branch', NIGHT, dsh('git push origin cs/x'), null, { branch: 'cs/x' }],
  ['Rs4 pr-advisor: switch --detach origin/main, shadowed', PRA, dsh('git switch --detach origin/main'), DENY, { shadow: true }, SHADOWED],
  ['Rs4 pr-advisor: switch --detach origin/main, no full ref', PRA, dsh('git switch --detach origin/main'), DENY, { noRemoteMain: true }],
  // fetch and pull: no refspec with ':' or a leading '+', no --refmap (Ri1, Re3)
  ['Ri1 night: fetch origin cs/evil:refs/remotes/origin/main', NIGHT, dsh('git fetch origin cs/evil:refs/remotes/origin/main'), DENY, {}, /no refspec with ':'/],
  ['Re3 pr-advisor: fetch origin cs/evil:refs/remotes/origin/main', PRA, dsh('git fetch origin cs/evil:refs/remotes/origin/main'), DENY],
  ['night: fetch origin +main', NIGHT, dsh('git fetch origin +main'), DENY],
  ['night: fetch origin cs/evil --refmap=...', NIGHT, dsh('git fetch origin cs/evil --refmap=refs/heads/cs/evil:refs/remotes/origin/main'), DENY],
  ['night: fetch origin cs/evil --refmap ... (two words)', NIGHT, dsh('git fetch origin cs/evil --refmap refs/heads/x'), DENY],
  ['night: fetch control: git fetch origin cs/x', NIGHT, dsh('git fetch origin cs/x'), null],
  // new branch names (Re1, Re2): not origin/, refs/, remotes/, HEAD or main
  ['Re1 night: git branch origin/main cs/evil', NIGHT, dsh('git branch origin/main cs/evil'), DENY, {}, /no new branch named origin\/main/],
  ['Re2 pr-advisor: git branch origin/main origin/cs/evil', PRA, dsh('git branch origin/main origin/cs/evil'), DENY],
  ['night: git branch -v refs/remotes/origin/main (-v is no list mode)', NIGHT, dsh('git branch -v refs/remotes/origin/main'), DENY],
  ['night: git branch -t remotes/origin/main', NIGHT, dsh('git branch -t remotes/origin/main'), DENY],
  ['night: git branch main origin/cs/x', NIGHT, dsh('git branch main origin/cs/x'), DENY],
  ['night: git switch -c origin/main cs/evil', NIGHT, dsh('git switch -c origin/main cs/evil'), DENY],
  ['night: git switch --create=HEAD', NIGHT, dsh('git switch --create=HEAD'), DENY],
  ['night: git switch -corigin/x (attached)', NIGHT, dsh('git switch -corigin/x'), DENY],
  ['night: git switch --orphan origin/main', NIGHT, dsh('git switch --orphan origin/main'), DENY],
  ['night: git switch -c main cs/evil', NIGHT, dsh('git switch -c main cs/evil'), DENY],
  ['night: git switch --orphan main', NIGHT, dsh('git switch --orphan main'), DENY],
  ['night: git switch origin/main (git may create it from a remote branch)', NIGHT, dsh('git switch origin/main'), DENY],
  ['night: git worktree add -b origin/main .gate/w', NIGHT, dsh('git worktree add -b origin/main .gate/w'), DENY],
  ['night: git worktree add -Bmain .gate/w (attached)', NIGHT, dsh('git worktree add -Bmain .gate/w'), DENY],
  ['night: git worktree add .gate/main (the path names the branch)', NIGHT, dsh('git worktree add .gate/main'), DENY],
  ['night: branch control: git branch cs/y origin/main', NIGHT, dsh('git branch cs/y origin/main'), null],
  ['night: branch control: git branch --contains origin/main', NIGHT, dsh('git branch --contains origin/main'), null],
  ['night: branch control: git branch -a', NIGHT, dsh('git branch -a'), null],
  ['night: branch control: git branch -u origin/main', NIGHT, dsh('git branch -u origin/main'), null],
  ['night: branch control: git switch -c cs/y origin/main', NIGHT, dsh('git switch -c cs/y origin/main'), null],
  ['night: branch control: git switch main (an existing name)', NIGHT, dsh('git switch main'), null],
  ['night: branch control: git worktree add .gate/b origin/main', NIGHT, dsh('git worktree add .gate/b origin/main'), null],
  ['night: branch control: git worktree add --detach .gate/main', NIGHT, dsh('git worktree add --detach .gate/main'), null],
  // symbolic-ref (mine: it can repoint refs/remotes/origin/main) and git remote (writes remote.origin.*) only read
  ['night: git symbolic-ref refs/remotes/origin/main refs/heads/cs/evil', NIGHT, dsh('git symbolic-ref refs/remotes/origin/main refs/heads/cs/evil'), DENY, {}, /only reads HEAD/],
  ['pr-advisor: git symbolic-ref HEAD refs/heads/cs/evil', PRA, dsh('git symbolic-ref HEAD refs/heads/cs/evil'), DENY],
  ['night: git symbolic-ref --delete HEAD', NIGHT, dsh('git symbolic-ref --delete HEAD'), DENY],
  ['night: git symbolic-ref -m why HEAD refs/heads/x', NIGHT, dsh('git symbolic-ref -m why HEAD refs/heads/x'), DENY],
  ['night: symbolic-ref control: --short HEAD', NIGHT, dsh('git symbolic-ref --short HEAD'), null],
  ['pr-advisor: symbolic-ref control: -q HEAD', PRA, dsh('git symbolic-ref -q HEAD'), null],
  ['night: git remote set-url origin /tmp/evil', NIGHT, dsh('git remote set-url origin /tmp/evil'), DENY, {}, /git remote only reads/],
  ['night: git remote add evil /tmp/evil', NIGHT, dsh('git remote add evil /tmp/evil'), DENY],
  ['night: git remote set-branches origin x', NIGHT, dsh('git remote set-branches origin x'), DENY],
  ['pr-advisor: git remote rename origin o2', PRA, dsh('git remote rename origin o2'), DENY],
  ['night: remote control: git remote -v', NIGHT, dsh('git remote -v'), null],
  ['pr-advisor: remote control: git remote get-url origin', PRA, dsh('git remote get-url origin'), null],
  ['night: remote control: git remote', NIGHT, dsh('git remote'), null],
  ['night: git branch --edit-description (writes git config, waits for an editor)', NIGHT, dsh('git branch --edit-description'), DENY],
  // day sessions are not changed by these rules (L12: a day fetch can repoint origin/main; Fi1-Fi3)
  ['Fi1 day: fetch origin cs/evil:refs/remotes/origin/main goes on (L12)', DAY, dsh('git fetch origin cs/evil:refs/remotes/origin/main'), null],
  ['Fi3 day: fetch ...:refs/remotes/origin/main && merge --ff-only origin/main goes on (L12)', DAY,
    dsh('git fetch origin cs/evil:refs/remotes/origin/main && git merge --ff-only origin/main'), null],
  ['day: git branch origin/main cs/x goes on (unchanged)', DAY, dsh('git branch origin/main cs/x'), null],
  // step 9 (WT-NIGHT (a)), rows added only: the night's switch check also refuses a target it cannot resolve, which
  // hid the name rule in the switch rows above; these targets resolve and equal main in kit files, so the name rule
  // alone refuses them
  ['night: git switch -c main cs/ok (cs/ok equals main in kit files)', NIGHT, dsh('git switch -c main cs/ok'), DENY,
    { diffs: { 'cs/ok': '' } }, /no new branch named main/],
  ['night: git switch remotes/x (the branch remotes/x equals main in kit files)', NIGHT, dsh('git switch remotes/x'), DENY,
    { diffs: { 'remotes/x': '' } }, /no new branch named remotes\/x/],
];
for (const [name, role, input, want, opts, re] of ROUTINE_FIX_CASES) {
  test(`routine (fix round 2) ${want ?? 'goes on'}: ${name}`, async () => {
    const d = await decide(input, dayIo({ ...opts, role }));
    assert.equal(kindOf(d), want, `${name}: ${JSON.stringify(d)}`);
    if (re) assert.match(d.ask ?? d.deny, re);
  });
}
const ROUTINE_FIX_MUTANTS = [
  ['SA-2 npmCheck checks origin/main', 'const shadow = originMainCheck(io.projectDir, io, role); // SA-2', 'const shadow = null;'],
  ['SA-2 interpreter checks origin/main', 'const shadow = originMainCheck(root, io, role);', 'const shadow = null;'],
  ['SA-2 protectedInBranch checks origin/main', "const shadow = originMainCheck(vcwd, io, 'night-shift'); // SA-2", 'const shadow = null;'],
  ["SA-2 the PR Advisor's switch checks origin/main", "if (sub === 'switch' && role === 'pr-advisor') { const s = originMainCheck(vcwd, io, role); if (s) return deny(s); }", ''],
  ['SA-2 origin/main compared by commit', "return commitOf('origin/main', vcwd, io) === full ? null", 'return true ? null'],
  ['SA-2 the full ref must resolve', 'if (full === null) return `routine (${role}): can\'t resolve', 'if (false) return `routine (${role}): can\'t resolve'],
  ['SA-2 fetch: no refspec rule', "if ((sub === 'fetch' || sub === 'pull') && args.some(", "if (false && args.some("],
  ["SA-2 fetch: ':'", "w.includes(':') || w.startsWith('+')", "w.startsWith('+')"],
  ["SA-2 fetch: a leading '+'", "w.includes(':') || w.startsWith('+')", "w.includes(':')"],
  ['SA-2 fetch: --refmap', " || /^--refmap(=|$)/.test(w))) {", ')) {'],
  ['SA-2 new branch names refused', 'if (bad !== undefined) return deny(`routine (${role}): no new branch named', 'if (false) return deny(`routine (${role}): no new branch named'],
  ['SA-2 names: HEAD and main', "|| n === 'HEAD' || n === 'main');", ');'],
  ['SA-2 names: git branch', "if (sub === 'branch') return mode || !pos.length ? [] : [pos[0]];", "if (sub === 'branch') return [];"],
  ['SA-2 names: git branch list and upstream modes', "if (sub === 'branch' && (/^--(list|all|", "if (false && (/^--(list|all|"],
  ['SA-2 names: switch -c and --orphan', "if (sub === 'switch' && /^(-c|--create|--orphan)$/.test(t)) {", 'if (false) {'],
  ['SA-2 names: switch -cX and --create=X', "if (sub === 'switch' && (/^-c./.test(t) || /^--(create|orphan)=/.test(t))) {", 'if (false) {'],
  ['SA-2 names: a plain git switch', "return made || detach || !/^(origin\\/|refs\\/|remotes\\/)/.test(pos[0] ?? '') ? names : [pos[0]];", 'return names;'],
  ['SA-2 names: worktree add -b/-B', "if (sub === 'worktree' && /^-[bB]$/.test(t)) {", 'if (false) {'],
  ['SA-2 names: worktree add -bX', "if (sub === 'worktree' && /^-[bB]./.test(t)) {", 'if (false) {'],
  ["SA-2 names: worktree add's path", "made || detach || pos.length !== 2 ? names : [basename(pos[1])];", 'names;'],
  ['SA-2 symbolic-ref only reads HEAD', "if (sub === 'symbolic-ref' && !(rest.length === 1", "if (false && !(rest.length === 1"],
  ['SA-2 symbolic-ref: only listed options', "args.every((w) => w === 'HEAD' || /^(-q|--quiet|--short|--no-short|--recurse|--no-recurse)$/.test(w))", 'true'],
  ['SA-2 git remote only reads', "if (sub === 'remote' && !(args.length === 0", "if (false && !(args.length === 0"],
  ['SA-2 git branch --edit-description', "if (sub === 'branch' && args.includes('--edit-description')) return deny(", 'if (false) return deny('],
];
test('mutation runs (fix round 2, SA-2 routine rules): every rule is needed by some routine case', async () => {
  const src = readFileSync(join(HERE, 'routine-guard.mjs'), 'utf8');
  const dir = mkdtempSync(join(tmpdir(), 'guard-mut-sa2-'));
  const survivors = [];
  for (const [name, role, input, want, opts] of ROUTINE_FIX_CASES) assert.equal(kindOf(await decide(input, dayIo({ ...opts, role }))), want, `unmutated rules: ${name}`);
  try {
    for (const [i, [label, from, to]] of ROUTINE_FIX_MUTANTS.entries()) {
      assert.equal(src.split(from).length - 1, 1, `mutant "${label}": its target text occurs exactly once`);
      const file = join(dir, `r${i}.mjs`);
      writeFileSync(file, src.replace(from, () => to));
      const m = await import(pathToFileURL(file).href);
      let caught = false;
      for (const [, role, input, want, opts] of ROUTINE_FIX_CASES) {
        let d;
        try { d = await m.decide(input, dayIo({ ...opts, role })); } catch { d = 'threw'; }
        if ((d === 'threw' ? 'threw' : kindOf(d)) !== want) { caught = true; break; }
      }
      if (!caught) survivors.push(label);
    }
  } finally { rmSync(dir, { recursive: true, force: true }); }
  assert.deepEqual(survivors, [], `mutants no routine case catches: ${survivors.join('; ')}`);
});

// --- step 4b (kit-v3-3): the security review of f22c4df (S-1, S-2) and fix 1, shown on the guard's own git calls ---
test('S-1 no guard-run git command gets a word of the command that starts with "-"', async () => {
  for (const command of ['git switch -- --output=/tmp/x', 'git switch -c n -- --output=/tmp/x',
    'git switch --detach -- --output=/tmp/x', 'git merge -- --output=/tmp/x', 'git merge --ff-only --output=/tmp/x']) {
    const calls = [];
    const d = await decide(dsh(command), dayIo({ calls, diffs: OTHER }));
    assert.equal(kindOf(d), ASK, `${command}: ${JSON.stringify(d)}`);
    const bad = calls.filter((c) => c.some((w) => w.includes('--output')));
    assert.deepEqual(bad, [], `${command}: the guard ran git with the command's word`);
  }
});
test('real git (step 4b): `--` after the ref (fix 1), no file written (S-1), origin/main by commit (S-2)', async () => {
  const dir = realpathSync(mkdtempSync(join(tmpdir(), 'guard-git-4b-')));
  const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
  const g = (...a) => spawnSync('git', a, { cwd: dir, env, encoding: 'utf8' });
  const prev = process.env.CLAUDE_PROJECT_DIR;
  try {
    g('init', '-q', '-b', 'main');
    mkdirSync(join(dir, 'docs'));
    writeFileSync(join(dir, 'docs', 'a.md'), 'a\n');
    writeFileSync(join(dir, 'CLAUDE.md'), 'x\n');
    g('add', '-A');
    g('commit', '-q', '-m', 'one');
    g('update-ref', 'refs/remotes/origin/main', 'HEAD');
    // fix 1: without `--`, git reads a folder name that is no ref as a path and lists nothing; with it, git fails
    const noDash = g('diff', '--name-only', 'origin/main', 'docs');
    assert.deepEqual([noDash.status, noDash.stdout], [0, ''], 'without --: exit 0, an empty list (the old fail-open)');
    assert.notEqual(g('diff', '--name-only', 'origin/main', 'docs', '--').status, 0, 'with --: git fails on a name that is no ref');
    process.env.CLAUDE_PROJECT_DIR = dir;
    const io = { ...realIo(), env: {} };
    const day = (command) => decide(dsh(command, { cwd: dir }), io);
    // S-1: the switch target is never handed to git as an option
    for (const [i, form] of ['git switch -- --output=', 'git switch -c n -- --output=', 'git switch --detach -- --output='].entries()) {
      const out = join(dir, `leak${i}.txt`);
      assert.equal(kindOf(await day(`${form}${out}`)), ASK, `${form}<path> asks`);
      assert.equal(existsSync(out), false, `${form}<path>: the guard wrote no file`);
    }
    // S-2: origin/main is the commit refs/remotes/origin/main names
    assert.equal(await day('git merge origin/main'), null, 'origin/main is the remote ref: a plain merge goes on');
    assert.equal(await day('git switch -c cs/n origin/main'), null, 'a new branch from origin/main goes on');
    g('switch', '-q', '-c', 'cs/k');
    writeFileSync(join(dir, 'CLAUDE.md'), 'y\n');
    g('commit', '-q', '-am', 'two');
    g('switch', '-q', 'main');
    g('branch', 'origin/main', 'cs/k'); // a local branch named origin/main, at an unreviewed commit
    for (const c of ['git merge origin/main', 'git merge --ff-only origin/main', 'git switch -c cs/n origin/main']) {
      assert.equal(kindOf(await day(c)), ASK, `${c}: a local origin/main elsewhere asks`);
    }
    assert.equal(await day('git merge --ff-only refs/remotes/origin/main'), null, 'the fully qualified ref goes on');
  } finally {
    if (prev === undefined) delete process.env.CLAUDE_PROJECT_DIR; else process.env.CLAUDE_PROJECT_DIR = prev;
    rmSync(dir, { recursive: true, force: true });
  }
});

// --- step 4c (kit-v3-3): the second security review of 2e82216, finding 6, on real git ---
// A branch and a tag share the name dup. The tag is at origin/main's commit, the branch one commit on, with CLAUDE.md
// changed. git reads the short name as the tag (rev-parse), but `git switch dup` moves to the branch.
test('real git (step 4c, finding 6): git switch <name> is checked as the branch refs/heads/<name>, never a same-name tag', async () => {
  const dir = realpathSync(mkdtempSync(join(tmpdir(), 'guard-git-4c-')));
  const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
  const g = (...a) => spawnSync('git', a, { cwd: dir, env, encoding: 'utf8' });
  const prev = process.env.CLAUDE_PROJECT_DIR;
  try {
    g('init', '-q', '-b', 'main');
    writeFileSync(join(dir, 'CLAUDE.md'), 'x\n');
    writeFileSync(join(dir, 'shamble-game.js'), 'a\n');
    g('add', '-A');
    g('commit', '-q', '-m', 'one');
    g('update-ref', 'refs/remotes/origin/main', 'HEAD');
    g('tag', 'dup');
    g('switch', '-q', '-c', 'dup');
    writeFileSync(join(dir, 'CLAUDE.md'), 'y\n');
    g('commit', '-q', '-am', 'two: a kit file');
    g('switch', '-q', 'main');
    g('switch', '-q', '-c', 'cs/g');
    writeFileSync(join(dir, 'shamble-game.js'), 'b\n');
    g('commit', '-q', '-am', 'three: a game file');
    g('switch', '-q', 'main');
    const rev = (r) => g('rev-parse', '--verify', '--end-of-options', `${r}^{commit}`).stdout.trim();
    assert.equal(rev('dup'), rev('refs/remotes/origin/main'), 'setup: the short name dup reads as the tag, at origin/main');
    assert.notEqual(rev('refs/heads/dup'), rev('dup'), 'setup: the branch dup is at another commit');
    process.env.CLAUDE_PROJECT_DIR = dir;
    const io = { ...realIo(), env: {} };
    const day = (command) => decide(dsh(command, { cwd: dir }), io);
    const d = await day('git switch dup');
    assert.equal(kindOf(d), ASK, `git switch dup asks: ${JSON.stringify(d)}`);
    assert.match(d.ask, /refs\/heads\/dup/);
    assert.equal(await day('git switch --detach dup'), null, '--detach dup reads the tag, as git does: no kit difference');
    assert.equal(await day('git switch cs/g'), null, 'a branch alone, game files only: goes on');
    assert.equal(await day('git switch main'), null, 'main, a branch alone at origin/main: goes on');
    // what the guard checked is where git goes: the switch lands on the branch's commit, not the tag's
    assert.equal(g('switch', '-q', 'dup').status, 0);
    assert.equal(rev('HEAD'), rev('refs/heads/dup'), 'git switch dup moved to the branch');
  } finally {
    if (prev === undefined) delete process.env.CLAUDE_PROJECT_DIR; else process.env.CLAUDE_PROJECT_DIR = prev;
    rmSync(dir, { recursive: true, force: true });
  }
});

// --- fix round (step 8 reviews, R-a..R-e) on real folders: the forked forms ask, real bash writes the kit file, and
// the ruling's quiet controls stay quiet in a real worktree ---
test('real folders (fix round R-a..R-e): SA-1 forms ask, a failed cd asks, the quiet controls go on', async () => {
  const dir = realpathSync(mkdtempSync(join(tmpdir(), 'guard-cd-fix-')));
  const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
  const g = (...a) => spawnSync('git', a, { cwd: dir, env, encoding: 'utf8' });
  const prev = process.env.CLAUDE_PROJECT_DIR;
  try {
    g('init', '-q', '-b', 'main');
    mkdirSync(join(dir, '.claude', 'hooks'), { recursive: true });
    mkdirSync(join(dir, 'tools'));
    writeFileSync(join(dir, '.claude', 'settings.json'), 'a\n');
    writeFileSync(join(dir, '.claude', 'hooks', 'g.mjs'), 'a\n');
    writeFileSync(join(dir, 'tools', 'gate.mjs'), 'a\n');
    g('add', '-A');
    g('commit', '-q', '-m', 'one');
    g('update-ref', 'refs/remotes/origin/main', 'HEAD');
    g('switch', '-q', '-c', 'cs/kit');
    writeFileSync(join(dir, 'tools', 'gate.mjs'), 'b\n');
    g('commit', '-q', '-am', 'kit');
    g('switch', '-q', 'main');
    const wt = join(dir, '.gate', 'wt');
    assert.equal(g('worktree', 'add', '-q', '-b', 'cs/x', wt, 'main').status, 0, 'setup: a worktree under .gate/');
    process.env.CLAUDE_PROJECT_DIR = dir;
    const io = { ...realIo(), env: {} };
    const day = async (command, cwd = dir) => kindOf(await decide(dsh(command, { cwd }), io));
    for (const c of ['cd .claude && sed -i s/a/b/ settings.json && git status | cat', '(cd .claude && sed -i s/a/b/ settings.json)',
      'cd .claude && echo x > settings.json; ls | head', 'cd .claude/hooks && sed -i s/a/b/ g.mjs && node --version | tail -1',
      'cd .claude/hooks && sed -i s/a/b/ g.mjs &', 'cd .claude && sed -i s/a/b/ settings.json && echo $(date)',
      'git status | cat; cd .claude && rm settings.json', '(cd ./.claude && sed -i s/a/b/ settings.json)']) {
      assert.equal(await day(c), ASK, `SA-1 form asks: ${c}`);
    }
    // what the guard asks about is real: the piped line does edit the kit file
    spawnSync('bash', ['-c', 'cd .claude/hooks && sed -i s/a/b/ g.mjs && node --version | tail -1'], { cwd: dir, env });
    assert.equal(readFileSync(join(dir, '.claude', 'hooks', 'g.mjs'), 'utf8'), 'b\n', 'real bash: the forked line wrote the kit file');
    // R-b: a cd that fails leaves the shell where it was (bash: the switch runs in the main folder)
    assert.equal(await day(`cd ${wt}/gone; git switch cs/kit`), ASK, 'a cd to a missing folder is unknown');
    assert.equal(await day(`cd ${dir}/.claude/settings.json; git switch cs/kit`), ASK, 'a cd to a file is unknown');
    const fail = spawnSync('bash', ['-c', `cd ${wt}/gone; git rev-parse --show-toplevel`], { cwd: dir, env, encoding: 'utf8' });
    assert.equal(fail.stdout.trim(), dir, 'real bash: after the failed cd the next command runs in the main folder');
    // the ruling's quiet controls
    assert.equal(await day(`cd ${wt} && git switch cs/kit`), null, 'QC1 cd <abs wt> && git switch');
    assert.equal(await day('cd ./.gate/wt && git switch cs/kit'), null, 'QC2 cd ./.gate/wt && git switch');
    assert.equal(await day(`cd ${wt} && echo x > notes.md`), null, 'QC3 cd <abs wt> && echo x > notes.md');
    assert.equal(await day(`cd ${dir}/tools && npm test 2>&1 | tail`), null, 'QC4 cd <abs>/tools && npm test 2>&1 | tail');
    assert.equal(await day('git switch cs/kit'), ASK, 'control: the same switch in the main folder asks');
    assert.equal(await day(`command -p cd ${dir} && git switch cs/kit`, wt), ASK, 'command -p cd <main> from the worktree asks');
  } finally {
    if (prev === undefined) delete process.env.CLAUDE_PROJECT_DIR; else process.env.CLAUDE_PROJECT_DIR = prev;
    rmSync(dir, { recursive: true, force: true });
  }
});

// --- fix round 2 on real git: a bare origin, and a project whose cs/evil branch changes a kit file and a script ---
function realOrigin(prefix) {
  const root = realpathSync(mkdtempSync(join(tmpdir(), prefix)));
  const O = join(root, 'origin.git'), P = join(root, 'proj');
  const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
  const g = (...a) => spawnSync('git', a, { cwd: P, env, encoding: 'utf8' });
  spawnSync('git', ['init', '-q', '--bare', '-b', 'main', O], { env });
  mkdirSync(join(P, '.claude'), { recursive: true });
  mkdirSync(join(P, 'tools'));
  mkdirSync(join(P, 'docs', 'sub', 'x'), { recursive: true });
  writeFileSync(join(P, 'CLAUDE.md'), 'rules\n');
  writeFileSync(join(P, '.claude', 'settings.json'), '{"a":1}\n');
  writeFileSync(join(P, 'tools', 'sim.js'), 'console.log(1)\n');
  writeFileSync(join(P, 'docs', 'a.md'), 'a\n');
  writeFileSync(join(P, 'docs', 'sub', 'x', 'CLAUDE.md'), 'nested\n');
  g('init', '-q', '-b', 'main');
  g('add', '-A');
  g('commit', '-q', '-m', 'base');
  g('remote', 'add', 'origin', O);
  g('push', '-q', 'origin', 'main');
  g('fetch', '-q', 'origin');
  g('switch', '-q', '-c', 'cs/evil');
  writeFileSync(join(P, '.claude', 'settings.json'), '{"a":2}\n');
  writeFileSync(join(P, 'tools', 'sim.js'), 'console.log(2)\n');
  g('commit', '-q', '-am', 'evil');
  g('push', '-q', 'origin', 'cs/evil');
  g('switch', '-q', 'main');
  g('fetch', '-q', 'origin');
  return { root, O, P, g };
}
test("real git (fix round 2, SA-2): the auditor's routine rows are refused; origin/main is refs/remotes/origin/main's commit", async () => {
  const { root, P, g } = realOrigin('guard-git-sa2-');
  const prev = process.env.CLAUDE_PROJECT_DIR;
  const rev = (r) => g('rev-parse', '--verify', '--end-of-options', `${r}^{commit}`).stdout.trim();
  try {
    process.env.CLAUDE_PROJECT_DIR = P;
    const io = (role) => ({ ...realIo(), env: role ? { SHAMBLE_ROLE: role } : {} });
    const run = async (role, command) => decide(dsh(command, { cwd: P }), io(role));
    const kind = async (role, command) => kindOf(await run(role, command));
    const forge = 'git fetch origin cs/evil:refs/remotes/origin/main';
    assert.equal(await kind(NIGHT, forge), DENY, 'Ri1');
    assert.equal(await kind(PRA, forge), DENY, 'Re3');
    assert.equal(await kind(NIGHT, 'git branch origin/main cs/evil'), DENY, 'Re1');
    assert.equal(await kind(PRA, 'git branch origin/main origin/cs/evil'), DENY, 'Re2');
    assert.equal(await kind(NIGHT, 'git symbolic-ref refs/remotes/origin/main refs/heads/cs/evil'), DENY, 'symbolic-ref repointing origin/main');
    g('switch', '-q', 'cs/evil');
    assert.equal(await kind(NIGHT, `${forge} && git push origin cs/evil`), DENY, 'Rp2: the forging fetch stops the line');
    assert.equal(await kind(NIGHT, `${forge} && node tools/sim.js`), DENY, 'Rn2');
    assert.equal(await kind(NIGHT, 'git push origin cs/evil'), DENY, 'Rp1: a branch that changes .claude/settings.json');
    // what the refused commands would do: each repoints the commit refs/remotes/origin/main or origin/main names
    const main0 = rev('refs/remotes/origin/main'), evil = rev('cs/evil');
    g('fetch', '-q', 'origin', 'cs/evil:refs/remotes/origin/main');
    assert.equal(rev('refs/remotes/origin/main'), evil, 'real git: the refspec fetch repoints refs/remotes/origin/main');
    g('fetch', '-q', 'origin', '+main:refs/remotes/origin/main');
    assert.equal(rev('refs/remotes/origin/main'), main0, 'setup: restored');
    // a local branch named origin/main at cs/evil (made here by the test, as Re1 would): every site refuses
    g('branch', 'origin/main', 'cs/evil');
    assert.equal(rev('origin/main'), evil, 'real git: the short name now reads the local branch');
    for (const [role, command, label] of [[NIGHT, 'git push origin cs/evil', 'Rp4'], [NIGHT, 'node tools/sim.js', 'Rn4'],
      [PRA, 'git switch --detach origin/main', 'Rs4']]) {
      const d = await run(role, command);
      assert.match(d?.deny ?? '', SHADOWED, `${label}: ${JSON.stringify(d)}`);
    }
    g('branch', '-q', '-D', 'origin/main');
    g('switch', '-q', 'main');
    g('tag', 'origin/main', 'cs/evil'); // a tag of that name: the same
    assert.match((await run(PRA, 'git switch --detach origin/main'))?.deny ?? '', SHADOWED, 'Rs4 with a tag');
    g('tag', '-d', 'origin/main');
    // controls: the real origin/main, and the full ref missing
    assert.equal(await run(PRA, 'git switch --detach origin/main'), null, 'the PR Advisor switch goes on');
    assert.equal(await run(NIGHT, 'node tools/sim.js'), null, "main's copy of a tracked script goes on");
    g('switch', '-q', '-c', 'cs/ok');
    writeFileSync(join(P, 'docs', 'a.md'), 'b\n');
    g('commit', '-q', '-am', 'docs');
    assert.equal(await run(NIGHT, 'git push origin cs/ok'), null, 'a branch with docs changes only goes on');
    g('update-ref', '-d', 'refs/remotes/origin/main');
    assert.match((await run(NIGHT, 'git push origin cs/ok'))?.deny ?? '', /can't resolve refs\/remotes\/origin\/main/, 'no full ref: refused');
    g('fetch', '-q', 'origin', 'main');
    assert.equal(rev('refs/remotes/origin/main'), main0, 'setup: fetched again');
    // day sessions: stated limit L12 (Fi1-Fi3 go on), the routines' rules do not reach them
    assert.equal(await run('', forge), null, 'Fi1 (L12)');
    assert.equal(await run('', 'git merge --ff-only origin/main'), null, 'Fi2');
    assert.equal(await run('', `${forge} && git merge --ff-only origin/main`), null, 'Fi3 (L12)');
  } finally {
    if (prev === undefined) delete process.env.CLAUDE_PROJECT_DIR; else process.env.CLAUDE_PROJECT_DIR = prev;
    rmSync(root, { recursive: true, force: true });
  }
});
// SA-2 item 4: no command a routine may run writes the git config that names the remote or what git loads. Each
// allowed form runs on real git; the config may change only in git's own branch tracking (branch.<name>.remote and
// .merge, from push -u, switch -c from origin/main and branch -u). The writers of remote.*, url.*, core.* and include.*
// are refused for both roles.
test('real git (fix round 2, SA-2 item 4): no command a routine may run writes the remote config', async () => {
  const { root, P, g } = realOrigin('guard-git-cfg-');
  const prev = process.env.CLAUDE_PROJECT_DIR;
  const cfg = () => g('config', '--local', '--list').stdout.split('\n').filter(Boolean);
  try {
    process.env.CLAUDE_PROJECT_DIR = P;
    g('config', 'user.email', 't@t'); // so the config read exits 0
    const io = (role) => ({ ...realIo(), env: { SHAMBLE_ROLE: role } });
    const allowed = [[NIGHT, 'git fetch -q origin main'], [NIGHT, 'git fetch -q origin cs/evil'], [NIGHT, 'git switch -q -c cs/n origin/main'],
      [NIGHT, 'git branch cs/m origin/main'], [NIGHT, 'git branch -u origin/main'], [NIGHT, 'git worktree add -q .gate/b origin/main'],
      [NIGHT, 'git worktree remove .gate/b'], [NIGHT, 'git push -q -u origin cs/n'], [NIGHT, 'git remote -v'], [NIGHT, 'git remote get-url origin'],
      [NIGHT, 'git remote show origin'], [NIGHT, 'git symbolic-ref --short HEAD'], [NIGHT, 'git config user.email'], [NIGHT, 'git status'],
      [NIGHT, 'git ls-remote --heads origin'], [NIGHT, 'git log -1'], [NIGHT, 'git branch -a'], [PRA, 'git fetch -q origin main'],
      [PRA, 'git switch --detach origin/main'], [PRA, 'git remote -v']];
    for (const [role, command] of allowed) {
      assert.equal(await decide(dsh(command, { cwd: P }), io(role)), null, `${role}: ${command} is allowed`);
      const before = cfg();
      const r = g(...command.split(' ').slice(1));
      assert.equal(r.status, 0, `${command} ran: ${r.stderr}`);
      const after = cfg();
      const plain = (l) => l.filter((x) => !x.startsWith('branch.'));
      assert.deepEqual(plain(after), plain(before), `${command}: only branch.* may change`);
      for (const x of after.filter((y) => !before.includes(y))) assert.match(x, /^branch\.[^=]+\.(remote|merge)=/, `${command} wrote ${x}`);
    }
    const writers = ['git remote add evil /tmp/evil', 'git remote set-url origin /tmp/evil', 'git remote set-branches origin x',
      'git remote rename origin o2', 'git remote remove origin', 'git remote prune origin', 'git remote update', 'git remote set-head origin main',
      'git config remote.origin.url /tmp/evil', 'git config --add remote.origin.fetch +refs/heads/x:refs/remotes/origin/main',
      'git config core.hooksPath /tmp/h', 'git config user.email e@x', 'git branch --edit-description'];
    for (const role of [NIGHT, PRA]) {
      for (const command of writers) assert.equal(kindOf(await decide(dsh(command, { cwd: P }), io(role))), DENY, `${role}: ${command}`);
    }
  } finally {
    if (prev === undefined) delete process.env.CLAUDE_PROJECT_DIR; else process.env.CLAUDE_PROJECT_DIR = prev;
    rmSync(root, { recursive: true, force: true });
  }
});
test('real git (fix round 2, F-b and SA-6): worktree add and clone into kit folders, folders holding kit files', async () => {
  const { root, O, P, g } = realOrigin('guard-git-fb-');
  const prev = process.env.CLAUDE_PROJECT_DIR;
  try {
    process.env.CLAUDE_PROJECT_DIR = P;
    const io = { ...realIo(), env: {} };
    const day = async (command, extra = {}) => kindOf(await decide(dsh(command, { cwd: P, ...extra }), io));
    assert.equal(await day('git worktree add .claude/rules/wt cs/evil'), ASK, 'F-b worktree add into .claude/rules');
    assert.equal(await day('git worktree add .claude/rules/wt cs/evil', { agent_id: 'a1' }), DENY, 'F-b, an agent');
    assert.equal(await day(`git clone ${O} .claude/skills/x`), ASK, 'F-b clone into .claude/skills');
    assert.equal(await day(`git clone ${O} .claude/skills/x`, { agent_id: 'a1' }), DENY, 'F-b clone, an agent');
    assert.equal(await day('git worktree add .gate/wt cs/evil'), null, 'control: a worktree under .gate/');
    assert.equal(await day(`git clone ${O} ${join(root, 'copy')}`), null, 'control: a clone outside the project');
    // what F-b asks about is real: another ref's kit files land in a kit folder
    assert.equal(g('worktree', 'add', '-q', '.claude/rules/wt', 'cs/evil').status, 0);
    assert.equal(readFileSync(join(P, '.claude', 'rules', 'wt', '.claude', 'settings.json'), 'utf8'), '{"a":2}\n', "real git: cs/evil's settings under .claude/rules/wt");
    g('worktree', 'remove', '--force', '.claude/rules/wt');
    // SA-6 on real folders: a tracked nested CLAUDE.md, the project's parent, a file, a folder without kit files
    assert.equal(await day('rm -rf docs'), ASK, 'Fc1 docs holds docs/sub/x/CLAUDE.md (two levels down)');
    assert.equal(await day(`rm -rf ${root}`), ASK, "Fc2 the project folder's parent");
    assert.equal(await day('rm -rf docs/a.md'), null, 'a file target');
    mkdirSync(join(P, 'docs', 'plain'));
    writeFileSync(join(P, 'docs', 'plain', 'b.md'), 'b\n');
    g('add', 'docs/plain/b.md');
    assert.equal(await day('rm -rf docs/plain'), null, 'a folder with no kit file');
    assert.equal(await day('rm -rf node_modules_none'), null, 'a folder that is not there');
  } finally {
    if (prev === undefined) delete process.env.CLAUDE_PROJECT_DIR; else process.env.CLAUDE_PROJECT_DIR = prev;
    rmSync(root, { recursive: true, force: true });
  }
});

// --- step 9, WT-NIGHT (a) (Mustafa, 2026-11-05): the night shift switches its folder to an item's branch only when
// the branch's rules, CI and gate files equal main's ---
// A fake io for the night's switch check, on top of dayIo (role night-shift). kit answers the check's diff by the
// target's name: NUL-separated names, as `git diff --name-only -z` prints them. A name kit lists resolves (as dayIo
// resolves the names of diffs: a branch refs/heads/<name>, a full ref such as refs/remotes/origin/<name>, a commit
// word, HEAD); a name it does not list does not. The diff fails when diffFail is set, or when it is not given
// refs/remotes/origin/main's commit first and `--` last; a target SHA kit does not list is equal to main.
function nightIo(o = {}) {
  const { kit = {}, diffFail = false, calls, ...rest } = o;
  const io = dayIo({ ...rest, role: NIGHT, diffs: Object.fromEntries(Object.keys(kit).map((n) => [n, ''])) });
  const bySha = new Map(Object.entries(kit).map(([n, out]) => [shaOf(n), out]));
  return {
    ...io,
    git: (args, cwd) => {
      calls?.push(args);
      if (args[0] === 'diff' && args.includes('-z')) {
        const [a, b] = args.slice(-3, -1);
        if (diffFail || a !== shaOf('origin/main') || args.at(-1) !== '--') return { code: 128, out: '' };
        return { code: 0, out: bySha.get(b) ?? '' };
      }
      return io.git(args, cwd);
    },
  };
}
const NIGHT_HIT = /rules, CI or gate files differ from main's .*stop the item and record it under Decisions needed for a day session/;
const KIT = (...files) => files.map((f) => `${f}\0`).join('');
// [name, input, expected kind, nightIo options, reason pattern]
export const NIGHT_SWITCH_CASES = [
  ['T15 night git switch cs/x, the diff names .claude/hooks/guard.mjs: deny', dsh('git switch cs/x'), DENY,
    { kit: { 'cs/x': KIT('shamble-game.js', '.claude/hooks/guard.mjs') } }, /cs\/x's rules, CI or gate files differ from main's \(\.claude\/hooks\/guard\.mjs\)/],
  ['T15r night, the diff names a nested CLAUDE.md (rules): deny', dsh('git switch cs/x'), DENY, { kit: { 'cs/x': KIT('docs/sub/CLAUDE.md') } }, NIGHT_HIT],
  ['T15c night, the diff names .github/workflows/gate.yml (CI): deny', dsh('git switch cs/x'), DENY,
    { kit: { 'cs/x': KIT('.github/workflows/gate.yml') } }, NIGHT_HIT],
  ['T15g night, the diff names tools/package-lock.json (gate): deny', dsh('git switch cs/x'), DENY,
    { kit: { 'cs/x': KIT('docs/a.md', 'tools/package-lock.json') } }, NIGHT_HIT],
  ['T15w night, in a worktree too (every night switch is checked): deny', dsh('git switch cs/x', { cwd: '/w' }), DENY,
    { kit: { 'cs/x': KIT('.claude/settings.json') } }, NIGHT_HIT],
  ['T16 night, a diff of shamble-game.js only: goes on', dsh('git switch cs/x'), null, { kit: { 'cs/x': KIT('shamble-game.js') } }],
  ['T16 night, docs and agent memory only: goes on', dsh('git switch -q cs/x'), null,
    { kit: { 'cs/x': KIT('docs/STATUS.md', '.claude/agent-memory-local/advisor/x.md', '.checkpoints/x.md') } }],
  ['T16 night, git switch -- cs/x: goes on', dsh('git switch -- cs/x'), null, { kit: { 'cs/x': KIT('docs/a.md') } }],
  ['T16a night --detach an old commit whose CLAUDE.md differs: deny', dsh('git switch --detach 4def8d2'), DENY,
    { kit: { '4def8d2': KIT('CLAUDE.md') } }, /4def8d2's rules, CI or gate files differ/],
  ['T16a night -d an old commit whose CLAUDE.md differs: deny', dsh('git switch -d 4def8d2'), DENY, { kit: { '4def8d2': KIT('CLAUDE.md') } }],
  ['T16a control: night --detach origin/main goes on', dsh('git switch --detach origin/main'), null],
  ['T16a night --detach with no ref (HEAD), HEAD differs: deny', dsh('git switch --detach'), DENY, { kit: { HEAD: KIT('.claude/rules/x.md') } }],
  ['T16b night git switch -: deny', dsh('git switch -'), DENY, {}, /can't tell where git switch - goes/],
  ['T16b night git switch --orphan x: deny', dsh('git switch --orphan x'), DENY, { kit: { x: '' } }, /can't tell/],
  ['T16b night git switch --orphan=x: deny', dsh('git switch --orphan=x'), DENY, { kit: { x: '' } }],
  ['T16c night, the diff fails: deny', dsh('git switch cs/x'), DENY, { kit: { 'cs/x': '' }, diffFail: true }, /can't compare cs\/x with origin\/main/],
  ['T16e night git switch -f cs/ok (another option; the option check refuses it first): deny', dsh('git switch -f cs/ok'), DENY, { kit: { 'cs/ok': '' } }],
  ['T16e night git switch --guess cs/ok (another option): deny', dsh('git switch --guess cs/ok'), DENY, { kit: { 'cs/ok': '' } }, /can't tell/],
  ['T16e night git switch -C cs/n cs/ok (force-create): deny', dsh('git switch -C cs/n cs/ok'), DENY, { kit: { 'cs/ok': '' } }],
  ['T16e night git switch --force-create=cs/n: deny', dsh('git switch --force-create=cs/n'), DENY, {}],
  ['T16e night git switch --discard-changes cs/ok: deny', dsh('git switch --discard-changes cs/ok'), DENY, { kit: { 'cs/ok': '' } }],
  ['T16e night git switch -m cs/ok: deny', dsh('git switch -m cs/ok'), DENY, { kit: { 'cs/ok': '' } }],
  ['T16e night git switch --track cs/ok: deny', dsh('git switch --track cs/ok'), DENY, { kit: { 'cs/ok': '' } }],
  ['T16e night git switch -qc cs/n (bundled): deny', dsh('git switch -qc cs/n'), DENY, {}],
  ['T16e night git switch -c with no name: deny', dsh('git switch -c'), DENY, {}],
  ['T16e night git switch -c a -c b: deny', dsh('git switch -c a -c b'), DENY, {}],
  ['T16f night git switch cs/ok cs/ok2 (two targets): deny', dsh('git switch cs/ok cs/ok2'), DENY,
    { kit: { 'cs/ok': '', 'cs/ok2': '' } }, /names no single target/],
  ['T16f night git switch (no target): deny', dsh('git switch'), DENY, {}, /names no single target/],
  ['T16g night -c cs/n from a start point whose gate file differs: deny', dsh('git switch -c cs/n cs/kit'), DENY,
    { kit: { 'cs/kit': KIT('tools/gate.mjs') } }, /cs\/kit's rules, CI or gate files differ/],
  ['T16g control: night -c cs/n origin/main goes on', dsh('git switch -c cs/n origin/main'), null],
  ['T16g control: night -q -c cs/n origin/main goes on', dsh('git switch -q -c cs/n origin/main'), null],
  ['T16g control: night -ccs/n origin/main goes on', dsh('git switch -ccs/n origin/main'), null],
  ['T16g control: night --create=cs/n origin/main goes on', dsh('git switch --create=cs/n origin/main'), null],
  ['T16g night --create cs/n cs/kit: deny', dsh('git switch --create cs/n cs/kit'), DENY, { kit: { 'cs/kit': KIT('.claude/settings.json') } }],
  ['T16h night -c cs/n with no start point, HEAD differs: deny', dsh('git switch -c cs/n'), DENY,
    { kit: { HEAD: KIT('.claude/hooks/routine-guard.mjs') } }, /HEAD's rules, CI or gate files differ/],
  ['T16h control: night -c cs/n, HEAD equals main in kit files', dsh('git switch -c cs/n'), null, { kit: { HEAD: KIT('docs/a.md') } }],
  ['T16j night git switch cs/gone (no branch, no origin/cs/gone): deny', dsh('git switch cs/gone'), DENY, {}, /can't resolve cs\/gone to a commit; fetch it first/],
  ['T16j night -c cs/n cs/gone: deny', dsh('git switch -c cs/n cs/gone'), DENY, {}, /can't resolve cs\/gone/],
  ['T16j night git switch -- --output=/tmp/x (a word starting with -): deny', dsh('git switch -- --output=/tmp/x'), DENY, {}, /can't resolve/],
  ['T16k night -c cs/n origin/main, origin/main shadowed: deny', dsh('git switch -c cs/n origin/main'), DENY, { shadow: true }, SHADOWED],
  ['T16k night git switch cs/x, origin/main shadowed: deny', dsh('git switch cs/x'), DENY, { shadow: true, kit: { 'cs/x': '' } }, SHADOWED],
  ['T16l night, refs/remotes/origin/main does not resolve: deny', dsh('git switch cs/x'), DENY,
    { noRemoteMain: true, kit: { 'cs/x': '' } }, /can't resolve refs\/remotes\/origin\/main/],
  ['T16t night git switch dup: the branch refs/heads/dup is checked, not a same-name tag', dsh('git switch dup'), DENY,
    { kit: { dup: KIT('.claude/settings.json') }, tags: ['dup'] }, NIGHT_HIT],
  ['T20 night, no local cs/y, origin/cs/y equal to main: goes on', dsh('git switch cs/y'), null,
    { kit: { 'refs/remotes/origin/cs/y': KIT('shamble-game.js') } }],
  ['T20 night, no local cs/y, origin/cs/y changes CLAUDE.md: deny', dsh('git switch cs/y'), DENY,
    { kit: { 'refs/remotes/origin/cs/y': KIT('CLAUDE.md') } }, /cs\/y's rules, CI or gate files differ from main's \(CLAUDE\.md\)/],
  ['T20 night, the local cs/y differs and origin/cs/y does not: the local branch is checked (git takes it)', dsh('git switch cs/y'), DENY,
    { kit: { 'cs/y': KIT('.claude/settings.json'), 'refs/remotes/origin/cs/y': '' } }, NIGHT_HIT],
  // fix pass 2 (V-3), added: a plain name is looked up by the exact refs refs/heads/<name>, then
  // refs/remotes/origin/<name> (dwim: what rev-parse's guessing would answer, as a tag refs/tags/refs/heads/<name> makes it)
  ['V-3 (finding 13a) a tag refs/tags/refs/heads/cs/evil at a clean commit, no local cs/evil, origin/cs/evil changes the kit: deny',
    dsh('git switch cs/evil'), DENY, { kit: { 'cs/ok': KIT('docs/a.md'), 'refs/remotes/origin/cs/evil': KIT('.claude/settings.json') },
      dwim: { 'refs/heads/cs/evil': 'cs/ok' } }, /cs\/evil's rules, CI or gate files differ from main's \(\.claude\/settings\.json\)/],
  ['V-3 a tag refs/tags/refs/heads/cs/t1 at a clean commit, no branch cs/t1, no origin/cs/t1: deny', dsh('git switch cs/t1'), DENY,
    { kit: { 'cs/ok': '' }, dwim: { 'refs/heads/cs/t1': 'cs/ok' } }, /can't resolve cs\/t1 to a commit/],
  ['V-3 a tag refs/tags/refs/remotes/origin/cs/t2 at a clean commit, no branch cs/t2, no origin/cs/t2: deny', dsh('git switch cs/t2'), DENY,
    { kit: { 'cs/ok': '' }, dwim: { 'refs/remotes/origin/cs/t2': 'cs/ok' } }, /can't resolve cs\/t2 to a commit/],
  ['V-3 refs/heads/cs/p points at an annotated tag object: the commit it peels to is checked', dsh('git switch cs/p'), DENY,
    { kit: { 'cs/kit': KIT('CLAUDE.md') }, peel: { 'refs/heads/cs/p': 'cs/kit' } }, /cs\/p's rules, CI or gate files differ from main's \(CLAUDE\.md\)/],
  ['V-3 control: -c from the start point cs/ok is read by rev-parse, as git reads it', dsh('git switch -c cs/n cs/ok'), null,
    { kit: { 'cs/ok': KIT('docs/a.md') } }],
];
for (const [name, input, want, opts, re] of NIGHT_SWITCH_CASES) {
  test(`night switch (WT-NIGHT (a)) ${want ?? 'goes on'}: ${name}`, async () => {
    const d = await decide(input, nightIo(opts));
    assert.equal(kindOf(d), want, `${name}: ${JSON.stringify(d)}`);
    if (re) assert.match(d.deny, re);
  });
}
test('WT-NIGHT (a): unchanged for pr-advisor (its single switch) and day sessions (their main-folder check)', async () => {
  const kit = { 'cs/x': KIT('docs/a.md') };
  assert.match((await decide(dsh('git switch cs/x'), { ...nightIo({ kit }), env: { SHAMBLE_ROLE: PRA } }))?.deny ?? '', /the only switch is git switch --detach origin\/main/);
  assert.equal(await decide(dsh('git switch --detach origin/main'), { ...nightIo({ kit }), env: { SHAMBLE_ROLE: PRA } }), null);
  assert.equal(await decide(dsh('git switch cs/other', { cwd: '/w' }), dayIo({ diffs: OTHER })), null, 'day, a worktree: T8 unchanged');
  assert.equal(kindOf(await decide(dsh('git switch cs/other'), dayIo({ diffs: OTHER }))), ASK, 'day, the main folder: T1 unchanged');
});
test('WT-NIGHT (a): only SHAs reach the diff, refs/remotes/origin/main\'s commit first; no word of the command', async () => {
  for (const [name, input, , opts] of NIGHT_SWITCH_CASES) {
    const calls = [];
    await decide(input, nightIo({ ...opts, calls }));
    for (const c of calls.filter((x) => x[0] === 'diff' && x.includes('-z'))) {
      assert.deepEqual(c.slice(0, 6), ['diff', ...NAMES_Z.split(' ')], name); // fix pass 3 (P3-6): 6, was 5
      assert.equal(c[6], shaOf('origin/main'), `${name}: origin/main by commit`);
      assert.match(c[7], /^[0-9a-f]{40}$/, `${name}: the target as a SHA`);
      assert.equal(c[8], '--', name);
    }
    for (const c of calls.filter((x) => x[0] === 'rev-parse' && x.includes('--verify'))) {
      assert.equal(c.at(-2), '--end-of-options', `${name}: ${c.join(' ')}`);
    }
  }
});
const NIGHT_SWITCH_MUTANTS = [
  ['the night switch is checked', "const night = sub === 'switch' && role === 'night-shift' ? switchTargetCheck(args, vcwd, io) : null;", 'const night = null;'],
  ['only the listed forms (another option is not skipped)', "    if (t.startsWith('-')) {\n      return `${R}: the guard can't tell",
    "    if (t.startsWith('-')) continue;\n    if (false) {\n      return `${R}: the guard can't tell"],
  ['a single target', 'if (pos.length > 1 || (!pos.length && create === null && !detach)) {', 'if (false) {'],
  ['origin/main is refs/remotes/origin/main\'s commit (SA-2)', "const forged = originMainCheck(vcwd, io, 'night-shift');", 'const forged = null;'],
  // fix pass 3 (P3-6): the check lists with DIFF_NAMES now; the next two rows' target text moved (named)
  ['the diff gets the commit, not the word origin/main', '...DIFF_NAMES, main, sha, \'--\'], vcwd);', '...DIFF_NAMES, \'origin/main\', sha, \'--\'], vcwd);'],
  ['an unresolved target is refused', 'if (sha === null) return `${R}: can\'t resolve', 'if (false) return `${R}: can\'t resolve'],
  ['a failed diff is refused', 'if (d.code !== 0) return `${R}: can\'t compare ${target}', 'if (false) return `${R}: can\'t compare ${target}'],
  ['a protected name is refused', "return hit ? `${R}: ${target}'s rules", "return false ? `${R}: ${target}'s rules"],
  ['the names are NUL-separated', "const hit = d.out.split('\\0').filter(Boolean).find((f) => protectedRel(f));", "const hit = d.out.split('\\n').filter(Boolean).find((f) => protectedRel(f));"],
  ['-z is passed', "['diff', ...DIFF_NAMES, main", "['diff', '--name-only', '--no-renames', '--no-relative', '--ignore-submodules=none', main"],
  // fix pass 2 (V-3): the next three rows target the plain name's lookup, now exactCommit (their target text moved)
  ['the origin/<name> fallback (T20)', ' ?? exactCommit(`refs/remotes/origin/${name}`, vcwd, io);', ';'],
  ['the local branch first (as git switch)', 'exactCommit(`refs/heads/${name}`, vcwd, io) ?? exactCommit(`refs/remotes/origin/${name}`, vcwd, io);',
    'exactCommit(`refs/remotes/origin/${name}`, vcwd, io) ?? exactCommit(`refs/heads/${name}`, vcwd, io);'],
  ['a plain name is the branch refs/heads/<name>', 'exactCommit(`refs/heads/${name}`, vcwd, io) ?? exactCommit(`refs/remotes', 'commitOf(name, vcwd, io) ?? exactCommit(`refs/remotes'],
  // fix pass 2 (V-3), added: each leg by its exact name, never through rev-parse's guessing
  ['V-3 the branch leg is exact', 'exactCommit(`refs/heads/${name}`, vcwd, io) ??', 'commitOf(`refs/heads/${name}`, vcwd, io) ??'],
  ['V-3 the origin/<name> leg is exact', '?? exactCommit(`refs/remotes/origin/${name}`, vcwd, io);', '?? commitOf(`refs/remotes/origin/${name}`, vcwd, io);'],
  ['V-3 show-ref reads the exact name', "io.git(['show-ref', '--verify', '--hash', full], vcwd);", "io.git(['rev-parse', '--verify', '-q', full], vcwd);"],
  ['V-3 the SHA is peeled to its commit', '? commitOf(sha, vcwd, io) : null;', '? sha : null;'],
];
test('mutation runs (step 9, WT-NIGHT (a)): every rule of the night switch check is needed by some case', async () => {
  const src = readFileSync(join(HERE, 'routine-guard.mjs'), 'utf8');
  const dir = mkdtempSync(join(tmpdir(), 'guard-mut-night-'));
  const survivors = [];
  for (const [name, input, want, opts] of NIGHT_SWITCH_CASES) assert.equal(kindOf(await decide(input, nightIo(opts))), want, `unmutated rules: ${name}`);
  try {
    for (const [i, [label, from, to]] of NIGHT_SWITCH_MUTANTS.entries()) {
      assert.equal(src.split(from).length - 1, 1, `mutant "${label}": its target text occurs exactly once`);
      const file = join(dir, `n${i}.mjs`);
      writeFileSync(file, src.replace(from, () => to));
      const m = await import(pathToFileURL(file).href);
      let caught = false;
      for (const [, input, want, opts] of NIGHT_SWITCH_CASES) {
        let d;
        try { d = await m.decide(input, nightIo(opts)); } catch { d = 'threw'; }
        if ((d === 'threw' ? 'threw' : kindOf(d)) !== want) { caught = true; break; }
      }
      if (!caught) survivors.push(label);
    }
  } finally { rmSync(dir, { recursive: true, force: true }); }
  assert.deepEqual(survivors, [], `mutants no night switch case catches: ${survivors.join('; ')}`);
});
// Real git: a bare origin and a project (realOrigin: cs/evil changes .claude/settings.json and tools/sim.js). Each row
// runs decide() with the real io in the night-shift role; an allowed switch is then run, and lands on the commit the
// guard checked. The three diff flags are shown needed on real git: each removed in a copy, a real row goes through.
test('real git (step 9, WT-NIGHT (a)): the night switches only to a branch whose rules, CI and gate files equal main\'s', async () => {
  const { root, P, g } = realOrigin('guard-git-night-');
  const prev = process.env.CLAUDE_PROJECT_DIR;
  const rev = (r) => g('rev-parse', '--verify', '--end-of-options', `${r}^{commit}`).stdout.trim();
  const branch = (name, edit) => {
    g('switch', '-q', '-c', name, 'origin/main');
    edit();
    g('add', '-A');
    g('commit', '-q', '-m', name);
    g('push', '-q', 'origin', name);
    g('switch', '-q', 'main');
  };
  const dir = mkdtempSync(join(tmpdir(), 'guard-mut-night-real-'));
  try {
    process.env.CLAUDE_PROJECT_DIR = P;
    const io = { ...realIo(), env: { SHAMBLE_ROLE: NIGHT } };
    const run = (command, cwd = P, mod = { decide }) => mod.decide(dsh(command, { cwd }), io);
    branch('cs/ok', () => writeFileSync(join(P, 'docs', 'a.md'), 'ok\n'));
    branch('cs/rules', () => writeFileSync(join(P, 'CLAUDE.md'), 'other rules\n'));
    branch('cs/ren', () => g('mv', '.claude/settings.json', 'docs/settings.json'));
    branch('cs/uni', () => { mkdirSync(join(P, '.claude', 'agents'), { recursive: true }); writeFileSync(join(P, '.claude', 'agents', 'é.md'), 'x\n'); });
    g('fetch', '-q', 'origin');
    // a branch equal to main in kit files: allowed, and git lands where the guard looked
    assert.equal(await run('git switch cs/ok'), null, 'cs/ok changes docs only');
    const okSha = rev('refs/heads/cs/ok');
    assert.equal(g('switch', '-q', 'cs/ok').status, 0);
    assert.equal(rev('HEAD'), okSha, 'the switch landed on the checked commit');
    g('switch', '-q', 'main');
    // a branch with its own .claude/ or CLAUDE.md change: refused
    assert.match((await run('git switch cs/evil'))?.deny ?? '', /\.claude\/settings\.json.*Decisions needed/, 'cs/evil changes .claude/settings.json');
    assert.match((await run('git switch cs/rules'))?.deny ?? '', /\(CLAUDE\.md\)/, 'cs/rules changes CLAUDE.md');
    assert.match((await run('git switch -c cs/n origin/cs/evil'))?.deny ?? '', /origin\/cs\/evil's rules/, '-c from a kit-changing start');
    assert.match((await run('git switch --detach origin/cs/rules'))?.deny ?? '', /CLAUDE\.md/, '--detach to a kit-changing commit');
    assert.equal(await run('git switch --detach origin/main'), null, '--detach origin/main');
    assert.equal(await run('git switch -q -c cs/n origin/main'), null, '-c from origin/main');
    // T20: no local branch; git switch creates it from origin/<name>, and the guard checks that commit
    g('branch', '-q', '-D', 'cs/ok', 'cs/evil');
    assert.equal(await run('git switch cs/ok'), null, 'origin/cs/ok, docs only');
    assert.match((await run('git switch cs/evil'))?.deny ?? '', /settings\.json/, 'origin/cs/evil, a kit change');
    assert.equal(g('switch', '-q', 'cs/ok').status, 0);
    assert.equal(rev('HEAD'), rev('refs/remotes/origin/cs/ok'), 'created from origin/cs/ok, the commit the guard checked');
    g('switch', '-q', 'main');
    assert.match((await run('git switch cs/gone'))?.deny ?? '', /can't resolve cs\/gone/, 'neither a branch nor origin/cs/gone');
    // a kit file moved away, a kit file with a non-ASCII name, and diff.relative from a subfolder: refused
    const hard = [['git switch cs/ren', P, /\.claude\/settings\.json/], ['git switch cs/uni', P, /\.claude\/agents\/é\.md/],
      ['git switch cs/uni', join(P, 'docs'), /\.claude\/agents/]];
    g('config', 'diff.relative', 'true');
    for (const [command, cwd, re] of hard) assert.match((await run(command, cwd))?.deny ?? '', re, `${command} from ${cwd}`);
    // a forged origin/main (a local branch or a tag of that name at cs/evil's commit): refused through originMainCheck
    g('branch', 'origin/main', 'origin/cs/evil');
    for (const c of ['git switch -c cs/n origin/main', 'git switch --detach origin/main', 'git switch cs/ok']) {
      assert.match((await run(c))?.deny ?? '', SHADOWED, `${c}, origin/main a local branch`);
    }
    g('branch', '-q', '-D', 'origin/main');
    g('tag', 'origin/main', 'origin/cs/evil');
    assert.match((await run('git switch -c cs/n origin/main'))?.deny ?? '', SHADOWED, 'origin/main a tag');
    g('tag', '-d', 'origin/main');
    assert.equal(await run('git switch -c cs/n origin/main'), null, 'control: the real origin/main again');
    // the diff flags, each removed in a copy: some real row goes through
    const src = readFileSync(join(HERE, 'routine-guard.mjs'), 'utf8');
    const flags = [["'-z', ", 'NUL-separated names (-z)'], ["'--no-renames', ", '--no-renames'], ["'--no-relative', ", '--no-relative']];
    const survivors = [];
    for (const [i, [flag, label]] of flags.entries()) {
      // fix pass 3 (P3-6): the check lists with ...DIFF_NAMES; the copy spells the list out with the flag left out
      const from = "['diff', ...DIFF_NAMES, main";
      const full = "['diff', '--name-only', '-z', '--no-renames', '--no-relative', '--ignore-submodules=none', main";
      assert.equal(src.split(from).length - 1, 1, 'the diff line occurs once');
      const file = join(dir, `f${i}.mjs`);
      writeFileSync(file, src.replace(from, () => full.replace(flag, '')));
      const m = await import(pathToFileURL(file).href);
      let caught = false;
      for (const [command, cwd] of hard) if ((await run(command, cwd, m)) === null) caught = true;
      if (!caught) survivors.push(label);
    }
    assert.deepEqual(survivors, [], `diff flags no real row needs: ${survivors.join('; ')}`);
  } finally {
    if (prev === undefined) delete process.env.CLAUDE_PROJECT_DIR; else process.env.CLAUDE_PROJECT_DIR = prev;
    rmSync(root, { recursive: true, force: true });
    rmSync(dir, { recursive: true, force: true });
  }
});

// --- fix pass 2, routine paths (the night shift's wall) ---
// SD-1 (+ A-3): in the night-shift role, a line holding git switch, git commit or git push holds that one command and
// nothing else (no cd, pipe, background job, subshell or substitution), because the guard checks the switch's target,
// the commit's files and the pushed branch before the line runs. pr-advisor and day sessions are unchanged.
// SD-2: the guard's file lists (protectedHit: the night push and the day main-folder check; the night commit's staged
// and -a lists) are read NUL-separated with renames and relative output off. The fakes answer with the names git
// prints under these flags (a renamed kit file: both names; a non-ASCII name: unquoted; diff.relative: the full names);
// the real git test below shows each flag needed. [name, input, expected kind, io, reason pattern]
const ALONE = (sub) => new RegExp(`^routine \\(night-shift\\): git ${sub} runs alone, the only command of its call`);
// fix pass 3: R-PLAIN's refusal (what it found, the role) and P3-4's, word for word
const reEsc = (s) => s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
const PLAIN = (found, role = NIGHT) => new RegExp(`^${reEsc(`routine (${role}): write the command plainly (${found}): one command ` +
  'per line or joined with && ; values in quotes; no comments, $, backslashes, here-documents, braces, brackets or parentheses')}$`);
const FOREGROUND = /^routine \(night-shift\): routines run git in the foreground \(no background & or run_in_background on a line with git\)$/;
const nightFake = (o = {}) => fakeIo({ role: NIGHT, ...o });
const OK_X = () => nightIo({ kit: { 'cs/ok': KIT('docs/a.md'), 'cs/x': KIT('docs/a.md') } });
export const PASS2_ROUTINE_CASES = [
  // SD-1: refused
  ['SD-1 C3b night: git fetch -q origin cs/x && git switch -q cs/x', dsh('git fetch -q origin cs/x && git switch -q cs/x'), DENY, OK_X,
    /^routine \(night-shift\): git switch runs alone, the only command of its call \(the guard checks its target before the line runs\): run git fetch in its own call first, then git switch <branch> alone$/],
  ['SD-1 C3c night: git branch cs/n origin/cs/evil && git switch -q cs/n', dsh('git branch cs/n origin/cs/evil && git switch -q cs/n'), DENY,
    () => nightIo({ kit: { 'cs/n': '', 'origin/cs/evil': KIT('.claude/settings.json') } }), ALONE('switch')],
  ['SD-1 night: git switch cs/ok; git status', dsh('git switch cs/ok; git status'), DENY, OK_X, ALONE('switch')],
  // fix pass 3 moved the next three reasons (named): a background & with git is P3-4's; parentheses are R-PLAIN's
  ['SD-1 night: git switch cs/ok & x', dsh('git switch cs/ok & x'), DENY, OK_X, FOREGROUND],
  ['SD-1 night: git switch cs/ok & (a background job, one command)', dsh('git switch cs/ok &'), DENY, OK_X, FOREGROUND],
  ['SD-1 night: (git switch cs/ok) (a subshell, one command)', dsh('(git switch cs/ok)'), DENY, OK_X, PLAIN('a parenthesis, brace or bracket (()')],
  // added in fix pass 3: the one plain line left that is one command with a fork (it keeps "no pipe ..." needed)
  ['SD-1 night: git switch cs/ok | (a trailing pipe, one command)', dsh('git switch cs/ok |'), DENY, OK_X, ALONE('switch')],
  ['SD-1 night: git switch cs/ok | cat (a pipe)', dsh('git switch cs/ok | cat'), DENY, OK_X, ALONE('switch')],
  ['SD-1 night: cd <project> && git switch cs/ok (a cd counts)', dsh('cd /p && git switch cs/ok'), DENY, OK_X, ALONE('switch')],
  ['SD-1 night: git add f && git commit -m m', sh('git add f && git commit -m m'), DENY, () => nightFake(),
    /^routine \(night-shift\): git commit runs alone, the only command of its call \(the guard checks what it commits before the line runs\): run git add in its own call first, then git commit alone$/],
  ['SD-1 night: git commit -m m && git push origin cs/x', sh('git commit -m m && git push origin cs/x'), DENY, () => nightFake(), ALONE('commit')],
  ['SD-1 night: git status && git push origin cs/x', sh('git status && git push origin cs/x'), DENY, () => nightFake(),
    /^routine \(night-shift\): git push runs alone, the only command of its call \(the guard checks what it pushes before the line runs\): run git commit in its own call first, then git push alone$/],
  ['SD-1 night: git push origin cs/x | cat', sh('git push origin cs/x | cat'), DENY, () => nightFake(), ALONE('push')],
  // SD-1: as before
  ['SD-1 night: git switch cs/ok alone goes on', dsh('git switch cs/ok'), null, OK_X],
  ['SD-1 night: git switch cs/ok; (parseShell drops the empty command after the ;) goes on', dsh('git switch cs/ok;'), null, OK_X],
  ['SD-1 night: git switch cs/ok and a newline goes on', dsh('git switch cs/ok\n'), null, OK_X],
  ['SD-1 night: git commit -m m alone goes on', sh('git commit -m m'), null, () => nightFake()],
  ['SD-1 night: git push origin cs/x alone goes on', sh('git push origin cs/x'), null, () => nightFake()],
  ['SD-1 night: git fetch -q origin cs/x alone goes on', dsh('git fetch -q origin cs/x'), null, OK_X],
  ["SD-1 pr-advisor: its step-0 line git fetch -q origin main && git switch --detach origin/main goes on (unchanged)",
    sh('git fetch -q origin main && git switch --detach origin/main'), null, () => fakeIo({ role: PRA })],
  ['SD-1 day, a worktree: git fetch -q origin cs/x && git switch -q cs/x goes on (unchanged)', dsh('git fetch -q origin cs/x && git switch -q cs/x', { cwd: '/w' }), null,
    () => dayIo({ diffs: { 'cs/x': 'docs/a.md\n' } })],
  ['SD-1 day, the main folder: git fetch -q origin main && git switch cs/x, cs/x a kit change: ask (unchanged)', dsh('git fetch -q origin main && git switch cs/x'), ASK,
    () => dayIo({ diffs: { 'cs/x': '.claude/settings.json\n' } })],
  ['SD-1 day: git add f && git commit -m m goes on (unchanged)', dsh('git add f && git commit -m m'), null, () => dayIo()],
  ['SD-1 day: git commit -m m && git push origin cs/x goes on (unchanged)', dsh('git commit -m m && git push origin cs/x'), null, () => dayIo()],
  // SD-2: the night push (protectedHit through protectedInBranch)
  ['SD-2 night push: a kit file renamed away (tools/gate.mjs to docs/gate.mjs, both names listed): deny', sh('git push origin cs/x'), DENY,
    () => nightFake({ branchDiff: 'docs/gate.mjs\ntools/gate.mjs\n' }), /the branch changes tools\/gate\.mjs/],
  ['SD-2 night push: a non-ASCII kit path: deny', sh('git push origin cs/x'), DENY, () => nightFake({ branchDiff: '.claude/agents/é.md\n' }), /\.claude\/agents\/é\.md/],
  ['SD-2 night push: diff.relative set (the full names): deny', sh('git push origin cs/x'), DENY,
    () => nightFake({ branchDiff: 'docs/a.md\ntools/package.json\n' }), /tools\/package\.json/],
  // SD-2: the night commit, the staged list (--cached) and the -a list
  ['SD-2 night commit (--cached): a kit file renamed away: deny', sh('git commit -m m'), DENY,
    () => nightFake({ staged: 'docs/gate.mjs\ntools/gate.mjs\n' }), /the commit would change tools\/gate\.mjs/],
  ['SD-2 night commit (--cached): a non-ASCII kit path: deny', sh('git commit -m m'), DENY, () => nightFake({ staged: '.claude/agents/é.md\n' }), /é\.md/],
  ['SD-2 night commit (--cached): diff.relative set: deny', sh('git commit -m m'), DENY, () => nightFake({ staged: 'docs/a.md\ntools/smoke.mjs\n' }), /tools\/smoke\.mjs/],
  ['SD-2 night commit -a: a kit file moved away: deny', sh('git commit -a -m m'), DENY,
    () => nightFake({ unstaged: 'docs/a.md\ntools/gate.mjs\n' }), /the commit would change tools\/gate\.mjs/],
  ['SD-2 night commit -a: a non-ASCII kit path: deny', sh('git commit -a -m m'), DENY, () => nightFake({ unstaged: '.claude/agents/é.md\n' }), /é\.md/],
  ['SD-2 night commit -a: diff.relative set: deny', sh('git commit -a -m m'), DENY, () => nightFake({ unstaged: 'docs/a.md\ntools/battery.js\n' }), /tools\/battery\.js/],
  ['SD-2 night commit --all: deny', sh('git commit --all -m m'), DENY, () => nightFake({ unstaged: 'docs/a.md\ntools/battery.js\n' })],
  ['SD-2 control: night commit without -a, a kit file changed but not staged: goes on', sh('git commit -m m'), null,
    () => nightFake({ unstaged: 'docs/a.md\ntools/battery.js\n' })],
  ['SD-2 night commit, git cannot list the staged files: deny', dsh('git commit -m m'), DENY, () => nightIo(), /git can't list what the commit would change/],
  // SD-2: the day main-folder check (protectedHit through mainFolderMove)
  ['SD-2 day git switch cs/ren in the main folder, a kit file renamed away: ask', dsh('git switch cs/ren'), ASK,
    () => dayIo({ diffs: { 'cs/ren': 'docs/gate.mjs\ntools/gate.mjs\n' } }), /cs\/ren's tools\/gate\.mjs differs/],
  ['SD-2 day git switch cs/uni in the main folder, a non-ASCII kit path: ask', dsh('git switch cs/uni'), ASK,
    () => dayIo({ diffs: { 'cs/uni': '.claude/agents/é.md\n' } }), /é\.md differs/],
  ['SD-2 day git switch cs/rel in the main folder, diff.relative set: ask', dsh('git switch cs/rel'), ASK,
    () => dayIo({ diffs: { 'cs/rel': 'docs/a.md\ntools/gate.mjs\n' } }), /tools\/gate\.mjs differs/],
  ['SD-2 control: day git switch cs/doc, docs only: goes on', dsh('git switch cs/doc'), null, () => dayIo({ diffs: { 'cs/doc': 'docs/a.md\n' } })],
];
for (const [name, input, want, io, re] of PASS2_ROUTINE_CASES) {
  test(`routine (fix pass 2) ${want ?? 'goes on'}: ${name}`, async () => {
    const d = await decide(input, io());
    assert.equal(kindOf(d), want, `${name}: ${JSON.stringify(d)}`);
    if (re) assert.match(d.ask ?? d.deny, re);
  });
}
const PASS2_ROUTINE_MUTANTS = [
  ['SD-1 the night line rule', "if (role === 'night-shift' && NIGHT_ALONE[sub] && !line.alone) {", 'if (false) {'],
  ['SD-1 bash() passes the line to git()', 'r = await git(argv, vcwd, role, io, input, line);', 'r = await git(argv, vcwd, role, io, input, { alone: true });'],
  ['SD-1 one command only', 'const line = { alone: cmds.length === 1 && !forks && !subs };', 'const line = { alone: !forks && !subs };'],
  ['SD-1 no pipe, background job or subshell', 'const line = { alone: cmds.length === 1 && !forks && !subs };', 'const line = { alone: cmds.length === 1 && !subs };'],
  ['SD-1 switch is covered', "  switch: ['checks its target',", "  nope: ['checks its target',"],
  ['SD-1 commit is covered', "  commit: ['checks what it commits',", "  nope: ['checks what it commits',"],
  ['SD-1 push is covered', "  push: ['checks what it pushes',", "  nope: ['checks what it pushes',"],
  ['SD-1 night only (pr-advisor and day unchanged)', "if (role === 'night-shift' && NIGHT_ALONE[sub] && !line.alone) {", 'if (NIGHT_ALONE[sub] && !line.alone) {'],
  ['SD-2 protectedHit lists with DIFF_NAMES', "const d = io.git(['diff', ...DIFF_NAMES, ...range], vcwd);", "const d = io.git(['diff', '--name-only', ...range], vcwd);"],
  ["SD-2 protectedHit splits on NUL", "const hit = d.out.split('\\0').filter(Boolean).find((f) => protectedRel(worktreeRel(f, top, vcwd, io)));",
    "const hit = d.out.split('\\n').filter(Boolean).find((f) => protectedRel(worktreeRel(f, top, vcwd, io)));"],
  ['SD-2 the staged list uses DIFF_NAMES', "io.git(['diff', '--cached', ...DIFF_NAMES], vcwd)", "io.git(['diff', '--cached', '--name-only'], vcwd)"],
  ['SD-2 the -a list uses DIFF_NAMES', "lists.push(io.git(['diff', ...DIFF_NAMES], vcwd));", "lists.push(io.git(['diff', '--name-only'], vcwd));"],
  ["SD-2 the commit's lists split on NUL", "lists.flatMap((d) => d.out.split('\\0'))", "lists.flatMap((d) => d.out.split('\\n'))"],
  ['SD-2 -a adds the unstaged list', 'if (all) lists.push(', 'if (false) lists.push('],
  ['SD-2 a list git cannot give is refused', 'if (lists.some((d) => d.code !== 0)) {', 'if (false) {'],
];
test('fix pass 2 (SD-1): the three reasons, word for word', async () => {
  const why = async (c, io) => (await decide(c, io))?.deny;
  assert.equal(await why(dsh('git fetch origin cs/x && git switch cs/x'), OK_X()), 'routine (night-shift): git switch runs alone, the only command of its call (the guard checks its target before the line runs): run git fetch in its own call first, then git switch <branch> alone');
  assert.equal(await why(sh('git add f && git commit -m m'), nightFake()), 'routine (night-shift): git commit runs alone, the only command of its call (the guard checks what it commits before the line runs): run git add in its own call first, then git commit alone');
  assert.equal(await why(sh('git status; git push origin cs/x'), nightFake()), 'routine (night-shift): git push runs alone, the only command of its call (the guard checks what it pushes before the line runs): run git commit in its own call first, then git push alone');
});
test('mutation runs (fix pass 2, routine paths): every new rule is needed by some case (kind and reason)', async () => {
  const src = readFileSync(join(HERE, 'routine-guard.mjs'), 'utf8');
  const dir = mkdtempSync(join(tmpdir(), 'guard-mut-pass2-'));
  const survivors = [];
  const right = (d, want, re) => kindOf(d) === want && (!re || re.test(d.ask ?? d.deny));
  for (const [name, input, want, io, re] of PASS2_ROUTINE_CASES) assert.ok(right(await decide(input, io()), want, re), `unmutated rules: ${name}`);
  try {
    for (const [i, [label, from, to]] of PASS2_ROUTINE_MUTANTS.entries()) {
      assert.equal(src.split(from).length - 1, 1, `mutant "${label}": its target text occurs exactly once`);
      const file = join(dir, `p${i}.mjs`);
      writeFileSync(file, src.replace(from, () => to));
      const m = await import(pathToFileURL(file).href);
      let caught = false;
      for (const [, input, want, io, re] of PASS2_ROUTINE_CASES) {
        let d;
        try { d = await m.decide(input, io()); } catch { caught = true; break; }
        if (!right(d, want, re)) { caught = true; break; }
      }
      if (!caught) survivors.push(label);
    }
  } finally { rmSync(dir, { recursive: true, force: true }); }
  assert.deepEqual(survivors, [], `mutants no fix pass 2 case catches: ${survivors.join('; ')}`);
});
// Real git (fix pass 2, routine paths): realOrigin's project P, and a second clone Q that moves origin's cs/x to a
// kit-changing commit after P's last fetch. Each row (its own state set up and undone) is decided with the real io by
// the rules and by each mutant in a copy; a mutant is killed when every row named for it goes through (null) where
// the rules refuse or ask. Then the night's handoff in two calls: the fetch alone, then the switch alone, which checks
// the commit it lands on.
test('real git (fix pass 2, SD-1, V-3, SD-2): the night line rule, the exact branch lookups and the file lists\' flags', async (t) => {
  const { root, O, P, g } = realOrigin('guard-git-pass2-');
  const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
  const Q = join(root, 'q');
  const q = (...a) => spawnSync('git', a, { cwd: Q, env, encoding: 'utf8' });
  const rev = (r) => g('rev-parse', '--verify', '--end-of-options', `${r}^{commit}`).stdout.trim();
  const prev = process.env.CLAUDE_PROJECT_DIR;
  const dir = mkdtempSync(join(tmpdir(), 'guard-mut-pass2-real-'));
  const branch = (name, edit) => {
    g('switch', '-q', '-c', name, 'origin/main');
    edit();
    g('add', '-A');
    g('commit', '-q', '-m', name);
    g('push', '-q', 'origin', name);
    g('switch', '-q', 'main');
  };
  const uni = join(P, '.claude', 'agents', 'é.md');
  try {
    process.env.CLAUDE_PROJECT_DIR = P;
    const ioFor = (role) => ({ ...realIo(), env: role ? { SHAMBLE_ROLE: role } : {} });
    branch('cs/ok', () => writeFileSync(join(P, 'docs', 'a.md'), 'ok\n'));
    branch('cs/x', () => writeFileSync(join(P, 'docs', 'a.md'), 'x1\n'));
    branch('cs/n', () => writeFileSync(join(P, 'docs', 'a.md'), 'n1\n'));
    branch('cs/ren', () => g('mv', '.claude/settings.json', 'docs/settings.json'));
    branch('cs/uni', () => { mkdirSync(join(P, '.claude', 'agents'), { recursive: true }); writeFileSync(uni, 'x\n'); });
    g('fetch', '-q', 'origin');
    g('branch', '-q', '-D', 'cs/ok', 'cs/x', 'cs/n', 'cs/evil'); // on origin only; cs/ren and cs/uni stay local (day rows)
    assert.equal(spawnSync('git', ['clone', '-q', O, Q], { env }).status, 0);
    q('switch', '-q', 'cs/x');
    writeFileSync(join(Q, '.claude', 'settings.json'), '{"a":3}\n');
    q('commit', '-q', '-am', 'x2');
    q('push', '-q', 'origin', 'cs/x');
    const clean = rev('refs/remotes/origin/cs/ok');
    const onBranch = (b, more = () => {}) => () => { g('switch', '-q', b); more(); };
    const back = () => { g('reset', '-q', '--hard'); g('switch', '-q', 'main'); g('branch', '-q', '-D', 'cs/c'); };
    const scratch = (more) => () => { g('switch', '-q', '-c', 'cs/c', 'origin/main'); more(); };
    const relative = (on) => g('config', ...(on ? ['diff.relative', 'true'] : ['--unset', 'diff.relative']));
    // [label, role, command, cwd, want, setup, undo]
    const rows = [
      ['C3b', NIGHT, 'git fetch -q origin cs/x && git switch -q cs/x', P, DENY],
      ['C3c', NIGHT, 'git branch cs/n origin/cs/evil && git switch -q cs/n', P, DENY],
      ['pr-advisor step 0', PRA, 'git fetch -q origin main && git switch --detach origin/main', P, null],
      ['13a', NIGHT, 'git switch cs/evil', P, DENY, () => g('update-ref', 'refs/tags/refs/heads/cs/evil', clean), () => g('update-ref', '-d', 'refs/tags/refs/heads/cs/evil')],
      ['tag heads/cs/t1', NIGHT, 'git switch cs/t1', P, DENY, () => g('update-ref', 'refs/tags/refs/heads/cs/t1', clean), () => g('update-ref', '-d', 'refs/tags/refs/heads/cs/t1')],
      ['tag remotes/origin/cs/t2', NIGHT, 'git switch cs/t2', P, DENY,
        () => g('update-ref', 'refs/tags/refs/remotes/origin/cs/t2', clean), () => g('update-ref', '-d', 'refs/tags/refs/remotes/origin/cs/t2')],
      ['day switch rename', DAY, 'git switch cs/ren', P, ASK],
      ['day switch non-ASCII', DAY, 'git switch cs/uni', P, ASK],
      ['day switch relative', DAY, 'git switch cs/uni', join(P, 'docs'), ASK, () => relative(true), () => relative(false)],
      ['push rename', NIGHT, 'git push origin cs/ren', P, DENY, onBranch('cs/ren'), () => g('switch', '-q', 'main')],
      ['push non-ASCII', NIGHT, 'git push origin cs/uni', P, DENY, onBranch('cs/uni'), () => g('switch', '-q', 'main')],
      ['push relative', NIGHT, 'git push origin cs/uni', join(P, 'docs'), DENY, onBranch('cs/uni', () => relative(true)),
        () => { relative(false); g('switch', '-q', 'main'); }],
      ['commit --cached rename', NIGHT, 'git commit -m m', P, DENY, scratch(() => g('mv', '.claude/settings.json', 'docs/s.json')), back],
      ['commit --cached non-ASCII', NIGHT, 'git commit -m m', P, DENY,
        scratch(() => { mkdirSync(dirname(uni), { recursive: true }); writeFileSync(uni, 'y\n'); g('add', '-A'); }), back],
      ['commit --cached relative', NIGHT, 'git commit -m m', join(P, 'docs'), DENY,
        scratch(() => { writeFileSync(join(P, '.claude', 'settings.json'), '{"a":4}\n'); g('add', '-A'); relative(true); }), () => { relative(false); back(); }],
      ['commit -a moved away', NIGHT, 'git commit -a -m m', P, DENY,
        scratch(() => { writeFileSync(join(P, 'docs', 's.json'), readFileSync(join(P, '.claude', 'settings.json'))); rmSync(join(P, '.claude', 'settings.json')); }),
        () => { rmSync(join(P, 'docs', 's.json'), { force: true }); back(); }],
      ['commit -a non-ASCII', NIGHT, 'git commit -a -m m', P, DENY, onBranch('cs/uni', () => writeFileSync(uni, 'z\n')),
        () => { g('reset', '-q', '--hard'); g('switch', '-q', 'main'); }],
      ['commit -a relative', NIGHT, 'git commit -a -m m', join(P, 'docs'), DENY,
        scratch(() => { writeFileSync(join(P, '.claude', 'settings.json'), '{"a":5}\n'); relative(true); }), () => { relative(false); back(); }],
      ['control: night switch cs/ok alone', NIGHT, 'git switch cs/ok', P, null],
      ['control: night commit of docs', NIGHT, 'git commit -m m', P, null, scratch(() => { writeFileSync(join(P, 'docs', 'a.md'), 'c\n'); g('add', '-A'); }), back],
    ];
    // [label, from, to, the rows it must let through]
    const src = readFileSync(join(HERE, 'routine-guard.mjs'), 'utf8');
    const muts = [
      ['SD-1 the night line rule', "if (role === 'night-shift' && NIGHT_ALONE[sub] && !line.alone) {", 'if (false) {', ['C3b', 'C3c']],
      ['V-3 the branch leg is exact', 'exactCommit(`refs/heads/${name}`, vcwd, io) ??', 'commitOf(`refs/heads/${name}`, vcwd, io) ??', ['13a', 'tag heads/cs/t1']],
      ['V-3 the origin/<name> leg is exact', '?? exactCommit(`refs/remotes/origin/${name}`, vcwd, io);', '?? commitOf(`refs/remotes/origin/${name}`, vcwd, io);',
        ['tag remotes/origin/cs/t2']],
      // fix pass 3 (P3-6): the DIFF_NAMES line holds --ignore-submodules=none too; the next three rows' texts follow it
      ['SD-2 -z', "const DIFF_NAMES = ['--name-only', '-z', '--no-renames', '--no-relative', '--ignore-submodules=none'];", "const DIFF_NAMES = ['--name-only', '--no-renames', '--no-relative', '--ignore-submodules=none'];",
        ['day switch non-ASCII', 'push non-ASCII', 'commit --cached non-ASCII', 'commit -a non-ASCII']],
      ['SD-2 --no-renames', "const DIFF_NAMES = ['--name-only', '-z', '--no-renames', '--no-relative', '--ignore-submodules=none'];", "const DIFF_NAMES = ['--name-only', '-z', '--no-relative', '--ignore-submodules=none'];",
        ['day switch rename', 'push rename', 'commit --cached rename']],
      ['SD-2 --no-relative', "const DIFF_NAMES = ['--name-only', '-z', '--no-renames', '--no-relative', '--ignore-submodules=none'];", "const DIFF_NAMES = ['--name-only', '-z', '--no-renames', '--ignore-submodules=none'];",
        ['day switch relative', 'push relative', 'commit --cached relative', 'commit -a relative']],
      ['SD-2 protectedHit uses DIFF_NAMES', "const d = io.git(['diff', ...DIFF_NAMES, ...range], vcwd);", "const d = io.git(['diff', '--name-only', ...range], vcwd);",
        ['day switch rename', 'day switch non-ASCII', 'day switch relative', 'push rename', 'push non-ASCII', 'push relative']],
      ['SD-2 the staged list uses DIFF_NAMES', "io.git(['diff', '--cached', ...DIFF_NAMES], vcwd)", "io.git(['diff', '--cached', '--name-only'], vcwd)",
        ['commit --cached rename', 'commit --cached non-ASCII', 'commit --cached relative']],
      ['SD-2 the -a list uses DIFF_NAMES', "lists.push(io.git(['diff', ...DIFF_NAMES], vcwd));", "lists.push(io.git(['diff', '--name-only'], vcwd));",
        ['commit -a non-ASCII', 'commit -a relative']],
    ];
    const mods = [['rules', { decide }]];
    for (const [i, [label, from, to]] of muts.entries()) {
      assert.equal(src.split(from).length - 1, 1, `real mutant "${label}": its target text occurs exactly once`);
      const file = join(dir, `r${i}.mjs`);
      writeFileSync(file, src.replace(from, () => to));
      mods.push([label, await import(pathToFileURL(file).href)]);
    }
    const seen = new Map(mods.map(([label]) => [label, new Map()]));
    for (const [label, role, command, cwd, , setup, undo] of rows) {
      setup?.();
      try {
        for (const [m, mod] of mods) seen.get(m).set(label, kindOf(await mod.decide(dsh(command, { cwd }), ioFor(role))));
      } finally { undo?.(); }
    }
    for (const [label, , , , want] of rows) assert.equal(seen.get('rules').get(label), want, `real row ${label}`);
    for (const [label] of muts) t.diagnostic(`${label}: lets through ${[...seen.get(label)].filter(([r, k]) => k === null && seen.get('rules').get(r) !== null).map(([r]) => r).join(', ')}`);
    const survivors = muts.filter(([label, , , through]) => !through.every((r) => seen.get(label).get(r) === null)).map(([label]) => label);
    assert.deepEqual(survivors, [], `real mutants a named row does not let through: ${survivors.join('; ')}`);
    // the trap V-3 closes is real in this git: rev-parse reads the tag refs/tags/refs/heads/cs/evil for refs/heads/cs/evil
    g('update-ref', 'refs/tags/refs/heads/cs/evil', clean);
    assert.equal(rev('refs/heads/cs/evil'), clean, 'rev-parse guesses the tag');
    assert.notEqual(g('show-ref', '--verify', '--hash', 'refs/heads/cs/evil').status, 0, 'show-ref --verify does not');
    g('update-ref', '-d', 'refs/tags/refs/heads/cs/evil');
    // SD-1, the handoff in two calls: the fetch alone, then the switch alone sees what the fetch brought
    const night = async (command) => kindOf(await decide(dsh(command, { cwd: P }), ioFor(NIGHT)));
    assert.equal(await night('git fetch -q origin cs/x'), null, 'the fetch alone');
    assert.equal(g('fetch', '-q', 'origin', 'cs/x').status, 0);
    assert.match((await decide(dsh('git switch -q cs/x', { cwd: P }), ioFor(NIGHT)))?.deny ?? '', /cs\/x's rules, CI or gate files differ from main's \(\.claude\/settings\.json\)/,
      'then the switch alone is checked against the fetched commit');
    q('switch', '-q', 'cs/ok');
    writeFileSync(join(Q, 'docs', 'a.md'), 'ok2\n');
    q('commit', '-q', '-am', 'ok2');
    q('push', '-q', 'origin', 'cs/ok');
    assert.equal(await night('git fetch -q origin cs/ok'), null, 'the fetch alone');
    assert.equal(g('fetch', '-q', 'origin', 'cs/ok').status, 0);
    const checked = rev('refs/remotes/origin/cs/ok');
    assert.equal(checked, q('rev-parse', 'HEAD').stdout.trim(), 'the fetch brought the new cs/ok');
    assert.equal(await night('git switch -q cs/ok'), null, 'cs/ok changes docs only');
    assert.equal(g('switch', '-q', 'cs/ok').status, 0);
    assert.equal(rev('HEAD'), checked, 'the switch landed on the commit the guard checked');
    g('switch', '-q', 'main');
  } finally {
    if (prev === undefined) delete process.env.CLAUDE_PROJECT_DIR; else process.env.CLAUDE_PROJECT_DIR = prev;
    rmSync(root, { recursive: true, force: true });
    rmSync(dir, { recursive: true, force: true });
  }
});
// Real git and bash (fix pass 2, day paths): realOrigin's project P with a worktree W under .gate/, a missing folder M,
// a folder N outside any repository, and a second clone Q that moves origin's cs/x to a kit-changing commit after P's
// last fetch. Each row is decided with the real io by the rules and by each mutant in a copy (a mutant is killed when
// every row named for it goes through); then real bash runs the SD-3 and SD-4 rows and shows the main folder moved.
test('real git (fix pass 2, day: SD-3 C3d, SD-4 C1 C1c C2 C3, SD-6): the asks are real, the controls stay quiet', async (t) => {
  const { root, O, P, g } = realOrigin('guard-git-pass2-day-');
  const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
  const Q = join(root, 'q'), W = join(P, '.gate', 'wt'), M = join(root, 'missing'), N = join(root, 'nonrepo');
  const q = (...a) => spawnSync('git', a, { cwd: Q, env, encoding: 'utf8' });
  const bash = (line) => spawnSync('bash', ['--norc', '--noprofile', '-c', line], { cwd: P, env, encoding: 'utf8' });
  const prev = process.env.CLAUDE_PROJECT_DIR;
  const dir = mkdtempSync(join(tmpdir(), 'guard-mut-pass2-day-real-'));
  try {
    process.env.CLAUDE_PROJECT_DIR = P;
    const io = { ...realIo(), env: {} };
    g('switch', '-q', '-c', 'cs/x', 'origin/main');
    writeFileSync(join(P, 'docs', 'a.md'), 'x1\n');
    g('commit', '-q', '-am', 'x1');
    g('push', '-q', 'origin', 'cs/x');
    g('switch', '-q', 'main');
    g('branch', '-q', '-D', 'cs/x');
    g('fetch', '-q', 'origin');
    assert.equal(spawnSync('git', ['clone', '-q', O, Q], { env }).status, 0);
    q('switch', '-q', 'cs/x');
    writeFileSync(join(Q, '.claude', 'settings.json'), '{"a":3}\n');
    q('commit', '-q', '-am', 'x2');
    q('push', '-q', 'origin', 'cs/x');
    assert.equal(g('worktree', 'add', '-q', '--detach', W, 'main').status, 0, 'setup: a worktree under .gate/');
    mkdirSync(N);
    mkdirSync(join(P, '.gate', 'empty'));
    // [label, command, want]; every row from the main folder P
    const rows = [
      ['C3d', 'git fetch -q origin cs/x && git switch --detach origin/cs/x', ASK],
      ['WT-UPDATE', 'git fetch origin main && git merge --ff-only origin/main', null],
      ['step 0', 'git fetch -q origin main && git switch --detach origin/main', null],
      ['C1', `cd ${M} && cd ${W}; git switch -q -c cs/c1 origin/cs/evil`, ASK],
      ['C1c', `cd ${M} && cd ${N}; echo x > CLAUDE.md`, ASK],
      ['C2', `! cd ${P} && cd ./.gate/wt; git switch -q -c cs/c2 origin/cs/evil`, ASK],
      ['C3', `cd ${P} && cd ${M} && cd ${W}; git switch -q -c cs/c3 origin/cs/evil`, ASK],
      ['; control', `cd ${M}; cd ${W}; git switch -q -c cs/c0 origin/cs/evil`, null],
      ['rm -rf .gate', 'rm -rf .gate', ASK],
      ['rm -r .gate', 'rm -r .gate', ASK],
      ['rm -rf <abs>/.gate', `rm -rf ${P}/.gate`, ASK],
      ['mv .gate x', 'mv .gate x', ASK],
      ['rm -rf .gate/empty', 'rm -rf .gate/empty', null],
    ];
    // [label, from, to, the rows it must let through]
    const src = readFileSync(join(HERE, 'routine-guard.mjs'), 'utf8');
    const muts = [
      ['SD-3 a switch after an earlier git command asks', "else if (line.gitBefore && pos[0] !== 'origin/main') {", 'else if (false) {', ['C3d']],
      ['SD-4 a cd after && while the folder is unknown', "if (c.sep === '&&' && !dirs.known) {", 'if (false) {', ['C1', 'C1c', 'C3']],
      ['SD-4 ! does not reach a cd', "const CD_REACH = new Set(['if', '{', 'time',", "const CD_REACH = new Set(['if', '{', '!', 'time',", ['C2']],
      ['SD-6 a folder holding a worktree of this repository', 'if (wt !== undefined) return', 'if (false) return',
        ['rm -rf .gate', 'rm -r .gate', 'rm -rf <abs>/.gate', 'mv .gate x']],
    ];
    const mods = [['rules', { decide }]];
    for (const [i, [label, from, to]] of muts.entries()) {
      assert.equal(src.split(from).length - 1, 1, `real mutant "${label}": its target text occurs exactly once`);
      const file = join(dir, `d${i}.mjs`);
      writeFileSync(file, src.replace(from, () => to));
      mods.push([label, await import(pathToFileURL(file).href)]);
    }
    const seen = new Map(mods.map(([label]) => [label, new Map()]));
    for (const [label, command] of rows) {
      for (const [m, mod] of mods) seen.get(m).set(label, kindOf(await mod.decide(dsh(command, { cwd: P }), io)));
    }
    for (const [label, , want] of rows) assert.equal(seen.get('rules').get(label), want, `real row ${label}`);
    for (const [label] of muts) t.diagnostic(`${label}: lets through ${[...seen.get(label)].filter(([r, k]) => k === null && seen.get('rules').get(r) !== null).map(([r]) => r).join(', ')}`);
    const survivors = muts.filter(([label, , , through]) => !through.every((r) => seen.get(label).get(r) === null)).map(([label]) => label);
    assert.deepEqual(survivors, [], `real mutants a named row does not let through: ${survivors.join('; ')}`);
    // what the asks are about is real: real bash moves the main folder, or writes its CLAUDE.md
    const head = () => g('rev-parse', 'HEAD').stdout.trim();
    const mainHead = head();
    const settings = () => readFileSync(join(P, '.claude', 'settings.json'), 'utf8');
    for (const [label, branch] of [['C1', 'cs/c1'], ['C2', 'cs/c2'], ['C3', 'cs/c3']]) {
      bash(rows.find(([l]) => l === label)[1]);
      assert.equal(g('branch', '--show-current').stdout.trim(), branch, `real bash ${label}: the main folder switched to ${branch}`);
      assert.equal(settings(), '{"a":2}\n', `real bash ${label}: the main folder now runs cs/evil's settings`);
      g('switch', '-q', 'main');
    }
    bash(rows.find(([l]) => l === 'C1c')[1]);
    assert.equal(readFileSync(join(P, 'CLAUDE.md'), 'utf8'), 'x\n', 'real bash C1c: the main CLAUDE.md was overwritten');
    g('restore', '--', 'CLAUDE.md');
    bash(rows.find(([l]) => l === '; control')[1]);
    assert.equal(g('branch', '--show-current').stdout.trim(), 'main', 'real bash, the ; control: the main folder stayed on main');
    assert.equal(head(), mainHead);
    bash(rows.find(([l]) => l === 'C3d')[1]);
    assert.equal(settings(), '{"a":3}\n', 'real bash C3d: the fetch moved origin/cs/x, and the main folder landed on the kit change');
    g('switch', '-q', 'main');
    assert.equal(settings(), '{"a":1}\n');
  } finally {
    if (prev === undefined) delete process.env.CLAUDE_PROJECT_DIR; else process.env.CLAUDE_PROJECT_DIR = prev;
    rmSync(root, { recursive: true, force: true });
    rmSync(dir, { recursive: true, force: true });
  }
});

// --- 6. fix pass 3, routine side A (advisor-pass3 §2-§3): R-PLAIN, the date word, P3-4 ---
// R-PLAIN: a routine's Bash line is refused unless it is in the plain form the guard splits exactly as bash does, checked
// on the raw text before parseShell. The rows: the reviewers' lines that hid a command or a write from parseShell (P3-1
// comments: H, A; P3-2 here-documents and a backslash-newline: D; P3-3 ANSI-C quoting: Q; P3-9 >&word: R), one row per
// refused construct, and the controls (the routines' real lines). dateCheck: a +FORMAT word holds only letters, digits and
// % : _ space . , / -. P3-4: a git command on a line with a background &, or in a call run in the background, is refused.
const P3_LINES = { // security-auditor-pass3 (H, D, Q), skeptic-pass3-P3-1 (A), advisor-pass3 (R, HD, QX, BR)
  H1: "git fetch -q origin cs/ok # '\ngit switch -q cs/evil # '",
  H2: "git status # '\ngit switch -q cs/evil # '", // H8 is this line in a day session
  H5: "git log -1 --oneline # '\necho x > CLAUDE.md # '",
  H6: 'git status # "\ngit switch -q cs/evil # "',
  H7: "git status # '\necho x > CLAUDE.md # '",
  A1: "git status  # what's changed\ngit switch -q cs/evil",
  A2: "git status #'\ngit switch -q cs/evil #'",
  A3: "git status # it's\ngit push -q origin HEAD:main",
  A4: "git status # it's\ngit commit -qam m",
  D1: "git log -1 --oneline <<E\\ O\nE O\ngit switch -q cs/evil\nE",
  D2: "git log -1 --oneline <<'E O'\nE O\ngit switch -q cs/evil\nE",
  D3: "git status <<E\\ O\nE O\necho x > CLAUDE.md\nE",
  D4: "git log -1 <<E\\ O\nE O\necho x > CLAUDE.md\nE",
  D5: "git status \\\n# '\ngit switch -q cs/evil # '",
  HD: 'git commit -F - <<EOF\nm\nEOF',
  Q5: "date +$'\\''x\ngit switch -q cs/evil\n'",
  Q6: "date +$'\\''x\ngit switch -q cs/evil\ndate +'%s'",
  Q7: "date +$'\\''x\necho x > CLAUDE.md\ndate +'%s'",
  QX: "date +$'x'",
  R1: 'git status >&CLAUDE.md', // R4 is this line in a day session
  R2: 'git log -1 --oneline >&CLAUDE.md',
  R3: 'git status >&.claude/settings.json',
  BR: 'git add CLAUDE.{md,x}',
};
const bgSh = (command) => ev('Bash', { command, run_in_background: true });
const praFake = () => fakeIo({ role: PRA });
const dayFake = () => fakeIo({});
const DATE_ONLY = /^routine \(night-shift\): date reads the clock only \(date, date -u, date \+FORMAT\)$/;
const FG = (role) => new RegExp(`^routine \\(${role}\\): routines run git in the foreground \\(no background & or run_in_background on a line with git\\)$`);
const SESSION_TRAILER = '--trailer "Claude-Session: https://claude.ai/code/session_x"';
const HDOC = 'a here-document or here-string (<<)', GT_AMP = 'a >& before a word that is not a number or -', QNAME = 'a quoted NAME= before the program';
export const PASS3_ROUTINE_CASES = [
  // R-PLAIN: the reviewers' lines (P3-1, P3-2, P3-3, P3-9)
  ['P3-1 H1 night: a # comment with a quote hides a switch', sh(P3_LINES.H1), DENY, nightFake, PLAIN('a comment (#)')],
  ['P3-1 H2 night: git status, a # comment hides a switch', sh(P3_LINES.H2), DENY, nightFake, PLAIN('a comment (#)')],
  ['P3-1 H5 night: a # comment hides a write to CLAUDE.md', sh(P3_LINES.H5), DENY, nightFake, PLAIN('a comment (#)')],
  ['P3-1 H6 night: a # comment with a double quote hides a switch', sh(P3_LINES.H6), DENY, nightFake, PLAIN('a comment (#)')],
  ['P3-1 H7 pr-advisor: a # comment hides a write to CLAUDE.md', sh(P3_LINES.H7), DENY, praFake, PLAIN('a comment (#)', PRA)],
  ["P3-1 A1 night: an ordinary comment (# what's changed) hides a switch", sh(P3_LINES.A1), DENY, nightFake, PLAIN('a comment (#)')],
  ["P3-1 A2 night: #' with no space", sh(P3_LINES.A2), DENY, nightFake, PLAIN('a comment (#)')],
  ['P3-1 A3 night: a comment hides a push to main', sh(P3_LINES.A3), DENY, nightFake, PLAIN('a comment (#)')],
  ['P3-1 A4 night: a comment hides a commit -a', sh(P3_LINES.A4), DENY, nightFake, PLAIN('a comment (#)')],
  ['P3-2 D1 night: a here-document delimiter E\\ O hides a switch', sh(P3_LINES.D1), DENY, nightFake, PLAIN(HDOC)],
  ["P3-2 D2 night: a here-document delimiter 'E O' hides a switch", sh(P3_LINES.D2), DENY, nightFake, PLAIN(HDOC)],
  ['P3-2 D3 night: a here-document delimiter hides a write', sh(P3_LINES.D3), DENY, nightFake, PLAIN(HDOC)],
  ['P3-2 D4 pr-advisor: a here-document delimiter hides a write', sh(P3_LINES.D4), DENY, praFake, PLAIN(HDOC, PRA)],
  ["P3-2 D5 night: a backslash-newline, then # '", sh(P3_LINES.D5), DENY, nightFake, PLAIN('a backslash')],
  ['P3-2 night: a plain here-document (git commit -F - <<EOF)', sh(P3_LINES.HD), DENY, nightFake, PLAIN(HDOC)],
  ["P3-3 Q5 night: date +$'\\''x hides a switch (unterminated end)", sh(P3_LINES.Q5), DENY, nightFake, PLAIN('a $')],
  ["P3-3 Q6 night: date +$'\\''x hides a switch", sh(P3_LINES.Q6), DENY, nightFake, PLAIN('a $')],
  ["P3-3 Q7 pr-advisor: date +$'\\''x hides a write", sh(P3_LINES.Q7), DENY, praFake, PLAIN('a $', PRA)],
  ["P3-3 night: date +$'x'", sh(P3_LINES.QX), DENY, nightFake, PLAIN('a $')],
  ['P3-9 R1 night: git status >&CLAUDE.md', sh(P3_LINES.R1), DENY, nightFake, PLAIN(GT_AMP)],
  ['P3-9 R2 pr-advisor: git log -1 --oneline >&CLAUDE.md', sh(P3_LINES.R2), DENY, praFake, PLAIN(GT_AMP, PRA)],
  ['P3-9 R3 night: git status >&.claude/settings.json', sh(P3_LINES.R3), DENY, nightFake, PLAIN(GT_AMP)],
  ['R-PLAIN night: brace expansion (git add CLAUDE.{md,x})', sh(P3_LINES.BR), DENY, nightFake, PLAIN('a parenthesis, brace or bracket ({)')],
  // R-PLAIN: one row per refused construct
  ['R-PLAIN night: an unquoted #', sh('git status #x'), DENY, nightFake, PLAIN('a comment (#)')],
  ['R-PLAIN night: an unquoted $', sh('git log $x'), DENY, nightFake, PLAIN('a $')],
  ['R-PLAIN pr-advisor: an unquoted $', sh('git log $x'), DENY, praFake, PLAIN('a $', PRA)],
  ['R-PLAIN night: a backtick', sh('git log `x`'), DENY, nightFake, PLAIN('a backtick')],
  ['R-PLAIN night: a backslash', sh('git log a\\ b'), DENY, nightFake, PLAIN('a backslash')],
  ...['(', ')', '{', '}', '[', ']'].map((ch) => [`R-PLAIN night: an unquoted ${ch}`, sh(`git log a${ch}b`), DENY, nightFake,
    PLAIN(`a parenthesis, brace or bracket (${ch})`)]),
  ...['!', '*', '?'].map((ch) => [`R-PLAIN night: an unquoted ${ch}`, sh(`git add a${ch}b`), DENY, nightFake, PLAIN(`a ${ch}`)]),
  ['R-PLAIN night: a ~ that starts a word', sh('git log ~/x'), DENY, nightFake, PLAIN('a ~ that starts a word or follows = or :')],
  ['R-PLAIN night: a ~ after =', sh('git log --x=~/y'), DENY, nightFake, PLAIN('a ~ that starts a word or follows = or :')],
  ['R-PLAIN night: a ~ after :', sh('git log a:~/y'), DENY, nightFake, PLAIN('a ~ that starts a word or follows = or :')],
  ['R-PLAIN night: <<-', sh('git log <<-E\n\tE'), DENY, nightFake, PLAIN(HDOC)],
  ['R-PLAIN night: <<< (a here-string)', sh('git log <<<x'), DENY, nightFake, PLAIN(HDOC)],
  ['R-PLAIN night: <>', sh('git log <>x'), DENY, nightFake, PLAIN('a <> redirect')],
  ['R-PLAIN night: a carriage return', sh('git status\r'), DENY, nightFake, PLAIN('a control character (U+000D)')],
  ['R-PLAIN night: another control character (BEL)', sh('git status\x07'), DENY, nightFake, PLAIN('a control character (U+0007)')],
  ['R-PLAIN night: a tab inside single quotes', sh("git commit -m 'a\tb'"), DENY, nightFake, PLAIN('a control character (U+0009) in quotes')],
  ['R-PLAIN night: a carriage return inside double quotes', sh('git commit -m "a\rb"'), DENY, nightFake, PLAIN('a control character (U+000D) in quotes')],
  ['R-PLAIN night: >& and a space before the number', sh('git status >& 2'), DENY, nightFake, PLAIN(GT_AMP)],
  ['R-PLAIN night: >&2x', sh('git status >&2x'), DENY, nightFake, PLAIN(GT_AMP)],
  ['R-PLAIN night: a $ inside double quotes', sh('git commit -m "a$b"'), DENY, nightFake, PLAIN('a $ inside double quotes')],
  ['R-PLAIN night: a backtick inside double quotes', sh('git commit -m "a`b"'), DENY, nightFake, PLAIN('a ` inside double quotes')],
  ['R-PLAIN night: a backslash inside double quotes', sh('git commit -m "a\\b"'), DENY, nightFake, PLAIN('a \\ inside double quotes')],
  ['R-PLAIN night: an unclosed single quote', sh("git commit -m 'a"), DENY, nightFake, PLAIN('an unclosed quote')],
  ['R-PLAIN night: an unclosed double quote', sh('git commit -m "a'), DENY, nightFake, PLAIN('an unclosed quote')],
  // R-PLAIN narrowed where parseShell and bash read the form differently (fix pass 3, builder)
  ['R-PLAIN night: <& (parseShell reads its & as a background &)', sh('git status <&0'), DENY, nightFake, PLAIN('a <& redirect')],
  ["R-PLAIN night: '2'>x (bash passes 2 as an argument)", sh("git status '2'>x"), DENY, nightFake, PLAIN('a quoted word right before a redirect')],
  ['R-PLAIN night: "TZ=x" date (bash runs TZ=x as the program)', sh('"TZ=x" date'), DENY, nightFake, PLAIN(QNAME)],
  ['R-PLAIN night: T"Z=x" date', sh('T"Z=x" date'), DENY, nightFake, PLAIN(QNAME)],
  ['R-PLAIN night: >out "TZ=x" date (a redirect target is no program)', sh('>out "TZ=x" date'), DENY, nightFake, PLAIN(QNAME)],
  ['R-PLAIN night: 2>/dev/null "TZ=x" date (a descriptor number is no program)', sh('2>/dev/null "TZ=x" date'), DENY, nightFake, PLAIN(QNAME)],
  ['R-PLAIN night: git status; "TZ=x" date', sh('git status; "TZ=x" date'), DENY, nightFake, PLAIN(QNAME)],
  ['R-PLAIN night: git status, a newline, "TZ=x" date', sh('git status\n"TZ=x" date'), DENY, nightFake, PLAIN(QNAME)],
  ['R-PLAIN night: ;; (a case token)', sh('git status ;; git log'), DENY, nightFake, PLAIN('a ;;')],
  ['R-PLAIN night: ;& (a case token)', sh('git status ;& git log'), DENY, nightFake, PLAIN('a ;&')],
  ['R-PLAIN night: a non-ASCII letter outside quotes', sh('git log é'), DENY, nightFake, PLAIN('the character U+00E9')],
  // R-PLAIN: the controls (the routines' real lines go on; the checks after R-PLAIN apply as before)
  ["R-PLAIN pr-advisor: its step-0 line goes on", sh('git fetch -q origin main && git switch --detach origin/main'), null, praFake],
  ['R-PLAIN night: the NIGHT_NPM line goes on', sh('npm ci --prefix tools --ignore-scripts --no-audit --no-fund'), null, nightFake],
  ['R-PLAIN night: the checkpoint save line goes on', sh(`node .claude/scripts/checkpoint.mjs save kit-v3-3 -m "step 2: what's next" ${SESSION_TRAILER}`), null, nightFake],
  ['R-PLAIN night: git commit -q -m "fix #16: it\'s done" (docs staged) goes on', sh('git commit -q -m "fix #16: it\'s done"'), null, nightFake],
  ["R-PLAIN night: git commit -q -m 'a, newline, newline, b' goes on", sh("git commit -q -m 'a\n\nb'"), null, nightFake],
  ["R-PLAIN night: git log -1 --format='%h %s' goes on", sh("git log -1 --format='%h %s'"), null, nightFake],
  ["R-PLAIN pr-advisor: git log -1 --format='%h %s' goes on", sh("git log -1 --format='%h %s'"), null, praFake],
  ['R-PLAIN night: git log HEAD~1..HEAD goes on', sh('git log HEAD~1..HEAD'), null, nightFake],
  ['R-PLAIN night: git diff --name-only origin/main...HEAD goes on', sh('git diff --name-only origin/main...HEAD'), null, nightFake],
  ['R-PLAIN night: TZ=Europe/Amsterdam date +%H:%M goes on', sh('TZ=Europe/Amsterdam date +%H:%M'), null, nightFake],
  ['R-PLAIN night: TZ="Europe/Amsterdam" date goes on (the quote after =)', sh('TZ="Europe/Amsterdam" date'), null, nightFake],
  ["R-PLAIN night: date +'%Y-%m-%d %H:%M' goes on", sh("date +'%Y-%m-%d %H:%M'"), null, nightFake],
  ['R-PLAIN night: git status 2>&1 goes on', sh('git status 2>&1'), null, nightFake],
  ['R-PLAIN night: git status >&2 goes on', sh('git status >&2'), null, nightFake],
  ['R-PLAIN night: 2>&1 git status goes on', sh('2>&1 git status'), null, nightFake],
  ['R-PLAIN night: git status, a newline, git log -1 goes on', sh('git status\ngit log -1'), null, nightFake],
  ["R-PLAIN night: git log --grep '#x' goes on", sh("git log --grep '#x'"), null, nightFake],
  // moved by D-ASK (Mustafa's scope stop): a day comment asks, with D-ASK's reason, not R-PLAIN's (was null)
  ['R-PLAIN day: git status # note asks (D-ASK: a comment; not R-PLAIN)', sh('git status # note'), ASK, dayFake,
    /^kit and gate files ask first in a day session: the guard can't fully read this line \(a comment\)/],
  ['R-PLAIN day: echo a#b goes on', sh('echo a#b'), null, dayFake],
  // dateCheck: the +FORMAT word
  ["dateCheck night: date '+%s;x' (the ; is quoted)", sh("date '+%s;x'"), DENY, nightFake, DATE_ONLY],
  ['dateCheck night: date "+a\'b"', sh('date "+a\'b"'), DENY, nightFake, DATE_ONLY],
  ['dateCheck night: date +%s goes on', sh('date +%s'), null, nightFake],
  ["dateCheck night: date '+%H:%M' goes on", sh("date '+%H:%M'"), null, nightFake],
  ["dateCheck night: date -u +'%Y-%m-%d %H:%M' goes on", sh("date -u +'%Y-%m-%d %H:%M'"), null, nightFake],
  // P3-4: git runs in the foreground
  ['P3-4 night: git fetch -q origin cs/ok &', sh('git fetch -q origin cs/ok &'), DENY, nightFake, FG(NIGHT)],
  ['P3-4 night: git fetch -q origin cs/ok with run_in_background', bgSh('git fetch -q origin cs/ok'), DENY, nightFake, FG(NIGHT)],
  ['P3-4 pr-advisor: git fetch -q origin main with run_in_background', bgSh('git fetch -q origin main'), DENY, praFake, FG(PRA)],
  ['P3-4 night: the saver (checkpoint.mjs watch) with run_in_background goes on', bgSh(`node .claude/scripts/checkpoint.mjs watch kit-v3-3 ${SESSION_TRAILER}`), null, nightFake],
  ['P3-4 night: git status 2>&1 goes on', sh('git status 2>&1'), null, nightFake],
  ['P3-4 night: git status | node tools/gate.mjs goes on (a pipe is no background &)', sh('git status | node tools/gate.mjs'), null, nightFake],
  ['P3-4 night: date & goes on (no git)', sh('date &'), null, nightFake],
  ['P3-4 day: git fetch -q origin main & goes on (day sessions unchanged)', sh('git fetch -q origin main &'), null, dayFake],
];
for (const [name, input, want, io, re] of PASS3_ROUTINE_CASES) {
  test(`routine (fix pass 3) ${want ?? 'goes on'}: ${name}`, async () => {
    const d = await decide(input, io());
    assert.equal(kindOf(d), want, `${name}: ${JSON.stringify(d)}`);
    if (re) assert.match(d.ask ?? d.deny, re);
  });
}
test("fix pass 3: R-PLAIN's and P3-4's reasons, word for word", async () => {
  assert.equal((await decide(sh('git status # x'), nightFake())).deny, 'routine (night-shift): write the command plainly (a comment (#)): one command per line or joined with && ; values in quotes; no comments, $, backslashes, here-documents, braces, brackets or parentheses');
  assert.equal((await decide(bgSh('git fetch -q origin main'), praFake())).deny, 'routine (pr-advisor): routines run git in the foreground (no background & or run_in_background on a line with git)');
});
const PLAIN_OFF = 'if (role) { const found = plainForm(command); if (found) return deny(plainReason(role, found)); }';
const PASS3_ROUTINE_MUTANTS = [
  ['R-PLAIN removed', PLAIN_OFF, ''],
  ['R-PLAIN applied to day lines too', 'if (role) { const found = plainForm(command);', 'if (true) { const found = plainForm(command);'],
  ["R-PLAIN on parseShell's words, not the raw text", 'const found = plainForm(command);',
    "const found = plainForm(parseShell(command).cmds.map((c) => [...c.argv, ...c.redirects].join(' ')).join('\\n'));"],
  ['R-PLAIN class: comment', "if (c === '#') return 'a comment (#)';", ''],
  ['R-PLAIN class: dollar', "if (c === '$') return 'a $';", ''],
  ['R-PLAIN class: backtick', "if (c === '`') return 'a backtick';", ''],
  ['R-PLAIN class: backslash', "if (c === '\\\\') return 'a backslash';", ''],
  ['R-PLAIN class: parentheses, braces, brackets', "if ('(){}[]'.includes(c)) return", 'if (false) return'],
  ['R-PLAIN class: ! * ?', "if ('!*?'.includes(c)) return", 'if (false) return'],
  ['R-PLAIN class: ~ at a word start, after = or :', "if (i === start || src[i - 1] === '=' || src[i - 1] === ':') return", 'if (false) return'],
  ['R-PLAIN class: here-document', "if (src.startsWith('<<', i)) return", 'if (false) return'],
  ['R-PLAIN class: <>', "if (src.startsWith('<>', i)) return", 'if (false) return'],
  ['R-PLAIN class: <&', "if (src.startsWith('<&', i)) return", 'if (false) return'],
  ['R-PLAIN class: ;; and ;&', "if (src.startsWith(';;', i) || src.startsWith(';&', i)) return", 'if (false) return'],
  ['R-PLAIN class: a quoted word right before a redirect', "if ((c === '<' || c === '>') && quoted) return", 'if (false) return'],
  ['R-PLAIN class: >&word', "if (src.startsWith('>&', i)) {", 'if (false) {'],
  ['R-PLAIN class: double-quote content', "if (c === '\"' && PLAIN_DQ.test(body)) return", 'if (false) return'],
  ['R-PLAIN class: control characters', 'const PLAIN_CONTROL = /[\\x00-\\x09\\x0b-\\x1f\\x7f]/;', 'const PLAIN_CONTROL = /(?!)/;'],
  ['R-PLAIN class: an unclosed quote', "if (j < 0) return 'an unclosed quote';", 'if (j < 0) return null;'],
  ['R-PLAIN class: any other character', 'if (!PLAIN_CHAR.test(c)) return', 'if (false) return'],
  ['R-PLAIN class: a quoted NAME= before the program', "if (/['\"]/.test(w) && NAME_EQ.test(w.replace(/['\"]/g, ''))) return", 'if (false) return'],
  ['R-PLAIN: a descriptor number is no word', 'if (!w || (fdOp && /^\\d+$/.test(w))) return null;', 'if (!w) return null;'],
  ['R-PLAIN: a redirect target is no program', 'if (target) { target = false; return null; }', 'if (false) { target = false; return null; }'],
  ['R-PLAIN: an operator starts a new command', 'if (redirect) target = true; else { atCmd = true; target = false; }', 'if (redirect) target = true;'],
  ['R-PLAIN: a newline starts a new command', "if (c === '\\n') atCmd = true;", ''],
  ['dateCheck back to any +word', 'if (/^\\+[A-Za-z0-9%:_ .,\\/-]*$/.test(a)) continue;', "if (a.startsWith('+')) continue;"],
  ['P3-4 the & rule dropped', '(background || bgCall)', '(bgCall)'],
  ['P3-4 the run_in_background rule dropped', '(background || bgCall)', '(background)'],
  ['P3-4 background read from forks', '(background || bgCall)', '(forks || bgCall)'],
  ['P3-4 routines only', "if (role && prog === 'git' && (background || bgCall)) {", "if (prog === 'git' && (background || bgCall)) {"],
  ['P3-4 parseShell marks a background &', "if (c === '&') { background = true; endCmd(); i++; continue; }", "if (c === '&') { endCmd(); i++; continue; }"],
];
test('mutation runs (fix pass 3, routine side A): every new rule and refused class is needed by some case (kind and reason)', async () => {
  const src = readFileSync(join(HERE, 'routine-guard.mjs'), 'utf8');
  const dir = mkdtempSync(join(tmpdir(), 'guard-mut-pass3-'));
  const survivors = [];
  const right = (d, want, re) => kindOf(d) === want && (!re || re.test(d.ask ?? d.deny));
  for (const [name, input, want, io, re] of PASS3_ROUTINE_CASES) assert.ok(right(await decide(input, io()), want, re), `unmutated rules: ${name}`);
  try {
    for (const [i, [label, from, to]] of PASS3_ROUTINE_MUTANTS.entries()) {
      assert.equal(src.split(from).length - 1, 1, `mutant "${label}": its target text occurs exactly once`);
      const file = join(dir, `p${i}.mjs`);
      writeFileSync(file, src.replace(from, () => to));
      const m = await import(pathToFileURL(file).href);
      let caught = false;
      for (const [, input, want, io, re] of PASS3_ROUTINE_CASES) {
        let d;
        try { d = await m.decide(input, io()); } catch { caught = true; break; }
        if (!right(d, want, re)) { caught = true; break; }
      }
      if (!caught) survivors.push(label);
    }
  } finally { rmSync(dir, { recursive: true, force: true }); }
  assert.deepEqual(survivors, [], `mutants no fix pass 3 case catches: ${survivors.join('; ')}`);
});
// R-PLAIN refuses every role line that four of section 4's rules guarded (a substitution, an unquoted glob, a
// here-document, an expanding redirect target), so those mutants moved here: each must still be caught by the case
// table with R-PLAIN switched off too (the parser's checks stay a second layer under R-PLAIN).
const PLAIN_BEHIND_MUTANTS = [
  ['no expansion in a redirect target', 'if (/[$`*?[{]/.test(p) || /^~/.test(p)) return deny(', 'if (false) return deny('],
  ['no command substitution', 'if (role && subs) return deny(', 'if (false) return deny('],
  ['unquoted expansion (git add *)', 'if (role && c.expand) return deny(', 'if (false) return deny('],
  ['an unterminated here-document', 'if (role && unterminated) return deny(', 'if (false) return deny('],
];
test('mutation runs (fix pass 3): the role checks R-PLAIN now covers are still needed with R-PLAIN off', async () => {
  const src = readFileSync(join(HERE, 'routine-guard.mjs'), 'utf8');
  assert.equal(src.split(PLAIN_OFF).length - 1, 1, 'the R-PLAIN line occurs exactly once');
  const off = src.replace(PLAIN_OFF, () => '');
  const dir = mkdtempSync(join(tmpdir(), 'guard-mut-pass3-behind-'));
  const survivors = [];
  try {
    writeFileSync(join(dir, 'off.mjs'), off);
    const base = await import(pathToFileURL(join(dir, 'off.mjs')).href);
    for (const [name, role, input, refused, opts] of CASES) {
      assert.equal(!!(await base.decide(input, fakeIo({ role, ...opts }))), refused, `R-PLAIN off, unmutated: ${name}`);
    }
    for (const [i, [label, from, to]] of PLAIN_BEHIND_MUTANTS.entries()) {
      assert.equal(off.split(from).length - 1, 1, `mutant "${label}": its target text occurs exactly once`);
      const file = join(dir, `b${i}.mjs`);
      writeFileSync(file, off.replace(from, () => to));
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
  assert.deepEqual(survivors, [], `mutants no case catches with R-PLAIN off: ${survivors.join('; ')}`);
});
// Real git and bash (fix pass 3, the RB rows): realOrigin's project P (cs/evil changes .claude/settings.json). Each row
// is refused by R-PLAIN; in a copy with R-PLAIN off and dateCheck back to any +word, the guard reads another line than
// bash and lets it through (where parseShell at this head still misses the hidden command); then real bash runs the row and the hidden command
// really runs: the main folder lands on cs/evil, or CLAUDE.md or the live settings file is overwritten.
test('real git and bash (fix pass 3, R-PLAIN): the RB rows hide a command from parseShell; R-PLAIN refuses them', async (t) => {
  const { root, P, g } = realOrigin('guard-git-pass3-');
  const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
  const bash = (line) => spawnSync('bash', ['--norc', '--noprofile', '-c', line], { cwd: P, env, encoding: 'utf8' });
  const { parseShell } = await import(pathToFileURL(join(HERE, 'routine-guard.mjs')).href);
  const prev = process.env.CLAUDE_PROJECT_DIR;
  const dir = mkdtempSync(join(tmpdir(), 'guard-mut-pass3-real-'));
  try {
    process.env.CLAUDE_PROJECT_DIR = P;
    const ioFor = (role) => ({ ...realIo(), env: { SHAMBLE_ROLE: role } });
    const settings = () => readFileSync(join(P, '.claude', 'settings.json'), 'utf8');
    const rules = () => readFileSync(join(P, 'CLAUDE.md'), 'utf8');
    const branchNow = () => g('branch', '--show-current').stdout.trim();
    const switched = () => branchNow() === 'cs/evil' && settings() === '{"a":2}\n';
    const seesSwitch = (line) => parseShell(line).cmds.some((c) => c.argv[0] === 'git' && c.argv[1] === 'switch');
    const seesWrite = (f) => (line) => parseShell(line).cmds.some((c) => c.redirects.includes(f));
    // [label, role, line, the harm real bash does, parseShell sees the hidden command]
    const rows = [
      ['A1', NIGHT, P3_LINES.A1, switched, seesSwitch],
      ['D1', NIGHT, P3_LINES.D1, switched, seesSwitch],
      ['D4', PRA, P3_LINES.D4, () => rules() === 'x\n', seesWrite('CLAUDE.md')],
      ['Q6', NIGHT, P3_LINES.Q6, switched, seesSwitch],
      ['Q7', PRA, P3_LINES.Q7, () => rules() === 'x\n', seesWrite('CLAUDE.md')],
      ['R1', NIGHT, P3_LINES.R1, () => rules() !== 'rules\n', seesWrite('CLAUDE.md')],
      ['R3', NIGHT, P3_LINES.R3, () => !settings().startsWith('{'), seesWrite('.claude/settings.json')],
    ];
    const controls = [['pr-advisor step 0', PRA, 'git fetch -q origin main && git switch --detach origin/main'],
      ['night git status 2>&1', NIGHT, 'git status 2>&1'], ["night git log -1 --format='%h %s'", NIGHT, "git log -1 --format='%h %s'"]];
    const src = readFileSync(join(HERE, 'routine-guard.mjs'), 'utf8');
    assert.equal(src.split(PLAIN_OFF).length - 1, 1, 'the R-PLAIN line occurs exactly once');
    // the guard before fix pass 3's routine side A: R-PLAIN off, and dateCheck back to any +word
    const DATE_WORD = 'if (/^\\+[A-Za-z0-9%:_ .,\\/-]*$/.test(a)) continue;';
    assert.equal(src.split(DATE_WORD).length - 1, 1, 'the date word rule occurs exactly once');
    writeFileSync(join(dir, 'off.mjs'), src.replace(PLAIN_OFF, () => '').replace(DATE_WORD, () => "if (a.startsWith('+')) continue;"));
    const off = await import(pathToFileURL(join(dir, 'off.mjs')).href);
    const reset = () => { g('switch', '-q', '-f', 'main'); g('checkout', '-q', '--', '.'); };
    for (const [label, role, line, harm, sees] of rows) {
      const d = await decide(dsh(line, { cwd: P }), ioFor(role));
      assert.match(d?.deny ?? '', new RegExp(`^routine \\(${role}\\): write the command plainly \\(`), `real row ${label}: R-PLAIN refuses it`);
      const without = kindOf(await off.decide(dsh(line, { cwd: P }), ioFor(role)));
      if (sees(line)) t.diagnostic(`${label}: parseShell at this head sees the hidden command; with R-PLAIN off: ${without}`);
      else assert.equal(without, null, `real row ${label}: with R-PLAIN off the guard reads another line and lets it through`);
      bash(line);
      assert.ok(harm(), `real bash ${label}: the hidden command ran`);
      reset();
      assert.deepEqual([branchNow(), settings(), rules()], ['main', '{"a":1}\n', 'rules\n'], `reset after ${label}`);
    }
    for (const [label, role, line] of controls) assert.equal(kindOf(await decide(dsh(line, { cwd: P }), ioFor(role))), null, `control ${label}`);
  } finally {
    if (prev === undefined) delete process.env.CLAUDE_PROJECT_DIR; else process.env.CLAUDE_PROJECT_DIR = prev;
    rmSync(root, { recursive: true, force: true });
    rmSync(dir, { recursive: true, force: true });
  }
});
// C-PLAIN (fix pass 3, advisor-pass3 §3): on the plain form, parseShell's commands (argv after quote removal), in order,
// are what bash runs, and its redirect targets are the files bash creates. bash runs each line with a PATH of stub
// programs that log their argv (NUL-separated) and exit 0, in a folder of its own; the builtins the lines name are
// switched off (enable -n), so they log too. Input (a): every routine row of this file's tables that R-PLAIN accepts,
// and the routines' lines from NIGHT_SHIFT.md, CHECKPOINTS.md and main's advisor.md step 0; input (b): 2,000 seeded
// random plain-form lines (no ||). A pipeline's commands run at the same time, so they are compared without regard to
// order. Lines bash cannot run as written here are left out of (a) and named (|| after a stub that exits 0, a reserved
// word, a program named by a path, PATH or PWD set, a target outside the folder, a syntax error).
const KIT_LINES = [ // the routines' lines in the kit texts, placeholders filled
  'TZ=Europe/Amsterdam date', // NIGHT_SHIFT.md step 1
  'git ls-remote --heads origin cs/x', 'git fetch origin cs/x', 'git switch cs/x', // NIGHT_SHIFT.md step 4
  'git add docs/a.md', 'git commit -m "a message"', // NIGHT_SHIFT.md step 7 (fix pass 3)
  `node .claude/scripts/checkpoint.mjs save kit-v3-3 -m "step 1" ${SESSION_TRAILER}`, // CHECKPOINTS.md §3
  `node .claude/scripts/checkpoint.mjs watch kit-v3-3 ${SESSION_TRAILER}`, // CHECKPOINTS.md §5
  'git worktree prune', 'git worktree add .gate/kit-v3-3-wt cs/x', 'git log --oneline -5', // CHECKPOINTS.md §8
  'git ls-tree -r --name-only HEAD -- .checkpoints', // CHECKPOINTS.md §9
  'git fetch -q origin main && git switch --detach origin/main', // main's advisor.md step 0
  `git diff --name-only origin/main...${SHA}`, // main's advisor.md step 0
];
const seeded = (seed) => () => { // mulberry32
  seed = (seed + 0x6D2B79F5) | 0;
  let x = Math.imul(seed ^ (seed >>> 15), 1 | seed);
  x = (x + Math.imul(x ^ (x >>> 7), 61 | x)) ^ x;
  return ((x ^ (x >>> 14)) >>> 0) / 4294967296;
};
// one random plain-form line: pipelines (1-3 commands joined by | or |&) joined by ; && or a newline; returns the line
// and its pipelines' sizes
function plainLine(r, n) {
  const pick = (a) => a[Math.floor(r() * a.length)];
  const PLAIN_W = ['status', '-q', '--oneline', 'origin/main...HEAD', 'HEAD~1', 'a=b', '--format=%h', 'x,y', 'u@v', '+%H:%M', 'a^b',
    'k:v', '-1', '.', '..', 'docs/a.md', '100%', '--', 'cs/x'];
  const SQ = ['a b', 'it is', '#x', '<in>', 'a > b', 'x\ny', '"', '$x', '`x`', '\\', ';|&', 'é', '*', '', '  ', '&& b', '# c'];
  const DQ = ["it's", 'a b', '#16', '<a>', 'x > y', 'p\nq', ';|&', 'é', '*?', '', "'", "fix #16: it's done", '<<EOF'];
  const chunk = () => { const k = r(); return k < 0.45 ? pick(PLAIN_W) : k < 0.75 ? `'${pick(SQ)}'` : `"${pick(DQ)}"`; };
  const word = () => Array.from({ length: 1 + Math.floor(r() * 3) }, chunk).join('');
  let files = 0;
  const target = () => { files++; return pick([`f${n}x${files}`, `'f ${files}'`, `"g'${files}"`]); };
  const command = () => {
    const parts = [];
    if (r() < 0.2) parts.push(pick(['TZ=x', "A_1='v w'", 'B="p q"']));
    parts.push(pick(['git', 'gh', 'date', 'node', 'npm']));
    for (let k = Math.floor(r() * 5); k > 0; k--) parts.push(word());
    for (let k = Math.floor(r() * 3); k > 0; k--) {
      const to = target();
      parts.push(pick([`> ${to}`, `>> ${to}`, `&> ${to}`, `>${to}`, `2> ${to}`, `&>> ${to}`, `>| ${to}`, '2>&1', '>&2']));
    }
    return parts.map((p, k) => (k ? pick([' ', ' ', '  ', '\t']) : '') + p).join('');
  };
  const sizes = [];
  const pipes = Array.from({ length: 1 + Math.floor(r() * 4) }, () => {
    const k = 1 + Math.floor(r() * 3);
    sizes.push(k);
    return Array.from({ length: k }, command).join(pick([' | ', '|', ' |& ']));
  });
  return { line: pipes.reduce((a, p) => `${a}${pick([' ; ', ';', ' && ', '&&', '\n'])}${p}`), sizes };
}
test('C-PLAIN (fix pass 3): on the plain form, parseShell reads what bash runs and the files it writes', async (t) => {
  const ver = spawnSync('bash', ['--norc', '--noprofile', '-c', 'echo "${BASH_VERSINFO[0]} $BASH"'], { encoding: 'utf8' });
  const [major, BASH] = (ver.stdout ?? '').trim().split(' ');
  if (process.platform === 'win32' || ver.status !== 0 || Number(major) < 5) {
    t.skip('bash 5 is missing here: C-PLAIN runs on Linux with bash 5 (the routines\' shell)');
    return;
  }
  const started = Date.now();
  const { parseShell, plainForm } = await import(pathToFileURL(join(HERE, 'routine-guard.mjs')).href);
  const { readdirSync } = await import('node:fs');
  const { relative } = await import('node:path');
  // R-PLAIN refuses every P3-1, P3-2, P3-3 and P3-9 line
  for (const [label, line] of Object.entries(P3_LINES)) assert.notEqual(plainForm(line), null, `R-PLAIN refuses ${label}`);
  const root = realpathSync(mkdtempSync(join(tmpdir(), 'guard-cplain-')));
  try {
    const NAME = /^[A-Za-z_][A-Za-z0-9_]*=/;
    const program = (argv) => { let k = 0; while (k < argv.length && NAME.test(argv[k])) k++; return argv.slice(k); };
    // input (a)
    const lines = new Set(KIT_LINES);
    const add = (role, input) => { if ([NIGHT, PRA].includes(role) && input?.tool_name === 'Bash') lines.add(String(input.tool_input.command)); };
    for (const [, role, input] of CASES) add(role, input);
    for (const [, role, input] of ROUTINE_FIX_CASES) add(role, input);
    for (const [, input] of NIGHT_SWITCH_CASES) add(NIGHT, input);
    for (const [, input, , io] of [...PASS2_ROUTINE_CASES, ...PASS3_ROUTINE_CASES]) add(io().env.SHAMBLE_ROLE, input);
    const accepted = [...lines].filter((l) => plainForm(l) === null);
    const names = new Set(['git', 'gh', 'date', 'node', 'npm']);
    for (const l of accepted) for (const c of parseShell(l).cmds) { const p = program(c.argv)[0]; if (p && /^[A-Za-z0-9_.+-]+$/.test(p)) names.add(p); }
    const typeOf = new Map(spawnSync(BASH, ['--norc', '--noprofile', '-c', `PATH=/nonexistent; for n in ${[...names].join(' ')}; do echo "$n $(type -t "$n")"; done`],
      { encoding: 'utf8' }).stdout.trim().split('\n').map((s) => s.split(' ')));
    const outside = (f) => f.startsWith('/') || f.split('/').includes('..');
    const left = [];
    const corpus = accepted.filter((l) => {
      const cmds = parseShell(l).cmds;
      const why = cmds.some((c) => c.sep === '||') ? '|| (the stubs exit 0)'
        : cmds.some((c) => { const p = program(c.argv)[0]; return p !== undefined && !/^[A-Za-z0-9_.+-]+$/.test(p); }) ? 'a program named by a path'
          : cmds.some((c) => typeOf.get(program(c.argv)[0]) === 'keyword') ? 'a reserved word'
            : cmds.some((c) => ['enable', 'wait'].includes(program(c.argv)[0])) ? 'enable or wait (the harness uses them)'
              : cmds.some((c) => c.argv.some((w, k) => k < c.argv.length - program(c.argv).length && /^(PATH|PWD)=/.test(w))) ? 'PATH or PWD set'
                : cmds.some((c) => [...c.redirects, ...c.reads].some((f) => outside(f) && !/^\/dev\/(null|stdout|stderr)$/.test(f))) ? 'a target outside the folder'
                  : spawnSync(BASH, ['--norc', '--noprofile', '-n', '-c', l]).status !== 0 ? 'a syntax error' : null;
      if (why) left.push(`${JSON.stringify(l)}: ${why}`);
      return !why;
    });
    // input (b)
    const r = seeded(20261105);
    const random = Array.from({ length: 2000 }, (_, n) => plainLine(r, n));
    for (const { line } of random) assert.equal(plainForm(line), null, `the generator writes plain lines: ${JSON.stringify(line)}`);
    // the stubs: one script under every program name; a record is the argv, each word NUL-terminated, then \x01 NUL
    const bin = join(root, 'bin'), log = join(root, 'log');
    mkdirSync(bin);
    const stub = `#!/bin/sh\nprintf '%s\\0' "\${0##*/}" "$@" '\x01' >> '${log}'\n`;
    for (const n of [...names, '__mark']) { writeFileSync(join(bin, n), stub); spawnSync('chmod', ['755', join(bin, n)]); }
    const off = [...names].filter((n) => typeOf.get(n) === 'builtin');
    const items = [...corpus.map((line) => ({ line, sizes: null })), ...random];
    const dirOf = (k) => join(root, 'w', `L${k}`);
    const before = [];
    const walk = (d, base = d) => readdirSync(d, { withFileTypes: true }).flatMap((e) => (e.isDirectory() ? walk(join(d, e.name), base)
      : [relative(base, join(d, e.name))]));
    items.forEach(({ line }, k) => {
      mkdirSync(dirOf(k), { recursive: true });
      for (const c of parseShell(line).cmds) {
        for (const f of [...c.redirects, ...c.reads]) if (!outside(f)) mkdirSync(join(dirOf(k), dirname(f)), { recursive: true });
        for (const f of c.reads) if (!outside(f)) writeFileSync(join(dirOf(k), f), '');
      }
      before.push(new Set(walk(dirOf(k))));
    });
    const on = off.length ? `enable ${off.join(' ')}\n` : '', offLine = off.length ? `enable -n ${off.join(' ')}\n` : '';
    for (let s = 0; s < items.length; s += 400) {
      const script = items.slice(s, s + 400).map(({ line }, j) => `${on}cd -- '${dirOf(s + j)}' || exit 97\n${offLine}__mark ${s + j}\n${line}\nwait\n`).join('');
      const file = join(root, `batch${s}.sh`); // a file, not -c: one argument holds at most 128 KiB
      writeFileSync(file, script);
      const run = spawnSync(BASH, ['--norc', '--noprofile', file], { cwd: root, env: { PATH: bin, HOME: root, LC_ALL: 'C' }, encoding: 'utf8', maxBuffer: 1 << 26 });
      assert.equal(run.error, undefined, `bash ran the batch from line ${s}`);
      assert.notEqual(run.status, 97, 'the harness could not enter a line\'s folder');
    }
    // the log, split at the markers
    const ran = new Map();
    let cur = null;
    for (const rec of readFileSync(log, 'utf8').split('\x01\0').filter(Boolean)) {
      const argv = rec.split('\0').slice(0, -1);
      if (argv[0] === '__mark') { cur = Number(argv[1]); ran.set(cur, []); } else ran.get(cur).push(argv);
    }
    const sorted = (g) => g.map((a) => JSON.stringify(a)).sort();
    const grouped = (cmds, sizes) => { // ordered groups; a group's commands in any order
      const out = [];
      for (let k = 0, z = 0; k < cmds.length; z++) { const n = sizes ? sizes[z] ?? cmds.length : 1; out.push(sorted(cmds.slice(k, k + n))); k += n; }
      return out;
    };
    const bad = [];
    items.forEach(({ line, sizes }, k) => {
      const parsed = parseShell(line);
      const want = parsed.cmds.map((c) => program(c.argv)).filter((a) => a.length);
      const all = sizes ?? (parsed.forks ? [want.length] : null);
      const got = ran.get(k) ?? [];
      const files = parsed.cmds.flatMap((c) => c.redirects).filter((f) => !/^\/dev\//.test(f)).map((f) => join(f)).sort();
      const made = walk(dirOf(k)).filter((f) => !before[k].has(f)).sort();
      if (JSON.stringify(grouped(want, all)) !== JSON.stringify(grouped(got, all)) || JSON.stringify([...new Set(files)]) !== JSON.stringify(made)) {
        bad.push(`${JSON.stringify(line)}: parseShell ${JSON.stringify(want)} ${JSON.stringify(files)}; bash ${JSON.stringify(got)} ${JSON.stringify(made)}`);
      }
    });
    t.diagnostic(`C-PLAIN: ${corpus.length} corpus lines (of ${accepted.length} that R-PLAIN accepts) and ${random.length} random lines, ` +
      `${[...ran.values()].reduce((n, g) => n + g.length, 0)} commands run, in ${Date.now() - started} ms`);
    for (const l of left) t.diagnostic(`left out of the corpus: ${l}`);
    assert.deepEqual(bad.slice(0, 5), [], `${bad.length} lines where parseShell and bash differ`);
    assert.equal(ran.size, items.length, 'every line ran');
  } finally { rmSync(root, { recursive: true, force: true }); }
});

// --- fix pass 3, routine side B: the night commit allowlist (P3-5) and submodules never hidden (P3-6) ---
// P3-5: a night git commit takes only -m/--message, -a, -q, -s, --allow-empty and bundles of a, q, s with m last (the
// message attached or the next word), and needs a message; any other word (a path, --, a pathspec option, -i, -o, -p,
// --amend, -C, -F, --fixup) is refused, so the staged list (and the unstaged list with -a) is all git commits.
// [name, input, expected kind, io, reason pattern]
const COMMIT_ONLY = /^routine \(night-shift\): git commit here is only git commit \[-q\] \[-a\] \[-s\] \[--allow-empty\] -m <message>; stage files with git add in its own call first$/;
const DIRTY = () => nightFake({ unstaged: 'CLAUDE.md\n' }); // docs/a.md staged, CLAUDE.md modified and not staged
const DOCS = () => nightFake({ unstaged: 'docs/b.md\n' }); // docs changes only
const shIn = (command, cwd) => ev('Bash', { command }, { cwd });
export const PASS3B_COMMIT_CASES = [
  // refused: every word that is not on the list
  ['P3-5 S2a night: git commit -q -m m CLAUDE.md (a path without --)', sh('git commit -q -m m CLAUDE.md'), DENY, DIRTY, COMMIT_ONLY],
  ['P3-5 night: git commit -m m docs/a.md CLAUDE.md (two paths)', sh('git commit -m m docs/a.md CLAUDE.md'), DENY, DIRTY, COMMIT_ONLY],
  ['P3-5 S2b night: git commit -q -m m -i CLAUDE.md', sh('git commit -q -m m -i CLAUDE.md'), DENY, DIRTY, COMMIT_ONLY],
  ['P3-5 night: git commit -q -m m --include CLAUDE.md', sh('git commit -q -m m --include CLAUDE.md'), DENY, DIRTY, COMMIT_ONLY],
  ['P3-5 S2c night: git commit -q -m m -o CLAUDE.md', sh('git commit -q -m m -o CLAUDE.md'), DENY, DIRTY, COMMIT_ONLY],
  ['P3-5 night: git commit -q -m m --only CLAUDE.md', sh('git commit -q -m m --only CLAUDE.md'), DENY, DIRTY, COMMIT_ONLY],
  ['P3-5 S2f night: git commit -q -m m --pathspec-from-file=ps.txt', sh('git commit -q -m m --pathspec-from-file=ps.txt'), DENY, DIRTY, COMMIT_ONLY],
  ['P3-5 night: git commit -q -m m --pathspec-file-nul --pathspec-from-file=ps.nul', sh('git commit -q -m m --pathspec-file-nul --pathspec-from-file=ps.nul'), DENY, DIRTY, COMMIT_ONLY],
  ['P3-5 S2i night: git commit -q -m m -p', sh('git commit -q -m m -p'), DENY, DIRTY, COMMIT_ONLY],
  ['P3-5 night: git commit -q -m m --patch', sh('git commit -q -m m --patch'), DENY, DIRTY, COMMIT_ONLY],
  ['P3-5 night: git commit -q -m m --interactive', sh('git commit -q -m m --interactive'), DENY, DIRTY, COMMIT_ONLY],
  ['P3-5 night: git commit -q -m m -- . (the whole tree after --)', sh('git commit -q -m m -- .'), DENY, DIRTY, COMMIT_ONLY],
  ['P3-5 night: git commit -q -m m . (the whole tree, no --)', sh('git commit -q -m m .'), DENY, DIRTY, COMMIT_ONLY],
  ["P3-5 night: git commit -q -m m -- 'CLAUDE.*' (a glob after --)", sh("git commit -q -m m -- 'CLAUDE.*'"), DENY, DIRTY, COMMIT_ONLY],
  ["P3-5 S2g night: git commit -q -m m 'CLAUDE.*' (a glob, no --)", sh("git commit -q -m m 'CLAUDE.*'"), DENY, DIRTY, COMMIT_ONLY],
  ["P3-5 night: git commit -q -m m -- ':(top)CLAUDE.md' (pathspec magic after --)", sh("git commit -q -m m -- ':(top)CLAUDE.md'"), DENY, DIRTY, COMMIT_ONLY],
  ["P3-5 S2h night: git commit -q -m m ':(top)CLAUDE.md' (pathspec magic, no --)", sh("git commit -q -m m ':(top)CLAUDE.md'"), DENY, DIRTY, COMMIT_ONLY],
  ["P3-5 night: git commit -q -m m -- '*.md'", sh("git commit -q -m m -- '*.md'"), DENY, DIRTY, COMMIT_ONLY],
  ['P3-5 S2d night: git commit -q -m m -- CLAUDE.md (refused by the named list before; by the allowlist now)', sh('git commit -q -m m -- CLAUDE.md'), DENY, DIRTY, COMMIT_ONLY],
  ['P3-5 N3 night: git commit -q -m m -- settings.json from .claude/', shIn('git commit -q -m m -- settings.json', `${P}/.claude`), DENY,
    () => nightFake({ unstaged: '.claude/settings.json\n' }), COMMIT_ONLY],
  ['P3-5 night: git commit -m m -- (a bare -- after the message)', sh('git commit -m m --'), DENY, DOCS, COMMIT_ONLY],
  ['P3-5 night: git commit -m -- CLAUDE.md (-- is the message; CLAUDE.md is a path)', sh('git commit -m -- CLAUDE.md'), DENY, DIRTY, COMMIT_ONLY],
  ['P3-5 night: git commit --amend -m m', sh('git commit --amend -m m'), DENY, DOCS, COMMIT_ONLY],
  ['P3-5 night: git commit -C HEAD', sh('git commit -C HEAD'), DENY, DOCS, COMMIT_ONLY],
  ['P3-5 night: git commit -c HEAD', sh('git commit -c HEAD'), DENY, DOCS, COMMIT_ONLY],
  ['P3-5 night: git commit --fixup=HEAD', sh('git commit --fixup=HEAD'), DENY, DOCS, COMMIT_ONLY],
  ['P3-5 night: git commit -F msg.txt', sh('git commit -F msg.txt'), DENY, DOCS, COMMIT_ONLY],
  ['P3-5 night: git commit --file=msg.txt', sh('git commit --file=msg.txt'), DENY, DOCS, COMMIT_ONLY],
  ['P3-5 night: git commit -qo -m m (o in a bundle)', sh('git commit -qo -m m'), DENY, DOCS, COMMIT_ONLY],
  ['P3-5 night: git commit -om m (o in a bundle with m)', sh('git commit -om m'), DENY, DOCS, COMMIT_ONLY],
  ['P3-5 night: git commit -v -m m', sh('git commit -v -m m'), DENY, DOCS, COMMIT_ONLY],
  ['P3-5 night: git commit --no-verify -m m', sh('git commit --no-verify -m m'), DENY, DOCS, COMMIT_ONLY],
  ['P3-5 night: git commit --author=x -m m', sh('git commit --author=x -m m'), DENY, DOCS, COMMIT_ONLY],
  ['P3-5 night: git commit -q (no message)', sh('git commit -q'), DENY, DOCS, COMMIT_ONLY],
  ['P3-5 night: git commit (no word at all)', sh('git commit'), DENY, DOCS, COMMIT_ONLY],
  ['P3-5 night: git commit -qa (no message)', sh('git commit -qa'), DENY, DOCS, COMMIT_ONLY],
  ['P3-5 night: git commit -q -m (no message word)', sh('git commit -q -m'), DENY, DOCS, COMMIT_ONLY],
  ['P3-5 night: git commit -q --message (no message word)', sh('git commit -q --message'), DENY, DOCS, COMMIT_ONLY],
  // refused still: what git commits holds a kit file
  ['P3-5 night: git commit -m m, CLAUDE.md staged', sh('git commit -m m'), DENY, () => nightFake({ staged: 'docs/a.md\nCLAUDE.md\n' }), /the commit would change CLAUDE\.md/],
  ['P3-5 night: git commit -am m, CLAUDE.md modified', sh('git commit -am m'), DENY, DIRTY, /the commit would change CLAUDE\.md/],
  ['P3-5 night: git commit -q -a -m m, CLAUDE.md modified', sh('git commit -q -a -m m'), DENY, DIRTY, /the commit would change CLAUDE\.md/],
  ['P3-5 night: git commit -m -am (a message that looks like -a): the -a list is read too', sh('git commit -m -am'), DENY, DIRTY, /the commit would change CLAUDE\.md/],
  // go on: docs changes only
  ['P3-5 night: git commit -q -m "m"', sh('git commit -q -m "m"'), null, DOCS],
  ['P3-5 night: git commit -am m', sh('git commit -am m'), null, DOCS],
  ['P3-5 night: git commit -qam m', sh('git commit -qam m'), null, DOCS],
  ['P3-5 night: git commit --message=m', sh('git commit --message=m'), null, DOCS],
  ['P3-5 night: git commit --message m', sh('git commit --message m'), null, DOCS],
  ['P3-5 night: git commit -s -m a -m b', sh('git commit -s -m a -m b'), null, DOCS],
  ['P3-5 night: git commit --allow-empty -m m', sh('git commit --allow-empty -m m'), null, DOCS],
  ['P3-5 night: git commit -q --all --signoff --quiet -m m', sh('git commit -q --all --signoff --quiet -m m'), null, DOCS],
  ['P3-5 night: git commit -qs -mm (the message attached)', sh('git commit -qs -mm'), null, DOCS],
  ['P3-5 night: git commit -qamCLAUDE.md (CLAUDE.md is the attached message)', sh('git commit -qamCLAUDE.md'), null, DOCS],
  ['P3-5 night: git commit --message CLAUDE.md (CLAUDE.md is the message)', sh('git commit --message CLAUDE.md'), null, DOCS],
  ["P3-5 night: git commit -q -m 'fix: it is -- done' (the message holds --)", sh("git commit -q -m 'fix: it is -- done'"), null, DOCS],
  // unchanged: the day and the PR Advisor
  ['P3-5 day: git commit -m m docs/a.md goes on (night only)', sh('git commit -m m docs/a.md'), null, () => fakeIo()],
  ['P3-5 pr-advisor: git commit -m m (refused as before: the PR Advisor only reads)', sh('git commit -m m'), DENY, () => fakeIo({ role: PRA }), /the PR Advisor only reads/],
];
for (const [name, input, want, io, re] of PASS3B_COMMIT_CASES) {
  test(`routine (fix pass 3, P3-5) ${want ?? 'goes on'}: ${name}`, async () => {
    const d = await decide(input, io());
    assert.equal(kindOf(d), want, `${name}: ${JSON.stringify(d)}`);
    if (re) assert.match(d.ask ?? d.deny, re);
  });
}
test('fix pass 3 (P3-5): the night commit refusal, word for word', async () => {
  assert.equal((await decide(sh('git commit -q -m m CLAUDE.md'), DIRTY()))?.deny,
    'routine (night-shift): git commit here is only git commit [-q] [-a] [-s] [--allow-empty] -m <message>; stage files with git add in its own call first');
});
// The guard's lines the mutants below change (each occurs once in routine-guard.mjs)
const P3B = {
  allow: "  if (sub === 'commit' && role === 'night-shift' && !nightCommitPlain(args)) return deny(NIGHT_COMMIT_ONLY);\n",
  flags: '    if (NIGHT_COMMIT_FLAG.test(t) || /^-[aqs]+$/.test(t)) continue;',
  msgEq: "    if (t.startsWith('--message=')) { message = true; continue; }",
  bundleM: '/^-[aqs]*m([^]*)$/.exec(t)',
  next: "if (m[1] === '' && ++i >= args.length) return false;",
  ret: '  return message;\n}',
  names: "const DIFF_NAMES = ['--name-only', '-z', '--no-renames', '--no-relative', '--ignore-submodules=none'];",
};
const P3B_AFTER = (word) => `    if (t === '${word}') break; // the mutant: ${word} and every word after it accepted\n${P3B.flags}`;
const P3B_MUTANTS = [
  ['P3-5 the allowlist', P3B.allow, ''],
  ['P3-5 night only', P3B.allow, P3B.allow.replace("role === 'night-shift' && ", '')],
  ['P3-5 -- and what follows accepted', P3B.flags, P3B_AFTER('--')],
  ['P3-5 -i (with its paths) accepted', P3B.flags, P3B_AFTER('-i')],
  ['P3-5 -o (with its paths) accepted', P3B.flags, P3B_AFTER('-o')],
  ['P3-5 --pathspec-from-file accepted', P3B.msgEq, `    if (t.startsWith('--pathspec-from-file=')) continue;\n${P3B.msgEq}`],
  ['P3-5 the bundle letters widened to o', P3B.flags, P3B.flags.replace('[aqs]+', '[aqso]+')],
  ['P3-5 the bundle letters before m widened to o', P3B.bundleM, P3B.bundleM.replace('[aqs]*', '[aqso]*')],
  ['P3-5 a message is required', P3B.ret, '  return true;\n}'],
  ['P3-5 the next word is the message', P3B.next, "if (m[1] === '' && i + 1 >= args.length) return false;"],
  ['P3-5 a message attached to m', P3B.bundleM, '/^-[aqs]*m()$/.exec(t)'],
];
test('mutation runs (fix pass 3, P3-5): every rule of the night commit allowlist is needed by some case (kind and reason)', async () => {
  const src = readFileSync(join(HERE, 'routine-guard.mjs'), 'utf8');
  const dir = mkdtempSync(join(tmpdir(), 'guard-mut-pass3b-'));
  const survivors = [];
  const right = (d, want, re) => kindOf(d) === want && (!re || re.test(d.ask ?? d.deny));
  for (const [name, input, want, io, re] of PASS3B_COMMIT_CASES) assert.ok(right(await decide(input, io()), want, re), `unmutated rules: ${name}`);
  try {
    for (const [i, [label, from, to]] of P3B_MUTANTS.entries()) {
      assert.equal(src.split(from).length - 1, 1, `mutant "${label}": its target text occurs exactly once`);
      const file = join(dir, `b${i}.mjs`);
      writeFileSync(file, src.replace(from, () => to));
      const m = await import(pathToFileURL(file).href);
      let caught = false;
      for (const [, input, want, io, re] of PASS3B_COMMIT_CASES) {
        let d;
        try { d = await m.decide(input, io()); } catch { caught = true; break; }
        if (!right(d, want, re)) { caught = true; break; }
      }
      if (!caught) survivors.push(label);
    }
  } finally { rmSync(dir, { recursive: true, force: true }); }
  assert.deepEqual(survivors, [], `mutants no P3-5 case catches: ${survivors.join('; ')}`);
});
// Real git (fix pass 3, P3-5 and P3-6): realOrigin's project P.
// Commit rows run on a scratch branch cs/w from origin/main with docs/b.md staged and, unless a row sets up otherwise,
// CLAUDE.md modified and not staged. Each row is decided by the rules, by a copy without the allowlist (the RB copy:
// the commit check as it stands without P3-5) and by each mutant. A row marked harm is refused by the rules, let
// through by the RB copy, and then run by real bash: its commit holds the kit file. A row the rules let through is run
// too: its commit holds exactly the guard's lists (the staged names, and the unstaged ones with -a).
// P3-6 rows: the branch cs/sub adds a gitlink at .claude/sub with a .gitmodules entry `ignore = all`.
test('real git (fix pass 3, P3-5, P3-6): the night commit allowlist; submodules never hidden', async (t) => {
  const { root, P, g } = realOrigin('guard-git-pass3b-');
  const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
  const bash = (line, cwd) => spawnSync('bash', ['--norc', '--noprofile', '-c', line], { cwd, env, encoding: 'utf8', input: '' });
  const prev = process.env.CLAUDE_PROJECT_DIR;
  const dir = mkdtempSync(join(tmpdir(), 'guard-mut-pass3b-real-'));
  try {
    process.env.CLAUDE_PROJECT_DIR = P;
    const ioFor = (role) => ({ ...realIo(), env: role ? { SHAMBLE_ROLE: role } : {} });
    const ps = join(root, 'ps.txt'), psNul = join(root, 'ps.nul');
    writeFileSync(ps, 'CLAUDE.md\n');
    writeFileSync(psNul, 'CLAUDE.md\0');
    const names = (...a) => g('diff', '--name-only', '-z', ...a).stdout.split('\0').filter(Boolean).sort();
    const committed = () => g('show', '--name-only', '--format=', '-z', 'HEAD').stdout.split('\0').filter(Boolean).sort();
    const scratch = (more = () => writeFileSync(join(P, 'CLAUDE.md'), 'changed\n')) => () => {
      g('switch', '-q', '-c', 'cs/w', 'origin/main');
      writeFileSync(join(P, 'docs', 'b.md'), 'b\n');
      g('add', 'docs/b.md');
      more();
    };
    const back = () => { g('reset', '-q', '--hard'); g('switch', '-q', '-f', 'main'); g('branch', '-q', '-D', 'cs/w'); rmSync(join(P, '.gitmodules'), { force: true }); };
    const docsOnly = scratch(() => writeFileSync(join(P, 'docs', 'a.md'), 'a2\n'));
    // the P3-6 branch: a gitlink at .claude/sub, hidden by .gitmodules ignore = all
    const SUBMODULE = '[submodule "s"]\n\tpath = .claude/sub\n\turl = ./x\n\tignore = all\n';
    const gitlink = () => {
      writeFileSync(join(P, '.gitmodules'), SUBMODULE);
      g('update-index', '--add', '--cacheinfo', `160000,${g('rev-parse', 'origin/main').stdout.trim()},.claude/sub`);
      g('add', '.gitmodules');
    };
    g('switch', '-q', '-c', 'cs/sub', 'origin/main');
    gitlink();
    g('commit', '-q', '-m', 'sub');
    g('push', '-q', 'origin', 'cs/sub');
    // git reads `ignore` from the worktree's .gitmodules: the switch rows start on cs/gm, which adds only that file
    g('switch', '-q', '-c', 'cs/gm', 'origin/main');
    writeFileSync(join(P, '.gitmodules'), SUBMODULE);
    g('add', '.gitmodules');
    g('commit', '-q', '-m', 'gm');
    assert.deepEqual(names('origin/main', 'cs/sub', '--'), ['.gitmodules'], 'setup: ignore = all hides the gitlink from a plain diff');
    assert.deepEqual(names('--ignore-submodules=none', 'origin/main', 'cs/sub', '--'), ['.claude/sub', '.gitmodules'], 'setup: the flag lists it');
    g('switch', '-q', 'main');
    g('fetch', '-q', 'origin');
    const onGm = () => g('switch', '-q', 'cs/gm');
    const offGm = () => g('switch', '-q', 'main');
    // [label, role, command, cwd, want, setup, undo, harm: the kit file the RB copy's commit holds]
    const rows = [
      ['S2a', NIGHT, 'git commit -q -m m CLAUDE.md', P, DENY, scratch(), back, 'CLAUDE.md'],
      ['two paths', NIGHT, 'git commit -q -m m docs/b.md CLAUDE.md', P, DENY, scratch(), back, 'CLAUDE.md'],
      ['-i', NIGHT, 'git commit -q -m m -i CLAUDE.md', P, DENY, scratch(), back, 'CLAUDE.md'],
      ['--include', NIGHT, 'git commit -q -m m --include CLAUDE.md', P, DENY, scratch(), back, 'CLAUDE.md'],
      ['-o', NIGHT, 'git commit -q -m m -o CLAUDE.md', P, DENY, scratch(), back, 'CLAUDE.md'],
      ['--only', NIGHT, 'git commit -q -m m --only CLAUDE.md', P, DENY, scratch(), back, 'CLAUDE.md'],
      ['--pathspec-from-file', NIGHT, `git commit -q -m m --pathspec-from-file=${ps}`, P, DENY, scratch(), back, 'CLAUDE.md'],
      ['--pathspec-file-nul', NIGHT, `git commit -q -m m --pathspec-file-nul --pathspec-from-file=${psNul}`, P, DENY, scratch(), back, 'CLAUDE.md'],
      ['-p', NIGHT, 'git commit -q -m m -p', P, DENY, scratch(), back],
      ['--patch', NIGHT, 'git commit -q -m m --patch', P, DENY, scratch(), back],
      ['--interactive', NIGHT, 'git commit -q -m m --interactive', P, DENY, scratch(), back],
      ['-- .', NIGHT, 'git commit -q -m m -- .', P, DENY, scratch(), back, 'CLAUDE.md'],
      ['.', NIGHT, 'git commit -q -m m .', P, DENY, scratch(), back, 'CLAUDE.md'],
      ["-- 'CLAUDE.*'", NIGHT, "git commit -q -m m -- 'CLAUDE.*'", P, DENY, scratch(), back, 'CLAUDE.md'],
      ["-- ':(top)CLAUDE.md'", NIGHT, "git commit -q -m m -- ':(top)CLAUDE.md'", P, DENY, scratch(), back, 'CLAUDE.md'],
      ["-- '*.md'", NIGHT, "git commit -q -m m -- '*.md'", P, DENY, scratch(), back, 'CLAUDE.md'],
      ['-- CLAUDE.md', NIGHT, 'git commit -q -m m -- CLAUDE.md', P, DENY, scratch(), back, 'CLAUDE.md'],
      ['N3', NIGHT, 'git commit -q -m m -- settings.json', join(P, '.claude'), DENY, scratch(() => writeFileSync(join(P, '.claude', 'settings.json'), '{"a":9}\n')), back,
        '.claude/settings.json'],
      ['--amend', NIGHT, 'git commit --amend -m m', P, DENY, scratch(), back],
      ['-C HEAD', NIGHT, 'git commit -C HEAD', P, DENY, scratch(), back],
      ['--fixup=HEAD', NIGHT, 'git commit --fixup=HEAD', P, DENY, scratch(), back],
      ['-F msg.txt', NIGHT, 'git commit -F msg.txt', P, DENY, scratch(), back],
      ['-qo -m m', NIGHT, 'git commit -qo -m m', P, DENY, scratch(), back],
      ['still: CLAUDE.md staged', NIGHT, 'git commit -m m', P, DENY, scratch(() => { writeFileSync(join(P, 'CLAUDE.md'), 'changed\n'); g('add', 'CLAUDE.md'); }), back],
      ['still: -am, CLAUDE.md modified', NIGHT, 'git commit -am m', P, DENY, scratch(), back],
      ['null: -q -m "m"', NIGHT, 'git commit -q -m "m"', P, null, docsOnly, back],
      ['null: -am m', NIGHT, 'git commit -am m', P, null, docsOnly, back],
      ['null: -qam m', NIGHT, 'git commit -qam m', P, null, docsOnly, back],
      ['null: --message=m', NIGHT, 'git commit --message=m', P, null, docsOnly, back],
      ['null: -s -m a -m b', NIGHT, 'git commit -s -m a -m b', P, null, docsOnly, back],
      ['null: --allow-empty -m m', NIGHT, 'git commit --allow-empty -m m', P, null, docsOnly, back],
      ['P3-6 night switch', NIGHT, 'git switch cs/sub', P, DENY, onGm, offGm],
      ['P3-6 night push', NIGHT, 'git push origin cs/sub', P, DENY, () => g('switch', '-q', 'cs/sub'), offGm],
      ['P3-6 day switch in the main folder', DAY, 'git switch cs/sub', P, ASK, onGm, offGm],
      ['P3-6 night commit of a staged gitlink', NIGHT, 'git commit -m m', P, DENY, scratch(gitlink), back],
    ];
    // [label, from, to, the rows it must let through]
    const src = readFileSync(join(HERE, 'routine-guard.mjs'), 'utf8');
    const harmful = rows.filter((r) => r[7]).map(([label]) => label);
    const muts = [
      ['P3-5 the allowlist', P3B.allow, '', harmful],
      ['P3-5 -- and what follows accepted', P3B.flags, P3B_AFTER('--'), ['-- .', "-- 'CLAUDE.*'", "-- ':(top)CLAUDE.md'", "-- '*.md'", '-- CLAUDE.md', 'N3']],
      ['P3-5 -i (with its paths) accepted', P3B.flags, P3B_AFTER('-i'), ['-i']],
      ['P3-5 -o (with its paths) accepted', P3B.flags, P3B_AFTER('-o'), ['-o']],
      ['P3-5 --pathspec-from-file accepted', P3B.msgEq, `    if (t.startsWith('--pathspec-from-file=')) continue;\n${P3B.msgEq}`, ['--pathspec-from-file']],
      ['P3-6 --ignore-submodules=none', P3B.names, P3B.names.replace(", '--ignore-submodules=none'", ''),
        ['P3-6 night switch', 'P3-6 night push', 'P3-6 day switch in the main folder', 'P3-6 night commit of a staged gitlink']],
    ];
    const mods = [['rules', { decide }]];
    for (const [i, [label, from, to]] of muts.entries()) {
      assert.equal(src.split(from).length - 1, 1, `real mutant "${label}": its target text occurs exactly once`);
      const file = join(dir, `r${i}.mjs`);
      writeFileSync(file, src.replace(from, () => to));
      mods.push([label, await import(pathToFileURL(file).href)]);
    }
    const rb = mods[1][1]; // the RB copy: the allowlist removed
    const seen = new Map(mods.map(([label]) => [label, new Map()]));
    for (const [label, role, command, cwd, want, setup, undo, harm] of rows) {
      setup?.();
      try {
        for (const [m, mod] of mods) seen.get(m).set(label, kindOf(await mod.decide(dsh(command, { cwd }), ioFor(role))));
        if (harm) {
          assert.equal(seen.get('rules').get(label), DENY, `real row ${label}: refused`);
          assert.equal(kindOf(await rb.decide(dsh(command, { cwd }), ioFor(role))), null, `real row ${label}: the RB copy lets it through`);
          assert.equal(bash(command, cwd).status, 0, `real bash ${label}: the commit ran`);
          assert.ok(committed().includes(harm), `real bash ${label}: the commit holds ${harm} (${committed().join(', ')})`);
        } else if (want === null) {
          const listed = [...new Set([...names('--cached'), ...(command.split(' ').some((w) => w === '--all' || /^-[a-z]*a[a-z]*$/.test(w)) ? names() : [])])].sort();
          assert.equal(bash(command, cwd).status, 0, `real bash ${label}: the commit ran`);
          assert.deepEqual(committed(), listed, `real bash ${label}: the commit holds exactly the guard's lists`);
        }
      } finally { undo?.(); }
    }
    for (const [label, , , , want] of rows) assert.equal(seen.get('rules').get(label), want, `real row ${label}`);
    for (const [label] of muts) t.diagnostic(`${label}: lets through ${[...seen.get(label)].filter(([r, k]) => k === null && seen.get('rules').get(r) !== null).map(([r]) => r).join(', ')}`);
    const survivors = muts.filter(([label, , , through]) => !through.every((r) => seen.get(label).get(r) === null)).map(([label]) => label);
    assert.deepEqual(survivors, [], `real mutants a named row does not let through: ${survivors.join('; ')}`);
    assert.deepEqual([g('branch', '--show-current').stdout.trim(), readFileSync(join(P, 'CLAUDE.md'), 'utf8')], ['main', 'rules\n'], 'the project is back on main');
  } finally {
    if (prev === undefined) delete process.env.CLAUDE_PROJECT_DIR; else process.env.CLAUDE_PROJECT_DIR = prev;
    rmSync(root, { recursive: true, force: true });
    rmSync(dir, { recursive: true, force: true });
  }
});

// ---------- 8. fix pass 3, the day side (Mustafa's scope stop, 2026-11-05): D-ASK and P3-7 ----------
// D-ASK: the guard will never parse all of bash, so parseShell learns no new syntax. It only notices, where its reading
// still agrees with bash's, the first of five forms it does not read (a comment, a line continuation, $'...', a
// here-document delimiter that is not a plain word, >& or <& before a file name) and names it in `unreadable`. In a day
// session the main session is asked and an agent refused; the scan goes on, so a later deny in the line still wins.
// Routines are unchanged: R-PLAIN refuses all five first. P3-7: git clone's targets are the positional words after the
// first and the git-dir values; the folder git makes from the URL only from the only positional word, and from the
// second one after an option the guard does not know.
const F_COMMENT = 'a comment', F_CONT = 'a line continuation', F_ANSI = "ANSI-C quoting ($'...')";
const F_HDOC = 'a here-document delimiter that is not a plain word', F_FD = '>& or <& before a file name';
const UNREAD = (form) => new RegExp(`^${reEsc(`kit and gate files ask first in a day session: the guard can't fully read this line (${form}), so it asks instead of guessing; write it without ${form}, or approve it`)}$`);
const UNREAD_AGENT = (form) => new RegExp(`^${reEsc(`kit and gate files are edited by the main session, which asks Mustafa first; an agent writes its shell lines without ${form} (the guard can't fully read them)`)}$`);
const NOT_DASK = /^(?!kit and gate files ask first in a day session: the guard can't fully read)/;
const EVIL = { diffs: { 'cs/evil': '.claude/settings.json\n' } }; // cs/evil differs from main in a kit file
const AG = { agent_id: 'a1' };
const DL = { // the day lines (security-auditor-pass3 H8, skeptic-pass3-P3-1 A1, D5, advisor-pass3 §3, R4)
  H8: P3_LINES.H2,
  A1: P3_LINES.A1,
  D5: P3_LINES.D5,
  AC: "printf $'a\\'b'; git switch -q cs/evil",
  HQ: "cat <<'E O'\nE O\ngit switch -q cs/evil\nE",
  R4: P3_LINES.R1,
};
// [name, input, expected kind, dayIo options, reason pattern]
const PASS3_DAY_CASES = [
  // D-ASK: each form asks for the main session and is refused for an agent
  ["D-ASK H8 day: git status # ', then a main-folder switch: ask", dsh(DL.H8), ASK, EVIL, UNREAD(F_COMMENT)],
  ['D-ASK H8 an agent: deny', dsh(DL.H8, AG), DENY, EVIL, UNREAD_AGENT(F_COMMENT)],
  ["D-ASK A1 day: # what's changed, then a main-folder switch: ask", dsh(DL.A1), ASK, EVIL, UNREAD(F_COMMENT)],
  ['D-ASK A1 an agent: deny', dsh(DL.A1, AG), DENY, EVIL, UNREAD_AGENT(F_COMMENT)],
  ["D-ASK D5 day: a line continuation, then # ': ask", dsh(DL.D5), ASK, EVIL, UNREAD(F_CONT)],
  ["D-ASK day: $'b', then a comment (the first form is named): ask", dsh("echo $'b' # a"), ASK, {}, UNREAD(F_ANSI)],
  ['D-ASK D5 an agent: deny', dsh(DL.D5, AG), DENY, EVIL, UNREAD_AGENT(F_CONT)],
  ["D-ASK day: printf $'a\\'b'; git switch -q cs/evil: ask", dsh(DL.AC), ASK, EVIL, UNREAD(F_ANSI)],
  ["D-ASK an agent: printf $'a\\'b'; git switch -q cs/evil: deny", dsh(DL.AC, AG), DENY, EVIL, UNREAD_AGENT(F_ANSI)],
  ["D-ASK day: cat <<'E O' hides a main-folder switch: ask", dsh(DL.HQ), ASK, EVIL, UNREAD(F_HDOC)],
  ["D-ASK an agent: cat <<'E O' ...: deny", dsh(DL.HQ, AG), DENY, EVIL, UNREAD_AGENT(F_HDOC)],
  ['D-ASK R4 day: git status >&CLAUDE.md: ask', dsh(DL.R4), ASK, {}, UNREAD(F_FD)],
  ['D-ASK R4 an agent: deny', dsh(DL.R4, AG), DENY, {}, UNREAD_AGENT(F_FD)],
  // D-ASK, more of each form
  ['D-ASK day: git status # note (a plain comment): ask', dsh('git status # note'), ASK, {}, UNREAD(F_COMMENT)],
  ['D-ASK day: a comment line first: ask', dsh('# note\ngit status'), ASK, {}, UNREAD(F_COMMENT)],
  ["D-ASK day: #' right after a ;: ask", dsh("git status;#'\ngit switch -q cs/evil"), ASK, EVIL, UNREAD(F_COMMENT)],
  ['D-ASK day: a line continuation outside quotes: ask', dsh('git status \\\n  -s'), ASK, {}, UNREAD(F_CONT)],
  ['D-ASK day: a line continuation inside double quotes: ask', dsh('echo "a\\\nb"'), ASK, {}, UNREAD(F_CONT)],
  ["D-ASK day: $' inside a word: ask", dsh("echo a$'b'"), ASK, {}, UNREAD(F_ANSI)],
  ['D-ASK D1 day: <<E\\ O (an escaped space) hides a main-folder switch: ask', dsh(P3_LINES.D1), ASK, EVIL, UNREAD(F_HDOC)],
  ['D-ASK day: <<E""O: ask', dsh('cat <<E""O\nEO\ngit switch -q cs/evil'), ASK, EVIL, UNREAD(F_HDOC)],
  ['D-ASK day: <<-"E O": ask', dsh('cat <<-"E O"\n\tE O\ngit switch -q cs/evil\nE'), ASK, EVIL, UNREAD(F_HDOC)],
  ["D-ASK day: <<'E'OF (quoted in part): ask", dsh("cat <<'E'OF\nb\nEOF"), ASK, {}, UNREAD(F_HDOC)],
  ['D-ASK day: git status >& CLAUDE.md (a space after >&): ask', dsh('git status >& CLAUDE.md'), ASK, {}, UNREAD(F_FD)],
  ['D-ASK day: git status >&2x (the word goes on after the digits): ask', dsh('git status >&2x'), ASK, {}, UNREAD(F_FD)],
  ['D-ASK day: cat <&CLAUDE.md: ask', dsh('cat <&CLAUDE.md'), ASK, {}, UNREAD(F_FD)],
  ['D-ASK day: a comment, then git push -f: the later deny still wins', dsh('git status # x\ngit push -f origin cs/x'), DENY, {}, NOT_DASK],
  // D-ASK controls: the guard reads these as bash does (no ask)
  ['D-ASK control: echo a#b goes on', dsh('echo a#b'), null],
  ["D-ASK control: git log --grep '#x' goes on", dsh("git log --grep '#x'"), null],
  ['D-ASK control: echo "${#x}" goes on', dsh('echo "${#x}"'), null],
  ['D-ASK control: echo x\\ #y (an escaped space, then #) goes on', dsh('echo x\\ #y'), null],
  ["D-ASK control: echo 'a\\<newline>b' (inside single quotes) goes on", dsh("echo 'a\\\nb'"), null],
  ['D-ASK control: echo "a$\'b" ($\' inside double quotes) goes on', dsh('echo "a$\'b"'), null],
  ['D-ASK control: cat <<EOF with a body goes on', dsh('cat <<EOF\nbody\nEOF'), null],
  ['D-ASK control: cat <<-EOF goes on', dsh('cat <<-EOF\n\tbody\n\tEOF'), null],
  ["D-ASK control: cat <<'EOF' goes on", dsh("cat <<'EOF'\nb\nEOF"), null],
  ['D-ASK control: cat <<"EOF" goes on', dsh('cat <<"EOF"\nb\nEOF'), null],
  ['D-ASK control: cat <<\\EOF goes on', dsh('cat <<\\EOF\nb\nEOF'), null],
  ['D-ASK control: git status 2>&1 goes on', dsh('git status 2>&1'), null],
  ['D-ASK control: git status >&2 goes on', dsh('git status >&2'), null],
  ['D-ASK control: git status >&- goes on', dsh('git status >&-'), null],
  ['D-ASK control, an agent: echo a#b goes on', dsh('echo a#b', AG), null],
  ['D-ASK control, an agent: git status 2>&1 goes on', dsh('git status 2>&1', AG), null],
  // P3-7: git clone's targets
  ['P3-7 git clone <origin> .gate/tools goes on', dsh('git clone /tmp/o/origin.git .gate/tools'), null],
  ['P3-7 git clone <url of tools.git> .gate/t goes on', dsh('git clone https://example.invalid/a/tools.git .gate/t'), null],
  ['P3-7 git clone <url of tools.git> /tmp/t goes on', dsh('git clone https://example.invalid/o/tools.git /tmp/t'), null],
  ['P3-7 git clone -q <url> .gate/tools (a known flag) goes on', dsh('git clone -q https://example.invalid/o/r.git .gate/tools'), null],
  ['P3-7 git clone --depth=1 <url> .gate/tools (an option with =) goes on', dsh('git clone --depth=1 https://example.invalid/o/r.git .gate/tools'), null],
  ['P3-7 git clone --frobnicate x <url of tools.git> (an unknown option): ask', dsh('git clone --frobnicate x https://example.invalid/o/tools.git'), ASK, {}, /\btools\b/],
  ['P3-7 an agent: git clone --frobnicate x <url of tools.git>: deny', dsh('git clone --frobnicate x https://example.invalid/o/tools.git', AG), DENY],
  ['P3-7 git clone <url of tools.git> with no folder: ask', dsh('git clone https://example.invalid/o/tools.git'), ASK, {}, /\btools\b/],
  ['P3-7 git clone --revision main <url> .claude/sub still asks', dsh('git clone --revision main https://example.invalid/o/r.git .claude/sub'), ASK],
  // P3-8 (named, no code; SD-3's shape from fix pass 2): a fetch earlier on the line can move local main
  ['P3-8 day: git stash && git switch -q main from the main folder: ask', dsh('git stash && git switch -q main'), ASK, {}, NOT_DASK],
];
for (const [name, input, want, opts, re] of PASS3_DAY_CASES) {
  test(`day (fix pass 3) ${want ?? 'goes on'}: ${name}`, async () => {
    const d = await decide(input, dayIo(opts));
    assert.equal(kindOf(d), want, `${name}: ${JSON.stringify(d)}`);
    if (re) assert.match(d.ask ?? d.deny, re);
  });
}
test("fix pass 3 (D-ASK): the two reasons, word for word", async () => {
  assert.equal((await decide(dsh('git status # note'), dayIo())).ask, "kit and gate files ask first in a day session: the guard can't fully read this line (a comment), so it asks instead of guessing; write it without a comment, or approve it");
  assert.equal((await decide(dsh('git status # note', AG), dayIo())).deny, "kit and gate files are edited by the main session, which asks Mustafa first; an agent writes its shell lines without a comment (the guard can't fully read them)");
});
test('fix pass 3 (D-ASK): parseShell names the first form it does not read, and reads the rest of the line as before', async () => {
  const { parseShell } = await import(pathToFileURL(join(HERE, 'routine-guard.mjs')).href);
  const rows = [[DL.H8, F_COMMENT], [DL.D5, F_CONT], [DL.AC, F_ANSI], [DL.HQ, F_HDOC], [DL.R4, F_FD], ['cat <&x', F_FD],
    ["git status # a\necho $'b'", F_COMMENT], ["echo $'b' # a", F_ANSI], ['echo $(git status # x\n)', F_COMMENT],
    ['echo a#b', null], ['cat <<EOF\n# x\nEOF', null], ['git status 2>&1 >&-', null], ['echo "${#x}"', null]];
  for (const [line, form] of rows) assert.equal(parseShell(line).unreadable, form, JSON.stringify(line));
  // the other fields are what the reader gave before D-ASK (no new syntax): the hidden switch stays unseen
  const h8 = parseShell(DL.H8);
  assert.deepEqual(h8.cmds.map((c) => c.argv), [['git', 'status', '#', "\ngit switch -q cs/evil # "]]);
  assert.deepEqual(parseShell(DL.R4).cmds.map((c) => [c.argv, c.redirects]), [[['git', 'status', 'CLAUDE.md'], []]]);
});
const DASK_ASK = 'dayAsk ??= ask("kit and gate files ask first in a day session: the guard can\'t fully read this line " +';
const PASS3_DAY_MUTANTS = [
  ['D-ASK detection: a comment', "if (c === '#' && word === null) odd('a comment');", ''],
  ['D-ASK detection: a line continuation, outside quotes', String.raw`if (c === '\\' && s[i + 1] === '\n') odd('a line continuation');`, ''],
  ['D-ASK detection: a line continuation, inside double quotes', String.raw`if (s[j] === '\\' && s[j + 1] === '\n') odd('a line continuation');`, ''],
  ["D-ASK detection: $'", `if (c === '$' && s[i + 1] === "'") odd("ANSI-C quoting ($'...')");`, ''],
  ['D-ASK detection: a here-document delimiter', 'if (!PLAIN_DELIM.test(s.slice(from, i))) odd(', 'if (false) odd('],
  ['D-ASK: a quoted or backslash-led plain delimiter stays readable', String.raw`const PLAIN_DELIM = /^(?:[A-Za-z0-9_.-]+|'[A-Za-z0-9_.-]+'|"[A-Za-z0-9_.-]+"|\\[A-Za-z0-9_.-]+)$/;`,
    'const PLAIN_DELIM = /^[A-Za-z0-9_.-]+$/;'],
  ['D-ASK detection: >&', "if (s[i] === '&' && !fdOnly(s, i + 1)) odd(", 'if (false) odd('],
  ['D-ASK detection: <&', "if (s[i + 1] === '&' && !fdOnly(s, i + 2)) odd(", 'if (false) odd('],
  ['D-ASK fdOnly: the word ends after the digits', 'return e > k && (e === s.length || FD_END.test(s[e]));', 'return e > k;'],
  ['D-ASK fdOnly: a digit or - comes first', 'return e > k && (e === s.length || FD_END.test(s[e]));', 'return e === s.length || FD_END.test(s[e]);'],
  ['D-ASK names the first form found', 'unreadable ??= form;', 'unreadable = form;'],
  ['D-ASK parseShell reports unreadable', 'return { cmds, subs, unterminated, forks, background, unreadable };',
    'return { cmds, subs, unterminated, forks, background, unreadable: null };'],
  ['D-ASK the day ask', DASK_ASK, 'void ("" +'],
  ['D-ASK the agent refusal', 'if (input.agent_id) {\n      return deny(`${KIT_AGENT}; an agent writes its shell lines', 'if (false) {\n      return deny(`${KIT_AGENT}; an agent writes its shell lines'],
  ['D-ASK the scan goes on (a later deny still wins)', DASK_ASK, DASK_ASK.replace('dayAsk ??= ask(', 'return ask(')],
  ['P3-7 back to every positional word', 'const made = [...(pos.length === 1 ? [pos[0]] : []), ...(unknown && pos.length > 1 ? [pos[1]] : [])]', 'const made = pos'],
  ['P3-7 the only positional word', '...(pos.length === 1 ? [pos[0]] : [])', '...[]'],
  ['P3-7 the second positional word after an unknown option', '...(unknown && pos.length > 1 ? [pos[1]] : [])', '...[]'],
  ['P3-7 a known flag is not unknown', '!CLONE_FLAGS.has(w)) unknown = true', 'true) unknown = true'],
  ['P3-7 an option with = is not unknown', "if (!w.includes('=') && !CLONE_FLAGS.has(w))", 'if (!CLONE_FLAGS.has(w))'],
];
test('mutation runs (fix pass 3, day side): every D-ASK and P3-7 rule is needed by some case (kind and reason)', async () => {
  const src = readFileSync(join(HERE, 'routine-guard.mjs'), 'utf8');
  const dir = mkdtempSync(join(tmpdir(), 'guard-mut-pass3-day-'));
  const survivors = [];
  const right = (d, want, re) => kindOf(d) === want && (!re || re.test(d.ask ?? d.deny));
  for (const [name, input, want, opts, re] of PASS3_DAY_CASES) assert.ok(right(await decide(input, dayIo(opts)), want, re), `unmutated rules: ${name}`);
  try {
    for (const [i, [label, from, to]] of PASS3_DAY_MUTANTS.entries()) {
      assert.equal(src.split(from).length - 1, 1, `mutant "${label}": its target text occurs exactly once`);
      const file = join(dir, `q${i}.mjs`);
      writeFileSync(file, src.replace(from, () => to));
      const m = await import(pathToFileURL(file).href);
      let caught = false;
      for (const [, input, want, opts, re] of PASS3_DAY_CASES) {
        let d;
        try { d = await m.decide(input, dayIo(opts)); } catch { caught = true; break; }
        if (!right(d, want, re)) { caught = true; break; }
      }
      if (!caught) survivors.push(label);
    }
  } finally { rmSync(dir, { recursive: true, force: true }); }
  assert.deepEqual(survivors, [], `mutants no fix pass 3 day case catches: ${survivors.join('; ')}`);
});
// The RB rows on the day side (advisor-pass3 §3): on a real repository, the guard without D-ASK (the D-ASK block
// removed: for these rows the guard at 01b158c reads them the same way) lets each row through, and real bash does the
// harm: the main folder lands on cs/evil (its settings), or CLAUDE.md is overwritten. The guard at this head asks.
test('real git and bash (fix pass 3, D-ASK): the day rows hide a command or a write; the guard asks', async () => {
  const { root, P, g } = realOrigin('guard-git-pass3-day-');
  const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
  const bash = (line) => spawnSync('bash', ['--norc', '--noprofile', '-c', line], { cwd: P, env, encoding: 'utf8' });
  const prev = process.env.CLAUDE_PROJECT_DIR;
  const dir = mkdtempSync(join(tmpdir(), 'guard-mut-pass3-day-real-'));
  try {
    process.env.CLAUDE_PROJECT_DIR = P;
    const io = { ...realIo(), env: {} };
    const settings = () => readFileSync(join(P, '.claude', 'settings.json'), 'utf8');
    const rules = () => readFileSync(join(P, 'CLAUDE.md'), 'utf8');
    const branchNow = () => g('branch', '--show-current').stdout.trim();
    const switched = () => branchNow() === 'cs/evil' && settings() === '{"a":2}\n';
    // [label, line, the form, the harm real bash does]
    const rows = [['H8', DL.H8, F_COMMENT, switched], ['A1', DL.A1, F_COMMENT, switched], ['D5', DL.D5, F_CONT, switched],
      ['AC', DL.AC, F_ANSI, switched], ['HQ', DL.HQ, F_HDOC, switched], ['R4', DL.R4, F_FD, () => rules() !== 'rules\n']];
    const controls = ['git status 2>&1', 'git status >&2', 'cat <<EOF\nbody\nEOF', "git log -1 --format='%h #%s'", 'echo a#b'];
    const src = readFileSync(join(HERE, 'routine-guard.mjs'), 'utf8');
    const DASK = 'if (!role && unreadable) {';
    assert.equal(src.split(DASK).length - 1, 1, 'the D-ASK line occurs exactly once');
    writeFileSync(join(dir, 'off.mjs'), src.replace(DASK, () => 'if (false) {'));
    const off = await import(pathToFileURL(join(dir, 'off.mjs')).href);
    const reset = () => { g('switch', '-q', '-f', 'main'); g('checkout', '-q', '--', '.'); };
    for (const [label, line, form, harm] of rows) {
      assert.match((await decide(dsh(line, { cwd: P }), io))?.ask ?? '', UNREAD(form), `real row ${label}: the guard asks`);
      assert.match((await decide(dsh(line, { cwd: P, agent_id: 'a1' }), io))?.deny ?? '', UNREAD_AGENT(form), `real row ${label}: an agent is refused`);
      assert.equal(kindOf(await off.decide(dsh(line, { cwd: P }), io)), null, `real row ${label}: without D-ASK the guard lets it through`);
      bash(line);
      assert.ok(harm(), `real bash ${label}: the hidden command or write ran`);
      reset();
      assert.deepEqual([branchNow(), settings(), rules()], ['main', '{"a":1}\n', 'rules\n'], `reset after ${label}`);
    }
    for (const line of controls) assert.equal(kindOf(await decide(dsh(line, { cwd: P }), io)), null, `control ${JSON.stringify(line)}`);
  } finally {
    if (prev === undefined) delete process.env.CLAUDE_PROJECT_DIR; else process.env.CLAUDE_PROJECT_DIR = prev;
    rmSync(root, { recursive: true, force: true });
    rmSync(dir, { recursive: true, force: true });
  }
});
