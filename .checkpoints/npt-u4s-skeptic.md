# Checkpoint: npt-u4s-skeptic
- **Goal:** a skeptic check of U4s's merged code (spike commit f080551 against its parent 5f61f8f: the seam types, the
  wire grid, the tick constants and the manifests; 19 files, +1,979 −13 outside `.checkpoints/`, confirmed with
  `git diff --stat`); notes only, no build, no test run, no pull request · plan: item 3 of `night-2026-11-10.md` at
  2dc1f31, approved in issue #16's Queue under Mustafa's rule of 2026-11-09
- **Branch:** cs/npt-u4s-skeptic · **Base:** origin/main 44446a5
- **Spike read in:** `.gate/npt-spike` (read-only worktree, detached at df8975b; never edited)
- **Last update:** 2026-11-10 06:47 Amsterdam

## Summary (findings by severity, each with its proof)
pending the rust-reviewer. netcode-architect's part (DONE 06:46): no Critical or High.
- S1 Medium (may become High): one field layout per kind serves two catalogue rows. The soldier layout carries
  velocity (`seam.rs:114-116`) that the remote-soldier row leaves out (`messages.md:137`), and has no ammo or fire
  timers, which the own-state row needs (`:136`). The doc at `seam.rs:139-141` calls the lists "the catalogue's spatial
  rows", which is false for velocity. **Checked by the night main session at 06:46 by reading those lines: holds.** Not
  High: it costs bytes (about +54 % per remote soldier record, the agent's estimate) and risks a false FAIL, never a
  false pass. Proof: read the lines; day run: a npt-proto test that the remote soldier group's width equals the row's.
  Fix: an audience per layout entry or a separate own-state layout, before U4a's table lands.
- S2 Low: nothing ties `TickInput::tick` to the World's counter (`seam.rs:67-71`, `:80-81`, `:201-204`); a World built
  with a first tick other than the room's `tick_start` would miss every history lookup (lag compensation silently off).
  Proof: `debug_assert_eq!(input.tick, self.next_tick)` in U4d's `World::step`; a U4e test at `tick_start` 63,736.
- G1 Low (design note for the ADR): the per-tick 1 cm snap makes speed depend on direction and tick rate (about 6 %
  diagonal vs straight at 60 Hz; below 0.15 m/s per axis at 30 Hz never moves). Needs a run (day): a World test.
- G2 Low: a projectile's start tick is 32 bits per add where the catalogue re-bases it (`messages.md:139`).
- T1 Low: `LOOP_CATCHUP = TICK_HZ / 4` is 0 for 1-3 Hz, which the 1-1,000 assert allows (`ticks.rs:18-21`, `:42-47`).

## Steps done
1. Branch set up (the worktree from item 2 reused as it is, plan rule 1) · cd2a3e3 · ok
2b. netcode-architect (xhigh) → `.checkpoints/npt-u4s-skeptic/netcode-architect-u4s.md` (DONE 06:46) · 5 findings,
   no Critical or High; the seam, the grid and the tick constants otherwise held up

## Current step
2a. rust-reviewer (xhigh) → `.checkpoints/npt-u4s-skeptic/rust-reviewer-u4s.md`, asked to finish by 07:05.

## Next steps
3. The night main session tries once more to refute each Critical or High; a refuted finding is kept with its
   refutation.
4. The summary at the top of this file; commit and push; stop the saver; night report paragraph; tick item 3.

## Open findings
- none yet

## Key numbers
- none

## Decisions taken
- Started at 06:37, before the 06:45 no-new-item time; agents time-boxed to 07:05 so the report is done by 07:20 ·
  night shift · 2026-11-10

## Running work
- rust-reviewer (xhigh) · notes `.checkpoints/npt-u4s-skeptic/rust-reviewer-u4s.md` · launched 06:37 · running
- netcode-architect (xhigh) · notes `.checkpoints/npt-u4s-skeptic/netcode-architect-u4s.md` · launched 06:37 · running
- saver: background task bnnx8ohmi · started 06:37 · stops by 08:27 (110 minutes on) · last check 06:40: started
- Night report comment: issue #16 comment 6759962485 (created 06:40 with item 2); status comment 6759514658.
