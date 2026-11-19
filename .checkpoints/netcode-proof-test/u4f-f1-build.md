# U4f-F1 build notes (the builder's; ADV-U4F20-1; plan rev 1.1 + advisor-u4f-f1-plan.md conditions 1-9)

## Step 0 (2026-11-19)
- `git merge --ff-only cb7e3745e14f5960a7390a9c83499a1c93c99738`: fast-forward from f5030a3, done.
- `git log -1 --format=%H` -> cb7e3745e14f5960a7390a9c83499a1c93c99738
- `git merge-base --is-ancestor 25d316e HEAD` -> exit 0
- A container restart at 23:18 UTC cut the first run before any note or edit (the coordinator's check at 23:21:
  worktree at cb7e374, status empty, no log). Resumed here.

## Plan (blocks in order, each compiling with its tests passing before the next)
- **B1:** `Command::edges_taken(last: u8, now: u8) -> u8` in sim/src/seam.rs, in `impl Command` beside the press
  indices (integer-only; doc cites ADR 0001:226, :234-237; returns `now.wrapping_sub(last)`). `run_command`
  (world.rs:1335-) takes `edges[k]` from it; the clock once (as now); reload: a loop of `edges[RELOAD]` passes of
  today's check; fire: the first edge (or the held trigger) is today's code with `edge = edges[FIRE] > 0`, then each
  further fire edge is refused by the kind test (Ammunition if `s.ammo == 0`, else Cooldown), authority only; throw: a
  loop of `edges[THROW]` `throw` calls (authority only); melee: taken, no effect. `run_one` (room.rs:1330-): advance
  = `wrapping_sub`, taken = `edges_taken`; not repeated: seen += advance, lost += advance - taken; repeated: doubled
  += taken. Docs: world.rs:30-38, :1340-1341; room.rs:352-356, :1326-1329. Then `cargo test -p npt-sim`,
  `cargo test -p npt-room`.
- **B2:** T1 in room/tests/commands.rs (option B, two late drops, a 2-edge fire command: seen 2, lost 0, doubled 0;
  the sim's Cooldown refusal of the second edge with that command's seq). Plant: `edges_taken` -> `(now -
  last).min(1)`; T1 fails by name; restored by Edit, sha256 recorded.
- **B3:** T2 in sim/tests/world.rs (d10 twin form, cases (a)-(f)); plants: the old rule (count half), a per-edge
  clock (twin half).
- **B4:** check.mjs twice; counts as deltas on 634 measured and 731 bites.

## Findings and records (a line each, as they come)
- F-read-1: the recordings manifest (crates/lab/recordings.json) pins only three synthetic recordings' bytes (open-loop
  input, no sim in the generator) and std-sin-scan.txt pins catches at ticks 2 and 1000002 (vehicle[8].pos.y, before
  any press): neither reads a state the fix could move. No runner-kind entry is in the manifest.
- F-read-2: the only lab test reading the press series (scenario.rs:3699-3700, f28_20) prints `lost` and asserts only
  `presses > 0`; lab/src/logs.rs:617 writes the series. bots/tests/behaviour.rs:177 (b3) is the client's own one-edge
  rule, untouched.
- F-read-3: d4 (sim/tests/world.rs:394-482) now takes 255 edges per button on each slot's first command (the stress
  scene's throws take up to the free pool, `spawn_acc` goes negative by 30,000 each: i64, the spawner waits; no
  assert reads it). Its asserts are grid ranges only: passes (B1 log).

## B1 (done): the rule, the sim loop, the room's counts, the four docs
- seam.rs: `Command::edges_taken(last: u8, now: u8) -> u8` (`pub const fn`, `now.wrapping_sub(last)`), in `impl
  Command` after PRESS_MELEE; doc cites ADR 0001:226 and :234-237.
- world.rs `run_command`: `edges[k] = Command::edges_taken(claimed, now)`, `edge = edges.map(|n| n > 0)`; the clock
  once (unchanged); reload: `for _ in 0..edges[RELOAD]` around today's check; fire: the first edge is today's code
  (bites included, untouched), then `for _ in 1..edges[FIRE]` (authority only) refused by the kind test (Ammunition
  if `s.ammo == 0` after the shot, else Cooldown), placed after the shot and before the flags; throw: `for _ in
  0..edges[THROW]` (authority only) calls `throw` (throws or PoolFull); melee: taken into `claimed`, no effect.
