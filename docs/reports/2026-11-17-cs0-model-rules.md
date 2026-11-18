# Advisor report: cs0-model-rules (2026-11-17, branch claude/wonderful-hamilton-jdmlnf, base main c94c81b)
Change-set 0 of the build plan; pull request #56; docs only; the branch stands in for `cs/cs0-model-rules`.

## Goal
Record Mustafa's decisions of 14 and 17 November, carry the build plan to main, and fix on paper the rules a model
file must follow, so that game code can start and the Blender work has its rules. Nothing a player sees.

## Outcome
DONE: built, reviewed in two rounds and pushed; it waits for Mustafa's go. The rules are on paper: no export yet.

## Plan and approval
- The advisor agent on the plan: REVISE at 19:30 UTC (1 High, 10 Medium, 5 Low), then APPROVE with conditions
  at 20:43 UTC. The High (the joint list could not carry the animation port): engine-architect ruled on 20
  joints; the advisor agent ruled that `root` is the node above the skeleton. The conditions, met before the first
  commit: two more pointers in DECISIONS, each with his COMPARE exception; the draw check counts draws, as PM-0005-36
  does; dated answers under OQ-0002-6 and OQ-0006-18; the size stop told to him in three lines (20:25 UTC).
- Mustafa, 20:39 UTC (SIZE-0): one pull request, stop line 1,200 written lines, his records and the plan's carry in
  commits of their own (3747bb3, 4a1b0d7; 212f55b), apart from the rules and the joint list (5f356ef, 65c8fe8, cacea55).

## What changed
- `docs/DECISIONS.md`: five entries by script (his decision of 2026-11-14, his addition, his go of 2026-11-17, his
  messages of 20:03 and 20:39 UTC) and six dated pointers. `docs/reports/`: the plan's report ("Since the plan", then
  the plan copied from commit 68addf9) and its appendix (A1 to A11: his messages and the Art lead's four texts).
- `docs/research/`: `asset-export-rules.md` (graphics-engineer) and `soldier-joint-list.md` (engine-architect):
  sixteen rules, ten checks, 20 joints, on paper (no export has gone through them). ADR 0005: OQ-0005-AS3 and AS4
  answered in part; ten new marks, PM-0005-47 to -56 (test-engineer). ADR 0002, 0004, 0006, the register, SPIKE_PLAN:
  his answers and dated pointers (open questions 98 to 96; SPIKE_PLAN's asset-spike lines name the new marks).
  LOOK_V2 section 8: one line. STATUS, ROADMAP, the repository map.

## Evidence
- His words: five entries equal byte for byte, and 11 appendix blocks with 0 mismatches (`decisions-insert.mjs --check`,
  `appendix-verify.mjs HEAD`); every quote in the pointers and in "Since the plan" is in his saved messages. The
  verifier extracted his five messages again from the transcript (equal) and confirmed 71 of 72 key claims of the
  research files at their sources; each file now carries its spot-check lines. Not verified by it: the smoke check here;
  A10 against his attachment; the renderer's draw loop behind "6 draws per pass".
- Gate: `node tools/gate.mjs --no-smoke` PASS; `repo-map-check.mjs` OK. Smoke cannot start here (Chromium 1194 wanted,
  1243 installed); CI's `gate`, `smoke` and `toolchain` were green on 4857f19 and run again on the last commit.
- Size (`size-count.mjs`): 1,200 written lines count toward the stop line of 1,200. Not counted, by his words of 22:10
  UTC, the lines that findings of the verifier and the advisor agent asked for: 129 in all (116 are the spot-check lines
  in the two research files, most of them copied from the verifier's notes; 4 the last sentence of his rule in seven
  pointers; 2 in the plan's report; 7 here). Copied, 1,788: his words in DECISIONS 123, the plan 1,275, the appendix's
  blocks 390 (68 of them sheet notes this session typed with the plan on 2026-11-14).

## Reviews
- Round 1: test-engineer, 21:45 UTC: REVISE (2 Medium, 4 Low, word changes in rows), all applied (f61f43c).
  engine-architect, 21:51 UTC: APPROVE (5 Low): four applied; the fifth ("the model's entry" as the one name for where
  socket lists and `palm` live) is deferred to the first plan that writes one into an entry, change-set 10 or M. The
  verifier, 22:12 UTC: PASS at cacea55 (2 Medium, 6 Low), all fixed (d6b6b30).
- Round 2, on what changed: the verifier, 22:36 UTC: PASS (1 Medium, 4 Low), all fixed in the last commits. The advisor
  agent, 22:35 UTC: APPROVE of the content at 47546e9, with five conditions for the last commits, all met; its word on
  the final head is in the pull request's description.

## Decisions and deviations
- Five entries in DECISIONS, not the three his CHANGE 1 named (his two later messages hold decisions for this
  change-set); two research files, not one; the appendix gives two sizes for the two Art texts that arrived with Windows
  line ends. Part 15, "The second lane", is not in it: he answered its six questions at 22:10 UTC, all (a) (its text:
  commit ea851a8; his words: commit 023e6f3; both under `.checkpoints/part-15/`). The notes of this change-set and of
  the plan are deleted in the last commit; the commit before it holds them: d2c4caa6450cabc99af21746ecf352c50efcbb77.
- The Advisor chat's log reached this chat by mistake. One read-only agent was started on it at 19:16:35 UTC, before
  his word at 19:59; five lines of its digest stood in the checkpoint card from 8ea40cd (19:44:56 UTC) to 9b31dad
  (20:01 UTC). His answer LOG-HISTORY: the history stays, so a merge commit brings those commits into main's history.
- Refused: `git -C <dir>` by the guard, 19:23 UTC (route: cd, then git); a scratch command that began with `rm -rf`,
  about 19:58 UTC (route: a new folder). His message at 19:59 UTC stopped two agents and the saver; all were restarted
  from notes by 20:05 UTC.

## Risks
1. The rules are on paper: the first real Blender export may fail PM-0005-49 or -56 (how the armature is written).
2. PM-0005-54 fails with today's rifle prototype (6 draws per pass against 4): his call, with change-set M's plan.
3. Change-set 1 waits for Part 15 to be on main with his answers.

## Proposed next step
Part 15 with his answers to main, as a docs change of its own (it also updates the rules file's line on gun files,
which still says "until he answers"); then change-set 1, Types (foundation step 2.4).

## Decisions needed
Nothing. He answered Part 15's six questions on 2026-11-17 at 22:10 UTC.

## For Mustafa
1. Your go on merging pull request #56, after the advisor agent's review of its final head. No Brave check.
