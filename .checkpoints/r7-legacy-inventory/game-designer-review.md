# Task: R7 legacy inventory, game-designer review of rules rows (GE all + rule rows of GC GN GW CD GP) against LOOP_V2
Context: LOOP_V2 [OPEN] tags only at §5 (Spy cost + spy details), §9 (aiming at height, map size). No [OPEN] on economy/eras/win/spawn.
Rows in scope: GE1-17; GC1,2,4?,9,10,11,17,19; GN2,3,9; GW1,2,10,20; CD4,7,10,11,12,13,14; GP11,(GP3,GP14).
GE1: sed game.js:40-44; RULES_DEFAULT matches tuning json. OK.
GE2: sed game.js:688-704, core:81; ECO numbers match json. OK.
GE6: sed 845-876: refinery-dead self-destruct, respawn 12 s, 600 cr only to player, 5 tech. OK.
