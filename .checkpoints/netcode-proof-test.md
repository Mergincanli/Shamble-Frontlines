# Checkpoint: netcode-proof-test
- **Goal:** the netcode proof test (SPIKE_PLAN item 1) under its approved plan `.checkpoints/netcode-proof-test/plan.md`,
  rev 3.3 (the advisor and Mustafa, 2026-11-06; verdict advisor-verdict.md) · PR #40 (draft).
- **Branch:** `claude/netcode-proof-test-plan-hcky5g` (this cloud session's stand-in for `cs/netcode-proof-test`).
  **Base:** main b763ba3 merged in at 0f12d9e (MAIN-60, 2026-11-18 23:41 UTC: #55-#60, the first v2 code, DEP-HOLD,
  a CI workflow; one conflict, docs/STATUS.md: this branch's #40 row kept beside main's #60 row), which clears PR #40's
  conflict. MAIN-52 takes main in when it changes code, CI or rules, and once before #40 is marked ready.
- **Last update:** 2026-11-19 13:5x UTC: SIZE-U4F ANSWERED (a) (history.md:6896-6903): U4f closes with what is built;
  its review panel's round 1 next, on builder 3's tree (snapshot 74ae26aa). Disk 5,067 MiB free (13:44 UTC).
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
- **The brief, filled for the build:** `.checkpoints/netcode-proof-test/u4f-brief.md`, sha256
  135325a0bb524104bba9144b366b9ec52525851f5db140b1aca93ba4681a502f, 2,040 lines: rev 3.3 (50530a82…, 1,835 lines) filled
  by `u4f_fill.py --apply` (20dbb02b…, 449 lines) at 11:3x UTC with `<base>` 5b29296b03f4e116c0afac8b89d9bc15cc1adaf7.
  Its path: rev 2 (34a2e55), rev 3 (e1508bc), round 1 (3 Highs), rev 3.1 (9f76a02), round 2 (7 Medium), rev 3.2
  (8ca5ede), the advisor's APPROVE WITH CONDITIONS L1-L6 (advisor-u4f-brief.md: findings :65-172, rulings :173-198,
  bars :325-367, conditions :369-419), rev 3.3 by the fold (u4f-brief-r33-notes.md) and its check
  (advisor-u4f-fold-check.md: ACCEPT, 20 edits applied; D3-1 ruled: a `reorder` event at the run's end, SERIES.md rows
  as B9, test f40; section 4: the fill's form). Counts: at least +119 measured, +134 bites build, 67 negative cases; 21
  pre-runs; disk about 3,727 MiB; planned size about 5,150 added lines (his stop past 6,000). Runner u4f-run.sh
  70eb3da5b8ce9a8e2eda582536eeab93f96fb7fbe98e2a74a28e0492ccf6ddae (u4f_runner.py from u4r-run.sh e9437ae2…; 6 lines
  changed; `bash -n` exit 0; u4f-run.sh.diff). Carry 90 (SA-11, U4h's section, :598-601) added by L3.
- **LEAD-BOUND ANSWERED: (a) 400 ms, frozen** (Mustafa, 2026-11-18 11:31 Amsterdam by his label; reached the chat
  11:10:41 UTC; word for word at history.md's end, "## Decision: Mustafa, 2026-11-18 11:31 Amsterdam", by
  record_lead_bound.py): the entry u4f-lead-bound.md equal to 9ad2effb's (sha256 57c588ec43bae7b7…), its fenced value's
  sha256 4717cd895d9834154d3179f397859096b5a30f7ec290ea03f30148b912e53a2b; A1's movement reading: yes, and **U4f's
  report names it in a line of its own with who reviewed it** (the advisor's A1; read by the advisor and the
  test-engineer); no marks.json, judge file or existing SERIES.md row may change: if the build needs one, stop and ask
  him; "Build U4f"; his size stop past 6,000 added lines stays.
- **The advisor's ruling** (advisor-u4f-build1.md, 283 lines, sha256 da98295e0d0eae7c…, read in full): **APPROVE the
  continuation with conditions.** R2 (g): the room counts from the tick it runs next, 2.4's anchor, so the frozen value
  and its meaning hold (nothing to him; the row quotes the frozen sentence, line 8). New Mediums: F1 the fast clock's
  past-bound stamp never reaches kind 1 (every 10th at newest + 3 + the bound); F2 `World::CHECK_BITES` unrouted in
  `Bites::of`; F3 f28 runs no contract check; F4 `brawl-presses` widens SERIES.md:348's closed list (`brawl` plus a new
  `--brawl-press-every` with its own row). Lows F5-F11. R1 split f6 and f28 per option. R3 blocks B1 fixes, B2 -20's
  rest, B3 the cap's measurement, B4 -47, B5 -22, B6 scripts and one counted check, B7-B9 -21, B10 end checks, each
  at most about 400 lines. R4 build to his 6,000 stop; at the first block end past it, nothing more but the end checks;
  the three lines then, in that day's message, first line "stopped on you": (a) split U4f (close at -20, -47 and -22;
  -21 its own unit) or (b) go on; the advisor's provisional lean (a). Its 22 lines (:162-246) are builder 2's.
- **Builder 3 stopped at his size stop** (its record moved to history.md:6905-6919, card_trim_1119d.py): 6,213 added,
  307 removed, 29 files; end checks A2 and B pass on one tree (630 measured, 727 bites); hand runs p3, p9, p16, p20 done.
  The advisor's stop ruling: advisor-u4f-stop.md (R1-R4; Lows S1-S7, note N1 at :198-210).
- **SIZE-U4F ANSWERED: (a)** (Mustafa, 2026-11-19 12:00 Amsterdam by his label; reached the chat 13:36:25 UTC; word for
  word at history.md:6896-6903 by record_size_u4f.py): U4f closes with what is built; the three finished cheat tests
  stay in it and are reviewed with it; the forged-command test, its six missing cheats and all of -21's runs become a
  small unit of their own next, **U4f2**, with its own size line and review; no pass mark, judge rule, marks.json text
  or run count changes. U4f2 holds advisor-u4f-stop.md:80-89's list (f28's two B halves with it: R3 (ii)); its short
  brief goes to the advisor before its build; its size line by SIZE-CHECK (its plan entry states it; the card writes it
  with its source before its first snapshot). -21 is judged only after U4f2.
