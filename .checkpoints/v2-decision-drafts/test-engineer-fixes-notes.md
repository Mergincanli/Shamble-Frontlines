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
- 13:59 read brief, GLOSSARY:107-123 (server tick time; "whole tick" retired exact), verifier 0001 F1, verifier 0006 findings 5, 6.
  ADR line numbers moved since verification (0001: PM-0001-4 now l.1106, -39 l.1143). Research line numbers cited at 9e7db28.
  0006 f5 reason: PM-0001-2's stress-scene load (700 shots/s, 300 projectiles) is not R2's counts (r2:1759, 1761: 1.5 shots/player/s = 72/s at 48; 32 projectiles); R6-COST-REC1 mark r6:716-717 = 3.16 GB at R2's counts.
  0006 f6: one bullet under "How to read the table" with a reason per count (ledger TE60, TE76-77, TE81, TE84, TE98, TE170); ledger is a checkpoint file (deleted at PR end) so reasons go inline.
- 14:02 gathered all inputs. Other authors' fixes for 0001, 0004, 0006 already committed (03df0ae, 7da03b1, 09803a9); my BEFOREs are copied from HEAD (915d3ef). apply_fixes.py now takes an optional FILE: line (not needed: one file per draft).
  0004 TE69: r3:1123 p95, 1124 median -> "frame rows (r3 lines 1123–1124)".
  0003 TE44: ADR 0003 part 8 (l.331) defines "CPU frame time" from performance.now(); add "CPU" in PM-0003-4.
  0003 X7: r3:1121 table head "Budget (all passes of one frame)"; R3-MEASURE-F3 r3:617-618 drawCalls "The number of draw calls of the current frame."; R3-LOOK-F13 r3:397-402 bloom 12 passes (count inferred). Margins 800-786=14, 1,100-1,058=42. Row says post passes count (source's definition), reports their share (post off, subtracted), bloom only "if the look keeps it" (look not decided).
  0002 F-13: r1 OQ2 2724-2727, OQ3 2728-2731, OQ6 2737-2740, OQ9 2747-2752 (read r1 today).
  0002 F-14: R5-TSLINT-F12 r5:221-224 (construct list unverified); verifier S-18: tsconfig page example is a parameter property. Bite becomes a parameter property; enum planted too, result recorded, not required. PM-0002-42 carries R5-TSLINT-REC1's own enum (r5:1259-1261): not in brief, reported only.
  0002 F-7: align PM-0002-26 to ADR 0004's PM-0004-68..70 (ADR 0004 part 5 owns S18 and PM-0002-26 already said ADR 0004's stand). Its own bite "timer re-armed, no accumulator" = PM-0004-68's bite; its hidden-gap bite moves to PM-0004-70 (which had none) so both read the same: FIX in the 0004 file.
- 14:07 fix files written by script (scratchpad make_fixes.py; BEFOREs copied from the files, whole lines):
  0001: F1 x2 (PM-0001-4, -39). 0002: F-13 x3 (PM-1, -10, -21), F-14 (PM-17), F-7 + F-13 OQ2 (PM-26). 0003: TE44 (PM-4), X7 (PM-12).
  0004: TE69 (PM-8), F-7 counterpart (PM-0004-70 gains the hidden-gap bite). 0006: finding 5 (PM-17), finding 6 (new "Counts and settings" bullet after "Bites").
  Checks on scratch copies: apply_fixes.py dry-applied all 13 blocks OK; glossary script exact hits 2 -> 0 (0001), no new in-context hits except one "48" in PM-0006-17's R2 arithmetic (P = 48, as the row already uses); cite_check output unchanged; every PM row still 7 columns; new bullet lines <= 120 chars.
  Not fixed (outside the brief): PM-0002-42 plants an `enum` from R5-TSLINT-REC1's own mark (r5:1259-1261), same unverified premise; ADR 0001 F4's restated R2-BUD-REC2 precision mark names no PM row (PM-0001-17 checks quantised fidelity only).
DONE

