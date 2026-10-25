checker-GC (R7 legacy harvest, checker for area GC fixes2 ops)
started
GC3,GC7 diff checked: text matches corrections
GC3: diff OK; problem: syncSimWorld call is :1125 not :1124 (1124 is the comment)
GC7: diff OK, claims hold (1509 DEV.god, 1552 hsMult, 1553 flat, 1561 no gameOver)
GC8: diff OK; :1536 onKill, :1546 playerDie, :2096-2102 onKill, :2963 dmgDirT fade, :68 Math.random confirmed
GC13: diff OK; problem: Molotov has no arc preview (throwMolotov :1405-1411 is called from fire at :1469, preview only via tickCharge :1729/:1733)
GC14: diff OK; :1409 fuse, :2859 1.8, :2881/:2883 1.5, :2456, :1846, :2543, :552 confirmed
GC16: diff OK; JUG reads 1255/1279/1345, 1303 in _jugFirePilot (1299), 1343/1354 typeof guards, 1368 palette build confirmed
GC18: diff OK; lastAttackerId 1519/1842/3213 no reads, 2477 em dash confirmed
GC19: diff OK; 1158 applyUnit cap, core 77 regen, core 128/131/134/137 maxHealth 100/150/120/110 confirmed
Only targeted fields changed (field-by-field diff of final-GC vs after-GC); apply-ops rc=0.
Problems: (1) GC3 what syncSimWorld (:1124) -> :1125. (2) GC13 v2_note Molotovs have no preview.
DONE
