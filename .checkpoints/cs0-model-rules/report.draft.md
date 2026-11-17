# Advisor report: cs0-model-rules (2026-11-17, branch claude/wonderful-hamilton-jdmlnf, base main c94c81b)

The branch stands in for `cs/cs0-model-rules` (a cloud session limited to its `claude/` branch). Pull request #56.
Change-set 0 of the build plan (`docs/reports/2026-11-17-build-plan-slice-1.md`). Docs only, no game code.

## Goal
Record Mustafa's decisions of 14 and 17 November, carry the build plan to main, and fix on paper the rules a model
file must follow, so that game code can start and the Blender work has its rules. Nothing a player sees.

## Outcome
DONE_OR_NOT

## Plan and approval
- The plan (`.checkpoints/cs0-model-rules/plan.md`, deleted with this change-set, last held by commit LASTNOTES). The
  advisor agent: REVISE at about 19:29 UTC (1 High: the joint list could not carry the animation port; 10 Medium, 5
  Low), then APPROVE with conditions at about 20:45 UTC on revision 2.
- The High was closed by engine-architect's ruling (20 joints); the advisor agent ruled that `root` is the node above
  the skeleton. Its conditions 18 to 22, each met before the first commit: two more pointers in DECISIONS; every
  pointer carries his COMPARE exception and his sentence on separate tests; the draw check counts draws, as
  PM-0005-36 does; dated answers under OQ-0002-6 and OQ-0006-18 in their drafts; the size stop told to Mustafa.
- Mustafa, 20:39 UTC (SIZE-0): one pull request; stop line 1,200 written lines; his records and the plan's carry in
  commits of their own. Written lines at the end: WRITTEN (`size-count.mjs`; 1,788 copied lines not counted).

## What changed
- `docs/DECISIONS.md`: five entries by script (his task and decision of 2026-11-14, his addition, his go of
  2026-11-17 with twelve answers and four changes, his messages of 20:58 and 21:33); six dated pointers.
- `docs/reports/2026-11-17-build-plan-slice-1-appendix.md`: A1 to A11, his messages and the Art lead's four texts.
- `docs/reports/2026-11-17-build-plan-slice-1.md`: "Since the plan" and the plan, copied from commit 68addf9.
- `docs/research/asset-export-rules.md` (graphics-engineer) and `docs/research/soldier-joint-list.md`
  (engine-architect): sixteen rules, ten checks, 20 joints; on paper, no export has gone through them.
- `docs/adr/0005-...`: OQ-0005-AS3 and AS4 answered in part; ten new marks PM-0005-47 to -56 (test-engineer).
- `docs/adr/0002`, `0004`, `0006`, `docs/adr/README.md`, `docs/SPIKE_PLAN.md`: his answers and dated pointers where
  the drafts ask (the jump's reach, a committed model may be public, soldiers push softly, the machine classes,
  PM-0004-34); the register's open questions 98 to 96.
- `docs/LOOK_V2.md` section 8: "Spline or Blender" (his words of 2026-11-03). STATUS, ROADMAP, the repository map.

## Evidence
- His words: `decisions-insert.mjs --check`: five entries equal byte for byte; `appendix-verify.mjs HEAD`: 11 blocks,
  0 mismatches; every quote in the pointers and in "Since the plan" is in his saved messages (`quotes-check.mjs`).
- The verifier: VERIFIER_LINE
- Gate: `node tools/gate.mjs --no-smoke` PASS (checks 1 to 4, 5b to 5d). `repo-map-check.mjs`: REPOMAP_LINE
- CI on HEADSHA: CI_LINE

## Reviews
REVIEWS_BLOCK

## Decisions and deviations
- Five entries in DECISIONS, not the three his CHANGE 1 named: his messages of 20:58 and 21:33 hold decisions for
  this change-set. Two research files, not one. Part 15 is not in this change-set: it waits for his answers (its text:
  commit ea851a8, `.checkpoints/part-15/part-15-the-second-lane.md`; sent to him as a file).
- Two Art texts arrived with Windows line ends; the repository stores LF, so the appendix gives both sizes (4a1b0d7).
- The Advisor chat's log reached this chat by mistake. One read-only agent was started on it at 19:16:35 UTC, before
  his word at 19:59; five lines of its digest stood in the checkpoint card from 8ea40cd (19:44:56 UTC) to 9b31dad
  (20:01 UTC). His answer LOG-HISTORY: the history stays, so a merge commit brings those commits into main's history.
- Refusals and routes: the guard refused `git -C <dir>` (route: cd, then git); one long scratch command starting with
  `rm -rf` was denied (route: short commands in a new scratch folder). His message at 19:59 UTC stopped two running
  agents and the saver; all three were restarted from their notes by 20:05 UTC.

## Risks
1. The rules are on paper. The first real Blender export may fail PM-0005-49 or -56 (how the armature is written).
2. PM-0005-54 fails with today's rifle prototype (6 draws per pass against 4): his call, with change-set M's plan.
3. Change-set 1 waits for his answers to Part 15.

## Proposed next step
Change-set 1, Types (foundation step 2.4), once Part 15 is answered.

## Decisions needed
Nothing new in this change-set. Part 15's six questions (the lane split, the record files, three on Shamble VFX, the
gun files) are in the file sent on 2026-11-17 and were repeated in chat.

## For Mustafa
1. Your go on merging pull request #56 at head HEADSHA, after the advisor agent's review of that head. No Brave check.
2. Answer Part 15's six questions.
