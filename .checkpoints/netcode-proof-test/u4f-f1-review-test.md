# U4f-F1 review round, test-engineer seat (2026-11-20)

Diff: wip/u4f-f1-build.patch, sha256 37d87a10bed52b81... (checked: equal to `git diff cb7e374 -- spikes/` in the
builder's worktree agent-ad5f72ed146da5f93; numstat: both test files pure additions, 33+0 and 113+0). Read only; nothing
run. Line numbers below are the worktree's (the diff applied on cb7e374).

## Plan
1. T1 (room/tests/commands.rs:618) against condition 3.
2. T2 (sim/tests/world.rs:1809-1921, cases a-f) against condition 4; the two plants (b3-plant1, b3-plant2).
3. Tests that cannot fail, missing negatives: the room's lost and doubled, the u8 wrap, bites on extra edges, wasm.
4. The +7 counts (b4-check1, b4-check2).
5. What stays unproven until -20's re-run (condition 5), and after it.
6. Verdict, findings.

## Block 1: T1 against condition 3 (done)
- Setup holds: option B room, cmd 1 at j+1 runs and repeats at j+2, j+3 (asserted [1, 1, 1], :625); cmds 2 (fire 1),
  3 (fire 2), 4 (fire 2) in one Input; 2 and 3 late (asserted 2, :634); 4 accepted by kind 5 (2 edges, 3 ticks covered
  from j+1) and run alone (asserted [4], :633).
- Room half: `presses()` == (2, 0, 0) at :636. Sim half: `reports().refusals` == [slot 0, seq 4, Cooldown] at :642
  (reports are per tick: seam.rs:538-540, cleared at world.rs:2170, so this is tick j+4's). Exactly one refusal implies
  the first edge fired (a dead slot gives none, a not-ready weapon two), so "the first fires" is pinned indirectly.
- Plant (seam.rs `edges_taken` only, min(advance, 1)): f1-b2-plant.log, T1 FAILED by name at :636, left (2, 1, 0) =
  -20's defect, 26 others pass; restore proven by f1-b2-restored.log's diff hash be6bedb4... equal to f1-b2-t1.log's.
- Gap: the plant stops T1 at :636, so :642 (the sim half) has never been seen to fail. By reading it fails under the
  same plant (refusals [] vs [seq 4]); T2's count halves show the sim's take bites. TE-F1-3, Low.
- Deterministic: tick-driven, no wall clock, no thread, fixed scene; no flake path.

## Block 2: T2 against condition 4 (done)
- Form: `two_edges` (:1813-1824) builds the twin with `course_stream(press)` and the forged stream with `(k, at)` named
  again; `course_stream` (:1605-1631) adds one per entry at its step, so the forged command there is 2 edges on and
  every later counter one higher, the W1-U4D twin form. `twins` (:558-586) asserts equal event lists, state hashes and
  dumps on every step, accumulates refusals, and returns the running counters (cumulative: seam.rs:541-550).
- Cases: (a) held + ready: shots start [10, 13], cooldown (1, 0), [10 Cooldown] / [] (:1826-1835). (b) not held,
  ready: shots [10, 20], (1, 0), [10 Cooldown] (:1837-1844). (c) cooldown: (2, 1), [11, 11] / [11] (:1846-1854).
  (d) last round: 31 shots ending [84, 100, 170], ammunition (1, 0), [100 Ammunition] (:1856-1866): the new loop's
  Ammunition branch. (e) reload: reload (1, 0), [50 Reload], last shot 120 (:1868-1878). (f) apart (:1895-1921):
  [10, 10] live seqs and no refusal; 15 throws then 2 edges at 15: seqs 0..=15, [15 PoolFull], pool_full 1.
  Each list exact, each count +1: condition 4 met.
- Both branches of the further-fire-edge loop (world.rs:1489-1499) are reached: Cooldown in a, b, c; Ammunition in d.
- Plant 1 (old rule in the fn): f1-b3-plant1.log, all six FAILED by name in the count half (a, b, d, e (0, 0) vs
  (1, 0); c (1, 1) vs (2, 1); f [10] vs [10, 10]); the twin half passed first in a-e, as it must under the old rule.
- Plant 2 (per-edge clock in the further-fire-edge loop, world.rs:1490): f1-b3-plant2.log, a-d FAILED by name at
  world.rs:573 "step N: the state hashes" (10, 10, 11, 100); e and f pass (no further fire edge), as expected.
- Restores: the final tree's diff hash 37d87a10 in f1-b3-t2, -fmt, -clippy and both b4 check logs (no untracked file:
  e3b0c442... is the empty hash).
- Not shown: (e)'s twin half able to fail (no reload plant: a second edge restarting the reload at the same tick sets
  the same value, so only an additive plant would show it). Not required by condition 4; noted, no finding.
