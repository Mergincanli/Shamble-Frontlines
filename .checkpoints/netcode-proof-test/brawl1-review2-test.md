# BRAWL-1 review round 2: test-engineer seat (notes)

Snapshot under review: wip/brawl1.patch sha256 c2561b6ca54c8cde994fcdad5954c63d332df73f2db038c12aba3d71a0daa49f (checked by sha256sum at start).
Packet: brawl1-r2-packet.md sha256 e50a22f70ae876f962510693e03c414700200a7bf3a37a13ad31bf5fb5dfa8b5 (checked).
Round 1 snapshot: ee2973aa...; round 1 read copy .gate/brawl1-snap1; round 2 read copy .claude/worktrees/brawl1.
Logs (if I run anything): .gate/u4r/brawl1-review2-test/ via u4r-run.sh -x.

## Plan
1. Confirm round-2 read copy's `git diff HEAD | sha256sum` = c2561b6c.
2. Diff ee2973aa -> c2561b6c (interdiff of the two patches, or snap1 vs worktree).
3. Per ruling "Who reads what" (test-engineer):
   a. TE-B1-1 48-record assert + P1 record (bite rule; door-entry bar if added).
   b. FP-3 figure test + P2; bots area test.
   c. FP-2 unit-test edits vs advisor's re-computed sequence (commands 12, 15, 16; blocked loop; const assert).
   d. FP-4 client cases + P3, P4 (TE-B1-4, NA-B1-1, RR-B1-5, EA-B1-3 (b)).
   e. Dead-bit tie + dead count (TE-B1-6, NA-B1-3, RR-B1-3).
   f. RR-B1-4 two fp3 cases, ADV-PT-L2.
   g. FP-8 counts, full test equal figures, pins.
4. FP-0 runner diffs (TE-B1-7).
5. Report words FP-9 (TE-B1-5, TE-B1-8).
6. Run (maybe) the new tests through u4r-run.sh -x to reproduce.
7. Verdict + StructuredOutput.

## Round-1 findings state

## New findings

## Progress log
