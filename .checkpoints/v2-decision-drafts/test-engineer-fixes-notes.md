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
- 14:20 gathered all inputs. Other authors' fixes for 0001, 0004, 0006 already committed (03df0ae, 7da03b1, 09803a9); my BEFOREs are copied from HEAD (915d3ef). apply_fixes.py now takes an optional FILE: line (not needed: one file per draft).
  0004 TE69: r3:1123 p95, 1124 median -> "frame rows (r3 lines 1123–1124)".
  0003 TE44: ADR 0003 part 8 (l.331) defines "CPU frame time" from performance.now(); add "CPU" in PM-0003-4.
  0003 X7: r3:1121 table head "Budget (all passes of one frame)"; R3-MEASURE-F3 r3:617-618 drawCalls "The number of draw calls of the current frame."; R3-LOOK-F13 r3:397-402 bloom 12 passes (count inferred). Margins 800-786=14, 1,100-1,058=42. Row says post passes count (source's definition), reports their share (post off, subtracted), bloom only "if the look keeps it" (look not decided).
  0002 F-13: r1 OQ2 2724-2727, OQ3 2728-2731, OQ6 2737-2740, OQ9 2747-2752 (read r1 today).
  0002 F-14: R5-TSLINT-F12 r5:221-224 (construct list unverified); verifier S-18: tsconfig page example is a parameter property. Bite becomes a parameter property; enum planted too, result recorded, not required. PM-0002-42 carries R5-TSLINT-REC1's own enum (r5:1259-1261): not in brief, reported only.
  0002 F-7: align PM-0002-26 to ADR 0004's PM-0004-68..70 (ADR 0004 part 5 owns S18 and PM-0002-26 already said ADR 0004's stand). Its own bite "timer re-armed, no accumulator" = PM-0004-68's bite; its hidden-gap bite moves to PM-0004-70 (which had none) so both read the same: FIX in the 0004 file.
