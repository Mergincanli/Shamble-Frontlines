# U4d round 2 (combined with W1-S1's npt-proto half): test-engineer

Copy: /home/user/Shamble-Frontlines/.gate/u4d-s1 (detached at 21a8597; nothing committed). Round 1 (builder's state):
.gate/u4d-prefix. Fix record: u4d-build.md "Fix pass, round 1"; proto half: s1-proto.md. Rulings: advisor-u4-wave1.md
W1-S1 :45-111, W1-U4D :113-304, W1-PUB :353-436. Round 1 mine: u4d-review-test.md (M1-M5, L1-L6, N1-N3).
Other round 2 notes (read-only): u4d-round2-{netcode,security,rust,arch}.md. Memory: none (no agent-memory folder).
Rules: CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target, CARGO_NET_OFFLINE=true; `cd <copy> || exit 1`
alone on its own line in a subshell; never background a chain with cd; idle wait (pgrep cargo/rustc) + touch all crates
files + 2 s toucher inside the same command for counted runs; provenance pre-run (Compiling npt-sim / npt-proto with
this copy's path); df > 2 GB; plants one at a time, restore, sha256 check; commit nothing.
Expected counts: measured 351, bites 388, self 52 of 52.
DEVIATION (stated): this run has no Edit tool (the harness refused it: "No such tool available: Edit"), as in round 1.
So: notes are rewritten whole with Write; plants are applied by a scratch script (scratchpad r2t-plant.py) that
replaces exactly one unique match per site (asserts count == 1), saves a byte copy first, restores from it and checks
sha256 against r2t-ref.sha256 after each plant. Same method as round 1's u4d-test-bites.py. Runner: scratchpad
r2t-run.sh (idle 8 s, df, touch, 2 s toucher inside the script, output to stdout only). Plants Q1, Q10a2, Q12 and X3
were added to the scratch script by small python edits of that scratchpad file (outside the repo).
MY SLIP: block [4] below was first appended to this file by a shell heredoc (`cat >> ... <<EOF`), not by Write; notes
only, no other file; this whole file was then rewritten with Write.

## Plan
1. sha256 of every file in crates/sim and crates/proto; compare with u4d-build.md's list and s1-ref.sha256.
2. Read the diff (prefix vs copy for sim; 21a8597 vs copy for proto) and the other round 2 notes.
3. Round-1 findings M1-M5, L1-L6, N1-N3: CLOSED / NOT CLOSED with proof (test + plant).
4. Re-plant (one at a time, named test fails, restore, sha256): Q2, Q6, Q11a, Q13, Q19; P1, P3, P7; plus my own
   plants where a test's power is in doubt (O1 hull-part rotation sign, O2 rotation ignored, O3 phase 3 stale soldier
   records, O4 decode cap undone, O5 carry 13's derived clone_from); probes X1 (H1 count over 54,000 SS ticks), X2 (ES
   bands over 18,000 ticks = 10 min).
5. Carries 9, 12, 13, 14, 36 against code and tests.
6. The fixer's open points A (ES knobs), B (ES drift), C (SOLDIER 8 vs 9; new public items).
7. Settle by test/plant any other reviewer's finding a run can settle.
8. check.mjs once at the end (fresh touch, idle, 2 s toucher): every step's counts.
9. Final sha256 = step 1's.

## Progress
- [1] sha256 of all 34 files under crates/sim (17) and crates/proto (17): scratchpad r2t-ref.sha256. crates/sim: 19 of
  20 f-ref.sha256 entries OK (u4d-build.md :305-315's list plus the unchanged files and workspace files); crates/sim
  also `diff -r` identical to U4d's worktree wf_f54246ff-e0d-1. The one mismatch is check.mjs (f-ref 6d1ed188, here
  7eddc285), which equals 21a8597's (U4a's merged check.mjs; `git diff 21a8597` clean): expected. crates/proto: all 7
  s1-ref.sha256 entries OK (snapshot 48ca1183, error f52dbb94, common b23b687b, wire 69ef15ec, bites 96d07cc9, sim
  seam 0c41b406, quant 0d41e906). Workspace Cargo.toml 75b6c160 and Cargo.lock 3da1e15f = 21a8597. git status: 10 M,
  9 ??. df 11 GB free.
