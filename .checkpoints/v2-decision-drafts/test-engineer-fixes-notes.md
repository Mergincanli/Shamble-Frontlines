# test-engineer: pass-mark fix round (plan step 9) notes
Started 2026-11-01 13:57 Amsterdam. Brief: prompts/fix-passmarks-test-engineer.txt. Deadline 15:30.
Output files: test-engineer-passmarks-000N-fixes.md (N = 1, 2, 3, 4, 6). No memory dir found (.claude/agent-memory absent).

## Fix list (from brief)
- 0001 F1: PM-0001-4, PM-0001-39 "whole tick timed directly" -> glossary wording
- 0006 f5: PM-0006-17 inline "proposed"; f6: PM-0006-4,-6,-7,-8,-12,-22 run counts/method settings "proposed"
- 0004 TE69: r3 line 1123 -> 1123–1124
- 0002 F-7: PM-0002-26 vs PM-0004-68..70 align; F-13 OQ2/3/6/9 line numbers; F-14 erasableSyntaxOnly enum unverified
- 0003 TE44: PM-0003-4 "CPU"; X7: PM-0003-12 post passes vs 800 draw-call Low cap

## Progress
- 14:05 read brief, GLOSSARY:107-123 (server tick time; "whole tick" retired exact), verifier 0001 F1, verifier 0006 findings 5, 6.
  ADR line numbers moved since verification (0001: PM-0001-4 now l.1106, -39 l.1143). Research line numbers cited at 9e7db28.
  0006 f5 reason: PM-0001-2's stress-scene load (700 shots/s, 300 projectiles) is not R2's counts (r2:1759, 1761: 1.5 shots/player/s = 72/s at 48; 32 projectiles); R6-COST-REC1 mark r6:716-717 = 3.16 GB at R2's counts.
  0006 f6: one bullet under "How to read the table" with a reason per count (ledger TE60, TE76-77, TE81, TE84, TE98, TE170); ledger is a checkpoint file (deleted at PR end) so reasons go inline.
