checker3 CS: R7 legacy harvest, checker for area CS (fixes3 ops). Started.
verifier-core#4 (CS10 where): accepted, fixed. grep -o Math.hypot editor -> 10, on 9 distinct lines, line 517 has two. Only that phrase changed.
verifier-core#5 (CS6 what): accepted, fixed; Node eval of core:1529-1543 reproduces 0/-1/0/-1. Minor: a ray with TWO NaN components (x,y NaN, z=1) also hits (eval -> 0), so 'only one NaN component' understates; suggest 'one or two'.
engine-architect#12 (CS8 v2_note once per frame): accepted, fixed. game:1444 frame guard, 2672 _frameId++, re-syncs 778/2862/editor:564 are listed in CS8 what; core:1513-1514 comment says 'once per / shot' (verified sed). Both 'per-shot' phrases replaced; nothing else changed.
engine-architect#3 (CS8 v2_note boundary fact): accepted, fixed. All cites hold: game:2780, 2933, 2710, 1514, 1420, 2857, 1452, 2526, 85 (Box3().setFromObject(m)), 506, core:1044 (timberHouse Box3.setFromObject), core:1430 _bkWorldBoxes.
engine-architect#8 (CS8 evidence stale 561 B): accepted, fixed. STATUS.md:118 still says 561 B; reports/2026-10-16-three-teams-core.md:31 says array removed; game:1458 hb?hb[k]:houses[i].box no array; GW16 says same.
engine-architect#15 (CS5 v2_note): accepted, fixed. EDITOR_V2.md:32 (15/45/90 snaps) and :35 (120 mirroring) both under ## 3 (line 28); rayStatic core:1519-1524 handles any Y rot via rc/rs; VISION_V2 §6 (lines 61-81) has no rotation need.
engine-architect#16 (CS1 where): accepted, fixed (better than proposed op). tools/tuning-dump.js:12 lists 'BTN' and 'FIXED_DT'; core:19 header comment names both; repo grep (js/html/mjs, excl .checkpoints/.gate) finds only core:19,1479-1481 and tuning-dump:12.
netcode-architect#17 (CS2 v2_note): accepted, fixed. 'model is fitted to it' removed; content data draft exists (V2_PHASE0_BRIEF.md:130-132, 'content data and the ability system'). Rest of field unchanged.
Problems: 1 minor (CS6 'only one NaN component' -> 'one or two'; eval rayStatics(S,0,1,-5,NaN,NaN,1,10,true) -> 0).
DONE
