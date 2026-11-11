# Checkpoint: npt-u4r-brief
- **Goal:** draft the builder brief for U4r (the netcode proof test's L1 runner and its judged runs, with O1 / carry
  40), checked by netcode-architect and revised once by test-engineer; notes only, no build, no test run, no pull
  request · plan: item 4 of `night-2026-11-10.md` at 2dc1f31 (`git show 2dc1f3131c6b20c089045ae8611ad8a883940084:.checkpoints/netcode-proof-test/night-2026-11-10.md`),
  approved in issue #16's Queue under Mustafa's rule of 2026-11-09
- **Branch:** cs/npt-u4r-brief · **Base:** origin/main 2d22b69
- **Spike read in:** `.gate/npt-spike` (read-only worktree, detached at df8975b; never edited)
- **Last update:** 2026-11-11 03:04 Amsterdam

## Steps done
1. Branch and read-only worktree set up · 4b8df30 · ok
2. test-engineer (Opus 5.5, xhigh) drafted rev 1, `.checkpoints/npt-u4r-brief/u4r-brief-draft.md`, notes
   `test-engineer-u4r.md` (DONE) · 3bdfb3b · 11 sections: 12 judged-run rows (-17 ×4, -57 ×4, -58 ×2, -18 ×1, the
   -30 pre-check), 4 carries quoted (40 routed to U4r; 6, 14, 20 bind its code), 32 recheck rows, 12 advisor
   questions (U4R-Q1..Q12), 9 "needs a run (day)" items; O1: re-run after U4h (6 runs read as 3 pairs); the
   advisor's O1 cites :678-679 and :787 are stale at df8975b (now :690-691, :799-800). The agent had no Edit tool:
   it wrote parts/p1-p5.md and notes-*.md as scratch (out of date, kept)

3. netcode-architect (Opus 5.5, xhigh) checked the -30 pre-check part, O1, the RTT argument and -18's fresh-delta
   reading: `netcode-architect-u4r-check.md`, notes `netcode-architect-u4r-check-notes.md` (DONE) · 524d310 ·
   REVISE: 0 Critical, 0 High, 8 Medium (N1 the RTT rule names the wrong link end, so it could never fire; N2 the
   early-input bite cannot fire as worded at P0; N4 "6 runs" has a second reading, 3 recordings × 2 transports; N5
   O1's room claims rest on the plan only, crates/room and crates/server are module docs at df8975b; N6 no send
   timing or tick origin for the replay clients, so late_count may void every pair; N7 "per-tick server hashes" must
   be the World hash, not the mirror hash; N8 the wallclock bite must change a sim input every run; N9 -18's heal
   must count Snapshot deltas only, never a FullState), 8 Low. Confirmed: carry 40 word for word, O1's statement at
   :690-691 and :799-800, the stale cites, the pre-check's parts, Loopback Some(0) and L1 None before its first
   sample, the RTT argument sound at P0, -18's fresh-delta reading

## Current step
4. test-engineer (Opus 5.5, xhigh) revises once: rev 2 in the same file, "Changes in rev 2" at its top, notes
   `test-engineer-u4r-rev2.md`, scratch `rev2-parts/`.

## Next steps
1. Commit and push; summary in this checkpoint; the night report's paragraph; tick item 4.
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
- netcode-architect (Opus 5.5, xhigh), checked rev 1 · launched 02:45 · done 03:02
- test-engineer (Opus 5.5, xhigh), revising to rev 2 · launched 03:03 · running
- saver: background task bw73qu3vn · output in the session's tasks folder (bw73qu3vn.output) · started 02:18 · stops
  by 04:08 (110 minutes on) · last check 03:04: ok (autosaves 8571db0, 3b704f2, 89690a0, c3e2a82)
