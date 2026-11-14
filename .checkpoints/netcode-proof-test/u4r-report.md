# U4r report: the L1 runner, the -30 pre-check and the run scripts (npt-lab's `scenario`, `replay`, `contract`, `precheck`)

Merged 2026-11-14 into the spike branch by the main session (this report's own commit, "U4r: the L1 runner, the -30
pre-check and the run scripts, merged after review round 2": advisor-u4r-r2-full.md:15). The approved brief:
u4r-brief.md rev 3.2 (sha256 59d570b8…, 1,909 lines: u4r-build.md:20-21; the advisor's check of rev 3.1,
advisor-u4r-brief.md:10-11: APPROVE WITH CONDITIONS on L1 to L5), built on 2fbec77 (the spike head, whose spikes/ is
the brief's `<base>` 614823c's: u4r-build.md:5-14). The bytes merged: wip/u4r.patch, sha256 21ca46be…, 26 files. The
judged runs come after this merge and are pending (last section). For U4's report, which folds this one
(advisor-u4r-r2-full.md:23).

## Who built it, and how
- **The build:** one builder, run as six copies in one worktree (`.claude/worktrees/agent-a11e2354169f39acf`), each
  resuming from the notes u4r-build.md, every cargo run through u4r-run.sh (u4r-build.md:16-23). Copy 1 (2026-11-13,
  first build 19:37 UTC: the core, logs, mirror, manifest, recordings: :34-282); copy 2 (from about 20:55 UTC: `replay`,
  the -30 pre-check, build ruling 1: :284-569); copy 3 (from 21:54 UTC: r2's contract checker and suites, build ruling 2: :571-906);
  copy 4 (2026-11-14 from 00:54 UTC: check.mjs's carries 83-85, det-u4.mjs, c-u4r-runs.mjs: :908-1171); copy 5 (from
  01:41 UTC: build rulings 3 and 4, r5, both end checks: :1173-1587); copy 6 (from 11:17 UTC: the round-1 fix pass:
  :1588-2111). A container restart (about 01:18 UTC, :1066) and the spend limit (02:31:54 UTC, while end check run 1
  went on to its end, :1491-1492) interrupted copies 4 and 5; each went on from its notes.
- **Counts:** before U4r measured 491, bites 561, self 71, 11 steps (u4r-build.md:1508); at round 1's snapshot
  526 / 603 / 76 (:1499-1508, :1524); after the fix pass **531 / 609 / 77, 11 steps** (:2012, c6-end2.log, c6-end3.log).
- **Review round 1** (wf_a3541dc5-9e1, snapshot 572c4f69): six seats; rust-reviewer REQUEST CHANGES, netcode-architect,
  test-engineer, security-auditor, engine-architect and infra-engineer APPROVE WITH FINDINGS; 50 findings, 10 Medium
  and 40 Low, no Critical or High (u4r-review-r1-full.md:5-323; advisor-u4r-r1-full.md:46).
- **The fix pass** (copy 6; FP-0, the runners' MiB floor, by the main session; FP-1 to FP-9 by the builder): every plant
  restored by Edit with its sha256 checked; two end checks passing all 11 steps; det-u4.mjs and the pre-check
  regeneration PASS (u4r-build.md:1955-2060).
- **Review round 2** (wf_5e07787a-ae9, snapshot 21ca46be): five seats (engine-architect sat out: no pub item, dependency
  or seam change was ruled, advisor-u4r-r1-full.md:40), all APPROVE WITH FINDINGS: 1 Medium and 12 Low
  (u4r-review-r2-full.md:5-167). **The advisor's round-2 ruling** (wf_24167e6e-996): no round 3, the merge on M1 to M9,
  the follow-ups below, nothing for Mustafa (advisor-u4r-r2-full.md:7, :32).

## What the four build rulings and both rounds settled
- **Build ruling 1** (wf_21a67b58-19b, 10 points): the determinism route (c): the two runner recordings go in their own
  manifest crates/lab/det-recordings.json, run by ops/runs/det-u4.mjs on check.mjs's wasmStep, its non-cargo calls at
  most 240 s (advisor-u4r-build1-full.md:12); the mirror's radius copy pinned by placement (:19); -57's series written
  only on runs that list it (:26); -17's bite becomes PM-0001-17/whole-as-lost (:33); `--lead`/`--every-step` refused
  with `--record` (:68).
- **Build ruling 2** (wf_083df555-757, 7 points): the pre-check's depth header made exact (advisor-u4r-build2-full.md:12);
  its result lines are the record, no per-leg log (:19); a pre-existing npt-server bites failure is not U4r's (:26);
  test scratch folders move under the cargo target (:33); builder logs are evidence and the record is re-made from the
  committed code (:40).
- **Build ruling 3** (wf_345cc3c9-ca1, 8 points, one High that does not block the merge and is not U4r's code): B1's
  everyone-in-view FAIL is the room's, and its fix with -17's and -18's L1 B1 evidence go to U4g
  (advisor-u4r-build3-full.md:13; u4r-build.md:1383-1392); the ES brawl is mis-set, the band stays, and the fix is U4h's
  (:20; u4r-build.md:1391-1398); the tick16 bite run's settings change (:27); -18's bite expectation corrected (:34);
  `contract` and `precheck` accepted as subcommands, r2's record is the main session's run of c-u4r-r2.mjs from the
  merge commit (:48); the mirror keeps the stale baseline's frame (:62).
- **Build ruling 4** (wf_f276e5ae-878, 3 points): `after_capped` records how a run ended and is not expected; tick16d and
  tick16-clean kept, no re-run (advisor-u4r-build4-full.md:11); the bite id stays a run-time lookup plus one assertion
  (:18); c-u4r-runs.mjs's two items confirmed (:25); carry 62 (h)'s check is a plain check, the main session's after the
  merge (u4r-build.md:1428).
- **Round 1:** two real code defects (a closed connection left a ghost slot in the room: NC-U4R-1 = SEC-U4R-1, fixed by
  FP-1; manifest text could inject extra settings: RR3-M1 = SEC-U4R-2, fixed by FP-3), a weak test (T-1, FP-2), an
  unproven regeneration (T-2 = INF-U4R-1, FP-8) and four disk-safety problems in the scripts and runners (FP-0, FP-5); of the
  40 Lows, 26 rode the fix pass, 9 in part, 5 became follow-ups (advisor-u4r-r1-full.md:46). Mustafa chose SIZE-U4R
  (a), the fix pass as ruled (history.md:2044).
- **Round 2:** every round-1 fix confirmed (advisor-u4r-r2-full.md:32). INF-R2-1 (Medium: the run scripts compare the
  whole repository's HEAD and status before the judge, so a checkpoint commit mid-set stops the set; it fails closed)
  becomes U4r-F1, a change-set of its own before any c-u4r-runs.mjs set (:38-41). INF-R2-2 (Low) is done at this merge:
  report words and `uptime` first in M5 (:45-48). The other 11 Lows are follow-ups (below).

## The two deviations accepted
Both accepted as made; nothing re-run (advisor-u4r-r2-full.md:28; logs under .gate/u4r/). (1) FP-2 plant (i)'s first run failed at the Kick
count, not the ruled assert, because copy 6's r8 read c1 after the room tick (c6-plant-fp2i.log); the test was moved to
the ruling's own placement (drain right after the join, scenario.rs:2045-2068), and then plant (i) failed at "the
first's Inputs reach no room call" and plant (ii) at the Kick assert (c6-plant-fp2i-b.log, c6-plant-fp2ii.log). (2) End
check 1 crashed at the wasm step on a fractional spawnSync timeout from copy 6's own FP-6 item 11 edit ("Received
296513.41192700004", c6-end1.log; u4r-build.md:1840-1848); the call never started, so it is none of the no-retry kinds.
The fix, `Math.ceil` at check.mjs:2357-2359, is pinned by a self-test whose plant fails without it (76 of 77,
c6-plant-ceil.log); the failed run is kept whole, and two fresh runs passed all 11 steps. In both cases copy 6 did not
stop and report as its prompt said; that line stays for later copies (advisor-u4r-r2-full.md:28). Not a deviation:
copy 6's by-hand node --test at 73 of 76, re-pinned in the same step, recorded under TE2-5 (u4r-build.md:2075-2077).

## Checks on the merged tree
- **M1:** wip/u4r.patch sha256 21ca46be…, unchanged since 450af1a (`git diff --stat 450af1a HEAD` on it: empty);
  .gate/u4r-snap2's spikes/ equals copy 6's worktree; the tree fingerprint 63713cf344482a9b… from the copy's root, the
  builder's own (u4r-build.md:1867; the main session's checkpoint 2494918: "M1-M4 hold").
- **M2:** spikes/ unchanged since 2fbec77 (`git diff --quiet 2fbec77 HEAD -- spikes/` exit 0), every commit since
  touching only .checkpoints/; main still 4460ef5 (docs only), not merged in (Mustafa's MAIN-52, history.md:2082-2086).
- **M3:** the 26 files applied under spikes/netcode-proof (13 modified, 13 new): 8,362 lines added (6,906 in the 13 new
  files, 1,456 into the 13 existing ones), 106 removed (`git apply --numstat` of the patch).
- **M4:** each of the 26 files cmp-equal to .gate/u4r-snap2's copy; the fingerprint from the main folder's root
  63713cf344482a9b3e5f911bab4012d16201dab753b972c6e042c5fd894aa5bf.
- **M5** (the counted run on the merged tree, .gate/u4r-merge/m5.log; expected as advisor-u4r-r2-full.md:12 says):
{{M5}}
- **M6** (TE2-5, advisor-u4r-r2-full.md:13):
{{M6}}
- **M7** (the gate, advisor-u4r-r2-full.md:14):
{{M7}}

## Carries, by number (u4s-carries.md §U4r, :255-441; the brief's table, u4r-brief.md:884-908)
- **Met by U4r's code** (the builder's evidence; each judged figure comes from the record run or the sets, pending):
  6 (`Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP)`, scenario.rs:323, precheck.rs:322); 14's U4r half (the
  recording's `span_ms` is the room's ring, ROOM_HISTORY_MS, scenario.rs:855; a copy of a private constant:
  u4r-build.md:168-169); 49 (the placement refusal above 3,955 cm, u4r-build.md:135; its figures in r2, :680-681); 58
  (the queue high-water mark reported, :683; a judged -17 or -18 run with overflow resyncs is flagged to the advisor,
  `overflowFlag`, c-u4r-runs.mjs:152); 59 (the ES suite reported FAIL beside U4d-F's 7.250, routed to U4h:
  :1391-1398); 60 (missing header params are a contract failure, :1235); 62 (a) to (f) (the client writer:
  u4r-review-r1-full.md:13), (h)'s check the main session's (step 12 below); 63 (two refusals in crates/log, each
  test shown failing first: u4r-build.md:65-77); 73 (the tick16 bite run at ruling 3's settings: tick16d, :1430-1439);
  74 (lead 2 at P0; no clean short run above 1 % late, the highest 0.617 %: :680, u4r-review-r1-full.md:27); 75 (the
  evstart suite, :677; the bound is checked on logged runs only, :1826); 76 (session handling, with FP-1's closed
  decoder or link ending the session and freeing the slot: scenario.rs:492-506, u4r-build.md:1967-1969); 77 (b) and
  (c) (-18's heals counted apart, :1399-1410; stale 0, :632); 82 (the manifest's `kind` schema, the runner entries in
  det-recordings.json: :151-170); 83 (b) to (d), 84 and 85 (a) to (e) (check.mjs and det-u4.mjs, each with its
  self-test and plant: :933-1065).
- **Its judged half pending:** 40 (the tolerance and the cap are not built at U4r; the pre-check set, step 16 below,
  stops if the recording refused a rewind: advisor-u4r-r2-full.md:22); 73's judged bite run and 58's flagged runs (the
  P0 set, step 17).
- **Not found by number in the builder's notes, not verified here:** 20 (`grid::Config` never hashed across builds;
  the brief names it at u4r-brief.md:145 and :597) and 69 (NF-L1's U4r half; the brief at :430). For the advisor's
  review of this head.

## Public items
- npt-lab's binary: L1Scenario is built as scenario::run plus Server (scenario.rs:341-552, :742-1199); the pre-check's
  run_leg is a second loop with 7.2's pacing, sharing Server for step 2 (precheck.rs:285) (EA-U4R-1's line, the
  fix-pass tree's numbers: u4r-build.md:2063-2064; checked in the merged tree). Subcommands `scenario` (with
  `--record`) and `replay` (brief 2.7), and `contract` and `precheck` beyond 2.7's list, accepted by build ruling 3,
  point 6 (main.rs:225-285; advisor-u4r-build3-full.md:48).
- npt-bots: `Behaviour` with `brawl`, `script`, `every_step`, `given`, `skipped`, `step` (bots/src/lib.rs:77-178).
  npt-server, bites builds only: `Decoder::with_bite` (pump.rs:279-282); `schedule::BITES` now 4 entries (:252).
- check.mjs: 2 new exports, `cargoConfigDirs` and `run` (67 to 69, against HEAD's file); `wasmStep` gains its options
  (manifest, bitesHalf: u4r-build.md:959-961).

## Timing, each figure labelled
- **The wasm step:** round 1 39.6 s (c5-end1: warm, up about 1 h) and 41.9 s (c5-end2: after a restart, page cache
  cold, dependencies warm, up 5 min); the fix pass 43.1 s (c6-end2: warm, up 1:38) and 48.7 s (c6-end3: warm, up 1:51,
  load 1.3-2.0); against 300 s. The cold figure (every release artifact built from nothing) was not measured for U4r's
  tree; it is bounded by U4d2's 61.6 s plus U4r's added builds (u4r-build.md:1916-1923).
- **det-u4.mjs, the new base (INF-R2-2):** non-cargo 223.7 s of 240 s (npt-lab hash 220.3 + Wasm 2.5 + 0.9;
  c6-det-u4.log, up 2:05, load average 1.32 at its start), against round 1's 190.9 s (det-u4.log, load 0.11). The split
  between code and machine load is not measured. Adding a runner entry to det-recordings.json will cross 240 s, so U4h's
  plan settles the route, RR-L4 weighed first, before recording (advisor-u4r-r2-full.md:45). The sources differ on one
  cause: the builder names FP-1's per-session link check among the changes (u4r-build.md:1922); the advisor finds the
  32.8 s rise unexplained by FP-1's constant-time state() read (advisor-u4r-r2-full.md:48).
- **The merge step's det-u4 figure, with its uptime:** M5 above (`uptime` is that call's first command).
- **The runner's rate (r5, builder's tree):** slowest kept 79.6 ticks/s at SS P0, 63.1 P2, 60.1 P3, 67.1 B1, 161.6 ES
  P3; the judged set and pre-check estimated at 5,240.9 s, about 1.5 h, builds and judge passes not counted
  (c5-r5.log; u4r-build.md:1448-1486). D4 measures it again in the main folder (step 14).

## Size (SIZE-CHECK)
8,362 lines added (6,906 in 13 new files, 1,456 into 13 existing; 106 removed) for a unit sized M
(M: about 3,000 lines, his figure; past at more than 6,000: history.md:2048-2051 and the main session's reading,
:2067-2073); 7,568 at snapshot 572c4f69 (:2073-2074). Mustafa's SIZE-U4R answer (a) covers the fix pass (history.md:2044-2045;
advisor-u4r-r2-full.md:32).

## Follow-ups (each with its home; none triggers a push)
- **U4r-F1** (before any c-u4r-runs.mjs set; plan first, infra-engineer as its one round): INF-R2-1 (compare only the
  spike's tree hash and its own status before the judge or r2.txt), INF-R2-3 (no set folder before the first floor),
  INF-R2-4 (a 3,600 s limit on each npt-lab and judge call) (advisor-u4r-r2-full.md:19, :38-41, :108-118). Until then
  only r2's record run may run, with saves paused (:40).
- **The check.mjs follow-up** (check.mjs's next change; before PR #40 is marked ready at the latest): SEC-U4R-R2-1,
  SEC-U4R-R2-2, TE-R2-3 (a TERM-trapping child in the real-process test), INF-R2-5 (`run()` returns the signal) and
  round 1's INF-U4R-10 (2) (a group kill after a timed call that did not exit 0) (advisor-u4r-r2-full.md:24, :52-62,
  :87-90, :122-125; advisor-u4r-r1-full.md:17).
- **The next npt-lab lane:** N2-1 = TE-R2-1 (the roster check in closed_checks), TE-R2-2 (the untested overflow
  refusals), RR4-L2 (bounds on a run's last tick and the bursts), RR4-L1 (a 36,000-tick manifest cap, before U4h adds its
  carry-87 entry at the latest), with round 1's EA-U4R-1's shared loop, EA-U4R-2's `Sim` type and one server
  constructor (before the pre-check's re-run after U4h) and RR3-L2 (a) (advisor-u4r-r2-full.md:24, :66-104;
  advisor-u4r-r1-full.md:17).
- **Round 1's others** (advisor-u4r-r1-full.md:17; u4r-build.md:1936-1950): NC-U4R-4 (U4g's brief); NC-U4R-5 with
  EA-U4R-7's logs half (the first logged replacement: U4f or U6); EA-U4R-4 (the next lane that opens crates/sim);
  EA-U4R-5 (the U6 brief's bots'-view follow-up); NC-U4R-7's derivation (carry 18's route); RR3-L6 (measured inside
  the judge timing of step 17); T-6's contract half (the next unit that edits contract.rs); INF-U4R-6 (U4h's brief,
  now with INF-R2-2's base above); SEC-U4R-4's U6 half (U6's TickLoop carry).
- **Housekeeping:** /tmp/npt-lab-tests holds 124 folders, none deleted (Mustafa's LEFTOVER-DELETE; u4r-build.md:1927-1929).

## Open security findings (for PR #40's report)
- **SEC-U4R-R2-1 (Low, open):** check.mjs's attribute rule sees only attributes, so a macro could build a `#[path]`
  that compiles a non-.rs file into npt-hash-wasm or npt-tcpinfo; it needs committed, reviewed code, and no crate has a
  macro today. Until fixed, PR #40 stays needs-mustafa for it, and the advisor reads any diff that adds a macro there or
  in npt-sim (advisor-u4r-r2-full.md:52-55).
- **SEC-U4R-R2-2 (Low, message only):** a profile refusal under a quoted table header names line 1; the refusal itself
  fires (:59-62).

## The judged runs (pending, in the ruling's order)
1. **The r2 record run** from the merge commit: `node ops/runs/c-u4r-r2.mjs --date <today>`, lock exclusive, saves paused
   until r2.txt is written; b1, es and b1-nofallback are expected to fail as ruled and are never re-run
   (advisor-u4r-r2-full.md:17). Pending.
2. **Carry 62 (h)** from r2: if every clean short L1 P0 suite's measured-phase sum of snap_incomplete is 0, the main
   session's own commit adds the plain check and PM-0001-17/whole-as-lost to -17's L1 P0 part in marks.json, plus the
   readings-PR list line, and `npt-judge --only PM-0001-17` on r2's whole-as-lost and tick16 folders must read both
   bites Shown, never INVALID; otherwise the cause goes to the advisor first, before any -17 run (:18). Pending.
3. **U4r-F1** (above; :19). Pending.
4. **D4:** the runner's ticks per second in the main folder's own cache, the set's wall time redone (:20). Pending.
5. **The disk plan**, in writing: 4,681 MiB free; kept logs of about 162 + 16 + 360 + 545 + 629 + 3,020 MiB plus the
   4 GiB floor and builds leave it about 4.1 GiB short; space only from the approved part-cleans; never delete a log;
   if P0 still cannot fit, it goes to Mustafa under Decisions needed (:21). Pending.
6. **The pre-check set** after F1 and D4; the late-command pair counts only on 'result: VOID (L1 late_count 1)' (:22).
   Pending.
7. **The judged sets, one at a time: P3, then B1, then P2, then P0 last**; peak memory by getrusage (no /usr/bin/time
   here); B1's -17 and -18 INVALID, recorded and never counted; every run flagged for overflow resyncs to the advisor
   before it counts (P0's tick16 bite run expected, about 48); P0's judge memory projected first, over 10 GiB to the
   advisor (:23). Each pending; this section and the card are filled after each set.

## Decisions needed
None.

## For Mustafa
Nothing to decide or do now (advisor-u4r-r2-full.md:32). The merge into the spike branch needs no go from you: it is not
a merge to main, and nothing a player sees changes. One line, as you asked for SIZE-U4R: U4r has 8,362 lines after its
fix pass, against about 3,000 for its size M; your answer (a) covers it. Two heads-ups, neither a decision now: (1) the
judged runs need about 4 GiB more free disk than today; the approved cache cleans should cover it, and if they do not, it
comes to you under your LEFTOVER-DELETE rule; (2) one Low security weakness in the spike's own code checker
(SEC-U4R-R2-1) stays open as a follow-up and will be listed in PR #40's report, which needs your go anyway.
