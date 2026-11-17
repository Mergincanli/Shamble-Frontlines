# The Art lead's correction of 15 November 2026, 18:40: the gun poses, restated from the sockets

Saved by the Advisor on Tuesday 17 November 2026, 20:58 (Amsterdam time), from Mustafa's paste of 20:54 ("its been 2/3 days, idk if I had already given this art director block"). It had not reached the Advisor or the build chat before. Everything below the line is the Art lead's text, word for word as pasted.

## The Advisor's check

- **What it is:** the Art lead's answer to the Advisor's block of 15 November, 17:50 (the finding that the rifle poses did not fit the rifle file's sockets, and the SMG brief's three small things). It replaces section A1 of the addition of 15 November (`art/ART_BRIEF_ADDITION_2026-11-15_poses_yard.md`) and the section "How the Conscript holds it" of the SMG brief (`art/ART_BRIEF_2026-11-15_1735_smg_stage1.md`), and corrects that brief's socket names and magazine.
- **It agrees with the build chat's go of 17 November, 20:10:** the hands sit on `socket_grip` and `socket_support`; the butt and the muzzle follow from the gun's file; the socket names carry the `socket_` prefix; the muzzle is where flash and tracer start, and a shot's start is simulation data.
- **Recomputed by script** (`/home/claude/checks/poses_1840.py`: the rifle file's sockets read from `AMK9_v4_stage1.glb`; the SMG's from its brief; no roll on the gun):
  - 14 of the 15 check points hold to 0.01 m.
  - **The one that differs:** the rifle's butt in the aiming pose is given at U 1.44. By the file the butt end runs from 0.072 below to 0.030 above the grip socket, so its top edge is at U 1.39 and its middle at 1.34. By the correction's own rule the file wins.
  - The magazine: three pieces of 0.06 bent 0, 8 and 16 degrees put the far end 0.0249 nearer the muzzle and 0.257 left of the bore line. Holds.
- **Reach of the left arm** (shoulder joint (0.19, 0, 1.44) from the soldier brief; the chest turns about the body's centre; the lean about the belt line), shoulder to the support socket: rifle idle 0.23, aiming 0.59, sprinting 0.21; SMG aiming 0.53, idle 0.56, sprinting 0.49. All inside the 0.62 the correction names. The soldier brief's own arm measures 0.636 from the shoulder joint to the hand's centre.
- **For the body step (the build plan's change-set 10):** the rifle's aiming pose uses 96 % of the arm, so little is left for recoil and the walk. Not computed: how far the SMG's side magazine stays from the left forearm (it depends on where the elbow goes); the correction itself asks the body step to check it.
- **Not checked:** how the poses look. That is for the pictures the briefs ask back.

---

CORRECTION: THE GUN POSES, RESTATED FROM THE SOCKETS. From the Art lead, 2026-11-15, 18:40.
For the builder chat. It replaces section A1 of my addition of 2026-11-15 and the section "How the Conscript holds it" of my SMG brief of 17:35, and it corrects two lines of that brief. Everything here is the Art lead's proposal.

THE RULE
A pose is three things: where the gun's grip socket sits, which way the gun points, and how far the chest turns. The right hand is on socket_grip and the left hand on socket_support. The butt and the muzzle follow from the gun's file. If the left hand cannot reach (the arm is 0.62 from the shoulder joint to the middle of the hand), the pose is wrong, not the gun. Positions are (L, F, U) in the soldier's frame. The "check" values are where the sockets land, for you to compare.

THE RIFLE (AMK9_v4_stage1.glb: support socket 0.452 ahead of the grip, muzzle 0.864 ahead, butt end 0.259 behind)
- Idle: grip at (-0.13, 0.22, 1.06). The gun points to his left and 42 degrees up, across the chest. Chest square.
  Check: support (0.19, 0.22, 1.38), muzzle (0.46, 0.22, 1.70).
- Aiming and firing, also while walking: grip at (-0.13, 0.21, 1.36). The gun points straight ahead, level. The chest turns 45 degrees to his right, so the left shoulder comes forward; the head keeps facing forward.
  Check: butt (-0.13, -0.05, 1.44), support (-0.13, 0.66, 1.39), muzzle (-0.13, 1.07, 1.44). The left arm is almost straight.
- Sprinting: grip at (-0.13, 0.22, 1.10). The gun points to his left and 25 degrees up, across the chest. Chest square, body leaning 10 degrees forward.
  Check: support (0.27, 0.22, 1.32), muzzle (0.62, 0.22, 1.54).

THE SMG (support socket 0.34 ahead of the grip and 0.034 lower, muzzle 0.64 ahead, butt end 0.16 behind)
- Aiming and firing: grip at (-0.12, 0.19, 1.36). Straight ahead, level. The chest turns 30 degrees to his right.
  Check: butt (-0.12, 0.03, 1.39), support (-0.12, 0.53, 1.33), muzzle (-0.12, 0.83, 1.40), the magazine's far end (0.14, 0.48, 1.40).
- Idle: grip at (-0.12, 0.19, 1.32). Straight ahead, tipped 15 degrees down. The chest turns 30 degrees to his right.
  Check: support (-0.12, 0.51, 1.20), muzzle (-0.12, 0.82, 1.19).
- Sprinting: grip at (-0.14, 0.18, 1.18). Straight ahead, tipped 50 degrees up. Chest square, body leaning 10 degrees forward.
  Check: support (-0.14, 0.42, 1.42), muzzle (-0.14, 0.56, 1.69).
Why the SMG's idle and sprint changed: turned to his left, the side magazine swings into the left forearm, and at belt height the left hand cannot reach the grip bump. The body-movement step still has to check that the magazine stays clear of his arms.

TWO LINES OF THE SMG BRIEF, CORRECTED
- Socket names: socket_grip, socket_support, socket_muzzle and socket_sight, as in the rifle file, plus socket_eject.
- The magazine: three straight pieces of 0.06. The first points straight out to the left, the second is bent 8 degrees towards the muzzle, the third 16 degrees. The far end then sits 0.025 nearer the muzzle and reaches 0.26 left of the bore line.

ONE WORDING
Where I wrote "shots leave from there": the muzzle socket is where the flash and the tracer start. The simulation's shot origin is its own data and is not read from the model.
