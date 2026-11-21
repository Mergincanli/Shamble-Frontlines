# U4f review panel, round 1, seat `test` (test-engineer), 2026-11-19

Tree: /home/user/Shamble-Frontlines/.claude/worktrees/agent-a0825a5a2491eb175 (HEAD 07289a4 + uncommitted U4f change). Read only.
(No Edit tool in this seat: the notes file is rewritten whole with Write at each save.)

## Plan
1. Mustafa's SIZE-U4F words (history.md:6896-6903) and advisor-u4f-stop.md :80-89 (the U4f2 list), :198-210 (S1-S7, N1).
2. u4f-brief.md 2.10 (tests, negative cases, minimums), 2.11 (bites/plants), 5.3, 5.6, 6; advisor-u4f-brief.md rulings + L1-L6; advisor-u4f-build1.md R1-R4/F1-F11.
3. The patch: numstat; confirm no marks.json / judge file touched; SERIES.md +20 lines, -0.
4. Each new test file + bites.rs: map brief 2.10's named tests to code, count U4f's negative cases; tests that cannot fail; timing-dependent tests.
5. marks.json bite ids vs the code's BITES lists (NA-11).
6. End-check logs A2/B: measured and bite counts vs minimums; hand logs.
7. SERIES.md's 20 new lines: the read.
8. Runs only to prove a finding (runner, one command per call, logs under .gate/u4f/review-test/).