- [2] Read: sim diff prefix -> copy (scratchpad r2t-sim.diff, 3,103 lines: history, lib, replay, scene, seam, world,
  tests replay/seam/world), proto src + tests diff vs 21a8597 (r2t-proto-src.diff, r2t-proto-tests.diff), the four
  other round-2 notes (netcode, security, rust, arch: all APPROVE WITH FINDINGS, no Critical/High). Doubts to settle
  by plant: (a) hull o part composition has no semantic test (h1 checks "box moved", a_m1 compares the function with
  itself): O1, O2; (b) phase 3's present volumes now come from refreshed soldier records: which test sees a stale
  refresh? O3; (c) rust L1 = my L5 (decode cap) has no test: O4; (d) h1's "changes only when health does" is measured
  over 600 ticks only, and its comparison includes the record's area (not on the wire: proto between() reads grid
  only, snapshot.rs:86-115): X1 over an S30; (e) fixer's point B: X2 over ES's 10-minute run length.
- [3] Provenance pre-run 11:36:36-11:36:40 UTC: `cargo test --locked --no-run -p npt-sim -p npt-proto` printed
  "Compiling npt-sim (.gate/u4d-s1/.../crates/sim)" and "Compiling npt-proto (.gate/u4d-s1/.../crates/proto)"; idle,
  11 GB free.
