# The movement rules and SLICE-FEEL for change-set 4's plan (the main session's draft, for game-designer's check)

_Written by the main session, 2026-11-25. Sources: the old game's code (`legacy/shamble-game.js`,
`legacy/shamble-core.js`, `docs/legacy-tuning.json`); his words (START-RULES in the build plan, line 34; SLICE-FEEL in
`docs/DECISIONS.md`, 2026-11-19, 13:36:37 UTC; point 8 of `mustafa-2026-11-25-takeover.md`); `docs/LOOP_V2.md` section
7; and the behaviour of his Testlines sandbox as its README describes it (read-only copy at commit be608cad; looked at,
no code copied, no value taken). game-designer checks every line of this file against those sources and corrects it
in its own notes (`game-designer-cs4.md`); design decisions stay Mustafa's._

## 1. The old game's movement, rule by rule
| # | Rule | Where in the old game |
|---|---|---|
| R1 | The move direction comes from W, A, S, D relative to the camera, made one unit long | `shamble-game.js:2762-2768` |
| R2 | Sprint: Shift held while moving, not while aiming down sights, not with the shield up | `:2770` |
| R3 | Stamina: sprinting drains it in `STAM.max` seconds; otherwise it refills at `STAM.regen` a second; run dry, sprint is locked until it is back at `STAM.lockUntil` | `:2771-2773`; `STAM` at `:2216` (max 7, regen 0.2, lockUntil 0.3) |
| R4 | Speed: `T.moveSpeed` (6.3) times the unit's sprint multiplier when sprinting (the Conscript: 1.214), times `T.adsMoveMult` (0.55) when aiming down sights | `:2774`; `shamble-core.js:67, :76, :128` |
| R5 | The velocity eases toward the wanted velocity by the share `1 - exp(-T.accel * dt)` each frame (`T.accel` 14), in the air as on the ground | `:2775` |
| R6 | The position moves by velocity times `dt`, is held inside the arena, then pushed out of obstacles along the nearest side with a body radius of 0.7 m (the velocity is not changed) | `:2776, :2779-2780`; `resolveObstacles` at `:2526` |
| R7 | Step-up: he may not move onto a surface more than `T.stepUp` (0.55 m) above his feet; the blocked part of the move is taken back per axis (a slide) | `tickVertical`, `:2533-2536` |
| R8 | Jump: while Space is held and he is on the ground, his upward speed becomes `T.jumpV` (11); held Space jumps again on every landing | `:2537`, called with `keys['Space']` at `:2780` |
| R9 | Gravity `T.gravity` (24) every frame, speed first and then height; he lands on the highest surface under him | `:2538-2540` |
| R10 | The highest ledge a jump gains: `jumpV² / (2 × gravity) + stepUp`, about 3.07 m (`EJUMP_REACH`) | `:1575` |
| R11 | Soldiers pass through each other | no rule in the code; the Testlines README says the same |

