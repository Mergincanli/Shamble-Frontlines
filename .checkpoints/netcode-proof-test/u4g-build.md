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

- Snapshot after block 1: wip/u4g-early.patch 45,130 B, 3 files, sha256 1891ff494f52298057ddf9a4d48ccc87825cd5aaf530a41819bf28d201f2e890.

## Block 2 (npt-client Predictor) — split 2a (Predictor + its tests) / 2b (endpoint wiring + its tests); STOPPED ON DISK
- 2a written, NOT compiled: crates/client/src/predict.rs (new, 413 lines: Predictor, Applied, Correction, CorrectionKind, OwnChange, consts SNAP_UM, TRACE_UM, FLAG_UM, FADE_MS, HELD_MS); lib.rs +6 (`mod predict;` and its `pub use`, at :23-30, outside B2's client ranges 357-364 and 614-622); crates/client/tests/predict.rs (new: Lab harness + 6 tests: the_fade_and_the_history_follow_the_rate, p41_every_command_in_time..., p41_a_late_command..., p60_each_own_change..., p60_an_own_state_section_without..., fade_snap_and_clear_and_the_5_cm_flag).
- STOP (condition 6): the first 2a build call (runner, .gate/u4f/u4g-early-b2a-client1.log) exited 3 before any command: "STOP: under 4 GB free" (free 3,618 MiB at 07:43:31 UTC). `df -B1` then 3,794,055,168 B available, under the 5.5 GB U4g build line. Cause: block 1's two whole-workspace test builds at this new worktree path: the shared target is now 5,992,017,250 B (`du -sb`), about 3.81 GB above the 2,182,039,788 B the advisor measured (advisor-u4g-early.md:40), against B3's 0.5-1.5 GB estimate. No build ran after the stop; no plant in place; snapshot taken; returned.
- On resume (only in a condition-5 window, at >= 6.5 GB free, after a part-clean P27 by the main session): compile and run client/tests/predict.rs in both builds (`cargo test --locked -p npt-client --test predict` and `--all-features`), fix what fails, then plants: "every correction taken as traced" (predict.rs `correct`'s `traced` -> true), "the fade off" (`self.fade_left = self.fade_ticks` -> 0), "the clear skipped" (drop the else branch), "the threshold moved" (FLAG_UM 50_000 -> 30_000); then the 2a block-end workspace run; then 2b: endpoint wiring (field `predictor`, `with_predictor`, `predictor()`, `close_prediction`; JoinAccepted -> set_slot; `stamp` -> `command`; Tick gains `last_run`; `group` and `full_state` -> `received` + `snapshot` with the hashed own/entity lists; `step` -> `advance` + StepOut fields `applied`, `own_changes`; a `widened` helper using `self.wrap` so lt-tick-offset moves the replay start) and its tests at the file's end (`mod endpoint`: FullStates from an authority, lead 2: no correction; lt-tick-offset across the wrap with `with_starts(65_530, 0)`: clean none, bitten an untraced one). Then blocks 3 and 4 as planned below.
- Plan kept for block 3 (room): ClientCounters `own_cosmetic_sent` (cosmetic loop room.rs:1586-1594), -60's bite `PM-0001-60/own-state-missing` (BITES 16 -> 17 at room lib.rs:39 and the list's end; RoomBite variant after NotOlderThan; field `pre_change` after `scripted_attributes` :494; captured in `tick()` after `fill_input` from the previous tick's OwnState records; written in `encode_all` after the own list is built, :1303-1315); room/tests/bites.rs pin updated; new file room/tests/prediction.rs. Block 4 (client CueLog): cues.rs (CueKind, OwnCue, CueLog keyed by (slot, weapon, command index), closed at last_run), the bite `PM-0001-62/cues-from-replay` (client BITES 6 -> 7, client/tests/bites.rs pin), Predictor hooks in `command` and the replay loop, StepOut `cues`.

- Snapshot at the stop: wip/u4g-early.patch 61,743 B, 5 files, sha256 a842de672bd6a254c5a184825f3fde6427d7918793fd97d834e0d763e450251e. `df -B1` at the stop: 3,793,887,232 B. Added lines in spikes/ so far: block 1 457 (world.rs +101, sim/tests/predict.rs 356); block 2a uncompiled 776 (client predict.rs 413, lib.rs +6, client/tests/predict.rs 357). Clippy and fmt not run (the runner refuses any command under 4 GB; condition 6 forbids build calls under 5.5 GB).

## Resume 1 (coordinator: P27 dev part-clean ~07:50 UTC, 5.4 GiB; snapshot a842de67… checked; U4G-DISK (a))
- Worktree as left (`git status`: the same 6 entries). df before the 2a compile call: 9,372,377,088 B.

- 2a compile call 1 (.gate/u4f/u4g-early-b2a-client2.log, exit 101 both builds): 5 of 6 pass; p60_each_own_change failed: the MoveSpeed change counted the fire-interval change's first correction (corrections 2, misplaced 1). Fix (finding F3): a snapshot that opens a change gives its correction to that change only (`own_changes`). df after: 9,184,997,376 B.
- 2a compile call 2 (u4g-early-b2a-client3.log): 6 passed in each build, exit 0. df after: 9,132,306,432 B. Clean predict.rs sha256 524819710a45df235affe6200470bc5c328f6e7ce9531ff0a90aa14efb40d71f.
- Plants, one at a time, each runner -x -k, measured build, restored by Edit and the hash checked equal to 52481971… after each:
  - "every correction taken as traced" (`let traced = true || …`): FAILED by name fade_snap_and_clear_and_the_5_cm_flag (predict.rs:335) and p60_each_own_change (:292); log u4g-early-b2a-plant-traced.log.
  - "the fade off" (`self.fade_left = 0` at a fade): FAILED fade_snap_and_clear… (:338); log u4g-early-b2a-plant-fade.log.
  - "the clear skipped" (the snap/respawn branch removed): FAILED fade_snap_and_clear… (:350, the snap's clear); log u4g-early-b2a-plant-clear.log.
  - "the threshold moved" (FLAG_UM 50_000 -> 30_000): FAILED fade_snap_and_clear… (:345, the 4 cm case); log u4g-early-b2a-plant-flag.log.

- fmt: u4g-early-b2a-fmt.log exit 1 (rustfmt's line breaks in client predict.rs, client/tests/predict.rs, sim world.rs, sim/tests/predict.rs): the diff applied by Edit; u4g-early-b2a-fmt2.log exit 0. df after: 9,133,453,312 B.
- Pre-lint (not the stop's): `cargo clippy --locked -p npt-sim -p npt-client --all-targets -- -D warnings`, both builds, exit 0 (u4g-early-b2a-clippy-pre.log). df after: 9,056,931,840 B. Then the 2a block-end workspace run (df before it: 9,056,931,840 B).

- 2a block-end workspace run (condition 7): .gate/u4f/u4g-early-b2a-ws.log, runner -k, exit 0 both: measured 650 passed, 0 failed, 15 ignored (to 08:01:20 UTC); bites 750 passed, 0 failed, 15 ignored (to 08:10:43). Deltas on 642/739: +8 measured (sim 2, client 6), +11 bites (sim 5, client 6). df after: 4,524,601,344 B (the run took about 4.53 GB).
- BLOCK 2a DONE. Added lines in spikes/ now: tracked +109 (world.rs +104, lib.rs +5 by `git diff --stat 8f9d97d`), new files 1,227 (client predict.rs 431, client/tests/predict.rs 418, sim/tests/predict.rs 378): 1,336 in all; block 1 482 (after fmt), block 2a 854 (the Predictor alone 431: past ~400).
- STOP (condition 6): 4,524,601,344 B free, under 5.5 GB: no build call; snapshot; return. Next cycle (after a part-clean): 2b as planned under "On resume" (endpoint wiring + its tests at client/tests/predict.rs's end), then blocks 3 and 4.

- Snapshot after block 2a: wip/u4g-early.patch 63,277 B, 5 files, sha256 2ec04263c31fcd169557ff2e0e77290c213b97c9661ff6cbe8313de44d070ba5. df at the stop: 4,524,601,344 B.

## Findings (one line each)
- F3: an own-state change's first correction was also counted, as misplaced, in an older change still open (the move-speed change counted the fire-interval change's); fixed in `own_changes` (the snapshot that opens a change gives its correction to it alone). A judged -60 run with two kinds open at once would have failed "at most one correction per change" on a correct build.
- F2 (disk): one whole-workspace test pass in both builds at a fresh worktree path costs about 3.8 GB of the shared target (07:24-07:41 UTC: 7,565,500,416 B free before, 3,794,055,168 B after), so condition 6's 6.5 GB launch line leaves room for one block end, not four. Each remaining block end needs a part-clean (P27, the main session's, no builder running) or a decision to run block ends on the touched crates only (condition 7 says workspace-wide: not mine to change).
- F1: -58's three command counters (`tick.v.cmd_refused_count`, `.cmd_lost_count`, `.cmd_doubled_count`) MOVED to the re-base, beside block 5's `wrap` suite (condition 3): the refused count's site is the refusal path room.rs:930-1019 or `Room::refusals()` (B2 ranges 930-938, 980-986, 1006-1019; names `refusals`, `Refusal`), and what it counts (forged kinds only, or late/outside-window drops too) rides C-U4F2-3's record form. Lost and doubled would fit (run path, run_one room.rs:1333-1376), but the rule builds the three early only if all fit.
