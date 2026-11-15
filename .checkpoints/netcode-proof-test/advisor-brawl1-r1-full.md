# Advisor: the ruling on BRAWL-1's review round 1 (workflow wf_59e64b31-fa2), 2026-11-15

Written by script, word for word, from the workflow's journal (advisor-r1's StructuredOutput result). The round: brawl1-review-r1-full.md. Its notes: advisor-brawl1-r1.md. Nothing below is the main session's.

**Snapshot:** wip/brawl1.patch, sha256 ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7. I checked that the read copy (/home/user/Shamble-Frontlines/.claude/worktrees/brawl1, HEAD 352a245, 10 files modified, nothing new) gives the same value for `git diff HEAD | sha256sum`. numstat: 875 inserted and 123 deleted, 998 changed lines. I checked every finding against the code in the read copy and against the builder's logs (.gate/u4r/brawl1/). I ran no cargo or check.mjs command. The plants need edits I may not make, and the reviewers had already re-run the tests at this snapshot. I diffed their full-test prints (.gate/u4r/brawl1-review-test/t2-full-release.log, .gate/u4r/brawl1-review-netcode/run1.log) against final-regen-tests.log: they are identical. Disk at my check: 6,566 MiB free. The shared target holds 6,442 MiB: debug 6,008 (npt-* deps 1,026 and incremental 3,171 of it), release 250, doc 15, wasm32 172. My notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-brawl1-r1.md. I wrote no logs, because I ran nothing.

## Summary

All three Mediums are confirmed, and one fix pass closes them. TE-B1-1: no run shows that the sim-only test can fail. So a leash-off plant is run and recorded, and the test now asserts that it sees 48 Soldier records. If the plant does not bite on area, a door-entry distance bar (1,800 cm) is added and must bite. TE-B1-2 and RR-B1-1 are the same gap, and one fix closes both. A logs.rs test teleports a soldier into building 0 and checks three things: the building figure reads 1 in a measured tick; the warm-up tick writes the pair but does not count it; the summary line prints 'in a building: 1 slot-ticks (slots 3)'. A plant shows that this test fails if the figure is cut. A bots test shows that npt-sim gives area 1 there too. Of the 20 Lows, 16 ride this push. All of them are tests or doc words, and none changes behaviour. RR-B1-7 and EA-B1-1 ride only in part; the rest of each becomes a follow-up. Two are recorded as report words, and two need no action because the points ruling already settled them. Of my points ruling's findings: ADV-PT-L1 and ADV-PT-L2 ride, ADV-PT-L3 stays a follow-up, ADV-PT-M1 goes into U4g's brief merged with NA-B1-2 (c) and TE-B1-5, and ADV-BP-3 and ADV-BP-4 go in the report. One correction to a reviewer: TE-B1-5's 'obstacle at 18.3 m / the 14 m vehicle' is wrong. The sim-only test runs with stand-ins off, and no solid lies within 1.6 m of the three maxima. On the recording path no fix changes library behaviour, so the three new pins must stay. The end checks prove it, and the full test's round-1 figures must come out equal. Round 2: test-engineer and rust-reviewer, in one batch. The merge-step clean may come right before the counted run, but only as the approved part-cleans (B) and (C). The folder-level `cargo clean --target-dir .gate/shared-target` is not approved. Expected size after the fix pass: about 1,270 lines, under 1,400. Nothing for Mustafa.

## The fix pass, in order

### Step 1: 1. THE COPY AND ITS RULES (the main session sends this with the steps below)

- **Findings:** TE-B1-1, TE-B1-2, RR-B1-1
- **Words:** A fresh builder copy (copy 2): general-purpose, opus, effort xhigh. It works in the same worktree (/home/user/Shamble-Frontlines/.claude/worktrees/brawl1), with no isolation. Its prompt is copy 1's launch prompt plus a resume part. Before any edit it checks that `git diff HEAD | sha256sum` gives ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7. If it does not, the copy stops and tells the main session. It reads brawl1-build.md and the diff first, and adds a new section 'Fix pass, round 1' to brawl1-build.md as it goes. The rules of brawl1-brief.md section 4 hold throughout:
- Every change is made with the Edit tool. rustfmt's diffs are applied by Edit; never `cargo fmt` writing files.
- Every run goes through u4r-run.sh with -x, df first, and 4 GiB free before any release or wasm32 build. A floor stop is reported and never cleaned around.
- Each plant is restored by Edit, and the file's sha256 is then checked equal to its pre-plant value.
- Counts are deltas from 535/613, and each new test is named with its file.
- A check.mjs test-step failure is recorded whole (TE2-5) and never retried.
- Any SHA-256 mismatch in det-u4.mjs or the precheck regeneration: stop. No re-pin and no retry; the log comes to me whole, with the first differing tick.
- Any change to a figure the full sim-only test printed in round 1 (FP-8 (4)): stop, and it comes to me whole.
- SIZE-CHECK: past 1,400 changed lines by numstat against 352a245 (SERIES.md included), stop. It goes to Mustafa in three lines.
- No commit.
The Lows ride this push because the Mediums need it anyway; a Low never triggers a push on its own. At the end the main session cuts a new snapshot by script, and round 2 reviews its diff against ee2973aa.
- **Evidence:** From FP-0 on, each runner log's header carries the diff hash at that moment (a log run before any edit shows ee2973aa...). The notes section in brawl1-build.md. Logs under .gate/u4r/brawl1/ named fp-*.

### Step 2: 2. FP-0 (the main session, before copy 2 starts; this ruling approves the runner edit, outside the change set, as FP-0 in U4r's round 1)

- **Findings:** TE-B1-7
- **Words:** In /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-run.sh, inside the header block (:64-67), after the `echo "pwd $(pwd); free ...; HEAD ..."` line (:66), add:
`echo "diff $(git diff HEAD -- . | sha256sum | cut -d' ' -f1); untracked $(git status --porcelain --untracked-files=all -- . | sha256sum | cut -d' ' -f1)"`
In main-run.sh, add the same line after :41. Its working folder is the main folder's spikes/netcode-proof, set at :26.
Rules: Edit tool only. Save each change as a .diff beside its runner (u4r-run.sh.te-b1-7.diff, main-run.sh.te-b1-7.diff), in INF2-4's form. `bash -n` on each runner must print nothing. No other runner line changes.
Why: today every log records only 'HEAD 352a245' while the change is uncommitted. Run from spikes/netcode-proof, `git diff HEAD -- .` equals the whole `git diff HEAD`, because every change lies under that folder. So the line gives the snapshot's own sha256. On a clean committed tree, both hashes are the sha256 of empty input.
- **Evidence:** The two .diff files. Empty `bash -n` output. Copy 2's first log shows the diff line. At M3, the counted run's header should show e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 for both diff and untracked.

