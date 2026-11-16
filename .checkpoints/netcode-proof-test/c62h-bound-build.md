# C62H-BOUND build notes (the brief: c62h-bound-brief.md, 663 lines, sha256 7936074c15c5eb49…)

## Plan (the main session, 2026-11-16)
- Route W: one builder launch per commit step, in `/home/user/Shamble-Frontlines/.claude/worktrees/c62h-bound`
  (branch c62h-bound-build), no isolation; the main session commits each step, checks its diff hash and SIZE-CHECK.
- Order: (0) base checks, runner hashes, the worktree, the built check shown to fail, insert part 1 (done, below);
  (1) step 1's launch, commit 1; (2) step 2's launch, commit 2; (3) insert part 3 and the built check, step 3's
  launch (B-2's four fixture files and check.mjs), commit 3, the built check again; (4) step 4's launch, commit 4;
  (5) the review round (test-engineer, netcode-architect, rust-reviewer, xhigh), one fix pass; (6) the merge by
  cherry-pick, the counted main-run.sh call, the gate, the built check, push, CI, the advisor's head check; (7) the P0
  re-judge on 93509dfd7fd1 with the compare script (self-test first); (8) P2, then P3, each with the set check (shown
  to fail first); (9) the report.
- Expected test counts from LANE_BASE's 538 measured / 616 bites: step 1 +7 / +9; step 2 +6 / +6; step 3 +1 / +1;
  step 4 0 / 0; final 552 / 632. A different count stops for the advisor.

## Log (one line per item; times UTC)
- 09:18 LANE_BASE = 044fab188419e8859dbca699d851563c44c078c9 (the branch head at the build's start: 442a284 plus card
  commits, the second pack's list 6c8c7cf and its card save). `git diff --quiet 442a284 044fab1 -- .
  ':(exclude).checkpoints' ':(exclude)spikes/netcode-proof/results'` exit 0; under results/ only
  `A results/u4r/ab10f058a828/P2/raw-logs.sha256` (a kept-logs list).
- 09:18 Runner sha256s equal the brief's: u4r-run.sh e9437ae2…, main-run.sh 8df2dee3…, peak.py dfacdcc0…,
  c-u4r-runs.mjs 99dd070e… (LANE_BASE's). Texts v3 3f21c9b7…, freeze 9cb8be17…, paste cfac7502…, brief 7936074c…;
  marks.json 9fb5be57…, SERIES.md 3411eee7…, README.md b49f3441… (LANE_BASE's).
- 09:19 Worktree made by hand: `git worktree add -b c62h-bound-build .claude/worktrees/c62h-bound 044fab1`.
- 10:2x The main session's scripts written: c62h_bound_insert.py and c62h_bound_built_check.py (notes folder).
- 10:2x The built check shown to fail on the worktree as LANE_BASE left it
  (.gate/u4r/c62h-bound/built-check-negative.log): exit 1; texts v3, freeze, paste and all 8 block hashes PASS (12);
  both marks.json blocks, the four rows, both header lines, the sentence, the three file hashes and the word diff FAIL
  (13), as the brief requires.
- 10:2x Insert part 1 on the worktree (.gate/u4r/c62h-bound/insert-part1.log): exit 0; SERIES.md 3411eee7… →
  89295b57d5c9ceec… (the brief's expected); numstat +2 -1 (unstaged, for step 1's launch to stage with its files).
- 10:2x Part 3 dry run on a scratch copy only (.gate/u4r/c62h-bound/dry-p3/, log insert-part3-dry.log; not the
  worktree): all three expected hashes reproduced (marks.json c1935a5b…, SERIES.md 6898e119…, README.md f3c3c804…;
  json.loads check equal; README refilled into 16 lines, the longest 120). The real part 3 runs before step 3's launch.

## Launch 1 (step 1)
- 10:29 Start. Read brief sections 0, 1, 3.1, 4, 10 (brief sha256 7936074c… checked; texts v3 3f21c9b7… checked);
  runner u4r-run.sh e9437ae2… checked. Worktree at 044fab1, only ` M marks/SERIES.md` (89295b57…, +2 -1). Plan: items
  1-11 of 3.1 in order (l1 read + test; lab join + tests; call; totals; plant; record; tally; summary phrase; short P2
  tests; module texts); then plants CB-P1..P7, P12, P13; stage; check.mjs whole; SIZE-CHECK.
- 10:5x Items 1-11 written (not yet compiled): l1 lib.rs DatagramCounts (after FateLog) and
  FateLog::server_datagrams_since (after for_each_canonical); l1 tests.rs down_datagrams_in_canonical +
  c62hb_server_datagrams_since_counts_down_datagrams_by_fate (after reorder_counts' test); scenario.rs module paragraph
  (link record), link_lost (after impl Server), LAB_BITES [&str; 3], Bites::link_lost_overstated, Summary
  link_sent/link_lost/link_burst, the call + totals + plant in run's per-slot loop, the summary phrase, the 5 join
  tests, c62hb_join, the short P2 test and the 2 bite tests; logs.rs module text, Logs::link_lost map, Counts
  lost_room_sent_nothing/lost_link/lost_unexplained, room_tick's link_lost parameter (+ the house-style
  too_many_arguments allow, 8 inputs now), the tally in write_client_tick, the two test call sites `&[]`.
  Reading taken (to the main session): Summary.link_lost = sum of c.lost + c.burst (link_lost()'s own value, so the
  phrase "lost by the link L (in bursts B)" has B inside L); link_burst = sum of c.burst; link_sent = sum of c.sent.
- 10:39 cargo fmt --all --check: a diff in scenario.rs's new tests (step1-fmt-1.log, exit 1); applied by Edit;
  step1-fmt-2.log exit 0.
- 10:40 New tests, measured build (step1-new-measured.log, `cargo test --locked --workspace c62hb`): exit 0; l1
  "1 passed" (independent count = read = {sent 90, lost 5, burst 30} from index 115); lab "6 passed" (5 join tests,
  the short P2 test: measured incomplete 29 = E1 0 + E2 29 + N 0; join unexplained 0, whole despite loss 0; measured
  link-lost sum 29; S 1920, L 29, B 0; fate log ca5edd4bde19b9bd).
- 10:42 New tests, bites build (step1-new-bites.log, `--all-features`, -x): exit 0; l1 "1 passed", lab "8 passed";
  whole-as-lost: join 16 / 0, N 16 (measured incomplete 45 = 29 + 16); link-lost-overstated: join 0 / 16, N 0, L 29 =
  the clean run's 29, fate logs equal (ca5edd4bde19b9bd). No unexplained tick in any clean run; no join Err.
- 10:43 sha256 before the plants (.gate/u4r/c62h-bound/step1-sha-before-plants.txt): lib.rs 5a7b81dd…, tests.rs
  99ed3c27…, scenario.rs c75440f6…, logs.rs d54f7c8e….
- 10:4x CB-P1 (l1 lib.rs:420, `|| e.dir != Dir::Down` dropped; plant-CB-P1.log, -x, measured build): "test result:
  FAILED. 0 passed; 1 failed" naming tests::c62hb_server_datagrams_since_counts_down_datagrams_by_fate; left {sent 180,
  lost 5, burst 60} against the independent {90, 5, 30}: sent above. Restored by Edit; sha 5a7b81dd… equal.
- 10:4x CB-P12 (lib.rs:420, `e.carries != Carries::Datagram ||` dropped; plant-CB-P12.log): "1 failed", same test;
  left {sent 202, lost 5, burst 40}: sent above. Restored; sha equal.
- 10:4x CB-P13 (lib.rs:419, `get(from..)` as `get(0..)`; plant-CB-P13.log): "1 failed", same test; left {sent 120,
  lost 5, burst 30}: sent above. Restored; sha equal.
- 10:5x CB-P2 (scenario.rs link_lost, the `c.sent as usize != groups` block dropped; plant-CB-P2.log): "test result:
  FAILED. 0 passed; 2 failed" naming c62hb_join_one_datagram_too_many and _too_few, each left Ok(0) right its Err.
  Restored; sha c75440f6… equal.
- 10:5x CB-P3 (link_lost's server-hash check dropped; plant-CB-P3.log): "1 failed",
  c62hb_join_groups_with_a_null_server_hash, Ok(0) where Err. Restored; sha equal.
- 10:5x CB-P4 (link_lost's no-session check dropped; plant-CB-P4.log): "1 failed", c62hb_join_groups_with_no_session,
  Ok(0) where Err. Restored; sha equal.
- 11:0x CB-P5 (run's `link[usize::from(slot)] = lost` as `= 0`; plant-CB-P5.log): "1 failed",
  c62hb_p2_short_every_measured_lost_tick_is_explained, at "the measured link-lost sum is 0" (its print: join 29
  unexplained, E2 0, N 29). Restored; sha equal.
- 11:0x CB-P6 (run: a `prev_link` vector and room_tick passed `&std::mem::replace(&mut prev_link, link.clone())`, the
  record one room tick late; plant-CB-P6.log): "1 failed", same test, at the join assert: left (28, 29) right (0, 0)
  (unexplained 28, whole despite loss 29). Both edits restored; sha equal.
- 11:0x CB-P7 (logs.rs tally: `self.logged.get(&w)` after its removal in place of the kept `logged`; plant-CB-P7.log):
  "1 failed", same test, at "E2 is 0" (E1 29, E2 0, N 0). Restored; sha d54f7c8e… equal. All four files equal the
  pre-plant record (diff against step1-sha-before-plants.txt: none). No plant in place.
- 11:1x Staged: l1/src/lib.rs, l1/src/tests.rs, lab/src/scenario.rs, lab/src/logs.rs, marks/SERIES.md (89295b57…,
  unchanged). Staged numstat vs LANE_BASE (results excluded): lib.rs +33 -0, tests.rs +98 -0, logs.rs +47 -7,
  scenario.rs +303 -8, SERIES.md +2 -1; total 499 (plan for step 1 about 285-400; under the 1,050 stop). check.mjs
  whole started (step1-check.log, -x).
- 10:58 check.mjs whole on the staged bytes (.gate/u4r/c62h-bound/step1-check.log): exit 0, "check: all 11 steps
  pass"; test step "measured build: 545 tests passed, 0 failed, 0 reported a skip; bites build: 625 tests passed, 0
  failed, 0 reported a skip" (+7 / +9 from 538 / 616, as expected). Header: diff 0b41121e5cb1b569…, untracked
  e3b0c44298fc1c14… (none). `git diff --cached HEAD -- . | sha256sum` from spikes/netcode-proof = 0b41121e…; no
  unstaged change, no untracked file after the run.
- 10:59 SIZE-CHECK (staged, `git diff --cached --numstat 044fab1 -- spikes/netcode-proof/ ':(exclude)…/results'`):
  499 (+483 -16), as at 11:1x above.

## State (launch 1, step 1, 10:59 UTC: a clean point; nothing in flight, no plant in place)
- Staged in the worktree (branch c62h-bound-build at 044fab1, nothing committed): crates/l1/src/lib.rs (+33 -0),
  crates/l1/src/tests.rs (+98 -0), crates/lab/src/scenario.rs (+303 -8), crates/lab/src/logs.rs (+47 -7),
  marks/SERIES.md (+2 -1, B-1, 89295b57…, the main session's). Total 499 against the plan's 285-400 for step 1.
- Check: step1-check.log, exit 0, all 11 steps PASS, diff hash 0b41121e5cb1b5692b7f0a6ea3eb223e8fed5fa7d9b0df4c85f635b372c3267f,
  untracked hash e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855.
- Counts: 545 measured / 625 bites (+7 / +9). New tests by name, from the runner's own output (step1-new-measured.log,
  step1-new-bites.log; check.mjs prints no per-test lines): both builds: tests::c62hb_server_datagrams_since_counts_
  down_datagrams_by_fate (npt-l1), scenario::tests::c62hb_join_one_datagram_too_many, _too_few,
  c62hb_join_groups_with_a_null_server_hash, c62hb_join_groups_with_no_session, c62hb_join_ok_counts_lost_and_burst,
  c62hb_p2_short_every_measured_lost_tick_is_explained; bites only: c62hb_p2_short_bite_whole_as_lost_gives_16_
  unexplained, c62hb_p2_short_bite_link_lost_overstated_gives_16_whole_despite_loss.
- Plants: CB-P1, -P12, -P13 (l1 test), -P2 (2 failed), -P3, -P4 (join tests), -P5, -P6, -P7 (short P2 test): each bit
  as section 4 says, each restored, each file's sha256 equal to the record (logs plant-CB-*.log).
- Stop rules: none met. Clean short P2: N 0 = the join's 0; E1 + E2 + N = measured incomplete (0 + 29 + 0 = 29); no
  join Err; no floor stop; counts as expected. The lab-vs-judge comparison is not reachable in step 1 (no figure yet).
- Next (the main session): commit 1 in the worktree; check `git diff HEAD^ HEAD -- . | sha256sum` (spikes/netcode-
  proof) = 0b41121e…; the committed SIZE-CHECK = 499; then step 2's launch.

## Main session after launch 1
- 11:00 Launch 1's return saved word for word: c62h-bound-step1-full.json (150 lines, sha256 80b28a16…; status done).
  Read in full: 7 measured and 9 bites new tests by name (+7 / +9: 545 / 625, as expected); 9 plants bitten; points:
  (1) Summary's L counts c.lost + c.burst and B c.burst (B is part of L; on P2 and P3 B is 0): for the review round
  (netcode-architect) to confirm; (2) size 499 against the plan's 285-400; (3) one lint allow on Logs::room_tick
  (logs.rs:520-523), the only deviation; (4) the clean short P2 run: incomplete 29 = E1 0 + E2 29 + N 0, S 1920, L 29.
- 11:01 **Commit 1: 54046d4a83e445a212670dd548d5a5c1b9dd00ef** in the worktree (message from brief section 1).
  `git diff HEAD^ HEAD -- . | sha256sum` from its spikes/netcode-proof = 0b41121e5cb1b569… = step1-check.log's
  header diff hash; untracked e3b0… (none); nothing unstaged. SIZE-CHECK from LANE_BASE: 499 (+483 -16; lib.rs 33,
  tests.rs 98, logs.rs 54, scenario.rs 311, SERIES.md 3), equal to the launch's staged count.
- 11:02 Step 2's launch next (section 3.2; plants CB-P8 to -P10, CB-P14 to -P18; expected +6 / +6 → 551 / 631). The
  size: 499 of 1,050 used after step 1; the launch is asked to keep to the brief's tests and asserts, never weaker.

## Launch 2 (step 2)
- Start. Read brief sections 0, 1, 3.1, 3.2, 4, 10 (sha256 7936074c… checked); texts v3's three series rows; TE-CBB-2,
  -7, -8; runner u4r-run.sh e9437ae2… checked (header read). Worktree at 54046d4, clean. Plan: figures.rs: one reader
  `bound` (struct Bound) after decoded's `coverage`, its eight gaps in the frozen row's order (slot_hashes' own
  refusals first), the two figures and their FIGURES entries after decoded_hash_compared_count; engine.rs: shared
  builders (bound_mark, bound_run on hash_log, bound_out, ab) and the six tests after the u4j hash tests; then fmt,
  the new tests in both builds, plants CB-P8, -P9, -P10, -P14 to -P18, stage, check.mjs whole, SIZE-CHECK.
- 11:1x Items 1-4 (figures.rs): struct Bound (link map, server SlotHashes, measured lost and whole pairs; methods
  sent, link) and const LINK_GAPS (the eight phrases, the row's order) after `coverage`; fn bound: slot_hashes'
  refusals (server_hash, decoded_hash), the room logs' clients (one value, else "its room logs disagree on clients"),
  then one walk that keeps each gap's first occurrence and returns the first by index (row order, each note its own
  phrase only); the measured window is `t.ph != Phase::Measured => continue` on the clients' (source) records; figures
  A (`sent && link == Some(0)` over lost) and B (`link >= 1` over whole; no room record skipped; the sentinel left out
  where whole is built), never Empty; two FIGURES entries after decoded_hash_compared_count, unit C62H-BOUND, `what`
  as the brief's item 4. Item 5 (engine.rs): bound_mark, BoundTick, bound_run (on hash_log), bound_out, ab,
  UNEXPLAINED; the six tests after u4j_an_empty_comparison_fails_the_run_and_shows_no_bite; import line gains RunOut
  and a `use npt_log::Phase::{After, Measured, Warmup}` line (no assertion touched).
- 11:11-11:18 cargo fmt --all --check: diffs (step2-fmt-1..3.log) applied by Edit (and the code reshaped to rustfmt's
  60-column call/chain widths); step2-fmt-4.log exit 0. The new tests, measured build (step2-fmt-4.log, `cargo test
  -p npt-judge --test engine c62hb`): "test result: ok. 6 passed; 0 failed".
- 11:18 clippy -D warnings flagged `[&dyn Fn(&mut [Log]); 8]` (type_complexity, step2-judge-pre.log); now
  `[fn(&mut [Log]); 8]`. step2-judge-pre-2.log: fmt, clippy (measured and --all-features, all targets) exit 0;
  `cargo test -p npt-judge` exit 0: engine "49 passed; 0 failed" (43 + 6), fixtures "19 passed", the rest unchanged.
  No existing judge test or fixture answer moved (none in existing_judge_answers_moved).
- 11:20 sha256 before the plants (.gate/u4r/c62h-bound/step2-sha-before-plants.txt): figures.rs 555a8fe1…,
  engine.rs ec03cf04…. Each plant: one Edit, `cargo test --locked -p npt-judge --test engine <home>` through the
  runner (-x, measured build), restore by Edit, `sha256sum -c` of the record (both OK each time).
- CB-P8 (figures.rs bound's client loop, `t.ph != Phase::Measured` as `t.ph == Phase::Warmup`; plant-CB-P8.log):
  "test result: FAILED. 0 passed; 1 failed", c62hb_warmup_and_after_phase_ticks_are_not_read at engine.rs:1624, its
  pair assert left ["Fail 1", "Fail 1"] right ["Pass 0", "Pass 0"]: (A, B) = (1, 1). Restored; sha equal.
- CB-P9 (bound's `!t.hs.contains_key("server_hash")` gap dropped; plant-CB-P9.log): "1 failed",
  c62hb_eight_gaps_read_on_both_figures at :1677, case "a room tick record without hs.server_hash": valid, A Pass 0,
  B Pass 0 where both Gap are wanted. Restored; sha equal.
- CB-P10 (where whole is built, `h.is_some_and(|(h, _)| h != 0)` as `h.is_some()`; plant-CB-P10.log): "1 failed",
  c62hb_the_converse_and_the_sentinel at :1632 (the sentinel's assert): left ["Pass 0", "Fail 1"]: B 1. Restored.
- CB-P14 (figure A `b.sent(slot, t) &&` dropped; plant-CB-P14.log): "1 failed", c62hb_each_explanation_alone_reads_0
  at :1610, left ["Fail 1", "Pass 0"]: A 1 (the plant's own dead-code warnings: `sent`, `server` unused). Restored.
- CB-P15 (figure A `&& b.link(slot, t) == Some(0)` dropped; plant-CB-P15.log): "1 failed", the same test at :1610,
  left ["Fail 1", "Pass 0"]: A 1. Restored; sha equal.
- CB-P16 (bound's `if link.is_empty() { gap(0, ..) }` dropped; plant-CB-P16.log): "1 failed",
  c62hb_eight_gaps_read_on_both_figures at :1680 (the phrase assert), case 1's note "run c1: tick 1: a room tick
  record without tick.s.snap_link_lost_count" lacks "no room log gives tick.s.snap_link_lost_count". Restored.
- CB-P17 (bound's `gap(1, ..)` in the let-else dropped; plant-CB-P17.log): "1 failed", the same test at :1680, case
  2's note "run c1: slot 1, tick 1: a measured incomplete tick the room log has no record of" lacks its phrase.
  Restored; sha equal.
- CB-P18 (bound's `l.len() != clients` check of tick.s.snap_link_lost_count dropped; plant-CB-P18.log): "1 failed",
  the same test at :1677, case "a list that does not hold the run's clients entries" (tick 2's list [0]): valid,
  Pass 0 / Pass 0 where Gap is wanted. Restored; both files equal the record; no plant in place; no untracked file.
- 11:22 New tests by name, measured build (step2-new-measured.log, `cargo test --locked --workspace c62hb`, -x): exit 0;
  engine "6 passed": c62hb_a_bite_with_an_unexplained_tick_is_shown, c62hb_each_explanation_alone_reads_0,
  c62hb_an_unexplained_tick_reads_1, c62hb_warmup_and_after_phase_ticks_are_not_read,
  c62hb_eight_gaps_read_on_both_figures, c62hb_the_converse_and_the_sentinel; step 1's l1 "1 passed", lab "6 passed".
- 11:23 Bites build (step2-new-bites.log, `--all-features`): exit 0; engine the same six, "6 passed"; l1 "1 passed";
  lab "8 passed" (step 1's, the two short P2 bites among them).
- 11:24 Staged: crates/judge/src/figures.rs (+170 -0), crates/judge/tests/engine.rs (+149 -1); step numstat 320 (plan
  255-355). check.mjs whole started (step2-check.log, -x).
- 11:25 Review of the staged diff: the frozen row counts "the (slot, tick) pairs" (brief 3.2 item 2: "each measured
  (slot, tick)"); `lost` was a Vec, so one pair filed lost by two client logs would count twice (`whole` comes from
  slot_hashes' map, already one per pair). Not reachable on a valid L1 run (one client log), but the row is exact:
  `lost` becomes a BTreeSet after this check.mjs run ends; then fmt, the eight plants again on the final code, the
  named tests in both builds, re-stage, and check.mjs whole again as the last run (step2-check-2.log).
- 11:35 The first check.mjs (step2-check.log, on the Vec version): exit 0, "check: all 11 steps pass"; test step
  "measured build: 551 tests passed, 0 failed, 0 reported a skip; bites build: 631 tests passed, 0 failed, 0 reported a
  skip"; header diff 7845b463…, untracked e3b0c442… (none). Superseded by the run below (the staged bytes changed).
- 11:36 figures.rs: Bound.lost `BTreeSet<(usize, u64)>`, `lost` built as a BTreeSet, `1 => { lost.insert(..); }`
  (+2 lines). step2-judge-final.log: fmt, clippy (both builds, all targets) exit 0; `cargo test -p npt-judge` exit 0
  (engine 49 passed, fixtures 19 passed).
- 11:37-11:39 The eight plants again on the final code (record step2-sha-before-plants-2.txt: figures.rs 338fa93d…,
  engine.rs ec03cf04…), same edits, logs plant-CB-P8-2, -P9-2, -P10-2, -P14-2, -P15-2, -P16-2, -P17-2, -P18-2.log:
  each "test result: FAILED. 0 passed; 1 failed" naming its home test at the same line and cause as the first round
  (P8 :1624 ["Fail 1", "Fail 1"]; P9 :1677 valid Pass 0 / Pass 0; P10 :1632 ["Pass 0", "Fail 1"]; P14 and P15 :1610
  ["Fail 1", "Pass 0"]; P16 :1680 case 1's note gap 2's words; P17 :1680 case 2's note gap 8's words; P18 :1677 valid
  Pass 0 / Pass 0). Each restored by Edit, `sha256sum -c` OK after each (CB-P9 was once planted before P8's check; it
  was undone, P8's restore checked OK, then P9 planted again). No plant in place.
- 11:39-11:40 Named tests on the final code: step2-new-measured-2.log (exit 0; engine the six, "6 passed"; l1 1; lab
  6) and step2-new-bites-2.log (exit 0; engine the six, "6 passed"; l1 1; lab 8).
- 11:40 Re-staged; step numstat figures.rs +172 -0, engine.rs +149 -1 = 322; lane against 044fab1 = 821. check.mjs
  whole started on these staged bytes (step2-check-2.log).
- 11:52 check.mjs whole, the last run on the staged bytes (.gate/u4r/c62h-bound/step2-check-2.log): exit 0, "check: all
  11 steps pass"; test step "measured build: 551 tests passed, 0 failed, 0 reported a skip; bites build: 631 tests
  passed, 0 failed, 0 reported a skip" (+6 / +6 from 545 / 625, as expected). Header: diff f88bf7d56b845c33…, untracked
  e3b0c44298fc1c14… (none). `git diff --cached HEAD -- . | sha256sum` from spikes/netcode-proof = f88bf7d5… (equal);
  nothing unstaged, no untracked file.
- 11:53 SIZE-CHECK (staged, results excluded): step vs HEAD 322 (figures.rs +172 -0, engine.rs +149 -1); lane vs
  044fab1 821 (+804 -17: figures.rs 172, engine.rs 150, l1 lib.rs 33, l1 tests.rs 98, logs.rs 54, scenario.rs 311,
  SERIES.md 3). Under the 1,050 stop; step 2 inside its plan (255-355).

## State (launch 2, step 2, 11:53 UTC: a clean point; nothing in flight, no plant in place)
- Staged in the worktree (branch c62h-bound-build at 54046d4, nothing committed): crates/judge/src/figures.rs (+172 -0)
  and crates/judge/tests/engine.rs (+149 -1). Step 322; lane 821 of 1,050.
- Check: step2-check-2.log, exit 0, all 11 steps PASS, diff hash
  f88bf7d56b845c33dd4ea7e659150aeb267f86ca2c53148ed0c0bf8c3381f5ec, untracked hash
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855. (step2-check.log, the Vec version, also all 11
  PASS, 551 / 631; superseded.)
- Counts: 551 measured / 631 bites (+6 / +6). New tests by name, both builds (step2-new-measured-2.log,
  step2-new-bites-2.log): c62hb_each_explanation_alone_reads_0, c62hb_an_unexplained_tick_reads_1,
  c62hb_warmup_and_after_phase_ticks_are_not_read, c62hb_the_converse_and_the_sentinel,
  c62hb_eight_gaps_read_on_both_figures, c62hb_a_bite_with_an_unexplained_tick_is_shown.
- Plants: CB-P8, -P9, -P10, -P14, -P15, -P16, -P17, -P18 each bit as section 4 says, twice (first on the Vec version,
  logs plant-CB-*.log; then on the final code, plant-CB-*-2.log); each restored, sha256 equal to its record
  (step2-sha-before-plants.txt, step2-sha-before-plants-2.txt).
- Existing judge answers: none moved (engine 43 old tests and fixtures 19 pass unchanged; no assertion edited).
- Stop rules: none met.
- Next (the main session): commit 2 in the worktree; check `git diff HEAD^ HEAD -- . | sha256sum` (spikes/netcode-
  proof) = f88bf7d5…; the committed SIZE-CHECK = 821; then insert part 3 and the built check before step 3's launch.

## Main session after launch 2
- 11:53 Launch 2's return saved word for word: c62h-bound-step2-full.json (123 lines, sha256 366a2910…; status done).
  Read in full: 6 new engine.rs tests in each build (551 / 631 = +6 / +6, as expected); 8 plants bitten (re-run on
  the final code after a lost-set fix: the (slot, tick) pairs are a BTreeSet, so a pair is counted once, as the frozen
  row counts pairs); no existing judge answer moved. For the review round: the gap order (slot_hashes' own refusals
  and decoded's "room logs disagree on clients" before the row's eight; figures.rs:565-655); "an entry below 0" read
  on the link lists only; the notes carry the phrases without the row's backticks; deviations: the reader sits after
  decoded's helper `coverage`, engine.rs's imports gained RunOut and Phase's three names.
- 11:54 **Commit 2: b423a7d9231f68c87a1af9004dc1f2c218b43948.** `git diff HEAD^ HEAD -- . | sha256sum` =
  f88bf7d56b845c33… = step2-check-2.log's header; nothing unstaged; untracked none. SIZE-CHECK from LANE_BASE: **821**
  (+804 -17; step 2 alone 322: figures.rs 172, engine.rs 150). Patch: c62h-bound-commit2.patch (387 lines, addbe9d9…).
- 12:19 **Insert part 3 on the worktree** (.gate/u4r/c62h-bound/insert-part3.log): exit 0; marks.json 9fb5be57… →
  c1935a5b…, SERIES.md 89295b57… → 6898e119…, README.md b49f3441… → f3c3c804… (all three the brief's expected; the
  json.loads check equal; README refilled into 16 lines, the longest 120); numstat marks.json +26 -0, SERIES.md +4 -2,
  README.md +16 -11 (59).
- 12:19 **The built check after part 3** (HEAD b423a7d plus part 3, unstaged; built-check-b423a7d9231f.log): exit 0,
  **25 PASS, 0 FAIL** (texts v3, freeze, paste, 8 blocks; both marks blocks once; the four rows as whole lines; both
  headers; the sentence; the three hashes; the README word diff adds the sentence's 73 words in order, removes
  nothing); marks.json after c1935a5b996fb0aa….
- 12:20 Step 3's launch (wf_009969c7-069): B-2's four files with the real marks.json, judge tests before and after,
  check.mjs, expected +1 / +1 → 552 / 632; the lane about 880 before B-2.

## Launch 3 (step 3)
- 12:21 Start. Read brief sections 0, 1, 3.2, 3.3, 4, 10, 11 (sha256 7936074c… checked); runner u4r-run.sh e9437ae2…
  checked (header read). Worktree at b423a7d (commit 2), unstaged ` M` marks.json c1935a5b…, SERIES.md 6898e119…,
  README.md f3c3c804… (the main session's part 3; equal to the expected). Plan: (1) the judge's tests before any edit
  in both builds, and each named case's reading by the judge's own CLI on scratch copies (written with Write, each
  `cmp`-equal to its fixture) under .gate/u4r/c62h-bound/b2-before/; (2) B-2's four files; (3) the judge's tests and
  the readings again; (4) stage; (5) check.mjs whole; (6) SIZE-CHECK.
- 12:23 Item 1, the judge's tests before any edit (.gate/u4r/c62h-bound/step3-judge-before.log, -x -k, `cargo test
  --locked -p npt-judge` then `--all-features`): exit 101 in both builds; lib 14, cli 2, engine 49 passed; fixtures
  "test result: FAILED. 18 passed; 1 failed" in each build, the one failure
  pm_0001_17_equal_hashes_pass_and_the_lost_group_is_not_compared at fixtures.rs:229 on `base`: INVALID, "a gap in the
  record: run p2-1: no room log gives tick.s.snap_link_lost_count" on both new checks (its room log has no series).
  No other test fails. (Cargo stops after the failing target, so the doc-tests did not run before; the judge has none
  of note: the after-run shows them.)
- 12:24 Item 1, the readings by the CLI (step3-readings-before.log, exit 0; verdicts b2-before/*.json): base INVALID
  (both new checks gap 1); lost-group INVALID (both gap 1; the brief's "FAIL (A 1)" is its reading once base's room
  log carries the series: computed A 1 with zero links, shown after the edits); hashed-and-incomplete INVALID (its
  coverage gap, plus gap 1 on both new checks; its test still passes before the edits; FAIL (A 1) is again the reading
  with the series); none-compared FAIL, valid, the empty comparison, both new checks gap 1 beside (A 6 with the
  series). A read-only scan of every -17 case (base files plus the case's): every measured client record holds
  `snap_incomplete`, every entry 0 or 1, every incomplete tick at a room tick (1 to 3): launch 2's gaps 6 and 8 meet
  no fixture. With zero links, A is 0 on every case but hashed-and-incomplete (1), lost-group (1) and none-compared
  (6); B 0 on all. So nothing beyond B-2's named cases moves: edits go ahead.
- 12:26 Item 2, B-2's four files (Edit and Write only): fixtures/pm-0001-17/base/p2-room.jsonl:2-4, each tick record's
  `s` gains `"snap_link_lost_count"` with 48 zeros (one replace_all Edit; checked 48 entries, all 0, on ticks 1-3; +3
  -3); lost-group/p2-room.jsonl (new, 7 lines): base's new room log with slot 1's entry at tick 2 set to 1 (diff
  against base: that one entry only); tests/fixtures.rs: `hashed-and-incomplete` out of
  pm_0001_17_every_server_hash_is_resolved_on_the_client_side's INVALID list (-4), its doc line :390-391 "Each case is
  INVALID (both at once: the next test)" (+1), and the new test
  pm_0001_17_hashed_and_incomplete_fails_with_its_coverage_gap_beside at :425-437 (L1 P2 FAIL; a why line holding
  "(0 unexplained) = 1, needs == 0"; the run's `gaps` holding "slot 1, tick 2: a client hash and
  tick.s.snap_incomplete 1 at once"); fixtures/README.md rows :22 (base: the room log's series 0 on every slot), :23
  (lost-group: its own room log, the link lost a datagram of it, E2) and :32 (hashed-and-incomplete: FAIL, A 1, the
  coverage gap beside, INVALID before). numstat: fixtures.rs +16 -5, README +3 -3, base room +3 -3, lost-group +7.
- 12:27 Item 2, the judge's tests after (step3-judge-after.log, -x -k): `cargo fmt --all --check` exit 0 (no diff);
  `cargo test --locked -p npt-judge` exit 0 and `--all-features` exit 0: lib 14, cli 2, engine 49, fixtures "test
  result: ok. 20 passed; 0 failed" (19 + the new one, named in both builds), doc-tests 0, in each build.
- 12:28 Item 2, the readings after (step3-readings-after.log, exit 0; verdicts b2-after/*.json; scratch inputs
  `cmp`-equal to the fixtures): base NOT-RUN, the run valid, 0 mismatches, 6 compared, 0 incomplete, A 0, B 0 (as
  before this lane); lost-group NOT-RUN, valid, 0 mismatches, 5 compared, 1 incomplete, A 0, B 0 (as before this
  lane); hashed-and-incomplete FAIL, valid, A fail 1, B 0, `gaps` = the coverage gap; none-compared FAIL, valid, the
  empty comparison, A fail 6, B 0. Also lost-group's client log on base's new room log (no room log of its own): FAIL,
  A 1, which is why it needs its own (the brief's "FAIL (slot 1's tick 2: A 1)").
- 12:29 Item 3, staged (from a subshell in the worktree): marks/marks.json, marks/SERIES.md, marks/README.md (the main
  session's part 3, untouched) and B-2's four. sha256 after staging, of the files and of the staged blobs alike:
  marks.json c1935a5b996fb0aa…, SERIES.md 6898e119109a794a…, README.md f3c3c804c4e4d078… (all three as expected).
  Nothing unstaged, nothing untracked; `git diff --cached HEAD -- . | sha256sum` from spikes/netcode-proof =
  a1238e901f4a5885… (equal to step3-check.log's header).
- 12:29 Item 6, SIZE-CHECK on the staged work (results excluded): step vs HEAD 99 (+75 -24: fixtures README 6, base
  room 6, lost-group room 7, fixtures.rs 21, marks.json 26, SERIES.md 6, marks README 27); B-2's four files 40 (the
  brief's "about 40"); lane vs 044fab1 920 (+879 -41), under the 1,050 stop. Item 4, check.mjs whole started 12:29:32
  (step3-check.log, -x).
- 12:42 Item 4, check.mjs whole on the staged bytes (.gate/u4r/c62h-bound/step3-check.log): exit 0, "check: all 11
  steps pass" (fmt first: PASS, no diff); test step "measured build: 552 tests passed, 0 failed, 0 reported a skip;
  bites build: 632 tests passed, 0 failed, 0 reported a skip" (+1 / +1 from 551 / 631, as expected); marks "81 marks
  read and checked"; refs 1291 tracked files and redact 297 items (each +1: the new lost-group room log). Header:
  diff a1238e901f4a5885…, untracked e3b0c44298fc1c14… (none). After the run: nothing unstaged, no untracked file, the
  staged hash and the three marks hashes unchanged.
- Item 5, the new test by name from the test runner's own output (step3-judge-after.log :102 measured, :228
  `--all-features`; check.mjs prints no per-test lines): pm_0001_17_hashed_and_incomplete_fails_with_its_coverage_gap_
  beside, in both builds.

## State (launch 3, step 3, 12:43 UTC: a clean point; nothing in flight, no plant in place)
- Staged in the worktree (branch c62h-bound-build at b423a7d, nothing committed): marks/marks.json (+26 -0,
  c1935a5b…), marks/SERIES.md (+4 -2, 6898e119…), marks/README.md (+16 -11, f3c3c804…) as part 3 left them; B-2's
  crates/judge/fixtures/pm-0001-17/base/p2-room.jsonl (+3 -3), .../lost-group/p2-room.jsonl (new, +7),
  crates/judge/tests/fixtures.rs (+16 -5), crates/judge/fixtures/README.md (+3 -3). Step 99 (B-2's four 40); lane 920
  of 1,050.
- Check: step3-check.log, exit 0, all 11 PASS, diff hash
  a1238e901f4a5885ed12391c3988b535171021a5e8d57aab295ce8191c0e215c, untracked hash
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855.
- Counts: 552 measured / 632 bites (+1 / +1); the new test pm_0001_17_hashed_and_incomplete_fails_with_its_coverage_
  gap_beside in both builds.
- Answers: before the edits only pm_0001_17_equal_hashes_pass_and_the_lost_group_is_not_compared failed (base INVALID,
  gap 1), in both builds; after them base and lost-group read as before this lane, hashed-and-incomplete FAIL (A 1, the
  coverage gap beside), none-compared FAIL (A 6). No other answer moved. Scratch inputs and verdicts:
  .gate/u4r/c62h-bound/b2-before/, b2-after/ (outside the worktree).
- Stop rules: none met.
- Next (the main session): commit 3 in the worktree (message from brief section 1); `git diff HEAD^ HEAD -- . |
  sha256sum` from spikes/netcode-proof = a1238e90…; the committed SIZE-CHECK = 920; the built check again; then step
  4's launch.
