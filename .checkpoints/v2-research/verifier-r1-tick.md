Task: verify R1 simulation core, part R1-TICK (fixed tick rate) recommendations, wave 1 step 3 Verify (v2-research)
Scope: R1-TICK-REC1..REC6; basis F1-F18, F20-F24, F26 + legacy CS1 GP9 GP13 GP14 GW14 GC15 GN4 GN10 TL13 TL15 KT13 WEAPONS T.
Method: raw source files fetched with curl from allow-listed domains to scratchpad (verbatim grep); HTML pages via WebFetch asking for verbatim quotes.
- F9 confirmed: q3 code/server/sv_init.c:596 `sv_fps = Cvar_Get ("sv_fps", "20", CVAR_TEMP );`
- F10 confirmed: sv_main.c SV_Frame :775 `frameMsec = 1000 / sv_fps->integer ;` :777 `sv.timeResidual += msec;` :832 `while ( sv.timeResidual >= frameMsec ) {` :837 `VM_Call( gvm, GAME_RUN_FRAME, svs.time );` :848 `SV_SendClientMessages();` (after loop)
- F11 confirmed: sv_client.c SV_UserinfoChanged :1151 "snaps", :1156-1161 `} else if ( i > 30 ) {` `i = 30;` `cl->snapshotMsec = 1000/i;` else `cl->snapshotMsec = 50;`
- F12 confirmed: bg_pmove.c Pmove :2041-2042 "// chop the move up if it is too long, to prevent framerate" / "// dependent behavior", :2046 `msec = finalTime - pmove->ps->commandTime;`, :2048 `if ( pmove->pmove_fixed ) {`, :2054 `if ( msec > 66 ) {`; g_main.c :178-179 pmove_fixed "0", pmove_msec "8".
Next: Source SDK F13 F14.
- F13 confirmed (with note): source-sdk-2013 src/game/server/gameinterface.cpp :825 "See shareddefs.h for redefining this.  Don't even think about it, though, for HL2.  Or you will pay.", :829 GetTickInterval, :835 "[Forrest] For Counter-Strike, set default tick rate of 66 and removed -tickrate command line parameter.", :837 "Ignoring this for now, server ops are abusing it", :838 `#if !defined( TF_DLL ) && !defined( CSTRIKE_DLL ) && !defined( DOD_DLL )`. Note: 66 rests on a code comment; DEFAULT_TICK_INTERVAL value not in the two files (finding says so).
- F14 confirmed: shareddefs.h :17 TIME_TO_TICKS (0.5f + dt/TICK_INTERVAL, i.e. round to nearest), :18 TICKS_TO_TIME, :407 "// NOTE: The discrete ticks can have quantization error, so these numbers are biased a little to" (context: HL2 fall-speed defines).
Next: HTML pages (gafferongames x3, docs.rs bevy/rapier/tokio, MDN x6) via curl + text extraction.
- All HTML quotes checked by exact string match (whitespace/curly-quote normalized) on curl'd text. Results:
- F1 confirmed; location error: quote is in section "Variable delta time" (para 2), not "opening section". gafferongames fix_your_timestep.
- F2 confirmed: accumulator code in "Free the physics" (and again in "The final touch").
- F3 confirmed; location note: spiral-of-death + "significantly less than X seconds" quotes are in "Semi-fixed timestep" (last two paras), not "Free the physics"; 0.25 clamp `if ( frameTime > 0.25 )` / `frameTime = 0.25;` in "The final touch" loop. Also same para: "Alternatively you can clamp at a maximum # of steps per-frame".
- F4 confirmed: "The final touch", all three quotes verbatim.
- F5 confirmed: "Free the physics" para 1; para 2 adds "why is it necessary to have fully fixed delta time to do this?" (supports precondition).
- F6 confirmed: docs.rs bevy 0.20.0 struct Fixed, type docs: "The default timestep() is 64 hertz, or 15625 microseconds." + pathological/alternates/power-of-two quotes.
- F7 confirmed: Fixed "If the virtual clock is paused, the FixedUpdate schedule will not run."; Virtual (0.20.0) set_max_delta para: laptop-suspended + "limiting the maximum time" + "death spiral".
- F8 confirmed: docs.rs rapier3d 0.36.1 IntegrationParameters dt "(default: 1.0 / 60.0)", "Set this to 1.0 / your_target_fps.", "Smaller timesteps ... CPU time per second of simulated time.", max_ccd_substeps "(default: 1)".
- F15 confirmed: MDN Games/Anatomy (modified 2026-08-21), all quotes + "in need of technical review".
- F16 confirmed: Anatomy, Summary list: pause / simulate / "can be a problem for long drops" / recover; numTicks quote is a code comment over two lines ("// Note: As we mention in summary, you should keep track of how large numTicks is." "// If it is large, then either your game was asleep, or the machine cannot keep up."). "You can probably see how this is problematic for most multiplayer games." IS verbatim (finding under-labels it inferred). Same list: recover-from-server "is ineffective if your peers or server are out-of-date too".
- F17 confirmed for dedicated worker (MDN DedicatedWorkerGlobalScope/requestAnimationFrame, modified 2025-09-25): refresh-rate and "paused in most browsers when running in background tabs" quotes verbatim. "On the main thread" not on this page (unsupported part, trivial).
- F18 confirmed: MDN setTimeout (modified 2026-10-26) quotes verbatim; heading is "Reasons for longer delays than specified" (doc says "...delays longer than..."). Chrome intensive throttling needs nesting>=5, invisible>5 min, silent>30 s, WebRTC inactive (doc compresses to "hidden for 5 minutes").
- F20 confirmed: MDN Performance/now (modified 2025-08-25): "Resolution in isolated contexts: 5 microseconds", "...non-isolated contexts: 100 microseconds", "Note: This feature is available in Web Workers.", "(... or the time when the worker is run in Worker and ServiceWorker contexts)".
- F21 confirmed: same page "Ticking during sleep" quote verbatim.
- F22 confirmed: MDN Atomics/wait (modified 2026-08-31) four quotes verbatim.
- F23 confirmed: docs.rs tokio 1.54.1 fn.sleep + enum.MissedTickBehavior quotes verbatim.
- F24 confirmed: gafferongames what_every_programmer... section on client-side prediction: "if it takes 100ms from client to server and 100ms back, then any server correction ... will appear to be 200ms in the past"; ring buffer + replay quotes verbatim.
- F26 confirmed: gafferongames integration_basics: four quotes verbatim; "Most commercial game physics engines use this integrator." is under heading "Semi-implicit Euler" (so "this" = semi-implicit).
Next: legacy rows in docs/LEGACY_INVENTORY.md and docs/legacy-tuning.json.
- F17 main-thread half: also confirmed on MDN Window/requestAnimationFrame (modified 2026-08-21): "The frequency of calls to the callback function will generally match the display refresh rate." / "requestAnimationFrame() calls are paused in most browsers when running in background tabs or hidden <iframe>s, in order to improve ..." (extra page, not cited by the doc).
- F12 inferred part / REC2 Q3 claim: extra read q3 sv_client.c SV_UserMove :1424 `SV_ClientThink (cl, &cmds[ i ]);` per received usercmd, :1317 `VM_Call( gvm, GAME_CLIENT_THINK, ...)` -> movement runs per command on receipt, outside the SV_Frame world loop (g_active.c not read).
Legacy (checked against docs/LEGACY_INVENTORY.md rows and code):
- CS1 confirmed: row :169; shamble-core.js:1479 `const FIXED_DT=1/60;`; shamble-game.js:3053 `const dt=Math.min(clock.getDelta(),0.05);`.
- GP13 confirmed :323 (rAF loop, update(dt) skipped when paused/shopping/tutorial/hit-stop). GP9 confirmed :319 (hit-stop 0.12 s pauses update() for everyone).
- GP14 confirmed :324 (player aim/shots/movement before bots; performance.now and Math.random in step); AUDIT_BUGS_PERF.md:10 "`update()` 0.49 ms/frame CPU".
- GW14 confirmed :202 (2.25/2.43/2.48 m at 20/60/144 Hz vs 2.52 m formula; ~2.98 m reach at 60 Hz vs EJUMP_REACH 3.07 m); shamble-core.js:82 gravity 24, jumpV 11, stepUp 0.55; shamble-game.js:1575 EJUMP_REACH formula. Arithmetic 11^2/48=2.52, +0.55=3.07 ok.
- GN10 confirmed :275 (EJUMP_REACH 3.07 by formula, ~2.98 at 60 Hz). Note: GN10 is not in the tick unit's "Legacy facts used" list though REC6 rests on it.
- GC15 confirmed :254 ("the flight integrates with the frame dt (2865), so the landing point shifts with the frame rate"; swept test).
- GN4 confirmed :269 (0.25 s retarget staggered by slot, 0.1 s recheck, 480/s banked 0.1 s; fairness at 60 and 144 fps).
- TL13 confirmed :398 + tools/sim.js:19 `const dt=1/60`. TL15 confirmed :400 + tools/perfbots.js:24 `update(1/60)`; docs/reports/2026-10-16-three-teams-core.md:48 "48 soldiers (head): combat 0.80–1.00, p95 0.97–1.54".
- KT13 confirmed :422 (fixed tick, no wall clock, no Math.random).
- WEAPONS/T/JUG/T_CHARGE confirmed in docs/legacy-tuning.json: fireInterval amk 0.2, shabzon 0.072, lancer 0.42, burstInterval 0.06; JUG.rippleMs 80; T fragSpeed 64, moveSpeed 6.3, sprintMult 1.4286; bow sMax 94; FIXED_DT 0.01666. Smallest interval in all tables = 0.06 (UNIT_TYPES 0.09/0.13/0.35/0.8; T.enemyFireInterval 0.22) -> "one shot per tick at 30 Hz" holds.
Flags:
1. REC2: "30 Hz halves the CPU per simulated second (F8)" - F8 gives direction only ("Smaller timesteps ... require more CPU time per second of simulated time"); "halves" is not in the source and assumes constant per-tick cost (REC3's cadence systems would not halve). Unsupported as a sourced number; mark as inferred.
2. REC2 P1 cites F3 ("headroom itself is Fiedler's advice (F3)") but Rests on omits F3.
3. REC2: "the rate of every legacy measurement (CS1, TL13, TL15, GW14)" overstated: GW14 measured at 20/60/144 Hz, GN4 fairness at 60 and 144 fps.
4. REC2 P3: 200 ms "100 ms ping plus buffers" vs F24's 200 ms = 100 ms each way (RTT 200). Marked proposed; derivation differs from the source example. Low.
5. REC3 Serves "§6.1 (bots fill empty slots)": that text is VISION_V2 §3 [DEFAULT], not §6.1. REC4 "§6.6 (the server records what matters, overruns included)": §6.6 is balance telemetry; overrun/dropped-time metrics trace to no §6 item (ops). Low.
6. REC5 "read from shared memory (boundary unit)" vs TL;DR / BND "one transferred block per tick": known conflict R1-XU-OQ3. Low.
7. Location errors (verdict unaffected): F1 quote is in "Variable delta time" not "opening section"; F3 spiral quotes in "Semi-fixed timestep" not "Free the physics"; F18 heading is "Reasons for longer delays than specified"; F16's "problematic for most multiplayer games" is verbatim (labelled inferred); F18 compresses Chrome intensive-throttling conditions to "hidden for 5 minutes".
No REC rests on nothing. No wrong claim.
TABLE (claim · source · verdict · location)
F1 variable dt changes behaviour · gafferongames fix_your_timestep · confirmed · "Variable delta time" ("The problem is that the behavior...")
F2 accumulator loop · same · confirmed · "Free the physics" code
F3 spiral of death, headroom, 0.25 s clamp · same · confirmed · "Semi-fixed timestep" last paras; clamp in "The final touch"
F4 alpha interpolation · same · confirmed · "The final touch"
F5 fixed step for reproducibility · same · confirmed · "Free the physics" para 1-2
F6 Bevy 64 Hz, reasons · docs.rs bevy 0.20.0 Fixed · confirmed · type docs
F7 paused clock / max_delta · docs.rs bevy Fixed + Virtual · confirmed · Fixed type docs; Virtual set_max_delta paras
F8 Rapier dt 1/60, CPU per sim second · docs.rs rapier3d 0.36.1 IntegrationParameters · confirmed · fields dt, max_ccd_substeps
F9 Q3 sv_fps 20 · sv_init.c:596 · confirmed
F10 Q3 ms accumulator, send after loop · sv_main.c SV_Frame :775-848 · confirmed
F11 Q3 snaps cap 30, default 50 ms · sv_client.c SV_UserinfoChanged :1151-1161 · confirmed
F12 Q3 Pmove chop, pmove_fixed 0 / msec 8 · bg_pmove.c :2041-2054, g_main.c :178-179 · confirmed
F13 Source CS 66, -tickrate compiled out · gameinterface.cpp :825-838 · confirmed (66 from a code comment)
F14 TIME_TO_TICKS rounding, quantization note · shareddefs.h :17-18, :407 · confirmed
F15 MDN loop, Worker timer option · MDN Games/Anatomy · confirmed
F16 pause / simulate / recover; numTicks · same, Summary list + code comment · confirmed
F17 rAF follows refresh, paused in background · MDN DedicatedWorkerGlobalScope/rAF (+Window/rAF) · confirmed
F18 4 ms nesting, Firefox 1 s, Chrome once/min, busy delay · MDN setTimeout "Reasons for longer delays than specified" · confirmed
F20 performance.now in workers, 100/5 µs, origin · MDN Performance/now · confirmed
F21 not ticking during sleep except Windows · same "Ticking during sleep" · confirmed
F22 Atomics.wait rules · MDN Atomics/wait · confirmed
F23 Tokio ms granularity, Burst default · docs.rs tokio 1.54.1 fn.sleep, enum.MissedTickBehavior · confirmed
F24 prediction ring buffer, 200 ms example · gafferongames what_every_programmer... · confirmed
F26 Euler error, semi-implicit · gafferongames integration_basics · confirmed
CS1 GP9 GP13 GP14 GW14 GN10 GC15 GN4 TL13 TL15 KT13 · LEGACY_INVENTORY rows + code · confirmed
WEAPONS / T tuning values · docs/legacy-tuning.json · confirmed
Counts: 37 confirmed / 0 wrong / 0 unsupported (basis claims). Sourced-number issue: REC2 "halves" unsupported.
DONE
