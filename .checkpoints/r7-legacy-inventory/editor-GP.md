R7 editor of area GP (legacy harvest)
Inputs read: rows-and-checks.json GP (16 rows, 16 checks, 2 missing), coverage.json suggested_rows naming GP (3).
Plan: apply skeptic corrections GP2 (evidence), GP5 (what), GP10 (v2_note), GP11 (what), GP13 (v2_note), GP15 (v2_note), GP16 (what; v2_note). GP1,3,4,6,7,8,9,12,14 confirmed.
Missing-system decisions (checked in code):
- load-time side effects outside bootGame (66,69,611,683,1099,1702-1705,1861-1863): REAL cross-cutting fact, each statement owned per system elsewhere (CL12/GW3 66, GW5 69, GW10 611, CC6 683, GC13 1702-1705, GE17 1861-1863, GP10 1099); widen GP15 what+where (verified sed).
- game-over restart inputs: R branch 3264 widens GP2; clicks 3305-3306 already GE12 (restart clicks :3305-3306) -> those parts rejected.
Coverage: (1) HUD handles -> widen GP10 with 1096-1099, 1854-1858, 2342, 2444, 2512, 2546; 3044 _curX/_curY -> GP2 (placeCursor 3045-3052); 2418-2419 era HUD rejected (GE4 owns updateEraHUD 2420). (2) GP4 + 1094 camPitchOverride/adsPitchK/adsPanK, 1413 camRecoil/camShake (verified, read 2822-2846). (3) GP3 + 1115 _wish (used 2757, 2763), 2603 WORLD_FWD/WORLD_RIGHT (never read).
GP13 reason: VISION_V2.md now exists on disk (absent in 48ffb51 tree); reason re-cited to V2_PHASE0_BRIEF:121-122 (R1 fixed tick, Worker; R2 snapshots/interpolation).
GP1, GP3(base), GP4(base), GP6, GP7, GP8, GP9, GP12, GP14: confirmed, unchanged by corrections.
GP2: evidence correction applied; widened: R restart-or-reload 3264, scroll 3275, _curX/_curY 3044 (coverage).
GP3: widened (coverage): _wish 1115 (2757, 2763), WORLD_FWD/RIGHT 2603 never read (grep: only 2603).
GP4: widened (coverage): 1094 cam vars, 1413 camRecoil/camShake (reads 2838, 2846).
GP5: what correction applied; IIFE line fixed to 3321-3355 (3320 is the comment), dvMap 3336 noted.
GP10: v2_note correction applied; widened (coverage): handles 1096-1099, 1854-1858, 2342, 2444, 2512, 2546; calls 1470, 2688, 2515.
GP11: what correction applied.
GP13: v2_note correction applied; reason re-cited to V2_PHASE0_BRIEF:121-122.
GP15: v2_note correction applied; widened (missing system 1): load-time statements outside bootGame listed with owners.
GP16: what and v2_note corrections applied.
final-GP.json written (16 rows).
DONE
