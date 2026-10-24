Task: R7 legacy inventory, reader CS (core headless SIM math, shamble-core.js 1474-1543)
## Checks
- HEAD 698fbad = 48ffb51 + .checkpoints only. Block core:1474-1544 (hasLOS at 1544, just past 1543). hyp2/hyp3 core:743-744. VISION_V2 absent at 48ffb51; read /home/user/Shamble-Frontlines/docs/VISION_V2.md §6.
- FIXED_DT/BTN/makeInputCommand: no reader anywhere (repo grep); loop variable dt clamp 0.05 game:3053.
- PART + res.mult: no reader; damage uses T.headMult/bodyMult/legMult core:88 at game:1552 (per-weapon headMult).
- BODY = CHAR_DEFAULT.hit core:455; crouchDrop dead: syncSimWorld sets stance=0 game:1447-1448.
- rayStatic single: grenade-arc preview game:1742 (START_HERE says 1516: drift). hasLOS editor:574 (START_HERE says 565).
- resolveShot callers game:1126,1309,1489,1625,2869. teamFilter game:519.
- rayCapsule: origin inside cylinder -> returns -1 (raySphere returns 0) code read, not measured.
## Evidence: team-prep-fixes report :56-58,64-65,95-98,126; STATUS:90,117,118,68; AUDIT:25,71,109,110,111,123; TECHNICAL:87-91,123
## Rows
CS1 FIXED_DT/BTN/makeInputCommand core:1479-1481 C drop
CS2 BODY hit shape core:1482,455 C rust
CS3 PART/res.mult core:1483,1509 C drop
CS4 raySphere/rayCapsule core:1484-1485 C rust
CS5 rayAabb+rayStatic core:1486-1491,1517-1525 M rust
CS6 rayStatics core:1526-1543 M rust
CS7 resolveShot core:1491-1510 M rust
CS8 SIM world core:1516 C rust
CS9 hasLOS core:1544 M rust
CS10 hyp2/hyp3 core:743-744 M drop
DONE
