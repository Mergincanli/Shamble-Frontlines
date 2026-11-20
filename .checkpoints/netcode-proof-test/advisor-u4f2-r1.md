# Advisor round ruling: U4f2's review round 1 (2026-11-20), from main's advisor.md

Read only. Role: main's `.claude/agents/advisor.md` (`git show origin/main:.claude/agents/advisor.md`, read whole).
Diff: wip/u4f2-build.patch (base a289def) and the builder's worktree
(.claude/worktrees/agent-aa62d31b6fa5b9807, read by absolute path). Nothing run but git/grep/sed/wc/sha256sum.

## Plan
1. Confirm the diff (patch hash) and read the code at each High and Medium (world.rs dead return and revive; forged.rs
   kind-7 rule; bots forge_step, FIRE_REST, RELOAD_ITEMS, forge_input's twin arm; scenario.rs lag client, f16 pair, f20;
   logs.rs lagswitch excess; marks.json -21 checks; SERIES.md:158).
2. F-B (NC-U4F2-1 = TE-U4F2-1, High): the cause in code; route (a) inside the plan or his under TEST-FIRST; other
   routes the rules allow; the proof owed.
3. Every other finding: F-A, F-E, FIRE_REST, the shared pair, F-C, the pre-F-D hand runs, the 1,207 s margin, every Low.
4. The builder's departures (the five the panel brief names, `HOLD_STEPS` included).
5. The fix pass: FP-n with acceptance checks and plants; size against 3,000 (2,077 now); who builds; runs owed against
   the disk (about 10 GB free, 4 GiB floor, about 6.3 GB per check.mjs run).
6. The round decision.
7. What goes to Mustafa, in which message; the paste-ready question if any; "Choices taken without him".
8. Tables: ruling per finding, FP-n, round decision, the Mustafa line.
Each block is added below as it finishes.
