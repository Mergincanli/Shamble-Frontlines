netcode-architect review of R7 simulation rows (CS, GC, GN, GW movement/collision/vertical, CD sim tuning, GP main loop/input/dispatch) for docs/LEGACY_INVENTORY.md
Rows in scope: CS1-11, GC1-19, GN1-14, GW6 GW7 GW9 GW13 GW14 GW15 GW16 GW20 GW21, CD4 CD6 CD7 CD8 CD10-14 CD21, GP2 GP3 GP9 GP13 GP14 GP15 GP16.
Engine files unchanged since 48ffb51 (git diff --stat empty). core wc 1543 lines + last line w/o newline (hasLOS 1544).
## Cross-cutting checks
- wall clock: grep performance.now/Date.now game = 10+1 = 11 (STATUS 11 holds). Sites 65 comic cd, 957 tbox bob, 2173 perf, 2253 sfxImpact throttle, 2674 _animClock (anim only), 2781/2947 tt -> animateJuggernaut only, 3294 dev wheel. NONE decides a sim outcome directly; they matter because cosmetic code gated by them draws from shared Math.random (sfxImpact 2253->2254 noise buffer sampleRate*0.04 draws; sfxEnemyShot 2256 sampleRate*0.09; comicKill 68; FX 1029-1088; camShake 2846 inside update()).
- Math.random sim sites: 1477 player spread, 1619 bot spread, 1827 respawn x offset, 1842/2939 unit pick + fireCd, 2904 strafe, 2913 burst pause, 2923/2927 siege cadence, 3212 init fireCd/strafeDir/wander, 899/919/922 TBOX next point (GE, out of scope), 602/606 seed when none given.
- three-teams-core.md:39: FX-cull identical damage logs were measured with "Seeded random, effects stubbed" -> GC7 omits both qualifiers (finding).
