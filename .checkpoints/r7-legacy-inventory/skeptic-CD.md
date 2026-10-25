Task: R7 legacy harvest, skeptic for area CD (core data and persistence), rows CD1-CD21
Checkout: a45c9fc; engine files + docs identical to 48ffb51 (git diff --stat empty). VISION_V2.md is NOT in 48ffb51; read from /home/user/Shamble-Frontlines/docs/VISION_V2.md (4afb714); §3, §4, §5, §6.1/.3/.4[OPEN]/.10 exist.
Dead T keys (grep T.<k> over 3 js + 2 html, no T[ ] access, no alias): sprintMult, camPosLean, camLookLean, maxLean, lookSideGain, posFwdGain, posSideGain, enemyCount, eliteCount, enemyPreferRange, enemyBurst, enemyDamage, incomeBase, incomeRefinery, meleeDamage/Range/Cooldown, shieldCost. eliteHealth read only at game:820 on en.elite branch; every bot elite:false (game:3212).
T.arenaX written only core:225 and game:77 setMapSize (dev slider game:3336 only); no per-map write (comment at :77 claims one). applyCamStyle only from dev buttons 3332/3334.
CD1 confirmed (STATUS:125 gate 134/691/349; STATUS:56 5c layering)
CD2 corrected: evidence misses TECHNICAL.md:206 26-assertion Node harness; where: editor MapStore :695-699, PrefabStore :704-708, TutStore game:2336-2338, ProgStore game:1952-1954
CD3 confirmed
CD4 corrected: v2_note "arena size per map, camera styles" false (dev-only); where game:77 is dev override
CD5 corrected: dev sliders are game:3326-3328 and 3332-3334 (3329-3331 halftone)
CD6 corrected: eliteHealth dead branch -> 6 dead keys
CD7 confirmed
CD8 confirmed (STATUS:27 NC-7 blast reach by LOS)
CD9 confirmed
CD10 confirmed (TEAMS.short never read too)
CD11 corrected: 27 TUNE. reads on 22 lines
CD12 corrected: falloff only on Shabzon (core:114 falloffStart 15/End 35/Min 0.2); reload+tracer in 6 of 7; CS9 (AUDIT:109) REMOVED javelin path and buy helpers
CD13 confirmed (unitOf 14 calls game)
CD14 confirmed (seats/desc 0 reads)
CD15 corrected: mark redesign (look brief pending), consistent with CD17; colour output = look pipeline
CD16 confirmed
CD17 corrected: v2_note "shadow pass roughly doubles draw calls" unsupported (AUDIT:16 main+shadow combined; TECH:249 incl shadow)
CD18 corrected: core 4 calls + definition (:395,:1445,:1457,:1470); sea/beach inline no-op :230,:231
CD19 confirmed
CD20 corrected: generateGrassTexture call in applyEnv is :206 not :205
CD21 corrected: v2_note setMapSize is dev-only override, not per-map
Missing: none substantive; V8 const-tracking-let workaround game:3374-3379 touches core lets (htOpacity, htSize, _natureGain) - other area
DONE
