# Checkpoint: npt-u3d-skeptic
- **Goal:** a skeptic check of U3d's merged code (spike commit 1ccff06 against d012437: ImpairSocket's timing
  self-test and its seven fast tests, socket_tests.rs +1,057, SERIES.md, results/u3d-timing/run.sh; and the dry run's
  record 78b0c07): netcode-architect and test-engineer in parallel, then one refutation pass by the night main
  session; notes only, no build, no test run, no pull request · plan: item 5 of `night-2026-11-10.md` at 2dc1f31
  (`git show 2dc1f3131c6b20c089045ae8611ad8a883940084:.checkpoints/netcode-proof-test/night-2026-11-10.md`),
  approved in issue #16's Queue under Mustafa's rule of 2026-11-09
- **Branch:** cs/npt-u3d-skeptic · **Base:** origin/main 2d22b69
- **Spike read in:** `.gate/npt-spike` (read-only worktree, detached at df8975b; never edited)
- **Last update:** 2026-11-11 03:41 Amsterdam

## Summary (findings by severity, each with its proof)
- to follow

## Steps done
1. Branch set up (worktree reused from item 4); diff d012437..1ccff06 confirmed: 3 files, +1,227 −5; 78b0c07 adds
   results/u3d-timing/selftest-20261109T035556Z.txt (+113) · a47de32 · ok

## Current step
2. netcode-architect (`netcode-architect-u3d.md`: the measurement's validity) and test-engineer (`test-engineer-u3d.md`:
   the seven fast tests and their bites), Opus 5.5 at xhigh, in parallel.

## Next steps
1. The night main session tries once more to refute each Critical or High, and keeps a refuted finding with its
   refutation.
2. Summary at the top of this checkpoint; commit and push; the night report's paragraph; tick item 5.

## Open findings
- test-engineer (`test-engineer-u3d.md`, DONE, saved in 131f655): 0 Critical, 0 High, 2 Medium, 5 Low. All seven
  tests hold for their planted defects. TE1 (Medium): R-31's "never the reverse" (plan.md:1423, advisor-u3d.md:216)
  holds on average over the delay draws, not per run: at E = 10,000 ns (a PASS) -3's p99.9 fails about half the time
  (expected count above T = n × E / 2a = 990.0 against 990 allowed at n = 990,012), and U3D-MISS has no route for "E
  PASS, -3 FAIL". TE2 (Medium): socket.rs:726 changed from `sh.now_ns()` to the pop time `now` passes all seven (T7
  pops after the lock is released; T1, T2 never read down's stamp), though T7's doc (:1263-1264) and SERIES.md claim
  it. Lows: TE3 (swap socket.rs:501/:503 passes all seven), TE4 (`agree("MISS", true)` unasserted), TE5 (verdict
  words pinned on one side only), TE6 (T2's 1,000 ns tolerance), TE7 (run.sh reads no UDP drop counters; R1 is
  covered by C-U8-6). Status: follow-ups for U3d's report (review budget), each with its day command in the notes.

## Key numbers
- none yet

## Decisions taken
- Items worked one at a time, top to bottom, in the main folder (`git switch -c <new> origin/main` per item) · night
  shift · 2026-11-11 02:18

## Running work
- netcode-architect (Opus 5.5, xhigh), the measurement's validity · `netcode-architect-u3d.md` · launched 03:25 ·
  running
- test-engineer (Opus 5.5, xhigh), the seven fast tests and their bites · `test-engineer-u3d.md` · launched 03:25 ·
  done 03:40
- saver: background task bqpwo2txe · output in the session's tasks folder (bqpwo2txe.output) · started 03:24 · stops
  by 05:14 (110 minutes on) · last check 03:41: ok (autosave 2aa8bac)
