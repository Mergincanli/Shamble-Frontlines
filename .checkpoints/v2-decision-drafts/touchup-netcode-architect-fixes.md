# touchup-netcode-architect: FIX and NOTE blocks (ADR 0001, after fixcheck-d1)

Findings: F2, F1, F13 (ADR 0001 half). Docs at commit 449356c.

### FIX F2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
   changes move speed or a fire interval causes no correction; that section is sent unquantised, or the server
   quantises its own copy the same way each tick, and like every field it is sent when it changes (its real size is
   measured, R2-XU-OQ15). Never predicted: damage, deaths and hits on others; the uplink countdown and steals;
   purchases; upgrade offers and picks; weapon pickups (interim: OQ-0001-10); the player's own abilities
   until ABL-PREDICT is answered (interim: an ability press waits for the server's confirmation; ADR 0005's ability
   system carries both answers). SEC-SPREAD (a) applies: weapon spread is known only to the server (in a local match,
   the Worker's authority sim), drawn from an
AFTER:
   changes move speed or a fire interval causes no correction after the first snapshot that carries it; that section is
   sent unquantised, or the server quantises its own copy the same way each tick, and like every field it is sent when
   it changes (its real size is measured, R2-XU-OQ15). Never predicted: damage, deaths and hits on others; the uplink
   countdown and steals; purchases; upgrade offers and picks; weapon pickups (interim: OQ-0001-10); the player's own
   abilities until ABL-PREDICT is answered (interim: an ability press waits for the server's confirmation; ADR 0005's
   ability system carries both answers). SEC-SPREAD (a) applies: weapon spread is known only to the server (in a local
   match, the Worker's authority sim), drawn from an
END

