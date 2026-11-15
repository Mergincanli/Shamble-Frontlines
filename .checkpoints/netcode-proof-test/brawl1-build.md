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
- At the new pins, all at the final head (the diff handed over):
  - check.mjs: .gate/u4r/brawl1/final-check.log, exit 0, ended 04:04:33 UTC: all 11 steps PASS; test 630.1 s: measured
    535 passed, 0 failed, 0 skips; bites 613 passed, 0 failed, 0 skips (Part 1's 532/610 plus the three new tests;
    the #[ignore]d full test is not counted).
  - det-u4.mjs: .gate/u4r/brawl1/final-det.log, exit 0. uptime first: "04:04:39 up 1:30, load average: 1.42, 1.81,
    1.48" (no other lane building or running). PASS det-u4 227.2 s (18,000 ticks equal native/native and
    native/Wasm; h2 equal); seconds: build npt-lab 11.3, wasm 1.3, npt-lab hash 211.1, Wasm 2.5 + 0.9. Non-cargo sum
    214.5 s (under 240). Both new det pins reproduced.
  - Precheck regeneration and the tests: .gate/u4r/brawl1/final-regen-tests.log, every command exit 0: "regenerated
    SS-P0-s4: 16950444 bytes, sha256 1f00d2d5...0b1b42, as the manifest's", legs equal, real 1m53.069s; short copy
    alone: measured build 10.61 s, bites build 10.71 s; full test in release 23.52 s, 1 passed (results in the report).
- Size: 869 added, 123 deleted, 992 changed lines (bots 426, scenario.rs 293, logs.rs 82, client test 60, precheck.rs
  52, hash.rs 42, client lib 31, manifests 6). Past the plan's ~700 (450-750), under the 1,400 stop line. Free at the
  end: 6,579 MiB.

### The SERIES.md rows (drafted for the main session's Edit; condition 10)

One written name only: no lab end-event field carries the total (the summary line prints it; Counts is not logged).
A new heading after "Names added with U4r" (marks/SERIES.md:333-366), before "## Series read only by figures":

```
### Names added with BRAWL-1 (npt-lab `scenario`'s room log; advisor-u4r-sets-r2-full.md, BRAWL-1 Fix (b))

| Name | Unit | Written by (crate) | Meaning | Marks |
|---|---|---|---|---|
| `tick.s.in_building_slot_area` | pairs (slot, area) | npt-lab (from npt-sim's Soldier records' `area`) | the room log, on every logged tick (any phase) where a soldier's feet are in a building area: each such soldier's slot, then its area (building i is area i + 1: the greybox's area rule, half-open on each footprint), in slot order; absent on a tick with none. The run's summary line prints the measured total as "in a building: N slot-ticks (slots ...)" | none (recorded) |
```

## Report

BRAWL-1 is built in the worktree (/home/user/Shamble-Frontlines/.claude/worktrees/brawl1, detached at 352a245c, all of it
uncommitted). Part 1's proofs (a) to (e) hold at the old pins, read as noted in item 7. Part 2 is built, and its tests
pass. The three pins have moved, and check.mjs (all 11 steps), det-u4.mjs and the precheck regeneration pass at the new
pins. Nothing stopped me. Three points go to the advisor (item 7). The size is 992 changed lines, against ~700 planned.