## Findings log (one line each, as found)
- [14:0x] Tree unchanged: `git diff HEAD -- . | sha256sum` and the untracked hash, recomputed read-only by u4f-run.sh:67's own commands in spikes/netcode-proof: diff 54cb6faa..., untracked e8dc3663... (equal to A2 and B).
- [14:0x] Bars: `git apply --numstat` on wip/u4f-build.patch (sha256 74ae26aa..., checked) lists 29 files; no marks.json, no judge file, no marks/README; SERIES.md 20 added / 0 removed. `git diff --quiet 5b29296b HEAD -- spikes/netcode-proof` exit 0 in the worktree. His 13:35 rule: nothing to read; confirmed.
- [14:0x] Test inventory (scratchpad tests_list.py, read only, worktree vs HEAD): 96 `#[test]` fns added, 1 removed (P33's move: loopback.rs -> lab/tests/bite_homes.rs). 17 bites-only (14 unit proofs + f30 x2 + lab `f2_the_sims_check_bites_are_routed...`), so +78 measured, +95 bites: equals A2 (w3-end-check-a2.log:196) and B (w3-end-check-b.log:183).
- [14:0x] Minimums arithmetic: 2.10's 67 negative = 65 from the table's neg rows + U4F-Q6 (b) 1 + U4F-Q9 (a) 1. U4f2's share: f15 x6, f16 24, f22 2, f23 1, f33 13 = 46 measured/bites, 22 negative. U4f's share = measured 73, bites 88, negative 45. Built 78 / 95 (met); negative 44 (below).
- [14:0x] NA-11 met: all 17 of 2.11's ids are string literals in exactly one crate's src (client lib.rs:34-40: 3 new; room lib.rs:39-56: 9 new + -22's; proto quant.rs:40-41; sim world.rs:611-612 in BITE_IDS, `World::CHECK_BITES` :951), spelled as marks.json's `"id"` entries (17 ids grepped, each found). check.mjs's biteHomeProblems (:936) reads literals, so CHECK_BITES is covered; LAB_BITES unchanged (scenario.rs:219-223); pins updated (client/tests/bites.rs, room/tests/bites.rs).
- [14:0x] The 14 unit proofs are two-sided (clean refuses, bitten lets through): `shows` (commands.rs:919-934, asserts [false, true]), no-seq-forward :963-990, no-catchup-cap :1042-1066 ([8, 16]), no-cooldown :867-891, refuse-not-clamp :896-916, -47's two and redundancy-1 (trace.rs:487-539); f30 x2 fail on a panicking thread (unwrap_or to the failing value).
- [14:0x] Plants: f9 (plant-f9.log:13,19: carry_21 FAILED, via npt-sim's debug order assert world.rs:2129), f26 x2 (w2-plant-f26-1.log:41 at carry80.rs:112; -2.log:13 at :124), f27 (w2-plant-f27.log:13 at :147), D14 (w2-d14.log:13-14, misorders, no panic): each fails by name, each restored with sha256 equal. All ran on earlier trees (827518b / d1b24c3 + snapshots), not the final one; carry80.rs is 158 lines then and now. No finding.
- [14:1x] FINDING TE-U4F-1 (Medium): f5 measures another share than its bar and loosens B1 to 3 % unruled. client/tests/trace.rs:385 `late = 1 - on_time/ticks` counts never-arrived ticks as late; the brief's f5 (u4f-brief.md:688) bars "at most 1 % of commands arriving after their stamped tick", per profile, B1 included; trace.rs:391 `budget = if p.burst.is_some() { 0.03 } else { 0.01 }`, while its doc (:374-375) still says 1 %. No ruling (advisor-u4f-build1.md and -stop.md: no B1 budget line); builder 1 named it (u4f-build.md:122-123). On the final tree (.gate/u4f/review-test/f5-b1.log:16): B1 not on time 0.01879, missing 23 of about 1,490 ticks, so about 5 ticks (0.34 %) arrived after their tick: the brief's bar, measured as written, passes; the 3 % is not needed. Fix: count arrived-after-stamp apart from missing, hold it <= 1 % on all six profiles, print missing beside it.
- [14:1x] FINDING TE-U4F-2 (Medium): U4F-Q9 (a)'s flood test is not built. 2.10 (u4f-brief.md:655-656) makes it firm: "+1 by U4F-Q9 (a) (the kept window never holds more than K under a flood of new sequences, a negative case)"; the advisor's ruling (advisor-u4f-brief.md:265) "Q9 (a)'s flood test is a negative case". No test reads `kept`'s length (grep: KEPT_WINDOW only at input.rs:179, :229); not on any builder's not-built list (u4f-build.md:256, :539-541, :849-857); not in U4f2's list (advisor-u4f-stop.md:80-89). f12 (iv) (commands.rs:464-484) shows a sequence 18 back is outside, which holds for any K up to 17: K = 16's edge is unpinned too. The bound holds by construction (input.rs:229-231). Fix: an in-crate test in input.rs offering 10,000 new sequences (across the u16 wrap) asserting `kept.len() <= KEPT_WINDOW` after each, plus f12 (iv)'s edge: 16 back inside (kind 3 on changed content), 17 back outside.
- [14:1x] FINDING TE-U4F-3 (Low): the not-older-than unit proof cannot tell its plant from no-newer-tick's: commands.rs:952-960 both run `equal_tick` (:946-950); a plant of input.rs:272 `d >= 0` -> `true` (kind 2 removed whole) passes both. Fix: in pm_0001_21_not_older_than also send an older stamp (j+1 after j+2) and assert the bitten room still refuses it as kind 2. Related evidence: the bite reaches the fast-clock site at 2 s (w3-hand/p9/lab.jsonl: not_newer 0, excess -7/-6 against p3's 29-30 and -36/-36), but no test asserts it; a bites-build f19 twin asserting not_newer 0 and excess above the clean run's would make it a test.
- [14:1x] FINDING TE-U4F-4 (Low): f24 does not pin U4F-Q16's bound: carry80.rs:64-82 uses newest+40 (dropped) and newest+20 (taken), which passes for any bound from 20 to 39 (ACK_RING_TICKS is 32). Fix: cases at +32 (taken, sentinel) and +33 (dropped, no move).
- [14:1x] FINDING TE-U4F-5 (Low): f4 is counted as a negative case but feeds no hostile input: trace.rs:368-371 asserts an honest P0 endpoint sends 98..=101 pings in 10 s (a total, not "never exceeds" in any second); the brief's "(a faster rate is limited and counted by the pump)" half is U4e's e9 (server/tests/pump.rs:193-212), not new. f31 (clock.rs:337-347) pins the gaps at every 3rd step, which does cover the per-second rate. Fix: name e9 as f4's negative half in the report's count, or assert the per-second count.
- [14:2x] FINDING TE-U4F-6 (Low): the header's `lead_bound_ms` records the constant, not the room's setting: scenario.rs:1899 inserts `Room::LEAD_BOUND_MS`; `Room::with_lead_bound_ms` (room.rs:601) exists, so a run built with another bound would still record 400. No lab path sets another today. Fix: write `room.lead_bound().0`.
- [14:2x] FINDING TE-U4F-7 (Low): f2_the_sims_check_bites_are_routed_to_the_world_alone (scenario.rs:4186-4197) iterates `Sim::CHECK_BITES`, so it passes if an id is dropped from that list; nothing else pins CHECK_BITES' two ids (sim/tests pins World::BITES only). Fix: iterate the two literal ids.
- [14:2x] FINDING TE-U4F-8 (Low): seeded traces whose docs do not name their seed, against 2.10's generator rule (TE3-11, u4f-brief.md:661-664): f4 (trace.rs:365-371, seed 4), f5 (:373-394, seed 5), f32 (ii) (:416-432, 32), f32 (iii) (:434-449, 33), f36 (i) (:451-474, 36), the three bites and f30 (47, 48, 20, 30). Fix: one doc phrase each.
- [14:2x] Noted, no finding: f12 (iii) prints seq_outside_window unasserted (by design: never forged); f36 (i) runs one option-agnostic trace "for both options" by argument (trace.rs:451-455), with f28's lab halves covering both options at P3 short; f19 lab's excess bar (<= 0) cannot see not-older-than at 2 s (-6), as builder 3 said: the 60 s fail stays arithmetic until 5.3's p9 (known: advisor-u4f-stop.md R3 (iii)); the lag switch's catch-up check cannot fail at 200 ms (TE-7, ruled U4F-Q15). f36 (ii)'s one failure (240 stamps) was root-caused, not retried (u4f-build.md:398-400); no other retry or flake in the notes.
- [14:2x] Timing: no Instant, SystemTime, sleep or elapsed in the new code or tests (grep of the diff and the six new test files); the lab tests run on simulated time; scratch folders carry the process id (scenario.rs:2438-2451); shared runs use OnceLock per option. No timing-dependent test found.
- [14:2x] -21 cannot be judged before U4f2 by anything built: c-u4f-runs.mjs's SETS holds 47, 22 and 20 only (:47-51) and its usage refuses any other key (:67); c-u4f-r2.mjs writes only under .gate/netcode-proof-test/ (its -21 rows are pre-runs, the advisor's "early read"); the hand runs' -21 judge call (w2-hand.log:78-81) sits under .gate/. S6 (the header order) stands as ruled.

## Verdict: APPROVE WITH FINDINGS

No Critical or High. Two Mediums (TE-U4F-1, TE-U4F-2) and six Lows, all for the one fix pass; none needs a marks.json, judge or SERIES.md row change, and none touches a pass mark.

## Claim · proof · gap (this seat)
| Claim | Proof | Gap |
|---|---|---|
| +78 measured, +95 bites on base | A2 :196, B :183; inventory 96 added, 1 moved | none: U4f's share 73 / 88 met |
| U4f's negative cases (share 45) | counted by row in the code | 44: U4F-Q9 (a)'s flood test missing (TE-U4F-2); f4's negative half is U4e's e9 (TE-U4F-5) |
| 17 bites, each with a unit proof; NA-11 | literals grep; marks.json ids; 14 two-sided proofs; pins | not-older-than's proof = no-newer-tick's (TE-U4F-3); CHECK_BITES unpinned (TE-U4F-7) |
| plants f9, f26 x2, f27, D14 bite | runner logs, each FAILED by name, restored, sha256 equal | ran on earlier trees (no finding) |
| f5: settled lead in 1..=9, late <= 1 % per profile | trace.rs:377-394; f5-b1.log:13-18 | B1 at 3 %, and "late" includes missing (TE-U4F-1) |
| f1 p99 <= 4 ms; f36 >= 99.9 % on time | trace.rs:347-363, :456-474; builder figures 2.380 ms, 99.988 % | f36 "both options" by argument (noted) |
| lead bound from the tick run next | commands.rs:163-173 (j+12 taken, j+13 kind 1) | none |
| kinds 1-6, 8, 10 under both options, one kind per refusal | commands.rs f10 x8, f11 x2, f12 x5, f13 x4 | K's edge unpinned (in TE-U4F-2) |
| never-acker, cap 1,600 | f21 (scenario.rs:3803), room/tests/room.rs:1141's pin, hand-s22 | none in this seat |
| carry 80 (b)-(e) | f24-f27 + plants | f24's ring edge (TE-U4F-4) |
| fast clock refused by kinds 1 and 2, excess <= 0 | f19 bots + lab; w3-hand p3/p9/p16/p20 | the bite's 60 s fail is arithmetic (known) |
| no marks.json or judge change | numstat; HEAD = 5b29296b in spikes | none |
| -21 not judgeable before U4f2 | c-u4f-runs.mjs SETS; r2 under .gate | none |
| no timing-dependent test | grep; simulated time | none |

## Lines asked for (each its own line)
- U4f's share of 2.10's minimums: measured +78 against its share of 73 (119 less U4f2's 46: f15 x6, f16 24, f22 2, f23 1, f33 13), bites +95 against 88 (134 less 46), negative cases 44 against its share of 45 (67 less U4f2's 22: f15 x6, f22 2, f23 1, f33 13); the one short is U4F-Q9 (a)'s flood test (TE-U4F-2), and f4's negative half is U4e's e9 (TE-U4F-5).
- U4f's 44 negative cases by row: f3 2, f4 1, f7 1, f8 3, f10 8, f11 2, f12 5, f13 4, f14 3, f15 5, f19 1, f20 1, f21 1, f24 1, f25 1, f26 1, f27 1, f35 1, f39 1, U4F-Q6 (b)'s Pong field 1 (server/tests/pump.rs:336).
- SERIES.md's 20 new lines, read in full: one new subsection (heading, two blanks, the table's two head lines, 15 rows); no existing row touched (-0); 14 rows' Marks cells read "none (recorded)" or "none (reported)", and the `event.reorder.*` row names the existing `reorder` rule, unchanged (S5, ruled); the `header.params.lead_bound_ms` row quotes u4f-lead-bound.md:22's fenced sentence byte for byte (that file's sha256 57c588ec..., equal to the freeze); no new name repeats an old row's (the old -21 rows :33-34, :144-145, :155-156 keep their words; the new fast_clock, lagswitch_move_apart and clamp_kind fields sit beside them); every name the new lab tests' runs write has a row (contract (b) asserted in f28 x5, f18, f20, f14 (iii)). One nit outside the build: advisor-u4f-stop.md's "20 new rows; 19 rows marked none" counts lines as rows (15 rows, 14 marked none).
- His 13:35 rule: nothing to read: the patch changes no marks.json and no judge file (numstat of the 29 files; spikes/netcode-proof at HEAD equals 5b29296b).
- S1-S7 and N1: no disagreement (N1 is the netcode seat's). On S3: the four hand runs prove the command lines; hand-p9 at 2 s cannot show the bite failing its check, though its lab log shows the bite reaching the site (TE-U4F-3).

## Ran
- bash u4f-run.sh <worktree> .gate/u4f/review-test/f5-b1.log cargo test --locked -p npt-client --test trace f5_ -- --nocapture (shared, 14:11:59-14:12:33 UTC, exit 0; free 5,061 MiB; diff 54cb6faa..., untracked e8dc3663...).
- Read only, outside the runner: the tree hashes by u4f-run.sh:67's commands; sha256sum and `git apply --numstat` of the patch; `git diff --quiet 5b29296b HEAD -- spikes/netcode-proof`; python3 -I scratchpad/tests_list.py (test inventory); greps of the sources, marks.json, SERIES.md and the .gate/u4f logs; `df -h`.

## Not checked
- The room's check order and semantics against ADR 0001 and 2.4 beyond what the tests assert (netcode and security seats); Rust safety, overflow and allocation (rust seat); public items (arch seat).
- I re-ran no plant and no bite; the plants' failures are read from the builders' logs on earlier trees. I ran no full test step, no check.mjs, no lab scenario, no judge call.
- f1's, f36's and the cap's figures are the builders' (logs and notes), not re-run; the fast clock's 60 s arithmetic (+180) is unproven until 5.3's p9.
- u4f-brief.md 5.3, 5.6 and 6 read only where my findings needed them; advisor-u4f-brief.md L1-L6 and the fold check not read in full; c-u4f-cap.mjs not read.
