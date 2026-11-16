Task: night 2026-11-16 item 7 (npt-u4f-brief), test-engineer check of rev 1 of U4f's build brief (tests, negative cases, bites, runs, counts, carries).

(No Edit tool in this agent: the notes file is rewritten whole with Write at each save.)

## Plan (blocks, in order)
- B1-B8: DONE (plan, draft, drafter's notes, scout, build plan, carries, marks.json, README, ADR, plan.md, advisor rulings).
- B9: code and table spot checks: SERIES.md DONE; room input.rs DONE; client lib.rs DONE; lab scenario.rs r8/r9 DONE; c-u4r-runs.mjs :125-219, :318-332 DONE; c-u4r-r2.mjs :1-95 DONE; l1 profiles :160-204 DONE; bots FAR_CM / greybox HALF_MAP DONE; room tests thread grep DONE (no parallel-thread bite test in room/client today).
- B10: cross-check and write findings (TE-n) and summary. In progress.

## Notes (one line per block; findings collected in B10)
- B1-B8 done (facts folded into the findings below).
- B9 done. Key facts: input.rs:80-125 order today seq (Repeat) -> press -> late (incl. a duplicate stamp) -> Full; client lib.rs:443 truncates unacked to REDUNDANCY_CAP (each Input re-sends up to 8); lab scenario.rs:2172-2175 r9 is carry 76's kicked-session test (U4r's); c-u4r-runs.mjs:153 56 MB a run, :132 FLOOR 4 GiB, :176 judge fit; c-u4r-r2.mjs:8-9 r2 runs 10 s warm-up + 20 s measured, :80 -20 P3 B shown with the brawl (MARKS_17), :82-85 -21 B stand-ins presses/clamp/steady/steady at P0/P1, :88-91 r2 ran its bites short; l1 P4 200 +-40 ms (lead 9); bots/src/lib.rs:76 FAR_CM 32,000 vs greybox.rs:62 HALF_MAP 12,800; SERIES.md :21, :33-34, :61, :93, :101, :128, :144-145, :149, :155-159, :170, :240, :322, :342, :344.
- Next: B10 (write the check below). 
