# Checkpoint: npt-u4e-brief
- **Goal:** draft the builder brief for U4e (the netcode proof test's room and codec path), checked by test-engineer
  and revised once; notes only, no build, no test run, no pull request · plan: item 2 of
  `night-2026-11-10.md` at 2dc1f31 (`git show 2dc1f3131c6b20c089045ae8611ad8a883940084:.checkpoints/netcode-proof-test/night-2026-11-10.md`),
  approved in issue #16's Queue under Mustafa's rule of 2026-11-09
- **Branch:** cs/npt-u4e-brief · **Base:** origin/main 44446a5
- **Spike read in:** `.gate/npt-spike` (read-only worktree, detached at df8975b; never edited)
- **Last update:** 2026-11-10 06:28 Amsterdam

## Steps done
1. Branch and read-only worktree set up · d152470 · ok
2. netcode-architect (Opus 5.5, xhigh) drafted `.checkpoints/npt-u4e-brief/u4e-brief-draft.md` (rev 1), notes
   `netcode-architect-u4e.md` (DONE) · ea2dd8a · all sections complete by 06:18: 8 carries quoted (21-27, 39),
   recheck table 28 rows (U4a R1-R14, U4b R15-R16, U4d R17-R24, R25-R28 wave 2 / U4j), advisor questions
   U4E-Q1..Q10, 4 "needs a run (day)" items

3. test-engineer (Opus 5.5, xhigh) checked rev 1: `.checkpoints/npt-u4e-brief/test-engineer-u4e-check.md` (DONE) ·
   aa535c8 (the saver's commit) · REVISE: no Critical or High; 11 Medium (priority F1 no-fallback's proof cannot fail,
   F2 the 1 s gap vs the 32-tick ring, F21 the base-count command, F22 recheck R24's freeze test, F3 U4f overlap, F7
   "no generated edits" vs §7.2's scratch-script bites, F13 Q2 option (c), F15 Q6 says U4r for U4f, F14 Q4 leaves carry
   24 unmet, F4 e6 wording, F5 carry 22's test) and 12 Low; carries 21-27 and 39 word for word (compared by eye);
   plan already answers Q7 and Q9 (and Q1 in substance); 16 recheck rows spot-checked, builders right

## Current step
4. netcode-architect (xhigh) revises once: rev 2 of the draft (same file, `u4e-brief-draft.md`) with a "Changes in
   rev 2" list at its top, notes `netcode-architect-u4e-rev2.md` · launched 06:28, asked to finish by 06:50.

## Next steps
5. Commit and push; stop the saver (TaskStop); night report paragraph; tick item 2 in issue #16.
6. If before 06:45: item 3 (npt-u4s-skeptic) on `git switch -c cs/npt-u4s-skeptic origin/main`; else no new item.

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
- test-engineer (xhigh) checking · launched 06:20 · done 06:26 (DONE in its notes)
- netcode-architect (xhigh) revising · notes `.checkpoints/npt-u4e-brief/netcode-architect-u4e-rev2.md` · launched
  06:28 · running
- saver: background task bmd9klxvv · its output file in the session's tasks folder · started 06:06 · stops by 07:56
  (110 minutes on) · last check 06:28: ok (autosave aa535c8)
