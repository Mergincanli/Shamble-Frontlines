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
0.4 m short of the face) the pull covers up to 0.265 m in 5 steps at 30 Hz (0.133, 0.066, 0.033, 0.017, then the last
0.017 once under PULL_MIN), about 0.17 s, on top of his own walking. The rise itself is instant (:166); the pull is
horizontal only ("spread over a few steps so it does not lurch", :27). In the air the same test runs against the arc's top (movement.ts:123-131,
testFeet): that is how a jump catches a ledge.

**At a step of exactly the step height and just above.** 0.55: top 0.55 <= 0 + 0.55 + 0.005, so the body walks into it
(yard.ts:309: no wall) and steps up: gained. 0.56: 0.56 > 0.555, so it is a wall (yard.ts:309 fails, pushed out to
distance exactly r, :340-341) and the step-up refuses it (yard.ts:245): stopped on the ground. (The refusal is the height test itself, yard.ts:245 in the step-up and :309 in the wall test; the strict
overlap at :182 is incidental.) The sandbox's real line is 0.555 m (step-up + 5 mm): anything up to 0.555 is gained. **So with his Export it
gains 0.55 m and stops at 0.56 m.** Ledges: rise = reach - stepUp = 3.071 - 0.55 = 2.521 m (movement.ts:105), so a
ledge is caught up to 2.521 + 0.555 = 3.076 m: 3.05 gained, 3.10 not (the yard's marks hold).

**At an edge.** On a top, he stays up while his 0.15 m foot circle overlaps it: his centre holds up to 0.15 m past the
edge. Then a drop of at most SNAP_DOWN = 0.2 m (movement.ts:65) is snapped (he walks down it); a larger drop starts a
fall from rest (:205-209). So his centre is never more than 0.15 m over air.

**On a ramp.** The ramp's height for standing is taken at the footprint's point nearest his centre (`heightNear`,
yard.ts:170-175): with his centre over the ramp that is the height under his centre, so he stands exactly on the slope
(no float). Going down, the snap allowance is 0.2 m + (this step's move + 0.15) x slope under the foot (movement.ts:199;
slope as rise per metre, yard.ts:286-294), so he stays on it at either step rate. Going up: the ramp is a wall only
where its height ahead is more than feet + stepUp, and then he is set back along the slope, not out of its face
(yard.ts:318-334). **There is no steepest-slope limit:** any ramp whose rise over one step's move is at most 0.55 m is
walked (at 30 Hz and 6.3 m/s, 0.21 m a step, that is any grade up to about 260 %; slower first steps creep up any grade).
The test scene's roofs (89 % and 95 %) would be walkable under the sandbox's rule.

**Ground.** Implicit: every height search starts at 0 (yard.ts:161, :215), so the ground is a solid plane at y = 0
everywhere; the yard's side is a clamp at 59.5 m (yard.ts:355-356, YARD_HALF - 0.5), applied inside resolveWalls.
- N5 (trace, the sandbox with the pull removed, foot 0.15): first-touch tick, the body has walked up to one tick's move (0.21 m at walk) into the 0.55 step, centre 0.19 to 0.40 m short of the face; feet set to 0.55, grounded (movement.ts:165-168). Then standing on the foot (not pulling, :194): the foot does not reach the top, surf = 0; drop 0.55 > SNAP_DOWN 0.2 + (move + 0.15) x 0 (flat), so :205-209 makes him airborne, vy 0. Next tick he falls 0.5 x 24 x (1/30)^2 = 13.3 mm, walks 0.21 m on, the step-up fires again (top 0.55 within the step of 0.537), grounded again; he stands once his centre is under 0.15 m from the face. So he alternates grounded / airborne for 1 to 2 ticks at walk (more at the slow first steps from rest beside a step; 0 to 1 at sprint 0.32 m/tick), a 13 mm dip each, and a jump pressed in an airborne tick is dropped (jump only from the ground, :104). The pull is what carries the small foot onto the top; it is not there to smooth the rise (the rise is instant, :166). So the question file's row 2 line "The sandbox's pull onto the block is left out: the blend between two ticks already smooths the rise" misreads its role: without the pull, a small foot needs another bridge.
- N6: his words. SLICE-FEEL (DECISIONS.md:2431): "aiming down sights and the sprint-to-fire delay are game rules of Slice 1. I tune both in my Shamble Testlines sandbox ... and the real game must not feel different from it": said of those two rules, not of the body shape. STEP-60 (a) (docs/DECISIONS.md:832): "I play the sandbox at 30 steps and export again." No decision of his names a capsule (DECISIONS.md: no "capsule"); the capsule is the plan's word (plan.md:257) and ADR 0004's PM-0004-35 ("the soldier's capsule or cylinder", adr/0004:1156): both shapes already allowed there. His Export's measured soldier height is 1.8 m (testlines-export.json `measured.soldier_height_m`), the sandbox's BODY_HEIGHT source (main.ts:67-68).
- N7 (corrected after the advisor's check): neither game has a named slope limit, but the old game has an implied one. Its guard samples the surface one radius AHEAD of the new position (pos + u x r) and refuses it if it is more than T.stepUp above the feet (legacy/shamble-game.js:2530-2536), with r = 0.7 (game:2780, the player; :2933, the bots) and stepUp 0.55 (docs/legacy-tuning.json:113). On a ramp the feet were set at the last position, so the sample is (move + 0.7) further up: refused when grade x (move + 0.7) > 0.55, i.e. above 0.55 / 0.7 = 79 % from rest, 0.55 / (0.7 + 0.105) = 68 % at walk (6.3 m/s, one 60 fps frame), 0.55 / (0.7 + 0.17) = 63 % at sprint (6.3 x 1.62 = 10.2 m/s) (arithmetic). So the test scene's roofs (89 %, 95 %) are refused by the old game and walked by the sandbox's code (N3: no radius look-ahead there, heightNear at the centre). The steepest he actually walked in the sandbox: its stair as a hidden slope, 0.18 / 0.28 = 64 % (his Export's stair_rise_m and stair_tread_m; metrics.ts:5, :62), and the yard ramp 2.5 / 9 = 28 % (yard.ts:68).
- N8: rust-reviewer's snap rule (its read, item 9, read whole: "grounded before, not jumping, no ground found in dy: sweep down by step_up; a walkable face snaps; else airborne from rest") is not the sandbox's: the sandbox snaps a drop of at most 0.2 m + (move + 0.15) x slope (movement.ts:65, :199), so on flat ground he FALLS off anything over 0.2 m (a 0.55 m block: a fall of sqrt(2 x 0.55 / 24) = 0.21 s); with a down-sweep of 0.55 a flat walkable floor is found and he snaps to it at once from any block up to the step-up (0.55 m exactly sits on the sweep's end, so at the skin's mercy). A felt difference under MOVE-RULES row 4 and one more NO-SOURCE number (0.2 m).

## Block 2. BODY-SHAPE for him, in plain words (DONE; not a decision)
**The question:** what shape is the soldier's body when he meets steps, edges and ramps? Why it matters: the plan told
him "a vertical capsule" (plan.md:257), and rust-reviewer's prototype shows that shape fails the yard's test (he must
walk up the 0.55 m step and be stopped by the 0.56 m one, ADR 0004's PM-0004-30). No decision of his names a shape
(N6): the capsule is the plan's word, and ADR 0004 already allows "the soldier's capsule or cylinder" (PM-0004-35).

**What he played in Testlines (block 1):** not a capsule. A flat-bottomed upright body 0.4 m in radius meets walls and
starts a step; he stands on a smaller flat circle, 0.15 m in radius, round his centre; on a ramp he stands at the
slope's height under his centre; after a step-up a short pull carries him onto the top.

| | (1) Capsule, as the plan told him | (2) Upright cylinder, 0.4 m, flat foot | (3) Testlines' own body: 0.4 m cylinder plus a 0.15 m foot |
|---|---|---|---|
| Yard step 0.55 / 0.56 m (9 runs each) | fails under all three rules tried: edges landed on: 0.56 GAINED 9 of 9; faces only: 0.55 MISSED 6 of 9 (at walk and sprint); step from the last face: 0.56 left PERCHED on its rounded edge 9 of 9 (feet 0.554 m, centre 8 cm short) and needs a further rule to slide off | gained 9 of 9 / stopped 9 of 9 (rust-reviewer's prototype) | the gain / stop call is made by the 0.4 m body, as in (2) (rust-reviewer's item 7 runs the step-up with body (A); the sandbox triggers on the body circle, yard.ts:243), so it inherits (2)'s 9 of 9 and 9 of 9; NOT RUN: the bridge onto the top (foot and pull), the ledges and the ramps. In the sandbox, by reading: gained / stopped (block 1) |
| Ledges 3.05 / 3.10 m | untested (prototype has no vertical motion) | untested | sandbox by reading: caught / not; ours untested |
| How far past an edge he stands | not measured | 0.4 m: his centre 0.4 m over air; the stand-in is 0.73 m wide over the hands (params.ts:575), so his whole drawn figure can stand beyond the edge | 0.15 m, as in Testlines |
| Floating over a ramp (gentle 25 %, steep 50 %; roofs 89 %, 95 %) | not measured | 0.10 / 0.20 m; roofs 0.36 / 0.38 m (radius x grade) | 0 if he stands at the height under his centre, as Testlines does; 0.04 / 0.08 m (roofs 0.13 / 0.14) if the 0.15 m foot is swept as a flat disc |
| New numbers not in his Export | none for the shape; a perch rule | none | the foot radius 0.15 m (movement.ts:60); with (3a) the pull's three: 0.5 a tick, done under 0.03 m, to 0.9 of the foot (movement.ts:61-63, :172) |
| START-RULES (reach 3.07 m; soft push) | reach untested in our sweeps | reach untested in our sweeps | reach: sandbox by reading, ours untested. Push: the same under every body, it uses feet heights (push.ts:43) and the 0.4 m body circle against walls (push.ts:112) |
| "Must not feel different" | his SLICE-FEEL says it of aiming down sights and the sprint-to-fire delay (DECISIONS.md:2431), not of the body; in spirit (he tuned on Testlines' body) (1) is furthest | in spirit: differs at edges and ramps | in spirit: the same as Testlines |
| MOVE-RULES row 2 as told to him ("rises onto a step as soon as his body touches it, and holds a little past an edge (the body's round foot rests on it)") | keeps "round foot" literally; fails the step test | keeps "as soon as his body touches it"; changes "a little past an edge" into 0.4 m | keeps both; "round foot" reads as round in plan view (a flat circle), which is what the code has |

**The bridge in (3).** With a small foot, something must carry him from "his body touched the step" to "his foot is on
it": without the sandbox's pull he flickers between standing and falling for 1 to 2 ticks at walk, dipping 13 mm each
tick, and a jump pressed in such a tick is lost (N5). Row 2 as told to him says the pull is left out ("the blend between
two ticks already smooths the rise"): that holds for a body that stands on its full width, as in (2), and breaks only
with the 0.15 m foot (the rise is instant in Testlines too, movement.ts:166; the pull is the forward carry). Two ways:
- **(3a) the pull, as Testlines has it:** a short forward glide of up to 0.265 m over 5 ticks (about 0.17 s) onto the
  top, on top of his walk; walking back cancels it. Exactly what he played. Row 2's "pull left out" is withdrawn, and
  three numbers join NO-SOURCE.
- **(3b) no pull: he stands on his full body circle on the top he just stepped onto, until his foot reaches it.** No new
  number and no glide; he walks onto the step at his own speed. Differs from Testlines: if he stops right after
  touching, he stands on the step with his centre up to 0.4 m short of it (on the step side only).

**What he would feel, side by side.** (1): untried in feel; stands on rounded edges and needs rules not yet found.
(2): the same steps and walls as Testlines, but he can stand far out over an edge and visibly hovers on ramps and roofs.
(3a): the same as Testlines. (3b): the same as Testlines except for a slightly slower, smoother mount onto a step.

**Recommendation (his to decide): (3a), conditional.** It is the body he tuned his values on, it keeps every word of row
2 except the pull line (which fits (2), not a small foot), and it has no float or overhang to explain. Condition:
rust-reviewer re-runs its prototype (`rr-sweep-proto.rs`) with the 0.15 m foot and the pull (the step call itself is
(2)'s, already 9 of 9), adding vertical motion for
the 3.05 / 3.10 m ledges and the 25 % / 50 % ramps, several runs per case; it passes only if 0.55 is gained, 0.56
stopped, 3.05 caught and 3.10 not, in every run. **Fallback: (2)**, told to him with its 0.4 m overhang and its 0.10 /
0.20 m float before his Brave check. Not recommended: (1), which has failed every rule tried.

**Related points for the same question (not body shape, but the same rows):**
- The snap down (N8): Testlines falls off anything over 0.2 m on flat ground; a snap of the full step-up (rust-reviewer
  item 9) would drop him to the ground at once from a 0.55 m block. Recommended: Testlines' 0.2 m rule, one more
  NO-SOURCE number.
- Testlines gains a step up to 5 mm above the step-up (EPS, yard.ts:39), so its true reach is 3.076 m, not 3.071.
  PM-0004-30's 0.55 / 0.56 m marks hold either way; a ledge between 3.072 and 3.076 m would differ. Recommended: no
  allowance in the game (his numbers exactly), stated to him.

## Block 3. The soft push: formulas and limits, from the sandbox's code (DONE)
Commit be608cad (unchanged at 703fd922), `src/push.ts` unless marked. Called once per tick after every soldier has
moved (main.ts:366-376) with radius = pushRadius (his Export 0.4 m), strength = pushStrength (8 per s), dt = 1/30 s
(his Export is at 30: STEP-60 (a), "I play the sandbox at 30 steps and export again", docs/DECISIONS.md:832), wallRadius =
bodyRadius 0.4 m, stepUp 0.55 m. **rust-reviewer's worry about a 60 Hz sandbox (its item 9) does not apply: his Export
is at 30, the game's rate, so the per-tick values carry over as they are.** The push moves positions only; no
velocity is changed.

**The four named limits**
| Name in code | Value | Unit | What it does | Line |
|---|---|---|---|---|
| `HEIGHT_GAP` | 1.0 | m (feet heights) | two soldiers whose feet differ by 1.0 m or more do not push | :10-11, :43 |
| `SIDEWAYS` | 0.7 | share of the push (no unit) | how far a head-on push turns sideways | :12-13, :71-72 |
| `KEEP_GOING` | 0.5 | share of his own step this tick | a push never undoes more than half of his own step forward | :14-15, :100 |
| `MAX_STEP` | 0.25 | m per tick (= 7.5 m/s at 30 Hz; it would be 15 m/s at 60) | the most one tick's push may move him | :16-17, :106-110 |

**The formulas, in order**
1. Off if strength <= 0 or radius <= 0 (:30).
2. Share per tick: `share = 1 - exp(-strength x dt)` (:31) = 1 - e^(-8/30) = 1 - 0.76593 = **0.23407**. Exponential, so
   the per-second feel is the same at any rate: an overlap closes by 23.4 % a tick, half gone in ln 2 / 8 = 0.087 s,
   90 % gone in ln 10 / 8 = 0.29 s (arithmetic).
3. Contact distance `min = 2 x radius` = 0.8 m (:32). Every pair i < j of active bodies (:37-42), skipped if the height
   gap is reached (:43) or if `d^2 >= min^2` (:47).
4. Direction `n = (b - a) / dist`; if `dist < 1e-6` (exactly on top of each other) a fixed angle per pair,
   `(i x 7 + j x 13) x 0.7` rad, as `(cos, sin)` (:51-55): **trigonometry in the step**, against plan 6.1's "No
   trigonometry runs in the step" (a point for rust-reviewer: the game needs another fixed tie rule, for example by id
   order along one axis).
5. Head-on turn, only if the relative speed `|va - vb| > 0.2 m/s` (:62-65): `headOn = |n . (va - vb)| / |va - vb|`
   (0 to 1, :66); side from the sign of `n x (va - vb)` (:67-68; exactly in line the cross is 0 and the side is +1, a
   fixed side); `n += t x SIDEWAYS x headOn` with t the perpendicular on that side, then n made unit length again
   (:69-75).
6. Each body moves `move = (min - dist) x share x 0.5` along -n and +n (:78-84); with one body fixed (the ridden Hornet)
   the other moves `2 x move` (:79-80); both fixed: no push (:77). Sums over all pairs are added per body (:81-84).
   Largest for one pair: 0.8 x 0.23407 x 0.5 = 0.094 m a tick (0.187 m against a fixed body), so MAX_STEP (0.25 m)
   bites only where several pushes add up (a crowd) or against the Hornet with two contacts.
7. Keep going, only if his own speed `> 0.1 m/s` (:95-96): `along = push . (v / |v|)`; `limit = -KEEP_GOING x |v| x dt`
   (:99-100); if `along < limit`, the excess backwards part is removed along his direction (:101-104).
8. Cap: if the push is longer than MAX_STEP it is scaled to MAX_STEP (:106-110).
9. Walls: the pushed spot goes through `resolveWalls(x + mx, z + mz, wallRadius, y, stepUp)` (:112; yard.ts:300-357),
   the sandbox's push-out of the body circle (not a sweep: the plan's "swept against the world" is the game's own rule).

**Unnamed numbers beside the four** (code constants, no name of their own): the head-on gate 0.2 m/s (:65), the
keep-going gate 0.1 m/s (:96), the coincidence test 1e-6 m (:51) and its angle rule (7, 13, 0.7 rad, :53). If
NO-SOURCE (b) is taken, the two gates are speeds the game must also hold (as content rows or as stated code rules).

**The steepest slope he may walk:** none in the sandbox (block 1, N3; nor in the old game, N7). The only slope unit in
the sandbox's code is **rise per metre**, a plain fraction (a ramp's `top / (high - low)`, yard.ts:124, :264, :291;
`slopeUnder`, yard.ts:286-294). A degree figure appears only as text in the panel's stair note (params.ts:405, an
`atan` for display).

## Block 4. GROUND and the slope's unit: what the rules and his words imply (DONE)
**GROUND.** No word of his and no rule row speaks of the floor. MOVE-RULES row 8 (as told to him) covers only the
sides: "the map's edge is the map's own bounds, and the speed into it is dropped". What both games he has played do:
the sandbox's ground is a solid plane at height 0 under everything (every height search starts at 0, yard.ts:161,
:215; its side a clamp at 59.5 m, yard.ts:355-356); the old game's too (`surfaceHeightAt` starts at h = 0,
legacy/shamble-game.js:2528). And the question STREET-GREY tells him "both ground kinds lie at height 0". So the rules
imply that he stands on solid ground at 0 wherever the map has no block, and never falls through it. Nothing implies
holes or pits. How the map holds that (the bounds' floor, or a ground shape) is engine-architect's; one point for
either form: a soldier must not be able to fall forever if a map lacks it (a content check or a floor that always
exists).

**The slope's unit and the limit itself.**
- Neither game has a named steepest slope (N3, N7), so "the steepest slope he can walk" (question file, NO-SOURCE) is
  not a sandbox number to take. The two games differ: by the sandbox's code a ramp is walked wherever it rises no more
  than the step-up within one tick's move (yard.ts:308-309, :318-334: about 260 % at walk); the old game samples one
  radius (0.7 m) ahead and so refuses anything above roughly 63 to 79 % (N7; legacy/shamble-game.js:2530-2536). **That
  is a sandbox-versus-old-game difference he has not been shown: it belongs with MOVE-RULES (a new row), not with
  NO-SOURCE.** Consequence for the test scene: by the sandbox's code he walks up the roofs (89 % and 95 %) after
  gaining their 2.6 m edges; by the old game's rule he cannot; a grade limit (a new number) sits between. Not "as he
  played": the steepest slope he walked in the sandbox was its stair, 64 %, and its yard ramp is 28 % (N7).
- The unit: the sandbox's code uses **rise per metre**, a plain fraction (block 3, last paragraph). A grade (rise per
  metre, or the same x 100 as a percent) is therefore the sandbox's own unit and needs no new maths routine; degrees
  appear in the sandbox only as panel text. Nothing in his words asks for degrees.
