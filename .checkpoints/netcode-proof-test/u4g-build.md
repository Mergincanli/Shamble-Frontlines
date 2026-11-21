Task: U4g's early builder (PR #40, lane claude/netcode-proof-test-plan-hcky5g), blocks 1-4 of u4g-brief.md 1.3 under U4G-EARLY (advisor-u4g-early.md conditions 1-10); stop after block 4, snapshot, return.

## Start, first try (2026-11-21): STOPPED, returned
- Worktree head f5030a33185b1ea1b15329174e82823f0215f993 (main; harness made it from main), no spikes/netcode-proof (`git rev-parse HEAD:spikes/netcode-proof` exit 128); df 7,662,850,048 B. Returned; the coordinator answered: fast-forward to b25ee7d (u4f-brief.md section 1 item 1's `git merge --ff-only <base>` step), then build as launched.

## Start, second try (after the coordinator's message)
- `git merge --ff-only b25ee7dd931add8c0f71b827b95270a983e93b2d`: fast-forward f5030a3..b25ee7d, done.
- `git log -1 --format=%H`: b25ee7dd931add8c0f71b827b95270a983e93b2d. PASS.
- `git rev-parse HEAD:spikes/netcode-proof`: 2b7647ffc3c1927e0bb8d6269bbf240217b8f6db. PASS.
- `git diff --stat 8f9d97d1ed86bc2767e9d16de4b60fd8c0e5f853 HEAD -- spikes/netcode-proof`: prints nothing. PASS.
- `df -B1 /home/user/Shamble-Frontlines`: 7,634,649,088 B available (>= 6.5 GB). PASS.
- `git status --short`: only this notes file untracked.

## Plan (blocks; each <= about 400 added lines in spikes/)
- B1 npt-sim: carry 47's reset + test, NC-F1-4 beside it; -55/vehicle-snap's site; TE-F1-5's twins only if open and cheap.
- B2 npt-client `Predictor` (+ tests in a new test file).
- B3 npt-room's half: carry 79 with -62/server-echo; -58's three counters only if condition 3 allows; -60's site.
- B4 npt-client `CueLog` (+ -62/cues-from-replay's site; tests in a new file).
- Stop: clippy (both targets) and fmt through the runner, snapshot, return.

## Read (before building)
- advisor-u4g-early.md whole (207 lines); u4g-brief.md whole (305 lines here, its section 9 the early fill); u4f-brief.md section 1 (:288-355) and section 4 (:831-958); .claude/rules/netcode.md; u4s-carries.md U4g section (:560-608); advisor-u4f-f1-r1.md :115-164; u4-build-plan.md §4.1-§4.2, §4.7-§4.14, U4g :881-901; advisor-u4g-brief.md Q3-Q9 (:135-186); marks.json -41, -55, -58, -60, -62; SERIES.md rows 25-28, 57, 83, 122, 178-183.

## Design decisions (one line each)
- D1 (block 1): `World::reset_slot(soldier, own, presses) -> bool` and `World::slot_records(slot)` in npt-sim; values from grid integers through the wire grid (= the authority's snapped state), respawn_left 0, records refreshed.
- D2 (block 1): -55/vehicle-snap = a `Bite::VehicleSnap` in world.rs, id in `World::CHECK_BITES` (2 -> 3), so npt-lab's router (scenario.rs:261) takes it with no lab edit; site: `collide`'s vehicle-part push gets `snap_out` (0.1 m, authority only, set in `run_command` beside -41's `gain`); static boxes pass 0.0 (bit-identical: x - 0.0 == x; x + 0.0 == x for x != -0.0, and min/max + RADIUS is never -0.0).
- D3 (blocks 2-4): the client's rate-derived values follow carry 6's pattern (the type takes `tick_hz`, `npt_sim::ticks_for_ms_at` at run time, no const derived from TICK_HZ outside npt-sim): the fade 100 ms (3 ticks at 30 Hz, 6 at 60 Hz) and the command history 2,000 ms (60 at 30 Hz, 120 at 60 Hz); a test pins both at 30 and 60 Hz. Recorded per carry 39 (name, definition, values, site): no new npt-sim constant needed.

## Block 1 (npt-sim) — built, tests pass; block-end workspace run below
- world.rs: `reset_slot`, `slot_records` (after `predict`); `Bite::VehicleSnap` + `VEHICLE_SNAP_M` (bites only), BITE_IDS 6 -> 7, CHECK_BITES 2 -> 3; `collide(.., snap_out)`, `run_command`'s `snap_out` beside -41's `gain`.
- New file crates/sim/tests/predict.rs: measured +2 (carry_47_a_world_reset..., carry_47_the_reset_refuses...); bites +5 (those 2 + pm_0001_55_vehicle_snap..., te_f1_5_no_cooldown..., te_f1_5_refuse_not_clamp...). TE-F1-5 taken (open and cheap: a new file, ~70 lines).
- Logs: .gate/u4f/u4g-early-b1-sim1.log (both builds, exit 0); u4g-early-b1-sim2.log (vehicle-snap figures: most apart clean 0 cm, bitten 10 cm; blocked moves 180 of 232 in each).
- Plant NC-F1-4 ("the reset leaves the press counters": `self.claimed[slot] = presses;` -> `let _ = presses;` by Edit): carry_47_a_world_reset... FAILED by name at predict.rs:42 (OwnState grid ammo 29 vs 30, fire timer 2000001 vs -999999); log .gate/u4f/u4g-early-b1-plant-ncf14.log (runner -x -k, exit 101); restored by Edit, world.rs sha256 dbd08f351c8eea39d4185832f763257112d84d2c294e124182a0a196a7848be8 = the clean hash taken before the plant.

- Block-end workspace run (condition 7): .gate/u4f/u4g-early-b1-ws.log, runner -k, exit 0 both: measured `cargo test --locked --workspace` 644 passed, 0 failed, 15 ignored (07:24:00-07:31:59 UTC); bites `--all-features` 744 passed, 0 failed, 15 ignored (to 07:41:20). Deltas: measured +2 (642 -> 644), bites +5 (739 -> 744): equal to the new tests.
- BLOCK 1 DONE. Added lines in spikes/: world.rs +101 (-14) by `git diff --stat 8f9d97d -- spikes/netcode-proof`, plus the new file sim/tests/predict.rs 356 = 457 (past the ~400 block line by ~57: TE-F1-5's twins and -55's proof are ~180 of it). (crates/client/src/predict.rs, 413 lines, is block 2's draft, unreferenced, not compiled yet: it rides block 1's snapshot.) Next: block 2.

## Findings (one line each)
- F1: -58's three command counters (`tick.v.cmd_refused_count`, `.cmd_lost_count`, `.cmd_doubled_count`) MOVED to the re-base, beside block 5's `wrap` suite (condition 3): the refused count's site is the refusal path room.rs:930-1019 or `Room::refusals()` (B2 ranges 930-938, 980-986, 1006-1019; names `refusals`, `Refusal`), and what it counts (forged kinds only, or late/outside-window drops too) rides C-U4F2-3's record form. Lost and doubled would fit (run path, run_one room.rs:1333-1376), but the rule builds the three early only if all fit.
