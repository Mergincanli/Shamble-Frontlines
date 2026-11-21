# Advisor ruling U4G-EARLY (2026-11-21): may U4g's blocks 1-4 build now, while U4f2 waits on F-B-COUNT?

Stand-in: a general-purpose agent following `git show origin/main:.claude/agents/advisor.md` (read at start).
Seat: day session, plan ruling inside the netcode proof test (PR #40, branch claude/netcode-proof-test-plan-hcky5g).

## Plan (blocks I will write, each saved as soon as it is done)
- B1. Facts checked myself (each fact in the request, with file:line or command output; any that does not hold).
- B2. The overlap: U4f2's patch hunks in crates/room and crates/client, set against what U4g blocks 1-4 must touch
  there (u4g-brief.md blocks 1-4), so the hunk list (iii) rests on evidence.
- B3. (i) Worth the risks? conflicts at the re-base; the base without U4f2's or round 2's room and client edits;
  agent and quiet-machine limits; disk.
- B4. (ii) Whose choice: TEST-FIRST rule 2 or Mustafa's.
- B5. (iii) The hunk list, or the rule that replaces it.
- B6. (iv) Rules the early half needs that u4g-brief.md rev 2.1 lacks (-21's evidence kept clean, and others).
- B7. (v) The one-line wording for his message.
- B8. Verdict and numbered conditions.

## Notes (one line per finding as I go)
- N1: the lane rule is the advisor's (u4-build-plan.md:3, rev 2 approved by advisor-u4.md; Lanes :951-952), not a stop
  his words give him; u4f-brief.md:292 repeats it ("U4f, U4g and U4h run one at a time"), and U4g's brief reuses U4f's
  start, so the exception must name both lines.
- N2: -58's `cmd_refused` writer would most likely sit in `receive_input`'s refusal path (room.rs:930-1019 at 1faf894),
  where three of U4f2's seven room.rs hunks are: the one real overlap in blocks 1-4.
- N3: F-B-COUNT (b) opens crates/sim (msg-2026-11-21.md:74-76); U4g's block 1 is crates/sim: a second overlap, only
  under (b).
- N4: the runner's lock is shared for plain builds and exclusive for counted runs and plants, and a call that cannot
  take it exits 5 (u4f-brief.md:838-842): a U4g plain build would make an FP-2 plant or counted call fail, so the two
  builders must not overlap in time, not only in files.

## B1. Facts checked myself (DONE)
| Fact (the request's) | Holds? | My evidence |
|---|---|---|
| Lane rule u4-build-plan.md:951-952; U4g depends on U4f only (:881); earlier lanes ran in parallel (:943-954) | yes | read :870-958. Also u4f-brief.md:292 says the same ("run one at a time") |
| u4g-brief.md rev 2.1: line 3; :27-31; lane :44-50; blocks :66-99; size :217-234; section 8 | yes | read whole (278 lines). Blocks 1-4 rows: sim 150-320, client src 440-700, client tests 300-480, room 230-420 = 1,120-1,920 (my sum) |
| U4f2's lane u4f2-brief.md:170-172; counts :166-168 | yes | read whole (267 lines) |
| wip/u4f2-build.patch 184,651 B, 03:10 UTC; headers at :1387, :1416, :2623, :2772, :2789, :2887 | yes | `ls -l` 184651 B, 03:10:53 UTC; Grep `^diff --git` gives exactly those lines; no crates/sim file in it |
| The patch is U4f2's whole built tree | yes | U4f2's worktree commit 0376c76 has 1faf894's spike tree (`git diff --stat 1faf894 0376c76 -- spikes/netcode-proof ':(exclude)…/results'` empty); no `.rs` under its crates/ is newer than the patch (`find -newer` empty) |
| FP-2 under (a): forged.rs, scenario.rs's wiring, SERIES.md (advisor-u4f2-parta.md:103-104) | yes | read; (b) opens crates/sim (msg-2026-11-21.md:74-76) |
| Spike tree unchanged 1faf894 to the head | yes | the same `git diff --stat 1faf894 HEAD …` prints nothing. Note: the head is now 738ec34 (the card save after 762e001), spike tree the same |
| Disk 7,675,834,368 B at 06:38 | yes (now 7,674,400,768 B) | `df -B1`; U4f2's worktree 76,333,670 B (`du -sb`); the shared target 2,182,039,788 B |
| msg-2026-11-21.md:28-30 and :97 | yes | read whole (98 lines); :90-91 and :97 also say nothing waits on U4G-Q1 until the prediction half is done, which stays true |
| TEST-FIRST rule 2 (history.md:2259-2262) and KEEP-PROGRESSING (:7402-7410) | yes | read verbatim; rule 2 keeps for him "the stops and run days your plan gives me" and any decision CLAUDE.md or the plan gives him |
| Card :150-164: nothing in U4f2 moves before FP-2; no agent runs | yes | the card :162-164, :177-179 |

## B2. The overlap, from U4f2's patch (DONE)
Old-file line ranges at 1faf894, from each hunk's `@@ -a,b` header (each range already holds git's 3 context lines):
- **client/src/lib.rs:** 357-364 (`with_clock`'s doc), 614-622 (`plan`'s doc). Doc text only; no item changes.
- **client/tests/endpoint.rs:** 487-492 (a helper `sent` and C-U4F2-2's test inserted between :488 and the bites module
  at :490).
- **room/src/input.rs:** 47-55, 68-85, 155-161, 251-261, 263-269, 271-277, 280-286, 293-299, 302-308, 321-327, 443-449,
  482-488, 542-548 (the enum `Refusal` renamed `CommandRefusal`, every use).
- **room/src/lib.rs:** 23-32 (the `pub use` lines: `Refusal` out, `CommandRefusal` and `RefusalRecord` in).
- **room/src/room.rs:** 18-24 (imports), 34-39 (new `pub struct RefusalRecord` after `REFUSALS_HELD`), 510-517
  (`incoming`, `refusals` retyped), 630-636 (`Room::refusals()` now `&[RefusalRecord]`), 930-938 (the `record`
  closure), 980-986 (kind 8's records), 1006-1019 (the `Offered::Refused` match).
- **room/tests/commands.rs:** 79-85 (`refused` reads `r.seq`, `r.kind`).
- **What changes meaning, not only text:** room `Refusal` is gone (now `CommandRefusal`), `npt_room::Refusal` is no
  longer exported, `Room::refusals()` returns `RefusalRecord`s, not tuples. The client's public items do not change.
- **Where U4g's blocks 1-4 land** (u4g-brief.md 1.2, :55-62; read at 1faf894): carry 79 at room.rs:119 (counters),
  :1209-1226 (the fan-out), :1586-1593 (the packing); -60's site in the own-group packing (:1437-1519); the bite list at
  room lib.rs:32-56 (an id added changes `BITES: [&str; 16]` at :39, the doc at :32-37); `Predictor` and `CueLog` beside
  `StepOut` (client lib.rs:203-219) and the replay near :455. All outside the ranges above, with 3 or more unchanged
  lines between (lib.rs: U4f2 changes :26 and :29; :30-32 are context). **The exception:** -58's three command counters
  (`tick.v.cmd_refused_count`, `.cmd_lost_count`, `.cmd_doubled_count`: marks.json -58's series) have no writer yet
  (Grep: none in crates/), and the refused count's natural site is the refusal path (room.rs:930-1019) or the records
  `Room::refusals()` returns: both are U4f2's (N2). Lost and doubled sit in the run path (`run_presses`, `begin_tick`
  :374-395, input.rs's `take`), outside the ranges, but input.rs is mostly U4f2's.
- **The tests:** U4f2 inserts at endpoint.rs's one natural insertion point (before the bites module) and edits
  commands.rs's helper; a U4g test added at the same point conflicts by construction (both insert between the same two
  lines). New tests in new files, or at a file's end, do not.
- **crates/sim:** not in the patch; U4f2 keeps it shut under (a). Under (b) it opens (N3).

## B3. (i) Is the early start worth its risks? Yes, with bounds (DONE)
| Risk | How big | What bounds it |
|---|---|---|
| Conflicts at the re-base | Low. U4f2's room and client hunks are one rename, two doc rewraps and one inserted test (B2); blocks 1-4 land elsewhere, but for -58's refused counter and the test insertion point | the hunk and name rule (condition 3), -58's counters and input.rs left to the re-base when they would cross it, tests in new files; `git apply --3way` on full-index patches; the main session resolves, the builder re-runs the tests |
| A base without U4f2's (or round 2's) room and client edits | Low under F-B-COUNT (a). U4f2 changes no behaviour U4g reads: C-U4F2-3 changes the records' type, not when they are made; the client's public items are unchanged. Round 2 is on the fix pass's diff (advisor-u4f2-parta.md:145), mostly bots and lab | condition 3's name rule; the re-base's test re-run; U4f2's merged changes expected there (condition 8). **Under (b) it is not low:** the sim gains a refusal kind and block 1 edits the sim, so condition 5 stops it for a ruling |
| The 3-agent limit | none now (0 agents); 1 with the builder | the builder yields at F-B-COUNT's answer, before FP-2 and round 2's three reviewers (condition 5) |
| A quiet machine for timing | real if the two builders overlap: the lane lock exits 5 rather than waits (N4), and R6 times the test step | the two never build at the same time; nothing of U4g runs during U4f2's runs, R6, the merge check or -21's set (condition 5) |
| Disk | 7,674,400,768 B free, 3.38 GB above the 4 GiB floor; U4g's debug and bites artifacts at a new worktree path, about 0.5 to 1.5 GB (my estimate: U4f2's debug folder holds 1.72 GB with the shared deps); FP-2 and its re-runs about 0.5 GB | launch at 6.5 GB, no U4g build below 5.5 GB, U4g's artifacts are a cache the part-cleans may empty (condition 6) |
| -21's evidence | none by construction if U4g stays out of the main folder and out of every commit's spikes/ until the set closes | condition 4; the run scripts already refuse uncommitted tracked changes (c-u4f-r2.mjs:116-123) and name their commit |
| Paper | one `<base>` fill now, one at the re-base, one apply | small against the gain |

- **The gain:** blocks 1-4 are 1,120 to 1,920 planned lines (about 1,700 to 2,900 at U4f's 1.5; B1), the core of the
  prediction half, built in about two idle days instead of after -21's set, which is itself hours of runs. A Predictor
  or carry-47 problem in the sim surfaces while FP-2 waits, not after U4f2's merge. Parallel builders on the shared
  target have precedent (wave 1's three: u4-build-plan.md:926-929), and so has a lane moved by a ruling (W1-S1, :937).
- **What the main session's proposal misses** (each folded into the conditions): u4f-brief.md:292's "one at a time"
  line (N1); -58's refused counter is the one block-3 item that crosses U4f2's hunks (N2); F-B-COUNT (b)'s overlap in
  crates/sim (N3); the lock makes overlap in time, not only in files, the risk (N4), so "pause at a block end" is not
  enough once FP-2 starts; the "there now" re-read must cover room and client too (U4f2's room.rs gains 14 lines at
  :34, so every later room.rs line in the brief moves); an inserted test at endpoint.rs:488-490 conflicts by
  construction; a shut crate's test that a block-3 change breaks cannot be fixed early.

## B4. (ii) Whose choice: mine under TEST-FIRST rule 2, not his (DONE)
- **All four of rule 2's tests hold** (history.md:2261): inside the plan (U4g's own work by its approved brief; only its
  timing moves, and the lane order is the U4 build plan's, approved by the advisor: u4-build-plan.md:3; advisor.md
  gives me "the order of approved work"); undoable (a worktree and a patch, nothing merged; dropping them loses only
  builder time); no pass mark, judge, marks.json, run count or SERIES.md row moves; no money, nothing on main.
- **Not one of "the stops and run days your plan gives me"** (:2262): his stops are SIZE-CHECK's (U4g's past 6,000, or
  his U4G-Q1 answer's), Stop B and Stop C; none moves. The lane line is a file-conflict rule, not a stop he holds.
- **CLAUDE.md is kept:** two agents never edit the same working copy (separate worktrees, U4f2's builder idle now and
  in lab files under (a)), lanes merged one at a time (U4g after U4f2).
- **What he was told** (msg-2026-11-21.md:28-30) was the plan's order, stated as a fact, not a decision of his; the
  next day's message corrects it in one line and names the choice, which he can reverse. KEEP-PROGRESSING
  (history.md:7406) points this way. Not "very important" enough for a chat question outside the day's message.

## B5. (iii) The hunk list, and the rule around it (DONE)
- **The list:** B2's ranges, old-file lines at 1faf894 (the early `<base>`'s spike tree), each range holding git's 3
  context lines, so a U4g change outside every range leaves at least 3 unchanged lines between it and U4f2's changes and
  a three-way apply merges it cleanly.
- **The rule (condition 3):** no U4g changed line inside a range; room/src/input.rs shut whole in the early half (13 of
  its hunks span lines 47-548 of 636); no use of the names U4f2 renames or retypes (the room's `Refusal`,
  `npt_room::Refusal`, `Room::refusals()`, the fields `incoming` and `refusals`, `Offered::Refused`); new tests in new
  files (under crates/room/tests/, crates/client/tests/, or a new source file's test module) or at an existing file's
  end, never at endpoint.rs:488-490. A needed edit that would cross the rule is **deferred to the re-base**, named with
  file:line in the notes, not a stop; it stops the builder for the main session only when the block's other items
  cannot be built or tested without it. -58's three command counters are built early only if all of them fit the rule;
  else they move to the re-base, beside block 5's `wrap` suite (-58 is judged last anyway, after U4G-Q2's yes:
  u4g-brief.md:112).
- **Why a list and not "room and client shut":** it keeps carry 79, server-echo, -60's site, `Predictor` and `CueLog`
  early (most of blocks 2-4), which a shut rule would lose.
- **At the re-base** the list is not re-checked by hand: U4f2's merged diff (round 2's fixes included) is what
  `git apply --3way` merges against; the main session records each hand-resolved hunk.

