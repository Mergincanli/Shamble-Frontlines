Task: R7 legacy harvest, reader for area CD (core data and persistence, shamble-core.js 1-232) at 48ffb51
Checks done: line numbers verified by grep/sed on checkout (HEAD dcd4dcd = 48ffb51 + .checkpoints only).
T has 99 keys; 18 never read as T.<key> in core/game/editor/index/editor html: camLookLean camPosLean maxLean lookSideGain posFwdGain posSideGain sprintMult enemyCount eliteCount enemyBurst enemyDamage enemyPreferRange incomeBase incomeRefinery meleeDamage meleeRange meleeCooldown shieldCost (perfbots.js comment mentions enemyCount for older builds).
Boot overwrites: T.arenaX/Z core:225; applyRules game:43 (resupplyCost); applyCamStyle game:2202; dev sliders game:3326-3333.
KVStore users: game ProgStore shamble_prog (KVStore.set game), editor vmap:/vprefab:/shamble_charlab (editor:444,696-708).
Weapon fields with 0 reads by grep: .short; vehicle .seats .desc 0 reads.
Rows (id | mark | lbp):
CD1 manifest header core:1-27 | drop | F
CD2 KVStore API + localStorage/memory core:28-65 | port TS | F
CD3 KVStore window.storage branch core:37,41,44,49,54,59 | drop | F
CD4 T movement/health/respawn core:67,76,77,82,225 | Rust | F
CD5 T camera framing + PerspectiveCamera core:68-75,161 | port TS | F
CD6 T bot tuning core:78-80 | Rust | F
CD7 T economy + structure HP core:81,86 | redesign LOOP_V2 §3,§8 | F
CD8 T grenade/melee/shield/locational core:83-85,88 | Rust | F
CD9 T grenade VFX + cam kick core:89-92 | port TS | T
CD10 TEAMS core:94-99 | Rust | F
CD11 TUNE core:100-107 | Rust (spawn sub-table redesign §2) | F
CD12 WEAPONS core:108-122 | Rust | F
CD13 UNIT_TYPES/unitOf core:124-140 | Rust | F
CD14 VEHICLE_TYPES core:141-144 | Rust | F
CD15 renderer core:146-147,156-158 | drop | T
CD16 halftone core:148-155 | redesign (look brief pending) | T
CD17 scene/fog/lights/sun shadow core:160,163-167 | redesign (look brief pending) | T
CD18 noRay core:168 | drop | F
CD19 ENV system core:169-183,203-223 | redesign (look brief pending) | T
CD20 grass texture + ground ramp core:184-201 | port TS | T
CD21 ground/sea/beach + arena size core:224-231 | port TS | T
DONE
