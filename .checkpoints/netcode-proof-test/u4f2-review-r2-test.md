# U4f2 review round 2 (2026-11-21): the test seat (plus the rust seat's FP-8 items under NPT-TRIM)

Read only; this file is my one write (no Edit tool here: rewritten whole with Write each time, as the rules allow).
Scope: the fix pass's diff 3441d47f..7c362a63 (round 1's snapshot e9a0ae3e on a289def against the final snapshot
e33d14f0 on a289def), the builder's record u4f2-build.md Blocks 9-10, the logs under .gate/u4f/. Rules:
u4f2-panel-brief.md:65-79.

## Plan
1. Read the rulings (advisor-u4f2-r1.md blocks 4-5 and the FP table; advisor-u4f2-parta.md), Mustafa's F-B-COUNT (a)
   and NPT-TRIM (history.md:7544-7593), the build record's Blocks 9-10.
2. The diff: --stat, then forged.rs, bots lib.rs, scenario.rs, the rest.
3. TE-U4F2-1's proof: FP-2's rule against his words, the f33 case's five cases, T-K7's extension, the plant (fails
   both by name), the restore hash.
4. Every new plant and restore (FP-1 to FP-8, F-G, FP-2): log, name, restore sha256.
5. FP-6, FP-7, FP-8's tests; FP-8's arithmetic and bounds (parser top bound, checked_mul, counted_count, projection).
6. R2-R5 figures against the acceptance lines (r1:279-297; parta:97-105, :159-166); "report the number left out
   beside the check on every run" and what -21's set report must print.
7. The test-step time against 1,207 s before R6 (R0, the new tests' times); clippy and doc logs.
8. The departure: bots lib.rs touched by FP-2; does the lag-switch A proof cover the non-forged lines?
9. Findings, the table claim / proof / gap, verdict.

## Notes (one line per finding as I go)
- Read: r1 ruling :240-423, parta whole, panel brief, history :7544-7593, build record :268-591, the diff (7 files,
  +638 -88). FP-2's own change isolated as the diff of the committed snapshots f0da659 (F-G's, 184,651 B) to 096a49b
  (FP-2's, 190,245 B; the main folder's wip patch sha256 e33d14f0..., 190,245 B by `wc -c`: the record's "190,233 B"
  and "184,639 B" read 12 B low, no effect).
- World step order (sim world.rs:3-15, :2151-2199): inputs (0), movers (1), commands (2), shots (4: deaths at
  :1698-1702), respawns (7), records (8). The lab's run scripts no spawn or attribute (`room.script` only in tests:
  logs.rs:1058, scenario.rs:3280, precheck). So "health 0 after tick t-1" (scenario.rs:1470-1473) is the soldier's
  state at tick t's command phase: the lab's flag is his "as that command ran". T-K7's flag (lib.rs `soldiers`: dead
  bit tied to health 0) is the same, plus its own test spawn (`spawn != Some(k)`), which the lab never has.
- Callers of `kind_7_left_out`: bots lib.rs:180 (def), :2082 (T-K7, cfg(test)), lab forged.rs:106 (Attribution, built
  only for a forged non-twin run: scenario.rs:1368-1369). The scenario.rs wiring sits inside `if let Some(a) =
  attribution` (:1465-1474). So no non-forged line reaches FP-2's code.
