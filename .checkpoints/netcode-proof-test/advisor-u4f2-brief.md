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
  rule 5, :848-852); U4f2's own are on 25d316e's 634 and 731 (u4f-report.md:232) (ADV-U4F2B-7).
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
  (ADV-U4F2B-9): C-U4F2-7 with T-6, C-U4F2-8 and ADV-U4FP-3, about 80-215 lines together, ride by my rulings because the
  lane opens their files; they are not what his (a) sent.

## 6. The lane and the rules that stand (DONE)
- **Bars:** "Nothing in marks.json, the judge or an existing SERIES.md row changes" with his (a)'s words quoted exactly
  (brief :82-83 against history.md:6903), and a build that needs one stops for him: right. U4f's line 22
  (advisor-u4f-build1.md:242-246) also names marks/README.md; it stands through section 4's reuse. C-U4F2-7 changes the
  lab's contract checker, not the judge: the contract is the lab's pre-run check (contract.rs:1-5: "the judge's own half
  ... is `npt-judge --only`"), no verdict reads it (r2 :126), and the clock suite carries no PM-0001-2 precondition, so
  "not applied" there loosens no bar. PM-0001-2's own shot check stays in marks.json (:287-300) and the judge.
- **LEAD-BOUND** 400 ms frozen (history.md:6791-6806: "never changed to fit a run's result"): kept. **The event cap**
  1,600 (advisor-u4f-stop.md:14; u4-build-plan.md:362-366): kept; "unless -20's set read moves it first" is R2's own
  route (an honest overflow resync comes to me, the value re-set by the same rule), not a new one.
- **The lane** (brief :80-81): npt-bots, npt-lab, ops/runs, SERIES.md new rows, npt-room for C-U4F2-3 and -8, npt-client
  for C-U4F2-2's doc and ADV-U4FP-3. Two gaps: C-U4F2-2's `skip_command` unit test needs npt-client too (ADV-U4F2B-6);
  crates/sim is rightly shut, but the "51 lines" pointer reaches into it (ADV-U4F2B-11). Forgers 8-11 sit in npt-bots by
  2.6 (u4f-brief.md:502-504: decoded and re-encoded through npt-proto), so no client hook is planned; if one is needed,
  it is a lane stop (section 1 item 5).
- **The builder's numbers:** section 4 reuses U4f's rules, but section 1 (base, runner, stops) is not in the brief's read
  list (:11-19), and the stops as written there are U4f's: "your added lines past 6,000" (u4f-brief.md:323), counts as
  deltas on 5b29296b's (:848-852, :1166-1168), the test step against 5b29296b's 1.5 times or line 22's 1,207 s. U4f2's
  are: past 3,000 at a block end (his SIZE-CHECK on his (a)'s figure), deltas on 25d316e's 634 and 731, and a named
  test-step line (ADV-U4F2B-7).
- **Agents and writes:** one builder, Opus 5.5 at xhigh (netcode: CLAUDE.md Models), at most three agents, the Write/Edit
  sentence word for word (brief :86-88): right.

## 7. C-U4F2-5's fix: what it reaches (DONE; feeds Q1, Q5)
- `steady`'s walk is `MOVES[((k / TURN_EVERY + phase) % len)]` with `k = self.given` (bots/src/lib.rs:638, :651), and the
  `_ => walk` arm (:655) serves Kind::Steady, Kind::NoFire (the -47 clock suite: `never_fires`, :522-524; scenario.rs:1213-1214),
  Kind::Presses and Kind::Clamp (whose move is [0, 0], :664-668). Kind::Brawl leashes that same MOVES walk (:653).
- So a fix made in MOVES, TURN_EVERY or the shared arm changes -47's script (judged PASS on 25d316e's code, results
  committed), and through MOVES the brawl of -20 (running now at 350677f, equal to 25d316e's spike outside results/:
  u4f-report.md:589-591) and -22 (next), whose set may also re-run by R2's cap rule (u4-build-plan.md:362-366). Their
  runs would no longer re-run the same from U4f2's tree.
- The plan's twin rule: "every client of the forged and clamp suites runs an open-loop seeded script (commands a function
  of seed, slot and tick only), so nothing it decodes feeds back into what it sends" (u4-build-plan.md:411-412); 2.6's
  forged row and 2.12 say the same (u4f-brief.md:533, :774-776). A leash reads the decoded position (:608-613): option
  (a) breaks that rule for the forged and clamp suites, and -21's honest clients run one script (U4F-Q11 (ii) (a)).
