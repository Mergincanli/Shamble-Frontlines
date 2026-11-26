# test-engineer: 4b test-list read (what a planted defect slips past)

Task: before 4b ("Content and the world") is built, read its test list; per step 1-9 and the links check, say which
plausible defect the listed tests miss and the smallest test/plant that closes it. Not a review round (advisor ruling).
Read-only; notes only here. Started 2026-11-26.

Memory: no local agent memory exists (.claude/agent-memory-local/test-engineer absent in main checkout and worktree).

## Plan
1. reread-sheet.md (4b steps, B.1 grammar cut, Mustafa's answers of today).
2. plan.md s5 (5.1-5.3), s9 row 4b, s11 (testlines --check), s17.
3. engine-architect-cs4.md 1.12 (+1.3, 1.4); rust-reviewer-cs4.md blocks 2 and 5.
4. Mark texts as needed: build-plan report Part 2/3; ADR 0005, 0004 PM rows.
5. 3a/3b/4a reports (Evidence, Reviews, Not verified); plants-4a.py at c17a054.
6. Today's code where tests hook in (crates, packages, CI, tools/gate.mjs pieces).
7. Per step: gap -> closing test/plant, where, lines. Rank must/should/optional.
8. Handback (<~120 lines).

## Source notes (one line per source as read)

## Findings (one line per point as found)

- reread-sheet.md: steps 0-9 + LINKS-CHECK; B.1 cut grammar (objects/arrays/strings/numbers; refused by name: null,
  true, false, dup keys, BOM, trailing comma, comments, NaN/Infinity, leading zero, depth>32, string bytes >0x7E or
  <0x20, escapes other than \" \\ \n); numbers keep full RFC 8259 grammar; 3b green before any content/ file (but
  content/source/); B.2 MapBeyondAim (x,z within +-327.67, y 0..655.35, constants beside AimPoint; plant 1 cm too wide).
- plan.md s3.5 (this read), s5 (steps, ladder: never moved = built-file check, collision world + PM-0004-32, 7 stages
  + 3b proofs, Testlines check), 5.3 (4 refusal classes, 5 plants + 2 controls, native baseline, target-cpu=native
  plant), s9 row 4b, s11d (testlines --check, 3 plants), s14 (no CI edit; tools' --check run by plain tests), s17.
- engine-architect-cs4.md 1.3 (7 stages; stage 2 still says "escapes per RFC 8259, lone surrogate refused": superseded
  by B.1), 1.4 (writer = JSON.stringify(sortedKeys, null, 2)+"\n", -0 written "-0"), 1.12 (mark table; PM-0005-4
  classes list "bad UTF-8", "__proto__", 10,000-deep, 50 MB files; written before the B.1 cut).
