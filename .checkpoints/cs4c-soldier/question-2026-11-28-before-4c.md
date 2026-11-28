# Questions to Mustafa before 4c's steps (2026-11-28, Builder V6)

Written by the main session at 00:18 Amsterdam (from the clock), after pull request #79 (4b-2) merged at 23:12:04
UTC. The first four are the ones the walk plan lists before 4c (`.checkpoints/cs4-walk/plan.md`, section 13, items 6,
7, 8 and 10), asked again here in plain words, as his message of 2026-11-27, 23:00:39 UTC, item 5 asks. The other
four came up tonight. Each says which step of 4c waits for it. What waits for none of them is built meanwhile, after
the advisor agent's review of the plan sheet: first the page's scene with the map's reading (4b's old step 12).

## 1. MOVE-RULES: which movement rules of his Testlines sandbox does the game build where they differ from the old game?
- **Why it matters:** each row changes how the soldier meets walls, steps and slopes, and his point 8e lets none in
  without his word. **Needed before 4c's movement step** (rows 6 and 7 only before 4d).
- **The rows** (game-designer's check, `.checkpoints/cs4-walk/game-designer-cs4.md`, block 2):
  1. **Body size against walls:** his Export's radius, 0.4 m, with the stand-in's 1.80 m height for headroom. The old
     game: 0.7 m, written into its code; at 1.4 m wide he would have 0.1 m to spare in the street corner's 1.5 m alley.
  2. **Steps and edges:** he rises onto a step as soon as his body touches it, and holds a little past an edge (the
     body's round foot rests on it). The old game: he rises only once his centre crosses the step's edge, and drops
     off a ledge as his centre passes it. The sandbox's pull onto the block is left out: the blend between two ticks
     already smooths the rise.
  3. **Walls:** he slides along a wall and loses the speed that went into it; corners are round. The old game: he is
     pushed out, keeps the speed, and corners are square.
  4. **Slopes and stairs going down:** he stays on them. The old game: he falls down them in small hops.
  5. **Real heights:** every block has its real height, and a jump that reaches a top lands on it. In the street
     corner he gets onto the 1.0 m cover with less of a jump, and onto the houses' roof edges (2.6 m, under his
     3.07 m reach). The old game: walls and houses are endlessly high; only platforms, ramps and cover can be stood on.
  6. **Q aims too**, beside the right mouse button (4d). The old game: the right button only.
  7. **The sights camera** tips toward the point the cursor marks in the resting view, so it settles, and never so far
     that his feet leave the screen (4d). The old game: it tips toward the live aim point, down to a fixed floor.
  8. **Listed only, nothing he feels today:** two values for speeding up and for stopping, and one for steering in the
     air (the old game has one value and full steering; his Export holds the old game's values in all three); the
     head bumps a block above (nothing hangs overhead in the test scene); the map's edge is the map's own bounds,
     and the speed into it is dropped (the old game: the ground less 1 m, the speed kept).
- **Not asked**, because his START-RULES decided them: the jump's reach of 3.07 m, and the soft push between soldiers.
- **Options:** (a) all eight rows as written; (b) the rows he names, the rest as the old game; (c) the old game's rule
  in every row.
- **Recommended: (a).** He has played these rules, and his values were tuned under them. Rows 3, 4 and 5 follow from
  moving a real body with the collision library's queries; the old game's rule there would have to be built against
  the library on purpose.

## 2. NO-SOURCE: where do the few numbers come from that the sandbox keeps in its code and not in its Export?
- **Why it matters:** his point 8b takes values from his Export and from nothing else, and the movement needs a few
  numbers the Export does not carry: the steepest slope he can walk; the soft push's four limits (how far a head-on
  push turns sideways; that a push never undoes more than half of a walker's own step; the most one step may push;
  the height gap above which two soldiers do not push); and the body's height (the stand-in's 1.80 m). **Needed
  before 4c's movement step.**
- **Options:** (a) his Testlines chat makes them sliders and he exports again; the game takes them from there, and
  "from my Export and from nothing else" stays whole. (b) The movement design picks them, starting from the sandbox's
  own numbers where it has one; each is a row of the table marked "not from your Export", lives in the game's content,
  and he changes it by telling this chat. A yes to (b) is an exception to "from nowhere else" for exactly these
  numbers. (c) Drop the rules that need them and keep the old game's (not possible for the soft push: the old game
  has none, and START-RULES asks for it).
- **Recommended: (b).** The movement then waits on no other chat, and each number still has one home (his point 8g).

