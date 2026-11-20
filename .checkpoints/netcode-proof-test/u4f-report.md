# U4f report: clock and commands (`ClockSync`, the tick-lead controller, option A and the command checks, the cheats, and the suites for -47, -20, -22 and three of -21's four)

Merged 2026-11-19 into the spike branch (PR #40's lane, `claude/netcode-proof-test-plan-hcky5g`) by the main session,
in one commit (M8: advisor-u4f-r1.md:339-355). The approved brief: u4f-brief.md rev 3.3 (sha256 135325a0…, 2,040
lines, filled with `<base>` 5b29296b: the card, "Now: U4f"). The bytes merged: wip/u4f-build.patch, sha256
286aac60561ba684…, 394,400 B, 29 files (20 changed, 9 new), 6,425 lines added and 310 removed (`git apply --numstat`
of the patch, my run; the advisor's check names that sha256: advisor-u4f-pass.md:17-18). The judged runs come after
this merge (below, "What comes next"). For U4's report, which folds this one.

## In plain words
- **What U4f is:** the clock a test client uses to stamp its commands for a server tick, the controller that picks how
  far ahead it stamps, the server room's second command-timing option (option A) beside U4e's option B, and every
  room check that refuses a forged or out-of-rule command, each with its own counter. With them come the cheating
  test clients (a client whose clock runs 1.5 times too fast, a lag switch, an aim-beyond-reach cheat, a client that
  never acknowledges, five of the eleven command forgers) and the test suites that the marks -47, -20, -22 and three of
  -21's four suites run on.
- **Where it stands:** every check passes on the merged tree: all 11 steps, 634 tests in the measured build and 731 in
  the bites build, 0 failed (.gate/u4f-merge/m5-check.log:451, :458). No pass mark, judge file or marks.json text
  changed. Nothing here is a judged result yet: the short pre-runs and the judged sets for -47, -20 and -22 come next.
- **What it cost:** 6,425 added lines against about 3,000 for a unit of its size and his 6,000 stop; his SIZE-U4F
  answer (a) of 2026-11-19 (history.md:6896-6903) closed U4f as built, and the one review round and its
  fix pass sit inside that answer (advisor-u4f-r1.md:301-305).
- **What moved out:** the forged-command test (six missing forgers, the lab suite, the attribution, the twins) and
  all of -21's judged runs go to a new, small unit, U4f2, which comes next (advisor-u4f-stop.md:80-99).

## The two lines his words ask for
- **A1's movement reading, reviewed by the advisor and the test-engineer:** -21's two movement checks count a soldier's
  movement made by the sim's movement step only; a tick that begins or ends with the soldier dead, or whose respawn
  moves it, adds no movement and is counted apart, and the fast clock's honest reference sums the same live ticks. It
  is the advisor's A1 (advisor-u4f-brief.md:69-90; u4f-brief.md:540-546, 2.6's Movement paragraph), read in the
  LEAD-BOUND entry by the advisor (advisor-u4f-lead-bound.md) and the test-engineer (test-engineer-u4f-lead-bound.md),
  both SEND WITH EDITS, the edits applied (u4f-lead-bound.md:3-6, :38-42); his yes: history.md:6791-6806. Built as
  `logs::moved_um` (lab/src/logs.rs:200; `MOVE_CM_S` :196) with `tick.v.lagswitch_move_apart` and
  `event.fast_clock.move_apart_count` (SERIES.md:383, :390); tested by f19 and f20, each with a death and a respawn on
  hand-made records
  (lab/src/scenario.rs:4130-4134, :4045-4049), and U4F-Q11 (iii) (c)'s lane test, which walks every live tick of 610 s
  (bots/src/lib.rs:992; u4f-build.md:750-753); FP14's walk test pins `MOVE_CM_S` against the sim's own walk
  (lab/src/scenario.rs:4243; .gate/u4f/w4-fp14b.log). It joins the readings PR's list (the card, "Open findings").
- **D10:** no marks.json or judge change; nothing for his 13:35 read.
  - The evidence: `git diff --stat 5b29296b -- spikes/netcode-proof/marks/marks.json spikes/netcode-proof/crates/judge
    spikes/netcode-proof/marks/README.md` prints nothing on the merged files (my run, before the commit; the card's M7
    line). The commit carries exactly those files, so `5b29296b..<merge>` reads the same; the main session reads it once
    more on the commit. SERIES.md: 20 lines added, 0 removed (`git diff --numstat 5b29296b`, my run), every one in U4f's
    own new subsection (below), no existing row touched. His 13:35 rule: history.md:2125, :2127.

## Who built it, and how
- **The brief:** rev 2 (34a2e55), rev 3 (e1508bc), round 1 (three Highs), rev 3.1 (9f76a02), round 2 (seven Mediums),
  rev 3.2 (8ca5ede); the advisor's APPROVE WITH CONDITIONS L1-L6 (advisor-u4f-brief.md, sha256 bfeb090c…); rev 3.3
  by the fold, its check ACCEPT (advisor-u4f-fold-check.md, 833e93ba…); LEAD-BOUND (a), 400 ms, frozen, his yes at
  history.md:6791-6806 (the card, "Now: U4f").
- **The build:** four builders in turn (general-purpose, opus, effort xhigh), each in its own worktree on the shared
  target, every cargo call through u4f-run.sh, notes in u4f-build.md:
  - builder 1, 2026-11-18, ended 13:13 UTC: npt-room (option A, the order and checks, 9 bites), the Pong's server time
    (npt-proto, the catalogue, npt-server's tests), npt-client (ClockSync, the controller, carry 80), npt-sim's two
    bite sites; 3,753 added; end checks 1 and 2 pass, 605 / 701 (u4f-build.md:1-308, :204, :215, :232-235);
  - builder 2, ended 17:59 UTC: build ruling 1's fixes, -20's rest, the cap's measurement, -47's clock suite, -22's
    never-acker, the run scripts, the clamp and lag-switch suites; 5,515 added; end checks A and B pass, 619 / 716
    (:310-690, :515-531);
  - builder 3, 18:10 to 22:59 UTC: the fast clock and B9a (forgers of kinds 1, 2, 4, 5, 6), then the size stop at
    6,213 added; end checks A2 and B pass, 630 / 727 (:692-866, :800-814; history.md:6906-6919);
  - builder 4, 2026-11-19 14:57 to 18:10 UTC: the round-1 fix pass FP1-FP19; 6,425 added; end checks A2 and B pass,
    634 / 731 (u4f-build.md:868-1170, :1105-1125).
- **Review round 1** (wf_a011d9f2-abf, 2026-11-19 13:50-14:32 UTC, on builder 3's worktree = 74ae26aa…): five seats
  (netcode-architect, security-auditor, test-engineer, rust-reviewer, engine-architect), all APPROVE WITH FINDINGS:
  4 Medium (NC-U4F-1, TE-U4F-1, TE-U4F-2, RR-U4F-1), 25 Low, no Critical or High (u4f-review-r1-full.md, sha256
  6747175e…; the card, "Now: U4f").
- **The advisor's round-1 ruling** (advisor-u4f-r1.md, 07c1bcd5…): APPROVE WITH CONDITIONS, one fix pass FP1-FP19, no
  round 2 by default (the 2026-11-17 choice, history.md:6818-6819), nothing to him (:11-29, :89-98).
- **The advisor's check of the pass** (advisor-u4f-pass.md, 580698d2…; it replaces round 2): APPROVE WITH CONDITIONS
  C1-C3; every FP item as named, no departure; four Lows ADV-U4FP-1 to -4; nothing to him (:7-39, :80-114).

## What the rulings settled
- **The brief's ruling** (advisor-u4f-brief.md): all 18 questions ruled, none escalated; three Medium (A1-A3) and four
  Low (A4-A7) folded into rev 3.3; LEAD-BOUND to him for his yes (:9-29).
- **The fold check** (advisor-u4f-fold-check.md): ACCEPT with 20 edits; D3-1 ruled (a `reorder` event at a run's end,
  SERIES.md's `event.reorder.*` row as B9, test f40) (:9-14; the card, "Now: U4f").
- **Build ruling 1** (advisor-u4f-build1.md, da98295e…, on builder 1's tree): APPROVE the continuation; R2 (g) the
  lead bound counts from the tick the room runs next (the frozen value and meaning hold); F1 the fast clock's every
  10th stamp at newest + 3 + the bound; F2 `World::CHECK_BITES` routed in `Bites::of`; F3 f28 runs the contract
  checker; F4 `--brawl-press-every` as its own setting and row; R1 f6 and f28 one test per option; the late check after
  kinds 4-6 and before 10 approved (R2 (b), :101); the 22 instruction lines for builders 2-4 (:13-36, :162-246).
- **The stop ruling** (advisor-u4f-stop.md, 38821ebd…): R1 the three lines, his answer (a); R2 the event cap 1,600, no
  longer provisional, with -20's set before -22's; R3 the fast clock's no pings a recorded deviation, f28's two B halves
  to U4f2, the four fast-clock hand runs before the panel, USAGE's two options in the fix pass; R4 MAIN-52 then (done as
  MAIN-60); Lows S1-S7 and note N1 (:10-23, :198-210).
- **Round 1** (advisor-u4f-r1.md): the four Mediums fixed in the pass, each failing first where its finding said
  (FP1, FP2, FP3, FP4); TE-U4F-1's 3 % dropped, f5 measured as the brief writes it; -22's set under U4f's controller
  (NC-U4F-2); both security findings fixed in U4f, the parser half carried to U4f2; five carries into U4f2's brief and
  five into later units (:12-29, :65-87, :137-154). Every finding's route is in its own section below.
- **The pass check** (advisor-u4f-pass.md): the pass's whole diff is 10 files, 330 insertions and 121 deletions, every
  hunk an FP item; each fails-first and plant log fails where ruled; the snapshot is the final tree byte for byte
  (:9-22).

## The deviations accepted
Each accepted as built; nothing re-run for it.
- **S1, the fast client sends no `Ping`** (against 2.6's "its pings come at its steps"): the fast client is a
  fixed-lead endpoint, so no controller's due rule paces its commands back to real time; no -21 check reads a ping
  series. Accepted by the stop ruling R3 (i) (advisor-u4f-stop.md:140-146), which has this report name it for the
  security-auditor (SA3-7 was its finding) and the netcode-architect; both confirmed it in round 1
  (u4f-review-r1-full.md:84, :47). 5.3's report
  line for the fast clock reads "`ping_limited` 0 by design: a fixed-lead endpoint sends no `Ping`", not "above 0".
- **`ClientEndpoint::with_quant`** (unfeatured, beyond 2.13's list): the identity outside bites builds; it lets
  npt-server's tests pass npt-proto's planted `Quant`, since npt-server's bites feature does not turn on npt-client's
  `with_bite` (P33's second error, E0599 behind the named E0425: u4f-build.md:91-96). Consumer:
  server/tests/loopback.rs:259. Accepted by build ruling 1, R2 (e) (advisor-u4f-build1.md:113) and the arch seat
  (u4f-review-r1-full.md:262, :317).
- **`World::CHECK_BITES`** (bites builds only; 2.13 said "two ids in `World::BITES`"): a list apart, since
  npt-hash-wasm's replayer indexes `World::BITES` and npt-sim's tests pin its four (sim/src/world.rs:945-951;
  u4f-build.md:128-132).
  Accepted by build ruling 1, R2 (d) (advisor-u4f-build1.md:111); routed in `Bites::of` (lab/src/scenario.rs:261, F2);
  its two ids pinned by literal in f2's test (FP17, scenario.rs:4209).
- **EA-U4F-3's inert lines** (accepted as built: advisor-u4f-r1.md:125): five pieces of the -21 check bites' control
  flow in `World::step`'s authority shot phase carry no `#[cfg(feature = "bites")]` (sim/src/world.rs in the merged
  tree): `let mut ready` with its `unused_mut` allow (:1429-1433), `let mut refuse_far = false` with its allow
  (:1434-1438), the refuse_far refusal block (:1452-1455), `let ready = ready && !refuse_far;` (:1456) and
  `&& !refuse_far` in the fire-edge test (:1457). Outside bites builds `refuse_far` is always false, so the measured
  build's behaviour is unchanged (the wasm step, 27,000 ticks and 0 mismatching both ways, in every end check and M5:
  m5-check.log:453). **Correction to the record:** the build notes' "bites builds only" (u4f-build.md:42, :132, :239)
  is accurate for the two bites' effects and `World::CHECK_BITES`, not for the sites: those five lines compile in every
  build and are inert there.
- **Other recorded order notes, each ruled or harmless:** the late check placed after kinds 4-6 (approved, build ruling
  1 R2 (b)); B3's cap runs on a tree holding B4's clock code, and B6's checks on B1-B7 (inert outside their suites;
  the stop ruling R2 makes -20's 10-minute set the check: advisor-u4f-stop.md:118-125; u4f-build.md:453-455, :620-622).

## Checks on the merged tree (M1-M9, advisor-u4f-r1.md:325-358, with C1-C3)
- **M1:** wip/u4f-build.patch sha256 286aac60561ba68409a082beb40babbd82706f00863ccef25b4f557200268519, 394,400 B
  (`sha256sum`, `wc -c`, my runs), the sha256 the pass check names (advisor-u4f-pass.md:18); unchanged since 549ed77
  (`git diff --quiet 549ed77 HEAD` on it: exit 0, my run).
- **M2:** spikes/netcode-proof and docs/design at the lane's head equal 07289a4's and b25b96a's (`git diff --quiet`,
  exit 0 both, my run at 9eb4abe); every commit since 549ed77 touches .checkpoints/ only (`git log 549ed77..HEAD --
  spikes docs .claude .github`: empty, my run). Main: f5030a3, already in at a6acc8a (MAIN-61; the local origin/main is
  an ancestor of HEAD, my run). The main session re-read main at 19:1x UTC, after the apply and before the commit
  (`git fetch origin main`: unmoved, f5030a3).
- **M3 and M4:** `git apply --check` exit 0, then `git apply` (the card, "Running work"); the 29 files byte-equal to
  builder 4's worktree (the main session's cmp_files.py: the card; my own `cmp` of the 29: 0 differ); the runner's two
  hashes from spikes/netcode-proof, diff 2f07abc2… and untracked 1a767d5b… (m5-check.log:3), equal to end check B's
  (u4f-build.md:1156-1157) and the advisor's recompute (advisor-u4f-pass.md:21-22).
- **C1, disk:** 4,360 MiB free at the part-clean's start (.gate/u4f-merge/clean-shared.log:2), under C1's 4,600, so
  P27's part-clean ran first, 18:27:04-18:27:10 UTC, exit 0, no lane running (that log); 11,662 MiB free at M5's start
  (m5-check.log:2).
- **M5:** one counted call, .gate/u4f-merge/m5-check.log, 18:27:42-18:46:38 UTC, lane lock exclusive (:1), HEAD
  d4f3f9a, the seven commands of the end checks (the two `--no-run` test builds, the two release npt-lab builds, the two
  wasm32 builds, `node check.mjs`), each exit 0 (:149-459); C2's limit met (7,200,000 ms: the card).
  - "check: all 11 steps pass" (:458). fmt 0.9 s; clippy 37.6 s; clippy-wasm32 6.2 s; doc 6.2 s; **test 865.9 s:
    measured build 634 passed, bites build 731, 0 failed, 0 reported a skip**; deps 1.6 s (143 crate files); wasm 50.5 s
    (27,000 ticks, 0 mismatching both ways, h2 equal; hash-map caught at SS-s1 tick 1234, std-sin at SS-s1 tick 2);
    refs 1,569 tracked files; marks 81 read and checked, the ADR rows unchanged; self 78 of 78; redact 10 of 10, 316
    items, 0 findings (:447-457).
  - Provenance (:460-477): every npt-* crate "Compiling … (/home/user/Shamble-Frontlines/spikes/netcode-proof/
    crates/…)"; npt-room, npt-client, npt-bots and npt-lab 4 times each, npt-sim 6.
  - The counts as deltas on 5b29296b's 552 and 632 (u4f-build.md:19-21): +82 and +99, as in builder 4's end checks A2
    and B (u4f-build.md:1108, :1121).
- **M6:** no test failed; nothing was re-run.
- **M7:** `node tools/gate.mjs --no-smoke` PASS (the main session's run; its output kept at .gate/u4f-merge/gate.txt,
  copied from the session's tool result, as ADV-F1H-2's lesson asks). CI's gate and toolchain jobs run on the pushed
  head (M9).
- **D10:** its own line above.
- **DEP-HOLD:** no Cargo.toml, Cargo.lock, package*.json or .npmrc among the 29 files (my grep of the numstat), and the
  9 new files are mode 100644 (the patch's 9 `new file mode 100644` lines; advisor-u4f-pass.md:127).
- **M8:** the merge commit holds the 29 files as applied and `git rm` of wip/u4f-build.patch (WIP-CLEAN). The papers
  are .checkpoints/ files, which the checkpoint script commits on its own, so they went in by its saves around the merge
  commit: u4-build-plan.md's §4.7 cap text (2.8 item 5), its §5 U4f entry and the new U4f2 entry, its §4.1 rows, and
  plan.md's §12.1 rows with the five later carries (fca16a9, by apply_u4f_merge_edits.py from u4f-merge-edits.md, word
  for word); u4f-build.md's merge section and this report (the save that pushes the merge commit).
- **M9 (after the push):** the saver restarted, CI green, the card updated; then "What comes next". The merge into the
  lane needs no go of his (his (a), as U4r's: history.md:4659-4662); PR #40 keeps its own go.

## SERIES.md: U4f's new subsection (SERIES.md:372-391)
20 lines added, 0 removed against 5b29296b: a heading, two blank lines, the table's two head lines and 15 rows. 14
rows' Marks cells read "none (recorded)" or "none (reported)"; the `event.reorder.*` row names the existing `reorder`
rule, unchanged (judge eval.rs:864-881 at 5b29296b: advisor-u4f-stop.md:205). Read in full by the test seat
(u4f-review-r1-full.md:171) and the advisor (advisor-u4f-pass.md:15-16).

| Line | Name(s) | Written by | Marks |
|---|---|---|---|
| :376 | `header.params.lead_bound_ms` (ms) | npt-lab (from the room's `lead_bound()`) | none (recorded) |
| :377 | `header.params.connects_count` | npt-lab | none (recorded) |
| :378 | `header.params.brawl_press_every_count` | npt-lab | none (recorded) |
| :379 | `header.params.controller_max_lead_count` | npt-lab | none (recorded) |
| :380 | `event.client_counts.lead_refused_count` and the other room and endpoint counts | npt-lab (npt-room's and npt-client's counters) | none (reported) |
| :381 | `event.client_counts.pump_*_count` | npt-lab (npt-server's `Counters`) | none (reported) |
| :382 | `event.session_end.*` | npt-lab (`ClientCounters`, `Counters`) | none (reported) |
| :383 | `tick.v.lagswitch_move_apart` (flag) | npt-lab (npt-sim's Soldier records) | none (reported) |
| :384 | `header.params.lag_hold_ns`, `.lag_every_ns` | npt-lab | none (recorded) |
| :385 | `event.clamp_kind.rerun_reports_count` | npt-lab (npt-sim's `reports()`) | none (reported) |
| :386 | `tick.s.cmd_ran_early_count` (signed) | npt-lab (`ClientSlot::ran_early`) | none (reported) |
| :387 | `second.s.event_queue_max_count` | npt-lab (the cap less `ClientSlot::headroom`) | none (reported) |
| :388 | `event.reorder.permille`, `.up_permille`, `.down_permille` | npt-lab (npt-l1's fate log) | the `reorder` rule (no change to it) |
| :389 | `header.params.fast_clock_episode_ns`, `.fast_clock_idle_ns` | npt-lab | none (recorded) |
| :390 | `event.fast_clock.lead_refused_count`, `.not_newer_count`, `.left_out_count`, `.move_apart_count` | npt-lab (npt-room's counters, `Behaviour::fast_left_out`, Soldier records) | none (reported) |

**The pass's cell edits** (all inside that subsection; the reviewed tree's lines against the merged ones, my diff of the
two patches): FP9 (S5) the heading at :372 now names the `event.reorder.*` row as the one an existing rule reads; FP7
:376's "Written by" cell, from `Room::LEAD_BOUND_MS` to the room's `lead_bound()` (its meaning cell, with the frozen
sentence, unchanged: advisor-u4f-pass.md:53); FP12 :390 gains `.left_out_count`, its source (`Behaviour::fast_left_out`)
and its meaning. The lead-bound row's meaning cell is untouched (advisor-u4f-build1.md line 8; advisor-u4f-r1.md:93-94).

## What U4f built, by mark (u4f-build.md:847-854, with the pass)
- **-47:** `ClockSync` (client/src/clock.rs), the `Pong`'s server time (`Pong::server_ns`, the catalogue's row), the
  clock suite (`--suite clock --connects`), its two bites, f1-f5, f29, f31; 5.3's p10, p11, p21.
- **-20:** option A beside option B (`CmdTiming`), the controller (`with_clock(9)`), `--brawl-press-every`, every -20
  series, f6, f28's -20 halves, f34, f36, f40, the redundancy-1 bite; p5, p6, p12.
- **-22:** the never-acker, its suite and series, 2.5's counts at every session end, f14, f21, f35, the event cap
  measured at 1,600 (U4F-Q12 (b), the stop ruling R2).
- **-21's room and sim side, whole:** kinds 1-6, 8 and 10 with their counters and records, option A's run path, the 13
  room and sim bites' unit proofs, f7-f13, f17, f38, f39.
- **-21's three suites, whole:** clamp (f18; p2, p8, p15), lag switch (f20; p4, p13, p18, p19), fast clock (f19, f37,
  U4F-Q11 (iii) (c)'s test, f28 A and B; p3, p9, p16, p20).
- **Built ahead for U4f2:** below.

## Built ahead for U4f2, and what moved there
- **Built ahead, no consumer in U4f** (C-U4F2-2, advisor-u4f-r1.md:76-80): `ClientEndpoint::skip_command` (the twin's
  skip; its own unit test is U4f2's) and `twin_left_out` (its doc names both causes, the twin's skip and the codec's
  pair rule after FP1; f25 (ii) reads it, carry80.rs:123); `Behaviour::forger` and `take_forged`, with the five
  forgers of kinds 1, 2, 4, 5 and 6 and f15's five cases plus FP5's interval case (bots/src/lib.rs:310, :322,
  :1143-1170); `Room::refusals_dropped` (room/src/room.rs:637). The stop ruling names the five forgers as built ahead
  for U4f2's suite, their consumer (advisor-u4f-stop.md:72-79).
- **Moved to U4f2** (advisor-u4f-stop.md:80-89; his (a), history.md:6896-6903): the forgers of kinds 3, 7, 8, 9, 10
  and 11 with f15's six cases; `--suite forged` and `--forge-every`; the attribution (2.4 item 6) and
  `event.forged_kind.*`; the twins and the twin's skip in the lab; f16 (24), f22 (2), f23 and f33 (13); 5.3's p1, p7,
  p14 and p17; -21's set in c-u4f-runs.mjs; its own end checks.
- **f28's two B halves, moved** (the stop ruling R3 (ii), advisor-u4f-stop.md:147-150): f28's clamp and lag-switch
  halves under option B were not built (builder 2 folded each A half into f18 (ii) and f20; build ruling 1's line 6
  asks one per suite and option: u4f-build.md:860-862). They go to U4f2 as its first block. No -21 run counts before
  U4f2 merges, and U4f's after-merge c-u4f-r2.mjs run calls `contract` on p15 and p18 (clamp B, lag switch B), so a
  name failure would show there first, as a result row. Named against line 6 here, as R3 (ii) asks.

## U4f's share of 2.10's minimums (C3; advisor-u4f-r1.md:290-292)
- **The share:** 2.10's minimums less U4f2's (advisor-u4f-stop.md:95-97; the test seat's arithmetic,
  u4f-review-r1-full.md:169): measured +73 (119 less U4f2's 46 tests: f15 x6, f16 24, f22 2, f23 1, f33 13), bites
  build +88 (134 less 46), negative cases 45 (67 less U4f2's 22: f15 x6, f22 2, f23 1, f33 13).
- **As merged:** **measured +82** (634 against 552) and **bites +99** (731 against 632) (m5-check.log:451;
  u4f-build.md:1125). Builder 3's +78 and +95, then the pass's +4 in both builds: f25 (ii), U4F-Q9 (a)'s flood test
  (`q9_a_…`, room/src/input.rs:568), f15's interval case (`f15_a_forgers_interval_below_2_is_refused`,
  bots/src/lib.rs:1170) and EA-U4F-6's walk test (`ea_u4f_6_…`, lab/src/scenario.rs:4243), each with its `--list`
  line (.gate/u4f/w4-list.log; u4f-build.md:1126, :1132-1136).
- **Negative cases: 46 against 45, by row** (u4f-build.md:1127-1131): f3 2, f4 1, f7 1, f8 3, f10 8, f11 2, f12 5,
  f13 4, f14 3, f15 5, f19 1, f20 1, f21 1, f24 1, f25 2 (f25 and FP1's f25 (ii)), f26 1, f27 1, f35 1, f39 1,
  U4F-Q6 (b)'s Pong field 1 (server/tests/pump.rs), U4F-Q9 (a) 1 (FP4's flood test). Not counted: f15's interval case
  (the forger's own parameter, no hostile input to the room).
  - **f4's negative half is U4e's e9** (server/tests/pump.rs:193-212, the pump limiting and counting a faster ping
    rate), named in f4's doc (client/tests/trace.rs:401-403; FP18, TE-U4F-5). f4 itself feeds no hostile input, and
    f31 pins the ping gap per step (client/src/clock.rs:337). So f4's 1 rests on an existing U4e test; without it the
    new negative cases are 45, still the share.

## Public items, re-derived from the merged tree (FP20, EA-U4F-2; 2.13; A5's rule, advisor-u4e-r1.md:158-167)
From `git diff HEAD -U0 -- crates/*/src` (every added or changed `pub` line) and the new files, with each item's
consumer from a grep over spikes/netcode-proof/crates and ops/runs (my runs). Against plan.md §4.7 (:569-634) and
u4-build-plan.md §4.1 (:178-203); the §4.1 rows go in with this commit.
- **npt-client:**
  - `ClockSync` (plan.md §4.7:610; 2.1), re-exported with `ClockSample` (its five pub fields `offset_ns`, `rtt_ns`,
    `at_ns`, `tick`, `server_ns`; what `samples()` returns). ClockSync's pub methods: `offset_ns` (the lab's -47 read;
    f1), `samples` (the lab's first-sample instant, scenario.rs:1552), `resets` (f32 (ii), trace.rs:479, :489);
    `new`, `period_ns`, `unknown_pongs`, `next_tick_at`, `one_way_hi_ns` and `ping_every_ns`: no consumer outside
    client/src (EA-U4F-1), each made crate-private, removed or given a consumer by **C-U7-4**, `ping_every_ns` removed.
  - `ClientEndpoint::with_clock` (2.2, U4F-Q7: the controller's setting; the lab and f36), `clock` (2.13: the clock
    estimate for -47), `commands_due` (2.2; npt-bots' `controlled`), `stamps_skipped` (2.2; the lab's
    `stamps_skipped_count`), `unencodable_commands` (2.9 (c); the lab), `lead_now` (beyond 2.13's list, named by
    builder 2: the lab's `lead_now_count`, scenario.rs:1726), `not_due` (beyond 2.13: f36 (ii) asserts it 0,
    bots/src/lib.rs:922, FP12), `unknown_pongs` (pub(crate) at U4e, now public: the lab's `unknown_pongs_count`,
    scenario.rs:1727), `with_quant` (beyond 2.13: the deviation above).
  - **Built ahead for U4f2:** `skip_command`, `twin_left_out` (C-U4F2-2).
  - `BITES` 3 to 6 ids, bites builds only (2.11).
  - Removed in the pass: `join_tick` (FP12; 2.13's "no new public item"; no caller left: advisor-u4f-pass.md:58).
- **npt-room:**
  - `CmdTiming` (2.3), `Refusal` with `kind()` (the refusal records' kind, U4F-Q9; to be renamed `CommandRefusal` with
    the sim's struct form by **C-U4F2-3**), re-exported from lib.rs.
  - `Room::LEAD_BOUND_MS` (400, frozen; FP11's const assert reads it, scenario.rs:61-64), `with_cmd_timing` (2.3; the
    room tests), `set_cmd_timing` (beyond 2.13: the in-place form for the lab, scenario.rs:1165; it debug-asserts no
    slot has joined), `with_lead_bound_ms` (U4F-Q8 (i) (b), the bound as room data; one caller, a test,
    room/tests/commands.rs:818: EA-U4F-5), `lead_bound` (the header's `lead_bound_ms` from the live room, FP7,
    scenario.rs:1184), `refusals` (U4F-Q9's records; room/tests/commands.rs:82; U4f2's attribution reads them),
    `cmd_timing` (the setting's getter, 2.3: **no consumer found**, see "What was not verified").
  - **Built ahead for U4f2:** `refusals_dropped` (C-U4F2-2, C-U4F2-SEC-1).
  - `ClientCounters`' new fields `lead_refused`, `not_newer`, `seq_refused`, `move_refused`, `aim_refused`,
    `seq_outside_window`, `rejoin_overlap` (2.4's per-kind counters; written per session, f14).
  - `ClientSlot::on_time`, `presses`, `ran_early`, `commands_run` (beyond 2.13's words: the per-tick values -20's
    series and U4F-Q3 (c) read; lab/src/logs.rs:613-621, :703).
  - `BITES` 7 to 16 ids, bites builds only. Crate-private: `EventQueue::CAP` (1,600, replacing `PROVISIONAL_CAP`) and,
    by FP12, `InputBuffer::KEPT_WINDOW`.
- **npt-bots** (`Behaviour`'s constructors and hooks; u4-build-plan.md §4.1:197 names "the cheats, the never-acker"):
  `controlled` (2.2's command rule), `with_press_every` (U4F-Q11 (i) (a) as F4 ruled), `never_fires` (2.13's no-fire
  clock script, U4F-Q4 (b)), `never_acks`, `with_points` (the clamp twin), `lag_switch`, `fast_clock`,
  `fast_left_out` (kept public with a consumer: the lab writes `event.fast_clock.left_out_count`, f19 asserts it 0:
  FP12); beyond 2.13's words: `outgoing` (the cheats' hook on the endpoint's datagrams; its host order is **C-U6-18**)
  and `take_bursts` (the lag switch's burst record, `event.lag_burst.held_ns`). **Built ahead for U4f2:** `forger`,
  `take_forged` (C-U4F2-2). Every other new item has its consumer in the lab (my grep).
- **npt-sim:** `World::CHECK_BITES`, bites builds only (the deviation above). The seam is unchanged (`Command`,
  `Rewind`, `TickInput`, `SimCore`'s methods; no seam.rs change: u4f-review-r1-full.md:317).
- **npt-proto:** `Pong::server_ns` (U4F-Q6 (b); the catalogue's Pong row, docs/design/netcode-proof-messages.md:92).
- **npt-server:** none (the room's `Pong` sets the field, server/src/pump.rs: 2 lines).
- **npt-lab:** binary settings only, no library item: `--cmd-timing`, `--controller`, `--brawl-press-every`,
  `--event-cap`, `--connects`, `--twin-logs`, `--lag-hold-ms`, `--lag-every-ms`, `--fast-clock-episode-s`,
  `--fast-clock-idle-s`, and the suites `clock`, `never-ack`, `clamp`, `lag-switch`, `fast-clock` (USAGE,
  lab/src/scenario.rs:1985-1993, with FP9's two lag options).

## Size (SIZE-CHECK against SIZE-U4F (a))
- **His rule:** SIZE-CHECK, history.md:2048-2051 (the main session's reading :2067-2072: M about 3,000 lines, past at
  more than 6,000). **His answer for U4f:** SIZE-U4F (a), history.md:6896-6903.
- **At the stop:** 6,213 added, 307 removed, 29 files (advisor-u4f-stop.md:48-53): M's 3,000 x 2.07.
- **The pass:** +212 by the notes' counting rule (6,213 to 6,425); its own diff 330 in, 121 out, 10 files (the 3 more
  than 330 - 121 are lane-head lines FP1 rewrote, which moved the deletions from 307 to 310: u4f-build.md:1064-1067;
  advisor-u4f-pass.md:10-11). Under the advisor's 500-line guard (6,713; advisor-u4f-r1.md:307-309), against its
  estimate of 250-320 (:306).
- **As merged:** **6,425 added**, 310 removed (3,609 into 20 files plus 2,816 in 9 new files: `git diff HEAD`'s
  shortstat and `wc -l`, my runs; the numstat of the patch gives the same). M's 3,000 x 2.14; the brief's 5,150
  (:939) x 1.25.
- **Why it needs nothing from him:** his (a) closes U4f with one review round, and a round is a review plus one fix
  pass; the pass built no new part (advisor-u4f-r1.md:301-305; advisor-u4f-pass.md:114).
- **By component, as merged** (the patch's numstat by crate, my run; the stop's figures from u4f-build.md:835-845):

| Component | Estimate (brief :927-939) | At the stop | Merged |
|---|---|---|---|
| npt-client | about 700 | 1,421 | 1,521 |
| npt-room | about 950 | 1,956 | 1,985 |
| npt-bots | about 650 | 703 (702 plus the one-line doc fix) | 720 |
| npt-lab | about 1,400 | 1,453 | 1,514 |
| npt-sim | about 40 | 48 | 48 |
| SERIES.md | about 25 | 20 | 20 |
| ops/runs/c-u4f-*.mjs | about 600 | 448 | 453 |
| npt-proto, the catalogue, npt-server | in the 300 | 164 | 164 |
| **Total** | **about 5,150** | **6,213** | **6,425** |

## Round 1: every finding's route (advisor-u4f-r1.md:100-135; the pass check's table, advisor-u4f-pass.md:45-65)
Every FP item met its seat's named fix and its acceptance check, with no departure (advisor-u4f-pass.md:9-12). The
logs named are under .gate/u4f/.

| Id | Severity | Route | Done |
|---|---|---|---|
| NC-U4F-1 | Medium | fix FP2 | `plan()`'s far-goal arm; f32 (ii) and a stall case fail first, then pass (w4-fp2-first.log, w4-fp2b.log) |
| NC-U4F-2 | Low | fix FP6 | `--controller 1` in -22's set, header and short run; s22, s22b by hand (w4-hand.log) |
| NC-U4F-3 | Low | docs FP10; the stamp cap **C-U6-17** | three docs (w4-fp10.log); the carry in plan.md's U6 row |
| NC-U4F-4 | Low | **C-U7-3** | plan.md's U7 row |
| NC-U4F-5 | Low | -20's set report and the OQ-0001-6 reading | after the merge |
| SEC-U4F-1 | Low | fix FP5; the parser half **C-U4F2-1** | `forger` refuses `every < 2` (w4-fp5.log) |
| SEC-U4F-2 (= TE-U4F-6, EA-U4F-5's header half) | Low | fix FP7 | the header from the live room (w4-fp7.log) |
| TE-U4F-1 | Medium | fix FP3 | late apart from missing, 1 % on six profiles (w4-fp3b.log) |
| TE-U4F-2 | Medium | fix FP4 | the flood test and f12 (iv)'s edge; plant fails by name (w4-fp4-plant.log) |
| TE-U4F-3 | Low | rides FP15 | the older stamp; plant fails by name (w4-fp15-plant.log) |
| TE-U4F-4 | Low | rides FP16 | f24 at +32 and +33 (w4-fp16.log) |
| TE-U4F-5 | Low | rides FP18; the report names e9 | f4's doc; the counts section above |
| TE-U4F-6 | Low | = SEC-U4F-2: FP7 | as SEC-U4F-2 |
| TE-U4F-7 | Low | rides FP17 | f2 loops over the two literal ids (w4-fp17.log) |
| TE-U4F-8 | Low | rides FP18 | ten doc lines name their seeds (w4-fp18.log) |
| RR-U4F-1 | Medium | fix FP1 | the longest newest prefix that encodes whole; f25 (ii) fails first (w4-fp1-first.log) |
| RR-U4F-2 | Low | **C-U7-4** | plan.md's U7 row |
| RR-U4F-3 | Low | rides FP1 | no `input.clone()` |
| RR-U4F-4 | Low | **C-U5-1** | plan.md's U5 row |
| RR-U4F-5 | Low | (1), (2), (5) ride FP19; (3), (4) **C-U7-4** | `saturating_sub`s and the doc (w4-fp19b.log) |
| EA-U4F-1 | Low | the open files' items FP12; clock.rs's six **C-U7-4**; built-ahead **C-U4F2-2** | `join_tick` removed, `KEPT_WINDOW` pub(crate), two consumers (w4-fp12b.log) |
| EA-U4F-2 | Low | FP20: the list from the merged tree; §4.1's rows in the merge commit | this report; u4-build-plan.md §4.1 by this commit |
| EA-U4F-3 | Low | accepted as built; the report names the lines | the deviations above |
| EA-U4F-4 | Low | rides FP13 | the room's extent, no `12_800` left (w4-fp13.log) |
| EA-U4F-5 | Low | header half FP7; const assert FP11; the cap's source **C-U7-3** | w4-fp11b.log |
| EA-U4F-6 | Low | rides FP14 | the walk test (w4-fp14b.log) |
| EA-U4F-7 | Low | **C-U4F2-3** | U4f2's brief |
| EA-U4F-8 | Low | fix FP8; **C-U4F2-4** | the early-read line at c-u4f-r2.mjs:24 and :145 (w4-fp8.log) |
| EA-U4F-9 | Low | **C-U6-18** | plan.md's U6 row |
| N1 | note | -22's set report, one line | after the merge |
| C-OQ6-SEC-1 | note | the OQ-0001-6 reading, with NC-U4F-5 | after the merge |
| N-SEC-1 | note | no action | advisor-u4f-r1.md:86-87 |
| The stop ruling's count | Low | corrected | FP20's items below |

The stop ruling's Lows (advisor-u4f-stop.md:198-210): S1 and S2 are above (the deviations; f28's B halves); S3's four
fast-clock hand runs were done before the panel (.gate/u4f/w3-hand.log, each exit 0, contract PASS:
history.md:6948-6949); S4 (USAGE's two lag options), S5 (the subsection's heading) and S6 (the set runner's order line)
rode the pass as FP9 (w4-fp9.log, w4-fp9-check.log); S7 is in FP20's items below; N1 goes in -22's set report.

## Carries, by number
**U4f's own (u4f-brief.md section 3; u4s-carries.md §U4f :507-540 and the entries it names):**
- **46: met at the room's level:** f17, a forged press stream and its twin give equal state hashes on every tick
  (room/tests/commands.rs:579). The lab's -21 twins (2.12, f16) are U4f2's (advisor-u4f-stop.md:80-83).
- **80 (a): met:** `canonical` takes a total key (room/src/room.rs:444); f8's three cases (room/src/room.rs:1695,
  :1715; room/tests/commands.rs:143); f9's plant failed carry 21's test by name, restored (.gate/u4f/plant-f9.log);
  D14's old comparator misorders under f8 (.gate/u4f/w2-d14.log). **(b): met:** the refused far-ahead header bound,
  `ACK_RING_TICKS` (U4F-Q16 (b)); f24, with FP16's +32 and +33 cases (client/tests/carry80.rs:64). **(c): met:**
  `drop_unencodable` drops and counts the command, keeps the acks; f25 and FP1's f25 (ii) (carry80.rs:95, :114).
  **(d): met:** f26 (carry80.rs:131), each half failing under its plant (.gate/u4f/w2-plant-f26-1.log, -2.log). **(e):
  met:** `full_state` files the sentinel on another hash; f27 (carry80.rs:156), failing under its plant
  (.gate/u4f/w2-plant-f27.log).
- **86: met in code for the built suites, its runs pending:** c-u4f-r2.mjs holds 17 of 5.3's 21 rows (p2-p6, p8-p13,
  p15, p16, p18-p21: its RUNS, ops/runs/c-u4f-r2.mjs:57-75), -20's option A at P3 among them (p5) and -21's three
  built suites under option A; the runs are the main session's after this merge. The forged suite's rows p1, p7, p14
  and p17 move to U4f2 by number (advisor-u4f-stop.md:83, :97).
- **57: met:** the cap measured (ops/runs/c-u4f-cap.mjs; .gate/u4f/w2-cap/cap.txt:2-44) and set at 1,600 in the room's
  default (room/src/events.rs:44, `event_queue_cap_provisional` false); approved (advisor-u4f-stop.md R2); the plan's
  text by this commit (u4-build-plan.md §4.7).
- **74: met for the endpoint:** the controller replaces the fixed lead in controller runs; f5's settled leads per
  profile beside U4R-Q14's, late at most 1 % on all six profiles (FP3: B1 late 0.00336, missing 23; P4 0.00067; the
  rest 0: .gate/u4f/w4-fp3b.log). 5.3's late share in both readings comes with the pre-runs.
- **21: met:** option A's several commands of one slot in a tick, sorted; f7 (commands.rs:108); carry 21's room test
  under both options (room/tests/room.rs:237-243).
- **27: met:** `CMD_CATCHUP` and its const assert kept; option A's per-tick cap reads it (f7, f10's kind 10); FP11's
  const assert ties the controller's cap to it (lab/src/scenario.rs:59-64).
- **18: not applicable:** U4F-Q8 (b), the lead bound is a room setting in ms with its ticks by `ticks_for_ms_at`; no
  npt-sim constant.
- **39: met:** no local tick constant; f31 pins ClockSync's period, the honest ping interval and the lead bound's ticks
  at 30 and 60 Hz (client/src/clock.rs:337; the room half, u4f-build.md:135); the lab's 60 s and 200 ms stay in ns;
  check.mjs's deps step holds ("no other *_TICKS or *CATCHUP* constant is set from a literal": m5-check.log:452).
- **50: no code change:** 1,600 lies below its bound 4,328 (cap.txt:44).
- **61: (b) completed by 80 (e)** (f27).
- **58: met for the cap runs:** per-client high-water marks per 30 s bin and per burst, each run's overflow resyncs 0
  (cap.txt); U4f's runs write `overflow_resyncs_count`; -20's 10-minute set reads them before -22's (R2).
- **76: kept and widened:** 2.5's counts at every session end (`Server::session_ends`; f35, scenario.rs:3951), the Kick
  path through the never-acker (f21, scenario.rs:3823, now under the controller), U4r's r8 and r9 pass (M5's test
  step). Its "no forged kind closes a session" for kinds 9 and 11 is shown by U4f2's suite, which builds those
  forgers (u4f-brief.md section 3's row 76).
- **77: (a)** wraps per counter name, as before; **(c)** `stale()` in `client_counts`, read on 5.3's short runs (with
  the pre-runs).
- **Not U4f's:** 63, 64, 67 (U4f opened none of crates/log, crates/judge or the synthetic recordings); 62 (h) and 75
  as the brief says (u4f-brief.md:822-827).

**Into U4f2's brief, by number** (beside advisor-u4f-stop.md:80-89's list, which stands): C-U4F2-1 (SEC-U4F-1's
parser half), C-U4F2-SEC-1 (the attribution logs `refusals_dropped`), C-U4F2-2 (the built-ahead items), C-U4F2-3
(`CommandRefusal` in the sim's struct form), C-U4F2-4 (FP8's line reworded at U4f2's merge), word for word at
advisor-u4f-r1.md:69-85; **ADV-U4FP-3** (below), beside C-U4F2-2; and from U4f's own: 46's lab half and 86's forged
rows (above).

**Later units, written in plan.md's §12.1 rows by this commit** (advisor-u4f-r1.md:137-154; the numbers checked free:
plan.md held C-U6-1 to -16 and C-U7-1, -2 and no C-U5, and no other file in the repository uses these five but the
ruling, the pass check, the card, u4f-build.md and the three code docs that cite C-U6-17): **C-U5-1** (an Input fuzz
target through `Room::receive`, U5), **C-U6-17** (the controller's cap off L1), **C-U6-18** (a Behaviour's host
order), **C-U7-3** (the send time in each `Ping`; the cap's source for the browser client), **C-U7-4** (ClockSync's
hardening as the Worker's).

## The pass check's lines (C3, advisor-u4f-pass.md:33-39)
- **The snapshot sizes, in bytes (ADV-U4FP-1):** the final patch 286aac60… is **394,400 bytes** (394,313 characters);
  builder 3's 74ae26aa… is **380,363 bytes** (380,276 characters) (`wc -c` and a character count, my runs; :82-83).
  worktree_patch.py prints `len()` of a string, so the labels "394,313 B" (history.md:6978; commit 549ed77's message)
  and "380,276 B" (the card, "Running work") are character counts. The sha256s are right and nothing reads the size;
  the card's label is corrected at its next save.
- **The process slips (ADV-U4FP-2, :86-94):** builder 4 named two: two read-only commands chained in one call (P17;
  from its own account, not on disk), and FP9's partial check.mjs (refs and marks) under the shared lock, re-run `-x`
  with the same 2 PASS (.gate/u4f/w4-fp9-check.log; u4f-build.md:988-991). Not named by it: FP1-FP7's per-item clippy
  calls ran `-s` (16 calls in w4-fp1…fp7), while u4f-run.sh's header asks `-x` for a clippy confirmation; builders 2
  and 3 did the same (w2-b7, w2-b8a, w3-b8f, w3-b9a). No counted result rests on any: the counted clippy and check.mjs
  runs (pre-doc, A2, B) were `-x`. Also recorded: builder 4's notes went to its worktree's copy of u4f-build.md, since
  the harness refused the main folder's file (u4f-build.md:874-880), and were copied over whole (copy_b4_notes.py:
  history.md:6977-6978); its end check A's first attempt was stopped by its own 600 s limit, not a failure, and re-run
  whole as A2 (u4f-build.md:1096-1100).
- **The size:** 6,425 added, the pass +212, its own diff 330 in and 121 out (the size section).
- **The counts:** +82 and +99, with 46 negatives by row (the share section).
- **Two notes on how items were built** (advisor-u4f-pass.md:53, :48): **FP7** writes the header's `lead_bound_ms` in
  `run()`, from `server.room.lead_bound().0`, beside `event_queue_cap_count` (lab/src/scenario.rs:1179-1186), since
  `params()` runs at parse time before the room exists: the means of the named fix, not a departure. **FP2** adds a
  stall harness to the trace test: `stall` (no input or step for a span, then the host makes the overdue steps at once,
  `host = next_due_ns.max(host)`) and `check_from` (u4f-build.md:917-933); the stall case sits inside f32 (ii) (+0
  tests), and the harness rule is inert on every other run: f5's, f36's and f32 (ii)'s other figures equal the
  reviewed tree's (advisor-u4f-pass.md:48).
- **ADV-U4FP-3, a carry into U4f2's brief beside C-U4F2-2** (:95-98): client/src/lib.rs:360, in `with_clock`'s doc, is
  one 186-character line (FP2's sentence appended without rewrapping; my `awk` of line lengths), and `plan()`'s doc
  leaves a short line (:617, "… counted. No"). rustfmt.toml sets no `wrap_comments`, so fmt passes. It rides U4f2's
  lane, which opens client/src/lib.rs for C-U4F2-2's `twin_left_out` doc; no push for it.
- **ADV-U4FP-4** (disk) was met by C1 above.

## FP20's items, done here (advisor-u4f-r1.md:275-281)
- **The public-item list** from the merged tree, each with its reason or "built ahead for U4f2": the section above;
  §4.1's rows by this commit (u4-build-plan.md §4.1, the rows U4e's T6 kept: advisor-u4e-r2.md:66).
- **EA-U4F-3's inert lines**, named, and "bites builds only" corrected: the deviations section.
- **TE-U4F-5's e9**, in the negative count: the share section.
- **The stop ruling's count, corrected** (advisor-u4f-r1.md:27-29): advisor-u4f-stop.md:220-221's "20 new rows" and
  "19 rows marked none" read 20 new lines (a heading, two blank lines, the table's two head lines and 15 rows), 14 rows
  marked none, the reorder row naming the unchanged rule. The test seat counted the same (u4f-review-r1-full.md:171).
- **S7's two cites:** already re-pointed. The card's block moved to history.md:6905-6919 carries u4f-build.md
  :692-866 and :827-866 (history.md:6906), and the card holds neither old cite nor the old "confirms" line (my grep).

## Security (for PR #40's report)
No security finding stays open on U4f (advisor-u4f-r1.md:23-24): SEC-U4F-1's guard (FP5) and SEC-U4F-2's header (FP7)
are fixed; SEC-U4F-1's parser half is C-U4F2-1, and C-U4F2-SEC-1 is the security seat's carry; N-SEC-1 needs no
action; C-OQ6-SEC-1 is a recorded design property for the OQ-0001-6 reading (u4f-review-r1-full.md:89). The seat's
read: every structure a client sizes is bounded (kept window 16, stamps 64, buffer 8, refusal records 4,096 a tick,
event queue 1,600), and every new tick compare is a signed distance (:87).

## Timing, each figure labelled
- **The test step** against its line 1,207 s (1.5 x E6's 804.8 s: u4f-build.md:194): builder 1 698.9 and 693.8 s
  (605 / 701 tests: :204, :215); builder 2 668.2 and 682.9 s (619 / 716: :518, :527); builder 3 692.1 and 685.6 s
  (630 / 727: :802, :812); builder 4 882.8 and 834.0 s (634 / 731: :1108, :1121); **M5 865.9 s** (m5-check.log:451).
  Builder 4's machine was slower: the C62H-BOUND reference test, untouched by the pass, ran 13.59 s against builder 3's
  11.41 s (+19 %: advisor-u4f-pass.md:70-72). The split between code and machine is not measured.
- **Each new test alone** (2.10's line: twice the C62H-BOUND reference in the same build): builder 4's line 27.18 s,
  its four new tests 0.00 s each; changed f32 (ii) 0.03, f5 0.16, f21 11.69, lab f19 8.64 s (.gate/u4f/w4-times.log;
  u4f-build.md:1137-1140). Builder 1's f6 23.86 s and f28 23.78 s were past its 22.96 s line (u4f-build.md:224-226);
  build ruling 1's R1 split each per option (:339-344), and builder 2 read them at about 11.3-11.5 s against 22.26 s
  (:615-619). Builder 3's all under its 22.82 s line (:818-821).
- **The wasm step:** 50.5 s in M5 (m5-check.log:453); builder 4's 54.5 s and 53.4 s (u4f-build.md:1110, :1122).

## Choices taken without him (TEST-FIRST rule 2, history.md:2259-2262), and the cuts
**Choices, each inside the plan, undoable, no pass mark, no money, nothing on main** (the card, "Choices taken without
him"; each named in a day's message or for the next one):
- 2026-11-18: D3-1's fix (the `reorder` event at a run's end, B9, f40); the fill's form; build ruling 1's choices (the
  lead bound counted from the tick the room runs next; F1's every 10th stamp; F4's new setting and row; R1's split);
  builders 2's and 3's fresh worktrees (history.md:6935-6946).
- 2026-11-19, from the stop ruling (R2-R4): the event cap 1,600, no longer provisional; -20's set before -22's (order
  -47, -20, -22, then -21's), its 10-minute P3 runs read for honest overflow resyncs first; the fast clock sends no
  pings; main b763ba3 taken in then (MAIN-60).
- 2026-11-19, after his answer: f28's clamp and lag-switch B halves go to U4f2 (R3 (ii)).
- 2026-11-19, from the round-1 ruling: -22's judged set runs U4f's controller (NC-U4F-2); f5 measured as the brief
  writes it, late apart from missing, the unruled 3 % dropped (TE-U4F-1; no bar changes); no second review round, the
  advisor's check of the pass instead.
- 2026-11-19: main f5030a3 taken in while builder 4 ran (MAIN-61, plan first).

**The cuts by the spend limit during U4f** (SPEND-LIMIT (a); each checked before the resume for a plant left in place
and a half edit, none found, and each notice recorded word for word where cited):
- 2026-11-18 14:06:57 to about 16:38 UTC, builder 2 (history.md:6928-6932);
- 2026-11-18 18:47:03 to about 22:39 UTC, builder 3, while its end check A2 ran on and passed at 18:50:00
  (history.md:6922-6927);
- 2026-11-19 16:00:10 to about 17:45 UTC, builder 4 and the session, one second after end check A2 ended (the card,
  "Cuts by the spend limit").

## What was not verified
- **`Room::cmd_timing()`** has no consumer anywhere in spikes/netcode-proof (the drafter's grep), and round 1's EA-U4F-1
  list does not name it. Its reason is 2.3's setting; whether it stays public, goes pub(crate) or waits for U4f2's lane
  (which opens npt-room for C-U4F2-3) goes to the advisor's check of the pushed head.
- I built, ran and timed nothing. Every pass and count is read from the logs named, whose headers tie them to the
  final tree's hashes. My own runs were read only: sha256sum, `wc -c`, `git apply --numstat`, `git diff`, `git log`,
  the `cmp` of the 29 files against builder 4's worktree, and greps.
- Not read whole: the builders' logs beyond the lines named; clock.rs and the lab outside the items cited; the seats'
  own notes files.
- No judged or 5.3 result exists yet for any U4f suite. The controller at P3 for 10 minutes, the cap over 10-minute
  windows, and ClockSync on the lab's L1 at 1,000 connects are shown only by the sets to come (u4f-build.md:240-243;
  advisor-u4f-stop.md:118-125).

## What comes next (M9, then the plan)
1. The push, the saver restarted, CI's gate and toolchain jobs green, the card updated (with ADV-U4FP-1's label).
2. **5.3's pre-runs:** `node ops/runs/c-u4f-r2.mjs --date <day>` from the merge commit (r2's record rule), whole as
   built: 17 of 5.3's 21 rows, since p1, p7, p14 and p17 (the forged suite's) come with U4f2. Its -20 and -47 rows
   (p5, p6, p10-p12, p21) serve those sets; its -21 rows (p2-p4, p8, p9, p13, p15, p16, p18-p20) are an early read of
   the built suites, not -21's 5.3 record (FP8; advisor-u4f-stop.md:90-94). A row that reads other than its `expect`
   comes to the advisor before its set (5.3's last paragraph).
3. **The part-clean at A6's place** (u4f-brief.md 5.5 item 2): after the pre-runs, with no lane running, before the
   first judged set.
4. **The judged sets, one per call, in R2's order** (advisor-u4f-stop.md:126-134; c-u4f-runs.mjs 47, 20, 22): -47
   (3 measured at P2, 1,000 connects, and 2 bites), then -20 (3 + 3 at P3 under options A and B, and the redundancy-1
   bite), then -22 (3 under the controller and the unbounded bite over a full S30) (u4f-brief.md 5.1, 5.5). Before
   -22's set the main session reads -20's six runs: an honest client with any overflow resync goes to the advisor
   first; with none, 1,600 stands and -20's report states the 10-minute high-water mark and its margin (under 1.5 named
   for U4g).
5. **The set reports:** -20's carries NC-U4F-5 (B's depth as the controller's padded target with its three terms) and
   C-OQ6-SEC-1, with the OQ-0001-6 reading (advisor-u4f-r1.md:283-285); -22's carries N1 (the refused rewinds are slot
   0's, the never-acker's, and why: u4f-review-r1-full.md:46) and the load each run carried (5.2, advisor condition 4).
6. **Then U4f2:** its short brief to the advisor before its build (TEST-FIRST rule 3), with the carries above and its
   size line on the card, with its source, before its first snapshot (advisor-u4f-stop.md:84-88). -21 is judged only
   after U4f2 merges.

## After the merge: the pre-runs, the part-clean and the judged sets (items 1 to 5 above)
1. **The push and CI: done.** 25d316e pushed at c26ff6a; CI green. The advisor's head check: APPROVE WITH FINDINGS
   (advisor-u4f-merge-head.md): ADV-U4FMH-1, the card's size, fixed by card_trim_1119h.py; -2, `Room::cmd_timing()`,
   goes to U4f2's lane by number; -3, gate.txt's head, corrected to de3e9e4.
2. **5.3's pre-runs: done.** `node ops/runs/c-u4f-r2.mjs --date 2026-11-19` through main-run.sh (lock exclusive),
   19:35-19:42:17 UTC, exit 0 (.gate/u4f-merge/r2.log). It ran at d502f54, whose spikes/netcode-proof tree is
   25d316e's (f7ed90ad…; `git rev-parse` of both). r2.txt: .gate/netcode-proof-test/u4f-r2-d502f54de6d5/r2.txt, 330
   lines.
   - The advisor's read (advisor-u4f-r2.md): 14 of the 17 rows came as expected. p4, p13 and p19 (-21's lag switch
     under option A) did not: PM-0001-2's in-view precondition fails, because honest slot 24 sits in building area 3
     from tick 205 to the end (:62-71). Those rows hold only -21's set, which is U4f2's, so -47, -20 and -22 could run.
   - Its findings go to U4f2's brief by number: F1 as C-U4F2-5 (Medium: the `steady` script has no leash); F2 as
     C-U4F2-6 (Medium: p19 contradicts U4F-Q15 (c)'s arithmetic); F4 as C-U4F2-7 (Low: the clock suite's shot band,
     item 4 below). F3 and F5 go into U4f2's record lines, and F6 into -20's report (item 5).
3. **The part-clean: done.** P27's six `cargo clean` calls and their guards ran in one main-run.sh call, with no lane
   running (.gate/u4f-merge/clean-shared-2.log, all 16 steps exit 0). D2: 5,204 MiB free before, 11,552 after (about
   20:00 UTC).
4. **-47's set: done. PM-0001-47 PASSES.**
   - **The run:** `flock -x .gate/shared-target/.lane.lock node spikes/netcode-proof/ops/runs/c-u4f-runs.mjs 47 --date
     2026-11-19`, from the repo root with CARGO_TARGET_DIR unset. 20:00:55-20:06:40 UTC, exit 0, at b57221e
     (spikes/netcode-proof tree f7ed90ad…, 25d316e's). The results are in
     spikes/netcode-proof/results/u4f/b57221ed139d/pm-0001-47/ (runs.txt, verdict.json, verdict-bites.json), committed
     at a4cac39. The binaries: npt-lab sha256 3f21c27b…, the bites build 7b9ce9b2…, npt-judge d8b8c91e…; marks.json
     c1935a5b…. The set projected 53 MiB against 11,551 MiB free.
   - **The part** (L1 P2, suite clock, 1,000 connects, a 10 s warm-up and 120 s measured, seeds 1 to 3): all three runs
     valid. The p99 of |estimate - true| reads 2.597, 2.516 and 2.638 ms against the bar of at most 5 ms, each about
     1.9 times under it (r2's p21 read 2.60 ms). The reported maximum reads 3.280, 3.907 and 3.842 ms.
   - **The bites** (seed 1, both runs valid): newest-sample shown (p99 9.246 ms); tick-quantised shown (p99 11.962 ms).
   - **The rest of each run's line** (runs.txt): overflow resyncs 0, heals 0, measured incomplete 0, press refused 0,
     buffer full 0, rewinds refused 0, stale 0, 0 audit breaches. Late 11, 6 and 5. The lead is the controller's, at
     most 9. Wall times 59.3 to 62.1 s.
   - **F4 (C-U4F2-7), named here as the advisor routes it (advisor-u4f-r2.md:123-129):** every -47 run reads "shots
     per measured second 0".
     - Why: the clock suite's clients never fire (`Behaviour::never_fires`, lab/src/scenario.rs:1213-1214;
       bots/src/lib.rs:658-662).
     - The lab contract's shot band applies by client count alone (lab/src/contract.rs:227), so it fails on this suite
       by construction.
     - No verdict reads the contract, and c-u4f-runs.mjs runs none, so -47's verdict stands. The fix ("not applied" for
       a never-firing suite) rides U4f2's lane.
   - **Seed 2: 1,916 slot-ticks in a building (2 slots).**
     - Why: the clock suite's walk is open loop. Only `Kind::Brawl` takes BRAWL-1's leash (bots/src/lib.rs:651-656),
       F1's cause too.
     - -47's part has no in-view precondition (advisor-u4f-r2.md:81-82), so the run stays valid and the verdict stands.
     - Carry 49's "holding every very large unit" reads 0.9938 on that run and 1.0000 on the others. It is a report
       figure.
5. **-20's set: done at 350677f; option B's FAIL is a stand-in defect, so the set re-runs on the fix**
   (u4f-20-read.md; the advisor's ruling advisor-u4f-20.md, read in full). **Re-run on U4f-F1's merge: PASS** (46ca143;
   -20's L1 record, at this item's end; F1's report u4f-f1-report.md).
   - **The run:** `… c-u4f-runs.mjs 20 --date 2026-11-19`, 20:07:24 to about 20:48 UTC, exit 0, at 350677f, whose
     spikes/netcode-proof differs from 25d316e's only by -47's results (`git diff --quiet … ':(exclude)…/results'` exit
     0). Results in results/u4f/350677f0aef2/pm-0001-20/, committed at 1840e68.
   - **The verdicts:**
     - PM-0001-2's SS part PASSES on all six runs.
     - PM-0001-20 under option A PASSES: on time 0.999843, 0.99987 and 0.999861; missing 0.000157, 0.00013 and
       0.000139; 0 lost, 0 doubled.
     - PM-0001-20 under option B FAILS on one check only, "0 presses lost": 3, 1 and 2. Its on-time and missing shares
       equal A's per seed.
     - The bite redundancy-1 is shown.
   - **The cause (ADV-U4F20-1, High):** every lost press comes on a tick where one client's command runs after its
     previous 2 or 3 commands missed their ticks. There are six such ticks (13594, 13824, 17417; 4441; 4446, 4955), two
     of them checked by hand by the advisor.
     - Option B drops the late commands, so the next command's press counter is two edges on.
     - U4f's kind 5 accepts that (room/src/input.rs:285-300: one edge per covered tick).
     - npt-sim takes one edge per changed counter (sim/src/world.rs:1340-1346, U4d's). Its premise, "the room allows
       one per button per tick", stopped holding with U4f.
     - ADR 0001 part 5 keeps the press across a dropped command (:226, :234-237). So the loss is the stand-in's, not
       option (b)'s.
     - **Route** (the advisor's R20-Q1): npt-sim takes every edge through its own cooldown, ammunition and reload
       checks, and the room's lost count comes from the same rule. Two tests: an option-B room test where two late
       drops leave lost 0 and seen 2, and a sim test that the state hash is unchanged. Then -20's L1 set re-runs whole on
       the fixed commit, and that verdict is -20's L1 record. The 350677f folder stays as the defect's record.
   - **The OQ-0001-6 reading** (with NC-U4F-5 and C-OQ6-SEC-1):
     - The 350677f B FAIL is not evidence against (b). After dropped late commands, (b) runs a command whose counters
       carry several edges, and the authority must take each one; (a) meets the same case after a command lost in all
       eight copies (about 0.05^8 per command at P3).
     - -20 separates the options by buffer depth only: A 0 (median and max); B median 2 and max 5 ticks. That depth is
       the controller's padded target with its three terms (NC-U4F-5).
     - C-OQ6-SEC-1, a design property, not a finding (ruled by U4F-Q18 (a); u4f-review-r1-full.md:89): the sim
       advances movement, the weapon clock and the reload once per command run (world.rs:1409-1417). So under option A
       a burst after an idle squeezes up to W + bound = 20 ticks of all three into about 3 ticks (f38's bound); under
       option B it stays one command a tick.
     - F2's p19 reading (C-U4F2-6) is an early, unjudged read: under option A an honest 400 ms hold can reach the
       per-tick cap and lose 7 commands under kind 10. No B-side 400 ms run exists.
     - The pick stays R3's (plan.md:846-848; ADR 0001:1164-1166).
   - **The other figures:**
     - **Overflow resyncs:** 0 on every client of all seven runs, so 1,600 stands.
     - **The queue's 10-minute high-water mark:** A 866, 1,117 and 1,083; B 1,001, 683 and 1,045. The largest, 1,117 (A
       seed 2, slot 27), is at the run's second 52, the 43rd measured second. Every run's maximum falls in the run's
       seconds 30 to 53. Beside it, the 60 s figures (cap.txt) reach 1,045. **The margin is 1,600 / 1,117 = 1.43,
       under 1.5, so it is named for U4g** (advisor-u4f-stop.md:132-134).
     - **F6:** carry 74's B reading uses the session counters against the commands sent: 142 / 879,392 = 0.0161 %, 114
       / 879,452 = 0.0130 % and 123 / 879,387 = 0.0140 %, far under 1 %. The counters cover the whole run: the warm-up's
       6, 2 and 3 are included, and the measured sums are 136, 112 and 120. Option A by its own reading (not on time by
       stamp) reads 0.0157 %, 0.0130 % and 0.0139 %.
     - **Option A's presses:** 432,003, 432,003 and 432,009, against the scripted 432,000. Option A counts a press by
       the tick its command runs, so commands stamped in the warm-up run inside measured tick 300. This is the window's
       edge, not doubling (ADV-U4F20-3).
     - **The measured reorder share:** the report row prints "not available", because it reads a header param that
       L1 runs never write (ADV-U4F20-2, Medium: marks.json:4786-4790, and -58's row :4263-4267, his to change, on the
       after-close list). By hand from the lab's `reorder` event, the lower share is 100, 101 and 101 per mille (up
       160, 161 and 160; down 100, 101 and 101), A and B alike. Validity holds through that event.
   - **-20's L1 record: the re-run on U4f-F1's merge (2026-11-20).** The 350677f folder stays as the defect's record.
     - **The run:** `flock -x .gate/shared-target/.lane.lock node spikes/netcode-proof/ops/runs/c-u4f-runs.mjs 20 --date
       2026-11-20`, about 02:22 to 03:07 UTC, exit 0, under the lane lock, CARGO_TARGET_DIR unset, no uncommitted
       tracked file. HEAD 96de6ac (a card save); its header's spikes/netcode-proof tree is
       d4f6cd41b8d789a7d9ad00f729f3a32224d123c4, the same as 1faf894's (`git rev-parse` of both). Results in
       results/u4f/96de6acf8223/pm-0001-20/, committed at 46ca143.
     - **The verdicts:** PM-0001-2's SS part PASSES, 6 of 6 valid. PM-0001-20 L1 option A PASSES 3 of 3 and option B
       PASSES 3 of 3; the mark's headline reads NOT-RUN because its L2 parts are not run yet. The bite redundancy-1 is
       shown.
     - **Held to the advisor's expected figures** (advisor-u4f-f1-plan.md:119-140; r1 :191-205; head check condition
       3), all equal: the seven summary lines equal to 350677f's but wall time (npt-lab's two binary sha256s new;
       npt-judge's and marks.json's unchanged); the per-tick world hash equal on all 18,335 ticks of all seven runs
       (the scratchpad's compare_20.py on both raw-log folders); presses lost 0 and doubled 0 everywhere; presses B
       432,000 each, A 432,003, 432,003 and 432,009; on time per seed 0.999843, 0.99987 and 0.999861, A and B alike;
       the bite at 0.894544, its lost 3,614 to 0; the reorder row "not available" (ADV-U4F20-2, his; the event's
       figures above stand). A leaf-by-leaf diff of the verdicts finds 14 of 593 measured leaves and 2 of 65 bite leaves
       changed, all of them the lost check and its reasons.
     - **(i)** "'0 presses lost' is 0 by the press-counter design after U4f-F1: npt-room counts with the rule npt-sim
       runs (`Command::edges_taken`), as '0 presses doubled' has been 0 by design since U4f; it guards that rule against
       the room's own wrapping difference, not the sim's take. The sim's take of every edge is proven by T1 and T2 and
       their planted bugs. The run's evidence that no scripted press went missing is presses = 432,000 per option-B
       run, the scripted count (48 x 18,000 / 2); the per-tick state hashes equal to 350677f's show the 1-edge path
       unchanged and the extra fire edge state-neutral at run scale." (advisor-u4f-f1-r1.md:67-72)
     - **(ii)** Wasm, by reading only: the rule is integer-only and an extra throw runs the single throw's float path;
       the wasm step's 27,000 ticks are not shown to hold a multi-edge command.
     - **(iii)** The stand-in's throw has no cooldown, so neither T2 (f) nor any -21 figure shows ADR 0001:236's
       ability check; a press bank of up to 127 throw edges bursts in one tick.
     - **(iv)** SA-F1-1 (Medium) stays open until C-U6-19 closes it, before -19's runs; no -20, -21 or -22 client rejoins
       or throws. RR-F1-1: no action (bounded, no allocation; its large reach leaves with C-U6-19; kind 5 caps a command
       at 127 edges in a session).
     - **(v)** The advisor's plan review's premise (advisor-u4f-f1-plan.md:143-145) is corrected: the count is what the
       one rule does not take, not what the sim does not take; T1 and T2's count halves tie the sim's take to the rule.
       r1's FP-3 restore hash 63057544…16499 was the pre-FP-1 file; "as before" is 901bb546…4c90.
     - **ADV-U4FF1H-1's correction:** 1faf894's commit message calls the advisor's check of F1's pass "APPROVE"; the
       verdict was APPROVE WITH CONDITIONS (advisor-u4f-f1-pass.md:181).
6. **-22's set: done. PM-0001-22 PASSES.**
   - **The run:** `… c-u4f-runs.mjs 22 --date 2026-11-19`, 20:51 to 21:17 UTC, exit 0, at 7192607, the same spike
     tree outside results/. Results in results/u4f/7192607d99a1/pm-0001-22/, committed at 2dae645.
   - **The verdicts:**
     - The part "L1, one never-acker among 48": 3 of 3 runs valid, PASS.
     - The bite PM-0001-22/unbounded: shown (headroom -3,922 against == 0).
   - **The load each run carried** (5.2's report call, never counted): PM-0001-2's checks over the measured folder
     pass except "a connected snapshot-receiving client on every soldier" = 47 on each run. That is the never-acker's
     designed drop (marks.json:5227). Overflow resyncs per client: slot 0 has 1 (its forced resync before the kick);
     every other slot has 0. `rss_bytes` is "not available" on L1, by design (NA-2).
   - **N1:** the refused rewinds (33, 39 and 37), the overflow resync and the kick are all slot 0's, the never-acker's
     (lab/src/scenario.rs:851-852, :1242-1244). No check reads them.
   - **Untouched by ADV-U4F20-1:** -22's bots press no button (no `--brawl-press-every`; bots/src/lib.rs:639-645), so
     the set stands and does not re-run.

## Decisions needed
None.

## For Mustafa
Nothing to decide or do. The merge into the spike branch needs no go from you (under your SIZE-U4F answer (a), as for
U4r): it is not a merge to main, and nothing a player sees changes. One line on size: U4f closed at 6,425 added lines,
the review's fix pass adding 212 to the 6,213 you saw. The choices taken without you since your answer (above) are named in the day's
message. Next come the short test runs and the judged runs for the clock (-47), command timing (-20) and the
never-acknowledging client (-22); then the forged-command unit, U4f2.
