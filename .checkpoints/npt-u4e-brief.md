# Checkpoint: npt-u4e-brief
- **Goal:** draft the builder brief for U4e (the netcode proof test's room and codec path), checked by test-engineer
  and revised once; notes only, no build, no test run, no pull request · plan: item 2 of
  `night-2026-11-10.md` at 2dc1f31 (`git show 2dc1f3131c6b20c089045ae8611ad8a883940084:.checkpoints/netcode-proof-test/night-2026-11-10.md`),
  approved in issue #16's Queue under Mustafa's rule of 2026-11-09
- **Branch:** cs/npt-u4e-brief · **Base:** origin/main 44446a5
- **Spike read in:** `.gate/npt-spike` (read-only worktree, detached at df8975b; never edited)
- **Last update:** 2026-11-10 06:20 Amsterdam

## Steps done
1. Branch and read-only worktree set up · d152470 · ok
2. netcode-architect (Opus 5.5, xhigh) drafted `.checkpoints/npt-u4e-brief/u4e-brief-draft.md` (rev 1), notes
   `netcode-architect-u4e.md` (DONE) · ea2dd8a · all sections complete by 06:18: 8 carries quoted (21-27, 39),
   recheck table 28 rows (U4a R1-R14, U4b R15-R16, U4d R17-R24, R25-R28 wave 2 / U4j), advisor questions
   U4E-Q1..Q10, 4 "needs a run (day)" items

## Current step
3. test-engineer (Opus 5.5, xhigh) checks rev 1: `.checkpoints/npt-u4e-brief/test-engineer-u4e-check.md` (launched
   06:20, asked to finish by 06:45).

## Next steps
4. netcode-architect revises once: rev 2 of the draft (same file, `u4e-brief-draft.md`) with a change list at its top,
   answering the check's findings.
5. Commit and push; stop the saver (TaskStop); night report paragraph; tick item 2 in issue #16.

## Open findings
- Draft's own open points (rev 1): no wave-1 builder brief exists in the spike's .checkpoints, so the shape follows the
  plan's item text; `CARGO_NET_OFFLINE` and `df` come from the item text, not plan §7.1; the `df` floor is a
  `<floor>` placeholder; marks.json's bite ids not read (recheck row R27). Status: open, for the check.

## Key numbers
- rev 1: 8 carries, 28 recheck rows, 10 advisor questions, tests e1-e12, 8 bite rows

## Notes for the night report
- This run started at 06:04 Amsterdam (05:04 UTC); issue #16 has no comment from an earlier run tonight. The schedule
  in STATUS ("Routines", line 785) is hourly from 00:10. The nights of 2026-11-08 and 11-09 also posted first at
  05:04 UTC. With the 06:45 no-new-item rule, only item 2 fits tonight; items 3-5 stay unchecked.

## Decisions taken
- Items worked one at a time, top to bottom, in the main folder (the guard refuses `git -C` and `cd`, and the main
  folder may only `git switch -c <new> origin/main`) · night shift · 2026-11-10 06:06

## Running work
- netcode-architect (xhigh) drafting · launched 06:07 · done 06:18 (DONE in its notes)
- test-engineer (xhigh) checking · notes `.checkpoints/npt-u4e-brief/test-engineer-u4e-check.md` · launched 06:20 ·
  running
- saver: background task bmd9klxvv · its output file in the session's tasks folder · started 06:06 · stops by 07:56
  (110 minutes on) · last check 06:20: ok
