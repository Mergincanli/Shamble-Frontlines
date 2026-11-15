# BRAWL-1's build notes (builder copy 1)

## Start (brief section 1)

- Brief: brawl1-brief.md, sha256 921c3243958a09dd5f914fb7eb247f07333558d0905b3d849d2f13222d6d2a50, 261 lines (checked).
- Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/brawl1, detached (no branch), HEAD
  352a245c3018c50843f62dcb90e91973ea56d9da (= `<base>`), `git status --short` empty, spikes/netcode-proof tree
  abad063555b0105e2afd47dcff96acd7bd74f63e (checked 2026-11-15).
- Runner u4r-run.sh sha256 6efb91572cf06fd9ad6472c077cd39c9a59bfed8d4a3dac561992aafd5729bc8 (checked). Logs under
  /home/user/Shamble-Frontlines/.gate/u4r/brawl1/.
- Disk at start: 13,174 MiB free.
- No earlier notes: this is the first copy.

## Plan

1. Read the binding docs (advisor plan review, plan, rulings, facts) and check the facts I rely on.
2. Capture the precheck tests' --nocapture "before" log (r6, late-command bite, wallclock) at <base>.
3. Part 1 items 1-6 (EA-U4R-2, EA-U4R-1 narrow, N2-1 + TE-R2-1 with two plants, TE-R2-2 + RR4-L2, RR4-L1, RR3-L2 (a)).
4. Part 1 proofs (a)-(e) at the old pins; record each here before Part 2.
5. Part 2: npt-client read item + test; npt-bots leash (crate-private) + unit test + sim-only test (full ignored,
   short copy); the figure in Logs::room_tick + Counts + summary line; SERIES.md rows drafted here.
6. Full test by name in release; short copy times.
7. Re-pin the three entries; check.mjs, det-u4.mjs, precheck regeneration at the new pins.
8. git diff --stat; report.

## Progress

- 02:3x: read the brief, advisor-brawl1-plan-full.md (conditions 1-18, Q1-Q8), brawl1-plan.md, the rulings
  (sets-r2 BRAWL-1; r2 N2-1/TE-R2-1/TE-R2-2/RR4-L1/RR4-L2/item 18; r1 EA-U4R-1/-2/RR3-L2), brawl1-facts-full.md. Facts
  re-checked at <base>: closed_checks scenario.rs:2150-2164, fp1 :2171, closed branch :493-503 (leave :499, owner.remove
  :500, ended.push :502), Room::roster room.rs:568, Room::leave :690-695 (roster.leave :691), Roster::is_live
  roster.rs:80; fp3 :2423; parse's 64-bit checks :1376-1381, B1 check :1442-1453; hash.rs caps :229-237, bytes cap
  :325-331, runner checks from :332, r10_fp3 :1594, `one` helper :1531; run_leg precheck.rs:285-503 (`now` :372, :475);
  run's build scenario.rs:742-766, loop :879-1106; lab_room :2019-2034; Logs logs.rs:141-221, filed :325-364,
  room_tick :470-589; client Tick::complete lib.rs:161-165, Filed :140-145, ring :218; Wrap::after32 proto wrap.rs:60;
  Room::new next_event 0 (room.rs:459), so with_event_start(0) is the identity.
