checker3-GP: R7 fixes3 checker for area GP
Ran apply-ops.py GP: 36 ops, 19 fields, 10 rows, no errors. fixes3 touches only GP1.what, GP2.what, GP2.v2_note x2, GP3.where x2, GP6/GP9/GP11/GP14 v2_note.
- verifier-game2#1 accepted: OK. 1082 builds new SphereGeometry+material; 2960 disposes non-pooled.
- verifier-game2#2 accepted: OK. sprintMult only core:67, 128-137, game 2756, 2774 (unitOf).
- game-designer#7 accepted: OK. core:67 arenaX:60, :224 GROUND_W 372/252, :225 sets /2; legacy-tuning.json T 186/126.
- engine-architect#5 accepted: PROBLEM. All line ranges hold (2675-2690, 2692-2698, 2700, 2707, 2722-2731, 2738-2743, 2748-2780, 2856-2859, 2862, 2863-2885, 2893-2941, 2944, 2947, 2948-2960) but the order omits tickPilot (2781): piloted Juggernaut drive and ripple cannon fire (_jugFirePilot 1299), a sim phase between movement and grenades. Fix: insert it.
- netcode-architect#7 accepted: OK. grep: 11 textual reads, 2169 a comment; 2674 _animClock -> _animSt.t 2782/2936 -> breath 363; core:453 weapon parent chest; tickPilot 2781 -> animateJuggernaut t bobs body when idle (1290); tickJuggernauts 1349-1355 idle only; 1473/1477; 2227-2231 dur 0.07-0.2 (amk returns early); 2286-2290; 2253; 2255-2256; 2846; report :39 seeded random + effects stubbed.
- netcode-architect#8 accepted: OK. 1520 V.alive=false; 2895 if(en.alive) / 2937 else dead branch; 2862 re-sync; bolts player-only (1802).
- engine-architect#10 accepted: OK. VISION_V2 §6 has no rebinding.
- netcode-architect#6 accepted: PROBLEM. Lines 3264-3269, 3273, 3284-3285, 1724 hold. But 'Only movement, sprint, jump, held or queued fire and the cursor aim are sampled inside update()' is false: keys['KeyE'] is read at 948 inside updateTechBox (called 2697) to channel the tech box. Fix: add it.
- graphics-engineer#4 accepted: OK.
- graphics-engineer#9 accepted: OK. only createStereoPanner (2248); botSfxR 90 core:106; R3 row (brief:123) lists spatial audio with Web Audio.
- game-designer#9 accepted: OK. VISION_V2 §3 [DECIDED] two teams 2-8; §6 item 1 modes as data.
DONE