- [4] Re-plants (r2t-plant.py, one at a time, the named test run alone from this copy, restored, `check` ALL-OK (34
  files) after each):
  - Q2 (predict leaves the own-state record stale): d3 FAILED at tests/world.rs:246 "own-state record" (client grid
    [0,0,0,30,0,...] vs authority [600,0,0,30,-999999,...]: velocity and fire timer stale). = fixer.
  - Q6 (part x = hull's world x + offset): both h1 tests FAILED (:779 "tick 1: the parts' records"; :837). = fixer.
  - Q11a (a throw takes no spawn): t_m1 FAILED at :1064, live 37,726 thousandths (spawns 24,660). = fixer.
  - Q13 (refusals not cleared per step): d10 cooldown, ammunition, reload, rewind FAILED (:1499, :1521, :1547,
    :1637); clamp passed (it has no refusals). = fixer.
  - Q19 (cap back to 2^20): the replayer test FAILED "a span above 1 MB: Done" (tests/replay.rs:577); carry14 FAILED
    "one ms past the edge" (:1964), run separately (cargo stops at the first failing binary). = fixer.
  - P1 (get_delta's check off): w1_s1 FAILED "Vehicle moved out of the entity section" (tests/wire.rs:297). = s1.
  - P3 (put_delta's check off): w1_s1 FAILED "Vehicle own true: Add(...)" (tests/wire.rs:225). = s1.
  - P7 (SOLDIER [Field; 11] + 3 VELOCITY): the GROUPS unit test FAILED at snapshot.rs:747, widths [16,16,12,16,10,8,
    10,4,16,16,16] vs the row's 8. = s1. (npt-sim, npt-link, npt-proto recompiled from this copy.)
  - Q12 (my round-1 E6: blend by / 255), 11:52:45: the blend unit test FAILED at world.rs:2173 "the soldier, fraction
    1, axis 0: [-127.396, -126.596] against the reference centre -127 ± 0.4" (3.9 mm); both D-9 tests still pass
    (the hit-or-miss check is the second test, as ruled). = fixer.
  - Q1 (my M4 = N-M3, P11's form: a fired flag bit 1 << 4 set on a shot, cleared per tick in step, per command in
    predict; 3 sites), 12:00:39: d3 FAILED at :245 "CO step 4: record" (flags 0 vs 16; step 4 = the first catch-up
    tick). = fixer.
  - Q10a2 (my M5, probe C's form: load_movers grows doors_closed to a decoded index), 12:01:01: the module test FAILED
    at world.rs:2322 "CO: no mover grew"; r_m1 passes (a grown door vector changes no volume or state). = fixer's Q10a'.
- [5] My own plants (whole npt-sim measured suite, --no-fail-fast; restored, ALL-OK after each):
  - O1 (part_center rotates by -yaw: signs of the s terms flipped, world.rs:452-453), 11:39:46-11:40:34: ONLY d7
    FAILED, at tests/replay.rs:351 (SS's state after 1/30/300 ticks pin). lib 3, seam 8, world 30 pass: h1 x2, a_m1,
    r_m1, D-1, D-3, D-6 all pass. D-2 (in replay) passes too (5 of 6 passed).
  - O2 (part_center ignores the hull's yaw, c = 1, s = 0), 11:41:01-11:41:50: ONLY d7 FAILED (:351), the rest pass.
  - O3 (phase 3 does not refresh the soldiers' records: the present volumes are the soldiers' boxes before this
    tick's moves, world.rs:1476-1478), 11:42:06-11:42:56: ONLY d7 FAILED (:351), the rest pass.
  - O4 (decode reserves by the header's count again, replay.rs:323), 11:43:27-11:44:18: NOTHING fails (47 pass).
  - O5 (carry 13, `*self = source.clone()`): carry13 FAILED "tick 2: soldiers' buffer" (:1927). = builder's P2.
  Reading: D-7's pins were re-pinned by this same fix pass (u4d-build.md F4) after the composition and phase-3 code
  were written, so a pin is no evidence that the frame convention or the phase-3 refresh is right, only that nothing
  changed since F4. By reading, both are right now: part_center (world.rs:446-458) is the old place_vehicles formula
  (x' = ox c - oy s, y' = ox s + oy c; the prefix's place_vehicles) on the hull record's grid yaw; present_volumes
  refreshes records[slot] from soldier_record (pre-snap raw -> grid = the snapped position's grid, carry 9), which is
  the old build_volumes' to_grid(s.pos) for the snapped s.pos.
- [6] Probes (a test's length raised, restored like a plant):
  - X1 h1 count over 54,000 SS ticks (an S30), 11:45:27-11:48:29: PASS. Records changed a tick: Soldier 47.73, Vehicle
    14.99, VehiclePart 5.62, Projectile 2.66, Door 0.03, Wall 0.40, Effect 7.40, OwnState 47.49 (126.3 a tick, was
    574 in round 1's probe K); parts hit 303,949 = part records changed 303,949 (no area-driven change in 30 min).
  - X2 t_m1 over 18,000 ES ticks (600 s, PM-0001-12's ES run length), 11:48:46-11:48:55: FAILED "gameplay events:
    7578" (thousandths a second; band 9,500-10,500). shots 70.783, spawns 21.120 (2,167 thrown), live 32.342: in band.
    Over the first 300 s the test's 9.853; so the second 300 s average (2 x 7.578 - 9.853) = 5.30 a second.
  - X3 D-6 over 1,800 s of SS (an S30), 11:50:10-11:51:59: PASS. 1,234,196 shots, 685.664 a second; 1,800 portal
    changes, 1.00 a second; 489 kills (0.272 a second; 100 s gives 28, 0.28); every per-tick minimum held (48
    soldiers, teams >= 16, >= 15 vehicles of which >= 4 large, >= 300 projectiles, 384 effect rows); large hulls <=
    1,400 cm out, others >= 5,199 cm, stand-ins start <= 1,200 cm. Magazine 32,000 lasts 2,194 s at one soldier's rate.
- [7] Carries and reading: carry 9 (raw pre-snap, grid of it; now per field over Soldier and OwnState records,
  tests/world.rs:1741-1802; the fixer's Q16 fails it; -41 quant's unit proof unchanged, tests/world.rs:2219-2237):
  MET. Carry 12 (layouts within FIELDS; s6 pins all 8 kinds by Field identity, tests/seam.rs:477-557; the fixer's
  Q22): MET, with W1-S1's change (velocity and weapon state out of SOLDIER, the ruling's lift); VELOCITY unchanged
  (grid.rs: +4 doc lines only vs 21a8597). Carry 13: storage identity MET (O5 fails it), counting half to U5 (C13;
  u4s-carries.md entry 48). Carry 14: MET at 10^6 B (220 ticks, 7,333 ms; Q19 fails both tests). Carry 36: MET
  (exhaustive match, tests/seam.rs:70-80; the fixer's Q23). ES's synthetic recording is 9,000 ticks by plan (§4.15,
  RQ5 "3 recordings x 5 min"; tests/replay.rs:20-28); ES's judged runs are 10 min each, every ES run checked by
  PM-0001-2's counter script within ±5 % (ADR 0001:1474-1480, :1600), driven by U4r's runner (plan §5 U4r :742:
  npt-bots' brawl and open-loop scripts), not by this generator today.
- [8] check.mjs, the one counted run, 11:54:59-11:59:37 UTC, by absolute path, cwd this copy's spikes/netcode-proof,
  shared target, offline, idle 12 s before, 2 s toucher, no other cargo at the end: all 10 steps PASS. fmt 0.4 s;
  clippy 8.8 s (measured and bites); clippy-wasm32 1.9 s (both); doc 33.4 s (both builds, host and wasm32, -D
  warnings); test 227.7 s: measured 351 passed, 0 failed, 0 skips; bites 388 passed, 0 failed, 0 skips (= expected);
  deps 0.7 s (108 crate files, 1 cargo config); refs 0.5 s (625 tracked files); marks 2.5 s (81 marks, drafts
  626df678); self 1.5 s (52 of 52); redact 0.8 s (10 of 10; 182 items, 0 findings). df 11 GB after.
- [9] Final: r2t-plant.py check ALL-OK (34 files = step 1's sha256, after Q1 and Q10a2 too); no r2t-backup left; no
  PLANT text in crates (grep); git status the same 10 M + 9 ?? as at the start; HEAD 21a8597; check.mjs 7eddc285,
  workspace Cargo.toml 75b6c160, Cargo.lock 3da1e15f unchanged. Nothing committed; nothing edited beyond restored
  plants.

## Round-1 findings (mine): CLOSED / NOT CLOSED
- M1 (ES misses PM-0001-2 bands; no ES test): CLOSED as ruled, on ES's 9,000-tick synthetic recording: t_m1
  (tests/world.rs:1005-1070) asserts the four bands and 48/15 every tick: 70.776 shots, 9.853 events, 21.120 spawns a
  second, 32.300 live; Q11a fails it (and the fixer's Q11b, Q11c). But see T2-M1: the event band holds for that
  window only.
- M2 (1 MB = 2^20): CLOSED (history.rs:7, 10^6 B; carry14 220 / 7,333; replay span 7,333 / 7,334; Q19 fails both).
- M3 (D-9 blind to a wrong blend): CLOSED (world.rs's blend unit test, fractions 0, 1, 127, 128, 255 against an f64
  reference; Q12 fails it 3.9 mm off at fraction 1).
- M4 (catch-up tick, FLAG_FIRED): CLOSED (FLAG_FIRED dropped, flags dense, world.rs:116-120; D-3 has a catch-up tick
  every 9th step, catch_ups > 40 asserted, and compares the own-state record; Q1 fails "CO step 4: record").
- M5 (load_movers grows pools): CLOSED (Movers sized at World::new, world.rs:782-795; load_movers_into guards every
  index, :2031-2058; the module test and r_m1; Q10a2 fails the module test).
- L1 (D-4 misses a NaN the snap launders): CLOSED (non_finite reads `pre`, world.rs:2005-2023; the fixer's Q24).
- L2 (std-sin unit test cannot fail): NOT CLOSED, not claimed (u4d-build.md "Left for", test L2): follow-up.
- L3 (clamp idempotence above about 40.57 m): CLOSED by the TUNE-2 ruling (bound in World's and the module's doc; D-8's
  half at the bound; the in-module scene test; the fixer's Q26, Q27).
- L4 (Replayer panics on a non-canonical recording): CLOSED (replay.rs:698-702; tests/replay.rs:608-637; the fixer's
  Q20).
- L5 (decode reserves 2^20 x 104 B from a 60 B header): CLOSED in code (replay.rs:320-323), NOT by any test: O4
  passes all 47 tests (T2-L2).
- L6 (SS magazine runs dry; D-6's 100 s cannot see it): CLOSED (32,000 rounds, const assert scene.rs:167-170; X3:
  685.664 shots a second and every per-tick minimum over a whole S30).
- N1 (d10 clamp shots not asserted), N2 (D-2 pipes without timeout): NOT CLOSED, not claimed, listed follow-ups.
- N3 (carry 13 storage identity): ruled C13, met in part; O5 shows the test bites; counting half to U5.

## New findings (round 2)
- T2-M1 MEDIUM, not before merge (with netcode N2-M1 and arch A2-L3, now measured): ES's synthetic recording meets
  PM-0001-2's 10 gameplay events a second only over the 300 s t_m1 reads. X2 (t_m1 at 18,000 ticks): 7.578 a second
  over 600 s, so about 5.30 in the second 300 s; shots, spawns and live projectiles stay in band. Cause (the fixer's
  point B): the generator's dead reckoning (replay.rs:431-432) ignores crouch, sprint and scripted speeds, so the aim
  among the other teams (replay.rs:458-466, one command in 11) drifts off its targets. `aim_one_in` = 11
  (scene.rs:80) was tuned on that same window, and scene.rs:114, :177 state "10 gameplay events a second" as a scene
  property. Scenario: U4d2 or U4r lengthens or windows an ES recording from this generator (PM-0001-12 judges ES runs
  of 10 minutes, each checked by PM-0001-2 within ±5 %: ADR 0001:1474-1480, :1600) and reads 7.6 events a second,
  24 % low, so PM-0001-2 fails and PM-0001-12's ES part is invalid. Reach today: none judged (the plan pins the
  synthetic ES recording at 9,000 ticks; judged ES runs come from U4r's runner). Cheapest proof: t_m1 at 18,000 ticks
  (X2). Fix: either t_m1 asserts each 100 s window in band over the length any consumer uses, or the generator's
  reckoning follows crouch, sprint and scripted speeds at the next GENERATOR bump (re-pinning D-7's ES); the doc
  lines say "over its 9,000-tick recording"; a U4r carry: PM-0001-2's ES checks shown on a 10-minute ES run of
  whatever drives ES before PM-0001-12's marks count.
- T2-L1 LOW, not before merge: two new invariants are held only by D-7's after-tick pin (tests/replay.rs:351), which
  this fix pass re-pinned after writing them: hull o part's frame convention (part_center, world.rs:446-458: O1 rotate
  by -yaw and O2 ignore yaw pass every semantic test, h1, a_m1, r_m1, D-1, D-2, D-3, D-6 included) and phase 3's
  refresh of the soldiers' records before the shots (present_volumes, world.rs:1475-1480: O3 passes the same). Both
  are right by reading today. Scenario: a later edit (e.g. write_records' order, arch A2-L4) breaks one and re-pins D-7
  with its own legitimate change; shots without a Rewind then meet last tick's boxes, or parts' boxes sit mirrored
  about the hull, and every semantic test stays green. Cheapest proof: two unit tests in world.rs's module:
  part_center at hull yaw 90 degrees maps offset (+120, 0, 100) to hull + (0, +120, 100) cm within 1 cm; and in one
  step a soldier moved out of its old box is hit at its new place by a no-Rewind shot (or: phase 3's soldier boxes
  equal hit_volumes_from of the post-move records).
- T2-L2 LOW, not before merge: rust L1 = my L5 is closed in code only (replay.rs:320-323): O4 (the 2^20 reserve
  restored) passes all 47 npt-sim tests; an allocation is unobservable without a counting allocator. Fix: one line in
  U5's carry 48 (u4s-carries.md): decoding a 60 B header that claims 2^20 ticks allocates at most room x
  size_of::<TickInput>().
- T2-L3 LOW (the fixer's point A, for the advisor): `aim_one_in` = 11 and `grenade_fuse_ms` = 1,515 are new stand-in
  tuning. aim_one_in is read only by the generator and its value is valid for the 300 s window (T2-M1);
  grenade_fuse_ms is read by World (world.rs:1466), so it also changes every bot-driven ES run's grenades (46 ticks,
  1.533 s, vs 2 s). Both need the advisor's nod as TUNE-1's did; the report states X2's number beside them.

## Fixer's open points
- (A): T2-L3. (B): T2-M1, Medium, measured. (C): SOLDIER's 8 fields are the ruling's 8 names (advisor-u4-wave1.md:79),
  the catalogue's row (:137) and the GROUPS test's 92-bit row; "(9 fields)" is a count slip. Weapon, Scene::weapon,
  Scene::health: public beyond W1-PUB's list, traced to TUNE-1's header condition, only caller D-6
  (tests/world.rs:618-628): text for §4.1 (as arch A2-L2, rust R2-L4). Carry 13: met in part, ruled (C13); O5 bites.
- For U4e's budget and TUNE-1's report: X1's 126.3 records changed a tick over an S30 (OwnState 47.49 of 48:
  owner-only, one per client); SS's kill rate 0.272 a second over an S30 (X3), not probe F's 0.42 (before this pass).

## Verdict
APPROVE WITH FINDINGS. Every round-1 finding of mine that the fix claims is CLOSED: M1-M5 with a plant I re-ran here
(Q11a, Q19, Q12, Q1, Q10a2); L1, L3, L4 by the fixer's plants (Q24, Q26-Q27, Q20), read, not re-run by me; L6 by
measurement (X3); L5 in code only (T2-L2); N3 by the C13 ruling (O5 shows its test bites). W1-S1's both halves proven:
Q2 (D-3's own-state record), P1, P3, P7 re-planted here; the fixer's Q16, Q17, Q18, Q22, Q23 read, not re-run; the
GROUPS row's 129 bits on the wire. check.mjs in this copy: 351 / 388 / 52 of 52, all 10 steps. No Critical or High,
so no round 3 from this lane. T2-M1 (Medium) and T2-L1 to T2-L3 (Low) are follow-ups.
DONE (2026-11-11 12:04 UTC)
