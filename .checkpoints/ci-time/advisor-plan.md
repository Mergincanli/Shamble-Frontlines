# Advisor plan review: CI-TIME (plan at afdb078), 2026-11-27
Worked from main b9aad01's .claude/agents/advisor.md (read in full with `git show origin/main:...`; the worktree copy
has no diff to it). Process note: my first three Bash calls chained `cd ... && git ...` (git not starting its own
command), against the interim rule; later git calls ran on their own.

## Checked myself
- toolchain.yml:38 is `    timeout-minutes: 15` (job toolchain). The file's only other limits: Playwright
  `--global-timeout=300000` at 205 (Chrome) and 212 (Firefox). gate.yml's 10 (:22) and 15 (:54) are a different file.
- sha256 toolchain.yml d017b83b... and draft-toolchain.diff cf98135b...: match the plan; the diff is that one line.
  The "after" sha256 d1dd16d7... reproduced (sed on line 38 piped to sha256sum, no file written).
- "Nothing else names the 15-minute limit": grep of docs/, .github/, .claude/, README.md, tools/README.md. Only
  historical text (STATUS_ARCHIVE:1979, reports 11-18, 11-19, 11-26) and r5 research about gate.yml; none is a live
  description to change. REPO_MAP's toolchain row (264) names the 5-minute stops only. Holds.
- His block (1280 bytes, by script) matches the task summary. The second saved message
  (mustafa-2026-11-26-74-merged.md, 148 bytes, by script) is his word ("#74 is merged ... Set your board thread to
  done"): fits DECISIONS as a record, decides nothing new.
- Push route: e8340eb is an ancestor of origin/main; remote claude/keen-tesla-v807cf is at afdb078 (fast-forward done).
- Repository public (REST visibility: public): the minutes claim holds, no tier-3 money effect.
- Guard (routine-guard.mjs, every session): `gh pr merge` refused (:1452); `gh api` with any write or non-GET refused
  (:1466) -> the plan's REST routes for opening, ready-for-review and merging are refused; GitHub MCP tools only;
  merge_pull_request needs expectedHeadSha = full 40-char SHA, checks head, fork, whole check list, gate and smoke
  (:1557ff); enable_pr_auto_merge refused (:1505).
- tools/node_modules absent in .gate/ci5.

## Verdict: APPROVE with conditions
Blocking:
1. Reviewers: add infra-engineer (CLAUDE.md step 5 names it for CI; tier 2 is advisor + security-auditor "on top of
   step 5's subject list"). One short read in the same round as security-auditor.
2. Order: the workflow commit first; security-auditor and infra-engineer read it; the report records their findings
   (each addressed or deferred in writing); then docs and the notes deletion; then my head review of the final head,
   whose `.github/` diff must be byte-identical to what they read. STATUS in final wording ("merges on his go"),
   never a push after his go; his go and the merge go in the next PR.
3. No [skip ci] on the final head: gate, smoke and toolchain completed success on that exact SHA.
4. Final file list only: toolchain.yml (one line), DECISIONS (his two messages whole, by script), STATUS, ROADMAP
   (one line, or the report says why none), REPO_MAP (toolchain row + report row), the report. Checked with
   `git diff --raw --no-renames origin/main...<head>` (100644 modes, no .checkpoints/).
5. GitHub route: the GitHub MCP tools (create_pull_request with draft; mark ready; merge_pull_request with
   expectedHeadSha). `gh api` writes and `gh pr merge` are refused by the guard. If a step has no tool, tell him; no
   workaround, no auto-merge.
Notes (not blocking):
6. Run node tools/gate.mjs on the final head in a .gate worktree if the toolkit installs; I run it at head review.
   The kit tests: kit files are identical to main, so counts are unchanged; say so in the report.
7. Evidence wording: one-line diff and hashes, YAML parse, toolchain green. The "a run over 15 minutes now passes"
   benefit is not verifiable until such a run; say so. Citing the 13-minute run id is Low (the text is his decision).
8. Low: checkpoint commits by hand instead of checkpoint.mjs (it pushes to the local name); say so in the report.

## Rulings asked
- Scope: one line, right.
- Docs: right-sized; add nothing more.
- Push route: right (own claude/ branch, fast-forward, no force).
- Open and merge: allowed (his points 4-5 + standing permission + CLAUDE.md merge rule; tier 2 CI from a day session),
  under condition 5 and the merge rule.
