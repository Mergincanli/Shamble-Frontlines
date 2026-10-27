# R1 unit tick: a fixed tick rate

Brief question (quoted): "a fixed tick rate" (docs/V2_PHASE0_BRIEF.md §4, row R1). Unit scope from the approved plan
(.checkpoints/v2-research/plan.md, Units, R1): "fixed tick rate"; as briefed: the fixed step, the fixed-timestep loop
on server and client (accumulator, interpolation for rendering), how the tick relates to the send and snapshot rates,
the cost per tick for 48+ players plus bots on the server and in a Web Worker on the minimum hardware, 30 Hz vs 60 Hz
(and others) for a top-down shooter with hitscan, projectiles and big vehicles, and what shipped games use and why.

Author: engine-architect (unit agent), 2026-10-27. Status: draft, not yet verified. Every source was read 2026-10-27.
WebFetch caps a verbatim quote at about 125 characters, so long passages are quoted as consecutive fragments.
"Confirmed in the source" means the quoted words were returned verbatim; "inferred" means my reasoning or arithmetic
on top of a source, or a claim seen only in WebFetch's own summary.

## 2. Findings

**R1-TICK-F1.** A simulation stepped with a variable delta time behaves differently at different frame rates.
Quote: "The problem is that the behavior of your physics simulation depends on the delta time you pass in."
Source: https://gafferongames.com/post/fix_your_timestep/ (Glenn Fiedler, "Fix Your Timestep!", page dated 2004-06-10),
opening section. Tier: other (practitioner article; the brief names Fiedler's articles as a reference). Read 2026-10-27.
Sure: confirmed in the source.

**R1-TICK-F2.** The standard fixed-step loop: the renderer produces real time, the simulation consumes it in fixed
`dt` steps, and the leftover is carried in an accumulator. Quote (code): `accumulator += frameTime;` /
`while ( accumulator >= dt )` / `integrate( state, t, dt );` / `accumulator -= dt;`. Source: as F1, section "Free the
physics". Tier: other. Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F3.** A fixed-step loop that cannot keep up falls into a "spiral of death"; the simulation must cost much
less real time than the time it simulates, and the sample loop caps the time fed in per frame at 0.25 s. Quotes:
"It’s what happens when your physics simulation can’t keep up with the steps it’s asked to take." / "You really need
to ensure that it takes significantly less than X seconds of real time" "to update X seconds worth of physics
simulation." / code `if ( frameTime > 0.25 ) frameTime = 0.25;`. Source: as F1, section "Free the physics" and the
final loop. Tier: other. Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F4.** Rendering between fixed steps interpolates the previous and current state by the accumulator's
remainder. Quotes: "One solution to this problem is to interpolate between the previous and current physics state"
"based on how much time is left in the accumulator:" / `const double alpha = accumulator / dt;` /
`State state = currentState * alpha + previousState * ( 1.0 - alpha );`. Source: as F1, section "The final touch".
Tier: other. Read 2026-10-27. Sure: confirmed in the source. That the rendered state therefore trails the newest step
by up to one `dt` is inferred (the page does not quantify it).

**R1-TICK-F5.** A fixed step is the precondition for exact reproducibility given the same inputs. Quotes: "What if
you want exact reproducibility from one run to the next given the same inputs?" / "This comes in handy when trying to
network your physics simulation using deterministic lockstep". Source: as F1. Tier: other. Read 2026-10-27. Sure:
confirmed in the source.

**R1-TICK-F6.** Bevy's fixed timestep defaults to 64 Hz rather than 60 Hz, for two stated reasons: a 60 Hz step can
beat against a 60 Hz monitor, and 1/64 s is exact in binary floats. Quotes: "The default timestep() is 64 hertz, or
15625 microseconds." / "using 60 hertz has the potential for a pathological interaction with the monitor refresh rate"
/ "the game alternates between running two fixed timesteps and zero fixed timesteps per frame" / "the value is a power
of two which losslessly converts into f32 and f64." Source: https://docs.rs/bevy/latest/bevy/time/struct.Fixed.html
(bevy 0.20.0), type-level docs. Tier: official (engine documentation). Read 2026-10-27. Sure: confirmed in the source
(fragments).

**R1-TICK-F7.** Bevy's fixed schedule does not run while its virtual clock is paused, and the virtual clock caps the
time added at once so a freeze or a suspended laptop does not trigger a catch-up "death spiral". Quotes: "If the
virtual clock is paused, the FixedUpdate schedule will not run." (Fixed page) / "Usually if a laptop is suspended for
an hour, it doesn’t make sense to try to simulate the game logic" / "By limiting the maximum time that can be added at
once, we also limit the amount of virtual time" (Virtual page). Sources:
https://docs.rs/bevy/latest/bevy/time/struct.Fixed.html and https://docs.rs/bevy/latest/bevy/time/struct.Virtual.html
(bevy 0.20.0). Tier: official. Read 2026-10-27. Sure: confirmed in the source (fragments). The default cap value was
not on the part of the page read; no number is claimed.

**R1-TICK-F8.** Rapier's physics step defaults to 1/60 s, is meant to match the target rate, and a smaller step costs
more CPU per simulated second. Quotes: "The timestep length - how much simulated time passes per physics step
(default: `1.0 / 60.0`)." / "Set this to `1.0 / your_target_fps`." / "Smaller timesteps are more accurate but require
more CPU time per second of simulated time." / `max_ccd_substeps`: "Maximum number of CCD substeps performed by the
solver (default: `1`)." Source: https://docs.rs/rapier3d/latest/rapier3d/dynamics/struct.IntegrationParameters.html
(rapier3d 0.36.1), fields `dt` and `max_ccd_substeps`. Tier: official. Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F9.** Quake III Arena's server runs its world at 20 frames per second by default. Quote:
`sv_fps = Cvar_Get ("sv_fps", "20", CVAR_TEMP );`. Source:
https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_init.c (GPL source, "Copyright (C)
1999-2005 Id Software, Inc."). Tier: shipped game (source code). Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F10.** Quake III's server frame is an integer-millisecond accumulator loop, and it sends client messages
after the frame loop. Quotes: `frameMsec = 1000 / sv_fps->integer ;` / `sv.timeResidual += msec;` /
`while ( sv.timeResidual >= frameMsec ) {` / `VM_Call( gvm, GAME_RUN_FRAME, svs.time );` / then
`SV_SendClientMessages();`. Source: https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_main.c,
function `SV_Frame`. Tier: shipped game. Read 2026-10-27. Sure: confirmed in the source. That integer division turns a
60 Hz setting into 16 ms frames (62.5 Hz) is inferred arithmetic.

**R1-TICK-F11.** Quake III sets each client's snapshot rate separately from the server frame rate, capped at 30 per
second, 20 by default. Quotes: `} else if ( i > 30 ) {` / `i = 30;` / `cl->snapshotMsec = 1000/i;` /
`cl->snapshotMsec = 50;`. Source:
https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_client.c, `SV_UserinfoChanged`
("snaps"). Tier: shipped game. Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F12.** Quake III integrates player movement per user command over that command's own duration, chops long
moves "to prevent framerate dependent behavior", and offers an optional fixed movement step (off by default, 8 ms).
Quotes: `msec = finalTime - pmove->ps->commandTime;` / "// chop the move up if it is too long, to prevent framerate"
"// dependent behavior" / `if ( pmove->pmove_fixed ) {` / `if ( msec > 66 ) {` (bg_pmove.c) /
`{ &pmove_fixed, "pmove_fixed", "0", CVAR_SYSTEMINFO, 0, qfalse},` /
`{ &pmove_msec, "pmove_msec", "8", CVAR_SYSTEMINFO, 0, qfalse},` (g_main.c). Sources:
https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_pmove.c (`Pmove`) and
https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/g_main.c (cvar table). Tier: shipped
game. Read 2026-10-27. Sure: confirmed in the source. That movement therefore ran at the client's command rate, finer
than the 20 Hz world frame, is inferred (the client input code was not read).

**R1-TICK-F13.** In the Source engine the tick interval is a per-game constant: Counter-Strike's default is 66 ticks
per second, and the `-tickrate` override is compiled out for TF2, Counter-Strike and Day of Defeat. Quotes:
"[Forrest] For Counter-Strike, set default tick rate of 66 and removed -tickrate command line parameter." /
`#if !defined( TF_DLL ) && !defined( CSTRIKE_DLL ) && !defined( DOD_DLL )` / "Ignoring this for now, server ops are
abusing it" / "See shareddefs.h for redefining this.  Don't even think about it, though, for HL2.  Or you will pay."
Source: https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/gameinterface.cpp,
`CServerGameDLL::GetTickInterval` (Source SDK 2013, master). Tier: shipped game (source code). Read 2026-10-27. Sure:
confirmed in the source. The value of `DEFAULT_TICK_INTERVAL` itself was not found in the files read.

**R1-TICK-F14.** Source expresses game time in whole ticks, rounding a duration to the nearest tick, and notes the
quantization error. Quotes: `#define TIME_TO_TICKS( dt ) ( (int)( 0.5f + (float)(dt) / TICK_INTERVAL ) )` /
`#define TICKS_TO_TIME( t ) ( TICK_INTERVAL *( t ) )` / "// NOTE: The discrete ticks can have quantization error, so
these numbers are biased a little to". Source:
https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/shared/shareddefs.h. Tier: shipped
game. Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F15.** MDN describes the browser game loop as a constant-frequency update with rendering as often as
possible, and lists updating in a Web Worker on a timer as one way to do it. Quotes: "One common technique is to update
the simulation at a constant frequency and then draw as much (or as little) of the actual frames as possible." / "Draw
on `requestAnimationFrame()` and update on a `setInterval()` or `setTimeout()` in a Web Worker." / "This is the same as
above, except update does not hog the main thread (nor does the main thread hog it). This is a more complex solution,
and might be too much overhead for simple updates." Source: https://developer.mozilla.org/en-US/docs/Games/Anatomy
("Anatomy of a video game", last modified 2026-08-21). Tier: official (MDN). Read 2026-10-27. Sure: confirmed in the
source. MDN marks its tick example "in need of technical review".

**R1-TICK-F16.** MDN names three ways to handle a long gap (a hidden tab or a slow machine): treat it as a pause,
simulate it, or recover state from the server; and it advises watching the catch-up tick count. Quotes: "Consider the
gap 'a pause' and skip the time." / "You can simulate the gap to catch up." / "This can be a problem for long drops
and/or complex updates." / "You can recover the game state from a peer or the server." / "you should keep track of how
large numTicks is. If it is large, then either your game was asleep, or the machine cannot keep up." Source: as F15.
Tier: official. Read 2026-10-27. Sure: confirmed in the source. (MDN calls the pause option "problematic for most
multiplayer games": seen in WebFetch's summary, so inferred.)

**R1-TICK-F17.** `requestAnimationFrame`, on the main thread or in a dedicated worker, runs at the display's refresh
rate and is paused in background tabs in most browsers. Quotes: "The frequency of calls to the callback function will
generally match the display refresh rate. The most common refresh rate is 60 Hz, (60 cycles/frames per second), though
75 Hz, 120 Hz, and 144 Hz are also widely used." / "`requestAnimationFrame()` calls are paused in most browsers when
running in background tabs or hidden `<iframe>`s, to improve performance and battery life." Source:
https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/requestAnimationFrame (last modified
2025-09-25; Baseline "available across browsers since March 2023"). Tier: official. Read 2026-10-27. Sure: confirmed in
the source.

**R1-TICK-F18.** Browsers clamp and throttle timers: a 4 ms minimum after five nested `setTimeout` calls; minimum
delays in inactive tabs (Firefox desktop: 1 s); Chrome's intensive throttling (from Chrome 88) checks timers once per
minute after a page has been hidden for 5 minutes; and timers fire late when the thread is busy. Quotes: "As specified
in the HTML standard, browsers will enforce a minimum timeout of 4 milliseconds once a nested call to setTimeout has
been scheduled 5 times." / "To reduce the load (and associated battery usage) from background tabs, browsers will
enforce a minimum timeout delay in inactive tabs." / "Firefox Desktop has a minimum timeout of 1 second for inactive
tabs." / "Timers in this state are checked once per minute" / "The timeout can also fire later than expected if the
page (or the OS/browser) is busy with other tasks." Source: https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout
(last modified 2026-10-26), section "Reasons for delays longer than specified". Tier: official. Read 2026-10-27. Sure:
confirmed in the source.

**R1-TICK-F19.** Neither MDN's `setTimeout` page nor Chrome's throttling post says whether timers inside a dedicated
worker are throttled when the tab is hidden. Sources: as F18, and https://developer.chrome.com/blog/timer-throttling-in-chrome-88
("Last updated 2021-01-18"; Chrome 88), which says hidden-page timers are checked "once per minute" and exempts pages
where "WebRTC is in use". Tier: official. Read 2026-10-27. Sure: inferred (an absence reported by WebFetch on both
pages).

**R1-TICK-F20.** `performance.now()` works in workers, is coarsened to 100 µs (5 µs when cross-origin isolated), and
in a worker counts from when the worker started. Quotes: "Resolution in isolated contexts: 5 microseconds" /
"Resolution in non-isolated contexts: 100 microseconds" / "Note: This feature is available in Web Workers." / "the time
when the worker is run in Worker and ServiceWorker contexts". Source:
https://developer.mozilla.org/en-US/docs/Web/API/Performance/now (last modified 2025-08-25), "Security requirements"
and description. Tier: official. Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F21.** `performance.now()` stops counting during OS sleep on every OS except Windows. Quote: "The
specification (Level 2) requires that performance.now() should tick when the operating system sleeps or the browser
process freezes in any other way. It appears that only browsers on Windows keep ticking during sleep." Source: as F20,
section "Ticking during sleep". Tier: official. Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F22.** `Atomics.wait` can block a worker thread until a notify or a millisecond timeout, needs a typed array
on a `SharedArrayBuffer`, and is not allowed on the main thread. Quotes: "The Atomics.wait() static method verifies
that a shared memory location contains a given value and if so sleeps, awaiting a wake-up notification or a timeout." /
"It is blocking and cannot be used in the main thread." / "This method only works with an Int32Array or BigInt64Array
that views a SharedArrayBuffer." / timeout: "Time to wait in milliseconds." Source:
https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/wait (last modified
2026-08-31). Tier: official. Read 2026-10-27. Sure: confirmed in the source.

**R1-TICK-F23.** Tokio's timers have millisecond granularity (coarser on Windows), and its `Interval` catches up on
missed ticks by bursting unless told otherwise. Quotes: "`Sleep` operates at millisecond granularity and should not be
used for tasks that require high-resolution timers." / "some platforms (specifically Windows) will provide timers with
a larger resolution than 1 ms." (fn.sleep) / Burst: "Ticks as fast as possible until caught up." / Skip: "Skips missed
ticks and tick on the next multiple of `period` from `start`." / "For these reasons, MissedTickBehavior::Burst is the
default for MissedTickBehavior." / "these strategies will only apply when the delay is greater than 5 milliseconds."
(enum.MissedTickBehavior). Sources: https://docs.rs/tokio/latest/tokio/time/fn.sleep.html and
https://docs.rs/tokio/latest/tokio/time/enum.MissedTickBehavior.html (tokio 1.54.1). Tier: official. Read 2026-10-27.
Sure: confirmed in the source.

**R1-TICK-F24.** Client-side prediction keeps a ring buffer of the local player's inputs and states and, on a server
correction, replays from the corrected state to the present predicted time; with 100 ms each way the correction is
200 ms old. Quotes: "The solution is to keep a circular buffer of past character state and input for the local player
on the client" / the client "first discards any buffered state older than the corrected state from the server" "and
replays the state starting from the corrected state back to the present “predicted” time on the client" / "if it takes
100ms from client to server and 100ms back". Source:
https://gafferongames.com/post/what_every_programmer_needs_to_know_about_game_networking/ (Glenn Fiedler, 2010-02-24).
Tier: other. Read 2026-10-27. Sure: confirmed in the source (fragments). That the number of replayed steps is about the
round-trip time divided by the tick length, and so doubles from 30 Hz to 60 Hz, is inferred arithmetic.

**R1-TICK-F25.** The interpolation delay for remote entities follows the snapshot send rate, not the simulation rate:
Fiedler gives 150 ms at 30 snapshots per second and 85 ms at 60. Quotes: "a delay of 150ms" (30 pps) / "60 packets
per-second needs only 85ms". Source: https://gafferongames.com/post/snapshot_interpolation/ (Glenn Fiedler, 2014-11-30),
conclusion. Tier: other. Read 2026-10-27. Sure: confirmed in the source for the two numbers; his "about 3x the packet
interval for 2-5% loss" rule was seen only in WebFetch's summary (inferred). R2 owns these budgets.

**R1-TICK-F26.** Explicit Euler is exact only when the rate of change is constant over a step, so integration error
depends on the step size; Fiedler recommends semi-implicit Euler, which most commercial physics engines use. Quotes:
"It is only 100% accurate when the rate of change is constant over the timestep." / "After 10 seconds, the object
should have moved 500 meters, but explicit euler gives a result of 450 meters." / "Most commercial game physics engines
use this integrator." / "My recommendation is semi-implicit euler." Source:
https://gafferongames.com/post/integration_basics/ (Glenn Fiedler, 2004-06-01). Tier: other. Read 2026-10-27. Sure:
confirmed in the source (fragments).

## 3. Legacy facts used
- **CS1** (docs/LEGACY_INVENTORY.md:169): `FIXED_DT` = 1/60 s is declared (shamble-core.js:1479) but never read; the
  live loop uses `clock.getDelta()` clamped to 0.05 s (shamble-game.js:3053). Mark: drop. Tells v2: a declared tick
  is not a tick; the v2 tick must drive the loop and be proven by tests.
- **GP13** (:323): one `requestAnimationFrame` loop runs `update(dt)` and freezes the whole world for pause, shop,
  tutorial and hit-stop. Mark: redesign (a fixed-tick Rust sim in a Worker or on the server, a render loop that
  interpolates). Tells v2: a shared authoritative sim never pauses for one player's UI.
- **GP14** (:324): the step's order decides outcomes; the player acts before every bot; wall-clock reads and a shared
  `Math.random` leak into the step. AUDIT_BUGS_PERF.md:10: `update()` 0.49 ms per frame (scripted 60 s combat).
  Tells v2: the tick is explicit phases in a fixed order, with no clock read inside it.
- **GW14** (:202): the jump apex depends on the frame rate, measured 2.25 / 2.43 / 2.48 m at 20 / 60 / 144 Hz against
  2.52 m from the formula, so the reachable ledge is about 2.98 m at 60 Hz while `EJUMP_REACH` assumes 3.07 m.
  Tells v2: reach follows from the tick and the integrator; map validation must use the integrated value.
- **GN4** (:269): bot sight checks run on a budget per game second (`TUNE.ai.losPerS` 480, banked up to 0.1 s), bots
  retarget every 0.25 s and re-check sight every 0.1 s, staggered by slot; fairness was measured at 60 and 144 fps.
  Tells v2: slow systems already run on their own cadence, independent of the step rate.
- **GC15** (:254) and **GC14** (:253): bolts are swept from the previous to the new position each step, so they never
  tunnel, but flight integrates the frame `dt`, so the landing point moves with the frame rate. Tells v2: keep swept
  tests at any tick; fix the step.
- **GC17** (:256): the Juggernaut ripples shells 80 ms apart, and a wall-clock idle bob moves its muzzles. Tells v2:
  vehicle fire timing in ticks, from fixed mount points.
- **GP9** (:319): hit-stop freezes `update()` for everyone for 0.12 s. Tells v2: hit-stop is client-only presentation
  time.
- **TL13** (:398) and **TL15** (:400): `tools/sim.js` and `tools/perfbots.js` step `update(1/60)`
  (tools/sim.js:19, tools/perfbots.js:24). At 48 soldiers in combat, one 1/60 s step costs 0.80–1.00 ms mean, p95
  0.97–1.54 ms (docs/reports/2026-10-16-three-teams-core.md:48; headless Node on the dev container, comparable only on
  the same machine). Tells v2: the legacy behaviour we have measured is at 60 steps per second; the legacy cost is no
  prediction of v2's (true 3D, physics, abilities), only an order of magnitude for this JS step.
- **KT13** (:422): the netcode rule already asks for a fixed tick, no wall clock and seeded randomness in the shared
  sim.
- **Tuning** (docs/LEGACY_TUNING.md, from docs/legacy-tuning.json): Simulation `FIXED_DT` 0.016666…; `WEAPONS`
  `fireInterval` 0.072 s (shabzon), 0.2 s (amk), 0.42 s (lancer) and `burstInterval` 0.06 s (lancer); `JUG.rippleMs`
  80; `T.moveSpeed` 6.3 m/s, `T.sprintMult` 1.4286, `T.gravity` 24, `T.jumpV` 11, `T.fragSpeed` 64; `T_CHARGE.bow.sMax`
  94 m/s. Arithmetic on these (inferred): the shortest legacy interval (0.06 s) is 1.8 ticks at 30 Hz and 3.6 ticks at
  60 Hz, so no legacy weapon needs two shots in one tick at either rate, but none of 0.072, 0.06 or 0.08 s is a whole
  number of ticks at 30, 60 or 64 Hz; a sprinting soldier (9.0 m/s) moves 0.30 m per tick at 30 Hz and 0.15 m at 60 Hz;
  the fastest bolt (94 m/s) 3.13 m and 1.57 m.

## 4. Options

### 4a. The tick rate
Arithmetic in this table (periods, per-tick distances, replay counts) is inferred from the findings and tuning above.

| Option | Shipped or official precedent (read) | Fit for SHAMBLE (VISION_V2 §4, §6) | Cost and risk |
|---|---|---|---|
| 20 Hz (50 ms) | Quake III world frame (F9); its player movement was integrated per user command, not per world frame (F12; the command rate is inferred) | Shots and movement quantised to 50 ms; legacy's jump at 20 Hz lost 11 % of its apex (GW14) | Cheapest; coarse input timing; would need movement or shots off the tick (Q3 style), which is the frame-rate dependence legacy and Q3 both fought (F12, GW14) |
| 30 Hz (33.3 ms) | None read for a shooter | Every legacy weapon fits (at most one shot per tick: tuning arithmetic in §3); up to 33 ms extra input wait and up to one tick of render-interpolation lag | Half the CPU of 60 Hz per simulated second; half the replay steps per correction (F24); legacy parity harness would run at a rate legacy never measured |
| 60 Hz (16.7 ms) | Rapier's default `dt` (F8); legacy `FIXED_DT` and every legacy measurement (CS1, TL13, TL15, GW14) | Finest of the practical options; one new tick per frame on the most common display rate, 60 Hz (F17; inferred); Rapier step equals the tick | Twice the CPU of 30 Hz; may beat against a 60 Hz display when steps are paced by frames (F6); 1/60 is not exact in binary floats (F6) |
| 64 Hz (15.625 ms) | Bevy's default fixed step, chosen against the 60 Hz beat and for an exact float `dt` (F6) | As 60 Hz; every legacy number shifts slightly when converted to ticks | As 60 Hz plus about 7 % more steps; no legacy measurement at this rate |
| 66 Hz (15 ms) | Counter-Strike on Source (F13) | Closest precedent for a competitive shooter | As 64 Hz; no Rust or web precedent read |
| Variable-duration commands (movement per input, world on a slower frame) | Quake III (F12) | Lets a slow world tick keep fine movement | Frame-rate dependent unless chopped (F12); exactly the legacy defect (GW14, GC15); no clean tick to replay or rewind |

### 4b. How slow systems relate to the tick
| Option | Trade-off |
|---|---|
| Everything every tick | Simplest; cost of AI, sight and route searches scales with the tick rate (F8's "more CPU time per second of simulated time" applies to the whole step) |
| One tick, with per-system cadences in whole ticks (bot thinking, sight budget per game second, route replans, economy, telemetry sampling), staggered by slot | What legacy already does in seconds (GN4); keeps 60 Hz affordable; cadence must be stated in game time so it holds at any tick |

### 4c. The server loop
| Option | Trade-off |
|---|---|
| Timer period as the clock (`tokio::time::interval`, default `Burst`) | Simple; timers are millisecond-granular (coarser on Windows, F23) so 16.67 ms cannot be hit exactly; `Burst` catches up without a bound (F23) |
| Monotonic clock accumulator: wake on a timer, compute due ticks from elapsed time (Q3's `timeResidual`, F10; Fiedler's accumulator, F2), bounded catch-up | Average rate exact regardless of timer granularity; needs an explicit overrun policy (F3, F7) |
| Integer-millisecond period (Q3's `1000 / sv_fps`, F10) | Simple integers, but 60 Hz becomes 62.5 Hz (inferred); avoid |

### 4d. The client loop (Worker sim, main-thread render)
| Option | Trade-off |
|---|---|
| W1. Main-thread `requestAnimationFrame` posts a frame message; the Worker steps its accumulator | Display-synced; rate varies 60–144 Hz and stops in background tabs (F17); a main-thread hitch stalls the sim |
| W2. Worker self-paced by `setTimeout` with a `performance.now()` accumulator | Independent of the main thread (F15); 100 µs clock resolution is ample (F20); timers fire late under load and their hidden-tab behaviour in workers is not stated in the sources read (F18, F19) |
| W3. Worker blocking loop on `Atomics.wait` with a timeout, woken early by `Atomics.notify` | Precise wake, lowest input latency; needs `SharedArrayBuffer` (cross-origin isolation, boundary unit) (F22); while blocked the Worker's own message queue is not served (inferred from "It is blocking"), so inputs must arrive through shared memory |
| W4. Worker `requestAnimationFrame` | Display-synced inside the Worker (F17); same rate variation and background pause as W1 |
Rendering stays on the main thread's `requestAnimationFrame` in every option and interpolates between the two newest
ticks (F4).

## 5. Recommendations

**R1-TICK-REC1. One fixed tick, counted in integers, the same constant on server and client; durations are converted
to ticks at load, with remainders carried.**
The simulation advances only in whole ticks of one build-time constant `dt`; a `u32` tick counter is the sim's only
clock (sim time = tick × `dt`); the step takes no time argument and reads no wall clock. Content data stays authored in
seconds (LOOP_V2: numbers in one table) and is converted to ticks when loaded (as Source does, F14). Cooldowns and
fire intervals keep their fractional remainder (the next allowed shot is scheduled from the previous scheduled time,
not from the tick it landed on), so a 0.072 s weapon fires at the same average rate at any tick rate. The tick rate is
not a server option (Valve removed `-tickrate` for its shooters, F13).
Reason: variable steps change outcomes (F1, F26), which legacy measured (GW14 jump reach, GC15 bolt landing), and a
fixed step is the precondition for reproducible runs (F5) and for netcode, replays and parity tests.
Rests on: F1, F2, F5, F13, F14, F26; legacy CS1, GP14, GW14, GC15, KT13, `WEAPONS`.
Serves: VISION_V2 §6.2 (jumps and ledges at height), §6.3 (abilities and effects built from data with timed
durations), §6.5 (economy and objectives server-side, tunable), §6.8 (moments; [LATER] replays not designed out).
A spike must prove (proposed pass marks):
- With held fire for 60 s of game time, every legacy `fireInterval` and `burstInterval` from docs/legacy-tuning.json
  gives the same shot count at 30 Hz and at 60 Hz, within 1 shot, and within 1 shot of 60 / interval.
- The same seed and the same input log, run twice on the same build, give the same state hash at every tick for
  36,000 ticks (identity across Wasm and native is the determinism unit's test).
- A CI check shows the sim crate's step reads no clock (no `std::time`, no host time import in the Wasm module).

**R1-TICK-REC2. Start the spike at 60 Hz, measure 30 Hz beside it on the same scene, and choose by the pass marks.**
60 Hz is the starting candidate: it is Rapier's default step (F8), legacy's declared step and the rate of every legacy
measurement (CS1, TL13, TL15, GW14), and it keeps player movement and shots on a fine fixed step. Quake III did not
integrate movement on its 20 Hz world frame but per user command, and still had to chop long moves "to prevent
framerate dependent behavior" and add an optional fixed 8 ms movement step (F12); a single fine tick avoids both
problems. Shipped shooters read span 20 Hz (F9) to 66 Hz (F13). 30 Hz halves the CPU per
simulated second (F8) and the replay work per correction (F24), and every legacy weapon still fits one shot per tick
(tuning arithmetic), so it is the measured fallback. 64 Hz stays an open question (OQ1).
Rests on: F6, F8, F9, F12, F13, F24; legacy CS1, GW14, TL13, TL15, `WEAPONS`, `T`.
Serves: VISION_V2 §4 (fair up to about 100 ms ping; 30+ fps minimum, 60 fps recommended), §6.1 (modes up to 48+ with
bots), §6.2 (aiming and jumping at height), §6.4 (big vehicles: physics steps with the tick).
A spike must prove (proposed pass marks; reference hardware proposed, to be named with R3, R5 and R6):
- P1 server, native: N = 12 / 24 / 36 / 48 / 64 soldiers (bots with full AI, worst case: all in one combat cluster),
  plus 4 big vehicles and 200 live projectiles, on a DM_Tri-sized greybox, 36,000 ticks per run, 3 runs. At 60 Hz: p99
  tick time at N = 48 at most 4.2 ms (25 % of 16.7 ms) on one core of the proposed reference server, max at most
  16.7 ms, zero overruns. At 30 Hz: p99 at most 8.3 ms (25 % of 33.3 ms). Report mean, p50, p95, p99 and max per N.
  The 25 % leaves room for snapshot encoding, interest management and several matches per core (R2, R6); headroom
  itself is Fiedler's advice (F3).
- P2 client, Wasm in a dedicated Worker on the proposed minimum laptop, offline at the largest mode (OQ7 decides N): p99
  tick time at most 40 % of the period (6.7 ms at 60 Hz, 13.3 ms at 30 Hz) while the main thread holds 30+ fps on Low
  for the whole run (VISION_V2 §4; R3 owns the render budget).
- P3 prediction replay on the same laptop: re-simulating the local soldier, or the vehicle it drives, for 200 ms of
  game time (12 ticks at 60 Hz, 6 at 30 Hz: 100 ms ping plus buffers, proposed) takes at most 1 ms p99.
- Decision rule: 60 Hz if P1–P3 pass at 60 Hz; otherwise 30 Hz if they pass at 30 Hz, with R2 weighing sub-tick input
  timestamps (OQ4); otherwise back to the drafts.

**R1-TICK-REC3. Slow systems run on cadences in whole ticks, stated in game time and staggered by slot.**
Bot target scans, sight-check budgets per game second, route replans, the economy trickle and telemetry sampling run
every k ticks (k from a cadence in seconds), with the work spread across ticks by slot, as legacy does in seconds
(GN4). Only movement, shots, projectiles, collisions and damage need every tick.
Reason: the per-tick cost then grows with what must be fine-grained, not with all of AI; this is what keeps REC2's
60 Hz candidate affordable (F8: cost grows per simulated second).
Rests on: F8; legacy GN4, GP14, TL15.
Serves: VISION_V2 §6.1 (bots fill empty slots), §6.5 (economy), §6.6 (balance telemetry).
A spike must prove (proposed): at N = 48 in P1's scene, p99 tick time at most 2 × the mean tick time (no tick where all
bots think at once); sight checks per game second per bot equal within 1 % at 30 Hz and 60 Hz.

**R1-TICK-REC4. The server counts due ticks from a monotonic clock, catches up within a bound, and sends snapshots
after the tick at R2's rate.**
Each match loop wakes on a timer but takes the number of due ticks from elapsed monotonic time (Q3's
`timeResidual`, F10; Fiedler's accumulator, F2), not from the timer period, because Tokio's timers are
millisecond-granular and coarser on Windows (F23). Catch-up per wake is bounded (proposed: at most 0.25 s of game time,
the clamp in Fiedler's loop, F3; 15 ticks at 60 Hz); a longer stall drops time and records it. Every overrun and every
dropped interval is a metric (R6 monitoring). Snapshots go out after the tick (as Q3 does, F10) at a send rate that is
separate from the tick (Q3: per client, up to 30 per second, F11); R2 sets that rate.
Rests on: F2, F3, F7, F10, F11, F23.
Serves: VISION_V2 §6.5 (authoritative economy and objectives), §6.6 (the server records what matters, overruns
included), §4 (one EU region, fair up to about 100 ms).
A spike must prove (proposed): on the reference server (Linux), tick start jitter (actual minus scheduled) p99 at most
2 ms over 36,000 ticks; after an injected 200 ms stall the loop is back on schedule within 1 s, never running more than
15 ticks in one wake; overrun and dropped-time counters appear in the metrics output.

**R1-TICK-REC5. In the browser, the Worker owns the tick; the main thread renders every frame and interpolates; a gap
is a pause offline and a resync online.**
The Worker paces the sim with a `performance.now()` accumulator (F2, F20) woken by `setTimeout` (W2, MDN's Worker
pattern, F15); `Atomics.wait` (W3, F22) is the measured alternative if W2 fails the jitter mark. `requestAnimationFrame`
is the render clock only, never the tick clock, because its rate follows the display and it stops in background tabs
(F17). The main thread draws every frame from the two newest ticks with `alpha` (F4), read from shared memory (boundary
unit). After a long gap (hidden tab, sleep: F18, F21) the client does not simulate the gap: offline it pauses, as
Bevy's paused clock stops its fixed schedule (F7, F16 "a pause"); online it resyncs from the server's state (F16:
"You can recover the game state from a peer or the server."), with catch-up per frame bounded as in REC4. Pause, shop, tutorial and
hit-stop never stop the shared tick in multiplayer; hit-stop is presentation-time only (GP9, GP13).
Rests on: F2, F4, F7, F15, F16, F17, F18, F20, F21, F22; legacy GP9, GP13.
Serves: VISION_V2 §4 (30+ fps on the minimum laptop, 60 fps recommended, on Chrome, Edge, Firefox and Safari), §6.9
(spectacle budget: the render loop is never blocked by the sim), and the brief's principle that the client runs the
same sim in a Worker for prediction and offline play.
A spike must prove (proposed), on Chrome, Edge, Firefox and Safari:
- Foreground Worker tick start jitter p99 at most 4 ms over 10 minutes.
- A unit moving at constant speed shows a per-frame on-screen displacement whose standard deviation is at most 5 % of
  its mean, on 60, 120 and 144 Hz displays.
- Hide the tab for 60 s, then return: offline, no tick ran while hidden and play resumes where it stopped; online, the
  client is back in step with the server within 0.5 s and never runs more than 15 ticks in one frame.

**R1-TICK-REC6. Map validation computes reach with the sim's own integrator at the shipped tick.**
The editor's reachability check (EDITOR_V2, LOOP_V2 §9) runs the sim's movement code at the shipped `dt`, not the
closed-form formula, because integration error depends on the step (F26) and legacy's formula overstates the real
reach (GW14).
Rests on: F26; legacy GW14, GN10.
Serves: VISION_V2 §6.2 (ledges, floors, bots that move between levels).
A spike must prove (proposed): the editor's reach value equals the measured jump apex plus step-up in the sim within
1 mm at the shipped tick, for the legacy values `T.jumpV` 11, `T.gravity` 24, `T.stepUp` 0.55.

## 6. Open questions

**R1-TICK-OQ1. 60 Hz or 64 Hz?** Bevy picks 64 Hz for an exact float `dt` and against a 60 Hz display beat (F6); legacy
parity and Rapier's default favour 60 (F8, CS1). Decided by: the determinism unit (does an exactly representable `dt`
matter for identical Wasm and native results?) and REC5's smoothness test (does a Worker-paced 60 Hz tick judder on a
60 Hz display?).

**R1-TICK-OQ2. Are worker timers throttled in hidden tabs?** The sources read do not say (F19); the WHATWG HTML timer
text is outside the allow list. Decided by: REC5's hidden-tab test in all four browsers, or a reading of the HTML
standard if the Advisor allows that domain.

**R1-TICK-OQ3. The tick rates of the brief's named references.** Overwatch's talk could not be read (GDC Vault
redirected to a maintenance page outside the allow list; video only), and Valve's networking pages returned HTTP 403.
Leads outside the allow list claim figures that are not cited here. Decided by: R2's reading of those sources, and the
consistency critic aligning both topics.

**R1-TICK-OQ4. Sub-tick input timestamps.** Whether shots carry their moment inside a tick, so a 30 Hz tick resolves
them as finely as 60 Hz. No source was read on this. Decided by: R2's lag-compensation unit; it matters only if REC2
falls back to 30 Hz.

**R1-TICK-OQ5. The real cost per tick of v2 at 48+.** No source read gives it; legacy's 0.80–1.00 ms per JS step
(TL15) is no prediction for a true-3D Rust sim with physics and abilities, and no Wasm-versus-native speed figure was
read. Decided by: REC2's P1–P3.

**R1-TICK-OQ6. The reference hardware.** VISION_V2 §4 names "a laptop with integrated graphics" and "a mid-range
graphics card", no CPU. P2 and P3 need a named minimum laptop CPU and P1 a named server core. Decided by: R3, R5 and R6
proposals, approved by Mustafa.

**R1-TICK-OQ7. Offline at the largest mode?** The brief's principle says the client runs the sim for offline play; VISION
does not say whether the largest mode (48 soldiers, 47 of them bots) must run offline on the minimum laptop. Decided by:
Mustafa and the Advisor; it sets N in P2.

**R1-TICK-OQ8. Client tick speed-up and slow-down.** Whether the client nudges its tick rate to keep its inputs arriving
just in time at the server. Not settled by any source read. Decided by: R2's time-sync design; REC5's loop must allow it.

**R1-TICK-OQ9. Big vehicles at 30 Hz.** If REC2 falls back to 30 Hz, do multi-part vehicles need physics substeps (two
Rapier steps per tick)? Rapier says smaller steps are more accurate and cost more (F8). Decided by: R4.

## 7. Sources read (all read 2026-10-27)
1. https://gafferongames.com/post/fix_your_timestep/ · Glenn Fiedler, "Fix Your Timestep!" · page dated 2004-06-10 (read twice for quotes)
2. https://gafferongames.com/post/snapshot_interpolation/ · Glenn Fiedler, "Snapshot Interpolation" · 2014-11-30
3. https://gafferongames.com/post/what_every_programmer_needs_to_know_about_game_networking/ · Glenn Fiedler · 2010-02-24
4. https://gafferongames.com/post/integration_basics/ · Glenn Fiedler, "Integration Basics" · 2004-06-01
5. https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_init.c · Quake III Arena GPL source, master, (C) 1999-2005
6. https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_main.c · same repository
7. https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_client.c · same repository
8. https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/g_main.c · same repository
9. https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_pmove.c · same repository
10. https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/shared/shareddefs.h · Source SDK 2013, master, no date
11. https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/gameinterface.cpp · Source SDK 2013, master, no date (first 100,000 characters)
12. https://developer.mozilla.org/en-US/docs/Games/Anatomy · MDN, "Anatomy of a video game" · last modified 2026-08-21
13. https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout · MDN · last modified 2026-10-26
14. https://developer.mozilla.org/en-US/docs/Web/API/Performance/now · MDN · last modified 2025-08-25
15. https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/requestAnimationFrame · MDN · last modified 2025-09-25
16. https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics/wait · MDN · last modified 2026-08-31
17. https://developer.chrome.com/blog/timer-throttling-in-chrome-88 · Chrome for Developers, "Heavy throttling of chained JS timers beginning in Chrome 88" · last updated 2021-01-18
18. https://docs.rs/bevy/latest/bevy/time/struct.Fixed.html · bevy 0.20.0 (first 100,000 characters)
19. https://docs.rs/bevy/latest/bevy/time/struct.Virtual.html · bevy 0.20.0 (first 100,000 characters)
20. https://docs.rs/rapier3d/latest/rapier3d/dynamics/struct.IntegrationParameters.html · rapier3d 0.36.1
21. https://docs.rs/tokio/latest/tokio/time/enum.MissedTickBehavior.html · tokio 1.54.1
22. https://docs.rs/tokio/latest/tokio/time/fn.sleep.html · tokio 1.54.1

By tier: official 11 (sources 12–22), shipped game (source code) 7 (sources 5–11), other 4 (sources 1–4, the
practitioner articles the brief names).

## 7b. Sources not read
1. https://developer.valvesoftware.com/wiki/Source_Multiplayer_Networking · HTTP 403 from the site (not retried)
2. https://developer.valvesoftware.com/wiki/Latency_Compensating_Methods_in_Client/Server_In-game_Protocol_Design_and_Optimization · HTTP 403 from the site (not retried)
3. https://gdcvault.com/play/1024001/-Overwatch-Gameplay-Architecture-and (and the www. host) · redirected to gdconf.com/gdc-vault-maintenance/, outside the allow list; the talk is video only
4. https://gdconf.com/gdc-vault-maintenance/ · outside the allow list
5. https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/mp/src/game/shared/shareddefs.h · HTTP 404 (the repository was restructured)
6. https://edgegap.com/blog/game-backend-deep-dive-overwatch-2016-netcode-architecture-rollback · outside the allow list (search lead)
7. https://www.pcgamesn.com/overwatch/overwatch-netcode · outside the allow list (search lead)
8. https://gamesbeat.com/overwatch-netcode-tick-rate-60hz/ · outside the allow list (search lead)
9. https://gamedev.net/forums/topic/696756-command-frames-and-tick-synchronization/5378869/ · outside the allow list (search lead)
10. https://en.wikipedia.org/wiki/Source_(game_engine) · outside the allow list (search lead)
11. https://wiki.facepunch.com/gmod/engine.TickInterval · outside the allow list (search lead)
12. https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html · outside the allow list (the timer rules MDN cites)

## 8. Cross-unit notes
- **R2 tick rate and bandwidth:** the snapshot and send rate is separate from the tick (F10, F11); remote-entity
  interpolation delay follows the send rate (F25); lag-compensation history has one entry per tick; a 16-bit tick on
  the wire wraps every 18.2 minutes at 60 Hz (inferred arithmetic); see OQ3, OQ4, OQ8.
- **R1 determinism unit:** float exactness of `dt` (OQ1); the tick counter as the only clock; seeded randomness drawn
  per tick.
- **R1 boundary and Worker unit:** the two newest tick states in shared memory for interpolation; W3 needs
  `SharedArrayBuffer` and cross-origin isolation; a Worker's `performance.now()` counts from the Worker's start, not
  the page's (F20).
- **R1 ability and effect unit:** durations, periods and cooldowns authored in seconds, converted to ticks with
  remainders (REC1).
- **R1 data and hot reload unit:** the tick constant underlies every converted duration; whether it may change on hot
  reload is that unit's call.
- **R4:** Rapier's `dt` equals the tick or a whole fraction of it (OQ9); swept tests for fast projectiles (GC15) or
  Rapier's CCD (F8).
- **R3:** render interpolation and frame pacing on 60 / 120 / 144 Hz displays (REC5); hit-stop as presentation time.
- **R5:** a Rust tick bench and a Worker tick bench in CI (REC2's P1–P3); the legacy-parity harness steps at 1/60
  (TL13).
- **R6:** tick overrun and dropped-time metrics (REC4); matches per core at 30 vs 60 Hz.
- **EDITOR_V2:** reachability runs the sim's integrator (REC6).
