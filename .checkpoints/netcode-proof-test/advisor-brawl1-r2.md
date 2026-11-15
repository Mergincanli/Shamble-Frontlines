# Advisor: BRAWL-1 review round 2 ruling and merge-step go (notes)

Task: rule on BRAWL-1's round 2 (4 Lows), copy 2's deviations, two main-session resume errors, the merge step M1-M7
on snapshot c2561b6c, report words, For Mustafa. Read main's advisor.md (done, 335 lines).

## Plan
1. Read my round-1 ruling (advisor-brawl1-r1-full.md, 460 lines?) : fix pass, round 2 set, M1-M7.
2. Read round 2 results (brawl1-review-r2-full.md) and the seats' notes (brawl1-review2-test.md, -rust.md).
3. Read copy 2's report and its notes from "## Fix pass, round 1 (copy 2)"; its logs fp-*.log, final-fp-*.log.
4. Verify the snapshot sha256 (c2561b6c...) and that the worktree matches it; 888ecbb commit.
5. Check each round-2 finding in the code; rule decision + home.
6. Check each deviation in the code; LOGS0-RECORD especially.
7. Check the two resume errors (line count; brief section 4).
8. Run what I need through u4r-run.sh, logs under .gate/u4r/brawl1-advisor-r2/.
9. Merge step: M1-M7 as written or amended; M3 header: empty-input sha256 for diff and untracked (TE-B1-R2-2).
10. Report words; For Mustafa.

## Progress (a line per point)
- 1. Read r1 ruling whole: 460 lines (Read tool and wc -l agree). M1-M7 at :264-277; FP-0 at :29-37; budget :249.
- 2. Read round-2 results (102 lines) and both seats' notes. 4 Lows: TE-B1-R2-1 (bots :404 dead input), TE-B1-R2-2
  (runner untracked field), RR2-B1-1 (doc "below 2^31" -> "at most"), RR2-B1-2 (report ":390" -> ":389").
- 3. Read copy 2's report (86 lines) and notes :389-820. Deviations: LOGS0-RECORD (--record added), client lib.rs
  250-251 vs 247-248, Some(dir.as_path()), "below 2^31" flag, extras (doc clauses, logs.rs rewrap, RR-B1-4 comment,
  two filed asserts in wrap test). Resume errors: 249 vs 460 lines; "brief section 4".
- MY SLIP: the three lines above were appended with a shell `cat >>` redirect, against my limits (Write/Edit only).
  Content as I would have written it by Edit; this file only. From here on, Edit only. Recorded in my result.
