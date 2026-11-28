## C62H-BOUND (every lost tick on L1 P2, P3 and B1 explained by the room's or the link's record)

_The lane C62H-BOUND: Mustafa's C62H-BOUND (a) of 2026-11-15, 16:35 (history.md:2143-2146) and his C62H-BOUND-YES (a)
of 20:50 to texts v3 and the planned size, with his P0 re-judge added (history.md:2172-2175). Built under the brief
c62h-bound-brief.md (663 lines, sha256 7936074c15c5eb49ed3d8e0e130eaf0d75ce6f30067f650072caa764c3414857, unchanged:
C8). This report is a section of u4r-report.md, as BRAWL-1's is, so it sits beside the two lines it replaced under
"The re-run sets" (:997 and :1008). It carries brief section 9 item by item, each marked PASS or FAIL, and the
advisor's P3 ruling's order_to_the_report items 4-9 (advisor-c62h-bound-p3-full.json, sha256 facf298e…, conditions
P1-P7). Times are UTC, 2026-11-16 unless named, except Mustafa's own word times, which are Amsterdam time as his
messages give them: his 16:35 and 20:50 of 2026-11-15 (15:35 and 19:50 UTC), his 13:35 rule of 2026-11-15 (12:35
UTC), his 21:50 and 22:09 words of 2026-11-16 (20:50 and 21:09 UTC). Card line numbers are those of the card at
67de11e._