## B6. (iv) Rules the early half needs that u4g-brief.md rev 2.1 lacks (DONE)
1. **The exception, written where it is read:** u4g-brief.md's line 3 and section 0 (:27-31), u4f-brief.md:292 and
   u4-build-plan.md:951-952 all say U4g waits; one pointer line at u4g-brief.md's head names this ruling for blocks
   1-4 only. The plan's lane line stands for every other unit and for blocks 5 on.
2. **-21's evidence kept clean:** nothing of U4g's under spikes/ in any commit of the lane branch until -21's set is
   closed and recorded; snapshots only as files under .checkpoints/netcode-proof-test/wip/; the builder never writes in
   the main folder; the judged runs keep the main folder's own target (u4f-brief.md:2003; `CARGO_TARGET_DIR` unset),
   never the shared one; -21's report names its runs' commit and spike tree (U4f2's merge commit, or a later commit
   equal to it outside results/). The re-base commit comes after the set's last record.
3. **Time-sharing, not only file-sharing** (N4), and **F-B-COUNT (b) or (c)** (N3): condition 5.
4. **Disk** with a stop line for U4g alone: condition 6.
5. **Block ends read workspace-wide:** a shut crate's test broken by a block-1-4 change cannot be fixed early, so that
   change waits for the re-base with the crate's edit (the brief's "each compiling with its tests passing before the
   next" otherwise fails in a crate the builder may not open).
