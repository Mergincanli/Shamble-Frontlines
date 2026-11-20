# Advisor round ruling: U4f-F1's review round 1 (2026-11-20), from main's advisor.md

Read only. Diff: wip/u4f-f1-build.patch (sha256 37d87a10bed52b81..., base cb7e374) and the builder's worktree
(.claude/worktrees/agent-ad5f72ed146da5f93). The four seats' files read whole; the panel brief, plan rev 1.1, my plan
review (advisor-u4f-f1-plan.md), the build record, U4f2's brief rev 1.1 and my check of it, TEST-FIRST
(history.md:2234-2282). Nothing run but git/grep/sed/wc/sha256sum.

## Plan
1. Confirm the diff (patch hash, worktree hash) and read the code at each finding's lines (seam.rs, world.rs, room.rs,
   input.rs, the tests).
2. TE-F1-1 = NC-F1-1: the room's lost by construction; NC-F1-1's in-lane form (the sim reports the edges it ran);
   TEST-FIRST's reach (pass marks, judge, marks.json); rule fix vs wording.
3. NC-F1-2 and SA-F1-1: the first-command reach, honest rejoin; carry to U4f2 widened or fix in F1; what it adds to
   U4f2's lane and size; U4F2-SIZE.
4. Every Low (RR-F1-1..4, TE-F1-2..6, NC-F1-3, -4, SA-F1-2, -3): fix, carry or no action.
5. The fix pass: list, size against 300, round decision.
6. The -20 re-run's expected figures (my plan review :119-140): restate if moved.
7. What goes to Mustafa, which message.
8. Tables: ruling per finding, FP-n with acceptance checks, round decision, expected figures, the Mustafa line.
