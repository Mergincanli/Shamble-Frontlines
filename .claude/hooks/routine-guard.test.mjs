// Tests for the routine guard (kit-v3-1 plan, evidence C3): node --test .claude/hooks/routine-guard.test.mjs
// 1. The case table: role x tool x input -> refused or not, through decide() with a fake io. The git option lists come
//    from the container's real git (`git <sub> --git-completion-helper`), so the abbreviation cases match its git.
// 2. Real git on a throwaway repository (no network): abbreviated options really do what the full ones do, and the
//    guard refuses them.
// 3. The launcher (guard.mjs) in its error modes: broken rules, unreadable input; routines refuse, day sessions go on.
// 4. Mutation runs: each rule, switched off in a copy of the rules, makes at least one case of the table fail.
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
    changed = ['changed.mjs'], manifestSame = true, npmrc = [], symlinks = { [`${P}/link`]: '/etc' },
    headSha = SHA, fork = false, checks = [['gate', 'completed', 'success'], ['smoke', 'completed', 'success']],
    prFiles = 'docs/a.md\n',
  } = o;
  const pr = (ref, n) => JSON.stringify({ number: n, head: { sha: headSha, ref, repo: { full_name: fork ? 'someone/fork' : REPO } }, base: { repo: { full_name: REPO } } });
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
      const path = args.find((x) => x.startsWith('repos/'));
      return path in ghAnswers ? { code: 0, out: ghAnswers[path] } : { code: 1, out: '' };
    },
    exists: (p) => (basename(p) === '.npmrc' ? npmrc.includes(p) : true),
    realpath: (p) => {
      for (const [from, to] of Object.entries(symlinks)) if (p === from || p.startsWith(`${from}/`)) return to + p.slice(from.length);
      return p;
    },
    readFile: (p) => {
      if (p === `${P}/.claude/settings.json`) return JSON.stringify({ permissions: { allow: ['WebFetch(domain:doc.rust-lang.org)', 'WebFetch(domain:github.com)', 'WebFetch(domain:docs.rs)', 'WebFetch(domain:crates.io)'] } });
      throw new Error(`no file ${p}`);
    },
  };
}
const ev = (tool_name, tool_input, extra = {}) => ({ hook_event_name: 'PreToolUse', tool_name, tool_input, cwd: P, scratchpad_dir: SCRATCH, ...extra });
const sh = (command) => ev('Bash', { command });
const merge = (extra = {}) => ev('mcp__github__merge_pull_request', { owner: 'Mergincanli', repo: 'Shamble-Frontlines', pullNumber: 27, expectedHeadSha: SHA, ...extra });
const gw = (name, input) => ev(`mcp__github__${name}`, { owner: 'Mergincanli', repo: 'Shamble-Frontlines', ...input });

