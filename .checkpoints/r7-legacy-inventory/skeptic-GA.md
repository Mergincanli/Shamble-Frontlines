Task: R7 skeptic for area GA (game animation ANIM v2), rows GA1-GA16
Checkout HEAD a45c9fc; engine+docs identical to 48ffb51 (git diff --stat empty).
Code checks done (lines verified): ANIM 132-193; springStep 197; solveArmIK 210-223 (ONE invert per call at 221); _animInit 241-270, also called eagerly at 774 (spawn); updateSoldierAnim 271-444; animHit 448-451; animThrow 453; deathStart 457-460; animDeath 461-475; applyRifleHold 479-489. Area after 489 = buildings (not anim).
GUN_KIT core 785-791 (5 guns, not 786-788). makeSword 1435, makeAxe 1448, makeShield 1460. Held models 3192-3194; 684 comment stale (685 = ps).
Callers: onDamageEvent 1529-1532 (amount/40); 1405/1758 animThrow; 1825 playerDie; 2683 player dead; 2782 player (skipped if pilotJug); 2936 bot; 2938 bot dead. Editor Anim Lab 380-388 and 672 call hooks too.
_animClock = performance.now() (2674) -> breath in sync confirmed. phase random per soldier (248).
Leftovers: _blendPose never read; ps.toppleT/_tt (2683), ps._tx/_tz written 1826 never read; en.topple written 1520,1842,2938,2939 never read; ARM_L fallback never taken (all 9 call sites pass L1,L2).
springStep not used for vel (279 exp) or thrW (370 exp) -> "every" overstated.
Docs checked: ANIMATION_PLAN §3-§8 all figures match; AUDIT row 10 (0.18 ms for 7), P1 (17 %, invert 5 %, ~8 MB/s), CS11 (4.9 MB/s, -1.5 MB/s held) match; STATUS 83 risk2 (7.2-8.2, driver updateSoldierAnim hit/death) and 84 risk3 (~120) match; report 2026-10-16-three-teams-core.md:72 matches (1.4-1.8x); TECHNICAL 169 (§5.5) matches. LOOP_V2 §4 table/§5 T1 abilities Molotov/Riot Shield/Banzai blade, no bayonet; §7 feel kit/flinch+knockback/active reload/split feedback. T.moveSpeed 6.3, sprintMult 1.4286 (core:67) -> 9.0.
Verdicts:
GA1 corrected (per hold style not per gun; spring not every value: 279, 370 exp)
GA2 corrected (v2_note: ONE invert per call, line 221; ARM_L 0.46/0.42 vs plan §5 0.58/0.59)
GA3 corrected (also eager init at 774; boots-below-ground is §6 not §5)
GA4 confirmed; GA5 confirmed; GA6 confirmed; GA7 confirmed (_animClock 2674 shared)
GA8 corrected (GUN_KIT core 785-791)
GA9 confirmed; GA10 confirmed (throw pose only inside hasGun 368; wind-up only for 'nade' 2782)
GA11 corrected (makeAxe 1448; rotation.y written at 2740-2741 from swingPose 1657; hit = meleeHitCheck arc+range 2742)
GA12 corrected (coat panels come after flinch, 439-443)
GA13 confirmed; GA14 confirmed; GA15 confirmed (no culling around 2918-2936; player skipped dead/pilotJug)
GA16 corrected (add ps._tx/_tz 1826/685; en.topple also 1520,1842,2939; ANIM.aim.carry/ads read by nothing, pose tool has own copy editor:245-248)
Missing: airborne foot pose 320; aim-pitch side channel 369/417 (pitchWeapon 1428); A.supT metric hook 412; scratch pools 231-233/237.
pitchWeapon 1428-1432 writes wpn.rotation.x from the animated gun's world position. DONE
