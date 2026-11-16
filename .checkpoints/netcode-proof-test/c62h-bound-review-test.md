# C62H-BOUND build review, round 1: test-engineer (effort xhigh)

Read only. Diff 044fab188419e8859dbca699d851563c44c078c9..42b964618eed741f65ac04c1e5a06da0a82524d0 in
/home/user/Shamble-Frontlines/.claude/worktrees/c62h-bound (branch c62h-bound-build). No cargo, no build, no test run.
(No Edit tool in this agent: this file is rewritten whole with Write after each block.)

## Plan
1. Read brief (sha 7936074c... checked), texts v3, freeze, design, rulings (advisor full, r2, brief), step1-4 returns,
   build notes.
2. 13:35 read: marks.json (the two checks and the bite, lines in built file), figures.rs and engine.rs ranges,
   fixtures.rs and fixture files; sha before/after; diff --stat at close.
3. SERIES.md and README.md against texts v3.
4. Plants CB-P1..CB-P18: each log under .gate/u4r/c62h-bound/plant-CB-*.log, check each bit as section 4 says.
5. Test counts 538/616 -> 552/632 per step.
6. B-2: changed fixture answers; no other assertion moved; launch 3's before-readings; fixtures.rs:390-391 doc line;
   README "E2" gloss.
7. Main session's four scripts and their dry-run logs vs brief sections 2, 7, 8.3-8.4: can each fail?
8. Section 8.4 expected lines: are they checked by set_check?
9. Findings, points, read_line; return.

## Progress
- brief sha256 7936074c15c5eb49ed3d8e0e130eaf0d75ce6f30067f650072caa764c3414857 (matches). texts v3 3f21c9b7...,
  freeze 9cb8be17... (match).
- Lane: 4 commits 54046d4, b423a7d, b01d3ac, 42b9646; numstat (results excluded) +904 -63 = 967. Worktree clean at
  42b9646.
- marks.json: sha256 at 044fab1 9fb5be578b27d4c378a890d297c5ba636af0ac239bb6d50eae157ed44884a3c5; at 42b9646 (and the
  worktree file) c1935a5b996fb0aaff82b0b3c1c604255e1534f6ca7e8d2e55a9f472cd3b22a9. Lines read in the built file:
  PM-0001-17 at :3845-4047 (whole object); the two checks :3983-3993 (snap_lost_unexplained_count) and :3994-4004
  (snap_whole_despite_link_loss_count); the bite :4035-4038 (link-lost-overstated). My own python3 -I read (scratchpad
  mcheck.py): check-objects block sha cea24dd8... and bite-object d5945f92... = freeze; each appears once; lines
  3983-4004 == check-objects exactly; 4035-4038 == bite-object exactly; json(built) == json(base) + exactly those two
  checks after 'L1 P0 loses no group' and that bite after whole-as-lost. PASS.
- SERIES.md 6898e119... (= expected): the four frozen rows each a whole line once (:291, :292, :322, :378); headers
  '(38)' :258, '(19)' :372. README.md f3c3c804...: word diff adds only the sentence's words, removes none; sentence
  (ws collapsed) once; refilled lines 67-82 all <= 120 (lines over 120: 14 before, 14 after: table lines, unchanged).
- figures.rs diff read (+172: Bound :538-563, LINK_GAPS :566-575, bound :580-674, the two figures :676-698, FIGURES
  entries :817-828). Analysis in progress.

## Findings (a line after each)
