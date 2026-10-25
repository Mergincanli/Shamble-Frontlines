editor-CC: R7 legacy harvest, editor of area CC (character composition, wardrobe, guns, IK, melee props)
Inputs: rows-and-checks.json CC (12 rows, 12 checks: 2 confirmed, 10 corrected; 5 missing; 8 not_covered). coverage.json suggested_rows: none name CC (core 138/138 covered).
Verified at HEAD f930865; git diff --stat 48ffb51 on engine files and pages is empty.
- CC1 confirmed, unchanged.
- CC2 skeptic where applied (assembleChar 628-634, clone 629; clAssembled editor:278 composes over charLabDef).
- CC3 skeptic evidence applied (steps-1-6.md:57 luminance; ART_DIRECTION :158-172, :160, :172; report :103-104).
- CC4 skeptic what applied (9 primitive types plus group); v2_note sentence replaced (report:58) and toonify sentence added; setEnemyType calls game:1842, 2939 verified by grep.
- CC5 skeptic what applied (CHAR_MALE is every gameplay soldier's body, game:127).
- CC6 skeptic evidence applied (steps-1-6.md:60-61).
- CC7 skeptic what applied (0x262636, ink 0x7B2FD6, MeshBasicMaterial visor 0xc24bff).
- CC8 skeptic v2_note sentence replaced (materials do not add draws or programs by themselves; toonify core:415).
- CC9 skeptic where applied (field reads 378-398, 1423 fallback, 1427 muzzle, 1687 weapon switch).
- CC10 skeptic mark/reason applied (port to TypeScript); v2_note made consistent with the skeptic's code evidence game:405-435, gate 420-421 (sed).
- CC11 confirmed, unchanged.
- CC12 skeptic what and v2_note append applied; WIDENED for missing system 5 (block flash): where adds core:1464, 1471, game:1539 (ps.shieldFlash=1), 2746 (decay), 1685/1689 (grep/sed verified).
- Rejected: hyp2/hyp3 (CS10), cabanaRepublicMap (CM1, TL16), applyRifleHold (GA16, ED6), orphan comments (not a system; noted in CC4/CC8/CC10/CC11). coverage.json: no suggestion names CC.
- final-CC.json written (12 rows).
DONE
