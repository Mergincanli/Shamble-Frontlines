applier3-CS (R7 legacy harvest, fixes3 applier for area CS)
verifier-core#4 (CS10 where): accepted. grep -o Math.hypot shamble-editor.js -> 10, on 9 lines (517 has two). Op: "10 Math.hypot calls on 9 lines (two on shamble-editor.js:517)".
verifier-core#5 (CS6 what): accepted. Node eval of core:1529-1543: (NaN,0,1) dir maxT 10 -> 0 hit; all-NaN -> -1; maxT NaN -> 0; no statics -> -1. Fix_op as written.
engine-architect#12 (CS8 v2_note): accepted. game:1444 _frameId guard, game:2672 _frameId++, forced syncs game:778/2862/editor:564; only core:1513-1514 comment says once per shot. Changed to once per frame + forced re-syncs, noted the stale comment, also "per-shot rebuild" -> "per-frame rebuild".
engine-architect#3 (CS8 v2_note): accepted. game:2780, 2933, 2710, 1514, 1420, 2857, 1452, 2526, 85 Box3.setFromObject, core:1430 _bkWorldBoxes all verified. Appended (no leading double space; added game:506 and core:1044 Box3.setFromObject; dropped "prop").
engine-architect#8 (CS8 evidence): accepted. reports/2026-10-16-three-teams-core.md:31 says the per-house array is gone; game:1458 hb?hb[k]:houses[i].box. Fix_op as written.
engine-architect#15 (CS5 v2_note): accepted. EDITOR_V2.md:32 (15/45/90 snaps) and :35 (120 deg mirror); no rotation need in VISION_V2. Op adds "the oriented-box test already covers it".
engine-architect#16 (CS1 where): accepted with better op: tools/tuning-dump.js:12 names BTN and also FIXED_DT; core:19 header comment names them too.
netcode-architect#17 (CS2 v2_note): accepted. V2_PHASE0_BRIEF.md:131-132 lists content data as its own draft; fix_op as written.
apply-ops.py CS: 18 ops applied cleanly (fixes2 + fixes3).
DONE
