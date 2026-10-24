Task: R7 legacy harvest, reader CC (core characters, factions and guns, shamble-core.js 560-845)
Checks: HEAD 698fbad = 48ffb51 + .checkpoints only. Out-of-range scope items located: applyVillain core:268, CHAR_DEFAULT 421, _BODY_HIT 462, CHAR_MALE 463, CHAR_FEMALE 508, SOLDIER_KIT/ELITE_KIT/buildSoldier game:124-131, solveArmIK game:207-223, makeSword/Axe/Shield core:1434-1472. buildCharacter does NOT exist (only comments core:9, game:1258; TECHNICAL:136,272 drift). Orphan comments: core:704-707 (soldier, now game), 794-799 (ANIM, now game:132), 813 (IK header, followed by map). In range but other areas: hyp2/hyp3 core:743-744 (CS), cabanaRepublicMap 814-844 (CM).
Finding: setEnemyType game:820 tints head material at game:3215 BEFORE stylizeScene game:3363; CHAR_MALE head mat 'skin' (core:485) shared per soldier -> whole bot skin takes unit accent (matches steps-1-6 report:59 "arm and crest colours come from the unit"). Code read only.
CC1 CPARTS core:567-625 | port TS | LBP T | C
CC2 assembleChar core:628-634 (+ editor:278 clAssembled duplicate) | port TS | T | C
CC3 FACTION_PALETTES 635-639 + applyFaction 643-653 (editor:287 only caller; coat role 0 parts) | port TS | T | M (3tc shader programs 16 vs 9)
CC4 buildEntity 654-703 (12 mats/entity, geomOf 10 types, hard AMK mount 695, userData.mats 0 readers) | port TS | T | M (~120 dc/soldier, 5,843 at 48)
CC5 base bodies CHAR_DEFAULT/MALE/FEMALE + _BODY_HIT 421-555 (overlap CL10/CL11) | port TS (hit -> Rust per CL11) | T | C
CC6 SOLDIER_KIT/ELITE_KIT/buildSoldier game:124-131 (elite dead D6) | port TS | T | C
CC7 applyVillain core:268-277 (overlap CL5; game:3364 never fires; editor:288) | drop | T | C
CC8 gun models makeAMKRifle 708-732, _gunParts 745-758, makers 759-784 | port TS | T | M (AUDIT P6)
CC9 GUN_KIT 785-791 + buildGunModel 792 (sockets) | port TS | T | M (ANIM_PLAN §6 0.0 cm)
CC10 _ARMP 800-805 + poseArms 806-812 | drop (keep armMode enum) | T | C
CC11 solveArmIK game:207-223 (9 calls pass lengths; defaults stale) | port TS | T | M (§6 0.0 cm, stretch .98, ~51 us)
CC12 melee props core:1434-1472 (axe dead, shield teal) | port TS (axe drop) | T | C
DONE
