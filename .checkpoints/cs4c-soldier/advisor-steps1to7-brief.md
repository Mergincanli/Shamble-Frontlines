# advisor agent: 4c-2's plan, steps 1 to 7 (the soldier), for APPROVE or REVISE

DRAFT, written 2026-11-28 while 4c-1 closes; sent only after your verdict on 4c-1's final head (one review at a
time). Asked by Builder V6 (the main session). The worktree is `/home/user/Shamble-Frontlines/.gate/cs4b-2` (local
branch `cs/4c-soldier`, pushed as `claude/ecstatic-wozniak-5cbdzs`). 4c-2 is built on this branch on top of 4c-1's
final head (his SIZE-4C (b), item 6 b); its first commit brings the notes back. Your agent file on main last changed
2026-11-13 (8523d50), before this session started, and this branch does not change it.

**What you approved before, and what was left.** Your re-read (`advisor-reread-4c.md`) approved step 0 and step S
under conditions 1 to 9 and held steps 1 to 7 for its item 10's thirteen points. His answers of 2026-11-28, 12:10
Amsterdam (`.checkpoints/cs4-walk/mustafa-2026-11-28-answers.md`, in the register by step 0) settle most of them.
The plan as it stands: `reread-sheet.md`, sections E (the reads), F2 (the recount), G (evidence, reviewers, his
checks, risks) and **I (revision 4, his answers applied; where I and the text above differ, I holds)**, with I4b (the
Testlines read's result) and I6 (what you are asked).

**The thirteen points now** (section H's table is revision 3; this is the state today):
1. BODY-SHAPE: his (3a). 2. The body prototype: rust-reviewer's blocks 5 to 10 and 12 (`rust-reviewer-read.md`),
   on his sandbox's own rules (game-designer's blocks 6 to 8, `game-designer-read.md`). 3. SWEEP-ROUTE: his DEPART-4C
   (a), on his terms (the collision library checks our sweeps for every shape the game uses, the ramp included,
   before the movement step counts as done; the report shows PM-0004-20's and PM-0004-35's old and new wording side by
   side, the new asking no less about the players' file; PM-0004-60 at 2,000,000 per CI run now, the mark open in
   STATUS until the nightly 10 million; PM-0004-20's 48-soldier run open in STATUS). 4. GROUND: engine-architect's
   ruling (the bounds' bottom is solid ground). 5. FACING-SPAWN (a), with its reviewers. 6. The slope: his MOVE-RULES
   row 9 is open, not decided; nothing is built that depends on roofs being walkable or not (section I2, applied at
   14:03 to the walk route and step 1's tests 7 and 8). 7. The record rule: applied (section E). 8. NO-SOURCE (b): the
   sandbox's numbers, each marked "not from his Export" (I2). 9. `Rules::take`'s refusals and the jump tests:
   block 11's 11.4 and 11.7. 10. The dependency hold: security-auditor APPROVE on `deps-4c-v2.diff` (F1, F2 and the
   stricter libm scan rule in the commit that applies it, `dependency-check.md`). 11. CI's Chromium time: his
   CHROME-TIME (the CI Builder raises the limit after Tuesday 1 December, 20:00; never cut or thin a test). 12. The
   recount: about 8,550 by the designs' count, his stop line 18,700 for 4c-2, two thirds about 12,470, the early look
   at about 15,360 to 19,110 written lines (the card's "Key numbers"; the rung is yours, I1). 13. Section G's steps 1
   to 7 with their evidence, reviewers and his 35-minute checklist (G and I).

**Step 1's design in one place:** rust-reviewer's block 11 with block 12 (the tick in the sandbox's order, the
60-byte row and 2,902 shared bytes, every rule with its numbers, `Rules::take`'s 28 fields, exp and sincos with the
dependency batch, the symbols by file, 25 tests each with its plant, the pins re-made in the same commit, about
1,510 / 1,840 lines). The other steps: sections E and G as revised in I.

**Asked of you:**
- **Q1.** APPROVE or REVISE steps 1 to 7 as revised in section I, with step 1 as block 11 with block 12.
- **Q2. HEADROOM-FEET's default while he has not answered** (his question 13). Block 11 builds the headroom rule as a
  switch with both ways tested (test 15) and its default at (a), "never a wall at or below his feet", which differs
  from his sandbox. This chat proposes **(b), his sandbox's rule, until he answers** (his MOVE-RULES (a): "all rows,
  as the game's starting rules"; FALL-STEP (c): "The game does what my sandbox does"), switched to (a) only on his
  word. Your ruling, or ESCALATE.
- **Q3.** The rung (section I1): the far-corner map moves to 4d at 13,150 written lines (the plan's) or at about
  14,960 (the plan's rule on his new line of 18,700)? He is told before it moves, either way.
- **Q4.** The order inside 4c-2 (your item 7's six conditions, now that SIZE-4C is answered and 4c-2 sits on top of
  4c-1's final head).
- **Q5.** Information to him, not questions (game-designer's block 8): the swept wall pass against his sandbox's
  push-out (the same in the open and on every face; at corners up to 0.069 m at his walk, 4 micrometres at his sprint;
  one tick at the roofs' ridge, the same result); and the ridge flag left as a STATUS follow-up (rust-reviewer's
  block 12). Agree, or say what must go to him as a question.

**Rules for you.** Read-only: no source, doc or note edits, and no commit; your verdict file goes to
`/home/user/Shamble-Frontlines/.gate/cs4b-2/.checkpoints/cs4c-soldier/advisor-steps1to7.md` once 4c-2's notes are
back (until then `/home/user/Shamble-Frontlines/.gate/cs4b-2/.gate/advisor-steps1to7.md`; the main session moves
it); git read-only, each git command alone in its shell call with nothing in front of it, one git per call; the
repository is public. Your final answer: the verdict per question, with conditions, and what you did not check.
