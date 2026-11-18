# Checkpoint: netcode-proof-test
- **Goal:** the netcode proof test (SPIKE_PLAN item 1) under its approved plan `.checkpoints/netcode-proof-test/plan.md`,
  rev 3.3 (the advisor and Mustafa, 2026-11-06; verdict advisor-verdict.md) · PR #40 (draft).
- **Branch:** `claude/netcode-proof-test-plan-hcky5g` (this cloud session's stand-in for `cs/netcode-proof-test`).
  **Base:** main last merged in at 3f2fa21 (main 82c605e, PR #54). Main is now 22c6afa (PRs #55, #56, #57: docs/ only,
  `git diff --stat HEAD origin/main -- .claude/ CLAUDE.md AGENTS.md .github/ tools/` empty, 2026-11-18 before 11:44 UTC):
  not taken in; MAIN-52 takes main in when main changes code, CI or rules, and once before #40 is marked ready.
- **Last update:** 2026-11-18 11:50 UTC: U4f's brief filled at launch; L4 item 10 re-run; the builder RUNNING.
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
- **LAUNCH (2026-11-18), steps 1-4 DONE:** (1) no process running; the u4r-P3-0f066338dd8d list (11 files, 543,998,603
  bytes) committed alone at d4654df, then packed by 8.1: archive 149,827,571 bytes, "read back: 11 of 11 files equal the
  list", free 5,433 → 5,808 MiB; (2) the six part-cleans in one main-run.sh call (.gate/u4f-launch/clean-shared.log,
  11:35:52, exit 0; the lock and both tags stay), free 12,154 MiB; (3) the light run at d4654df
  (.gate/u4f-launch/base-light.log, 11:36:52, exit 0): refs 1463 tracked files, marks 81, self 78 of 78, redact 10 of
  10 and 307 items, 0 findings; the test counts stay 0ad9661's (552 / 632: spikes/ unchanged but results/'s new list);
  (4) the fill: every assert passes (36 copies once in 12 item 3's list, 102 pointers, his words in no 7-word run
  outside item 3; the stamps re-checked: plan and plan.md unchanged, carries +entry 90 only, history appended only);
  r31_check.py on the result: no unknown id, 0 rev-3 markers, the `<<P`, `<base>` and `@@` left are the name-only
  mentions the fill lists (:125, :151, :164, :266, :275) and item 3's own list (its "P3 to P13" names P4-P12 as a
  range, so the checker's "NOT USED" lines for them are its pre-fill meaning).
  L4 item 10 at launch: D6's diff (7cfae88..`<base>`) prints only carries (+4, entry 90; its cites re-walked by the
  fill) and the new list under results/ (rows 43 and 50 still hold); stamps_anchors.py: P1-P36 each found once, plan
  and plan.md unchanged; adv_words_check.py: 37 hit lines, all pass by the fold check's rule (§3): 34 outside his
  quoted words, :2212 as ruled there, and :6797 and :6799 (his LEAD-BOUND answer) quote rev 3.3's own text back (brief
  :1510 and :541-542, both in 06d419d4 at 01:17:17 UTC, before his message reached the chat at 11:10:41).
- **(5) DONE 11:44 UTC:** the check-in re-enabled (next 12:34 UTC), the saver restarted. **(6) LAUNCHED 11:4x UTC
  (before 11:49):** the builder, one Agent (general-purpose, opus, xhigh, isolation worktree), told to build by the
  filled brief; its prompt names the launch head 827518bde465cf208e461da5fb605993c2bb3220 (`<base>` plus this card and
  the filled brief only) for section 1 item 1, as U4r's correction did (u4r-build.md:5-15), since main's docs-only
  #55-#57 make origin/main no ancestor of this branch; and points to his LEAD-BOUND yes and his two stops at
  history.md:6801, :6803. SIZE-CHECK (brief :921-943): the plan's M is about 3,000 added lines; the brief's estimate
  about 5,150 (4,200 to 6,200); his stop past 6,000.
