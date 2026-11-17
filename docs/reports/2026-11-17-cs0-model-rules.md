# Advisor report: cs0-model-rules (2026-11-17, branch claude/wonderful-hamilton-jdmlnf, base main c94c81b)
Change-set 0 of the build plan (`2026-11-17-build-plan-slice-1.md`); pull request #56; docs only; the branch stands
in for `cs/cs0-model-rules`.

## Goal
Record Mustafa's decisions of 14 and 17 November, carry the build plan to main, and fix on paper the rules a model
file must follow, so that game code can start and the Blender work has its rules. Nothing a player sees.

## Outcome
IN REVIEW (completed before the pull request is marked ready): built and pushed; review round 1 is under way.

## Plan and approval
- The advisor agent on the plan: REVISE at about 19:29 UTC (1 High, 10 Medium, 5 Low), then APPROVE with conditions
  at about 20:45 UTC. The High (the joint list could not carry the animation port): engine-architect ruled on 20
  joints; the advisor agent ruled that `root` is the node above the skeleton. The conditions, met before the first
  commit: two more pointers in DECISIONS, each with his COMPARE exception; the draw check counts draws, as PM-0005-36
  does; dated answers under OQ-0002-6 and OQ-0006-18; the size stop told to him in three lines.
- Mustafa, 20:39 UTC (SIZE-0): one pull request, stop line 1,200 written lines, his records and the plan's carry in
  commits of their own (3747bb3, 4a1b0d7; 212f55b), apart from the rules and the joint list (5f356ef, 65c8fe8, cacea55).

## What changed
- `docs/DECISIONS.md`: five entries by script (his decision of 2026-11-14, his addition, his go of 2026-11-17, his
  messages of 20:58 and 21:33) and six dated pointers. `docs/reports/`: the plan's report ("Since the plan", then the
  plan copied from commit 68addf9) and its appendix (A1 to A11: his messages and the Art lead's four texts).
- `docs/research/`: `asset-export-rules.md` (graphics-engineer) and `soldier-joint-list.md` (engine-architect):
  sixteen rules, ten checks, 20 joints, on paper (no export has gone through them). ADR 0005: OQ-0005-AS3 and AS4
  answered in part; ten new marks, PM-0005-47 to -56 (test-engineer). ADR 0002, 0004, 0006, the register, SPIKE_PLAN:
  his answers and dated pointers (open questions 98 to 96); SPIKE_PLAN's asset-spike lines also name the new marks and
  one pointer there says section 8 is written. LOOK_V2 section 8: one line. STATUS, ROADMAP, the repository map.

## Evidence
- His words: `decisions-insert.mjs --check`: five entries equal byte for byte; `appendix-verify.mjs HEAD`: 11 blocks,
  0 mismatches; every quote in the pointers and in "Since the plan" is in his saved messages (`quotes-check.mjs`).
- Gate: `node tools/gate.mjs --no-smoke` PASS; `repo-map-check.mjs` OK. The browser smoke check cannot start in this
  container (the gate's Playwright wants Chromium build 1194; the container has 1243): CI's `smoke` job runs it.
- The verifier: PASS at cacea55 on all six groups: his five messages extracted again from the transcript and equal;
  71 of 72 key claims of the research files confirmed at their sources; ADR 0005's rows equal test-engineer's file. Not
  verified by it: the smoke check; A10 against his attachment (its size matches his words); the renderer's draw loop
  behind "6 draws per pass". Its second look at the commits after cacea55, and CI on the final head: pending.
- Size (`size-count.mjs`): 1,195 written lines before the verifier's findings; the 6 lines they asked for do not
  count (his words of 22:10 UTC); stop line 1,200. Counted as copied, 1,788: his words 123, the plan 1,275, the
  appendix's blocks 390, of which 68 are sheet notes this session typed with the plan on 2026-11-14.

## Reviews
- test-engineer: REVISE (2 Medium, 4 Low, all word changes in rows): all applied (f61f43c).
- engine-architect: APPROVE (5 Low): four applied; the fifth ("the model's entry" as the one name for where socket
  lists and `palm` live, so none sits in content the simulation loads) is deferred to change-set M's plan.
- The verifier: PASS, 2 Medium and 6 Low, all fixed: seven pointers now carry his rule's last sentence ("If a mark
  fails, the code that rests on it is redone."); the plan's report says which copied texts still carry "no plain
  tubes"; its times and measured sizes; the research files' line citations renumbered to this branch; the pointer
  script's own check. The advisor agent on the final head: pending.

## Decisions and deviations
- Five entries in DECISIONS, not the three his CHANGE 1 named (his two later messages hold decisions for this
  change-set); two research files, not one. Part 15, "The second lane", is not in it: he answered its six questions at
  22:10 UTC, all (a) (its text: commit ea851a8; his words: commit 023e6f3; both under `.checkpoints/part-15/`). Two Art
  texts arrived with Windows line ends; the repository stores LF, so the appendix gives both sizes.
- The Advisor chat's log reached this chat by mistake. One read-only agent was started on it at 19:16:35 UTC, before
  his word at 19:59; five lines of its digest stood in the checkpoint card from 8ea40cd (19:44:56 UTC) to 9b31dad
  (20:01 UTC). His answer LOG-HISTORY: the history stays, so a merge commit brings those commits into main's history.
- Refused: `git -C <dir>` by the guard (route: cd, then git); a scratch command that began with `rm -rf` (route: a
  new folder). His message at 19:59 UTC stopped two agents and the saver; all were restarted from notes by 20:05 UTC.

## Risks
1. The rules are on paper: the first real Blender export may fail PM-0005-49 or -56 (how the armature is written).
2. PM-0005-54 fails with today's rifle prototype (6 draws per pass against 4): his call, with change-set M's plan.
3. Change-set 1 waits for Part 15 to be on main with his answers.

## Proposed next step
Part 15 with his answers to main, as a docs change of its own; then change-set 1, Types (foundation step 2.4).

## Decisions needed
Nothing. He answered Part 15's six questions on 2026-11-17 at 22:10 UTC.

## For Mustafa
1. Your go on merging pull request #56, after the advisor agent's review of its final head. No Brave check.