**In short:**
- **What was built.** The test lab now records, for each client and each tick, how many snapshot packets the simulated
  network lost, and checks that count one to one against the packets it sent. The judge gained two checks on
  PM-0001-17 (each decoded snapshot is the server's state), read on L1 P2, P3 and B1:
  - figure A (snap_lost_unexplained_count): every measured tick a client files as lost must be explained, either the
    room sent nothing that tick or the network lost a packet of it;
  - figure B (snap_whole_despite_link_loss_count): no measured tick may arrive whole while the network says it lost a
    packet of it.
  Each check has its planted bug (whole-as-lost on A, link-lost-overstated on B), and the lab's summary line now
  counts its own lost ticks by reason: the room sent nothing (E1), the network lost a packet (E2), neither (N).
- **What ran.** Four commits; one review round (12 Lows, nothing Medium or above); the merge step (its first counted
  check failed on an older network-layer test this lane did not touch; the second passed whole); Mustafa's P0
  re-judge; a short pre-flight; the judged P2 set, then the P3 set.
- **The result.**
  - P0 re-judge: **SAME ANSWERS** (the new judge gives the kept P0 set the verdicts it had).
  - **P2 counts and P3 counts** (the advisor's P3 ruling, P1). On all six clean ten-minute runs both new checks read
    0, and the lab's own count of unexplained ticks (N) equals the judge's (A): 0 every time. Each planted bug reads
    exactly 48 on its own check, with every other check passing and no gap.
  - Two P3 runs (s1, s3) carry the overflow-resync flag (carry 58); the advisor traced every one of their "room sent
    nothing" ticks to a forced full state waiting for its acknowledgement, and both count.
  - No stop rule of Mustafa's was met (section 12 below).
- **Size:** 967 changed lines, above the plan (about 650) and the brief's estimate (650-875), under the 1,050 stop.
- **Still unbounded** (section 16): -58's two wrap parts, -17's real-network parts, -17's B1 part until U4g, the
  minutes outside each measured window, and a double fault. One decision: C62HB-Q4 (b), at the end.

Key to the per-run figures used below: S = measured snapshot datagrams; L = lost by the link (bursts included);
bursts B = the part of L lost in bursts; E1 + E2 + N = the measured incomplete count. L counts lost plus burst-lost
datagrams, so bursts B are inside L (the netcode-architect's point (a), confirmed); on P2 and P3 bursts B are 0.

### 1. The four commits, the fix pass and the size (SIZE-CHECK): PASS

- **The commits** (built in the worktree .claude/worktrees/c62h-bound, branch c62h-bound-build, from LANE_BASE
  044fab188419e8859dbca699d851563c44c078c9; each commit's diff hash equals its check log's header):
  - 1/4 npt-l1's server_datagrams_since and npt-lab's link record: **54046d4a83e445a212670dd548d5a5c1b9dd00ef**
    (11:01), cherry-picked as **eb2bebfb712e8e8a40da3a3f306702d10039075c**; diff hash 0b41121e…; size 499.
  - 2/4 npt-judge's two figures and one shared reader: **b423a7d9231f68c87a1af9004dc1f2c218b43948** (12:18), as
    **b06991e1328f06a2f4c12ab8f18762b1eadf1cf3**; diff hash f88bf7d5…; size 322.
  - 3/4 PM-0001-17's two checks and one planted bug, SERIES.md and README.md, B-2's four files:
    **b01d3ac0c42701002a2239ad0a89a841f56464a1** (14:23), as **15102d1242afb37d686e624fa10dad2d481b476d**; diff hash
    a1238e90…; size 99.
  - 4/4 the set runner plans -17's two bites on P2: **42b964618eed741f65ac04c1e5a06da0a82524d0** (15:36), as
    **561bf42b8996bbc2ad798cfc27c39c05222524c8**; diff hash 7b775057…; size 47.
  - Pushed head **ddd38bace3c63050d2087a1acf3a6e74c1d2ecb9** (16:49; the card, the notes, the merge-stop ruling and the
    set-check script on top of 561bf42). Patch-ids equal pair by pair (the advisor's head check: 036447a4, 654d080f,
    77464093, f1d1de9f); `git diff --quiet c62h-bound-build 561bf42 -- spikes/netcode-proof` (results excluded) exit
    0. Each lane commit is also kept as a patch on the branch (c62h-bound-commit1.patch to -commit4.patch).
  - **No fix pass:** the review round's Lows were deferred in writing (section 15), so commit 4 is the last code
    commit.
- **SIZE-CHECK** (`git diff --numstat 044fab1 <step> -- spikes/netcode-proof/`, results excluded), per step against
  the brief's plan per commit (brief section 4): step 1 **499** (about 285-400); step 2 **322** (about 255-355); step 3
  **99** (about 100); step 4 **47** (about 10-20). Lane total after each step: 499, 821, 920, 967.
- **The total: 967 changed lines (+904 -63) at 42b9646 / 561bf42** (`efe8401..561bf42` gives the same +904 -63).
  - Against the plan Mustafa said yes to (about 650, range 545-755; history.md:2172): above its top by 212.
  - Against the brief's estimate (about 650-875; brief section 11): above its top by 92.
  - Under the 1,050 stop by 83 (brief section 11; his stop line, history.md:2172). About 1.5 times the plan.
  - What grew: step 1 (scenario.rs alone 311: the five join tests, the short P2 test and its two bite tests) and
    step 4 (33 of its 47 lines are the runner's two header bullets refilled at 120 columns; the netcode-architect
    notes an insert-only edit would have been about 6).
- **Per file** (14 files): l1 lib.rs 33, l1 tests.rs 98, lab scenario.rs 311, lab logs.rs 54, judge figures.rs 172,
  judge tests/engine.rs 150, judge tests/fixtures.rs 21, judge fixtures/README.md 6, fixtures/pm-0001-17/base/
  p2-room.jsonl 6, fixtures/pm-0001-17/lost-group/p2-room.jsonl 7 (new), marks/marks.json 26, marks/SERIES.md 9,
  marks/README.md 27, ops/runs/c-u4r-runs.mjs 47.
- **Counted, beyond the paste's table:** B-2's four files, 40 lines (fixtures.rs 21, fixtures/README.md 6, base
  p2-room.jsonl 6, lost-group p2-room.jsonl 7; brief section 11, B-2).
- **Not counted:** the committed evidence under results/ (results/u4r/8ee453c22882/P2/, results/u4r/d66e23cd78f2/P3/,
  results/c62h-bound/p0-rejudge-93509dfd7fd1/a87c7ec39b96/: verdicts, runs.txt, rejudge.txt); and the main session's
  scripts, which are not lane code (the head check's H6; the merge-stop ruling's script condition (iii)):
  c62h_bound_insert.py, c62h_bound_built_check.py, c62h_bound_rejudge_compare.py, c62h_bound_set_check.py and
  c62h_bound_c_lines.py in the notes folder. Also outside the count: launch 4's dry-print harness
  .gate/u4r/c62h-bound/harness/plan-dry.mjs (an evidence harness under .gate/, outside spikes/).

### 2. Each step's check.mjs counts and each new test by name: PASS

Expected from LANE_BASE's 538 measured / 616 bites (TE-CBB-4): +7 / +9, +6 / +6, +1 / +1, 0 / 0, final 552 / 632.
Every count came out as expected. Pass or fail is the exit code and the test runner's own summary line.
- **Step 1** (.gate/u4r/c62h-bound/step1-check.log, 10:46:22-10:58:53, exit 0, all 11 steps): **545 / 625** (+7 /
  +9). New in both builds (step1-new-measured.log, step1-new-bites.log):
  - npt-l1: tests::c62hb_server_datagrams_since_counts_down_datagrams_by_fate;
  - npt-lab: scenario::tests::c62hb_join_one_datagram_too_many, c62hb_join_one_datagram_too_few,
    c62hb_join_groups_with_a_null_server_hash, c62hb_join_groups_with_no_session,
    c62hb_join_ok_counts_lost_and_burst, c62hb_p2_short_every_measured_lost_tick_is_explained;
  - bites build only: c62hb_p2_short_bite_whole_as_lost_gives_16_unexplained,
    c62hb_p2_short_bite_link_lost_overstated_gives_16_whole_despite_loss.
  - The short P2 lab test (16 clients): measured incomplete 29 = E1 0 + E2 29 + N 0; the join's unexplained 0 and
    whole-despite-loss 0; S 1,920, L 29, bursts B 0. Its bites: whole-as-lost N 16; link-lost-overstated join 0 / 16.
- **Step 2** (step2-check-2.log, 11:40:16-11:52:06, exit 0, all 11): **551 / 631** (+6 / +6). New in both builds
  (step2-new-measured-2.log, step2-new-bites-2.log), all in judge tests/engine.rs:
  c62hb_each_explanation_alone_reads_0, c62hb_an_unexplained_tick_reads_1,
  c62hb_warmup_and_after_phase_ticks_are_not_read, c62hb_the_converse_and_the_sentinel,
  c62hb_eight_gaps_read_on_both_figures, c62hb_a_bite_with_an_unexplained_tick_is_shown. The 43 old engine tests and
  19 old fixture tests passed unchanged. (An earlier run, step2-check.log, on the first version with a list in place
  of a set of lost (slot, tick) pairs, also passed 11 of 11; it is superseded.)
- **Step 3** (step3-check.log, 12:29:32-12:42:37, exit 0, all 11): **552 / 632** (+1 / +1). New in both builds
  (step3-judge-after.log :102 and :228): pm_0001_17_hashed_and_incomplete_fails_with_its_coverage_gap_beside.
- **Step 4** (step4-check.log, 14:34:29-14:49:56, exit 0, all 11): **552 / 632** (0 / 0; no new test). Its own proof
  is the dry print: plan-dry.log, "22 asserts, 22 passed, 0 failed" (harness sha256 693bcf3f…, recorded in
  sources.txt before its first run).

### 3. The planted bugs CB-P1 to CB-P18: PASS (all 18 bit)

Each was a temporary Edit (one each, except CB-P6, which took two: a `prev_link` vector, and room_tick passed
`std::mem::replace(...)`; c62h-bound-build.md:79-81), run through u4r-run.sh -x as `cargo test` of its home test, then
restored by Edit with the file's sha256 checked equal to the record. Each failed its home test at the cause brief
section 4 names. Times are the logs' own (the test-engineer asked for these; the notes' "11:0x" and "11:1x" for CB-P5
to -P7 were wrong).
- **Step 1** (10:43:01-10:46:00; record step1-sha-before-plants.txt):
  - CB-P1, CB-P12, CB-P13 (server_datagrams_since counts up-direction entries / drops the datagram filter / ignores
    `from`): "1 failed", c62hb_server_datagrams_since_counts_down_datagrams_by_fate; sent 180, 202 and 120 against the
    independent 90 (plant-CB-P1.log, -P12.log, -P13.log).
  - CB-P2 (the join drops the sent-count comparison): "2 failed", c62hb_join_one_datagram_too_many and _too_few, Ok
    where Err (plant-CB-P2.log).
  - CB-P3, CB-P4 (the join drops the server-hash check / the no-session check): "1 failed" each, Ok where Err
    (plant-CB-P3.log, -P4.log).
  - CB-P5, CB-P6, CB-P7 (the record written as 0 / one room tick late / the tally reading a removed entry): "1 failed"
    each, c62hb_p2_short_every_measured_lost_tick_is_explained, at "the measured link-lost sum is 0", at the join
    assert (unexplained 28, whole despite loss 29) and at "E2 is 0" (plant-CB-P5.log, -P6.log, -P7.log).
- **Step 2** (run twice: on the first version, then on the final code at 11:36:39-11:38:56; records
  step2-sha-before-plants.txt and -2.txt; logs plant-CB-P8.log … and plant-CB-P8-2.log …): "1 failed" each:
  - CB-P8 (the window "not warm-up" in place of "measured"): c62hb_warmup_and_after_phase_ticks_are_not_read, (A, B)
    = (1, 1) at engine.rs:1624;
  - CB-P9 (a missing hs.server_hash read as null) and CB-P18 (no length check on the link list):
    c62hb_eight_gaps_read_on_both_figures, Pass 0 where a gap is wanted (:1677);
  - CB-P10 (figure B counts the sentinel): c62hb_the_converse_and_the_sentinel, B 1 (:1632);
  - CB-P14, CB-P15 (figure A drops E1 / drops E2): c62hb_each_explanation_alone_reads_0, A 1 (:1610);
  - CB-P16, CB-P17 (gap 1 / gap 2 dropped): c62hb_eight_gaps_read_on_both_figures, the case's note lacks its phrase
    (:1680).
- **Step 4** (14:33:59): CB-P11 (link-lost-overstated planned on P0 too): plan-dry exit 1, "17 passed, 5 failed", P0
  at 7 bites against 6 (plant-CB-P11.log); restored, sha256 f2002890… equal.
- The judged planted bugs (marks.json's) were read in the P2 set: 48 and 48 (section 10). The test-engineer read all
  18 plant logs and found every pre-plant sha256 equal to the committed blob.

### 4. The built check and the three shown-to-fail runs: PASS

- **The built check** (c62h_bound_built_check.py, sha256 4e35e703…), brief section 2:
  - shown to fail first on LANE_BASE (built-check-negative.log): exit 1, **12 PASS, 13 FAIL** (texts and freeze pass;
    every built text, header, hash and the word diff fail, as required);
  - after insert part 3 on b423a7d (built-check-b423a7d9231f.log), on b01d3ac (built-check-b01d3ac0c427.log) and on
    the merged head 561bf42 (built-check-561bf42b8996.log): **25 PASS, 0 FAIL** each.
  - The freeze re-checked each time: texts v3 3f21c9b7…, the freeze 9cb8be17…, the paste cfac7502…, its 8 blocks
    each by sha256; each frozen block once, whole, in the built files.
  - The three "after" hashes: marks.json **c1935a5b996fb0aaff82b0b3c1c604255e1534f6ca7e8d2e55a9f472cd3b22a9**,
    SERIES.md **6898e119109a794a7779ad0395301e48a461848b60cfdc16431ec64faad87460**, README.md
    **f3c3c804c4e4d078ca07461f956eb60bc96e74eb4ed507bf28f652383a5742fe**; README's word diff adds only the
    sentence's 73 words and removes nothing.
- **The re-judge comparison** (c62h_bound_rejudge_compare.py): its --self-test before the comparison
  (rejudge-compare-negative.log): **SELF-TEST PASS, 4 of 4** (the expected additions give SAME ANSWERS; a changed -17
  L1 P0 value gives that one difference; the new bite entry twice is refused; a missing why line gives a difference).
- **The set check** (c62h_bound_set_check.py, sha256
  **e9eb9b2363a587c632e2092facc2fba057e966205a41e2f74bfde26296c29ea8**, the bytes committed at ddd38ba: C5): after
  P2's set, before its first real use (set-check-negative.log):
  - on the old results/u4r/fc411116c1f1/P2: **12 PASS, 29 FAIL**, all 29 lines that name the new phrase or the new
    checks FAIL (the two dry runs before it, set-check-dry.log and -dry-2.log, gave the same);
  - its --self-test on the new P2: **SELF-TEST PASS** (the planted-bug rule passes as the file is and fails with one
    planted gap; each bite line passes as the file is and fails with its "late" field bumped).

### 5. The review round and the marks.json and judge read (his 13:35 rule): PASS

- **Review round 1** (15:38-16:03; test-engineer, netcode-architect, rust-reviewer, each at xhigh, on
  044fab1..42b9646): all three **APPROVE WITH FINDINGS**, 12 findings, all Low, none touching a frozen text or what a
  figure computes (c62h-bound-review-test-full.json 3bf8888c…, -netcode-full.json 5adfdf18…, -rust-full.json
  3f39ade6…; each read in full). Ten deferred in writing (section 15); TE-CBH-1 and TE-CBH-2 fixed in the main
  session's set-check script (not lane code, not counted; the merge-stop ruling accepted both).
- **The marks.json and judge read, its own line (history.md:2127):** read by the **test-engineer** in review round 1
  (notes .checkpoints/netcode-proof-test/c62h-bound-review-test.md; the read_line field of
  c62h-bound-review-test-full.json).
  - marks.json lines read (the built file at 42b9646): PM-0001-17 whole at :3845-4047; the two checks at :3983-3993
    and :3994-4004 (byte-equal to the frozen block, cea24dd8…); the bite link-lost-overstated at :4035-4038
    (byte-equal to its frozen block, d5945f92…). Parsed, the built file is LANE_BASE's plus exactly those three
    objects.
  - Judge lines read: figures.rs :200-536 (context), :538-698 (the reader, its eight gaps and both figures), :805-830
    (the two FIGURES entries); engine.rs :1-10, :1296-1340, :1555-1700 (the six new tests); eval.rs :1176-1233 and
    :1440-1484 (context); B-2's fixtures.rs :13-40, :200-300, :380-440, fixtures/README.md :19-35, and the fixture
    files. It read SERIES.md and README.md too.
  - marks.json sha256 before **9fb5be578b27d4c378a890d297c5ba636af0ac239bb6d50eae157ed44884a3c5**, after
    **c1935a5b996fb0aaff82b0b3c1c604255e1534f6ca7e8d2e55a9f472cd3b22a9**.
  - `git diff --stat 044fab1..42b9646 -- spikes/netcode-proof/marks/marks.json spikes/netcode-proof/crates/judge` at
    the close: 7 files changed, 376 insertions, 12 deletions. No fix pass, so no fix-pass read; nothing under those
    paths changed after the read (the merged bytes equal the reviewed bytes; P6's spike-tree check).
- **B-2's changed fixture answers, each with its why** (brief section 11, B-2; the read_line's b2_changed_answers):
  - hashed-and-incomplete: **INVALID to FAIL** on -17's L1 P2 part, A 1, B 0, the run valid, its coverage gap ("a
    client hash and tick.s.snap_incomplete 1 at once") kept beside the failure. Why: with the series in the room log,
    the new check reads the tick nothing explains. The only changed answer.
  - base: unchanged (NOT-RUN, valid, 0 mismatches, 6 compared); its room log gains the series, 48 zeros on each of
    its 3 ticks. Why: without the series the real marks.json reads it INVALID by gap 1.
  - lost-group: unchanged (NOT-RUN, valid, 5 compared, 1 incomplete); its own new room log has slot 1's entry at tick
    2 set to 1, so the link explains its lost tick (E2). On base's room log it would read FAIL, A 1.
  - none-compared: still FAIL by the empty comparison, now with A 6 beside it; no edit.
  - No other existing judge test's assertion moved: the lines removed are engine.rs's import line, one doc line
    (fixtures.rs:390, reworded) and hashed-and-incomplete's 4-line entry in the INVALID list.
  - As recorded before the edits (step3-readings-before.log): the real marks.json read base, lost-group and
    hashed-and-incomplete INVALID by gap 1 and none-compared FAIL with gap 1 beside; only base's test failed
    (fixtures.rs:229), in both builds.

### 6. The merge step (C1-C8): PASS on the second counted call; the first stays FAIL

- **The cherry-picks** (16:05, main folder): eb2bebf, b06991e, 15102d1, 561bf42; each lane commit touches only
  spikes/netcode-proof/; the merged bytes equal the reviewed bytes (exit 0).
- **The first counted call: FAIL** (.gate/u4r/c62h-bound/merge.log, 16:06:04-16:16:23, exit 1; C1). check.mjs's
  test step failed: the measured build 552 passed, 0 failed; the bites build 429 passed, **1 failed**: npt-net's
  l2::delay::tests::under_loss_the_held_bytes_stay_within_64_kb_and_the_front_records_acknowledged_prefix. check.mjs
  ran its later steps (deps, wasm, refs, marks, self, redact: all PASS; every other step passed too) and ended "check:
  1 step(s) failed". The main-run.sh call stopped there, so det-u4, the release build and the hash never ran. The
  lane stopped as the brief says ("a check.mjs step fails that no edit of this lane explains"), retried nothing, ran
  no gate and pushed nothing.
  - **Where the failure is kept (H3):** merge.log:51-52 (the test's name), c62h-bound-build.md:428-435 (the
    assertion, "held <= 64 KB + the front record's acknowledged prefix", left [1, 0], right [0, 0]) and the merge-stop
    ruling. The step's whole output (.gate/netcode-proof-test/failed-steps/test.log, 1,577 lines) is gone:
    merge-2's check.mjs deleted it by design when its own test step began (check.mjs:2655-2657, FX-10).
  - **The advisor's merge-stop ruling** (16:17-16:30; advisor-c62h-bound-merge-stop-full.json, sha256 55ad3561…):
    CONTINUE WITH CONDITIONS C1-C8. A timing false positive in U3c's test-only held-bound check, not this lane's.
    **npt-lab links npt-net through npt-server and npt-bots, whose sources never name DelayStream or l2::delay;
    crates/net is byte-equal to LANE_BASE.** (H4, ADV-CBH-4.) No manifest or lockfile changed; the same test passed
    in the same run's measured build and reads "... ok" in 96 other logs under .gate/.
- **The second counted call: PASS, whole** (merge-2.log, 16:31:00-16:48:07, exit 0), every pass-bar item:
  - header: free 5,278 MiB, HEAD 561bf42, diff and untracked e3b0c442… (none): PASS;
  - check.mjs exit 0, "check: all 11 steps pass"; test "measured build: 552 tests passed, 0 failed, 0 reported a
    skip; bites build: 632 tests passed, 0 failed, 0 reported a skip" (622.2 s): PASS;
  - det-u4 PASS (215.5 s), non-cargo time 204.2 s (npt-lab hash 200.8, Wasm 2.5 and 0.9) under 240: PASS;
  - the release build exit 0 (9.36 s): PASS;
  - "regenerated SS-P0-s4: 16950444 bytes, sha256 1f00d2d5f70a2c7b6836ede611b5f1c281319a5312060ead91e016da3c0b1b42,
    as the manifest's" and "9000 ticks compared, 0 mismatching, equal (SS-P0-s4 9000)": PASS;
  - the timed hash call: real 1m55.935s (115.9 s), beside 113.1, 121.1 and 123.8 s at the same pin (the
    rust-reviewer's baseline: .gate/u4r/brawl1/ and .gate/brawl1-merge/ logs). No third counted call.
- Then the gate --no-smoke on 561bf42: **GATE PASS** (gate-merge.log); the built check 25 / 25; the push at 16:49;
  **CI on ddd38ba: all six check runs success** (gate, smoke, toolchain; runs 43911726413, 43911714149, 43911727938,
  43911713654).
- **C1-C8:** C1 met (the first call stays FAIL by name; both calls named here). **C2 not triggered** (the test did not
  fail again). C3 and C4 met on the card (below). C5 met (the set check's sha256 above, committed at ddd38ba, its
  shown-to-fail runs first). **C6 met:** P2 had its two judge calls (measured, then bites), and no stop by hand was
  needed. C7 met: the pre-flight ran exactly as H2 replaced it (section 9). C8: the brief unchanged (7936074c…), and
  with it the brief's section 8 order, its expected lines and its section 10 stops.
- **U3C-HELD-1** (Medium; the card's Open findings :3760-3773): "G-NC2's held-bound check (delay.rs:528-543) counts a
  release transient as over the bound". Taker: the main session, as its own planned change (plan first with the
  advisor; netcode-architect and test-engineer), merged before another lane's counted check.mjs run, at the latest
  U4f's merge. Never a looser bound: the fix still checks the bound at every write and shows by a planted bug (for
  example, drop released_model from down_free) that it still catches a real overflow. Its first step, a by-name loop
  of the bites build's npt-net tests, also captures the failure's output again.
- **CHECK-NFF-1** (Low; card :3774-3775; taker: check.mjs's next owner, outside this lane): the bites build's `cargo
  test` gains --no-fail-fast (or a second pass), so one failure no longer hides the rest; here it hid 202 of 632.

### 7. The advisor's check of the pushed head ddd38ba: APPROVE WITH CONDITIONS H1-H6, all met

(16:54-17:06; advisor-c62h-bound-head-full.json, sha256 a64259f5…, read in full.) It checked the commit chain, the
merged bytes (patch-ids pair by pair), merge.log's FAIL by name, merge-2's pass bar item by item, the gate, the built
check (its own sha256s of the three files), the size 967 of 1,050, the builders' returns, the review round, the
follow-ups and C1-C8. Its conditions: H1 the P0 re-judge may run as written; H2 the pre-flight's commands replaced
word for word (ADV-CBH-1 Medium, its own wording, and ADV-CBH-2 Low: both fixed before the pre-flight ran, section 9);
H3 and H4 carried in section 6; H5 the merge-stop conditions stand; H6 the size line in section 1.

### 8. Mustafa's P0 re-judge: SAME ANSWERS (PASS)

His addition: a judge call, no new runs, its verdicts beside the earlier ones (history.md:2174).
- **The kept P0 set is 93509dfd7fd1** (B-3): it is the P0 set that counts (BRAWL-1's re-run set; "P0 STANDS AND
  COUNTS", advisor-brawl1-sets-p0-full.md); 051995db9cea is U4r's P0, whose -17 L1 P0 part is recorded and never
  counted (advisor-u4r-sets-r2-full.md:10) and whose -2 SS part failed at 0.861889 (brief section 7).
  051995db9cea is still on disk; its re-judge would be one more judge call, which Mustafa can name.
- **The calls:** the kept set's two judge calls, measured/ then bites/, as that set was judged (U4r-F2's memory
  split), **with no new runs**. One main-run.sh call (.gate/u4r/c62h-bound/p0-rejudge.log, 17:06:52-17:08:39, every
  command exit 0) at head a87c7ec39b966f83cc927b627d8f0e83104ac572, whose spike tree equals ddd38ba's. npt-judge
  d8b8c91eccc2fdf9aeb3d2f29e77103635e6ca3161b2752e14a3bb1ed7073f6e, marks.json c1935a5b… (the earlier answers' judge
  65acacfe…, marks 9fb5be57…). Peaks 3,920.3 MiB (measured/, 48.2 s) and 4,504.6 MiB (bites/, 52.8 s) against the
  10,240 MiB bound.
- **The verdicts beside the earlier ones:**
  - verdict.json: new **e5ed24e2469508506e0af4cdbc1bfba64a11648af40de7ab0a3daf7d804843f7** beside earlier
    **e926b812cbe57d81e251bfb5c6685b889a4d2859e9cac25022f6788bf74af095**;
  - verdict-bites.json: new **13b6b107263cc83c1bb21732aacc40977850803fa1b149eb0c1a41abfc81c593** beside earlier
    **b472784f2c7208aeef38af373a8bc257bf0b3b88a18dc5dc376fb6533536942e**;
  - each part and bite that holds a run, new | earlier: PM-0001-2 SS runs PASS | PASS; -17 L1 P0 PASS | PASS; -57 L1
    P0 PASS | PASS (each l1-p0-s1 to -s3 valid); bites tick16, client-rounding, whole-as-lost (-17), pos-4cm,
    yaw-12bit, pitch-8bit (-57): shown | shown.
  - **SAME ANSWERS** (rejudge-compare.log). The only differences are this lane's two expected additions: -17's new
    bite entry link-lost-overstated (not run, no runs) and its one why line.
- Committed at cb37f5668727d46e61907e3c10702b89d9ab5396 (results/c62h-bound/p0-rejudge-93509dfd7fd1/a87c7ec39b96/:
  both verdicts and rejudge.txt, which holds the commands, both sha256s, the peaks and both outputs whole; redact 300
  items, 0 findings).

### 9. The pre-flight (NA-CBB-2): PROCEED

Run once, after SAME ANSWERS and before the sets, exactly as the head check's H2 replaced the commands
(c62h-bound-build.md:460-472; the run :474-489). ADV-CBH-1 (Medium) and ADV-CBH-2 (Low) were fixed before it ran:
`--only PM-0001-2,PM-0001-17`, npt-lab rebuilt in the call, both hash checks with a stop, --commit the full HEAD after
the spike-tree check, --date the day it ran, --warmup-s 10.
- (0) 20:44:40: the spike tree equals ddd38ba's (exit 0); HEAD ea08f89298f32bb6561c81a5333346af416ec0a3.
- The call (.gate/u4r/c62h-bound/preflight.log, 20:44:46-20:45:43, every command exit 0): the rebuild compiled 12 npt
  crates (46.04 s); npt-lab 98c3967e04e4260985d24173b1fb2126ea1444636f4725e12cf76333d90b5113; npt-judge d8b8c91e…
  equal to p0-rejudge.log's; marks.json c1935a5b… as built.
- The short run (48 clients, P2, 10 s warm-up + 4 s): **S 5,760, L 111, bursts B 0; E1 0, E2 111, N 0** (= measured
  incomplete 111); overflow resyncs 0.
- The judge: -17's L1 P2 part **NOT-RUN ("1 of 3 valid runs"), as expected**; the run valid, gaps []; mismatches 0
  pass, **A 0 pass, B 0 pass**. The lab's N 0 equals the judge's A 0. Nothing committed; nothing under results/.
- NA-CBB-2 (no npt-lab log judged before the P2 set) is therefore done.

### 10. The judged sets P2 and P3: every expected line PASS

- **Disk (8.1), no pack needed:** free 6,057 MiB before P2 (at least 5,500) and 5,306 MiB before P3 (at least 5,100;
  pre-P3-c62h.log reads 5,306, where the notes and the ruling give 5,307).
- **Before each set (8.2), one main-run.sh call:** pre-P2-c62h.log (20:47:14; HEAD
  8ee453c228825ec85308d83589c86159736bb4be, spike tree 25c5b760…) and pre-P3-c62h.log (21:19:05; HEAD
  d66e23cd78f2c75673224109228a5014a0d20a52, spike tree 14d78602…): the spike tree equals ddd38ba's (exit 0);
  name-status only .checkpoints/ and new files under results/; the four runner sha256s as the brief's (section 14);
  no cargo, rustc, npt-lab or npt-judge. (Before P2 the `pgrep -f` line matched the check's own command line, which
  names the runner; checked apart, no runner ran.)
- **P2** (20:48:07-21:17:23; .gate/u4r-sets/P2-8ee453c22882.log; peak.py wall 1,755.5 s, exit 0, 1,799.0 MiB):
  results committed at **daec788fca8b2601c74511dbc84f0fc5b0a712d2** (results/u4r/8ee453c22882/P2/: runs.txt,
  verdict.json, verdict-bites.json; redact 303 items, 0 findings). Set check (set-check-P2.log): **46 PASS, 0 FAIL, 0
  on Mustafa's stop rules**; 34 lines name the new phrase or checks, 0 FAIL.
- **P3** (about 21:20-21:37; P3-d66e23cd78f2.log; wall 1,029.4 s, exit 0, 995.8 MiB): results committed at
  **8ca774a27292963ae83427214e2c6e92447d9f32** (results/u4r/d66e23cd78f2/P3/: runs.txt, verdict.json; redact 305
  items, 0 findings). Set check (set-check-P3.log): **32 PASS, 0 FAIL, 0 on Mustafa's stop rules**; 19 lines name the
  new, 0 FAIL.
- **Both sets, brief 8.4:**
  - runs.txt's marks.json sha256 is c1935a5b…, the built check's "after": **PASS** (both).
  - npt-judge d8b8c91e… recorded; it equals the re-judge's although the sets used the main folder's own build cache
    and the re-judge the shared target (it need not have): **PASS**. npt-lab 98c3967e… equals the pre-flight's too.
- **P2, verdict.json:** PM-0001-2 SS runs, PM-0001-17 L1 P2 and PM-0001-57 L1 P2 each PASS (l1-p2-s1, -s2, -s3
  valid): **PASS**. On each run A 0 pass, B 0 pass, mismatches 0 pass, gaps empty: **PASS**.
- **P2, verdict-bites.json:**
  - whole-as-lost shown (valid): A **48 fail**, B 0 pass, mismatches 0 pass, gaps []: **PASS**;
  - link-lost-overstated shown (valid): B **48 fail**, A 0 pass, mismatches 0 pass, gaps []: **PASS**;
  - no other bite holds a run (-57's bites are not run on P2): **PASS**.
  - Mustafa's planted-bug rule holds for both, as one conjunction: valid, its own figure exactly 48, every other
    check passing, no gap, and the lab's N equal to the judge's A (48 = 48; 0 = 0).
- **P2, the summary lines:** each clean line, without the new phrase and the wall time, equals
  results/u4r/fc411116c1f1/P2/runs.txt:24-26 whole: **PASS**. Fate logs 911d96e802b38294 / 6f4d3bf75a875a92 /
  819c74ce1c037ef1; measured incomplete 17,260 / 17,089 / 17,532; client hashes 861,996 / 862,011 / 861,731; decided
  lost at the stop 30 / 24 / 23; stale 0, sentinels 0, overflow resyncs 0.
  - The new phrase's bar (bursts B 0, E1 + E2 + N = measured incomplete, N 0) on each run: **PASS**. L is about 2 %
    of S (2.00 / 1.98 / 2.03 %), a reported value, not a bar.
  - bite-whole-as-lost against l1-p2-s1: differs in exactly two fields, client hashes 861,948 (s1's minus 48) and
    measured incomplete 17,308 (s1's plus 48); its S, L, bursts B, E1, E2 equal s1's; N 48: **PASS**.
  - bite-link-lost-overstated against l1-p2-s1: differs in no field; its phrase equals s1's, N 0: **PASS**.
- **P3, verdict.json:** PM-0001-2 SS runs, PM-0001-17 L1 P3 and PM-0001-57 L1 P3 each PASS (l1-p3-s1, -s2, -s3
  valid): **PASS**. On each run A 0 pass, B 0 pass, mismatches 0 pass, gaps empty: **PASS**. P3 has no bites.
- **P3, the summary lines:** each equals results/u4r/fd4612050cd6/P3/runs.txt:24-26 without the phrase and the wall
  time: **PASS**. Measured incomplete 43,295 / 43,210 / 43,454; overflow resyncs 27 / 0 / 1; decided lost at the stop
  87 / 62 / 59; fate logs b99096c62a5c51dd / 7dbd92ec2098f6da / f167ce6f8dfc3c09; client hashes 835,238 / 835,419 /
  835,088.
  - s1 and s3 carry the carry-58 flag: to the advisor before they count; **ruled: both count** (section 11).
  - s1's E1 is above 0 (201): **PASS**, so condition 2 (a null server hash explains a lost tick) is shown on a real
    run, not only by the fixtures.
  - E1 + E2 + N = measured incomplete and N 0 on each run: **PASS**.
- **B1:** nothing now. -17's L1 B1 part stays INVALID by pre:PM-0001-2 until U4g's judged B1 runs, which carry both
  checks.

**Per run, S / L / bursts B and E1 / E2 / N** (runs.txt; each E1 + E2 + N equals the measured incomplete count):
- P2 l1-p2-s1: S 864,000, L 17,260, bursts B 0; E1 0, E2 17,260, N 0.
- P2 l1-p2-s2: S 864,000, L 17,089, bursts B 0; E1 0, E2 17,089, N 0.
- P2 l1-p2-s3: S 864,000, L 17,532, bursts B 0; E1 0, E2 17,532, N 0.
- P2 bite-whole-as-lost: S 864,000, L 17,260, bursts B 0; E1 0, E2 17,260, N 48 (measured incomplete 17,308).
- P2 bite-link-lost-overstated: S 864,000, L 17,260, bursts B 0; E1 0, E2 17,260, N 0.
- P3 l1-p3-s1: S 863,772, L 43,094, bursts B 0; E1 201, E2 43,094, N 0 (measured incomplete 43,295).
- P3 l1-p3-s2: S 864,000, L 43,210, bursts B 0; E1 0, E2 43,210, N 0.
- P3 l1-p3-s3: S 863,994, L 43,449, bursts B 0; E1 5, E2 43,449, N 0 (measured incomplete 43,454).
- P2's bursts B 0 is asserted by the set check; **P3's bursts B 0 is the advisor's own read of runs.txt**, since the
  P3 path of the set check has no such line (brief 8.4 names none; ADV-CBP3-5).
- On every run the lab's N equals the judge's A: 0, except whole-as-lost, where both are 48.

### 11. Carry 58 on P3: the advisor's P3 ruling (P1)

(21:38-21:48; advisor-c62h-bound-p3-full.json, 71 lines, read in full.) **P3 COUNTS AND P2 COUNTS.** l1-p3-s1 and
l1-p3-s3 count under carry 58, as they did in fd4612050cd6 (the precedent advisor-brawl1-sets-p3-full.md:18-22: the
resync is the designed recovery path, triggered by the event queue's cap, never by a mismatch).
- **What the "room sent nothing" ticks are,** from its read of the code and of the raw logs
  (.gate/netcode-proof-test/runs/u4r-P3-d66e23cd78f2/):
  - s1: 27 overflow resyncs, all in the measured window, on 25 slots at ticks 76,910-77,844, no kick. Its 201 E1
    ticks are wholly the waits for an unacknowledged forced full state: each belongs to the latest resync of its slot
    and runs unbroken from the resync tick + 1, 4 to 18 ticks each (the acknowledgement limit is 60).
  - s3: 1 resync (slot 35, tick 76,803); its 5 E1 ticks are slot 35 at 76,804-76,808, right after it.
  - The 228 client-ticks without a snapshot datagram in s1 (864,000 - S) are 201 + 27: the 27 forced full-state ticks
    themselves go on the control channel with a server hash, are decoded and compared, and are not filed lost; the
    201 are the ticks while each full state waited for its acknowledgement (no server hash, filed lost, E1). s3: 6 =
    5 + 1. This refines the advisor's earlier sentence on those ticks (advisor-brawl1-sets-p3-full.md:20;
    ADV-CBP3-2); the counting conclusion is unchanged.
- So the resyncs hide no loss and leave none unexplained, and condition 2 is shown on a real run. s2 has E1 0 and no
  resync.
- Equal across both build caches: npt-judge d8b8c91e… and npt-lab 98c3967e….
- Its five Lows: ADV-CBP3-1 (the notes gained P3's lines: c62h-bound-build.md:515-533), -3 and -4 (done by the (c)
  script, section 13), -2 and -5 (carried in this section and section 10). Nothing at Medium or above.
- **Carry 58's capacity side stays open** (P7): EVQ-LOAD, U4f's measurement of the cap, and the per-burst
  high-water marks before -18's judged B1 runs (U4g).
- **Its conditions, each met or carried in this report:**
  - P1: P2 and P3 count, s1 and s3 under carry 58: this section; the card and c62h-bound-build.md's section 8 say so.
  - P2: the (c) replacement ran only as the_c_lines gives it: section 13.
  - P3: every item of brief section 9 and the carries of the ruling's order_to_the_report: sections 1-18.
  - P4: c62h-bound-build.md's section 8 gained P3's lines before this report was written (ADV-CBP3-1, :515-533).
  - P5: both P0 folders, the reference sets and the c-u2 folders stay unpacked until this report has reached
    Mustafa: section 14.
  - P6: the head that carries this report gets the advisor's check, naming its full SHA, before the report goes to
    Mustafa; nothing in spikes/netcode-proof outside results/ changes from ddd38ba to that head (`git diff --quiet
    ddd38ba HEAD -- spikes/netcode-proof ':(exclude)spikes/netcode-proof/results'`: exit 0 at e2d06ec, the head that
    holds the round-1 text, and at a5baab8 in the verifier's round 2; run again on the head that carries this report).
  - P7: every other bar and stop in the brief stands unchanged (C8, section 6); carry 58's capacity side stays open
    (above).

### 12. The stop rules: none was met

- **Mustafa's** (the paste's stop rules, c62h-bound-paste.md:120-124, adopted by him at history.md:2175; read as the
  advisor's condition 3 reads them):
  - a lost tick nothing explains on any clean run or test: not met. A 0 and N 0 on the six clean judged runs and the
    pre-flight; the short P2 lab test, which asserts the join's 0 unexplained, passed in every check.mjs run.
  - the lab's count of unexplained ticks differing from the judge's: not met. N equals A on every run, the bite runs
    included (48 = 48 and 0 = 0).
  - a planted-bug run reading other than 48 on its own check with everything else passing and no gap: not met. Both
    read exactly 48, all else passing, gaps [], valid.
- **His size line** (history.md:2172): not met; 967 is under 1,050.
- **His other two:** no frozen text and no figure's computation changed after his yes (the built check 25 / 25 on
  561bf42; none of the 12 review findings touches either); the P0 re-judge gave the earlier answers (SAME ANSWERS).
- **The advisor's:** one was met and handled as written: a check.mjs step failed that no edit of this lane explains
  (merge.log), so the lane stopped and reported whole, and the advisor ruled (section 6). None of the others: no
  built hash differed; no summary field or bite line differed beyond 8.4's allowance; no run met a join error (every
  run valid, both sets exit 0); no floor stopped a runner; det-u4's non-cargo time was 204.2 s; every test count was
  as expected; no shown-to-fail run passed; no judge answer moved beyond B-2's named one; and both worker restarts
  came at clean points (no plant in place, no half edit, no run in flight).

### 13. The (c) report lines, replaced only after both sets passed: PASS

Mustafa's words kept them in until the bound was built (history.md:2146). At 21:58, after both set checks passed and
the advisor's P3 ruling, c62h_bound_c_lines.py replaced the two sub-lines under -17 L1 P3 and -17 L1 P2 with the exact
texts the ruling gives (its the_c_lines), after checking every number in them against the files they name. It added
the one note line (ADV-CBP3-4), and left B1's lines unchanged (now :1035 and :1167). Committed at
7526ffe89f954cf781d480c43eaffe4452dae939 (u4r-report.md +3 -2; sha256 76680d3a… to 777ee782…).

### 14. Kept logs, the P0 folders, the deferred brief items, the runners' sha256s: PASS

- **The kept logs packed:** one folder, before the build (09:10-09:16): **u4r-P2-ab10f058a828** (U4r's P2, closed by
  advisor-u4r-sets-r2-full.md:45). Its list was committed alone and pushed at 6c8c7cf; the pack exited 0, the archive
  217,268,267 bytes from 816,778,012, read back 11 of 11 files equal; **free 5,883 MiB before, 6,455 MiB after**
  (.gate/c62h-bound-pre/kl-pack-p2.log). No pack during the sets (section 10).
- **Both P0 folders are still unpacked** (B-3, P5): u4r-P0-93509dfd7fd1 and u4r-P0-051995db9cea stay until this
  report has reached Mustafa, as do this lane's reference sets fc411116c1f1 and fd4612050cd6 and the c-u2 folders.
- **Deferred in writing (brief section 11):** (i) re-judging both P0 folders: not done, since the advisor confirmed
  B-3; the other folder's re-judge stays one more judge call he can name. (ii) The design's claim that old P2 and P3
  logs read INVALID under the new marks.json got no judge call on old logs: **it is shown by the gaps fixture**
  (c62hb_eight_gaps_read_on_both_figures, its first case: room logs without the series read INVALID on both figures
  with gap 1's phrase), not by old logs. B-2's before-edit readings (section 5) agree.
- **The runners' and peak.py's sha256s** (LANE_BASE's, and at both pre-set checks):
  - u4r-run.sh e9437ae200d234725b392bc5090e2b802d9938bb8ba5b12c23123c3d607e97d7 (unchanged);
  - main-run.sh 8df2dee311a6f00d147b876be787313e54900309625cfbabd12059aac86d0ae6 (unchanged);
  - peak.py dfacdcc0ad51fbde42668293509c541eed009a8d53243d9df3572a59318a31ac (unchanged);
  - c-u4r-runs.mjs 99dd070eb20ffced3e7dcd7a892b2456027817a6c81287f6857831c990ebd09b at LANE_BASE, and
    f2002890e297b2df02eea5d3524953f0170ff6de6d85fad6ad345773610644ee after commit 4 (the bytes both sets ran on).

### 15. The deferred Lows (STATUS follow-ups; none triggers a push)

Ten Lows, nine distinct issues (TE-CBH-5 and NA-CBB-1 are one), deferred in writing and accepted by the merge-stop
ruling. Taker for each (the card's Open findings, :3776-3779): the next npt-judge or npt-lab change, U4f, U4g or
JDG-ONLY-1. Paths are under spikes/netcode-proof/; lines are at 42b9646, whose bytes 561bf42 carries.
- **TE-CBH-3**, crates/judge/tests/engine.rs:1680-1682: c62hb_eight_gaps_read_on_both_figures asserts "no other
  gap's phrase" for its first case only; fix by counting the phrases in each note.
- **TE-CBH-4**, crates/judge/src/figures.rs:595-599 and :633-640: the reader's own length checks on hs.server_hash,
  hs.decoded_hash and tick.s.snap_incomplete have no test (only the link list's is planted, CB-P18).
- **TE-CBH-5 = NA-CBB-1**, ops/runs/c-u4r-runs.mjs:14-15 (NA-CBB-1 gives :13-14): the stop instruction still says
  only P0 has bites; P2 now has two judge calls. Covered meanwhile by C6's operating note.
- **TE-CBH-6**, crates/judge/tests/fixtures.rs:390-391 and crates/judge/fixtures/README.md:23: wording (a terse doc
  line; the code E2 is not defined in that README).
- **TE-CBH-7**, crates/judge/src/figures.rs:613-616 and :643-648: a negative tick.s.snap_incomplete entry gets gap
  phrase 7, not phrase 5; same outcome (a gap, INVALID on both checks), only the note's text differs.
- **NA-CBB-2**, crates/judge/src/figures.rs:579-667 and crates/lab/src/scenario.rs:2859-2897: no npt-lab log judged
  before the P2 set. **Done by the pre-flight** (section 9).
- **NA-CBB-3**, crates/judge/src/figures.rs:583 and :623 (:224-233): the one-line note: the frozen rows say "process
  client", the reader takes client or bots; on every run these checks read (only a client log), the value is the
  same.
- **RR-CB-1**, crates/judge/src/figures.rs:651 (doc :578): gap 7's note names the lowest slot's first unmatched tick,
  not the earliest tick; only the note's location differs.
- **RR-CB-2**, crates/lab/src/scenario.rs:800-805 and :198-200, crates/lab/src/logs.rs:181-187: two docs cover three
  fields from the first field only, and one bite doc sentence lacks a word.
- Fixed, not deferred: TE-CBH-1 (the planted-bug rule now reads every check of a bite run) and TE-CBH-2 (the bite-line
  comparisons shown to fail in the self-test), both in the main session's c62h_bound_set_check.py.

### 16. Still unbounded (the advisor's item 4)

- **-58's two wrap parts** (L1 P0 and L1 P3: PM-0001-58, every counter survives its wrap): nothing bounds the ticks
  a client may file as lost there. Offered under Decisions needed (C62HB-Q4 (b)).
- **-17's L2 parts** (the real-network layer): no network record exists there to explain a loss.
- **-17's L1 B1 part,** until U4g's judged B1 runs carry both checks; its line stays (u4r-report.md:1035 and :1167).
- **Anything outside each run's measured ten minutes** (the warm-up and the after phase): no lost tick is checked
  there, as the wording says. The ticks decided lost at the stop (P2 30 / 24 / 23, P3 87 / 62 / 59) all fall there.
- **A double fault:** a wrong network record at exactly the tick where a client bug files a snapshot as lost. The
  two-way checks, the planted bugs and the 17,000 to 43,000 real losses per run make it narrow, not zero.

### 17. What was not verified

- No test drives a full run into a join error; that a join error fails the run is shown by reading only
  (scenario.rs:1051-1055, the rust-reviewer's read; the test-engineer's point). The five join tests and three plants
  cover the join itself.
- The lab tally's "room sent nothing" branch has no lab test that makes it non-zero; P3's s1 (201) and s3 (5) show it
  on real runs.
- TE-CBH-4's three length checks (section 15) are untested.
- Speed: not measured, except the merge step's timed hash (section 6). The Wasm module's size is unchanged by
  construction (npt-hash-wasm depends on npt-sim alone, which the lane does not touch); its bytes were not measured.
- U3C-HELD-1's cause rests on the advisor's reading of the code, not on a reproduced run; its fix's first step
  confirms it.
- CI on ddd38ba is the main session's relay; the advisor did not re-read it.
- Nothing was re-run to write this report: every figure is copied from the log or file named beside it.

### 18. Process notes

- **12:44-14:22, a worker restart** (not the spend limit; card :3212-3217): the call to commit step 3, sent at
  12:44:13, ran only after the notice at 14:22:42 (commit 3 at 14:23). Nothing lost: launch 3 had ended at 12:43; the
  saver had ended on its time limit at 13:12:34 and was restarted at 14:24.
- **14:52-15:36, a wait with no notice:** the call that made commit 4, sent at 14:52:04, ran at 15:36:16; the machine
  had booted at about 14:22 (uptime 1:15 at 15:37; card :55-56, c62h-bound-build.md:416).
- **17:09-20:36, an unanswered approval, then a worker restart** (card :3198-3211): the call holding the rejudge.txt
  quote check and the redact step, sent at 17:09:35, waited for an approval nobody answered; the saver ended on its
  time limit at 18:39:55; the notice at 20:36:11 closed the call unrun, and the machine had rebooted. Resumed at
  20:37: the saver restarted, the step re-run in two simple calls. Nothing lost: the re-judge had ended at 17:08:39,
  rejudge.txt was written at 17:09:32. Since his 21:50 words (history.md:2187), commands use the simple forms that
  need no approval, one per call, and a call that must ask him is named first.
- **Slips 15-17:** (15) a `python3 -I -m py_compile` of the set-check script left a .pyc file; removed at once
  (c62h-bound-build.md:422-423). (16) the re-judge's redact step ran as `cd spikes/netcode-proof && node check.mjs
  --only redact` without a subshell; the shell was returned to the repository root at once (card :3209-3211). (17)
  the record script's heading for his 22:09 words first said "about 21:52 Amsterdam", written before the
  transcript's time was read; corrected by Edit (card :3601-3602).
- **Launch 4's calls outside the runner:** two read-only `node -e` calls (marks.json's bite ids; the runner's line
  lengths) and two read-only `python3 -I -c` text-fill computations ran directly, not through u4r-run.sh; they wrote
  nothing and are not evidence. Every recorded run went through the runner (c62h-bound-step4-full.json, deviations).
- **Recorded times corrected here:** the plant times in section 3 are the logs' own; the notes' "11:0x" and "11:1x"
  for CB-P5 to CB-P7 and step 1's staging were wrong (the test-engineer's point).

### Decisions needed

- **C62HB-Q4 (b): should -58's two counter-wrap parts get this lane's bound, now that both sets pass?** (The code:
  the design's question C62HB-Q4 kept these parts out of this lane and, as its option (b), offered you this decision
  once both sets passed; c62h-bound-design-full.json, brief section 9.)
  - The question: PM-0001-58 ("every counter survives its wrap") has two L1 parts, P0 and P3, read on the "wrap"
    suite's runs (at least three valid runs each, marks.json). Neither has any check on how many ticks a client files
    as lost. Should they get the same two checks as -17's P2 and P3 parts?
  - Why it matters: until they do, a client bug that files snapshots as lost around the wrap goes unseen there; it is
    the same hiding place this lane closed on -17 (c62h-bound-paste.md:102).
  - Options:
    - (a) Now, as a small lane of its own: the exact wording and size to you for a yes first; the two checks on
      -58's two parts in marks.json, with a reviewer's read under your 13:35 rule; then -58's wrap runs made again
      with the new lab and judged, since runs logged before this lane lack the network's record and read INVALID on
      the two checks (section 14 (ii)). The design names the L1 P3 wrap runs (C62HB-Q4); by the same reason (this
      report's reading), the L1 P0 wrap runs would need it too.
    - (b) With U4g, which already takes -58 and is likely to change marks.json (advisor-c62h-review-full.md:90), and
      whose judged B1 runs bring -17's B1 part under the same two checks: the same wording yes from you first.
    - (c) Not now: the two parts stay on the "Still unbounded" list.
  - Recommendation: (b), with U4g. The two figures and their reader already exist, and U4g opens -58 and these
    checks anyway, so one wording yes and one reviewer's read cover both, without a separate lane.

### For Mustafa

1. Answer C62HB-Q4 (b) above: (a), (b) or (c).
2. If you also want U4r's own P0 set (051995db9cea) re-judged with the new judge, say so: one more judge call, no new
   runs. Both P0 folders stay unpacked until you have read this report.
3. Nothing to check in Brave: this is spike code on PR #40's test branch, nothing a player sees, hears or feels
   changes, and nothing merges to main.
4. Separately, not this lane: PR #54 (the three NIGHT_SHIFT.md lines you said yes to at 22:09 Amsterdam time) waits
   for your go on its merge: its review round is done, and it has been ready since 22:11 UTC (23:11 Amsterdam) on
   2026-11-16.