## 3. RESUME-FLASH: what does the first picture show when he comes back to a tab he had hidden?
- **Why it matters:** in 3b's browser check he saw a key box stay lit for a moment, and a soldier now stands where that
  box was. **Needed before 4c's page step.**
- **Options:** (a) the match as it stood when the tab hid; one tick later the keys count as let go, and the soldier
  slows to a stop over about half a metre. No rule changes. (b) As (a), and the development key diagram goes dark the
  moment the tab hides, so what he saw in 3b is gone (about 10 lines and one test). (c) Keys held when the tab hid
  count as held until his first input: not recommended, since it undoes 3b's rule that a hidden tab lets every key
  go. (d) The soldier stops dead at the return: not recommended, since the hidden tab would become an input of the
  simulation.
- **Recommended: (b).** That lit box is what he saw and asked about, and (b) removes it. netcode-architect, whose
  design this is, recommends (a), "(b) if he also wants the 3b flash gone".

## 4. TUNE-HOME: where does he tune once the game has a soldier?
- **Why it matters:** his Export block says "Values go one way, from this Export into the game, and from nowhere
  else". Option (a) keeps that rule whole; a yes to (b) or (c) changes it for the values they move, which from then on
  are tuned in the game and no longer come from an Export. **Needed before 4c's camera panel step.**
- **Options:** (a) the sandbox stays the home of every value; the game's camera panel is for trying only, and what he
  finds there he sets in the sandbox and exports. (b) One home per group: movement, jump, stamina, pushing and the
  aiming rule stay in the sandbox (he exports, the tool rewrites the content, the check holds them equal); the
  camera's values start from his Export once and then live in the game's camera file, tuned in the panel on the real
  picture, with "copy values" pasted into the chat. (c) Everything moves to the game now.
- **Recommended: (b).** The camera is judged on the real picture, and his queued words name the panel as its tool
  ("Slice 1 ships with a live camera tuning panel from its first playable build").

## 5. SIZE-4C: how is 4c cut, now that the page's scene is inside it?
- **Why it matters:** 4c is likely to pass its stop line well before its end, and he asked to hear such things early
  (SIZE-4B) rather than at the line. By the designs' count, 4c is 7,315 lines (steps 1 to 7) and about 630 for the
  scene with the map's reading: about 7,950. Written lines have run at 2.4 times the designs' count in 4a, 2.17 in
  4b-1 and 2.7 in 4b-2, so 4c would come to about 17,250 to 21,450 written lines. The plan's stop line for 4c is
  16,450, set before the scene moved in. The STEP12-GO question said nothing about 4c's line, so the number is his.
- **Options:**
  - (a) **One pull request** for all of 4c, the scene first, with a stop line he sets. By the plan's rule (planned
    size times 1.25) the scene adds about 1,420, which gives about 17,870. 4c is still likely to reach that line and
    come back to him there.
  - (b) **Two:** the scene alone first (4c-1: he checks the grey street corner and training yard in Brave; my estimate
    about 1,650 written lines, against about 1,420 by the plan's rule, so its line is his to set), then the soldier as
    4c-2 at the plan's 16,450 (steps 1 to 7 alone come to about 15,900 to 19,750 written lines at those rates).
  - (c) **Three:** the scene alone first, as in (b); then the soldier in two halves, at a seam engine-architect draws
    in the plan's re-read with no proof leaving the code it proves (for example the simulation's movement with its
    recordings first, which he cannot play yet, then the page he plays, with the camera panel and the early look).
    Their sizes and stop lines come to him with the re-read sheet. If no clean seam is found, the soldier stays one
    pull request, as in (b).
- **The cost of cutting:** each pull request needs his go and, where a player sees it, his Brave check. While one waits
  for him, this chat can only write notes, because it has a single branch: a push would move the head he approved.
- **Recommended: (c).** Each pull request stays near the size 4b's halves were reviewed at (4b-2: 8,408 lines, two
  review rounds, by six reviewers and then five), and he sees the scene in Brave first. Until he answers, the scene is built (it is the same
  code under every option), held to about 1,420 lines, the lowest line any option gives it. Its reviews wait for his
  answer, because under (a) they run on 4c's final diff.

## 6. BOARD-WORDS: what goes into the register from his message of 23:59 Amsterdam?
- **Why it matters:** that message (2026-11-27, 23:00:39 UTC) holds MERGE-ROUTE (a) and how 4c starts, and its item 3
  names his board. His board rule lets a note in the repository say only that the board add-on is active, so this
  chat has kept that message out of the repository, notes included. His go for 4b-2's merge ("you have my go", 22:50:52
  UTC) is a separate message and goes in as it is.
