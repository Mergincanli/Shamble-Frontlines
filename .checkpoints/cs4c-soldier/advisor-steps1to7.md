# Advisor agent: 4c-2 "The first soldier", plan review of steps 1 to 7 (2026-11-28)

Branch head read: eebbbcc2 (local cs/4c-soldier, pushed as claude/ecstatic-wozniak-5cbdzs); 4c-1's final head
946fef73 under it; main dae6d107.

## Plan
1. Read the brief, my re-read (advisor-reread-4c.md) and my 4c-1 final verdict; his answers of 2026-11-28 (saved file).
2. Read the sheet's E, F2, G, H, I (I holds), I4b, I6; rust-reviewer's blocks 9 to 12 (11 and 12 whole); game-designer's
   block 8; the question file's question 13 and its MOVE-RULES/NO-SOURCE/BODY-SHAPE rows.
3. Spot-check against the branch: the notes' revert, the batch's base files, world.rs's cited lines, the test scene's
   shapes and spawns, content's FIELDS.
4. Rule Q1 to Q5; findings one line each; verdict; DONE.

## Notes (one line per finding)
- Checked: `git diff 98107d74 d0e30068` empty (the notes' revert brings back exactly 4c-1's pre-removal tree);
  d0e30068..eebbbcc2 touches only the card and advisor-final-4c1.md.
- Checked: the batch's five base files are unchanged since a4e93679 (`git diff --stat a4e93679 HEAD` on Cargo.toml,
  Cargo.lock, sim, room and client-endpoint manifests: empty), so deps-4c-v2.diff's digests can still match.
- Checked: world.rs at the head: SHARED_STATE_BYTES :18, PlayerCommand :40-43, World :112-117, `const fn new` :122,
  authority_step :149 with "With none the record holds" :143-144, known answers :401-415 (0x9c1e_17d4_0d7a_14dd ...):
  as block 11's B11-1 says. crates/sim/src line counts match (world 477, collision 954, vec 24, lib 48).
- Checked: content/maps/test-scene.json: 23 shapes (17 boxes, 6 ramps), every shape's bottom at y 0 except the four
  roof ramps, which sit at 2.6 m on their house's top (nothing floats); 8 spawns, all facing_deg 90 (the checklist's
  "faces +x" holds); bounds y 0..20. content's FIELDS is [Field; 15] at movement.rs:18.
- F1 (Medium) WORLD-ROOM clash: block 11.6 (rust-reviewer-read.md:904) puts `rules`, `statics`, `spawns` inside `World`
  ("match constants outside the state"), carried from block 1.6 (:245). engine-architect's 4B-WORLD-ROOM ruling
  (engine-architect-read.md:530-538, written after block 1) says "the room owns it", `authority_step` takes
  `&StaticWorld` and `&Rules` borrowed from the room, and "Not in World" (it would ride every state copy, change-set 7's
  resets). The sheet's E3-E5 (:313-319) holds engine-architect's form. No block reconciles them. Condition: before step
  1's code, one written answer agreed by both (default: engine-architect's ruling, the sheet's text); block 11.6's
  World line and 11.1's caller lines follow it.
