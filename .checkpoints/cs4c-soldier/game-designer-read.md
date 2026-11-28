Task: game-designer's focused read for 4c (BODY-SHAPE, the soft push's numbers, GROUND, slope unit). Read only.

## Plan
1. Sandbox code (branch claude/sleepy-edison-ygmwfe, fetched in its own command): body shape and size, step-up, edge, ramp, step of exactly the step height and one above; would 0.55 gain and 0.56 stop with his Export's values.
2. BODY-SHAPE options for him, plain words, recommendation (not a decision).
3. Soft push: each formula and its four limits, names, values, units, file:line; the steepest walkable slope and its unit.
4. GROUND and slope unit: what the rules and his words imply.

## Notes
- N1: origin has no branch claude/sleepy-edison-ygmwfe (the sandbox is its own repository); fetched it by URL (git fetch https://github.com/Mergincanli/Shamble-Testlines.git claude/sleepy-edison-ygmwfe) into FETCH_HEAD = 703fd922 (2026-11-26T17:58Z). Read at be608cad (the commit of the earlier read and his Export's tip): `git diff --stat be608cad 703fd922` shows movement.ts, yard.ts, push.ts unchanged (only exporter.ts and params.ts moved), so every line below holds at both.
- N2: his Export (content/source/testlines-export.json): step_rate_hz 30, warnings []; body_radius_m 0.4, step_up_m 0.55, jump_reach_m 3.071, gravity 24, push_radius_m 0.4, push_strength_per_s 8. The sandbox's start rate is also 30 (params.ts:428-437, start '30').
- N3: the sandbox has NO steepest-slope number (grep of every src/*.ts for slope/steep/grade: only slopeUnder, a rise per metre used for the snap-down allowance). A ramp is a wall only where its height ahead exceeds feet + step-up (yard.ts:308-309, 318-334). The question file's NO-SOURCE lists "the steepest slope he can walk" as a sandbox number: there is none to take.
- N4: the sandbox's ground support is a 0.15 m foot circle (movement.ts:59-60 FOOT_RADIUS, :163, :194), a flat disc, not a round (capsule) foot; plus the pull (PULL_SHARE 0.5, PULL_MIN 0.03, target 0.9 x foot: movement.ts:61-63, :172) and SNAP_DOWN 0.2 m (:65, :199). My block 2 (game-designer-cs4.md:95) listed the pull, the foot circle and the snap as NO-SOURCE numbers; the question file's NO-SOURCE list (question-2026-11-28 item 2) dropped all three.

## Block 1. What his sandbox does (DONE)
Commit read: be608cad (identical files at the branch tip 703fd922). All lines are `src/<file>:<line>` at be608cad.
Values below are his Export's (N2). Not run: the reading is the code's comparisons with his numbers (arithmetic).

**The body.** A flat-bottomed vertical cylinder, though the code never names one: against blocks the body is a circle
of radius r = bodyRadius (0.4) in plan view (`overlaps`, yard.ts:177-183: closest point of the block's footprint,
strictly inside r), between the feet height y and y + BODY_HEIGHT (yard.ts:32: 1.8, then set at start to the stand-in's
measured top, main.ts:67-68). Walls: the circle is pushed out to distance r from the block's closest point
(yard.ts:336-341), so corners are round; the speed into the wall is removed (movement.ts:147-157). A block is no wall if
its top is at most feet + stepUp + EPS with headroom (yard.ts:309; EPS = 0.005 m, yard.ts:39).

**But he does not stand on that body.** He stands on a second, smaller flat circle, the foot: FOOT_RADIUS = 0.15 m
(movement.ts:59-60, "a small circle, so he never stands on air past an edge"), `rs = min(0.15, r)` (:163). Standing is
`surfaceAt(x, z, rs, ...)` (:194, yard.ts:214-222): the highest top his foot circle overlaps that is at most a step above
his feet, else the ground at 0.

**Stepping up** (movement.ts:161-192, yard.ts:230-256). Trigger: the BODY circle (r 0.4) overlaps a block whose top is
above the feet and at most feet + stepUp + 0.005 (yard.ts:243-245), with headroom, not a floating block, while moving
towards it (yard.ts:252). Then at once: feet = that top, vertical speed 0, grounded (movement.ts:165-168). Then the pull:
the centre is drawn towards the block's nearest point until it is 0.9 x 0.15 = 0.135 m from it (:169-176), half of what
is left each step (PULL_SHARE 0.5), all of it once less than PULL_MIN 0.03 m (:61-63, :183-192); walking back cancels it
(:179-182). While the pull runs he stands on the body circle (:193-194, `pulling ? r : rs`). From first touch (centre
0.4 m short of the face) the pull covers up to 0.265 m in 4 steps at 30 Hz (0.133, 0.066, 0.033, then the last 0.033),
about 0.13 s, on top of his own walking. In the air the same test runs against the arc's top (movement.ts:123-131,
testFeet): that is how a jump catches a ledge.

**At a step of exactly the step height and just above.** 0.55: top 0.55 <= 0 + 0.55 + 0.005, so the body walks into it
(yard.ts:309: no wall) and steps up: gained. 0.56: 0.56 > 0.555, so it is a wall (yard.ts:309 fails, pushed out to
distance exactly r, :340-341); `overlaps` is strict (< r^2, yard.ts:182), so the step-up never fires: stopped on the
ground. The sandbox's real line is 0.555 m (step-up + 5 mm): anything up to 0.555 is gained. **So with his Export it
gains 0.55 m and stops at 0.56 m.** Ledges: rise = reach - stepUp = 3.071 - 0.55 = 2.521 m (movement.ts:105), so a
ledge is caught up to 2.521 + 0.555 = 3.076 m: 3.05 gained, 3.10 not (the yard's marks hold).

**At an edge.** On a top, he stays up while his 0.15 m foot circle overlaps it: his centre holds up to 0.15 m past the
edge. Then a drop of at most SNAP_DOWN = 0.2 m (movement.ts:65) is snapped (he walks down it); a larger drop starts a
fall from rest (:205-209). So he never stands with more than 0.15 m of his centre over air, and his drawn feet stay
close to the edge.

**On a ramp.** The ramp's height for standing is taken at the footprint's point nearest his centre (`heightNear`,
yard.ts:170-175): with his centre over the ramp that is the height under his centre, so he stands exactly on the slope
(no float). Going down, the snap allowance is 0.2 m + (this step's move + 0.15) x slope under the foot (movement.ts:199;
slope as rise per metre, yard.ts:286-294), so he stays on it at either step rate. Going up: the ramp is a wall only
where its height ahead is more than feet + stepUp, and then he is set back along the slope, not out of its face
(yard.ts:318-334). **There is no steepest-slope limit:** any ramp whose rise over one step's move is at most 0.55 m is
walked (at 30 Hz and 6.3 m/s, 0.21 m a step, that is any grade up to 262 %; slower first steps creep up any grade).
The test scene's roofs (89 % and 95 %) would be walkable under the sandbox's rule.

**Ground.** Implicit: every height search starts at 0 (yard.ts:161, :215), so the ground is a solid plane at y = 0
everywhere; the yard's side is a clamp at 59.5 m (yard.ts:355-356, YARD_HALF - 0.5), applied inside resolveWalls.