- **Options:** (a) the whole message, word for word (item 3 names a board thread and gives no address); (b) the message
  without its item 3, the entry saying that item 3 is left out because it names his board; (c) none of it: the register
  records MERGE-ROUTE (a) in this chat's words only.
- **Recommended: (b).** The register keeps his decisions word for word, and the repository says nothing of the board.

## 7. PASTE-BREAKS: may the paste's broken lines be mended when his section 14 goes into the loop doc?
- **Why it matters:** his design notes (22:15 Amsterdam) say to place the text of their item 8 in `docs/LOOP_V2.md`
  "as it stands", but the paste broke every list line: each "- " stands on a line of its own, and each wrapped line
  is cut off from its two-space indent. Placed byte for byte, the section would show empty list marks and loose lines.
- **Options:** (a) mend only those breaks: each "- " joined to the line after it, and each two-space indent to the
  line after it, with no word or sign changed; a script does it and checks that every word and sign is his message's,
  in the same order. (b) Place it byte for byte as it came. (c) He pastes item 8 again.
- **Recommended: (a).** It places his text as written, and the check proves no word moved. His notes go into the
  register whole either way, as they came.

## 8. DESIGN-CLASH: two lines that clash with his design notes, beyond the pointers his item 3 asks for
- **Why it matters:** his item 5 says to list such lines, not resolve them. Left as they are, the design docs say two
  things.
- **The lines:**
  - `docs/EDITOR_V2.md:45`: "**Objectives as data:** spawns, barracks and terminals, harvest fields (the centre and the
    safe ones), uplink sites, cache spawn points." His notes replace the open fields with resource points.
  - `docs/LOOP_V2.md:52`: "`[DEFAULT]` Uplinks replace silos and tech boxes: one tech source, the exciting one." His
    notes give each team a resource silo on its harvester building, as a money source.
  - Left out on purpose: `docs/GAME_DESIGN.md`'s silo lines (27, 79, 91 and 182), which describe the old game, and the
    look brief's "plastered house with the silo tower" (`docs/LOOK_V2.md:22`), which is a building's look.
- **Options:** (a) the same dated pointer as his item 3 at both lines, "(2026-11-27: see §14)", with no other word
  changed; (b) leave both lines as they are; (c) his own words for each.
- **Recommended: (a).** It changes no decision and points the reader to §14.

