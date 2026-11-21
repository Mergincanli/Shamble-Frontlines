# U4f2 review round 2: the netcode seat (netcode-architect), 2026-11-21

Round 2 (the last, NPT-TRIM history.md:7561-7593: netcode and test seats; the rust seat's FP-3 and bots items split
to me). Scope: the fix pass's diff `git diff 3441d47f…ca 7c362a63…a` (7 files). Read only. (No Edit tool in this
agent: the file is rewritten whole with Write after each block.)

## Verdict: (block 6)

## Plan
1. Read the ruling's blocks 4-5 and FP table (done), part A's ruling (done), his F-B-COUNT words and NPT-TRIM (done),
   the build record's Blocks 9-10 (done), my round-1 file (done).
2. The diff's stat, then each file's hunks: bots lib.rs (FP-1 `forge_step`, FP-3 `fire_rest_at`/`forger`, F-G, FP-4
   `lane_west`/`lane_walk`, FP-5, FP-8 `checked_mul`, FP-2 `kind_7_left_out`, T-K7, fp3, fp5), forged.rs (FP-2's
   window, FP-6, FP-8), scenario.rs (`dead_before`, FP-4 wiring, FP-7, FP-8 usage, f16/f20), room.rs, SERIES.md, the
   two scripts.
3. FP-2's window against his words: when `dead_before` is sampled relative to the command phase, `recent`'s first-run
   rule, option B's re-run, d-1 never ran, the counts, "on every run".
4. FP-1's construction (reload half), FP-3 const fn and pins, F-G, FP-4 lane and the fast clock (R5's proof), FP-5, the
   bots refactor.
5. The departure: bots lib.rs `kind_7_left_out` and T-K7 lines; can any non-forged line move.
6. The evidence logs (R2, R3, lag proof, fp2 tests/plant) against the figures claimed.
7. Findings table and verdict.

## Blocks
