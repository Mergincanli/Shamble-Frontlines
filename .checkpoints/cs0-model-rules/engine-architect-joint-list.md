Task: joint-list (engine-architect design ruling for change-set 0: the soldier's engine joint list; output joint-list.md)

## Plan (blocks, in order)
B1. Read LEGACY_INVENTORY rows GA1-GA15, CC9, CC10.
B2. Read ADR 0003 lines 301-323 (part 6) and 790-799 (what ports).
B3. Read ADR 0005 lines 1549-1555 (AP-1) and 1771-1776 (GPU rule 3).
B4. Read the Art lead's briefs in .checkpoints/build-plan-slice-1/inputs/ (blockout part list, addition: three poses, SMG brief).
B5. Read Mustafa's words 2026-11-17 CHANGE 3 (mustafa-2026-11-17-go.md).
B6. Read plan.md "The names this change-set fixes" + advisor-plan-r1.md findings 1, 2, 3, 16.
B7. Read asset-rules.md section 2 (loader facts).
B8. Check legacy code only where a GA/CC row's cited line is needed (rig parents, fist offset, hip/coat).
B9. Write joint-list.md (<=130 lines), sections 1-7.
B10. DONE; answer from notes.

## Notes
- Memory: engine-architect has memory: local; no agent-memory folder exists in this environment (fresh). Nothing to load.

## B1 DONE: LEGACY_INVENTORY GA1-GA16, CC5, CC8-CC11 (docs/LEGACY_INVENTORY.md)
- GA2 (:219): two-bone IK, arms AND legs (shoulder-elbow, hip-knee), pole vector; leg calls game:358,472; arm calls 415,416,427,432,435,486,488; metrics leg IK stretch 0.98, hands on grip/rail 0.0 cm. => needs upper+lower leg joints (knee) and upper+lower arm (elbow).
- GA3 (:220): _animInit measures where the FIST sits beyond the WRIST (palm measure game:243-247), re-hangs boot parts under a PROCEDURAL ANKLE pivot (253-264) "because the rig has no ankle joint"; greatcoat lookup 265-268. v2 note: "A v2 character builder should emit an ankle joint and a hand socket so this fix-up step disappears." Rig facts (ANIMATION_PLAN §5): pelvis y 0.98, thigh 0.46, knee to ankle 0.36, ankle to sole 0.16. Metric: boots below ground 0 cm all gaits.
- GA4 (:221): hip yaw toward movement within twist budget (297-303) => pelvis/hips joint yaws separately from chest.
- GA5 (:222): planted feet; airborne tuck feet 0.18 m above root, pitched 0.25 rad (foot pitch => ankle/foot joint).
- GA6 (:223): pelvis bob/sway/roll + crouch; leg IK onto ANKLE point; boot held flat, turned with foot, toe-off/heel-strike roll (354-360) => ankle joint (foot child of lower leg).
- GA7 (:224): counter-twists SPINE and CHEST toward aim + blade; speed lean, breathing, stride roll, recoil+reload dip on chest; HEAD turns to aim (361-366). => spine + chest + head(neck) joints.
- GA8 (:225): gun-first stance; trigger hand IK onto grip socket; support hand slides along rail; GUN_KIT sockets grip/stock/rail/muzzle/mass/hold/reload (core:785-791); muzzle child of wpn (core:696-697); metric hand on grip/rail 0.0 cm (target <1 cm).
- GA9 (:226): reload: support hand between rail, belt pouch/quiver, gun reload point (hand targets only).
- GA10 (:227): throw: LEFT hand throws, right keeps gun.
- GA11 (:228): shield: left hand turned to face shield + arm solved; katana: right wrist travels, left fist follows hilt; props hung on the hands. Slice 1? (abilities; Riot Shield/Banzai blade are Tier 1 abilities -> check Slice 1 scope).
- GA12 (:229): flinch snaps upper body (gun and hands) + whips head; applied after holds+arm IK; only greatcoat panels updated after.
- GA13 (:230): 4 greatcoat skirt panels gc_skFL/FR/BL/BR rotate WITH THE THIGH ON THEIR SIDE + flare at speed via springs; death: fixed tilt; v2 note "Prefer a shared double-sided material or skinned panels."
- GA14 (:231): death: KNEES buckle into kneel, feet tucked, upper body tips FROM THE PELVIS (inverted pendulum); ROOT never tilts or sinks.
- GA15 (:232): driver contract; LOD needed at 48; "rigid skinning of the parts would cut the per-part draw calls and Object3D matrix updates"; ~51 us/soldier headless, ~26 us runtime.
- CC5 (:136): legacy joint map: pelvis, spine, chest, neck, hips, knees, shoulders, elbows, hands (CHAR_* joints core:454 etc). Rig facts: pelvis y 0.98, chest joint 1.75, shoulders +-0.6, upper arm 0.58, forearm 0.59, hips +-0.21, thigh 0.46. Hit shape separate (core:455/462), visual rig never drives hit.
- CC8 (:138): gun model space: trigger grip near origin, barrel runs +Z.
- CC9 (:139): GUN_KIT sockets grip (trigger hand), stock (butt), rail [zMin,zMax,y] (support hand), muzzle (FX), mass, hold, reload{style,at}; cosmetic only; shots from playerFireOrigin (game:1420).
- CC10 (:140): rest arm poses _ARMP per armMode (core:800-812); armMode enum is the live contract (game:288).
- Next: B2 (ADR 0003 301-323, 790-799).
- COORDINATOR MESSAGE (mid-task, weigh; readings not decisions) from graphics-engineer's draft asset-export-rules.draft.md:
  (1) root must be a BONE, not the armature object or an empty (Blender exporter behaviour; see draft's armature + names findings): section 2 agree or say why not.
  (2) sockets: direction along local +Z; each gun entry lists its own sockets (rifle file has 4; socket_eject new with SMG); check asks for the listed sockets, not always five.
  (3) brief builds "one object per part"; skinned file must join into ONE skinned mesh (one mesh object, one armature) before export; section 3 table reads as the stand-in's WEIGHTS, not separate objects.
  => added block B7b: read the draft's armature/names/sockets findings.

## B2 DONE: ADR 0003 (docs/adr/0003-rendering.md)
- :301-305 part 6: ONE SKINNED MESH PER SOLDIER (option C-B) on both backends, "with any skin weights", soldiers are authored models (LOOK_V2 §3, §8; model is ADR 0005's); instanced skinning measured vs it, adopted only if >=20% frame time cut.
- :306-312 budget LOD L-2: fixed number of soldiers get the FULL PROCEDURAL RIG each frame by importance; rest lower rate or CHEAPER RIG; bounds fixed padded sphere.
- :313-319 per-instance data from blocks into typed arrays; faction colour slots (ADR 0005 I-8).
- :793 CL10, CC1-CC5: redesign: soldiers become authored, skinned models (part 6; ADR 0005 pipeline).
- :794 CC6, CC8, CC9, CC11, CC12, GC6: port: kit data, SOCKETS, IK and props port; gun/prop shapes authored models.
- :795 CC10: arm-mode enum in anim state; rest pose folded into the animation port on the authored skinned soldier; pose values are tuning.
- :797 GA1-GA15: port to TypeScript; GA15 adds budget LOD (part 6).
- :798 GA14: pending "Mustafa's call: whether the toy-soldier death fall fits LOOK_V2 §3's adult audience" => GA14's knee buckle is ported but its keeping is Mustafa's call; the knee is needed anyway by GA2/GA6.
- Next: B3 (ADR 0005 1549-1555, 1771-1776).

## B3 DONE: ADR 0005 (docs/adr/0005-content-abilities-assets.md)
- :1549-1555 AP-1 "An asset decides nothing": no collider, hit shape, SHOT ORIGIN, nav data or vehicle-part layout read from a model; shots leave from sim's fixed points; a weapon model's sockets are cosmetic (CC9); server loads no model; no glTF crate in sim/server trees.
  => joints and sockets are client-only cosmetic names; nothing in the joint list may feed the sim (hit shape stays CL11 sim data).
- :1771-1776 GPU rule 3: a soldier model is ONE SKINNED MESH with skin indices+weights as attributes, any weights the artist paints (C-B); very large units are part groups named to sim vehicle-part records.
- :1778-1779 (context) a soldier as drawn (body, headgear, held weapon) uses at most 4 materials per pass (ADR 0003 caps 4 draw calls/pass).
- Next: B4 (Art lead's briefs in build-plan-slice-1/inputs/).

## B4 DONE: Art lead's briefs (.checkpoints/build-plan-slice-1/inputs/)
Conscript blockout (ART_BRIEF_2026-11-14_1635_conscript_stand_in.md, "CB"):
- CB:2 "This brief covers his shapes and colours only. Movement, shooting and the rig's rules are the build plan's."
- CB:11 axes (L,F,U) metres, L his left, F forward, U up; origin on the ground between the feet. CB:12 size placeholder 1.80 top of hat; "Hang every part on one root so the whole soldier scales with one number."
- CB:13 coarse shapes, "One part per name below". CB:29 "Each line ends with its material and, for a rigid skin, the bone it follows (working bone names)."
- Parts->bones: 1 coat_skirt U0.43-1.045 Hips (CB:30); 2 torso U1.10-1.47 Chest (:31); 3 belt U1.045-1.10 Hips (:32); 4 boot_L/R U0.10-0.50 "Leg (one piece from hip to ankle)" (:33); 5 foot_L/R U0-0.10 heel F-0.08 toe F0.20 Foot (:34); 6 trouser_L/R U0.36-0.50 round boot top Leg (:35); 7 arm: rest joints shoulder (0.19,0,1.44) elbow (0.255,0,1.14) wrist (0.31,0.03,0.875), arm 12 deg out (:36); upper_arm Upper arm (:37); forearm Forearm (:38); cuff Forearm (:39); hand box centre (0.32,0.04,0.82) Hand (:40); 8 board, board_plate Chest (:41-42); 9 collar U1.45-1.51 Chest (:43); 10 scarf U1.49-1.615 Head (:44); 11 head U1.54-1.70 Head (:45); 12 hat_band Head "as are 13 to 15" (:46); 13 hat_crown + hat_top (:47-48); 14 hat_flap_front (:49); 15 ear_flap_L/R (:50); 16 small parts: pouch hips; buckle (no bone; on belt); buttons (no bone; on torso front) (:51); 17 rifle stand-in 1.12 long, grip origin, butt 0.26 behind, muzzle 0.86 ahead, or AMK9_v4_stage1.glb "which has grip, support, muzzle and sight sockets" (:52); idle hold (:53) "Other poses are the build plan's".
- No thigh part exists: the thigh is inside the coat skirt (U0.43-1.045); all visible leg parts (boot U0.10-0.50, trouser U0.36-0.50) lie below U0.50 [inferred from CB:33,:35].
- CB:73 Blender: Z up, faces -Y, left +X, (L,F,U)=(X,-Y,Z); "Parent all parts to one empty at the origin, apply scale, one object per part"; "No export until the export settings are decided."
Addition (ART_BRIEF_ADDITION_2026-11-15_poses_yard.md, "AD"):
- AD:5 "Place the hands; the arms follow." AD:7 aim: butt (-0.12,0.05,1.40), right hand grip (-0.12,0.31,1.36), left hand fore-end (-0.10,0.58,1.37), muzzle (-0.12,1.17,1.40) "shots leave from there" (conflicts AP-1: Art lead proposal; sim origin decides); "The chest turns 20 degrees to his right ... The head keeps facing forward." (=> chest twist separate from head). AD:8 sprint "body leans 10 degrees forward". AD:10 SMG same three poses.
- AD:16 hat is part of the body model, "following the head bone". AD:17 rifle a model of its own: right hand at its grip socket, left hand to its support socket; "Slice 1 has two guns on one soldier".
SMG brief (ART_BRIEF_2026-11-15_1735_smg_stage1.md, "SB"):
- SB:11 "glTF 2.0, 1 unit = 1 m, +Y up, +Z to the muzzle, +X to the gun's left. Origin at the grip socket." SB:13 names letters/digits/underscores. SB:14 Blender: "Z up, the muzzle towards -Y, origin at the grip socket, all transforms applied."
- SB:35 "SOCKETS: empty nodes, named exactly as in the rifle file, plus one new one": grip d0.64 0.036 below bore = origin (:36); support on fore_grip d0.30 0.07 below bore (:37); muzzle d0 on bore (:38); sight d0.56 0.04 above bore (:39); eject (new) d0.37, 0.028 right of bore, "pointing right and 20 degrees up" (:40).
- SB:42 "The left hand is always on the support socket." SB:43-45 poses; SB:46 "The movement plan may change these poses."
- Next: B5 (Mustafa 2026-11-17 CHANGE 3).

## B5 DONE: Mustafa 2026-11-17 (.checkpoints/build-plan-slice-1/mustafa-2026-11-17-go.md, "MG")
- MG:14 FIRST-SOLDIER (a): the Conscript; SMG and rifle both in change-set 8. MG:22 STAND-IN-LOOK (a): "I judge proportions and weapon shapes on the Blender model." MG:26 active reload in Slice 1 (simplified), joins cs8. MG:38 picture track proposal: 6 Look, 10 Body, 11 Feel, M (second lane).
- MG:43 "the soldier stays a skinned mesh, as your Part 11 says." MG:44 "hang the soldier on one root so it scales with one number." MG:45 1 unit = 1 metre.
- MG:46 "The brief's names for colour areas and bones are working names; the names of your change-set 0 win."
- MG:48 AMK9_v4_stage1.glb: glTF 2.0, 2 meshes, 2,596 tris, 1 material, four sockets, 1.12 m, +Y up, +Z to the muzzle; "stays with me until you ask for it".
- MG:49 "Where the addition's hand points and a gun's sockets disagree, the sockets win: the right hand's point and the gun's direction set the pose; the left hand, the butt and the muzzle follow from the gun model's sockets, as the SMG brief already does." (=> v2 hold is grip-first; legacy GA8 was stock-first; no butt/stock socket is named: butt position must come from gun data/geometry -> open).
- MG:50 "The muzzle is where the flash and the tracer start. In the simulation a shot's start is data of its own" (AP-1).
- MG:51 SMG sockets: socket_grip, socket_support, socket_muzzle, socket_sight, new socket_eject (rifle file has the first four).
- Next: B6 (cs0 plan.md names section + advisor findings 1, 2, 3, 16).

## B6 DONE: cs0 plan.md + advisor-plan-r1.md
- plan.md:34-57 names section: colour areas faction_coat/top/trim, unit_dark (:37-41); WITHDRAWN 13-joint list (:42-46) "leg one piece ... the list belongs to the model's entry"; sockets socket_grip (origin), socket_support, socket_muzzle, socket_sight, socket_eject (:47-50); axes soldier +Y up, front +Z, left +X, origin ground between feet, everything under root; gun +Y up, +Z muzzle, +X gun's left, origin socket_grip; Blender Z up, front/muzzle -Y (:51-53); marking_ prefix (:54-55); :56 Part 11 proposed spine, upper and lower leg before the briefs.
- plan.md rev 2 :114-118 item 1: 13-joint list withdrawn + "belongs to model's entry" withdrawn; ONE engine list; engine-architect rules vs GA1-GA15 + ADR 0003 794-797 (knees, spine, coat panels); brief parts mapped (boot+trouser follow lower leg); research file takes the list word for word. :120-121 item 2 root: skin joint or parent node; mark 3 follows. :123-125 item 3 sockets have axes (eject). :136-146 marks 6, 7 (forward axis within 0.5 deg; planted: muzzle socket turned 5 deg), 9; mark 3 "every joint name equals the model's list; no extra joint (exporter's neutral bone is the planted defect)" (:64). :148-150 AMK9 written by script: proves marks 1,2,5,7,8; marks 3,4,6 need real Blender export (test box + small skinned figure). :192-193 item 16: how hand meets socket_grip; soldier origin+facing check (mark 10, with mark 6's export).
- advisor-plan-r1.md: :102 finding 1 HIGH (as task states; build plan l.271-272 "legacy's animation metrics as goldens"; Part 11 rule 21 / Part 12 l.1059 "bound to the same joints ... so the animation of cs10 drives both"). :103 finding 2 MED root. :104 finding 3 MED socket axes "+Z = out, +Y = up; Blender -Y / Z". :59 F-sockaxes "Blender empty identity -> glTF identity (exporter conjugates)". :60 F-hand: hand joint = wrist; hand box centre 5.5 cm off; "engine-architect to decide hand nodes vs offset in model entry". :117 finding 16 LOW: hand grip point (GA3 measured fist beyond wrist) + soldier origin check (min y 0, feet centred x 0). :40 build plan Part 11 rule 21 old list (16 joints, two-bone limbs, spine); Part 11 l.1021 legacy drives "two-bone arms and legs, spine, chest, head". :68 asset-rules s4 l.348 legacy rig: two-bone arms/legs, spine, chest, head, hand targets, four coat panels, no ankle. :69 s5 l.389-391 stand-in same list; ported animation finds bones by these names. :71 neutral bone at armature root for unweighted vertex (sure).
- Next: B7 (asset-rules.md s2) + B7b (graphics-engineer draft: armature/root, names, sockets); then build plan Part 11 rule 21 / Part 12 l.1059 / cs10 row (l.271-272) for section 7.

## B7 DONE: asset-rules.md s2 (.checkpoints/build-plan-slice-1/asset-rules.md, "AR")
- AR:160-161 glTF spec:732 "+Y as up; the front side ... faces +Z, the left side ... faces +X" (sure). AR:162-163 metres.
- AR:183-186 skins: spec:2084 joints REQUIRED, inverseBindMatrices optional; spec:2129 "The joint hierarchy used for controlling skinned mesh pose is simply the node hierarchy"; spec:2242 "A skinned mesh primitive does not use the global transform of the node that instantiates the mesh." (=> scaling must sit on an ancestor of the joints; the mesh node's own transform is ignored).
- AR:175-180 4 influences per JOINTS_0/WEIGHTS_0 set; three.js reads only one set.
- AR:197 export_yup swizzle Vector((loc[0], loc[2], -loc[1])) (nodes.py:611-616): Blender +Z -> glTF +Y, Blender -Y -> glTF +Z.
- AR:210 Use Rest Position Armature default True. AR:211 Export Deformation Bones Only default False.
- AR:219 object -> node name = blender_object.name; AR:223-224 bone -> joint node name = blender_bone.name (joints.py:102-103; "leaf bones get _leaf"); skin name = armature object's name (skins.py:153-154).
- AR:231-235 vertex groups map to joints by name; a vertex weighted to no bone -> "neutral bone" added at the armature root (primitive_extract.py:1672-1675) sure; influence cut/renormalise unsure.
- AR:261-266 three.js: SkinnedMesh per primitive when skinned; normalizeSkinWeights(); joint nodes -> new Bone(); one Skeleton; every SkinnedMesh under the skinned node bound to that one skeleton (mesh.bind(skeleton, identity)).
- AR:267-271 several materials -> Group of one mesh per primitive sharing one Skeleton.
- AR:272-278 names: sanitizeNodeName replaces spaces with _ and removes [ ] . : / ; "hand.L" -> "handL"; original in userData.name; materials keep names. (=> `_l`/`_r` survive; Blender ".L" would not.)
- AR:289-292 bones on node path: uniform buffer if fits, else bone texture; no hard cap on dev.
- Next: B7b graphics-engineer draft (armature/root, names, sockets).

## B7b DONE: graphics-engineer draft (.checkpoints/cs0-model-rules/asset-export-rules.draft.md, "GD")
- GD:103 export_leaf_bone default False, rule False. GD:108-111 AX-F8 names written as they are; bone -> joint node; armature object names the skin.
- GD:115-118 AX-F10 neutral bone at armature root for unweighted vertex (confirmed); influence cut unsure.
- GD:119-121 AX-F11 location swizzle (x,y,z)->(x,z,-y) CONFIRMED for locations; ROTATIONS INFERRED. (=> my Blender socket-axis rule must be [inferred] and proven by mark 7's forward-axis test.)
- GD:132-137 AX-F13 joints -> Bones; all SkinnedMeshes of the node bind one Skeleton; missing joint warns.
- GD:145-146 AX-F16 empty node -> Object3D (loader:4493): a socket loads as named Object3D.
- GD:201-206 AX-R10: (old 13+root list) "`root` is the armature's top bone, not the armature object and not an empty (S:73's 'one empty at the origin' becomes this bone); vertex groups carry the bone names exactly."
- GD:207-213 AX-R11 sockets empties; open: which sockets per gun; socket axis; proposed: gun's entry lists them; socket points along local +Z.
- GD:224-227 AX-R14 one mesh object: parts joined before export, parented to one armature, each part's vertices weighted 1.0 to its bone (rigid skin) until someone paints weights; "one object per part" (S:73) is the modelling form.
- GD:228-231 AX-R15 every vertex 1-4 bones summing to 1; unweighted vertex -> neutral bone; rest position = bind pose, author chooses pose; gun has no armature/skin.
- GD:249 mark 3 not confirmed: the neutral bone's name; WHETHER THE ARMATURE OBJECT BECOMES A JOINT. GD:316-317 not read: whether armature object becomes node or joint; how an object parented to a bone is written.
- GD:264-267 point 2: rifle has four sockets; mark 7 checks the sockets the gun's entry lists (or rifle gains eject).
- GD:290-295 draw-cap route (b): gun geometry merged into body's unit_dark geometry weighted 1.0 to `hand_r` (=> hand_r must carry the gun's pose exactly; my hand rule must work for a separate gun node AND for route (b)).
- GD:321 14 bones x 64 B = 896 B uniform path (limit unread).
- Next: B8 legacy code: palm/fist measure game:241-270, hand IK 405-416, joint map core:454 + CHAR_MALE joints, weapon mount core:694-698, leg IK 345-360, coat 439-443, GA7 361-366.

## B8 (in progress) legacy code facts (legacy/shamble-game.js, "G")
- G:242 joints read by name from g.userData.joints: kneeL/kneeR, hipL/hipR, handR/handL, elR(/elL), shR/shL, spine, chest, neck (G:364-366), pelvis Pl.
- G:243 `let palm=0.08; // fist centre beyond the wrist joint (so the fist, not the wrist, lands on a socket)`; G:244-247 measured = mean world distance of handR's child meshes from the handR joint (skipping sword/axe/shield), clamped 0.03..0.2; ONE value from the RIGHT hand used for both.
- G:250 lengths from joint POSITIONS assuming limbs hang along -Y: thigh=|kneeL.position|; shin=pelY+hipL.y-thigh; armU=-elR.position.y; armF=-handR.position.y. (=> hidden coupling: v2 must read lengths as joint-to-joint distances in the bind pose; Blender rolls/rest poses are not -Y; CB:36 arm hangs 12 deg out).
- G:253-264 procedural ankle: pivot at the top of the foot under the knee; foot/bt_foot/bt_cuff parts re-hung; A.leg {h: foot height, len: knee->ankle}.
- G:265-268 coat panels gc_skFL/FR/BL/BR found by pid; [side, front]; material clone (GA13 avoid).
- G:351-352 pelvis Pl position (sway, pY) + rotation (pYaw, roll) every frame (=> hips moves; cannot be the fixed top).
- G:356-358 leg IK: target = ankle above the sole; solveArmIK(hip, knee, hip.position, target, pole _aKnee, 1, A.thigh, Lg.len) (=> chain hip->knee->ankle: upper_leg, lower_leg, foot).
- G:359-360 ankle pivot quaternion: flat, foot pitch/yaw + toeOut (=> foot joint at the ankle).
- G:364 spine.rotation = (lean + bayonet lean*0.4, twist*0.45, 0); G:365 chest.rotation = (breath - kick + dip*0.13, twist*0.55, stride roll); G:366 neck.rotation = (-breath*0.6 - lean*0.5 + kick*0.07 + dip*0.15, -blade*0.85, 0) (=> head hangs on the NECK joint: v2 `head` joint pivot sits at the neck).
- G:413 hands keep their rest rotation (hqR/hqL) during the hold. G:415-416 arm IK: solveArmIK(shR, elR, shR.position, target, poleR, 1, armU, armF + palm) => the second segment is forearm + palm: the FIST (on the forearm's extended line) lands on the socket, not the wrist.
- G:438 flinch: spine.x += fp*pitch; chest.x += fp*(1-pitch); chest.z += fr*roll; neck.x -= fp*head.
- G:441-443 coat panel k: rotation.x from the angle of the THIGH on its side (hipL/hipR quaternion applied to down), front/back gains, flare at speed, clamped, spring.
- Brief check (computed): forearm elbow (0.255,0,1.14) -> wrist (0.31,0.03,0.875) length 0.272, unit (0.202,0.110,-0.973); hand box centre (0.32,0.04,0.82) is 0.057 from the wrist and lies on the forearm's extended line (wrist + 0.057*unit = (0.3215,0.0363,0.8197)). => legacy's fist-on-forearm-line rule fits the brief's rest pose.

## RESTART 2 (2026-11-17, after the cut at about 19:55 UTC)
- Last block marked done: B7b. B8 was in progress (not marked done): redo it with bounded greps, written over. Then B8b (build plan: cs10 row l.271-272, Part 11 rule 21, Part 12 l.1059, for section 7), B9 write joint-list.md in blocks (one append per section), B10 DONE.
- The restart message said the notes end at "B5 done; next B6"; the file shows B6, B7, B7b done: trusting the file.
- Restart rules: open nothing under /root/.claude/uploads/; never commit or push; final answer at most 25 lines. No Edit tool in this run: notes and result file are written with shell appends.
