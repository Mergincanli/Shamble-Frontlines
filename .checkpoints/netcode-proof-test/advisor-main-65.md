# Advisor: MAIN-65 plan check (2026-11-24, read only, effort xhigh)

## My plan
1. Rules read from origin/main:.claude/agents/advisor.md. Plan hash checked.
2. Read main-64-plan.md, advisor-main-64.md, advisor-main-64-head.md, main-61-plan.md, advisor-main-61.md,
   history.md:7821-7855.
3. Check every fact the plan states with read-only git: origin/main SHA, merge base, HEAD, name-status counts,
   shortstat, kit/CI pathspec, .gitignore lines and tracked __pycache__ files, docs/adr untouched, spikes/ grep,
   DEP-HOLD paths, merge-tree tree and conflict, the conflict block lines, STATUS.md 120 lines, N0 = ls-files count,
   checkpoint tests' 33, repo map numbers, CI runs on GitHub if reachable.
4. Judge: verdict, conditions (each with where it is met), fallbacks for every bound number, the U6c order question.
5. Write the verdict, end with DONE.

## Notes (one line per check)
- Plan sha256 fd6e8c4c...952eb2: matches.
- Read: main-64-plan.md, advisor-main-64.md, advisor-main-64-head.md, main-61-plan.md, advisor-main-61.md,
  history.md:7815-7900 (MAIN-64's record; LEAN-MODE word for word at :7857-7879), :2050-2090 (MAIN-52).
- SHAs: origin/main 28254a73ab22... = ls-remote main. Merge base 1ec6f6c3852d... HEAD 1d36067 = remote branch
  (ls-remote). HEAD moved 40a3481 -> 92fdf65 -> 1d36067: only .checkpoints/ (card, advisor-u6c-r2.md, the plan,
  u6c-r2-ruling-request.txt). Main: one merge on 1ec6f6c (parents 1ec6f6c, 1624f08), 22:40:38 UTC.
- name-status 1ec6f6c..origin/main: 64 paths, 23 A, 41 M (STATUS.md among the M), 0 D, 0 R; shortstat 64 files
  +4677/-317; without STATUS.md 63 files +4644/-284; STATUS.md numstat 33/33. Step 2's "63 others (A 23, M 40)" holds.
- Guard pathspec (main-61-plan.md:37-40) on origin/main...HEAD: empty. Kit/CI/tools pathspec on HEAD...origin/main:
  empty (main changes no .claude/, CLAUDE.md, AGENTS.md, .github/, tools/). Main's checkpoint.test.mjs: 33 test() calls.
- .gitignore: +packages/sim-worker/pkg-rules/ and +__pycache__/ (as stated). Tracked under pkg-rules/: none. Tracked
  under __pycache__/: the three .pyc files the plan names, all in .checkpoints/. spikes/ has no untracked file (only
  ignored node_modules), so the light run's untracked hash (main-run.sh:42, spikes/netcode-proof only) is not moved.
- check.mjs: refs counts `git ls-files` at ROOT (:2626), the whole index; redact scans the spike folder only (:2643-2647),
  so M0 and R0 do not move with main's files; self is check.test.mjs (spike only).
- HEAD moved again during this check: e8e276f (22:57:18, a save; it took this file's first text). Index 1,825 =
  tree 1,825 (1,822 at 40a3481 + main-65-plan.md + advisor-u6c-r2.md + this file). git status clean. No saver running
  (pgrep exit 1).
- merge-tree HEAD origin/main now: tree 9475d37 (5f57b5ae differs from it in .checkpoints/ only, six names), one
  conflict docs/STATUS.md (content); DECISIONS, REPO_MAP, STATUS, STATUS_ARCHIVE auto-merge lines as stated. 1,848
  files = 1,825 + 23.
- STATUS.md in 9475d37: 126 lines; markers :24, :28, :31; ours :25 (#40, 2026-11-15 text), :26 (#64), :27 (#63, #62);
  theirs :29 (#40 = base text), :30 (none yet, 3a). Outside the block it equals main's. Main's file: 120 lines.
  Branch changed only the #40 row since 1ec6f6c (1/1). Keep :25, :30 -> 120 lines, #40 row alone differs from main.
- `git diff --shortstat HEAD 9475d37 -- . ':(exclude)docs/STATUS.md'`: "63 files changed, 4644 insertions(+), 284
  deletions(-)", as the plan's preview. Names HEAD..9475d37: main's 64. 81aa7f1e exists (tree).
- C1 (c) on 9475d37: equal to main outside spikes/, .checkpoints/, docs/ (exit 0); docs/ differs in the six names
  (DECISIONS, REPO_MAP, STATUS, STATUS_ARCHIVE, design/netcode-proof-messages, design/ws-reconnect); spikes/ = HEAD's.
- Auto-merged docs: DECISIONS.md patch-id f0822ab both ways (branch's change = merged vs main), STATUS_ARCHIVE 348c3a6
  both ways; REPO_MAP differs with context, equal with -U0 (a75358f): same lines, moved context, as at MAIN-61/64.
- Repo map (rules of repo-map-check.mjs on main, worked out, not run): entry rows HEAD 173, main 177, merged 186
  (main +13: 9 crates rows, rules.test.mjs, tests/rules/, two reports; the two "built, not in git" rows do not match
  the pattern); no duplicates. Required: main's new files at mapped depths are two docs/reports/ files only, and
  nothing outside .checkpoints/ changed on the branch since 72b0dc3 (110 there) -> 110. Expected "repo map OK: 186
  entries cover 110 required paths".
- DEP-HOLD: Cargo.lock, package-lock.json, crates/devtools/Cargo.toml, packages/sim-worker/package.json, Cargo.toml,
  package.json: `git diff --stat origin/main 9475d37` empty. checkpoint.mjs:23-24 says it as cited; the code
  (:115-141) diffs against the remote branch and frees a file whose blob equals refs/remotes/origin/main's. Not held,
  as long as origin/main is still 28254a7 locally at the save.
- Marks: main changes nothing under docs/adr/, spikes/, docs/design/, .checkpoints/.
- refs: the plan's `git grep -I` prints nothing. check.mjs's referenceProblems (:1852-1867) also reads binaries
  (utf8, outside spikes/, docs/, .checkpoints/, *.md): `git grep -a -l "spikes/"` on main over that set prints only
  kept-logs.mjs and its test (the exception), the two new .bin vectors hold no such text. So refs passes.
