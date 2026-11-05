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
      // kit-v3-3 step 4b (S-2): origin/main resolves to a commit, as real git answers it
      if (/^rev-parse --verify --end-of-options (refs\/remotes\/)?origin\/main\^\{commit\}$/.test(a)) return { code: 0, out: `${SHA}\n` };
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
  ['no expansion in a redirect target', 'if (/[$`*?[{]/.test(p) || /^~/.test(p)) return deny(', 'if (false) return deny('],
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
  const { role = '', diffs = {}, platform, threeDot = '', shadow = false, calls, cdpath } = o;
  const names = new Map(Object.keys(diffs).map((n) => [shaOf(n), n]));
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
      if (args[0] === 'diff' && args.some((x) => x.startsWith('--output'))) return { code: 0, out: '' };
      if (args.length === 4 && args.slice(0, 3).join(' ') === 'rev-parse --verify --end-of-options' && args[3].endsWith('^{commit}')) {
        const r = args[3].slice(0, -'^{commit}'.length);
        if (r === 'refs/remotes/origin/main') return { code: 0, out: `${shaOf('origin/main')}\n` };
        if (r === 'origin/main') return { code: 0, out: `${shaOf(shadow ? 'a local origin/main' : 'origin/main')}\n` };
        return r in diffs || r === 'HEAD' ? { code: 0, out: `${shaOf(r)}\n` } : { code: 128, out: '' };
      }
      if (a === 'diff --name-only refs/remotes/origin/main...HEAD') return threeDot === null ? { code: 128, out: '' } : { code: 0, out: threeDot };
      if (args[0] === 'diff' && args[1] === '--name-only' && args[2] === 'refs/remotes/origin/main' && args[4] === '--' && args.length === 5) {
        const n = args[3] === 'HEAD' ? 'HEAD' : names.get(args[3]);
        return n !== undefined && n in diffs ? { code: 0, out: diffs[n] } : { code: 128, out: '' };
      }
      return { code: 0, out: '' };
    },
    gh: () => ({ code: 1, out: '' }),
    exists: (p) => (basename(p) === 'CLAUDE.md' ? CLAUDE_MD.has(p) : true),
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
  ['T9 git worktree add', dsh('git worktree add .gate/x 4def8d2'), null],
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
  ['S-4 cd -P to a worktree is followed', dsh('cd -P /w && git switch cs/other'), null, { diffs: OTHER }],
  ['S-4 a relative cd into a worktree is followed', dsh('cd .gate/x && git switch cs/other'), null, { diffs: OTHER }],
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
  ['a GIT_DIR= prefix asks', 'if (prefix.some((w) => /^GIT_(DIR|WORK_TREE)=/.test(w))) {', 'if (false) {'],
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
  ['S-4 pushd and popd are followed', "if (!role && (prog === 'cd' || prog === 'pushd' || prog === 'popd')) {", "if (!role && prog === 'cd') {"],
  ['S-4 a move in a forked line is lost', 'if (forked) return lost();', ''],
  ['S-4 the forks flag', "(c === '&' && s[i + 1] !== '>')) forks = true;", "(c === '&' && s[i + 1] !== '>')) forks = false;"],
  ['S-4 an untracked target is lost', 'if (a.length > 1 || /^[-+]/.test(t) || /[$`*?[{]/.test(t) || /^~[^/]/.test(t)) return lost();', ''],
  ['S-4 popd with an empty stack is lost', "if (prog === 'popd') return a.length || !dirs.stack.length ? lost() : dirs.stack.shift();",
    "if (prog === 'popd') return dirs.stack.shift() ?? vcwd;"],
  ['S-4 a relative move after a loss', 'if (!absolute && (dirs.lost || (io.env.CDPATH', 'if (!absolute && ((io.env.CDPATH'],
  ['S-4 CDPATH', '(io.env.CDPATH && !/^', '(false && !/^'],
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
