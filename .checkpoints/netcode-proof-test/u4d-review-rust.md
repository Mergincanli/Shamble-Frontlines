# U4d build review: rust-reviewer (read only)

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-1 (base 097f897, uncommitted).
Plan: u4-build-plan.md (sha 7ec1b324 per builder; plan now 0d69332c after F1) §5 U4d, §4.1-4.3, §4.13-4.15; advisor-u4.md U4s ruling; u4s-carries.md (U4d entries).

## Plan
1. Read carries (U4d entries) and builder notes.
2. Read diffs: seam.rs, history.rs, lib.rs, grid.rs, tests/seam.rs.
3. Read new files: rng, hash, trajectory, greybox, scene, world, replay; tests world, replay.
4. Check: float allow-list (sim/clippy.toml) vs uses; libm only; NaN assert; HashMap/clock; fixed pools / no per-tick alloc; integer overflow (as casts, wrapping, sat); hand-written Clone of HitVolumes; wasm32 parity (usize, f32 ops, fma, mul_add); safety (panics on recording decode, unbounded alloc, unsafe).
5. Findings + verdict.

## Progress
- 07:08 UTC: steps 1-3 mostly done (carries, notes, seam/history/lib/grid diffs, rng, hash, greybox, scene, trajectory, world read). File sha256s match the builder's final list.
- OPS: an orphaned touch loop (pid 15338, started 06:38, the builder's step-8 command) still touches every file under the worktree's crates/ every 2 s. Content unchanged; it forces rebuilds. Report to main session; not killed (read only).
- Candidate findings so far:
  - F-claimed: World::claimed (last press counters seen) decides later ticks but is outside write_state/hash (doc of write_state says "every field that decides a later tick" then excepts it). Needed for D-10's twin; plan §4.2 says sim input state holds what the server accepted, never the client's claim. Also not restorable for U4g rollback. Severity: medium (design; netcode to rule) / low for Rust.
  - F-finite: D-4's finite check walks write_state only; pre (records' raw), movers, blend boxes not checked. Low.
  - F-stream-idx: rngs[Stream::X as usize - 1] relies on discriminant = STREAMS position + 1; no const assert. Nit.
  - F-fired-cap: fired Vec::with_capacity(256) can grow past 256 with catch-up (48x8 commands); one-time growth. Low.
- RESUME (after usage-limit cut): sha256s still match builder list; touch loop pid 15338 no longer running. Re-reading files to finish steps 3-5 (replay, tests, world detail).
- world.rs fully re-read (1-1878). New candidates:
  - F-movers: load_movers_into (world.rs:1760-1797) resizes parts_cm/parts/vehicles/doors_closed to any record index (u16) though its doc (:770-773) says a record the world has no entity for is skipped; extra parts_cm then flow into build_volumes' cover (:1233), breaking MAX_COVER (the 1 MB ring bound's premise) and shifting land()'s wall index (:1349). Client-side allocation sized by decoded data, bounded 64K x 36 B. Medium.
  - F-damage-idx: damage() indexes self.soldiers[v] (:1364) with a slot from the lent ring's boxes (blend :1268); a HitHistory from another scene (more boxes) panics; `i as u8` truncates past 255 (:829). Low (API contract).
  - F-clone-exhaustive: hand-written Clone (seam.rs) would silently skip a future field; destructure exhaustively. Nit.
  - F-events-cap: events with_capacity(512), fired 256: one-time growth past (48 slots x 8 catch-up = 384 commands). Low.
  - F-clamp-z: clamp idempotency fails when the reach point's z lies in (40.95, origin z] and saturates to 40.95 (soldier teleported near z cap): distance grows up to 1.4 m, so a sent-back point is clamped again. Builder's D-8 note skips it. Low.
  - checked OK: integer widening (move_xy i32, aim f64), wait/reload i64, saturating move_speed, fire_interval max(1), rng between/below safe for all call sites (constants), no float min/max/mul_add, libm only (sinf/cosf/atan2f), HashMap only in bites fn with allow+reason, blend_key reset per step (:1820) and on death (:1390), hash = dump walk (write_state), no usize hashed.
