# Draft: BRAWL-1's section of u4r-report.md, with two (and one optional) replacements elsewhere in the report

Drafted 2026-11-15 by the drafting agent of workflow wf_634ed418-227, from the rulings, the builder reports, the card,
the logs and the results folders. Every figure was checked against its file; where the card and a file differ, the
file is used (brawl1-report-draft-notes.md, "Card against files"). Lines between `[[` and `]]` are placement notes for
the main session, not report text. Quoted blocks keep their source's line breaks.

---

[[KEEP u4r-report.md:614 unchanged: "## BRAWL-1 (the bots' leash toward the centre, from the client's own decoded position)"]]
[[INSERT the text below, down to the next KEEP, after :614 and before :616]]

_The lane BRAWL-1 (Medium), homed by advisor-u4r-sets-r2-full.md, finding BRAWL-1, with the items the r2 ruling put in
the next npt-lab lane (brawl1-plan.md:4-6). Merged into the spike branch on 2026-11-15 at b30314a. Its re-run sets
count (advisor-brawl1-sets-p0-full.md, Summary)._

**In short** (each figure's source is given in the subsections below):
- The test bots now steer back toward the middle by the position their own client decoded, never by the server's
  state. That is the fix for BRAWL-1, the drift that carried a bot into a building and made -17's U4r runs invalid.
- Two builder copies built it. Two review rounds found no Critical or High. The advisor's review of the pushed head
  approved it with three record-keeping Lows (advisor-brawl1-m6-full.md, Verdict). Its size is 1,266 changed lines,
  against about 700 planned.
- Both 10-minute evidence runs passed every bar. The three re-run sets P3, P2 and P0 all count:
  - -2's stress-scene part passes at every profile, with everyone in view 1 on every run. That is the fix's proof.
  - -17's L1 P3, P2 and P0 parts pass with 0 mismatches.
  - All six bites are shown.
  - -57 passes everywhere.
- Two lines go forward: EVQ-LOAD, the event queue's cap under full view, for U4f and U4g; and a corrected
  TICK16-WORDING.
- Still open: the advisor reads this section against the files before BRAWL-1 is recorded closed. That read is not a
  review round (advisor-brawl1-sets-p0-full.md, The commit and the close, 3c). PR #40 stays a draft, and nothing here
  merges to main (the same section, "Unchanged").

### The plan and the build

- **The plan:** brawl1-plan.md, written by the main session on 2026-11-14 at about 23:45 UTC.
  - The advisor's plan review: APPROVE WITH CONDITIONS 1-18 (advisor-brawl1-plan-full.md, workflow wf_533068a9-b2b,
    sha256 a7e68b90…).
  - Its four Mediums:
    - BP-1: run_leg's half of the refactors needs its own proof (condition 6);
    - BP-2: the leash's bounds and the stuck print (conditions 1-2);
    - BP-3: the sim-only test feeds an 8-step-old position and prints a B1 pattern (condition 3);
    - BP-4: the merge step in the M-list form (condition 9).
  - Its Lows: BP-5 to BP-10 and ADV-BP-1 (:65-105).
  - The choices Q1 to Q8 are all (a). The plan's R_IN of 1,200 cm is refused: R_IN is at least 1,300 cm, and R_OUT
    lies from R_IN + 100 up to 1,500 cm (:9, :30).
- **Planned size:** about 700 changed lines, 450-750 with the conditions. "Well past" is more than about 1,400
  (advisor-brawl1-plan-full.md:109; brawl1-plan.md section 12).
- **The builder's brief:** brawl1-brief.md.
  - The advisor's conformance check: CONFORMS AFTER EDITS, edits E1-E23 (advisor-brawl1-brief-full.md, workflow
    wf_12d3f66b-ca1, sha256 e0cb1a99…).
  - The edits were applied word for word. The brief is sha256 921c3243…, committed at 926a6cb (the card, "Earlier NOW
    (2026-11-14 about 23:50 UTC)").
- **How it was built** (the card, same block):
  - Copy 1's first launch (workflow wf_1d4a86b9-79f) was cut by the spend limit at 00:00:39 UTC and wrote nothing.
  - Its relaunch got a worktree on main (4460ef5) and stopped at its start check.
  - The main session then made the worktree by hand at the spike head 352a245: .claude/worktrees/brawl1, spike tree
    abad0635.
  - Copy 1 (agent a4be43b1f756e45dd) built there from about 02:45 to about 04:10 UTC.
  - The lesson, on the card: neither the workflow's nor the Agent tool's worktree isolation bases on the spike branch
    now.
- **Part 1, the refactors and N2-1** (brawl1-build-report.md:10-66):
  - EA-U4R-2: `pub(crate) type Sim = World` and one `Server::build`, used by run, run_leg and lab_room.
  - EA-U4R-1, in the narrow form: `struct Wake` and `fn wake` cover steps 1-2 plus the due count, and run_leg runs on
    a SimClock. There is no one-loop driver.
  - N2-1 with TE-R2-1: closed_checks gains its fourth check. Under the leave plant two of the four checks fail by name;
    under the roster plant the new check fails alone (p1-plant-leave.log, p1-plant-roster.log).
  - TE-R2-2 with RR4-L2: parse() bounds the run's last tick and the bursts, and fp3 pins each message.
  - RR4-L1: the 36,000-tick manifest cap, built as `MAX_ENTRY_TICK_COUNT` (hash.rs:239; the points ruling, Point 1).
  - RR3-L2 (a): on a no-log run, filed() only resolves.
- **Part 1's proofs at the old pins** (condition 6; brawl1-build-report.md:61-66):
  - (a) check.mjs: all 11 steps pass, 532 and 610 tests (p1-a-check2.log). The first run failed on Part 1's own code
    and was run again on the fixed code. The points ruling accepts this as not a retry.
  - (b) det-u4.mjs: PASS in 232.9 s, non-cargo sum 222.0 s (p1-b-det.log).
  - (c) The precheck regeneration: 6933b56a, "as the manifest's" (p1-cd.log).
  - (d) The clean pre-check pair: 109 lines against faf42aeb's runs.txt:17-125. Only the build line's commit and binary
    sha256 differ (p1-cd.log). The advisor re-checked it (advisor-brawl1-points-full.md:36).
  - (e) The precheck tests' prints before and after Part 1 are equal, apart from the wallclock bite's L1 hash and
    libtest's filtered-out count. Accepted as ADV-BP-3 (the corrections, below).
- **Part 2, the fix** (brawl1-build-report.md:68-95):
  - **The read item (npt-client, condition 5):** `pub fn own_soldier(&self) -> Option<([i32; 3], bool)>`. It returns
    the own soldier's decoded grid position and dead bit, from the newest complete tick filed Hash, with ticks compared
    by the endpoint's wrap rule. It is the lane's only new pub item.
  - **The decision (npt-bots, condition 4):** a crate-private `next_command(newest, view, own)`, which step() calls
    with `endpoint.own_soldier()`.
    - The leash applies to Kind::Brawl only.
    - `toward_centre` works in integers, and every vector has x² + y² at most 127² (condition 1).
  - **The leash:** R_IN 1,300 cm and R_OUT 1,400 cm, with no sideways part.
    - That leaves 4 m to the 1,800 cm door entry.
    - These were the first values tried; nothing was tuned.
  - **The figure (condition 10):**
    - room.jsonl's `in_building_slot_area`, on every logged tick where it is not empty, in any phase;
    - Counts' measured total, printed in the summary line as "in a building: N slot-ticks (slots …)";
    - one marks/SERIES.md row, "none (recorded)";
    - no judge change and no marks.json change.
  - **The tests:**
    - the steering unit test and the npt-client read test;
    - the sim-only test's short copy, seed 1 over 120,000 ms, in every check.mjs: 10.61 s in the measured build and
      10.71 s in the bites build;
    - the full test, `#[ignore]`, seeds 1 to 3 at 610 s each, run by name in release: 23.52 s
      (brawl1-build-report.md:99-109).
- **The full test's figures.** They were unchanged after the fix pass and at M3 (m3.log, the full test's nine lines;
  brawl1-build-report.md:111-115):

  | Seed | (i) every tick: largest distance | (ii) skips: largest distance | (ii) skips per 1000, longest streak | (iii) B1's bursts: largest distance, at |
  |---|---|---|---|---|
  | 1 | 1,575 cm | 1,654 cm | 304, 6 | 1,832 cm, (1734, -592) |
  | 2 | 1,575 cm | 1,720 cm | 303, 6 | 1,832 cm, (1671, -751) |
  | 3 | 1,570 cm | 1,686 cm | 305, 6 | 1,832 cm, (1586, -917) |

  In all nine runs no soldier was in a building, and none was named stuck.
