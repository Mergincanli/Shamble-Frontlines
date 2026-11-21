# The advisor's ruling at U4f's size stop, 2026-11-18

Seat: a day session's advisor (general agent on main's `.claude/agents/advisor.md`, read with `git show origin/main:` at
b763ba3e5f34c67f…), effort as the session set it. Read only; this file is my one write; no build, no run of the code,
no commit. Under TEST-FIRST (history.md:2254-2279): rulings 2 to 4 are inside plan rev 3.3, undoable, change no pass
mark, cost no money and touch nothing on main (rule 2), so they go by this ruling and the main session names them in the
day's message. Ruling 1 is his: the split is a plan change, and his size stop sends it to him (history.md:2048-2051,
:2067-2075; restated at :6803, "My size stop stays: past 6,000 added lines the build stops and I get three lines").

## Verdict, most important first
- **R1, the three lines: my lean (a) stands, confirmed with the real numbers.** Filled below, ready to paste. Under (a)
  the built parts of -21 stay in U4f's merge and its panel reviews them; the forged suite, f28's two B halves and all of
  -21's runs go to a small new unit. Under (b) U4f runs to about 7,600-7,800 added lines, one review, one merge.
- **R2, the event cap: APPROVE 1,600 as the room's default, no longer provisional**, for -20's, -21's and -22's judged
  runs. Nothing new is measured before the merge. One condition: -20's judged set runs before -22's (order -47, -20,
  -22, then -21), and its six 10-minute P3 runs are read for honest overflow resyncs before -22's set starts.
- **R3, builder 3's notes:** (i) the fast client sends no `Ping`: accepted as a recorded deviation (Low); (ii) f28's
  clamp and lag-switch B halves go to the new unit under (a), into the continuation under (b) (Low); (iii) the final
  pair of end checks settles the counted check, but not section 6 item 5's hand runs of builder 3's four fast-clock
  rows: run them before the panel; verification adds no line (Low); (iv) the usage line's two missing options: Low,
  rides the panel's fix pass.
- **R4, MAIN-52: now, before his answer and before the panel**, plan first as MAIN-54 was. Main's 49 changed files miss
  every file of U4f's patch; the only conflict is docs/STATUS.md (git merge-tree, my read).

## The three lines, ready to paste (they go in the day's message, whose first line says it is stopped on him)
```text
1. What grew: U4f, the clock-and-commands unit, is at 6,213 added lines, against about 3,000 for a unit of its size
   and your stop at 6,000. Its client and server-room parts each came to about twice their estimate; a little over
   half of all its lines are tests.
2. Why: every server check, both command-timing options and every planted bug has its own test, as the brief's
   minimum test counts ask. Built and checked twice, all 11 steps passing: the client's clock (mark -47), command
   timing under both options (-20), the client that never acknowledges (-22), every server check on commands, and
   three of -21's four cheat tests (aim beyond reach, lag switch, fast clock). Not built: -21's fourth, the
   forged-command test, about 1,400 to 1,550 lines more.
3. The cheaper option: split U4f here, (a); or go on, (b).
(a) Close U4f with what is built. The three finished cheat tests stay in it and are reviewed with it: one review
    round, then the merge, then the runs for -47, -20 and -22. The forged-command test, its six missing cheats and all
    of -21's runs become a small unit of their own next (about 1,500 lines, with its own size line and review). -21 is
    judged only after that unit, since the mark needs all four cheat tests. No pass mark, judge rule, marks.json text
    or run count changes.
(b) Go on: U4f grows to about 7,600 to 7,800 added lines, then one review of the whole and one merge, and every run
    of its four marks after that. The build would stop again past 8,000.
My lean: (a). The stop fell at a clean edge (whole tests built, the forged one barely begun), three marks can be
judged sooner, and the hardest part, the forged commands, gets a review of its own.
```

