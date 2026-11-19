# U4f2's brief, rev 1 (draft of 2026-11-19, for the advisor's check before the build)
**The unit:** -21's forged-command suite and all of -21's runs, cut from U4f by Mustafa's SIZE-U4F (a) (2026-11-19
12:00 Amsterdam, word for word at history.md:6896-6903). The plan entry is u4-build-plan.md:855-879. This brief is
short by TEST-FIRST rule 3 (history.md:2234-2282): it says what U4f2 builds, how it is tested and what is open, and it
points to the rest. U4f's brief stays the reference for every rule U4f2 reuses.

## 0. Where things are (stamps at the draft: sha256's first 16 hex, lines)
- **The base:** U4f's merge 25d316e (spikes/netcode-proof tree f7ed90ad…). `<base>` is the lane head at launch, filled
  by the main session in U4f's fill form (u4f-brief.md section 12), with the spike tree outside results/ named equal to
  25d316e's or each difference listed.
- **Read, never retyped:** u4f-brief.md (135325a0bb524104, 2,040: 2.4 item 6, 2.6, 2.7, 2.10, 2.12, section 4's build
  rules :831-958, 5.1-5.4, section 6's end checks :1163-1183, section 7's return list :1187-1222); the stop ruling
  advisor-u4f-stop.md (38821ebd2c870ca9, 247: U4f2's list :80-89, its counts :95-97); the 22 lines of
  advisor-u4f-build1.md:162-246 (da98295e0d0eae7c, 283); round 1's ruling advisor-u4f-r1.md (07c1bcd593de4855, 407:
  U4f2's carries :65-87); the pass check advisor-u4f-pass.md (580698d24b7e57a3, 186: ADV-U4FP-3 :95-98); the r2 read
  advisor-u4f-r2.md (ac888088d2d23f97, 247: F1-F6 :61-153); the merge head check advisor-u4f-merge-head.md
  (5b1e95b4e0400bdf, 242: ADV-U4FMH-2 :41-66; U4f2's stop :133-142); u4-build-plan.md (d3e3ddbbc04e0411, 1,163);
  u4s-carries.md (4ef5909df8e14079, 694: U4f2's section :541-547); builder 3's record u4f-build.md (1ef1e1d71e4432b8,
  1,182: what is not built :855-866); U4f's report u4f-report.md (e7fe7afbdb0516d7, 602: the share :228-244).

## 1. What U4f2 builds, in blocks (each at most about 400 added lines, as the build ruling's R3)
1. **f28's clamp and lag-switch halves under option B** (the stop ruling's R3 (ii), advisor-u4f-stop.md:147-150;
   u4f-report.md:222-226): one test per suite and option, as build ruling 1's line 6 asks.
2. **npt-bots:** the forgers of kinds 3, 7, 8, 9, 10 and 11, with f15's six cases (u4f-brief.md 2.6). Kinds 1, 2, 4,
   5 and 6, `Behaviour::forger` and `take_forged` were built ahead in U4f (bots/src/lib.rs:310, :322, :1143-1170).
3. **npt-lab, the suite:** `--suite forged` (11 forger slots, 37 honest open-loop clients) and `--forge-every`, with
   its header param and its SERIES.md row (2.7); C-U4F2-1's parser half (a value below 2 is a usage error).
4. **npt-lab, the attribution** (2.4 item 6) with `event.forged_kind.sent_count` and `.unrejected_count`. C-U4F2-3
   comes first (npt-room's `Refusal` renamed `CommandRefusal`, its records in the sim's struct form `{ slot, seq, kind
   }`; `git grep -w Refusal` finds 51 lines in crates/ today), then C-U4F2-SEC-1 (`Room::refusals_dropped()` logged
   beside the records the attribution reads).
5. **npt-lab, the twins** (2.12; carry 46's lab half): the twin's skip through the built-ahead
   `ClientEndpoint::skip_command`, kind 10's and kind 3's twin rows, the clamp twins' `--twin-logs`; C-U4F2-2's unit
   test of `skip_command` and `twin_left_out`'s doc and row naming both causes.
6. **The tests f16 (24), f22 (2), f23 and f33 (13)** (2.10), f16 or a room-level case in its shape carrying C-U4F2-1's
   proof (`forger(2, ·)` and `forger(5, ·)` at the judged interval leave no forged sequence without a refusal record).
7. **The routes from the r2 read and the head check:** C-U4F2-5 (the honest open-loop script kept out of buildings,
   with its sim-only test and plant), C-U4F2-7 (the contract's shot band "not applied" for a never-firing suite),
   C-U4F2-8 (`Room::cmd_timing()`'s consumer, or its removal), ADV-U4FP-3 (client/src/lib.rs:360's doc rewrapped).
8. **The scripts:** 5.3's p1, p7, p14 and p17 in c-u4f-r2.mjs, with C-U4F2-4's reworded FP8 line and F5's rewordings
   (advisor-u4f-r2.md:130-142); -21's set in c-u4f-runs.mjs (24 measured, 12 twins, 13 bites, two judge calls: 5.4).
9. **The end checks** (section 6 below).

## 2. How it is tested
- **The share of 2.10's minimums** (u4f-report.md:228-231): at least **+46 measured and +46 bites** (f15 x6, f16 24,
  f22 2, f23 1, f33 13) and **22 negative cases** (f15 x6, f22 2, f23 1, f33 13). U4f2's report states its own counts
  and their sum with U4f's (+82, +99, 46) against 2.10's +119, +134 and 67 (advisor-u4f-stop.md:95-96).
- **Plants:** every new check and every new bite site fails by name under a plant, restored with the file's sha256
  (U4f's return list item 4, u4f-brief.md:1202-1204). C-U4F2-5's test is sim-only, on BRAWL-1's model
  (brawl1-review-r1-full.md:30-32).
- **The end checks:** u4f-brief.md section 6, items 1 to 5 (:1165-1182), unchanged as they apply: check.mjs twice with
  all 11 steps, both clippy targets, fmt, npt-server's tests compiling (item 4, only if U4f2 touches them), each
  run-script command line by hand.
- **After the merge (the main session's):** c-u4f-r2.mjs whole from U4f2's merge commit, all 21 rows (5.3's record
  rule; advisor-u4f-stop.md:90-94), every -21 row reading "in a building: 0" and PM-0001-2 passing (C-U4F2-5); a row
  that reads other than its `expect` comes to the advisor before -21's set. Then -21's set, judged in two calls. -21 is
  judged only after U4f2 merges.

## 3. The carries, by number (met by number in U4f2's report)
- **Already in u4s-carries.md's U4f2 section (:541-547):** 46's lab half; 86 for the forged suite's option-A rows and
  -21's 5.3 record; C-U4F2-1, C-U4F2-SEC-1, C-U4F2-2, C-U4F2-3 and C-U4F2-4 (advisor-u4f-r1.md:69-85); ADV-U4FP-3
  (advisor-u4f-pass.md:95-98).
- **Four numbers this brief adds** (one pointer line into u4s-carries.md's U4f2 section at the advisor's check, as
  :545-547 does; the texts stay where they are):
  - **C-U4F2-5** = ADV-U4FR2-1 (F1, Medium), its route word for word at advisor-u4f-r2.md:87-97: every -21 suite's
    honest clients stay out of buildings for 610 s at P0 and P1 under both options; a sim-only test with a plant
    shows it; the choice of fix comes to the advisor (question U4F2-Q1).
  - **C-U4F2-6** = ADV-U4FR2-2 (F2, Medium), :110-113: this brief restates p19's reading and corrects U4F-Q15 (c)'s
    sentence (section 5 below); -20's set report names it in the OQ-0001-6 reading.
  - **C-U4F2-7** = ADV-U4FR2-4 (F4, Low), :127-129: the contract's shot band, "not applied" for a never-firing suite
    (lab/src/contract.rs:227 applies it by client count alone; -47's five runs read shots 0 by construction:
    u4f-report.md, "After the merge", item 4).
  - **C-U4F2-8** = ADV-U4FMH-2 (Low), advisor-u4f-merge-head.md:57-64: the lab writes the header's `params.cmd_timing`
    from the live room (`server.room.cmd_timing()`) and f6's header assert pins it, else the getter goes; §4.1's
    npt-room row updated in U4f2's merge either way.
- **Into U4f2's record lines (no number):** F3, p20's episode (advisor-u4f-r2.md:114-122); F5 (a) and (b), the
  wording slips (:130-137); F5 (c), 5.3's list gaining "0 presses lost" as reached by the redundancy-1 bite (:138-141).

## 4. The rules that stand
- U4f's build rules (u4f-brief.md section 4), with U4f2's lane: npt-bots, npt-lab, ops/runs, SERIES.md (its new rows
  only), npt-room for C-U4F2-3 and C-U4F2-8 only, npt-client for C-U4F2-2's doc and ADV-U4FP-3 only.
- **Nothing in marks.json, the judge or an existing SERIES.md row changes** (his SIZE-U4F (a): "No pass mark, judge
  rule, marks.json text or run count changes"). A build that needs one stops and the main session asks him.
- LEAD-BOUND (a), 400 ms, frozen (history.md:6791-6806); the event cap 1,600 (advisor-u4f-stop.md R2), unless -20's
  set read moves it first.
- Every agent brief carries, word for word: "Write files with the Write and Edit tools only (no sed, no shell
  redirect, no python writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit
  tool rewrites its notes file whole with Write."
- One builder in an isolated worktree, Opus 5.5 at xhigh (netcode), notes in its worktree's copy of u4f2-build.md;
  snapshots by worktree_patch.py; the shared target under the lane lock; at most three agents at a time.

## 5. C-U4F2-6: p19's reading restated, and U4F-Q15 (c)'s sentence corrected
- **What p19 showed** (r2.txt; advisor-u4f-r2.md:100-105): over the five 400 ms holds under option A, the per-tick cap
  (room/src/input.rs:323) refused **7** of the holder's commands under kind 10, a forged kind (`buffer_full_count` 7,
  slot 0); `lagswitch_cmds_count` reached 8, the cap; W dropped 2, 2, 2, 0 and 0 (6 in all); the Input bucket limited
  25 datagrams (about 5 a hold); 48 sequences fell outside the kept window.
- **The sentence as ruled** (advisor-u4f-brief.md:293-295; u4f-brief.md:1620-1624): "p19's arithmetic holds (the Input
  bucket passes 8 of 12 datagrams, pump.rs:20, :84; W drops the oldest 3 or 4), so an honest hold never puts more than
  8 new commands into a tick", and "the cap refuses nothing there". **Corrected:** under option A an honest 400 ms hold can reach the per-tick cap of 8, and
  the cap then refuses the excess under kind 10 (p19: 7 refused). p19 is unjudged and no -21 check reads refusals, so
  no bar or verdict moves; the attribution's honest-client rule (2.4 item 6) must not count those 7 as forged kind 10
  (question U4F2-Q4).

## 6. Size (SIZE-CHECK, history.md:2048-2051)
- **His figure:** about 1,500 added lines (his SIZE-U4F (a), on advisor-u4f-stop.md:36-38), so the stop is past
  3,000. This brief may confirm the figure, not raise it; an estimate above about 1,500 goes to him before the build,
  and the stop does not move (advisor-u4f-merge-head.md:133-142).
- **This brief's estimate** (added lines in spikes/; builder 3's items from u4f-build.md:863-866, the rest mine):

| Item | Where | Estimate |
|---|---|---|
| f28's two B halves (block 1) | lab tests | about 40 |
| Six forgers and f15's six cases (block 2) | bots | about 250 |
| The suite, the attribution, the twins, f16, f22, f23, f33 (blocks 3-6) | lab | 950 to 1,100 |
| SERIES.md, c-u4f-r2.mjs's rows, c-u4f-runs.mjs's -21 set (block 8) | docs, ops | about 165 |
| **Builder 3's estimate, before round 1** | | **1,400 to 1,550** |
| C-U4F2-1 (parser and its case) | lab | 15 to 65 |
| C-U4F2-SEC-1 (`refusals_dropped` logged) | lab | 5 to 15 |
| C-U4F2-2 (`skip_command`'s test, `twin_left_out`'s doc and row) | client, docs | 25 to 40 |
| C-U4F2-3 (`CommandRefusal` and the struct form; 51 lines name `Refusal`) | room, lab | 60 to 90 |
| C-U4F2-4, ADV-U4FP-3, F5's rewordings | ops, client | 7 to 13 |
| C-U4F2-5 (the fix and its sim-only test with a plant) | bots, tests | 80 to 180 |
| C-U4F2-7 (the band "not applied" and its test) | lab | 15 to 50 |
| C-U4F2-8 (`cmd_timing`'s consumer and f6's assert) | lab | 2 to 10 |
| **The carries since** | | **about 210 to 460** |
| **Total** | | **about 1,610 to 2,010 (midpoint about 1,800)** |

- U4f's own estimates ran about 1.2 times (advisor-u4f-stop.md:104-105); the total above is not scaled by it.
- **Reading:** above about 1,500, so it goes to him before the build (question U4F2-Q2), with my lean: build U4f2
  whole at this estimate, the stop past 3,000 unchanged (the midpoint leaves a factor of about 1.67 to the stop, the
  top about 1.49). The carries since builder 3's estimate are review fixes his (a) sends with the unit.

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

## 9. Questions for the advisor (each where the plan or a ruling leaves a choice open)
- **U4F2-Q1, C-U4F2-5's fix.** (a) the leash on `steady` (`Kind::Steady` beside `Kind::Brawl` at
  bots/src/lib.rs:652-656: closed loop on the client's own decoded position); (b) an open-loop walk kept clear of the
  doors (the twins' hashes stay deterministic by construction); (c) another honest script. Lean: (b) if it can be shown
  for 610 s at P0 and P1 under both options by the sim-only test, since the twins (2.12) compare runs whose decoded
  positions differ by the skipped command; else (a) with the twins' rows shown equal on it.
- **U4F2-Q2, the size.** Confirm the estimate (section 6) and that it goes to him before the build, the stop past
  3,000 unchanged. Lean: as section 6.
- **U4F2-Q3, C-U4F2-7's form.** (a) the band reads "not applied" when the header's suite is one whose clients never
  fire (`clock`); (b) a header flag the lab writes from the behaviour. Lean: (a), the smaller change, with a test that
  fails under a plant reading the band as applied.
- **U4F2-Q4, the attribution and an honest hold's kind-10 refusals (section 5).** The attribution counts a forged kind
  by the forger's slot and stamp (2.4 item 6), so p19's honest slot-0 refusals are not forged sends; confirm that the
  forged suite's honest clients never hold (no lag switch in the forged suite), so no honest kind-10 refusal reaches
  `event.forged_kind.unrejected_count`. Lean: confirm, with a negative case in f16 or f33.
- **U4F2-Q5, the clock suite's open walk** (-47's seed 2: 1,916 slot-ticks in a building). -47 has no in-view
  precondition (advisor-u4f-r2.md:81-82). Lean: no change in U4f2 (outside -21; nothing reads it), named in U4f2's
  report beside C-U4F2-5's fix.

## What goes to Mustafa from this brief
- **U4F2-SIZE** (if the advisor confirms section 6): U4f2's estimate is about 1,600 to 2,000 added lines against the
  1,500 of his (a); the stop past 3,000 stays. In the next day's message, first line "stopped on you", with the lean.
- Nothing else: no pass mark, judge, marks.json text, money or main.
