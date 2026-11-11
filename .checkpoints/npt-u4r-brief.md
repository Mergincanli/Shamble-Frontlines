# Checkpoint: npt-u4r-brief
- **Goal:** draft the builder brief for U4r (the netcode proof test's L1 runner and its judged runs, with O1 / carry
  40), checked by netcode-architect and revised once by test-engineer; notes only, no build, no test run, no pull
  request · plan: item 4 of `night-2026-11-10.md` at 2dc1f31 (`git show 2dc1f3131c6b20c089045ae8611ad8a883940084:.checkpoints/netcode-proof-test/night-2026-11-10.md`),
  approved in issue #16's Queue under Mustafa's rule of 2026-11-09
- **Branch:** cs/npt-u4r-brief · **Base:** origin/main 2d22b69
- **Spike read in:** `.gate/npt-spike` (read-only worktree, detached at df8975b; never edited)
- **Last update:** 2026-11-11 02:46 Amsterdam

## Steps done
1. Branch and read-only worktree set up · 4b8df30 · ok
2. test-engineer (Opus 5.5, xhigh) drafted rev 1, `.checkpoints/npt-u4r-brief/u4r-brief-draft.md`, notes
   `test-engineer-u4r.md` (DONE) · 3bdfb3b · 11 sections: 12 judged-run rows (-17 ×4, -57 ×4, -58 ×2, -18 ×1, the
   -30 pre-check), 4 carries quoted (40 routed to U4r; 6, 14, 20 bind its code), 32 recheck rows, 12 advisor
   questions (U4R-Q1..Q12), 9 "needs a run (day)" items; O1: re-run after U4h (6 runs read as 3 pairs); the
   advisor's O1 cites :678-679 and :787 are stale at df8975b (now :690-691, :799-800). The agent had no Edit tool:
   it wrote parts/p1-p5.md and notes-*.md as scratch (out of date, kept)

## Current step
3. netcode-architect (Opus 5.5, xhigh) checks the -30 pre-check part, O1, the RTT argument and -18's fresh-delta
   reading: `netcode-architect-u4r-check.md`, notes `netcode-architect-u4r-check-notes.md`.

## Next steps
1. test-engineer (xhigh) revises once: rev 2 in the same file, a change list at its top, notes
   `test-engineer-u4r-rev2.md`.
2. Commit and push; summary in this checkpoint; the night report's paragraph; tick item 4.
3. Then item 5 (npt-u3d-skeptic) on its own branch `cs/npt-u3d-skeptic` from origin/main.

## Open findings
- rev 1's blockers, as advisor questions: Q1 (npt-lab has no npt-bots dependency, no unit owns that manifest edit),
  Q2 (no owner for SERIES.md rows, the recordings manifest, the determinism-step file), Q3 (clock-ban clippy.toml for
  npt-lab and npt-bots unowned), Q4 (four -21 suites and half of -20's P3 need InputBuffer option A, U4f's), Q5 (-58's
  "0 untraced corrections" needs U4g's Predictor), Q6 (the wallclock bite's site is npt-server's pump, outside U4r's
  files)

## Key numbers
- wall-time estimate (rev 1): 15,744 s simulated (472,320 ticks at 30 Hz), 13,944 s if -18 reuses -17's B1 runs; at
  300 / 90 / 30 / 10 ticks per second about 26 min / 1.5 h / 4.4 h / 13.1 h plus builds

## Decisions taken
- Items worked one at a time, top to bottom, in the main folder (`git switch -c <new> origin/main` per item) · night
  shift · 2026-11-11 02:18

## Running work
- netcode-architect (Opus 5.5, xhigh), checking rev 1 · launched 02:45 · running
- saver: background task bw73qu3vn · output in the session's tasks folder (bw73qu3vn.output) · started 02:18 · stops
  by 04:08 (110 minutes on) · last check 02:46: ok (saved and pushed 8571db0, 3b704f2)