## R1: the size stop, with the real numbers
- **The count, checked here:** `git apply --numstat` of wip/u4f-build.patch (sha256 74ae26aa…, matches) gives 6,213
  added and 307 removed in 29 files. I rebuilt the snapshot from builder 3's worktree by worktree_patch.py's own
  commands (`git diff HEAD --binary` plus each untracked file, .checkpoints/ left out), and its sha256 equals 74ae26aa.
  The runner's two tree hashes, recomputed in that worktree, equal end checks A2's and B's header line (diff
  54cb6faa…, untracked e8dc3663…). 07289a4 equals 5b29296b in spikes/ and docs/ (`git diff --quiet`, exit 0). Builder
  3's 6,212 predates its one-line doc fix, as the main session says.
- **Against the estimates** (brief :927-939; the stop record's table, u4f-build.md :835-845 at 0e4f1f8): client 1,421 against
  about 700, room 1,956 against about 950 (each about 2.0 times); bots 702 against 650 (5 of 11 forgers in), lab 1,453
  against 1,400 (every suite but `forged`), the run scripts 448 against 600. In all, 6,213 against 5,150 (its range ran
  4,200-6,200), with about 80 % of the planned items built. Tests are 3,342 of the 6,213 lines (53.8 %: my read-only
  count, lines in tests/ folders plus each file's lines from its first `#[cfg(test)]`), so the template's "about 60 %"
  becomes "a little over half".
- **Why (a) still wins with these numbers:**
  - The edge is clean. -47, -20 and -22 are whole; -21's room and sim side is whole (it is U4f's own scope by the plan:
    "every other room-side check with its -21 counter", u4-build-plan.md:806-807); three of -21's four suites are whole
    with their tests; the forged suite has npt-bots' forgers for kinds 1, 2, 4, 5 and 6 and nothing in the lab.
  - The rest is the most intricate part: 11 kinds' attribution (2.4 item 6), the twins with the skip (2.12), f16's 24
    cases, f33's 13. Estimates for the lab held at about 1.0 times so far, but client and room ran 2 times. A unit of
    its own, with its own size line, contains an overrun there. It is also the hostile-input core: the
    security-auditor's main subject gets a review sized to it.
  - -20's, -47's and -22's pre-runs and sets (about 1.5 to 2 hours of runs by 5.5) can run while the forged suite
    is being built.
  - Cost of (a): one more panel and one more merge into the lane, and the built -21 pre-run rows run twice (below):
    about 20 minutes and 140 MiB.
- **What (a) means now that -21 is partly built:**
  - **U4f closes and merges with** the proven tree (74ae26aa, 6,213 added), with R3 (iii)'s hand runs before the
    panel, then the panel's one round (two batches, brief 12 item 8), its one fix pass and the Lows that ride, then
    the merge. The built -21 parts (the clamp, lag-switch and fast-clock suites whole; npt-bots' five forgers with
    f15's five cases) **stay in U4f's merge and U4f's panel reviews them.** Cutting them out would make a new,
    unproven tree: a re-cut through scenario.rs and bots lib.rs, which -20, -22 and -47 share, and two new end checks.
    The new unit would then add the same lines back. That is work with no evidence gained, and each line still gets
    one review. U4f's report names the five forgers as built ahead for the new unit's suite, which is their consumer.
  - **The new unit takes:** the forgers of kinds 3, 7, 8, 9, 10 and 11 with f15's six cases; `--suite forged` (11
    forger slots, 37 honest open-loop clients) and `--forge-every` with its header param and row; the attribution by
    2.4 item 6 and `event.forged_kind.*`; the twins and the twin's skip in the lab (kind 10's and kind 3's rows); f16
    (24), f22 (2), f23 and f33 (13); 5.3's p1, p7, p14 and p17; -21's set in c-u4f-runs.mjs; f28's clamp and
    lag-switch B halves (R3 (ii)); its own end checks (section 6, items 1 to 5). Its brief is short (TEST-FIRST rule
    3): what it builds and tests, pointing to U4f's brief sections (2.4 item 6, 2.6, 2.7, 2.10, 2.12, 5.1 to 5.4),
    to advisor-u4f-build1.md's 22 lines and to this ruling. It goes to me before its build. Its size line follows his
    SIZE-CHECK reading: its plan entry states its size, and the card writes it, with its source (builder 3's estimate),
    before its first snapshot. Its reviewers are U4f's (netcode-architect, security-auditor, test-engineer,
    rust-reviewer).
  - **-21's runs:** none count before the new unit merges. After U4f's merge, c-u4f-r2.mjs runs whole as built: it
    has no row choice (its main loops over RUNS), so its -20 and -47 rows (p5, p6, p10-p12, p21) serve those sets,
    while its built -21 rows (p2-p4, p8, p9, p13, p15, p16, p18-p20) are an early read of the built suites and not
    -21's record. After the new unit merges, c-u4f-r2.mjs runs whole again from that commit (all 21 rows: 5.3's
    record rule, "from the merge commit"), then -21's set: 24 measured, 12 twins and 13 bites in two judge calls (5.4).
  - **The counts:** 2.10's minimums (+119 measured, +134 bites, 67 negative cases) are for U4f's whole scope. Under
    (a) each report states its own share, and the new unit's report states the sum against them. Today the counts are
    +78 and +95 (A2 and B). U4f's not-met carries for -21 (86's forged rows among them) move to the new unit by number.
  - **The plan's text** (the main session's, after his yes): U4f's entry moves -47, -20 and -22 and builds -21's room
    side and three suites; the new entry (it depends on U4f) moves -21's eight parts with their twins and 13 bites.
