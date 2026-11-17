BUILD BRIEF: THE CONSCRIPT'S FIRST BLOCKOUT. From the Art lead, 2026-11-14, 16:35. It replaces my brief of 16:20.
For whoever builds it: the builder chat, in code, as the stand-in soldier of Slice 1. A Blender session can use the same list later. This brief covers his shapes and colours only. Movement, shooting and the rig's rules are the build plan's.

WHAT IT IS
- Unit: the Conscript, the tier-one rifleman (bolt-action rifle, Molotov), in the uniform all three factions share: fur hat with ear flaps, scarf over the lower face, long double-breasted coat with a belt, flat shoulder boards, tall boots.
- Build: slim and serious, about 8 heads tall. Wide shoulders, a clear waist at the belt, a coat that flares to the hem. Not chunky. The proportions are measured from the Conscript artwork, which Mustafa calls the style direction.
- Game camera: 35.3 degrees above the ground, 15 degree lens, turned 45 degrees. Headgear, shoulders and the coat's outline matter most.
- The picture of all this is Sheet 16 (s16_engine_soldier.jpg). Where the picture and these numbers differ, the numbers win.

AXES AND SIZE
- Metres. L = to his left, F = forward, the way he faces, U = up. Positions are (L, F, U). Origin on the ground between the feet.
- Size is a placeholder: 1.80 to the top of the hat (180 Unreal units). Hang every part on one root so the whole soldier scales with one number.
- Coarse shapes only: boxes and 8-sided cylinders. "Tapered" means the two ends have different sizes. One part per name below.

HEIGHTS TO HIT (U, and share of the height)
ground 0 | boot top 0.36 (20 %) | coat hem 0.43 (24 %) | fists 0.77 (43 %) | belt 1.045 to 1.10 (60 %) | shoulders 1.49 (83 %) | chin 1.54 (86 %) | brow 1.66 (92 %) | top of hat 1.80
WIDTHS TO HIT (L)
shoulders over the arms 0.49 (27 %) | chest 0.38 | waist at the belt 0.33 (18 %) | coat hem 0.61 (34 %) | hat 0.235 | hat with ear flaps 0.28

MATERIALS: 4 flat colours per soldier, the rifle included. WORKING NAMES; the build plan's names replace them.
           Soviets    Allies     Empire
Coat       #A72123    #235DBD    #F5F3EA
Top        #D41414    #1F6FF0    #890089
Trim       #8F6F30    #FFFD82    #8F6F30
Dark       #4D4843    #4D4843    #4D4843
Mustafa's: all Coat and Top values, the Soviet and Allied Trim. The Art lead's: the Empire Trim and Dark.
No skin, leather, wood or steel: hands, face strip, belt, scarf, boots and rifle are all Dark (Mustafa, 14 November: no skin on this first stand-in).

PARTS, biggest first. Size is width (L) by depth (F); U is from-to. Left parts are given; mirror them for the right. Each line ends with its material and, for a rigid skin, the bone it follows (working bone names).
1. coat_skirt. Tapered. U 0.43 to 1.045. At the hem 0.61 by 0.46; at the belt 0.33 by 0.23. Coat. Hips.
2. torso. Tapered. U 1.10 to 1.47. At the belt 0.33 by 0.22; at the top 0.38 by 0.25. Coat. Chest.
3. belt. U 1.045 to 1.10. 0.345 by 0.245. Dark. Hips.
4. boot_L/R. Tapered, U 0.10 to 0.50 (the top is hidden in the coat), 0.12 across at the ankle, 0.145 at the top, centre at L 0.11. Dark. Leg (one piece from hip to ankle).
5. foot_L/R. Box 0.11 by 0.28, U 0 to 0.10, heel at F -0.08, toe at F 0.20. Dark. Foot.
6. trouser_L/R. Cylinder 0.165 across, U 0.36 to 0.50, round the boot top. Dark. Leg.
7. Arm. Joints in the rest pose: shoulder (0.19, 0, 1.44), elbow (0.255, 0, 1.14), wrist (0.31, 0.03, 0.875). The arm hangs clear of the body, 12 degrees out.
   upper_arm_L/R: tapered, shoulder to elbow, 0.115 across at the shoulder, 0.10 at the elbow. Coat. Upper arm.
   forearm_L/R: tapered, elbow to wrist, 0.10 to 0.09. Coat. Forearm.
   cuff_L/R: 0.125 across, on the last 0.07 of the forearm. Wider than the forearm on purpose. Coat. Forearm.
   hand_L/R: box 0.09 by 0.10, 0.10 high, centre (0.32, 0.04, 0.82). Dark. Hand.
