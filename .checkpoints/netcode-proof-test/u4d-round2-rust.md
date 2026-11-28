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
- Read diffs: seam, lib, history, replay, scene, world (whole diff, 1391 lines); tests replay, seam; tests/world.rs new
  tests h1 x2, n_m5, a_m1, r_m1, t_m1, d6 head, w1_s1.
- Checked OK: fire timer range [1-UPT, interval*30] holds by construction (world.rs:1370-1401: wait floored at 1-UPT,
  +cost only when <=0); reload_left [0, 3e8] (:1372-1387); const assert seam.rs (FIRE/RELOAD fit i32); attribute
  changes clamped (:1116-1119); World::new asserts move speed, interval, reload, vehicles<=64 (:689-695) and cover
  (:775-779); `as i32` casts in own_state_record (:1780-1781) in range; Effect end `as i32` into TICK (i32 full range).
- Teleport worry (pre not refreshed at :1109) VOID: step refreshes every pre after apply_inputs+run_movers (:2088-2090).
- present_volumes (:1475) indexes records[slot]: write_records pushes soldiers first (:1794-1796), phase 1 ran: no panic.
- load_movers_into (:2031-2058): guards i<len, parent<vehicles (try_from); hulls[p[3] as usize] safe (p[3] from new or
  a guarded record). compose_volumes (:483-545): hulls [None;64] on stack, try_from+get on parent; resizes `into` to the
  world's counts, never the records': no panic, no growth from data.
- spawn_acc i64 (throws subtract 30,000): deterministic, `as u64` in hash two's complement; no overflow reachable.
- decode cap: (h.ticks as usize).min((len-60)/12) (replay.rs:322-323). Replayer canonical check (replay.rs:698-702).
- refusals Vec: InputReports::default() (world.rs:828), grows once then clear() keeps capacity: same class as L6.
- unsafe/HashMap/clock grep: only the bites fn's HashMap (world.rs:551-580, allow+reason); unsafe_code forbid unchanged
  (Cargo.toml:93); lints/clippy.toml unchanged (sha = base). Bites still per instance (World.bite, Quant::with_bite).
- Determinism of new float code: part_center (:446-458) libm cosf/sinf + f32 mul/add (Rust never contracts to FMA),
  same function on authority and client; ES aim `(est*100.0) as i32` saturating, D-7 pins SS/TC recordings unchanged.
- proto: Section, Groups, carries() private (snapshot.rs:163-179); every put/get call site passes its table and section
  (:545-548, :591-608, :682-707); decoder refuses right after the kind (Malformed), encoder before writing (OutOfRange).
  Quant::value (quant.rs:197-216) refuses a decoded integer outside its field, so a U4g reset from decoded OwnState
  records keeps record()'s debug_assert true.
- Public items: OwnState (W1-S1), hit_volumes_from (A-M1, ruled "now"), Refusal/RefusalKind/refusals (N-M2, ruled now,
  D-10 reads them): needed now. Weapon/Scene::weapon/Scene::health: TUNE-1's header condition; only caller is D-6
  (tests/world.rs:618-628); no header writer in the tree yet: Low (list in §4.1, name the writer).
- (C) SOLDIER: catalogue :137 remote row = x, y, z, yaw, pitch, health, flags, weapon id = 8; "(9 fields)" is a slip.

## Round-1 findings (mine)
- M1 F-movers: CLOSED (Movers sized at new :782-795; load_movers_into :2031-2058 no resize; tests world.rs:2280-2343,
  tests/world.rs:944-1002; plants Q10a/Q10a'/Q10b).
- M2 F-claimed: CLOSED by ruling (W1-U4D N-M2): refusals now (seam.rs Refusal/RefusalKind, world.rs:1424-1431,
  cleared :2080); claimed restore carried to U4g (A-M2).
- L1 decode pre-alloc: CLOSED (replay.rs:320-323).
- L2 non-canonical: CLOSED for the Replayer (replay.rs:698-702; test tests/replay.rs:608-637). decode still accepts
  (documented).
- L3 damage index: NOT CLOSED, not claimed (follow-up, listed).
- L4 clamp z: CLOSED by ruling TUNE-2 (doc world.rs:48-54, World doc; test :2346; D-8 half at the bound).
- L5 carry 13: CLOSED by ruling C13 (counting half to U5).
- L6 events/fired capacity: NOT CLOSED, not claimed; refusals Vec joins it.
- L7 drain/dump perf: NOT CLOSED, not claimed.
- L8 finite scope: CLOSED (non_finite reads pre, :2005-2023; movers/volumes are integer-derived).
- L9 write_records twice: NOT CLOSED, not claimed; phase 3 now also rewrites 48 soldier records (:1475-1480).
- N1, N2, N3, N5: NOT CLOSED, not claimed (follow-ups listed in u4d-build.md:367-372).
- N4 record range: CLOSED (world.rs:427-436; plant Q18).

## New findings
- R2-L1 (Low, perf): part_center recomputes its hull's cosf/sinf for each part, 3 passes a tick (phase 1 movers,
  phase 3 and 8 volumes): ~372 trig pairs where 15 would do (world.rs:446-458, :2054-2057, :536-544). Bit-identical if
  cached per hull. Follow-up.
- R2-L2 (Low, perf): predict scans every record (~900) for its slot's two (world.rs:1001-1012); the order is fixed
  (soldiers at [slot], own states at [len-slots+slot]). Follow-up.
- R2-L3 (Low, perf): carries() is a linear scan of the table per record per section (snapshot.rs:175-179); a const
  per-section kind mask is O(1). Matters at U4e/U5 encode benches. Follow-up.
- R2-L4 (Low, A19): Weapon, Scene::weapon, Scene::health not in W1-PUB's list; their only caller is D-6; name the
  run-header writer (U4r or U4d2) and list them in §4.1's npt-sim row.
- R2-N1 (nit): World::new asserts the scene's move speed, interval, reload but not health <= HEALTH.max (1,023) nor
  ammo (u16 always fits); record()'s debug_assert catches it in debug only (scene.rs:167, :223 are in range).

## Open points weighed
- (A) aim_one_in 11: new stand-in tuning in a recording generator knob (pub(crate), ES only, draws only when > 1 so
  other scenes' streams are unchanged: D-7's SS and TC digests unchanged): needs the advisor's nod like TUNE-1, no Rust
  objection. grenade_fuse_ms 1,515: ADR 0001:1478's ES fuse applied to grenades too (ticks_for_ms rounds up: 46 ticks,
  1.533 s); traceable, no objection.
- (B) ES drift: deterministic generator property, no Rust defect. The band holds only for the 9,000-tick recording;
  a carry should say any longer or windowed ES run re-measures (or the generator's reckoning is fixed at the next
  GENERATOR bump). Netcode/test to rule; Low for me.
- (C) "(9 fields)" is a counting slip: 8 is right. Weapon/Scene::weapon/Scene::health: R2-L4. Carry 13: C13 ruling.

## Verdict
APPROVE WITH FINDINGS. No Critical or High; nothing opens round 3. New Lows R2-L1..L4 and R2-N1 are follow-ups, as are
my unclaimed round-1 Lows (L3, L6, L7, L9, N1, N2, N3, N5). Not run by me (read only): builds, tests, clippy, plants;
the numbers cited are the fixer's (u4d-build.md F8-F10) and s1-proto.md's.
DONE
