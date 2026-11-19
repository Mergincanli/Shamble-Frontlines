# U4f2's brief, rev 1.1 (2026-11-19, after the advisor's check; the build waits for his U4F2-SIZE answer)
**The unit:** -21's forged-command suite and all of -21's runs, cut from U4f by Mustafa's SIZE-U4F (a) (2026-11-19
12:00 Amsterdam, word for word at history.md:6896-6903). The plan entry is u4-build-plan.md:855-879. This brief is
short by TEST-FIRST rule 3 (history.md:2234-2282): it says what U4f2 builds, how it is tested and what is open, and it
points to the rest. U4f's brief stays the reference for every rule U4f2 reuses.

**Rev 1 to rev 1.1:** the advisor's check, APPROVE WITH CONDITIONS (advisor-u4f2-brief.md, 290 lines: findings
ADV-U4F2B-1 to -11 at :247-259, rulings :198-233, conditions 1-8 at :266-286). Each condition met here: 1 (T-6) in
blocks 7, sections 2, 3 and 6; 2 (Q1 as ruled) in block 7 and section 3; 3 (C-U4F2-1 word for word) in block 6; 4 (p19's
`expect`) in block 8 and section 5; 5 (the builder's numbers, the npt-client lane) in section 4 and section 0; 6 (the
figure, the riders) in section 6 and the last section; 7 (the Lows) in blocks 4 and 7 and section 3; 8 (Q3, Q4) in
blocks 6 and 7. The advisor needs no new check of rev 1.1 unless a condition cannot be met as written (:244-245).

## 0. Where things are (stamps at rev 1: sha256's first 16 hex, lines; each re-checked by the advisor at :19-28)
- **The base:** U4f's merge 25d316e (spikes/netcode-proof tree f7ed90ad…). `<base>` is the lane head at launch, filled
  by the main session in U4f's fill form (u4f-brief.md section 12), with the spike tree outside results/ named equal to
  25d316e's or each difference listed.