- F2 (Medium) DEPART-4C's terms have no draft: no new wording of PM-0004-20 or PM-0004-35 exists in the notes I read.
  His term (answers item 12a): "the new wording asks no less than the old about what the players' file does". The old
  rows (ADR 0004 :1141, :1156) ask: 0 native-vs-Wasm mismatches over the run; parry3d's query time per tick, flag on
  and off, native and in a browser, mean and p99; a counter on every query's result with a planted unsupported pair.
  Condition: both new wordings drafted in the sheet, old and new side by side, before step 1 counts as done; each keeps
  the mismatch count on the players' build, the per-tick query time (our sweeps plus the tree) native and in a browser,
  and a counter with its bite (a shape kind our sweep does not handle is refused and counted); any half that cannot run
  in 4c (the flag-off timing; N = 48) stays open in STATUS by name, never reworded away. Where it lands: his words ask
  only that the report shows old and new side by side; ADR 0004's own rows are not edited in 4c-2. As DEPART-4 did in
  4b (STATUS_ARCHIVE.md:2210-2218, a dated pointer for the drafts' next revision), the new wordings go into the report
  and a dated pointer in STATUS; an edit to the ADR's rows would be a decision-doc edit needing his go.
- F3 (Medium) the oracle (test 21) and the ramp: block 11.3 sweeps a ramp only at its sides and an exposed high end, so
  a plain t-against-t comparison with parry3d's ConvexPolyhedron cast cannot hold on the low end or the slope face.
  Condition: test 21 lists which ramp faces it compares, which it leaves out (to the set-back or the step-up) and why;
  it compares "no hit" as well as hit times (a miss where the library hits fails, on the compared faces); seeded
  heights include the box's and the ramp's bottom and top within 1e-4. That is what makes his "the ramp included" true.
- F4 (Medium) his 35-minute checklist (sheet G, :571-593) still says "by MOVE-RULES" (step 3), "only if he takes row 5"
  (step 5), "what TUNE-HOME says" (step 7), "by RESUME-FLASH" (step 8). His answers are in. Condition: with the zip,
  every step states its expected result in plain words: one jump per press, and held, a jump again at each landing
  (J1 to J5's design, netcode-architect-read.md:162); the cover wall's and the ledges' tops gained by a jump (row 5); "copy values" gives the camera
  file's bytes to paste (TUNE-HOME (b)); on return the match as it stood, a tick later the keys let go, the soldier
  slows to a stop, the key diagram dark while hidden (RESUME-FLASH (b)); the roofs walked as the sandbox (row 9 open),
  with the ridge hold below (Q5).
- F5 (Medium) steps 2 to 7's exact symbols: this review saw them as the sheet's summaries of the reads (E2 to E6, G,
  I3), not as consolidated tables like block 11. netcode-architect's read marks its superseded lines in place, except
  :355 and :430 (the roof lines I2 overrides, found by game-designer). Condition: before each of steps 2 to 7 starts,
  its symbol table (add / change / remove by file) with every superseded line and his answers applied, written in the
  sheet and confirmed by its read's owner (netcode-architect for 2, 3's wire, 6, 6b, 6c; engine-architect for 3's
  layout, 4, 5, 7); a table that departs from the read or the sheet comes to me as a plan delta (not a round).
- F6 (Medium) Q2's default: block 11.4 (:877) and 11.3 (:847) set HEADROOM-FEET's default to (a), a difference from
  his sandbox he has not answered. Ruled below (Q2): (b).
- F7 (Low) stale text: I4b :840 says "the 3.10 m ledge's corner, by up to 0.063 m" (block 12 corrected it to the
  3.05 m ledge's corner at x 38, z -4, 0.069 m largest horizontal distance, 0.063 m per axis); G's step 6b "waits for
  the CI Builder's answer" is superseded by I5 (build on, never cut a test, re-run once); the card's :179 rung 13,150.
  Fix in the sheet and card before step 1; the report uses block 12's numbers.
- F8 (Low) block 2's ray counts and sweep proof (90 / 70, "step 1 or 6, the sheet's call", rust-reviewer-read.md:975)
  have no step: the sheet names it before step 1 starts (my lean: step 6, with walk_reaches.rs).
- F9 (Low) step 1's reviewers (G :528-529) leave out security-auditor, whose dependency answer sets conditions on the
  batch's commit (F1's test, the stricter libm scan, the maths list's exact paths), and engine-architect (F1 above,
  layering.rs). Both read step 1's part of the diff; still one round across the part.
- Q5 scan of block 11 for every stated difference from his sandbox: the swept wall pass (11.3), the pull's clip
  (11.1 g), G6's guard as the sweep's ramp rule (11.3), the landing crossing in the vertical phase (11.1 c, G9), the
  1e-4 m/s rest speed (11.1 d), the one-spot push normal +x from the lower id (11.1 3), the bounds' top as an underside
  (11.1 c), HEADROOM-FEET (Q2), the ridge's one-tick hold (block 12). Ruled in Q5.

## Q1. Steps 1 to 7 as revised in section I, step 1 as block 11 with block 12: APPROVE, with conditions
Risk tier 1, needs-mustafa (a player sees, hears and feels it: his Brave check; ADR 0004's PM rows are a decision doc;
his words in the register). Item 10's thirteen points: (1) BODY-SHAPE (3a) his; (2) the prototype, blocks 5 to 10 and
12, with what was not run said; (3) SWEEP-ROUTE his DEPART-4C (a) on his terms; (4) GROUND ruled, test 23; (5)
FACING-SPAWN (a) with its reviewers; (6) row 9 open, nothing built on roofs (I2, tests 7 and 8 moved to made maps); (7)
the record rule in the row (11.2); (8) the 13 NO-SOURCE rows with their version rule and `--check` (11.4); (9) the
refusals and the jump against `jump_reach_m` (11.4, test 2 and 6); (10) the batch answered, base files unchanged; (11)
CHROME-TIME his; (12) the line 18,700 and 12,470 on the card, the rung below; (13) G per step. All met, under these
conditions:
1. F1: one written answer from engine-architect and rust-reviewer on where `Rules`, `StaticWorld` and the spawns live,
   before step 1's code (default: the room owns them and `authority_step` borrows them, as the sheet's E3-E5 says).
2. F2: PM-0004-20's and PM-0004-35's new wordings drafted, side by side with the old, before step 1 counts as done,
   asking no less of the players' file; what cannot run stays open in STATUS by name.
