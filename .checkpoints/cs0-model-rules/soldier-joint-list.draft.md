# The soldier's joint list: engine-architect's ruling for change-set 0

engine-architect's ruling of 2026-11-17 for change-set 0 of the build plan, upheld by the advisor agent's re-read of
that plan the same day. It fixes names and structure only: shapes, proportions, poses and the feel of the walk stay
with Mustafa, the Art lead and change-set 10. Everything it takes from the Art lead's texts is the Art lead's proposal
(Mustafa's words of 2026-11-17, A3, CHANGE 3).

Sources (file:line unless named otherwise): LI `docs/LEGACY_INVENTORY.md`; 0003, 0005 `docs/adr/0003-rendering.md`,
`docs/adr/0005-content-abilities-assets.md`; C, G `legacy/shamble-core.js`, `legacy/shamble-game.js`. A3 to A10 are
blocks of `docs/reports/2026-11-17-build-plan-slice-1-appendix.md`, cited with the line inside the block (the file's
own line number): A3 Mustafa's go of 2026-11-17, A4 the Art lead's Conscript blockout, A5 the poses addition, A6 the
SMG brief, A10 the gun-pose correction. BP is the build plan, `docs/reports/2026-11-17-build-plan-slice-1.md`, cited
by Part and rule. AER is `docs/research/asset-export-rules.md`, cited by finding (AX-F), rule (AX-R), mark and open
point (O); spec:N is a line of the glTF 2.0 specification as AER read it on 2026-11-17. "[inferred]" marks a step no
source states; "[proposal]" a number offered for the marks to fix.

## 1. The ruling: one engine joint list for Slice 1's soldier (20 joints)
Build plan rule 21's 16 joints with their names unchanged (BP Part 11, rule 21), plus four coat joints. The stand-in
and the Blender model both carry exactly these.

| Joint | Parent | Why it is there |
|---|---|---|
| `hips` | none: the only joint without a joint parent (section 2) | Pelvis bob, sway, roll, crouch, and the hip turn toward the move (GA4, GA6: LI:221, 223; G:302, 351-352); the upper body tips from it at death (GA14, LI:231). Legacy `pelvis` (C:465). |
| `spine` | `hips` | Takes 0.45 of the aim twist, the lean, part of the flinch and of the death tip (GA7, LI:224; G:364, 438, 474). Legacy `spine` (C:478). |
| `chest` | `spine` | The other 0.55 of the twist, breathing, recoil, reload dip, stride roll (G:365); carries the shoulders (C:492) and legacy's gun (C:453). |
| `head` | `chest` | Pivot at the neck: turns to aim and keeps facing forward while the chest turns (G:366; A10:10); the flinch's head whip (GA12, G:438); scarf, head and hat follow it (A4:44-50, A5:16). Legacy `neck` (C:483). |
| `upper_arm_l`, `upper_arm_r` | `chest` | Pivot at the shoulder (A4:36): first bone of each arm's two-bone IK (GA2, LI:219; G:415-416). Legacy `shL`, `shR`. |
| `lower_arm_l`, `lower_arm_r` | `upper_arm_l`, `upper_arm_r` | Pivot at the elbow (A4:36): second IK bone (G:415-416). Legacy `elbowL`, `elbowR` (C:495-501). |
| `hand_l`, `hand_r` | `lower_arm_l`, `lower_arm_r` | Pivot at the wrist (A4:36): the hand turns apart from the forearm (G:413) and does the hold, reload, throw and props (GA8-GA11, LI:225-228). Legacy `handL`, `handR`. |
| `upper_leg_l`, `upper_leg_r` | `hips` | Pivot at the hip: first bone of each leg's two-bone IK (GA2, GA6; G:358); the coat reads its angle (G:441-443). Legacy `hipL`, `hipR` (C:468-473). |
| `lower_leg_l`, `lower_leg_r` | `upper_leg_l`, `upper_leg_r` | Pivot at the knee: second leg IK bone (G:358); the knees buckle at death (GA14, G:471-472). Legacy `kneeL`, `kneeR`. |
| `foot_l`, `foot_r` | `lower_leg_l`, `lower_leg_r` | Pivot at the ankle: foot held flat, pitched, rolled heel to toe, tucked in the air, boots never below ground (GA3, GA5, GA6: LI:220, 222, 223; G:359-360). Legacy builds it at load because "the rig has none" (G:253); LI:220 asks v2 to emit it. |
| `coat_front_l`, `coat_front_r`, `coat_back_l`, `coat_back_r` | `hips` | Pivot at the skirt's top edge: each quarter follows the thigh on its side with front and back gains, flares at speed, stops at limits, tilts at death (GA13, LI:230; G:168, 265, 441-443). Legacy hangs its four panels on the pelvis (C:589); LI:230 suggests "skinned panels". |

- **Slice 1 needs all 20**: change-set 10's port drives every one (BP Part 1, the card of change-set 10; 0003:797),
  on the stand-in and the model alike (BP Part 12). No joint waits for a later change-set.
- **The coat has joints of its own.** GA13 turns each quarter by the thigh's angle on its side times a front or back
  gain (0.95, 0.75), adds a spring flare at speed and clamps it to -0.8..0.6 (G:168, 441-443). Fixed skin weights on
  hips and thighs give a fixed blend of joint turns, with no gain, spring or limit [inferred], so a weights-only coat
  would change how the coat moves: not the engine's call (section 6). The skirt (U 0.43 to 1.045, A4:30) hangs below
  the knee, since the boot that follows the lower leg tops at U 0.50 (A4:33) [inferred], so in Slice 1 it must move
  with the legs.
- **The spine joint stays.** GA7 splits the aim twist 0.45 spine, 0.55 chest (G:364-365); the flinch and the death
  tip use the split (G:438, 474); the correction's poses turn the chest 45 and 30 degrees against the hips (A10:10,
  16, 18). One joint would put the whole turn at one pivot [inferred].
