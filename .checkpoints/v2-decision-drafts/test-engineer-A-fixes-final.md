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

### FIX PA-9 (the re-flood's tick parity at 60 Hz), TE-4 (the soak's wrap is a second check)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **Soaks.** **S30**: all 48 clients join, a 60 s warm-up starts once the last client has acknowledged its first full
  state, then 30 minutes are measured: 54,000 ticks at 30 Hz (proposed warm-up: joins and first full states are not the
  steady state). **S60**: the same at a 60 Hz tick, 108,000 measured ticks. A p99 reads the measured ticks; a max, an
  overrun count and a dropped-time count read the whole run, joins included (proposed: a real match has its join burst
  too). The run's tick counter starts 1,800 ticks (1 minute at 30 Hz) below 65,536, so every S30 crosses a 16-bit wrap
  (proposed: the draft's part 6 (the snapshot header's tick) and its "Failure modes" row "A tick counter wraps"
  expect it only at 60 Hz). Portal changes are scripted at one
  per second (doors opening and closing near the fight), which puts re-flood ticks on 3.3 % of ticks at 30 Hz and 1.7 %
  at 60 Hz (1 ÷ 30, 1 ÷ 60), above the 1 % tail a p99 reads (proposed: "re-flood ticks included" names no rate).
AFTER:
- **Soaks.** **S30**: all 48 clients join, a 60 s warm-up starts once the last client has acknowledged its first full
  state, then 30 minutes are measured: 54,000 ticks at 30 Hz (proposed warm-up: joins and first full states are not the
  steady state). **S60**: the same at a 60 Hz tick, 108,000 measured ticks. A p99 reads the measured ticks; a max, an
  overrun count and a dropped-time count read the whole run, joins included (proposed: a real match has its join burst
  too). The run's tick counter starts 1,800 ticks (1 minute at 30 Hz) below 65,536, so every S30 crosses a 16-bit wrap
  (proposed: a second check beside PM-0001-58's deterministic wrap test, which the "Failure modes" row "A tick or
  sequence counter wraps" names; from 0, the 54,000 ticks of an S30 never reach 65,536). Portal changes are scripted at one
  per second (doors opening and closing near the fight), which puts re-flood ticks on 3.3 % of ticks at 30 Hz and 1.7 %
  at 60 Hz (1 ÷ 30, 1 ÷ 60), above the 1 % tail a p99 reads (proposed: "re-flood ticks included" names no rate). At
  60 Hz a snapshot is encoded on every second tick (part 2), and changes 60 ticks apart all land on one parity, all on
  encoding ticks or none (the perf review of 2026-11-02, finding 9), so the door script is phase-locked to the encoder
  and every re-flood tick also encodes: the tight case of part 3's restated reason, 8.28 of 8.33 ms on the planning
  check (GLOSSARY.md:62–75) (proposed: the case a 60 Hz pass must cover). Each re-flood's tick and whether it encoded
  are logged, and the share is reported.
END

### FIX EA-7 and IE-2 (V-S: the rooms-per-VM row exempt from the pinning and steal clauses), IE-9/10 (LG's own CPU)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **Runs and validity (V-S).** A timed server mark runs 3 times and every run must meet it (R1-TICK-REC2 P1's "3 runs",
  r1-simulation-core.md line 2047; proposed for every timed server mark). A run is valid when: the tick thread was alone
  on its vCPU (PM-0001-1); the vCPU's steal time stayed at or under 1 % of the run (proposed: a VM's neighbours show as
  steal; not researched); LG's clients sent their 30 inputs per second each within 1 % (proposed: a starved load
  generator lightens the load). A mark compared with "a clean run" uses the median of 3 clean runs whose p99s lie within
  2.5 % of their median (proposed: half the tightest tolerance compared against, R2-TRN-REC6 mark 2's 5 %, so the
  comparison can tell a change from noise). An invalid run is neither a pass nor a fail: its cause is found and written
  down before it is measured again.
