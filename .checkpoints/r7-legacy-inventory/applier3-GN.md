R7 applier3 for area GN (fixes3-ops-GN.json)
- verifier-game2#3 GN4 evidence: accepted; risk 2 (three-teams-core.md:97-98) gives only 2.3 vs 4, the 1,100/s figure is LOS-SIZE (:118-119); replaced with both, cited.
- verifier-game2#4 GN7 v2_note: accepted; shamble-game.js:1455-1457 rampLo/rampHi, 1462 copy skipping ramps, core:1544 hasLOS on SIM.statics; "two static sets ... besides the movement collision".
- verifier-game2#5 GN12 evidence: accepted; bot-poplar-wedge.md:43 (134->1 Cabana seed 4, 6->2 Romania seed 6, others 1-3); added seeds and the 1-3 note.
- engine-architect#14 GN5 v2_note: accepted; VISION_V2.md:78 replays [LATER], V2_PHASE0_BRIEF.md:121 R1 identical Wasm/native; reason now same results Wasm/native.
- netcode-architect#12 GN10 what: accepted; 1575 EJUMP_REACH = 11^2/48+0.55 = 3.07; tickVertical 2537-2538 semi-implicit Euler, integrated at 60 Hz apex 2.430 + 0.55 = 2.98 (own python check; 144 Hz 3.03); 1602 compares against EJUMP_REACH.
- netcode-architect#13 GN8 evidence: accepted; three-teams-core.md:39 "Seeded random, effects stubbed, cull off vs on".
- netcode-architect#14 GN12 v2_note: accepted with better op; 2621 Float32Array g/f, 2648-2649 f64 math with 1.4142/0.5858 stored into f32 (and ng<G[e] compares f64 to f32), 2630-2632 <= / >= ties; appended without the leading space, also names the constants.
- game-designer#1 GN2 v2_note: accepted with better op; shamble-game.js:701 era:1, :1178 tier>era blocks, so tier 2 opens after one era-up; LOOP_V2.md:55-59 §4 ladder Tech 2 = Officer, :73 §5 Officer moves to Tier 2.
apply-ops.py GN: 12 ops applied to 10 fields of 7 rows, all after-texts read well.
DONE
