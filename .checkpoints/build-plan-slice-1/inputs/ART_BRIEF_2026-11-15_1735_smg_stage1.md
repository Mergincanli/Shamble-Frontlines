BUILD BRIEF: THE SMG, STAGE 1 (BLOCKOUT). From the Art lead, 2026-11-15, 17:35. It replaces my SMG brief of earlier today.
For the builder chat. A second gun beside the rifle (AMK9_v4_stage1.glb), built the same way and to the same conventions, so the two can be swapped in the soldier's hands. Mustafa gives you the line drawing it comes from; that drawing is his chosen design. Where the drawing and these numbers differ, the numbers win. All sizes are the Art lead's and are placeholders that scale with the soldier.

WHAT IT IS
- A submachine gun with a rifle-style stock, a fat barrel jacket, a long curved magazine that sticks out to the gun's left side, and a small grip bump under the gun where the barrel starts. The magazine on the left and the grip bump are Mustafa's words of 2026-11-15.
- In Slice 1 the Conscript carries it; Mustafa judges its feel first.
- Total length 0.80 m (the rifle is 1.12 m).
- Four shapes make its silhouette: the jacket, the receiver tube with its magazine housing, the side magazine, and the wedge of the stock.

CONVENTIONS, THE SAME AS THE RIFLE FILE
- glTF 2.0, 1 unit = 1 m, +Y up, +Z to the muzzle, +X to the gun's left. Origin at the grip socket.
- One material: Dark (#4D4843), a working name. The soldier's Dark material is the same one, so soldier and gun stay within 4 materials.
- Names with letters, digits and underscores only.
- If it is built in Blender instead: Z up, the muzzle towards -Y, origin at the grip socket, all transforms applied.
- Coarse shapes only: boxes and 8-sided cylinders. Report the triangle count; no budget is set yet. The rifle has 2,596.

HOW TO READ THE PART LIST
"d" is the distance from the muzzle tip along the barrel, in metres. The bore line is the barrel's centre line. Up, down, left and right are the gun's own, seen from behind it. Round parts are given by their diameter; boxes as width (left-right) by height.

PARTS, biggest first
1. receiver. Cylinder, 0.042 across, d 0.28 to 0.63, on the bore line.
2. jacket. Cylinder, 0.05 across, d 0.11 to 0.28, on the bore line. Fatter than the receiver on purpose. No holes at this stage.
3. stock, in three pieces that touch:
   fore_end: box 0.04 by 0.032, d 0.42 to 0.61, its top 0.018 below the bore line.
   wrist: tapered, 0.036 wide, d 0.61 to 0.675; 0.04 high at the front, 0.036 at the back, sinking so its centre goes from 0.032 to 0.04 below the bore line.
   butt: tapered, 0.036 wide, d 0.675 to 0.80; 0.04 high at the front, 0.10 high at the butt end, with its top edge just below the bore line.
4. mag_housing. Box, d 0.32 to 0.42, 0.045 high, centred on the bore line, reaching from the gun's centre to 0.08 on its left.
5. magazine. A flat curved slab sticking out to the left from the housing, level with the barrel, from 0.08 to 0.26 left of the bore line; 0.045 along the gun, 0.02 high. Build it from three straight pieces, each bent 8 degrees further towards the muzzle, so its far end sits about 0.035 nearer the muzzle. A floor plate closes the end: 0.05 by 0.026, 0.008 thick.
6. collar. 8-sided block, 0.058 across, d 0.28 to 0.32, between jacket and housing.
7. fore_grip. The grip bump: a box 0.03 wide, d 0.28 to 0.32, hanging 0.06 down from the collar's underside. The left hand holds it.
8. barrel. Cylinder, 0.02 across, d 0 to 0.11. muzzle_collar: cylinder 0.034 across, d 0.035 to 0.07.
9. trigger_guard. A thin plate 0.011 wide, d 0.56 to 0.61, hanging 0.03 below the fore-end.
10. Small parts, may be left out at game size: front sight (0.015 wide, 0.028 high, at d 0.09, on top); rear sight (0.022 wide, 0.018 high, at d 0.56, on top); bolt handle (0.014 across, 0.028 long, pointing right, at d 0.37).

SOCKETS: empty nodes, named exactly as in the rifle file, plus one new one
- grip: d 0.64, on the centre, 0.036 below the bore line. This is the origin. The butt end is 0.16 behind it, the muzzle 0.64 ahead.
- support: on the fore_grip, d 0.30, on the centre, 0.07 below the bore line.
- muzzle: d 0, on the bore line.
- sight: d 0.56, 0.04 above the bore line.
- eject (new): d 0.37, 0.028 to the right of the bore line, pointing right and 20 degrees up. Shells leave here, on the side away from the magazine.

HOW THE CONSCRIPT HOLDS IT. Positions are (L, F, U) in the soldier's frame, as in the soldier's brief. The left hand is always on the support socket.
- Aiming and firing: level, pointing straight ahead. Butt at (-0.12, 0.05, 1.40), right hand on the grip at (-0.12, 0.21, 1.36), left hand on the grip bump at (-0.12, 0.55, 1.33), muzzle at (-0.12, 0.85, 1.40). The magazine then sticks out to his left, about 0.48 in front of him.
- Idle: the same grip, lowered to belt height, turned 30 degrees to his left and tipped 20 degrees down. Right hand at (-0.14, 0.20, 1.08).
- Sprinting: level, turned 45 degrees to his left and pulled close. Right hand at (-0.12, 0.22, 1.15). The body leans 10 degrees forward.
Keep the magazine clear of his body and arms in every pose. The movement plan may change these poses.

MUST READ AT GAME SIZE
The magazine sticking out to the side; the fat jacket at the front; the wedge of the stock at the back; a gun clearly shorter than the rifle.

SEND BACK, for the Art lead's check
- Pictures on plain grey: side, top, front, and the game's camera with the Conscript holding it in the aiming pose, beside the same soldier with the rifle. Full frame and a crop enlarged 3 times.
- Numbers: total length (0.80), the magazine's reach to the left (0.26), triangle count.
The Art lead answers "pass" or a numbered list of fixes.

DO NOT
No holes in the jacket, no textures, no second colour, no bevels or smoothing, no parts that are not listed, before the check passes.