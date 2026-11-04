# art-director review: kit-v3-2 (A2) final diff, TOY-BOX and LOOK-LINES (T1-T6)

## Plan
1. Diff of CLAUDE.md, art-director.md, graphics-engineer.md vs plan-rev2 §4.2-§4.4 texts.
2. Read docs/LOOK_V2.md; check each line only points at it, keeps "never decide the look", says nothing beyond it.
3. Condition 9 ("readable" in art-director opening), condition 8 (upbeat line).
4. Legacy Living Comic references still point at ART_DIRECTION.
5. Grep remaining toy-box / joyful / upbeat / "look is not decided" in CLAUDE.md, .claude/.

## Notes
- Step 1 done: diff matches plan texts T1-T6 word for word (rewraps only).
- Step 2: LOOK_V2 read (78 lines; §1 DECIDED cel-shaded comic + bold ink; §3 adults; §8 [OPEN] tree style).
- Step 3: condition 9 met (art-director.md:10 "a readable battlefield"); condition 8 met (art-director.md:28, "upbeat" gone).
- Step 4/5: grep CLAUDE.md + .claude/: 0 hits toy-box/joyful/upbeat/"look is not decided"; "never decide/choose the look"
  kept at CLAUDE.md:6, :24, art-director.md:19, graphics-engineer.md:20. No line describes the v2 look beyond LOOK_V2
  (graphics-engineer.md:20 "comic look" = LOOK_V2.md:9 [DECIDED] cel-shaded comic; "[OPEN] items stay open" = LOOK_V2.md:68).
  Changed lines all <=120 cols (CLAUDE.md:7 123 and art-director.md:30 122 are unchanged main lines).

## Findings
- AD-1 minor, art-director.md:11-13: the legacy pointer "(below)" names no file; the authorities line it points at is not
  scoped to legacy and lists GAME_DESIGN §2-3, whose text is "upbeat" (GAME_DESIGN.md:28) and "joyful ... toy-box style"
  (GAME_DESIGN.md:50-51). A v2 review can still take the toy-box tone as an authority, against LOOK_V2.md:27.
  Fix: line 11 "Living Comic for the legacy build (docs/ART_DIRECTION.md)"; line 12 "Your authorities for the legacy build:".
- AD-2 nit, docs/STATUS.md:375-376: the later docs pass lists only GAME_DESIGN.md:17; plan-rev2:602 names :14,17,28,50-51.
  Fix: list GAME_DESIGN.md:14,17,28,50-51 (and README.md:4).

## Verdict: PASS WITH NOTES (APPROVE with one minor and one nit)
