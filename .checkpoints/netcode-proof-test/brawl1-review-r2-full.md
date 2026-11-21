# BRAWL-1 review round 2: the two seats' results (workflow wf_19a5485a-e64), 2026-11-15

Written by script, word for word, from the workflow's journal. Snapshot reviewed: wip/brawl1.patch c2561b6c...; packet brawl1-r2-packet.md e50a22f7.... Each seat's notes: brawl1-review2-<seat>.md. Nothing below is the main session's.

## review2-test (APPROVE WITH FINDINGS; snapshot c2561b6ca54c)

**Seat:** test-engineer (BRAWL-1 round 2, batch seat with rust-reviewer; effort xhigh). Notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/brawl1-review2-test.md. My own log: /home/user/Shamble-Frontlines/.gate/u4r/brawl1-review2-test/r2-tests.log

**Its snapshot field, whole:** c2561b6ca54c8cde994fcdad5954c63d332df73f2db038c12aba3d71a0daa49f (wip/brawl1.patch, checked with sha256sum; round 2's read copy /home/user/Shamble-Frontlines/.claude/worktrees/brawl1 gives the same `git diff HEAD | sha256sum`; round 1's copy /home/user/Shamble-Frontlines/.gate/brawl1-snap1 gives ee2973aa...; packet brawl1-r2-packet.md e50a22f70ae876f962510693e03c414700200a7bf3a37a13ad31bf5fb5dfa8b5). The two findings below are both Low and become follow-ups: nothing triggers a push and nothing opens round 3.

What I checked myself, and it holds:
(1) The interdiff from ee2973aa to c2561b6c, read whole. Exactly 3 new #[test] functions and none removed (bots 3->4, endpoint.rs 14->15, logs.rs 2->3). That matches 535->538 and 613->616 at final-fp-check.log:9.
(2) The plants. Each file's current sha256 equals its before and after records. I also reproduced every planted sha256 with a pipeline that writes nothing, applying the ruled line to the reviewed bytes: P1 77be9b00, P2 b902a7ee, P3 7a77c71a, P4 6aeeec0a. So each plant was exactly the ruled line.
(3) P1 bit on area in both builds. The short copy fails at the area assert on Skips (planted :700 = clean :699). The full test lists Skips for all 3 seeds, and EveryTick for seeds 1 and 2. EveryTick in the short copy stays at 0, which the record notes as the closed-circle baseline. The door-entry bar was rightly not added. P2 fails at logs.rs:924, P3 at endpoint.rs:446, P4 at endpoint.rs:483, each as ruled.
(4) FP-2. I recomputed splitmix64 myself: phase 7. The walks are k12 [-89,-89], k15 [-89,-89], k16 [-49,-117], and toward_centre(1450,200) is [-125,-17]. Every new expected value differs from the twin's walk, and LEASH_OUT is now pinned to exactly 1,400.
(5) FP-8. check.mjs: 11 PASS. det-u4: PASS in 227.3 s, uptime first, non-cargo time 213.3 s. My own sha256sum of det-u4's regenerated files gives 1ec25de7... and 6abf7643..., equal to det-recordings.json:4-5. The regeneration prints 1f00d2d5 'as the manifest's'. Both manifests are unchanged. I diffed the 13 printed lines myself, after only the stated changes (dead count removed, old skip wording mapped to the new): identical to round 1.
(6) FP-0. Each .diff is one line, as ruled. `bash -n` is silent on both runners. Every fix-pass log header carries the diff line.
(7) My own run at c2561b6c (r2-tests.log, -x -k, exit 0). Bots brawl1 tests pass in both builds (11.17 s and 10.82 s). Client brawl1 tests pass in both builds. The lab figure test, fp3 and r10_carry_82 pass in both builds. The full test passes in release in 24.31 s. All 13 prints are byte-identical to final-fp-regen-tests.log.

On LOGS0-RECORD, option (a) is sound: summary_line reads only `s.logs` (scenario.rs:1615), so the added `--record` cannot change the branch under test.

**Its round-1 findings:**

