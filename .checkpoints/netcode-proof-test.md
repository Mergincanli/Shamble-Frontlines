# Checkpoint: netcode-proof-test
- **Goal:** the netcode proof test (SPIKE_PLAN item 1) under its approved plan `.checkpoints/netcode-proof-test/plan.md`,
  rev 3.3 (the advisor and Mustafa, 2026-11-06; verdict advisor-verdict.md) · PR #40 (draft).
- **Branch:** `claude/netcode-proof-test-plan-hcky5g` (this cloud session's stand-in for `cs/netcode-proof-test`).
  **Base:** main f5030a3 merged in at a6acc8a (MAIN-61, 2026-11-19 15:4x UTC: #61, change-set 2's v2 code; one
  conflict, docs/STATUS.md: this branch's #40 row kept beside main's #61 row), which clears PR #40's second conflict;
  before it main b763ba3 at 0f12d9e (MAIN-60). MAIN-52 takes main in when it changes code, CI or rules, and once before #40 is marked ready.
- **Last update:** 2026-11-19 15:0x UTC: SIZE-U4F ANSWERED (a) (history.md:6896-6903); round 1 returned and ruled
  (advisor-u4f-r1.md); builder 4's fix pass next. Disk 11,784 MiB free (14:53 UTC, after the part-clean).
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
- **Round 1 RETURNED and RULED** (13:50-14:32 UTC on builder 3's worktree, read only): **u4f-review-r1-full.md** (333
  lines, 6747175e…, word for word by u4f_review_r1_full.py; read in full): five APPROVE WITH FINDINGS, 4 Medium
  (NC-U4F-1, TE-U4F-1, TE-U4F-2, RR-U4F-1), 25 Low, no Critical or High. **The advisor's ruling** (advisor-u4f-r1.md,
  407 lines, 07c1bcd5…, read in full): APPROVE WITH CONDITIONS: one fix pass FP1-FP19 in its order (:156-273), each
  with its acceptance check; FP20 the main session's at the close (:275-281); no round 2 by default (the 2026-11-17
  choice, history.md:6818-6819, checked), its check of the pass instead (:311-323); nothing to him; U4f2 takes
  C-U4F2-1 to -4 and C-U4F2-SEC-1 (:65-87); the later carries C-U5-1, C-U6-17, C-U6-18, C-U7-3, C-U7-4 (:137-154) go
  into plan.md's rows at the merge; its guard: past 500 added lines in the pass, the builder stops and comes back.
- **The fix pass, builder 4:** a fresh worktree on the lane's head plus the snapshot (its runner hashes must equal
  54cb6faa…/e8dc3663… before any edit); logs .gate/u4f/w4-*; notes u4f-build.md "## Fix pass (round 1): builder 4";
  end checks of section 6 in both builds, FP6's two hand runs; the main session snapshots it at check-ins
  (worktree_patch.py). The reviewed snapshot 74ae26aa stays at 7a59d7a:wip/u4f-build.patch.
- **After the pass:** the advisor's check of it (:315-320); the merge on M1-M9 (:325-358): one commit with plan §4.7's
  cap text, the U4f and U4f2 entries, §4.1's rows, the carries in plan.md's rows and u4f-report.md (A1's movement
  reading in its own line naming the advisor and the test-engineer; the D10 line; the rest as M8 lists); then 5.3's 21
  pre-runs, the part-clean at A6's place, the sets -47, -20, -22 (-20's 10-minute P3 runs read for honest overflow
  first); the set reports with NC-U4F-5 and C-OQ6-SEC-1 (-20) and N1 (-22).

## Choices taken without him (TEST-FIRST rule 2: inside the plan, undoable, no pass mark, no money, nothing on main)
- **2026-11-18's three** (D3-1's fix, the fill's form, the build-1 rulings; each named in its day's message): word for
  word at history.md:6935-6946 (card_trim_1119f.py); MAIN-60's done line at :6947-6949.
