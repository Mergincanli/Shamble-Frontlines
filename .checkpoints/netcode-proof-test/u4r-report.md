# U4r report: the L1 runner, the -30 pre-check and the run scripts (npt-lab's `scenario`, `replay`, `contract`, `precheck`)

Merged 2026-11-14 into the spike branch by the main session (this report's own commit, "U4r: the L1 runner, the -30
pre-check and the run scripts, merged after review round 2": advisor-u4r-r2-full.md:15). The approved brief:
u4r-brief.md rev 3.2 (sha256 59d570b8…, 1,909 lines: u4r-build.md:20-21; the advisor's check of rev 3.1,
advisor-u4r-brief.md:10-11: APPROVE WITH CONDITIONS on L1 to L5), built on 2fbec77 (the spike head, whose spikes/ is
the brief's `<base>` 614823c's: u4r-build.md:5-14). The bytes merged: wip/u4r.patch, sha256 21ca46be…, 26 files. The
runs after this merge: the r2 record run, carry 62 (h), U4r-F1 (f1f131b), U4r-F2 (791bc55), the pre-check and the four
judged sets are all done, and the judged runs are closed (advisor-u4r-sets-r2-full.md; last section). For U4's
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
- **U4r-F2, done at 791bc55 (before the P0 set):** the P0 set judged in two calls, with INF-F1-1's header sentence
  riding it in the advisor's words (condition 8). The record:
  - the plan, approved with conditions 1 to 9 (advisor-u4r-f2-plan-full.md);
  - the evidence: 17 stagings, all PASS, and on the real judge one call equals two (.gate/u4r-f2/);
  - infra-engineer's one round: APPROVE WITH FINDINGS (u4r-f2-review-full.md; INF-F2-1 fixed in the card's run
    procedure);
  - the advisor's review of the head: APPROVE (advisor-u4r-f2-head-full.md); CI green.
  - Size: 52 insertions and 22 deletions in one file.
  - ADV-F2H-1: the stagings ran on harness.mjs sha256 99f583a2 (sources.log). The kept copy, sha256 d0dd4ec3, differs
    only by the NOTES re-point to .gate/u4r-f2/scratch/ (INF-F2-3), made after the run. infra-engineer's re-run used
    the original.
- **U4r-F3, the one runner change, done at 493b3d7** (both run scripts, before BRAWL-1's re-run sets: the P0 ruling's
  next step 3):
  - **What it does:**
    - INF-F2-1's header sentence, and INF-F2-2's three wordings with the change of words the advisor allowed (Q1 (b));
    - INF-F1-5's untrimmed status read, in both scripts;
    - RUNS-LINE: runs.txt prints, under each judge call that exits 0, each part and bite holding a run of that call,
      copied from the verdict file;
    - RATES-1: BYTES_10MIN_57 from the largest measured run per profile (P0 603.7, P2 272.9, P3 183.3, B1 291.4 MB),
      with OTHER kept at 56 MB;
    - the judge's projected peak (302.2 + 2.3403 x MiB, the six-point fit) printed for each call, and a set whose call
      projects over 10,240 MiB stops before the builds.
  - **The plan:** approved with conditions 1-10 (advisor-u4r-f3-plan-full.md). F3P-2, the Medium, put every new line in
    judgeCall behind one guard.
  - **The evidence:** 25 of 25 stagings PASS; the helper test (setParts equals an independent reading of all five
    verdict files; all 19 verdicts match the one-table summary); node --check, redact-check (287 items, 0 findings) and
    gate --no-smoke (.gate/u4r-f3/logs/).
    - The evidence agent's summary named HEAD's copy "c5cd4d9c", which is F1's copy. sources.log records 18c1dcf6, the
      true HEAD copy, so the evidence holds.
    - The on-disk projection's value is proven by infra-engineer's own check (.gate/u4r-f3-review/logbytes-check.mjs,
      INF-F3-2), with the advisor's find sums (P3 543,998,603 B; P0/bites 1,856,944,887 B).
  - **CI:** gate run 43654541605 (gate and smoke) and toolchain run 43654541555, both green.
  - **The reviews:**
    - infra-engineer's one round: APPROVE WITH FINDINGS, two Lows (u4r-f3-review-full.md);
    - the advisor's review of the head: APPROVE (advisor-u4r-f3-head-full.md).
    - Both readings of the conditions are accepted: one guard helper used twice, and the on-disk line's "printed only"
      ending.
  - **Size:** 157 insertions and 34 deletions, against the planned 100/25 and the advisor's 100-130 and 20-30. That is
    under the 250-line "well past" line; the excess is the conditions' required text.
  - **The pre-check's projection**, carried in condition 8's words: the folder holds the recording (16,950,444 B, the
    entry's bytes) and marks.json (372,637 B). The projection counted the recording plus three pairs' output (75,492
    B), which goes into results/runs.txt. So the folder is 297,145 B over the projection, which is immaterial.
  - **Its follow-ups**, on the check.mjs follow-up's list below: INF-F3-1, ADV-F3H-1 and INF-F1-3.
- **BRAWL-1, widened, done: merged at b30314a, and its re-run sets P3, P2 and P0 count (results committed at fc41111,
  93509df and be41ce6; advisor-brawl1-sets-p0-full.md). The record is this report's last section, "BRAWL-1 (the
  bots' leash toward the centre, from the client's own decoded position)", with its own follow-ups** (Medium; the next
  npt-lab lane with N2-1, before U4g's judged B1 runs and before U4 closes; advisor-u4r-sets-r2-full.md, finding
  BRAWL-1, which replaces the sets-r1 fix line):
  - **For U4g's brief (BRAWL-1's round-1 ruling, 2026-11-15):** the advisor's words on B1's bursts are in
    u4g-brief-carry.md. They merge ADV-PT-M1, NA-B1-2 (c) and TE-B1-5's bound, copied by script from
    advisor-brawl1-r1-full.md:279-288 (sha256 c3abeccb…). In short:
    - "area 0 in three seeds" is no bound under B1, and a B1 judged run can carry a soldier into a door gap;
    - U4g's plan says how a B1 building entry reads in -17, or measures how often it happens, before its judged B1
      runs.
  - **The cause:** the brawl's walk is open loop. It closes only if every step moves its full length. A re-run command
    (P2, P3, B1) or a step cut short by collide() leaves a net offset that nothing corrects. collide() cuts steps on
    crates, the wall stand-in, closed doors and the fight vehicles' parts, at every profile, P0 included.
  - **The fix:** plan first, with the advisor's plan review; reviewers test-engineer, netcode-architect and
    rust-reviewer.
    - (a) Each brawler steers by its own soldier's position, as the host tells it from the decoded snapshot, never from
      the server's state. For example, a leash toward its team's home or the cluster centre, in cm, with no
      rate-derived constant. The option "a walk that closes despite re-runs" is withdrawn.
    - (b) A run figure names every soldier whose feet are in a building area, per measured tick.
    - (c) Evidence:
      - a unit test under skipped and blocked steps;
      - a sim-only test with no network (18,300 ticks, seeds 1 to 3, with a command every tick and with a seeded
        skip pattern), showing no soldier in a building area on any tick;
      - one 10-minute SS P0 seed-1 run and one SS P3 run, each with everyone in view at 0.99 or more.
    - (d) The re-run sets P3, P2 and P0 (with its six bites) at the fix's commit. -2's SS part passing everywhere is
      the fix's proof. -17's parts and bites count from that commit. -57's results there are recorded beside U4r's,
      and any difference goes to the advisor.
- **The kept-logs decision (to Mustafa under Decisions needed, before the re-run sets if the approved cleans cannot
  hold them; drafted with BRAWL-1's plan):** free space, the 5,385 MiB of kept logs, the refill figures, and options
  within LEFTOVER-DELETE (a narrow kit rule, never a one-off delete).
- **U4r-F1, done at f1f131b** (the judged runs, item 3) (before any c-u4r-runs.mjs set; plan first, infra-engineer as
  its one round): INF-R2-1 (compare only the
  spike's tree hash and its own status before the judge or r2.txt), INF-R2-3 (no set folder before the first floor),
  INF-R2-4 (a 3,600 s limit on each npt-lab and judge call) (advisor-u4r-r2-full.md:19, :38-41, :108-118). Until then
  only r2's record run may run, with saves paused (:40). Two Lows ride it: RUN-L1 (r2.log's provenance block is empty:
  both run scripts' sh() keeps cargo's output; both print cargo's "Compiling/Fresh npt-* (path)" lines from stderr into
  the runner log; the record holds through r2.txt's clean-tree check, commit and sha256s: advisor-u4r-r2run-full.md:17);
  and c-u4r-r2.mjs's whole-as-lost suite (:87) gets marks "PM-0001-2,PM-0001-17" (header and --only, tick16's form) and
  the expect line "contract PASS; bite PM-0001-17/whole-as-lost shown: mismatches 0 pass, L1 P0 loses no group 48
  FAIL; the run valid", for a future run only (advisor-u4r-c62h-full.md:45). The check itself was built on the
  advisor's rulings and is kept by Mustafa's C62H-KEEP (a) of 2026-11-15 16:35 Amsterdam (history.md:2131-2146).
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
  :87-90, :122-125; advisor-u4r-r1-full.md:17), and BRAWL-1's ADV-PT-L3 (TICK_NAMED becomes
  /_TICKS$|_TICK_COUNT$|CATCHUP/, NOT_TICK_COUNTS gains MAX_ENTRY_TICK_COUNT with its reason, and check.test.mjs gains
  one case; advisor-brawl1-points-full.md:87-91). It also carries three items in c-u4r-runs.mjs, since it edits
  judgeCall's "by a signal" phrases anyway (advisor-u4r-f3-head-full.md, next step 3):
  - INF-F1-3: both scripts name the signal;
  - INF-F3-1: collapse whitespace in guarded()'s catch (`String(e?.message ?? e).replace(/\s+/g, " ")`);
  - ADV-F3H-1: the RATES-1 comment reads "the builder's rates from 30 to 50 s runs (u4r-build.md:739-751)".
