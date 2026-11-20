# U4f2 review round 1: test seat (test-engineer)

Read only. The patch `.checkpoints/netcode-proof-test/wip/u4f2-build.patch` (sha256 e9a0ae3e3f48af18ea7a947f…: equal to
the panel brief's except that the brief's copy drops one hex digit, "…af18a7a9…"), the builder's worktree by absolute
path, the logs under `.gate/u4f/`. Nothing run (no cargo, check.mjs, npt-lab, npt-judge or run script). This agent has
no Edit tool, so the file is rewritten whole with Write at each block (the rule's own fallback).

## Plan
1. Spec: u4f2-brief.md rev 1.2; advisor-u4f2-brief.md (conditions, Q1-Q5); u4f-brief.md 2.4 item 6, 2.6, 2.10, 2.12,
   4, 5.1-5.4, 6, 7.
2. The builder's record u4f2-build.md and the patch: inventory the 60 tests.
3. Map 2.10's 51 + 9 extras + 22 negatives + Q4 + C-U4F2-10 to tests; can each fail; each plant's log.
4. Weigh plant 9 (F-E), plant 1's history, f16's shared pair, 702/799, F-C, the 1,207 s margin.
5. c-u4f-r2.mjs's rows and p19's `expect`; c-u4f-runs.mjs's set 21 against 5.1-5.4.
6. F-B, and what stays unproven until -21's set.
7. Findings, table, verdict.

## Block 1: the tests and the counts (DONE)
- **60 new tests, as the record says (u4f2-build.md:192-196), checked in the patch:** bots 7 (f15 kinds 3, 8, 9, 11 via
  `f15_datagram`, kind 7, kind 10 under both options in one test; `c_u4f2_5_sim_short`), plus `c_u4f2_5_sim_full`
  ignored (not counted: check.mjs's `cargoCounts`, check.mjs:1943-1953, counts `passed` only and never `ignored`);
  client 1 (`c_u4f2_2`); lab scenario 30 (f28 B x2, `c_u4f2_1`, f22 x2, f23, f16 x24); lab forged 15 (f33 x13, q4,
  c_u4f2_10); lab contract 7 (c_u4f2_7 x2, t6 x5). 51 of 2.10 (f15 6, f16 24, f22 2, f23 1, f33 13, T-6 5) + 9 extras.
- **702 / 799 hold:** end-a3 and end-b "measured build: 702 tests passed ... bites build: 799" (u4f2-end-b.log:327);
  base 642 / 739 at 1faf894 (u4f-f1-merge/m5-check.log and f1-b5-check2.log both read 642 / 739). No bites-only test
  added, so +60 on each. Sum with U4f: +142 measured, +159 bites against 2.10's +124 and +139 (with T-6's +5).
- **22 negatives:** f15 x6, f22 x2, f23, f33 x13; each f33 case has an unrejected half that can fail (room_whole's item
  2, kind 3's copy that ran, kind 7 with no record, by_count's 1 of 2, the other-kind case, the planted copy). Q4 and
  C-U4F2-10 beyond. The floor has no slack, as the advisor said (advisor-u4f2-brief.md:96-98).
- **C-U4F2-1's proof** (advisor-u4f-r1.md:70-72): f16 kinds 2 and 5 on the forged pair: `--forge-every 12`, steady,
  P0, the lab's `Room<World>`; `unrejected_count` 0 means every item had an own-kind record and never ran (forged.rs
  `unrejected`, :113). Met, on the shared pair.
- **T-6's five** match advisor-u4r-r1-full.md:122 case for case; each asserts its exact failure line or exit code.

## Block 2: the 11 plants against their logs (DONE)
| # | Plant (record) | Log: what failed | Reading |
|---|---|---|---|
| 1 | forged.rs any kind matches | plant1.log: only f33 other-kind; c_u4f2_10 ok. plant1b.log:34-35 after the case change: c_u4f2_10 and f33 other-kind FAIL | bites now (Block 3) |
| 2 | kind 3's copy rule off | plant2.log:34-35: planted copy, kind 3 FAIL | bites |
| 3 | an unforged seq made an item | plant3.log:35: q4 FAIL | bites |
| 4 | kinds 8, 9, 11 counts ignored | plant4.log:33-35: the three FAIL | bites |
| 5 | `--forge-every 1` taken | plant5.log:22: c_u4f2_1 FAIL | bites |
| 6 | header option fixed at B | plant6.log:21: f6 A FAIL (B ok, as expected) | bites (pins the value, not its source: equal by construction) |
| 7 | band applied to clock | plant7.log:22: the first c_u4f2_7 FAIL, the second ok | bites |
| 8 | today's walk | plant8.log:43: c_u4f2_5 short FAIL (Skips 8,433) | bites |
| 9 | twin of kinds 1-6 makes the command honest | plant9.log:21-34: f16_option_a_* 12 ok; option B not run | NOT caught (F-E) |
| 10 | room kind 4's check removed | plant10.log:21-25: f16 A kinds 1, 4, 10, 11 and B kind 4 FAIL | bites, not specific (shared pair) |
| 11 | skip_command ignores the index | plant11.log:13: c_u4f2_2 FAIL | bites |
- The logs hold the runner's tree fingerprint (diff and untracked sha256s) but not the plant's text or file:line; the
  record names each in words (u4f2-build.md:154-186). U4f's return list item 4 asks file:line (u4f-brief.md:1202-1204).
