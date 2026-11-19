# U4f-F1: the stand-in takes every press edge (ADV-U4F20-1), then -20's L1 set again (plan rev 1.1, 2026-11-19)
**Rev 1.1:** the advisor's review APPROVE WITH CONDITIONS 1-10 (advisor-u4f-f1-plan.md: findings :173-184, rulings
:155-171, conditions :186-225, read in full). Conditions 1-9 bind the build as written there (they replace this
plan's text where they differ: all four press counters in today's order, one shot per command, the shared fn's
contract, T1's sim half, T2 in the d10 twin form with cases (a)-(f) and two plants, the re-run compared tick by tick
with 350677f's kept logs, security-auditor in the round, a session's first command named and carried to U4f2, the four
doc sites, the wording of item 3); condition 10 (U4f2's brief rebased on F1's merge, with condition 7's item) is the
main session's after the merge. Rulings: Q1 one integer-only `pub fn` in sim/src/seam.rs, called by npt-room; Q2 no
change; Q3 F1 first, before U4f2.
**For the advisor's plan review before any edit.** Short by TEST-FIRST rule 3: what it changes, how it is tested,
what is open. Source: the advisor's -20 ruling (advisor-u4f-20.md, R20-Q1 :114-140, ADV-U4F20-1 :162), on the
main session's read (u4f-20-read.md). Inside plan rev 3.3 by that ruling (TEST-FIRST rule 2; R20-Q2 :141-145): no
pass mark, judge rule, marks.json text or run count changes; no money; nothing on main.

## 1. Why, in one paragraph
-20's option-B part failed "0 presses lost" (3, 1, 2) at 350677f. After late drops, option B runs a command whose press
counter is two edges on; U4f's kind 5 accepts that (room/src/input.rs:285-300: one edge per covered tick); npt-sim then
takes one edge per changed counter (sim/src/world.rs:1340-1346, U4d's, "the room allows one per button per tick"), so
one press is lost. ADR 0001 part 5 keeps the press across a dropped command (:226, :234-237). The defect is the
stand-in's, not option (b)'s.

## 2. What changes (the lane: crates/sim and crates/room, their tests; nothing else)
1. **npt-sim** (world.rs `run_command`, :1340-1346 and where `edge` is used, :1408-1466): a button's edges are its
   counter's advance (`now.wrapping_sub(claimed)`, as the room counts), and each edge goes through the same checks as
   today in order (cooldown, ammunition, reload); a refused edge counts as a refusal does now and leaves no trace in
   the state (world.rs:30-35). The comment at :1340-1341 is corrected.
2. **The rule defined once:** one `pub fn` in npt-sim (beside the press-counter types it already owns) gives the edges
   of an advance; npt-sim's `run_command` and npt-room's `run_one` (room.rs:1330-1349) both call it, so the room's
   seen/lost/doubled counts cannot drift from what the sim takes. "Lost" then means edges the sim could not take: 0 by
   construction for a taken edge; the series' meaning (SERIES.md:42, "press edges lost this tick (against the press
   counters)") does not change, so no SERIES.md row changes.
3. **No other behaviour changes:** a 1-edge advance (every honest command today but the six) runs exactly as now.

## 3. How it is tested
- **T1 (npt-room, option B):** two commands dropped late before a command whose counter is 2 edges on: seen 2, lost 0,
  doubled 0. **Plant:** the old one-edge rule; T1 fails by name.
- **T2 (npt-sim):** from one state, a 2-edge fire command and a 1-edge fire command give the same state hash (the
  second fire edge meets the cooldown and is refused, state-neutral: the advisor's Block 3 reading, expected, to be
  proven); and a 2-edge reload or ability advance takes both edges through their checks. **Plant:** the old rule;
  T2's edge-count half fails by name.
- **The existing suites:** every sim, room and lab test passes unchanged, the twins' and fate logs' pins included
  (no 1-edge path changes); check.mjs twice, all 11 steps, the counts as deltas on 2dae645's (634 measured and 731
  bites, as U4f merged them), +2 or more in both builds.
- **The record:** `c-u4f-runs.mjs 20 --date <day>` from U4f-F1's merge commit, whole (7 runs, about 41 min, no money),
  judged in its two calls; expected: option A as before, option B "0 presses lost" 0 on all three, the bite shown.
  That verdict is -20's L1 record; 350677f's folder stays as the defect's record (R20-Q1 :129-131). -22 does not re-run.

## 4. Size, reviewers, who builds
- **Size:** about 60 to 150 added lines (the fix 20 to 50, T1 and T2 40 to 100). Size S: SIZE-CHECK's stop past twice
  the plan's figure, so past 300 added lines.
- **Reviewers (one round, at most two):** rust-reviewer (determinism: the wasm and native hashes; the wrap of
  `wrapping_sub`), netcode-architect (the ADR part 5 reading, the room's counts), test-engineer (T1, T2, the plants).
- **Who builds:** one builder (Opus 5.5, xhigh) in an isolated worktree on the lane head, its notes in its worktree's
  copy of u4f-f1-build.md, snapshots by worktree_patch.py, the shared target under the lane lock; the standing write
  sentence word for word in its brief. Then the main session merges (gate --no-smoke, check.mjs, CI), and runs -20.

## 5. Questions for the advisor
- **U4FF1-Q1:** where the one rule lives (npt-sim, beside its press-counter types, as section 2 item 2), and whether
  npt-room's `run_one` may call it (npt-room already depends on npt-sim: room/Cargo.toml:21).
- **U4FF1-Q2:** a press edge on a dead or plate slot (world.rs:1348-1350 returns before the edges are used): its
  `claimed` still advances, as today; confirm no change there.
- **U4FF1-Q3:** the order: U4f-F1 before U4f2 (my lean: U4f2 waits for his U4F2-SIZE answer, and U4f2's lane keeps
  crates/sim shut), or inside U4f2 (R20-Q1 allows either).
