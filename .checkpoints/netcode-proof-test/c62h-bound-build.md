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
