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
