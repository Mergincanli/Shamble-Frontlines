# security-auditor: kit-v3-4 (A4, the review policy), review of ad1f9bd (2026-11-08)

Role read from origin/main (`git show origin/main:.claude/agents/security-auditor.md`, `CLAUDE.md`). Read-only in the
worktree. Base: origin/main 626df67 (merge-base). Head: ad1f9bd.

## Plan
1. Threat model: the assets are main's integrity (branch protection, the merge check: expected head, the advisor's
   APPROVE before his go), Mustafa's go on kit/CI/guard/decision docs, the night shift's merge ban, the "Never" list.
   Actors: a day session, the night shift routine, the PR Advisor routine, a subagent, a hostile PR text (data).
2. Read the full diff of the five kit files against main; list every changed rule sentence; compare with
   kit-draft.md §8 (E2) for completeness and honesty.
3. Tier 0 merge path: can a kit/CI/guard/decision-doc change be classed tier 0 and merged without his go?
4. Revert exception: abuse paths (non-revert change, kit revert without his go); the exact-undo check.
5. Budget and severity: can Critical/High end as a follow-up; can a security finding stay open and merge?
6. CHECKPOINTS.md §10 vs the guard per seat (run guard.mjs per seat where possible).
7. E3/E6: diff --stat for hooks/settings/CI/gate; the three kit test files on head and main, counts compared.

## Findings
(appended as confirmed)