- New checks with no plant: SEC-1's `refusals_dropped_count` (f16 asserts 0, which a constant 0 passes); the
  attribution's slot filters (forged.rs:81, :86); kinds 1, 2, 4, 5, 6's twin (plant 9, uncaught).

## Block 3: the weighed items (DONE)
- **Plant 9 / F-E:** confirmed by reading: no bots test builds a twin of kinds 1, 2, 4, 5 or 6 (`.twin()` appears only
  in `sides`, bots/src/lib.rs:1585 / :1599, used by the kind 3, 7, 8, 9, 10, 11 cases; U4f's `f15` at :1426-1532 makes
  forgers only). f16 cannot tell "never made" from "made honest" under the standing script, as the builder says. The
  claim the twin figure tests (no effect) still holds, but U4F-Q13 (b)'s construction is pinned nowhere. Cheap fix in
  bots: `sides(kind, false)` for those kinds with the kind-10 case's sequence-set assertion (:1891-1901): the twin never
  sends a forged sequence and every other sequence equals the forger's; plant 9 then fails by name; about 0 s.
- **Plant 1's history:** the first case could not fail (both items ran, so `ran` made them unrejected under any kind
  rule); the fix (item 2 not run) makes its count rest on the separation alone: right, and the plant now bites. Note
  for the report: a sim record exists only for a command that ran, so in a real run the `ran` rule always reaches a
  bitten kind-5 item first; C-U4F2-10's case pins the separation on records a run cannot make. No finding.
- **f16's shared pair:** sound (any kind's twin defect fails all 11 of that option: no false pass), and closer to the
  judged runs (all 11 forgers together). But 22 of 24 tests share two equality checks; plant 10 shows a kind-4 defect
  failing kinds 1, 10 and 11 by name; the panic text names the test's kind, not the culprit; return list item 6 ("a
  kind whose twin mismatched, with its first differing tick", u4f-brief.md:1207) cannot be given per kind. A departure
  from 2.10's scale text (:670: "one forger or clamp client, 15 honest"), for the advisor. The budget reason holds.
- **F-A:** confirmed: `Kind::Steady => [0, 0]` (bots/src/lib.rs:1046) and the lag client wraps the run's steady
  behaviour, so `tick.v.lagswitch_excess_move_um` is constant; f20's `most(...) <= 0` (scenario.rs:4225), p4/p18's
  expect and -21's movement check pass whatever the lab measures. U4f2 turns f20's movement half into a test that
  cannot fail.
- **F-B:** the dead branch is code-confirmed: world.rs:1351-1355 sets `claimed` and returns before any refusal when
  `!s.alive`; `revive` (world.rs:1192-1202) sets `wait` 0, `reload_left` 0, ammo full. So (a) a forged edge while dead:
  no record, unrejected, no effect; (b) dead at the honest edge (d-1), revived by the forged one (d): the forged fire
  edge finds a ready weapon and fires: unrejected AND a real effect the twin lacks. hand-long2 (A, seed 1, 600 s):
  kind 7 18 of 1,523 unrejected, every other kind 0 (its lab.jsonl:52-62); no twin at that length, so (b) is
  unmeasured. marks.json:4959-4968 reads `max == 0`. The short pair (6 s) holds no death, so f16 cannot see it.
- **F-C:** forged runs 680-688 at 5.3's lengths (hand-a), 677 / 685 at 600 s (hand-long, -long2); the thinnest is the
  bitten run: no-lead-bound 669 (hand-b.log:20, 5 s) and p7 668 (:59, 20 s), against PM-0001-2's 665
  (marks.json:285-297). By reading: under no-lead-bound the kind-1 forger's far-ahead command is buffered and its next
  honest commands fail kind 2 (not newer than the buffered stamp), so about one soldier's fire is lost; structural,
  not noise. All on the old kind-7 shape; none at 600 s, none under B at 600 s, none at seeds 2-3.