His decisions on top of the old game, already taken: the jump's reach is the formula's value, 3.07 m, and two
soldiers meeting push each other softly and never block (START-RULES (b), 2026-11-17: "Both are numbers and rules I
can change after I have felt them in Brave").

## 2. Where his sandbox's movement differs from the old game (his point 8e: none enters without his word)
His SLICE-FEEL says of aiming down sights and the sprint-to-fire delay that "the real game must not feel different
from" the sandbox; his point 8e says no sandbox rule that differs from the old game enters a change-set without his
word in that change-set's plan. So each difference that touches Walk is listed here and asked in the plan (MOVE-RULES):
| # | In the sandbox | In the old game | What Walk would build, and why |
|---|---|---|---|
| D1 | The body is 0.4 m in radius against walls and ledges (a slider with no old number) | 0.7 m against obstacles | The Export's value: it is in the table (his tuning). Asked, because it changes every doorway and gap. |
| D2 | The jump follows its exact arc within each step, so its height is the same at 30 or 60 steps a second | speed, then height, once a frame (R9) | The exact arc: at 30 steps a second the old way tops out about 0.18 m lower than the formula's 3.07 m (11 × 1/30 ÷ 2), and START-RULES names the formula's value. |
| D3 | A ledge is gained in the air too, when the arc's top reaches its height less the step-up; the feet are then pulled onto the block over a few steps | the same reach (R7 with R9); no pull | The same reach. The pull is the sandbox's own smoothing: whether the simulation needs it or the picture's blend between two ticks already gives it is for the movement design, and it is asked. |
| D4 | Against a wall he slides along it and loses the speed that went into it | pushed out along the nearest side; the speed stays (R6) | The slide: our own movement on the collision library is a slide-and-step move (ADR 0004 part 10). |
| D5 | A stair is walked as a slope; going down a slope he stays on it | surfaces and ramps looked up under him | Slopes through the collision library's queries; the training yard has ramps (PM-0004-54). |
| D6 | His head bumps a block above him (a lintel) | no such rule: the old world has no blocks to walk under | Follows from colliding in three dimensions; named so he knows it. |
| D7 | Speeding up and stopping are two values (both start at the old game's one) | one value, `T.accel` | Two values in the movement table. |
| D8 | Steering in the air is a value (starts at full, as the old game) | full | The value in the movement table. |
| D9 | Sprint also ends while firing or while a shot waits for the gun to come up | aiming and the shield end it (R2); firing does not | With the guns, in change-set 8; aiming ends it from change-set 4. |
| D10 | Soft pushing between soldiers, with a strength and a radius | none (R11) | His START-RULES; the two values come from the Export. Felt from change-set 5, when other soldiers walk. |
Not a difference: held Space jumps again on every landing in both (R8); a tap between two steps is not lost in the
sandbox, which the game's press counters also give.

## 3. SLICE-FEEL priced and placed (his words: "the input, the rule, the fields, what lands in change-set 8, and the lines they add")
**Aiming down sights.** His words: "I hold a key to aim. Start from the old game's rules as Testlines has them: the
soldier walks at 0.55 of his speed and cannot sprint, each gun's spread rests at its sights value, and the camera
reacts (the sights mode that change-set 4 already ports)."
- The input (change-set 4): the right mouse button or Q, held (the sandbox also offers click-to-toggle: a page
  setting). One more held button in the page's post and in the command; no press counter is needed for it.
- The rule (change-set 4): while aiming, the speed is the walk speed times `ads_speed_multiplier`, and sprint is
  off (R2, R4). A soldier in the air may aim (the old game sets no rule; asked of game-designer).
- The fields (change-set 4): the soldier's aiming flag in the simulation's state, in the snapshot and in the
  block (lane 2's wish W1, a need by his words).
- The camera (change-set 4): the aiming camera's mode and its four numbers, in the camera's data and its panel.
- What lands in change-set 8: each gun's spread resting at its sights value; the sight line (it needs a gun).
**The sprint-to-fire delay.** His words: "Coming out of a sprint, firing and aiming down sights each wait a short
delay, set per gun. It is a value I tune; it may end up subtle, and it must be there."
- The value is per gun, so it arrives with the weapon tables in change-set 8, with the rule for firing.
- What change-set 4 can honestly build without a gun: the soldier's "time since the sprint ended" in the state (in
  the integer time unit), and the block's field for how far the swing from the sprint pose to the aiming pose has
  got (lane 2's wish W4, a need by his words), so the block's layout does not change again in change-set 8. Whether
  aiming down sights already waits in change-set 4 needs a delay value with no gun to take it from: asked
  (SPRINT-DELAY-4), with the options "the rule and its value both arrive in change-set 8" (recommended: one home
  for a per-gun value) and "change-set 4 already holds aiming back, by one value in the movement table that
  change-set 8 replaces with each gun's".
- The lines they add, first estimate (the designs fix it): change-set 4 about 120 lines of code and 160 of tests
  (the held button through the post, the command, the room's check, the rule, the state, the snapshot, the block,
  the page's key and mouse handling, the readout); change-set 8 about 90 and 130 (the per-gun values, the wait
  before the first shot and before aiming, the spread's resting value, their tests).

## 4. 3B-ONE-PER-TICK in Walk
The only system that acts on a press in Walk is the jump: at most one jump per tick, and only from the ground, however
many presses one command brings (an honest client after an outage brings up to 255). Its test: a command with 200
presses of the jump button gives one jump. Aiming and sprinting read held buttons, not presses.

## 5. Questions this draft leaves for game-designer's check
1. Are R1 to R11 right, line by line, and is a rule of the old game's movement missing (the arena's edge, the
   `banzai` and shield multipliers at `:2774`, the dead or game-over damping at `:2778`)? Which of them belong to Slice 1's
   Conscript at all?
2. Is each row of section 2 a true difference, and is one missing?
3. Stamina in Walk: the old game and the sandbox both have it; no card of the build plan names it. Walk builds it
   unless he says otherwise (a rule of "walk, sprint ... with legacy's numbers", the build plan line 339): right?
4. Does any of this clash with `docs/LOOP_V2.md` or `docs/VISION_V2.md`?