- **What (b) means:** a fourth builder takes the rest by the same 22 lines (advisor-u4f-build1.md:162-246) and this
  ruling, f28's two B halves and R3 (iii)'s hand runs included. U4f then holds about 7,600-7,800 added lines (builder
  3's estimate: bots about 250, lab about 950-1,100, scripts about 160, f28's halves about 40). End checks run twice,
  one panel reviews the whole, and one merge follows; then 5.3's 21 pre-runs and the sets -47, -20, -22, -21. A new
  stop past 8,000 is my proposal, inside (b) for his yes: the estimate's top plus a margin, since U4f's estimates ran
  1.2 times overall.

## R2: the event cap (U4F-Q12)
- **The figures** (.gate/u4f/w2-cap/cap.txt, read whole): 9 runs (SS, 48 clients, the brawl under the controller,
  option B, P2, P3 and B1 with two bursts each, seeds 1-3, the cap opened to 8,192), every run exit 0 with 0 overflow
  resyncs. Per-client high-water marks: P2 at most 188; P3 689 / 1,001, 683 / 449 and **1,045** / 935; B1 at most 747,
  bursts 615-727. 1,045 x 1.5 = 1,567.5, rounded up to a multiple of 64, is **1,600**. Carry 50's bound is the lowest
  client's mean queued rate x 255 / 30 = 509.2 x 8.5 = **4,328**. 1,600 lies below it, so a never-acker reaches the
  cap before its events expire. This is U4F-Q12 (b) exactly as ruled (advisor-u4f-brief.md:278-282), measured as 2.8
  item 3 asks.
- **P0 needs no run.** At P0 every acknowledgement returns within a step (2.6), so an honest queue empties at each
  step and stays under P2's 188. The never-acker's side is shown at P0 by builder 2's hand run: hand-s22 at cap 1,600
  had 1 overflow resync in 6 s (w2-hand.log:32), and the judge's two -22 checks passed on it (notes :605-606).
