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
point (O); PM-0005-N is a pass mark of ADR 0005 (the export checks are PM-0005-47 to -56); spec:N is a line of the
glTF 2.0 specification as AER read it on 2026-11-17. "[inferred]" marks a step no source states; "[proposal]"
something offered and not yet ruled.

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
ruling in AER, beside mark 10 (PM-0005-56).
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
the client's node alone. No node in a file may be named `root` (PM-0005-49 fails such a file), and the engine never
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

## 4. Sockets on a gun, and how the hands meet them
**Names:** `socket_grip`, `socket_support`, `socket_muzzle`, `socket_sight`, `socket_eject` (A3:46; A10:25). Each
gun's entry lists the sockets its file has, and the check asks for those (AER AX-R11; PM-0005-53): the rifle file has
four (A3:43), the SMG five (A3:46). Every held gun lists grip, support and muzzle, because the two hands and the flash
need them (A10:5; A3:45). Sockets are cosmetic: a shot's start is simulation data (A10:29; AP-1, 0005:1549-1555).
**Axes, one rule.** A socket is an empty node without a mesh. In glTF (the gun's frame: +Y up, +Z to the muzzle, +X
the gun's left, origin at `socket_grip`: A6:11; AER AX-R6) a socket's local +Z is the direction it points, its local
+Y its up at right angles to +Z, and +X completes a right-handed frame. Grip, support, muzzle and sight point along
the bore with the gun's up: rotation identity, only their positions differ (`socket_grip` at the origin: A6:11).
`socket_eject` points where the case flies: for the SMG, "right and 20 degrees up" (A6:40) gives +Z =
(-0.940, 0.342, 0) and +Y = (0.342, 0.940, 0) in the gun's axes [computed: cos 20° = 0.940, sin 20° = 0.342].
**In Blender** (the gun modelled Z up, muzzle toward -Y, its left +X: A6:14) the exporter turns Blender -Y into glTF
+Z and +Z into +Y: confirmed for locations, inferred for rotations (AER AX-F11). So in Blender a socket's direction is
its local -Y and its up its local +Z: the grip, support, muzzle and sight empties keep identity rotation, and the
SMG's eject empty is turned until its -Y points along (-0.940, 0, 0.342) and its +Z along (0.342, 0, 0.940). The
check is PM-0005-53: each socket's whole frame (up included) within 0.5 degrees. Because the rotation step is
inferred, a real export of a gun with one empty turned must pass it before Blender files rely on this rule; the
script-written rifle file cannot prove that (PM-0005-53).
**How the hands meet the gun.** The rule is the correction's; Mustafa's go agrees with it on the hands and the gun's
direction, and the chest's turn is the Art lead's alone: a pose sets where `socket_grip` sits, which way the gun points
and how far the chest turns; the right hand is on `socket_grip`, the left on `socket_support`; the butt and the muzzle
follow from the gun's file (A10:5; A3:44). The pose numbers are change-set 10's.
- Right hand: `upper_arm_r` → `lower_arm_r` → `hand_r`, onto `socket_grip`. Left hand: `upper_arm_l` → `lower_arm_l`
  → `hand_l`, onto `socket_support` (A10:5), one point in place of legacy's rail slide (CC9, LI:139). Legacy solves
  both the same way (G:415-416).
- The fist, not the wrist, meets the socket (GA3, LI:220; G:243): the hand's point lies `palm` metres beyond the hand
  joint (the wrist) on the lower arm's line, so the IK's second bone is lower arm + palm (G:415). `palm` is one number
  per soldier for both hands (legacy measures one, G:247), kept in the soldier model's entry, not as a bone (BP Part
  11, rule 21); for each hand, the point `palm` beyond the hand joint lies within 1 cm of the centre of that hand's
  weighted vertices (ruled: PM-0005-56, line 7). The brief's hand centre lies 0.057 m beyond the wrist, within 4 mm
  of the forearm's line [computed from A4:36, 40], so the rule fits the brief; the correction's reach, "0.62 from the
  shoulder joint to the middle of the hand" (A10:5), is upper arm + lower arm + palm: 0.307 + 0.272 + 0.057 = 0.636 m
  for the brief's arm [computed from A4:36, 40].
- The hand's turn: `hand_*` keeps its rest turn in the hold, as in legacy (G:413), unless change-set 10 aligns it to
  the socket; under AER's O1, way (b), `hand_r` carries the gun, so change-set 10 sets its turn from the gun. The list
  is the same either way.
- What carries the gun: in legacy's default soldier, `chest` (C:453, 697), so breathing, recoil and the flinch move
  gun and hands together (GA7, GA12: LI:224, 229); the correction's poses, given in the soldier's frame, convert into
  the chest's frame at their own chest turn [inferred]. Change-set 10 may choose another joint of the list.
