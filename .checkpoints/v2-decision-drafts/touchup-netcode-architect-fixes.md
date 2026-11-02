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

### FIX F1
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  and the server-only load in the same fight, and the margin is a tail statistic (our inference). The 30 Hz baseline
  marks may use a stand-in (with S1's stand-ins at 30 Hz: 384 effect rows and a 1 ms burn, the 2.80 ms remainder
  topped up): on the planning check they leave 7.49 ms. · *Decider:* the Advisor. · *What decides it:* the Advisor's
  reading of the options and the sim core spike's readiness. · *Research ids:* R2-BUD-REC6, R1-TICK-REC2.
AFTER:
  and the server-only load in the same fight, and the margin is a tail statistic (our inference). The 30 Hz baseline
  marks may use a stand-in (with S1's stand-ins at 30 Hz: 384 effect rows and a 1 ms burn, the 2.80 ms remainder topped
  up): on the planning check they leave 4.69 ms (16.67 − 9.18 − 2.80). · *Decider:* the Advisor. · *What decides it:*
  the Advisor's reading of the options and the sim core spike's readiness. · *Research ids:* R2-BUD-REC6, R1-TICK-REC2.
END

### FIX F13
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  vector, buttons, press counters and view time, about 15–20 B in all (the perf review's estimate, 2026-11-02; the
  real size is measured, R2-XU-OQ4). With the newest-received acknowledgement (part 5), a datagram carries the
  commands of one round trip plus up to two ticks of waiting (the snapshot's and the client's): 80–186.7 ms at P2's
  100 ± 20 ms, so 3 to 6 commands ((80 to 186.7) ÷ 33.3, rounded up; arithmetic). Sent whole (U1), with a header of
  about 6 B (an assumption) and the 57 B of IPv4 transport overhead (r2-netcode.md, line 1742), at 30 datagrams a
  second (240 bits a second per byte of datagram), that is (3 × 15 + 63) × 240 ≈ 25.9 to (6 × 20 + 63) × 240 ≈
  43.9 kbps at P2, and at the cap of 8, reached at P3 and P4 or after a loss, (8 × 15 + 63) × 240 ≈ 43.9 to
  (8 × 20 + 63) × 240 ≈ 53.5 kbps, against a mark of at most 48 kbps (IPv4: R2-BUD-REC1), 200 B per datagram
AFTER:
  vector, buttons, press counters and view time, about 15–20 B in all (proposed: the perf review's estimate, 2026-11-02;
  the real size is measured, R2-XU-OQ4). With the newest-received acknowledgement (part 5), a datagram carries the
  commands of one round trip plus up to two ticks of waiting (the snapshot's and the client's): 80–186.7 ms at P2's
  100 ± 20 ms, so 3 to 6 commands ((80 to 186.7) ÷ 33.3, rounded up; arithmetic). Sent whole (U1), with a header of
  about 6 B (proposed: the same estimate) and the 57 B of IPv4 transport overhead (r2-netcode.md, line 1742), at
  30 datagrams a second (240 bits a second per byte of datagram), that is (3 × 15 + 63) × 240 ≈ 25.9 to
  (6 × 20 + 63) × 240 ≈ 43.9 kbps at P2, and at the cap of 8, reached at P3 and P4 or after a loss, (8 × 15 + 63) × 240
  ≈ 43.9 to (8 × 20 + 63) × 240 ≈ 53.5 kbps, against a mark of at most 48 kbps (IPv4: R2-BUD-REC1), 200 B per datagram
END