- **The 10-minute window is the real gap, and -20's set measures it.** At 1,024, BRAWL-1's 10-minute P3 set overflowed
  in two seeds of three, at room-wide rate peaks (P32's copy, brief :1946-1950), while 60 s runs already reach 1,045.
  -20's judged set is six 10-minute P3 runs (both options) on the merge commit, and P3 is the profile that set the
  maximum. Every run writes `second.s.event_queue_max_count` and each client's `overflow_resyncs_count`. It also
  settles both order notes. B3's runs were made on a tree holding B4's clock code (inert: behind `--suite clock`),
  and later blocks touched paths every client uses (B5's `outgoing` hook on every step's datagrams; B8f's host
  mapping at the arrival sites). No test pins an honest run's fate log across trees, so the merge commit's own
  10-minute figures are the check, not a re-run of the nine.
- **The condition (rule 2: by this ruling):** -20's set runs before -22's. U4F-Q2 (a)'s order becomes -47, -20, -22,
  then -21's. The runner takes one set per call (c-u4f-runs.mjs:3), so no code changes; its header sentence (:7)
  follows in the fix pass. Before -22's set starts, the main session reads -20's set:
  - an honest client with any overflow resync comes to me before -22's set. Then the need is measured with the cap
    opened at 600 s (c-u4f-cap.mjs's method, P3, seeds 1-3), the value is re-set by (b) under carry 50's bound, and
    -20's set is re-run at it (2.8 item 5: -20, -21 and -22 run at one value);
  - with none, 1,600 stands. -20's set report states the 10-minute high-water beside the 60 s figures, with the margin
    it leaves (1,600 over the largest); a margin under 1.5 is named for U4g, whose B1 judged runs are where the cap
    bites (P32).
- The cap is a design constant, not a bar (marks.json:5227; brief 5.6 B4). Plan §4.7's text (the measure, both bounds
  and the figures) is the main session's commit with U4f's merge (2.8 item 5). room/tests/room.rs's pin at (1,600,
  false) changed with the plan, as its doc line says (notes :444-445): accepted.

## R3: builder 3's notes for the main session
- **(i) The fast client sends no `Ping` (notes :720-725): accepted, Low.** Under the controller, its due rule would
  pace the cheat's commands back toward real time and refuse them. The cheat's point is its stamps, which F1's rule
  sets, not its clock estimate. No -21 check reads a ping series (marks.json: no ping series in any -21 part, by grep),
  and 2.6 makes `ping_limited` "expected and reported for this client, never a failure" (SA3-7). 5.3's report line for
  the fast clock reads "`ping_limited` 0 by design: a fixed-lead endpoint sends no `Ping`", not "above 0". U4f's report
  names the deviation from 2.6 for the security-auditor (SA3-7 was its finding) and the netcode-architect. The server's
  ping bucket keeps its own pump tests. Under (a) and (b) alike: no build.
- **(ii) f28's clamp and lag-switch B halves (notes :861-862): Low.** Under (a) they go to the new unit, as its first
  block: no -21 run counts before that unit merges, and U4f's after-merge c-u4f-r2.mjs run calls `contract` on p15
  and p18 (clamp B, lag switch B), so a name failure would show there first, as a result row. U4f's report names the
  move against line 6. Under (b) they go in the continuation's first block (inside builder 3's estimate).
- **(iii) Builder 2's second order note: settled for the counted check, not for the hand runs.** B6's counted check
  ran on B1-B7 (and B8's lag switch: w2-b8a at 17:03 came before the hand runs at 17:09-17:14). Its job (line 20) was
  to show a test-step problem early. A2 and B ran on the final tree (all 11 steps pass; test step 692.1 s and 685.6 s
  against 1,207 s) and supersede every earlier counted check. Section 6 item 5 is different: builder 3 added c-u4f-r2.mjs's
  FAST rows p3, p9, p16 and p20 and ran none of their command lines by hand (no w3 hand log under .gate/u4f/; notes
  :741-765). f19's lab test parses `--suite fast-clock --fast-clock-episode-s 2` under both options through the lab's
  own settings parser (scenario.rs:4091-4097), so most of the line is covered. Not covered: `--fast-clock-idle-s` (p20),
  the bites build with `--bite PM-0001-21/not-older-than` on this suite (p9), and 48 clients. **Fix (Low, before the
  panel, both options):** the four command lines run by hand on a short setting, `-x -k` through the runner, outputs
  under .gate/u4f/, as builder 2's w2-hand.log did. This is verification, which adds no line (R4 of
  advisor-u4f-build1.md).
- **(iv) The usage line lacks `--lag-hold-ms` and `--lag-every-ms`: Low.** USAGE (scenario.rs:1967-1973) omits them.
  The parser takes them (:2122-2125), the scripts pass them, and no run or check reads the usage text. It rides U4f's
  fix pass. They are the only two options the parser takes that USAGE does not name (my compare of its `take("...")`
  names).