- If he keeps "no limit", the game still needs a way to tell a floor from a wall in its sweeps; the sandbox's way (rise
  per tick at most the step-up) carries no number but depends on speed. That is engineering, for rust-reviewer and
  engine-architect, within his answer.

## Corrections to the question file (question-2026-11-28-before-4c.md), for the main session (not decisions)
1. MOVE-RULES row 2: "the body's round foot rests on it" describes a capsule; the sandbox's foot is a FLAT circle of
   0.15 m round his centre (movement.ts:59-60, :163, :194), under a flat-bottomed 0.4 m body. Suggested words: "he
   stands on a small circle round his centre (0.15 m), so he holds a little past an edge".
2. MOVE-RULES row 2: "The sandbox's pull onto the block is left out: the blend between two ticks already smooths the
   rise." True for a body that stands on its full width (BODY-SHAPE (2)); with the sandbox's 0.15 m foot it breaks:
   the rise is instant in the sandbox (movement.ts:166) and the pull is the forward carry onto the top (N5). Without it, or another bridge (block 2, 3b), he flickers grounded / airborne for 1 to 2
   ticks after each step-up and can lose a jump press.
3. NO-SOURCE: "the steepest slope he can walk" is not a sandbox number: neither game names one (block 4). The sandbox
   walks any grade a tick's step allows; the old game refuses above about 63 to 79 % (its 0.7 m look-ahead): a
   MOVE-RULES difference (the roofs, 89 % and 95 %), not a NO-SOURCE row.
