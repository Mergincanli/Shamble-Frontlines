Task: R7 verifier spot-check, group core (CD CL CC CM CS): M rows + sampled CD14 CD13 CL5 CL2 CC10 CC12 CM14 CM1 CS4 CS3
M rows: CL4 CL6 CL7 CL8 CL9 CL12 | CC3 CC4 CC8 CC9 CC11 | CM2 CM3 CM4 CM5 CM7 CM8 | CS5 CS6 CS7 CS9 CS10. CD has none.
- CL2 (sample): sed core 185,234,243-251,267; ART_DIRECTION 50,53; TECH 43. All confirmed.
- CL4 (M): core 179,267,278-286,404,415; grep toonify (no game/editor ref); AUDIT:112 CS12 "first attempt merged nothing"; TECH 44,53 (1.56; code core:174). Confirmed.
- CL5 (sample): core 268-277 values; game 3211,3212,3364; editor 288; no elite=true anywhere; AUDIT:17; AD:72, §7. Confirmed.
- CL6 (M): core 295-303, 389-396, 387; CHAR_OUTLINE=0.8 core:242; STATUS:84; TTC.md:50 (1,480 head @12, 5,843 @48); steps-1-6:57 (5 % vs 15-25 %). Confirmed.
- CL7 (M): core 310-347, 344, 393, 744; editor 877,882; TECH 47,49,50 (33 ms, 1 ms). Confirmed.
- CL8 (M): core 397-419; game 3363; stylizeOne 25 game / 4 editor. FINDING blocking: evidence attributes draw calls 1992/2406/3502 -> 1992 and heap 119.5->44.5 to stylizeOne; report (editor-undo-ground.md:12-17) shows growth came from clearMap not clearing trees/props (missing stylizeOne = fewer hulls). FINDING should: v2_note "two real bugs ... parked bots left unstyled in 3a" - steps-1-6.md:89-92 says the reviewer's fix AS WRITTEN would have left them unstyled (near-miss caught in review, not shipped).
- CL9 (M): core 353-388; game 577,583,598,604,608; AUDIT:112 all numbers; STATUS:121 202; TECH:249 'No mesh merging yet'; vertexColors only in _matSig. Confirmed.
- CL12 (M): game 45-68, 1388,1394,1473(fireOnce),1536(onDamageEvent),1768,2959,3243; life 0.42 (game:63); 150 ms (game:65); AUDIT 23 (1.1 ms), 65 (CS1 block 61-66), 81. FINDING should: callers miss game:2761 IMPALE! and game:2911 SLASH!.