const DAY = '', NIGHT = 'night-shift', PRA = 'pr-advisor';
// [name, role, input, refused?, fake io options]
export const CASES = [
  // --- every session: git option forms (day sessions are bound by these too) ---
  ['git -C elsewhere', DAY, sh('git -C /tmp status'), true],
  ['git -c option', DAY, sh('git -c core.pager=x log'), true],
  ['git --version', DAY, sh('git --version'), false],
  ['push a cs branch', DAY, sh('git push -u origin cs/x'), false],
  ['push --del (abbreviated delete)', DAY, sh('git push origin cs/x --del'), true],
  ['push --delete', DAY, sh('git push origin --delete cs/x'), true],
  ['push -f', DAY, sh('git push -f origin cs/x'), true],
  ['push bundled -uf', DAY, sh('git push -uf origin cs/x'), true],
  ['push --force-with-lease', DAY, sh('git push --force-with-lease origin cs/x'), true],
  ['push +refspec', DAY, sh('git push origin +cs/x'), true],
  ['push :delete refspec', DAY, sh('git push origin :cs/x'), true],
  ['push main', DAY, sh('git push origin main'), true],
  ['push HEAD:main', DAY, sh('git push origin HEAD:main'), true],
  ['push HEAD', DAY, sh('git push origin HEAD'), true],
  ['bare push', DAY, sh('git push'), true],
  ['push remote only', DAY, sh('git push origin'), true],
  ['branch --show-current', DAY, sh('git branch --show-current'), false],
  ['branch -a', DAY, sh('git branch -a'), false],
  ['branch -D', DAY, sh('git branch -D cs/x'), true],
  ['branch --del (abbreviated)', DAY, sh('git branch --del cs/x'), true],
  ['branch -m (move)', DAY, sh('git branch -m a b'), true],
  ['commit -m', DAY, sh('git commit -m "a message"'), false],
  ['commit -m value that looks like flags', DAY, sh('git commit -m "-f fix"'), false],
  ['commit --amend', DAY, sh('git commit --amend --no-edit'), true],
  ['commit --am (abbreviated amend)', DAY, sh('git commit --am --no-edit'), true],
  ['reset --hard', DAY, sh('git reset --hard'), true],
  ['reset --ha (abbreviated)', DAY, sh('git reset --ha'), true],
  ['reset HEAD~1', DAY, sh('git reset HEAD~1'), true],
  ['reset unstage', DAY, sh('git reset -q HEAD -- .checkpoints'), false],
  ['fetch --upload (abbreviated upload-pack)', DAY, sh('git fetch --upload=touch .'), true],
  ['fetch origin main', DAY, sh('git fetch -q origin main'), false],
  ['ls-remote --upload', DAY, sh('git ls-remote --upload=touch .'), true],
  ['ls-remote -u', DAY, sh('git ls-remote -utouch .'), true],
  ['ls-remote heads', DAY, sh('git ls-remote --heads origin cs/x'), false],
  ['grep -O bundled', DAY, sh('git grep -nOtouch foo'), true],
  ['grep --open (abbreviated)', DAY, sh('git grep --open=touch foo'), true],
  ['grep -n', DAY, sh('git grep -n foo'), false],
  ['checkout', DAY, sh('git checkout main'), true],
  ['restore', DAY, sh('git restore x'), true],
  ['rebase', DAY, sh('git rebase main'), true],
  ['clean', DAY, sh('git clean -fd'), true],
  ['merge in a day session', DAY, sh('git merge origin/main'), false],
  ['worktree remove --force', DAY, sh('git worktree remove --force .gate/x'), true],
  ['worktree remove', DAY, sh('git worktree remove .gate/x'), false],
  ['stash drop', DAY, sh('git stash drop'), true],
  ['switch -c', DAY, sh('git switch -c cs/y origin/main'), false],
  ['switch -C', DAY, sh('git switch -C cs/y'), true],
  ['switch --force', DAY, sh('git switch --force cs/y'), true],
  ['config extraheader', DAY, sh('git config --get http.https://github.com/.extraheader'), true],
  ['config --list', DAY, sh('git config --list'), true],
  // --- every session: wrapped and hidden forms ---
  ['timeout wrapper', DAY, sh('timeout 30 git push -f origin cs/x'), true],
  ['env wrapper', DAY, sh('env GIT_TRACE=1 git push --force origin cs/x'), true],
  ['VAR= prefix', DAY, sh('GIT_TRACE=1 git push -f origin cs/x'), true],
  ['subshell', DAY, sh('(git push -f origin cs/x)'), true],
  ['brace group', DAY, sh('{ git push -f origin cs/x; }'), true],
  ['if/then', DAY, sh('if true; then git push -f origin cs/x; fi'), true],
  ['command substitution', DAY, sh('echo $(git push -f origin cs/x)'), true],
  ['substitution with parens inside', DAY, sh('echo $(echo (a); git push -f origin cs/x)'), true],
  ['after cd and &&', DAY, sh('cd sub && git reset --hard'), true],
  ['absolute git path', DAY, sh('/usr/bin/git push -f origin cs/x'), true],
  ['quoted program', DAY, sh("'git' push -f origin cs/x"), true],
  ['xargs', DAY, sh('echo x | xargs git branch -D'), true],
  ['heredoc body is not a command', DAY, sh("cat <<'EOF'\ngit push -f origin cs/x\nEOF"), false],
  // --- every session: gh ---
  ['gh api read', DAY, sh(`gh api repos/${REPO}/pulls/1`), false],
  ['gh api -X POST', DAY, sh(`gh api -X POST repos/${REPO}/issues`), true],
  ['gh api -XPOST', DAY, sh(`gh api -XPOST repos/${REPO}/issues`), true],
  ['gh api --method=PUT', DAY, sh(`gh api --method=PUT repos/${REPO}/pulls/1/merge`), true],
  ['gh api -f field', DAY, sh(`gh api repos/${REPO}/issues -f title=x`), true],
  ['gh api -fstuck', DAY, sh(`gh api repos/${REPO}/issues -ftitle=x`), true],
  ['gh pr merge', DAY, sh('gh pr merge 27'), true],
  ['gh auth', DAY, sh('gh auth status'), true],
  ['gh pr view', DAY, sh('gh pr view 27'), false],
  // --- every session: the merge check ---
  ['merge without expectedHeadSha', DAY, merge({ expectedHeadSha: undefined }), true],
  ['merge with a short SHA', DAY, merge({ expectedHeadSha: 'abc123' }), true],
  ['merge with a short SHA that is the head', DAY, merge({ expectedHeadSha: 'abc1234' }), true, { headSha: 'abc1234' }],
  ['merge, head moved', DAY, merge(), true, { headSha: 'b'.repeat(40) }],
  ['merge, fork head', DAY, merge(), true, { fork: true }],
  ['merge, no smoke', DAY, merge(), true, { checks: [['gate', 'completed', 'success']] }],
  ['merge, smoke running', DAY, merge(), true, { checks: [['gate', 'completed', 'success'], ['smoke', 'in_progress', null]] }],
  ['merge, gate failed', DAY, merge(), true, { checks: [['gate', 'completed', 'failure'], ['smoke', 'completed', 'success']] }],
  ['merge, green', DAY, merge(), false],
  ['merge, rules PR in a day session', DAY, merge(), false, { prFiles: 'CLAUDE.md\n' }],
  ['auto-merge', DAY, gw('enable_pr_auto_merge', { pullNumber: 27 }), true],
  // --- day sessions keep their freedom ---
  ['day: edit CLAUDE.md', DAY, ev('Edit', { file_path: `${P}/CLAUDE.md` }), false],
  ['day: WebFetch anywhere', DAY, ev('WebFetch', { url: 'https://example.com/x' }), false],
  ['day: AskUserQuestion', DAY, ev('AskUserQuestion', {}), false],
  ['day: curl', DAY, sh('curl https://example.com'), false],
  ['day: node -e', DAY, sh('node -e "1"'), false],
  ['day: PermissionRequest', DAY, { hook_event_name: 'PermissionRequest', tool_name: 'Bash', tool_input: { command: 'x' } }, false],
  ['day: ConfigChange', DAY, { hook_event_name: 'ConfigChange', source: 'project_settings' }, false],
  // --- night shift: files (C3c: the builder's worktree paths) ---
  ['night: edit docs', NIGHT, ev('Edit', { file_path: `${P}/docs/a.md` }), false],
  ['night: edit CLAUDE.md', NIGHT, ev('Edit', { file_path: `${P}/CLAUDE.md` }), true],
  ['night: edit a nested CLAUDE.md', NIGHT, ev('Write', { file_path: `${P}/docs/CLAUDE.md` }), true],
  ['night: edit settings', NIGHT, ev('Edit', { file_path: `${P}/.claude/settings.json` }), true],
  ['night: edit settings.local', NIGHT, ev('Write', { file_path: `${P}/.claude/settings.local.json` }), true],
  ['night: edit the guard', NIGHT, ev('Edit', { file_path: `${P}/.claude/hooks/guard.mjs` }), true],
  ['night: edit CI', NIGHT, ev('Edit', { file_path: `${P}/.github/workflows/gate.yml` }), true],
  ['night: edit a gate file', NIGHT, ev('Edit', { file_path: `${P}/tools/gate.mjs` }), true],
  ['night: edit the lockfile', NIGHT, ev('Edit', { file_path: `${P}/tools/package-lock.json` }), true],
  ['night: write .mcp.json', NIGHT, ev('Write', { file_path: `${P}/.mcp.json` }), true],
  ['night: write a .npmrc', NIGHT, ev('Write', { file_path: `${P}/tools/.npmrc` }), true],
  ['night: write a git hook', NIGHT, ev('Write', { file_path: `${P}/.git/hooks/pre-commit` }), true],
  ['night: worktree docs', NIGHT, ev('Edit', { file_path: `${P}/.claude/worktrees/b1/docs/a.md` }), false],
  ['night: worktree CLAUDE.md', NIGHT, ev('Edit', { file_path: `${P}/.claude/worktrees/b1/CLAUDE.md` }), true],
  ['night: worktree .claude', NIGHT, ev('Edit', { file_path: `${P}/.claude/worktrees/b1/.claude/agents/x.md` }), true],
  ['night: worktree .github', NIGHT, ev('Edit', { file_path: `${P}/.claude/worktrees/b1/.github/x.yml` }), true],
  ['night: .. escape', NIGHT, ev('Write', { file_path: `${P}/docs/../../etc/x` }), true],
  ['night: symlink escape', NIGHT, ev('Write', { file_path: `${P}/link/passwd` }), true],
  ['night: scratchpad', NIGHT, ev('Write', { file_path: `${SCRATCH}/notes.md` }), false],
  ['night: agent memory (local)', NIGHT, ev('Write', { file_path: `${P}/.claude/agent-memory-local/a/m.md` }), false],
  ['night: read secrets in /proc', NIGHT, ev('Read', { file_path: '/proc/self/environ' }), true],
  ['night: read ~/.gitconfig', NIGHT, ev('Read', { file_path: '/home/u/.gitconfig' }), true],
  ['night: read .git/config', NIGHT, ev('Read', { file_path: `${P}/.git/config` }), true],
  ['night: read docs', NIGHT, ev('Read', { file_path: `${P}/docs/a.md` }), false],
  // --- night shift: programs (R2) ---
  ['night: the gate (main copy)', NIGHT, sh('node tools/gate.mjs origin/main'), false],
  ['night: kit tests (main copy)', NIGHT, sh('node --test .claude/scripts/checkpoint.mjs'), false],
  ['night: preview.js', NIGHT, sh('node tools/preview.js . eval tools/sim.js'), true],
  ['night: xvfb-run preview', NIGHT, sh('xvfb-run -a node tools/preview.js . shot a.png 0 0'), true],
  ['night: node -e', NIGHT, sh('node -e "require(1)"'), true],
  ['night: node --require', NIGHT, sh('node --require ./x.js tools/gate.mjs'), true],
  ['night: a .gate harness', NIGHT, sh('node .gate/h.mjs'), true],
  ['night: changed script', NIGHT, sh('node changed.mjs'), true],
  ['night: untracked script', NIGHT, sh('node new.mjs'), true],
  ['night: python from stdin', NIGHT, sh("python3 - <<'EOF'\nprint(1)\nEOF"), true],
  ['night: python main script', NIGHT, sh('python3 tools/village.py'), false],
  ['night: python -c', NIGHT, sh('python3 -c "1"'), true],
  ['night: bash -c', NIGHT, sh('bash -c "git status"'), true],
  ['night: eval', NIGHT, sh('eval "git status"'), true],
  ['night: source', NIGHT, sh('source x.sh'), true],
  ['night: find -exec', NIGHT, sh('find . -name x -exec rm {} ;'), true],
  ['night: curl', NIGHT, sh('curl https://example.com'), true],
  ['night: sudo', NIGHT, sh('sudo ls'), true],
  ['night: npx', NIGHT, sh('npx --no -- playwright install chromium'), true],
  ['night: npm ci, exact', NIGHT, sh('npm ci --prefix tools --ignore-scripts --no-audit --no-fund'), false],
  ['night: npm ci, other form', NIGHT, sh('npm ci --prefix tools'), true],
  ['night: npm ci after cd', NIGHT, sh('cd tools && npm ci --prefix tools --ignore-scripts --no-audit --no-fund'), true],
  ['night: npm ci, .npmrc in home', NIGHT, sh('npm ci --prefix tools --ignore-scripts --no-audit --no-fund'), true, { npmrc: ['/home/u/.npmrc'] }],
  ['night: npm ci, manifest changed', NIGHT, sh('npm ci --prefix tools --ignore-scripts --no-audit --no-fund'), true, { manifestSame: false }],
  ['night: npm install', NIGHT, sh('npm install left-pad'), true],
  ['night: command substitution', NIGHT, sh('echo $(date)'), true],
  ['night: variable in git', NIGHT, sh('git push origin $BRANCH'), true],
  ['night: checkpoint save', NIGHT, sh('node .claude/scripts/checkpoint.mjs save s -m x'), false],
  ['night: checkpoint save, rules file in branch', NIGHT, sh('node .claude/scripts/checkpoint.mjs save s -m x'), true, { branchDiff: 'docs/a.md\n.claude/settings.json\n' }],
  // --- night shift: Bash writes ---
  ['night: redirect into CLAUDE.md', NIGHT, sh('echo x > CLAUDE.md'), true],
  ['night: append into settings', NIGHT, sh('echo x >> .claude/settings.json'), true],
  ['night: redirect into docs', NIGHT, sh('echo x > docs/a.md'), false],
  ['night: 2>&1 is no target', NIGHT, sh('git status 2>&1'), false],
  ['night: read redirect', NIGHT, sh('wc -l < .claude/settings.json'), false],
  ['night: tee into .claude', NIGHT, sh('echo x | tee .claude/x'), true],
  ['night: cp into .github', NIGHT, sh('cp a .github/x'), true],
  ['night: rm the guard', NIGHT, sh('rm .claude/hooks/guard.mjs'), true],
  ['night: sed -i a gate file', NIGHT, sh('sed -i s/a/b/ tools/gate.mjs'), true],
  ['night: mkdir notes', NIGHT, sh('mkdir -p .checkpoints/x'), false],
  // --- night shift: git ---
  ['night: merge', NIGHT, sh('git merge origin/main'), true],
  ['night: push own branch', NIGHT, sh('git push -u origin cs/x'), false],
  ['night: push another branch', NIGHT, sh('git push origin cs/other'), true],
  ['night: push, rules file in branch', NIGHT, sh('git push origin cs/x'), true, { branchDiff: 'CLAUDE.md\n' }],
  ['night: push to a fork remote', NIGHT, sh('git push fork cs/x'), true],
  ['night: commit docs', NIGHT, sh('git commit -m x'), false],
  ['night: commit a staged rules file', NIGHT, sh('git commit -m x'), true, { staged: 'docs/a.md\n.claude/agents/x.md\n' }],
  ['night: commit -a with an edited CI file', NIGHT, sh('git commit -am x'), true, { unstaged: '.github/workflows/gate.yml\n' }],
  ['night: fetch a PR ref', NIGHT, sh('git fetch origin refs/pull/1/head'), true],
  ['night: fetch from a URL', NIGHT, sh('git fetch https://github.com/a/b main'), true],
  ['night: git rm checkpoint', NIGHT, sh('git rm -r .checkpoints/x.md .checkpoints/x'), false],
  ['night: git rm a rules file', NIGHT, sh('git rm -r .checkpoints/x CLAUDE.md'), true],
  ['night: git apply', NIGHT, sh('git apply p.diff'), true],
  ['night: git config read', NIGHT, sh('git config user.email'), false],
  ['night: git config --get', NIGHT, sh('git config --get user.name'), false],
  ['night: git config write', NIGHT, sh('git config user.name x'), true],
  ['night: git config, a key not on the list', NIGHT, sh('git config core.editor'), true],
  // --- night shift: GitHub and other tools ---
  ['night: merge tool', NIGHT, merge(), true],
  ['night: comment on #16', NIGHT, gw('add_issue_comment', { issue_number: 16, body: 'x' }), false],
  ['night: comment on its own PR', NIGHT, gw('add_issue_comment', { issue_number: 27, body: 'x' }), false],
  ['night: comment on another PR', NIGHT, gw('add_issue_comment', { issue_number: 5, body: 'x' }), true],
  ['night: edit a #16 comment', NIGHT, gw('update_issue_comment', { comment_id: 111, body: 'x' }), false],
  ['night: update #16', NIGHT, gw('issue_write', { method: 'update', issue_number: 16, body: 'x' }), false],
  ['night: create an issue', NIGHT, gw('issue_write', { method: 'create', title: 'x' }), true],
  ['night: open a PR from its branch', NIGHT, gw('create_pull_request', { head: 'cs/x', base: 'main', title: 'x' }), false],
  ['night: open a PR from another branch', NIGHT, gw('create_pull_request', { head: 'cs/other', base: 'main', title: 'x' }), true],
  ['night: change a PR base', NIGHT, gw('update_pull_request', { pullNumber: 27, base: 'x' }), true],
  ['night: mark its PR ready', NIGHT, gw('update_pull_request', { pullNumber: 27, draft: false }), false],
  ['night: write to another repo', NIGHT, ev('mcp__github__add_issue_comment', { owner: 'x', repo: 'y', issue_number: 1, body: 'x' }), true],
  ['night: push files by API', NIGHT, gw('push_files', { branch: 'main' }), true],
  ['night: read a PR', NIGHT, gw('pull_request_read', { method: 'get', pullNumber: 27 }), false],
  ['night: create a routine', NIGHT, ev('mcp__claude-code-remote__create_trigger', {}), true],
  ['night: list routines', NIGHT, ev('mcp__claude-code-remote__list_triggers', {}), false],
  ['night: a docs connector', NIGHT, ev('mcp__Claude_Docs__create', {}), true],
  ['night: AskUserQuestion', NIGHT, ev('AskUserQuestion', {}), true],
  ['night: plan mode', NIGHT, ev('EnterPlanMode', {}), true],
  ['night: WebFetch a doc domain', NIGHT, ev('WebFetch', { url: 'https://doc.rust-lang.org/book/' }), false],
  ['night: WebFetch elsewhere', NIGHT, ev('WebFetch', { url: 'https://example.com/' }), true],
  ['night: WebFetch this repo', NIGHT, ev('WebFetch', { url: `https://github.com/${REPO}/blob/main/README.md` }), false],
  ['night: WebFetch another repo', NIGHT, ev('WebFetch', { url: 'https://github.com/someone/evil' }), true],
  ['night: docs.rs named crate', NIGHT, ev('WebFetch', { url: 'https://docs.rs/hecs/latest/hecs/' }), false],
  ['night: docs.rs other crate', NIGHT, ev('WebFetch', { url: 'https://docs.rs/evilcrate/' }), true],
  ['night: crates.io named crate', NIGHT, ev('WebFetch', { url: 'https://crates.io/crates/rapier3d' }), false],
  ['night: PermissionRequest', NIGHT, { hook_event_name: 'PermissionRequest', tool_name: 'Bash', tool_input: { command: 'x' } }, true],
  ['night: ConfigChange project', NIGHT, { hook_event_name: 'ConfigChange', source: 'project_settings' }, true],
  ['night: ConfigChange skills', NIGHT, { hook_event_name: 'ConfigChange', source: 'skills' }, true],
  ['night: ConfigChange policy (cannot be blocked)', NIGHT, { hook_event_name: 'ConfigChange', source: 'policy_settings' }, false],
  // --- PR Advisor ---
  ['advisor: git status', PRA, sh('git status'), false],
  ['advisor: fetch origin main', PRA, sh('git fetch -q origin main'), false],
  ['advisor: switch to main', PRA, sh('git switch --detach origin/main'), false],
  ['advisor: switch -c', PRA, sh('git switch -c x'), true],
  ['advisor: add', PRA, sh('git add x'), true],
  ['advisor: commit', PRA, sh('git commit -m x'), true],
  ['advisor: push', PRA, sh('git push origin cs/x'), true],
  ['advisor: worktree add', PRA, sh('git worktree add .gate/pr27 HEAD'), true],
  ['advisor: node', PRA, sh('node tools/gate.mjs'), true],
  ['advisor: npm ci', PRA, sh('npm ci --prefix tools --ignore-scripts --no-audit --no-fund'), true],
  ['advisor: gh pr view', PRA, sh('gh pr view 27'), false],
  ['advisor: gh pr comment', PRA, sh('gh pr comment 27 -b x'), true],
  ['advisor: edit docs', PRA, ev('Edit', { file_path: `${P}/docs/a.md` }), true],
  ['advisor: write notes', PRA, ev('Write', { file_path: `${P}/.checkpoints/n.md` }), false],
  ['advisor: redirect into docs', PRA, sh('echo x > docs/a.md'), true],
  ['advisor: mkdir in .gate', PRA, sh('mkdir -p .gate/x'), false],
  ['advisor: comment on a PR', PRA, gw('add_issue_comment', { issue_number: 27, body: 'x' }), false],
  ['advisor: comment on #16', PRA, gw('add_issue_comment', { issue_number: 16, body: 'x' }), true],
  ['advisor: edit its review comment', PRA, gw('update_issue_comment', { comment_id: 222, body: 'x' }), false],
  ['advisor: edit a #16 comment', PRA, gw('update_issue_comment', { comment_id: 111, body: 'x' }), true],
  ['advisor: label a PR', PRA, gw('issue_write', { method: 'update', issue_number: 27, labels: ['ready-to-merge'] }), false],
  ['advisor: rewrite a PR body', PRA, gw('issue_write', { method: 'update', issue_number: 27, body: 'x' }), true],
  ['advisor: open a PR', PRA, gw('create_pull_request', { head: 'cs/x', base: 'main', title: 'x' }), true],
  ['advisor: merge a green docs PR', PRA, merge(), false],
  ['advisor: merge a rules PR', PRA, merge(), true, { prFiles: 'docs/a.md\n.claude/settings.json\n' }],
  ['advisor: merge a .mcp.json PR', PRA, merge(), true, { prFiles: '.mcp.json\n' }],
  ['advisor: merge a CI PR', PRA, merge(), true, { prFiles: '.github/workflows/gate.yml\n' }],
  // --- deny-by-default: a routine runs only the listed programs (verdict-3; security review F1/F3) ---
  ['night: a read tool', NIGHT, sh('cat docs/a.md'), false],
  ['night: ls a path', NIGHT, sh('ls docs'), false],
  ['night: ls /opt/pw-browsers', NIGHT, sh('ls /opt/pw-browsers'), false],
  ['night: grep a doc', NIGHT, sh('grep -n foo docs/a.md'), false],
  ['night: head a doc', NIGHT, sh('head -5 docs/a.md'), false],
  ['night: an unlisted program (perl)', NIGHT, sh('perl x.pl'), true],
  ['night: make', NIGHT, sh('make -f Makefile'), true],
  ['night: an arbitrary tool', NIGHT, sh('zip -r a.zip b'), true],
  ['night: a program by a relative path', NIGHT, sh('./.gate/x.sh'), true],
  ['night: a program by an absolute path', NIGHT, sh('/usr/bin/node tools/gate.mjs'), true],
  ['night: tar', NIGHT, sh('tar -czf a.tgz docs'), true],
  ['night: ln', NIGHT, sh('ln -s a b'), true],
  ['night: chmod', NIGHT, sh('chmod +x x'), true],
  ['night: dd', NIGHT, sh('dd if=a of=b'), true],
  // --- deny-by-default: credential reads (F5) ---
  ['night: cat a credential file', NIGHT, sh('cat ~/.git-credentials'), true],
  ['night: grep /proc', NIGHT, sh('grep x /proc/1/environ'), true],
  ['night: a variable in a read path', NIGHT, sh('cat $HOME/.netrc'), true],
  ['night: read-redirect a credential', NIGHT, sh('cat < ~/.git-credentials'), true],
  ['night: read-redirect a doc', NIGHT, sh('wc -l < docs/a.md'), false],
  ['night: cat .git/config', NIGHT, sh('cat .git/config'), true],
  // --- deny-by-default: find (the one read tool kept that has write/exec options; verdict-3 condition 1) ---
  ['night: find -delete', NIGHT, sh('find . -name x -delete'), true],
  ['night: find -fprintf', NIGHT, sh('find . -fprintf out.txt %p'), true],
  ['night: find -exec', NIGHT, sh('find . -name x -exec rm {} ;'), true],
  ['night: date -s', NIGHT, sh('date -s "2020-01-01"'), true],
  ['night: find read-only', NIGHT, sh('find . -name "*.md"'), false],
  // --- deny-by-default: tools with rich option surfaces are off the list (security review, second pass) ---
  ['night: sort is off the list', NIGHT, sh('sort -o CLAUDE.md x'), true],
  ['night: xxd is off the list', NIGHT, sh('xxd -r a b'), true],
  ['night: tree is off the list', NIGHT, sh('tree -o out.txt'), true],
  ['night: fd is off the list', NIGHT, sh('fd -x rm'), true],
  ['night: awk is off the list', NIGHT, sh('awk "{print}" a'), true],
  ['night: rg is off the list', NIGHT, sh('rg foo docs'), true],
  // --- deny-by-default: reads are confined to the project, scratchpad and browser dir (G2) ---
  ['night: read outside the project', NIGHT, sh('cat /etc/passwd'), true],
  ['night: grep -r outside the project', NIGHT, sh('grep -r x /etc'), true],
  ['night: find the filesystem root', NIGHT, sh('find / -name x'), true],
  ['night: a .. escape in a read', NIGHT, sh('cat ../../etc/shadow'), true],
  ['night: an option-attached credential path', NIGHT, sh('grep -f/proc/1/environ x'), true],
  ['night: grep -f a credential file', NIGHT, sh('grep -f ~/.netrc x'), true],
  ['night: read a project file', NIGHT, sh('cat docs/a.md'), false],
  ['night: a grep pattern with regex metachars', NIGHT, sh('grep "foo$" docs/a.md'), false],
  // --- deny-by-default: recursive cp/mv/rm and directory clobber (G3) ---
  ['night: rm -rf the project root', NIGHT, sh('rm -rf .'), true],
  ['night: rm -r a directory', NIGHT, sh('rm -r docs'), true],
  ['night: cp -r', NIGHT, sh('cp -r a b'), true],
  ['night: rm the project root (no -r)', NIGHT, sh('rm .'), true],
  ['night: mv the .claude directory', NIGHT, sh('mv .claude elsewhere'), true],
  ['night: tee the project root', NIGHT, sh('tee .'), true],
  // --- deny-by-default: environment on a command (verdict-3 condition 4) ---
  ['night: a bad env prefix (PATH)', NIGHT, sh('PATH=.gate node tools/gate.mjs'), true],
  ['night: a bad env prefix (NODE_OPTIONS)', NIGHT, sh('NODE_OPTIONS=--require=./x node tools/gate.mjs'), true],
  ['night: env with a bad var', NIGHT, sh('env NODE_OPTIONS=--require=./x node tools/gate.mjs'), true],
  ['night: env -S', NIGHT, sh('env -S "node -e 1"'), true],
  ['night: bare env prints the environment', NIGHT, sh('env'), true],
  ['night: printenv', NIGHT, sh('printenv'), true],
  ['night: TZ prefix is allowed', NIGHT, sh('TZ=Europe/Amsterdam date'), false],
  ['night: env TZ is allowed', NIGHT, sh('env TZ=UTC date'), false],
  // --- deny-by-default: re-exec wrappers and shells (verdict-3 condition 1) ---
  ['night: command wrapper', NIGHT, sh('command git push -f origin cs/x'), true],
  ['night: exec wrapper', NIGHT, sh('exec node x'), true],
  ['night: time wrapper', NIGHT, sh('time node tools/gate.mjs'), true],
  ['night: xargs', NIGHT, sh('echo x | xargs rm'), true],
  // --- deny-by-default: file tools, operand checks (verdict-3 condition 3) ---
  ['night: cp reads a protected file into notes', NIGHT, sh('cp CLAUDE.md .gate/x'), false],
  ['night: cp writes a protected file', NIGHT, sh('cp a CLAUDE.md'), true],
  ['night: cp -t a protected dir', NIGHT, sh('cp -t .github a b'), true],
  ['night: mv removes a protected file', NIGHT, sh('mv CLAUDE.md x'), true],
  ['night: mv a doc', NIGHT, sh('mv docs/a.md docs/b.md'), false],
  ['night: rm a doc', NIGHT, sh('rm docs/a.md'), false],
  ['night: touch -r a reference', NIGHT, sh('touch -r docs/a.md docs/b.md'), false],
  ['night: a redirect outside the project', NIGHT, sh('echo x > /etc/foo'), true],
  ['night: a redirect to the scratchpad', NIGHT, sh(`echo x > ${SCRATCH}/n`), false],
  ['night: a variable in a redirect', NIGHT, sh('echo x > $HOME/y'), true],
  // --- deny-by-default: gh (F6) ---
  ['night: gh browse', NIGHT, sh('gh browse'), true],
  ['night: gh gist', NIGHT, sh('gh gist create x'), true],
  ['advisor: gh search', PRA, sh('gh search prs x'), true],
  ['advisor: gh pr view', PRA, sh('gh pr view 27'), false],
  // --- heredocs (F2) ---
  ['night: an unterminated here-document', NIGHT, sh("cat <<'EOF'\ngit push -f origin cs/x"), true],
  ['night: a dash-heredoc closed by a tab-indented terminator', NIGHT, sh('cat <<-EOF\n\tbody\n\tEOF\nls'), false],
  ['night: a plain heredoc is not closed by a tab-indented terminator', NIGHT, sh('cat <<EOF\nbody\n\tEOF'), true],
  // --- the PR Advisor's read tools (verdict-3 condition 3: not refused as writes) ---
  ['advisor: head a doc', PRA, sh('head -5 docs/a.md'), false],
  ['advisor: cp into its notes', PRA, sh('cp CLAUDE.md .gate/x'), false],
  ['advisor: sed is off the list', PRA, sh('sed -n 1,5p docs/a.md'), true],
  ['advisor: cp outside its notes', PRA, sh('cp a docs/x'), true],
  // --- an unknown role is refused outright ---
  ['unknown role', 'nightshift', sh('git status'), true],
];