- **2026-11-19 (named in that day's message):** the advisor's stop rulings (advisor-u4f-stop.md R2-R4): the event cap
  1,600, no longer provisional; -20's set before -22's (order -47, -20, -22, -21), its 10-minute P3 runs read for
  honest overflow resyncs first; the fast-clock cheat sends no pings (a recorded deviation); main b763ba3 taken in now,
  plan first (MAIN-60). Why: inside the plan, undoable, no pass mark, bar text, judge file or marks.json changes.
- **2026-11-19 after his answer (to name in the next day's message):** f28's clamp and lag-switch B halves go to U4f2
  with the rest of -21 (the advisor's R3 (ii); his (a) lists the forged test, its six cheats and -21's runs). Why:
  inside the plan, undoable, no pass mark; neither half is built, and no -21 run counts before U4f2 merges.
- **2026-11-19 by the advisor's round-1 ruling (to name in the next day's message):** -22's judged set runs U4f's
  controller (NC-U4F-2); f5 is measured as the brief writes it, late apart from missing, the unruled 3 % dropped
  (TE-U4F-1; no bar changes); no second review round on the fix pass (the 2026-11-17 choice), the advisor's check of
  it instead. Why: inside the plan, undoable, no pass mark, judge file, marks.json or existing SERIES.md row changes.
- **2026-11-19 (to name in the next day's message):** main f5030a3 taken in now, while builder 4 runs, plan first
  (MAIN-61; advisor-main-61.md C1-C6). Why: his MAIN-52 rule; inside the plan, undoable; nothing on main changes.

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
- **U4f's panel, round 1, ENDED** 14:32 UTC (wf_a011d9f2-abf: 5 of 5 results); **the advisor's ruling ENDED** about
  14:52 UTC (agent abf879016ff011b9c). **Part-clean before builder 4 DONE** (P27's six cleans and checks in one
  main-run.sh call, .gate/u4f-b4/clean-shared.log, exit 0, 14:53 UTC): free 5,054 → 11,784 MiB.
- **MAIN-61** (main f5030a3, PR #61, in; PR #40 un-mergeable since 15:11 UTC, STATUS.md the one conflict): plan
  main-61-plan.md, **APPROVED WITH CONDITIONS C1-C6** (advisor-main-61.md, 201 lines, read in full; "As approved" in
  the plan). MAIN-61 approved (advisor-main-61.md), running steps 1-7. If `git rev-parse -q --verify MERGE_HEAD` prints
  f5030a33185b1ea1b15329174e82823f0215f993, the merge is in progress: no save, no `git merge --abort`, no reset. If `git
  diff --name-only --diff-filter=U` names docs/STATUS.md, resume at step 3; otherwise resume at step 3's checks before
  the commit (C2 (b)), then step 4. If HEAD^2 is f5030a3 and HEAD is not on the remote yet, resume at step 5. Builder
  4's notes and this card may show unstaged: leave them; step 6's card save takes them. Anything else: the advisor rules.
  **Step 1 (15:3x-15:4x UTC):** the saver bsf862iz2 stopped by TaskStop; the proof used (C5 (b)): its output ends
  "[killed]" with no save logged since 14:57:30, and `pgrep -af "checkpoint.mjs watch"` finds no process (exit 1);
  its last save 52f9f4e (holding this resume line) equals the remote, so no foreground save ran (C5 (c)). **H0 =
  52f9f4ea10f4681a748b48b5ce47b9d8c5855798.** `git fetch origin`: main unmoved. Pre-checks all pass: status clean;
  origin/main f5030a3; base b763ba3; the guard's pathspec empty; merge-tree's one conflict docs/STATUS.md; main's CI on
  f5030a3 green (push runs gate 44551896122, toolchain 44551894194: completed, success). The base light run RUNNING in
  the background (task brz7ok5bs; .gate/main-61/light-base.log). No save until the push (C2 (c)).
  **Steps 1-5c DONE:** the base run passed C1 (a)'s bar (N0 1,515, M0 307); `git merge origin/main` exit 1, STATUS.md
  the one CONFLICT; resolved by Edit (the #40 row ours, #61 main's); C2 (b)'s checks equal; **merge commit a6acc8a**
  (git's message); step 4's checks and C1 (c)'s three all as stated; gate PASS, repo map 170/110, checkpoint tests
  33/33 (.gate/main-61/step5-outputs.txt). **Step 5d PASSED** (.gate/main-61/light.log, 15:42:17-15:42:21 UTC, exit 0,
  HEAD a6acc8a, hashes e3b0c442…: refs 1,565 = N0 + 50 with the exception line; the full marks line; self 78 of 78;
  redact 10 of 10, 307 items, 0 findings). **Step 6 DONE:** pushed at 631edf3 (15:43 UTC); the saver bbe44e8id
  restarted 15:43 (both trailers; window to about 17:33); builder 4's notes copied (copy_b4_notes.py, 866 -> 1,104
  lines) and its worktree snapshotted (wip/u4f-build.patch, base b25b96a, 394,313 B, 29 files), pushed 549ed77.
  **Next:** CI on the pushed head (gate, smoke, toolchain; red stops the lane), then step 7, the advisor's head check.
  Free disk 4,508 MiB at 15:42 (builder 4's cold end checks; the runner stops under 4,096).
  **15:5x UTC, Mustafa (92 % usage): "make sure to save your progress (Same for the agents)".** Pushed: everything to
  52f9f4e. Local only: merge commit a6acc8a and these card lines (no push before 5d, C2 (c)); 5d waits for builder 4's
  lane lock (no log yet). Builder 4 told by SendMessage to bring its notes block up to date. **If cut:** the resume line
  above (HEAD^2 = f5030a3 and HEAD not on the remote: step 5, so 5d again in the background, then step 6, the saver,
  then builder 4's snapshot by worktree_patch.py).
  **Builder 4 at 15:39 UTC** (its message): FP1-FP19 done, each with its log line; no plant in place (FP4's and FP15's
  restored, sha256 equal); SIZE-CHECK 6,425 (the pass +212); end check A running (-x, .gate/u4f/w4-end-check-a.log,
  cold), then B, --list, times, items 4 and 5. **Its notes are in its worktree's copy**
  (.claude/worktrees/agent-a5b1297a320917dfb/.checkpoints/netcode-proof-test/u4f-build.md: :1-866 equal to the main
  folder's, then its block): the harness keeps an isolated agent in its worktree. After the push, copy that block to
  the main folder's file by script, word for word, then snapshot its worktree.
- **Builder 4 (the fix pass) RUNNING** from 14:57 UTC (agent a5b1297a320917dfb, general, opus, xhigh; its isolation
  worktree under .claude/worktrees/, branch u4f-fix1 from b25b96a plus the snapshot; logs .gate/u4f/w4-*). **If cut:**
  SendMessage to it; first check its worktree's runner hashes and plants against its notes block.
- **The saver** restarted about 14:58 UTC (bde01r5pv stopped then, by TaskStop, a foreground save after it), both
  attribution lines as trailers (ADV-M60-8); its window ends about 16:48: restart it while agents run
  (run_in_background, timeout 7200000). The 2-hour check-in trig_01EKqNqwZrQTVJ6caAyRrug5
  ENABLED again at 13:49 UTC (next 14:34); pause it when no agent runs.

## Cuts by the spend limit (SPEND-LIMIT (a): each cut's notice, word for word, its cut and its resume time, UTC)
- **2026-11-19 16:00:10 to about 17:45 UTC (builder 4; the session too):** "You've hit your org's monthly spend limit
  · ask your admin for a higher limit · your 5-hour limit resets 5:40pm (UTC)" (16:00:10.105, builder 4's transcript,
  one second after its end check A2 ended: .gate/u4f/w4-end-check-a2.log, 15:43:29-16:00:09 UTC, exit 0, all 11 steps,
  634 measured / 731 bites, test step 882.8 s). Mustafa's own limit cut the session too ("I hit my usage limit while
  you were working, but it has reset now", reached about 17:42); the worker restarted, ending the saver bbe44e8id; the
  16:34 check-in reached the session at 17:42. Checked at 17:43: no process left; no plant (its notes at 15:39); no
  half edit (the worktree's runner hashes equal A2's start, 2f07abc2… and 1a767d5b…, rebuilt from the 15:43 snapshot);
  w4-end-check-a.log (15:32) was cut off after its doc step, then re-run as A2. Resumed by SendMessage about 17:45.
- Cuts until 2026-11-17: history.md from :5791 on (the old card's block, the latest first) and the 19:46 one; the two
  of 2026-11-18 (builders 2 and 3) word for word at history.md:6921-6932 (card_trim_1119e.py).