- **The 1,207 s margin:** 82 s (end-b 1,125.1 s). The same 642/739 tests read 931.9 s (worktree, f1-b5-check2.log)
  and 1,007.5 s (main folder, u4f-f1-merge/m5-check.log): 75.6 s apart (8 %); A2 and A3 33.6 s apart on equal tests.
  At +8 % 1,125 s reads about 1,215 s. U4g re-bases its line at 1.5 times its base (u4g-brief.md:206-207), so the
  next unit is not bound by 1,207 s; the fix pass is, with about 50 s of real headroom.
- **Hand runs on a superseded tree:** hand-a, hand-b and hand-long at diff d211149a… (their logs' line 3), before
  F-D's kind-7 rebuild (b8-k7 at e6e8b2a4…); on the final shape only hand-long2 (one measured run, no twin) and hand2's
  no-cooldown bite; final tree 4a9dde09… (end-b.log:3). The 48-client forged twin check on the final shape is unjudged.

## Block 4: the scripts against 5.1-5.4 (DONE)
- **c-u4f-r2.mjs:** 21 rows (grep count 21). p1, p7, p14, p17 as 5.3 gives them; FORGED = P21 + `--suite forged
  --forge-every 12`; FORGED_SAYS names the length failure (about 75 a kind) and the twin Gap; C-U4F2-4's line in the
  header and r2.txt's head; F5 (a), (b), (c). **p19's `expect`** (:71) reads condition 4 exactly: unjudged, reported,
  W's late drops, the bucket's limited datagrams, the cap's kind-10 refusals under A, the per-tick max at most 8, no
  fixed counts. Gap: no -21 row's expect names "in a building: 0", which section 2 (u4f2-brief.md:110) asks of every
  -21 row.
- **c-u4f-runs.mjs set 21:** PARTS_21 forged and clamp with twins, fast-clock 610 s, lag-switch P1; options A, B;
  seeds 1-3: 24 measured; twins 6 + 6 = 12, each right after its measured run, `--twin` naming the other both ways, the
  clamp twin's `--twin-logs @run` placed under measured/; NO_STAR's 11 ids equal 2.11's, plus refuse-not-clamp (clamp)
  and not-older-than (fast-clock, 610 s): 13 bites under A (U4F-Q10 (a), u4f-brief.md:1536), seed 1; two judge calls,
  the twins in the measured one (5.4, C11); STOP_NOTE names it. 49 runs. No test pins the plan (the record's Node check
  is in words only). Projection: 56 MB a 10-minute run (c-u4r-runs.mjs:154) against the measured 610 s forged run's
  65,188,828 B (`du -sb`, hand-long2): 14 % under.

## Block 5: unproven until -21's set (DONE)
1. Kind 7 at 600 s: unrejected 0 (F-B: fails as built, seen once).
2. The forged twin figure at 600 s and 48 clients, both options (F-B (b); no long pair ever run).
3. PM-0001-2's shot band on every forged and bite run at 600 s, seeds 1-3, both options (F-C; thinnest: no-lead-bound).
4. Ten of the 11 no-* bites at judged length (5.3 runs p7 only; hand-b's short runs show each own kind unrejected, but
   at 5 s every bite reads shown by length).
5. "1,000 commands of every kind" under option B at 600 s (A seed 1 measured: 1,523; kind 10 1,656).
6. "In a building: 0" on every -21 run, beyond the sim-only model (the 21-row record first).
7. The lag-switch movement check: cannot fail as built (F-A), so no set run proves it.
- **Does F-B mean -21's set as built fails?** Yes for the forged part, with high confidence: a clean measured run read
  `unrejected_count` 18 where the check needs max 0, and forger deaths are routine (thousands of dead soldier-ticks a
  run in the sim-only model). A clean partner failing that check also voids the forged bites' reading (the clean
  partner rule, u4f-brief.md:1047-1050). The twin failure is possible, unmeasured.

## Notes (one line per finding as written)
- TE-U4F2-1 High F-B; -2 Medium F-E; -3 Medium slot filters; -4 Medium shared pair; -5 Medium F-A; -6 Medium F-C;
  -7 Medium superseded hand runs; -8 Medium time margin; -9 Low SEC-1 count; -10 Low "in a building"; -11 Low
  projection; -12 Low 2.5's zeros; -13 Low C-U4F2-2's stamp half; -14 Low records and docs; -15 Low parser top.

