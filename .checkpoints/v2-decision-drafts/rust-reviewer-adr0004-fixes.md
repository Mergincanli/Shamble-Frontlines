# Fixes for ADR-0004 (the simulation core), plan step 9

Author: rust-reviewer, 2026-11-01. Against `docs/adr/0004-simulation-core.md` as assembled (1,235 lines; checked at
the branch head d7f8d9a). Each BEFORE was copied from the file by script and is unique in it; the fixes were applied
in this order to a scratch copy and each BEFORE stayed unique, so they do not overlap. Rows of the "### Pass marks"
table (test-engineer's) are not touched. No fix changes a decision; each makes the text true to its source.

Findings with no FIX entry:
- Row 20: no change. STATUS now records UPGRADE-SCOPE under "Decisions pending" ("Taken by Mustafa 2026-10-31, about
  18:41 … in his words", docs/STATUS.md:94–96, commit 549490f), so doc:50's "recorded in STATUS with the day's
  decisions" is true as written.
- Row 38: refuted by the skeptic; no change.
- N3: the fix adds per-fragment damage to the list after OQ-0004-7, but says "Not yet in that register", because
  `docs/adr/README.md`'s register has no entry for it (its only fragment line is G5, visual effects). Adding it there is
  the README owner's (engine-architect), if wanted.

Ledger rows that change with these fixes (rust-reviewer-adr0004-claims.md): 39 (STATUS, "Open bugs and risks",
line 327, added: settled), 166 (now F8's words: research verified, F8 checked), 182, 253 (r1:1719 quote: research
verified, F34 checked at R1's fix step), 304, 305, 329, 335, 337 (the 60 Hz halves moved to the 60 Hz margin's test,
as PM-0004-72), and new rows 361 (N1, the Options preface) and 362 (N3, per-fragment damage). The restated rows are in
the ledger's last section, "Fix round".

### FIX row-166
BEFORE:
    - Why: C++ Detour cannot be linked into the browser build, no Rust navigation crate promises determinism, baking
      once makes generation's floats shared data, and Quake III shipped bots on a precomputed area graph.
AFTER:
    - Why: C++ Detour cannot simply be linked into the sim's Wasm build (`wasm32-unknown-unknown` has no C/C++
      toolchain, R4-NAV-F8, and R4-NAV-REC1 reads linking it as not a supported path), no Rust navigation crate
      promises determinism, baking once makes generation's floats shared data, and Quake III shipped bots on a
      precomputed area graph.
END

### FIX row-39
BEFORE:
- Bot navigation does not scale: the 2 m grid closes gaps of 3 to 5.4 m and sends 27.5 % of objective routes across a
  roof bots cannot climb (GN11; r4-physics-navigation.md, legacy facts, lines 391–393), the route budget resets per
AFTER:
- Bot navigation does not scale: the 2 m grid can close gaps of 3 to 5.4 m, depending on grid alignment (STATUS,
  "Open bugs and risks"), and sends 27.5 % of objective routes across a roof bots cannot climb (GN11;
  r4-physics-navigation.md, legacy facts, lines 391–393), the route budget resets per
END

### FIX row-182
BEFORE:
    research bakes two states and a ruin's shape (R4-OBST-REC2), so the third stage is an extension the sim core spike
AFTER:
    research bakes two states, the second of which may be a ruin's shape (R4-OBST-REC2), so the third stage is an
    extension the sim core spike
END

### FIX N1
BEFORE:
Every option below comes from a research Options section (cited per question). Each table carries template 0000's
AFTER:
Every option below comes from the research or a settled input, cited per question (the Options sections for Q1–Q8,
GLOSSARY §4 for Q9, UPGRADE-SCOPE for Q10, R1-RR-OQ3 for Q11). Each table carries template 0000's
END

### FIX row-253
BEFORE:
  portable, reference-tested, 2^64 streams per seed and a settable position); `rand_pcg`'s Pcg32 (portable, fast on 32-
  and 64-bit, but stream parameters "may be closely correlated", R1-DET-OQ11); legacy `_mulberry32` (tiny, quality not
AFTER:
  portable, reference-tested, 2^64 streams per seed and a settable position); `rand_pcg`'s Pcg32 (portable, "a good
  choice on both 32-bit and 64-bit CPUs", but stream parameters "may be closely correlated", R1-DET-OQ11); legacy
  `_mulberry32` (tiny, quality not
END

### FIX N3
BEFORE:
(R4-OBST-OQ2); player building (R4-OBST-OQ3); offline play at the largest mode (R1-TICK-OQ7, R1-BND-OQ1, critic rank
15); moment rules and thresholds (R6-TEL-OQ3); the caps on vehicles and projectiles that replace the stress scene's
counts (R2-BUD-OQ5).
AFTER:
(R4-OBST-OQ2); player building (R4-OBST-OQ3); offline play at the largest mode (R1-TICK-OQ7, R1-BND-OQ1, critic rank
15); moment rules and thresholds (R6-TEL-OQ3); the caps on vehicles and projectiles that replace the stress scene's
counts (R2-BUD-OQ5). Not yet in that register: whether grenade fragments deal damage per fragment (LOOK_V2 §3 and §7
bring fragmentation back as a visible effect, which ADR 0003 draws from the blast event; a rule for Mustafa, with no
research id).
END

### FIX N6-S3
BEFORE:
| S3 | Cadences keep the sim step time flat from tick to tick | 4 | p99 sim step time ≤ 2 × mean; sight checks per game second per bot equal within 1 % at 30 Hz and 60 Hz (R1-TICK-REC3, lines 2074–2076) | the sim core spike |
AFTER:
| S3 | Cadences keep the sim step time flat from tick to tick | 4 | p99 sim step time ≤ 2 × mean at 30 Hz; sight checks per game second per bot at 60 Hz within 1 % of the 30 Hz count, the 60 Hz form only in the 60 Hz margin's test (R1-TICK-REC3, lines 2074–2076) | the sim core spike |
END

### FIX N6-S4
BEFORE:
| S4 | One time representation holds every rate | 4 | with held fire for 60 s of game time, every legacy `fireInterval` and `burstInterval` gives the same shot count at 30 and 60 Hz within 1 shot and within 1 shot of 60 / interval (R1-TICK-REC1, lines 2023–2024), which also meets R1-ABL-REC3's 1 % for the Shabzon (1 shot in 833 is 0.12 %: arithmetic); CD13's Warrior-kills-Conscript time of 0.86 s within one tick (R1-ABL-REC3, lines 2416–2419); a CI check finds no clock read in the step (no `std::time`, no host time import in the Wasm module: R1-TICK-REC1, line 2027); the units-per-tick figure chosen from the rounding report (proposed: the research gives none) | the sim core spike |
AFTER:
| S4 | One time representation holds every rate | 4 | with held fire for 60 s of game time, every legacy `fireInterval` and `burstInterval` gives a shot count within 1 shot of 60 / interval at 30 Hz, and the same count within 1 shot at 60 Hz, the 60 Hz form only in the 60 Hz margin's test (R1-TICK-REC1, lines 2023–2024), which also meets R1-ABL-REC3's 1 % for the Shabzon (1 shot in 833 is 0.12 %: arithmetic); CD13's Warrior-kills-Conscript time of 0.86 s within one tick (R1-ABL-REC3, lines 2416–2419); a CI check finds no clock read in the step (no `std::time`, no host time import in the Wasm module: R1-TICK-REC1, line 2027); the units-per-tick figure chosen from the rounding report (proposed: the research gives none) | the sim core spike |
END

### FIX N6-S9
BEFORE:
| S9 | parry3d plus our own movement is enough | 10 | the six marks of R4-PHYS-REC1 (r4-physics-navigation.md, lines 692–717): movement parity (a 0.55 m step climbed and 0.56 m stopped; the jump apex within 1 mm of the integrated value; 0 tunnelling through a 0.2 m wall in 10,000 random moves at about 13.3 m/s, which is 0.44 m per tick at 30 Hz); movement plus all physics queries ≤ 1 ms native mean with p99 reported, Wasm ≤ 2 × native; 0 per-tick hash mismatches native against Wasm and a byte-identical static BVH; the sim .wasm size with parry3d, with rapier3d and with neither, after wasm-opt and gzip (also R1-ECS-REC1 mark 5, lines 1946–1949, against a bevy_ecs 0.20 stub, and a one-line incremental native rebuild ≤ 10 s); Rapier's controller measured once as the fallback; 0 `Err(Unsupported)` shape pairs. Also R4-PHYS-REC2 (lines 739–743: 20,000-ray parity with the legacy SIM, 0 of 1,200 bot shots more than 1 cm above a ramp, byte-identical queries after a restore, static BVH build ≤ 50 ms native), REC3 (lines 758–760: 0 tunnelling at the fastest projectile speed at 30 and 60 Hz, grenades bounce back in 1,000 of 1,000 throws, identical landing points), REC4 (lines 779–784: 0 ticks with a soldier or wall more than 1 cm inside a vehicle hull over 10 minutes with 48 soldiers) and REC5 (lines 796–798: the marks hold at the far corner of the largest planned map; distance per second within 0.1 %) | the sim core spike |
AFTER:
| S9 | parry3d plus our own movement is enough | 10 | the six marks of R4-PHYS-REC1 (r4-physics-navigation.md, lines 692–717): movement parity (a 0.55 m step climbed and 0.56 m stopped; the jump apex within 1 mm of the integrated value; 0 tunnelling through a 0.2 m wall in 10,000 random moves at about 13.3 m/s, which is 0.44 m per tick at 30 Hz); movement plus all physics queries ≤ 1 ms native mean with p99 reported, Wasm ≤ 2 × native; 0 per-tick hash mismatches native against Wasm and a byte-identical static BVH; the sim .wasm size with parry3d, with rapier3d and with neither, after wasm-opt and gzip (also R1-ECS-REC1 mark 5, lines 1946–1949, against a bevy_ecs 0.20 stub, and a one-line incremental native rebuild ≤ 10 s); Rapier's controller measured once as the fallback; 0 `Err(Unsupported)` shape pairs. Also R4-PHYS-REC2 (lines 739–743: 20,000-ray parity with the legacy SIM, 0 of 1,200 bot shots more than 1 cm above a ramp, byte-identical queries after a restore, static BVH build ≤ 50 ms native), REC3 (lines 758–760: 0 tunnelling at the fastest projectile speed at 30 Hz (the 60 Hz form only in the 60 Hz margin's test), grenades bounce back in 1,000 of 1,000 throws, identical landing points), REC4 (lines 779–784: 0 ticks with a soldier or wall more than 1 cm inside a vehicle hull over 10 minutes with 48 soldiers) and REC5 (lines 796–798: the marks hold at the far corner of the largest planned map; distance per second within 0.1 %) | the sim core spike |
END

