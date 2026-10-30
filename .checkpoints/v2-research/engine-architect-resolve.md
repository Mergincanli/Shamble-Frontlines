Task: step 3a of plan-amendments-2 (glossary GLOSSARY.md + resolutions.md for the 12 contradictions + cross-topic-oqs.md), engine-architect, 2026-10-30.

## Progress
- started; reading plan-amendments-2.md and advisor-amendments-2-verdict.md
- read: plan-amendments-2.md (whole), advisor-amendments-2-verdict.md (whole; decider table), critic JSON (41 items; other29 copied to scratchpad), my consistency notes, VISION_V2 §3/§4/§6.
- read topic ranges: r1:1870-2420 (RECs), r1:2531-2970 (OQs, XU, RR, pointers); r2:1669-1787 (budget options, worked estimate), r2:1975-2845 (RECs), r2:2853-3346 (OQs, XU, pointers); r3:1066-1195, 1355-1381; r4:650-985, 1105-1135; r5:1205-1235, 1299-1340, 1460-1484, 1577-1670, 1766-1960, 822-900 (PREV F10-F20); r6:1-205, 397-702, 814-970.
- Key facts for resolutions (all from files):
  - R6-HOST-F23 quotes AWS "c6a.large (1 core, 2 vCPU, 4 GB)": a vCPU is not always a core. R6 r6:942-943 maps R2's unit to one vCPU of CCX23; r6:453 reads R1-ECS-OQ4's "one per core" as one per vCPU.
  - Budget arithmetic: encode 48 x 50 us = 2.4 ms/snapshot (R2-BUD-REC6, r2:1724); lag comp 700 shots/s x 20 us = 14 ms/s -> 0.23 ms/tick at 60 Hz, 0.47 at 30 (R2-SYNC-REC5 m4); interest 0.2 + 0.5 ms (R2-INT-REC1, REC4); abilities 1 ms (R1-ABL-REC6); movement+physics 1 ms mean (R4-PHYS-REC1 m2).
  - Under B25 (whole tick 25%) the parts do not fit at 60 Hz; under B50 (R2-ROOM-REC2, sim 25% R1 + server-only 25% R2-SYNC-REC9 read against the period) they fit at 60 and 30 Hz.
  - Previews: Pages sourced on noindex (R5-PREV-F12), hostname rules (F18), access policy (F12), deletion except latest (F13), Free plan limits (R6-HOST-F3); Workers sourced on free static (R6-HOST-F5), Access + 1000 aliases (R5-PREV-F14), _headers (R6-HOST-F2); Workers noindex / deletion / hostname rules unread.
  - R5 file line shift: critic cited r5:1609-1610 for PREV-REC4; current file has it at r5:1615-1621.
- Decisions taken (proposals, for the advisor 3b): #1 recommend B50 (one budget 50%: sim 25% + server-only 25%); #9 stress scene = stress N in three teams, 15 big vehicles (4 very large units on screen), 300 projectiles, 8 active effects per soldier; #11 one vCPU of CCX23; #19 Pages primary + PREV-DELETE spike; #22 production header set by default, isolation flag; #29 type ban stays, allow-with-reason; #6 authority-only streams; #5 aligned ABL-PREDICT.
- unit 0: GLOSSARY.md skeleton and resolutions.md skeleton written
- unit G2: glossary §2 (6 terms: player count, team size, start/goal modes, largest mode, design point/headroom, test-mode list) written
- unit #10: resolution 10 written (escalated MODES-3; 23 changes R10-1..R10-23)