- **Read, never retyped:** u4f-brief.md (135325a0bb524104, 2,040: **section 1, the start and the stops, :288-355**;
  2.4 item 6, 2.6, 2.7, 2.10, 2.12, section 4's build rules :831-958, 5.1-5.4, section 6's end checks :1163-1183,
  section 7's return list :1187-1222); the stop ruling advisor-u4f-stop.md (38821ebd2c870ca9, 247: U4f2's list :80-89,
  its counts :95-97); the 22 lines of advisor-u4f-build1.md:162-246 (da98295e0d0eae7c, 283); round 1's ruling
  advisor-u4f-r1.md (07c1bcd593de4855, 407: U4f2's carries :65-87); the pass check advisor-u4f-pass.md
  (580698d24b7e57a3, 186: ADV-U4FP-3 :95-98); the r2 read advisor-u4f-r2.md (ac888088d2d23f97, 247: F1-F6 :61-153); the
  merge head check advisor-u4f-merge-head.md (5b1e95b4e0400bdf, 242: ADV-U4FMH-2 :41-66; U4f2's stop :133-142);
  u4-build-plan.md (d3e3ddbbc04e0411, 1,163); u4s-carries.md (4ef5909df8e14079, 694: U4f2's section :541-547); builder
  3's record u4f-build.md (1ef1e1d71e4432b8, 1,182: what is not built :855-866); U4f's report u4f-report.md
  (e7fe7afbdb0516d7, 602: the share :228-244); **this brief's check advisor-u4f2-brief.md** (290 lines).

## 1. What U4f2 builds, in blocks (each at most about 400 added lines, as the build ruling's line 4)
1. **f28's clamp and lag-switch halves under option B** (the stop ruling's R3 (ii), advisor-u4f-stop.md:147-150;
   u4f-report.md:222-226): one test per suite and option, as build ruling 1's line 6 asks.
2. **npt-bots:** the forgers of kinds 3, 7, 8, 9, 10 and 11, with f15's six cases (u4f-brief.md 2.6). Kinds 1, 2, 4,
   5 and 6, `Behaviour::forger` and `take_forged` were built ahead in U4f (bots/src/lib.rs:310, :322, :1143-1170).
3. **npt-lab, the suite:** `--suite forged` (11 forger slots, 37 honest open-loop clients) and `--forge-every`, with
   its header param and its SERIES.md row (2.7); C-U4F2-1's parser half (a value below 2 is a usage error).
4. **npt-lab, the attribution** (2.4 item 6) with `event.forged_kind.sent_count` and `.unrejected_count`. C-U4F2-3
   comes first: npt-room's `Refusal` enum renamed `CommandRefusal`, its records in the sim's struct form `{ slot, seq,
   kind }` (39 lines in crates/room name the enum: input.rs 22, room.rs 16, lib.rs 1; its records' one consumer is
   room/tests/commands.rs:82; **the sim's own struct `Refusal` stays**, crates/sim being outside the lane). Then
   C-U4F2-SEC-1 (`Room::refusals_dropped()` logged beside the records the attribution reads).
5. **npt-lab, the twins** (2.12; carry 46's lab half): the twin's skip through the built-ahead
   `ClientEndpoint::skip_command`, kind 10's and kind 3's twin rows, the clamp twins' `--twin-logs`; C-U4F2-2's unit
   test of `skip_command` and `twin_left_out`'s doc and row naming both causes.
6. **The tests f16 (24), f22 (2), f23 and f33 (13)** (2.10). C-U4F2-1's proof, word for word (advisor-u4f-r1.md:70-72):
   "f16, or a room-level case in its shape, shows that `forger(2, ·)` and `forger(5, ·)` at the judged interval, on the
   steady script at P0 into a `Room<World>`, leave no forged sequence without a refusal record of its own kind."
   **U4F2-Q4's case** (advisor-u4f2-brief.md:224-230): in f33, on hand-made records, a kind-10 refusal record naming a
   (slot, seq) the forger did not forge, on the forger's slot and on an honest one, changes neither `sent_count` nor
   `unrejected_count` (+1 test, a negative case beyond the 13).
7. **The routes from the r2 read and the head check:**
   - **C-U4F2-5** by U4F2-Q1 as ruled (section 3 gives its three points whole, with its stop).
   - **C-U4F2-7** by U4F2-Q3 as ruled (advisor-u4f2-brief.md:218-223): (a), keyed on the header's `params.suite` =
     `clock` (scenario.rs:1902; the one suite on `never_fires`, :1213-1214), printing "not applied (suite clock: its
     clients never fire)"; the band stays applied to every other 48-client run, the forged suite's included. The test:
     a 48-client clock record with shots 0 reads "not applied" and no failure; a 48-client record of another suite with
     shots 0 still fails (e); the plant (the band applied to `clock`) fails the first by name. **T-6's contract half
     rides in the same edit** (condition 1): five doctored cases, for carry 60's missing header param, a missing
     class_audit event, a duplicate admit, a log npt-log refuses (exit 1) and a usage error (exit 2) (P34's copy,
     u4f-brief.md:1958-1960; advisor-u4r-r1-full.md:122).
   - **C-U4F2-8** (`Room::cmd_timing()`'s consumer: the lab writes `params.cmd_timing` from the live room, f6's assert
     pins it; else the getter goes).
   - **ADV-U4FP-3:** client/src/lib.rs:360's doc rewrapped, and `plan()`'s short doc line ("… counted. No") with it
     (advisor-u4f-pass.md:95-96).
8. **The scripts:** 5.3's p1, p7, p14 and p17 in c-u4f-r2.mjs, with C-U4F2-4's reworded FP8 line and F5's rewordings
   (advisor-u4f-r2.md:130-142); **p19's `expect` (c-u4f-r2.mjs:64) reworded to section 5's reading** (condition 4):
   unjudged and reported, W's late drops, the bucket's limited datagrams, the cap's kind-10 refusals under option A,
   the per-tick maximum at most 8, with no fixed counts; -21's set in c-u4f-runs.mjs (24 measured, 12 twins, 13 bites,
   two judge calls: 5.4).
9. **The end checks** (section 2).

## 2. How it is tested
- **The share of 2.10's minimums** (u4f-report.md:228-231; condition 1): at least **+51 measured and +51 bites** (f15
  x6, f16 24, f22 2, f23 1, f33 13, and T-6's five, since C-U4F2-7 edits lab/src/contract.rs, untouched since U4r's
  71ee13d) and **22 negative cases** (f15 x6, f22 2, f23 1, f33 13). U4f2's report states its own counts and their sum
  with U4f's (+82, +99, 46) against 2.10's **+124 and +139** (its +119 and +134, plus "+5 if contract.rs is edited":
  u4f-brief.md:658, :1167) and **67** (advisor-u4f-stop.md:95-96). The 22 negatives are a floor with no slack
  (advisor-u4f2-brief.md:96-98). The extras the carries add (f28's halves, `skip_command`'s test, C-U4F2-1's parser
  case, C-U4F2-5's sim-only test, C-U4F2-7's two cases, Q4's f33 case) are named in the report with their `--list` lines.
- **Plants:** every new check and every new bite site fails by name under a plant, restored with the file's sha256
  (U4f's return list item 4, u4f-brief.md:1202-1204); C-U4F2-5's and C-U4F2-7's plants as their rulings give them.
- **The end checks:** u4f-brief.md section 6, items 1 to 5 (:1165-1182), as they apply: check.mjs twice with all 11
  steps, both clippy targets, fmt, npt-server's tests compiling (item 4, only if U4f2 touches them), each run-script
  command line by hand.
- **After the merge (the main session's):** c-u4f-r2.mjs whole from U4f2's merge commit, all 21 rows (5.3's record
  rule; advisor-u4f-stop.md:90-94), every -21 row reading "in a building: 0" and PM-0001-2 passing (C-U4F2-5); a row
  that reads other than its `expect` comes to the advisor before -21's set. Then -21's set, judged in two calls. -21 is
  judged only after U4f2 merges.

## 3. The carries, by number (met by number in U4f2's report)
- **Already in u4s-carries.md's U4f2 section (:541-547):** 46's lab half; 86 for the forged suite's option-A rows and
  -21's 5.3 record; C-U4F2-1, C-U4F2-SEC-1, C-U4F2-2, C-U4F2-3 and C-U4F2-4 (advisor-u4f-r1.md:69-85); ADV-U4FP-3
  (advisor-u4f-pass.md:95-98).
- **Four numbers this brief adds** (one pointer line into u4s-carries.md's U4f2 section at launch, as :545-547 does;
  the texts stay where they are):
  - **C-U4F2-5** = ADV-U4FR2-1 (F1, Medium), its route at advisor-u4f-r2.md:87-97, met by U4F2-Q1 as ruled
    (advisor-u4f2-brief.md:199-214), word for word: "(b), or (c) if (b) cannot be shown; never (a)." … "Under (b) or (c):
    1. only Kind::Steady's `move_xy` changes: its fire, aim and presses stay (the shot band, carry 86), and MOVES,
    TURN_EVERY and the Brawl, NoFire, Presses, Clamp and Lane arms stay as at 25d316e (the report shows it from `git diff
    25d316e -- crates/bots/src/lib.rs` outside the test module); 2. the content stays a function of seed, slot and step
    (2.12's rule), so the twin's skip leaves later commands equal (f16 shows it); 3. the sim-only test, on BRAWL-1's
    model with the script as its parameter: option B's patterns as today plus an option-A pattern at P1 (each slot's
    commands run in the tick they arrive: several in one tick, none in another, from P1's one-way 25 ms ± 5 ms and the
    controller's lead), all 48 slots on `steady` (the 11 forgers' base walk included, since PM-0001-2's in-view check
    reads every slot); 610 s, seeds 1 to 3, in release (`--ignored`), with a short copy in the debug test step; **the
    plant is today's walk, and it must read in a building above 0 in at least one pattern and seed**. If no pattern
    makes today's walk enter, the test cannot show the fix: the build stops on it and comes to me with the figures."
    Why never (a): a leash feeds the decoded position back into the commands, against the plan's open-loop twin rule
    (u4-build-plan.md:411-412; u4f-brief.md:533, :774-776).
  - **C-U4F2-6** = ADV-U4FR2-2 (F2, Medium), :110-113: this brief restates p19's reading and corrects U4F-Q15 (c)'s
    sentence (section 5); -20's set report names it in the OQ-0001-6 reading.
  - **C-U4F2-7** = ADV-U4FR2-4 (F4, Low), :127-129, met by U4F2-Q3 as ruled (block 7), **with T-6's contract half
    riding** (advisor-u4r-r1-full.md:122; u4f-brief.md P34 :1958-1960 and 9.2 :1309: "rides if … contract.rs" is edited).
  - **C-U4F2-8** = ADV-U4FMH-2 (Low), advisor-u4f-merge-head.md:57-64: the lab writes the header's `params.cmd_timing`
    from the live room (`server.room.cmd_timing()`) and f6's header assert pins it, else the getter goes; §4.1's
    npt-room row updated in U4f2's merge either way.
- **Into U4f2's record lines (no number):** F3, p20's episode (advisor-u4f-r2.md:114-122); F5 (a) and (b), the
  wording slips (:130-137); F5 (c), 5.3's list gaining "0 presses lost" as reached by the redundancy-1 bite (:138-141).
- **Two deferred Lows U4f left "for their next taker"** (u4f-build.md:557-559): RR-CB-2's docs and RR-B1-7 (1)'s
  `room_tick` loop rewrite, in scenario.rs and logs.rs, which U4f2 opens. They ride or are left by line 15's rule
  (advisor-u4f-build1.md:215-219: only where the file is open and cheap, and left if the count nears the stop); the
  report says which.

## 4. The rules that stand
- **The builder's start and stops are u4f-brief.md section 1's** (:288-355; condition 5), read for U4f2 as follows:
  - "your added lines past 6,000" (:323) reads **past 3,000 added lines at a block end** (his SIZE-CHECK on his (a)'s
    figure), by the build ruling's line 5 (advisor-u4f-build1.md:178-181): build nothing more, run the end checks on
    the tree as it stands, write the count and the estimate for the rest, return;
  - the counts are deltas on **25d316e's 634 measured and 731 bites** (u4f-report.md:232), not 5b29296b's;
  - the test step's stop line is **line 22's 1,207 s**, which stands (advisor-u4f-build1.md:242).
- U4f's build rules (u4f-brief.md section 4), with U4f2's lane: npt-bots, npt-lab, ops/runs, SERIES.md (its new rows
  only), npt-room for C-U4F2-3 and C-U4F2-8 only, **npt-client for C-U4F2-2 (`skip_command`'s unit test,
  `twin_left_out`'s doc) and ADV-U4FP-3**. crates/sim stays shut.
- **Nothing in marks.json, the judge, marks/README.md or an existing SERIES.md row changes** (his SIZE-U4F (a): "No
  pass mark, judge rule, marks.json text or run count changes"; line 22). A build that needs one stops, and the main
  session asks him.
- LEAD-BOUND (a), 400 ms, frozen (history.md:6791-6806); the event cap 1,600 (advisor-u4f-stop.md R2), unless -20's
  set read moves it first by R2's own route.
- Every agent brief carries, word for word: "Write files with the Write and Edit tools only (no sed, no shell
  redirect, no python writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit
  tool rewrites its notes file whole with Write."
- One builder in an isolated worktree, Opus 5.5 at xhigh (netcode), notes in its worktree's copy of u4f2-build.md;
  snapshots by worktree_patch.py; the shared target under the lane lock; at most three agents at a time.

## 5. C-U4F2-6: p19's reading restated, and U4F-Q15 (c)'s sentence corrected
- **What p19 showed** (r2.txt:122-139; advisor-u4f-r2.md:100-105; checked against p19's own logs at
  advisor-u4f2-brief.md:109-113): over the five 400 ms holds under option A, the per-tick cap (room/src/input.rs:323)
  refused **7** of the holder's commands under kind 10, a forged kind (`buffer_full_count` 7, slot 0);
  `lagswitch_cmds_count` reached 8, the cap; W dropped 2, 2, 2, 0 and 0 (6 in all); the Input bucket limited 25
  datagrams (about 5 a hold); 48 sequences fell outside the kept window.
- **The sentence as ruled** (advisor-u4f-brief.md:293-295; u4f-brief.md:1620-1624): "p19's arithmetic holds (the Input
  bucket passes 8 of 12 datagrams, pump.rs:20, :84; W drops the oldest 3 or 4), so an honest hold never puts more than
  8 new commands into a tick", and "the cap refuses nothing there". **Corrected:** under option A an honest 400 ms hold
  can reach the per-tick cap of 8, and the cap then refuses the excess under kind 10 (p19: 7 refused). p19 is unjudged
  and no -21 check reads refusals (marks.json's -21 checks at :4805-5145 read `event.forged_kind.*` and the cheats'
  series), so no bar or verdict moves; the attribution never counts an honest refusal (U4F2-Q4, block 6).
- **Superseded with it:** 5.3's p19 row (u4f-brief.md:1037: "the kind-10 refusals (0 expected)", "about 3 or 4",
  "about 4") beside :1620-1624; its executable copy, c-u4f-r2.mjs:64's `expect`, is reworded in block 8.

## 6. Size (SIZE-CHECK, history.md:2048-2051)
- **His figure:** about 1,500 added lines (his SIZE-U4F (a), on advisor-u4f-stop.md:36-38), so the stop is past
  3,000. This brief may confirm the figure, not raise it; an estimate above about 1,500 goes to him before the build,
  and the stop does not move (advisor-u4f-merge-head.md:133-142).
- **The estimate** (added lines in spikes/; builder 3's items from u4f-build.md:863-866; the rest by the advisor's
  check, advisor-u4f2-brief.md:129-156):

| Item | Where | Estimate |
|---|---|---|
| f28's two B halves (block 1) | lab tests | about 40 |
| Six forgers and f15's six cases (block 2) | bots | about 250 |
| The suite, the attribution, the twins, f16, f22, f23, f33 (blocks 3-6) | lab | 950 to 1,100 |
| SERIES.md, c-u4f-r2.mjs's rows, c-u4f-runs.mjs's -21 set (block 8) | docs, ops | about 165 |
| **Builder 3's estimate, before round 1** | | **1,405 to 1,555** |
| C-U4F2-1 (parser and its case) | lab | 15 to 65 |
| C-U4F2-SEC-1 (`refusals_dropped` logged) | lab | 5 to 15 |
| C-U4F2-2 (`skip_command`'s test, `twin_left_out`'s doc and row) | client, docs | 25 to 40 |
| C-U4F2-3 (`CommandRefusal` and the struct form; 39 room lines) | room, lab | 60 to 90 |
| C-U4F2-4, F5's rewordings, p19's `expect` | ops | 6 to 13 |
| C-U4F2-5 (Steady's move and the sim-only test with an option-A pattern) | bots, tests | 100 to 220 |
| U4F2-Q4's f33 case | lab | 10 to 20 |
| RR-CB-2's docs, if it rides | lab | 0 to 5 |
| **Rider:** C-U4F2-7 (the band "not applied" and its two cases) | lab | 15 to 50 |
| **Rider:** T-6's contract half (five doctored cases) | lab | 60 to 150 |
| **Rider:** C-U4F2-8 (`cmd_timing`'s consumer and f6's assert) | lab | 2 to 10 |
| **Rider:** ADV-U4FP-3 (two doc lines) | client | about 2 |
| **Total** | | **about 1,700 to 2,240** |

- **The figure for him:** about **1,700 to 2,250** added lines (midpoint about 1,970); at U4f's overall 1.2
  (advisor-u4f-stop.md:104-105), up to about **2,700**. Both under the stop past 3,000, which stays.
- **What his (a) sends and what rides:** blocks 1-6, 8, C-U4F2-1 to -4, SEC-1, -5 and -6 are the forged-command test
  and -21's runs his (a) names, with their review fixes. **The four riders** (C-U4F2-7 with T-6, C-U4F2-8, ADV-U4FP-3:
  about 80 to 215 lines together) ride U4f2's lane by the advisor's rulings under TEST-FIRST rule 2, because U4f2 opens
  their files; they are not what his (a) sent (advisor-u4f2-brief.md:153-156). His cheaper option: leave the riders for
  the next unit that opens those files.

## 7. Reviewers and rounds
netcode-architect, security-auditor (hostile input: the forged kinds, the attribution, `--forge-every`'s parser),
test-engineer and rust-reviewer (advisor-u4f-stop.md:88-89), each at xhigh, in two batches (at most three agents at a
time). At most two review rounds (a third only for a Critical or High); every marks.json or judge change would need a
reviewer's read, and none is planned.

## 8. Disk and time (each an estimate; the runner projects at launch)
- -21's set: 24 measured, 12 twins and 13 bites, 49 runs (5.1), judged in two calls (5.4). At 10 minutes each and
  about 50 to 63 MiB of logs a run (-20's first run: 62.8 MiB at P3), about 2.4 to 3.0 GiB; the runner's projection at
  launch decides. With -20 and -22 done, about 10,500 MiB should be free against the 4,096 floor; D2 before the set.
- c-u4f-r2.mjs whole (21 rows) after the merge: about 80 MiB (r2's 17 rows took 79 MiB: advisor-u4f-r2.md:160).

## 9. The advisor's rulings on U4F2-Q1 to Q5 (advisor-u4f2-brief.md:198-233; applied above)
- **Q1, C-U4F2-5's fix:** (b), or (c) if (b) cannot be shown; never (a); points 1-3 and the stop as section 3 gives them.
- **Q2, the size:** confirmed above about 1,500, so to him before the build; the stop past 3,000 stays; the figure
  corrected to about 1,700 to 2,250 (up to about 2,700 at 1.2), the riders named apart (section 6).
- **Q3, C-U4F2-7's form:** (a), keyed on `params.suite` = `clock`, both cases and the plant, T-6 riding (block 7).
- **Q4, honest kind-10 refusals and the attribution:** confirmed (only forged items enter `unrejected_count`; the
  forged suite runs at P0 with no honest hold), with one f33 case on the forger's own slot (block 6).
- **Q5, the clock suite's open walk:** no change in U4f2; Q1's point 1 keeps NoFire's commands as they are; U4f2's
  report names it beside C-U4F2-5's fix.

## What goes to Mustafa from this brief
- **U4F2-SIZE**, in the next day's message, first line "stopped on you": U4f2's estimate is about 1,700 to 2,250 added
  lines (up to about 2,700 at U4f's 1.2 overrun) against the 1,500 of his (a); the stop past 3,000 stays. My lean (and
  the advisor's): build it whole. His cheaper option: leave the four riders (about 80 to 215 lines) for the next unit
  that opens those files. The build waits for his answer.
- Nothing else: no pass mark, judge, marks.json text, money or main.
