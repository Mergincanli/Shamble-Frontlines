# Advisor check: U4f2's brief, rev 1 (u4f2-brief.md, 167 lines), before its build
Role: main's `.claude/agents/advisor.md` (git show origin/main:..., 335 lines). Branch claude/netcode-proof-test-plan-hcky5g
at 8dd5c6d (read only: no switch, no commit, no push, nothing run under spikes/ or .gate/).

## Plan
1. Scope: brief section 1 against advisor-u4f-stop.md:80-89, u4-build-plan.md:855-879, u4f-build.md:855-866,
   u4f-report.md:212-226.
2. Carries: C-U4F2-5..-8 against advisor-u4f-r2.md:61-153 and advisor-u4f-merge-head.md:41-66; existing ones against
   u4s-carries.md:541-547, advisor-u4f-r1.md:65-87, advisor-u4f-pass.md:95-98.
3. Test share against u4f-report.md:228-244 and 2.10.
4. Section 5 against r2.txt (p19), advisor-u4f-r2.md:98-113, advisor-u4f-brief.md:293-295, u4f-brief.md:1616-1625.
5. Section 6's size against the code (Refusal grep, contract.rs:220-232, bots/src/lib.rs:640-665) and
   advisor-u4f-merge-head.md:133-142.
6. Lane and standing rules (section 4).
7. Rulings U4F2-Q1..Q5.
8. Escalation check (pass mark, judge, marks.json, money, main).
Each block is added below as it finishes.

