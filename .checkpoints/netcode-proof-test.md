# Checkpoint: netcode-proof-test
- **Goal:** the netcode proof test (SPIKE_PLAN item 1) under its approved plan `.checkpoints/netcode-proof-test/plan.md`,
  rev 3.3 (the advisor and Mustafa, 2026-11-06; verdict advisor-verdict.md) · PR #40 (draft).
- **Branch:** `claude/netcode-proof-test-plan-hcky5g` (this cloud session's stand-in for `cs/netcode-proof-test`).
  **Base:** main f5030a3 merged in at a6acc8a (MAIN-61, 2026-11-19 15:4x UTC: #61, change-set 2's v2 code; one
  conflict, docs/STATUS.md: this branch's #40 row kept beside main's #61 row), which clears PR #40's second conflict;
  before it main b763ba3 at 0f12d9e (MAIN-60). MAIN-52 takes main in when it changes code, CI or rules, and once before #40 is marked ready.
- **Last update:** 2026-11-19 20:1x UTC: U4f merged (25d316e); r2 done and read; part-clean done; **-47 PASS**
  (committed); **the -20 set running** (from 20:07 UTC); then -22; card trimmed (card_trim_1119i.py).
- **The old card, whole and word for word:** history.md:2284-6789 (the card's line N is history.md's line 2287 + N).
  Every "card, block X" written before 2026-11-17 22:44 UTC points there: its Rules in force at :6216, Cuts at :5791,
  Next steps and PR #40's closing checklist at :6333 and :6350, Open findings at :6387, Key numbers at :6522.

## Rules in force (his words word for word in history.md: read them there, never retype them)
- **TEST-FIRST, 2026-11-17 20:20 Amsterdam (history.md:2234-2282):** only plan rev 3.3's work until the test closes;
  findings outside it go on "After the close" below; KIT-FREEZE-2 stands (#53 and #54 used up); the one way out: a
  pass mark that cannot be built or judged, or lost work, told to him in three lines. Decisions at most one message a
  day, each with my lean, "stopped on you" in its first line. A choice inside the plan, undoable, changing no pass mark,
  costing no money and touching nothing on main goes by my lean or the advisor's ruling: recorded under "Choices taken
  without him" and named in that day's message. Still his: pass marks, the judge, marks.json, money, main, the plan's
  stops and run days. This card at most 200 lines; a workflow only for the rules' review rounds and truly parallel
  work; a brief says what is built, how it is tested, what is open, and points to the rest. Ultracode off; effort max.
- **Kept exactly (TEST-FIRST):** every pass mark and its number; the judge and marks.json; the planted-bug checks; a
  reviewer's read on every marks.json or judge change; at most two review rounds (a third only for a Critical or High);
  main taken in when it changes code, CI or rules; every PR the plan names stops for his go (he asks his Advisor first).
- **2026-11-16 21:50 (:2177-2187):** commands in the simple forms that need no approval, one per call; a call that will
  ask is named in the message before it is sent.
- **2026-11-15 16:35 (:2131-2146):** C62H-KEEP (a); C62H-BOUND (a): its wording frozen, never reworded to fit a run.
- **2026-11-15 13:35 (:2125, :2127):** a new bar, or a change to one, comes to him before it is built; every
  marks.json or judge change gets a reviewer's read before its unit closes, named in the report in its own line.
- **KEPT-LOGS (2026-11-15 08:20; .claude/CHECKPOINTS.md §11) and PACK-READ (a) (2026-11-17 10:43, :2204-2227):** only a
  closed set's folder, by brief 8.1's procedure (c62h-bound-brief.md:485-501), never during a build or a set; never the
  P0 folders (93509dfd7fd1, 051995db9cea), fc411116c1f1, fd4612050cd6 or the c-u2 folders without his word;
  u4r-P3-0f066338dd8d is the last folder his yes covers.
- **2026-11-14 (:2048-2051, the reading :2067-2075):** SIZE-CHECK: an M unit's plan is about 3,000 added lines; past
  6,000 the build stops and he gets three lines (what grew, why, the cheaper option). MAIN-52 (:2084-2085). /compact:
  ask at 35-40 % after the card is refreshed, then carry on.
- **2026-11-13 CHECK-IN, 2026-11-12 SPEND-LIMIT (a), 2026-11-11 USAGE-SAVINGS (a), 2026-11-10 and 2026-11-09:** as the
  old card's Rules in force gives them (:6216-6316): a 2-hour check-in while agents run; restore and resume after each
  cut, each cut's notice and times recorded; read every Medium or higher and all that touches the judge, the marks or a
  pass bar in full; the 9.5 floor (PM-0001-2) does not move; a Low never triggers a push; his words move by script and
  line ranges; RESUME-CARD, WIP-CLEAN, TCP-FREEZE, NETEM-BAR (a), SQUASH-40, PROMOTE; one shared target; LEFTOVER-DELETE.
- **Standing:** every agent brief carries, word for word: "Write files with the Write and Edit tools only (no sed, no
  shell redirect, no python writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no
  Edit tool rewrites its notes file whole with Write." Never switch the main folder (worktrees under .gate/); 3 agents at
  most; Opus 5.5 at xhigh for netcode; the advisor is a general-purpose agent reading `git show
  origin/main:.claude/agents/advisor.md`; commits end with the Co-Authored-By and Claude-Session lines; the five U4
  briefs go to the advisor before their builds; carries word for word from u4s-carries.md, met by number in reports.
- **DEP-HOLD (main's CHECKPOINTS.md §3, his of 2026-11-18; on this branch from MAIN-60's merge):** a save that changes a
  `Cargo.toml`, `Cargo.lock` or `package.json` against the remote branch (unless equal to main's) is held until
  security-auditor's answer and `.checkpoints/netcode-proof-test/dependency-check.md` exist; never push by hand.

## Now: U4f (clock and commands; u4-build-plan.md §5 :805-816; size M)
- **Moved to history.md:7060-7088** (card_trim_1119i.py, 20:1x UTC): U4f's filled brief (u4f-brief.md, sha256
  135325a0…, 2,040 lines, `<base>` 5b29296b), the advisor's build-1 ruling (advisor-u4f-build1.md), builder 3's stop
  line and MAIN-61's done line (a6acc8a, pushed 631edf3, CI green; advisor-main-61-head.md).
- **LEAD-BOUND ANSWERED: (a) 400 ms, frozen** (Mustafa, 2026-11-18 11:31 Amsterdam by his label; reached the chat
  11:10:41 UTC; word for word at history.md's end, "## Decision: Mustafa, 2026-11-18 11:31 Amsterdam", by
  record_lead_bound.py): the entry u4f-lead-bound.md equal to 9ad2effb's (sha256 57c588ec43bae7b7…), its fenced value's
  sha256 4717cd895d9834154d3179f397859096b5a30f7ec290ea03f30148b912e53a2b; A1's movement reading: yes, and **U4f's
  report names it in a line of its own with who reviewed it** (the advisor's A1; read by the advisor and the
  test-engineer); no marks.json, judge file or existing SERIES.md row may change: if the build needs one, stop and ask
  him; "Build U4f"; his size stop past 6,000 added lines stays.
- **SIZE-U4F ANSWERED: (a)** (Mustafa, 2026-11-19 12:00 Amsterdam by his label; reached the chat 13:36:25 UTC; word for
  word at history.md:6896-6903 by record_size_u4f.py): U4f closes with what is built; the three finished cheat tests
  stay in it and are reviewed with it; the forged-command test, its six missing cheats and all of -21's runs become a
  small unit of their own next, **U4f2**, with its own size line and review; no pass mark, judge rule, marks.json text
  or run count changes. U4f2 holds advisor-u4f-stop.md:80-89's list (f28's two B halves with it: R3 (ii)); its short
  brief goes to the advisor before its build; its size line by SIZE-CHECK (its plan entry states it; the card writes it
  with its source before its first snapshot). -21 is judged only after U4f2.
- **-20 and -22 DONE, all three sets run** (records at history.md:7124-7146, card_trim_1119k.py; u4f-report.md items
  5-6): -20 at 350677f (1840e68): PM-0001-2 PASS; PM-0001-20 A PASS, B FAIL "0 presses lost" 3/1/2; overflow 0;
  high-water 1,117 (margin 1.43). -22 at 7192607 (2dae645): PM-0001-22 PASS, bite shown, load report as designed.
- **The advisor's -20 ruling** (advisor-u4f-20.md, 170 lines, read in full): the count is honest, the cause is the
  stand-in: **ADV-U4F20-1 High**, npt-sim's one-edge rule (sim/src/world.rs:1340-1346, U4d's) loses a press that ADR
  0001 part 5 keeps (:226, :234-237) once U4f's kind 5 (room/src/input.rs:285-300) accepts a 2-edge command after late
  drops; fix: every edge taken through its checks, the room's count from the same rule, two tests; **-20's L1 set
  re-runs on the fixed commit** (that verdict is -20's record; 350677f's folder the defect's). **-22 stands**; 1,600
  stands (no honest overflow resync); margin 1.43 named for U4g. **ADV-U4F20-2 Medium:** marks.json's reorder report
  row (:4786-4790; -58's :4263-4267) reads a header param L1 never writes: his, "After the close". ADV-U4F20-3 Low: in
  the report. The OQ-0001-6 reading: the B FAIL is not evidence against (b); -20 separates the options by buffer depth
  only. The set reports for -20 and -22 are in u4f-report.md ("After the merge", items 5-6).
- **U4f-F1** (its own step before U4f2: U4FF1-Q3 ruled so): plan u4f-f1-plan.md rev 1.1 (size S, about 60-150, stop
  past 300); **the advisor's plan review APPROVE WITH CONDITIONS 1-10** (advisor-u4f-f1-plan.md, read in full: the
  four counters in today's order, one shot per command, the fn's contract, T1/T2 with plants, the re-run compared tick
  by tick with 350677f's logs, security-auditor joins, the first command to U4f2). After the round: the merge, -20's
  re-run, then condition 10 (U4f2's brief rebased on F1's merge, C-U4F2-9 = ADV-U4FF1-3 with NC-F1-2's reach).


## Choices taken without him (TEST-FIRST rule 2: inside the plan, undoable, no pass mark, no money, nothing on main)
- **2026-11-20 (to name in the next day's message, 2026-11-21 Amsterdam; advisor-u4f-f1-r1.md:207-221, ADV-U4FF1P-4):**
  U4f-F1's one review round (four seats, all APPROVE WITH FINDINGS) and its small fix pass (doc wording, one new test,
  two test touches), checked by the advisor, no second round; after F1, "0 presses lost" in -20 and -58 holds by the
  press counters' design, as "0 presses doubled" did (the room counts with the rule the sim runs; T1/T2 and their
  plants prove the take; the runs show presses equal to the scripted 432,000 per option-B run); a judged check of that
  count is a marks.json change, his, on "After the close"; the first-command gap F1 widened (a pool of grenades per
  rejoin, an honest restart's phantom presses) goes to U6 as C-U6-19, closed before -19 runs. Why: inside the plan,
  undoable, no pass mark, judge or marks.json change.
- The choices of 2026-11-19 named in msg-2026-11-20.md: history.md:7177-7193 (card_trim_1120c.py).

## Next steps (plan rev 3.3; units ahead)
1. U4f (above), then U4f2 (-21's forged suite and runs; its short brief to the advisor; its size line on the card before
   its first snapshot). 2. U4g (prediction, interpolation and events; M) and U4h (lag compensation and aim; M), each with a
   short brief (TEST-FIRST rule 3) to the advisor before its build; U4g's carry holds ADV-CBR-1 and C62HB-Q4 (b)'s two
   -58 lost-tick checks. 3. U4's other row groups as u4-build-plan.md places them, then U4's report (it folds U4r's);
   JDG-ONLY-1 at npt-judge's next change, as its own change-set. 4. U5 (S), U6 (M), U7 (M), U8 (S): plan.md §12.1
   (:1244-1255). 5. Stop B (before any VM exists), the run days R1-R7 (plan.md:818-824), Stop C (the close); R8 later.
6. PR #40's closing checklist: the old card's Next steps (history.md:6350-6371): squash merge with the four cites
   re-pointed, no .checkpoints/ in the final tree, no wip patches, PROMOTE's one-pager, the readings to confirm, U4j's
   open findings.

## Open findings (inside the plan; each with its taker)
- **U4f's brief, its 9.2:** taken by U4f: RR-CB-2, TE-B1-R2-1 with RR2-B1-1, RR-B1-7 (1); left with their takers:
  C62H-BOUND's judge Lows (JDG-ONLY-1), TE-CBH-5 = NA-CBB-1 (c-u4r-runs.mjs's next change), U3C-HELD-1's eight
  (npt-net's next change: u3c-held-1/build.md:116-122), CHECK-NFF-1 (check.mjs's next owner), ADV-CBR-1 (U4g).
- **For the readings PR before R5 (plan :970-983):** A1's reading of ADR 0001:1611's "moves" (the advisor's U4f ruling,
  :78-85); the items the old card lists (history.md:6458-6461: Q15 (a), WS-RECONNECT-BAR (b), the PM-0001-31 pointer);
  from U4f-F1's round (advisor-u4f-f1-r1.md:153-169): NC-F1-3 (part 5's per-command order, one shot per command,
  collapsed throws), SA-F1-3's v2 rule (every press action has server-side rate state, or takes at most one edge per
  command), and C-U6-19's session-start sentence.
- **Carried past U4 (the old card's Open findings, history.md:6430-6464):** the advisor's U2 carries C1-C5 and its
  Note; C-U7-1; the U1 carries (C4, C6, A3, the results ledger, Q13, Q23); C6 widened (U6, before R3); JITTER-ORDER
  (parked: to him before R3 only if C6 shows the model's reordering moves a WebTransport result).
- **For PR #40's ready head** (the advisor's MAIN-60 head check): CI's green is 631edf3's (MAIN-61); the head marked ready needs
  its own run. STATUS.md's #40 row still reads "main 53272a1 merged in": PR #40's closing docs pass (ADV-M60-7).

## After the close (TEST-FIRST rule 1's list: found on the way, outside the plan)
- ADV-U4F20-2 (Medium, his: a marks.json change): PM-0001-20's "measured reorder share" report row
  (marks.json:4786-4790) and -58's (:4263-4267) read `header.params.reorder_permille`, which L1 scenario runs never write
  (the share is the lab's `reorder` event since D3-1); reports state the event's figures by hand until he rules.
  Beside it (NC-F1-1; advisor-u4f-f1-r1.md:220-221, his): a judged "presses seen equal the scripted count" check for -20
  and -58, since "0 presses lost" holds by design after U4f-F1.
- NS-TRAILER and NS-FIRST: the night-shift lines' attribution option and status refresh, for the kit's next change
  after the freeze (his RECS-1043 (b), history.md:2217).
- For STATUS (the advisor's U4f L3): the hold-time cap's -21 counter (U4F-Q14 (b): a follow-up for the first unit that
  runs ws sessions) and SA-16's variant (a client that acknowledges only forced full states cycles resyncs and is never
  kicked: U6's limits).
- The kit's paper costs I named to him on 2026-11-17 (my message after the card cut): for a decision after the close.

## Key numbers
- U4f's `<base>` 5b29296b counts (D1): 0ad9661's E6 (check.mjs's 11 steps pass; tests 552 measured and 632 bites build,
  0 failed, 0 skipped; marks 81; self 78 of 78; deps 137 crate files; wasm 27000 ticks, 0 mismatching) but refs 1463
  tracked files and redact 307 items (the light run at d4654df; 0ad9661: 1382 and 306). spikes/ outside results/ is
  unchanged since 0ad9661.
- Disk (D2, 2026-11-18 11:35 UTC, after the pack and the part-cleans): 12,154 MiB free (the floor 4,096); still kept:
  u4r-P3-d66e23cd78f2 298 MiB, u4r-P3-fd4612050cd6 291, u4r-P2-8ee453c22882 741, u4r-P2-fc411116c1f1 623,
  u4r-P0-051995db9cea 3,249, u4r-P0-93509dfd7fd1 3,393, the c-u2 folders 998 each; the main folder's target 776. U4f's
  sets and pre-runs need about 3,727 MiB.
- The packs so far (history.md, the old card's Key numbers :6523-6554): u4r-precheck-30, u4r-P2-ab10f058a828,
  u4r-B1-64f3307d5c7f, u4r-P3-0f066338dd8d (2026-11-18: 375 MiB back); each gave back about 72-73 % of its folder.
- Mustafa's count at 9024840 (2026-11-17 19:04 UTC, against main): 72,678 Rust lines, 7,688 TypeScript and JavaScript,
  608 Rust tests; 177,993 lines of working papers in 934 files.

## Running work
- **U4f-F1 BUILT** (00:0x UTC; 37d87a10…, 205 of 300): `edges_taken` seam.rs:78; T1 commands.rs:618; T2
  sim/tests/world.rs:1813-1920; plants by name; check.mjs x2 all 11 pass, 641 / 738 (f1-b4-check1/2.log, read).
- **U4f-F1's round CLOSED** (four seats APPROVE WITH FINDINGS: u4f-f1-review-<seat>.md; **the advisor's ruling
  APPROVE WITH CONDITIONS FP-1 to FP-5**, advisor-u4f-f1-r1.md, 266 lines, read in full; no round 2, its check of the
  pass instead): TE-F1-1 = NC-F1-1 keep the code, bind the wording ("0 presses lost" holds by design, as "doubled" did;
  the room never read the sim); NC-F1-2 and SA-F1-1 carried widened to **U6 as C-U6-19** (gated before -19, PM-0004-61
  online, any throwing or rejoining hostile client); U4f2 gets C-U4F2-9 (a record line) and -10; **U4F2-SIZE unchanged**.
  FP-5's carries filed (u4s-carries.md U4f2, U4g, U4h, U5; plan.md:1253 C-U6-19). **The fix pass (FP-1 to FP-4) by the
  builder**: stopped at 00:49 by the runner's 4 GB floor (3,850 MiB free; FP-1 done, FP-2 to FP-4 written, not run);
  **the shared target part-cleaned 00:53** (P27's six `cargo clean --target-dir` commands run directly under the lane
  lock, main-run.sh's own floor refusing them: dev 7.3 GiB, release 258.8 MiB, doc 12.8 MiB, wasm32 134.5 / 37.3 / 8.7
  MiB; guards hold; 10,576 MiB free). **The pass DONE** (01:44 UTC): FP-1 to FP-4 in, plants by name, check.mjs 1b and
  2 all 11 pass, 642 / 739 (+8); 239 of 300; snapshot d0a66a68… (6 files); departures: FP-4 through the World, seam.rs
  restored to its post-FP-1 hash 901bb546…, a rustfmt layout. **Part-clean 2** (01:45: 3,865 -> 10,574 MiB). **The
  advisor's check of the pass running** (advisor-u4f-f1-pass.md); then the merge. The saver bii6uxhfc.
- **U4f2's brief: the advisor's check APPROVE WITH CONDITIONS** (advisor-u4f2-brief.md, 290 lines, read in full:
  Mediums ADV-U4F2B-1 T-6 rides, -2 and -3 Q1's fix confined to Steady's open-loop move with an option-A sim pattern
  whose plant bites, -4 C-U4F2-1 word for word, -5 p19's `expect`, -7 the builder's numbers, -9 the size; Lows -6, -8,
  -10, -11; rulings Q1 (b) else (c) never (a), Q2 to him, Q3 (a), Q4 confirmed +1 f33 case, Q5 no change). **Rev 1.1
  written** (u4f2-brief.md, 222 lines: conditions 1-8 met; no new check needed unless one cannot be met, :244-245).
  **U4f2's size line** (SIZE-CHECK, written before its first snapshot): his (a)'s about 1,500 added lines
  (history.md:6896-6903, on advisor-u4f-stop.md:36-38), the stop past 3,000 (history.md:2048-2051;
  advisor-u4f-merge-head.md:133-142); the brief's estimate about 1,700-2,250 (up to about 2,700 at 1.2;
  advisor-u4f2-brief.md:149-156), so **U4F2-SIZE goes to him** in the next day's message (2026-11-20 Amsterdam, after
  23:00 UTC; the 11-19 one was SIZE-U4F at 00:2x), first line "stopped on you"; lean: build it whole; his cheaper
  option: leave the four riders (C-U4F2-7 with T-6, C-U4F2-8, ADV-U4FP-3: about 80-215) to a later unit. The build
  waits for his answer.

## Cuts by the spend limit (SPEND-LIMIT (a): each cut's notice, word for word, its cut and its resume time, UTC)
- **2026-11-20 about 01:01 to 01:05 UTC (a container restart: the F1 pass's first check.mjs and the saver):** "The
  container was restarted. The following background tasks were running and are now stopped: - Restart the background
  checkpoint saver (task b1d4cn50i) - Run check.mjs, first run of the fix pass (task bp98t6o2k) Re-create them if still
  needed." (up since 01:01:16 by /proc/uptime). Checked at 01:04: FP-2 to FP-4 run with their plants restored (seam.rs
  901bb546…4c90, the post-FP-1 hash; world.rs 4a4e138c…), no plant left, no process; f1-b5-check1.log cut at 00:56;
  7,528 MiB free. The saver restarted; the builder resumed by SendMessage to re-run check.mjs twice.
- Cuts until 2026-11-17: history.md from :5791 on (the old card's block, the latest first) and the 19:46 one; the two
  of 2026-11-18 (builders 2 and 3) word for word at history.md:6921-6932 (card_trim_1119e.py); 2026-11-19's 16:00:10
  one (builder 4) at history.md:7124-7146 (card_trim_1119k.py); the 20:53:34-22:40:41 one (the advisor's -20 ruling
  and the session) and the 11-18/11-19 choices at :7148-7160 (1119l); the 23:18 restart at :7169-7175 (1120b).
