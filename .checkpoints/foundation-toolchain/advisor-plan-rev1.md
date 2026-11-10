Task: advisor plan review, foundation step 2.2 (toolchain), plan revision 1 at f535752.

Plan (blocks, in order):
B1 verify plan at f535752 == working copy
B2 read plan.md (whole)
B3 read Mustafa's words (session file, 828a3e0 words + answers, STATUS 680-710)
B4 read sketch (828a3e0 plan Part 2/5/App B) + advisor-plan-rev2 (Q-A3, G0)
B5 read research-network.md; spot-check claims vs plan use
B6 check repo rules (CLAUDE.md, advisor.md, V2_PHASE0_BRIEF Don't list) vs plan 1.3
B7 check Part 5 edit list vs guard (protectedRel, mergeAllowed), run them
B8 review workflow draft Part 4.2 (permissions, pins, cut, probe/control, failure modes); PR #40 files at 411eeee
B9 rulings Q2.2-a, Q2.2-c, three changes, mutation commits, globs to 2.5, FDN-NOCODE
B10 verdict written

Notes:
- B1 done: git show f535752:plan.md == working copy (diff empty). Next B2.
- B2 done: plan read (421 lines). Session words file read (untracked, not in f535752; quotes in plan 1.2 match it).
- F? (K2 split): K2 bundles (i) heading scope to legacy/ (covered: "the lines that scope the legacy constraints") with (ii) "Phase 0 writes no v2 code" + exception at CLAUDE.md:51-53, which is the same kind of edit as K3/K4 (FDN-NOCODE). Plan marks K2 wholly "covered" -> inconsistent; (ii) belongs in FDN-NOCODE. Check CLAUDE.md lines.
- Note: his session prompt asks to "ask Mustafa whether his FREEZE-FDN covers each" kit edit; plan asks only K3/K4; K1 answered; K2 claimed covered. Check against that.
- B3 done: his words match plan 1.2 (decision "may plan and build foundation steps 2.1 and 2.2 now"; SCOPE (a); FREEZE-FDN at STATUS:686-690; LANE-RULES Models addition STATUS ~700-710). His 2.2 prompt: "Plan only, no build" -> build needs his start (plan says so). Next B4.
- B4 done: sketch Part 2 (Q2.2-a..d), Part 5, App B read; advisor rev2 Q-A3 provisional (a) confirmed in notes.
- F-1 (Medium): SCOPE (a) as he picked it (plan rev3, d389d91:.checkpoints/foundation/plan.md:524-536) reads "...and 2.2's kit edit scopes the constraints and adds your 2026-11-10 decision as the exception to the "no v2 code" lines". FDN-NOCODE omits this option text; it bears directly on the question (his own reply words do not repeat it, so asking is still right, but quote it). Plan 1.2 also lacks it.
- F-2 (Medium): K2's second half (CLAUDE.md:51-53 "Phase 0 writes no v2 code" + exception) is the same "no v2 code" edit as K3/K4, yet K2 is marked "covered". Fold into FDN-NOCODE (three lines, not two), or split K2.
- Sketch App B rule 4 said "2.2 (or 2.5 ...) narrows" the globs: leaving to 2.5 is within it.
