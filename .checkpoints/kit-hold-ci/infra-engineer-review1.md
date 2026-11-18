# infra-engineer, review round 1 of kit-hold-ci (PR #60), file .github/workflows/toolchain.yml only

Head reviewed: branch claude/wonderful-hamilton-jdmlnf; the workflow edit is commit 94c197f (12 added lines), base main 9b4b518.
Memory: .claude/agent-memory-local/infra-engineer/ does not exist (nothing to recall).

## Plan
1. Q1 byte check: extract blocks 1 to 3 of "The exact texts asked for" from `git show 1bfe462:docs/reports/2026-11-18-cs1-types.md`
   (and main's copy), compare byte for byte with the added lines of 94c197f (script, not by eye), and check placement.
2. Q2 cache-mode: read GitHub's docs page for `jobs.<job_id>.cache-mode` (docs.github.com), note date; check every step of the
   job for a cache restore/save (setup-node default package-manager-cache, rust toolchain, anything else); what the log line
   proves and does not; cost/side effects.
3. Q3 git inside the cut: read the whole job (user, env -i, HOME, nogroup), reason about safe.directory/ownership/index
   refresh, try locally on a scratch copy under .gate/ with a different uid if possible; set -euo pipefail behaviour of
   `changed=$(...)` command substitution failure.
4. Q4 compare lines: test on a scratch git repo which changes `git status --porcelain --untracked-files=all -- <path>` reports
   (content, mode, untracked, deleted, new subfolder, ignored file, core.fileMode=false, assume-unchanged), and what the
   generator writes (does it delete stale files?). Judge the plant.
5. Q5 tsc lines: word splitting / globbing of $strict $shipped $tests; empty list guard under set -e; `--ignoreConfig` in
   TypeScript 6.0 docs (typescriptlang.org); can both runs check nothing.
6. Q6 regressions: diff the job against main for the cut, permissions, order, --locked/--offline, timeout.
7. Q7 two lines each for the gate v2 list.
8. Read ci-evidence.md, plan.md "Evidence", resume card steps 12-17, Mustafa's words files.

## Findings (each block written as found)

### Q1 (done): byte for byte, placed where the report says
Method: scratch script (outside the repo, /tmp/.../scratchpad/q1/cmp.mjs) extracts the 4 fenced blocks of the section
"The exact texts asked for" from `git show 1bfe462:docs/reports/2026-11-18-cs1-types.md` (identical to main's copy: diff
empty), inserts blocks 3, 1, 2 into main's workflow (9b4b518) at their anchors ("    timeout-minutes: 15" once, line 31;
"cargo test --workspace --locked" once, line 84; "npx --no -- tsc --version" replaced, once, line 92) and compares bytes
with the branch file. Result: 5665 = 5665 bytes, identical. No CR, tab, non-ASCII or trailing space in any block. Part 2's
first line is absent from the branch file (correct: it waits for the gate v2). Workflow at HEAD 60335e2 == 94c197f (cmp);
only 94c197f touches the file in origin/main..HEAD.

### Evidence read (summary, as read 2026-11-18)
- ci-evidence.md: clean run 94c197f (toolchain push run 44357176860 job 134988309631, success; PR run 44357282401
  success). Log lines quoted: "Cache mode: none" (17:18:27.9099464Z, right after "Secret source: Actions"), setup-node
  with `package-manager-cache: true`, "Found in cache @ /opt/hostedtoolcache/node/22.23.4/x64" (that is the image's
  tool cache, not the Actions cache service), `uid=1001(runner) gid=65534(nogroup)`, network cut (curl exit 7), plain
  test run, write-mode run (0.01 s, all ok), no "changed" line, release tests, wasm build, npm ci offline, workspace
  tests, "Version 6.0.3", then 1.04 s silent (the two tsc runs), success. "No cache restored/saved line, searched by eye".
- Plant 0c0e271: Join.ts mode 100644 -> 100755, content same; push run 44357616602 failed with
  "writing the generated types again changed:" / " M packages/types/src/generated/protocol/Join.ts" / exit 1;
  PR run 44357633651 failed at step 7. Take-back 415bd73: four runs green.
- ci-evidence's own "not proven": compare step on a content change (unreachable: plain test fails first); tsc lines
  failing on a type error (shown locally only); what cache-mode stops beyond the log line.
- Mustafa's words: CI-TYPES "(a)... Its pull request shows two things from a real run: the compare step failing once on
  a planted change, and passing on the clean tree." CI-CUT "(a). Part 1 now... Its pull request shows "Cache mode: none"
  in the job's log." KIT-TEXT message 17:30: "The planted change for the compare step: yes, as you describe it (one
  generated file committed with the executable bit, taken back in the next commit)." So the plant's kind is his yes;
  my job is only what it proves and what the report must say.
- plan.md Evidence: write mode "deletes and rewrites the file" (to verify in the generator code), the stop rule.

### Sources read for Q2 (2026-11-18)
- S1 GitHub Docs, "Workflow syntax for GitHub Actions", sections `cache-mode` and `jobs.<job_id>.cache-mode`
  https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax (read 2026-11-18):
  modes read (restore yes, save no), write (both), write-only (save only), none (neither). "Access is enforced with
  scoped cache tokens, so a job cannot restore or save caches beyond the mode it is granted." Job value overrides the
  workflow value "for this job only". A not-permitted operation: the cache step logs an informational message and
  continues; job and workflow do not fail; skipped restore = cache miss; skipped save not performed. Omitted: read or
  write default by trigger. Page says nothing about runner type (hosted/self-hosted) or ACTIONS_CACHE_MODE.
