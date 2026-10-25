R7 legacy harvest checker for area CL (fixes3)
Ran apply-ops.py CL: 9 ops (4 from fixes2 on CL14, 5 fixes3) applied, 8 fields of 4 rows.
- verifier-core#0 accepted (CL8.evidence set): mostly right; report:12-15, :27, :29, :30, STATUS.md:124, steps-1-6:88-95, 5695e56 all hold. PROBLEM (minor): "each rebuild stacked another 70 trees and 36 props, :18, :26" contradicts :26 (70/36 -> 140/72 -> 280/144: copies double), and the 70/36 text is at :19 not :18. Fix op: replace that phrase.
- verifier-core#1 accepted (CL8.v2_note replace): correct, report:88-91 says "would have left"; 3a in report title :1; PR #8 = b5f4ecf. OK.
- verifier-core#2 accepted (CL12.where + what): :2761 IMPALE! and :2911 SLASH! hold; 8 spawnComic( in game, all covered. PROBLEM (minor): SLASH! only spawns when the lunging bot's target is the player (:2911 `if(en.targetId===SIM_PLAYER)`), so "bot lunges (SLASH!)" is too broad. Fix ops on what and where.
- graphics-engineer#3 accepted (CL10.what replace): game:127 default 'm', callers :683 :796, core:628-629, CHAR_FEMALE only in editor:446-450. OK.
DONE
