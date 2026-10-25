# editor-GA: R7 legacy harvest, editor of area GA
started; inputs loaded
Skeptic corrections verified (lines checked by grep/sed at current main, engine files identical to 48ffb51):
- GA1 what corrected (hold styles rifle/smg/scatter; velocity 279 and thrW 370 use own exp smoothing). Widened Where with scratch 231-233, _aRk 237 (skeptic missing #4 + coverage WIDEN).
- GA2 v2_note corrected (one invert per call at 221; more in updateSoldierAnim 354,359,387,422,428,433; ARM_L1/L2 0.46/0.42 vs plan 0.58/0.59).
- GA3 what/where/evidence corrected (eager init at bot spawn 774).
- GA4-GA7, GA9, GA10, GA13-GA15 confirmed.
- GA5 widened: airborne foot pose 320 (skeptic missing #1).
- GA8 where corrected (GUN_KIT 785-791, five guns); widened with A.supT metric hook 412 (no checked-in reader: grep supT finds only 252/412) (missing #3).
- GA11 where (makeAxe 1448) and v2_note (swing tick 2740-2741, meleeHitCheck 2742) corrected.
- GA12 what corrected (greatcoat 439-443 runs after flinch).
- GA15 widened: aim-pitch side channel A.wrx/A.aimPitch 369/375/417, written by pitchWeapon 1428 (owned by GC6) (missing #2).
- GA16 where/evidence corrected (editor 245-248, 307/308/315/317; topple writes 685,1520,1826,1827,1842,2683,2938,2939).
- final-GA.json written (16 rows, ids GA1-GA16, no new rows). Changed for missing/coverage: GA1, GA5, GA8, GA15.
- Rejected: none outright; skeptic missing #4 and coverage WIDEN both merged into GA1 (same system); supT merged into GA8 not a new row (write-only probe, no reader).
DONE
