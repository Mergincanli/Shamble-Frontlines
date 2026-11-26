# game-designer: which legacy tuning fields are durations (PM-0004-11 input), cs/4b-content step 5

## Plan
1. Read docs/legacy-tuning.json `about` and per-table notes (where each table lives in legacy code).
2. Walk duration-candidates.txt (727 fields) table by table; for every field whose name does not settle it, grep
   legacy/shamble-core.js / legacy/shamble-game.js for its use (dt, clock compare, rate, fraction, anim phase).
3. Write legacy-duration-paths.txt (field, unit, rules|look, evidence), sorted as candidates.
4. Notes blocks (a)-(f).

## Progress
- plan written; tables read (per-table `where` notes); code uses grepped; list written; list checked by a read-only
  node one-liner (every field present in the candidates file, order kept, 4 columns, counts below). DONE.

All line numbers: legacy/shamble-game.js (game.js) and legacy/shamble-core.js (core.js) on this worktree.

## (a) Counts
- On the list: **52 fields holding 69 numbers** (of 727 fields / 2,038 numbers).
- By unit: s 50 fields / 67 numbers; ms 2 fields / 2 numbers (JUG.rippleMs 80, WR_DEPLOY_MS 280).
- By kind: rules 42 fields / 59 numbers; look 10 fields / 10 numbers (T.dustLife, T.fragLife, T.scorchLife,
  WEAPONS.swingDur, WR_DEPLOY_MS, ANIM.cycleMax, ANIM.cycleMin, ANIM.death.buckleT, ANIM.idleStepT, ANIM.throw.t).
