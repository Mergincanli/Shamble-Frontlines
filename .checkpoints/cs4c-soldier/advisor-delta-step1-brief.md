# advisor agent: a plan delta for 4c-2's step 1 (your F5 rule), and step 1's build order

Asked by Builder V6 (the main session), 2026-11-28, after your verdict on steps 1 to 7
(`.checkpoints/cs4c-soldier/advisor-steps1to7.md`). The worktree is `/home/user/Shamble-Frontlines/.gate/cs4b-2` (local
branch `cs/4c-soldier`, pushed as `claude/ecstatic-wozniak-5cbdzs`). Your agent file on main last changed 2026-11-13
(8523d50), before this session started, and this branch does not change it. Your F5 says a table that departs from its
read comes back to you as a plan delta, not a review round. These are the departures since your verdict, each with
where it is written; then the build order.

## What changed since your verdict, and where
1. **F1 closed** (your condition 1): engine-architect's block 7 (`engine-architect-read.md`, end): the room owns
   `rules`, `statics`, `spawns` and the query counts; `World { tick, player: PlayerRecord, soldiers, seed }`;
   `authority_step(&mut self, &Rules, &StaticWorld, &mut QueryCounts, commands)`; 17 changes to 11.6; tests T-A to T-G.
   Confirmed one by one by rust-reviewer's block 14 (`rust-reviewer-read.md`).
2. **B5 (c):** the match plays the bundle's one movement document, refusing none or several (`CreateError::Soldier`);
   confirmed by rust-reviewer (row 33).
3. **B4 (a):** the one soldier exists from `World::new`, in slot 0 at the lowest-id spawn (the id's byte order), at
   the match's start: netcode-architect's block 9.2 (`netcode-architect-read.md`), with one sim constant for the
   player's slot and team, the fold on an empty slot touching no row, tests W-1 to W-4; game-designer's block 9: a
   build default, not a rule of his (the early look names `street_1` as a stand-in); grounded at spawn
   (rust-reviewer's block 15).
4. **The room's half of the move and aim moves from step 2 into step 1** (netcode-architect's 9.3): the room refuses a
   move longer than 127 (`MOVE_REACH`, `move_in_reach`, `refused_move`) and an aim the map does not hold
   (`refused_aim`), the two NONEs tied by a const assertion, any_bytes' lies; its T2-6 and T2-7 move to step 1; step 2
   then touches neither `crates/room/src` nor sim's movement files (its evidence: an empty `git diff` over them and
   `recordings.rs` green). The wish also caps a longer pair for a world fed directly (rust-reviewer, row 38).
5. **Q-1** (netcode-architect's 9.5): the step only writes its query counts (two worlds, fresh counts against counts
   near `u32::MAX`, equal hashes; plant: a branch on a count).
6. **layering.rs** (security-auditor's third answer, `security-auditor-deps-4c-v2.md`, end): REVISE on its wording
   only, the batch's approval standing; a narrow test-only marking with its plants and controls; shared in
   `crates/sim/tests/support/marking.rs` with content's panic scan, whose same leak (its F5) is fixed in the same
   commit (rust-reviewer's block 15); its F6 (the endpoint's dev line unused until step 2, whose commit must use it).
7. **Your Q2's re-run** (rust-reviewer's block 13): under (b) with the design's sweep, the test scene and every
   recorded route are the same as under (a); 5 of 329 made-layout runs differ (the window wall, the floor over a
   crate). The flicker of your condition 3 happens only with a push-out wall pass (his sandbox's, by the prototype's
   model), not with the game's sweep: question 13 is corrected to say so, and gains "his early look is the same
   either way" after the run (`question-2026-11-28-before-4c.md`, question 13).
8. **F2's draft** (your condition 2): PM-0004-20's and PM-0004-35's old rows copied by script beside a new wording that
   keeps your three keeps (`pm-wording-4c2.md`), for rust-reviewer's and netcode-architect's review at step 1.
9. **The counts:** step 1 about 1,633 / 2,300 (3,933) by rust-reviewer's 11.9 after block 15 (it was about 3,350 at
   your verdict); step 2 about 295 / 525 (netcode-architect's 9.3, less what moved to step 1). By the designs' count
   4c-2 is now about 8,900 (my sum: 8,550 + about 580 into step 1 - about 205 out of step 2; not recounted by an agent),
   about 19,300 to 24,000 written lines at the measured rates, against his stop line of 18,700 (his SIZE-4C (b): if the
   count reaches it before the early look is ready, two lines to him and the build goes on to the early look).

## Step 1's build order (`step1-build-order.md`)
C1 the soldier table and the aim's type (no state change); C2 content's thirteen fields, the frozen bundle re-frozen
(the recordings' headers move, their state hashes do not); C3 the dependency batch with fmath, rules, spawn points, the
room's constants and every condition of the batch (F1, F2's layering.rs as item 6, the libm scan, the maths list, the
vectors); C4 the soldier in the shared state (pins re-made); C5 the movement (pins re-made; step 1 counts as done only
with test 21 green and F2's wording drafted). Each commit: the gate, the full local checks, CI green before the next
code push.

## Asked of you
- **D1.** APPROVE or REVISE items 1 to 8 as plan deltas to steps 1 and 2.
- **D2.** APPROVE or REVISE the build order C1 to C5 (in particular C2 before C3: content's fields land, and the
  frozen bundle is re-frozen, before the batch; and C4 before C5: the soldier enters the hashed state standing, then
  the movement moves the pins again).
- **D3.** Whether the stop line's likely crossing (item 9) needs anything now beyond his own rule.

**Rules for you.** Read-only: no source, doc or note edits but your verdict file
`/home/user/Shamble-Frontlines/.gate/cs4b-2/.checkpoints/cs4c-soldier/advisor-delta-step1.md` (plan first, a line per
finding, the verdict, DONE); no commit; git read-only, each git command alone in its shell call with nothing in front
of it, one git per call; the repository is public. Your final answer: the verdict per question with its conditions,
and what you did not check.