- S2 GitHub Docs, "Dependency caching reference"
  https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching (read 2026-11-18):
  trusted triggers (push, workflow_dispatch, repository_dispatch, delete, registry_package, page_build, schedule) default
  write; low-trust (pull_request_target, issue_comment, workflow_run) default read; "The `pull_request` event is not
  affected". "`cache-mode` controls the cache access granted to the job's token, and it is enforced with scoped cache
  tokens." ACTIONS_CACHE_MODE "is only set in the environment of JavaScript and Docker container actions", "not available
  to `run` steps"; "the `actions/cache` action and the `@actions/cache` toolkit honor it". Skipped restore/save: "the
  action logs an informational message and the step and run continue without failing". Scope: a run restores caches of
  its current branch or the default branch; a PR run also the base branch; PR merge-ref caches are not restorable by the
  base branch or other PRs. Poisoning: "Cache contents are not signed or verified". The page does NOT say whether
  setup-* built-in caches follow cache-mode (by inference they do: setup-node uses @actions/cache, and the token is
  scoped server-side anyway).

### Q2 facts gathered (read-only gh, 2026-11-18)
- setup-node v7.0.0 README at the pinned SHA 8207627 (raw.githubusercontent.com, read 2026-11-18): "By default, caching
  for npm is enabled if either the devEngines.packageManager field or the top-level packageManager field in package.json
  specifies npm" (and no `cache` input); `package-manager-cache` "defaults to `true`"; "uses actions/cache under the hood".
  Root package.json has neither field (grep exit 1); the workspaces' package.json neither. So setup-node caches nothing
  in this job, before and after the edit. `cache-mode: none` guards the day a `packageManager` field lands (a manifest
  edit) and silently turns on a post-step save of ~/.npm, which code inside the cut can write (same uid, HOME kept).
- "Found in cache @ /opt/hostedtoolcache/node/22.23.4/x64" = the runner image's tool cache on local disk, not the Actions
  cache service; not affected by cache-mode, not shared between runs (fresh VM per job).
- Main run 44338178455 (9b4b518, push): "Cache mode: write"; PR run 44331706535 (1bfe462, pull_request): "Cache mode:
  write"; branch runs after the edit: "Cache mode: none" (push 44357176860 and PR 44357282401). No cache restore/save line
  in main's log either; setup-node's post printed only "Post job cleanup." (caching off). `gh cache list` refused by
  the proxy (HTTP 403), so the repository's cache list was not read.
- Time: run durations (created->updated, queue included) before the edit 31-48 s (10 runs), after 44-55 s (4 green
  runs). Branch run's extra seconds sit in checkout (7.0 s vs 1.2 s) and setup-node's "Environment details" (7.0 s vs
  0.3 s): version printing, cold VM, no cache call. The new lines' own work: write-mode run 17:19:05.34->05.51 (~0.17 s)
  plus the two tsc runs 17:19:13.60->14.64 (~1.04 s). Cost: public repository on standard hosted runners (no minutes
  billed; not checked on the billing page), Actions cache storage unchanged (nothing was ever saved by this job).
- S3 GitHub Changelog 2026-09-10 "Control GitHub Actions cache access with cache-mode"
  https://github.blog/changelog/2026-09-10-control-github-actions-cache-access-with-cache-mode/ (read 2026-11-18):
  "`none` prevents all cache access"; "The selected mode is enforced by the cache service"; "generally available on
  github.com for all GitHub plans"; job overrides workflow; nothing said about the log line.
- gate.yml's smoke job uses actions/cache@caa2961 (v5.1.0) for ~/.cache/ms-playwright (gate.yml:74-87), default mode
  (write on push and same-repo PR). Unchanged by this edit.

### Q2 conclusion
- What it does (S1, S2, S3): the job's token gets no cache scope; the cache service refuses restore and save; a cache
  step that tries logs an informational message and continues (the job does not fail).
- Does anything in this job still restore or save? No: setup-node's automatic npm cache needs a `packageManager` /
  `devEngines.packageManager` field naming npm (README at the pinned SHA), absent here; no actions/cache; rustup, cargo
  fetch, npm ci use local disk only. That was already true before the edit (main's log: no cache line). So the edit
  changes nothing observable today; it closes the channel for the future and against planted code: before, with mode
  write, code planted inside the cut (same uid 1001 owns /home/runner/work/_actions/**) could rewrite setup-node's post
  step JS, which runs after the cut with the runtime token and could save a cache under any key in this branch's scope
  (main's scope on a push to main), e.g. the Playwright key that gate.yml's smoke job restores and runs. After: refused
  by the cache service. The comment line's claim ("cannot reach another run through it") matches S1-S3.
- What "Cache mode: none" proves: the workflow with the key is valid and the runner received the job's effective mode
  `none` (printed in "Set up job" next to "Secret source"). Not proven: enforcement (no step tried a restore or save, so
  the cache service's refusal was never exercised; we rely on S1-S3); anything about gate.yml's jobs; and that the post
  steps cannot do other harm (part 2's subject).
- Cost/side effects: none measurable (see times above); gate.yml unaffected; no cache existed to lose. One future
  side effect: if a later edit adds `cache: npm` or a packageManager field to this job, the cache silently does nothing
  (informational message only, S1/S2), which is the intent but can puzzle a reader; the comment line explains it.
