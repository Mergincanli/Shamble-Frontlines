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
- **Done:** main 626df67 merged in (202951f, clean; CI green); docs-keeper's pass (`docs-keeper.md`; this commit):
  STATUS (#43 and #45 in the merge list, the vfx row merged, kit-v3-4's row, A4's queued words removed with a pointer to
  the report's appendix, follow-ups), ROADMAP (#43 MERGED, kit-v3-4 entry, the R1-R6 entry into Build history),
  START_HERE §1 step 5 (wait for the APPROVE; a pure revert needs none), REPO_MAP rows; checks PASS (repo map 114
  entries, gate +0 −0). Left: V2_PHASE0_BRIEF.md:63 "Review once" (a dated brief: a follow-up in the report).
- **Done:** the review round on ad1f9bd: infra-engineer PASS WITH NOTES (IE-1 High: a revert dropping game symbols
  fails CI; six Medium, three Low), the verifier PASS WITH NOTES (M1, L1-L3), security-auditor (re-run after a container
  restart lost the first run) PASS WITH NOTES for the diff (SA-2 High, the rename-blind tier list; SA-3, SA-4 Medium;
  SA-5-SA-8 Low) and SA-1 Critical on main outside the diff (the guard's rename gap: GUARD-RENAME for Mustafa,
  recommended (c)). The advisor's fix-pass ruling (`advisor-fix-pass.md`): two kit Writes, IE-2-SCOPE ruled (clean
  STATUS/ROADMAP-only later merges don't count), F1-F4 follow-ups. The fix pass: docs part 9f0c6a2 (L1, F1-F4, table
  rows); the kit part (this commit): advisor.md (SA-2, IE-1-IE-7, SA-3, SA-4, SA-6, SA-7; +23 lines) and CHECKPOINTS.md
  §10 (IE-8-IE-10; +9; the spike paths marked as on its branch until it merges), each shown to Mustafa, one at a time.
- **Done (2026-11-09):** the verifier's check of the fix pass (`verifier-fixpass.md`, f1dfa2e): FAIL on one High,
  H1: advisor.md:285-286's "exact undo" check uses `git patch-id --stable`, which strips whitespace, so a fake revert
  that only changes whitespace matches; fix `--verbatim` on both sides. Lows L1 (name `AGENTS.md` at :76), L2 (rows 80
  and 82 copy the landed words), L3 (the trigger test's kit case, strict side); row 68 (IE-3) looser, to confirm.
  Everything else PASS (his words, the fix list, tests 1034/127/27, gate +0 −0, repo map, line numbers, long lines).
  The session reproduced H1 and checked binary ids and file order (`session-h1.md`, 26255a9).
- **Current step:** 7, the advisor's ruling on H1's fix (`advisor-h1.md`, running), then the one-word kit edit
  (shown to Mustafa, one `cp`), tests, gate, push; then the verifier's re-check (or a second round, as ruled), the
  advisor's review of the pushed head (closing IE-1, SA-2, H1), the report (C12 appendix; draft started), the
  notes' deletion, the final-head review, GUARD-RENAME and his go.