# Round 2 (brief prompts/fix2-test-engineer.txt), started 14:10 Amsterdam; deadline 15:45
Output: test-engineer-fixes-2.md (FIX blocks with FILE lines). Round 1 applied in c70c6d2.
To do: F3 two decimals (0001 PM rows with 8.3/16.7/4.2/6.7/13.3/33.3; others); F7 two PM rows in 0004 (S21, S22 from rust-reviewer-fixes-2.md: R2-SYNC-REC6 mark 1 r2:2218; R5-LAYOUT-REC4 r5:1216-1218); F13 PM-0005-32 "4 active effects", PM-0006-13 "8 rooms at N = 48", PM-0006-17 "V = 15"; PM-0002-42 enum note; 0001 R2-BUD-REC2 precision row.
Left to me by rust-reviewer (NOTE F4-F2-PM): PM-0004-68 "MAC: Safari 26" -> "MAC"; 0004 key line ~924 "(arithmetic; the draft and GLOSSARY round the first two)" stale after F3. rust-reviewer edits 0004:873-874 (MAC key) and :1047 in my subsection: do not overlap.
- 14:20 cut off by the spend limit before writing test-engineer-fixes-2.md (nothing written). Found so far: one-decimal tick values in my rows: 0001 key l.1089 + PM-0001-4,-5,-39,-40,-43,-50; 0002 key BX-P l.961 + PM-0002-12,-13,-23,-25,-28,-72; 0004 key l.916-917 + PM-0004-9; 0005 PM-0005-20; 0006 PM-0006-13,-23. Frame-time budgets from R3 (33.3/16.7 ms p95) and display refresh periods are not tick shares: left.
- 19:07 resumed (coordinator: deadline now 20:15; re-copy BEFOREs: 8eb2f67 changed 0004, b375829 changed 0003, 0005). Added scope: rust-reviewer's held F4 and F2-PMtext (rust-reviewer-fixes-2-held-for-test-engineer.md), PM-0004-68 MAC cell, stale key line, PM rows for S21/S22; graphics-engineer's 0003 MAC key, fragments line (~1076, README pointer), checklist step 29 (camera-player cutaway, OQ-0003-15).
- Round 2 written: test-engineer-fixes-2.md, 42 FIX blocks with FILE lines (0001: 9, 0002: 9, 0003: 8, 0004: 7, 0005: 6, 0006: 3), generated by scratchpad make_fixes2.py from the files at c05fc15.
  Content: F3 two decimals (tick shares and periods only; R3 frame budgets and display refresh periods left); F7 PM-0004-73 (S21) and -74 (S22) after PM-0004-61 + intro (S1-S22); F13 PM-0005-32, PM-0006-13, PM-0006-17 ("V = 15 vehicles", R2's symbol and word); PM-0002-42 enum -> parameter property, enum recorded; new PM-0001-57 (R2-BUD-REC2 precision: max error per field vs pre-quantising values; bites 4 cm, 12-bit yaw, 8-bit pitch) + intro; F4 MAC = SPIKE_PLAN's machine in keys 0003/0004/0005, rows "MAC: Safari"; held F2-PMtext taken over; 0003 step 24 README pointer, step 29 camera-player cutaway.
  Checks: all 42 dry-applied on scratch copies (apply_fixes.py, FILE lines); every PM row 7 columns; glossary exact hits 0 everywhere; in-context: 0001 +1 ("48 headless clients" in PM-0001-57, as PM-0001-17), 0005 -1; new prose lines <= 120 chars.
  Caveats for the session: 0005 key MAC block may duplicate an engine-architect fix (critic gave 0005:1932 to the 0005 frame author); step 24 cites README's "fragments as damage" row, which README does not have yet (synthesis adds it; 0003:247-248 already cites it); SPIKE_PLAN:30 still says "MAC, a macOS machine with Safari" (synthesis, F4).
DONE (round 2)

# Round 3 (prompts/fix3-test-engineer.txt), started 19:18 Amsterdam; deadline 20:30; BEFOREs from 57c43c2
Items: PM-0002-3 (Wasm calls per tick, engine-architect's F-2 wording); the note on PM-0002-3 (held block); PM-0002-29 (Wasm size procedure, part 4); PM-0002-31 (decision map part g citation); 0006 PM rows ~882, 884 'the proof test's' -> 'the netcode proof test's'.
- 19:21 Round 3 written: test-engineer-fixes-3.md, 6 FIX blocks + 2 NOTE blocks (scratchpad make_fixes3.py; BEFOREs from `git show 57c43c2:`, each also unique in the working tree, where another author's uncommitted round touches 0006 outside my rows).
  PM-0002-3: engine-architect's reading word for word (one count, glue included, at most 4 / 8; R1-BND-REC1 mark 2 r1:2261-2263 checked, mark 2 sits on 2261-2263); bites: per-soldier accessor, a no-op export called 5 times per tick. The note on PM-0002-3 deleted (moot; heading "Notes on two rows."); engine-architect's held block should not be applied (NOTE). PM-0002-29: part 4's procedure (two inputs, the Advisor sets the first limit; G7 records until then). PM-0002-31: decision map part g -> part 5's text and SPIKE_PLAN item 3. PM-0006-15, -17: "the netcode proof test's" (3 places).
  Checks: dry-applied to 57c43c2 copies and working-tree copies (all 6 ok); PM rows keep 7 columns.
DONE (round 3)

# Round 4 (prompts/fix4-test-engineer.txt), started 19:28; deadline 20:15; BEFOREs from aa85bf0
Task: replace pointers into each draft's own lines (in my pass-mark sections) with stable anchors (part + row/item named); research citations stay.
- Round 4 written: test-engineer-fixes-4.md, 37 FIX blocks + 1 NOTE (scratchpad make_fixes4.py). 36 for ADR 0001 (key's wrap pointer + 35 rows: every "the draft, … (line N)" resolved against netcode-architect-adr0001-draft.md, whose lines the pointers match, and replaced by part + named row/bullet/item; no ADR 0001 line number kept), BEFOREs from aa85bf0 (= working tree). 1 for ADR 0003 step 29 (coordinator's addition: GW5's ground grid, red arena border, world text labels), BEFORE from disk (graphics-engineer's GW5 block applied, uncommitted).
  Other drafts 0002-0006: no self-pointers by line number in my sections (searched every line citation not tied to r1-r6/GLOSSARY/another file).
  Checks: all 37 dry-applied (0001 at aa85bf0 and working tree; 0003 on disk); 0 "the draft … line N" left in 0001's section; PM rows 7 columns; glossary exact hits 0.
DONE (round 4)
