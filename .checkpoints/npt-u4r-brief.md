# Checkpoint: npt-u4r-brief
- **Goal:** draft the builder brief for U4r (the netcode proof test's L1 runner and its judged runs, with O1 / carry
  40), checked by netcode-architect and revised once by test-engineer; notes only, no build, no test run, no pull
  request · plan: item 4 of `night-2026-11-10.md` at 2dc1f31 (`git show 2dc1f3131c6b20c089045ae8611ad8a883940084:.checkpoints/netcode-proof-test/night-2026-11-10.md`),
  approved in issue #16's Queue under Mustafa's rule of 2026-11-09
- **Branch:** cs/npt-u4r-brief · **Base:** origin/main 2d22b69
- **Spike read in:** `.gate/npt-spike` (read-only worktree, detached at df8975b; never edited)
- **Last update:** 2026-11-11 02:22 Amsterdam

## Steps done
1. Branch and read-only worktree set up · (this commit) · ok

## Current step
2. test-engineer (Opus 5.5, xhigh) drafts rev 1: `.checkpoints/npt-u4r-brief/u4r-brief-draft.md`, notes
   `test-engineer-u4r.md`.

## Next steps
1. netcode-architect (xhigh) checks the -30 pre-check part, O1, the RTT argument and -18's fresh-delta reading:
   `netcode-architect-u4r-check.md`.
2. test-engineer (xhigh) revises once: rev 2 in the same file, a change list at its top, notes
   `test-engineer-u4r-rev2.md`.
3. Commit and push; summary in this checkpoint; the night report's paragraph.

## Open findings
- none yet

## Key numbers
- none yet

## Decisions taken
- Items worked one at a time, top to bottom, in the main folder (`git switch -c <new> origin/main` per item) · night
  shift · 2026-11-11 02:18

## Running work
- none yet