- Change-set 10's pose needs and the joints that carry them: the chest turned against the hips by 45 and 30 degrees
  with the head facing forward (A10:10, 16, 18): `spine` + `chest`, with `head` turning back; the body leaning 10
  degrees forward (A10:12, 20): `spine` (legacy's lean, G:364), or `hips` if the whole body leans; the left hand
  reaching `socket_support`: the left arm chain plus `palm`.

## 5. The soldier's origin and facing: a check a file passes or fails
Read in the file's bind pose; the file passes only if every line holds (axes: A4:11; AER AX-R5; spec:732, AER
AX-F1). PM-0005-56 runs these lines, with its numbers ruled, and adds the root's scale (line 6) and `palm` (line 7).
1. Upright: `head` above `chest` above `hips` above both feet, by joint height (y). A Z-up file read as Y-up fails.
2. Front +Z: for each foot, the centre of the vertices weighted to `foot_*` lies ahead of the foot joint (larger z):
   the toes point forward (A4:34: heel F -0.08, toe F 0.20).
3. Left +X: every `_l` joint has x > 0 and every `_r` joint x < 0. With line 2 this catches a file turned round
   (both fail) and a mirrored one (line 3 fails).
4. Origin on the ground between the feet: the lowest vertex at y = 0 within 1 mm; the mid-point of the two foot
   joints at x = 0 within 1 cm; z = 0 lies between the least and greatest z of the feet's weighted vertices.
5. One root: `hips` is the only joint without a joint parent; the mesh node and every non-joint node above `hips` are
   identity, and each joint's rest transform equals its bind pose (from the inverse bind matrices), within 0.5
   degrees and 1 mm, so the stand-in and the model hand the engine the same lengths and frames.
Planted files that must fail include (PM-0005-56): turned 180 degrees, mirrored, Z-up, origin moved 1 m, the mesh node
scaled 0.01, exported in a pose, a joint other than `hips` left without a joint parent; the unweighted vertex is the
planted defect of AER mark 3 (PM-0005-49).

## 6. Left open, and who decides
- Where the joints the brief does not place sit (hips, spine, chest, head, hip, knee, ankle, coat pivots): the Art
  lead (shapes: A4:2), judged by Mustafa on the Blender model (A3:17). The stand-in may take placeholders from the
  brief's seams; the engine reads whatever the bind pose says. For rigid weights only, the knee must sit at or above
  the boot's top for the whole boot to follow the lower leg [inferred from A4:33]; painted weights may blend the boot
  across the knee instead.
- Weights beyond the rigid baseline (the ankle seam, the coat's quarters, the torso between `spine` and `chest`, the
  scarf): the Art lead; how they look, Mustafa in the pictures.
- Whether the coat keeps GA13's thigh-following, flare and limits: Mustafa, at change-set 10's pictures. Dropping it
  removes the four coat joints, a list change that should land before the Blender model is exported: a model
  exported before change-set 10's pictures would have to be rigged and exported again.