- room.rs `run_one`: advance = `wrapping_sub`, taken = `Command::edges_taken`; not repeated: seen += advance, lost +=
  advance - taken (saturating); repeated: doubled += taken.
- Docs (condition 8): world.rs module doc (the refused-input bullet), the comment at the press edges, room.rs
  `presses()` doc and `run_one` doc.
- Runs: f1-b1-sim.log `cargo test -p npt-sim` exit 0 (50 passed, 1 ignored: 6+6+8+30+0); f1-b1-room.log `cargo test -p
  npt-room` exit 0 (70 passed: 24+0+1+26+19+0); f1-b1-bites.log the same two with `--features bites`, exit 0 (sim
  58 passed + 1 ignored; room 91 passed).
- **What a bite does on an extra edge** (condition 1): every bite acts on the first edge only, as before; the extra
  edges follow the rule. PM-0001-21/no-cooldown: the first fire edge fires in the cooldown (its bite), a further fire
  edge is still refused (one shot per command is not the cooldown check), counted Cooldown (Ammunition if that shot
  emptied the magazine). PM-0001-21/refuse-not-clamp: the first edge's far point is refused as Cooldown when ready
  (its bite), and a further fire edge is refused and counted by the kind test as well (the further-edge loop is not
  gated by `refuse_far`). PM-0001-41's two (speed, quant) and PM-0004-21's two (hash-map, std-sin) do not read edges.
  The room's PM-0001-21/no-press-edge (kind 5 off): an advance beyond the ticks covered is accepted, and the sim now
  takes all of it (fire: one shot and the rest refused; reload: one start and the rest refused; throw: up to the free
  pool, the rest PoolFull); its unit proof (commands.rs:1017-1023, 2 fire edges) asserts only that seq 2 ran: passes.
- **A session's first command** (condition 7): kind 5 runs only against an accepted command (input.rs:287), and the
  sim's `claimed` and the room's `run_presses` outlive a session, so a session's first command carries an unchecked
  advance up to 255 per button. F1's behaviour: the sim takes it all (fire and reload: one shot or start, the rest
  refused, state-neutral; throw: one grenade per free pool slot, the rest PoolFull, so up to a full pool in one
  tick); the room counts seen = the advance, lost 0. Kind 5 is unchanged in F1; the item goes to U4f2's brief (-21).
- SIZE-CHECK after B1: `git diff --stat cb7e374 -- spikes/` 59 insertions, 24 deletions, 3 files; new files under
  spikes/ none. 59 of 300.

## B2 (done): T1
- room/tests/commands.rs, after f17: `t1_under_option_b_two_late_drops_then_a_command_two_edges_on_lose_no_press`.
  Option B room; cmd 1 at j+1 runs, B repeats it at j+2 and j+3 (asserted [1, 1, 1]); then one Input with cmd 2
  (j+2, fire counter 1), cmd 3 (j+3, 2) and cmd 4 (j+4, 2): 2 and 3 late (counted 2), 4 accepted (kind 5: covers 3
  from the newest accepted j+1) and run alone; asserts `presses()` = (2, 0, 0) and the tick's sim refusals exactly
  [Refusal { slot 0, seq 4, Cooldown }].
- With it: f1-b2-t1.log `cargo test -p npt-room --test commands` exit 0, 27 passed.
- Plant (seam.rs `edges_taken` only: `let advance = now.wrapping_sub(last); if advance < 1 { advance } else { 1 }`,
  i.e. min(advance, 1); `min` is not const): f1-b2-plant.log exit 101, T1 FAILED by name at commands.rs:636 "seen,
  lost, doubled": left Some((2, 1, 0)), right Some((2, 0, 0)) (-20's defect exactly); 26 others passed.
- Restored by Edit: seam.rs sha256 63057544bdc8eea58601aa0a2febef3af2ae3abb420d9d1adb109aa61be16499, equal to before
  the plant. f1-b2-restored.log `cargo test -p npt-room` exit 0 (71 passed: 24+0+1+27+19+0).
