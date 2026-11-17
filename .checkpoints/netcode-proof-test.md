# Checkpoint: netcode-proof-test
- **Goal:** the netcode proof test (SPIKE_PLAN item 1) under its approved plan `.checkpoints/netcode-proof-test/plan.md`,
  rev 3.3 (the advisor and Mustafa, 2026-11-06; verdict advisor-verdict.md) · PR #40 (draft).
- **Branch:** `claude/netcode-proof-test-plan-hcky5g` (this cloud session's stand-in for `cs/netcode-proof-test`).
  **Base:** main last merged in at 3f2fa21 (main 82c605e, PR #54). Main is now c94c81b (PR #55, five docs files only):
  not taken in; MAIN-52 takes main in when main changes code, CI or rules, and once before #40 is marked ready.
- **Last update:** 2026-11-17 22:44 UTC: the card cut to a handoff (TEST-FIRST); U4f's brief rev 3.2 approved with
  conditions by the advisor (advisor-u4f-brief.md).
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

## Now: U4f (clock and commands; u4-build-plan.md §5 :805-816; size M)
- **The brief:** `.checkpoints/netcode-proof-test/u4f-brief.md` rev 3.2 (sha256 e9f33e61…, 1,734 lines; committed at
  8ca5ede). Its path: the night's rev 2 (34a2e55); rev 3 (e1508bc); round 1, both checks FAIL with 3 Highs; rev 3.1
  (9f76a02); round 2, no High (7 Medium); rev 3.2, the last fix pass. Before the advisor: D2 (the disk, 18:54 UTC) and
  D4 (two ClockSync models: u4f-r3/d4/d4-output.md, u4f-r3/d4b/d4b-notes.md, compared in u4f-r3/d4/d4-compare.md).
- **The advisor's check: APPROVE WITH CONDITIONS** (`advisor-u4f-brief.md`, 439 lines, read in full where it matters:
  the findings :65-172, the bars :325-367, the conditions :369-419). A1 Medium: "movement" on a death or respawn tick
  (count only the sim's movement step); A2 Medium: U4F-Q6 (b) needs npt-server's `Pong` (pump.rs:458-467) and its two
  test literals, so section 6 item 4 and D9 apply; A3 Medium: `tick.s.cmd_on_time` counted by stamp (a lost command is
  not on time); A4-A7 Low. The 18 rulings at :173-198 (Q6 (b) with an averaging filter, f1 pinned at 4.0 ms; Q7 (b) with
  (i) and (iii) (c); Q8 (b) at 400 ms; Q18 (a) with (ii) (d); the rest at :173-198). After the fold: at least +118
  measured tests, +133 bites build, 67 negative cases, 21 pre-runs, disk about 3,727 MiB, planned size about 5,150
  added lines (4,200 to 6,200; his stop past 6,000).
- **Next, in the advisor's order (:411-419):**
  1. **L2, LEAD-BOUND:** draft its entry by :336-361; the advisor and the test-engineer read it once (one agent each, no
     round); then to him in the next daily message. Nothing that reads the bound is built before his yes.
  2. **L3:** SA-11's carry to u4s-carries.md's U4h section; the readings PR's list gains A1's reading of ADR
     0001:1611's "moves" (this card, Open findings); the hold-time cap's -21 counter (Q14 (b)) and SA-16's variant go
     on "After the close" for STATUS (TEST-FIRST rule 1: no docs work on main's files now).
  3. **L1:** fold the ruling into the brief as rev 3.3 by script (each old phrase found once, adv_phrases.py's method);
     a phrase that fails to match goes back to the advisor.
  4. **L4 (section 12):** `<base>` and D1; u4f-run.sh made from u4r-run.sh by script; the placeholders; re-stamp; D3 by
     the Grep tool, then the u4r-P3 pack by 8.1 and the shared target's part-clean, D2 before and after each.
  5. **L6, after his yes:** the launch: one builder (general-purpose, opus, xhigh) in its own worktree under
     W1-TARGET's lock and u4f-run.sh; then the panel in two batches; merge; 5.3's pre-runs; the part-clean at A6's
     place; the sets one at a time (-47, -22, -20, -21: about 6-8 h of runs); U4f's report.

## Choices taken without him (TEST-FIRST rule 2: inside the plan, undoable, no pass mark, no money, nothing on main)
- **2026-11-17:** the advisor's rulings on U4F-Q1 to Q18 (advisor-u4f-brief.md:173-198), all but Q8 (iii), which
  sends the lead bound's value to him (LEAD-BOUND). Why: they choose how U4f is built and tested inside plan §5 U4f;
  the brief is not built yet, so each can be undone; none changes a marks.json text or a judge file.
- **2026-11-17:** no second review round or second model by default from now on (one round; a second only where round
  1 leaves a High or a design change); a workflow only for parallel review rounds. Why: TEST-FIRST rule 3.

## Next steps (plan rev 3.3; units ahead)
1. U4f (above). 2. U4g (prediction, interpolation and events; M) and U4h (lag compensation and aim; M), each with a
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
- **At the next main take-in:** docs/STATUS.md :28-29 conflicts: keep this branch's #40 row and main's #55 row
  (DOCS-54's C3; history.md:6388-6394).

## After the close (TEST-FIRST rule 1's list: found on the way, outside the plan)
- NS-TRAILER and NS-FIRST: the night-shift lines' attribution option and status refresh, for the kit's next change
  after the freeze (his RECS-1043 (b), history.md:2217).
- For STATUS (the advisor's U4f L3): the hold-time cap's -21 counter (U4F-Q14 (b): a follow-up for the first unit that
  runs ws sessions) and SA-16's variant (a client that acknowledges only forced full states cycles resyncs and is never
  kicked: U6's limits).
- The kit's paper costs I named to him on 2026-11-17 (my message after the card cut): for a decision after the close.

## Key numbers
- `<base>` counts at 0ad9661 (U3C-HELD-1's counted run E6): check.mjs's 11 steps pass; tests 552 measured and 632 bites
  build, 0 failed, 0 skipped; refs 1382; marks 81; self 78 of 78; redact 10 of 10, 306 items, 0 findings; deps 137 crate
  files; wasm 27000 ticks, 0 mismatching. spikes/ is unchanged since 0ad9661.
- Disk (D2, 2026-11-17 18:54 UTC): 5,468 MiB free (the floor 4,096); runs/ u4r-P3-0f066338dd8d 519 MiB (its pack gives
  back about 380), u4r-P3-d66e23cd78f2 298, u4r-P3-fd4612050cd6 291, u4r-P2-8ee453c22882 741, u4r-P2-fc411116c1f1 623,
  u4r-P0-051995db9cea 3,249, u4r-P0-93509dfd7fd1 3,393, the c-u2 folders 998 each; .gate/shared-target 6,346; the main
  folder's target 776. U4f's sets and pre-runs need about 3,727 MiB.
- The packs so far (history.md, the old card's Key numbers :6523-6554): u4r-precheck-30, u4r-P2-ab10f058a828,
  u4r-B1-64f3307d5c7f; each gave back about 73 % of its folder.
- Mustafa's count at 9024840 (2026-11-17 19:04 UTC, against main): 72,678 Rust lines, 7,688 TypeScript and JavaScript,
  608 Rust tests; 177,993 lines of working papers in 934 files.

## Running work
- **Nothing runs now** (22:44 UTC). The saver bnwkv7xyd was stopped between 22:38 and 22:43 (no agent running); restart it
  (`node .claude/scripts/checkpoint.mjs watch netcode-proof-test --trailer "Claude-Session: …"`, run_in_background,
  timeout 7200000) whenever an agent starts. The 2-hour check-in trig_01EKqNqwZrQTVJ6caAyRrug5 stays enabled.
- The last workflow: wf_b2a20e74-6aa (the fix pass and the advisor's check), done before 22:43 UTC.

## Cuts by the spend limit (SPEND-LIMIT (a): each cut's notice, word for word, its cut and its resume time, UTC)
- Every cut until 2026-11-17 is in history.md from :5791 on (the old card's block, the latest first).
- **2026-11-17 19:46 to 22:16 UTC:** "You've hit your org's monthly spend limit · ask your admin for a higher limit ·
  your 5-hour limit resets 10:10pm (UTC)" (19:46:41.750, the advisor agent's transcript). Resumed 22:15:24 (the
  check-in); the workflow by resumeFromRunId at about 22:17. Nothing was half done.