## Claim, proof, gap
| Claim | Proof | Gap |
|---|---|---|
| 2.10's 51 + 9 extras, 702 / 799 | end-a3, end-b test lines; patch inventory | none |
| 22 negatives | f15 x6, f22 x2, f23, f33 x13 | floor exact; Q4's slot half cannot fail (TE-3) |
| Attribution per kind (2.4 item 6) | f33 x13 + plants 1-4; f16 unrejected 0 | slot filters unpinned (TE-3); dropped count unpinned (TE-9) |
| Q4 | q4 + plant 3 | honest-slot half cannot fail (TE-3) |
| C-U4F2-10 | c_u4f2_10 + plant 1b | none (artificial records, noted) |
| C-U4F2-1 | f16 kinds 2, 5 at 12, P0, steady | shared pair (TE-4) |
| Twins per kind (2.12, carry 46) | f16 x24; f15 kinds 3, 7, 8, 9, 10, 11 twin rows | kinds 1, 2, 4, 5, 6 unpinned (TE-2); not per kind (TE-4) |
| Forgers' kinds (f15) | 6 bots tests | none |
| C-U4F2-2 | c_u4f2_2 + plant 11 | stamp half only where no stamp is used (TE-13) |
| C-U4F2-5 (c) | sim-only short + full (b7-stand) + plant 8 | short copy passes (b)'s slow walk (TE-14) |
| C-U4F2-7, T-6 | contract tests + plant 7 | none |
| C-U4F2-8 | f6 + plant 6 | value, not source (by construction) |
| -21's set runs and passes | set 21's plan; hand-a/-b (old tree); hand-long2 | fails as built (TE-1); old tree (TE-7); band (TE-6); lag check vacuous (TE-5) |
| Test step under 1,207 s | end-a3, end-b | margin under noise (TE-8) |

## Findings (ranked)
- **TE-U4F2-1, High.** spikes/netcode-proof/crates/bots/src/lib.rs:868-930 (`forge_step`: kind 7's items, open loop)
  with crates/sim/src/world.rs:1351-1355 and :1192-1202. -21's forged part fails as built: a forged edge while the
  forger is dead gets no record (unrejected 18 of 1,523 on hand-long2; marks.json:4959-4968 needs 0); a revive between
  the honest edge and the forged one lets the forged press fire, which would also fail the twin figure (no long pair
  run). No test reaches a death next to a kind-7 item. Fix: netcode's route inside the plan (no mark, judge or
  marks.json change). Proof owed: a test where the kind-7 forger dies and revives inside an item's cadence, failing by
  name on today's code (30-60 lines, unit level); then a 610 s forged pair per option on the fix tree, judged in 5.4's
  form: every kind's unrejected 0, twin mismatch 0, the shot band (4 runs).
- **TE-U4F2-2, Medium.** bots/src/lib.rs:1585 (`sides`, the one `.twin()` at :1599) and :1426-1532. Plant 9 passes
  every test: no test pins U4F-Q13 (b)'s twin for kinds 1, 2, 4, 5, 6. Fix: a bots case on `sides(k, false)` for
  those kinds, asserting the twin's sequence set is the forger's less the forged ones and the rest equal (the kind-10
  pattern, :1891-1901); plant 9 fails it by name; run plant 9 under option B too. 15-25 lines, about 0 s.