3. F3: test 21 states the ramp faces it compares and leaves out, and compares misses as well as hit times. Step 1 counts
   as done only with test 21 green (his "before the movement step counts as done").
4. F5: each of steps 2 to 7 gets its consolidated symbol table, confirmed by the read's owner, before it starts.
5. F4: the zip's checklist with every expected result in words; the ridge's hold in its roof step (Q5).
6. F9: security-auditor reads the batch's commit against its own conditions; engine-architect reads step 1's boundary.
7. Lows F7 and F8 ride the next notes commit; no push of their own.
8. The plan's invariants stand: CI green at every commit that changes the hashed state (pins re-made in that commit,
   11.8); a head for me needs a green run (CHROME-TIME); his 25-minute save rule; 3 agents at a time.

## Q2. HEADROOM-FEET's default while he has not answered: (b), his sandbox's rule. My ruling, not an escalation
- Why: his words that cover headroom are MOVE-RULES (a), "all rows, as the game's starting rules", with row 1 as put to
  him ("the stand-in's 1.80 m height for headroom"), and BODY-SHAPE (3a), his sandbox's body. A default of (a) would
  build a difference from what he played while his question 13 is open: a rule of his decided without him
  (escalation category 2; High on CLAUDE.md's scale if built). (b) decides nothing new. His FALL-STEP (c) sentence
  ("The game does what my sandbox does") answered one question; I cite it only as the same leaning, not as the reason.
- The question stays with him (question 13); the switch flips to (a) only on his own word.
Conditions:
1. `HEADROOM_FEET`'s default is `SandboxTest`; the test-only constructor runs `OwnTopNeverWall`. Test 15 says what each
   mode asserts on each layout: the window wall and the floor over a crate (stopped 0.40 m short under (b), walked whole
   under (a)); the step with a beam over its edge (a wall in both); the beam 1 m into a 0.55 m step (under (a) no
   flicker; under (b) the prototype's 6 to 12 falls in 40 ticks, B9-2, pinned as the sandbox rule's measured
   behaviour, not as a wanted one). Its plant becomes "each mode run with the other's rule".
2. "(b) changes nothing on the test scene" is the prototype's claim (B10-5), and the notes hold a (b) output only for
   block 9's cases (rr-body-proto-8-b9-sandbox-headroom.txt), none for blocks 5, 6, 10's or 12's; every expected
   number in 11.7's tests 6 to 20 was measured under (a). Before step 1's movement code: rust-reviewer re-runs blocks 5,
   6, 9, 10 and 12 with the sandbox switch on and lists every case that differs, the six roof runs included (the roofs
   are where a top lies at or below his feet with a shape above it, and he walks them at the early look); 11.7's
   expected numbers follow the default. In the build: the walk and lane-walk hashes shown identical under both modes,
   or the flip commit re-makes every pin it moves.
3. Question 13 gains, before he next reads it: the default is (b) until his word; and under (b), by the prototype's
   model of his sandbox's rule (not tried in his sandbox), a soldier on a 0.55 m step with a beam 1 m into it flickers
   off and on 6 to 12 times in 40 ticks. Whether his early look is the same either way is said only after condition
   2's run shows it.

## Q3. The rung: about 14,960 written lines (the plan's rule on his line, 18,700 / 1.25)
- F2's table (revision 3, before his answer) called the recount-based rung "his to move to or keep". His answer set the
  stop line and said nothing of the rung; I1 (revision 4, which holds) gives it to me; his SIZE-4B point 2 asks that
  the card names what leaves first and that he is told before it moves, not that he sets its number. So it is mine
  (scope and order of approved work).
- 13,150 was the planned size under a line (16,450) he did not take; 14,960 keeps the plan's own proportion to the line
  he set. The stakes are low: the far-corner map is step 6c, after the early look; on the estimates (the early look at
  about 15,360 to 19,110) both numbers are crossed before 6c can start, so the rung decides nothing about the early look.
Conditions: the card's :179 and "What leaves first" say 14,960 before step 1; he is told in the next report before it
moves; the two-thirds sentence at about 12,470 (two thirds of his 18,700); at step 5's end, past 18,700, nothing after the early
look starts without his word.

## Q4. The order inside 4c-2: the plan's order, with these conditions
Withdrawn in writing: my item 7's "under (b) or (c), once 4c-1 has merged" (advisor-reread-4c.md:36). His item 6 b
(build the soldier on top of the scene's final head) replaces it; my 4c-1 verdict's Q2 conditions stand.
1. Order: 1, 2, 3, 4, 5, the early look's zip, then 6, 6b, 6c, 7. Pieces of steps 2 and 3 that need nothing of step 1
   may come earlier only on conditions 2 to 6; nothing else open hangs on his answers (HEADROOM-FEET is a switch with a
   default; row 9 builds nothing on roofs).
2. The batch lands whole in the commit that first calls exp and sincos, with F1's test, F2's layering test, the stricter
   libm scan, the maths list's exact paths and the vectors (11.5); its five digests checked against dependency-check.md
   when applied; any differing file goes back to security-auditor.
3. The record rule and "no aim yet" land whole in step 1, so step 2 moves no first-loop hash (11.8).
4. Every commit that changes the hashed state re-makes world.rs's known answers and first-loop in that commit; CI green
   at each.
5. J1 to J5 (the jump's rule, 3B-ONE-PER-TICK: 255 presses one jump, none kept for the landing, held jumps at each
   landing, two commands one jump, a short tap one jump; netcode-architect-read.md:162) are movement tests of step 1's
   code: block 11.7 gains them as a row with their plants and 11.9 their 10 / 80 lines; step 2's count drops them
   (netcode-architect-read.md:974-975 asks for one count).
6. `walk` lands in 4c-2 (FEATURE-COMPARE condition 4), with its route and walk_reaches.rs off the roofs (I2).
7. Step 1 counts as done only with test 21 green and F2's wordings drafted (his DEPART-4C terms).
8. After 4c-1's merge is on main: main comes in once, in this worktree, never the start folder; 4c-2's pull request is
   not marked ready before that. From then on the count is `git diff --numstat` from the merge-base with main to the
   head (main's tip after the take-in), not from 946fef73, so another lane's lines on main are never counted.
9. A scene change he asks for after his Brave check goes into 4c-2, reviewed as scene code (graphics-engineer and
   art-director at least).

## Q5. Information to him, not questions: AGREE, with rulings and conditions
- The swept wall pass: not a rule of his (game-designer block 8, N30): row 3's words (slides, loses the speed into the
  wall, round corners) hold under both, and the game's form he answered was a swept body. Information line with block
  12's corrected numbers: bit-equal to his sandbox's push-out in the open and on every face; at the 3.05 m ledge's
  corner up to 0.069 m in one case at his walk (a soldier rounding it while pushing another); 4 micrometres in one
  landing at his sprint; on the roofs at his sprint a one-tick hold 0.4 m before the ridge, the same result.
- The ridge flag: a STATUS follow-up, named with its trigger (row 9 answered, or the first ramp-to-ramp seam under about
  77 % in a map), about 15 / 20 lines. Since he walks the roofs in the early look to judge slopes (his words), the
  checklist's roof step says: "at a sprint over a roof's ridge the game may hold you one tick before it, where your
  sandbox carries you over; if you feel it, say so and the fix (about 35 lines) comes in".
- G6's guard (I4b asks me to rule): not his. No map of his has a ramp met from beyond its high end; his sandbox's
  literal set-back, in the prototype's model of it as read (not run in his sandbox), carries him 7.3 m through the
  ramp in one tick there (B9-5), a defect, not a feel; the guard is now
  the sweep's ramp rule. Information line, with the case.
- Also not his, each a tolerance or a fix in a case no player meets at his sizes, each listed in the report as a
  difference from the sandbox with its number, and game-designer's written yes at the step's review (owner): the pull's
  clip (bit-equal wherever no wall is in reach, B10-4); the landing crossing (G9: the prototype's model of his sandbox
  misses 4 of 21 high drops onto a 2.0 m top); the 1e-4 m/s rest speed; the one-spot push normal (+x from the lower id, where his sandbox's
  per-pair angle needs trigonometry); the bounds' top as an underside (20 m on the test scene, above any jump).
- Not information but his: HEADROOM-FEET (question 13, Q2).

## Verdict
**Q1 APPROVE steps 1 to 7 (as revised in I, step 1 as rust-reviewer's block 11 with block 12), with Q1's conditions 1
to 8.** Q2: (b) as the default until he answers, with its three conditions. Q3: 14,960. Q4: the plan's order on nine
conditions. Q5: agree, with the rulings above. No Critical or High open; Mediums F1 to F6 are conditions; Lows F7 to F9
ride the next notes commit.

Not checked:
- Nothing built or run: no gate, test, render or prototype of mine; every movement number is rust-reviewer's prototype
  (its own re-implementation of the sandbox as read, never the sandbox), taken from its notes.
- His Testlines sandbox's code: game-designer's reading, not mine; HEADROOM-FEET's "the sandbox ignores the feet" and
  the flicker under (b) rest on it.
- Steps 2 to 7's symbol tables against today's code (F5); engine-architect's blocks 3, 5, 6 and netcode-architect's
  blocks 1 to 8 read only where the sheet cites them.
- The designs' counts (8,550; step 1 about 1,510 / 1,840): taken from the reads, not recounted.
- Whether question 13 has reached him; his early-look checklist as the zip will carry it.
DONE