- **Left out**, as nothing in Slice 1 needs them: a root joint (section 2); a separate neck (G:366 drives one joint);
  toes (the toe-off roll is foot pitch at the ankle, G:359-360); twist joints (no GA row turns a limb along its
  length); helper, IK or hand-socket bones (BP Part 11, rule 21; the fist point is data, section 4); hat and flap
  joints (they follow the head, A4:46-50); a gun joint (the gun is its own model without a skin: A5:17; AER AX-R15).
- **Same work as legacy:** the 20 are the transforms legacy already moves, its 14 joints (C:454), the 2 ankles it
  adds at load (G:253-264) and its 4 coat panels (G:265). Bone upload: 20 x 64 B = 1,280 B per soldier per frame
  [computed: a 4 x 4 matrix of 4-byte floats is 64 B; how three.js uploads bones is not read: AER part 8].
- **Rules for every joint:** lower case, digits, underscores; `_l` and `_r` are the soldier's own left and right
  (A4:11), and survive three.js's name cleaning where a Blender `.L` would not (AER AX-F15). A joint is named after
  the segment it moves; its pivot (in Blender, the bone's head) is at the segment's upper end. The order in the skin
  is free: the engine maps names to bones once, at load. The engine reads lengths and rest frames from the bind pose,
  never from fixed numbers or legacy's hang-along-minus-Y assumption (G:250). Nothing here reaches the simulation:
  hits, hit shapes and shot starts stay simulation data (AP-1, 0005:1549-1555).

## 2. `root`: the node above the skeleton, not a joint
**Answer:** `root` is not a skin joint. It is the one node above both the skeleton and the mesh, which the game
moves, turns and scales with one number (A3:39, A4:12); `hips` is the skeleton's top joint.
**Reason.** (a) Nothing in Slice 1 moves a joint at the origin: the root never tilts or sinks (GA14, LI:231), and
legacy's root is the character group outside its joint map (C:454; `pelvis` has no parent, C:465), which the game
only places and turns (G:2710). The position, facing and scale must move the skeleton and the mesh node together, so
that the fixed padded bounding sphere (0003:306-312) moves with the soldier [inferred]; the node that carries them is
a common parent of both, which a skin joint is not [inferred]. A root bone would therefore stay identity: a joint for
later, and 64 B more per soldier per frame [computed, section 1]. (b) graphics-engineer's first draft of AER (before
its update of 2026-11-17) took `root` for the armature's top bone, because bone names export as written (AER AX-F8)
while how the armature object is written was not read (AER part 8). That matters only if something must find `root`
by name in the file, and under this ruling nothing does: the engine finds the skeleton by `hips`, and the check finds
non-joint nodes by structure. A root bone does not guard the unread case either: if the exporter made the armature
object a joint, AER mark 3 would see an extra joint with or without it. So this ruling departs from that first
reading; the advisor agent upheld it, and what the first real Blender export must show for it is the advisor agent's
ruling in AER, beside mark 10.
**In glTF:** `skin.joints` holds exactly the 20 names, in any order; `hips` is the only joint whose parent is not a
joint, so it is the skeleton root, and a written `skin.skeleton` names `hips` or a node above it (spec:2118: "That
node **MUST** be the root of all nodes listed in the `joints` array or their common ancestor."). The mesh node and
every non-joint node above `hips` (the armature's node, if the exporter writes one) carry an identity transform: the
skin ignores them (spec:2238; AER AX-F5), but three.js culls by the mesh node's world matrix [inferred]. No node in
the file may be named `root` (below).
**In the engine:** `root` is one client-owned node per soldier; it takes the position and facing from the snapshots
and the one scale number, and the loaded file's top nodes hang under it. The stand-in builder makes the same node and
hangs `hips` and its mesh on it, so the model replaces the stand-in with no engine change (BP Part 12).
**In Blender:** the armature object is the one root (the brief's "one empty at the origin", A4:73, becomes it): at
the origin, transforms applied, top bone `hips`, no root bone; the joined mesh is its child (AER AX-R14). Its name
follows AER AX-R7 and is never `root` (below); it names the skin and, by default, most likely a node of its own (AER
AX-F8). An empty above it would add one identity node and pass the check, but is not needed.
**The name `root` (graphics-engineer's point P1).** The exporter names every object's node after the object (AER
AX-F8) and by default most likely writes the armature object as a node [inferred in AER], so an armature object
called `root` would give the scene two nodes of that name, the client's and the file's. Ruling: the name belongs to
the client's node alone. No node in a file may be named `root` (the check fails such a file), and the engine never
looks a soldier's root up by name: it holds the node it made.

## 3. The brief's 17 parts on the joints (the stand-in's weights)
One skinned mesh: each part's vertices are weighted 1.0 to the part's joint (the rigid baseline: BP Part 11, rules 19
and 20). The brief's "one object per part" (A4:73) is the modelling form: the parts are joined into one mesh under one
armature before export (AER AX-R14), and vertex groups carry the joint names exactly (AER AX-R10, AX-F10).

| # | Part (A4 line) | Joint |
|---|---|---|
| 1 | `coat_skirt` (A4:30) | Split into quarters by the signs of L and F: `coat_front_l` (L > 0, F > 0), `coat_front_r` (L < 0, F > 0), `coat_back_l` (L > 0, F < 0), `coat_back_r` (L < 0, F < 0). The stand-in builds the quarters as separate pieces, as legacy's panels are (C:589), so rigid weights stretch no shared seam [inferred]. |
| 2 | `torso` (A4:31) | `chest` |
| 3 | `belt` (A4:32) | `hips` |
| 4 | `boot_L`, `boot_R` (A4:33) | `lower_leg_l`, `lower_leg_r`. The brief's leg is one piece "from hip to ankle"; split at the knee, every visible leg part lies below U 0.50 (A4:33, 35), so all of it follows the lower leg. The thigh, hidden under the coat (A4:30), has no part. |
| 5 | `foot_L`, `foot_R` (A4:34) | `foot_l`, `foot_r` |
| 6 | `trouser_L`, `trouser_R` (A4:35) | `lower_leg_l`, `lower_leg_r` |
| 7 | each arm: `upper_arm`, `forearm`, `cuff`, `hand` (A4:37-40) | `upper_arm_*`, `lower_arm_*`, `lower_arm_*`, `hand_*`. The rest joints shoulder, elbow, wrist (A4:36) are the pivots of `upper_arm_l`, `lower_arm_l`, `hand_l`; `_r` mirrors them. |
| 8 | `board`, `board_plate` (A4:41-42) | `chest` |
| 9 | `collar` (A4:43) | `chest` |
| 10 | `scarf` (A4:44) | `head` |
| 11 | `head` (A4:45) | `head` |
| 12 | `hat_band` (A4:46) | `head` |
| 13 | `hat_crown`, `hat_top` (A4:47-48) | `head` |
| 14 | `hat_flap_front` (A4:49) | `head` |
| 15 | `ear_flap_L`, `ear_flap_R` (A4:50) | `head` (both) |
| 16 | small parts (A4:51): pouch; buckle; buttons | `hips`; `hips` (it sits on the belt); `chest` (on the torso's front) |
| 17 | rifle stand-in (A4:52) | No joint: a gun is a model of its own, without a skin (A5:17; AER AX-R15). Legacy's default soldier hangs it on `chest` (C:453, 697); under AER's open point O1, way (b), its geometry is merged and weighted 1.0 to `hand_r`. |

- `spine` and `upper_leg_*` carry no vertices in the stand-in; they still move everything that hangs on them.
- The brief's (L, F, U) is glTF (x, y, z) = (L, U, F) (A4:11; spec:732, AER AX-F1) and Blender (x, y, z) = (L, -F,
  U) (A4:73): the left shoulder (0.19, 0, 1.44) is glTF (0.19, 1.44, 0). The brief's capital `_L`/`_R` are part
  names, gone once the parts are joined; only joint names follow section 1's rules.