- **TE-B1-1:** fixed as ruled. **Where:** spikes/netcode-proof/crates/bots/src/lib.rs:565 (48-record assert); P1 in brawl1-build.md:482-506 and .gate/u4r/brawl1/fp-plant-1.log **Note:** Each tick asserts `now.len() == n`. P1 bit on area: the short copy fails in both builds on Skips, and the full test lists Skips for seeds 1-3 (EveryTick too for seeds 1 and 2). The door-entry bar was not needed and was not added. EveryTick in the short copy stays 0 under the plant; it is recorded as the closed-circle baseline. I reproduced the planted sha256 77be9b00 myself.
- **TE-B1-2:** fixed as ruled. **Where:** spikes/netcode-proof/crates/lab/src/logs.rs:872-967 (figure test); bots/src/lib.rs:704-732 (npt-sim area test); P2 in fp-plant-2.log **Note:** Two room ticks. Warm-up: counted 0, pair written. Measured: 1 slot-tick, slots {3}. room.jsonl holds [3,1] on both ticks, and both summary-line cases are checked. P2 fails at :924. The bots test shows npt-sim gives area 1 at building 0's centre and 0 for the other 47 slots. My run passes both builds.
- **TE-B1-3:** fixed as ruled. **Where:** spikes/netcode-proof/crates/bots/src/lib.rs:380-388 (blocked), :408-413 (kept state, k12), :418-429 (outer radius, k15/k16), :338-341 (const bounds) **Note:** I recomputed the sequence myself (phase 7) and it matches the advisor's. Every assert_ne guard passes, so each check can fail. LEASH_OUT is pinned to exactly 1,400 with `>`. A residual gap in the kept-state check is new finding TE-B1-R2-1 (Low).
- **TE-B1-4:** fixed as ruled. **Where:** spikes/netcode-proof/crates/client/tests/endpoint.rs:437-450 (a) with P3; :459-488 (b, the wrap case) with P4 **Note:** (a) A complete tick turned sentinel is not read; P3 fails at :446. (b) The u32::MAX -> 0 wrap case replaces my 127/128 case, as ruled. Ring slots 31 and 0 kill both the plain-max and the last-entry mutants, and P4 fails at :483. The old test's 101/105 order still kills a first-entry mutant.
- **TE-B1-5:** deferred as ruled. **Where:** /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/brawl1-build.md:587-591 (FP-9 item 2); U4g words in advisor-brawl1-r1-full.md:279-288 **Note:** Recorded, with no code change. My 'obstacle at 18.3 m' mechanism was wrong and is withdrawn: sim_run runs with stand-ins off (lib.rs:541). The kinematic bound and the door-gap print option go to U4g.
- **TE-B1-6:** fixed as ruled. **Where:** spikes/netcode-proof/crates/bots/src/lib.rs:518-524 (tie), :502 and :568-570 (dead count), :700 (dead > 0); client/src/lib.rs:245-249 (doc) **Note:** Fixed by the ruling's per-record tie (bit 3 set exactly when health is 0), not by my separate kill test. HEALTH is an exact integer field (sim seam.rs:214), so the tie has no quantisation edge. The short copy sees deaths: 1,190 and 1,334 soldier-ticks.
- **TE-B1-7:** fixed as ruled. **Where:** .checkpoints/netcode-proof-test/u4r-run.sh:67, main-run.sh:42; u4r-run.sh.te-b1-7.diff, main-run.sh.te-b1-7.diff **Note:** The ruled form logs the full 64-hex hash, better than my 16-character cut. fp-0-header.log shows ee2973aa, and every final-fp log shows c2561b6c. A labelling nit in my own form is TE-B1-R2-2 (Low).
- **TE-B1-8:** deferred as ruled. **Where:** /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/brawl1-build.md:592-594 (report words), :615-616 (follow-up at U6's bots'-view item) **Note:** The ruled words are recorded. The glue line is now bots lib.rs:228, which the report notes.

### TE-B1-R2-1 (Low)

