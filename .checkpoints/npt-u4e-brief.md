# Checkpoint: npt-u4e-brief
- **Goal:** draft the builder brief for U4e (the netcode proof test's room and codec path), checked by test-engineer
  and revised once; notes only, no build, no test run, no pull request · plan: item 2 of
  `night-2026-11-10.md` at 2dc1f31 (`git show 2dc1f3131c6b20c089045ae8611ad8a883940084:.checkpoints/netcode-proof-test/night-2026-11-10.md`),
  approved in issue #16's Queue under Mustafa's rule of 2026-11-09
- **Branch:** cs/npt-u4e-brief · **Base:** origin/main 44446a5
- **Spike read in:** `.gate/npt-spike` (read-only worktree, detached at df8975b; never edited)
- **Last update:** 2026-11-10 06:38 Amsterdam
- **State: DONE.** Rev 2, the check and the notes are committed and pushed. The day session folds them into PR #40
  (plan rule 2: no pull request, delete nothing).

## Steps done
1. Branch and read-only worktree set up · d152470 · ok
2. netcode-architect (Opus 5.5, xhigh) drafted rev 1, `.checkpoints/npt-u4e-brief/u4e-brief-draft.md`, notes
   `netcode-architect-u4e.md` (DONE) · ea2dd8a · all sections by 06:18: 8 carries quoted (21-27, 39), recheck table
   28 rows, advisor questions U4E-Q1..Q10, 4 "needs a run (day)" items
3. test-engineer (Opus 5.5, xhigh) checked rev 1, `test-engineer-u4e-check.md` (DONE) · aa535c8 (the saver's commit)
   · REVISE: no Critical or High; 11 Medium, 12 Low; carries 21-27 and 39 word for word (compared by eye: the night
   shell has no diff); the plan already answers Q7 and Q9 (and Q1 in substance); 16 recheck rows spot-checked
4. netcode-architect (xhigh) wrote rev 2 in the same file, "Changes in rev 2" list at the top, notes
   `netcode-architect-u4e-rev2.md` (DONE) · 3a7ce98 · 23 of 23 findings accepted, none rejected; e5 split into e5a
   (B evicted: passes in both builds) and e5b (first snapshot after the gap coded against the per-type defaults: fails
   under the no-fallback bite); Q2 gains option (c) (U4e builds the rebuild, the judged run waits for U4g); Q4 (a)
   now says it leaves carry 24's tick16 half unmet; Q6 names U4f; Q7 and Q9 answered (kept as codes, marked
   answered); Q11 added (U4f's "server-side checks" read as their -21 counters and judged runs)

## Current step
none: the item is done.

## Next steps (for the day session)
1. Fold `.checkpoints/npt-u4e-brief/` into PR #40's notes.
2. After wave 1 (U4a, U4b, U4d) merges: fill `<base>` and `<floor>`, walk the "Recheck after wave 1" table (section 7,
   28 rows), run the "needs a run (day)" commands, then send the brief and its 9 open questions to the advisor.

## Open findings
- Advisor questions still open in rev 2: U4E-Q1 (confirm only: U4e plants lt-cmd-seq and lt-press), Q2 (lt-tick-offset:
  option (c) recommended), Q3 (bites-only `BITES` lists in npt-room and npt-server), Q4 (where the tick16 bite ids live;
  waits on recheck row R11), Q5 (`rewind_at` with a view time outside the ring before U4h), Q6 (the event queue's cap:
  U4f measures it), Q8 (hash-map bans in client's clippy.toml), Q10 (-45 and -62 ids), Q11 (the U4f overlap). For the
  advisor via the day session, not Mustafa.
- Cites taken from the check without reopening (rev 2's change list names them): F16's :627, F18's advisor :361,
  F19's :632, F20's :833-834. For the day session's recheck.
- Recheck rows not spot-checked by the check: R2, R3, R5, R7, R11, R12, R14, R22, R24, R25, R26.

## Key numbers
- rev 1: 8 carries, 28 recheck rows, 10 advisor questions, tests e1-e12, 8 bite rows
- check: tests 9 (7 testable, 2 not as worded), negatives 7/7, bites 9 (5 testable, 4 not), carries 8 (6 testable,
  2 not); 11 Medium, 12 Low
- rev 2: 23/23 accepted; +199 −85 lines against rev 1 (the reviser's count); 11 question codes, 9 open

## Notes for the night report
- This run started at 06:04 Amsterdam (05:04 UTC); issue #16 has no comment from an earlier run tonight. The schedule
  in STATUS ("Routines", line 785) is hourly from 00:10. The nights of 2026-11-08 and 11-09 also posted first at
  05:04 UTC. With the 06:45 no-new-item rule, items 3-5 may not all fit tonight.

## Decisions taken
- Items worked one at a time, top to bottom, in the main folder (the guard refuses `git -C` and `cd`, and the main
  folder may only `git switch -c <new> origin/main`) · night shift · 2026-11-10 06:06

## Running work
- none: the three agents finished (06:18, 06:26, 06:35); the saver (bmd9klxvv) stopped with TaskStop at 06:37 after
  its autosaves (1637b7a, aa535c8)
