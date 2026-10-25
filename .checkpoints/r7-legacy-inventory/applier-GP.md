R7 applier GP: fixes2-ops-GP.json from edit-verify-result corrected checks (GP2, GP3, GP4, GP10, GP15)
GP2: v2_note, 2 replaces (keydown/wheel branch lines 3255/3293; optional DEV.allGuns default-false sentence added after the Digit2 sentence, 786 verified).
GP3: what replace; where 3 replaces (yaw lines marked shared with GP4; stamina writes credited to GP10; tuning T values appended after WORLD_FWD entry, values verified at core 67/76/85/128-137).
GP4: what replace; where 3 replaces (camShake/camRecoil detail, crosshair 2850-2854 removed, T fields 68-75 and 92 verified); v2_note replace.
GP10: what replace; where 6 replaces + 1 append (cross-area note); v2_note 2 replaces. Rule 2: updateBaseHUD callers per grep are 848-851, 1561-1562, 1850, 3362 (correction listed only 1562, 1850, 3362); used the full list. Crosshair and stamina were already in where, so annotated in place rather than duplicated.
GP15: what, where, evidence, v2_note replaces (v2_note tail from 'The file manifest says' replaced).
Total 24 ops, apply-ops.py GP applies cleanly -> after-GP.json.
DONE
