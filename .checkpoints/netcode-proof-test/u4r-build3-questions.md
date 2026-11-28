# U4r build, copy 3: the points for the advisor (the main session, 2026-11-14 about 01:00 UTC)

The U4r builder's third copy (agent a5d1a2ea28e8704c8; 21:54-22:02 UTC, cut by the spend limit, resumed 00:16-00:50)
stopped at a clean point near its context limit, with item 4 (r2, the contract test) and the advisor's second ruling
done. Its notes: `.checkpoints/netcode-proof-test/u4r-build.md` (906 lines, an exact copy of the worktree's; copy 3's
part is :571-906). The snapshot of the worktree: `wip/u4r.patch` (20 files on 2fbec77, sha256
987946ccf717878329f55308f8b3f693f544863000402d9c3437daf6079ae097), applied on the fixed copy
`/home/user/Shamble-Frontlines/.gate/u4r-snap1` (its 20 files equal the worktree's by cmp): read code there. The logs:
`/home/user/Shamble-Frontlines/.gate/u4r/` (copy 3's are c3-*.log; the r2 suites' logs and verdicts under r2/). The
brief: `u4r-brief.md` (59d570b8…). The earlier rulings: `advisor-u4r-build1-full.md` and `advisor-u4r-build2-full.md`.

A fourth copy works meanwhile on ruling 1's point 1 (4) to (7) with r10's check.mjs half and on B12's run scripts. It
holds r5, section 5's end checks and anything the points below touch until the rulings reach it.

Each point gives what the notes say and the question. No option is preferred here. For each point the ruling gives:
the decision; the severity on the project's one scale; what the builder does (to be sent by message); what the main
session does; whether it blocks U4r's merge or becomes a follow-up or a later unit's item; and whether it goes to
Mustafa, and why.

## 1. B1: PM-0001-2's "everyone in view" fails, so -17's and -18's L1 B1 parts are INVALID (notes :671, :679-692)

- The short B1 suite (48 clients, seed 1, 10 s + 30 s, two 1 s bursts): PM-0001-2 FAIL, everyone in view 0.273333
  (needs >= 0.99), so -17 and -18 L1 B1 INVALID by pre:PM-0001-2; -57 passes; the mirror logged 0 breaches.
- Copy 3's reading (:685-692): after each burst every client's baseline is evicted (`burst_eviction`, baseline_age 0,
  all 96), the room then sends about 1,000 B a tick per client, and each client's set is rebuilt from nothing over
  about 10 s (per 30-tick window: 47, 0, 0, 0, 0, 0, 4, 12, 23, 31, 40, 44, 45, 46 slots holding everyone). At 10
  bursts in 600 s every judged L1 B1 run would be INVALID for -17 and -18. npt-room is outside U4r's lane.
- Asked: is this the room's designed behaviour (U4e's burst handling; -18's eviction and heal) or a defect; what
  follows for -17's and -18's B1 parts at U4r (an npt-room change and in which unit; the precondition's window; the
  parts deferred; another); and whether any part goes to Mustafa (PM-0001-2 holds his floor: history.md).

## 2. ES: reliable events 50.05 a second against the band 9.5 to 10.5 (notes :676, :692-695)

- The ES suite of the brawl (carry 59; ES, P0, 48 clients): PM-0001-2 FAIL on ES reliable events, 50.05 a second
  (needs at most 10.5); its other 11 ES checks pass (shots 72, projectiles 32.38, spawns 21.1, everyone in view 1).
- Copy 3's reading: the brawl aims at its target's exact position (mirror.rs `view`; bots `brawl_aim`), so about 70 %
  of its shots hit. The brawl is U4r's (npt-bots), but tuning it to a band is a design choice (carry 59; U4d-F's 7.250
  figure); not changed.
- Asked: whether and how the brawl changes (and what evidence shows it), with the band unchanged.

## 3. Carry 73's tick16 bite run is INVALID, not FAIL (notes :708-729)

- SS P0 seed 1 from 30 ticks below the wrap, `--bite PM-0001-17/tick16`: after copy 3's two fixes (point 7) the run
  ends clean and the mirror has 0 breaches, but the judge gives INVALID by pre:PM-0001-2, everyone in view 0 on every
  client and measured tick from tick 65,746 on: the room's own sets collapse past the wrap once the bitten clients'
  acks and commands stop matching. Carry 73 expects "the judge's outcome the mismatch figure FAIL with at least one
  mismatch, never INVALID". marks.json and the judge are the main session's.
- Asked: how carry 73's bite is shown (and whose change that is).

## 4. -18's no-fallback bite: 25 of 96 heals within 500 ms (notes :701-707)

- b1's settings with `--bite PM-0001-18/no-fallback`: 96 heal records, 25 within 500 ms, median 1.197 s, max 1.688 s;
  the bite fails -18's max check; the clean b1 has 96 of 96 within 500 ms. Brief 2.4 r2 expects "no client holds a heal
  within 500 ms" on the bite. The judge: -18 INVALID by pre:PM-0001-2 (as the clean b1).
- Asked: is the brief's expectation wrong, and what r2 reports.

## 5. Ruling 2's point 1 (4): "no deviation" at the recording's end (copy 3's report, finding 5)

- In r6 the recording's last L - 1 ticks hold 48 takes of depth 0 at tick 119 (no later command exists); copy 2 had
  read these 0s as the first takes. Copy 3 reports every deviation as point 1 (2) says, and r6 asserts the relation
  wherever it can hold.
- Asked: the words of point 1 (4) for the recording's end.

## 6. r2's design: the `npt-lab contract` subcommand (notes :590-605; crates/lab/src/contract.rs)

- r2 in two halves: a checker subcommand beyond 2.7's list (r2 (b), (c), (e), carry 60 as a failure, the carries'
  figures as report lines; U4R-Q11's four negative cases and a fifth in its unit tests, in check.mjs's count), and the
  suites run by hand through the release binary (`scenario`, `contract`, the real `npt-judge --only`). Ruling 2's point
  5 left this design to copy 3; copy 3 asks for a ruling.
- Asked: rule on it.

## 7. Copy 3's two code fixes found by the tick16 runs (notes :712-722)

- (a) A bite run that reaches the after phase's cap ends there (`Summary.after_capped`, `run_lengths.after_capped`, its
  SERIES.md row widened): the stop decides each client's unresolved ticks inside its span lost and the logs end clean.
  A measured run still fails at the cap (carry 62 (e)). (b) The mirror keeps frames and looks baselines up by the
  room's tick, not the wire's 16-bit tick (equal in a clean run; a clean P0 re-run equal figure for figure: :730-731).
- Asked: accepted or not; (a) touches carry 62 (e)'s rule for bite runs.

## 8. Anything else

- Anything in copy 3's part (:571-906) that departs from the brief or the rulings, including its SERIES.md rows
  (:622-645), point 33's labels (:834-858) and the per-profile bytes (:733-752).
