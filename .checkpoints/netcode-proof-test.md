# Checkpoint: netcode-proof-test
- **Goal:** the netcode proof test (SPIKE_PLAN item 1) under its approved plan `.checkpoints/netcode-proof-test/plan.md`,
  rev 3.3 (the advisor and Mustafa, 2026-11-06; verdict advisor-verdict.md) · PR #40 (draft).
- **Branch:** `claude/netcode-proof-test-plan-hcky5g` (this cloud session's stand-in for `cs/netcode-proof-test`).
  **Base:** main f5030a3 merged in at a6acc8a (MAIN-61, 2026-11-19 15:4x UTC: #61, change-set 2's v2 code; one
  conflict, docs/STATUS.md: this branch's #40 row kept beside main's #61 row), which clears PR #40's second conflict;
  before it main b763ba3 at 0f12d9e (MAIN-60). MAIN-52 takes main in when it changes code, CI or rules, and once before #40 is marked ready.
- **Last update:** 2026-11-21 07:01 UTC: U4f2 waits on F-B-COUNT (his Advisor); U5's brief rev 2.1 READY; U4G-EARLY
  APPROVED WITH CONDITIONS 1-10: U4g's blocks 1-4 builder running from 07:00 (Running work).
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
- **LEAD-BOUND (a), 400 ms frozen, and SIZE-U4F (a), U4f2 its own unit:** their answer lines word for word at
  history.md:7492-7505 (card_trim_1121b.py, 05:1x UTC); his words at history.md:6791 on (LEAD-BOUND) and :6896-6903
  (SIZE-U4F).
- **U4F2-SIZE ANSWERED: (a), build U4f2 whole** (Mustafa, 2026-11-20 14:10:29 UTC; word for word at
  history.md:7255-7271 by record_u4f2_size.py): **U4f2's size line** (SIZE-CHECK): about 1,700 to 2,250 added lines (up
  to about 2,700 at 1.2), the four riders included; **the stop past 3,000 at a block end**. Brief u4f2-brief.md rev 1.2,
  filled at launch (`<base>` a289def, counts 642/739). **The build starts now.**
- **Pointers:** the advisor's -20 ruling (ADV-U4F20-1 High: npt-sim's one-edge rule) and U4f-F1's build line at
  history.md:7195-7206 (card_trim_1120d.py); -20 and -22's first set records and F1's plan line at :7209-7216 (1120e).
  **-20's L1 now PASSES on F1's fix** (46ca143; its record at u4f-report.md item 5's end; u4f-f1-report.md).

## Choices taken without him (TEST-FIRST rule 2: inside the plan, undoable, no pass mark, no money, nothing on main)
- **2026-11-21 by the advisor's U4G-EARLY ruling (advisor-u4g-early.md:156-158; to name in the 2026-11-22 message by
  B7's line, :151-154):** U4g's blocks 1-4 now, beside U4f2's wait, under its conditions 1-10. Why: inside the plan,
  undoable, no pass mark, judge, marks.json or money; nothing on main.
- **2026-11-21 by the advisor's U5 brief check (advisor-u5-brief.md:157-203; to name in the message that carries
  U5-SIZE):** U5-Q1 (a) the three other fuzz targets run as -34's parts, reported beside it; Q2 (a) -35's tool at U5,
  its one counted run after U6's merge, the strict count; Q3 (a) npt-alloc the fourth unsafe exception; Q5 ASan if the
  runs fit, else coverage only; Q6 `NPT_BITE` in the bites build only; Q7 the fuzz crate under check.mjs. Why: inside
  the plan, undoable, no pass mark, judge, marks.json or money; nothing on main.
