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
     mark done (run 5 reread block 1's source). Fixed in CHECKPOINTS.md §4 in rounds B, C, D (65cd40e), each re-run;
     record 6e49303. The verifier's fix check: PASS WITH NOTES; N2-1 fixed as text E (49f9898), re-run in a workflow:
     3 of 3 restarts, the DONE and the fresh case PASS; N5-1 to N5-4 fixed in the record. Record:
     `.checkpoints/kit-v3-2/e9-dryrun.md` (25 runs). Gate PASS (symbols +0 -0, --no-smoke), repo map OK, 9 sections.
  6. The report drafted: `docs/reports/2026-11-04-kit-v3-2.md` (68 lines).
- **Current step:** 7, docs-keeper's STATUS and ROADMAP lines; then the last commit.
- **Agents:** Opus 5.5 at high, at most 3 at a time.

## Next steps
1. docs-keeper: STATUS and ROADMAP for the report, E9-1 and the PR's state (ready).
2. The last commit (not `[skip ci]`): the report, docs-keeper's edits, and `.checkpoints/kit-v3-2*` deleted; push;
   mark PR #37 ready with the report as its body.
3. The advisor's final-head review (a general agent reading main's advisor.md); its verdict on the PR; label
   `needs-mustafa`; Mustafa's go; merge with a merge commit and the head SHA as the expected head.