- **Pattern (iii), B1's bursts:** printed only, never barred (condition 3 (iii)).
  - The builder's reading, "a fixed radius stops it" with the 14 m vehicle as one candidate, is withdrawn (FP-9 item 2:
    stand-ins are off in the test, and no solid lies within 1.6 m of the three maxima).
  - The advisor's reading, labelled as computed: 1,832 cm is the leash's own reach under a 30-tick re-run. It passes the
    1,800 cm door entry, so "area 0 in three seeds" does not bound B1 (ADV-PT-M1; its home is under the corrections).
- **The pins.** Condition 7 allows exactly three entries. Their bytes are unchanged; only the sha256 values move
  (brawl1-build-report.md:125-131):

  | Entry | Manifest | Old sha256 | New sha256 |
  |---|---|---|---|
  | SS-P0-s4 | precheck-recordings.json | 6933b56a… | 1f00d2d5… |
  | SS-P2-s1 | det-recordings.json | 5b79abb2… | 1ec25de7… |
  | ES-P3-s2 | det-recordings.json | fad8f67d… | 6abf7643… |

  - Reproduced at copy 1's end: det-u4 PASS in 227.2 s, non-cargo sum 214.5 s (final-det.log), and the regeneration
    "as the manifest's" (brawl1-build-report.md:134-136).
  - Reproduced again at copy 2's end (FP-8) and at M3 (below). The advisor hashed them itself at M6
    (advisor-brawl1-m6-full.md, check 7).
  - The manifests are det a2faf3ea… and precheck 5ccf57f0… (brawl1-copy2-report.md:62).
  - check.mjs at copy 1's end: all 11 steps pass, 535 and 613 tests (final-check.log; brawl1-build-report.md:140).