- (f) pins the course's pool (16) and fuse (> 19 ticks) through the literals `0..=15` (:1899, :1904) and n = 20
  (:1908) with no assert naming them (d5 does the same). TE-F1-6, Low.

## Block 3: tests that cannot fail, missing negatives (done)
- **The room's lost is 0 by construction.** room.rs:1341-1347: advance = `p.wrapping_sub(presses[k])`, taken =
  `edges_taken(presses[k], p)` = the same wrapping difference, so lost = 0 for every u8 pair; the doc says so
  ("so 0 while the rule holds", room.rs:355-356). It can go non-zero only if the fn departs from the wrapping
  difference (the plant; or a broken fn at -58's wraps). No sim refusal reaches a log (advisor-u4f-f1-plan.md:75-80),
  and -20's bots press only fire, so the old and new sims give equal per-tick `h` (the extra fire edge is refused and
  traceless either way). So the re-run proves the 1-edge path unchanged and the extra fire edge state-neutral at
  scale, but it cannot tell F1's sim from cb7e374's on the take itself, and "0 presses lost: 0" is not evidence of
  the take. Both marks reading the series: marks.json:4214 (PM-0001-58), :4749 (PM-0001-20). TE-F1-1, Medium; touches a
  pass mark's meaning: named for Mustafa (TEST-FIRST), no mark change proposed.
- **Doubled:** on a repeat the command is the last run one and `run_presses` is not updated (room.rs:1349-1351), so
  taken = 0 and doubled stays 0, as before F1 (advance = taken). No test reads doubled > 0; pre-existing, not F1's.
- **The u8 wrap:** no test drives `edges_taken` across 255 -> 0: T1/T2 counters stay below 3; d4 goes 0 -> 255; the
  room's wrap test (room/tests/room.rs:1263-1314) wraps the melee counter and reads no press series; the lab's
  f28_20 prints lost and asserts only presses > 0 (lab/src/scenario.rs:3700-3705). A `now.saturating_sub(last)`
  mutation passes the unit suite and is caught only by -58's judged run. TE-F1-2, Low.
- **Bites on an extra edge** (no-cooldown, refuse-not-clamp): the build notes' reading (u4f-f1-build.md:53-61) matches
  world.rs:1446-1499 (no-cooldown: one shot then Cooldown/Ammunition; refuse-not-clamp: two Cooldown), but no
  bites-build test sends a 2-edge fire under either. Condition 1 asked only for the notes. TE-F1-5, Low.
- **Loop bounds:** d4 (sim/tests/world.rs:394-482) now runs 255 edges per button on each slot's first command in a
  debug test build (overflow checks on), asserting grid ranges only: it shows no panic at the bound, nothing more.
- **Predict path** (authority false): the reload loop runs, the further-fire and throw loops are authority-only, so a
  2-edge predict equals the authority's state by reading; no D-3 test feeds a 2-edge command. A client's own commands
  advance at most 1 per command, so no finding.

## Block 4: the counts and the wasm step (done)
- f1-b4-check1.log and -check2.log, both at diff 37d87a10: all 11 steps PASS; measured 641 passed, 0 failed, 0 skip;
  bites 738, 0, 0. +7 on 634 and 731 = T1 + six T2 tests, none feature-gated; both test diffs are pure additions, so no
  pre-existing test was changed, removed or ignored: "a 1-edge advance runs as now" holds for every existing pin.
- wasm: 27,000 ticks native-native and native-Wasm, 0 mismatching (SS-s1, ES-s2, TC-s3 synthetic streams). Those
  streams lose a command now and then and press fire 1 in 40, throw 1 in 400 (sim/src/replay.rs:347-348, :511-523), so
  a 2-edge advance is rare and, for fire, state-neutral; the run is not shown to contain one. TE-F1-4, Low.

## Block 5: unproven until the re-run, and after it (done)
- Until -20's L1 re-run (condition 5): per-tick `h` of every room log equal to 350677f's kept logs (present:
  .gate/netcode-proof-test/runs/u4f-20-350677f0aef2/measured/l1-20-{a,b}-s{1,2,3}, bites/, 434 MB); summary lines
  equal but wall time; B's press_count 432,000 per run, lost 0, doubled 0; the bite's 99.9 % still failing.
- Not proven by the re-run either: the sim's take of every edge at run scale (TE-F1-1); multi-edge reload and throw at
  scale (the bots press only fire); a session's first command (no run meets it; carried to U4f2, ADV-U4FF1-3).