- Every listed number is a whole number of milliseconds except FIXED_DT (1/60 s = 16.67 ms); at U = 3000 every one is a
  whole number of units (FIXED_DT: 50 units, off by the double's own error of 1/60, about 1e-17 s).
- boot_changes: no path there changes a duration field. The changed paths are BK.PROPS / BK.TREES sizes, HARV.fieldZ
  and HARV.lanes positions, TBOX/SILO points, NAV grid sizes (read from the file's boot_changes section); none listed.
- skipped: 36 entries, all functions (CPARTS.*, GUN_KIT.*.make, BK.build/buildCover/roll/tex, AI_STATE.rotatePush): no
  numbers, none listed. Durations hard-coded INSIDE functions are outside the tuning file; see (e) and (f).

## (b) Doubtful fields put ON the list
- T.meleeCooldown 0.55 s rules: defined at core.js:84, read nowhere in legacy/; a duration by meaning, kept.
- RIFLE_TIERS[*].reload 5 / 3.5 / 2.5 s rules: game.js:788; only `.span` is read (789-790, 1863); kept as above.
- FIXED_DT 1/60 s rules: core.js:1479, read only by makeInputCommand (core.js:1481), which nothing calls. It is the
  legacy sim step itself, under one 30 Hz tick by definition; see (d).
- TUNE.ai.losCapS 0.1 s rules: not a timer but a cap in seconds of refill (budget max = losPerS x losCapS,
  game.js:2889); it converts like a duration, so it is listed.
- T_CHARGE.max 1.0 s rules: the name says "max", but it is the full-charge time of jav/nade throws (charge.t capped at
  it, chargeFrac = t/max, game.js:1707,1729).
- STAM.max 7 s rules: "7s full sprint" (game.js:2216); stamina -= dt/STAM.max (2771). The bar itself is a 0..1
  fraction; max is the drain time.
- JUG.rippleMs 80 ms rules (not look): each ripple step fires a shell (_jugFirePilot, game.js:1345).
- WEAPONS[*].impact 0.13 s rules: swingT >= swingImpact triggers meleeHitCheck (game.js:2742), a hit time in seconds.
- WEAPONS[*].swingDur 0.4 s look: drives the swing pose (swingT/swingDur) and ends the swing (game.js:2740-2743);
  ps.swinging is read only by the pose and model code (1688-1689, 2739). Rules only through impact <= swingDur.
- T.enemyBurstPause / T.enemyFireInterval rules: both get random terms added in code (+0.6 s, x1.6 +0.4 s;
  game.js:2913, 2923, 2927); the table value is still a duration.

## (c) Duration-LIKE names left OFF (not a duration)
- T.camLerp, T.lookLerp, T.camPushLerp, T.camYawLerp: rates per second, 1-exp(-k*dt) (game.js:2837, 2811, 2750).
- T.camKickStiffness, T.camKickDamping: spring constants (game.js:2841-2843).
- T.regenRate: health per second (game.js:2700).
- T.incomeBase, T.incomeRefinery: income figures, read nowhere (core.js:81); rates by name, not durations.
- T.camYawSens: radians per mouse pixel (core.js:68).
- T.enemyBurst, UNIT_TYPES[*].burst, WEAPONS[*].burst: shot counts (game.js:2913, 2726).
- TUNE.ai.losPerS: line-of-sight checks per second (game.js:2889).
- WEAPONS[*].spreadDecay: spread per second, spread - spreadDecay*dt (game.js:2720).
- UNIT_TYPES[*].lunge: lunge damage (game.js:2911, passed to applyDamage); the lunge cooldown 2.5 s is hard-coded.
- RIFLE_TIERS[*].span: kills per tier (game.js:789-790).
- ECO.payTrickle, ECO.payTrickleDead, ECO.paySilo: credits per second per player (game.js:690-692).
- JUG_PILOT.turn, JUG_PILOT.aimTurn: turn rates per second, min(1, dt*turn) (game.js:1329, 1340).
- T_CHARGE.*.floor: release height in metres (game.js:1735-1748).
- STAM.regen: fraction of the bar per second (game.js:2772). STAM.lockUntil: fraction of the bar (game.js:2772).
- ANIM.breathRate: angular rate inside sin(t*rate) (game.js:363).
- ANIM.spr.*.k, ANIM.flinch.k, ANIM.aim.adsK: spring stiffness ("settle ~4/sqrt(k) s" is derived, game.js:149, 188).
- ANIM.duty[*]: share of a gait cycle (game.js:136, 308).
- ANIM.reload.styles.*.keys[*][*]: key[0] is a 0..1 phase of the reload's progress (game.js:169-170); progress =
  reloadT / w.reload (game.js:2685, 2782). The other key entries are weights and a stroke in metres.
- ANIM.idleSpeed: speed threshold (game.js:283). ANIM.death.lie: angle in radians (game.js:465). ANIM.death.g:
  angular gravity (game.js:180, 464). ANIM.katana.sweep: metres x sin (game.js:430).
- GUN_KIT.*.reload.at[*]: the point on the gun the support hand works at (core.js:740, 786-788).
- OCC_FADE: ghost opacity (game.js:2555).
- AI_STATE.step / budget / checks / scans, NAV.*: run-time state counters and grid sizes (game.js:1581, 2889).
- TBOX.range, SILO.range, HARV.speed, JUG.speed / maxSpeed: metres and speeds.
- boot_changes.*: sizes, positions, grid sizes (see (a)).

## (d) Rule intervals under one tick at 30 Hz (under 0.0333 s)
- Only FIXED_DT (0.0167 s = 0.5 tick). It is the legacy step, not a play interval, and is unread in play (b). The
  report either keeps it with a note "the legacy step, replaced by the v2 tick" or the list drops it; the caller decides.
  Counted as a play interval, it makes "0 legacy intervals under one tick" fail.
- Next smallest rules intervals: WEAPONS.burstInterval 0.06 (1.8 ticks), Shabzon fireInterval 0.072 (2.16), JUG.rippleMs
  0.08 (2.4), Warrior bot fireInterval 0.09 (2.7), losCapS / losRecheckS / molotov reload 0.1 (3.0). None under a tick.

## (e) Design question for Mustafa
- DUR-1, which fire rate is "the legacy rate"? Legacy never carries a timer's remainder: fireOnce sets
  ps.fireCd = interval (game.js:1468-1471), update subtracts the frame's dt (2690), and the frame's dt varies, capped at
  0.05 s (3053). So a shot waits a whole number of frames: the Shabzon's 0.072 s fired every 5 frames at 60 fps
  (83.3 ms, 720 a minute) and every 11 frames at 144 fps (76.4 ms, 785 a minute). ADR 0004 part 4 carries the remainder,
  which gives exactly the table's 833 a minute at 30 or 60 Hz; PM-0004-9 checks against 60/interval.
  Options: A, the table value is the rate (exact, the same at every tick rate; the Shabzon is about 16 % faster than at
  60 fps in legacy); B, keep the 60 fps legacy feel (re-author 0.072 to 0.0833 and similar); C, A now, then tune by
  playtest. Recommendation: C (the tuning file calls itself "Starting values, not decisions", its `about.what`). It
  reaches the 3-round burst too: 0.06 s gives 2-tick gaps at 30 Hz (66.7 ms), the same as legacy at 60 fps.
- For the caller, not Mustafa: rule durations hard-coded in legacy functions are outside the tuning file, so the report
  cannot see them: bot lunge cooldown 2.5 s (game.js:2911), bayonet-end fire lock 0.2 s (1403), weapon-swap fire locks
  0.12 / 0.15 / 0.1 s (1153, 1161, 1378, 1386, 1392, 1400), bot burst-pause jitter 0.6 s and siege x1.6 +0.4 s
  (2913, 2923, 2927). The v2 tables need them as data when those rules are built ("numbers live in data").

## (f) Not checked
- Durations hard-coded in functions: only the ones above, met while reading; no full sweep of legacy/ (hitStop and
  the FX tick lifetimes were not searched).
- Rules vs look rests on the code paths cited, not on a run; legacy was not executed.
- Values are as the candidates file writes them (its 12-distinct cap is not reached by any listed field); not re-read
  from docs/legacy-tuning.json number by number.
- Nothing about v2 code: the loader, the report script and the U = 3000 constants were not read.