- **After the build:** the panel in two batches (brief 12 item 8); merge; 5.3's 21 pre-runs; the part-clean at A6's
  place; the sets one at a time (-47, -22, -20, -21: about 6-8 h of runs); U4f's report, with A1's movement reading in
  a line of its own naming who reviewed it (the advisor's A1; read by the advisor and the test-engineer), and every
  marks.json or judge change's reviewer named in its own line (his 13:35 rule).

## Choices taken without him (TEST-FIRST rule 2: inside the plan, undoable, no pass mark, no money, nothing on main)
- **2026-11-17:** the advisor's rulings on U4F-Q1 to Q18 (advisor-u4f-brief.md:173-198), all but Q8 (iii), which
  sends the lead bound's value to him (LEAD-BOUND). Why: they choose how U4f is built and tested inside plan §5 U4f;
  the brief is not built yet, so each can be undone; none changes a marks.json text or a judge file.
- **2026-11-17:** no second review round or second model by default from now on (one round; a second only where round
  1 leaves a High or a design change); a workflow only for parallel review rounds. Why: TEST-FIRST rule 3.
- **2026-11-18 (named in that day's message):** D3-1's fix by the advisor's ruling (advisor-u4f-fold-check.md): U4f's
  lab writes a `reorder` event at a run's end (B9, test f40) so -20's existing reorder rule can read the measured share.
  Why: no bar text, judge file or marks.json changes; undone by dropping B9 before the build merges.
- **2026-11-18 (to name in the next message):** the fill's form by the advisor's fill ruling (advisor-u4f-fold-check.md
  section 4, on rule 3): each of his 36 quoted passages copied once, in the brief's 12 item 3 list, with 102 pointers
  elsewhere. Why: rule 3's "does not repeat them"; the words are his, byte for byte; undone by refilling from rev 3.3.

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
- **2026-11-18 from 11:4x UTC: U4f's builder runs** (agent a856ba7ccb7d032c3; resume it by SendMessage, or a fresh copy
  that reads its notes and `git diff` first). Its worktree: `/home/user/Shamble-Frontlines/.claude/worktrees/agent-a856ba7ccb7d032c3`,
  branch u4f-build at 827518b (its notes: `.checkpoints/netcode-proof-test/u4f-build.md` there). **Check-in 12:35
  UTC:** nothing stopped (its runner in block 4's fmt and clippy, logs .gate/u4f/b1-b4 from 12:06); snapshot
  wip/u4f-build.patch 47ec4a9aecb4d398…, 200,122 bytes, 20 files, on 827518b; its notes copied. **At each check-in:** snapshot it,
  `python3 -I .checkpoints/netcode-proof-test/worktree_patch.py <worktree>
  .checkpoints/netcode-proof-test/wip/u4f-build.patch u4f-build` (one snapshot per live worktree; WIP-CLEAN deletes it
  when merged or replaced), then save the card; on a cut, check the worktree against its last snapshot first.
- The saver bnvncmf4z runs from 11:44 UTC (`node .claude/scripts/checkpoint.mjs watch netcode-proof-test --trailer
  "Claude-Session: …"`, run_in_background, timeout 7200000: it ends by 13:44, restart it while agents run). The 2-hour
  check-in trig_01EKqNqwZrQTVJ6caAyRrug5 is enabled (11:44 UTC; next 12:34); pause it when no agent runs.
- The last workflow: wf_b2a20e74-6aa (the fix pass and the advisor's check), done before 22:43 UTC on 2026-11-17.

## Cuts by the spend limit (SPEND-LIMIT (a): each cut's notice, word for word, its cut and its resume time, UTC)
- Every cut until 2026-11-17 is in history.md from :5791 on (the old card's block, the latest first).
- **2026-11-17 19:46 to 22:16 UTC:** "You've hit your org's monthly spend limit · ask your admin for a higher limit ·
  your 5-hour limit resets 10:10pm (UTC)" (19:46:41.750, the advisor agent's transcript). Resumed 22:15:24 (the
  check-in); the workflow by resumeFromRunId at about 22:17. Nothing was half done.