for (const [name, role, input, refused, opts] of CASES) {
  test(`${refused ? 'refused' : 'goes on'}: ${name}`, async () => {
    const d = await decide(input, fakeIo({ role, ...opts }));
    assert.equal(!!d, refused, `${name}: ${JSON.stringify(d)}`);
  });
}

test('a refusal names what to use instead (verdict-3 condition 6)', async () => {
  const msg = async (command) => (await decide(sh(command), fakeIo({ role: NIGHT })))?.deny ?? '';
  assert.match(await msg('curl https://example.com'), /WebFetch/);
  assert.match(await msg('kill -TERM 123'), /TaskStop/);
  assert.match(await msg('pkill -f checkpoint.mjs'), /TaskStop/);
  assert.match(await msg('sed -i s/a/b/ docs/a.md'), /Edit/);
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

// --- 3. the launcher's error modes (R3c, C3d) ---
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

// --- 4. mutation runs (C3e): switch one rule off in a copy; some case of the table must notice ---
const MUTANTS = [
  ['abbreviated long options', 'if (!exact && known.some((k) => k.startsWith(name))) return', 'if (false) return'],
  ['dangerous long options', 'if (danger.includes(name)) return', 'if (false) return'],
  ['dangerous short flags', 'if (shortDanger.includes(t[k])) return', 'if (false) return'],
  ['options before the git subcommand', "if (argv[1]?.startsWith('-')) return deny(", 'if (false) return deny('],
  ['git subcommands never allowed', 'if (GIT_NEVER[sub]) return deny(', 'if (false) return deny('],
  ['force, delete and main refspecs', "if (r.startsWith('+') || r.startsWith(':') || /(^|:)(refs\\/heads\\/)?(main|master|HEAD)$/.test(r) || r === 'HEAD') return deny(", 'if (false) return deny('],
  ['reset only unstages', "if (!(before.length === 0 || (before.length === 1 && before[0] === 'HEAD'))) {", 'if (false) {'],
  ['the merge SHA', "if (!/^[0-9a-f]{40}$/.test(sha)) return deny(", 'if (false) return deny('],
  ['the merge checks', "if (rs.some((r) => !['success', 'skipped', 'neutral'].includes(r.conclusion)) || !rs.some((r) => r.conclusion === 'success')) return deny(", 'if (false) return deny('],
  ['protected writes (Edit/Write tool)', 'if (protectedRel(rel)) return deny(`routine (${role}): ${rel} is a rules, CI or gate file; those change', 'if (false) return deny(`routine (${role}): ${rel} is a rules, CI or gate file; those change'],
  ['protected writes (Bash file tool)', 'if (protectedRel(rel)) return deny(`routine (${role}): ${what} on ${rel} (a rules, CI or gate file) is not allowed', 'if (false) return deny(`routine (${role}): ${what} on ${rel} (a rules, CI or gate file) is not allowed'],
  ['the program allow list (deny-by-default)', "return deny(`routine (${role}): ${prog} is not one of the routines' programs", "return null && deny(`routine (${role}): ${prog} is not one of the routines' programs"],
  ['a program by a path', 'if (rawProg !== prog) return deny(`routine (${role}): run ${prog} by name', 'if (false) return deny(`routine (${role}): run ${prog} by name'],
  ['environment set on a command', 'if (!ROLE_ENV_OK.has(name)) return { deny: `routine (${role}): only ${okList} may be set', 'if (false) return { deny: `routine (${role}): only ${okList} may be set'],
  ['re-exec wrappers', "if (b === 'command' || b === 'builtin' || b === 'exec' || b === 'time' || b === 'nohup') return { deny:", 'if (false) return { deny:'],
  ['credential read operands', 'if (cred) return deny(`routine (${role}): ${p} may hold credentials', 'if (false) return deny(`routine (${role}): ${p} may hold credentials'],
  ['read-redirect targets are checked', 'for (const r of c.reads) {', 'for (const r of []) {'],
  ['a write outside the project', 'return deny(`routine (${role}): ${what} must write inside the project or the scratchpad', 'return null && deny(`routine (${role}): ${what} must write inside the project or the scratchpad'],
  ['read-tool exec options (find -delete)', "if (prog === 'find' && has(/^-(exec|execdir|ok|okdir|delete|fprint|fprintf|fprint0|fls)$|^--(exec|delete)(=|$)/)) return '-exec/-delete/-fprint run or write';", 'if (false) return "x";'],
  ['reads confined to the project (G2)', 'if (!inProject && !inScratch && !inBrowsers) return deny(', 'if (false) return deny('],
  ['a write that covers a protected directory (G3)', 'if (coversProtected(rel)) return deny(', 'if (false) return deny('],
  ['recursive cp/mv/rm (G3)', "return deny(`routine (${role}): recursive ${prog} isn't allowed", "return null && deny(`routine (${role}): recursive ${prog} isn't allowed"],
  ['an unterminated here-document', 'if (role && unterminated) return deny(', 'if (false) return deny('],
  ['gh deny-by-default', "if ((sub === 'pr' || sub === 'issue') && /^(view|list|status|diff|checks)$/.test(sub2)) return null;\n    return deny(`routine (${role}): gh", "if (true) return null;\n    return deny(`routine (${role}): gh"],
  ["the PR Advisor's notes-only writes", "if (role === 'pr-advisor' && !/^(\\.checkpoints|\\.gate|\\.claude\\/agent-memory-local)(\\/|$)/.test(rel)) {", 'if (false) {'],
  ['tracked scripts only', "if (io.git(['ls-files', '--error-unmatch', '--', rel], root).code !== 0) return deny(", 'if (false) return deny('],
  ["main's copy only", "if (io.git(['diff', '--quiet', 'origin/main', '--', rel], root).code !== 0) return deny(", 'if (false) return deny('],
  ['the exact npm form', "if (role === 'night-shift' && argv.join(' ') === NIGHT_NPM) {", "if (role === 'night-shift') {"],
  // NET, SHELLS, sudo, kill, sed/awk and INSTALLERS are message-only now: the allow-list catch-all already denies them
  // (verdict-3 condition 6 keeps them for a message naming what to use instead). Their messages, not their behaviour,
  // are the point, so they carry no behavioural mutant; the catch-all mutant above covers the deny.
  ['PermissionRequest', "return role ? deny(`routine (${rawRole}): this would have waited", "return false ? deny(`routine (${rawRole}): this would have waited"],
  ['ConfigChange', "return role && ['project_settings', 'local_settings', 'skills', 'user_settings'].includes(input.source)", 'return false'],
  ['the web allow list', "if (!ok) return deny(", 'if (false) return deny('],
  ['no substitutions in routines', 'if (role && subs) return deny(', 'if (false) return deny('],
  ['shell keywords', "'!', '{', '}', 'if', 'then', 'else',", "'!', 'else',"],
  ['parentheses separate commands', "if (c === ';' || c === '(' || c === ')') { endCmd(); i++; continue; }", "if (c === ';') { endCmd(); i++; continue; }"],
  ['no merge at night', "if (role === 'night-shift') return deny('routine (night-shift): the night shift never merges');", ''],
  ['protected files in a merge (PR Advisor)', 'if (hit) return deny(`routine (pr-advisor): the PR changes', 'if (false) return deny(`routine (pr-advisor): the PR changes'],
  ['own-branch pushes', 'if (refs.length !== 1 || refs[0] !== cur) return deny(', 'if (false) return deny('],
  ['git config reads only (routines)', 'if (!GIT_CONFIG_READ.has(key.toLowerCase())) return deny(', 'if (false) return deny('],
  ['no interpreter for the PR Advisor', "if (role === 'pr-advisor') return deny(\"routine (pr-advisor): the PR Advisor runs no program", "if (false) return deny(\"routine (pr-advisor): the PR Advisor runs no program"],
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
