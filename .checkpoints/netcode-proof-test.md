# Checkpoint: netcode-proof-test
- **Goal:** the netcode proof test (SPIKE_PLAN item 1) under its approved plan `.checkpoints/netcode-proof-test/plan.md`,
  rev 3.3 (the advisor and Mustafa, 2026-11-06; verdict advisor-verdict.md) · PR #40 (draft).
- **Branch:** `claude/netcode-proof-test-plan-hcky5g` (this cloud session's stand-in for `cs/netcode-proof-test`).
  **Base:** main f5030a3 merged in at a6acc8a (MAIN-61, 2026-11-19 15:4x UTC: #61, change-set 2's v2 code; one
  conflict, docs/STATUS.md: this branch's #40 row kept beside main's #61 row), which clears PR #40's second conflict;
  before it main b763ba3 at 0f12d9e (MAIN-60). MAIN-52 takes main in when it changes code, CI or rules, and once before #40 is marked ready.
- **Last update:** 2026-11-20 16:2x UTC: U4f2's build at its end checks, resumed 16:20 after the floor stop and a
  part-clean (Running work); next its end checks, hand-back, then the review round.
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
- **KEEP-PROGRESSING, 2026-11-20 22:37 Amsterdam (history.md:7402-7410):** while F-B-COUNT waits (his Advisor answers
  in about two days), all plan work that does not wait on it goes on; a very important question that needs his answer
  may go to him in the chat when it arises; everything else stays in the day's one message.
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
- **U4F2-SIZE ANSWERED: (a), build U4f2 whole** (Mustafa, 2026-11-20 14:10:29 UTC; word for word at
  history.md:7255-7271 by record_u4f2_size.py): **U4f2's size line** (SIZE-CHECK): about 1,700 to 2,250 added lines (up
  to about 2,700 at 1.2), the four riders included; **the stop past 3,000 at a block end**. Brief u4f2-brief.md rev 1.2,
  filled at launch (`<base>` a289def, counts 642/739). **The build starts now.**
- **The advisor's -20 ruling** (ADV-U4F20-1 High: npt-sim's one-edge rule; fix U4f-F1, -20 re-run) and U4f-F1's
  build line: history.md:7195-7206 (card_trim_1120d.py).
- **-20 and -22's first set records and U4f-F1's plan line:** history.md:7209-7216 (card_trim_1120e.py). **-20's L1
  now PASSES on F1's fix** (46ca143; its record at u4f-report.md item 5's end; F1's report u4f-f1-report.md).


## Choices taken without him (TEST-FIRST rule 2: inside the plan, undoable, no pass mark, no money, nothing on main)
- **2026-11-20 (to name in the next day's message, 2026-11-21 Amsterdam; advisor-u4f-f1-r1.md:207-221, ADV-U4FF1P-4):**
  U4f-F1's one review round (four seats, all APPROVE WITH FINDINGS) and its small fix pass (doc wording, one new test,
  two test touches), checked by the advisor, no second round; after F1, "0 presses lost" in -20 and -58 holds by the
  press counters' design, as "0 presses doubled" did (the room counts with the rule the sim runs; T1/T2 and their
  plants prove the take; the runs show presses equal to the scripted 432,000 per option-B run); a judged check of that
  count is a marks.json change, his, on "After the close"; the first-command gap F1 widened (a pool of grenades per
  rejoin, an honest restart's phantom presses) goes to U6 as C-U6-19, closed before -19 runs. Why: inside the plan,
  undoable, no pass mark, judge or marks.json change.
- **2026-11-20 by the advisor's U4g brief check (to name in the 2026-11-21 message; advisor-u4g-brief.md:135-186):**
  U4G-Q3 the scenes (-45 on SS with the brawl's load, its B1 set serving -17's and -18's B1 parts; -55 on SS with one
  client and printed contact ticks; -60 and -62 on CO, -60's soldier moving and firing); Q4 -60 under both options, +3
  short runs (no run dropped, no bar moved); Q5 the three bite sites; Q6 the client traces corrections, the room's
  records the test oracle; Q7 the cap read on U4g's own B1 and P4 pre-runs, 1,600 stands; Q8 the door-gap margin printed
  on every B1 run; Q9 -45's share read on slot 0, its bound on any slot, every slot printed (a per-slot series would be
  marks.json text: after the close); NC-U4R-4 by the lab's `tick` record. Why: inside the plan, undoable, no pass mark,
  judge or marks.json change.
- **2026-11-20 by the advisor's U4f2 round-1 ruling (to name in the 2026-11-21 message; advisor-u4f2-r1.md:331-344,
  its paste-ready text in msg-2026-11-21.md, choice 3):** the fix pass FP-1 and FP-3 to FP-9; the builder's five
  departures accepted (`HOLD_STEPS` in a line of its own); the test-step line read on the builder's two counted runs;
  a second round on what changed; U4f2's size after it about 2,320-2,490, told to him, not asked. Why: inside the
  plan, undoable, no pass mark, judge, marks.json or existing SERIES.md row change, no money, nothing on main.
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
- **Moved to history.md:7273-7311** (card_trim_1120f.py, 16:2x UTC): U4f-F1 CLOSED (:7274-7280; the advisor's close
  read APPROVE WITH FINDINGS, its three Lows fixed), U4g's brief from rev 1 to rev 2.1 (:7281-7299; REVISE, then
  APPROVE WITH CONDITIONS 1-5, all met: **u4g-brief.md rev 2.1 is ready**; **U4G-Q1's final text for the 2026-11-21
  message: advisor-u4g-brief.md:360-374**, lean (b) split; U4G-Q2 before -58's set only), U4f2's brief check
  (:7300-7311; APPROVE WITH CONDITIONS, rev 1.1, then rev 1.2 at launch; its size line, now answered).