- **2026-11-21 by the advisor's U4h brief checks (to name in the message that carries U4H-SIZE; advisor-u4h-brief.md
  :143-211, :397-407):** U4H-Q1 a 3-tick margin; Q2 the cap `open` by default, the span readable, re-pins allowed; Q3
  shots only in the measured window; Q5 -50's runs with five terms; Q6 both throw kinds by one stand-in builder; Q7
  SERIES.md's cells corrected in one edit; Q8 half the shots at an edge band; Q9 the cover record on SS; Q10 det-u4
  measured before recording; Q11 the bite sites; Q12 JDG-ONLY-1 inside the judge's change; TC's own weapon (spread 0,
  damage 0). Why: inside the plan, undoable, no pass mark, judge, marks.json or money; nothing on main.
- The choices of 2026-11-20 (U4f-F1's round; the advisor's U4g brief rulings; the advisor's U4f2 round-1 ruling),
  named in msg-2026-11-21.md (choices 1-3): history.md:7412-7433 (card_trim_1120i.py).
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
- U5-Q1 term 3 (his, a marks.json item; advisor-u5-brief.md:163-165): -34's notes name "plan §4.7's npt-proto
  messages", which include SessionStatus, while its parts do not.
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
- **U4f2: fix pass part A2 RUNNING; FP-2 waits for F-B-COUNT.** The record from round 1's ruling to now is word for
  word at history.md:7324-7367, :7369-7400 and :7446-7489 (card_trim_1120g/h, 1121a). Part A built (FP-1, 3, 5, 6, 7,
  4, 8; about 2,562 added; snapshot 23c5f788…). The advisor's part-A ruling (advisor-u4f2-parta.md): F-G fixed
  (`every` above kind 7's rest); R0, R4 and R5 now (R5's forged lines a smoke), R2-pre and R3-pre only above 6.5 GB
  free; after FP-2, R5's forged lines, R2 and R3 whole, and the lag-switch A line again; **its stop past 2,800**, his
  past 3,000. The builder resumed at 02:38 after the cut, checking its worktree first (no plant left, its last edit
  whole). **Part A2 DONE ~03:10** (Block 9 "Part A2"; logs u4f2-fg-*, u4f2-r0/r4/r5/r2pre/r3pre-*.log): F-G fixed
  (`every > r`; fp3 [false, false, true]; its plant fails by name); R0 the two timed tests 2 s faster (no narrowing);
  R4 excess 600,000 µm above 0 (counts); R5 all exit 0, in a building 0, the fast-clock proof equal over 450 ticks,
  both judge calls' twin checks 0 (counts, its forged lines a smoke); R2-pre twin mismatch 0 under A and B, shots
  685.77 and 691.52, unrejected kind 7 only (18 A, 8 B: FP-2's), counted = sent for 8, 9, 11, slot 6's `input_missing`
  0 under B (the lab writes no `commands_rerun_count`: a note for round 2); R3-pre shots 673 (3 above the 670 line),
  kind 1 unrejected 1,523, kind 7 10. About 2,566 added; free 7,712,194,560 B. **Nothing else in U4f2 moves before
  FP-2.** **FP-2 on his Advisor's F-B-COUNT answer** (KEEP-PROGRESSING: about two days from 2026-11-20 21:37 UTC);
  then round 2 (netcode, test, rust), R6, the merge, the r2 rows and -21's set.
- **U4h's brief rev 2.1 READY** (u4h-brief.md; the advisor's rev-2 check APPROVE WITH CONDITIONS, all met by my
  Edits; register entries 94-101 at u4s-carries.md:618-659). **U4H-Q4, U4H-SIZE and U4H-JUDGE** (final, section 8)
  go in the first day's message after his U4G-Q1 answer, 2026-11-22 at the earliest; none is a stop. **msg-2026-11-21.md
  SENT ~02:42 UTC** (F-B-COUNT, U4G-Q1, choices 1-3; held by the cut).
- **U5's brief rev 2.1 READY** (u5-brief.md; KEEP-PROGRESSING; U5 builds after U4's close): rev 1 (2db0976) REVISE
  (advisor-u5-brief.md:1-322: 2 High, 5 Medium, 7 Low); rev 2 (2cfa592) with option (a)'s per-type bars from the
  test-engineer's read (u5-alloc-bounds.md, 254 lines, measured at rustc 1.99.1: Input 256, SessionControl 480,
  RoomControl 272, Snapshot at most 155,952 through the client, FullState 2,213,272, the rest 0); the advisor's rev-2
  check APPROVE WITH CONDITIONS (:324-430: 3 Medium, 3 Low), all met by my Edits ~05:3x UTC (no new check needed).
  **U5-ALLOC and U5-SIZE are his** (final texts, section 8; neither a stop): in the next day's message (2026-11-22's),
  with the U4H texts if his U4G-Q1 answer has come. Register lines before U5's launch (section 9). S's letter: U4b 701
  (411eeee), U4f-F1 239 (u4f-f1-report.md:104).