- **Round 1 RETURNED** (13:50-14:32 UTC; u4f-panel-brief.md; builder 3's worktree read-only, its hashes equal at 13:44,
  the snapshot's `git apply --check` on the lane's head passing): **u4f-review-r1-full.md** (333 lines, sha256
  6747175e…, by u4f_review_r1_full.py, word for word from the journal; read in full). Five APPROVE WITH FINDINGS;
  **4 Medium** (NC-U4F-1 the controller never skips after a clock jump or stall; TE-U4F-1 f5's B1 budget 3 % unruled;
  TE-U4F-2 U4F-Q9 (a)'s flood test missing, negatives 44 of 45; RR-U4F-1 drop_unencodable's tick-step rule), 25 Low,
  no Critical or High; S1-S7 and N1 agreed (N1: slot 0, the never-acker). Next: the advisor's ruling (advisor-u4f-r1.md).
- **After the panel:** the results word for word by script (u4f-review-r1-full.md); every Medium or higher read in full;
  the advisor's ruling (the fix list); the fix pass in a fresh worktree from the lane's head with the snapshot applied
  (the Lows that ride: S4 USAGE's `--lag-*`, S5 SERIES.md's subsection title, S6 c-u4f-runs.mjs:7's order); round 2
  only on what changed; merge, with plan §4.7's cap text and the plan's U4f and U4f2 entries (stop ruling :98-99);
  5.3's 21 pre-runs; the part-clean at A6's place; the sets -47, -20, -22 (-20's six 10-minute P3 runs read for honest
  overflow resyncs before -22's); U4f's report: A1's movement reading in a line of its own naming who reviewed it (the
  advisor's A1; read by the advisor and the test-engineer), every marks.json or judge change's reviewer in its own line
  (none so far), the fast clock's no pings, f28's B halves moved, the five forgers built ahead for U4f2.

## Choices taken without him (TEST-FIRST rule 2: inside the plan, undoable, no pass mark, no money, nothing on main)
- **2026-11-18 (named in that day's message):** D3-1's fix by the advisor's ruling (advisor-u4f-fold-check.md): U4f's
  lab writes a `reorder` event at a run's end (B9, test f40) so -20's existing reorder rule can read the measured share.
  Why: no bar text, judge file or marks.json changes; undone by dropping B9 before the build merges.
- **2026-11-18 (named 2026-11-19):** the fill's form by the advisor's fill ruling (advisor-u4f-fold-check.md
  section 4, on rule 3): each of his 36 quoted passages copied once, in the brief's 12 item 3 list, with 102 pointers
  elsewhere. Why: rule 3's "does not repeat them"; the words are his, byte for byte; undone by refilling from rev 3.3.
- **2026-11-18 (named that day, 13:45 UTC; the worktrees 2026-11-19):** the advisor's build-1 rulings (advisor-u4f-build1.md): the lead bound
  counts from the tick the room runs next, 2.4's anchor (its one line :57-58; the value stays 400 ms); the fast clock's
  every 10th stamp at newest + 3 + the bound (F1); `--brawl-press-every` as a new setting with its own new row instead
  of a new value under SERIES.md:348's row (F4); f6 and f28 one test per option (R1). Why: inside the plan; no pass
  mark, bar text, judge file or existing SERIES.md row changes; each undone before the merge. Also builder 2's and
  builder 3's fresh worktrees (above), an operational choice.
- **2026-11-19 (named in that day's message):** the advisor's stop rulings (advisor-u4f-stop.md R2-R4): the event cap
  1,600, no longer provisional; -20's set before -22's (order -47, -20, -22, -21), its 10-minute P3 runs read for
  honest overflow resyncs first; the fast-clock cheat sends no pings (a recorded deviation); main b763ba3 taken in now,
  plan first (MAIN-60). Why: inside the plan, undoable, no pass mark, bar text, judge file or marks.json changes.
- **2026-11-19 after his answer (to name in the next day's message):** f28's clamp and lag-switch B halves go to U4f2
  with the rest of -21 (the advisor's R3 (ii); his (a) lists the forged test, its six cheats and -21's runs). Why:
  inside the plan, undoable, no pass mark; neither half is built, and no -21 run counts before U4f2 merges.

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
  :78-85); the items the old card lists (history.md:6458-6461: Q15 (a), WS-RECONNECT-BAR (b), the PM-0001-31 pointer).
- **Carried past U4 (the old card's Open findings, history.md:6430-6464):** the advisor's U2 carries C1-C5 and its
  Note; C-U7-1; the U1 carries (C4, C6, A3, the results ledger, Q13, Q23); C6 widened (U6, before R3); JITTER-ORDER
  (parked: to him before R3 only if C6 shows the model's reordering moves a WebTransport result).
- **For PR #40's ready head** (the advisor's MAIN-60 head check): CI's green is bb7a5ed's; the head marked ready needs
  its own run. STATUS.md's #40 row still reads "main 53272a1 merged in": PR #40's closing docs pass (ADV-M60-7).

## After the close (TEST-FIRST rule 1's list: found on the way, outside the plan)
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
- **Builders 1 and 2 ENDED** (13:13 and 17:59 UTC): their worktrees `.claude/worktrees/agent-a856ba7ccb7d032c3`
  (u4f-build, 827518b) and `.claude/worktrees/agent-aa8301a8782f8f434` (u4f-build2, d1b24c3), uncommitted, kept as
  backups; builder 2's is the snapshot 3d341c58… (builder 1's work in it). The advisor ENDED about 13:40 UTC.
- **Builder 3 ENDED 22:59 UTC at the stop** (agent a0825a5a2491eb175): worktree
  `.claude/worktrees/agent-a0825a5a2491eb175` (u4f-build3 at 07289a4, uncommitted), kept with builders 1's and 2's; its
  final snapshot wip/u4f-build.patch 74ae26aa29c229d6…, 380,276 B, 29 files: the one snapshot until the merge.
- **MAIN-60 DONE** (its record at history.md's end, card_trim_1119c.py): main b763ba3 in at 0f12d9e; CI green on
  bb7a5ed; the advisor's head check APPROVE WITH FINDINGS (advisor-main-60-head.md: six Lows, no push). R3 (iii)'s
  four hand runs done (.gate/u4f/w3-hand.log: each exit 0, contract PASS). The lane goes on from this head.
- **U4f's panel, round 1, ENDED** 14:32 UTC (workflow wf_a011d9f2-abf: 5 of 5 results, 0 errors; 393 tool uses).
  **The advisor's ruling on it RUNNING** (from 14:33 UTC; agent abf879016ff011b9c, general on main's advisor.md, xhigh;
  its file advisor-u4f-r1.md). **If cut:** SendMessage to it to continue from its file, or a fresh advisor reading it.
- **The saver bde01r5pv** (13:49 UTC on, both attribution lines as trailers; ADV-M60-8; its window ends about 15:39:
  restart it while agents run, run_in_background, timeout 7200000). The 2-hour check-in trig_01EKqNqwZrQTVJ6caAyRrug5
  ENABLED again at 13:49 UTC (next 14:34); pause it when no agent runs.

## Cuts by the spend limit (SPEND-LIMIT (a): each cut's notice, word for word, its cut and its resume time, UTC)
- Cuts until 2026-11-17: history.md from :5791 on (the old card's block, the latest first) and its end (the 19:46 one).
- **2026-11-18 18:47:03 to about 22:39 UTC (builder 3):** "You've hit your org's monthly spend limit · ask your admin
  for a higher limit · your 5-hour limit resets 9:30pm (UTC)" (18:47:03.951, builder 3's transcript, while it waited on
  end check A2, which went on and passed at 18:50:00). Checked at 22:38: no process left; no plant (its notes: none in
  B8f or B9a); no half edit (the worktree's runner hashes equal A2's start, 54cb6faa… and e8dc3663…); snapshot taken;
  resumed by SendMessage about 22:39. The 20:34 check-in reached the session at 22:36 with the 22:34 one; the worker
  restarted twice; the saver ended at its window.
- **2026-11-18 14:06:57 to 16:38 UTC (builder 2):** "You've hit your org's monthly spend limit · ask your admin for a
  higher limit · your 5-hour limit resets 4:10pm (UTC)" (14:06:57.143, builder 2's transcript, right after its Edit at
  14:06:56 that added SERIES.md's `header.params.connects_count` row whole). Checked at 16:37: no plant in place
  (room.rs 8d7ad78f…, client lib.rs 1f932bd5…, its notes' clean sha256s), no half edit; snapshot taken; resumed by
  SendMessage about 16:38 UTC. The 14:35 and 16:34 check-ins reached the session only at 16:36 (handled together).