- **The size** (SIZE-CHECK, by numstat against 352a245, SERIES.md included):
  - 998 changed lines at round 1's snapshot ee2973aa: 875 inserted and 123 deleted, in 10 files
    (advisor-brawl1-points-full.md:5). Copy 1's own count of 992 left out SERIES.md's 6 lines
    (brawl1-build-report.md:144).
  - **1,266** after the fix pass: 1,139 inserted and 127 deleted, at snapshot c2561b6c (brawl1-copy2-report.md:63;
    M6's check 3).
  - That is about 1.8 times the ~700 planned (450-750 with the conditions), and under the 1,400 "well past" line. So no
    SIZE-CHECK note was due. The round-1 ruling expected about 1,270 (advisor-brawl1-r1-full.md, The size).
  - npt-bots' lib.rs took 426 of round 1's 998 lines, about 330 of them in its test module (the card, "(3)
    SIZE-CHECK").

### Round 1 and its ruling

- **Where round 1's record sits** (ADV-M6-L3):
  - copy 1's report: brawl1-build.md:251 ("## Report"), word for word in brawl1-build-report.md (sha256 f8df429d…);
  - copy 2's fix-pass report: brawl1-build.md:620 ("## Fix pass, round 1: report"), word for word in
    brawl1-copy2-report.md (sha256 b089826a…);
  - the four seats: brawl1-review-r1-full.md (sha256 fc979c63…);
  - the advisor's points ruling: advisor-brawl1-points-full.md (sha256 0c1f6f5d…);
  - the round-1 ruling: advisor-brawl1-r1-full.md (sha256 86bec63b…).
- **The points ruling** (workflow wf_bbfeece0-4c7, on snapshot ee2973aa): no diff change (advisor-brawl1-points-full.md,
  Summary and Rulings).
  - Point 1, TICKS-NAME: `MAX_ENTRY_TICK_COUNT = 36_000` stands, because RR4-L1's ruled name fails check.mjs's
    TICK_NAMED rule (ADV-BP-4).
  - Point 2, PROOF-E: proof (e) is accepted with its two differences (ADV-BP-3).
  - Point 3: r10_carry_82's boundary move is accepted.
  - The zero-test reruns and the size (998) are accepted.
  - Its findings: ADV-PT-M1 (Medium: pattern (iii)'s reading, for U4g), and ADV-PT-L1, -L2 and -L3 (Low).
- **Review round 1** (the same workflow; snapshot ee2973aa; packet brawl1-packet.md, sha256 16eab462…):
  - The seats: test-engineer, netcode-architect, rust-reviewer and engine-architect, opus at effort xhigh, in two
    batches (condition 11).
  - All four APPROVE WITH FINDINGS: 3 Medium and 20 Low, no Critical or High (brawl1-review-r1-full.md:5, :83, :110,
    :185).
  - The Mediums:
    - TE-B1-1: nothing showed that the sim-only test can fail, and nothing checked that 48 Soldier records are seen;
    - TE-B1-2 and RR-B1-1, one gap: nothing showed that the lab's building figure can read above 0.
- **The round-1 ruling** (workflow wf_59e64b31-fa2; advisor-brawl1-r1-full.md, Summary):
  - The three Mediums are confirmed and fixed in one fix pass.
  - 16 of the 20 Lows ride it; they are tests and doc words only. RR-B1-7 and EA-B1-1 ride in part.
  - Recorded: TE-B1-5, TE-B1-8, ADV-BP-3, ADV-BP-4 and ADV-PT-M1. No action: RR-B1-6 and EA-B1-5. A follow-up:
    ADV-PT-L3.
  - A correction to a reviewer: TE-B1-5's "obstacle at 18.3 m / the 14 m vehicle" is wrong, because stand-ins are off
    in the test (:342-345).
  - The pins must not move, because on the recording path the fix pass changes only doc comments and Server::new's
    visibility (The pins, :194-211).
- **FP-0, the main session's runner line (TE-B1-7)** (the card, "FP-0 DONE"; advisor-brawl1-r1-full.md:29-37):
  - Both runners' headers gained the `diff …; untracked …` line: u4r-run.sh is now sha256 2f754d12…, and main-run.sh
    f0b7a3d0….
  - Committed at 7402dac, with u4r-run.sh.te-b1-7.diff and main-run.sh.te-b1-7.diff, and `bash -n` silent on both.
- **The fix pass, copy 2** (agent a2e54bc7201dcffed, in the same worktree). It was cut by the spend limit at 06:53:13
  UTC and resumed at 07:13, with no plant in place (brawl1-copy2-report.md:10).
  - **Three new tests, +3 per build (538 and 616)** (:34-37):
    - bots: `tests::brawl1_a_soldier_teleported_into_building_0_reads_area_1`;
    - logs.rs: `logs::tests::brawl1_the_building_figure_counts_a_soldier_in_a_building_in_measured_ticks_only`;
    - endpoint.rs: `brawl1_own_soldier_compares_ticks_by_the_endpoint_s_wrap_rule`.
  - **The plants.** Each file came back to its exact sha256 (:41-49):
    - P1, the leash off, bit on area alone, in both builds and in the full test (Skips for seeds 1-3). The short copy's
      Skips run had 7,728 soldier-ticks in buildings, at most 3,813 cm out. So the door-entry bar was not added.
    - P2 cut the figure and failed at logs.rs:924.
    - P3 and P4 failed at their new client asserts.
  - **FP-8.** Each log's header shows diff c2561b6c (:52-63):
    - check.mjs: all 11 steps pass, 538 and 616 tests (final-fp-check.log);
    - det-u4.mjs: PASS in 227.3 s, non-cargo sum 213.3 s, reproducing 1ec25de7 and 6abf7643 (final-fp-det.log);
    - the regeneration reproduces 1f00d2d5 (final-fp-regen-tests.log);
    - the full test's 13 printed lines are identical to round 1's;
    - both manifests are unchanged.
  - Its deviations, LOGS0-RECORD and three more, were all accepted at round 2 (below).
- **TE-B1-8, in the ruling's words** (FP-9 item 3, advisor-brawl1-r1-full.md:188): "No test drives Behaviour::step
  with an own soldier past the outer radius. The glue at bots lib.rs:223 is shown only by the moved pins (own_soldier
  returns Some and changes commands) and by the two 10-minute evidence runs (condition 14)." After the fix pass, the
  glue line is lib.rs:228 (brawl1-copy2-report.md:68).
- **RR4-L1's constant** (FP-9 item 5, advisor-brawl1-r1-full.md:190): "RR4-L1's constant: MAX_ENTRY_TICK_COUNT stands
  (points ruling, Point 1). RR-B1-6 and EA-B1-5 are answered there, and ADV-PT-L3 carries the point to the check.mjs
  follow-up."
- **RR2-B1-2, round 2's correction:** FP-6 (a) made Server::new private (scenario.rs:396). Its only caller is :389, not
  :390 (advisor-brawl1-r2-full.md:55-58).

[[KEEP u4r-report.md:616-655 unchanged: "### Review round 2 and the advisor's round-2 ruling" and its quoted words]]
[[INSERT the text below, down to the next KEEP, after :655 and before :657]]

### The merge step (M1-M7) and the advisor's head review (M6)

_The steps are the round-1 ruling's M1-M7 (advisor-brawl1-r1-full.md, The merge step), with the round-2 ruling's A1-A6
(advisor-brawl1-r2-full.md, The merge step) and M6's M7-1 to M7-4 (advisor-brawl1-m6-full.md, M7). The logs are under
.gate/brawl1-merge/._

- **M1, the bytes** (the card, "M1 DONE"; b30314a's commit time 08:09:41 UTC by `git log`; checked at M6, checks 2
  and 3):
  - The base held: spikes/ at b30314a^ equals 352a245's (tree 63de1632).
  - `git apply --check`, then `git apply`, of wip/brawl1.patch (sha256 c2561b6c…, committed at 888ecbb).
  - All 10 files are cmp-equal to the builder's worktree, and the fingerprint `git diff HEAD --binary -- spikes/ |
    sha256sum` is c2561b6c.
  - The merge commit **b30314a** holds 11 files: the 10, plus `git rm` of wip/brawl1.patch. Its message has no
    [skip ci]. Nothing from round 2 rode (A1).
- **M2, the part-cleans, from 08:37:41 to before 08:38:44** (ADV-M6-L1):
  - No cargo, rustc, npt-lab, npt-judge or loader process was running, and no agent. This is on the card only.
  - (B), the release, doc and wasm32 roots: m2-clean-B.log, 08:37:41-08:37:47, 15 commands, each exit 0.
  - (C), the dev profile: m2-clean-C.log, 08:38:04-08:38:09, 6 commands, each exit 0.
  - Both logs' "run:" lines equal 2026-11-14's clean-shared-B.log and clean-shared-C.log (M6, check 4).
  - Free space, from the card only: 6,911,565,824 B before (B), 7,367,524,352 B after (B) and 13,579,448,320 B after
    (C), 12,950 MiB. These match the runner headers: 6,591 and 7,026 MiB (m2-clean-B.log:2, m2-clean-C.log:2), and
    M3's 12,950.
  - (B) freed 434.8 MiB, and (C) freed 5,924.2 MiB (M6, check 4).
  - The headers read `diff e3b0…; untracked e3b0…`.
- **M3, the counted run** (m3.log; 08:38:44-08:59:43 UTC; built cold, so its timings are cold):
  - The header: HEAD b30314a, free 12,950 MiB, `diff e3b0c442…; untracked e3b0c442…` (A2; m3.log:1-3).
  - `cat /proc/uptime`: 227.41 s (:5). uptime: "08:53:06 up 18 min, load average: 2.31, 2.21, 1.54" (:26).
  - check.mjs: "check: all 11 steps pass" (:8-19):
    - test 691.2 s, cold: measured build 538 passed, bites build 616 passed, 0 failed, 0 skips;
    - self 77 of 77; marks 81; redact 287 items, 0 findings.
  - df: 8,342,093,824 B after check.mjs and 8,342,081,536 B after det-u4, both above 4 GiB (A4; :23, :35).
  - det-u4: "PASS det-u4 234.1 s" (:30).
    - Its non-cargo sum is 219.4 + 2.5 + 0.9 + 0.0 = 222.8 s, under 240.
    - It reproduces 1ec25de7 and 6abf7643: det-recordings.json is unchanged, and the advisor hashed both files at M6.
  - The regeneration: "regenerated SS-P0-s4: 16950444 bytes, sha256 1f00d2d5…, as the manifest's" (:54).
  - The full sim-only test, in release in 23.90 s (:86). Its nine printed lines are byte-identical to
    final-fp-regen-tests.log's, dead figures included (A3; M6, check 8).
  - All 9 commands exit 0 (:103). Both manifests are a2faf3ea and 5ccf57f0 (M6, check 7).
  - **After M3** (ADV-M6-L2), labelled "after M3 and M4, not at M3's end":
    - the advisor's reading, 8,320,163,840 B at 09:06:48 (advisor-brawl1-m6-full.md:51);
    - M7's first df, 8,318,758,912 B (m7-df-0.log).
- **M4, the gate:**
  - The full gate's 5a smoke could not start Chromium: "Executable doesn't exist at
    /opt/pw-browsers/chromium_headless_shell-1194/chrome-linux/headless_shell". So the full gate reads GATE FAIL, on 5a
    alone (m4-gate.log).
  - `node tools/gate.mjs --no-smoke`: "GATE PASS (5a smoke skipped: --no-smoke)" (m4-gate-no-smoke.log). CI's smoke job
    covers 5a.
- **M5, push and CI:**
  - b30314a was pushed alone, at 09:02:06 UTC (f7dff4b..b30314a; the card, "M5 DONE").
  - CI is green on it: gate run 43697261383 (jobs gate and smoke) and toolchain run 43697261438. It is green on 8f0939f
    too: gate 43697365073 and toolchain 43697365142 (M6, check 10).
  - Every commit after b30314a touches only .checkpoints/, until the re-run sets' results commits (A5).
- **M6, the advisor's head review: APPROVE WITH FINDINGS** on head 8f0939f (advisor-brawl1-m6-full.md, workflow
  wf_8067a67a-6fe, sha256 c2fc64c1…).
  - 14 checks, all PASS, including the advisor's own gate run (.gate/brawl1-merge/advisor-m6/gate.log).
  - Three Lows, record-keeping only: ADV-M6-L1 to -L3 (the corrections, below).
  - M7: GO, with M7-1 to M7-4.
- **M7, the cleans again** (M7-1, M7-2). Every log's header shows HEAD fc93733, spike tree ebbfeb40, and
  `diff e3b0…; untracked e3b0…`:
  - m7-df-0.log, 09:12:50: 8,318,758,912 B free;
  - m7-clean-B.log, 09:12:52-09:12:53: 15 commands, exit 0;
  - m7-df-1.log, 09:12:54: 8,722,419,712 B;
  - m7-clean-C.log, 09:13:04-09:13:09: 6 commands, exit 0;
  - m7-df-2.log, 09:13:10: 13,556,895,744 B (12,928.9 MiB).
  - The clean logs' "run:" lines equal M2's (diffed).
  - M7-4, for disk planning: the main folder's cold full set refilled about 5.0 GiB, not the 8.2-9 GiB estimated
    earlier. The floors stay as they are; P0 needs 7,711 MiB (advisor-brawl1-m6-full.md:67).
  - Then come the two evidence runs and the re-run sets, below.

### The two evidence runs (condition 14)

- **The call:** one main-run.sh call, 09:15:05-09:26:03 UTC (.gate/brawl1-merge/m7-ev.log).
  - Its header: HEAD d89cc9d, spike tree ebbfeb40, `diff e3b0…; untracked e3b0…`. That is M7-3's original form, which
    the evidence logs keep (advisor-brawl1-sets-p3-full.md, The M7-3 conflict, (d)).
  - All 10 commands exit 0.
- **Each run** (m7-ev.log:112-135):
  - `npt-lab scenario --run brawl1-ev-<p0|p3>-s1 --set brawl1-evidence … --seed 1 --scene SS --profile <P0|P3>
    --tick-start 63736 --warmup-s 10 --measured-s 600 --suite brawl1-evidence --role measured --marks
    PM-0001-2,PM-0001-17,PM-0001-57`;
  - then `du -sb`;
  - then npt-judge under peak.py, into .gate/brawl1-evidence/.
  - No run or set is named u4r-* (condition 14).
- **The bars (condition 14): all met.**

  | Bar | P0 (brawl1-ev-p0-s1) | P3 (brawl1-ev-p3-s1) |
  |---|---|---|
  | everyone in view, at least 0.99 | 1 (verdict-P0.json) | 1 (verdict-P3.json) |
  | building figure 0 | "in a building: 0 slot-ticks (slots none)" (m7-ev.log:113) | the same (:125) |
  | bytes within U4r-F3's rate | 538,353,013 B against 603.7 MB (:116) | 98,642,683 B against 183.3 MB (:128) |

- **Recorded as facts, not a verdict** (condition 14):
  - **P0:**
    - every stress-scene check passes, with 693.09 shots a second;
    - -17: 0 mismatches of 878,352 compared, and 0 incomplete ("L1 P0 loses no group" passes);
    - overflow resyncs 0 (m7-ev.log:113);
    - -57: 6,984 / 2,750 / 88,063, position p99 5,737 (verdict-P0.json).
  - **P3:**
    - every stress-scene check passes, with 693.18 shots a second;
    - -17: 0 mismatches of 835,238 compared (verdict-P3.json);
    - -57: 6,952 / 2,750 / 88,063, p99 5,111.
  - **Added by ADV-P3-1** (the card's first record left these out):
    - The P3 run had 27 overflow resyncs on 25 clients, at ticks 76,910-77,844, all in the measured window. The count
      is in m7-ev.log:125; the clients and ticks are in its lab.jsonl.
    - Its incomplete count is 43,295 in the measured window (npt-lab, m7-ev.log:125) and 44,395 over the whole run (the
      judge, verdict-P3.json).
    - In the P3 ruling's words (advisor-brawl1-sets-p3-full.md, The report's figures): "**The evidence run
      brawl1-ev-p3-s1** is identical to l1-p3-s1: 27 resyncs at the same ticks and slots, 835,238 compared, 44,395
      incomplete."
  - -17's parts read NOT-RUN in both verdicts, because one run is not three seeds. That is expected for evidence runs.
  - The judge: P0 8.6 s and 1,856.3 MiB; P3 2.3 s and 392.9 MiB (m7-ev.log:122, :134).
  - Free space: 13,556,174,848 B before and 12,643,663,872 B after (:6, :138).

### The re-run sets (P3, P2, P0) at the fix's commit

_Copied word for word from advisor-brawl1-sets-p0-full.md, "The report words", lines 212-253 (sha256 4b16dab5…). One
change is made, as the ruling's own "NOTE TO THE MAIN SESSION" (:266) orders: the P3 carry-58 line reads "U4r 0 / 0 /
0"._

All three sets ran with c-u4r-runs.mjs 99dd070e, the runners unchanged (A6), each set's results committed before the next set started, and the amended M7-3 met at each start (pre-P2.log, pre-P0.log). Figures are for runs s1 / s2 / s3; "U4r" means U4r's same-profile set.

**P3:** commit fd4612050cd6, spike tree ebbfeb40. Log .gate/u4r-sets/P3-fd4612050cd6.log (sha256 cc2f626c…), exit 0, wall 1,020.5 s, peak 973.2 MiB. Results results/u4r/fd4612050cd6/P3/ (runs.txt 5361fd53…, verdict.json 21c9abbf…), committed at fc41111.
- -2 SS runs: FAIL to PASS. Everyone in view went from 0.352778 / 0.085444 / 0.203556 to 1 / 1 / 1.
- -17 L1 P3: INVALID to PASS, with 0 mismatches of 835,238 / 835,419 / 835,088 compared. The judge counts 44,395 / 44,253 / 44,535 incomplete over the whole run; npt-lab counts 43,295 / 43,210 / 43,454 in the measured window.
- -57 L1 P3: PASS, as in U4r's. Position max 6,952 / 6,963 / 6,970 against 6,984; p99 5,111; yaw 2,750 and pitch 88,063, unchanged. Largest change: s1's position max, -0.46 %.
- Carry 58: overflow resyncs 27 / 0 / 1 (U4r 0 / 0 / 0).
  - s1: 25 clients at ticks 76,910-77,844. s3: slot 35 at 76,803.
  - All are in the measured window. No client was kicked, and there are 0 mismatches.
  - The advisor ruled that s1 and s3 count (EVQ-LOAD).
- Building figure 0 on every run. Bytes 290.7 MiB against 524.4 projected (0.554); U4r's P3 had 518.8.

**P2:** commit fc411116c1f1, spike tree 079ffffc. Log .gate/u4r-sets/P2-fc411116c1f1.log (sha256 a24d5856…), exit 0, wall 1,046.7 s, peak 1,760.3 MiB. Results results/u4r/fc411116c1f1/P2/ (runs.txt c2995379…, verdict.json 1d2ff9e5…), committed at 93509df.
- -2 SS runs: FAIL to PASS. Everyone in view went from 1 / 0.5875 / 0.7295 to 1 / 1 / 1.
- -17 L1 P2: INVALID (s2 and s3 by pre:PM-0001-2) to PASS, with 0 mismatches of 861,996 / 862,011 / 861,731 compared. The judge counts 17,739 / 17,566 / 18,027 incomplete over the whole run; npt-lab counts 17,260 / 17,089 / 17,532 in the measured window.
- -57 L1 P2: PASS, as in U4r's. Max 6,984 / 2,750 / 88,063 on every run, equal to U4r's. Position p99 5,111 on every run, against 5,257 / 5,280 / 5,283. Largest change: s3's position p99, -3.26 % (ADV-P0-2 corrects the card's '0.0 %').
- Overflow resyncs 0, so no flag. Every other summary figure equals U4r's same-seed line.
- Building figure 0. Bytes 622.2 MiB against 780.8 projected (0.797); U4r's P2 had 778.9.

**P0 (split):** commit 93509dfd7fd1, spike tree 07baa855. Log .gate/u4r-sets/P0-93509dfd7fd1.log (sha256 e07e882e…), exit 0, wall 2,566.1 s, peak 4,504.3 MiB. Results results/u4r/93509dfd7fd1/P0/ (runs.txt ada5a58a…, verdict.json e926b812…, verdict-bites.json b472784f…).
- -2 SS runs: FAIL to PASS. Everyone in view went from 0.861889 / 1 / 1 to 1 / 1 / 1. Shots per second 693.09 / 692.97 / 693.06 (U4r 694.16 / 694.19 / 693.98).
- -17 L1 P0: INVALID (s1 by pre:PM-0001-2) to PASS. 0 mismatches of 878,352 compared on each run; 0 incomplete; 0 overflow resyncs.
- -17's bites: invalid in U4r's set, now shown.
  - tick16 fails both checks: 12,151 mismatches and 720 incomplete.
  - client-rounding fails mismatches (878,352); incomplete 0 passes.
  - whole-as-lost fails incomplete (48); 0 mismatches passes.
- -57 L1 P0: PASS, as in U4r's. Max 6,984 / 2,750 / 88,063, equal. Position p99 5,737 / 5,736 / 5,739 against 5,719 / 5,718 / 5,715. Largest change: s3's position p99, +0.42 %.
- -57's bites: shown in both sets, each failing only its own check. pos-4cm 34,856 (U4r 34,830, +0.075 %); yaw-12bit 46,692 (equal); pitch-8bit 440,313 (U4r 440,312).
- Building figure 0 on all nine runs. Bytes 3,391.4 MiB against 3,614.6 projected (0.938); U4r's P0 had 3,248.3.
- **The tick16 bite (carry 58):** 94 overflow resyncs, against U4r's 48.
  - All are in the after phase: 47 at 65,611, 46 at 65,729 and 1 at 65,791. The window ends at 65,550. 48 slots.
  - No kick: 48 snapshot clients on every tick. No mismatch inside the measured window.
  - The 720 incomplete are 48 × 15 ticks, by design.
  - The cause: after the wrap, the room re-runs each soldier's last command. Four soldiers walk into buildings in the after phase. Slot 5, inside since 65,588, fills its queue more slowly, and the run waits for its resync while the others resync a second time.
  - The advisor rules that tick16 is shown and counts. Its 'shown' now rests on this 10-minute run. TICK16-WORDING is corrected (below).

**Overall:**
- -2's SS part passes at every profile: that is the fix's proof.
- -17's L1 P0, P2 and P3 parts and its three bites count from the fix's commit.
- -57 passes everywhere. Its largest change across the sets is P2's s3 position p99, -3.26 %.
- The logs total 4,304.3 MiB against 4,919.8 projected.
- -17 L1 B1 stays with U4g's judged B1 runs.

_Added by the main session after the quoted words, each from its file:_
- P0's results were committed at be41ce6, with the card and the P0 ruling (`git show --stat be41ce6`).
- Before P2 and before P0, one read-only main-run.sh call checked the amended M7-3 (advisor-brawl1-sets-p3-full.md,
  The M7-3 conflict). Every command exited 0, and each header reads `diff e3b0…; untracked e3b0…`:
  - pre-P2.log, 09:58:11 UTC: HEAD fc411116c1f1, spike tree 079ffffc, df 12,326,227,968 B;
  - pre-P0.log, 10:16:45 UTC: HEAD 93509dfd7fd1, spike tree 07baa855, df 11,665,391,616 B, against P0's floor of
    7,711 MiB.
- The three results folders' redact check (ADV-P0-3): .gate/u4r-sets/redact-BRAWL1-sets.log, "redact-check: 7 item(s)
  checked, 0 finding(s)".
- The 94 tick16 resyncs (47 at 65,611, 46 at 65,729, 1 at 65,791, all in the after phase, on 48 slots, with slots 17
  and 5 once each) and the 27 / 1 P3 resyncs were also counted from the raw lab.jsonl files, read-only.

[[KEEP u4r-report.md:657-661 unchanged: "### EVQ-LOAD, the carry line for U4f's and U4g's briefs" and its quoted words]]
[[KEEP u4r-report.md:663-667 unchanged: "### TICK16-WORDING, corrected (it replaces the line at :562-564)" and its quoted words]]
[[INSERT the text below, to the end of this section, after :667]]

_tick16's "shown" now rests on BRAWL-1's 10-minute P0 bite run (12,151 mismatches; the new check 720), not on r2's
30 s record (advisor-brawl1-sets-p0-full.md, The tick16 bite, "Why it still counts as shown"). The U4r P0 record's
line on this, in "The judged runs" above, now says so too._

### The corrections

- **ADV-BP-1 and ADV-BP-2**, the advisor's, recorded under condition 18 (advisor-brawl1-plan-full.md, Conditions, item
  18): "ADV-BP-1: N2-1's 'all four checks fail' was wrong; under the leave plant two fail (Q5). ADV-BP-2: my sets-r2
  figure of 'about 4.2 GiB free after the refill' mixed units."
- **ADV-BP-3**, the advisor's (advisor-brawl1-points-full.md, Point 2, "The change"): "Condition 6 (ii)'s identical-diff
  test could not hold at <base>, because the wallclock bite's L1 hash varies with wall-clock ns (pump.rs:236-243); and
  'comes to me whole' did not say stop."
  - Its masking rule (the same Point 2, "Ruling"): "Future captures of these tests mask exactly the L1 leg's hash values
    in the wallclock bite's comparer and result lines ('theirs ... hash' and the L1 hex). They keep the index, the
    loopback hash, compared and mismatching, and they explain libtest's 'filtered out' count by the binary's test count.
    Nothing else is masked."
- **ADV-BP-4**, the advisor's (advisor-brawl1-points-full.md, Point 1, "The change"): "RR4-L1's ruled name
  MAX_ENTRY_TICKS falls under check.mjs's TICK_NAMED rule and is refused as a literal; I did not check it; the name is
  MAX_ENTRY_TICK_COUNT."
- **ADV-PT-M1's home** (Medium, the points ruling; advisor-brawl1-r1-full.md, ADV-PT-M1, "recorded"):
  - The home is U4g's brief, through u4g-brief-carry.md (sha256 c3abeccb…). That file holds the round-1 ruling's U4g
    words word for word (advisor-brawl1-r1-full.md:279-288), merged with NA-B1-2 (c) and TE-B1-5's kinematic bound.
  - In short: B1's 1,832 cm is the leash's reach, not a stop, and it passes the 1,800 cm door entry. U4g's plan says how
    a B1 building entry reads in -17, or measures how often it happens, before its judged B1 runs.
  - This report's BRAWL-1 entry under Follow-ups points there.
- **ADV-R1-1 and ADV-R1-2**, the advisor's, in its words (advisor-brawl1-r2-full.md, The report words; also quoted in
  the round-2 subsection above):
  - ADV-R1-1: 'Two lines in FP-3 could not run as I wrote them: `--logs 0` without `--record` cannot parse, and
    `Some(&dir)` would not compile; copy 2's repairs are right.'
  - ADV-R1-2: 'Step 1 named the brief's section 4 for the rules; they are in sections 1, 2 and 5.'
- **ADV-M6-L1 to -L3** (Low, record-keeping; advisor-brawl1-m6-full.md, Findings). Each is done in this section:
  - ADV-M6-L1: the card gave M2 as 08:38-08:40, but the logs give (B) at 08:37:41-08:37:47 and (C) at 08:38:04-08:38:09,
    before M3 started at 08:38:44. M2's pgrep and df readings exist only on the card. Done: the merge record above.
    From M7 on, every df reading is in a log (M7-2).
  - ADV-M6-L2: m3.log ends with no free-space figure. Done: the post-M3 figures in M3 above, labelled "after M3 and M4,
    not at M3's end".
  - ADV-M6-L3: this section lacked a pointer to round 1's record. Done: the first item of "Round 1 and its ruling".
- **ADV-P3-1 and -2, and ADV-P0-1 to -3**, in the P0 ruling's report words (advisor-brawl1-sets-p0-full.md, "The report
  words", "Advisor corrections", lines 260-264):

- ADV-P3-1 (Medium): the P3 flags were not raised.
- ADV-P3-2 (Low, the advisor's): M7-3's wording.
- ADV-P0-1 (Low, the advisor's): TICK16-WORDING's 'one per client' and the computed 'wrap + 54-60'.
- ADV-P0-2 (Medium): P2's -57 p99 was left out of the card's record, so its '0.0 %' was wrong.
- ADV-P0-3 (Low): no redact-check log was kept for the P3 and P2 commits. The advisor's own read-only run on all three folders found 0.

  What each means and how it was closed:
  - **ADV-P3-1** (advisor-brawl1-sets-p3-full.md, Summary, Findings):
    - Carry 58's flags on P3's s1 and s3 were not raised to the advisor.
    - The card's evidence record left out the P3 evidence run's resyncs and its incomplete count.
    - No effect: nothing counted, and the set stopped for the ruling anyway.
    - Closed: both figures are in the evidence subsection above.
    - The lesson: check every runs.txt flag and every summary-line figure against U4r's same-seed line, not only the
      verdict lines.
  - **The same ruling's unnamed Low:** P3's pre-launch readings (pgrep, porcelain, df) exist only on the card. From P2
    on, they are command output in pre-P2.log and pre-P0.log.
  - **ADV-P3-2:** M7-3, as first written, could not hold, for two reasons:
    - set logs carry no diff/untracked header;
    - committing each set's results changes the spike tree.
    - Closed: M7-3 is amended, (a) to (d) (advisor-brawl1-sets-p3-full.md, The M7-3 conflict). P2 and P0 met the
      amended form (pre-P2.log, pre-P0.log).
  - **ADV-P0-1:**
    - "One overflow resync per client" holds only when every client's queue fills on the same tick.
    - The computed "wrap + 54-60" used the measured event rate, which falls after the wrap. The first resync came at
      wrap + 75.
    - No effect.
    - Closed: the corrected TICK16-WORDING (above). It replaces the old line in this report's U4r corrections
      (advisor-brawl1-sets-p0-full.md, The tick16 bite).
  - **ADV-P0-2:**
    - P2's -57 position p99 went from 5,257 / 5,280 / 5,283 to 5,111 on all three runs (results/u4r/ab10f058a828/P2/
      and results/u4r/fc411116c1f1/P2/, verdict.json, "positions: error p99").
    - So the largest change at P2 is s3's position p99, -3.26 %, not 0.0 %.
    - No verdict is affected. The re-run sets above carry the corrected figure.
  - **ADV-P0-3:** the redact-check log is now kept: .gate/u4r-sets/redact-BRAWL1-sets.log, 7 items, 0 findings.

### Follow-ups, each with its home (none triggers a push)

- **TE-B1-R2-1 with RR2-B1-1** (Low, test and doc only; advisor-brawl1-r2-full.md:40-53):
  - The change: the unit test's dead input moves from (1450, 0) to (0, 0) at bots lib.rs:404, and toward_centre's doc
    reads "x² + y² is at most 2^31" (bots lib.rs:132).
  - The home: the next lane that edits npt-bots' lib.rs, U6's bots'-view item at the latest, beside TE-B1-8's glue
    test.
- **TE-B1-R2-2** (Low; advisor-brawl1-r2-full.md:45-48):
  - The change: the runners' "untracked" field becomes `untracked $(git ls-files -z --others --exclude-standard -- . |
    xargs -0 -r sha256sum | sha256sum | cut -d' ' -f1)` at u4r-run.sh:67 and main-run.sh:42. Each gets a .diff and a
    silent `bash -n` (INF2-4's form).
  - The home: a main-session runner edit outside the change set, after BRAWL-1's last re-run set and before U4g's first
    run.
  - P0 was the last set, so it is now due (advisor-brawl1-sets-p0-full.md, The commit and the close, "Then, outside
    BRAWL-1").
  - Until then, every BRAWL-1 log's "untracked" value is the porcelain status list's hash.
- **ADV-PT-L3** (Low; advisor-brawl1-points-full.md:87-91):
  - The change: TICK_NAMED becomes /_TICKS$|_TICK_COUNT$|CATCHUP/, NOT_TICK_COUNTS gains MAX_ENTRY_TICK_COUNT with its
    reason, and check.test.mjs gains one case.
  - The home: the check.mjs follow-up (check.mjs's next change; before PR #40 is marked ready at the latest).
- **EVQ-LOAD, for U4f and U4g** (the carry line above; advisor-brawl1-sets-p3-full.md, The next sets;
  advisor-brawl1-sets-p0-full.md, The commit and the close):
  - The home is U4f's and U4g's briefs.
  - U4f's cap measurement (plan §4.7) uses the full-view rate.
  - U4g's plan reads carry 58's per-burst high-water marks from a short B1 run, before -18's judged B1 runs count.
  - U4g's brief also carries u4g-brief-carry.md (ADV-PT-M1, above).
- **From round 1** (advisor-brawl1-r1-full.md, FP-9 item 6 and "Each finding"):
  - RR-B1-7 (1), the room_tick loop rewrite: the next lane that edits logs.rs's room_tick (:402-405).
  - EA-B1-1's later half, with NA-B1-3's npt-client half: npt-sim exposes the flags index and the dead bit as pub
    items. The home: the next lane that opens crates/sim (beside EA-U4R-4), U6 at the latest (:372-375, :407-410).
  - TE-B1-8's optional glue test: U6's bots'-view item, EA-U4R-5's home (:357-360).
- **-17 L1 B1** stays with U4g's judged B1 runs (advisor-brawl1-sets-p0-full.md, The commit and the close, 2).
- **KEPT-LOGS** is under Decisions needed, below.

### Decisions needed

_The advisor's words, copied from advisor-brawl1-sets-p0-full.md, "For Mustafa", lines 278-284. "Your" is Mustafa,
and "my recommendation" is the advisor's._

- **KEPT-LOGS** (your open question, unchanged; new numbers, and now in the next unit's way): should a narrow kit rule let a session clear out old test-run logs once their results are saved and their rulings closed?
  - Why it matters now: kept raw run logs take about 11.4 GiB (U2 1.9, U4r 5.3, the bot fix's re-runs 4.2), plus 0.6 GiB of evidence runs. Free space is 7.5 GiB against the 4 GiB safety floor, and the next unit's (U4g's) build cache refills about 8-9 GiB. Its build cannot start until space is freed.
  - Options:
    - (a) Keep everything. U4g cannot build.
    - (b) Delete each closed run set's raw-log folder. This frees up to about 11.4 GiB. Those runs could never be re-judged; their verdicts stay saved.
    - (c) The same, but first pack each set into one compressed file, checked readable. Computed with the draft's measured ratio of about 3.5 to 1, this frees about 8 GiB and keeps about 3.3 GiB, and a re-judge stays possible.
  - My recommendation: (c), with the same rule naming the old 4 GB build cache, as in the draft. It comes as its own reviewed kit pull request (advisor plus security review) and merges only on your go. Nothing is deleted before then.

### For Mustafa

_In plain words, the advisor's, copied from advisor-brawl1-sets-p0-full.md, "For Mustafa", lines 271-275:_

- The bot fix has now passed all three re-run sets. Every bot stayed in view in every run, none walked into a building while the measurements ran, and the main snapshot check passes at every network level with zero errors.
- Every planted fault was caught, including the one that tests the tick counter wrapping round. Its 10-minute run now counts.
- That test sent more "resend the full game state" messages than I had predicted. I traced the cause in the logs: once the planted fault breaks the bots' commands, a few bots walk into buildings after the measurements end. While the run waits for one of them, the others are resent the state a second time. Nothing broke: no client was dropped, and no snapshot inside the measured part went wrong. I corrected my own wording of what that test should show.
- I also corrected one figure in the record of the second set. One of the position-accuracy figures, the 99th-percentile error, improved by up to 3.3 %; the record had said nothing changed. It decides nothing.
- What remains is paperwork: committing the last results and finishing the write-up. Nothing to check in a browser and nothing to merge.

1. Answer KEPT-LOGS (a, b or c) before U4g's build needs the disk. The bot fix's remaining steps do not wait for it.

_The numbered item is the advisor's (the same file, line 287). Nothing here needs a Brave check: this is spike code on
the test branch, nothing a player sees, hears or feels changes, and nothing merges to main._

[[END OF THE SECTION]]

---

## (1) The replacement for u4r-report.md:562-564 (the old TICK16 line)

The P0 ruling orders it (advisor-brawl1-sets-p0-full.md, The tick16 bite, ADV-P0-1, "Fix": "the corrected wording
above, on the card and in u4r-report.md, replacing the line at :562-564").

**Old text, exactly (u4r-report.md:562-564):**

```
     - TICK16-WORDING: from BRAWL-1's re-run sets on, tick16's expected line reads "one overflow resync per client, all
       in the after phase, none in the measured window, no kick", with no fixed offset. r2 had them at wrap + 60 and
       P0 at wrap + 79, because the reliable queue fills at the event rate after the wrap.
```

**New text, exactly:**

```
     - TICK16-WORDING, as corrected by advisor-brawl1-sets-p0-full.md ("The tick16 bite"; ADV-P0-1): "tick16's run is
       valid and the bite shown, failing both -17 checks. Its measured incomplete is 720: 48 clients times the 15
       measured ticks from the wrap (65,536) to the window's end (65,550). Every client has at least one overflow
       resync, all in the after phase, none in the measured window, and no kick. The count and the ticks are not fixed:
       the run ends only when every client's last measured ticks resolve, and while one client's queue fills slowly,
       the others can resync again, each after its forced full state was acknowledged." It replaces the sets-r2
       wording "one overflow resync per client", which held only while every client's queue filled on the same tick.
       r2's resyncs came at wrap + 60, and U4r's P0 bite run had 48, one per client, at wrap + 79
       (advisor-u4r-sets-r2-full.md:30, :34). BRAWL-1's P0 bite run had 94 on 48 clients, the first at wrap + 75
       (this report's BRAWL-1 section).
```

---

## (2) The "done" marker for the BRAWL-1 follow-up entry at u4r-report.md:215-239

It follows the report's own form for finished follow-ups ("U4r-F2, done at 791bc55", "U4r-F3, … done at 493b3d7",
"U4r-F1, done at f1f131b"). Only the entry's first two lines change. The rest, :217-239, stays as it is: the U4g
pointer at :217-222 is still live, and the rest is the record of what was asked.

**Old text, exactly (u4r-report.md:215-216):**

```
- **BRAWL-1, widened (Medium; the next npt-lab lane with N2-1, before U4g's judged B1 runs and before U4 closes;
  advisor-u4r-sets-r2-full.md, finding BRAWL-1, which replaces the sets-r1 fix line):**
```

**New text, exactly:**

```
- **BRAWL-1, widened, done: merged at b30314a, and its re-run sets P3, P2 and P0 count (results committed at fc41111,
  93509df and be41ce6; advisor-brawl1-sets-p0-full.md). The record is this report's last section, "BRAWL-1 (the
  bots' leash toward the centre, from the client's own decoded position)", with its own follow-ups** (Medium; the next
  npt-lab lane with N2-1, before U4g's judged B1 runs and before U4 closes; advisor-u4r-sets-r2-full.md, finding
  BRAWL-1, which replaces the sets-r1 fix line):
```

---

## (3) Optional, beyond the two asked: the line at u4r-report.md:525-526

Item 3b of the P0 ruling also lists, in its words: an update to the line saying tick16's "shown" rests on r2's 30 s
record (advisor-brawl1-sets-p0-full.md, The commit and the close, 3b). That line is in U4r's P0 record. Item 3b's list
as the task gave it left this item out, so it is drafted here apart.

**Old text, exactly (u4r-report.md:525-526; the sentence starts on :524):**

```
       pre:PM-0001-2. tick16's "shown" still rests on r2's 30 s record (c62h: shown, 2,880 mismatches, the new check
       720).
```

**New text, exactly:**

```
       pre:PM-0001-2. At U4r's commit, tick16's "shown" rested on r2's 30 s record (c62h: shown, 2,880 mismatches, the
       new check 720). From BRAWL-1's P0 re-run set on, it rests on that set's 10-minute bite run (shown, 12,151
       mismatches, the new check 720; advisor-brawl1-sets-p0-full.md, "The tick16 bite").
```
