R7 legacy inventory, reader GA (game animation, shamble-game.js 224-573 + melee/shield models ~684)
## code read (verified lines @48ffb51 = HEAD engine files unchanged)
- ANIM const 132-193 (gait/spr/hold/reload/flinch/death/draw/charge/throw/shield/katana/aim); springStep 197; _animSt 204; _blendPose 205 DEAD (only defined); solveArmIK 210 (ARM_L1/L2 209)
- ANIM v2 header 224; helpers _aLerp/_aWrap/_aSmooth/_aKeys/_aNeutral 234-240; _animInit 241 (palm measure, ankle pivot 253-264, coat panels 265-268 clone mat DoubleSide)
- updateSoldierAnim 271: velocity/teleport 277-279; stance weights 288-293; hip yaw twist 297-303; gait reach/cycle 305-312; feet modes 313-344; pelvis 346-352; leg IK 354-360; upper body 362-366; gun stance 368-418 (reload 378-380, sway 382-384, recoil 390, reach clamp 392-395, rail support 396-402, reload hand 403-407, throw 408-411); shield/katana 419-435; flinch 437-438; coat 440-443
- hooks animHit 448, animThrow 453, deathStart 457, animDeath 461; applyRifleHold 479 (editor pose tool only)
- callers: player 2782 (kick, dip, rel, throwC, lean=bayonet, ads, swap), bots 2936 (no kick/reload/throw/swap: rel=-1); animHit/deathStart 1531-1532, playerDie 1825; animThrow 1405,1758; animDeath 2683,2938
- ANIM.aim.carry/ads NOT read in game (only adsK) -> dead data, editor pose tool exports to it
- comment at 684 stale: models built at 3192-3194 (makeSword/makeAxe/makeShield core 1435/1460), player only
- 493-573 (buildings/team/barracks/clearMap) outside the anim area: not_covered
## evidence gathered
- ANIMATION_PLAN §6 metrics (2026-09-26 headless): hands on grip/rail 0.0 cm; reload hands 0.0 cm; foot drift walk/strafe/backpedal 0.0, sprint mean 0.14 (one push-off frame 5.8 cm); boots below ground 0; leg IK stretch 0.98; hip yaw jump 0; katana fist-hilt 2.7 cm; shield dot 0.993; CPU ~51 us/soldier/frame. §7 limits: no props, bots no reload/throw/melee, applyRifleHold/ANIM.aim legacy
- AUDIT: soldier anim 0.18 ms for 7 (line 10); P1 updateSoldierAnim 17% of ~8 MB/s, Quaternion.invert (anim) 5% (line 25); CS11 residual ~4.9 MB/s, optional anim split -1.5 MB/s held (line 111)
- STATUS risk 2: 48 soldiers DM_Tri 7.2-8.2 MB/game-s, driver updateSoldierAnim hit/death states; 3a report l.72 deferred to CS11 split
- LOOP_V2 §5 T1 abilities Molotov/Riot Shield/Banzai blade (no bayonet); §7 feel kit (target flinch), flinch+knockback, active reload. VISION_V2.md NOT in checkout.
- no anim culling/LOD: every alive bot animated each frame (2936); feet plant at root y (baseY), no terrain sampling
- dead: _blendPose 205, ANIM.aim carry/ads 189-190 (game reads only adsK), ARM_L1/L2 fallback 209/211 (every caller passes lengths), _tt 2683, en._tx/_tz 2938 never read, stale comment 684
## rows planned GA1..GA16 (ANIM table+spring; IK; init/rig; locomotion; feet; pelvis+leg IK+ankle; upper body; gun stance; reload; throw; draw+bayonet; shield+katana+held models; flinch; coat; death; driver/state contract; dead code)
## final rows (16): all port to TypeScript + look pending, except GA16 drop (not pending)
GA1 ANIM+spring C | GA2 IK C | GA3 init/ankle C | GA4 locomotion C | GA5 feet M | GA6 pelvis/leg IK M | GA7 upper body C | GA8 gun stance+draw+bayonet M | GA9 reload M | GA10 throw C | GA11 shield/katana+held models M | GA12 flinch C | GA13 coat C | GA14 death M(alloc) | GA15 driver/contract/perf M | GA16 dead code C drop
DONE