### Step 3: 3. FP-1: the sim-only test (bots/src/lib.rs, the #[cfg(test)] module only)

- **Findings:** TE-B1-1, TE-B1-2, TE-B1-6, NA-B1-3, RR-B1-3, EA-B1-1, ADV-PT-L1
- **Words:** (a) TE-B1-1's record check: after `let now = soldiers(&world);` (bots lib.rs:519), add `assert_eq!(now.len(), n, "tick {t}: one Soldier record per slot");`. The World writes one record per soldier slot, dead or alive (npt-sim world.rs:1816-1818).

(b) The dead-bit tie: one fix for TE-B1-6, NA-B1-3, RR-B1-3 and the part of EA-B1-1 that is done now.
- In `soldiers()` (:474-488), after `let dead = r.grid[5] & (1 << 3) != 0;`, add `assert_eq!(dead, r.grid[6] == 0, "slot {}: flags (field 5) bit 3 is set exactly when health (field 6) is 0", r.index);`. npt-sim sets flags = FLAG_DEAD and health 0 together at a death (world.rs:1630-1634) and clears both at a revive (:1176-1188).
- Record (:459-468) gains `dead: u64`: the soldier-ticks seen dead, counted in sim_run's loop over `now` (bind the dead flag at :520). printed() prints it.
- The short copy (:636-640) asserts `r.dead > 0` for each pattern, with the message "the dead-bit tie is exercised". If a pattern at seed 1 shows none, that assert moves to the full test (any seed of that pattern), and the notes say why.
- soldiers()'s doc adds: 'This pins the literals npt-client copies (client lib.rs:247-248, SOLDIER_FLAGS and SOLDIER_DEAD) against npt-sim's real deaths.'
TE-B1-6's separate kill test is not needed: this tie runs on every record of every tick, with the brawl's real deaths.

(c) TE-B1-2's npt-sim half: a new #[test] in the same module, `brawl1_a_soldier_teleported_into_building_0_reads_area_1`.
- It builds `World::new(&Scene::stress().with_stand_ins(StandIns::OFF), 1, FIRST)`.
- It takes one step with `TickInput { tick: FIRST, teleports: vec![Placement { slot: 0, position: [2_200, 0, 0] }], ..TickInput::default() }` and `HitHistory::for_ms(2_000)`.
- It asserts that `soldiers(&world)` gives slot 0 area 1 and every other slot area 0.
The point is building 0's centre (greybox.rs:92). Its footprint is half-open, x 1,800 to 2,600 (:103-104, :176-183). A teleport moves only a live soldier (world.rs:1147-1151), and all soldiers start alive (:745). Import Placement.

(d) ADV-PT-L1: printed() (:610-631) prints the chances and the per-1000 rate only for Pattern::Skips. EveryTick prints 'skipped {n}' alone, and Bursts prints 'skipped {n} in its bursts'. The numbers come from the Record, with no literal. The longest streak stays for all three patterns.
- **Evidence:** fmt and clippy clean in both builds (check.mjs). The new test passes. The short copy passes in both builds; record each build's time (at most 60 s, condition 13) and its dead figure, which must be above 0. The full test's prints: FP-8 (4).

### Step 4: 4. FP-2: the steering unit test

- **Findings:** TE-B1-3
- **Words:** In brawl1_the_leash_steers_past_its_outer_radius_until_inside_its_inner_one (bots lib.rs:346-432). I re-computed the command sequence: slot 5 at seed 1 has walk phase 7.

(a) Replace the blocked loop (:365-371) with:
`let blocked = toward_centre(1_450, 200); for _ in 0..3 { let (got, walk) = give(at(1_450, 200)); assert_ne!(walk, blocked, "the twin's walk differs, so the check can fail"); assert_eq!(got, blocked, "blocked: the position does not move"); }`
toward_centre(1,450, 200) = [-125, -17], which is in no MOVES entry. Today's asserts sit on commands 4 to 6, whose walk is [-127, 0]: the steer itself, so they cannot fail.

(b) Right after the None case (:389-390), insert:
`let (got, walk) = give(at(1_350, 0)); assert_ne!(walk, steer, "the twin's walk differs, so the check can fail"); assert_eq!(got, steer, "None and dead keep the leash's state");`
This is command 12, whose walk is [-89, -89].

(c) Right after the case 'between the radii, not past the outer since' (:393-394), insert:
`let (got, walk) = give(at(1_400, 0)); assert_ne!(walk, steer, "the twin's walk differs, so the check can fail"); assert_eq!(got, walk, "on the outer radius: not past it"); let (got, walk) = give(at(1_401, 0)); assert_ne!(walk, steer, "the twin's walk differs, so the check can fail"); assert_eq!(got, steer, "1 cm past the outer radius");`
These are commands 15 and 16, whose walks are [-89, -89] and [-49, -117]. The diagonal case after them still steers to [-89, 89].

(d) At the top of the tests module: `const _: () = assert!(LEASH_IN_CM >= 1_300 && LEASH_OUT_CM >= LEASH_IN_CM + 100 && LEASH_OUT_CM <= 1_500, "the advisor's plan review, condition 1's bounds");`.
- **Evidence:** The unit test passes. Its count does not change: it is the same test.

### Step 5: 5. FP-3: the lab figure's positive control (logs.rs; TE-B1-2 and RR-B1-1 name the same gap and are fixed once)

