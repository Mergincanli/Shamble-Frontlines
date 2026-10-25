R7 legacy inventory, reader GN (game bots and navigation, shamble-game.js 1434-1653 + NAV 2609-2965)
## code read (engine files unchanged 48ffb51->HEAD; line numbers verified)
- SOLDIERS 794, enemies 793, spawnEnemyMesh 795-800 (buildSoldier + per-bot PlaneGeometry/MeshBasicMaterial hp bar), randSpawn 801 DEAD, teamSpawnPoint 806-818, respawnPos 819 DEAD, setEnemyType 820 (unit, maxHealth, setHeldGun, tints head mat with unit accent)
- roster creation 3205-3218 (en object fields incl dead state/wander/wanderT), activateRoster 762-779 (GC owns; sets role/retargetT stagger/rotatePush 769-770)
- siegeFireHarvester 852-856 (harvester region, but bot fire)
- nearestEnemyStructure 1566-1572; nearestFriendlyBuilding 1573 DEAD; EJUMP_REACH 1575; AI_STATE 1581-1583 (budget/checks/scans/step + rotatePush); acquireTarget 1584-1599; rampLowEnd 1600 DEAD (inlined at 1604); enemyClimbSteer 1602-1606
- eM/_eF/_eT/_eEnd/_zAxis 1609; _eHitN/_botFireWorld 1610; enemyFire 1611-1638; _botShotD2 1640-1641; siegeFire 1642-1649
- NAV header 2609-2616, NAV 2617, _navP/_navDir/_NDX/_NDZ 2618, navBuild 2619-2626, _navCell 2627, navClear 2628-2629, _navPush 2630, _navPop 2631-2632, _navNearestFree 2633-2635, navPath 2637-2656 (8000 cap 2644, string-pull 24 lookback 2654), navSteer 2658-2670 (replan: n==0||goal moved>3m||t>1.5s||stuck(0.8s,<0.35m)) ; NAV.budget=2 in update 2671
- bot loop in update 2888-2941: budget refill 2889, rotating start 2892-2893, combat 2901-2913 (preferRange +-3, strafe, lunge 2911, burst fire 2913), objective role 2915-2917, siege 2919-2928, clamp/jump/resolveObstacles/tickVertical 2931-2933, hitflash traverse 2934, hp bar lookAt 2935, anim 2936, death+respawn 2938-2939 (respawn x1.4 faster with refinery alive; random unit 0..2 -> Officer idx3 never used by bots)
- navBuild calls: deploy 1107, boot 3368 + warm-up 3371; clearMap 559 NAV.ok=false
- dead: randSpawn 801, respawnPos 819, nearestFriendlyBuilding 1573, rampLowEnd 1600, en.state/wander/wanderT (written 1519/1842/2939/3212, never read), T.enemyDamage/enemyPreferRange/enemyBurst/enemyCount/eliteCount core 78-80 (game unread), elite branches (elite always false, D6)
- tuning: core 78-80 T.enemy*, TUNE.ai 104, TUNE.spawn 105, TUNE.fx 106, UNIT_TYPES bot fields 126-139
## evidence
- bot-poplar-wedge report: pinned starts 39->0; sweep wedged 112/43 -> 0/0; seeded 240 pairs stall 9.12->8.50; sim.js 40 runs 8.82->7.73; navBuild 42.8 ms Cabana /15.5 Romania; nav ~0.02 ms/frame; deopts 134->1 warmup; PERF-2 replan storm budget-limited frames 0.9->1.8%; R1 gaps 3-5.4 m; 8000 cap
- AUDIT CS13: A* 0.26 ms each <=2/frame; 30 m wall 25 s stuck -> 9.9 s; stall 9.9->6.2 mean
- three-teams-core report: fairness scans 3.57/1.00/0.03 -> 2.39/2.42/2.25; 144fps 7-12% faster fixed; LOS 480/s caps ~1100/s demand at 48 (2.3 scans/s vs 4); reach Cabana 72.5% objective/75% siege (R19), DM_Tri 100%; FX cull identical damage logs; perfbots 48: 0.80-1.00 ms/step
- team-prep-fixes: B1 bot shot 30-93us 3.7-7.0KB -> 5.5-12us 0.88-1.0KB; hit rate 27.6/4.2/1.8 -> 20.0/2.9/1.2%; NC-1 fixed in 3a
- STATUS: hasLOS 103 B/call; risk1 R19; N1 editor Play never rebuilds nav; R1; replan storm; 8000 cap
- LOOP_V2 §2 spawn inside barracks, §5 units (Recruit, Officer T2), §10 bots roles 40/30/30, replan storm fix; §9 6 m gap rule; no [OPEN] on bots
- VISION_V2.md not in checkout
## extra findings (code)
- update(dt) variable frame dt (loop 3053, capped 0.05, hitStop freezes bots); NAV.budget=2 reset per update() call (2671) -> route searches per second scale with frame rate (unlike LOS budget, which is per game second)
- AI uses Math.random (strafeT, burst pause, fireCd, respawn unit, spread): 8 calls in bot loop 2895-2941
- enemyFire miss tracer end uses T.enemyRange 60 (1629), not the unit's eu.enemyRange (24..80)
- siegeFire / siegeFireHarvester apply flat T.enemySiegeDamage 11 with no ray or LOS (1648, 856); lunge 2911 applies melee by distance <3.2 only
- hasLOS (core 1544) tests SIM.statics incl ramp slots, bot fire excludes ramps (1462): sight and shot use different worlds
- hit flash 2934: traverse with arrow closure, mutates every mesh emissive on toggle
## rows (final)
GN1 bot body + overhead bar + hit flash (795-800, 2934-2935) C port TS look=true
GN2 roster record + unit assignment (3205-3218, 820, 2939, 1842) C redesign LOOP_V2 §5/§10 look=true (head tint)
GN3 teamSpawnPoint + respawn timer (806-818, 2939) M redesign LOOP_V2 §2
GN4 LOS budget + acquireTarget + fair order (1576-1599, 2889-2893) M Rust
GN5 combat movement + lunge (2901-2911) C Rust
GN6 enemyFire + burst cadence (1609-1629, 2912-2913) M Rust
GN7 _botFireWorld ramp-less view (1610, 1455-1462) M drop
GN8 bot-shot FX/sound cull (_botShotD2 1640, 1630-1637, 1645-1647, 853-855) M port TS look=true
GN9 siege + objective roles (nearestEnemyStructure 1566, rotatePush 1582, roles 770, 2914-2928, siegeFire 1642, siegeFireHarvester 852) M redesign §10/§4
GN10 verticality (EJUMP_REACH 1575, enemyClimbSteer 1602, jump/clamp/collide 2931-2933) C Rust
GN11 navBuild grid (2609-2626, 559, 1107, 3368) M Rust
GN12 A* + string pull (2627-2656, warm-up 3369-3371) M Rust
GN13 navSteer + replan + route budget (2657-2671) M Rust
GN14 dead code C drop
DONE