## Stamps (read at 8dd5c6d, each holds as the brief's section 0 gives it)
sha256 first 16 and lines, recomputed: u4f-brief.md 135325a0bb524104 2,040; advisor-u4f-stop.md 38821ebd2c870ca9 247;
advisor-u4f-build1.md da98295e0d0eae7c 283; advisor-u4f-r1.md 07c1bcd593de4855 407; advisor-u4f-pass.md 580698d24b7e57a3
186; advisor-u4f-r2.md ac888088d2d23f97 247; advisor-u4f-merge-head.md 5b1e95b4e0400bdf 242; u4-build-plan.md
d3e3ddbbc04e0411 1,163; u4s-carries.md 4ef5909df8e14079 694; u4f-build.md 1ef1e1d71e4432b8 1,182; u4f-report.md
e7fe7afbdb0516d7 602. The base: `git rev-parse 25d316e:spikes/netcode-proof` = f7ed90ad…; spikes/netcode-proof at HEAD
equals 25d316e's outside results/ (`git diff --quiet`, exit 0). The cites to Mustafa: SIZE-U4F (a) at history.md:6896-6903
(read: word for word), TEST-FIRST at :2234-2282 (rule 3's "five briefs go to the advisor" is the card's reading at
:1693 and :6297, not his text; his rule 3 is the brief's "what is built, how it is tested and what is still open"),
LEAD-BOUND at :6791-6806.

## 1. Scope (DONE)
Against the stop ruling's list (advisor-u4f-stop.md:80-89), the plan entry (u4-build-plan.md:855-879), builder 3's
not-built list (u4f-build.md:855-866) and U4f's report (u4f-report.md:211-226), item by item:
| Item (stop ruling :80-86) | Brief |
|---|---|
| forgers 3, 7, 8, 9, 10, 11 with f15's six | block 2 (:24-25) |
| `--suite forged` (11 forgers, 37 honest) and `--forge-every` with its param and row | block 3 (:26-27) |
| the attribution (2.4 item 6), `event.forged_kind.*` (sent_count, unrejected_count: u4f-build.md:857-858) | block 4 (:28-31) |
| the twins and the skip in the lab, kind 10's and kind 3's rows, the clamp twins' `--twin-logs` (u4f-build.md:860) | block 5 (:32-34) |
| f16 (24), f22 (2), f23, f33 (13) | block 6 (:35-36) |
| 5.3's p1, p7, p14, p17; -21's set in c-u4f-runs.mjs (24, 12, 13, two calls) | block 8 (:40-41) |
| f28's clamp and lag-switch B halves, as the first block (R3 (ii), :147-150) | block 1 (:22-23) |
| its own end checks (section 6 items 1-5) | block 9, section 2 (:51-53) |
| the size line (:86-88), the reviewers (:88-89) | sections 6, 7 |
| after the merge: c-u4f-r2.mjs whole from the merge commit, then -21's set (:90-94) | section 2 (:54-57) |
Nothing of the four lists is missing. Added beyond them: block 7 and parts of blocks 3-5 and 8, each a carry an advisor
ruling sent after his (a): C-U4F2-1 to -4 and SEC-1 (round 1, review fixes on the forged test's own parts), C-U4F2-5 (-21's
runs' precondition), C-U4F2-6 (text only). Those sit inside his "the forged-command test ... and all of -21's runs".
Three do not: C-U4F2-7 (the -47 clock suite's contract band), C-U4F2-8 (a room getter's consumer) and ADV-U4FP-3 (a doc
wrap) ride U4f2's lane because it opens those files (advisor-u4f-r2.md:127-128; advisor-u4f-merge-head.md:57-63;
advisor-u4f-pass.md:98), by my rulings under TEST-FIRST rule 2, not by his (a). The brief's size reading (:131) says "The
carries since builder 3's estimate are review fixes his (a) sends with the unit": true for five of the eight, not for
those three (ADV-U4F2B-9). One standing follow-up the brief's own carries trigger is missing: T-6's contract half
(ADV-U4F2B-1). Two deferred Lows U4f left for "their next taker" in files U4f2 opens go unnamed (ADV-U4F2B-8).

## 2. The carries (DONE)
- **Existing, by number** (u4s-carries.md:541-547; advisor-u4f-r1.md:69-85; advisor-u4f-pass.md:95-98): 46's lab half, 86's
  forged rows and -21's 5.3 record, C-U4F2-1, SEC-1, -2, -3, -4, ADV-U4FP-3: all listed (brief :60-62). Each route kept,
  with two slips in the brief's own restatements:
  - **C-U4F2-1** (r1 :70-72): "`forger(2, ·)` and `forger(5, ·)` at the judged interval, on the steady script at P0 into a
    `Room<World>`, leave no forged sequence without a refusal record **of its own kind**." Block 6 (brief :35-36) drops
    "of its own kind" and the "steady script at P0 into a `Room<World>`" setting. The kind is the proof's point: kinds 2
    and 5 forge against the previous command (bots/src/lib.rs:304-309), and a record under another kind counts
    unrejected (f33's case). Looser as restated (ADV-U4F2B-4).
  - **ADV-U4FP-3** (pass :95-98) also names `plan()`'s short doc line ("… counted. No"); block 7 names only :360. Low,
    the carry's text binds by number.
  - **C-U4F2-3** (r1 :81-83): kept. The brief's "51 lines in crates/ today" is right as a count (`git grep -n -w Refusal
    -- spikes/netcode-proof/crates`: 51), but 12 of them are crates/sim naming the sim's own struct `Refusal { slot,
    seq, kind }` (sim/src/seam.rs:498; lib.rs:31; world.rs:32, :63, :1492; tests/world.rs:10, :534-535, :1666-1667),
    the form the room's records take, which stays: the room's enum is input.rs:52, with 39 lines in crates/room (input.rs
    22, room.rs 16, lib.rs 1) and one consumer of the records (room/tests/commands.rs:82). crates/sim is outside the
    lane (ADV-U4F2B-11).
  - **C-U4F2-SEC-1, -2, -4:** kept (blocks 4, 5, 8). C-U4F2-2's `skip_command` unit test sits in npt-client, which the
    lane opens "for C-U4F2-2's doc and ADV-U4FP-3 only" (ADV-U4F2B-6).
- **The four new numbers:**
  - **C-U4F2-5 = ADV-U4FR2-1** (r2 :87-97): its three route lines: (1) before -21's set, every -21 suite's honest clients
    out of buildings for 610 s at P0 and P1 under both options; (2) a sim-only test with a plant, on BRAWL-1's model; (3)
    the 21-row record reads "in a building: 0" and PM-0001-2 all passing on every -21 row. The brief has (1) and (2) at
    :65-67 and (3) at :55-56; "the choice of fix comes to me" kept (Q1). Not loosened. Two gaps in how it is to be met
    (ADV-U4F2B-3): the fix's reach into scripts already judged, and the sim-only model lacking option A's pattern.
  - **C-U4F2-6 = ADV-U4FR2-2** (r2 :110-113): restate p19 with the figures and correct Q15 (c)'s sentence; -20's set
    report names it. Kept (brief :68-69, section 5). Section 5 checked in block 4 below.
  - **C-U4F2-7 = ADV-U4FR2-4** (r2 :123-129): "not applied" for a never-firing suite, riding U4f2's lane. Kept
    (:70-72). The fix edits lab/src/contract.rs (:227), untouched since U4r's merge 71ee13d (`git log -- …contract.rs`;
    `git diff --stat 5b29296b 25d316e` on it: empty), so it triggers T-6's contract half (ADV-U4F2B-1).
  - **C-U4F2-8 = ADV-U4FMH-2** (merge head :57-64): the lab writes `params.cmd_timing` from `server.room.cmd_timing()`
    (room/src/room.rs:621; today scenario.rs:1903-1907 writes it from the setting), f6's assert (scenario.rs:3649) pins
    it, else the getter goes; §4.1's npt-room row either way. Kept word for word (:73-75).
- **No number:** F3 (r2 :114-122), F5 (a)-(c) (:130-142) into the record lines: kept (:76-77). F6 is -20's report's, not
  U4f2's: rightly absent.

## 3. The test share (DONE)
- 2.10's rows (u4f-brief.md:645-728): f15 is 11 cases, neg (U4f built 5: u4f-report.md:237), so U4f2's 6; f16 24 (not
  neg); f22 2 neg; f23 1 neg; f33 13 neg. Measured and bites: 6 + 24 + 2 + 1 + 13 = **46** each (no bites-only test in
  U4f2's rows: the 13 room and sim bites' unit proofs are U4f's, u4f-build.md:851). Negative: 6 + 2 + 1 + 13 = **22**.
  Equal to u4f-report.md:229-231 and the stop ruling :95-97.
- Sum with U4f as merged (u4f-report.md:232, :237: +82, +99, 46): 128 against 119, 145 against 134, 68 against 67. All
  met; the negatives by one, and by none if f4's 1 is set aside (it rests on U4e's e9: u4f-report.md:241-244). So U4f2's
  22 is a floor with no slack.
- **What the brief misses:** 2.10's minimums carry "+5 by T-6 if contract.rs is edited" (u4f-brief.md:658; section 6
  item 1 :1167; return list :1199, :1217). C-U4F2-7 edits it, so the bars become +124 and +139 and U4f2 adds T-6's five
  doctored cases (ADV-U4F2B-1). With them: 133 against 124, 150 against 139.
- **The base for U4f2's own deltas** is not named: U4f's rules count on 5b29296b's 552 and 632 (u4f-brief.md section 4
  rule 5, :876-880); U4f2's own are on 25d316e's 634 and 731 (u4f-report.md:232) (ADV-U4F2B-7).
- Tests beyond the share that the carries add (f28's two B halves, `skip_command`'s unit test, C-U4F2-1's parser case,
  C-U4F2-5's sim-only test, C-U4F2-7's test, Q4's case below): fine as extras; the report names each with its `--list`
  line.

## 4. Section 5: p19 and the corrected U4F-Q15 (c) sentence (DONE)
- **The figures, against the run** (r2.txt:122-139 and p19's own logs, read only): the summary line reads "late 6; ...
  buffer full 7" (r2.txt:125); lab.jsonl line 7 (`client_counts`, slot 0) reads `buffer_full_count` 7, `late_count` 6,
  `pump_input_limited_count` 25, `seq_outside_window_count` 48; room.jsonl's `lagswitch_cmds_count` reaches 8 on 5 ticks
  (max 8). The per-hold W drops 2, 2, 2, 0, 0 sum to the 6. Every figure in brief :93-96 holds, and equals my r2 read
  (advisor-u4f-r2.md:100-105).
- **The ruled sentence, quoted:** advisor-u4f-brief.md:293-295 ("p19's arithmetic holds (the Input bucket passes 8 of 12
  datagrams, pump.rs:20, :84; W drops the oldest 3 or 4), so an honest hold never puts more than 8 new commands into a
  tick") and u4f-brief.md:1622-1623 ("so the cap refuses nothing there"): both quoted exactly.
- **The correction** (brief :99-102): "under option A an honest 400 ms hold can reach the per-tick cap of 8, and the cap
  then refuses the excess under kind 10 (p19: 7 refused)", no bar or verdict moves (r2 :106-107), the attribution must not
  count those 7 as forged. Right, and no wider than the run shows.
- **Two gaps:** (1) the same premise stands in 5.3's p19 row (u4f-brief.md:1037: "the kind-10 refusals (0 expected)",
  "about 3 or 4", "about 4", "an honest hold never puts more than 8 new commands into a tick") and, executable, in
  c-u4f-r2.mjs:64's p19 `expect` ("dropped late by W (about 3 or 4), the input_limited datagrams (about 4), kind-10
  refusals 0"). Section 5 corrects the sentence but no block rewords the line, so the 21-row record from U4f2's merge
  commit, which is -21's 5.3 record (carry 86), would carry an expect known false and send p19 to me by the brief's own
  rule (:56-57) (ADV-U4F2B-5). (2) The brief says "p19 is unjudged and no -21 check reads refusals": checked, true:
  marks.json's -21 checks (:4805-5145) read `event.forged_kind.*`, the clamp, fast-clock and lag-switch series, never an
  honest client's refusals.

## 5. Section 6: the size (DONE)
- **His figure and the stop:** his (a) chose "about 1,500 lines, with its own size line and review" (advisor-u4f-stop.md:36-38;
  history.md:6899-6903); SIZE-CHECK's "more than twice the plan" (history.md:2049) makes the stop past 3,000. Brief :105-107
  reads my merge head check (:133-140) right: the brief may confirm 1,500, not raise it; above it, him before the build.
- **The table, against the code and the sources:**
  - Builder 3's five items (u4f-build.md:863-866: bots 250, lab 950-1,100, SERIES.md 5, c-u4f-r2.mjs 10, c-u4f-runs.mjs
    150, f28 40) sum to 1,405-1,555: the brief's 1,400-1,550 and its block-8 row's 165 hold.
  - C-U4F2-3, 60-90: 39 room lines name the enum (input.rs:52's `Refusal`), plus the records' struct and its one consumer
    (room/tests/commands.rs:82; `Room::refusals()` at room.rs:631 returns `&[(u8, u16, Refusal)]`): holds. The "51"
    counts the sim's 12, which stay (ADV-U4F2B-11).
  - C-U4F2-7, 15-50: the band is contract.rs:221-243, the header names the suite (scenario.rs:1902, `params.suite`): the
    fix itself holds. Missing beside it: T-6's five doctored cases (missing header param, missing class_audit, duplicate
    admit, a log npt-log refuses with exit 1, a usage error with exit 2: advisor-u4r-r1-full.md:122), about 60-150.
  - C-U4F2-8, 2-10: one line at scenario.rs:1903-1907 to read `server.room.cmd_timing()` (room.rs:621), f6's assert
    (scenario.rs:3649) unchanged: holds.
  - C-U4F2-5, 80-180: the fix is small if it is a Steady arm (bots/src/lib.rs:651-656), but BRAWL-1's sim run is built
    for 48 brawlers (`Behaviour::brawl`, :1250-1262) with patterns EveryTick, Skips, Bursts (:1187-1198) and no option-A
    pattern; the test that ADV-U4F2B-3 asks for takes a script parameter and a new pattern. I read 100-220.
  - C-U4F2-1, -SEC-1, -2, -4 with ADV-U4FP-3 and F5: hold. Add 1-3 for p19's expect (ADV-U4F2B-5) and 10-20 for Q4's f33
    case (below); RR-CB-2's docs about 5 if it rides (ADV-U4F2B-8).
- **My total:** about **1,700 to 2,250** (midpoint about 1,970), against the brief's 1,610-2,010. At U4f's overall 1.2
  (advisor-u4f-stop.md:104-105) about 2,050 to 2,700. Both under the 3,000 stop; the top scaled leaves a factor of about
  1.1. The lab ran 1.0 times in U4f and is most of U4f2, the room ran 2.0 times and is about 4 % of it, so 1.2 is the
  cautious reading.
- **Reading:** above about 1,500, so it goes to him before the build, as the brief says, with the stop past 3,000
  unchanged. The brief's reading needs two fixes for his message: the figures (above), and the riders named as such
  (ADV-U4F2B-9): C-U4F2-7 with T-6, C-U4F2-8 and ADV-U4FP-3, about 85-215 lines together, ride by my rulings because the
  lane opens their files; they are not what his (a) sent.