## Claim, proof, gap
| Claim | Proof | Gap |
|---|---|---|
| The sim takes every edge, each through its own check | T2 a-e count halves and exact lists; T2 f; T1 :642; plant 1 fails all six by name | :642 unshown failing (TE-F1-3); not observable at run scale (TE-F1-1) |
| Extra fire and reload edges leave no trace | T2 a-e twin halves (hash, events, dump every step); plant 2 fails a-d at :573 | (e)'s twin half unshown failing; run scale pending (condition 5) |
| One shot per command, the clock once | T2 a-d; plant 2 | none at unit scale |
| Each throw edge throws or meets PoolFull | T2 f; plant 1 fails f | literals pin pool and fuse (TE-F1-6) |
| Room's seen/lost/doubled from the one rule | T1 :636 (2, 0, 0); plant gives (2, 1, 0) | lost 0 by construction (TE-F1-1); doubled untested (pre-existing) |
| A 1-edge advance runs as now, bites included | every existing test unchanged and passing, 641/738 (+7, +7), check.mjs x2 | per-tick `h` vs 350677f pending (condition 5) |
| The rule wraps (`wrapping_sub`) | none | TE-F1-2 |
| Identical on wasm32 and native | wasm step, 27,000 ticks, 0 mismatching | multi-edge not shown in those streams (TE-F1-4) |
| What a bite does on an extra edge | build notes, by reading | no test (TE-F1-5) |

## Findings
| Id | Severity | Where | What goes wrong | Fix |
|---|---|---|---|---|
| TE-F1-1 | Medium | room.rs:1341-1347, :355-356; marks.json:4214, :4749 | `press_lost_count` = advance - `edges_taken` = 0 on every run by construction; with sim refusals off the logs and fire-only bots, -20's re-run cannot show the sim took the extra edges, yet a "0 presses lost: PASS" reads as if it did | The re-run's report and STATUS say: lost is 0 by construction after F1 (it guards the fn against the room's own wrapping difference); the take is proven by T1:642 and T2's count halves only; per-tick `h` equality proves the 1-edge path and the extra fire edge's state-neutrality. Name for Mustafa (TEST-FIRST): whether both "0 presses lost" checks want a run-scale observable of the sim's take is his; no mark changes here |
| TE-F1-2 | Low | seam.rs:78-80 | No test drives `edges_taken` across 255 -> 0; a `saturating_sub` mutation passes the unit suite (caught only by -58's judged run) | A unit test: (255, 1) -> 2, (254, 0) -> 2, (1, 0) -> 255, (0, 255) -> 255, (7, 7) -> 0 |
| TE-F1-3 | Low | room/tests/commands.rs:636, :642 | T1's sim half (:642) never seen failing: the plant stops the test at :636 first | Assert (presses, refusals) as one tuple, or the refusal first; re-run the plant once and keep the log |
| TE-F1-4 | Low | .gate/u4f/f1-b4-check{1,2}.log (wasm step); sim/src/replay.rs:511-523 | The 27,000 native-Wasm ticks are not shown to hold a multi-edge advance, so the new loops' Wasm identity rests on reading | Count the 2-edge commands in SS-s1, ES-s2, TC-s3 once (a scratch read) and state the number, or say "not exercised" in the report |
| TE-F1-5 | Low | u4f-f1-build.md:53-61; world.rs:1446-1499 | A bite's behaviour on an extra edge is written, not tested | STATUS follow-up: a bites-build 2-edge twin for no-cooldown and refuse-not-clamp |
| TE-F1-6 | Low | sim/tests/world.rs:1899, :1904, :1908 | (f) pins the pool size (16) and fuse (> 19 ticks) through literals with no assert naming them; a scene change fails it under "live seqs" | Assert the pool size and fuse first, or derive the counts from the scene |

## Verdict
**APPROVE WITH FINDINGS.** T1 meets condition 3 (seen 2, lost 0, doubled 0, the sim's Cooldown refusal with seq 4;
the shared-fn plant fails it by name with -20's (2, 1, 0)). T2 meets condition 4 (the d10 twin form, cases a-f, exact
lists, counts +1; plant 1 fails all six in the count half, plant 2 fails a-d in the twin half, each by name). The +7
holds in both builds, twice, on the reviewed diff. One Medium (TE-F1-1) is about what the re-run's "0 presses lost"
can and cannot show; it binds the report's wording, not the code. Five Lows: follow-ups unless fixed in the merge.
For Mustafa (TEST-FIRST): TE-F1-1 names a pass mark's check that now guards only the shared fn.
