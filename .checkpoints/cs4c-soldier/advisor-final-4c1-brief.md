# advisor agent: change-set 4c-1 "The page's scene" on its final head, for Mustafa's Brave check and go

DRAFT until the verifier's PASS: the marks [after] are filled by the main session first.

Asked by Builder V6 (the main session), 2026-11-28. The worktree is `/home/user/Shamble-Frontlines/.gate/cs4b-2`
(local branch `cs/4c-soldier`, pushed as `claude/ecstatic-wozniak-5cbdzs`). **The final head: [after]** (it only
removes the notes; the commit before it holds the report, and the one before that the notes, the pull request's text
and this brief). Main is dae6d107 [after: fetched again before the final head; taken in if it moved]. Your agent file
on main last changed 2026-11-13 (8523d50), before this session started, and this branch does not change it.

**What it is.** Step 0 and step S of the plan sheet's revision 2 (`reread-sheet.md`, sections D and G), which you
approved under conditions 1 to 9 (`advisor-reread-4c.md`); steps 1 to 7 are not in it (4c-2, by his SIZE-4C (b)).
The report: `docs/reports/2026-11-28-cs4c1-scene.md` (its "Plan and approval" says how each condition was met). The
pull request's title and text: `pr-text-4c1.md` at the notes commit (this chat cannot open one; **his Advisor opens it
from a branch of its own pointed at exactly the final head**, his item 6 c, because this branch moves on with 4c-2).
Risk tier 1, needs-mustafa: his Brave check, and step 0 writes into decision docs (LOOP_V2, EDITOR_V2, the register).

**Reviews.** Round 1 on c1de41e5 (six reviewers, three at a time) and its one fix pass (2b2f44c2); round 2 on what
changed (graphics-engineer and test-engineer, both APPROVE; five Low gaps to STATUS); every finding in
`review-s-round.md`. **No reviewer read 0645432f** (one comment reworded on graphics-engineer's round 2 note): please
read it (`git diff 2b2f44c23a9ebf1fa4d5aaa9bd46c9ca72352a9b 0645432f648457b0e03e2f4d4280eb3389e1da65`). The docs
pass (9283d302) is docs-keeper's; the verifier checked its rows against the sources [after].

**Named questions for your ruling (APPROVE, REVISE with what, or ESCALATE to Mustafa with why):**
- **Q1.** The final head as a whole: APPROVE for his Brave check and his go, or not.
- **Q2.** The one-branch way (his item 6): 4c-2 is built on this branch on top of the final head, its first commit
  bringing the notes back (a revert of the final head's removal); a scene change he asks for after his Brave check
  goes into 4c-2's pull request (his item 6 d). Anything in the final head that this would break, or that his
  Advisor's own branch at that head would not carry?

**Evidence on the final code** (0645432f; the docs at 9283d302): the full local checks passed on the fix pass's files
(12:29 UTC; the verifier's own run [after]); the gate PASS with smoke at 9283d302 (`.gate/gate-4c1-docs.log`,
symbols +0 -0); CI green on 2b2f44c2 (runs 46145018063, 46145018054) and 0645432f (runs 46146007607, 46146007608);
the plants' whole run on 2b2f44c2, 45 caught of 45, 3 controls held of 3 (`plants-4c.txt`); the count 1,565 of the
hold 1,665; his words by `decisions-4c.mjs check`, `section14-4c.mjs check`, `loop-check-4c.mjs` and
`quote-check-12bc.mjs`; the verifier [after]. The final head's CI runs [after] (the main session reads them; you get
this brief only once they are green).

**The notes** are out of the final head (`.claude/CHECKPOINTS.md` §9); read them at the notes commit [after] with
`git show <that commit>:.checkpoints/cs4c-soldier.md` (the card) and the same for each notes file the report names.

**Rules for you.** Read-only: no source, doc or note edits, and no commit (the final head must not move after your
read); your verdict file goes to `/home/user/Shamble-Frontlines/.gate/cs4b-2/.gate/advisor-final-4c1.md`
(gitignored); git read-only, each git command alone in its shell call with nothing in front of it, one git per call;
run the gate yourself if you wish (`PLAYWRIGHT_BROWSERS_PATH=/home/user/Shamble-Frontlines/.gate/cs4b-2/.gate/pw-1194
node tools/gate.mjs`); the repository is public. Your final answer: the verdict per question, then for Q1 the full
head id you approve, and what you did not check.
