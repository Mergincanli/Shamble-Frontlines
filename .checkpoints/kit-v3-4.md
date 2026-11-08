# Checkpoint: kit-v3-4 (A4 on Mustafa's list: the review policy), plus his answers of 2026-11-07

- **Branch:** `claude/sleepy-tesla-ofsxsr`, standing in for `cs/kit-v3-4`: this cloud session may push only there.
  - Worktree: `/tmp/claude-0/-home-user-Shamble-Frontlines/263eddf3-7643-5640-b1d5-39be43d3567c/scratchpad/a4`.
  - Local branch `kit-v3-4`, from main 69ea88e (PR #43 merged). Notes committed by hand with [skip ci].
- **His words:** A4 and A4's addition (2026-11-04) and A4-FREEZE (a) stand in STATUS's "Queued, in Mustafa's words";
  the freeze has lifted (his UNDER-WAY (a), recorded by the netcode session). His answers of 2026-11-07 (XP-TERM (a),
  LOOK (a)) are in `.checkpoints/kit-v3-4/mustafa-2026-11-07-answers.md`.
- **Carried in:** the advisor's four Low notes on #43's final head (comment 6701970962): the vfx report's STATUS:660
  is :681; the report should say why R6-TEL-OQ2, OQ-0005-AB3 and AB5 got no clause; STATUS:107 and ROADMAP:94 say
  "four register rows" (five); the report's LOOK entry should name OQ-0003-15.
- **#43:** merged 69ea88e, 2026-11-07 about 11:29 UTC, on Mustafa's go after the advisor's APPROVE WITH NOTES of
  4e03bb5 (comment 6701970962); CI green on that head. (`merged_at` read at build time.)
- **Done:** step 0, the sources saved (bbf2822); draft PR #44 opened and subscribed; the facts (`facts.md`, 23b9543,
  §0-§8, with 12 tensions between his A4 words and today's text); plan revision 1 (`plan-rev1.md`): parts A-F, seven
  decisions (TIER0-SCOPE for Mustafa; T0-MERGE, DOCS-RIDE, SCALE-HOME, REVERT-EXCEPTION for the advisor).
- **Done:** step 1, the advisor's review (`advisor-plan-rev1.md`): APPROVE WITH CONDITIONS (13), two escalations
  to Mustafa: TIER0-SCOPE (recommend (a): tier 0 = docs that record no decision of his) and REVERT-EXCEPTION
  (recommend (a): a pure revert needs no plan, day sessions only). Rulings: T0-MERGE (a) the PR Advisor's pinned
  merge; DOCS-RIDE (a) (split F and D into their own PR if the kit sitting can't happen the same day); SCALE-HOME (a).
  The docs parts (F, then D) may start now; the kit sitting writes no tier-0 line before TIER0-SCOPE and no revert
  exception before REVERT-EXCEPTION. Conditions C1-C13 in the verdict file (C13: E3 runs the three kit test files,
  `.claude/hooks/routine-guard.test.mjs`, `.claude/hooks/settings.test.mjs`, `.claude/scripts/checkpoint.test.mjs`).
- **Done:** step 2, the docs parts F and D (this commit; builder notes `builder-docs.md`, quote check
  `builder-quotes.py` 15/15 PASS): VISION_V2's XP-TERM pointer (:71-78 now); README R2-ROOM-OQ7 (C11), OQ-0006-14 and
  I-18 clauses; ADR 0006 notes (part 9 :431, part 10 :461, OQ-0006-14 :1026, "A bad release" :1564); SPIKE_PLAN
  :438-443; STATUS follow-ups answered (XP-TERM, LOOK), B.2's follow-up, line numbers recomputed (ADR 0005 §6 83–102,
  :2550 27–102; ADR 0006:482-483), "five register rows"; ROADMAP:94; the vfx report's corrections. Deviation: Low note
  3 says AB5 is about how an unlocked weapon reaches a match (not balance rates). Gate PASS +0 −0; repo map OK.
- **Done:** the kit text drafted as notes (`kit-draft.md`; resumed after a usage limit on 2026-11-08): four diffs
  against main that `git apply --check` cleanly (CLAUDE.md 5 hunks, +25 lines; advisor.md 5; SKILL.md 1;
  security-auditor.md 2); 9 markers ([PENDING TIER0-SCOPE] ×5, [PENDING REVERT-EXCEPTION] ×4) with the (b) variants
  in its §6; C1-C9 and C13 mapped in its §7; the rule-diff table (E2) in its §8, 55 rows, looser rows all from his A4
  words (revert rows 17, 19, 21, 45 wait on REVERT-EXCEPTION); unsure rows 22, 25, 37 for the reviewers. Main moved to
  626df67 (#45, ADR 0001); a test merge with this branch is clean.
- **Queued (2026-11-08):** his disk-hygiene item for `.claude/CHECKPOINTS.md`, word for word in
  `mustafa-2026-11-08-disk-hygiene.md`; to STATUS's queue in this pass's docs-keeper step; when it is done is
  DISK-WHEN (asked: ride A4's kit sitting as a fifth kit edit, or its own change-set after A4).
- **Done (2026-11-08, the kit sitting, Mustafa present):** his answers DISK-WHEN (a), TIER0-SCOPE (a),
  REVERT-EXCEPTION (a) saved (ebbdaa2); the drafted diffs applied with the markers removed (default (a) text kept;
  advisor.md cites `docs/reports/2026-11-08-kit-v3-4.md`'s appendix), one cp per kit file, one at a time: CLAUDE.md
  (252 → 277), advisor.md (172 → 301), changeset SKILL.md (32 → 36), security-auditor.md (54 → 56). E3: the kit tests
  unchanged and passing: routine-guard 1034/1034, settings 127/127, checkpoint 27/27 (main's counts). The advisor's
  addendum check of CHECKPOINTS.md §10 (`advisor-addendum-disk.md`): REVISE with two exact text changes (who runs it;
  "until its work is saved"), approved once applied; its optional third sentence (routines note copies) left out as a
  new duty.
- **Done:** the fifth kit edit, CHECKPOINTS.md §10 "Disk hygiene" (+12 lines, with the advisor's two text changes);
  checkpoint tests 27/27, repo map OK, gate PASS. The kit sitting is complete (five edits, one at a time).
- **Current step:** 4, docs-keeper's pass (STATUS and ROADMAP: #43 merged, #45 on main, this change-set in flight, the
  queue), then the review round (security-auditor, the verifier, infra-engineer), the report
  (parts A, B, C, E: CLAUDE.md, advisor.md, changeset SKILL.md, security-auditor.md; four asks). Then docs-keeper's
  pass (#43 merged, this change-set's rows), the review round, the report.