### FIX N6-S11
BEFORE:
| S11 | Floors and links are correct by construction | 12 | R4-FLOORS-REC1 (lines 918–923: 0 generated jump links above the integrated reach; each kept link succeeds in 1,000 of 1,000 traversals at 30 and 60 Hz; 0 % of routes need an unreachable surface; drop-downs taken when shorter in ≥ 95 % of 200 trials); R4-FLOORS-REC2 (lines 936–939: separate layer and area ids per floor, no walking through ceilings in 100 of 100 trials, the barracks roof not walkable); R4-FLOORS-REC3 (lines 951–952: a changed `jumpV` without a re-bake is reported stale, 1 of 1) | the sim core spike |
AFTER:
| S11 | Floors and links are correct by construction | 12 | R4-FLOORS-REC1 (lines 918–923: 0 generated jump links above the integrated reach; each kept link succeeds in 1,000 of 1,000 traversals at 30 Hz (the 60 Hz form only in the 60 Hz margin's test); 0 % of routes need an unreachable surface; drop-downs taken when shorter in ≥ 95 % of 200 trials); R4-FLOORS-REC2 (lines 936–939: separate layer and area ids per floor, no walking through ceilings in 100 of 100 trials, the barracks roof not walkable); R4-FLOORS-REC3 (lines 951–952: a changed `jumpV` without a re-bake is reported stale, 1 of 1) | the sim core spike |
END

### FIX N6-S13
BEFORE:
| S13 | Changing structures and damage stages flip in one tick | 14 | R4-OBST-REC1 (lines 1037–1041: 0 bots stuck over 3 s against a vehicle, ≤ 2 replans per bot per second at p99, identical hashes); R4-OBST-REC2 (lines 1058–1061: route, sight and portal change in the same tick with 0 ticks of disagreement over 1,000 destroy and rebuild cycles; 1,000 traversals after a flip; a flip ≤ 0.1 ms native at p99); the same marks with three stages per structure on the two-floor test building, including a plank link that exists only in the ruin stage, and the bake with every stage still within S10's bake-time mark (proposed: LOOK_V2 §4 names three stages and the research tested two) | the sim core spike |
AFTER:
| S13 | Changing structures and damage stages flip in one tick | 14 | R4-OBST-REC1 (lines 1037–1041: over 10 minutes at 30 Hz (the 60 Hz form only in the 60 Hz margin's test): 0 bots stuck over 3 s against a vehicle, ≤ 2 replans per bot per second at p99, identical hashes); R4-OBST-REC2 (lines 1058–1061: route, sight and portal change in the same tick with 0 ticks of disagreement over 1,000 destroy and rebuild cycles; 1,000 traversals after a flip; a flip ≤ 0.1 ms native at p99); the same marks with three stages per structure on the two-floor test building, including a plank link that exists only in the ruin stage, and the bake with every stage still within S10's bake-time mark (proposed: LOOK_V2 §4 names three stages and the research tested two) | the sim core spike |
END