- **Where:** spikes/netcode-proof/crates/bots/src/lib.rs:404 (the dead input) and :408-413 (the new kept-state check, command 12)
- **Wrong:** The new check 'None and dead keep the leash's state' cannot tell 'dead keeps the state' from 'a dead soldier's position updates the state'. The dead input sits at (1450, 0), past the outer radius. Take a leash that recomputes `steering` from a dead soldier's position and then returns the walk: steering is still true at command 12, so it passes this check and the whole unit test. Only the 'None or dead resets to false' mutants are killed. In practice the surviving mutant is close to the real code, because respawns land round homes about 10 m out (sim scene.rs:153), mostly inside the inner radius, which resets the state anyway. Hence Low.
- **Evidence:** Sequence (my splitmix64, phase 7): k9 at(1301,0) keeps steering true; k10 dead at (1450,0); k11 None; k12 at(1350,0) expects the steer [-127,0]. Under the mutant, k10 computes d2 = 1450² > 1400², so steering stays true and k12 still steers. No later assert separates the two behaviours, because k13 at(1299,0) resets the state for both.
- **Fix:** Follow-up, test only. At lib.rs:404 replace `give(Some(([1_450, 0, 150], true)))` with `give(Some(([0, 0, 150], true)))` and keep both messages. 'dead: the walk' can still fail, because the walk at k10 is [-117,-49] and toward_centre(0,0) is [0,0]. Command 12's check then fails for three mutants that pass today or are caught only partly: update-from-position, ignore-dead, and dead-resets.

### TE-B1-R2-2 (Low)