- SIZE-CHECK after B2: 92 insertions, 24 deletions, 4 files; no new file under spikes/. 92 of 300.

## B3 (in progress): T2
- sim/tests/world.rs after d10_a_rewind_outside_the_ring_leaves_no_trace: helper `two_edges(n, held, press, (k, at))`
  (twin = `course_stream(n, held, press)`, forged = the same with `(k, at)` named again; `twins` asserts equal events,
  state hashes and dumps every tick; returns both reports and the forged shots), and six tests:
  - `t2_a_two_fire_edges_with_fire_held_and_the_weapon_ready_fire_once`: held from 10, fire x2 at 10; shots start
    [10, 13]; cooldown (1, 0); forged [10 Cooldown], twin [].
  - `t2_b_two_fire_edges_with_the_weapon_ready_fire_once`: fire x2 at 10, x1 at 20; shots [10, 20]; cooldown (1, 0);
    forged [10 Cooldown], twin [].
  - `t2_c_two_fire_edges_in_the_cooldown_are_both_refused`: fire at 10, x2 at 11, at 20; shots [10, 20]; cooldown
    (2, 1); forged [11 Cooldown, 11 Cooldown], twin [11 Cooldown].
  - `t2_d_two_fire_edges_on_the_last_round_fire_it_and_refuse_as_ammunition`: held 0-84 (29 rounds), fire x2 at 100,
    reload 105, fire 170; 31 shots ending [84, 100, 170]; ammunition (1, 0); forged [100 Ammunition], twin [].
  - `t2_e_two_reload_edges_start_one_reload_and_refuse_the_second`: held 0-29 (10 rounds), reload x2 at 50, fire 120;
    last shot 120; reload (1, 0); forged [50 Reload], twin [].
  - `t2_f_each_throw_edge_throws_or_meets_a_full_pool` (apart, helper `live_seqs` from the dump's projectile live/seq
    words): throw x2 at 10 -> live seqs [10, 10], no refusal; throws at 0-14 then x2 at 15 -> live seqs 0..=15,
    refusals [15 PoolFull], pool_full 1.
- f1-b3-t2.log `cargo test -p npt-sim --test world t2_` exit 0, 6 passed. f1-b3-fmt.log `cargo fmt --check -p npt-sim
  -p npt-room` exit 0 (no diff). f1-b3-clippy.log clippy -p npt-sim -p npt-room --all-targets, measured and
  --all-features, -D warnings: both exit 0.
- SIZE-CHECK at T2 written: 205 insertions, 24 deletions, 5 files; no new file under spikes/. 205 of 300.
- Plant 1, the old rule (seam.rs:78-80 `edges_taken` only, min(advance, 1) as in B2): f1-b3-plant1.log exit 101, all
  six FAILED by name at the count half (the twin half passed first in a-e): a (0, 0) vs (1, 0) at world.rs:1831; b
  (0, 0) vs (1, 0) at :1840; c (1, 1) vs (2, 1) at :1849; d (0, 0) vs (1, 0) at :1862; e (0, 0) vs (1, 0) at
  :1874; f live seqs [10] vs [10, 10] at :1916. Restored by Edit: seam.rs sha256 63057544...16499, as before.
- Plant 2, a per-edge clock (world.rs:1490, in the further-fire-edge loop: `s.wait = (s.wait -
  UNITS_PER_TICK).max(1 - UNITS_PER_TICK);` per edge): f1-b3-plant2.log exit 101, a, b, c, d FAILED by name in the
  twin half at world.rs:573 (`twins`): a and b "step 10: the state hashes", c "step 11", d "step 100"; e and f pass
  (no fire edge beyond the first). Restored by Edit: world.rs sha256
  4a4e138ca208acb3c37e978d3693f18701667e428d24d479961656f06ea7c041, equal to before the plant.
- B3 done. SIZE-CHECK after B3: 205 insertions, 24 deletions, 5 files; no new file under spikes/. 205 of 300.

## B4: the end checks (check.mjs twice)
- Run 1 started (background, -x): log .gate/u4f/f1-b4-check1.log. Expected test-step counts: 634 + 7 = 641 measured,
  731 + 7 = 738 bites (T1 and T2's six, none feature-gated).
