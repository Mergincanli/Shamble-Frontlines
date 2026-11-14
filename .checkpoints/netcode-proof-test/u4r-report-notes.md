# U4r report: drafting notes (agent, 2026-11-14)

## Plan
Deliverable: u4r-report.md, in u4d2-report.md's form, content order per advisor-u4r-r2-full.md:15 (step 9, M8).
Sections:
1. Header (merge commit {{MERGE_SHA}}, brief, base)
2. Who built it (copies 1-6, build rulings 1-4, rounds 1-2)
3. What the rulings and both rounds settled (r1: six seats, 50 findings; r2: five seats, 1 Medium, 12 Low)
4. The two deviations accepted
5. Checks on the merged tree (M1-M4 given; {{M5}} {{M6}} {{M7}})
6. Carries met, by number
7. Public items (EA-U4R-1's line, contract and precheck subcommands)
8. Timing with labels (u4r-build.md:1916-1923; INF-R2-2 det-u4 base; merge-step figure {{M5}})
9. Size (8,362 vs about 3,000; SIZE-U4R (a))
10. Follow-ups with homes (r1 FP-9 item 4; r2 ruling)
11. Open security findings (SEC-U4R-R2-1, -2)
12. Judged runs pending (steps 11, 12, 13-16, 17: P3, B1, P2, P0)
13. Decisions needed / For Mustafa

## Progress
- Sources read: r2 ruling whole; u4d2-report whole; u4r-build.md copies' starts/states, copy 5 (1173-1587), copy 6
  (1588-2111); build rulings 1-4 decisions; brief check verdict; r1 review headings + r1 ruling FP list, FP-9, For
  Mustafa; r2 review headings; carries §U4r entry list; history 396-401, 2039-2086.
- Verified read-only: patch sha 21ca46be, 26 files +8362 -106 (new 6,906, existing 1,456); spikes/ = 2fbec77's at
  HEAD; origin/main 4460ef5; patch unchanged since 450af1a; check.mjs exports 67->69 (cargoConfigDirs, run new);
  EA-U4R-1 lines hold in merged tree (scenario.rs:341, :742; precheck.rs:285); main.rs subcommands :225-285.
- No separate copy-5 "section 6 return list" file found; used copy 5's blocks in u4r-build.md (1173-1587).
- Not in builder notes by number: carries 20 and 69 (mark as not verified).
- DONE: all 13 sections written (u4r-report.md, 205 lines); cites re-checked by sed against the sources; placeholders
  {{MERGE_SHA}} (x2), {{M5}}, {{M6}}, {{M7}} on their own lines.
- Disagreement to state: det-u4 rise cause (builder names FP-1's link check, u4r-build.md:1922; ruling says
  unexplained by FP-1's constant-time state() read, advisor-u4r-r2-full.md:48).