- **The next npt-lab lane, done in BRAWL-1's Part 1** (merged at b30314a; EA-U4R-1 in the narrow form the plan ruled,
  with no one-loop driver; the record is this report's BRAWL-1 section, "Part 1, the refactors and N2-1"): N2-1 = TE-R2-1 (the roster check in closed_checks), TE-R2-2 (the untested overflow
  refusals), RR4-L2 (bounds on a run's last tick and the bursts), RR4-L1 (a 36,000-tick manifest cap, before U4h adds its
  carry-87 entry at the latest), with round 1's EA-U4R-1's shared loop, EA-U4R-2's `Sim` type and one server
  constructor (before the pre-check's re-run after U4h) and RR3-L2 (a) (advisor-u4r-r2-full.md:24, :66-104;
  advisor-u4r-r1-full.md:17).
- **Round 1's others** (advisor-u4r-r1-full.md:17; u4r-build.md:1936-1950): NC-U4R-4 (U4g's brief); NC-U4R-5 with
  EA-U4R-7's logs half (the first logged replacement: U4f or U6); EA-U4R-4 (the next lane that opens crates/sim);
  EA-U4R-5 (the U6 brief's bots'-view follow-up); NC-U4R-7's derivation (carry 18's route); T-6's contract half (the next unit that edits contract.rs); INF-U4R-6 (U4h's brief,
  now with INF-R2-2's base above); SEC-U4R-4's U6 half (U6's TickLoop carry).
- **Housekeeping:** /tmp/npt-lab-tests holds 124 folders, none deleted (Mustafa's LEFTOVER-DELETE; u4r-build.md:1927-1929).

## Open security findings (for PR #40's report)
- **SEC-U4R-R2-1 (Low, open):** check.mjs's attribute rule sees only attributes, so a macro could build a `#[path]`
  that compiles a non-.rs file into npt-hash-wasm or npt-tcpinfo; it needs committed, reviewed code, and no crate has a
  macro today. Until fixed, PR #40 stays needs-mustafa for it, and the advisor reads any diff that adds a macro there or
  in npt-sim (advisor-u4r-r2-full.md:52-55).
- **SEC-U4R-R2-2 (Low, message only):** a profile refusal under a quoted table header names line 1; the refusal itself
  fires (:59-62).

## The judged runs (all done, items 1 to 7; closed by advisor-u4r-sets-r2-full.md, Q3)
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
   the bite PM-0001-17/whole-as-lost (built on the advisor's rulings; kept by his C62H-KEEP (a),
   history.md:2131-2146); the readings-PR line in u4-build-plan.md section 11 went in at a36babd (`git show
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
     - **Correction (ADV-SR2-1):** the cause also includes blocked steps at every profile (advisor-u4r-sets-r2-full.md,
       Q1). The sets-r1 claim that "at P0 ... the circle closes" rested on r2's 30 s command counts, and was the
       advisor's error. No verdict rested on it.
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
   - **P2 (18:55-19:11:54 UTC, exit 0; results at 1eacb60: results/u4r/ab10f058a828/P2/; kept log
     .gate/u4r-sets/P2-ab10f058a828.log; peak.py 1,011.4 s, 2,173.5 MiB):**
     - -57's L1 P2 part: PASS (3 valid runs).
     - -2's SS part: run s1 passes every check (everyone in view 1.0); s2 and s3 fail on everyone in view only (0.5875,
       0.7295). So -17's L1 P2 part is INVALID by pre:PM-0001-2: recorded, never counted, and moved with -17's L1 P3
       part (the ruling's Q1 (3)).
     - **BRAWL-1's fingerprint** (brawl_fingerprint.py; .gate/u4r-sets/fingerprint-P2-ab10f058a828.log): onsets at
       ticks 73,816 and 76,325, with every slot holding before; the others' count folded on (t - 63,736) mod 360 sits
       at 0 for about 150 to 180 ticks and then climbs to about 45; overflow resyncs 0; very large units held in
       0.9839 and 0.9937 of sets.
     - **One difference from P3:** the least slot holds everyone in view on 795 of 8,220 (s2) and 842 of 5,711 (s3)
       ticks after the onset, against P3's 4 to 50. It went to the advisor to confirm the attribution.
     - **The advisor's ruling (advisor-u4r-f2-head-full.md, FP-P2): BRAWL-1's fingerprint.** It confirmed this from
       its own reads of the logs:
       - (1) s2's onset is at tick 73,816, on a door's close edge (phase 0 of the 360-tick cycle). s3's is at 76,325
         (phase 349), inside door 5's closed half: a soldier walked into a building whose door was already shut.
       - (2) After the onset, all 48 slots are at 0 in every closed half.
       - (3) s3 has one soldier in one building: slot 33's view returns 53 to 162 ticks after door 5 opens, while the
         others return in a median of 2 to 3 ticks.
       - (4) s2 has two soldiers in two buildings: slot 6 from the onset, and slot 28 behind door 2 from about tick
         75,436, where the others wait door 2's 60-tick gap.
       - (5) At P2's lower loss the re-adds finish inside the 180-tick open half, so the least slot holds only late in
         the open half. That is why it holds 795 and 842 ticks against P3's 4 to 50.
       - (6) Overflow resyncs 0; very large units held in 1.0000, 0.9839 and 0.9937 of sets.
       - **What follows:** -17's L1 P2 part moves with L1 P3 to BRAWL-1's re-run at the fix's commit. s2 shows that
         more than one soldier can drift, so BRAWL-1's run figure must name every soldier outside the cluster.
     - Raw logs 778.6 MiB of .jsonl against the runner's 628.9 MiB projection (1.24 times). Measured incomplete 17,260,
       17,089 and 17,532 of 864,000 client-ticks (2.0 %).
   - **P0 (21:28:45-22:07 UTC, exit 0; results at 930d2a6: results/u4r/051995db9cea/P0/, runs.txt, verdict.json and
     verdict-bites.json; kept log .gate/u4r-sets/P0-051995db9cea.log):**
     - **The run:** judged in two calls (U4r-F2), both exit 0. peak.py 2,316.2 s and 4,356.9 MiB, the bite-runs
       judge's peak. redact-check 287 items, 0 findings. The head's sha256s equal P3's, B1's and P2's.
     - **-57:** L1 P0 PASS (6,984 / 2,750 / 88,063 on each run; p99 5,719 / 5,718 / 5,715). Its bites are shown:
       pos-4cm 34,830, yaw-12bit 46,692, pitch-8bit 440,312, each failing only its own check.
     - **-2:** SS FAIL on s1 only (0.861889). Its other ten checks pass on every run; shots 694.16 / 694.19 / 693.98.
     - **-17:** L1 P0 INVALID. s1 is invalid. s2 and s3 are valid, with 0 mismatches of 878,352 compared and 0
       incomplete in each. Its three bites (tick16, client-rounding, whole-as-lost) are invalid at the same 0.861889,
       since they are seed-1 runs with the same drift.
     - **The cause (advisor-u4r-sets-r2-full.md, Q1):** BRAWL-1, through a second trigger. It is not a netcode fault.
       - (a) Not loss, timing or decoding. P0 has no loss: s1's commands are 878,352, with 0 steps without one, late
         0, overflow resyncs 0 and measured incomplete 0.
       - (b) One soldier behind one door. The onset at tick 77,476 is door 2's 77th toggle: door 2 starts open and
         shuts on that tick. The others' count is 47 in door 2's open half and 0 in its shut half, re-added within one
         30-tick bin since nothing is lost. Slot 21 is in team 1, whose home is about 8.7 m from building 2's door.
         The building is inferred, since no log carries positions.
       - (c) It is deterministic: client-rounding is identical, and tick16's onset is at the same elapsed tick
         (13,740).
       - (d) The mechanism: the walk is open loop, and collide() cuts steps on crates, the wall stand-in, closed doors
         and the four fight vehicles' parts (world.rs:1233-1285). Nothing pulls a soldier back.
       - (e) The sets-r1 claim that "at P0 the circle closes" was wrong (ADV-SR2-1). The step that held this case for
         a ruling anticipated it, so nothing counted wrongly.
     - **The new home:** -17's L1 P0 part and its three bites move to BRAWL-1's re-run sets at the fix's commit, with
       its L1 P3 and L1 P2 parts.
     - **The tick16 flag (Q2):** 48 overflow resyncs, one per client, all at tick 65,615, all in the after phase (65
       after ticks, not capped), none in the measured window, no kick. Each client had 79 steps without a command and
       20 rewinds refused, as the wrapped 16-bit header predicts. The run counts for nothing: it is invalid by
       pre:PM-0001-2. At U4r's commit, tick16's "shown" rested on r2's 30 s record (c62h: shown, 2,880 mismatches, the
       new check 720). From BRAWL-1's P0 re-run set on, it rests on that set's 10-minute bite run (shown, 12,151
       mismatches, the new check 720; advisor-brawl1-sets-p0-full.md, "The tick16 bite").
     - **The bites' mechanics** (not a judged result): whole-as-lost's measured incomplete 48; tick16's 720 and 48
       resyncs. Each bite bit as designed.
     - **Bytes and disk:** 3,248.3 MiB of logs against 3,073.3 projected (1.057). Free space 13,233 MiB afterwards.
   - **U4r's judged results in one table** (advisor-u4r-sets-r2-full.md, Q3 (3)):

   | Mark and part | Result |
   |---|---|
   | -57 L1 P0, P2, P3, B1 | PASS, counted; its three bites shown |
   | -17 L1 P0, P2, P3 | INVALID by BRAWL-1; moved to its re-run sets |
   | -17 and -18 L1 B1 | INVALID as ruled (U4g's) |
   | -17's three bites | invalid; moved |
   | -2's SS part | FAIL at every profile, on everyone in view only |
   | -17's valid runs (P0 s2, s3; P2 s1) | 0 mismatches in all 2,618,700 snapshots compared |
   | the -30 pre-check | as recorded (item 6) |

   - **The judge measurements, and RR3-L6 closed** (Q3 (4)):
     - read_log's share of the judge's time: one P3 run 70 %, the P3 folder 76 %, B1 70 %, P0's measured half 56 %,
       P0's bites half 79 %. P2's 40 % is not usable, because its page cache was cold after the restart.
     - Its share of peak memory: 66 / 85 / 82 / 81 / 78 / 82 %.
     - The longest call took 27.2 s, against RUN_LIMIT_MS 3,600 s. So time does not matter at these sizes, and
       npt-log needs no one-pass change for time. Memory is the limit: reading holds about 80 % of the peak. The split
       and the runner change's projection handle it.
     - The fits: the four-point fit (212.3 + 2.5062 x MiB) over-predicted P0's halves by 85 and 294 MiB, so it is
       conservative. The six-point fit is 302.2 + 2.3403 x MiB, with residuals of -90 to +70 MiB.
     - A single call at P0's actual bytes would have projected about 8.2 GiB (by the four-point fit; about 7.7 GiB by
       the six-point fit: advisor-u4r-f3-plan-full.md, F3P-1). The split was needed only because F was
       1.504, and it changed no verdict (U4r-F2's proof that one call equals two).
     - RR3-L6 is closed; it leaves "Round 1's others", and no STATUS line is needed.
   - **Byte ratios, RATES-1's inputs** (Q3 (5)): P3 1.44, B1 1.504, P2 1.238, P0 1.057.
   - **The cleans and the disk** (Q3 (6)):
     - (A) 8,881 to 8,937 MiB; (B) 8,138 to 8,618; (C) 8,618 to 16,492; 16,490 at P0's start; 13,233 after.
     - Kept raw logs total 5,385 MiB: pre-check 17, P3 519, B1 821, P2 779, P0 3,249.
     - The shared target is cold (1 MiB). Its next refill, about 9 GiB, leaves about 4.2 GiB free.
   - **Corrections** (Q3 (7)):
     - ADV-SR2-1: the BRAWL-1 cause, as above.
     - TICK16-WORDING, as corrected by advisor-brawl1-sets-p0-full.md ("The tick16 bite"; ADV-P0-1): "tick16's run is
       valid and the bite shown, failing both -17 checks. Its measured incomplete is 720: 48 clients times the 15
       measured ticks from the wrap (65,536) to the window's end (65,550). Every client has at least one overflow
       resync, all in the after phase, none in the measured window, and no kick. The count and the ticks are not fixed:
       the run ends only when every client's last measured ticks resolve, and while one client's queue fills slowly,
       the others can resync again, each after its forced full state was acknowledged." It replaces the sets-r2
       wording "one overflow resync per client", which held only while every client's queue filled on the same tick.
       r2's resyncs came at wrap + 60, and U4r's P0 bite run had 48, one per client, at wrap + 79
       (advisor-u4r-sets-r2-full.md:30, :34). BRAWL-1's P0 bite run had 94 on 48 clients, the first at wrap + 75
       (this report's BRAWL-1 section).
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
**After the judged runs (advisor-u4r-sets-r2-full.md):** nothing to decide or do now. All of U4r's test sets have run.
- **Accuracy:** the position-accuracy check passed in every setting.
- **The main check** (every snapshot a client decodes is exactly the server's state) has no counted result yet in
  this unit. The test bots walk blind circles and get knocked off course, by lost commands or by bumping into crates,
  the wall or a moving vehicle, until one drifts into a building. When its door shuts, the server rightly hides that
  bot, which breaks the test's "everyone in view" load condition. Those runs are recorded as invalid and move to
  re-runs after a bot fix, which must make bots steer back by where they actually are. In every run where the check
  could be judged, about 2.6 million snapshots in all, it found zero errors.
- **A heads-up:** the kept test logs now take about 5.4 GB. Once the build caches refill, the disk sits near its safety
  floor, so before the re-runs you get a decision about the kept logs, under your rule that nothing is deleted outside
  the approved methods.

**At the merge (advisor-u4r-r2-full.md:32):** The merge into the spike branch needs no go from you: it is not
a merge to main, and nothing a player sees changes. One line, as you asked for SIZE-U4R: U4r has 8,362 lines after its
fix pass, against about 3,000 for its size M; your answer (a) covers it. Two heads-ups, neither a decision now: (1) the
judged runs need about 4 GiB more free disk than today; the approved cache cleans should cover it, and if they do not, it
comes to you under your LEFTOVER-DELETE rule; (2) one Low security weakness in the spike's own code checker
(SEC-U4R-R2-1) stays open as a follow-up and will be listed in PR #40's report, which needs your go anyway.

## BRAWL-1 (the bots' leash toward the centre, from the client's own decoded position)

_The lane BRAWL-1 (Medium), homed by advisor-u4r-sets-r2-full.md, finding BRAWL-1, with the items the r2 ruling put in
the next npt-lab lane (brawl1-plan.md:4-6). Merged into the spike branch on 2026-11-15 at b30314a. Its re-run sets
count (advisor-brawl1-sets-p0-full.md, Summary)._

**In short** (each figure's source is given in the subsections below):
- The test bots now steer back toward the middle by the position their own client decoded, never by the server's
  state. That is the fix for BRAWL-1, the drift that carried a bot into a building and made -17's U4r runs invalid.
- Two builder copies built it. Two review rounds found no Critical or High. The advisor's review of the pushed head
  approved it with three record-keeping Lows (advisor-brawl1-m6-full.md, Verdict). Its size is 1,266 changed lines,
  against about 700 planned.
- Both 10-minute evidence runs passed every bar. The three re-run sets P3, P2 and P0 all count:
  - -2's stress-scene part passes at every profile, with everyone in view 1 on every run. That is the fix's proof.
  - -17's L1 P3, P2 and P0 parts pass with 0 mismatches.
  - All six bites are shown.
  - -57 passes everywhere.
- Two lines go forward: EVQ-LOAD, the event queue's cap under full view, for U4f and U4g; and a corrected
  TICK16-WORDING.
- Closed: the advisor read this section against the files (item 3c of advisor-brawl1-sets-p0-full.md; not a review
  round) and found it true once the corrections in advisor-brawl1-close.md were made. BRAWL-1 is recorded closed.
  PR #40 stays a draft, and nothing here merges to main (advisor-brawl1-sets-p0-full.md, "Unchanged").

### The plan and the build

- **The plan:** brawl1-plan.md, written by the main session on 2026-11-14 and committed at 9658c10 at 23:30 UTC (its own first line says 'about 23:45 UTC').
  - The advisor's plan review: APPROVE WITH CONDITIONS 1-18 (advisor-brawl1-plan-full.md, workflow wf_533068a9-b2b,
    sha256 a7e68b90…).
  - Its four Mediums:
    - BP-1: run_leg's half of the refactors needs its own proof (condition 6);
    - BP-2: the leash's bounds and the stuck print (conditions 1-2);
    - BP-3: the sim-only test feeds an 8-step-old position and prints a B1 pattern (condition 3);
    - BP-4: the merge step in the M-list form (condition 9).
  - Its Lows: BP-5 to BP-10 and ADV-BP-1 (:65-105).
  - The choices Q1 to Q8 are all (a). The plan's R_IN of 1,200 cm is refused: R_IN is at least 1,300 cm, and R_OUT
    lies from R_IN + 100 up to 1,500 cm (:9, :30).
- **Planned size:** about 700 changed lines, 450-750 with the conditions. "Well past" is more than about 1,400
  (advisor-brawl1-plan-full.md:109; brawl1-plan.md section 12).
- **The builder's brief:** brawl1-brief.md.
  - The advisor's conformance check: CONFORMS AFTER EDITS, edits E1-E23 (advisor-brawl1-brief-full.md, workflow
    wf_12d3f66b-ca1, sha256 e0cb1a99…).
  - The edits were applied word for word. The brief is sha256 921c3243…, committed at 926a6cb (the card, "Earlier NOW
    (2026-11-14 about 23:50 UTC)").
- **How it was built** (the card, same block):
  - Copy 1's first launch (workflow wf_1d4a86b9-79f) was cut by the spend limit at 00:00:39 UTC and wrote nothing.
  - Its relaunch got a worktree on main (4460ef5) and stopped at its start check.
  - The main session then made the worktree by hand at the spike head 352a245: .claude/worktrees/brawl1, spike tree
    abad0635.
  - Copy 1 (agent a4be43b1f756e45dd) built there from about 02:45 to about 04:10 UTC.
  - The lesson, on the card: neither the workflow's nor the Agent tool's worktree isolation bases on the spike branch
    now.
- **Part 1, the refactors and N2-1** (brawl1-build-report.md:10-66):
  - EA-U4R-2: `pub(crate) type Sim = World` and one `Server::build`, used by run, run_leg and lab_room.
  - EA-U4R-1, in the narrow form: `struct Wake` and `fn wake` cover steps 1-2 plus the due count, and run_leg runs on
    a SimClock. There is no one-loop driver.
  - N2-1 with TE-R2-1: closed_checks gains its fourth check. Under the leave plant two of the four checks fail by name;
    under the roster plant the new check fails alone (p1-plant-leave.log, p1-plant-roster.log).
  - TE-R2-2 with RR4-L2: parse() bounds the run's last tick and the bursts, and fp3 pins each message.
  - RR4-L1: the 36,000-tick manifest cap, built as `MAX_ENTRY_TICK_COUNT` (hash.rs:239; the points ruling, Point 1).
  - RR3-L2 (a): on a no-log run, filed() only resolves.
- **Part 1's proofs at the old pins** (condition 6; brawl1-build-report.md:61-66):
  - (a) check.mjs: all 11 steps pass, 532 and 610 tests (p1-a-check2.log). The first run failed on Part 1's own code
    and was run again on the fixed code. The points ruling accepts this as not a retry.
  - (b) det-u4.mjs: PASS in 232.9 s, non-cargo sum 222.0 s (p1-b-det.log).
  - (c) The precheck regeneration: 6933b56a, "as the manifest's" (p1-cd.log).
  - (d) The clean pre-check pair: 109 lines against faf42aeb's runs.txt:17-125. Only the build line's commit and binary
    sha256 differ (p1-cd.log). The advisor re-checked it (advisor-brawl1-points-full.md:36).
  - (e) The precheck tests' prints before and after Part 1 are equal, apart from the wallclock bite's L1 hash and
    libtest's filtered-out count. Accepted as ADV-BP-3 (the corrections, below).
- **Part 2, the fix** (brawl1-build-report.md:68-95):
  - **The read item (npt-client, condition 5):** `pub fn own_soldier(&self) -> Option<([i32; 3], bool)>`. It returns
    the own soldier's decoded grid position and dead bit, from the newest complete tick filed Hash, with ticks compared
    by the endpoint's wrap rule. It is the lane's only new pub item.
  - **The decision (npt-bots, condition 4):** a crate-private `next_command(newest, view, own)`, which step() calls
    with `endpoint.own_soldier()`.
    - The leash applies to Kind::Brawl only.
    - `toward_centre` works in integers, and every vector has x² + y² at most 127² (condition 1).
  - **The leash:** R_IN 1,300 cm and R_OUT 1,400 cm, with no sideways part.
    - That leaves 4 m to the 1,800 cm door entry.
    - These were the first values tried; nothing was tuned.
  - **The figure (condition 10):**
    - room.jsonl's `in_building_slot_area`, on every logged tick where it is not empty, in any phase;
    - Counts' measured total, printed in the summary line as "in a building: N slot-ticks (slots …)";
    - one marks/SERIES.md row, "none (recorded)";
    - no judge change and no marks.json change.
  - **The tests:**
    - the steering unit test and the npt-client read test;
    - the sim-only test's short copy, seed 1 over 120,000 ms, in every check.mjs: 10.61 s in the measured build and
      10.71 s in the bites build;
    - the full test, `#[ignore]`, seeds 1 to 3 at 610 s each, run by name in release: 23.52 s
      (brawl1-build-report.md:99-109).
- **The full test's figures.** They were unchanged after the fix pass and at M3 (m3.log, the full test's nine lines;
  brawl1-build-report.md:111-115):

  | Seed | (i) every tick: largest distance | (ii) skips: largest distance | (ii) skips per 1000, longest streak | (iii) B1's bursts: largest distance, at |
  |---|---|---|---|---|
  | 1 | 1,575 cm | 1,654 cm | 304, 6 | 1,832 cm, (1734, -592) |
  | 2 | 1,575 cm | 1,720 cm | 303, 6 | 1,832 cm, (1671, -751) |
  | 3 | 1,570 cm | 1,686 cm | 305, 6 | 1,832 cm, (1586, -917) |

  In all nine runs no soldier was in a building, and none was named stuck.
- **Pattern (iii), B1's bursts:** printed only, never barred (condition 3 (iii)).
  - The builder's reading, "a fixed radius stops it" with the 14 m vehicle as one candidate, is withdrawn (FP-9 item 2:
    stand-ins are off in the test, and no solid lies within 1.6 m of the three maxima).
  - The advisor's reading, labelled as computed: 1,832 cm is the leash's own reach under a 30-tick re-run. It passes the
    1,800 cm door entry, so "area 0 in three seeds" does not bound B1 (ADV-PT-M1; its home is under the corrections).
- **The pins.** Condition 7 allows exactly three entries. Their bytes are unchanged; only the sha256 values move
  (brawl1-build-report.md:125-131):

  | Entry | Manifest | Old sha256 | New sha256 |
  |---|---|---|---|
  | SS-P0-s4 | precheck-recordings.json | 6933b56a… | 1f00d2d5… |
  | SS-P2-s1 | det-recordings.json | 5b79abb2… | 1ec25de7… |
  | ES-P3-s2 | det-recordings.json | fad8f67d… | 6abf7643… |

  - Reproduced at copy 1's end: det-u4 PASS in 227.2 s, non-cargo sum 214.5 s (final-det.log), and the regeneration
    "as the manifest's" (brawl1-build-report.md:134-136).
  - Reproduced again at copy 2's end (FP-8) and at M3 (below). The advisor hashed them itself at M6
    (advisor-brawl1-m6-full.md, check 7).
  - The manifests are det a2faf3ea… and precheck 5ccf57f0… (brawl1-copy2-report.md:62).
  - check.mjs at copy 1's end: all 11 steps pass, 535 and 613 tests (final-check.log; brawl1-build-report.md:140).
- **The size** (SIZE-CHECK, by numstat against 352a245, SERIES.md included):
  - 998 changed lines at round 1's snapshot ee2973aa: 875 inserted and 123 deleted, in 10 files
    (advisor-brawl1-points-full.md:5). Copy 1's own count of 992 left out SERIES.md's 6 lines
    (brawl1-build-report.md:144).
  - **1,266** after the fix pass: 1,139 inserted and 127 deleted, at snapshot c2561b6c (brawl1-copy2-report.md:63;
    M6's check 3).
  - That is about 1.8 times the ~700 planned (450-750 with the conditions), and under the 1,400 "well past" line. So no
    SIZE-CHECK note was due. The round-1 ruling expected about 1,270 (advisor-brawl1-r1-full.md, The size).
  - npt-bots' lib.rs took 426 of round 1's 998 lines, about 330 of them in its test module (the card, "(3)
    SIZE-CHECK").

### Round 1 and its ruling

- **Where round 1's record sits** (ADV-M6-L3):
  - copy 1's report: brawl1-build.md:251 ("## Report"), word for word in brawl1-build-report.md (sha256 f8df429d…);
  - copy 2's fix-pass report: brawl1-build.md:620 ("## Fix pass, round 1: report"), word for word in
    brawl1-copy2-report.md (sha256 b089826a…);
  - the four seats: brawl1-review-r1-full.md (sha256 fc979c63…);
  - the advisor's points ruling: advisor-brawl1-points-full.md (sha256 0c1f6f5d…);
  - the round-1 ruling: advisor-brawl1-r1-full.md (sha256 86bec63b…).
- **The points ruling** (workflow wf_bbfeece0-4c7, on snapshot ee2973aa): no diff change (advisor-brawl1-points-full.md,
  Summary and Rulings).
  - Point 1, TICKS-NAME: `MAX_ENTRY_TICK_COUNT = 36_000` stands, because RR4-L1's ruled name fails check.mjs's
    TICK_NAMED rule (ADV-BP-4).
  - Point 2, PROOF-E: proof (e) is accepted with its two differences (ADV-BP-3).
  - Point 3: r10_carry_82's boundary move is accepted.
  - The zero-test reruns and the size (998) are accepted.
  - Its findings: ADV-PT-M1 (Medium: pattern (iii)'s reading, for U4g), and ADV-PT-L1, -L2 and -L3 (Low).
- **Review round 1** (the same workflow; snapshot ee2973aa; packet brawl1-packet.md, sha256 16eab462…):
  - The seats: test-engineer, netcode-architect, rust-reviewer and engine-architect, opus at effort xhigh, in two
    batches (condition 11).
  - All four APPROVE WITH FINDINGS: 3 Medium and 20 Low, no Critical or High (brawl1-review-r1-full.md:5, :83, :110,
    :185).
  - The Mediums:
    - TE-B1-1: nothing showed that the sim-only test can fail, and nothing checked that 48 Soldier records are seen;
    - TE-B1-2 and RR-B1-1, one gap: nothing showed that the lab's building figure can read above 0.
- **The round-1 ruling** (workflow wf_59e64b31-fa2; advisor-brawl1-r1-full.md, Summary):
  - The three Mediums are confirmed and fixed in one fix pass.
  - 16 of the 20 Lows ride it; they are tests and doc words only. RR-B1-7 and EA-B1-1 ride in part.
  - Recorded: TE-B1-5, TE-B1-8, ADV-BP-3, ADV-BP-4 and ADV-PT-M1. No action: RR-B1-6 and EA-B1-5. A follow-up:
    ADV-PT-L3.
  - A correction to a reviewer: TE-B1-5's "obstacle at 18.3 m / the 14 m vehicle" is wrong, because stand-ins are off
    in the test (:342-345).
  - The pins must not move, because on the recording path the fix pass changes only doc comments and Server::new's
    visibility (The pins, :194-211).
- **FP-0, the main session's runner line (TE-B1-7)** (the card, "FP-0 DONE"; advisor-brawl1-r1-full.md:29-37):
  - Both runners' headers gained the `diff …; untracked …` line: u4r-run.sh is now sha256 2f754d12…, and main-run.sh
    f0b7a3d0….
  - The two lines were committed at 7402dac (the 05:41 autosave). The .diff files u4r-run.sh.te-b1-7.diff and
    main-run.sh.te-b1-7.diff were cut from 7402dac's own diff by fp0_diffs.py and committed at ab3b7d7 (06:41 UTC).
    `bash -n` prints nothing for both.
- **The fix pass, copy 2** (agent a2e54bc7201dcffed, in the same worktree). It was cut by the spend limit at 06:53:13
  UTC and resumed at 07:13, with no plant in place (brawl1-copy2-report.md:10).
  - **Three new tests, +3 per build (538 and 616)** (:34-37):
    - bots: `tests::brawl1_a_soldier_teleported_into_building_0_reads_area_1`;
    - logs.rs: `logs::tests::brawl1_the_building_figure_counts_a_soldier_in_a_building_in_measured_ticks_only`;
    - endpoint.rs: `brawl1_own_soldier_compares_ticks_by_the_endpoint_s_wrap_rule`.
  - **The plants.** Each file came back to its exact sha256 (:41-49):
    - P1, the leash off, bit on area alone, in both builds and in the full test (Skips for seeds 1-3). The short copy's
      Skips run had 7,728 soldier-ticks in buildings, at most 3,813 cm out. So the door-entry bar was not added.
    - P2 cut the figure and failed at logs.rs:924.
    - P3 and P4 failed at their new client asserts.
  - **FP-8.** Each log's header shows diff c2561b6c (:52-63):
    - check.mjs: all 11 steps pass, 538 and 616 tests (final-fp-check.log);
    - det-u4.mjs: PASS in 227.3 s, non-cargo sum 213.3 s, reproducing 1ec25de7 and 6abf7643 (final-fp-det.log);
    - the regeneration reproduces 1f00d2d5 (final-fp-regen-tests.log);
    - the sim-only test's 13 printed lines (the short copy's 4 and the full test's 9) are identical to round 1's, once
      the new dead figure is set aside and the old skip text is mapped to the new wording (:61);
    - both manifests are unchanged.
  - Its deviations, LOGS0-RECORD and three more, were all accepted at round 2 (below).
- **TE-B1-8, in the ruling's words** (FP-9 item 3, advisor-brawl1-r1-full.md:188): "No test drives Behaviour::step
  with an own soldier past the outer radius. The glue at bots lib.rs:223 is shown only by the moved pins (own_soldier
  returns Some and changes commands) and by the two 10-minute evidence runs (condition 14)." After the fix pass, the
  glue line is lib.rs:228 (brawl1-copy2-report.md:68).
- **RR4-L1's constant** (FP-9 item 5, advisor-brawl1-r1-full.md:190): "RR4-L1's constant: MAX_ENTRY_TICK_COUNT stands
  (points ruling, Point 1). RR-B1-6 and EA-B1-5 are answered there, and ADV-PT-L3 carries the point to the check.mjs
  follow-up."
- **RR2-B1-2, round 2's correction:** FP-6 (a) made Server::new private (scenario.rs:396). Its only caller is :389, not
  :390 (advisor-brawl1-r2-full.md:55-58).

### Review round 2 and the advisor's round-2 ruling

_Copied by brawl1_report_words.py, word for word, from advisor-brawl1-r2-full.md lines 131-166 (sha256 64e361ab...)._

"Review round 2 (workflow wf_19a5485a-e64; test-engineer and rust-reviewer at effort xhigh, in one batch) reviewed snapshot c2561b6c (wip/brawl1.patch, 1,266 changed lines; packet e50a22f7). Both seats: APPROVE WITH FINDINGS. Every round-1 finding is fixed as ruled or deferred as ruled: TE-B1-1 to TE-B1-8, RR-B1-1 to RR-B1-7, and EA-B1-2 checked by rust-reviewer.

What the seats checked:
- They rebuilt each plant's planted sha256 without writing anything: P1 77be9b00, P2 b902a7ee, P3 7a77c71a, P4 6aeeec0a.
- Outside the test modules, the library diff is comments plus one visibility change (scenario.rs:396).
- Their own runs (r2-tests.log, lint-tests.log; header c2561b6c): every command exited 0. fmt and clippy are clean in both builds.

Round 2 found four Lows and no Critical, High or Medium, so round 3 did not open. Two rounds were used; the second was the last.
- TE-B1-R2-1: the kept-state check cannot catch a leash that updates its state from a dead soldier's position. Follow-up, test only: move the dead input from (1450, 0) to (0, 0) at bots lib.rs:404.
- RR2-B1-1: toward_centre's doc should say 'x² + y² is at most 2^31', not 'stays below' (bots lib.rs:132; the words were the advisor's). Follow-up, riding with TE-B1-R2-1. Home for both: the next lane that edits npt-bots' lib.rs, U6's bots'-view item at the latest.
- TE-B1-R2-2: the runners' 'untracked' field hashes the whole porcelain status list, modified files included. Follow-up, a runner edit outside the change set, after BRAWL-1's last re-run set and before U4g's first run, in the form `untracked $(git ls-files -z --others --exclude-standard -- . | xargs -0 -r sha256sum | sha256sum | cut -d' ' -f1)`, with a .diff and `bash -n`. Until then, every BRAWL-1 log's 'untracked' value is the porcelain list's hash: e3b0 means no modified and no untracked file under spikes/netcode-proof, and 17cf51d7 is the builder's ten modified files.
- RR2-B1-2: Server::new's only caller is scenario.rs:389, not :390. Corrected in this report.

The advisor's round-2 ruling (advisor-brawl1-r2.md):
- It checked every finding and every deviation in the code.
- It read the interdiff whole and counted +3 tests with none removed.
- It diffed the sim-only test's 13 printed lines against round 1's itself, after the stated normalisation: identical.
- It hashed det-u4's regenerated recordings (1ec25de7, 6abf7643) and both manifests (a2faf3ea, 5ccf57f0).
- Its own run (.gate/u4r/brawl1-advisor-r2/adv-r2-tests.log, header c2561b6c): fmt clean, and the BRAWL-1 tests pass in both builds.

Copy 2's deviations are all accepted:
- LOGS0-RECORD, option (a): the no-log case keeps `--record <scratch>/brawl1.rec`, because parse refuses `--logs 0` alone and summary_line reads only `s.logs`.
- soldiers()'s doc names client lib.rs:250-251.
- Logs::open gets `Some(dir.as_path())`.
- The extra doc and test lines.

The advisor's corrections, in its words, beside ADV-BP-1 to ADV-BP-4:
- ADV-R1-1: 'Two lines in FP-3 could not run as I wrote them: `--logs 0` without `--record` cannot parse, and `Some(&dir)` would not compile; copy 2's repairs are right.'
- ADV-R1-2: 'Step 1 named the brief's section 4 for the rules; they are in sections 1, 2 and 5.'

Process slips, none with an effect:
- The main session's resume part for copy 2 gave the round-1 ruling as 249 lines; it has 460, and its sha256 was right.
- The resume part repeated ADV-R1-2's 'section 4' twice: at :16, and at :19 as 'section 4 rule 6'.
- The advisor appended three lines to its own notes file with a shell redirect, against its write rule. Content unaffected.

The merge step goes ahead on c2561b6c, M1 to M7 with amendments A1 to A6 (the ruling's merge step). The counted run must show the empty-input sha256 e3b0c442... for both diff and untracked, and must reproduce all three pins."

### The merge step (M1-M7) and the advisor's head review (M6)

_The steps are the round-1 ruling's M1-M7 (advisor-brawl1-r1-full.md, The merge step), with the round-2 ruling's A1-A6
(advisor-brawl1-r2-full.md, The merge step) and M6's M7-1 to M7-4 (advisor-brawl1-m6-full.md, M7). The logs are under
.gate/brawl1-merge/._

- **M1, the bytes** (the card, "M1 DONE"; b30314a's commit time 08:09:41 UTC by `git log`; checked at M6, checks 2
  and 3):
  - The base held: spikes/ at b30314a^ equals 352a245's (tree 63de1632).
  - `git apply --check`, then `git apply`, of wip/brawl1.patch (sha256 c2561b6c…, committed at 888ecbb).
  - All 10 files are cmp-equal to the builder's worktree, and the fingerprint `git diff HEAD --binary -- spikes/ |
    sha256sum` is c2561b6c.
  - The merge commit **b30314a** holds 11 files: the 10, plus `git rm` of wip/brawl1.patch. Its message has no
    [skip ci]. Nothing from round 2 rode (A1).
- **M2, the part-cleans, from 08:37:41 to before 08:38:44** (ADV-M6-L1):
  - No cargo, rustc, npt-lab, npt-judge or loader process was running, and no agent. This is on the card only.
  - (B), the release, doc and wasm32 roots: m2-clean-B.log, 08:37:41-08:37:47, 15 commands, each exit 0.
  - (C), the dev profile: m2-clean-C.log, 08:38:04-08:38:09, 6 commands, each exit 0.
  - Both logs' "run:" lines equal 2026-11-14's clean-shared-B.log and clean-shared-C.log (M6, check 4).
  - Free space, from the card only: 6,911,565,824 B before (B), 7,367,524,352 B after (B) and 13,579,448,320 B after
    (C), 12,950 MiB. These match the runner headers: 6,591 and 7,026 MiB (m2-clean-B.log:2, m2-clean-C.log:2), and
    M3's 12,950.
  - (B) freed 434.8 MiB, and (C) freed 5,924.2 MiB (M6, check 4).
  - The headers read `diff e3b0…; untracked e3b0…`.
- **M3, the counted run** (m3.log; 08:38:44-08:59:43 UTC; built cold, so its timings are cold):
  - The header: HEAD b30314a, free 12,950 MiB, `diff e3b0c442…; untracked e3b0c442…` (A2; m3.log:1-3).
  - `cat /proc/uptime`: 227.41 s (:5). uptime: "08:53:06 up 18 min, 0 user, load average: 2.31, 2.21, 1.54" (:26, spaces folded).
  - check.mjs: "check: all 11 steps pass" (:8-19):
    - test 691.2 s, cold: measured build 538 passed, bites build 616 passed, 0 failed, 0 skips;
    - self 77 of 77; marks 81; redact 287 items, 0 findings.
  - df: 8,342,093,824 B after check.mjs and 8,342,081,536 B after det-u4, both above 4 GiB (A4; :23, :35).
  - det-u4: "PASS det-u4 234.1 s" (:30).
    - Its non-cargo sum is 219.4 + 2.5 + 0.9 + 0.0 = 222.8 s, under 240.
    - It reproduces 1ec25de7 and 6abf7643: det-recordings.json is unchanged, and the advisor hashed both files at M6.
  - The regeneration: "regenerated SS-P0-s4: 16950444 bytes, sha256 1f00d2d5…, as the manifest's" (:54).
  - The full sim-only test, in release in 23.90 s (:86). Its nine printed lines are byte-identical to
    final-fp-regen-tests.log's, dead figures included (A3; M6, check 8).
  - All 9 commands exit 0 (:103). Both manifests are a2faf3ea and 5ccf57f0 (M6, check 7).
  - **After M3** (ADV-M6-L2), labelled "after M3 and M4, not at M3's end":
    - the advisor's reading, 8,320,163,840 B at 09:06:48 (advisor-brawl1-m6-full.md:51);
    - M7's first df, 8,318,758,912 B (m7-df-0.log).
- **M4, the gate:**
  - The full gate's 5a smoke could not start Chromium: "Executable doesn't exist at
    /opt/pw-browsers/chromium_headless_shell-1194/chrome-linux/headless_shell". So the full gate reads GATE FAIL, on 5a
    alone (m4-gate.log).
  - `node tools/gate.mjs --no-smoke`: "GATE PASS (5a smoke skipped: --no-smoke)" (m4-gate-no-smoke.log). CI's smoke job
    covers 5a.
- **M5, push and CI:**
  - b30314a was pushed alone, at 09:02:06 UTC (f7dff4b..b30314a; the card, "M5 DONE").
  - CI is green on it: gate run 43697261383 (jobs gate and smoke) and toolchain run 43697261438. It is green on 8f0939f
    too: gate 43697365073 and toolchain 43697365142 (M6, check 10).
  - Every commit after b30314a touches only .checkpoints/, until the re-run sets' results commits (A5).
- **M6, the advisor's head review: APPROVE WITH FINDINGS** on head 8f0939f (advisor-brawl1-m6-full.md, workflow
  wf_8067a67a-6fe, sha256 c2fc64c1…).
  - 14 checks, all PASS, including the advisor's own gate run (.gate/brawl1-merge/advisor-m6/gate.log).
  - Three Lows, record-keeping only: ADV-M6-L1 to -L3 (the corrections, below).
  - M7: GO, with M7-1 to M7-4.
- **M7, the cleans again** (M7-1, M7-2). Every log's header shows HEAD fc93733, spike tree ebbfeb40, and
  `diff e3b0…; untracked e3b0…`:
  - m7-df-0.log, 09:12:50: 8,318,758,912 B free;
  - m7-clean-B.log, 09:12:52-09:12:53: 15 commands, exit 0;
  - m7-df-1.log, 09:12:54: 8,722,419,712 B;
  - m7-clean-C.log, 09:13:04-09:13:09: 6 commands, exit 0;
  - m7-df-2.log, 09:13:10: 13,556,895,744 B (12,928.9 MiB).
  - The clean logs' "run:" lines equal M2's (diffed).
  - M7-4, for disk planning: the main folder's cold full set refilled about 5.0 GiB, not the 8.2-9 GiB estimated
    earlier. The floors stay as they are; P0 needs 7,711 MiB (advisor-brawl1-m6-full.md:67).
  - Then come the two evidence runs and the re-run sets, below.

### The two evidence runs (condition 14)

- **The call:** one main-run.sh call, 09:15:05-09:26:03 UTC (.gate/brawl1-merge/m7-ev.log).
  - Its header: HEAD d89cc9d, spike tree ebbfeb40, `diff e3b0…; untracked e3b0…`. That is M7-3's original form, which
    the evidence logs keep (advisor-brawl1-sets-p3-full.md, The M7-3 conflict, (d)).
  - All 10 commands exit 0.
- **Each run** (m7-ev.log:112-135):
  - `npt-lab scenario --run brawl1-ev-<p0|p3>-s1 --set brawl1-evidence … --seed 1 --scene SS --profile <P0|P3>
    --tick-start 63736 --warmup-s 10 --measured-s 600 --suite brawl1-evidence --role measured --marks
    PM-0001-2,PM-0001-17,PM-0001-57`;
  - then `du -sb`;
  - then npt-judge under peak.py, into .gate/brawl1-evidence/.
  - No run or set is named u4r-* (condition 14).
- **The bars (condition 14): all met.**

  | Bar | P0 (brawl1-ev-p0-s1) | P3 (brawl1-ev-p3-s1) |
  |---|---|---|
  | everyone in view, at least 0.99 | 1 (verdict-P0.json) | 1 (verdict-P3.json) |
  | building figure 0 | "in a building: 0 slot-ticks (slots none)" (m7-ev.log:113) | the same (:125) |
  | bytes within U4r-F3's rate | 538,353,013 B against 603.7 MB (:116) | 98,642,683 B against 183.3 MB (:128) |

- **Recorded as facts, not a verdict** (condition 14):
  - **P0:**
    - every stress-scene check passes, with 693.09 shots a second;
    - -17: 0 mismatches of 878,352 compared, and 0 incomplete ("L1 P0 loses no group" passes; that check was built on
      the advisor's rulings and is kept by his C62H-KEEP (a));
    - overflow resyncs 0 (m7-ev.log:113);
    - -57: 6,984 / 2,750 / 88,063, position p99 5,737 (verdict-P0.json).
  - **P3:**
    - every stress-scene check passes, with 693.18 shots a second;
    - -17: 0 mismatches of 835,238 compared (verdict-P3.json);
    - -57: 6,952 / 2,750 / 88,063, p99 5,111.
  - **Added by ADV-P3-1** (the card's first record left these out):
    - The P3 run had 27 overflow resyncs on 25 clients, at ticks 76,910-77,844, all in the measured window. The count
      is in m7-ev.log:125; the clients and ticks are in its lab.jsonl.
    - Its incomplete count is 43,295 in the measured window (npt-lab, m7-ev.log:125) and 44,395 over the whole run (the
      judge, verdict-P3.json).
    - In the P3 ruling's words (advisor-brawl1-sets-p3-full.md, The report's figures): "**The evidence run
      brawl1-ev-p3-s1** is identical to l1-p3-s1: 27 resyncs at the same ticks and slots, 835,238 compared, 44,395
      incomplete."
  - -17's parts read NOT-RUN in both verdicts, because one run is not three seeds. That is expected for evidence runs.
  - The judge: P0 8.6 s and 1,856.3 MiB; P3 2.3 s and 392.9 MiB (m7-ev.log:122, :134).
  - Free space: 13,556,174,848 B before and 12,643,663,872 B after (:6, :138).

### The re-run sets (P3, P2, P0) at the fix's commit

_Copied word for word from advisor-brawl1-sets-p0-full.md, "The report words", lines 212-253 (sha256 4b16dab5…). One
change is made, as the ruling's own "NOTE TO THE MAIN SESSION" (:266) orders: the P3 carry-58 line reads "U4r 0 / 0 /
0"._

All three sets ran with c-u4r-runs.mjs 99dd070e, the runners unchanged (A6), each set's results committed before the next set started, and the amended M7-3 met at each start (pre-P2.log, pre-P0.log). Figures are for runs s1 / s2 / s3; "U4r" means U4r's same-profile set.

**P3:** commit fd4612050cd6, spike tree ebbfeb40. Log .gate/u4r-sets/P3-fd4612050cd6.log (sha256 cc2f626c…), exit 0, wall 1,020.5 s, peak 973.2 MiB. Results results/u4r/fd4612050cd6/P3/ (runs.txt 5361fd53…, verdict.json 21c9abbf…), committed at fc41111.
- -2 SS runs: FAIL to PASS. Everyone in view went from 0.352778 / 0.085444 / 0.203556 to 1 / 1 / 1.
- -17 L1 P3: INVALID to PASS, with 0 mismatches of 835,238 / 835,419 / 835,088 compared. The judge counts 44,395 / 44,253 / 44,535 incomplete over the whole run; npt-lab counts 43,295 / 43,210 / 43,454 in the measured window.
  - Against the profile's loss (C62H-REVIEW; the advisor's report words, advisor-c62h-review-full.md): P3: 43,295 / 43,210 / 43,454, against 43,200 (5 %). These are the advisor's estimates, assuming one datagram per client-tick; no check makes the comparison. Stale = 0 and sentinels = 0 on every run. -17 does not limit this count off P0, and neither do -58's two wrap parts.
- -57 L1 P3: PASS, as in U4r's. Position max 6,952 / 6,963 / 6,970 against 6,984; p99 5,111; yaw 2,750 and pitch 88,063, unchanged. Largest change: s1's position max, -0.46 %.
- Carry 58: overflow resyncs 27 / 0 / 1 (U4r 0 / 0 / 0).
  - s1: 25 clients at ticks 76,910-77,844. s3: slot 35 at 76,803.
  - All are in the measured window. No client was kicked, and there are 0 mismatches.
  - The advisor ruled that s1 and s3 count (EVQ-LOAD).
- Building figure 0 on every run. Bytes 290.7 MiB against 524.4 projected (0.554); U4r's P3 had 518.8.

**P2:** commit fc411116c1f1, spike tree 079ffffc. Log .gate/u4r-sets/P2-fc411116c1f1.log (sha256 a24d5856…), exit 0, wall 1,046.7 s, peak 1,760.3 MiB. Results results/u4r/fc411116c1f1/P2/ (runs.txt c2995379…, verdict.json 1d2ff9e5…), committed at 93509df.
- -2 SS runs: FAIL to PASS. Everyone in view went from 1 / 0.5875 / 0.7295 to 1 / 1 / 1.
- -17 L1 P2: INVALID (s2 and s3 by pre:PM-0001-2) to PASS, with 0 mismatches of 861,996 / 862,011 / 861,731 compared. The judge counts 17,739 / 17,566 / 18,027 incomplete over the whole run; npt-lab counts 17,260 / 17,089 / 17,532 in the measured window.
  - Against the profile's loss (C62H-REVIEW; the advisor's report words, advisor-c62h-review-full.md): P2: 17,260 / 17,089 / 17,532, against the 17,280 that 2 % predicts. These are the advisor's estimates, assuming one datagram per client-tick; no check makes the comparison. Stale = 0 and sentinels = 0 on every run. -17 does not limit this count off P0, and neither do -58's two wrap parts.
- -57 L1 P2: PASS, as in U4r's. Max 6,984 / 2,750 / 88,063 on every run, equal to U4r's. Position p99 5,111 on every run, against 5,257 / 5,280 / 5,283. Largest change: s3's position p99, -3.26 % (ADV-P0-2 corrects the card's '0.0 %').
- Overflow resyncs 0, so no flag. Every other summary figure equals U4r's same-seed line.
- Building figure 0. Bytes 622.2 MiB against 780.8 projected (0.797); U4r's P2 had 778.9.

**P0 (split):** commit 93509dfd7fd1, spike tree 07baa855. Log .gate/u4r-sets/P0-93509dfd7fd1.log (sha256 e07e882e…), exit 0, wall 2,566.1 s, peak 4,504.3 MiB. Results results/u4r/93509dfd7fd1/P0/ (runs.txt ada5a58a…, verdict.json e926b812…, verdict-bites.json b472784f…).
- -2 SS runs: FAIL to PASS. Everyone in view went from 0.861889 / 1 / 1 to 1 / 1 / 1. Shots per second 693.09 / 692.97 / 693.06 (U4r 694.16 / 694.19 / 693.98).
- -17 L1 P0: INVALID (s1 by pre:PM-0001-2) to PASS. 0 mismatches of 878,352 compared on each run; 0 incomplete; 0 overflow resyncs. ("L1 P0 loses no group" was built on the advisor's rulings and is kept by his C62H-KEEP (a).)
- -17's bites: invalid in U4r's set, now shown.
  - tick16 fails both checks: 12,151 mismatches and 720 incomplete.
  - client-rounding fails mismatches (878,352); incomplete 0 passes.
  - whole-as-lost fails incomplete (48); 0 mismatches passes.
- -57 L1 P0: PASS, as in U4r's. Max 6,984 / 2,750 / 88,063, equal. Position p99 5,737 / 5,736 / 5,739 against 5,719 / 5,718 / 5,715. Largest change: s3's position p99, +0.42 %.
- -57's bites: shown in both sets, each failing only its own check. pos-4cm 34,856 (U4r 34,830, +0.075 %); yaw-12bit 46,692 (equal); pitch-8bit 440,313 (U4r 440,312).
- Building figure 0 on all nine runs. Bytes 3,391.4 MiB against 3,614.6 projected (0.938); U4r's P0 had 3,248.3.
- **The tick16 bite (carry 58):** 94 overflow resyncs, against U4r's 48.
  - All are in the after phase: 47 at 65,611, 46 at 65,729 and 1 at 65,791. The window ends at 65,550. 48 slots.
  - No kick: 48 snapshot clients on every tick. No mismatch inside the measured window.
  - The 720 incomplete are 48 × 15 ticks, by design.
  - The cause: after the wrap, the room re-runs each soldier's last command. Four soldiers walk into buildings in the after phase. Slot 5, inside since 65,588, fills its queue more slowly, and the run waits for its resync while the others resync a second time.
  - The advisor rules that tick16 is shown and counts. Its 'shown' now rests on this 10-minute run. TICK16-WORDING is corrected (below).

**Overall:**
- -2's SS part passes at every profile: that is the fix's proof.
- -17's L1 P0, P2 and P3 parts and its three bites count from the fix's commit.
- -57 passes everywhere. Its largest change across the sets is P2's s3 position p99, -3.26 %.
- The logs total 4,304.3 MiB against 4,919.8 projected.
- -17 L1 B1 stays with U4g's judged B1 runs.

_Added by the main session after the quoted words, each from its file:_
- P0's results were committed at be41ce6, with the card and the P0 ruling (`git show --stat be41ce6`).
- Before P2 and before P0, one read-only main-run.sh call checked the amended M7-3 (advisor-brawl1-sets-p3-full.md,
  The M7-3 conflict). Every command exited 0, and each header reads `diff e3b0…; untracked e3b0…`:
  - pre-P2.log, 09:58:11 UTC: HEAD fc411116c1f1, spike tree 079ffffc, df 12,326,227,968 B;
  - pre-P0.log, 10:16:45 UTC: HEAD 93509dfd7fd1, spike tree 07baa855, df 11,665,391,616 B, against P0's floor of
    7,711 MiB.
- The three results folders' redact check (ADV-P0-3): .gate/u4r-sets/redact-BRAWL1-sets.log, "redact-check: 7 item(s)
  checked, 0 finding(s)".
- The 94 tick16 resyncs (47 at 65,611, 46 at 65,729, 1 at 65,791, all in the after phase, on 48 slots, with slots 17
  and 5 once each) and the 27 / 1 P3 resyncs were also counted from the raw lab.jsonl files, read-only.

### EVQ-LOAD, the carry line for U4f's and U4g's briefs

_Copied by brawl1_carry_lines.py, word for word, from advisor-brawl1-sets-p3-full.md line 158 (sha256 1bc5889c...)._

"With everyone in view, every client queues every hit and kill. The stress scene's room-wide reliable gameplay events run at about 460-560 a second at P3 (30 s bins), about 17.3 queued per client per tick, against the provisional cap of 1,024 (events.rs:23). Plan §4.7 sized the cap from about 350 a second (net F121). Every snapshot sits at the 1,000 B window (mean 999.2 B). In BRAWL-1's P3 set, the cap overflowed inside the measured window: s1 had 27 forced resyncs on 25 clients at ticks 76,910-77,844, where the room's rate peaked at 554-561 a second; s3 had one at 76,803; s2 none. There was no kick and 0 mismatches. B1 adds 1 s of total loss on P2's backlog, about 520-570 more events per client. So net F121's warning is live for U4g's judged B1 runs: a cap too low resyncs every client on every burst and changes -18's heal path. U4f's cap measurement (§4.7) uses this full-view rate. U4g's plan reads carry 58's per-burst high-water marks from a short B1 run before -18's judged B1 runs count."

### TICK16-WORDING, corrected (it replaces the sets-r2 line under "The judged runs", Corrections (Q3 (7)), above)

_Copied by brawl1_carry_lines.py, word for word, from advisor-brawl1-sets-p0-full.md line 135 (sha256 4b16dab5...)._

"tick16's run is valid and the bite shown, failing both -17 checks. Its measured incomplete is 720: 48 clients times the 15 measured ticks from the wrap (65,536) to the window's end (65,550). Every client has at least one overflow resync, all in the after phase, none in the measured window, and no kick. The count and the ticks are not fixed: the run ends only when every client's last measured ticks resolve, and while one client's queue fills slowly, the others can resync again, each after its forced full state was acknowledged."

_tick16's "shown" now rests on BRAWL-1's 10-minute P0 bite run (12,151 mismatches; the new check 720), not on r2's
30 s record (advisor-brawl1-sets-p0-full.md, The tick16 bite, "Why it still counts as shown"). The U4r P0 record's
line on this, in "The judged runs" above, now says so too._

### The corrections

- **ADV-BP-1 and ADV-BP-2**, the advisor's, recorded under condition 18 (advisor-brawl1-plan-full.md, Conditions, item
  18): "ADV-BP-1: N2-1's 'all four checks fail' was wrong; under the leave plant two fail (Q5). ADV-BP-2: my sets-r2
  figure of 'about 4.2 GiB free after the refill' mixed units."
- **ADV-BP-3**, the advisor's (advisor-brawl1-points-full.md, Point 2, "The change"): "Condition 6 (ii)'s identical-diff
  test could not hold at <base>, because the wallclock bite's L1 hash varies with wall-clock ns (pump.rs:236-243); and
  'comes to me whole' did not say stop."
  - Its masking rule (the same Point 2, "Ruling"): "Future captures of these tests mask exactly the L1 leg's hash values
    in the wallclock bite's comparer and result lines ('theirs ... hash' and the L1 hex). They keep the index, the
    loopback hash, compared and mismatching, and they explain libtest's 'filtered out' count by the binary's test count.
    Nothing else is masked."
- **ADV-BP-4**, the advisor's (advisor-brawl1-points-full.md, Point 1, "The change"): "RR4-L1's ruled name
  MAX_ENTRY_TICKS falls under check.mjs's TICK_NAMED rule and is refused as a literal; I did not check it; the name is
  MAX_ENTRY_TICK_COUNT."
- **ADV-PT-M1's home** (Medium, the points ruling; advisor-brawl1-r1-full.md, ADV-PT-M1, "recorded"):
  - The home is U4g's brief, through u4g-brief-carry.md (sha256 c3abeccb…). That file holds the round-1 ruling's U4g
    words word for word (advisor-brawl1-r1-full.md:279-288), merged with NA-B1-2 (c) and TE-B1-5's kinematic bound.
  - In short: B1's 1,832 cm is the leash's reach, not a stop, and it passes the 1,800 cm door entry. U4g's plan says how
    a B1 building entry reads in -17, or measures how often it happens, before its judged B1 runs.
  - This report's BRAWL-1 entry under Follow-ups points there.
- **ADV-R1-1 and ADV-R1-2**, the advisor's, in its words (advisor-brawl1-r2-full.md, The report words; also quoted in
  the round-2 subsection above):
  - ADV-R1-1: 'Two lines in FP-3 could not run as I wrote them: `--logs 0` without `--record` cannot parse, and
    `Some(&dir)` would not compile; copy 2's repairs are right.'
  - ADV-R1-2: 'Step 1 named the brief's section 4 for the rules; they are in sections 1, 2 and 5.'
- **ADV-M6-L1 to -L3** (Low, record-keeping; advisor-brawl1-m6-full.md, Findings). Each is done in this section:
  - ADV-M6-L1: the card gave M2 as 08:38-08:40, but the logs give (B) at 08:37:41-08:37:47 and (C) at 08:38:04-08:38:09,
    before M3 started at 08:38:44. M2's pgrep and df readings exist only on the card. Done: the merge record above.
    From M7 on, every df reading is in a log (M7-2).
  - ADV-M6-L2: m3.log ends with no free-space figure. Done: the post-M3 figures in M3 above, labelled "after M3 and M4,
    not at M3's end".
  - ADV-M6-L3: this section lacked a pointer to round 1's record. Done: the first item of "Round 1 and its ruling".
- **ADV-P3-1 and -2, and ADV-P0-1 to -3**, in the P0 ruling's report words (advisor-brawl1-sets-p0-full.md, "The report
  words", "Advisor corrections", lines 260-264):

- ADV-P3-1 (Medium): the P3 flags were not raised.
- ADV-P3-2 (Low, the advisor's): M7-3's wording.
- ADV-P0-1 (Low, the advisor's): TICK16-WORDING's 'one per client' and the computed 'wrap + 54-60'.
- ADV-P0-2 (Medium): P2's -57 p99 was left out of the card's record, so its '0.0 %' was wrong.
- ADV-P0-3 (Low): no redact-check log was kept for the P3 and P2 commits. The advisor's own read-only run on all three folders found 0.

  What each means and how it was closed:
  - **ADV-P3-1** (advisor-brawl1-sets-p3-full.md, Summary, Findings):
    - Carry 58's flags on P3's s1 and s3 were not raised to the advisor.
    - The card's evidence record left out the P3 evidence run's resyncs and its incomplete count.
    - No effect: nothing counted, and the set stopped for the ruling anyway.
    - Closed: both figures are in the evidence subsection above.
    - The lesson: check every runs.txt flag and every summary-line figure against U4r's same-seed line, not only the
      verdict lines.
  - **The same ruling's unnamed Low:** P3's pre-launch readings (pgrep, porcelain, df) exist only on the card. From P2
    on, they are command output in pre-P2.log and pre-P0.log.
  - **ADV-P3-2:** M7-3, as first written, could not hold, for two reasons:
    - set logs carry no diff/untracked header;
    - committing each set's results changes the spike tree.
    - Closed: M7-3 is amended, (a) to (d) (advisor-brawl1-sets-p3-full.md, The M7-3 conflict). P2 and P0 met the
      amended form (pre-P2.log, pre-P0.log).
  - **ADV-P0-1:**
    - "One overflow resync per client" holds only when every client's queue fills on the same tick.
    - The computed "wrap + 54-60" used the measured event rate, which falls after the wrap. The first resync came at
      wrap + 75.
    - No effect.
    - Closed: the corrected TICK16-WORDING (above). It replaces the old line in this report's U4r corrections
      (advisor-brawl1-sets-p0-full.md, The tick16 bite).
  - **ADV-P0-2:**
    - P2's -57 position p99 went from 5,257 / 5,280 / 5,283 to 5,111 on all three runs (results/u4r/ab10f058a828/P2/
      and results/u4r/fc411116c1f1/P2/, verdict.json, "positions: error p99").
    - So the largest change at P2 is s3's position p99, -3.26 %, not 0.0 %.
    - No verdict is affected. The re-run sets above carry the corrected figure.
  - **ADV-P0-3:** the redact-check log is now kept: .gate/u4r-sets/redact-BRAWL1-sets.log, 7 items, 0 findings.

### Follow-ups, each with its home (none triggers a push)

- **TE-B1-R2-1 with RR2-B1-1** (Low, test and doc only; advisor-brawl1-r2-full.md:40-53):
  - The change: the unit test's dead input moves from (1450, 0) to (0, 0) at bots lib.rs:404, and toward_centre's doc
    reads "x² + y² is at most 2^31" (bots lib.rs:132).
  - The home: the next lane that edits npt-bots' lib.rs, U6's bots'-view item at the latest, beside TE-B1-8's glue
    test.
- **TE-B1-R2-2** (Low; advisor-brawl1-r2-full.md:45-48):
  - The change: the runners' "untracked" field becomes `untracked $(git ls-files -z --others --exclude-standard -- . |
    xargs -0 -r sha256sum | sha256sum | cut -d' ' -f1)` at u4r-run.sh:67 and main-run.sh:42. Each gets a .diff and a
    silent `bash -n` (INF2-4's form).
  - The home: a main-session runner edit outside the change set, after BRAWL-1's last re-run set and before U4g's first
    run.
  - P0 was the last set, so it is now due (advisor-brawl1-sets-p0-full.md, The commit and the close, "Then, outside
    BRAWL-1").
  - Until then, every BRAWL-1 log's "untracked" value is the porcelain status list's hash.
- **ADV-PT-L3** (Low; advisor-brawl1-points-full.md:87-91):
  - The change: TICK_NAMED becomes /_TICKS$|_TICK_COUNT$|CATCHUP/, NOT_TICK_COUNTS gains MAX_ENTRY_TICK_COUNT with its
    reason, and check.test.mjs gains one case.
  - The home: the check.mjs follow-up (check.mjs's next change; before PR #40 is marked ready at the latest).
- **EVQ-LOAD, for U4f and U4g** (the carry line above; advisor-brawl1-sets-p3-full.md, The next sets;
  advisor-brawl1-sets-p0-full.md, The commit and the close):
  - The home is U4f's and U4g's briefs.
  - U4f's cap measurement (plan §4.7) uses the full-view rate.
  - U4g's plan reads carry 58's per-burst high-water marks from a short B1 run, before -18's judged B1 runs count.
  - U4g's brief also carries u4g-brief-carry.md (ADV-PT-M1, above).
- **From round 1** (advisor-brawl1-r1-full.md, FP-9 item 6 and "Each finding"):
  - RR-B1-7 (1), the room_tick loop rewrite: the next lane that edits logs.rs's room_tick (:402-405).
  - EA-B1-1's later half, with NA-B1-3's npt-client half: npt-sim exposes the flags index and the dead bit as pub
    items. The home: the next lane that opens crates/sim (beside EA-U4R-4), U6 at the latest (:372-375, :407-410).
  - TE-B1-8's optional glue test: U6's bots'-view item, EA-U4R-5's home (:357-360).
- **-17 L1 B1** stays with U4g's judged B1 runs (advisor-brawl1-sets-p0-full.md, The commit and the close, 2).
- **KEPT-LOGS** is under Decisions needed, below.

### Decisions needed

_The advisor's words, copied from advisor-brawl1-sets-p0-full.md, "For Mustafa", lines 278-284. "Your" is Mustafa,
and "my recommendation" is the advisor's._

- **KEPT-LOGS** (your open question, unchanged; new numbers, and now in the next unit's way): should a narrow kit rule let a session clear out old test-run logs once their results are saved and their rulings closed?
  - Why it matters now: kept raw run logs take about 11.4 GiB (U2 1.9, U4r 5.3, the bot fix's re-runs 4.2), plus 0.6 GiB of evidence runs. Free space is 7.5 GiB against the 4 GiB safety floor, and the next unit's (U4g's) build cache refills about 8-9 GiB. Its build cannot start until space is freed.
  - Options:
    - (a) Keep everything. U4g cannot build.
    - (b) Delete each closed run set's raw-log folder. This frees up to about 11.4 GiB. Those runs could never be re-judged; their verdicts stay saved.
    - (c) The same, but first pack each set into one compressed file, checked readable. Computed with the draft's measured ratio of about 3.5 to 1, this frees about 8 GiB and keeps about 3.3 GiB, and a re-judge stays possible.
  - My recommendation: (c), with the same rule naming the old 4 GB build cache, as in the draft. It comes as its own reviewed kit pull request (advisor plus security review) and merges only on your go. Nothing is deleted before then.

### For Mustafa

_In plain words, the advisor's, copied from advisor-brawl1-sets-p0-full.md, "For Mustafa", lines 271-275:_

- The bot fix has now passed all three re-run sets. Every bot stayed in view in every run, none walked into a building while the measurements ran, and the main snapshot check passes at every network level with zero errors.
- Every planted fault was caught, including the one that tests the tick counter wrapping round. Its 10-minute run now counts.
- That test sent more "resend the full game state" messages than I had predicted. I traced the cause in the logs: once the planted fault breaks the bots' commands, a few bots walk into buildings after the measurements end. While the run waits for one of them, the others are resent the state a second time. Nothing broke: no client was dropped, and no snapshot inside the measured part went wrong. I corrected my own wording of what that test should show.
- I also corrected one figure in the record of the second set. One of the position-accuracy figures, the 99th-percentile error, improved by up to 3.3 %; the record had said nothing changed. It decides nothing.
- What remains is paperwork: committing the last results and finishing the write-up. Nothing to check in a browser and nothing to merge.

1. Answer KEPT-LOGS (a, b or c) before U4g's build needs the disk. The bot fix's remaining steps do not wait for it.

_Since the P0 ruling: P0's results are committed (be41ce6), and this section was read against the files
(advisor-brawl1-close.md). BRAWL-1 is closed; KEPT-LOGS stays open._

_The numbered item is the advisor's (the same file, line 287). Nothing here needs a Brave check: this is spike code on
the test branch, nothing a player sees, hears or feels changes, and nothing merges to main._