- **Findings:** TE-B1-2, RR-B1-1, EA-B1-4
- **Words:** In logs.rs's #[cfg(test)] module (:775 on), add `brawl1_the_building_figure_counts_a_soldier_in_a_building_in_measured_ticks_only`:
1. `let scene = Scene::stress().with_stand_ins(StandIns::OFF); let first = 63_736u32; let mut server = crate::scenario::Server::build(&scene, 1, first, 48, 0, 0, &crate::scenario::Bites::default());`
2. `server.room.script(&[], &[Placement { slot: 3, position: [2_200, 0, 0] }], &[]); server.room.tick();` This puts soldier 3 in building 0 (area 1). With no command it stays there.
3. Take a folder from `crate::scenario::tests::scratch("brawl1-figure")` and make it with `std::fs::create_dir_all`; Logs::open's File::create_new needs the folder to exist (logs.rs:77-88). Then `let s = crate::scenario::tests::settings(&dir, "--profile P0 --seed 1"); let mut logs = Logs::open(Some(&dir), &s.meta, first, false)`.
4. `logs.room_tick(&server.room, server.room.last_tick(), Phase::Warmup, &[], &[], &[])`. Assert that `logs.counts.in_building_measured == 0` and `logs.counts.in_building_slots.is_empty()`.
5. `server.room.tick();` then `logs.room_tick(&server.room, server.room.last_tick(), Phase::Measured, &[], &[], &[])`. Assert that `in_building_measured == 1` and `in_building_slots == BTreeSet::from([3])`. Two room ticks are needed: npt-log's writer refuses a tick record at or before the last one (log lib.rs:467-474). So TE-B1-2's 'the same tick in both phases' would fail on the writer, not on the figure.
6. `let counts = logs.finish(Ended::Clean, &[])`. Read room.jsonl with `crate::scenario::tests::read(&dir, "room")`. Both tick records hold `in_building_slot_area` = [3, 1], the warm-up one too (condition 10: written in any phase).
7. `summary_line(&s, &Summary { counts: counts.clone(), ..Summary::default() })` ends with "in a building: 1 slot-ticks (slots 3)". With `settings(&dir, "--profile P0 --seed 1 --logs 0")`, it ends with "in a building: not counted (--logs 0)".
The test holds the Scratch guard until its last assert (T-5's rule). The figure's code (:584-599) does not change in this pass (RR-B1-7 (1) is deferred).

EA-B1-4: replace logs.rs:6-7's 'Every tick and second record carries all its counters, 0 included; every log ends with its footer.' with 'Every tick and second record carries all its counters, 0 included (BRAWL-1's `in_building_slot_area`, a list, is written only on ticks with one: the advisor's plan review, condition 10); every log ends with its footer.'
- **Evidence:** The new test passes in both builds and is named in check.mjs's counts. Plant P2 (FP-7) makes it fail at its first figure assert.

### Step 6: 6. FP-4: the client read item's tests and docs

- **Findings:** TE-B1-4, NA-B1-1, RR-B1-5, EA-B1-3, RR-B1-3
- **Words:** (a) TE-B1-4 (a): in brawl1_own_soldier_is_the_newest_complete_filed_tick_s_own_record (client/tests/endpoint.rs:397-442), after the tick 105 assert (:435) and before tick 106, add:
`let out = feed(&mut e, vec![datagram(own_only(105, 1, vec![soldier(0, -701)]))]); assert_eq!(out.hashes, [(105, 0)], "a second, different group 0 turns tick 105 into the sentinel (carry 61 (b))"); assert_eq!(e.own_soldier(), first, "a complete tick turned sentinel is not read: tick 101 again");`
The group handler passes a different group to sentinel(), which sets only `filed` (client lib.rs:626-637, :725-731). Tick 101 is then the newest complete tick filed Hash still in the ring. The test's doc line names this case.

(b) One wrap case, for TE-B1-4 (b), NA-B1-1, RR-B1-5 and EA-B1-3 (b): a new #[test] `brawl1_own_soldier_compares_ticks_by_the_endpoint_s_wrap_rule`, in NA-B1-1's form.
- A fresh endpoint; JoinAccepted for slot 0 at tick u32::MAX - 1.
- A complete own_only(u32::MAX, 1, vec![soldier(0, 11)]). Assert own_soldier = Some(([11, -11, 150], false)).
- Then a complete own_only(0, 1, vec![soldier(0, 22)]). Assert out.hashes.len() == 1, and own_soldier = Some(([22, -22, 150], false)) with the message 'tick 0 comes after u32::MAX by the endpoint's wrap rule'.
With ACK_RING_TICKS at 32, tick u32::MAX sits in ring slot 31 and tick 0 in slot 0. So this one case kills both mutants: a plain maximum and a 'last matching ring entry' read both return tick u32::MAX's record. TE-B1-4 (b)'s 127/128 case is then not needed.

(c) EA-B1-3 (a): in own_soldier's doc (client lib.rs:347-348), replace 'A newer tick that is incomplete, refused or the sentinel changes nothing.' with 'A newer tick that is incomplete, refused or the sentinel changes nothing while the complete tick stays in the ring (ACK_RING_TICKS ticks); a newer tick that takes its ring slot drops it, and then the newest complete tick still held is read, else `None`.'

(d) RR-B1-3's doc line: at the end of the doc above SOLDIER_FLAGS (client lib.rs:244-246), add: 'npt-bots' sim-only test pins the same two literals against npt-sim's real deaths (its `soldiers()`: the bit set exactly when health is 0), so a change there fails that test; both copies change together until npt-sim exposes them (BRAWL-1 round 1, EA-B1-1).'
- **Evidence:** Both client tests pass, and the wrap test is named in the counts. Plants P3 and P4 (FP-7) make each test fail at its new assert.

### Step 7: 7. FP-5: doc words in npt-bots' library (no code)

- **Findings:** NA-B1-2, RR-B1-2
- **Words:** (a) NA-B1-2 (a), bots lib.rs:57-59: replace '..., 4 m beyond it.' with '..., 4 m beyond it as geometry. A soldier overshoots the outer radius, because the leash reads a decoded position some steps old and a skipped tick re-runs the last command: at BRAWL-1's review the sim-only test's skip pattern reached 1,720 cm and B1's bursts 1,832 cm, past the door entry (it prints the largest distance per pattern; BRAWL-1 round 1, NA-B1-2).'

(b) NA-B1-2 (b), :245-246: replace 'so a re-run command or a step cut short is corrected at the next command' with 'so a re-run command or a step cut short is corrected once the decoded position shows it (lead - 1 steps later, plus the ticks between the newest and the newest complete tick)'.

(c) RR-B1-2, by its documented alternative, with no code change:
- toward_centre's doc (:125-127) adds: 'Its inputs are decoded grid positions (16 bits on the wire: |x|, |y| at most 32,768, npt-proto quant.rs:197-215), so x² + y² stays below 2^31 and the i64 arithmetic cannot overflow; it is not for other inputs.'
- leash's doc adds: '(x and y in toward_centre's domain)'.
The u64 rewrite is not taken. It would edit library code on the recording path for an input that cannot arrive.
- **Evidence:** fmt clean. The full test's figures are unchanged (FP-8 (4)) and the pins are reproduced (FP-8 (2), (3)). Together these show the library behaves as before.

### Step 8: 8. FP-6: small items in scenario.rs and hash.rs

- **Findings:** EA-B1-2, RR-B1-7, RR-B1-4, ADV-PT-L2
- **Words:** (a) EA-B1-2: at scenario.rs:397, `pub(crate) fn new(room: Room<Sim>, quant: Quant, rtt: RttSource) -> Server {` becomes `fn new(room: Room<Sim>, quant: Quant, rtt: RttSource) -> Server {`. Its one caller is Server::build (:390); I grepped crates/ and check.mjs.

(b) RR-B1-7 (2): rewrap Server::build's doc (:368-372) so that no line holds a lone fragment: '... behind one decoder per session on `bites`' `Quant`; `bites` plants its World bite (armed at `arm_tick`), its room bite and its pump bite. `run_leg` passes ...'. The words do not change.

(c) RR-B1-4. The code order is kept, because a line that breaks both bounds is refused either way.
- Reword :1433-1434 to: '// advisor-u4r-r2, RR4-L2, after the run-length 64-bit checks (which keep their messages): the run's last tick, counted from its first (the warm-up, measured and after phases' ticks), fits 32 bits, on every profile. The burst block's own 64-bit check comes below it, then its 32-bit cap.'
- Reword :2576-2577 to: '// u32::MAX is odd, so never a whole-second length; the largest below it (4,294,967,280 ticks) accepted.'
- After the after-max accepted case (:2584-2587), add two cases. First: `assert!(parsed("B1", "--bursts 2 --burst-first-s 0 --burst-every-s 143165566").is_ok(), "the last burst at 4,294,967,280 ticks");`. Second: `parsed("B1", "--bursts 2 --burst-first-s 0 --burst-every-s 143165567")` is refused with exactly '--bursts 2, --burst-first-s 0 and --burst-every-s 143165567: the last burst's tick, 4294967310 ticks from the run's first, is above 4294967295 (32 bits)'.
The arithmetic: (1 x 143,165,566 + 0) x 30 + 300 = 4,294,967,280, and burst_starts has no in-run check. fp3's doc (:2508) then stays true.

(d) ADV-PT-L2, at hash.rs:1560-1562:
- '360000' becomes '359993' in the replace.
- `let at = 360_000 / 12;` becomes `let at = 359_993 / 12;` (29,999).
- The comment reads: 'At the rule's boundary below RR4-L1's 36,000-tick cap (advisor-u4r-r2): 359,993 bytes hold at most 29,999 ticks; not a multiple of 12, so the rule's rounding down is checked (ADV-PT-L2).'
- **Evidence:** fp3 and r10_carry_82 pass in both builds. Clippy is clean: narrowing Server::new raises no dead-code or private-link warning.

### Step 9: 9. FP-7: the plants

- **Findings:** TE-B1-1, TE-B1-2, RR-B1-1, TE-B1-4, NA-B1-1, RR-B1-5, EA-B1-3
- **Words:** Each plant: the file's sha256 before, the plant by Edit, the sha256 planted, the runs, the restore by Edit, and the sha256 again, equal to before. All runs go through u4r-run.sh -x -k, so every command runs even after one fails.

P1, TE-B1-1 (the leash off): make `return walk;` the first line of `fn leash`'s body (bots lib.rs:249). Run in one call, in final-regen-tests.log's forms (:27, :48, :69):
(1) `cargo test --locked -p npt-bots --lib -- tests::brawl1_sim_short_no_soldier_enters_a_building --exact --nocapture`
(2) the same with `--all-features`
(3) `cargo test --locked --release -p npt-bots --lib -- tests::brawl1_sim_full_no_soldier_enters_a_building_seeds_1_to_3 --ignored --exact --nocapture`
The bite rule: the short copy fails in both builds, and the full test fails with Skips listed for at least one seed. Record EveryTick's figures as they come. If it never bites, the record says so: it is the closed-circle baseline.
If the bite rule does not hold on area: add the door-entry bar below to both barred patterns of both tests, and run the same call under the plant again. The rule must then hold. If it still does not, that item stops and its record comes to me whole; the rest of the pass goes on.
The bar:
- In the tests module: `/// The nearest door entry, cm from the centre: a soldier's centre enters building 0 at x 1,800 (centre x 2,200 minus ROOM 400) with |y| < 60 (npt-sim greybox.rs:64-70, :92-99). const DOOR_ENTRY_CM: i64 = 1_800;`
- In the short copy, after the area assert: `assert!(r.far.0 < DOOR_ENTRY_CM.pow(2), "{pattern:?}: largest distance {} cm, at or past the door entry", r.far.0.isqrt());`
- In the full test, a barred pattern with `r.far.0 >= DOOR_ENTRY_CM.pow(2)` is pushed to `barred` too.
Round 1's largest barred distance is 1,720 cm, so the bar holds with the leash on. This bounded fallback replaces TE-B1-1's open 'lengthen or harden until it does'.

P2, the figure (TE-B1-2 = RR-B1-1): in room_tick's building block (logs.rs:586-588), make the filter `.filter(|_| false)`. Run `cargo test --locked -p npt-lab --bin npt-lab -- logs::tests::<FP-3's test> --exact --nocapture`. It must fail at its first figure assert.

P3, the Hash filter (TE-B1-4 (a)): remove `&& matches!(t.filed, Filed::Hash(_))` from own_soldier (client lib.rs:356).
P4, the wrap rule (TE-B1-4 (b) = NA-B1-1 = RR-B1-5 = EA-B1-3 (b)): replace the `.reduce(...)` (:357-363) with `.max_by_key(|t| t.tick)`.
For each of P3 and P4, run `cargo test --locked -p npt-client --test endpoint -- brawl1 --nocapture`. P3 must fail at 'a complete tick turned sentinel is not read'; P4 at the wrap test's last assert.
- **Evidence:** For each plant, in brawl1-build.md's new section: the file, its three sha256 values, the planted line, the log path (.gate/u4r/brawl1/fp-plant-<n>.log), each command's exit code, and the failing assert's message. For P1 also, per seed and pattern: in_building, first, and the largest distance with its slot, tick, x and y; and whether the door-entry bar was needed. A plant among P2 to P4 that does not bite as ruled stops that item, and its record comes to me whole.

### Step 10: 10. FP-8: the end checks (after every edit above)

- **Findings:** TE-B1-1, TE-B1-2, RR-B1-1
- **Words:** Before its first edit, the copy records the sha256 of crates/lab/det-recordings.json and crates/lab/precheck-recordings.json. On the final code, through u4r-run.sh -x, df first, with each log carrying FP-0's diff line:
(1) check.mjs once: all 11 steps PASS. Counts as deltas from 535/613, each new test named with its file. Expected: +3 in each build (the bots area test, the logs.rs figure test, the client wrap test).
(2) `uptime`, then `node ops/runs/det-u4.mjs`, in one call, with no other lane building or running (condition 8). It must PASS and reproduce SS-P2-s1 1ec25de7c9ce8ea869ed9a8b343746ba131593e1de5c65bc9c1c626ce01cc76d and ES-P3-s2 6abf7643982b5970a630c97daa4d0815ceb2022958b18e18116397dcad9c893e. The non-cargo sum must stay under 240 s; over it, stop, and it comes to me whole.
(3) The precheck regeneration as final-regen-tests.log:18 ran it. It must print 'regenerated SS-P0-s4: 16950444 bytes, sha256 1f00d2d5f70a2c7b6836ede611b5f1c281319a5312060ead91e016da3c0b1b42, as the manifest's'.
(4) The short copy in both builds, with each build's time (at most 60 s), and the full test by name in release. Every figure round 1 printed must equal final-regen-tests.log:41-42, :62-63 and :83-91: in a building, first, the largest distance with its slot, tick, x and y, skipped, the chances and rate for Skips, the longest streak, and stuck. Only the dead figure and the reworded skip text are new. Any other difference means behaviour moved: stop, and it comes to me whole.
(5) Both manifests keep the sha256 recorded before the first edit.
(6) The size, by numstat against 352a245, SERIES.md included.
A det-u4.mjs or regeneration mismatch: stop. No re-pin and no retry; the log comes to me whole, with the first differing tick.
- **Evidence:** final-fp-check.log, final-fp-det.log and final-fp-regen-tests.log under .gate/u4r/brawl1/, each with its diff line. The figures go in the notes and the report.

### Step 11: 11. FP-9: report words, no code (in brawl1-build.md's new section and the copy's report, on disk before round 2)

- **Findings:** TE-B1-5, TE-B1-8, ADV-BP-3, ADV-BP-4, RR-B1-6, EA-B1-5, ADV-PT-M1, NA-B1-2
- **Words:** 1. The plant records P1 to P4, and whether the door-entry bar was added, and why.
2. Pattern (iii): withdraw 'a fixed radius stops it' and the 14 m vehicle. sim_run runs with stand-ins off (bots lib.rs:496). No solid lies within 1.6 m of (1734, -592), (1671, -751) or (1586, -917): building 0's corner (1770, -430) is 166 cm from the first point, the crates lie within 950 cm of the centre, and the wall stand-in is at y 1,180-1,220. The reading for U4g is this ruling's U4g words.
3. TE-B1-8: 'No test drives Behaviour::step with an own soldier past the outer radius. The glue at bots lib.rs:223 is shown only by the moved pins (own_soldier returns Some and changes commands) and by the two 10-minute evidence runs (condition 14).'
4. My corrections, in my words, beside ADV-BP-1 and ADV-BP-2: ADV-BP-3 and ADV-BP-4 as the points ruling words them.
5. RR4-L1's constant: MAX_ENTRY_TICK_COUNT stands (points ruling, Point 1). RR-B1-6 and EA-B1-5 are answered there, and ADV-PT-L3 carries the point to the check.mjs follow-up.
6. The follow-ups with their homes, as listed under 'Each finding': RR-B1-7 (1); EA-B1-1's pub constants, with NA-B1-3's npt-client half; ADV-PT-L3; TE-B1-8's optional glue test.
- **Evidence:** The section is on disk before the main session cuts round 2's snapshot.

## The pins

No ruled fix can move the three re-pinned recordings' bytes: SS-P0-s4 1f00d2d5..., SS-P2-s1 1ec25de7..., ES-P3-s2 6abf7643....

Why: the recordings come from `npt-lab scenario --record`, which writes no logs. The code on that path is npt-bots' library (Behaviour::step, next_command, leash, toward_centre), npt-client's decoding and own_soldier, the lab's run loop, the room and the sim. On that path the fix pass changes only two things:
- doc comments: bots lib.rs:57-59, :125-127 and :245-246; client lib.rs:244-246 and :347-348;
- Server::new's visibility (scenario.rs:397).
Everything else is test code (the #[cfg(test)] modules, client/tests/endpoint.rs, fp3, r10_carry_82), the logs.rs module doc, or comments. room_tick's code does not change in this pass (RR-B1-7 (1) is deferred), and it runs only on logged runs anyway (scenario.rs:1022-1026). So no bot decision and no recorded byte can change.

Proof at the copy's end (FP-8):
- det-u4.mjs (uptime first, condition 8) reproduces SS-P2-s1 1ec25de7c9ce8ea869ed9a8b343746ba131593e1de5c65bc9c1c626ce01cc76d and ES-P3-s2 6abf7643982b5970a630c97daa4d0815ceb2022958b18e18116397dcad9c893e.
- The precheck regeneration reproduces SS-P0-s4 1f00d2d5f70a2c7b6836ede611b5f1c281319a5312060ead91e016da3c0b1b42, 'as the manifest's'.
- Both manifests keep the sha256 they had at the snapshot.
- The full sim-only test's round-1 figures are equal. That test drives next_command directly, so equal figures show the decision function is unchanged.

Proof at the merge's counted run (condition 9, M3): the same det-u4.mjs (uptime first) and the same precheck regeneration, on the merged tree, reproducing the same three pins, plus the full test by name in release.

Any mismatch at either point: stop, and it comes to me whole. No retry and no re-pin (condition 7).

## Round 2

Round 2 is the second and last round, unless a Critical or High is open after it. Two seats, at effort xhigh (a netcode lane, as condition 11 set), in one batch: test-engineer and rust-reviewer.

The subject is only what changed:
- the diff from snapshot ee2973aa to the fix pass's new snapshot, cut by script as round 1's packet was;
- the plant records and logs, P1 to P4;
- FP-8's end-check logs;
- FP-0's two runner diffs;
- the copy's report section.
Each seat confirms its own round-1 findings fixed as ruled, or deferred as ruled, with file:line, and reviews the changed lines. Nothing unchanged is reviewed again.

Who reads what:
- test-engineer:
  - TE-B1-1's 48-record assert, and P1's record (the bite rule, and the door-entry bar if it was added);
  - FP-3's figure test with P2, and the bots area test;
  - FP-2's unit-test edits (TE-B1-3), against my re-computed command sequence;
  - FP-4's two client cases with P3 and P4: its own TE-B1-4, plus NA-B1-1, RR-B1-5 and EA-B1-3 (b);
  - the dead-bit tie and its count (TE-B1-6, with NA-B1-3 and RR-B1-3);
  - RR-B1-4's two fp3 cases, and ADV-PT-L2;
  - FP-8's counts, the full test's equal figures, and the pins.
- rust-reviewer:
  - RR-B1-1, through FP-3's test;
  - RR-B1-2's doc route, and RR-B1-3's tie;
  - RR-B1-4's rewords and cases;
  - RR-B1-5, through the wrap test;
  - RR-B1-7 (2), and EA-B1-2's narrowing;
  - fmt and clippy in both builds;
  - that the library diff on the recording path holds only doc comments and the one visibility change.

netcode-architect sits out. Its three Lows are covered elsewhere: the wrap case is a test (test-engineer reads it); NA-B1-2 (a) and (b) are doc words I give exactly (the main session checks them against this ruling and puts file:line on the card); the tie is test-engineer's. No sim, room, protocol or recording behaviour changes, and FP-8 proves the pins.

engine-architect sits out. No pub item, dependency or seam is added, and EA-B1-2 narrows one. The main session checks the doc words of EA-B1-3 (a), EA-B1-4 and RR-B1-7 (2) itself.

Either seat rejoins if the new snapshot adds a pub item, a dependency or a seam change, or if any library behaviour on the recording path changes. FP-8 makes that last case a stop anyway.

The budget: a round-2 finding below High becomes a STATUS follow-up named in the report; a Low never triggers a push. A Critical or High opens round 3 for that finding alone, and one still open after round 3 stops the change set for Mustafa. Not rounds: the gate after every step, and my review of the pushed head.

## The merge step

Condition 9's M-list. The answer to the clean question: yes, with two corrections.

(1) The form. The folder-level `cargo clean --target-dir .gate/shared-target` is not an approved method while the lane lock's file lives in that folder (W1-TARGET item 3 as amended, advisor-u4d2-m9.md:252-257: 'never by a clean without one of those three'). The approved cleans are the part-cleans:
- (B): the shared target's release, doc and wasm32 roots;
- (C): its dev profile.
Each is `cargo clean --target-dir <tagged root>` with `--release`, `--doc` or `--profile dev` and its test guards, exactly as .gate/u4r-sets/clean-shared-B.log and clean-shared-C.log ran them on 2026-11-14. (B) alone frees only about 440 MiB and (C) about 6 GiB, so both are needed.

(2) The place: right before the counted run, after round 2's ruling and M1, with no lane and no agent running. Round 2's seats run tests in the builder's copy through the shared target, so the clean must wait for them.

Why the clean comes first: a main-folder build re-makes every workspace crate beside the builder copy's artifacts. Today those take about 1.0 GiB of npt-* deps and 3.2 GiB of incremental data. Starting from about 6 GiB free, that would cross the 4 GiB floor mid-run, and the floor is checked only at the call's start.

The steps:
- M1. The merged bytes equal the reviewed bytes, in U4r's form: every changed file under spikes/ at the merge commit is cmp-equal to the round-2 snapshot's, and the tree fingerprint matches. The base also holds: spikes/ at the merge commit's parent equals 352a245's, and any commit in between touches only .checkpoints/.
- M2. pgrep for cargo, rustc, npt-lab, npt-judge and loader.mjs: all empty, and no agent running. Then df -B1; (B) in one main-run.sh call; df -B1; (C) in one main-run.sh call; df -B1. The guards: `test -f` on the lock file and on both CACHEDIR.TAG files, `test ! -e` on each cleaned folder, then `ls -A`. Every figure goes on the card.
- M3. One counted main-run.sh call on the merged tree:
  - `cat /proc/uptime`;
  - check.mjs, all 11 steps, counts equal to the copy's end figures;
  - `uptime` then det-u4.mjs, in the same call (condition 8; a non-cargo sum over 240 s stops it, and it comes to me whole);
  - the precheck regeneration at 1f00d2d5...;
  - the full sim-only test by name in release (condition 13, at the head that goes to my review), its prints equal to the copy's.
  The header's diff and untracked hashes should read the empty-input sha256. The run builds cold, so its timings are labelled cold (INF-U4R-7's labels). Free space is recorded at its start and end. A pin mismatch: stop, and it comes to me whole, with no retry.
- M4. The gate: `node tools/gate.mjs` (with --no-smoke, and the reason, if Chromium cannot start).
- M5. Push; CI green on the pushed head.
- M6. My review of the pushed head.
- M7. Then the plan's section 7 order. The cold counted run refills about 9 GiB, so (B) and (C) run again first, with df before and after; then the two 10-minute evidence runs; then the re-run sets, each set's results committed before the next.

## U4g's brief

These words go into U4g's brief, and the main session records them on the card and in U4r's report for U4g. They merge my ADV-PT-M1 with NA-B1-2 (c), and TE-B1-5's kinematic bound without its obstacle reading. The readings marked 'computed' were not run.

"BRAWL-1's pattern (iii), B1's bursts, in the sim-only test (printed only, never barred):
- Measured: in the full test (release, seeds 1 to 3, 610 s each), no Soldier record had area != 0. The largest distance from the centre was 1,832 cm in every seed, each at a burst's end, at (1734, -592), (1671, -751) and (1586, -917): three angles, one 1 cm band.
- Not a stop by a solid: the test runs with stand-ins off, and no solid lies within 1.6 m of those points. It is the leash's own reach under a 30-tick re-run. Computed, not run: about 1,830 to 1,840 cm. One reading takes the walk's 205 cm circle kept inside the 1,400 cm outer radius, plus 600 cm of re-run. The other takes a start just inside the 1,300 cm inner radius, plus 30 re-runs at about 18 cm. By kinematics the reach is at most about 2,000 cm: a soldier at the outer radius re-running 30 steps of 20 cm straight out.
- That reach passes the 1,800 cm door-entry radius of buildings 0 and 3, whose doors lie on the x axis (|y| < 60 for a soldier's centre). The door entries of buildings 1, 2, 4 and 5 lie at about 1,864 cm, within only the kinematic bound.
- The test is optimistic after a burst. It feeds each slot its World position from 8 steps earlier at once, while a real client may steer on its last complete pre-burst tick until a complete post-burst tick arrives.
- So 'area 0 in three seeds' is neither a bound nor a margin. A B1 judged run can carry a soldier into a door gap. That lowers -2's everyone-in-view share (bar 0.99), which makes -17's parts invalid (their validity is pre:PM-0001-2): BRAWL-1's INVALID again. The building figure on every B1 run is the check. U4g's plan says how a B1 building entry reads in -17, or measures how often it happens, before its judged B1 runs. One way to measure it is a printed door-gap margin per burst, TE-B1-5's option."

## The size

998 changed lines now (875 inserted and 123 deleted by numstat against 352a245, SERIES.md included). The fix pass adds about 270 changed lines, almost all of them tests:
- the bots tests, about 110;
- logs.rs's control test, about 60;
- the client tests and docs, about 50;
- the bots docs, about 15;
- scenario.rs and hash.rs, about 30.
Expected after the fix pass: about 1,270 (1,200 to 1,340); the conditional door-entry bar adds about 10 more. That is under my 1,400 'well past' line, so no SIZE-CHECK note is due. To keep that margin I deferred RR-B1-7 (1), a polish rewrite of production log code of about 30 lines. If numstat passes 1,400 at any point, the builder stops, and it goes to Mustafa in three lines (what grew, why, and the cheaper option).

## For Mustafa

BRAWL-1, the fix that keeps the test bots out of buildings, passed its first review round with no serious problem. The four reviewers found three medium gaps, all in the proof rather than in the code:
- nothing yet shows that the new 'no bot in a building' test can fail;
- nothing shows that the lab's building counter can count above zero.
I checked every finding against the code myself. One fix pass closes these gaps. A deliberate break of the leash shows that the test catches it. A test that puts a soldier inside a building shows that the counter counts it.

Most small findings ride the same pass. They are tests and comment wording, and nothing a bot does changes, so the three pinned recordings must stay the same byte for byte; the checks prove that. One reviewer misread a figure: it blamed a wall that is not there, and I corrected it. The warning behind that figure still holds and goes to the next unit: during a one-second burst of total packet loss, a bot can still walk into a building.

The change will be about 1,270 lines, under the 1,400-line mark where I would stop and ask you. Disk space holds if the build caches are cleared, by the approved method, just before the final counted run.

Decisions needed: none. Every point here is a technical choice inside the approved plan. KEPT-LOGS stays with you as raised at the plan review; nothing here changes it.

For Mustafa: nothing. This is spike code. Nothing a player sees, hears or feels changes, and it is not a merge to main.

## Each finding

### TE-B1-1 (Medium; fix now)

- **Home:** FP-1 (a) and FP-7 P1 (with the conditional door-entry bar); FP-8 (4)
- **Why:** Confirmed. No log runs the leash off; only the N2-1 plants exist. The sim test never checks that 48 Soldier records are present (World writes one per slot: world.rs:1816-1818). TE's 'lengthen or harden until it does' is open-ended. I replace it with a bounded fallback: a 1,800 cm door-entry distance bar, which holds with the leash on (round 1's barred maximum is 1,720 cm) and must bite under the plant.

### TE-B1-2 (Medium; fix now)

- **Home:** FP-3 (with RR-B1-1, fixed once), FP-1 (c), FP-7 P2
- **Why:** Confirmed by grep: no test makes a soldier's area non-zero, and none calls room_tick or summary_line with one. Its fix as worded would fail on npt-log's writer, which refuses a second record for the same tick (log lib.rs:467-474). So FP-3 uses two room ticks, a warm-up one and then a measured one.

### RR-B1-1 (Medium; fix now)

- **Home:** FP-3 (one test with TE-B1-2), FP-7 P2
- **Why:** The same gap as TE-B1-2, confirmed (logs.rs:584-599; scenario.rs:1612-1690). Its Server::build, script and tick route works: Server.room is pub(crate) (scenario.rs:347), and Room::script and Room::tick apply the teleport (room.rs:617, :805). Its room.jsonl and Warmup asserts are kept. Logs::open needs the folder made first (logs.rs:77-88).

### TE-B1-3 (Low; fix now)

- **Home:** FP-2
- **Why:** Confirmed by my own splitmix64: slot 5 at seed 1 has walk phase 7, so the walk at commands 4 to 7 is [-127, 0], the steer itself. The blocked asserts therefore cannot fail. The None/dead state and the outer boundary are untested. I re-sequenced the inserted cases (commands 12, 15, 16) so each check can fail. Test-only; it rides the push the Mediums need.

### TE-B1-4 (Low; fix now)

- **Home:** FP-4 (a) and (b), FP-7 P3 and P4
- **Why:** Confirmed: ticks 103 and 104 never complete, and the ring order is increasing. (a) The complete-then-sentinel case is added as worded. (b) is covered by the single wrap case: u32::MAX sits in ring slot 31 and 0 in slot 0, so that case also kills the ring-order mutant, and the 127/128 case is not needed.

### TE-B1-5 (Low; recorded)

- **Home:** U4g's brief words (this ruling); FP-9 item 2 (the report's correction)
- **Why:** Its conclusion holds: B1 can reach past the door entry, and area 0 is not evidence. Its mechanism is wrong. sim_run runs with stand-ins off (bots lib.rs:496), and no solid lies within 1.6 m of the three maxima (building 0's corner (1770, -430) is 166 cm from (1734, -592)), so there is no 'obstacle at 18.3 m'. Its kinematic bound of about 2,000 cm goes into the brief. No code change; the door-gap print is left as U4g's option.

### TE-B1-6 (Low; fix now)

- **Home:** FP-1 (b): the dead-bit tie in soldiers()
- **Why:** Confirmed: the literals are at client lib.rs:247-248 and bots lib.rs:479, and npt-sim's are private (world.rs:92, :120). Fixed once with NA-B1-3, RR-B1-3 and EA-B1-1. The per-record assert `dead == (health == 0)` runs on every tick of sim_run with real deaths (world.rs:1630-1634, :1176-1188), so no separate kill test is needed.

### TE-B1-7 (Low; fix now)

- **Home:** FP-0, by the main session, outside the change set (u4r-run.sh:66 and main-run.sh:41, with .diff files and bash -n)
- **Why:** Confirmed: the runner headers record only HEAD. From spikes/netcode-proof, `git diff HEAD -- .` equals the whole diff here, so the logged hash equals the snapshot sha256. This makes copy 2's logs and the counted run carry their exact code state.

### TE-B1-8 (Low; recorded)

- **Home:** FP-9 item 3 (report words); an optional glue test at U6's bots'-view item (EA-U4R-5's home)
- **Why:** Confirmed: no test drives Behaviour::step with an own soldier past the outer radius. The moved pins show only that own_soldier returns Some and changes commands. The fix was offered as optional, and the 10-minute runs (condition 14) are the integration proof. The report states this.

### NA-B1-1 (Low; fix now)

- **Home:** FP-4 (b), FP-7 P4
- **Why:** Confirmed: only ticks 100 to 106 are fed. Its wrap case is taken as the single case for TE-B1-4 (b), RR-B1-5 and EA-B1-3 (b). It kills both the plain-maximum and the ring-order mutants.

### NA-B1-2 (Low; fix now)

- **Home:** FP-5 (a) and (b) (doc words); (c) in U4g's brief words (this ruling)
- **Why:** Confirmed against final-regen-tests.log:84-91 (Skips at seed 2: 1,720 cm; Bursts: 1,832 cm in all three seeds) and its reading of the post-burst stale view. Doc-only, with no pin effect. Its (c) is merged into the U4g words with ADV-PT-M1.

### NA-B1-3 (Low; fix now)

- **Home:** FP-1 (b) (the bots tie with the dead count printed); its npt-client half goes to EA-B1-1's follow-up
- **Why:** Confirmed. The same fix as TE-B1-6, RR-B1-3 and EA-B1-1: its assert and the printed count are taken as worded. The client-side tie waits until npt-sim exposes the flag.

### RR-B1-2 (Low; fix now)

- **Home:** FP-5 (c): its documented alternative (domain stated), no code change
- **Why:** Confirmed but unreachable: decoded |x| and |y| are at most 32,768 (quant.rs:197-215), so x² + y² stays below 2^31. The u64 rewrite would edit library code on the recording path for no reachable input, so the doc route is taken.

### RR-B1-3 (Low; fix now)

- **Home:** FP-1 (b) and FP-4 (d) (the doc line on the client's constants)
- **Why:** Confirmed. The same fix as TE-B1-6: its assert, the dead count and the doc line naming the tie are all taken.

### RR-B1-4 (Low; fix now)

- **Home:** FP-6 (c): comments reworded, two fp3 cases added, code order kept
- **Why:** Confirmed: the run's 32-bit cap (scenario.rs:1435) comes before the burst block's 64-bit check (:1507-1515). Both lines are refused either way, so no wrong line is accepted. :2576's comment is false (u32::MAX is odd). I checked its case arithmetic myself (4,294,967,280 accepted, 4,294,967,310 refused). Test and comments only.

### RR-B1-5 (Low; fix now)

- **Home:** FP-4 (b), FP-7 P4
- **Why:** Confirmed. The same gap as NA-B1-1; one wrap case closes it.

### RR-B1-6 (Low; no action)

- **Home:** Answered by my points ruling (Point 1, ADV-BP-4); the check.mjs side is ADV-PT-L3's follow-up
- **Why:** The rename was ruled before this round: MAX_ENTRY_TICK_COUNT stands, because the ruled name fails check.mjs and the cap is a size bound, not a time. ADV-PT-L3 adds the name to NOT_TICK_COUNTS and _TICK_COUNT to TICK_NAMED at check.mjs's next change. Restoring the ruled name was refused there.

### RR-B1-7 (Low; fix now)

- **Home:** (2), the doc rewrap: FP-6 (b). (1), the loop rewrite: a follow-up for the next lane that edits logs.rs's room_tick
- **Why:** Confirmed: the `s as u8` cast after the i64 round trip, and the broken doc line. Soldier indices are below 64, so nothing truncates. (1) is a polish rewrite of production log code that round 2 would have to re-verify. I defer it to keep the size margin and to keep the fix pass free of library behaviour edits. (2) is doc-only.

### EA-B1-1 (Low; fix now)

- **Home:** Now: FP-1 (b) and FP-4 (d) (the tie and the doc line). Later: npt-sim exposes the flags index and the dead bit as pub items, in the next lane that opens crates/sim (beside EA-U4R-4), U6 at the latest
- **Why:** Confirmed (client lib.rs:246 says 'private there'). The bots tie closes today's drift risk without opening npt-sim. The pub items need a lane that edits crates/sim, so that part is a follow-up. The client-side cfg(test) variant is not needed now.

### EA-B1-2 (Low; fix now)

- **Home:** FP-6 (a)
- **Why:** Confirmed by grep: Server::new's only caller is scenario.rs:390, inside build, so making it private changes nothing else. It enforces EA-U4R-2's single constructor.

### EA-B1-3 (Low; fix now)

- **Home:** (a) FP-4 (c), doc words; (b) FP-4 (b) and FP-7 P4
- **Why:** Confirmed: slot_for resets a slot when a newer tick takes it (client lib.rs:564-574), so the doc claims more than the code does. The wrap gap is the same as NA-B1-1's.

### EA-B1-4 (Low; fix now)

- **Home:** FP-3 (EA-B1-4's words at logs.rs:6-7)
- **Why:** Confirmed: the module doc's 'all its counters, 0 included' conflicts with the list written only when non-empty (logs.rs:597-598; SERIES.md's row). Doc-only, in the file FP-3 opens.

### EA-B1-5 (Low; no action)

- **Home:** As RR-B1-6: my points ruling (Point 1); ADV-PT-L3's follow-up
- **Why:** The same item as RR-B1-6, already ruled. Its alternative (rename back and register the name) was refused at Point 1, in favour of ADV-PT-L3's form.

### ADV-PT-M1 (Medium; recorded)

- **Home:** U4g's brief words (this ruling), merged with NA-B1-2 (c) and TE-B1-5's bound
- **Why:** My finding, unchanged in substance: B1's 1,832 cm is the leash's reach, not a stop, and it passes the 1,800 cm door entry. The mechanism is labelled as computed, with the readings side by side. The B1 pattern is printed only by condition 3 (iii), so the diff does not change.

### ADV-PT-L1 (Low; fix now)

- **Home:** FP-1 (d)
- **Why:** It rides the push the Mediums need, as my points ruling said it would. The Bursts line's 'of 0 chances (0 per 1000)' goes into U4g's brief and means nothing.

### ADV-PT-L2 (Low; fix now)

- **Home:** FP-6 (d)
- **Why:** It rides the push the Mediums need. With 359,993 bytes the rule's boundary sits at 29,999 ticks and is not a multiple of 12, so a div_ceil plant is caught again.

### ADV-PT-L3 (Low; follow-up)

- **Home:** The check.mjs follow-up (check.mjs's next change; before PR #40 is marked ready at the latest)
- **Why:** Unchanged since my points ruling. It needs a check.mjs edit, which is outside this lane and would pull in the whole check.mjs follow-up.

### ADV-BP-3 (Low; recorded)

- **Home:** FP-9 item 4 (the report, in my words)
- **Why:** My wording error in condition 6 (ii); nothing to redo. Recorded beside ADV-BP-1 and ADV-BP-2.

### ADV-BP-4 (Low; recorded)

- **Home:** FP-9 item 4 (the report, in my words)
- **Why:** My error: RR4-L1's ruled name fails check.mjs's TICK_NAMED rule. MAX_ENTRY_TICK_COUNT stands.