## Added at 00:55 Amsterdam, after engine-architect's read (`engine-architect-read.md`, blocks 1 and 2)
### SIZE-4C, two new facts, and the recommendation changed from (c) to (b)
- **The scene is larger than this file said:** about 740 lines by the designs' count (not 630), so about 1,600 to
  2,000 written lines. By the plan's rule its stop line would be about 1,665 (not 1,420); a line of about 2,000 would
  let it finish without a stop. Until he answers, it is held to about 1,420, the number told to him, unless the
  advisor agent rules on that hold (it was this chat's own limit, not his).
- **Option (c) delays the first build he can play.** Its first soldier half (the simulation, the command path and the
  recordings) changes nothing on screen, and with one branch this chat cannot build the page half while that half
  waits for its reviews, its verifier and his go. Under (b) the same work comes before the early look with no stop.
- engine-architect found the seam (c) needs (it is clean, on three conditions, one of them netcode-architect's yes
  that a post with no ray is not counted as a refused ray): halves of about 8,600 to 11,900 and 6,300 to 9,100
  written lines.
- **Recommended now: (b).** He sees the scene first, in a small pull request: its build can run tonight (the same code
  under every option), and its reviews and verifier start the moment he answers (b), so that it is soon ready for his
  Brave check (its docs commit, with his words of 27 November, follows his answers to PASTE-BREAKS and BOARD-WORDS);
  then the soldier as one pull request with the early look inside it and no stop before he plays. It is large (about 15,900 to 19,750 written lines against the plan's 16,450),
  so it is likely to reach that line near its end and come back to him there. (c) stays the choice if smaller pull
  requests matter more to him than playing early.

### 9. STREET-GREY: may the street and the grass take two greys? (a default he can veto, like GREY-COLOURS)
- **Why it matters:** both ground kinds lie at height 0, so by "higher is lighter" alone they take one grey and the
  street does not show, while the plan told him he sees "a street" (the plan's section 2).
- **Options:** (a) one grey for both, darker than every block: the street shows only when the look comes (change-set
  6); (b) two neutral greys, one per ground kind, both darker than every block: the street shows; (c) the patches drawn
  as outlines in the grid's grey, with no fill.
- **Default: (b).** Unless he says otherwise, the scene is built with (b); it stays inside his "neutral greys".

## Corrected at 01:12 Amsterdam (from the clock), after game-designer's read of his sandbox's code (`game-designer-read.md`)
game-designer read the sandbox's code itself (his Testlines repository, commit be608cad, the commit his Export names;
fetched read-only by its public address, outside the one repository this session has attached: told to him). Three
things this file said were wrong, and they are corrected here; where this block and the text above differ, this block
holds.
### MOVE-RULES, row 2 corrected
- **What his sandbox really does:** a flat-bottomed body 0.4 m in radius meets the step; he rises as soon as the step's
  top is at most the step height (and 5 mm) above his feet; **a short pull then carries him onto the top** (up to
  0.265 m over 5 ticks, about 0.17 s); he then **stands on a small flat circle of 0.15 m round his centre**, so he
  holds a little past an edge (0.15 m). On a ramp he stands at the slope's height under his centre.
- Above I said "the body's round foot rests on it" (it is a flat circle) and "the sandbox's pull onto the block is left
  out" (that holds only for a body that stands on its full width): both withdrawn. Which body the game takes is the new
  question BODY-SHAPE below.
### MOVE-RULES, a new row 9: steep slopes
- **His sandbox:** he walks up any slope a tick's step allows (about 260 % at walking speed), so in the street corner he
  walks up the houses' roofs (89 % and 95 %) once he has gained their 2.6 m edge. **The old game:** it looks 0.7 m ahead
  and refuses anything steeper than about 63 to 79 %. (The steepest slope he walked in the sandbox was its stair, 64 %.)
- This row joins MOVE-RULES' options: under (a) he walks the roofs as in the sandbox; under (c) the old game's rule; or
  he names a limit of his own (a grade, a new number). Above, "the steepest slope he can walk" was listed under
  NO-SOURCE: the sandbox has no such number, so it moves here.
### NO-SOURCE, its list corrected
- **The body's height is in his Export already:** measured at 1.8 m (`measured.soldier_height_m`) and as the hit
  cylinder's height (`constants.shot_geometry.hit_cylinder_height_m`, 1.8 m). It leaves this list: the game takes it
  from his Export.
- **Numbers the sandbox keeps in its code that were missing above:** the foot's radius (0.15 m); the pull's three
  numbers (0.5 of the way a tick, done under 0.03 m, up to 0.9 of the foot); the snap down (he falls from anything over
  0.2 m; a snap of the full step height would put him on the ground at once from a 0.55 m block); the 5 mm step
  allowance (recommended: none in the game, his numbers exactly); and two speed gates of the soft push (0.2 and
  0.1 m/s).
- **The soft push's four limits, with the sandbox's values** (for option (b)'s starting values): no push between two
  soldiers whose feet differ by 1.0 m or more; a head-on push turns sideways by 0.7 of itself; a push never undoes more
  than half of his own step forward; at most 0.25 m a tick (7.5 m/s at 30 steps a second). His Export is at 30 steps a
  second, the game's rate, so these carry over as they are.
- The options and the recommendation stand: (b), each number a row marked "not from your Export".
### BODY-SHAPE: which body does the soldier have when he meets steps, edges and ramps? (new; the full text follows)
- **Why it matters:** the plan told him "a vertical capsule" (a round bottom), and rust-reviewer's prototype shows that
  body cannot gain the 0.55 m step and be stopped by the 0.56 m one (the yard's test) under any rule it tried.
- **Options:** (1) the capsule, as the plan said: fails the step test; (2) an upright cylinder of 0.4 m with a flat
  foot: passes the step test 9 of 9 and 9 of 9, but he can stand 0.4 m out over an edge and floats 0.10 to 0.20 m over
  the yard's ramps; (3a) his sandbox's own body, the 0.4 m body for walls and steps, the 0.15 m foot and the pull:
  exactly what he played; (3b) the same without the pull, standing on his whole body on a step he has just taken until
  his foot reaches it.
- **Recommended: (3a)**, if rust-reviewer's prototype, run again with the foot and the pull, passes every run (the two
  steps, the 3.05 and 3.10 m ledges, the ramps); the run is going on now, and its result comes with the next update.
  Fallback (2), with its overhang and float said to him before his Brave check.

## One line, no question
STEP12-GO is in the register already (`docs/DECISIONS.md:183`, since 4b-2's docs pass), so it is not written again.
Only his go for 4b-2's merge, his word on merging and his design notes (and his message of 23:59, by BOARD-WORDS) go
in at 4c's first docs commit, which follows the advisor agent's review of the plan sheet and waits for PASTE-BREAKS
and BOARD-WORDS.