- replay.rs read (1-695). Candidates:
  - F-decode-cap: Recording::decode pre-allocates Vec::with_capacity(min(header ticks, 2^20)) TickInputs (replay.rs:320) before any tick bytes exist: a 60-byte file asks ~109 MB (64-bit, 104 B each) / ~54 MB (wasm32). Cap by (len - 60) / 12. Low (recordings SHA-checked, local).
  - F-replay-canonical: Replayer/decode never check the canonical order; a non-canonical recording panics World::step's debug_assert (world.rs:1801) in debug builds (D-2 is debug) instead of Step::Failed. Low.
  - F-drain: Replayer::step drains bytes every 64 KiB consumed (replay.rs:631-634); feeding a whole 16.8 MB recording first makes it quadratic (~256 memmoves of ~8 MB avg). Check how tests/U4d2 drive it. Low/perf.
  - F-trailing: decode refuses trailing non-padding bytes; Replayer ignores anything after the last tick. Nit.
- libm 0.2.16 checked in registry source: sinf/cosf/atan2f pure Rust, no select_implementation; arch dispatch (src/math/arch/mod.rs:10-40) covers only wasm32 ceil/floor/fabs/rint/sqrt/trunc (needs intrinsics_enabled) and x86 sqrt/fma: all exact ops. So libm parity holds by construction (U4d2 proves it). No mul_add, no unsafe, HashMap only at the bites fn.
- NaN audit: every division guarded (len2>127^2, dist>reach, flat>0, dist>=0.01, d[a]!=0, period>0); sqrt args >= 0; atan2f(0,0)=0; u32::MAX move speed stays finite (4.3e7 m/s). No NaN reachable from integer input.
- Tests read: replay.rs (D-2 child protocol, D-7 pins, round trip, refusals), world.rs D-4 x3, carry 13/14/15, bites module. D-2 children feed whole recording then step -> F-drain applies (16.7 MB fed first).
- perf: write_records runs twice a tick (phase 1 for movers, phase 8), ~878 records each incl. 300 trajectory_at; phase 1 needs only parts/doors/wall. Low.
- More candidates: F-claimed upgraded to medium (predict updates claimed too, world.rs:1040; U4g's replay after a correction sees false press edges unless it restores claimed, and World has no API for it). N: record() writes Val::I unchecked vs field range (world.rs:396; SS health 1,000 vs HEALTH max 1,023). N: clamp runs for every command, firing or not (world.rs:1095) though §4.2 says "inside the shot phase" (netcode). Test perf: D-2 child builds a full dump for every parent word incl. "n" (tests/replay.rs:103-104).
- Main folder: git status shows only notes files (no stray source from the builder's "Not counted" slip).

## Final findings (ranked)
M1 F-movers (world.rs:770-773 doc vs :1760-1797 code; :1233-1239; :1349). M2 F-claimed (world.rs:480-481, :1036-1040, :1645-1647; plan §4.2 last bullet).
L1 decode pre-alloc (replay.rs:320). L2 non-canonical recording debug panic (world.rs:1801-1805; replay.rs:682-690). L3 damage index (world.rs:1364, :829). L4 clamp z (world.rs:1009-1015). L5 carry 13 partly met (tests/world.rs:1290-1340; Cargo.toml:93). L6 events/fired capacity (world.rs:629, :638). L7 drain + dump-per-word perf (replay.rs:631-634; tests/replay.rs:88-104). L8 finite check scope (world.rs:1751-1756). L9 write_records twice (world.rs:910, :1845).
N1 Clone destructure (seam.rs Clone impl). N2 stream index const assert (rng.rs:19-37; world.rs:1305,1437,1466,1492). N3 trailing bytes (replay.rs:327 vs :667). N4 record range debug_assert (world.rs:396). N5 clamp on non-firing commands (world.rs:1095).
Verdict: APPROVE WITH FINDINGS (fix M1 in U4d; M2 to netcode-architect/advisor).
DONE