1. **Part 1** (file:line in the final diff):
   - EA-U4R-2: scenario.rs:51 `pub(crate) type Sim = World`; :373 `Server::build(scene, seed, first_tick, clients,
     event_start, arm_tick, &Bites)`, used by run (:814), run_leg (precheck.rs:293: Bites::default(), event start 0;
     its decoder-bite lines kept) and lab_room (scenario.rs:2113). Bites::world/room, Server and Server::new name Sim.
     logs.rs:28 names Sim (dump_events, room_tick, room_second). precheck.rs names no World type any more. The printed
     "World::new(scene, ..)" strings are unchanged. hash.rs's bite-list and dump sites still name npt_sim::World (the
     alias's doc says why).
   - EA-U4R-1, the narrow form: scenario.rs:591 `struct Wake {admitted, ended, to_run}` and :601 `fn wake(..)`
     (steps 1-2 plus the due count). run (:940) sends to logs.admit (with carry 75's check) and logs.gone. run_leg
     (precheck.rs:368) sends to its map and o.ended, on a SimClock (:363, :461). No one-loop driver.
   - N2-1 + TE-R2-1: closed_checks (scenario.rs:2231) has the fourth check with the ruled message (:2246), and its doc
     says four. fp1's doc (:2259) rests "not a replacement" on the roster (roster.rs:116 `replaced = s.live`). The
     no-op assert is gone. Plants, each restored by Edit with its sha256 checked: room.leave deleted fails two of the
     four checks by name in both cases (p1-plant-leave.log); roster.leave (room.rs:691) deleted fails the new check
     alone (p1-plant-roster.log).
   - TE-R2-2 + RR4-L2 (parse()): the 64-bit checks come first, with their messages unchanged. Then, on every profile,
     the run's last-tick bound (:1430-1441). Then, where bursts are used (B1 with bursts > 0): the 64-bit last-burst
     check, the last burst's u32 bound (:1507-1521), the bursts cap (:1523), and every-s 0 (:1526). The :1442 comment
     now says what is bounded. My messages:
     - "--warmup-s, --measured-s and --after-max-s: the run's last tick, {t} ticks from its first, is above 4294967295
       (32 bits)"
     - "--bursts {n}, --burst-first-s {f} and --burst-every-s {e}: the last burst's tick, {t} ticks from the run's
       first, is above 4294967295 (32 bits)"
     - "--bursts {n}: above 1000"
     - "--burst-every-s 0 with --bursts {n}: more than one burst needs an interval above 0 s"

     fp3 (:2510) checks the exact text of every case: TE-R2-2's three (case 3 in full digits), RR4-L2's ruled case
     (it names --burst-every-s 600000000000, at 18000000001200), --bursts 1001, --burst-every-s 0 with 2 bursts, and
     --after-max-s 143165507. It also shows each bound's own value accepted.
   - RR4-L1: hash.rs:239 `const MAX_ENTRY_TICK_COUNT: u32 = 36_000;` (renamed: item 7). Its refusal at :338 comes
     right after the bytes cap and before the runner checks, with the ruled message. r10_fp3 refuses 36,001 by that
     message (:1659); a synthetic entry at 36,000 reads (:1663); the three committed manifests still read.
     r10_carry_82's bytes/12 boundary moved below the cap (:1563; item 7).
   - RR3-L2 (a): logs.rs:149 `logging` (set at :210 from dir.is_some()). On a no-log run, filed() resolves only
     (:342) and never grows `res`; `last` stays. New test at :849 (rr3_l2_a_...): after 9,000 ticks, `res` is empty,
     started() is true, and first/last are right.
   - The proofs, all on one code state, at the old pins:
     - (a) check.mjs, p1-a-check2.log: all 11 PASS (measured 532, bites 610). Run 1 (p1-a-check.log) failed test and
       deps on Part 1's own code (item 7); it was fixed, then run again.
     - (b) det-u4.mjs, p1-b-det.log: uptime first ("load average: 1.42, 1.96, 1.57"). PASS in 232.9 s; non-cargo sum
       222.0 s. It regenerated 5b79abb2 and fad8f67d.
     - (c) precheck regeneration, p1-cd.log: "regenerated SS-P0-s4: 16950444 bytes, sha256 6933b56a..., as the
       manifest's". PASS.
     - (d) clean pre-check pair, p1-cd.log: recorded SS-P0-s4 (sha256 6933b56a), then precheck with no bite. Of the
       109 lines in runs.txt:17-125, 108 are identical; the build line differs only in commit and binary sha256.
       result PASS.
     - (e) p1-after-e2.log against p1-before-a.log: every line the three tests print is identical, but for two
       differences that are not behaviour (item 7).
2. **Part 2:**
   - The read item: client lib.rs:350 `pub fn own_soldier(&self) -> Option<([i32; 3], bool)>`. It reads the grid
     x, y, z (decoded integers) and the dead bit (flags bit 3, via the private SOLDIER_FLAGS = 5 and SOLDIER_DEAD at
     :247-248). The source is the newest ring tick that is complete and filed Filed::Hash, compared by
     `self.wrap.after32("tick", ..)`. It returns None with no slot, no such tick, or no (Soldier, slot) record. It
     hands back no reference, no float, and adds no dependency.
   - The decision function: bots lib.rs:231 crate-private `next_command(newest, view, own)`, which step() calls with
     `endpoint.own_soldier()` (:223). It calls `command`, which applies `leash` (:248) for Kind::Brawl only, and counts
     the command given. `toward_centre` (:128) works in integers: each part is 127 × its share of ⌈|p|⌉, truncated, so
     x² + y² ≤ 127² (and above 125² at the radii). npt-bots gains no pub item, and its library names no World or
     SimCore.
   - The leash's values: R_IN = LEASH_IN_CM 1,300 cm, R_OUT = LEASH_OUT_CM 1,400 cm, with no sideways part. Its
     margin is 4 m to the door entry (1,800 cm at building 0's door). The spawn corners lie at 1,414 cm. I tuned
     nothing: these are the first values tried, and they passed.
   - The figure: room.jsonl's tick name `in_building_slot_area`, written as (slot, area) pairs on every logged tick
     where it is not empty, in any phase (logs.rs:598). Counts gains `in_building_measured` and `in_building_slots`.
     The summary line appends "; in a building: N slot-ticks (slots …)" ("not counted (--logs 0)" on a no-log run).
     No judge or marks.json change, and no lab end-event field. One SERIES.md row is drafted above ("The SERIES.md
     rows").
3. **The tests:**
   - The unit test brawl1_the_leash_... (bots :347) passes. It covers the walk inside R_IN (against a twin of the same
     seed), the steer past R_OUT, the hysteresis, blocked and skipped steps, None and dead, an open-loop script that
     never steers, and a sweep for the 127² bound.
   - The client test brawl1_own_soldier_... (endpoint.rs:397) passes.
   - The short copy brawl1_sim_short_... (bots :636; seed 1, ticks_for_ms(120_000), patterns (i) and (ii)) takes
     10.61 s in the measured build and 10.71 s in the bites build (earlier 10.97 s and 11.07 s), so it was not
     shortened.
   - The full test (bots :648, #[ignore]) ran by name in release in 23.52 s (final-regen-tests.log). In all nine runs
     no soldier was in a building and none was named stuck. Largest distances from the centre:
     - seed 1: (i) 1,575 cm; (ii) 1,654 cm (skips 304 per 1000, longest streak 6); (iii) 1,832 cm at (1734, -592).
     - seed 2: (i) 1,575 cm; (ii) 1,720 cm (303 per 1000, streak 6); (iii) 1,832 cm at (1671, -751).
     - seed 3: (i) 1,570 cm; (ii) 1,686 cm (305 per 1000, streak 6); (iii) 1,832 cm at (1586, -917).
   - Pattern (iii), for U4g's brief: no area != 0, and a largest distance of 1,832 cm in every seed. Each maximum
     falls at a burst's end, at three different points on one circle of about 18.32 m. That is 32 cm past the
     1,800 cm door-entry figure, and outside any building. That a fixed radius stops it is an inference I did not
     check. One candidate is the outer reach of the 14 m fight vehicle's parts.
4. **The re-pin** (bytes unchanged; the manifests' diff touches only the three sha256 values):
   - SS-P0-s4: 6933b56a...bab3 → 1f00d2d5f70a2c7b6836ede611b5f1c281319a5312060ead91e016da3c0b1b42
   - SS-P2-s1: 5b79abb2...32e3 → 1ec25de7c9ce8ea869ed9a8b343746ba131593e1de5c65bc9c1c626ce01cc76d
   - ES-P3-s2: fad8f67d...953b → 6abf7643982b5970a630c97daa4d0815ceb2022958b18e18116397dcad9c893e

   `npt-lab hash` stops at the first mismatch, so ES-P3-s2's pin came from a second det run, which also reproduced
   SS-P2-s1's new pin. At the new pins:
   - det-u4.mjs (final-det.log): uptime "04:04:39 up 1:30, load average: 1.42, 1.81, 1.48"; PASS in 227.2 s;
     non-cargo sum 214.5 s.
   - The precheck regeneration (final-regen-tests.log) reproduces 1f00d2d5..., "as the manifest's".
5. **check.mjs's final run** (final-check.log): all 11 steps pass. Measured build 535 passed, 0 failed, 0 skips; bites
   build 613 passed, 0 failed, 0 skips. Test step 630.1 s.
6. **The size:** `git diff --stat` shows 9 files, 869 insertions and 123 deletions: 992 changed lines, against ~700
   planned (450-750 with the conditions) and the 1,400 stop line. The bots crate takes 426 of them, about 330 in its
   test module (the unit test, the sim driver, and the short and full copies).
7. **Nothing stopped me. Three points go to the advisor, whole:**
   - **Proof (e)'s reading.** At <base>, two "before" runs (A and B) already differ in the wallclock bite's L1 hash.
     That bite XORs aims with wall-clock ns (server pump.rs:228-246). After Part 1 the only differences are that same
     L1 hash (index 3, the loopback hash, 120 compared and 117 mismatching all equal), and libtest's "52/58 filtered
     out" becoming 53/59, because the binary holds RR3-L2 (a)'s new test. I read neither as a behaviour change and
     went on.
   - **RR4-L1's name.** The ruled `const MAX_ENTRY_TICKS: u32 = 36_000;` fails check.mjs's deps step: TICK_NAMED
     reads every *_TICKS const outside npt-sim as rate-derived. The form check.mjs accepts,
     `4 * npt_sim::ticks_for_ms(300_000)`, is a local rate-derived constant, which carry 39 bars. Built: the same
     literal, placement, message and tests under the name MAX_ENTRY_TICK_COUNT.
   - **r10_carry_82.** Its bytes/12 boundary entry (799,693 / 12 = 66,641 ticks) meets the new cap, which the facts
     missed (they checked r10_fp3 only). The test now uses 360,000 bytes: 30,000 ticks at the boundary, and 30,001
     refused "cannot fit".

   Also recorded: p1-before-1.log ran `--lib`, so none of the three tests ran (not a capture), and
   p2-full-release.log's `--exact` without the module path ran 0 tests. Both were run again with the right selector;
   no failed run was re-run into a pass.
8. **The advisor's two corrections, in its words** (condition 18): "ADV-BP-1: N2-1's 'all four checks fail' was
   wrong; under the leave plant two fail (Q5). ADV-BP-2: my sets-r2 figure of 'about 4.2 GiB free after the refill'
   mixed units."

Decisions needed:
- TICKS-NAME: the ruled `MAX_ENTRY_TICKS = 36_000` fails check.mjs as written. Which form stands? It matters because
  check.mjs must pass at the merge. Options: (a) MAX_ENTRY_TICK_COUNT, a literal 36,000 (built); (b) MAX_ENTRY_TICKS =
  4 * npt_sim::ticks_for_ms(300_000), which check.mjs accepts but carry 39 bars; (c) list MAX_ENTRY_TICKS in check.mjs's
  NOT_TICK_COUNTS, a check.mjs change that belongs to its follow-up's lane. Recommendation: (a).
- PROOF-E: does proof (e) hold with its two differences (the wallclock L1 hash, which varies at base, and the +1 test
  count)? It matters because condition 6 says any difference comes to the advisor. Options: accept, or name another
  comparison. Recommendation: accept, and leave the wallclock hash out of the comparison in future captures.

For Mustafa: nothing (spike code: nothing a player sees, hears or feels).

## Fix pass, round 1 (copy 2)

### Start checks (2026-11-15, 06:39 UTC)

- Prompt brawl1-copy2-prompt.md sha256 e89905c169af935524b38d8ec6b009ec86e11e4eff00eb8389e6d1dc005c0ac6 (equal), 208
  lines, 33,105 bytes (the launch line says 33,103 characters: multi-byte characters such as ² and ×, not a mismatch).
- HEAD 352a245c3018c50843f62dcb90e91973ea56d9da (equal). Brief sha256 921c3243958a09dd5f914fb7eb247f07333558d0905b3d849d2f13222d6d2a50
  (equal). Runner u4r-run.sh sha256 2f754d125b10b6bde06a980f46cb4e078a582ca277ec8828bc48e8aacb064e50 (equal); its one-line
  change read in u4r-run.sh.te-b1-7.diff (the header's new `diff ...; untracked ...` line after the pwd line).
- `git diff HEAD | sha256sum` in the worktree: ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7 (equal
  to wip/brawl1.patch's). `git status --short`: the 10 files, all ` M`, nothing untracked.
- `df -h /home/user`: 6.5G available (6,565 MiB).
- Ruling advisor-brawl1-r1-full.md sha256 86bec63bfffbd676efbc09e09c35a952e63abc7865e89b20b5aebec11bee75c5 (equal), but
  `wc -l` gives 460 lines, not the 249 the resume part names: the hash is the identity, so it holds; noted for the main
  session. Findings file brawl1-review-r1-full.md sha256 fc979c63...3305 (equal).
- FP-8's "before": det-recordings.json a2faf3ea81623c39a13b04f9a97cadb900d8ec482df4375173720f70dae95d99,
  precheck-recordings.json 5ccf57f059026e072cb000c0f9e66ca41ec81b9a02089fd6c26c699c21416881.
- Size before the pass, numstat against 352a245 (SERIES.md included): 875 added, 123 deleted, 998 changed lines.
- Read before any change: these notes whole (388 lines), `git status --short` and the whole `git diff` (10 files), the
  brief whole, the ruling's fix pass (prompt lines 27-208 = ruling lines 11-192), its "The pins", "Round 2", "The merge
  step", "U4g's brief", "The size" and "Each finding", and the points ruling's ADV-BP-3/ADV-BP-4 and Point 2 words.
  Note: the resume part names "the brief's section 4" for the rules; the brief's section 4 is "What you hand over", and
  the rules themselves are in its sections 1, 2 and 5. The ruling's step 1 lists them, and I follow that list.

### Plan (copy 2)

1. FP-0 evidence: a first runner log (fp-0-header.log) before any edit, its header showing diff ee2973aa.
2. Edits, by Edit only: FP-1 (bots tests (a)-(d)), FP-2 (unit test (a)-(d)), FP-3 (logs.rs test + module doc), FP-4
   (client test (a), wrap test (b), docs (c), (d)), FP-5 (bots docs (a)-(c)), FP-6 (scenario.rs (a)-(c), hash.rs (d)).
3. One check run: fmt --check, clippy both builds, the new and changed tests by name, the short copy in both builds.
4. FP-7: P1 (with the door-entry bar only if the bite rule fails on area), P2, P3, P4, one at a time, each sha256-checked.
5. FP-8: check.mjs; uptime + det-u4.mjs; regeneration + short copy both builds + full test in release; manifests'
   sha256; numstat.
6. FP-9 words and the report, on disk.

### Rulings (advisor-brawl1-r1): state

- FP-0 (main session's): done at 7402dac. My first runner log, before any edit: .gate/u4r/brawl1/fp-0-header.log
  (06:41:08 UTC, `git status --short`, exit 0), header "diff ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7;
  untracked 17cf51d7c16f37afb4247c27fc96ede8b439d461e6d55d1e55c141e0ccd4a90b" (the porcelain list of the 10 modified
  files; nothing untracked). FP-0's evidence holds.
- FP-1 (a)-(d): written, not yet compiled (07:0x UTC). (a) the 48-record assert after `let now = soldiers(&world);`.
  (b) the tie in soldiers() with the ruled message, Record.dead counted in sim_run's loop (dead bound there),
  printed() prints "dead {n} soldier-ticks", the short copy asserts `r.dead > 0` ("the dead-bit tie is exercised"),
  soldiers()'s doc carries the ruled sentence. (c) `brawl1_a_soldier_teleported_into_building_0_reads_area_1`
  (Placement imported). (d) printed(): Skips "skipped {n} of {c} chances ({rate} per 1000)", EveryTick "skipped {n}",
  Bursts "skipped {n} in its bursts", streak for all three.
- FP-2 (a)-(d): written, not yet compiled. (a) blocked loop on at(1_450, 200) against toward_centre(1_450, 200);
  (b) after the None case; (c) after "not past the outer since"; (d) the const bounds after `use super::*;`. The unit
  test's doc gained one clause each for the kept state, the outer radius and TE-B1-3 (doc only).
- FP-3: written, not yet compiled: `brawl1_the_building_figure_counts_a_soldier_in_a_building_in_measured_ticks_only`
  at the end of logs.rs's tests (Placement imported; `Some(dir.as_path())` for Logs::open, the same path as the
  ruling's `Some(&dir)`); EA-B1-4's words at logs.rs:6-7, the paragraph rewrapped from line 4 (it held a short line).
- FP-4 (a)-(d): written, not yet compiled. (a) the tick-105 sentinel case in brawl1_own_soldier_... (its doc names
  it); (b) `brawl1_own_soldier_compares_ticks_by_the_endpoint_s_wrap_rule` (also asserts each tick filed); (c) and (d)
  the ruled doc words in client lib.rs. (d) moved the client constants to lib.rs:250-251, so soldiers()'s ruled sentence
  names "client lib.rs:250-251" in place of the ruling's "247-248" (the pre-edit lines): one deliberate deviation,
  flagged in the report.
- FP-6 (a)-(d): written, not yet compiled. (a) Server::new private (one caller, scenario.rs:390); (b) build's doc
  rewrapped, words unchanged; (c) both comments reworded as ruled, the two B1 cases after the after-max case (with a
  two-line comment naming RR-B1-4 and the arithmetic); (d) hash.rs 359,993 / 29,999 with the ruled comment.
- 06:49 UTC: every FP-1 to FP-6 edit written; numstat 1,084 added, 127 deleted, 1,211 changed (under 1,400). No new
  line of mine passes 120 characters (one rewrapped).
- FP-5 (a)-(c): written (bots lib.rs:57-62, :128-130, the leash doc). Note: the ruled "x² + y² stays below 2^31" is
  exact but for one point: (-32,768, -32,768) gives 2^31 itself (POSITION_XY min -32,768, sim grid.rs:106-113); no
  overflow either way. Written as ruled; flagged in the report.
- (The line "FP-6 (a)-(d): pending" that stood here was stale: FP-6 is written, as above. Corrected at 07:14 UTC.)
- fp-check-1 (.gate/u4r/brawl1/fp-check-1.log, -x -k, 06:49:41-06:52:01 UTC, free 6,563 MiB at start):
  - fmt --check exit 1: 10 hunks (bots lib.rs x4, client test x1, logs.rs x4, scenario.rs x1), applied by Edit after.
  - clippy --workspace --all-targets -D warnings: exit 0 in both builds (the const bounds, Server::new private and
    `counts.clone()` raise nothing).
  - bots `tests::brawl1`: 3 passed, 1 ignored, in both builds (10.84 s; 10.74 s). Short copy's prints equal round 1's
    (EveryTick 1575 cm slot 27 tick 65928 at 550, 1476; Skips 1629 cm slot 39 tick 63768 at -966, -1312, 53039 of
    172752 chances, 307 per 1000, streak 6; in a building 0; stuck none); new: dead 1190 (EveryTick) and 1334 (Skips)
    soldier-ticks, so the `r.dead > 0` assert stays in the short copy.
  - client `brawl1`: 2 passed in both builds.
  - lab `logs::tests::brawl1 scenario::tests::fp3_ hash::tests::r10_`: exit 101 in both builds, 4 (5) passed and the
    figure test FAILED at scenario.rs:1707 (settings()'s parse): "--logs 0 is for a recording run (--record): a run
    keeps its logs". The ruled `settings(&dir, "--profile P0 --seed 1 --logs 0")` cannot parse. Fix: that one line adds
    `--record <scratch>/brawl1.rec` (parse only stores the path; nothing writes it), with a comment saying why: the
    smallest line that gives a no-log Settings. A deviation from the ruled words, flagged in the report. fp3 and
    r10_carry_82 passed in both builds.
- 06:53:13 UTC: the session's spend limit cut copy 2 while the fmt hunks were being applied (the last Edit, scenario.rs,
  at 06:53:12). Resumed 07:13 UTC: no plant in place (checked in the worktree: `fn leash` starts with its let-else,
  logs.rs's filter and own_soldier's Hash filter and `.reduce` intact); numstat 1,139 added, 127 deleted, 1,266
  changed; free 6,531 MiB. Next: fmt --check again to see which hunks remain, then the check run again.
- fp-fmt-2.log (07:14:10 UTC; header diff c2561b6ca54c8cde994fcdad5954c63d332df73f2db038c12aba3d71a0daa49f): fmt --check
  exit 0, so every hunk had landed (the scenario.rs Edit at 06:53:12 included); nothing left to apply.
- fp-check-2.log (07:14:18-07:15:09 UTC, -x -k, the same diff hash, every command exit 0): fmt clean; clippy -D warnings
  clean in both builds; bots `tests::brawl1` 3 passed, 1 ignored, in both builds (10.87 s; 10.82 s; the short copy's
  prints unchanged from fp-check-1); client `brawl1` 2 passed in both builds; lab: the figure test, fp3,
  r10_carry_82 (both), r10_fp3 passed in both builds (5 and 6 passed: the bites build also matches r10_carry_83_d).
  **FP-1 to FP-6 compiled and pass. Next: FP-7.**
- FP-7 P1, TE-B1-1 (the leash off): done, restored. bots lib.rs sha256 before
  83a8541bbdb924bc4dec032330a6117d54d960cff0a01b8544bc1d7c0c0b6069; planted `return walk;` as the first line of `fn
  leash`'s body (lib.rs:255), sha256 planted 77be9b00ea9e4e014960173248567af4f2600e724952bd3fa7da17f582f0c377; restored by
  Edit (the line deleted) at 07:16:51 UTC, sha256 after 83a8541bbdb924bc4dec032330a6117d54d960cff0a01b8544bc1d7c0c0b6069
  (equal). Log .gate/u4r/brawl1/fp-plant-1.log (header diff 4f0a5624c748a893417e662cabdd50bbb2ec984a25cf7c964f10f4c65b5f6b2b,
  -x -k; the compiler warned "unreachable statement" and "unused variable: `own`", as a plant does).
  - (1) short copy, measured build: exit 101, "assertion `left == right` failed: Skips: first Some((64395, 11, 1))"
    (lib.rs:700, the area assert), 11.30 s.
  - (2) short copy, bites build: exit 101, the same message, 11.93 s.
  - (3) full test, release: exit 101 (lib.rs:750), "in a building: [(1, EveryTick, 11335, Some((70723, 16, 3))), (1,
    Skips, 68964, Some((64395, 11, 1))), (2, EveryTick, 9032, Some((69788, 33, 6))), (2, Skips, 95212, Some((63787, 33,
    5))), (3, Skips, 62107, Some((63950, 19, 3)))]", 24.05 s.
  - Per seed and pattern (in_building soldier-ticks; first (tick, slot, area); largest distance cm (slot, tick, x, y)):
    - short, seed 1, both builds alike: EveryTick 0; None; 2,175 (6, 65656, 2056, -710). Skips 7,728; (64395, 11, 1);
      3,813 (8, 67219, 550, 3774).
    - full seed 1: EveryTick 11,335; (70723, 16, 3); 2,205 (16, 70816, -1098, 1913). Skips 68,964; (64395, 11, 1);
      6,161 (6, 79418, 5876, 1853). Bursts 48,302; (64978, 36, 4); 2,784 (17, 75838, -554, 2729).
    - full seed 2: EveryTick 9,032; (69788, 33, 6); 2,328 (19, 73152, -1328, 1913). Skips 95,212; (63787, 33, 5);
      9,078 (21, 82018, -1258, 8991). Bursts 37,865; (64974, 29, 2); 3,265 (29, 81180, 2478, 2127).
    - full seed 3: EveryTick 0; None; 2,175 (0, 70156, 2056, -710). Skips 62,107; (63950, 19, 3); 7,432 (19, 80451,
      4374, 6009). Bursts 6,265; (75771, 29, 2); 2,895 (2, 77588, 2666, -1130).
  - The bite rule holds on area: the short copy fails in both builds and the full test lists Skips for all three seeds
    (EveryTick too for seeds 1 and 2). **The door-entry bar was not needed, so it was not added.** EveryTick alone
    shows the closed-circle reading the ruling expected: in a building 0 for seed 3 (and the short copy's seed 1), at
    2,175 cm, because without skips the open-loop circle closes.
- FP-7 P2, TE-B1-2 = RR-B1-1 (the figure cut): done, restored. logs.rs sha256 before
  83008fce50552257ade01f112cfcfb37a18da3ed5a744080937442df50ab229b; planted `.filter(|_| false)` for room_tick's building
  filter (logs.rs:588), sha256 planted b902a7ee16c00bf52105dc993aab63608cbb501f29a85941c5d121cda1f19f68; restored by Edit
  at 07:17:34 UTC, sha256 after 83008fce50552257ade01f112cfcfb37a18da3ed5a744080937442df50ab229b (equal). Log
  .gate/u4r/brawl1/fp-plant-2.log (header diff 59026680de997f2315a723bf40587f94b1d850c51fea517d1dc70a13bfc5a3a6, -x -k):
  `cargo test --locked -p npt-lab --bin npt-lab -- logs::tests::brawl1_the_building_figure_counts_a_soldier_in_a_building_in_measured_ticks_only
  --exact --nocapture` exit 101, panicked at logs.rs:924, "assertion `left == right` failed: measured: one slot-tick /
  left: 0 / right: 1". It bites as ruled: the warm-up asserts before it (count 0, no slot) assert absence, so they hold
  under any plant that removes the figure; the first assert that can fail, the measured count, fails.
- FP-7 P3, TE-B1-4 (a) (the Hash filter): done, restored. client lib.rs sha256 before
  2aca00c87822526a891de6d1c5e78ce4161f37de2de453dd01c94e28500e2d1c; planted: ` && matches!(t.filed, Filed::Hash(_))`
  removed from own_soldier's filter (lib.rs:359), sha256 planted 7a77c71aaeb9772cf79cbfeab53ab14a441169655c7bc2040185d66b18a4530a;
  restored by Edit at 07:18:07 UTC, sha256 after 2aca00c87822526a891de6d1c5e78ce4161f37de2de453dd01c94e28500e2d1c (equal).
  Log .gate/u4r/brawl1/fp-plant-3.log (header diff 683b28fa1d08ea537b0efa2a0268f4d65146dc28fd5d771866b3198385001705):
  `cargo test --locked -p npt-client --test endpoint -- brawl1 --nocapture` exit 101: the wrap test ok; the read test
  panicked at endpoint.rs:446, "assertion `left == right` failed: a complete tick turned sentinel is not read: tick 101
  again / left: Some(([-700, 700, 150], false)) / right: Some(([1234, -1234, 150], true))". Bites as ruled.
- FP-7 P4, TE-B1-4 (b) = NA-B1-1 = RR-B1-5 = EA-B1-3 (b) (the wrap rule): done, restored. client lib.rs sha256 before
  2aca00c87822526a891de6d1c5e78ce4161f37de2de453dd01c94e28500e2d1c; planted: own_soldier's `.reduce(|a, b| { if
  self.wrap.after32("tick", a.tick, b.tick) { b } else { a } })?` (lib.rs:360-366, seven lines) replaced by
  `.max_by_key(|t| t.tick)?`, sha256 planted 6aeeec0a0c6dee9ab393bd03683a6163e9e806ea607b5f0e17d17ce7ae632151; restored by
  Edit at 07:18:37 UTC, sha256 after 2aca00c87822526a891de6d1c5e78ce4161f37de2de453dd01c94e28500e2d1c (equal). Log
  .gate/u4r/brawl1/fp-plant-4.log (header diff fbea0f371cb5b26d96ca68dbacdf30db74572a3b392a848e515546eaafc5e8b5): `cargo
  test --locked -p npt-client --test endpoint -- brawl1 --nocapture` exit 101: the read test ok; the wrap test panicked
  at endpoint.rs:483, "assertion `left == right` failed: tick 0 comes after u32::MAX by the endpoint's wrap rule / left:
  Some(([11, -11, 150], false)) / right: Some(([22, -22, 150], false))", its last assert. Bites as ruled.
- **FP-7 complete, no plant in place:** bots lib.rs 83a8541b..., logs.rs 83008fce..., client lib.rs 2aca00c8... (each
  equal to its pre-plant value), and the tree's `git diff HEAD -- . | sha256sum` is c2561b6c... again (fp-check-2's).
  Next: FP-8.
- FP-8 (1) started 07:19 UTC: `node check.mjs` through the runner (-x), log .gate/u4r/brawl1/final-fp-check.log, on the
  final code (diff c2561b6c...). If cut mid-run: the log's end line says whether it finished; a run cut short is not a
  result and is run again whole (not a retry of a failure).

### FP-9: report words (no code)

1. **The plants:** P1 to P4 are recorded above (FP-7), each with its three sha256 values, log, exit codes and failing
   message. All four bit as ruled. **The door-entry bar was not added,** because P1's bite rule held on area alone: the
   short copy failed in both builds and the full test listed Skips for all three seeds (EveryTick too for seeds 1
   and 2). The ruling adds the bar only "if the bite rule does not hold on area".
2. **Pattern (iii), a correction to copy 1's report:** I withdraw "a fixed radius stops it" and the 14 m vehicle
   candidate. sim_run runs with stand-ins off (bots lib.rs:541; the ruling's :496 before this pass's lines). No solid
   lies within 1.6 m of (1734, -592), (1671, -751) or (1586, -917): building 0's corner (1770, -430) is 166 cm from the
   first point (I re-computed: (36, 162), 165.95 cm), the crates lie within 950 cm of the centre, and the wall stand-in
   is at y 1,180-1,220. The reading for U4g is the advisor's U4g words (advisor-brawl1-r1-full.md, "U4g's brief").
3. **TE-B1-8, in the ruling's words:** "No test drives Behaviour::step with an own soldier past the outer radius. The
   glue at bots lib.rs:223 is shown only by the moved pins (own_soldier returns Some and changes commands) and by the
   two 10-minute evidence runs (condition 14)." (That glue line is lib.rs:228 after FP-5's doc lines.)
4. **The advisor's corrections, in its words** (beside ADV-BP-1 and ADV-BP-2, condition 18):
   - ADV-BP-1: "N2-1's 'all four checks fail' was wrong; under the leave plant two fail (Q5)."
   - ADV-BP-2: "my sets-r2 figure of 'about 4.2 GiB free after the refill' mixed units."
   - ADV-BP-3: "Condition 6 (ii)'s identical-diff test could not hold at <base>, because the wallclock bite's L1 hash
     varies with wall-clock ns (pump.rs:236-243); and 'comes to me whole' did not say stop." With the masking rule from
     the points ruling's Point 2: "Future captures of these tests mask exactly the L1 leg's hash values in the
     wallclock bite's comparer and result lines ('theirs ... hash' and the L1 hex). They keep the index, the loopback
     hash, compared and mismatching, and they explain libtest's 'filtered out' count by the binary's test count.
     Nothing else is masked."
   - ADV-BP-4: "RR4-L1's ruled name MAX_ENTRY_TICKS falls under check.mjs's TICK_NAMED rule and is refused as a
     literal; I did not check it; the name is MAX_ENTRY_TICK_COUNT."
5. **RR4-L1's constant:** MAX_ENTRY_TICK_COUNT stands (points ruling, Point 1). RR-B1-6 and EA-B1-5 are answered
   there, and ADV-PT-L3 carries the point to the check.mjs follow-up. No change in this pass.
6. **The follow-ups, with their homes** (the ruling's "Each finding"):
   - RR-B1-7 (1), the `s as u8` loop rewrite in room_tick's building block: the next lane that edits logs.rs's
     room_tick.
   - EA-B1-1's pub constants (npt-sim exposes the flags index and the dead bit), with NA-B1-3's npt-client half: the
     next lane that opens crates/sim (beside EA-U4R-4), U6 at the latest.
   - ADV-PT-L3 (TICK_NAMED gains `_TICK_COUNT$`, NOT_TICK_COUNTS gains MAX_ENTRY_TICK_COUNT, one check.test.mjs case):
     the check.mjs follow-up, at check.mjs's next change, before PR #40 is marked ready at the latest.
   - TE-B1-8's optional glue test (Behaviour::step with an own soldier past the outer radius): U6's bots'-view item
     (EA-U4R-5's home).
- FP-8 (1)-(6): pending.
- FP-9 items 1-6: pending.