6. **Two fills:** the early `<base>` now and the real one at the re-base, with the "there now" re-read across all four
   crates U4f2 opened, and U4f2's merged room and client changes named as expected there, so u4f-brief.md:314-316's "an
   item another unit merged changed beyond what this brief names" does not fire on them.
7. **A size tripwire for the early half:** past about 2,900 added lines at its stop (1.5 times the top of 1,920), the
   count and a re-estimate of the prediction half come to me before block 5, since he was given 2,100 to 3,700 for it
   (msg-2026-11-21.md:86-87).
8. **No review round on the early half alone:** the reviewers read U4g's final diff (or each half's under U4G-Q1 (b)),
   inside the review budget; the early stop has no panel.

## B7. (v) The line for his 2026-11-22 message (DONE)
Paste-ready, as one item of "Choices taken without you":
> Correction to yesterday's line on U4g: by the advisor's ruling (U4G-EARLY), U4g's first four build steps (prediction
> in the simulation and the client, their server-side half, and the client's log of its own shot effects) are being
> built now in a separate copy while U4f2 waits for your F-B-COUNT answer; they leave every room and client line U4f2
> changes alone, the rest of U4g still waits for U4f2's merge and -21's runs, and you can reverse this.

For the card's "Choices taken without him": "2026-11-21 by the advisor's U4G-EARLY ruling (advisor-u4g-early.md):
U4g's blocks 1-4 now, beside U4f2's wait, under its conditions 1-10; why: inside the plan, undoable, no pass mark,
judge, marks.json or money; nothing on main. To name in the 2026-11-22 message (B7's line)."

## B8. Verdict: APPROVE WITH CONDITIONS (U4G-EARLY (a), with conditions 1-10) (DONE)
Mine under TEST-FIRST rule 2 (B4); no ESCALATE. No new advisor check is needed before launch unless a condition
cannot be met as written.
1. **Scope (the main session's items 1 and 6):** one builder, Opus 5.5 at xhigh, its own worktree from the lane head at
   launch, blocks 1-4 by u4g-brief.md rev 2.1 and U4f's start, stops and build rules as that brief reads them; shut:
   crates/lab, crates/bots, ops/runs, SERIES.md, room/src/input.rs, and all the brief already shuts. It stops when
   block 4's tests pass, snapshots (wip/u4g-early.patch by worktree_patch.py, its sha256 on the card) and returns. U4g's
   size line on the card before the first snapshot; U4G-Q1 unchanged and not a stop.
2. **The exception written down:** one pointer line at u4g-brief.md's head ("blocks 1-4 build early by U4G-EARLY,
   advisor-u4g-early.md; u4-build-plan.md:951-952 and u4f-brief.md:292 stand for every other unit and for blocks 5
   on"); the launch prompt carries conditions 3-7 word for word.
3. **The hunk and name rule (B5, replacing item 2):** no changed line inside B2's ranges; none of the names U4f2
   renames or retypes; tests in new files or at a file's end, never at endpoint.rs:488-490. A crossing edit is
   deferred to the re-base and noted, a stop only when the block's other items cannot be built or tested without it;
   -58's three command counters early only if they all fit, else beside block 5's `wrap` suite.
4. **-21's evidence (B6 item 2):** nothing of U4g's under spikes/ in any lane commit until -21's set is closed and
   recorded; snapshots only under .checkpoints/netcode-proof-test/wip/; never the main folder; the judged runs keep the
   main folder's own target; -21's report names its runs' commit and spike tree.
5. **Time-sharing (replacing item 4):** the builder runs only while no U4f2 builder or reviewer and no U4f2 runner
   call, counted or pre-read run, R6, merge check or -21 run is active, and inside the 3-agent limit with U4f2's
   agents first. At F-B-COUNT's answer FP-2 launches at once; the builder makes no new runner call once told, leaves no
   plant in place, snapshots and returns; it resumes from its notes only in such a window, and not between U4f2's merge
   and the re-base. Under F-B-COUNT (b) or (c) it stops the same way, and both come to me together before either builds
   on (crates/sim).
6. **Disk (replacing item 5):** launch only at 6.5 GB free or more (`df -B1` in the notes); no U4g build call below
   5.5 GB free (it returns at its next safe point); the main session's part-cleans (P27, no builder running) may empty
   U4g's artifacts from the shared target; the U4g worktree stays until the re-base's apply is checked.
7. **The early `<base>` and block ends:** filled now in U4f's fill form (the lane head, its spike tree equal to
   1faf894's outside results/ by `git diff --quiet`, counts 642 measured and 739 bites by D1's rule, the test-step line
   1,207 s, the "Read, never retyped" stamps refilled). At each block end the whole workspace compiles and its tests
   pass in both builds; a change that breaks a shut crate's test waits for the re-base with that crate's edit. At the
   stop, clippy (both targets) and fmt through the runner; no counted check.mjs run (the end checks run on the re-based
   tree).
8. **The re-base (replacing item 3's details):** after U4f2's merge and -21's set, the main session applies the patch
   in a fresh worktree from the new head with `git apply --3way` (the patch's `--binary` gives full index lines),
   records each hand-resolved hunk with file:line; `<base>` and the stamps refilled; U4f2's merged room and client
   changes (B2's and round 2's) are expected there, not u4f-brief.md:315's stop; the builder re-runs blocks 1-4's tests,
   recounts as deltas on the new `<base>`, re-reads every "there now" line in room, client, lab and bots, builds the
   deferred items first, then block 5 on; the size counts from the new `<base>`, the early lines included.
9. **Size and review:** at the stop the builder states blocks 1-4's added lines against 1,120-1,920; past about 2,900,
   the count and a re-estimate of the prediction half come to me before block 5 (it may change the 2,100-3,700 he was
   given). No review round on the early half alone.
10. **Recorded and named (item 7):** under "Choices taken without him" with B7's card line; the 2026-11-22 message
    carries B7's paste-ready line, correcting msg-2026-11-21.md:28-30.

What I could not check: the U4g builder's real artifact size at a new worktree path (B3's 0.5 to 1.5 GB is an estimate
from the shared target's folders); whether round 2 will ask for room or client changes beyond B2's ranges (the re-base
handles it); F-B-COUNT's answer.
