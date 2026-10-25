netcode-architect review of R7 simulation rows (CS, GC, GN, GW movement/collision/vertical, CD sim tuning, GP main loop/input/dispatch) for docs/LEGACY_INVENTORY.md
Rows in scope: CS1-11, GC1-19, GN1-14, GW6 GW7 GW9 GW13 GW14 GW15 GW16 GW20 GW21, CD4 CD6 CD7 CD8 CD10-14 CD21, GP2 GP3 GP9 GP13 GP14 GP15 GP16.
Engine files unchanged since 48ffb51 (git diff --stat empty). core wc 1543 lines + last line w/o newline (hasLOS 1544).
## Cross-cutting checks
- wall clock: grep performance.now/Date.now game = 10+1 = 11 (STATUS 11 holds). Sites 65 comic cd, 957 tbox bob, 2173 perf, 2253 sfxImpact throttle, 2674 _animClock (anim only), 2781/2947 tt -> animateJuggernaut only, 3294 dev wheel. NONE decides a sim outcome directly; they matter because cosmetic code gated by them draws from shared Math.random (sfxImpact 2253->2254 noise buffer sampleRate*0.04 draws; sfxEnemyShot 2256 sampleRate*0.09; comicKill 68; FX 1029-1088; camShake 2846 inside update()).
- Math.random sim sites: 1477 player spread, 1619 bot spread, 1827 respawn x offset, 1842/2939 unit pick + fireCd, 2904 strafe, 2913 burst pause, 2923/2927 siege cadence, 3212 init fireCd/strafeDir/wander, 899/919/922 TBOX next point (GE, out of scope), 602/606 seed when none given.
- three-teams-core.md:39: FX-cull identical damage logs were measured with "Seeded random, effects stubbed" -> GC7 omits both qualifiers (finding).
## Row checks (CS)
- CS1 ok: core:1479-1481, loop game:3053 (dt=min(getDelta,0.05)); grep FIXED_DT/BTN/makeInputCommand only core + tools/tuning-dump.js (R7's own). Mark drop ok.
- CS2 ok: core:455, :462, :506/:554 hit:_BODY_HIT, stance=0 game:1447-1448.
- CS3 ok: res.mult no reader; damageEnemy 1552; bot flat 1627.
- CS4 ok: node model of core:1484-1485 -> inside torso horiz -1, inside head 0, vertical 2.98.
- CS5 ok (rayStatic 1519, preview 1742, START_HERE:69 drift 1516).
- CS6 ok (NaN reasoning holds; report :55-58,:64-65,:95-98).
- CS7 ok (callers 1126,1309,1489,1625,2869; 1512 team check).
- CS8 ok (guard 1444, forced 778/2862, editor 564).
- CS9 ok (1588/1595, editor 574). CS10 ok. CS11 ok.
## GW
- GW13 ok (callers 2780/2933/2624/editor 671; h.boxes||[h.box]).
- GW14: FINDING should: jump apex frame-rate dependent. Model of tickVertical (2538-2539) gives apex 2.43 m @60, 2.48 @144, 2.25 @20 (dt cap) vs formula 2.52; ledge reach 2.98/3.03/2.80 vs EJUMP_REACH 3.07 (1575). Also nit: standOnCover only player (2780 true; 2933 omitted).
- GW16: FINDING should: rotated kit houses -> SIM gets loose world AABBs (_bkWorldBoxes core:1429-1431 applyMatrix4; syncSimWorld 1458) vs resolveObstacles oriented local boxes (2526): shots/LOS vs movement disagree. spawnKitBuilding rot 578-579.
- GW15 ok (losMeshes 552 render meshes). GW9 ok. GW7 ok. GW21 ok.
## GC
- GC3: FINDING blocking: "open ground and ledges are aimed at height 0" FALSE. Probe (xvfb-run node tools/preview.js . eval aimheight.js, default map): SIM ray stops on platform tops y=1.20/2.50/4.00 and ramp box top 2.50 (slope 1.25 at centre). updateAim 1126-1129 aimPoint=r.p when kind!='none'.
- GC3: FINDING should: v2 note "client sends an aim direction" -- legacy shot dir = aimPoint - playerFireOrigin (1473); aim is a world point.
- GC5: FINDING should: fire cooldown reset (1471 ps.fireCd=interval; decrement 2690) -> frame-quantised rates. Model: Shabzon 0.072 -> 12.0/s @60, 13.1 @144, 10.0 @30 vs 13.9 design; bots same pattern 2913. acquireTarget carries overshoot (1597) but fire does not.
- GC7: FINDING should: evidence omits "Seeded random, effects stubbed" (three-teams-core.md:39) and :77 camera-dependent RNG.
- GC17: FINDING should: cannon origin = animated muzzle (mz.getWorldPosition 1301; muzzles in cannonGroup 1232, child of body 1207) moved by gait bob 1290-1293, recoil 1294, smoothed turret yaw 1339-1340, idle bob 0.012*sin(t*1.6) with t=performance.now (2781->1343->1291). Wall clock reaches a sim outcome.
## GP
- GP2: FINDING should: listeners call actions at event time (3264-3269, 3273 releaseCharge->throwGrenadeCharged, 3284-3285 bow fireBolt / ads), not only keys map.
- GP14: FINDING should: wall-clock/RNG coupling mechanism (sfxShot 2227-2231 before spread 1477; sfxImpact 2253 throttle; sfxEnemyShot 2256; FX 1029-1088; camShake 2846; breath anim t at 363 moves muzzle -> throw origin) + player-first order (fire 2722-2731 before bots 2893; dead bots can't fire).
- CD8: FINDING should: weaponPitchMax called "aim pitch limit" but only clamps held-gun visual pitch (pitchWeapon 1431).
## Probes on the real code (xvfb-run -a node tools/preview.js . eval <scratch>, default map, read-only)
- firerate-real.js: Shabzon held 10 game s via update(1/hz): 10.00 / 12.00 / 13.10 shots/s at 30/60/144 Hz (design 13.89).
- jump-real.js: player jump apex rise 2.250 / 2.430 / 2.483 m at 20/60/144 Hz; EJUMP_REACH 3.071 (formula apex 2.521 + 0.55).
- aimheight.js: resolveShot from above stops at platform tops 1.20/2.50/4.00 m; ramp box top 2.50 (slope 1.25 at centre), 4.00 (slope 2.00).
## Other checks
- GC12 nit: hit check gated on sword/axe mesh (ps.swingMesh 1666, 2739); arc facing = player.rotation.y (1671).
- GC15 nit: bolt flight integrates frame dt (2865), stuck position follows target mesh (2880); swept segment test per step worth keeping.
- GN8 nit: evidence omits "effects stubbed".
- GN12 nit: G/F Float32Array with f64 math, heap ties <=/>= -> Rust parity needs same rounding.
- no for..in / Object.keys over sim state (comment game:702) -> no hash-order hazard in legacy sim.
- DEV god/freezeAI authority covered by GP5.
## Final findings (fix ops drafted; finds verified unique by python count)
blocking: GC3 (ledges aimed at 0 false), CD8 (weaponPitchMax is not an aim limit; pitchWeapon 1431)
should: GC3 (aim point not direction), GC5 (frame-quantised fire, measured), GC7 (evidence qualifiers), GC17 (animated cannon origin + wall clock 1290), GP2 (actions in listeners), GP14 x2 (RNG/wall-clock coupling incl. breath 363 -> chest-mounted wpn core:453; player-first order), GW14 (jump apex measured), GW16 (rotated houses loose AABB; editor rotates houses editor:151)
nit: GW14 (standOnCover), GN10, GN8, GN12, GC12, GC15, CS2 (model fitted to hit shape decides pipeline)
Confirmed in full: all other scoped rows.
Most important gap: frame-quantised timing (fire cooldowns reset, jump apex) + RNG coupling with audio/FX; no row tells R2 how legacy numbers depend on frame rate.
DONE
