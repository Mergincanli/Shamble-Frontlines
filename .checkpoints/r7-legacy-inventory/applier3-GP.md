R7 applier3 for area GP (fixes3-ops-GP.json)
Started. 11 findings.
verifier-game2#1 (GP1 what) accepted: 1082 builds new SphereGeometry+MeshBasicMaterial per blast; only `new THREE.Mesh` in 1009-1091; 2960 disposes non-pooled. fix_op as written.
verifier-game2#2 (GP3 where) accepted: grep sprintMult: core:67 def, unit defs 128-137, uses game:2756 unitOf(2).sprintMult, 2774 unitOf(ps.unit).sprintMult; T.sprintMult never read. fix_op as written.
game-designer#7 (GP3 where) accepted: core:225 T.arenaX=GROUND_W/2 (372/2=186), arenaZ=252/2=126; legacy-tuning.json 186/126; setMapSize (game:77) is a dev slider only. fix_op as written.
engine-architect#5 (GP14 v2_note) accepted, refined: verified order 2675-2960 (economy 2692-2698 before aim 2707; fire 2722-2731; melee impact 2738-2743; movement 2748-2780; grenades 2856-2859; resync 2862; bolts 2863-2885; bots 2893-2941; sink 2944; jugs 2947; FX 2948-2960); GW16 mark drop, bridge disappears. Added melee, line refs.
netcode-architect#7 (GP14 v2_note) accepted, refined: sfxShot 1473 before spread 1477; sfxShot 2227-2231 gated on ctx/muted, len from sampleRate; sfxAMK random 2286-2290; impact throttle 2253 wall clock; enemyShot 2255-2256; shake 2846; breath 363 via _animClock 2674/2782/2936; weapon parent chest core:453; getMuzzle 1760; report docs/reports/2026-10-16-three-teams-core.md:39. Clarified 11 count (10 calls + comment at 2169) and that tickJuggernauts' clock (2947) only animates idle mechs.
netcode-architect#8 (GP14 v2_note) accepted, trimmed to avoid repeating the order: applyDamage alive=false 1520; dead branch 2937; enemyFire only in alive branch 2913.
engine-architect#10 (GP2 v2_note) accepted: VISION_V2 §6 items 1-10 have no rebinding; grep rebind finds nothing in v2 docs; GP8 note wants the controls list generated from the input map.
netcode-architect#6 (GP2 what) accepted, plus a v2_note op: 3264-3273, 3284-3285 run reload/resetRound, useAbility, startCharge, releaseCharge->throwGrenadeCharged/fireBolt (1724), enterJug/exitJug/openShop/tryResupply, equip*, ADS at event time.
graphics-engineer#4 (GP9 v2_note) accepted: framing recommendation is a look decision; row has look_brief_pending true. fix_op as written.
graphics-engineer#9 (GP6 v2_note) accepted, refined: sfxBus 2246-2249 StereoPanner only (2248), no createPanner/AudioListener; throttle 2253; botSfxR 90 core:106; R3 includes spatial audio with Web Audio (V2_PHASE0_BRIEF.md:123, §4).
game-designer#9 (GP11 v2_note) accepted: VISION_V2.md:43 two teams 2-8 [DECIDED]; §6 item 1 modes as data.
Fixed sfxShot duration range to 0.07-0.2 s (amk returns early at 2227). apply-ops GP clean: 36 ops, 19 fields, 10 rows.
DONE