- **TE-U4F2-3, Medium.** lab/src/forged.rs:81, :86, :343-354. Q4's honest-slot record names a sequence the forger
  never forged (3), so removing either slot filter passes all 15 forged:: tests and f16; a cross-slot record can only
  make an item read rejected, the masking direction (kind 7's sim records included). Fix: in q4 or a new neg case, an
  own-kind record on another slot naming a forged sequence (room kind 10 on seq 2; sim Cooldown for a kind-7 item)
  leaves it unrejected; plants: each filter removed. 8-12 lines, 0 s.
- **TE-U4F2-4, Medium.** lab/src/scenario.rs:4105 (`pair`) and :4364 (`f16`). One pair per option for all 11 kinds
  departs from 2.10's scale (u4f-brief.md:670); a failure names the test's kind, not the culprit
  (plant10.log:21-25); return list item 6 cannot be met per kind. Fix: the advisor rules the share; f16's failure
  names the first differing fate entry's link (slot = kind - 1) and, on a hash mismatch, the forger slots whose Soldier
  records differ at the first differing tick. 10-30 lines, 0 s.
- **TE-U4F2-5, Medium.** bots/src/lib.rs:1046 (`Kind::Steady => [0, 0]`) through the lag client; scenario.rs:4225.
  f20's movement assertion, p4/p18's expect and -21's lag movement check cannot fail. Fix: the lag client on a moving
  open script (the Lane walk, the fast clock's) with f20 asserting the series moves above its standing value on some
  tick and stays at most 0, its "in a building: 0" shown; else the advisor records the check as unable to fail in
  -21's report. 5-15 lines, f20's run reused.
- **TE-U4F2-6, Medium.** Set 21's bite runs under A (c-u4f-runs.mjs SETS[21]). PM-0001-2's 665 floor: no-lead-bound
  reads 669 / 668 by construction; nothing at 600 s, under B, or at seeds 2-3. An invalid bite run leaves its bite
  unshown. Fix: before the set, bite-no-lead-bound at 610 s (seed 1, A) and a clean forged 610 s run under B on the
  final tree; a mean under about 670 goes to the advisor (U4F-Q10's option for that bite, or kind 1's k range; the
  band is Mustafa's). 0 lines, 2 runs.
- **TE-U4F2-7, Medium.** u4f2-hand-a.log:3 and u4f2-hand-b.log:3 against u4f2-end-b.log:3. Section 6 item 5's lines
  and both judge calls ran before F-D's rebuild. Fix: re-run the forged A and B pairs (10 s + 5 s, measured and twin)
  and the two judge calls in 5.4's form on the final tree; expected the twin check PASS on both options and every
  kind's unrejected 0. 0 lines, about 2 minutes.
- **TE-U4F2-8, Medium.** The test step: 82 s under 1,207 s, against a measured 75.6 s spread for one test set (931.9 s
  and 1,007.5 s at 1faf894). Fix: the advisor names which run the line reads (the worktree's counted check, as A3 and
  B) before the fix pass; fix-pass tests at unit level or on an existing pair; any new lab run to the advisor first.
- **TE-U4F2-9, Low.** forged.rs:136 (`records`). SEC-1's `refusals_dropped_count` is asserted only as 0. Fix: an f33
  line asserting `records(3)` carries 3 on each record; plant: write 0. 2 lines.
- **TE-U4F2-10, Low.** ops/runs/c-u4f-r2.mjs:61-75. No -21 row's `expect` names "in a building: 0"
  (u4f2-brief.md:110). Fix: add it to the -21 rows' shared phrases. About 3 lines.
- **TE-U4F2-11, Low.** c-u4f-runs.mjs:80 with c-u4r-runs.mjs:154. 56 MB a run against 65.2 MB measured. Fix: a
  measured -21 figure for set 21's projection. About 3 lines.
- **TE-U4F2-12, Low.** scenario.rs:4408-4416 (f16's client_counts asserts). 2.5's "0 on honest clients and on each
  forger outside its kind" is asserted for slot 9 only (f22). Fix: f16 asserts every forger's refusal counters but its
  own kind's, and every honest client's, 0 on the shared pair. About 10 lines, 0 s.
- **TE-U4F2-13, Low.** client/tests/endpoint.rs:509 with client/src/lib.rs:565-589. C-U4F2-2's "stamp used" is tested
  only on a fixed lead, where no stamp is used (stamp = newest + lead). Fix: a controller case where the skip takes
  the next due stamp. About 15 lines, 0 s.
- **TE-U4F2-14, Low.** Records and docs: the plants' file:line (u4f2-build.md:154-186); `twin()`'s doc says kinds 1-6
  skip "at the next step" (bots/src/lib.rs:562-565) while `forge_step` skips at the due step (:921); the short
  C-U4F2-5 copy passed (b)'s 1/32 walk that the full test failed (u4f2-b7-walk-fix.log:30, :52), to be said in the
  report; the panel brief's patch hash typo. Report lines and about 3 doc lines.
- **TE-U4F2-15, Low.** scenario.rs:2240 (`parse`'s `--forge-every`) with bots/src/lib.rs:944. Any u64 from 2 is
  taken; `every * 8` overflows above 2^61 (a debug panic). Fix: an upper bound with a case (about 4 lines); the
  security seat's call.

## Verdict: REQUEST CHANGES
The 60 tests and the counts hold, and 10 of 11 plants bite by name. But -21's forged part fails as built on the one
judged-length run (TE-1), three claims rest on checks that cannot fail (TE-2, TE-3, TE-5), and the judge calls ran on
a superseded tree (TE-7). Nothing here needs a pass mark, the judge, marks.json, money or main; the shot band and any
bar reading in TE-6 would be Mustafa's under TEST-FIRST if a fix reached for them.