- **U4f2: round 1 RULED; fix pass part A RUNNING from ~21:33 UTC.** Its record so far (the build, the four seats'
  verdicts, the ruling's summary) is word for word at history.md:7324-7367 and :7369-7400 (card_trim_1120g/h.py). The
  round-2 base is snapshot e9a0ae3e3f48af18… (2,077 added); the fix pass is in the builder's worktree
  .claude/worktrees/agent-aa62d31b6fa5b9807. **The ruling** (advisor-u4f2-r1.md): REQUEST CHANGES. Part A: FP-1, 3, 5,
  6, 7, 4, 8, then fmt/clippy/doc, then the builder stops. **FP-2 only on his F-B-COUNT answer** (on (b) or (c), to
  the advisor first). Then R2-R5, round 2 (netcode, test, rust; xhigh; read only) on the fix pass's diff, then R6
  (check.mjs twice). The advisor's stop past 2,700 added at an FP end; his past 3,000. About four part-cleans (P27's
  second form, no lane running). **msg-2026-11-21.md drafted** (F-B-COUNT, a stop; U4G-Q1, not a stop; three
  choices): send it at the 23:05 UTC reminder (trig_01YRVLwWYgbgKS1DLsHeCcGx). **While his Advisor answers
  (KEEP-PROGRESSING, about two days):** **U4h's brief rev 1 DONE ~21:59** (u4h-brief.md, 296 lines, read
  :192-297; notes u4h-brief-notes.md; saved 5e0f6d0): whole about 2,800-5,200 (about even to pass 6,000 at 1.5), a
  split U4h/U4h2 offered; U4H-Q1 to Q12 for the advisor; **U4H-SIZE and U4H-JUDGE (the judge gains four planned
  figures) are his**, neither a stop now (a later day's message, after the advisor's check); three items sent to U4h
  never reached the register (N2-L3 (b), INF-U4R-6 with det-u4's 240 s, pump_report's RTT). **The advisor's check
  RUNNING from ~22:00 UTC** (advisor-u4h-brief.md); U4g's build only after U4f2's merge
  (u4-build-plan.md:951-952, "never two at once"); U4G-Q2's text later, before -58's set. **Lesson:** never `cd` into an agent's
  worktree (the harness moved this session's main folder to it at 16:19; back at once); read it with Read/Grep.
  Check-in on (trig_01EKqNqwZrQTVJ6caAyRrug5); saver bh8edyqwv from 21:10 UTC (to about 23:00). Main f5030a3.

## Cuts by the spend limit (SPEND-LIMIT (a): each cut's notice, word for word, its cut and its resume time, UTC)
- **2026-11-20 18:12:22 to 21:09:44 UTC (a spend-limit cut during U4f2's round 1):** the notice, word for word
  (transcript line 63943, 18:12:22.988; the same text again at 18:34:48, 19:46:06-07 and 20:35:28-30, each time a
  turn tried to start): "You've hit your org's monthly spend limit · ask your admin for a higher limit · your 5-hour
  limit resets 8:50pm (UTC)". It ended the rust seat's task (an API error on the same text: rate_limit, HTTP 429)
  after its file was complete (verdict, four findings, Mustafa's section; in the saver's 18:16 save d96777a), so the
  seat was not resumed. The saver ended on its own time at 19:46:05 (exit 0); the 18:34 and 20:34 check-ins waited
  unread. His words on resuming (21:09:44 UTC): "I hit my usage limit while you were working, but it has reset now.
  Please continue from where you left off." The worker restarted at 21:09:44. Checked at 21:10: `git status` clean;
  no build or lane running (the panel reads only); the saver restarted (bh8edyqwv, 21:10 UTC).
- Cuts until 2026-11-17: history.md from :5791 on (the old card's block, the latest first) and the 19:46 one; the two
  of 2026-11-18 (builders 2 and 3) word for word at history.md:6921-6932 (card_trim_1119e.py); 2026-11-19's 16:00:10
  one (builder 4) at history.md:7124-7146 (card_trim_1119k.py); the 20:53:34-22:40:41 one (the advisor's -20 ruling
  and the session) and the 11-18/11-19 choices at :7148-7160 (1119l); the 23:18 restart at :7169-7175 (1120b); the
  01:01 restart at :7248-7253 (1120e); 2026-11-20's idle usage cut (06:16-10:57:41) at :7312-7315 and its 04:22
  container restart at :7316-7322 (1120f).