AFTER:
- **Runs and validity (V-S).** A timed server mark runs 3 times and every run must meet it (R1-TICK-REC2 P1's "3 runs",
  r1-simulation-core.md line 2047; proposed for every timed server mark). A run is valid when: the tick thread was alone
  on its vCPU (PM-0001-1); the vCPU's steal time stayed at or under 1 % of the run (proposed: a VM's neighbours show as
  steal; not researched); LG's clients sent their 30 inputs per second each within 1 %, and each LG VM's CPU use stayed
  under 80 % in every second (ADR 0006's LG key) (proposed: a starved load generator lightens the load). The first two
  clauses hold for single-room marks only: the rooms-per-VM row PM-0001-27 runs under ADR 0006's PM-0006-13 rules
  instead (no pinning, its bracket B, 3 sessions at 3 times of day, steal recorded per second and reported, never a
  validity condition), since eight tick threads cannot each be alone on four vCPUs and on SH steal is what is measured
  ("repeated at 3 times of day (shared hosts vary with neighbours)": r6-infrastructure.md, R6-HOST-REC2's mark, lines
  668–669). A mark compared with "a clean run" uses the median of 3 clean runs whose p99s lie within
  2.5 % of their median (proposed: half the tightest tolerance compared against, R2-TRN-REC6 mark 2's 5 %, so the
  comparison can tell a change from noise). An invalid run is neither a pass nor a fail: its cause is found and written
  down before it is measured again.
END

### FIX EA-7 and IE-2 (PM-0001-1: the rooms-per-VM row runs unpinned)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-1 | The rig is named and the measuring unit holds (part 14; a precondition of every server mark) | RS, SH and LG named with plan, CPU model, vCPU count, memory, location and kernel; ML, RD and MAC as ADR 0003 names them; the tick thread's CPU affinity is one vCPU that no other thread of any process uses; steal time recorded per second | RS, SH, LG, ML, RD, MAC | a results-log header per machine, written before the first run; the tick thread's affinity and the other threads' affinities read from the operating system at the start and end of every run | checked before the first run and in every run's log. Bites: a run with a second room's tick thread on the same vCPU is flagged invalid | GLOSSARY, the reference server (lines 225–243) and the measuring unit (lines 244–264); r6 R6-HOST-REC2 (lines 645–650); r3 R3-BUDGET-REC1 (lines 1096–1102) |
AFTER:
| PM-0001-1 | The rig is named and the measuring unit holds (part 14; a precondition of every server mark) | RS, SH and LG named with plan, CPU model, vCPU count, memory, location and kernel; ML, RD and MAC as ADR 0003 names them; the tick thread's CPU affinity is one vCPU that no other thread of any process uses, in every run but PM-0001-27's, whose unpinned rooms record each process's affinity instead (ADR 0006's PM-0006-13); steal time recorded per second | RS, SH, LG, ML, RD, MAC | a results-log header per machine, written before the first run; the tick thread's affinity and the other threads' affinities read from the operating system at the start and end of every run | checked before the first run and in every run's log. Bites: a run with a second room's tick thread on the same vCPU is flagged invalid | GLOSSARY, the reference server (lines 225–243) and the measuring unit (lines 244–264); r6 R6-HOST-REC2 (lines 645–650); r3 R3-BUDGET-REC1 (lines 1096–1102) |
END