- **Where:** /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-run.sh:67 and main-run.sh:42 (FP-0's header line; my own round-1 form, kept by the ruling)
- **Wrong:** The 'untracked' field hashes the whole `git status --porcelain` list, modified files included, not the untracked files. So every fix-pass log reads 'untracked 17cf51d7...' on a tree with nothing untracked, and a reader would think something untracked is there. Neither hash covers an untracked file's content. The ruled M3 expectation still holds: both read e3b0 on a clean tree, and the main folder's spikes/netcode-proof porcelain is empty today.
- **Evidence:** fp-0-header.log:3 and final-fp-check.log:3 both read 'untracked 17cf51d7c16f37afb4247c27fc96ede8b439d461e6d55d1e55c141e0ccd4a90b', while `git status --short` lists 10 ` M` files and nothing untracked. The builder's notes (brawl1-build.md:427-429) have to explain it.
- **Fix:** Follow-up, a main-session runner edit outside the change set, after M3. In both runners replace the untracked part with `untracked $(git ls-files --others --exclude-standard -- . | sha256sum | cut -d' ' -f1)`. On a tree with no untracked file it reads e3b0, so M3's expectation does not change. Save the change as a .diff and run `bash -n`, in INF2-4's form.

## review2-rust (APPROVE WITH FINDINGS; snapshot c2561b6ca54c)

**Seat:** rust-reviewer (BRAWL-1 round 2, batch with test-engineer per the advisor's ruling on round 1; effort xhigh). Notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/brawl1-review2-rust.md. My run: /home/user/Shamble-Frontlines/.gate/u4r/brawl1-review2-rust/lint-tests.log.

**Its snapshot field, whole:** Reviewed: wip/brawl1.patch, sha256 c2561b6ca54c8cde994fcdad5954c63d332df73f2db038c12aba3d71a0daa49f. I checked it with sha256sum, and the read copy /home/user/Shamble-Frontlines/.claude/worktrees/brawl1 (HEAD 352a245) gives the same value for `git diff HEAD | sha256sum`. Round 1's copy (.gate/brawl1-snap1) gives ee2973aa. The packet brawl1-r2-packet.md hashes to e50a22f70ae876f962510693e03c414700200a7bf3a37a13ad31bf5fb5dfa8b5.

**What changed between the two copies:** six files differ by cmp: bots lib.rs, client lib.rs, client/tests/endpoint.rs, hash.rs, logs.rs and scenario.rs. The two manifests, precheck.rs and SERIES.md are unchanged. numstat against 352a245 gives 1,139 added and 127 deleted, 1,266 changed lines.

**Recording path:** I compared the two copies with difflib, looking only at lines above each file's #[cfg(test)]. Every changed line there is a comment, except scenario.rs:396, where `pub(crate) fn new` became `fn new`. That holds the ruling's claim: doc comments plus one visibility change.

**My run (lint-tests.log):** u4r-run.sh -x -k, 07:50:28-07:51:06 UTC. The header shows diff c2561b6c, and 6,591 MiB was free. Every command exited 0:
- `cargo fmt --all --check`;
- `cargo clippy --locked --workspace --all-targets -- -D warnings`, in the measured build and with --all-features;
- lab tests (the brawl1 figure test, fp3, r10_carry_82): 4 passed in each build;
- client endpoint brawl1: 2 passed in each build;
- bots tests::brawl1: 3 passed and 1 ignored in each build (11.00 s; 10.82 s). The short copy's prints equal the builder's: 1575 cm and 1629 cm, dead 1190 and 1334.

**The builder's logs:**
- fp-check-2 and final-fp-check carry the same diff hash. check.mjs passes all 11 steps, with 538 and 616 tests.
- final-fp-det shows `uptime` first and 'PASS det-u4 227.3 s'.
- final-fp-regen-tests shows 1f00d2d5 'as the manifest's'.
- I hashed det-u4's regenerated recordings on disk myself: 1ec25de7 and 6abf7643.

**Plants:** I rebuilt each plant in memory on the current files. Each gives exactly the planted sha256 recorded: P1 77be9b00, P2 b902a7ee, P3 7a77c71a, P4 6aeeec0a. The current files equal the recorded before and after values: 83a8541b, 83008fce and 2aca00c8. The logs fail where the ruling says:
- P1: lib.rs:700 in both builds of the short copy, and :750 in the full test, which lists Skips for all three seeds;
- P2: logs.rs:924;
- P3: endpoint.rs:446;
- P4: endpoint.rs:483.

**FP-0:** `bash -n` on both runners prints nothing. The 'untracked' value hashes the porcelain name list, not file contents. It still trips on any untracked file. The main folder's spikes/netcode-proof reads e3b0c442 for both values today, so M3's expectation is reachable.

No Critical, High or Medium.

**Its round-1 findings:**

- **RR-B1-1:** fixed as ruled. **Where:** spikes/netcode-proof/crates/lab/src/logs.rs:871-967 (test brawl1_the_building_figure_counts_a_soldier_in_a_building_in_measured_ticks_only at :878; warm-up asserts :912-913; measured count :923-927; slot set; room.jsonl [3, 1] on both ticks; both summary forms at :957 and :964) **Note:** P2 (`.filter(|_| false)`) fails at logs.rs:924 with 0 against 1 (fp-plant-2.log). I rebuilt P2's planted sha256 b902a7ee and it matches. The ruling's line `settings(&dir, "--profile P0 --seed 1 --logs 0")` cannot run as written. parse refuses `--logs 0` without `--record` (scenario.rs:1486-1488), and the path is only stored (scenario.rs:1456, Settings.record :644). The added `--record` is therefore needed, and I agree with option (a) of the builder's LOGS0-RECORD question. `Some(dir.as_path())` in place of the ruling's `Some(&dir)` is required: `Some(&dir)` would be an Option<&Scratch>, which does not coerce to Option<&Path>.
- **RR-B1-2:** fixed as ruled. **Where:** spikes/netcode-proof/crates/bots/src/lib.rs:130-132 (toward_centre's domain words), :250 (leash's '(x and y in toward_centre's domain)') **Note:** The ruling's documented alternative: doc only, with no code change on the recording path. One word is off at the boundary; see RR2-B1-1.
- **RR-B1-3:** fixed as ruled. **Where:** spikes/netcode-proof/crates/bots/src/lib.rs:509-526 (soldiers(): doc and the tie assert at :519-524), :502 (Record.dead), :568-570 (counted), :675 and :686 (printed), :700 (`r.dead > 0` in the short copy); crates/client/src/lib.rs:246-249 (the doc line on SOLDIER_FLAGS/SOLDIER_DEAD at :250-251) **Note:** The tie is exact for npt-sim today. Health is a u16, set to 0 together with FLAG_DEAD at world.rs:1630-1634, and both are reset at revive (:1176-1188). Dead soldier-ticks: 1,190 (EveryTick) and 1,334 (Skips) in the short copy, and about 8,000 per full run. So the tie runs on real deaths in every pattern.
- **RR-B1-4:** fixed as ruled. **Where:** spikes/netcode-proof/crates/lab/src/scenario.rs:1432-1434 (reworded; true, because the burst block follows at :1504-1530), :2576 (reworded; true, because u32::MAX is odd), :2588-2607 (the two B1 cases) **Note:** Code order kept, as ruled. I redid the arithmetic. 143,165,566 x 30 + 300 = 4,294,967,280 is accepted. 143,165,567 gives 4,294,967,310 and is refused, with the exact text of the format at :1515-1520. burst_starts at 4.29e9 ticks gives about 4.29e18 ns, under u64::MAX, so the debug build cannot overflow. Both builds pass.
- **RR-B1-5:** fixed as ruled. **Where:** spikes/netcode-proof/crates/client/tests/endpoint.rs:460-488 (brawl1_own_soldier_compares_ticks_by_the_endpoint_s_wrap_rule at :463; last assert :483-487) **Note:** P4 (`.max_by_key(|t| t.tick)?`) fails at endpoint.rs:483, returning 11 where 22 is expected (fp-plant-4.log). I rebuilt P4's planted sha256 6aeeec0a and it matches. The ring holds slot 0 (tick 0) before slot 31 (tick u32::MAX), so the case also kills a plain `<` compare and a 'last matching ring entry' read. It passes in both builds.
- **RR-B1-6:** deferred as ruled. **Where:** spikes/netcode-proof/crates/lab/src/hash.rs:239 (MAX_ENTRY_TICK_COUNT, unchanged); brawl1-build.md 'FP-9: report words' item 5 **Note:** No action, as ruled: the points ruling's Point 1 answers it. The check.mjs side is ADV-PT-L3's follow-up, recorded in FP-9 item 6.
- **RR-B1-7:** fixed as ruled. **Where:** spikes/netcode-proof/crates/lab/src/scenario.rs:367-371 (part (2): Server::build's doc rewrapped, words equal to round 1's, no lone fragment) **Note:** Part (1) is deferred as ruled. room_tick's building block (logs.rs:584-599) is unchanged, and the `s as u8` cast is still at :596. FP-9 item 6 sends it to the next lane that edits room_tick. EA-B1-2, from my list, is also confirmed: `fn new` is private at scenario.rs:396, its only caller is :389 (I grepped crates/, check.mjs and ops/), and clippy -D warnings is clean in both builds.

### RR2-B1-1 (Low)

- **Where:** spikes/netcode-proof/crates/bots/src/lib.rs:131-132 (toward_centre's doc, FP-5 (c))
- **Wrong:** The doc says decoded inputs keep 'x² + y² ... below 2^31'. At the corner (-32,768, -32,768) the sum is exactly 2^31 (2^30 + 2^30). So the bound is 'at most', not 'below'. Nothing overflows either way, because the arithmetic is in i64.
- **Evidence:** POSITION_XY has min -32_768 (sim grid.rs:106-111). Quant::value accepts k == field.min() (proto quant.rs:212: it refuses only k < min or k > max). The builder flagged the same point in the report's section 6 and wrote the ruled words as given.
- **Fix:** At bots lib.rs:132, replace 'so x² + y² stays below 2^31' with 'so x² + y² is at most 2^31'. This is a doc comment only, with no effect on any recording. Per the budget it becomes a STATUS follow-up and triggers no push.

### RR2-B1-2 (Low)

- **Where:** /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/brawl1-build.md:665 ('## Fix pass, round 1: report', section 1, FP-6 (a))
- **Wrong:** The report says Server::new's 'only caller is :390'. In round 2's copy the caller is at scenario.rs:389. The rewrap of Server::build's doc removed one line above it. The report moved `fn new` to :396 but kept the caller at round 1's number.
- **Evidence:** In the read copy, `grep -n 'Server::new' crates/lab/src/scenario.rs` prints only `389:        let mut server = Server::new(bites.room(room), bites.quant(), RttSource::Link);`.
- **Fix:** In the report words, change 'Its only caller is :390.' to 'Its only caller is :389.' This is report text only, with no code. Fold it in when the report is folded.
