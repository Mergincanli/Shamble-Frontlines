# U4d round 2 review: rust-reviewer (read only)

Copy: /home/user/Shamble-Frontlines/.gate/u4d-s1 (detached at 21a8597, uncommitted). Round 1 state: .gate/u4d-prefix.
Round 1 notes: u4d-review-rust.md (M1 F-movers, M2 F-claimed, L1-L9, N1-N5).
(No Edit tool in this run: the file is rewritten whole with Write after each block.)

## Plan
1. Read rulings: advisor-u4-wave1.md W1-S1 (:45-111), W1-U4D (:113-304), W1-PUB (:353-436).
2. Read fixer's record (u4d-build.md "Fix pass, round 1") and s1-proto.md.
3. diff -ru u4d-prefix sim vs u4d-s1 sim; read the changed regions; check sha256 list.
4. git diff 21a8597 -- crates/proto (snapshot.rs, error.rs, tests).
5. Per round-1 finding: CLOSED / NOT CLOSED with file:line.
6. New checks: determinism, panics (debug_assert in record()), allocation (load_movers, decode cap), unsafe, clippy/clock bans, bites per instance, public items (A19) each needed now; proto Section/Groups/carries() private; new params.
7. Open points A/B/C weigh. Verdict.

## Progress
- Steps 1-2 done: rulings read (W1-S1, W1-U4D, W1-PUB), fixer's record (u4d-build.md:135-374), s1-proto.md.
- sha256 of crates/sim in u4d-s1 = fixer's list (history f11e9b59, lib 396e663d, replay 3b80b11b, scene c452d329, seam
  0c41b406, world 2f3cde3d, tests replay e483603f, seam 4900af4e, world 721c1c96; Cargo.toml fb98df48, clippy 4fd9facc).
- Read diffs: seam, lib, history, replay, scene, world (whole diff, 1391 lines).
- Checked OK so far: fire timer range [1-UPT, interval*30] holds by construction (world.rs:1370-1401: wait floored at
  1-UPT, +cost only when <=0); reload_left [0, 3e8] (:1372-1387); const assert seam.rs (FIRE/RELOAD fit i32); attribute
  changes clamped (:1116-1119); World::new asserts move speed, interval, reload, vehicles<=64 (:689-695) and cover
  (:775-779); `as i32` casts in own_state_record (:1780-1781) in range; Effect end `as i32` into TICK (i32 full range).
- Teleport worry (pre not refreshed at :1109) VOID: step refreshes every pre after apply_inputs+run_movers (:2088-2090).
- present_volumes indexes records[slot]: write_records pushes soldiers first (:1794-1796), phase 1 ran: no panic.
- load_movers_into: guards i<len, parent<vehicles (try_from); hulls[p[3] as usize] safe (p[3] from new or from a
  guarded record). compose_volumes: hulls array [None;64] on stack, try_from+get on parent: no panic.
- spawn_acc i64 (throws subtract 30,000): deterministic, `as u64` in hash two's complement; no overflow reachable.
- decode cap: (h.ticks as usize).min((len-60)/12) (replay.rs:320-323): bounded by bytes. L1 CLOSED.
- Replayer canonical check (replay.rs:698-702) before self.read += len: L2 CLOSED for the Replayer path.
- refusals Vec: InputReports::default() (world.rs:828), grows once then clear() keeps capacity: same class as L6.