- BRAWL-1's sim-only model feeds the World EveryTick, Skips (a skipped tick re-runs the last command, option B's take)
  and Bursts (:1187-1198), for 48 brawlers (:1250-1262). F1 showed only under option A at P1 (p4, p13, p19), never under
  option B or at P0 (advisor-u4f-r2.md:65-71), so a test on today's patterns may pass with today's walk: TE-B1-1's lesson
  (brawl1-review-r1-full.md:30-32) is that a plant must bite.

## Rulings on U4F2-Q1 to Q5 (DONE)
- **U4F2-Q1, C-U4F2-5's fix: (b), or (c) if (b) cannot be shown; never (a).** (a) is out for every -21 suite: a leash feeds
  the decoded position back into the commands, against u4-build-plan.md:411-412 and u4f-brief.md:533, :774-776, which the
  twin figure rests on; a closed loop would need a plan change brought to me first, and none is asked. The brief's
  reason for (b) ("runs whose decoded positions differ by the skipped command") is replaced by that rule. Under (b) or (c):
  1. only Kind::Steady's `move_xy` changes: its fire, aim and presses stay (the shot band, carry 86), and MOVES, TURN_EVERY
     and the Brawl, NoFire, Presses, Clamp and Lane arms stay as at 25d316e (the report shows it from `git diff 25d316e
     -- crates/bots/src/lib.rs` outside the test module);
  2. the content stays a function of seed, slot and step (2.12's rule), so the twin's skip leaves later commands equal
     (f16 shows it);
  3. the sim-only test, on BRAWL-1's model with the script as its parameter: option B's patterns as today plus an
     option-A pattern at P1 (each slot's commands run in the tick they arrive: several in one tick, none in another, from
     P1's one-way 25 ms ± 5 ms and the controller's lead), all 48 slots on `steady` (the 11 forgers' base walk
     included, since PM-0001-2's in-view check reads every slot); 610 s, seeds 1 to 3, in release (`--ignored`), with a
     short copy in the debug test step; **the plant is today's walk, and it must read in a building above 0 in at
     least one pattern and seed**. If no pattern makes today's walk enter, the test cannot show the fix: the build stops
     on it and comes to me with the figures.
- **U4F2-Q2, the size: confirmed above about 1,500, so it goes to him before the build; the stop past 3,000 stays.** The
  figure is corrected to about 1,700 to 2,250 (up to about 2,700 at U4f's 1.2), and the message names the riders apart
  (condition 6). The card writes the size line with its source before the first snapshot (advisor-u4f-stop.md:86-88).
- **U4F2-Q3, C-U4F2-7's form: (a).** Keyed on the header's `params.suite` = `clock` (scenario.rs:1902; the one suite on
  `never_fires`, :1213-1214), printing "not applied (suite clock: its clients never fire)". The band stays applied to
  every other 48-client run, the forged suite's included (carry 86 asks it of -21's option-A suites). The test: a
  48-client clock record with shots 0 reads "not applied" and no failure; a 48-client record of another suite with shots
  0 still fails (e). The plant (the band applied to `clock`) fails the first by name. T-6's contract half rides in the
  same edit (condition 1).
- **U4F2-Q4, honest kind-10 refusals and the attribution: confirmed.** `unrejected_count` counts forged items failing
  their rule (2.4 item 6, u4f-brief.md:440-456; SERIES.md:159), the items being the forger's own (slot, seq) from
  `take_forged` (bots/src/lib.rs:321-327), so an honest refusal never enters it; and the forged suite runs at P0 with no
  honest hold (2.6's row, u4f-brief.md:533). One case guards the forger's own slot, where kind 10's forger makes its own
  hold (2.6's kind-10 row, u4f-brief.md:526: "after holding 8 steps"): in f33, on hand-made records, a kind-10 refusal record naming a
  (slot, seq) the forger did not forge, on the forger's slot and on an honest one, changes neither `sent_count` nor
  `unrejected_count`. +1 test, a negative case beyond the 13 (f33, not f16: hand-made records, line 15's rule).
- **U4F2-Q5, the clock suite's open walk: no change in U4f2.** -47 is judged on 25d316e's code and has no in-view
  precondition (advisor-u4f-r2.md:81-82; u4f-report.md:584-588); U4f2's report names it beside C-U4F2-5's fix. Q1's
  point 1 keeps NoFire's commands as they are, so the two stay apart.

## Escalation check (item 8)
Nothing in the brief or in these rulings touches a pass mark, the judge, marks.json, an existing SERIES.md row, money or
main. C-U4F2-7 edits the lab's contract checker, which no verdict reads, for a suite with no PM-0001-2 precondition. The
size is his figure, so it goes to him as U4F2-SIZE under TEST-FIRST rule 2 ("the stops ... your plan gives me"); it is
not an ESCALATE of a bar.

## Verdict: APPROVE WITH CONDITIONS
The scope is whole against all four lists, the existing carries are kept by number, the share's arithmetic holds,
section 5's figures equal the run, and the lane and the frozen values stand. The conditions below are text fixes for
rev 1.1, made by the main session before launch; no new check of rev 1.1 is needed unless a condition cannot be met as
written. The build waits for his U4F2-SIZE answer.

| Id | Sev | Where | What goes wrong | The fix |
|---|---|---|---|---|
| ADV-U4F2B-1 | Medium | u4f2-brief.md:45-47, :70-72, :110-126 | C-U4F2-7 edits lab/src/contract.rs, the first edit since U4r (71ee13d), which triggers T-6's contract half: five doctored cases and 2.10's "+5 if contract.rs is edited" (advisor-u4r-r1-full.md:122; u4f-brief.md:658, :1167, :1309, :1958-1960). Not carried, not counted, not sized. | condition 1 |
| ADV-U4F2B-2 | Medium | u4f2-brief.md:146-150 | Q1's fallback (a), a leash on `steady`, feeds the decoded position back, against the plan's twin rule for the forged and clamp suites (u4-build-plan.md:411-412; u4f-brief.md:533, :774-776); the reason given for (b) is not the plan's. | Q1 as ruled (condition 2) |
| ADV-U4F2B-3 | Medium | u4f2-brief.md:37-38, :49-50, :65-67 | C-U4F2-5's fix is not confined: MOVES and the `_ => walk` arm (bots/src/lib.rs:651-656) also serve NoFire (-47, judged), Presses, Clamp and the brawl's walk (-20 running, -22 next). And BRAWL-1's sim-only patterns (:1187-1198) lack option A's, where F1 alone showed, so the test may pass with today's walk. | Q1's points 1-3 (condition 2) |
| ADV-U4F2B-4 | Medium | u4f2-brief.md:35-36 | C-U4F2-1's proof restated without "of its own kind" and without its setting ("on the steady script at P0 into a `Room<World>`": advisor-u4f-r1.md:70-72): looser than the carry. | condition 3 |
| ADV-U4F2B-5 | Medium | u4f2-brief.md:40-41, :97-102 | The corrected premise lives on in c-u4f-r2.mjs:64's p19 `expect` ("about 3 or 4", "about 4", "kind-10 refusals 0") and 5.3's row (u4f-brief.md:1037); no block rewords it, so -21's 5.3 record would carry an expect known false and route p19 to me by the brief's own rule (:56-57). | condition 4 |
| ADV-U4F2B-6 | Low | u4f2-brief.md:80-81 | npt-client opens "for C-U4F2-2's doc and ADV-U4FP-3 only", but C-U4F2-2 includes `skip_command`'s own unit test (advisor-u4f-r1.md:76-78), which the size table puts in client (:119). | condition 5 |
| ADV-U4F2B-7 | Medium | u4f2-brief.md:11-19, :80-90 | The builder's numbers are U4f's: section 1 is not in the read list, and its stop reads "past 6,000" (u4f-brief.md:323), counts are deltas on 5b29296b's (:848-852, :1166-1168), the test step's line unnamed. His size stop would act at the wrong figure. | condition 5 |
| ADV-U4F2B-8 | Low | u4f2-brief.md:59-77 | RR-CB-2's docs and RR-B1-7 (1), left by U4f "for their next taker" (u4f-build.md:557-559), sit in scenario.rs and logs.rs, which U4f2 opens; the brief names neither. | condition 7 |
| ADV-U4F2B-9 | Medium | u4f2-brief.md:126-131, :165-166 | The estimate misses T-6's cases and the wider sim-only test (my figure about 1,700-2,250; up to about 2,700 at U4f's 1.2), and "the carries since ... are review fixes his (a) sends with the unit" is not so for C-U4F2-7, -8 and ADV-U4FP-3, which ride by my rulings. The message to him would frame his own decision too widely. | condition 6 |
| ADV-U4F2B-10 | Low | u4f2-brief.md:39 | ADV-U4FP-3 also names `plan()`'s short doc line (advisor-u4f-pass.md:95-96); block 7 names :360 only. | condition 7 |
| ADV-U4F2B-11 | Low | u4f2-brief.md:29-30, :120 | "51 lines name `Refusal`" counts 12 in crates/sim naming the sim's own struct (seam.rs:498), which stays and is outside the lane; the room's enum has 39 (input.rs 22, room.rs 16, lib.rs 1). | condition 7 |

**Rulings:** Q1 (b), else (c), never (a); Steady's move only, open loop, an option-A sim-only pattern whose plant bites,
else stop. Q2 confirmed above 1,500, to him before the build, stop past 3,000 unchanged, figures corrected. Q3 (a),
keyed on `params.suite` = `clock`, with both cases and T-6 riding. Q4 confirmed, with one f33 case on the forger's own
slot. Q5 no change in U4f2, kept apart by Q1's point 1.

**Conditions (rev 1.1, before launch):**
1. T-6's contract half rides with C-U4F2-7: section 3 names it with its source (advisor-u4r-r1-full.md:122; u4f-brief.md
   P34 :1958-1960, 9.2 :1309); section 2's share reads +51 measured and +51 bites (T-6's five beside the 46; the 22
   negatives unchanged), the sum against +124 and +139 (2.10's +5) and 67; the size table gains its row (about 60-150).
2. Section 9 and section 3's C-U4F2-5 line carry U4F2-Q1 as ruled above, points 1 to 3 whole, with its stop.
3. Block 6 states C-U4F2-1's proof word for word (advisor-u4f-r1.md:70-72), or names it by number with no restatement.
4. Block 8 rewords p19's `expect` in c-u4f-r2.mjs:64 to section 5's reading (unjudged, reported: W's late drops, the
   bucket's limited datagrams, the cap's kind-10 refusals under option A, the per-tick max at most 8; no fixed counts),
   and section 5 names u4f-brief.md:1037 beside :1620-1624 as superseded.
5. Section 4 gains: the builder's start and stops are u4f-brief.md section 1's (added to section 0's read list), with
   "past 6,000" read as past 3,000 added lines at a block end (line 5's procedure: build nothing more, end checks,
   return), counts as deltas on 25d316e's 634 measured and 731 bites (u4f-report.md:232), and the test step's stop line
   named (line 22's 1,207 s stands); and the npt-client lane reads "for C-U4F2-2 (`skip_command`'s unit test,
   `twin_left_out`'s doc) and ADV-U4FP-3".
6. Section 6 and "What goes to Mustafa": the figure about 1,700 to 2,250 (about 2,700 at U4f's 1.2), the stop past 3,000
   unchanged, the riders (C-U4F2-7 with T-6, C-U4F2-8, ADV-U4FP-3: about 80-215 lines) named as my rulings riding the
   lane, with leaving them for the next unit that opens those files as his cheaper option; first line "stopped on you".
7. The Lows in the same rev: C-U4F2-3's pointer reads "39 lines in crates/room; the sim's struct `Refusal` stays"; block
   7 names `plan()`'s doc line with :360; section 3 names RR-CB-2's docs and RR-B1-7 (1) as taken or left by line 15's
   rule (advisor-u4f-build1.md:215-220).
8. Q3's and Q4's rulings go into blocks 7 and 6 as written above.

**What goes to Mustafa:** U4F2-SIZE only: U4f2's estimate is about 1,700 to 2,250 added lines (up to about 2,700 at
U4f's overrun) against his 1,500; the stop past 3,000 stays; lean: build it whole; the cheaper option: leave the three
riders (about 80-215 lines) to a later unit. Nothing on a pass mark, the judge, marks.json, money or main.
