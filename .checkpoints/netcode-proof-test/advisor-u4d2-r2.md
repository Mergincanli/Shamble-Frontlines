# The advisor's ruling on U4d2's round 2 (2026-11-13, from advisor-u4d2-r2-brief.md)

Seat: the advisor agent, run by the main session's workflow, working from main's `.claude/agents/advisor.md`
(read at origin/main). It rules; it edits no code and runs nothing that builds or touches a copy.

## Plan
1. Read the brief whole (done), main's advisor.md (done), my round-1 ruling whole (done: Blocks 1 to 9, T1 to T5,
   "Before the merge").
2. Read the five round-2 seats' notes whole (security, rust, infra, test, arch), then the builder's "Fix pass, round 1"
   section (claims only), the packet's head and outline.
3. Spot-check the load-bearing claims by reading, git and sha256 only: the copies' hashes against fx-hashes-end.sha256
   and r2-hashes-start.sha256; the patch's sha256 and its commit; FX-7's timeout code and run() wiring (INF-1's form);
   the lane lock's place against M9's clean (INF2-4); the TP10 removal (check.mjs's sha256, the labelled block); the
   counted logs' totals (fx-check1/2, r2-te-check); the leftover folder.
4. Rule, one block per item (1 to 6 of the brief), each written to this file as it is finished.
5. The amended texts (T3, T5 whole, in code blocks), then the two lists (before the merge; for Mustafa), then the
   summary for the caller.

## Notes (one line per check, as done)
- Read whole: the brief (73 lines); advisor.md at origin/main; my round-1 ruling (730 lines: Blocks 1-9, T1-T5,
  "Before the merge", the verdict table); the five seats' notes (infra 190, security 201, rust 141, arch 127, test 254
  lines); the builder's "Fix pass, round 1" (u4d2-build.md:258-556, its claims).
