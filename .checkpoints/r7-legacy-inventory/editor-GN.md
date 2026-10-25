R7 editor of area GN (legacy harvest)
Inputs read: rows-and-checks.json GN (14 rows, 14 checks, 8 missing), coverage.json suggested_rows naming GN (2).
Plan: apply skeptic corrections GN1,2,3,6,9,10,11,13,14; GN4,5,7,8,12 confirmed.
Missing-system decisions (checked in code):
- DEV.freezeAI: reject, GP5 covers DEV 786 and button IIFE 3320-3355 (3325); reads 2897/2899/2914 lie inside GN5/GN9 ranges.
- bot-victim applyDamage 1519-1523: reject, GC7 covers applyDamage 1506-1527; GN1 cites 1519, GN3 cites 1520.
- updateHostileCount 2457: reject, GP10 covers 2457.
- unitOf core:140: reject, CD13 covers core:140.
- objective/siege walk ignores speedMult (2917,2922,2926 vs 2909): REAL, widen GN9 (verified grep).
- activateRoster 762-779: reject, GC2 covers 762-779.
- bot anim driver 2936/2938: reject, GA15 (2936) and GA14 (2938).
- building sink 2944: reject, structure presentation not a bot system; GP14 cites 2944, damageBuilding sink start in GC7 (1561).
Coverage: "GN1 with 793 enemies" -> widen GN2 (roster array; 793 verified). "GN5 add 1113 _eEyes/_ePc" -> widen GN4 (used only at 1588,1595 in acquireTarget).
GN1: skeptic correction applied (what, where: bar colours by relation 771).
GN2: correction applied (what); widened with 793 enemies (coverage suggestion named GN1; the array is the roster).
GN3: correction applied (what: HQ toward centre, map spawn player-team only, golden-ratio scatter).
GN4: confirmed; widened with 1113 _eEyes/_ePc (used 1588, 1595; coverage named GN5).
GN5, GN7, GN8, GN12: confirmed, unchanged.
GN6: correction applied (where 1611-1638).
GN9: correction applied (what); widened: objective/siege walk ignores speedMult (2917, 2922, 2926 vs 2909), cadence 2923/2927.
GN10: correction applied (what). GN11, GN13: evidence corrections applied.
GN14: correction applied (where, v2_note); what widened to match the new dead fields.
final-GN.json written (14 rows).
DONE
