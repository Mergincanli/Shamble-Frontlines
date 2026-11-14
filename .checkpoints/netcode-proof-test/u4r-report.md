# U4r report: the L1 runner, the -30 pre-check and the run scripts (npt-lab's `scenario`, `replay`, `contract`, `precheck`)

Merged 2026-11-14 into the spike branch by the main session (this report's own commit, "U4r: the L1 runner, the -30
pre-check and the run scripts, merged after review round 2": advisor-u4r-r2-full.md:15). The approved brief:
u4r-brief.md rev 3.2 (sha256 59d570b8…, 1,909 lines: u4r-build.md:20-21; the advisor's check of rev 3.1,
advisor-u4r-brief.md:10-11: APPROVE WITH CONDITIONS on L1 to L5), built on 2fbec77 (the spike head, whose spikes/ is
the brief's `<base>` 614823c's: u4r-build.md:5-14). The bytes merged: wip/u4r.patch, sha256 21ca46be…, 26 files. The
runs after this merge: the r2 record run, carry 62 (h) and U4r-F1 (f1f131b) are done; the judged sets are pending
(last section). For U4's
report, which folds this one
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
  40 Lows, 26 rode the fix pass, 9 in part, 5 became follow-ups (advisor-u4r-r1-full.md:46). EA-U4R-2: a deviation
  from U4-SEAM (u4-build-plan.md:162-163): npt-lab names npt_sim::World at logs.rs:97, :472, :595 and precheck.rs:296,
  and builds the room twice (scenario.rs:751, precheck.rs:301); its follow-up is below (u4r-build.md:1930-1934;
  advisor-u4r-merge-full.md, MRG-L3). Mustafa chose SIZE-U4R (a), the fix pass as ruled (history.md:2044).
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
  one main-run.sh call, 13:44:35-14:06:39 UTC, lane lock exclusive, offline, free 4,679 MiB at its start. `uptime`
  first: up 3:09, load average 0.27, 0.09, 0.10. The provenance lines, counted over the whole call (seven builds, the
  regeneration's npt-lab build included; advisor-u4r-merge-full.md, MRG-L1), all name
  /home/user/Shamble-Frontlines/spikes/netcode-proof/crates/... (npt-lab 5, npt-bots 5, npt-hash-wasm 4, npt-sim 7).
  `node check.mjs`: "check: all 11 steps pass"; test 635.8 s, measured build 531 tests passed, 0 failed, 0 reported a
  skip, bites build 609, 0 failed, 0 reported a skip; self 77 of 77; marks 81 read and checked, the ADR rows
  unchanged; redact 0 findings; deps held ("no attribute holds a metavariable"; npt-sim's features one set); wasm
  43.5 s (27,000 ticks, 0 mismatching for both pairs; h2 equal; hash-map caught at SS-s1 tick 1234; std-sin at SS-s1
  tick 2, vehicle[8].pos.y). `node ops/runs/det-u4.mjs`: deps PASS 1.7 s; det-u4 PASS 240.3 s in all, 18,000 ticks
  and 0 mismatching for both comparisons; **its non-cargo calls 227.2 s** (npt-lab hash 223.8 + Wasm 2.5 + 0.9),
  under 240 s; the call's uptime at 13:44:36, 15 min 41 s before det-u4 started (after check.mjs; m5.log:3, :184):
  load average 0.27, 0.09, 0.10; the load at det-u4's own start was not measured (advisor-u4r-merge-full.md, MRG-L1).
  The pre-check regeneration: "regenerated SS-P0-s4: 16950444 bytes, sha256 6933b56a…bab3, as the manifest's", native
  legs equal over 9,000 ticks, 2 min 10 s.
- **M6** (TE2-5, advisor-u4r-r2-full.md:13):
  no test failed in the counted run (531 and 609, 0 failed); nothing was re-run. So TE2-5's rule ends at this merge
  (TE-Q2, as the ruling's M6 says).
- **M7** (the gate, advisor-u4r-r2-full.md:14):
  `node tools/gate.mjs` (the full gate): every check PASS except 5a smoke, which could not start: the tools'
  Playwright looks for chromium_headless_shell-1194, and this container holds build 1243 (.gate/u4r-merge/gate.log;
  no browser installed, by the session's rules). `node tools/gate.mjs --no-smoke`: "GATE PASS (5a smoke skipped:
  --no-smoke)" (.gate/u4r-merge/gate-no-smoke.log). CI's gate, smoke and toolchain jobs run on the pushed head (M9).

## Carries, by number (u4s-carries.md §U4r, :255-441; the brief's table, u4r-brief.md:884-908)
- **Met by U4r's code** (the builder's evidence; each judged figure comes from the record run or the sets, pending):
  6 (`Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP)`, scenario.rs:323, precheck.rs:322); 14's U4r half (the
  recording's `span_ms` is the room's ring, ROOM_HISTORY_MS, scenario.rs:855; a copy of a private constant:
  u4r-build.md:168-169); 20 (`grid::Config` never hashed across builds: crates/lab/src names it only at mirror.rs:23
  and :152, a local Config::default() used by value, and replay's re-decoder takes npt-client's own grid_with_bite,
  replay.rs:8-10; nothing hashes, records or compares a Config: this head's review, advisor-u4r-merge.md, C13 (b)); 49
  (the placement refusal above 3,955 cm, u4r-build.md:135; its figures in r2, :680-681); 58 (the queue high-water mark
  reported, :683; a judged -17 or -18 run with overflow resyncs is flagged to the advisor, `overflowFlag`,
  c-u4r-runs.mjs:152); 59 (the ES suite reported FAIL beside U4d-F's 7.250, routed to U4h: :1391-1398); 60 (missing
  header params are a contract failure, :1235); 62 (a) to (f) (the client writer: u4r-review-r1-full.md:13), (h)'s
  check the main session's (step 12 below); 63 (two refusals in crates/log, each test shown failing first:
  u4r-build.md:65-77); 69 (NF-L1's U4r half: Recording::synthetic appears only in hash.rs's regeneration, hash.rs:384,
  and neither crates/lab/src nor the run scripts report an ES byte or bandwidth figure; the scripts' "bytes" are disk
  sizes, c-u4r-r2.mjs:55-78: the same review, C13 (b)); 73 (the tick16 bite run at ruling 3's settings: tick16d,
  u4r-build.md:1430-1439); 74 (lead 2 at P0; no clean short run above 1 % late, the highest 0.617 %: :680,
  u4r-review-r1-full.md:27); 75 (the evstart suite, :677; the bound is checked on logged runs only, :1826); 76
  (session handling, with FP-1's closed decoder or link ending the session and freeing the slot: scenario.rs:492-506,
  u4r-build.md:1967-1969); 77 (b) and (c) (-18's heals counted apart, :1399-1410; stale 0, :632); 82 (the manifest's
  `kind` schema, the runner entries in det-recordings.json: :151-170); 83 (b) to (d), 84 and 85 (a) to (e) (check.mjs
  and det-u4.mjs, each with its self-test and plant: :933-1065).
- **Its judged half pending:** 40 (the tolerance and the cap are not built at U4r; the pre-check set, step 16 below,
  stops if the recording refused a rewind: advisor-u4r-r2-full.md:22); 73's judged bite run and 58's flagged runs (the
  P0 set, step 17).

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
- **The run scripts' next change** (c-u4r-runs.mjs, c-u4r-r2.mjs; from U4r-F1's round and head review): INF-F1-1's
  header sentence, in the advisor's words (advisor-u4r-f1-head-full.md); INF-F1-5's untrimmed status read; and
  INF-F1-3's signal name, once check.mjs's run() returns the signal (INF-R2-5, below).
- **U4r-F1, done at f1f131b** (the judged runs, item 3) (before any c-u4r-runs.mjs set; plan first, infra-engineer as
  its one round): INF-R2-1 (compare only the
  spike's tree hash and its own status before the judge or r2.txt), INF-R2-3 (no set folder before the first floor),
  INF-R2-4 (a 3,600 s limit on each npt-lab and judge call) (advisor-u4r-r2-full.md:19, :38-41, :108-118). Until then
  only r2's record run may run, with saves paused (:40). Two Lows ride it: RUN-L1 (r2.log's provenance block is empty:
  both run scripts' sh() keeps cargo's output; both print cargo's "Compiling/Fresh npt-* (path)" lines from stderr into
  the runner log; the record holds through r2.txt's clean-tree check, commit and sha256s: advisor-u4r-r2run-full.md:17);
  and c-u4r-r2.mjs's whole-as-lost suite (:87) gets marks "PM-0001-2,PM-0001-17" (header and --only, tick16's form) and
  the expect line "contract PASS; bite PM-0001-17/whole-as-lost shown: mismatches 0 pass, L1 P0 loses no group 48
  FAIL; the run valid", for a future run only (advisor-u4r-c62h-full.md:45).
- **JDG-ONLY-1 (Medium; npt-judge's next change, with its own plan, the advisor's plan review and cli.rs fixtures,
  before PR #40 is marked ready at the latest and before any expect-invalid bite with a pre: rule enters marks.json):**
  the judge's --only drops every unnamed mark before judging (main.rs:94), so pre:PM-0001-2 cannot find PM-0001-2 and
  the run reads INVALID, "not in marks.json" (eval.rs:958-962). It fails closed for today's marks.json (a latent
  expect-invalid bite with a pre: rule would read a false "shown"; none exists); no recorded verdict rests on it
  (advisor-u4r-c62h-full.md:7-20). Interim rule: every npt-judge --only that names a mark carrying pre:PM-0001-2 also
  names PM-0001-2 (:20, :44). The ruling says both run scripts already do, yet its reach notes c-u4r-r2.mjs's
  whole-as-lost suite names only PM-0001-17 (:19): the Low on the U4r-F1 line.
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

## The judged runs (in the ruling's order; 1 and 2 done, the rest pending)
1. **The r2 record run: done.** `node ops/runs/c-u4r-r2.mjs --date 2026-11-14` through main-run.sh, lock exclusive,
   run 15:40:37-15:44:52 UTC, exit 0 (.gate/u4r-merge/r2.log:1-3 and its end), at commit e59ece9, whose spikes/ equals
   the merge commit 71ee13d's (`git diff --quiet 71ee13d e59ece9 -- spikes/` exit 0). r2.txt
   (.gate/netcode-proof-test/r2-e59ece9ecafc/r2.txt, 278 lines): 14 suites, all 42 commands exit 0, no "stopped at"
   line. The advisor's ruling (advisor-u4r-r2run-full.md, wf_ec319c2d-f45): all 14 as expected, r2 at e59ece9 the
   record, the three ruled fails recorded and never re-run (:11). Each suite, expected, then what came (:27-109):
   - p0, p2, p3, tick16-clean: contract PASS; -2, -17, -57 pass. Came so: -2 11 of 11, -17 0 mismatches each; measured
     incomplete 0, 585 (P2's 2 % loss: 2.03 %), 1,485 (P3's 5 %: 5.16 %; carry 62 (g) shown) and 0 across the wrap.
   - b1: -2 FAIL at 0.273333, -17 and -18 L1 B1 INVALID, -57 passes. Came so (the ruled fail: ruling 3 point 1; U4g's).
   - forged, clamp, fast-clock: -2 passes; evstart: -2 and -17 pass. Came so, incomplete 0; evstart shows carry 75.
   - lag-switch: -2 passes. Came 11 of 11; its 300 incomplete client-ticks (1.04 % of 28,800) are profile P1's designed
     1 % loss: the run is P1, outside -17's L1 P0 part, so not carry 62 (h)'s condition (:69-73).
   - es: -2 FAIL by reliable events. Came 50.05 a second against 10.5, the other 11 pass (ruled: ruling 3 point 2; U4h's).
   - tick16 (bite): -17's bite run FAILs with at least 48 mismatches; -2 passes. Came: bite PM-0001-17/tick16 shown,
     2,880 mismatches; measured incomplete 720 = 48 x 15. "PM-0001-2 passes" is the bite run's validity (pre:PM-0001-2
     holds; the judge's -2 line reads NOT-RUN because no -2 part takes a bite run); carry 73 met (:87-91).
   - b1-nofallback (bite): -18's max FAILs, INVALID by pre:PM-0001-2. Came: max heal 1.69 s, INVALID at 0.453333 (ruled; U4g's).
   - whole-as-lost (bite): measured incomplete 48. Came 48; -17 NOT-RUN (the bite was not yet in marks.json).
2. **Carry 62 (h): done.** The six clean short L1 P0 suites (p0, forged, clamp, fast-clock, evstart, tick16-clean) read
   0 each (advisor-u4r-r2run-full.md:7). marks.json changed at commit 5e563b9 (+15 lines): PM-0001-17 gains the check
   "L1 P0 loses no group (no client-tick left incomplete)" (measured sum of tick.s.snap_incomplete == 0 on L1 P0) and
   the bite PM-0001-17/whole-as-lost; the readings-PR line in u4-build-plan.md section 11 went in at a36babd (`git show
   --stat` of both). On that tree: npt-judge --check-marks (r2's binary, sha256 65acacfe…) exit 0, 81 marks (5e563b9's
   message; the card); check.mjs all 11 steps, 531 and 609 tests, 0 failed, marks 81, ADR rows unchanged
   (.gate/u4r-merge/c62h-check.log, 15:59:43-16:14:08 UTC). The first try (.gate/netcode-proof-test/c62h-5e563b9589d3/
   README.txt) followed the advisor's wording, `--only PM-0001-17` alone, and read both bites INVALID ("pre:PM-0001-2:
   not in marks.json") through the judge's --only defect (JDG-ONLY-1, Follow-ups); the advisor's ruling says its own
   ruling had misread the judge, the error being the advisor's (advisor-u4r-c62h-full.md:7-13). The record is the
   second try, in the run scripts' form, `--only PM-0001-2,PM-0001-17` (.gate/netcode-proof-test/
   c62h-5e563b9589d3-record/README.txt and its two verdict files; marks.json sha256 9fb5be57…): whole-as-lost: bite
   shown, run valid, mismatches 0 (pass), the new check 48 (FAIL); tick16: bite shown, run valid, mismatches 2,880
   (FAIL), the new check 720 (FAIL); PM-0001-2 NOT-RUN with no runs, as expected; each verdict cmp-equal to the labelled
   diagnostic (per the README). Carry 62 (h) stands as committed (advisor-u4r-c62h-full.md:36-38).
3. **U4r-F1** (above; advisor-u4r-r2-full.md:19). **Done at f1f131b**, approved by the advisor's review of that head
   (advisor-u4r-f1-head-full.md, APPROVE, three Lows, no push). The details:
   - **Size:** 125 insertions and 48 deletions in the two run scripts, against the plan's "about 100" and the advisor's
     expected 110 to 125.
   - **The plan:** the advisor approved it with conditions 1 to 8 and choices Q1 to Q6 (advisor-u4r-f1-plan-full.md).
   - **The evidence:** 42 stagings and 47 runs, all PASS on their last run (.gate/u4r-f1/logs/summary.log; the harness
     is in .gate/u4r-f1/harness/). The first lim-* runs failed only the harness's own liveness check: kill(pid, 0)
     succeeds on a zombie. The fix reads the process state from /proc. That is not a loosened check: `pgrep -f` was
     empty in both runs (advisor-u4r-f1-head-full.md, check 11).
   - **The one round:** infra-engineer, APPROVE WITH FINDINGS (u4r-f1-review-full.md).
     - INF-F1-1 (Medium): the card's lock rule was wrong. It is fixed in the card's run procedure, items (5) to (7),
       with no code change. The wrong premise in condition 6 (b) was the advisor's own error (its head review, check
       17).
     - INF-F1-2 is fixed in the card's items (2) to (4). INF-F1-4's count is corrected on the card.
     - INF-F1-3 (Low): a call killed by a signal is reported "by a signal", without the signal's name. Its home: once
       check.mjs's run() returns the signal (INF-R2-5, below), both scripts name it. Step 17's peak-memory reading
       covers the memory side.
     - INF-F1-5 (Low): the quoted first status line loses its leading space (sh() trims it). Its home: an untrimmed
       read at the run scripts' next change.
   - **The advisor's three Lows on the head:**
     - ADV-F1H-1: fixed in the card's run procedure (`mkdir -p .gate/u4r-sets`; the getrusage wrapper for the judged
       sets).
     - ADV-F1H-2: the main session's own redact-check on the real tree and its gate --no-smoke run left no kept log.
       The redact claim holds by composition. F1's two files give 0 findings (redact.log), and spikes/ differs from
       5e563b9 only in those two files, where check.mjs's redact step gave 277 items and 0 findings
       (.gate/u4r-merge/c62h-check.log:17). The gate claim holds through CI: gate, smoke and toolchain were green on
       f1f131b (17:30 UTC). From now on, such runs' output is kept under .gate/<slug>/.
     - ADV-F1H-3: condition 6's two risks are in the card's run procedure but not in the plan's risks section (the plan
       stays as it was). Here they are, with the corrected lock mechanism:
       (a) The spike's tree hash covers results/. So a commit under spikes/netcode-proof while a set runs (an earlier
       set's results/ included) stops that set before the judge. Commit each set's results/ before the next set
       starts, and never commit under spikes/ during a set.
       (b) Node marks the lock's descriptor close-on-exec, so no npt-lab or npt-judge call holds the lane lock. If the
       script dies mid-call, the lock is freed at once, while the call runs on in its own session with no limit. So
       after any stopped or killed set, `pgrep -a npt-lab; pgrep -af npt-judge` must be empty before another lane
       takes the lock. Kill an orphan's group first (`kill -9 -- -<pid>`), then the script.
   - **INF-F1-1's optional header sentence:** deferred to the run scripts' next change, in the advisor's words
     (advisor-u4r-f1-head-full.md, "INF-F1-1's optional header sentence"). A comment-only push now would be a push for
     a Low.
