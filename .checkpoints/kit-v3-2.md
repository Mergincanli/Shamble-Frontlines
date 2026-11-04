# Checkpoint: kit-v3-2 (A2 on Mustafa's list: CHECKPOINTS.md rules for every session; the TOY-BOX rewording)

- **Branch:** `claude/sleepy-tesla-ofsxsr`. It stands in for `cs/kit-v3-2`: this cloud session may push only there.
  Worktree: `/tmp/claude-0/-home-user-Shamble-Frontlines/263eddf3-7643-5640-b1d5-39be43d3567c/scratchpad/a2`, local
  branch `kit-v3-2`, from main 8bcc65e (PR #36 merged). Pushes are fast-forwards. Checkpoint commits are made by
  hand, marked `[skip ci]`. PR #37 (draft).
- **Mustafa's words** (2026-11-03, item A.2), word for word: `.checkpoints/kit-v3-2/mustafa-2026-11-03-list.md`.
  His answer (2026-11-04 ~20:25 UTC): "recommendations” in V3." = AGENT-CAP (A), SCRATCH-HOME (A), SAVER-CHECK (A),
  LOOK-LINES (B).
- **Steps with SHAs:**
  1. Plan rev 1; the advisor's APPROVE with 11 conditions (`advisor-plan-rev1.md`); four decisions for Mustafa: b03c41c.
  2. His answer: bc0e028. Plan rev 2 folds the conditions and his answers (`plan-rev2.md`).
  3. The build (C1-C12 with C9a, M1-M3, T1-T6, S1-S4, K1, N1, AD1, D1) and docs-keeper: 2c12306; notes: 8c2abd6.
  4. Review wave (art-director, security-auditor narrow, verifier; 3 at a time): 9 upheld findings fixed: acfc17e.
  5. E9 (main session, condition 10): runs on acfc17e found E9-1, a restarted agent re-checks the blocks its notes
     mark done (run 5 reread block 1's source). Fixed in CHECKPOINTS.md §4 in three rounds (texts B, C, D), each
     re-run; the final text D passed every case (3 of 3 restarts, the DONE and the fresh case). Record:
     `.checkpoints/kit-v3-2/e9-dryrun.md`. Gate PASS (symbols +0 -0, --no-smoke), repo map OK, 9 sections, no line
     over 120.
- **Current step:** 6, commit the E9 fix and its record; then a verifier check of only what changed.
- **Agents:** Opus 5.5 at high, at most 3 at a time.

## Next steps
1. Commit the fix (CHECKPOINTS.md) and this record; push. Verifier fix-check of the §4 diff from acfc17e and E9's
   evidence (transcript call lists and file stats in the scratchpad `e9tools/`, run folders `e9*/`).
2. The report (`docs/reports/2026-11-04-kit-v3-2.md`, `/advisor-report`), E9-1 and its three rounds under "Decisions
   and deviations"; docs-keeper's STATUS and ROADMAP lines for the report.
3. Delete `.checkpoints/kit-v3-2*` in the last commit (not `[skip ci]`); mark PR #37 ready with the report as its
   body; the advisor's final-head review (main's advisor.md); label `needs-mustafa` (a rules change); Mustafa's go;
   merge with a merge commit and the head SHA.