- **U4G-EARLY APPROVED W/ CONDITIONS 1-10** (advisor-u4g-early.md): builder from 07:00 in .claude/worktrees/agent-abc25909b7dd4ad30
  (ff to b25ee7d 07:03); fill u4g-brief.md §9 (`<base>` 8f9d97d, 1faf894's code, 642/739, 1,207 s). **Size line:**
  prediction half ~2,100-3,700 (stop past 6,000); blocks 1-4 ~1,120-1,920 (past ~2,900: advisor). F-B-COUNT: stop it first.
- **Lessons:** never `cd` into an agent's worktree (read it with Read/Grep); an `isolation: worktree` agent starts at
  main, so its first step is `git merge --ff-only <lane commit>` (u4f-brief.md:292-295; missing from U4g's launch).
  **Check-in on again 06:43 UTC** (trig_01EKqNqwZrQTVJ6caAyRrug5; pause it when no agent runs); saver bqiy6tzo9 from
  06:43 (to about 08:43); the 2026-11-22 message at 23:05 UTC (trig_017GgVrcAi86paTAgrupT4yf). Main f5030a3 (06:38).

## Cuts by the spend limit (SPEND-LIMIT (a): each cut's notice, word for word, its cut and its resume time, UTC)
- **2026-11-20 22:39:36 to 2026-11-21 02:37:53 UTC (a spend-limit cut: U4f2's builder in F-G, the U4h advisor in
  its rev-2 check):** the notice, word for word (transcript line 64727, 22:39:36.282; the same text at 23:00:24,
  23:05:59-23:06:01 as the day-message trigger tried, and 00:37:13-19 as the 00:34 check-in tried): "You've hit your
  org's monthly spend limit · ask your admin for a higher limit · your 5-hour limit resets 2am (UTC)". Both agents'
  tasks ended on it (rate_limit, HTTP 429). The saver ended on its own time at 23:00:23 (exit 0) after 6419c2b; the
  23:05 day-message trigger and the 00:34 check-in waited unread. Resumed at 02:37:53 (the 02:35 check-in): `git
  status` clean; the saver restarted (bvyb8q0li, 02:38); both agents resumed by SendMessage, the builder first checking
  its worktree (no plant left, its last edit whole, the tree compiling); the day message sent about 02:40.
- Cuts until 2026-11-17: history.md from :5791 on (the old card's block, the latest first) and the 19:46 one; the two
  of 2026-11-18 (builders 2 and 3) word for word at history.md:6921-6932 (card_trim_1119e.py); 2026-11-19's 16:00:10
  one (builder 4) at history.md:7124-7146 (card_trim_1119k.py); the 20:53:34-22:40:41 one (the advisor's -20 ruling
  and the session) and the 11-18/11-19 choices at :7148-7160 (1119l); the 23:18 restart at :7169-7175 (1120b); the
  01:01 restart at :7248-7253 (1120e); 2026-11-20's idle usage cut (06:16-10:57:41) at :7312-7315 and its 04:22
  container restart at :7316-7322 (1120f); its 18:12:22-21:09:44 spend-limit cut at :7435-7444 (1120j).