4. **D4:** the runner's ticks per second in the main folder's own cache, the set's wall time redone (:20). **Done**
   (17:42:44-17:49:47 UTC, exit 0; .gate/u4r-d4/d4.log, written by d4.sh in one exclusive cache-run.sh call, with
   CARGO_TARGET_DIR unset).
   - **The build:** npt-lab's release build in the main folder's cache took 17.64 s. Its Compiling lines name
     /home/user/Shamble-Frontlines/spikes/netcode-proof/crates/….
   - **The runs:** r5's 15 timed runs, unchanged except for the binary's path, --commit (514a3c7, whose spikes/ equals
     f1f131b's) and --out. Each exited 0, and together they wrote 320 MiB of logs. R is ticks run divided by `time`'s
     real seconds (d4_table.py).

   | Scene, profile | Seed 1 | Seed 2 | Seed 3 | Slowest kept (ticks per second) | r5's slowest |
   |---|---|---|---|---|---|
   | SS P0 | 1,800 / 22.60 s = 79.7 | 1,800 / 25.85 s = 69.6 | 1,800 / 25.86 s = 69.6 | **69.6** | 79.6 |
   | SS P2 | 1,833 / 32.50 s = 56.4 | 1,833 / 33.81 s = 54.2 | 1,834 / 35.55 s = 51.6 | **51.6** | 63.1 |
   | SS P3 | 1,835 / 33.54 s = 54.7 | 1,834 / 33.41 s = 54.9 | 1,835 / 32.76 s = 56.0 | **54.7** | 60.1 |
   | SS B1 | 1,833 / 30.50 s = 60.1 | 1,833 / 31.36 s = 58.4 | 1,834 / 31.05 s = 59.1 | **58.4** | 67.1 |
   | ES P3 | 1,835 / 11.95 s = 153.5 | 1,835 / 12.30 s = 149.2 | 1,835 / 12.40 s = 148.0 | **148.0** | 161.6 |

   - **Slower than r5, by 8 to 18 %.** P0's seed 1 ran at r5's speed (22.60 s against r5's 22.61 s). The runs after
     it were slower. The load average was 0.11 at the start and 1.00 at the end, which is the run itself. The binaries
     differ too: r5's was built from 2fbec77 plus copy 5's uncommitted changes, before the fix pass, and D4's from
     f1f131b's tree. This measurement does not separate the two causes, and the slower R is the one that counts.
   - **7.3's estimate, redone at D4's slowest R:** each judged run is 18,335 ticks (300 + 18,000 + about 35). P0 now
     has six bites, since whole-as-lost joined -17's.

   | Item | Runs | Wall s at the slowest R |
   |---|---|---|
   | -17 measured (also -57's), P0 x 3 seeds | 3 | 790.1 |
   | P2 x 3 | 3 | 1,066.3 |
   | P3 x 3 | 3 | 1,005.3 |
   | B1 x 3 | 3 | 941.2 |
   | -17's bites (tick16, client-rounding, whole-as-lost) and -57's (pos-4cm, yaw-12bit, pitch-8bit), P0 | 6 | 1,580.2 |
   | -30 pre-check's recording (SS P0 seed 4, 9,000 ticks, --logs 0), at P0's R (upper bound) | 1 | 129.3 |
   | -30 pre-check, 3 pairs x 2 legs x 9,000 ticks, at P0's R (upper bound) | 6 | 775.6 |
   | **Total** | | **6,288.0 s = 104.8 min = 1.75 h** (r5: 5,240.9 s = 1.46 h) |

   - **Per set (runs only; the builds and the judge come on top):** pre-check about 15 min, P3 about 17, B1 about 16,
     P2 about 18, P0 about 40.
   - **The limit:** the longest judged run at the slowest R is 355.4 s. RUN_LIMIT_MS (3,600 s) is 10.1 times that.
     The code's comment says "about 12 times" from r5's figure. The margin is still ample, and it is re-checked
     against the judge's measured time before the P0 set (step 17).
5. **The disk plan**, in writing: 4,681 MiB free; kept logs of about 162 + 16 + 360 + 545 + 629 + 3,020 MiB plus the
   4 GiB floor and builds leave it about 4.1 GiB short; space only from the approved part-cleans; never delete a log;
   if P0 still cannot fit, it goes to Mustafa under Decisions needed (:21). **Done:** u4r-disk-plan.md, written before
   D4 and before any set.
   - **The projections:** taken from the runner's own planJudged and planPrecheck, with today's marks.json: pre-check
     16.2, P3 359.9, B1 545.3, P2 628.9 and P0 3,073.3 MiB. P0 has whole-as-lost's bite run in addition to the ruling's
     count, and the runner wants 7,169.3 MiB free at P0's start. On top come D4's 320 MiB, up to 400 MiB of release
     builds and up to 100 MiB of bin copies.
   - **The clean, done before D4:** with 4,470 MiB free, after D4 no set could start (the advisor's head review, next
     step 4, foresaw this case). So the one approved part-clean ran first, with no lane running: the main folder's
     cache's dev profile, `cargo clean --target-dir .gate/netcode-proof-test/target --profile dev`, with its test
     guards, under the lane lock (.gate/u4r-sets/clean-main-dev.log, 17:42:27-17:42:34 UTC, exit 0). Cargo removed
     29,079 files, and free space went from 4,467 to 10,592 MiB.
   - **What stays:** the shared target, check.mjs's warm cache.
   - **The projection with the clean:** about 5,090 MiB free after P0, so P0 fits without going to Mustafa.
6. **The pre-check set** after F1 and D4; the late-command pair counts only on 'result: VOID (L1 late_count 1)' (:22).
   **Done** (17:51-18:02 UTC, exit 0). Committed at d666aeb: results/u4r-precheck-30/faf42aeb034c/runs.txt, 345 lines.
   Kept log: .gate/u4r-sets/precheck-faf42aeb034c.log.
   - **The recording** SS-P0-s4 (9,000 ticks, wall 127.5 s) refused 0 rewinds. Its sha256 is 6933b56a…, the
     manifest's.
   - **The three pairs, all as expected; nothing goes to the advisor:**
     - clean: PASS, every World hash equal over 9,000 ticks (192.8 s);
     - wallclock: FAIL, as its bite expects: the first differing index is 3, and 8,997 of 9,000 differ (177.4 s);
     - late-command: VOID, with the whole line "result: VOID (L1 late_count 1)" (174.8 s).
   - **Condition 5 is met:** the measured build's Fresh npt-* lines and the bites build's Compiling and Fresh lines
     all name /home/user/Shamble-Frontlines/spikes/netcode-proof/crates/….
   - **Time and memory (peak.py):** wall 684.7 s, against D4's upper bound of about 905 s of runs at P0's R. Peak
     resident memory 538.1 MiB, the largest single process (a build's rustc or npt-lab).
   - **redact-check** on the spike folder with runs.txt: 278 items, 0 findings
     (.gate/u4r-sets/redact-precheck-faf42aeb034c.log).
   - **Raw logs:** 17 MiB, in .gate/netcode-proof-test/runs/u4r-precheck-30-faf42aeb034c/.
7. **The judged sets, one at a time: P3, then B1, then P2, then P0 last**; peak memory by getrusage (no /usr/bin/time
   here); B1's -17 and -18 INVALID, recorded and never counted; every run flagged for overflow resyncs to the advisor
   before it counts (P0's tick16 bite run expected, about 48); P0's judge memory projected first, over 10 GiB to the
   advisor (:23). Each set's lines are filled here after it.
   - **P3 (18:03:40-18:20:36 UTC, exit 0; results at 5afd4e7: results/u4r/0f066338dd8d/P3/; kept log
     .gate/u4r-sets/P3-0f066338dd8d.log; peak.py 1,018.1 s, 1,507.9 MiB):**
     - -57's L1 P3 part: PASS (3 valid runs).
     - -2: FAIL on everyone in view only (least per-slot share 0.352778, 0.085444 and 0.203556 against 0.99; its other
       ten checks pass on every run). So -17's L1 P3 part is INVALID by pre:PM-0001-2. That is not among the ruled
       INVALIDs (only B1's), so it went to the advisor before it counts.
     - **The advisor's ruling (advisor-u4r-sets-r1-full.md, Q1):** not as expected, and not a netcode fault. The
       cause is BRAWL-1. Under P3's loss, 1,026 to 5,172 of each client's roughly 18,300 ticks re-run a command, so
       the brawl's 16-step circle never closes and every soldier random-walks (bots/src/lib.rs:31-51, :206-215;
       room/src/input.rs take()). Within 60 to 210 s, one soldier reaches one of the six buildings. Its door shuts 6
       s of every 12 (world.rs:1203-1208; greybox.rs:89-99), and the room correctly drops it from every other
       client's set (room.rs:906-935). The onsets: ticks 70,336 (s1), 65,536 (s2) and 67,696 (s3). r2's 30 s suite
       and D4's 50 s runs were too short to reach one. The mirror (0 breaches) and an overflow resync (0) are ruled
       out.
     - **What follows:** -17's L1 P3 part stays INVALID: recorded, never counted, never re-run at this commit. It
       moves to BRAWL-1's fix and is re-run at that fix's commit, before U4g's judged B1 runs. -57's L1 P3 PASS
       counts. -2's SS part at P3 is recorded as FAIL. The 99 % bar stays.
     - Overflow resyncs 0, nothing flagged. Measured incomplete 43,117, 43,210 and 43,441 of 864,000 client-ticks
       (5.0 %, which is P3's loss).
     - Raw logs 518.8 MiB (du -sb), against the runner's 359.9 MiB projection: **1.44 times** (FACTS-1). The cause:
       lab.jsonl's -57 series grow through a run (P3 s1: 3,949 B a tick in minute 1, about 7,800 by minute 7), while
       the rates came from 20 to 30 s runs (RATES-1).
     - runs.txt's line "PM-0001-57 NOT-RUN L1 P0" is the mark's overall verdict, not this set's part (RUNS-LINE).
   - **B1 (18:29:45-18:46:14 UTC, exit 0; results at 73060fd: results/u4r/64f3307d5c7f/B1/; kept log
     .gate/u4r-sets/B1-64f3307d5c7f.log; peak.py 983.8 s, 2,260.5 MiB):**
     - -57's L1 B1 part: PASS (6,984, 2,750 and 88,063 on each run).
     - -17's and -18's L1 B1 parts: INVALID by pre:PM-0001-2 (everyone in view 0.129944, 0.268056 and 0.3075), as
       ruled: recorded, never counted. -2: FAIL on everyone in view.
     - Overflow resyncs 0; heals 480 per run; measured incomplete 31,346, 31,279 and 31,635.
     - Raw logs 821 MiB, against the runner's 545.3 MiB projection (1.51 times).
   - **The judge, measured apart** on one P3 run (judge-apart.sh; .gate/u4r-judge-apart/u4r-P3-0f066338dd8d-l1-p3-s1.log;
     178,922,504 B):
     - Full judging: 2.7 s and 641.1 MiB. Reading alone: 1.9 s and 426.2 MiB, so RR3-L6's read_log share is about
       70 %.
     - A first line, about 216 MiB plus 2.49 times the log MiB, was drawn through one judge point and two whole-call
       peaks. Those peaks are not the judge's alone: peak.py measures the whole set call, with cargo's builds, the
       npt-lab runs and the judge (FACTS-1). It gave 10.8 to 11.5 GiB for P0, over the advisor's 10 GiB line. The
       ruling (Q2) is to measure the judge on whole set folders apart (P3, B1, then P2) and project P0 at F x
       3,073.3 MiB, with F the largest byte ratio and at least 1.5. Those measurements are below.
   - **The judge on whole set folders, apart** (judge-apart.sh with no run; lane lock; scratch output;
     .gate/u4r-judge-apart/*-all.log):
     - P3: 518.4 MiB of logs. Full judging 7.6 s and 1,507.8 MiB; reading alone 5.8 s and 1,284.8 MiB (read_log
       share 76 %).
     - B1: 820.2 MiB, with -18 among the marks. Full judging 12.2 s and 2,260.6 MiB; reading alone 8.6 s and 1,851.4
       MiB (70 %).
     - With the one-run point (170.6 MiB: 641.1 MiB, 70 %), the three judge-only points fit about 215 MiB plus 2.49
       times the log MiB.
     - B1's byte ratio is 1.50, so F is at least 1.504 and P0's logs come to about 4,622 MiB. That projects about
       11.5 GiB of judge memory, over the 10 GiB line, so P0 needs the ruling's smallest change before it runs.
   - **The merged copies (the ruling's (A), by CHECKPOINTS.md section 10):** all 26 copies' HEADs are on the pushed
     spike branch. Three have an empty `git status --short --ignored` and were removed by plain `git worktree
     remove`: .gate/u4d2-prefix, .gate/u4df-prefix and .gate/u4e-prefix (free space 8,881 to 8,937 MiB). The other
     23 hold changed or untracked files, so git refuses them without --force, which is never used. They stay, listed
     on the card. They include U4r's copy 6 and .gate/u4r-snap1 and -snap2, whose work is in the branch through the
     merged patches.

## Decisions needed
None.

## For Mustafa
Nothing to decide or do now (advisor-u4r-r2-full.md:32). The merge into the spike branch needs no go from you: it is not
a merge to main, and nothing a player sees changes. One line, as you asked for SIZE-U4R: U4r has 8,362 lines after its
fix pass, against about 3,000 for its size M; your answer (a) covers it. Two heads-ups, neither a decision now: (1) the
judged runs need about 4 GiB more free disk than today; the approved cache cleans should cover it, and if they do not, it
comes to you under your LEFTOVER-DELETE rule; (2) one Low security weakness in the spike's own code checker
(SEC-U4R-R2-1) stays open as a follow-up and will be listed in PR #40's report, which needs your go anyway.