4. NO-SOURCE lacks numbers the sandbox's rules use: the foot radius 0.15 m (movement.ts:60); the pull's 0.5 a tick,
   0.03 m and 0.9 (movement.ts:61-63, :172); the snap-down 0.2 m (movement.ts:65, :199; and N8: rust-reviewer's snap
   by the full step-up differs); the 5 mm step allowance (yard.ts:39); the push's two speed gates 0.2 and 0.1 m/s
   (push.ts:65, :96). My block 2 of the walk check (game-designer-cs4.md:95) named the pull, the foot and the snap.
5. NO-SOURCE's push limits, with values for its option (b) start: height gap 1.0 m, sideways 0.7, keep-going 0.5, most
   per tick 0.25 m (= 7.5 m/s at 30 Hz). The body height: his Export's measured 1.8 m (`measured.soldier_height_m`).

DONE

## Block 5. An unjumped fall that meets a higher top; and the head bump (from my notes only, no new read of Testlines)
### Plan
1. My notes (this file; `.checkpoints/cs4-walk/game-designer-cs4.md`; no other game-designer notes or agent memory hold `132-138`, `testFeet`, "unjumped" or "fall from rest": grep of the repository's `.checkpoints/` and `.claude/`) on whether the in-air step-up runs on a fall from rest.
2. PM-0004-30's words (adr/0004:1151) and the yard (test-scene.json:194-219) against both readings; what each means for play, with the arithmetic.
3. The old game's side (legacy/shamble-game.js:2531-2540, read in this repository, not run).
4. A question for him if the notes do not settle it.
5. (Main session's added point) what F18's head bump does by my notes, and whether it is rust-reviewer's CEILING C2.
### Notes
- N9: what my notes say. Block 1 line 39: "In the air the same test runs against the arc's top (movement.ts:123-131, testFeet): that is how a jump catches a ledge." Walk check F18 (game-designer-cs4.md:29): "the wall/ledge test uses the arc's top when it falls between steps (:121-139)"; "steps up onto a block the body touches while moving toward it, in the air too, then pulls the feet on over a few steps (:161-192)"; "walking off an edge starts the fall from rest (:205-209)". N5 (line 64) is a by-reading trace (not run) of a variant with the pull removed, in which a tick that begins airborne after a :205-209 fall from rest has the step-up (:161-192) fire, against the same 0.55 top he had just stood on, from feet 0.537. So my notes do contain one unjumped airborne tick in which the step-up fires, but on a hypothetical variant, onto the top he just left, and from feet after the tick's fall. They LEAN "a fall steps up too"; none of them traces a fall from one top onto another, and none cites a line that limits the in-air test to a jump or extends it to a fall. **Not settled: his.**
- N10: which feet. My notes carry three readings: the arc's top (block 1 line 39; F18 "when it falls between steps"), the feet after the tick's fall (N5's trace), and rust-reviewer's stand-in, the tick's start. A fall from rest has its arc's top at the tick it starts, so the first and third agree for a fall; the second is lower by one tick's drop (at most 0.653 - 0.48 = 0.17 m here). For the yard's case all three catch the 0.56 top while feet are above 0.005 m. My notes support the arc's top (= the tick's start for a fall) as the sandbox's rule for a jump; N5's feet are my trace's simplification, not a cited line.
- N11: PM-0004-30 as worded (adr/0004:1151): "a soldier steps up 0.55 m and is stopped by 0.56 m". A rise is measured from his feet: from the 0.55 top the 0.56 step is a rise of 0.01 m, and from the ground it is still refused (rust-reviewer, 9 of 9). So gaining it after a walk-off does not break the mark's words. It does break the yard's 0.56 step as a limit met on a walk across both steps (`yard_step_055` x 30-32, `yard_step_056` x 34-36, both z -10 to -8, ground between; the marker `yard_steps` at (27, -9) faces 90 deg, which is +x by map.rs's convention as REPO_MAP:44 states it, so the straight walk from the marker meets 0.55 first, then 0.56 from above). Under either answer the 0.56 run must start on the ground beside the step (from any side), and say so.
- N12 (arithmetic, not run; under reading (a)): he falls once his foot leaves (centre 0.15 m past the edge, up to 0.31 m at the tick's grain: rust-reviewer block 6 row 593) and is caught once his body (0.4 m) touches the higher top, if his test feet are still within 0.555 m of its top: fall allowed = 0.555 - rise; time t = sqrt(2 x (0.555 - rise) / 24). Widest gap caught = v x t + 0.4 + (0.15 to 0.31) + up to one tick's move (tick-start test). For the yard (rise 0.01, t = 0.213 s): 6.3 m/s: 1.34 + 0.55 to 0.71 + up to 0.21 = 1.9 to 2.3 m, so the 2 m gap sits on the margin at walk (in the tick model: 7 ticks x 0.21 = 1.47 m against 1.29 to 1.45 m to cover, caught at tick 7 from feet 0.07 m); 13.27 m/s: 2.83 + 0.55 to 0.71 + up to 0.44 = 3.4 to 4.0 m. For a top 0.3 m above the one left (t = 0.146 s): at 6.3 m/s about 1.5 to 1.7 m. Play meaning: a forgiving catch whose width grows with speed (sprinting off a top, he lands on a slightly higher one up to about 4 m away), never more than 0.555 m above the top he left, so no height a jump (3.07 m reach) does not already give. A stair of slightly rising tops with gaps under these widths is climbed without one jump. Under (b) every such gap needs a jump and a walk-off always lands below.
- N13 (the old game, legacy/shamble-game.js:2531-2540, by reading, not run): `tickVertical` runs its step-up guard every tick, grounded or not, from the feet at the tick's start (`foot=pos.y`, `up=foot+T.stepUp`, sampled one radius ahead), then lands him on any surface under his centre that his feet have reached (`if(pos.y<=surf){pos.y=surf;...}`); an airborne soldier whose feet are within 0.55 m of a top is let in and set on it. So the old game is (a) as well, with its centre-point surface: the catch needs his centre over the top, so the yard's 2 m gap is caught only near a sprint (2 m / 10.2 m/s = 0.196 s, fall 0.46 m, feet 0.09 m: caught; at walk 0.317 s, he is on the ground first, then refused). (b) would be new to both games.
- N14 (head bump): F18 (game-designer-cs4.md:29) says only "head bumps a floating block (:132-138)". My notes do NOT say what the bump does (upward speed stopped, reversed, or the soldier moved down), nor which head height it uses; block 1 names headroom and "not a floating block" only inside the wall and step tests (yard.ts:309, :243-252), which is not the bump. So the sandbox HAS a head rule by my notes, against rust-reviewer's "by its reading has no head rule" (rust-reviewer-read.md:671-672): its C1 must not be called "as the sandbox". Whether the bump is C2 ("the head stops at a shape's bottom and he falls back") my notes cannot say; "bumps" fits C2 or a reversal. MOVE-RULES row 8 already tells him "the head bumps a block above" as a sandbox rule (question file :20-21), so under his (a) the game builds the sandbox's bump; its exact effect is his to state, or a re-read his to allow.
- N15: every pass of rust-reviewer's blocks 5 and 6 ran with the fall stepping up (row 597: "the run assumes it does"); its line 625 ("no answer ... forces a re-run") covers BODY-SHAPE, not this. Option (b) has not been run: under (b) the landing and walk-off cases (rows 593-597) need a re-run.

### Answer (VERDICT: ESCALATE on the rule; nothing else blocks)
1. My notes lean "an unjumped fall steps up too" (F18 "in the air too"; N5's trace, a variant onto the top just left), but none settles a fall from one top onto another (N9): his. The old game does step up on a fall, from the feet at the tick's start (N13).
2. It does not break PM-0004-30 as worded (a rise from the feet; from the ground 0.56 is still refused) but it does break the yard's 0.56 step as a limit met on the marker's straight walk from the 0.55 top: the mark's run must start on the ground beside the step, under either answer (N11). Play: (a) a forgiving catch on gaps, about 2 m wide at walk and 4 m at sprint, never above 0.555 m over the top left; (b) every gap needs a jump (N12).
3. Question FALL-STEP below. 4. Head bump: a head rule exists by F18 (movement.ts:132-138); its effect is not in my notes; not confirmed as C2 (N14).

**FALL-STEP: when the soldier walks off a block without jumping and, while falling, meets a block a little higher than the one he left, does he land on top of it?**
Why it matters: the yard's test steps are a 0.55 m step (the highest he may walk up) and a 0.56 m one (the first he may not) 2 m apart; walking off the first, he lands on the second at every speed tried in the prototype, so the 0.56 m step only stops him when he walks up to it from the ground.
- (a) Yes, as at the top of a jump: any block at most 0.55 m above his feet as he falls, so never more than 0.55 m above the block he left. The old game does the same (its catch needs his centre over the block, so it is narrower). The movement test checks the 0.56 m step from the ground, and says so.
- (b) No: only a jump catches a block in the air; walking off always lands him on whatever is below. One more condition in the step-up, no new number; new to both games; the prototype's runs must be repeated under it.
- (c) As his Testlines does: he walks off the 0.55 m step toward the 0.56 m one without jumping, at a walk and at a sprint, and says whether he lands on it (or allows his Testlines repository to be read again for this line).
Recommended: (c) to settle it (one try in Testlines), then build what he saw; if he would rather not, (a): my notes' plain reading of his sandbox is "in the air too", the old game does it, the prototype already passes under it, and it never lifts him higher than he could walk up from where he came. Under any answer the 0.56 m test starts on the ground.

DONE

## Block 6. Testlines read again, by his leave (TESTLINES yes; FALL-STEP (c) by reading; his answers 2026-11-28, items 13 and 15)
### Plan
1. Fetch the branch claude/sleepy-edison-ygmwfe by its public address into FETCH_HEAD (nothing pushed) to see whether it moved past 703fd922; read with git show at be608cad (his Export's commit) and at the tip if it moved. No code copied: rules in plain words and numbers with file:line.
2. FALL-STEP: movement.ts's air path (the in-air step-up, the walk-off fall) for whether a fall that began without a jump steps up.
3. The head bump (movement.ts:132-138): upward speed, position, circle, height; the headroom rule.
4. A ramp met from its side (yard.ts:318-334).
5. Every NO-SOURCE (b) number with file:line at be608cad, changed or not at the tip.
6. Compare the rest of the movement with rust-reviewer-read.md blocks 1, 5-8 and reread-sheet.md E1 and I2; findings with severity.
### Notes
- N16: fetched the branch by its public address into FETCH_HEAD (nothing pushed): still 703fd922 (has not moved since block 1). Correction to N1: `git diff --stat be608cad 703fd922` lists 13 files (README, play/testlines.html, audio, dummies, exporter, fx, look, main, metrics, params, sight, standins, tools/browser-check), not only exporter and params; movement.ts, yard.ts and push.ts are unchanged, and the diffs of main, dummies, params and metrics add only the vision rule, the dummies' sounds, two test dummies' loops and a list of the stair's steps for the sight test (no movement parameter's key or start value changes: grep of the diff for `key: '`). So every movement line below holds at both commits. All lines below are at be608cad.
- N17 (FALL-STEP, settled by the code): the step-up (movement.ts:161-177) runs every tick for every body, grounded or not, with no test of a jump: its one condition is that a target was found and its top is above the feet after this tick's arc (:165). The target search (yard.ts:230-256) takes position, radius, feet, step-up, horizontal speed and height only; nothing in it reads the vertical speed or the grounded flag. A walk-off sets airborne with vertical speed 0 (movement.ts:205-209); the next ticks fall (:126-131) and still run the step-up. Which feet: for a fall from rest the test feet are the feet AFTER this tick's fall (:131: the arc's-top substitute is used only when the rise turns to a fall within the tick, vy0 > 0 and vy <= 0), not the tick's start: this corrects N10's "arc's top = the tick's start for a fall". So: **yes, an unjumped fall steps up onto a top within 0.555 m above his feet at the tick's end (top <= feet + 0.55 + 0.005, yard.ts:245, EPS yard.ts:39), touched by the 0.4 m body circle (yard.ts:243, strict overlap :182), moving towards it (:252), not from under a floating block and with headroom (:248).** That is FALL-STEP (a), the old game's behaviour too (N13).
- N18 (the head bump, movement.ts:126-139, yard.ts:196-208): it runs only while airborne (:126). After this tick's arc, the lowest underside among floating blocks (underside above 0) that lies no lower than his head at the tick's START (feet at the start + body height, less 1 mm: yard.ts:204) and overlaps his BODY circle (0.4 m, less 1e-6: yard.ts:205), tested at his position at the tick's start (the horizontal move comes after, :142-146), gives a ceiling for the feet = underside - body height (:133; height = BODY_HEIGHT, his Export's measured 1.8 m, main.ts:67-68). If his feet passed it, they are set back down to it (the position is moved, so the head sits exactly at the underside, :134-135); the upward speed is set to 0 (stopped, not reversed; a downward speed is kept, :136); the wall and step-up test feet are capped there too (:138). Gravity then starts his fall from rest the next tick. No bump while grounded: walking under a block whose underside is below his head is a wall (yard.ts:306-309, a floating block is never stepped onto from below, :309 and :248). New numbers: the 1 mm head allowance (yard.ts:204) and the 1e-6 shrink (code tolerances, not tuning).
- N19 (the headroom rule, yard.ts:185-194): a top counts as reachable only if no floating block's underside lies strictly between that top and top + body height (1.8 m), with 1e-6 margins, overlapping the body circle (0.4 m, less 1e-6). Used by the step-up (yard.ts:248: no step onto a top without headroom, nor onto a floating block from below it) and by the wall test (:309: such a top is a wall). The standing test (surfaceAt, yard.ts:214-222) does not check headroom.
- N20 (item 4, a ramp met from its side; yard.ts:300-357 at be608cad, read whole): the wall pass treats a ramp like any block first: no wall if its height at the footprint point nearest his centre (heightNear, yard.ts:170-175, :308) is at most feet + 0.55 + 0.005 with headroom and no underside above his feet (:309). Otherwise, if his CENTRE lies within the slope's width (the across coordinate between the ramp's two side edges, :323-324), he is set back ALONG the slope to the point where the slope's height is feet + 0.55 (t = (feet + 0.55) / top clamped to 0..1, c = low + t x length, no 5 mm: :325-326), only if he is further up than that point (:328-332), and never pushed sideways (:333 continue). If his centre is outside the width (met from its side), the ordinary push-out applies: out to 0.4 m from the side edge's nearest point, round at the corners (:336-341), and the speed into it is removed (movement.ts:147-157). So met from its side the ramp is a step where low and a wall where high: the line is the slope's height at the side point nearest his centre against feet + 0.555 (from the ground on the yard ramp, 2.5 m over 9 m, the first 0.555 / (2.5 / 9) = 2.0 m of its length is stepped onto from the side, the rest is a wall). Stepped onto from the side, he rises at once to that height (yard.ts:243-253) and the pull draws his centre to 0.135 m inside the side edge (movement.ts:169-176). The side test reads one point (nearest his centre), not the highest point under his 0.4 m body circle (up to 0.4 x grade higher: 0.11 m on the yard ramp). Within the width, the set-back caps the climb at 0.55 m a tick (he then steps up to it, :243-253): a cap, not a slope limit (N3 holds).
- N21 (item 4, the set-back's reach, by reading, not run): the set-back (within the slope's width) never fires in the test scene: the largest climb in one tick is 13.27 / 30 x 0.95 = 0.42 m on house a's roof (under 0.555), and no fall there reaches the 0.555 m a tick it needs (about 16.2 m/s, a 5.5 m fall onto a slope). On a made map it (i) caps the climb at 0.55 m a tick, and (ii) catches a fast fall onto a slope by moving him DOWN the slope to the point at feet + 0.55 (then the step-up, centre already over it, puts him there): no miss. (iii) A centre beyond a ramp's EXPOSED high end, within its width, met from outside (on the ground behind a freestanding ramp), also passes :328 and is set back THROUGH the ramp to the point at feet + 0.55: on a yard-sized ramp (2.5 m over 9 m) from z -22.9 to -15.48, 7.4 m in one tick. The sandbox never meets it: the yard ramp's high end abuts the 2.5 m platform (yard.ts:68-69), the stair's slope ends inside its landing (metrics.ts:71-75), and in the test scene every high end abuts a platform (x 45) or the other roof half (the ridges). The sandbox's ramps all rise from height 0 (rampHeight = t x top, yard.ts:74-79; the set-back's t = (feet + 0.55) / top, :325); the test scene's roofs rise from 2.6 m (test-scene.json:106-133), so any formula taken over needs the slope's low height. The sandbox has floating blocks after all: the walls over the metrics room's door and window (metrics.ts:45-56, bottom = sill + opening height), so the head bump and headroom are rules he could meet there.
- N22 (item 5, every NO-SOURCE (b) number at be608cad; all code constants, no slider): TUNING (felt): foot radius 0.15 m (movement.ts:59-60; used :163, :194, :199); the pull's share 0.5 of what is left a STEP (PULL_SHARE, :61-62; a half time of 1/30 s only at 30 steps), done under 0.03 m (PULL_MIN, :63, :186), to 0.9 of the foot = 0.135 m from the block's nearest point (:172); the snap down 0.2 m (SNAP_DOWN, :64-65) plus (the tick's whole horizontal displacement, walls and pull included, + 0.15) x the steepest ramp under the foot circle in rise per metre (:199; slopeUnder yard.ts:286-294, any height); the 5 mm allowance (EPS, yard.ts:38-39; used in standing :219, the step-up :245, the slopes underfoot :265 and the wall test :309, NOT in the ramp's set-back :325); the push's HEIGHT_GAP 1.0 m (push.ts:10-11, test ">=" at :43), SIDEWAYS 0.7 (:12-13, :71-72), KEEP_GOING 0.5 (:14-15, :100), MAX_STEP 0.25 m a step (:16-17, :106-110), the head-on gate 0.2 m/s of relative speed (:65, ">") and the keep-going gate 0.1 m/s of own speed (:96, ">"). TOLERANCES (not tuning): the 1 mm head allowance (yard.ts:204: an underside up to 1 mm under the head at the tick's start still counts as a ceiling); the 1e-6 shrinks of the body circle in headroom (:191) and the ceiling (:205); the headroom band's 1e-6 margins (:189); "above the feet" 1e-6 (:245); "bottom no higher than the feet" 1e-6 (:248, :309); "wholly above the head" 1e-6 (:306); the push's coincidence 1e-6 m (push.ts:51); the slide's 1e-9 (movement.ts:151) and the wall's 1e-12 (yard.ts:336). Numbers NOT on his list that the sandbox's movement also uses: the slope-head rule's 1 cm level match (yard.ts:280) and its slope "underfoot" from 5 cm below the feet plus the slope's rise across the body (:264-265), both tolerances of that rule (N23, G4); the jump's minimum rise 0.01 m (movement.ts:105, a guard: Rules::take refuses a reach under the step instead, E1).
