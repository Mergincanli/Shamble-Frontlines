# Checkpoint: lane1-takeover
- **Goal:** the chat Mustafa calls Builder V4 takes over lane 1 of Slice 1's build from Builder V3 at the merge of
  change-set 3b (#68). First his answer to ORDER-3C, then the plan for the stop he chooses · his block:
  `.checkpoints/lane1-takeover/mustafa-2026-11-25-takeover.md` (by script; to be recorded word for word in
  `docs/DECISIONS.md` with this chat's first pull request, his point 2)
- **Branch:** `claude/cool-pasteur-oiqk1r` on GitHub (it stands in for a `cs/` branch). Local: `cs/lane1-takeover` in
  the worktree `.gate/lane1-takeover`; pushed with `git push origin cs/lane1-takeover:claude/cool-pasteur-oiqk1r`,
  alone in its command (the local branch tracks origin/main: never a plain `git push`). The main folder stays on
  its start commit and is never switched · **Base:** origin/main 3ab9eaecf42e617578c083b7fda01fbdf8f004e2
- **Last update:** 2026-11-25 17:45 Amsterdam

## Steps done
1. The takeover checks (2026-11-25, about 15:55 to 16:20 UTC; told to him in the chat's first answer, ten lines):
   the session's model and mode read; origin/main is exactly 3ab9eaec; Builder V3's branch
   `claude/loving-hopper-6p0mg2` has one commit (08d4d3d9) on 5e02ee73 that adds four notes files and nothing else;
   the handover note read; the repository's guard loads (it refused a read-only `git -C` probe with its own
   message); the GitHub tools to open and merge pull requests are present and reading works (issue #59, the open
   list), writing not yet tried.
2. ORDER-3C answered in the chat in five lines, then written out with its evidence:
   `.checkpoints/lane1-takeover/question-2026-11-25-order-3c.md`.
3. His block saved by script (`chat-words.mjs`, Builder V3's with the session and folder changed): 5,779 bytes,
   sha256 0dc59aaff9788c5e3ba0ec035fe47798a38da5d6261aecdb60fc7a2969d306a1, sent 2026-11-25T15:51:21.705Z.
4. The board add-on is active for this chat (since 2026-11-25).

## Current step
Stopped for his answer to ORDER-3C: (a) 3c whole then Walk, (b) Walk then 3c whole, (c) the recording as Walk's
first step and the link simulator after Walk, before change-set 7. Recommended: (c). Nothing is planned or built
before his letter.

## Next steps
1. On his letter: save his message by script, then open the chosen stop's checkpoint (its own slug; in this
   worktree `git switch -c cs/<slug>`), and write its plan first. No edit outside the notes before the advisor
   agent's APPROVE (and his go where the plan leaves the build plan).
2. Read before the plan, not yet read by this chat: `docs/REPO_MAP.md`, `docs/START_HERE.md`,
   `docs/MODULE_MAP.md`; `.claude/agents/advisor.md`; the reports of 3a and 3b whole
   (`docs/reports/2026-11-24-cs3a-rules-boundaries.md`, `2026-11-25-cs3b-the-loop-closes.md`); for Walk: ADR 0004
   parts 3, 7 and 10, ADR 0005's content parts, ADR 0001 part 5, `docs/LOOP_V2.md` section 7,
   `docs/legacy-tuning.json`, the build plan's Parts 4, 5 and 7, the archive's blocks "From change-set 3a" and
   "From change-set 3b", change-set 3's plan and the reviewers' notes at 80a51e42 under
   `.checkpoints/cs3b-loop-closes/`.
3. What the plan must hold, by his block (points 7 and 8): sized at the ratio 3a and 3b ran at (1.7 and 1.8 times),
   test lines counted, a stop line where the chat comes back to him. If it is change-set 4: the built-file check
   for the fused multiply-add (MUL-ADD); what the first picture after a pause shows (3B-RESUME-FLASH, his
   decision); SLICE-FEEL priced and placed (aiming down sights, the sprint-to-fire delay); lane 2's first list on
   issue #59 answered row by row (16 fields, 6 events, 5 wishes, W1 and W4 now needs); 3B-ONE-PER-TICK; and his
   Testlines values (point 8, a to g): the sandbox is read, no code copied; the Export's JSON is asked of him and
   saved whole by script with its bytes and sha256; one table of values; `old_game_terms` compared by script with
   `docs/legacy-tuning.json`, the differences listed for his yes; a script with a `--check` mode; the list of what
   is not taken; 30 steps a second (tell him first if the Export says 60); where he tunes after change-set 4.
4. From Builder V3's note (advice, not his word): plan at 1.8 times; test-engineer and security-auditor read the
   plan's test list before the build; each check written with its planted defect; the size ladder's cut points
   fixed before step 1; the advisor agent reads the report's draft before the zip is built; one browser-test run
   at a time (port 4173); a change-set's last commit carries no `[skip ci]`.
5. Pull requests (his point 9): this chat opens its own drafts if the GitHub tools write; if not, the title and
   text go into the notes (`pr-title.txt`, `pr-body.md`). A merge only on his go for the head the advisor agent
   approved, with that full commit id as the expected head, after his Advisor's merge check.
6. Later, on his word only (his point 10): one small pull request for lane 2, a second page in the build setup.

## Open findings
- The checkpoint script cannot push from this worktree: it pushes the local branch's own name, and this chat may
  push only to `claude/cool-pasteur-oiqk1r`. Notes are committed by path and pushed by hand, as Builder V3 did; the
  background saver cannot be used as it stands · this chat, 2026-11-25 · open (told to him with the first plan).
- Change-set 3's plan (at 80a51e42, lines 142-143) says change-set 4's "own milestone needs L1's profiles"; the
  build plan places those marks at change-set 7 (lines 486-489) · the ORDER-3C question, point 2 · for his
  Advisor's check.

## Key numbers
- 3c's parts as change-set 3's plan sized them: the replay runner, comparer and first recording 275 lines; the
  link simulator 595 lines (`cs3-plan.md` at 80a51e42, lines 136-137). At 1.8 times: about 500 and about 1,070.
- Walk's card: 3,900 planned lines, about 18 hours (the build plan, line 250), before SLICE-FEEL's additions.

## Decisions taken
- None yet. His rules for this chat that are not on main (his block of 2026-11-25): no CI, kit or gate file is
  written here; never a push to another chat's branch (the six he lists); until the guard is fixed, no `env`,
  `stdbuf` or `nice` with options in front of `git` or `gh`, in this session or its agents.

## Running work
- None: no agent, no workflow, no saver.