- Joint limits and turn budgets (knee range, spine and chest turn, GA4's twist budget): change-set 10's tuning (pose
  values are tuning: 0003:795).
- Twist joints, or any joint the Blender model's bending would need: none in Slice 1; if the Art lead asks,
  engine-architect rules on a list change with the advisor agent, before export.
- Bone roll (each joint's local axes): the Art lead's; the engine must not depend on it (section 7).
- GA9's reload targets (the gun's reload point, legacy `reload.at` C:786, and the pouch): no socket among the five
  (A3:46; A10:25). Change-set 10 proposes data or a socket; a new socket name is Mustafa's.
- The name of the `palm` field stays the entry schema's; the tolerances are now ruled (PM-0005-53, -56). The
  script-written rifle file cannot prove the Blender case of section 4: that needs a real export (PM-0005-53).
- How Blender writes the armature object (a node, or a joint: AER part 8, not read): the first real export shows it
  (the advisor agent's ruling in AER, beside mark 10). AER's mark 3 (PM-0005-49) now carries section 2's P1 ruling:
  no node in a model file is named `root`.
- GA14's death fall: Mustafa's call (0003:798); the knee is needed anyway (GA2, GA6).
- Whether legacy's sides are mirrored (section 7): change-set 10 confirms it with one legacy test.

## 7. What this list changes for change-set 10
- **Size:** smaller and more direct. The 20 joints are the 20 transforms legacy already moves, so change-set 10
  ports GA1-GA15 one to one by name (legacy `pelvis` → `hips`, `neck` → `head`, `hipL` → `upper_leg_l`, `kneeL` →
  `lower_leg_l`, `shL` → `upper_arm_l`, `elbowL` → `lower_arm_l`, `handL` → `hand_l`, the load-time ankle →
  `foot_l`, the same on the right, `gc_skFL` → `coat_front_l` and so on), and drops legacy's two load-time fix-ups
  (the ankle re-hang, G:253-264; the palm measure, G:243-247, now data): no knee, ankle or coat to rebuild.
- **Risk:** three port traps, each tested on the stand-in. Legacy's sides look mirrored (its `R` joints and its gun
  at +X while it faces +Z: C:453, 492; G:2710) [inferred], so its X-signed numbers and turn signs flip once, in one
  place. Legacy sets angles on joints whose rest turn is identity and reads lengths along -Y (G:250, 364-366), while
  a Blender bone has a rest turn of its own [inferred], so change-set 10 turns joints through their bind-pose frames
  and the stand-in gets non-identity rest turns to prove it [proposal]. World-to-soldier sums must include the root's
  scale (legacy uses the facing turn only, G:357).

## Verifier spot-check
One verifier pass (Opus 5.5, effort max) checked this file on 2026-11-17 at commit cacea55, and took a second look
at the commits after it the same day. Verdict: PASS. 36 of 36 key claims were checked at their sources: 36
confirmed, 0 wrong, 0 unsupported. The lines below are the verifier's own, copied by script from its notes
(`verifier-cs0.md`, in the commit that this change-set's report names): claim, source, verdict, place. They describe
the file as it stood at cacea55.

- JL1 legacy 14 joints, root outside the map · C:454 (14 entries), C:465 pelvis parent null · confirmed.
- JL2 legacy joint names (pelvis, spine, neck, hipL/R, shR, elbowL/R) · C:465, 468-473, 478, 483, 492, 495-501 ·
  confirmed.
- JL3 coat panels hang on pelvis · C:589 (gc_skFR/BR/BL/FL parent 'pelvis') · confirmed.
- JL4 GA13 gains 0.95/0.75, clamp -0.8..0.6, follow thigh · G:168, 441-443 · confirmed.
- JL5 GA7 twist split 0.45 spine / 0.55 chest; flinch and death tip use spine/chest · G:364-365, 438, 474 · confirmed.
- JL6 ankle built at load, "the rig has none"; LI:220 asks v2 to emit it · G:253-264; LI:220 ("should emit an ankle
  joint and a hand socket") · confirmed.
- JL7 palm measured (fist beyond wrist), IK second bone = lower arm + palm · G:243-247, G:415-416 · confirmed.
- JL8 legacy gun on chest; R joints and gun at +X while facing +Z ("mirrored", [inferred]) · C:453 (parent chest, pos x
  0.12), C:492 (shR x 0.6), C:697, G:2710 (rotation.y = atan2(fd.x, fd.z)) · confirmed (the inference is a fair one:
  with +Y up and +Z front, glTF's +X is the left).
- JL9 GA rows · LI:219 GA2 IK, :221 GA4 hip turn, :223 GA6 pelvis/legs/ankle, :224 GA7 upper body, :225-228 GA8-GA11,
  :229 GA12 flinch whips head, :230 GA13 ("Prefer ... skinned panels"), :231 GA14 ("The root never tilts or sinks"),
  LI:139 CC9 rail slide · confirmed.
- JL10 legacy reload point · C:786 reload {style:'ram', at:[...]} · confirmed. World-to-soldier uses facing only · G:357
  · confirmed. Lengths along -Y · G:250 · confirmed.
- JL11 brief axes (L, F, U), origin between the feet; glTF (x,y,z) = (L,U,F); Blender = (L,-F,U); one root scales with
  one number · A4:11, A4:12, A4:73 ("(L, F, U) is (X, -Y, Z)", "one empty at the origin", "one object per part") ·
  confirmed.
- JL12 part heights: skirt U 0.43-1.045 (A4:30), boot top U 0.50 (A4:33), foot heel F -0.08 toe F 0.20 (A4:34), trouser
  U 0.36-0.50 (A4:35) · confirmed.
- JL13 arm arithmetic from A4:36, 40: upper 0.3070, lower 0.2723, hand centre 0.0568 beyond the wrist (0.0566 along the
  forearm), sum 0.6361 m · confirmed (0.307 + 0.272 + 0.057 = 0.636). Nit: the hand centre is 4.03 mm off the forearm
  line; the file says "within 4 mm" (rounding; not a finding).
- JL14 correction: "0.62 from the shoulder joint to the middle of the hand", right hand socket_grip / left
  socket_support, butt and muzzle from the file (A10:5); chest 45/30 deg (A10:10, 16, 18); lean 10 deg (A10:12, 20);
  socket names (A10:25); muzzle wording (A10:29) · confirmed. A10 input is 3,244 B, 0 CR (= appendix's 3,243-B block +
  final LF).