### FIX PA-9 and the TE-1 follow-up (PM-0001-2: the door phase at 60 Hz, OQ-0001-2 (a)'s stand-ins), NA-2 (ES's counts)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-2 | The load is the stress scene (parts 3, 14; the setup of the 60 Hz margin) | on every measured tick: 48 soldiers in three teams of 16, each with a connected snapshot-receiving client; 15 big vehicles, 4 of them very large units of at least 20 parts; 300 live projectiles; up to 8 active effects per soldier where the sim has effects; hitscan fire at 700 shots per second (run mean within ±5 %, proposed); portal changes at 1 per second; for the downstream rows, every client's relevant set holds all 47 other soldiers on at least 99 % of its snapshots ("everyone in view"; proposed share) | RS (counters); LG | the server's own per-tick counters written to the results log and checked by script. A sim that lacks vehicles, projectiles or effects adds stand-in entities that are replicated, hit-tested and filtered by interest at the same counts, their change and spawn rates recorded (proposed: the server-only share must carry the stress scene's load, or its marks cannot fail where the real sim's would) | every run of PM-0001-4 to PM-0001-27 and PM-0001-39. Bites: a load script that stops firing after 10 minutes fails the 700 shots per second | GLOSSARY §3, the stress scene (lines 397–437); R2-BUD-REC6 (line 2412, "a snapshot-receiving client on every soldier"); R2-SYNC-REC5 mark 4 (lines 2186–2187) and R2-SYNC-REC9 (line 2280): 700 shots per second |
AFTER:
| PM-0001-2 | The load is the stress scene (parts 3, 14; the setup of the 60 Hz margin) | on every measured tick: 48 soldiers in three teams of 16, each with a connected snapshot-receiving client; 15 big vehicles, 4 of them very large units of at least 20 parts; 300 live projectiles; up to 8 active effects per soldier where the sim has effects; hitscan fire at 700 shots per second (run mean within ±5 %, proposed); portal changes at 1 per second (at 60 Hz on encoding ticks: the key's "Soaks"); for the downstream rows, every client's relevant set holds all 47 other soldiers on at least 99 % of its snapshots ("everyone in view"; proposed share) | RS (counters); LG | the server's own per-tick counters written to the results log and checked by script. A sim that lacks vehicles, projectiles or effects adds stand-in entities that are replicated, hit-tested and filtered by interest at the same counts, their change and spawn rates recorded (proposed: the server-only share must carry the stress scene's load, or its marks cannot fail where the real sim's would); the sim step share's missing parts are stood in as OQ-0001-2 (a) stands them in (ADR 0004's S1: 384 effect rows with the effect step's 1 ms ceiling burned per tick, and the remainder topped up to 2.80 ms at 30 Hz and 0.30 ms at 60 Hz), each stand-in named in the run's record. ES's counts and rates (the key) are checked the same way on every ES run | every run of PM-0001-4 to PM-0001-27 and PM-0001-39. Bites: a load script that stops firing after 10 minutes fails the 700 shots per second | GLOSSARY §3, the stress scene (lines 397–437); R2-BUD-REC6 (line 2412, "a snapshot-receiving client on every soldier"); R2-SYNC-REC5 mark 4 (lines 2186–2187) and R2-SYNC-REC9 (line 2280): 700 shots per second |
END

### FIX NA-5 and NA Q2 (PM-0001-3: the profiles checked per direction)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-3 | The network profiles are what they say (part 13) | per profile P1 to P4 and B1, on L1 and on L2: measured loss, mean delay and jitter over 100,000 packets within 5 % (relative) of the profile; on L1, the same seed and profile give an identical packet-fate log on two runs; P0 on L2 is the real path, its delay and jitter recorded; for MP's session, the server-measured RTT median within 100 ± 5 ms at P2, his own base RTT recorded | L1; L2 on RS with LG, ML and MP | the simulator's packet-fate log against the profile; on L2, the injected delay set to the profile's RTT minus the path's measured base RTT (median of 10 s of pings at session start; proposed) | once per profile and layer before any mark under that profile counts, and at the start of every L2 session | r5 R5-TEST-REC4's mark (lines 1469–1473; ADR 0002's simulator, applied here to L2: proposed); r2 profiles (lines 1987–1992); R2-TRN-REC4 mark 2's ±5 ms (line 2694) for MP's RTT (proposed) |
AFTER:
| PM-0001-3 | The network profiles are what they say (part 13) | per profile P1 to P4 and B1, on L1 and on L2, in each direction: measured loss, mean one-way delay and jitter over 100,000 packets within 5 % (relative) of the profile's per-direction values (half its latency, a uniform jitter of ± half its jitter: the key's split, OQ-0001-16), and the round trip's mean within 5 % of the profile's latency; on L1, the same seed and profile give an identical packet-fate log on two runs; P0 on L2 is the real path, its delay and jitter recorded; for MP's session, the server-measured RTT median within 100 ± 5 ms at P2, his own base RTT recorded | L1; L2 on RS with LG, ML and MP | the simulator's packet-fate log against the profile; on L2, the injected delay in each direction set to half of the profile's RTT minus the path's measured base RTT (median of 10 s of pings at session start; proposed), each direction's delays logged at the impairment layer | once per profile and layer before any mark under that profile counts, and at the start of every L2 session | r5 R5-TEST-REC4's mark (lines 1469–1473; ADR 0002's simulator, applied here to L2: proposed); r2 profiles (lines 1987–1992); R2-TRN-REC4 mark 2's ±5 ms (line 2694) for MP's RTT (proposed) |
END

### FIX NA-2 and PA-1 (PM-0001-12: the 160 kbps mean judged in ES, the cap in SS)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-12 | The downstream fits per client (parts 6, 9) | mean at most 160 kbps; p99 of 1-second windows at most 256 kbps; every 1-second window at most 256 kbps (the hard cap; proposed reading of "hard cap"); 0 datagrams above the cap rule; counted per client at the server's send side, UDP payload plus 28 B per datagram. Reported beside it: the same counts in ES (the scene the research mark was set on) and, if LG reaches RS over IPv6, the IPv6 figure | RS (send-side counters) | SS at P2, everyone in view (PM-0001-2), PM-0001-4's runs; ES for 10 minutes (R2-BUD-REC2's own length) | as PM-0001-4; rule B-N | r2 R2-BUD-REC2 (lines 2339–2344); the draft, the budgets table ("Budgets for the netcode proof test"), row "Downstream per client", and part 6's per-client hard cap; worked estimate (lines 1754–1779, 1784: +4.8 kbps for IPv6) |
AFTER:
| PM-0001-12 | The downstream fits per client (parts 6, 9) | in ES, the scene R2-BUD-REC2 set its mark on: mean at most 160 kbps and p99 of 1-second windows at most 256 kbps; in SS, the load the cap must hold: every 1-second window at most 256 kbps (the hard cap; proposed reading of "hard cap") and 0 datagrams above the cap rule, its mean reported beside R2's formula evaluated at the run's own counts (entities in each client's set, shots, spawns and events per second: lines 1764–1766), as ADR 0006's PM-0006-17 evaluates it for egress (SS's 700 shots a second alone add about 20.1 kbps to R2's 146.1: the draft's bandwidth estimate); both counted per client at the server's send side, UDP payload plus 28 B per datagram. Reported beside it: if LG reaches RS over IPv6, the IPv6 figure | RS (send-side counters) | ES at P2 for 10 minutes (R2-BUD-REC2's own length), 3 runs (proposed, as PM-0001-4); SS at P2, everyone in view (PM-0001-2), PM-0001-4's runs | as PM-0001-4, in each scene; rule B-N in each | r2 R2-BUD-REC2 (lines 2339–2344); the draft, the budgets table ("Budgets for the netcode proof test"), row "Downstream per client", and part 6's per-client hard cap; worked estimate (lines 1754–1779, 1784: +4.8 kbps for IPv6); GLOSSARY lines 415–416 ("R2's worked bandwidth estimate is not the stress scene"); the netcode and perf reviews of 2026-11-02 (findings 2 and 1) |
END

### FIX PA-3 and NA-9 as routed (PM-0001-13: both command forms and the commands per datagram recorded)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-13 | The upstream fits per client (part 5; measures R2-XU-OQ4) | mean at most 48 kbps per client (IPv4), counted at the UDP payload plus 28 B per datagram (the mean: proposed reading); reported: the p99 of 1-second windows, the largest input datagram and the real command size | RS (receive-side counters) | P2, PM-0001-4's runs; every input datagram's size logged per client | as PM-0001-4; rule B-N | r2 R2-BUD-REC1 (lines 2320–2325), restated against P2 (part 13); the draft, "Bandwidth estimate", the bullet "Upstream per client" |
AFTER:
| PM-0001-13 | The upstream fits per client (part 5; measures R2-XU-OQ4) | mean at most 48 kbps per client (IPv4), counted at the UDP payload plus 28 B per datagram (the mean: proposed reading); reported: the p99 of 1-second windows, the largest input datagram, the commands per datagram (median and max; 3 to 6 at P2 by the draft's bandwidth estimate) and the real command size, whole and delta-coded (U1 against U2, part 5), with the same counts at P4, where the redundancy cap of 8 is reached | RS (receive-side counters) | P2, PM-0001-4's runs; P4 for 10 minutes for the report; every input datagram's size and command count logged per client, and each command's size whole and as sent | as PM-0001-4; rule B-N | r2 R2-BUD-REC1 (lines 2320–2325), restated against P2 (part 13); the draft, "Bandwidth estimate", the bullet "Upstream per client" |
END

### FIX TE-4 as routed (the new row PM-0001-58: counter wraps from just below their wrap points)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-57 | Decoded values keep the protocol's precision (part 6; R2-BUD-REC2's precision mark as Appendix A restates it) | over every entity value a client decodes in a run, the max error against the server's value before quantising: positions at most 1 cm; yaw at most 0.01°; pitch at most one 0.18° step (the restated mark, proposed there: the 10-bit field's own resolution) | L1 (48 headless clients) | PM-0001-17's L1 runs: the server logs each sent entity's position, yaw and pitch before quantising, the client logs the values it decodes, and a script takes the max error per field (proposed: PM-0001-17's hash proves the decoded values equal the server's quantised ones, not how far quantising moved them) | as PM-0001-17's L1: 3 seeds per profile under P0, P2, P3 and B1; each meets the mark. Bites, one per field: positions quantised to 4 cm (errors up to 2 cm); yaw in 12 bits (0.088° steps); pitch in 8 bits (0.70° steps) | R2-BUD-REC2 (line 2341, "decoded positions within 1 cm and angles within 0.01°"); the 10-bit pitch (line 1748); the 16-bit yaw step, about 0.0055° (line 2263); the draft, part 6 and Appendix A |
AFTER:
| PM-0001-57 | Decoded values keep the protocol's precision (part 6; R2-BUD-REC2's precision mark as Appendix A restates it) | over every entity value a client decodes in a run, the max error against the server's value before quantising: positions at most 1 cm; yaw at most 0.01°; pitch at most one 0.18° step (the restated mark, proposed there: the 10-bit field's own resolution) | L1 (48 headless clients) | PM-0001-17's L1 runs: the server logs each sent entity's position, yaw and pitch before quantising, the client logs the values it decodes, and a script takes the max error per field (proposed: PM-0001-17's hash proves the decoded values equal the server's quantised ones, not how far quantising moved them) | as PM-0001-17's L1: 3 seeds per profile under P0, P2, P3 and B1; each meets the mark. Bites, one per field: positions quantised to 4 cm (errors up to 2 cm); yaw in 12 bits (0.088° steps); pitch in 8 bits (0.70° steps) | R2-BUD-REC2 (line 2341, "decoded positions within 1 cm and angles within 0.01°"); the 10-bit pitch (line 1748); the 16-bit yaw step, about 0.0055° (line 2263); the draft, part 6 and Appendix A |
| PM-0001-58 | Every counter survives its wrap (parts 5, 6; the failure mode "A tick or sequence counter wraps") | across every wrap: 100 % of decoded snapshots rebuild the server's quantised state by [the client-held state's hash](../research/GLOSSARY.md#state-hash) on every tick (PM-0001-17's check); 0 commands refused, lost or doubled and 0 presses lost or doubled (PM-0001-20's counts); 0 corrections larger than 1 mm (PM-0001-41's count) | L1 (48 headless clients): the proof test's harness, then the gate's network step (G11) from the foundation | a deterministic run that starts the server tick 30 ticks below a multiple of 65,536 and every 16-bit or narrower counter the protocol carries (command sequence numbers, press counters, the snapshot's tick offsets, event and acknowledgement numbers) a few steps below its own wrap (proposed: within the run's first 2 s, as the draft's test plan asks), then runs 10 s; at P0 and at P3 (so reordering crosses the wrap) | 3 seeds per profile; 0 mismatches in each. Bites, one per counter: the counter compared with a plain `<` instead of its wrap-aware comparison; the header tick narrowed to 16 bits (PM-0001-17's bite, caught here within seconds) | the draft, "Failure modes", row "A tick or sequence counter wraps", and the test plan's "Counter wraps" (proposed there: the 30 Hz soak's 54,000 ticks never reach 65,536 from 0); part 5 (sequence numbers only move forward, wrap-around handled); part 6 (the 32-bit header tick, 16-bit offsets); the test review of 2026-11-02, finding 4 |
END

### FIX NA-1 as routed (PM-0001-20: the buffer depth that feeds the rewind recorded)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-20 | Commands arrive in time and count once (part 5) | under P3: at least 99.9 % of commands reach the server before their tick runs; missing inputs at most 0.1 % of player-ticks; 10,000 scripted presses with reordering: 0 lost and 0 doubled | L1; L2 with LG | 48 clients, 10 minutes; the server logs each command's arrival against its tick and each press counter's edges | L1: 3 seeds; L2: 1 run. Bites: a redundancy cap of 1 (no repeats) fails the 99.9 % | r2 R2-SYNC-REC2 marks 1–2 (lines 2058–2059); R2-BUD-REC1 (lines 2323–2324: 5 % random loss, restated as P3) |
AFTER:
| PM-0001-20 | Commands arrive in time and count once (part 5) | under P3: at least 99.9 % of commands reach the server before their tick runs; missing inputs at most 0.1 % of player-ticks; 10,000 scripted presses with reordering: 0 lost and 0 doubled; recorded: the input buffer depth the server settles on per client (median and max, in ticks), since each buffered tick adds 33.3 ms to every honest rewind (part 8) and PM-0001-49 reads it | L1; L2 with LG | 48 clients, 10 minutes, under each of OQ-0001-6's two command-timing options (PM-0001-56); the server logs each command's arrival against its tick, each press counter's edges and each client's buffer depth every second | L1: 3 seeds; L2: 1 run. Bites: a redundancy cap of 1 (no repeats) fails the 99.9 % | r2 R2-SYNC-REC2 marks 1–2 (lines 2058–2059); R2-BUD-REC1 (lines 2323–2324: 5 % random loss, restated as P3) |
END

### FIX GD-1, SA-8 (PM-0001-21: the aim point clamped, not forged; strictly newer ticks; a fast-clock client)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-21 | Forged commands and lag switches change nothing (parts 1, 5) | every forged command of R2-SYNC-REC2 mark 3's set rejected and counted, with 0 effect beyond the command's valid part (the state hash of a run with the forged set equals that of a run with only their valid parts); a lag-switch client (inputs held 200 ms under P1, then sent in one burst) moves no further in any tick than the catch-up cap allows | L1 | 1,000 forged commands per kind (proposed); 100 lag-switch bursts (proposed) | 3 seeds; 0 accepted in each. Bites: each check removed in turn lets its forged kind through | r2 R2-SYNC-REC2 marks 3–4 (lines 2060–2066); the draft, part 5 (the server's checks on every command) |
AFTER:
| PM-0001-21 | Forged commands and lag switches change nothing (parts 1, 5) | every forged command of R2-SYNC-REC2 mark 3's set as part 5 changes it (an aim point off the map stays in it; one beyond the weapon's reach leaves it, and a tick equal to or older than the last command run joins it) rejected and counted, with 0 effect beyond the command's valid part (the state hash of a run with the forged set equals that of a run with only their valid parts); an aim point beyond the weapon's reach clamped, never refused or counted (the state hash of a run with such points equals that of a run with each point moved along its ray to the reach); a client whose clock runs 1.5 times fast for 60 s runs at most the server's ticks in that time plus the lead bound of commands, and moves no further than an honest client on the same inputs plus that many ticks of movement; a lag-switch client (inputs held 200 ms under P1, then sent in one burst) moves no further in any tick than the catch-up cap allows | L1 | 1,000 forged commands per kind (proposed); 1,000 aim points beyond reach per weapon kind the sim has (hitscan, melee, throw where built; proposed); 100 lag-switch bursts (proposed); 10 fast-clock runs of 60 s (proposed), under each of OQ-0001-6's two options | 3 seeds; 0 accepted in each. Bites: each check removed in turn lets its forged kind through; the tick check relaxed to "not older than" lets the fast clock run the per-tick cap on every tick under OQ-0001-6 (a); a refusal in place of the clamp counts honest far clicks as forged | r2 R2-SYNC-REC2 marks 3–4 (lines 2060–2066), changed by part 5 (the clamp; strictly newer ticks; Appendix A's row); the draft, part 5 (the server's checks on every command); the game-design and security reviews of 2026-11-02 (findings 1 and 8) |
END

### FIX SA-9, IE-9/10, SA-7 as routed (PM-0001-25: the deadline from the accept, the cap off and on, the /64)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-25 | Hostile input is contained on both transports (part 10) | a flood (1,000 sessions that never send `Join`, plus 10 clients sending oversize datagrams at full rate and opening streams) leaves server tick time p99 within 10 % of a clean run and resident memory under 64 MB; the same suite over WebSocket, plus oversized frames and messages, with the same marks; a `ws://` connection and a wrong Origin refused (403) from all four browsers; a session without `Join` after 2 s closed | RS with LG (flood); ML and MAC (Origin) | S30's room under each flood for 10 minutes (proposed) | 1 run per transport; V-S's clean reference. Bites: the library's default limits restored | r2 R2-TRN-REC1 mark 4 (lines 2597–2609); R2-TRN-REC3 mark 4 (lines 2670–2671) |
AFTER:
| PM-0001-25 | Hostile input is contained on both transports (part 10) | a flood (1,000 sessions that never send `Join`, plus 10 clients sending oversize datagrams at full rate and opening streams) leaves server tick time p99 within 10 % of a clean run and resident memory under 64 MB; the same suite over WebSocket, plus oversized frames and messages, with the same marks; a `ws://` connection and a wrong Origin refused (403) from all four browsers; a session without `Join` 2 s after its TCP accept or its QUIC handshake's first packet closed, and 10 slow-loris clients (one byte a second into the TLS ClientHello, and over WebSocket into the upgrade headers) each closed by that deadline, the handshakes in progress held at their cap; with the per-address cap on, the first connection over the cap from one address refused, and the same from a client cycling 1,000 addresses inside one IPv6 /64 (where LG has IPv6; recorded if it has not) | RS with LG (flood); ML and MAC (Origin) | S30's room under each flood for 10 minutes (proposed), run twice: with the per-address cap raised to the run's session count on LG's addresses (the never-`Join` flood as R2-TRN-REC1 mark 4 states it), then with it on (the refusals) | 1 run per transport and cap setting; V-S's clean reference. Bites: the library's default limits restored; the `Join` deadline counted from the upgrade's end (the slow-loris stays open); per-address limits keyed by the whole IPv6 address (the /64 cycler gets through) | r2 R2-TRN-REC1 mark 4 (lines 2597–2609); R2-TRN-REC3 mark 4 (lines 2670–2671); the draft, part 10 (the deadline from the accept, the handshake cap, keys by IPv4 /32 and IPv6 /64); the security review of 2026-11-02 (findings 7 and 9) and the infra review (findings 9 and 10); ADR 0006's spike item 2 (the cap raised for LG) |
END

### FIX RR-3 as routed (PM-0001-26: the poisoned-lock bite; the new row PM-0001-59: a tick-thread panic ends the match)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-26 | A panic in one connection ends only that connection (part 11) | the panicking connection closes; the room's other 47 connections stay (0 disconnects) and its tick goes on (0 overruns in the following minute, proposed) | RS with LG | a panic injected into one connection's handler, 10 times in one S30 room (proposed) | every injection. Bites: `panic = "abort"` in the build | r2 R2-ROOM-REC2 mark 6 (lines 2780–2782) |
AFTER:
| PM-0001-26 | A panic in one connection ends only that connection (part 11) | the panicking connection closes; the room's other 47 connections stay (0 disconnects) and its tick goes on (0 overruns in the following minute, proposed) | RS with LG | a panic injected into one connection's handler, 10 times in one S30 room (proposed) | every injection. Bites: `panic = "abort"` in the build; a `std::sync::Mutex` shared by the tick and the connection handlers, held by the handler when it panics (the poisoned lock's next `lock().unwrap()` panics the tick: part 11's bounded-channels rule) | r2 R2-ROOM-REC2 mark 6 (lines 2780–2782) |
| PM-0001-59 | A panic on the tick thread ends the match and the process (part 11; OQ-0001-18) | after a panic injected into the sim step: the process exits with a non-zero code within 1 s of the panic (a plain limit; proposed in the draft's failure-mode row); every client gets `RoomEnded` with a crash reason or has its connection closed within that second (0 left open); the match record is written and marked crashed; the supervisor's stand-in logs the exit; the report reaches the error log (ADR 0006 part 10's endpoint once it exists) | RS with LG | 10 injections (proposed), each in its own S30 room at a random tick after warm-up; the exit code and time from the supervisor's stand-in, each client's last message and close from LG's log | every injection. Bites: a tick thread whose panic leaves the process running (the loop without its top-level catch, under `panic = "unwind"`: the thread ends and the room holds its connections and sends nothing) | the draft, part 11 (a panic on the tick thread is fatal to the room) and "Failure modes", row "A panic on the room's tick thread" (its 1 s, proposed there); r2 R2-ROOM-F8 (lines 1459–1462: an uncaught panic "will exit" only its own thread); the Rust review of 2026-11-02, finding 3; if OQ-0001-18 picks its option (b), this row is rewritten for the resume |
END

### FIX EA-7 and IE-2 (PM-0001-27: PM-0006-13's setup, bracket B, 3 sessions, no pinning)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-27 | Rooms share a VM fairly (part 11; with ADR 0006's density figure) | 8 rooms as 8 processes on one machine: each room's server tick time p99 within 10 % of the single-room figure (PM-0001-4's) over 20 minutes; killing one room's process (SIGKILL) leaves the others' p99 within 5 % (proposed reading of "unchanged": R2-TRN-REC6 mark 2 writes "unchanged (within 5 %)") and 0 of their connections dropped | RS, then SH | each room in SS with a snapshot-receiving client on every soldier (proposed: a room of server-side bots sends no snapshots, so its density would leave out encoding and transport work) | 1 run per machine; V-S. ADR 0006 repeats SH at 3 times of day | r2 R2-ROOM-REC2 marks 2 and 4 (lines 2777, 2779–2780); r6 R6-HOST-REC2's mark (lines 667–671) |
AFTER:
| PM-0001-27 | Rooms share a VM fairly (part 11; with ADR 0006's density figure) | 8 rooms as 8 processes on one machine: each room's server tick time p99 within 10 % of the single-room figure over 20 minutes, that figure taken as ADR 0006's bracket B takes it (the mean of a lone room's p99, run first and last in the same session), not PM-0001-4's pinned figure; killing one room's process (SIGKILL) leaves the others' p99 within 5 % (proposed reading of "unchanged": R2-TRN-REC6 mark 2 writes "unchanged (within 5 %)") and 0 of their connections dropped | RS, then SH, each with LG | the setup and load of ADR 0006's PM-0006-13: 8 room processes under the supervisor, no CPU pinning, each process's CPU share of one vCPU and the VM's steal recorded per second and reported; each room in SS with a snapshot-receiving client on every soldier (proposed: a room of server-side bots sends no snapshots, so its density would leave out encoding and transport work) | PM-0006-13's: 3 sessions per machine at the 3 times of day, every valid session passes; one set of runs serves both records; V-S's LG clause holds, its pinning and steal clauses do not (the key) | r2 R2-ROOM-REC2 marks 2 and 4 (lines 2777, 2779–2780); r6 R6-HOST-REC2's mark (lines 667–671); ADR 0006's PM-0006-13 and its bracket B; the engine and infra reviews of 2026-11-02 (findings 7 and 2) |
END

### FIX IE-7 and IE-9/10 as routed (PM-0001-29: attempts one at a time under the per-address cap; a TCP-443-only network reported)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-29 | The fallback takes over when QUIC is blocked (part 10; decides R2-TRN-OQ2) | with UDP blocked to the server, each of the four browsers reaches a playing WebSocket session at most 3 s after the page starts connecting (a plain limit; WebTransport timeout 1.5 s) | ML, MAC; RS with its UDP range closed | 20 attempts per browser (proposed) | every attempt. Bites: no WebTransport timeout (the attempt hangs) | r2 R2-TRN-REC3 mark 1 (lines 2664–2665); the draft, part 10 (WebTransport tried first with a timeout, then WebSocket) |
AFTER:
| PM-0001-29 | The fallback takes over when QUIC is blocked (part 10; decides R2-TRN-OQ2) | with UDP blocked to the server, each of the four browsers reaches a playing WebSocket session at most 3 s after the page starts connecting (a plain limit; WebTransport timeout 1.5 s) | ML, MAC; RS with its UDP range closed | 20 attempts per browser (proposed), one at a time, each session closed before the next, so the per-address cap (part 10, above 1) never refuses one (its setting recorded); the fallback on the TCP port the lobby hands out (part 10's interim, OQ-0006-16's option (a)). Reported, no pass number: one attempt per browser from a network that allows only TCP 443 (proposed by the infra review of 2026-11-02), whether play is reached, for OQ-0006-16's 443 question | every attempt. Bites: no WebTransport timeout (the attempt hangs) | r2 R2-TRN-REC3 mark 1 (lines 2664–2665); the draft, part 10 (WebTransport tried first with a timeout, then WebSocket) |
END

### FIX TE-3 as routed (PM-0001-30: a run counts only with the late-command counter at 0)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-30 | Both transports give the same server (part 10) | one recorded input replay drives both transports to identical server [state hashes](../research/GLOSSARY.md#state-hash) on every tick | RS (any machine for the hash) | 3 recordings of 5 minutes (proposed), each replayed over WebTransport and over WebSocket | 3 recordings; 0 mismatching ticks. Bites: a wall-clock read on one transport's command path | r2 R2-TRN-REC3 mark 2 (lines 2666–2667) |
AFTER:
| PM-0001-30 | Both transports give the same server (part 10) | one recorded input replay drives both transports to identical server [state hashes](../research/GLOSSARY.md#state-hash) on every tick; a run counts only if every command ran at the tick stamped on it and the server's late-command counter reads 0 at its end, whatever OQ-0001-6 picks for play (a late command makes the run invalid, not failed: the draft's item 1) | RS (any machine for the hash) | 3 recordings of 5 minutes (proposed), each replayed over WebTransport and over WebSocket at P0, the input buffer set deep enough for P0 on both transports (the draft's item 1) and recorded | 3 recordings; 0 mismatching ticks. Bites: a wall-clock read on one transport's command path; the validity check itself: one command of a replay delayed past its tick must make that run invalid (counter above 0), never a pass | r2 R2-TRN-REC3 mark 2 (lines 2666–2667); the draft, "What a spike must prove" item 1 and the test plan (the late-command counter at 0); the test review of 2026-11-02, finding 3 |
END

### FIX NA-4 as it lands in part 5 (PM-0001-33: a bite for the pong's hold time)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-33 | The client's RTT agrees with the server's (parts 5, 8) | p99 of the absolute difference between app-level RTT and quinn's `rtt()`, sampled every second over 10 minutes, at most 5 ms at P2 (the draft's restatement); also recorded at a fixed 100 ms with no jitter or loss (R5-TEST-REC4's reading) | L2 on RS; ML (Chrome) | both estimates logged every second | 1 run per profile. Bites: pongs answered from the tick thread (up to one tick period late) | r2 R2-TRN-REC4 mark 2 (line 2694); the draft, "What a spike must prove" item 1; r5 R5-TEST-REC4 (lines 1471–1472) |
AFTER:
| PM-0001-33 | The client's RTT agrees with the server's (parts 5, 8) | p99 of the absolute difference between app-level RTT and quinn's `rtt()`, sampled every second over 10 minutes, at most 5 ms at P2 (the draft's restatement); also recorded at a fixed 100 ms with no jitter or loss (R5-TEST-REC4's reading) | L2 on RS; ML (Chrome) | both estimates logged every second | 1 run per profile. Bites: pongs answered from the tick thread (up to one tick period late); the client's pong sent without its hold time (up to one tick period added: part 5) | r2 R2-TRN-REC4 mark 2 (line 2694); the draft, "What a spike must prove" item 1; r5 R5-TEST-REC4 (lines 1471–1472) |
END