8. board_L/R. Flat box 0.17 (L) by 0.09 (F), 0.015 thick, lying on the shoulder, centre (0.165, 0, 1.485). Trim. Chest.
   board_plate_L/R: flat box 0.13 by 0.05 on top of it, so a 0.02 rim of Trim shows all round. Top. Chest.
9. collar. 8-sided, 0.20 across, U 1.45 to 1.51. Coat. Chest.
10. scarf. Tapered 8-sided, U 1.49 to 1.615, 0.15 across at the bottom, 0.17 at the top. Dark. Head.
11. head. 8-sided, 0.18 by 0.21, U 1.54 to 1.70. Only the strip from 1.615 to 1.66 shows. Dark. Head.
12. hat_band. 8-sided, 0.235 by 0.25, U 1.66 to 1.76. Coat. Head, as are 13 to 15.
13. hat_crown. Tapered, U 1.76 to 1.795, 0.225 across at the bottom, 0.20 at the top. Coat.
    hat_top: a disc 0.19 across on its flat top, U 1.795 to 1.80. Top.
14. hat_flap_front. Slab 0.19 wide, 0.02 thick, U 1.665 to 1.75, on the front of the band at F 0.13. Dark.
15. ear_flap_L/R. Slab 0.03 (L) by 0.12 (F), U 1.55 to 1.72, centre at L 0.125. Coat.
16. Small parts, may be left out. Pouch: box 0.12 by 0.06, U 0.88 to 1.04, on the coat's front at his left hip, centre (0.13, 0.155), Dark, hips. Buckle: 0.07 wide, U 1.04 to 1.105, front centre of the belt, Trim. Buttons: two rows of four, 0.026 squares at L 0.05 and -0.05, U 1.17, 1.25, 1.33 and 1.41, on the front of the torso, Trim.
17. Rifle. Stand-in: 1.12 long, 0.06 wide; butt 0.26 behind the grip, muzzle 0.86 ahead of it; 0.13 thick at the butt, 0.075 at the grip, 0.055 at the fore-end, barrel 0.03. Dark. Or load AMK9_v4_stage1.glb, which has grip, support, muzzle and sight sockets.
    Idle hold: across the chest, in a plane 0.22 in front of the body's centre, rising 42 degrees from the right hand at (-0.13, 0.22, 1.06) towards the left shoulder; the left hand under the fore-end at (0.21, 0.22, 1.32). Other poses are the build plan's.

MARKINGS (may be left out at game size, where they are about 4 by 3 px)
- One on each board plate, radius 0.022, and one on the hat's front flap, radius 0.03, centre at U 1.708.
- Soviets: star; Trim on the boards, Top on the hat. Allies: chevron; Trim on both. Empire: flower; the Coat's white on both.
- Shapes on a grid from -1 to 1. Star: 5 points, inner corners at 0.42 of the outer radius. Chevron, 6 corners: (-1, -0.15) (0, 0.85) (1, -0.15) (1, -0.85) (0, 0.15) (-1, -0.85). Flower: 5 round petals out to 1.0, dipping to 0.4 between them, with an open round centre of radius 0.27.

MUST READ AT GAME SIZE
The outline (fur hat, long coat that widens to the hem, rifle across the chest); the coat colour; the Top colour on the hat top and both boards; the Trim rim round each board; the black line round the whole figure.

SEND BACK, for the Art lead's check
1. Numbers: the soldier's bounding box in the rest pose without the rifle (expected about 0.73 wide over the hands, 0.46 deep, 1.80 high); shoulder width over the arms (0.49); body width without the arms at the belt (0.345) and at the hem (0.61); triangle count. No triangle budget is set yet, so only report it.
2. Pictures on a plain grey ground: front, side, back and top without perspective; and the game's own camera at 1080p with four copies in a row, 1.5 m apart, each a quarter turn from the next, so one faces the camera, one faces away and two show their sides. Send the full frame and a crop of the four enlarged 3 times. Each should be about 148 px tall.
The Art lead answers "pass" or a numbered list of fixes.

DO NOT
- No textures, no shading work, no extra parts, no rounding or smoothing before the check passes.
- No model file in the game's repo.

IF THIS IS BUILT IN BLENDER LATER
Z up, the soldier faces -Y, his left is +X, so (L, F, U) is (X, -Y, Z). Parent all parts to one empty at the origin, apply scale, one object per part, four materials with the working names. Game camera: vertical field of view 15 degrees, at (23.50, -23.50, 24.44), aimed at (0, 0, 0.9), 1920 by 1080. No export until the export settings are decided.