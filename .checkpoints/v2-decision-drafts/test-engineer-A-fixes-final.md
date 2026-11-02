# test-engineer unit A: final fix round (ADR 0001 and ADR 0002 pass marks), against 75615d1

### FIX new rows PM-0001-58 to -62 (TE-4, RR-3, the own-state NOTE GD-3, EA-1, NA-3): the section's row list
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
Written by test-engineer (plan step 4). One row per mark, in the order of "What a spike must prove": three
preconditions (PM-0001-1 to PM-0001-3); the 48 simulated clients within the budgets at the 30 Hz baseline (PM-0001-4
to PM-0001-27, with PM-0001-57 after PM-0001-17); the online path on four browsers (PM-0001-28 to PM-0001-38); the
60 Hz margin (PM-0001-39, PM-0001-40);
the human client at P2 (PM-0001-41 to PM-0001-53, Mustafa's checklist inside PM-0001-53); what the proof test decides
and records (PM-0001-54 to PM-0001-56). Every number is a research mark (cited), arithmetic on research numbers
AFTER:
Written by test-engineer (plan step 4). One row per mark, in the order of "What a spike must prove": three
preconditions (PM-0001-1 to PM-0001-3); the 48 simulated clients within the budgets at the 30 Hz baseline (PM-0001-4
to PM-0001-27, with PM-0001-57 and PM-0001-58 after PM-0001-17 and PM-0001-59 after PM-0001-26); the online path on four browsers (PM-0001-28 to PM-0001-38); the
60 Hz margin (PM-0001-39, PM-0001-40);
the human client at P2 (PM-0001-41 to PM-0001-53, with PM-0001-60 after PM-0001-42, PM-0001-61 after PM-0001-46 and
PM-0001-62 after PM-0001-52; Mustafa's checklist inside PM-0001-53); what the proof test decides
and records (PM-0001-54 to PM-0001-56). Every number is a research mark (cited), arithmetic on research numbers
END

### FIX EA-7 and IE-2 (the pinning exemption), IE-9/10 (LG sized by a calibration run, the per-address cap raised)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  every server time mark is the work's elapsed time on one vCPU of it with no other room on that vCPU
  ([the measuring unit](../research/GLOSSARY.md#the-measuring-unit-one-vcpu)): the room's tick thread is pinned alone to
  one vCPU and the transport runtime runs on the other three (proposed placement, so the measured vCPU holds only the
  tick). **SH**, the shared CX33 (4 vCPU, 8 GB) beside it, for the rooms-per-VM mark only (R6-HOST-REC2). **LG**, the load
  generator: a second VM in the same location running the headless bot clients, which speak the real protocol
  (r5-toolchain-quality.md, R5-TEST-REC5, lines 1475–1480); proposed: clients on RS would take CPU from the measured
  work. **ML, RD, MAC** as ADR 0003's pass marks name them (r3-rendering.md, R3-BUDGET-REC1, lines 1096–1102; the models
AFTER:
  every server time mark is the work's elapsed time on one vCPU of it with no other room on that vCPU
  ([the measuring unit](../research/GLOSSARY.md#the-measuring-unit-one-vcpu)): the room's tick thread is pinned alone to
  one vCPU and the transport runtime runs on the other three (proposed placement, so the measured vCPU holds only the
  tick), in every server time mark but the rooms-per-VM row PM-0001-27, which runs unpinned as ADR 0006's PM-0006-13
  sets it (eight tick threads cannot each be alone on one of four vCPUs). **SH**, the shared CX33 (4 vCPU, 8 GB) beside it, for the rooms-per-VM mark only (R6-HOST-REC2). **LG**, the load
  generator: one or more VMs in the same location (CX33s, or a larger plan) running the headless bot clients, which
  speak the real protocol (r5-toolchain-quality.md, R5-TEST-REC5, lines 1475–1480), never RS or SH; as many as a
  5-minute calibration run at the session's load shows keep each under 80 % of its CPU, their count in the run plan
  and their hours in the spike's bill (ADR 0006's spike item 2 and its LG key, PM-0006-18); on LG's addresses the
  per-address connection cap (part 10) is raised to the run's session count and recorded, except in PM-0001-25's run
  with the cap on (proposed: clients on RS would take CPU from the measured work; one 4-vCPU VM may not carry 384
  encrypted sessions, and the cap would refuse most sessions from one address: the infra review of 2026-11-02,
  findings 9 and 10). **ML, RD, MAC** as ADR 0003's pass marks name them (r3-rendering.md, R3-BUDGET-REC1, lines 1096–1102; the models
END

### FIX NA-2 and PA-1 (the key: ES is where the downstream mean is judged)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  run. **ES**, R2's estimate scene, reported beside the downstream mark only: 48 soldiers and 15 big vehicles all in
  view, 32 live projectiles at 0.44 spawns per player per second (worked bandwidth estimate, lines 1754–1779). **CO**,
AFTER:
  run. **ES**, R2's estimate scene, where PM-0001-12 judges the downstream mean and p99 (R2-BUD-REC2 set them on it,
  lines 2339–2341, and GLOSSARY lines 415–416 keep it apart from the stress scene): 48 soldiers and 15 big vehicles,
  every client's relevant set holding all 47 other soldiers and all 15 vehicles on at least 99 % of its snapshots
  (the estimate assumes every entity in view, lines 1765–1766; the share is PM-0001-2's, proposed), 1.5 shots per
  player per second (72 a second), 10 reliable gameplay events a second, 32 live projectiles at 0.44 spawns per player
  per second (worked bandwidth estimate, lines 1754–1779; rates from lines 1759–1761), with a snapshot-receiving client
  on every soldier; PM-0001-2's counter script checks these counts on every ES run (the rates within ±5 %, proposed). **CO**,
END

### FIX NA-5 and NA Q2 (the key: the profiles' direction split, OQ-0001-16)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  Latency is the round trip. L1 runs the headless clients; L2 is the impairment layer under quinn's socket (and a
AFTER:
  Latency is the round trip, and each direction carries half the profile's latency and a uniform jitter of ± half its
  jitter, so the round trip keeps the profile's mean and range (part 13; OQ-0001-16, the Advisor's, before the test
  starts; the half split of latency is R5-TEST-REC4's, r5-toolchain-quality.md lines 1455–1456, proposed there, and
  the jitter's split and its uniform shape are part 13's proposal): at P2, 50 ± 10 ms each way. A packet's expected
  arrival is its send time plus that one-way delay, and every lateness in these marks is counted from it. L1 runs the headless clients; L2 is the impairment layer under quinn's socket (and a
END