## R4: MAIN-52, the take-in of main b763ba3
- **When: now, while his answer is pending, before the panel.** His rule (history.md:2082-2086; TEST-FIRST "taking main
  in when main changes code, CI or rules") is triggered: #58 adds v2 code (crates/, packages/types), and #60 changes CI
  (.github/workflows/toolchain.yml) and rules (.claude/CHECKPOINTS.md's DEP-HOLD, checkpoint.mjs and its tests). The
  lane waits on his answer anyway: no build or run is going, and builder 3 has ended. Main's 49 files touch nothing of
  U4f's (`git diff --name-only 82c605e origin/main -- spikes/ docs/design/` is empty), so the patch applies unchanged.
  The panel's one round then lands on the lane as it will merge (R4 of advisor-u4f-build1.md), and PR #40's CI runs
  main's new toolchain job before U4f's merge rather than after it. Taking main in after the panel would move the
  reviewed head and need my head review again.
- **How: plan first, as MAIN-54** (main-54-plan.md; the conditions C1-C7 at history.md:2533-2541), to me before it
  runs. What differs this time, for that plan to settle:
  - a hand-resolved conflict in docs/STATUS.md, the only one (`git merge-tree --write-tree HEAD origin/main`: CONFLICT
    in docs/STATUS.md alone; the card expects :28-29). Keep both rows, by Edit, and name the commit form the guard
    allows: C3's rule, git's own message;
  - #60 changes checkpoint.mjs, which the saver runs: stop the saver first (C1), run `node --test
    .claude/scripts/checkpoint.test.mjs` after the merge with main's count, then restart the saver;
  - DEP-HOLD: the take-in brings main's Cargo.toml, Cargo.lock, crates/protocol/Cargo.toml and
    packages/types/package.json, each equal to main's, so they are not held (CHECKPOINTS.md's new lines). U4f's patch
    has no manifest or lockfile among its 29 files, so U4f's merge is not held either;
  - the light run and the gate --no-smoke as C4 and C5; after the push, CI's toolchain job now also builds and tests
    main's root workspace and checks packages/types. Expect it green, as on main.
- **The order:** the take-in, the push, CI green, my head check; then the four hand runs (R3 (iii)); his answer; under
  (a) the panel on U4f's tree, or under (b) the fourth builder; U4f's snapshot applied onto the lane's new head with
  `git apply --check` first (its base 07289a4 equals 5b29296b in spikes/ and docs/, which the take-in leaves alone).

## Choices to name in the day's message (rule 2; each undoable before the merge)
- The event cap is 1,600, no longer provisional (U4F-Q12 (b): the honest high-water mark 1,045 x 1.5, under the
  never-acker's bound 4,328); -20's set runs before -22's and is read for honest overflow first.
- The fast-clock cheat sends no pings (a fixed-lead endpoint): its ping count is reported as 0 by design.
- Main b763ba3 is taken in now, before U4f's review (MAIN-52), plan first.

## Findings, each with its severity (CLAUDE.md's scale)
| Id | Severity | What | Fix |
|---|---|---|---|
| S1 | Low | the fast client sends no `Ping`, against 2.6's "its pings come at its steps" | R3 (i): recorded deviation; 5.3's report line |
| S2 | Low | f28's clamp and lag-switch B halves not built (line 6) | R3 (ii): the new unit under (a), the continuation under (b) |
| S3 | Low | section 6 item 5: the fast-clock rows p3, p9, p16, p20 not run by hand | R3 (iii): before the panel, no line |
| S4 | Low | USAGE lacks `--lag-hold-ms` and `--lag-every-ms` (scenario.rs:1967-1973) | rides the fix pass |
| S5 | Low | SERIES.md's new subsection title says "no check reads them", but its `event.reorder.*` row's Marks cell names the existing `reorder` rule, which reads that event (judge eval.rs:864-881 at 5b29296b: `e.ev == "reorder"`, field `permille`) | the title names the one exception; rides the fix pass (a new row, no existing row touched) |
| S6 | Low | c-u4f-runs.mjs:7 states U4F-Q2 (a)'s order (-47, -22, -20); R2 swaps -20 and -22 | rides the fix pass |
| S7 | Low | the card's cites into u4f-build.md predate 0e4f1f8's 26 added lines (builder 3's end checks, time rule and bars line, now :800-825): its "stop record :801-840" is now :827-866 and "builder 3 :692-841" is :692-866; and its "the advisor confirms the three lines" (card :9-10, 0e4f1f8) was written before this ruling existed | the next card save re-points both cites and names this file |
| N1 | note | hand-s22 (P0, the never-acker) shows "rewinds refused 30", and its bite run 1,018 (w2-hand.log:32, :53); no note says whose they are | the panel or -22's set report says which slot and why; no check reads them |

No Critical, High or Medium. No finding blocks the stop's procedure.

## What I checked, and what I did not
- **Read:** main's advisor.md (`git show origin/main:`); advisor-u4f-build1.md whole; history.md :2040-2086,
  :2230-2282, :2524-2556 and :6785-6806; u4f-build.md :300-866 (as committed at 0e4f1f8, with its 26 new lines); the brief's
  2.6 (:499-547), 2.8 (:590-613), 2.10's minimums (:645-660), SIZE-CHECK (:915-945), 5.1 to 5.5 (:959-1126), 6 and 7
  (:1163-1224), U4F-Q2 (:1361-1387) and U4F-Q12 (:1566-1573), and P32's copy (:1946-1950); advisor-u4f-brief.md
  :275-282; u4-build-plan.md :805-816; the card whole (198 lines); cap.txt whole; w2-hand.log's -22 lines;
  c-u4f-runs.mjs :1-40 and its argument check; c-u4f-r2.mjs :1-66 and its loop; scenario.rs's USAGE, its option
  names and `fast_short`; main's diff since 82c605e (file list; toolchain.yml and CHECKPOINTS.md in full).
- **Read in full, by USAGE-SAVINGS (a):** SERIES.md's whole hunk (20 new rows; no existing row touched; 19 rows marked
  none, the reorder row naming the unchanged rule, checked against the judge's rule at 5b29296b); marks.json is
  unchanged on this branch (`git diff --quiet 5b29296b HEAD`, exit 0), and the patch holds no marks.json, README or
  judge file (its numstat); -22's checks and notes (marks.json:5160-5227), against which I read the cap.
- **Ran (read only):** sha256sum of the patch; `git apply --numstat`; the snapshot rebuilt from builder 3's worktree
  (sha256 equal); the runner's two tree hashes in that worktree (equal to A2's and B's); `git diff --quiet` of 5b29296b
  against 07289a4 in spikes/ and docs/; `git merge-tree --write-tree HEAD origin/main` (writes objects only, no ref or
  file); a read-only Python count of test lines (scratchpad, `-I`); greps of the logs and sources named above.
- **Not checked:** I built, ran and timed nothing. The end checks', the time rule's and the cap's figures are the
  builders' runs, read from their logs (w3-times.log only through the notes' summary). The code's correctness beyond
  the parts named is the panel's. Builder 3's fast-clock figures and its +180 projection for p9 at 60 s are
  arithmetic, shown only by 5.3's p9. Whether the merged tree's honest runs equal B3's binary is left to -20's set
  (R2). I did not read #58's v2 code or #60's checkpoint.mjs change beyond their file lists and the two diffs named,
  did not open CI on PR #40, and did not see the guard's answer to a conflicted merge's commit (the take-in's plan
  settles it).

## Decisions needed
- **SIZE-U4F: U4f passed your 6,000-line stop (6,213 added). Split it, or go on?** Why it matters: it sets how big the
  next review is and when -47, -20 and -22 can be judged. Options: (a) close U4f with what is built (its three finished
  -21 cheat tests kept and reviewed with it), merge, judge -47, -20 and -22, and build -21's forged-command test with
  all of -21's runs as a small unit of its own next (about 1,500 lines); (b) go on to about 7,600-7,800 lines with
  one review and one merge (a new stop past 8,000). No pass mark, judge rule, marks.json text or run count changes
  either way. **Recommendation: (a).**

## For Mustafa
1. In the day's message, read the three lines and answer SIZE-U4F with "(a)" or "(b)" in the chat. Nothing else waits
   for you: the cap, the run order, the fast clock's pings and the take-in of main go by this ruling and are named
   there.
