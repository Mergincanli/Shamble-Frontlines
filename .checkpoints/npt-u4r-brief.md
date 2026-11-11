# Checkpoint: npt-u4r-brief
- **Goal:** draft the builder brief for U4r (the netcode proof test's L1 runner and its judged runs, with O1 / carry
  40), checked by netcode-architect and revised once by test-engineer; notes only, no build, no test run, no pull
  request · plan: item 4 of `night-2026-11-10.md` at 2dc1f31 (`git show 2dc1f3131c6b20c089045ae8611ad8a883940084:.checkpoints/netcode-proof-test/night-2026-11-10.md`),
  approved in issue #16's Queue under Mustafa's rule of 2026-11-09
- **Branch:** cs/npt-u4r-brief · **Base:** origin/main 2d22b69
- **Spike read in:** `.gate/npt-spike` (read-only worktree, detached at df8975b; never edited)
- **Last update:** 2026-11-11 03:24 Amsterdam
- **State: DONE.** Rev 2, the check and the notes are committed and pushed. The day session folds them into PR #40
  (plan rule 2: no pull request, delete nothing).

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
4. test-engineer (Opus 5.5, xhigh) wrote rev 2 in the same file (about 1,010 lines), "Changes in rev 2" at its top,
   notes `test-engineer-u4r-rev2.md` (DONE), scratch `rev2-parts/` · this commit · 16 of 16 findings accepted, none
   rejected, two with a correction to the check's own fix: N2 (X3: at df8975b a P0 ack's sample applies at its
   packet's own send instant, l1/src/lib.rs:515, :527; checked by the night main session; so the run-level
   early-input bite may never fire, and rev 2 recommends a unit test on doctored records, new test r7, U4R-Q10
   (b')), N6 (the recording header, plan :463, has no start tick; recheck R30). Extra defects fixed: X1 run items
   renamed D1-D12 (they collided with the check's N ids), X2 Loopback and L1 may admit a client at different
   instants, X4 rev 1's room log had no per-tick World hash for the comparer, X6 rev 1's 8.3 wall-time range was
   wrong (2,100 s simulated is 3.5 to 35 min)

## Current step
none: the item is done.

## Next steps (for the day session)
1. Fold `.checkpoints/npt-u4r-brief/` into PR #40's notes (`parts/`, `notes-*.md` and `rev2-parts/` are scratch).
2. After U4r's dependencies (U4c, U4d2, U4e, U4j) merge: fill `<base>`, walk the "Recheck" table (section 9, R1-R34),
   run the "needs a run (day)" commands (section 11, D1-D12), then send the brief and its 12 questions to the advisor.

## Open findings
- Advisor questions, all 12 open in rev 2 (U4R-Q1..Q12): among them Q1 (npt-lab has no npt-bots dependency and no
  unit owns that manifest edit: blocks the bots until ruled), Q2 (no owner for the SERIES.md rows, the recordings
  manifest, the determinism-step file), Q3 (the clock-ban clippy.toml for npt-lab and npt-bots unowned), Q4 (four -21
  suites and half of -20's P3 need InputBuffer option A, U4f's), Q5 (-58's "0 untraced corrections" needs U4g's
  Predictor: INVALID runs or a 0 that cannot fail), Q6 (the wallclock bite's site is npt-server's pump, outside U4r's
  files), Q10 (show the first-sample VOID rule by a unit test, (b'), or a run bite, (a')), Q12 ("6 runs": (i) one
  recording, three pairs, 1,800 s simulated, recommended; (ii) -30's three-recording shape, 10 runs plus 2
  recordings, 3,600 s). For the advisor via the day session, not Mustafa.
- Not settled without the merged code (rev 2's R2/D2, R4/D12, R15, R17, R30, R33/D10, R34/D11): `rtt_ns` before the
  first sample; when the server end first holds a sample at P0; where a pump call reads `rtt_ns`; option B's lead
  and late counting; the World hash's name and whether a recording carries its start tick; every RTT reader and any
  replace or clip path; when the client sends `Resync`, and the event cap.

## Key numbers
- wall-time estimate: 15,744 s simulated (472,320 ticks at 30 Hz), 13,944 s if -18 reuses -17's B1 runs; at
  300 / 90 / 30 / 10 ticks per second about 26 min / 1.5 h / 4.4 h / 13.1 h plus builds; the runner's ticks per
  second measured first (release build), before the set starts
- rev 2: 12 judged-run table rows (11 parts + the pre-check, not judged), 4 carries, 34 recheck rows, 12 questions
  (12 open), 12 "needs a run (day)" items, tests r1-r7

## Decisions taken
- Items worked one at a time, top to bottom, in the main folder (`git switch -c <new> origin/main` per item) · night
  shift · 2026-11-11 02:18

## Running work
- none: the three agents finished (02:44, 03:02, 03:22); the saver (bw73qu3vn) stopped with TaskStop at 03:23 after
  its autosaves (8571db0, 3b704f2, 89690a0, c3e2a82, f2dce34, b1aa521)