- Before log, attempt 1: .gate/u4r/brawl1/p1-before-1.log ran `--lib` (the library's 5 tests): 0 of the three ran. Not
  a capture. The precheck tests live in the binary (`--bin npt-lab`).
- Before log A (condition 6 (ii), the "before"): .gate/u4r/brawl1/p1-before-a.log, at <base>, exit 0: measured build
  r6 1 passed (39.40 s); bites build r6 + late-command + wallclock 3 passed (103.04 s). Cold build first: 12,500 MiB
  free after it.
- Finding: the wallclock bite's comparer line prints the L1 leg's World hash, which the bite XORs with wall-clock ns
  (server pump.rs:228-246), so it can differ between two runs of the same code. Before log B (a second run at <base>)
  is captured to show which lines vary with no change at all.
- Before log B: .gate/u4r/brawl1/p1-before-b.log, at <base>, exit 0 (same counts). Normalised diff A against B (runner
  lines, Compiling/Finished/Running lines and libtest's "finished in" seconds left out; 67 lines each): exactly two
  lines differ, the wallclock bite's comparer line and its result line, and only in the L1 leg's hash (theirs
  9375924675163200265 vs 1388086141469307493; L1 hex 821df9d0f7ec1b09 vs 134378fdc3ebb265). Equal in both: index 3,
  loopback hash 6328158668484839173 (57d222c7ed590705), compared 120, mismatching 117. So proof (e) is judged as: every
  line equal to A, except those two L1 hash values, which vary at <base> itself. This reading goes into the report
  whole (item 7), for the advisor.
- Normaliser used for every (e) diff: `grep -v -E "^(lock |pwd |=== |--- |      [0-9]+ Compiling| *Compiling| *Finished|
  *Running| *Blocking)" | sed -E 's/finished in [0-9.]+s/finished in <t>/'`.
- Part 1 item 1, EA-U4R-2, done: scenario.rs `pub(crate) type Sim = World` (after the imports), `Server::build(scene,
  seed, first_tick, clients, event_start, arm_tick, &Bites)` (in `impl Server`, before `new`); Bites::world/room, Server
  and Server::new name Sim; run builds through it; run_leg passes `Bites::default()`, event start 0 (arm tick
  rec.first_tick, unused without a World bite) and keeps its decoder-bite lines; lab_room uses it. logs.rs names Sim
  (dump_events, room_tick, room_second, p6's `Sim::new`). precheck.rs names no World type now (imports Room, RttSource,
  due_ticks, Quant, World, CONTENT, credentials dropped); its printed "World::new(scene, ..)" strings unchanged.
  hash.rs's `npt_sim::World::BITES` / bite digest / dump sites left as they are (the hash tool's, not a room's: the
  alias's doc says so).
- Part 1 item 2, EA-U4R-1 narrow form, done: scenario.rs `struct Wake { admitted, ended, to_run }` and `fn wake(server,
  &mut dyn Listener, &mut Schedule, now) -> Wake` (steps 1-2 plus the due count), after `impl Server`. run: admitted ->
  carry 75 check + logs.admit, ended -> logs.gone, then its per-tick body. run_leg: SimClock (`clock.now()` at the loop
  top, `clock.advance_to(next.max(now + 1))`), admitted -> its map, ended -> o.ended. No one-loop driver.
- Compile check after items 1-2 (.gate/u4r/brawl1/p1-check-1.log): cargo check -p npt-lab --all-targets, both builds,
  exit 0, no warning.
- Part 1 item 3, N2-1 with TE-R2-1, done: closed_checks' fourth check `if server.room.roster().is_live(slot)` with the
  ruled message "the roster still holds slot {slot}: a rejoin would be a replacement"; its doc says four checks. fp1's
  doc rests "not a replacement" on the roster (Room::join reports a replacement only for a roster slot held live:
  roster.rs:116 `replaced = s.live`). The no-op `!a.ended.contains(&(0, "replaced"))` assert dropped.
- The plants (no plant in place now):
  - room.leave deleted (scenario.rs's closed branch; scenario.rs sha256 before 9f65b752..., restored by Edit, after
    9f65b75252d945d85b74117cf56eb5fcd87a98bccdb9d1093c1cfbf14482ae65, equal): .gate/u4r/brawl1/p1-plant-leave.log, exit
    101, fp1 FAILED, both cases ["the room still holds slot 0: its slot was not freed", "the roster still holds slot 0: a
    rejoin would be a replacement"]: two of four, by name (Q5 (a); ADV-BP-1).
  - roster.leave deleted (room.rs:691; room.rs sha256 8fbe015c659515d67995f60ea4d86a30306b41368de4d946287116c6d8472c33
    before, restored by Edit, equal after; room.rs not in git status): .gate/u4r/brawl1/p1-plant-roster.log, exit 101,
    both cases ["the roster still holds slot 0: a rejoin would be a replacement"]: the new check alone.
- Part 1 item 4, TE-R2-2 with RR4-L2, done (scenario.rs parse()). Order: the 64-bit checks first, messages unchanged
  (ticks_of's products, the sum, the after phase: now bound as `last_tick`); then on every profile the run's last tick
  bound; then, where bursts are used (B1 with bursts > 0, as today's guard): the 64-bit last-burst check (message
  unchanged, now bound as `last_burst`), the last burst's u32 bound, the bursts cap, every-s 0. The messages (mine):
  - "--warmup-s, --measured-s and --after-max-s: the run's last tick, {last_tick} ticks from its first, is above
    4294967295 (32 bits)";
  - "--bursts {n}, --burst-first-s {f} and --burst-every-s {e}: the last burst's tick, {t} ticks from the run's first,
    is above 4294967295 (32 bits)";
  - "--bursts {n}: above 1000";
  - "--burst-every-s 0 with --bursts {n}: more than one burst needs an interval above 0 s".
  The :1442 comment now says what is bounded: the last burst's tick boundary from the run's first tick, ((bursts - 1) x
  every_s + first_s) x TICK_HZ + the warm-up's ticks, in 64 bits then 32, --bursts <= 1,000, every-s > 0 with more than
  one burst. fp3's cases (each exact): the old --measured-s case; TE-R2-2 (1) B1 line starts_with the ruled prefix; (2)
  equals the ruled message; (3) full digits, equals "--after-max-s: the run's last tick overflows 64 bits"; RR4-L2's
  ruled case contains "--burst-every-s 600000000000" and equals the last-burst message with 18000000001200; --bursts
  1001 refused, 1000 accepted; --bursts 2 --burst-every-s 0 refused, --bursts 1 --burst-every-s 0 accepted;
  --after-max-s 143165507 refused (4294967310), 143165506 accepted.
- Part 1 item 5, RR4-L1, done (hash.rs): `const MAX_ENTRY_TICKS: u32 = 36_000;` after MAX_ENTRY_BYTES; the refusal right
  after the bytes cap, before `if let Kind::Runner`, with the ruled message; manifest()'s doc names it. r10_fp3: 36,001
  refused, equal to "manifest entry SS-P2-s1: ticks 36001 is above 36000 (four times today's largest entry): a longer
  recording comes to the advisor first"; `one("TC-s3", 36_000, synthetic)` reads; the three committed manifests still
  read (the test's existing loop).
- Part 1 item 6, RR3-L2 (a), done (logs.rs): `logging: bool` in Logs from open's `dir.is_some()`; on a no-log run filed()
  calls `resolved` only (hashes and each lost tick), never `entry`; `last` kept. New test
  `rr3_l2_a_a_run_with_no_log_folder_keeps_no_per_tick_resolution`: Logs::open(None, ..), admit slot 3, 9,000 ticks of
  filings (hashes, sentinels, lost runs), then `res` empty, started(3) true, first/last = first tick, first + 8,999.
- Check after items 1-6 (.gate/u4r/brawl1/p1-check-2.log): clippy -D warnings npt-lab --all-targets both builds exit
  0; fp1_, fp3_, r10_fp3, rr3_l2: 4 passed in each build; fmt --check found three diffs (hash.rs r10 case, logs.rs test,
  scenario.rs fp3 line), applied by Edit.

## Part 1's proofs (condition 6), at the old pins

- (e) after Part 1: .gate/u4r/brawl1/p1-after-e.log, exit 0: measured r6 1 passed (38.92 s); bites build 3 passed
  (102.32 s). Normalised diff against A (and against B): every line the three tests print is identical, but for
  (i) the wallclock bite's L1 hash in its comparer and result lines (after: theirs 3393553388473618463, L1 hex
  2f1852d3b64b101f; A 9375924675163200265 / 821df9d0f7ec1b09; B 1388086141469307493 / 134378fdc3ebb265): the same two
  values that differ between A and B at <base> itself, with index 3, loopback hash, compared 120 and mismatching 117
  equal in all three; and (ii) libtest's summary counts "52 filtered out" -> "53" (measured) and "58" -> "59" (bites):
  the npt-lab binary holds one more test, RR3-L2 (a)'s new logs.rs unit test that the brief requires. No line of r6's,
  the late-command bite's or the wallclock bite's own output moved. Both differences go to the advisor whole in the
  report (item 7); I read them as not a behaviour change and go on.
- (a) check.mjs, run 1: .gate/u4r/brawl1/p1-a-check.log, exit 1, 03:01:50-03:05:48 UTC (free after: 9,475 MiB).
  PASS fmt, clippy, clippy-wasm32, doc, wasm (SS-s1/ES-s2/TC-s3 equal; hash-map caught at SS-s1 tick 1234), refs,
  marks, self (77/77), redact (10/10). FAIL test: measured build 187 passed, 1 failed:
  hash::tests::r10_carry_82_the_manifest_refuses_a_bad_name_and_ticks_above_bytes_over_12 (hash.rs:1561 "ticks at bytes
  / 12": its boundary entry holds 799,693 / 12 = 66,641 ticks, which RR4-L1's 36,000 cap now refuses; the facts saw only
  r10_fp3's 83,333). FAIL deps: "crates/lab/src/hash.rs:238 sets MAX_ENTRY_TICKS from `36_000`: a tick-counted value is a
  const fn of npt_sim::TICK_HZ, never a literal (u4-build-plan.md §4.3)" (check.mjs's TICK_NAMED /_TICKS$|CATCHUP/ reads
  every *_TICKS const outside npt-sim). Both failures are Part 1's own code against a check, not a behaviour change of the
  refactors; fixed below, then check.mjs run again (a new run on changed code, not a retry of this one).
- Fix 1 (deps): the ruled `const MAX_ENTRY_TICKS: u32 = 36_000;` cannot pass check.mjs as written, and the form
  check.mjs accepts (`4 * npt_sim::ticks_for_ms(300_000)`) would be a local rate-derived constant, which carry 39 bars.
  Chosen: the same literal 36,000, placement, message and tests under the name `MAX_ENTRY_TICK_COUNT` (a manifest size
  bound like MAX_ENTRY_BYTES, the ruling's own analogy). For the advisor (report item 7), with the other option.
- Fix 2 (test): r10_carry_82's ticks <= bytes / 12 boundary moved below the cap: its entry's bytes set to 360,000
  (30,000 ticks at the boundary; 30,001 refused "cannot fit"), so it still tests that rule.
- Also rewrapped my comment and string lines that ran past 120 characters (no text change; the messages read the same).
- (a) check.mjs, run 2: .gate/u4r/brawl1/p1-a-check2.log, exit 0, ended 03:20:12 UTC: all 11 steps PASS. test 635.8 s:
  measured build 532 passed, 0 failed, 0 skips; bites build 610 passed, 0 failed, 0 skips. wasm 42.8 s (synthetic
  three equal; hash-map caught). deps held. PASS.
- (b) det-u4.mjs: .gate/u4r/brawl1/p1-b-det.log, exit 0. uptime first in the same call: "03:20:25 up 45 min, load
  average: 1.42, 1.96, 1.57" (no other lane building or running: only the session's checkpoint saver). PASS deps; PASS
  det-u4 232.9 s: native/native and native/Wasm 18,000 ticks equal (SS-P2-s1 9000, ES-P3-s2 9000), h2 equal; seconds:
  build npt-lab 9.5, build wasm 1.2, npt-lab hash 218.5, Wasm SS-P2-s1 2.6, Wasm ES-P3-s2 0.9, compare and h2 0.0.
  Non-cargo sum 222.0 s (under 240). Regenerated files: SS-P2-s1 16,937,652 B sha256 5b79abb2a2cba55b...a29a32e3,
  ES-P3-s2 16,929,501 B fad8f67ded2a220e...0045953b: the old pins. PASS.
- (c) and (d) in one call: .gate/u4r/brawl1/p1-cd.log, every command exit 0 (free 7,074 MiB after).
  - (c) `npt-lab hash --manifest crates/lab/precheck-recordings.json` (release, measured): "regenerated SS-P0-s4: 16950444
    bytes, sha256 6933b56a619197c719439ff0c16eed3b8927645c3a0bb66910568c6ccf83bab3, as the manifest's"; native leg 1 vs
    leg 2 9,000 ticks equal; real 2m8.958s. PASS (byte-equal at the old pin).
  - (d) SS-P0-s4 recorded at its entry's settings (`scenario --run record-SS-P0-s4 ... --scene SS --profile P0 --seed 4
    --clients 48 --tick-start 0 --warmup-s 10 --measured-s 290 --logs 0 --suite record --record <file>`, real
    2m4.470s; rewinds refused 0; file sha256 6933b56a..., the old pin), then `npt-lab precheck --manifest
    crates/lab/precheck-recordings.json --recording <file> --commit 352a245c...` with no bite (real 3m3.950s): its 109
    output lines against results/u4r-precheck-30/faf42aeb034c/runs.txt:17-125 ("## pair clean"): 108 identical, the
    build line differs only in commit (352a245c... vs faf42aeb...) and binary sha256 (e4e6a258d4609a15... vs
    b3b4a60da4cfb4b3...); the machine part equal. result: PASS (9000 ticks, every World hash equal). PASS.
- (e) was captured before the check.mjs fixes (the hash.rs rename, r10_carry_82, comment wrapping): captured again on
  the final Part 1 code as .gate/u4r/brawl1/p1-after-e2.log, so all five proofs stand on one code state.
- (e) again: p1-after-e2.log, exit 0 (measured r6 1 passed 40.27 s; bites 3 passed 102.94 s). Normalised diff against
  A: the same two kinds only: the wallclock L1 hash (now 4e64789de1560c6a; varies at <base> itself) and the +1 test in
  "filtered out" (52->53, 58->59). Every printed line of the three tests otherwise identical.
- **Part 1 complete; its proofs (a)-(e) recorded above, all on one code state.** Next: Part 2.

## Part 2

- Written (not yet compiled at this line): client lib.rs `pub fn own_soldier(&self) -> Option<([i32; 3], bool)>` (after
  newest_tick) + private SOLDIER_FLAGS = 5, SOLDIER_DEAD = 1 << 3; client tests/endpoint.rs
  `brawl1_own_soldier_is_the_newest_complete_filed_tick_s_own_record` + helper `own_only`. bots lib.rs: LEASH_OUT_CM
  1_400, LEASH_IN_CM 1_300, `steering` field, crate-private `next_command(newest, view, own)` (step calls it with
  `endpoint.own_soldier()`), `leash`, free fn `toward_centre`, `command` takes `own` (Kind::Brawl only); no sideways
  part; #[cfg(test)] mod tests: unit test, `sim_run`/`printed`, short copy `brawl1_sim_short_no_soldier_enters_a_building`,
  full `brawl1_sim_full_no_soldier_enters_a_building_seeds_1_to_3` (#[ignore]). logs.rs room_tick: tick name
  `in_building_slot_area` (pairs slot, area; written when not empty, any phase); Counts.in_building_measured +
  in_building_slots; summary_line appends "; in a building: N slot-ticks (slots …)" ("not counted (--logs 0)" on a
  no-log run).
- p2-check-1.log: fmt diffs (bots tests, client test) applied by Edit; clippy type_complexity on the test's VecDeque
  type: `type Seen = (u8, [i32; 3], bool, u16)` added in the test module.
- p2-check-2.log (exit 0): fmt clean; clippy -D warnings npt-bots, npt-client, npt-lab --all-targets, both builds,
  clean; client `brawl1_own_soldier_...` 1 passed; bots unit test 1 passed; short copy alone by name: measured (dev)
  build 10.97 s, bites (dev) build 11.07 s (under 60 s: not shortened). Short copy's prints (seed 1, 3,600 ticks):
  EveryTick in a building 0, largest distance 1,575 cm (slot 27, tick 65928), stuck none; Skips in a building 0,
  largest 1,629 cm (slot 39, tick 63768), skipped 53,039 of 172,752 chances (307 per 1000), longest streak 6, stuck
  none.
- Full test, release, by name: p2-full-release.log ran nothing (`--exact` needs the module path: 0 passed, 3 filtered
  out); p2-full-release-2.log (`tests::brawl1_sim_full_...` --ignored --exact): exit 0, 1 passed, finished in 23.51 s.
  Per seed and pattern (18,300 ticks each; in a building 0 soldier-ticks in all nine; stuck none in all nine):
  - seed 1: EveryTick largest 1,575 cm (slot 27, tick 65928); Skips 1,654 cm (slot 32, tick 72749), 267,095 of 878,352
    skipped (304 per 1000), longest streak 6; Bursts 1,832 cm (slot 7, tick 75766), longest streak 30.
  - seed 2: EveryTick 1,575 cm (slot 16, tick 68589); Skips 1,720 cm (slot 36, tick 69092), 266,728 (303 per 1000),
    streak 6; Bursts 1,832 cm (slot 7, tick 75766), streak 30.
  - seed 3: EveryTick 1,570 cm (slot 31, tick 69585); Skips 1,686 cm (slot 30, tick 67741), 267,925 (305 per 1000),
    streak 6; Bursts 1,832 cm (slot 5, tick 79366), streak 30.
  - Pattern (iii) for U4g's brief: no area != 0 in three seeds; largest distance 1,832 cm in all three, each at a burst's
    end (tick 75766 = k 12,030, burst 7's end; 79366 = k 15,630, burst 9's end): 32 cm past the nearest door entry's
    1,800 cm figure, outside a building. The same figure three times suggests a soldier stopped by a fixed face; the
    test now prints the soldier's x, y beside it, for the final run.

## The re-pin (Q3 (a); conditions 7 and 8)

- After Part 1's proofs and Part 2's tests passed (above). Regeneration: .gate/u4r/brawl1/repin-regen.log (release
  npt-lab built at 03:43:11):
  - SS-P0-s4 (`hash --manifest crates/lab/precheck-recordings.json`, exit 1 by the mismatch, real 2m5.524s): regenerated
    16,950,444 bytes, sha256 1f00d2d5f70a2c7b6836ede611b5f1c281319a5312060ead91e016da3c0b1b42; old 16,950,444 B
    6933b56a619197c719439ff0c16eed3b8927645c3a0bb66910568c6ccf83bab3.
  - SS-P2-s1 (`hash --manifest crates/lab/det-recordings.json`, exit 1, real 2m27.782s): regenerated 16,937,652 bytes,
    sha256 1ec25de7c9ce8ea869ed9a8b343746ba131593e1de5c65bc9c1c626ce01cc76d; old 16,937,652 B
    5b79abb2a2cba55b06a8916b235aea1766960867f09a038aed146805a29a32e3. `hash` stops at the first mismatch, so ES-P3-s2
    was not regenerated in that call: SS-P2-s1 and SS-P0-s4 pinned by Edit first, then the det manifest run again
    (which reproduces SS-P2-s1's new pin and prints ES-P3-s2's).
  - repin-regen2.log (det manifest with SS-P2-s1's new pin; exit 1 by ES-P3-s2's mismatch, real 3m49.476s): SS-P2-s1
    regenerated again equal to 1ec25de7... (its file kept: sha256 1ec25de7c9ce8ea8...ce01cc76d); ES-P3-s2: regenerated
    16,929,501 bytes, sha256 6abf7643982b5970a630c97daa4d0815ceb2022958b18e18116397dcad9c893e; old 16,929,501 B
    fad8f67ded2a220e42d7e4d42a3d3f2f76e980d4c55abb4fbcd01c550045953b.
- The pins, old -> new (bytes unchanged in all three):
  - SS-P0-s4 (precheck-recordings.json): 6933b56a619197c719439ff0c16eed3b8927645c3a0bb66910568c6ccf83bab3 ->
    1f00d2d5f70a2c7b6836ede611b5f1c281319a5312060ead91e016da3c0b1b42 (16,950,444 B)
  - SS-P2-s1 (det-recordings.json): 5b79abb2a2cba55b06a8916b235aea1766960867f09a038aed146805a29a32e3 ->
    1ec25de7c9ce8ea869ed9a8b343746ba131593e1de5c65bc9c1c626ce01cc76d (16,937,652 B)
  - ES-P3-s2 (det-recordings.json): fad8f67ded2a220e42d7e4d42a3d3f2f76e980d4c55abb4fbcd01c550045953b ->
    6abf7643982b5970a630c97daa4d0815ceb2022958b18e18116397dcad9c893e (16,929,501 B)
- The manifests' diff (Edit): only those three `sha256` values (checked: each removed line equals its added line but
  for sha256); both "about" texts, every other field and recordings.json's three synthetic entries unchanged.
- Next: check.mjs; uptime + det-u4.mjs; the precheck regeneration at the new pin; short copy times and the full test
  at this head.

### The SERIES.md rows (drafted for the main session's Edit; condition 10)

One written name only: no lab end-event field carries the total (the summary line prints it; Counts is not logged).
A new heading after "Names added with U4r" (marks/SERIES.md:333-366), before "## Series read only by figures":

```
### Names added with BRAWL-1 (npt-lab `scenario`'s room log; advisor-u4r-sets-r2-full.md, BRAWL-1 Fix (b))

| Name | Unit | Written by (crate) | Meaning | Marks |
|---|---|---|---|---|
| `tick.s.in_building_slot_area` | pairs (slot, area) | npt-lab (from npt-sim's Soldier records' `area`) | the room log, on every logged tick (any phase) where a soldier's feet are in a building area: each such soldier's slot, then its area (building i is area i + 1: the greybox's area rule, half-open on each footprint), in slot order; absent on a tick with none. The run's summary line prints the measured total as "in a building: N slot-ticks (slots ...)" | none (recorded) |
```
