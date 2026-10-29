# Task: R4 wave 2 fix step (apply verifier spot-check to docs/research/r4-physics-navigation.md)

- Started 2026-10-28. Step 0: save the verifier report verbatim to verifier-r4-result.md (next).
- Step 0 done: verifier-r4-result.md written verbatim. Next: read topic file, verifier notes, plan condition 8.
- Read topic file (1223 lines), verifier notes (DONE, 49/51), R3's part 7 format. Plan: W1 PHYS-F20 fix (verifier quotes, no new fetch), W2 NAV-F15 drop hashbrown-default clause, W3 FLOORS-F5 18+INVALID; C1 REC1 reason + mark 6 shape pairs; C2 mark 1 speed 13.3 m/s (check code lines); C3 §6.1 -> §3 traces; C4 OBST-REC3 conditional; C5 NAV option A + F8 + TL;DR; C6 absences inferred; C7 REC4 turning; C8 REC3 floor filter; C9 Rests-on; C10 F2 caveat; C11 vehicle count; C12 nits. No re-sourcing planned (verifier's quotes suffice).
## Change log (type · what)
- Resumed (second run, 2026-10-29). Topic file untouched so far (git diff empty). Re-read file, verifier notes, R3 part 7 format, R1-XU-OQ1 (P1 has 4 vehicles, ECS-REC1 mark 1 has 8; synthesis decides). Checked legacy code: shamble-core.js:128-137 sprintMult 1.214/1.165/1.62/1.214; shamble-game.js:2756 bayonet cs=T.moveSpeed*unitOf(2).sprintMult*1.3; :2774 banzai sp*1.3; bots T.enemyMoveSpeed 4.2 x speedMult <=1.5 (6.3 m/s). Fastest = 6.3*1.62*1.3 = 13.27 m/s (0.22 m/tick 60 Hz, 0.44 m 30 Hz). Decision: no new fetch; fn.cast_shapes quotes come from the verifier (added as source #47, read by the verifier). Now editing in file order.
- Unit 1 done (findings part, 14 replacements via scratchpad apply.py): TL;DR Detour line; PHYS-F2 caveat; PHYS-F15 count; PHYS-F20 fixed (supported pairs, verifier's Unsupported quotes, source 47); PHYS-F22 upgraded to confirmed; PHYS-F27/NAV-F16/LOS-F5 relabelled inferred; NAV-F2 units; NAV-F8 C-capable targets; NAV-F15 clause dropped; FLOORS-F5 18+INVALID; OBST scope line §6.4 wording. Next: options and recommendations.
## Change log (type · what) [running]
- recommendation · TL;DR: "cannot join the browser sim" -> "does not fit R1's browser target" (C5)
- nit · PHYS-F2: autostep cost warning added (C10)
- nit · PHYS-F15: "counting both ends and one beta" (C12)
- recommendation · PHYS-F20 corrected (W1): supported pairs only; Unsupported quotes; distance tests dropped (no quote); basis of PHYS-REC1/3/4
- nit · PHYS-F22: inferred -> confirmed with the verifier's exact words
- nit · PHYS-F27, NAV-F16, LOS-F5: "confirmed (absence)" -> "inferred" (C6)
- nit · NAV-F2: voxel units (C12)
- nit · NAV-F8: the two C-capable targets quoted (C5)
- recommendation · NAV-F15 (W2): hashbrown-default clause dropped (basis of NAV-REC1)
- number · FLOORS-F5: 19 -> 18 ways plus TRAVEL_INVALID (W3)
- nit · OBST scope line: §6.4's own words; "moving obstacles" inferred (C12)
- Unit 2 done (options and recommendations, 30 replacements): NAV option A cells; PHYS-REC1 reason, Rests on, marks 1/5, new mark 6; PHYS-REC2/3/4; NAV-REC1..4; FLOORS-REC2; LOS-REC2; OBST-REC1/2/3; wave-1 list (P1 count); pointers (ADR-0002 target; tile re-bake conditional). Next: part 7, sources (#47), header status and totals.
- recommendation · NAV option A: C-capable targets named; blocker holds only for R1's target (C5)
- nit · NAV option table determinism and effort cells reworded (NAV-F15, F8)
- recommendation · PHYS-REC1 reason: "any two shapes" -> supported pairs; autostep cost (W1, C1, C10)
- number · PHYS-REC1 mark 1: sprint speed 9.0 m/s -> fastest soldier 13.3 m/s (0.22/0.44 m per tick) (C2)
- recommendation · PHYS-REC1 mark 5: autostep on; new mark 6 (shape pairs, 0 Unsupported) (C1, C10)
- nit · PHYS-REC1 Rests on: tuning sprintMult -> legacy GP3
- nit · PHYS-REC2 Rests on: + F12, GN11 (C9)
- recommendation · PHYS-REC3 reason: supported pairs (W1); Rests on + LOOP_V2 §5, REC1 mark 6 (C9)
- recommendation · PHYS-REC4: turning (nonlinear cast or rotate-then-sweep), mark adds turns; Rests on + F13 (W1, C7)
- recommendation · NAV-REC1 reason: Detour/target; hashbrown clause dropped; voxel rounding (W2, C5, C12)
- nit · NAV-REC1..4, LOS-REC2, OBST-REC1 Serves: §6.1 -> VISION_V2 §3; NAV-REC4 + §6.2 (C3) [6 lines]
- number · NAV-REC2 mark: P1 with 8 big vehicles instead of 4; Rests on + R1-XU-OQ1, R4-PHYS-REC4 (C11)
- recommendation · NAV-REC3: neighbours filtered by floor layer; new floor mark; Rests on + R4-FLOORS-REC2 (C8)
- nit · FLOORS-REC2 Rests on: + LOOP_V2 §2, §9 (C9)
- nit · OBST-REC1 Serves: §6.4 words, moving obstacles inferred (C12)
- nit · OBST-REC2 Rests on: + LOOP_V2 §3 (C9)
- recommendation · OBST-REC3: tiling conditional on NAV-REC1 mark 1 failing; Serves says building is not a reason (C4)
- nit · wave-1 list: P1 vehicle-count line added (C11); pointers: ADR-0002 target line (C5), tile re-bake conditional (C4)
