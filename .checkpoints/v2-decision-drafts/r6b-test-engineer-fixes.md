# r6b test-engineer fixes (authors round, plan r6)

### FIX F-0001-KEY-RS-LG
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **Machines.** **RS**, the reference server: a Hetzner Cloud CCX23 (4 dedicated vCPU, 16 GB, x86-64 AMD) in one EU
  location (r6-infrastructure.md, R6-HOST-REC2, lines 645–650; [the reference server](../research/GLOSSARY.md#the-reference-server));
  every server time mark is the work's elapsed time on one vCPU of it with no other room on that vCPU
  ([the measuring unit](../research/GLOSSARY.md#the-measuring-unit-one-vcpu)): the room's tick thread is pinned alone to
  one vCPU and the transport runtime runs on the other three (proposed placement, so the measured vCPU holds only the
  tick), in every server time mark but the rooms-per-VM rows PM-0001-27 and PM-0001-63, which run unpinned as ADR
  0006's PM-0006-13 sets it (eight tick threads cannot each be alone on one of four vCPUs). **SH**, the shared CX33
  (4 vCPU, 8 GB) beside it, for the rooms-per-VM mark only (R6-HOST-REC2). **LG**, the load
  generator: one or more VMs in the same location (CX33s: "the CX33 runs the simulated clients", SPIKE-MACHINES,
  Mustafa, 2026-11-03) running the headless bot clients, which
  speak the real protocol (r5-toolchain-quality.md, R5-TEST-REC5, lines 1475–1480), never RS or SH; as many as a
  5-minute calibration run at the session's load shows keep each under 80 % of its CPU, their count in the run plan
  and their hours in the spike's bill (ADR 0006's spike item 2 and its LG key, PM-0006-18); on LG's addresses the
  per-address connection cap (part 10) is raised to the run's session count and recorded, except in PM-0001-25's run
  with the cap on (proposed: clients on RS would take CPU from the measured work; one 4-vCPU VM may not carry 384
  encrypted sessions, and the cap would refuse most sessions from one address: the infra review of 2026-11-02,
  findings 9 and 10). **ML, RD, MAC** as ADR 0003's pass marks name them (r3-rendering.md, R3-BUDGET-REC1, lines
AFTER:
- **Machines.** **RS**, the reference server: a Hetzner Cloud CCX23 (4 dedicated vCPU, 16 GB, x86-64 AMD) in
  Falkenstein (`fsn1`), "a CCX23 for the game server" (Mustafa, 2026-11-03; ADR 0006's RS key quotes his words), the
  machine every server row runs on (r6-infrastructure.md, R6-HOST-REC2, lines 645–650;
  [the reference server](../research/GLOSSARY.md#the-reference-server));
  every server time mark is the work's elapsed time on one vCPU of it with no other room on that vCPU
  ([the measuring unit](../research/GLOSSARY.md#the-measuring-unit-one-vcpu)): the room's tick thread is pinned alone to
  one vCPU and the transport runtime runs on the other three (proposed placement, so the measured vCPU holds only the
  tick), in every server time mark but the rooms-per-VM rows PM-0001-27 and PM-0001-63, which run unpinned as ADR
  0006's PM-0006-13 sets it (eight tick threads cannot each be alone on one of four vCPUs). No shared host is measured
  beside RS: the CX33 cannot be ordered in Falkenstein (Mustafa, 2026-11-03). **LG**, the load generator: "a CAX21 for
  the 48 simulated players" (Mustafa, 2026-11-03), Arm64, "so their build targets aarch64" (Mustafa, 2026-11-03, his
  console check), created in `fsn1` on the run day, running the headless bot clients, which speak the real protocol
  (r5-toolchain-quality.md, R5-TEST-REC5, lines 1475–1480), never on RS; a 5-minute calibration run at the session's
  load shows each CAX21 under 80 % of its CPU (ADR 0006's LG key). A run whose bots one CAX21 cannot carry under 80 %
  adds as many CAX21s as its 5-minute calibration shows are needed, inside the cost cap Mustafa OKs for that run
  (LG-COUNT, the option he chose, 2026-11-03); their count goes into the run plan, which names the CAX21 that carries
  MV (ADR 0006's metrics role), and their hours into the spike's bill (ADR 0006's spike item 2 and its LG key,
  PM-0006-18); on LG's addresses, the day's (the VMs are new at every run: ADR 0006's key), the per-address connection
  cap (part 10) is raised to the run's session count and recorded, except in PM-0001-25's run with the cap on
  (proposed: clients on RS would take CPU from the measured work; one 4-vCPU VM may not carry 384 encrypted sessions,
  and the cap would refuse most sessions from one address: the infra review of 2026-11-02, findings 9 and 10). LG's
  clients are native aarch64 builds, so no mark's verdict rests on them agreeing with x86-64 or Wasm: in the rows
  whose client side does float work (PM-0001-17, -19, -48 and -54), a mismatch seen on LG counts only once the same
  inputs, replayed on x86-64 (L1 on CI's x86-64 runner), mismatch too, and one that shows only on aarch64 is reported
  as a cross-architecture finding and decides nothing (the advisor's reading, 2026-11-03). **ML, RD, MAC** as ADR
  0003's pass marks name them (r3-rendering.md, R3-BUDGET-REC1, lines
END

### FIX F-0001-KEY-MAC
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  judges, never a measured result, and nothing stands in for RD or MAC (the advisor's reading, 2026-11-03). Mustafa
AFTER:
  judges, never a measured result, and nothing stands in for RD, nor for MAC beyond the macOS runner below
  (the advisor's reading, 2026-11-03). For the netcode proof test's Safari rows, Mustafa put one step first
  (MACHINES-TIMING, 2026-11-03): its plan checks "whether GitHub's macOS runners (real Safari, free while the
  repository is public) have a Safari new enough for WebTransport and can reach the test servers"; if they can, "that
  is the Safari check" (if not, "I borrow the Mac before the netcode proof test"), and a runner stands as MAC once the
  run's results record (i) the runner image's Safari version on the day, at least 26.4 (R2-TRN-F7), (ii) real Safari
  driven through safaridriver, not a WebKit build, and (iii) from the runner, one WebTransport session to RS that
  opens and carries datagrams and one WebSocket session on the domain, and then only in yes-or-no and count rows
  (PM-0001-28's four-browser path and library pick, its server then RS, since a runner sits on no local network of
  ours; PM-0001-14; PM-0001-25's Origin; ADR 0006's PM-0006-15), while a row whose limit is a time and ADR 0003's
  look proof test stay on the borrowed Mac and the runner's workflow is the proof test's plan's, reviewed by
  security-auditor and infra-engineer (the advisor's reading, 2026-11-03). Mustafa
END

### FIX F-0001-VALIDITY-SH
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  alone on four vCPUs and
  on SH steal is what is measured ("repeated at 3 times of day (shared hosts vary with neighbours)":
  r6-infrastructure.md, R6-HOST-REC2's mark, lines
  668–669). A mark compared with "a clean run" uses the median of 3 clean runs whose p99s lie within
AFTER:
  alone on four vCPUs (the 3 times of day: r6-infrastructure.md, R6-HOST-REC2's mark, lines 668–669). A mark
  compared with "a clean run" uses the median of 3 clean runs whose p99s lie within
END

### FIX F-PM-0001-1
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-1 | The rig is named and the measuring unit holds (part 14; a precondition of every server mark) | RS, SH and LG named with plan, CPU model, vCPU count, memory, location and kernel; ML, RD and MAC as ADR 0003 names them; the tick thread's CPU affinity is one vCPU that no other thread of any process uses, in every run but those of PM-0001-27 and PM-0001-63, whose unpinned rooms record each process's affinity instead (ADR 0006's PM-0006-13); steal time recorded per second; the number of threads on each core that `lscpu` reports and the map of which vCPUs are SMT siblings, with the tick thread's sibling kept free of the transport runtime where a core carries 2 threads (as ADR 0006's PM-0006-11 records it; proposed) | RS, SH, LG, ML, RD, MAC | a results-log header per machine, written before the first run; the tick thread's affinity and the other threads' affinities read from the operating system at the start and end of every run | checked before the first run and in every run's log. Bites: a single-room run with a second room's tick thread on the same vCPU is flagged invalid (the rooms-per-VM rows PM-0001-27 and PM-0001-63 are exempt: they run unpinned under PM-0006-13's rules) | GLOSSARY, the reference server (lines 225–243) and the measuring unit (lines 244–264); r6 R6-HOST-REC2 (lines 645–650); r3 R3-BUDGET-REC1 (lines 1096–1102) |
AFTER:
| PM-0001-1 | The rig is named and the measuring unit holds (part 14; a precondition of every server mark) | RS and LG named with plan, CPU model, vCPU count, memory, location, kernel and the day's address (the VMs are created fresh on the run day: ADR 0006's key); ML, RD and MAC as ADR 0003 names them; the tick thread's CPU affinity is one vCPU that no other thread of any process uses, in every run but those of PM-0001-27 and PM-0001-63, whose unpinned rooms record each process's affinity instead (ADR 0006's PM-0006-13); steal time recorded per second; the number of threads on each core that `lscpu` reports and the map of which vCPUs are SMT siblings, with the tick thread's sibling kept free of the transport runtime where a core carries 2 threads (as ADR 0006's PM-0006-11 records it; proposed) | RS, LG, ML, RD, MAC | a results-log header per machine, written before the first run; the tick thread's affinity and the other threads' affinities read from the operating system at the start and end of every run | checked before the first run and in every run's log. Bites: a single-room run with a second room's tick thread on the same vCPU is flagged invalid (the rooms-per-VM rows PM-0001-27 and PM-0001-63 are exempt: they run unpinned under PM-0006-13's rules) | GLOSSARY, the reference server (lines 225–243) and the measuring unit (lines 244–264); r6 R6-HOST-REC2 (lines 645–650); r3 R3-BUDGET-REC1 (lines 1096–1102) |
END

### FIX F-PM-0001-16
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-16 | A room starts fast (part 11) | every room start, process spawn to Open, under 2 s (a plain limit) | RS and SH | every room start in the spike, at least 20 (proposed), timed by the supervisor's stand-in | every start. Bites: a start that loads its mode file twice over a 1 s delay | r2 R2-ROOM-REC2 mark 5 (line 2780); the draft, the budgets table ("Budgets for the netcode proof test"), row "Room start-up" |
AFTER:
| PM-0001-16 | A room starts fast (part 11) | every room start, process spawn to Open, under 2 s (a plain limit) | RS | every room start in the spike, at least 20 (proposed), timed by the supervisor's stand-in | every start. Bites: a start that loads its mode file twice over a 1 s delay | r2 R2-ROOM-REC2 mark 5 (line 2780); the draft, the budgets table ("Budgets for the netcode proof test"), row "Room start-up" |
END

### FIX F-PM-0001-27
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-27 | Rooms share a VM fairly (part 11; with ADR 0006's density figure) | 8 rooms as 8 processes on one machine: each room's server tick time p99 within 10 % of the single-room figure over 20 minutes, that figure taken as ADR 0006's bracket B takes it (the mean of a lone room's p99, run first and last in the same session), not PM-0001-4's pinned figure; killing one room's process (SIGKILL) leaves the others' p99 within 5 % (proposed reading of "unchanged": R2-TRN-REC6 mark 2 writes "unchanged (within 5 %)") and 0 of their connections dropped | RS, then SH, each with LG | the setup and load of ADR 0006's PM-0006-13: 8 room processes under the supervisor, no CPU pinning, each process's CPU share of one vCPU and the VM's steal recorded per second and reported; each room in SS with a snapshot-receiving client on every soldier (proposed: a room of server-side bots sends no snapshots, so its density would leave out encoding and transport work) | PM-0006-13's: 3 sessions per machine at the 3 times of day, every valid session passes; one set of runs serves both records; validity is PM-0006-13's (bracket B and LG's 80 % rule), not V-S, and each vCPU's steal is recorded per second and reported, never a validity condition (the key) | r2 R2-ROOM-REC2 marks 2 and 4 (lines 2777, 2779–2780); r6 R6-HOST-REC2's mark (lines 667–671); ADR 0006's PM-0006-13 and its bracket B; the engine and infra reviews of 2026-11-02 (findings 7 and 2) |
AFTER:
| PM-0001-27 | Rooms share a VM fairly (part 11; with ADR 0006's density figure) | 8 rooms as 8 processes on one machine: each room's server tick time p99 within 10 % of the single-room figure over 20 minutes, that figure taken as ADR 0006's bracket B takes it (the mean of a lone room's p99, run first and last in the same session), not PM-0001-4's pinned figure; killing one room's process (SIGKILL) leaves the others' p99 within 5 % (proposed reading of "unchanged": R2-TRN-REC6 mark 2 writes "unchanged (within 5 %)") and 0 of their connections dropped | RS with LG | the setup and load of ADR 0006's PM-0006-13: 8 room processes under the supervisor, no CPU pinning, each process's CPU share of one vCPU and the VM's steal recorded per second and reported; each room in SS with a snapshot-receiving client on every soldier (proposed: a room of server-side bots sends no snapshots, so its density would leave out encoding and transport work) | PM-0006-13's: 3 sessions at the 3 times of day, every valid session passes; one set of runs serves both records; validity is PM-0006-13's (bracket B and LG's 80 % rule), not V-S, and each vCPU's steal is recorded per second and reported, never a validity condition (the key) | r2 R2-ROOM-REC2 marks 2 and 4 (lines 2777, 2779–2780); r6 R6-HOST-REC2's mark (lines 667–671); ADR 0006's PM-0006-13 and its bracket B; the engine and infra reviews of 2026-11-02 (findings 7 and 2) |
END

### FIX F-PM-0001-63
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-63 | The goal budget holds at the rooms per VM measured at 60 Hz (part 2; TICK-60-HEADROOM's addition; ADR 0006 part 8) | k60, the most rooms of N = 48 at a 60 Hz tick for which every room meets R2-ROOM-REC2 marks 1 and 2 at 60 Hz: server tick time p99 at most 8.33 ms ([the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) at 60 Hz), max under 16.67 ms and 0 overruns, and each room's p99 within 10 % of the single-room figure (PM-0006-14's counting rule, run at 60 Hz). Then the goal fleet at k60 by ADR 0006 part 8's formula, ceil(20 ÷ k60) + 1 CCX23 at 86.49 EUR a month plus the staging CX33 (8.99 EUR) and the match records (6.49 EUR), with part 8's 19–24 USD and Sentry beside it, costs at most the goal budget as it stands when the call is made: at slice 6, the figure OQ-0006-19 fixes, and there it decides; in wave 1's early signal the cost is reported beside part 8's provisional figures (about 361 EUR at 8 rooms per VM, about 534 EUR at 4) and decides nothing (the advisor's reading, 2026-11-03). Reported beside it: PM-0006-14's count at 30 Hz on the same machine | RS (the CCX23 that part 8's goal fleet prices: the same machine), with LG | PM-0006-14's search at a 60 Hz tick, N = 48 only: in one session with bracket B (its lone room at 60 Hz), k = 4, then 6, 8 and up by 2 until a k fails, then the odd k between the last pass and the first fail; if k = 4 fails, 3, 2, 1; then the count and count + 1 again at the other 2 times of day. Each room in SS with a snapshot-receiving client on every soldier, 30 snapshots a second encoded on every second tick (part 2), P2 on LG; k room processes under the supervisor, no CPU pinning; each run 2 minutes of warm-up, then 20 minutes measured, 72,000 ticks (20 × 60 × 60), the p99 the 71,280th. The sim is the key's "The sim in the run": in wave 1 its step is padded by a busy-wait to the sim step share's ceiling at 60 Hz, 4.17 ms per tick, as PM-0006-14 pads at 30 Hz (proposed: a lower bound for any sim inside its share, provisional until slice 6), and at slice 6 the real sim runs as PM-0006-14's slice-6 count runs it. The cost by script from k60 and part 8's prices, the arithmetic shown | the figure is the lowest count of the 3 sessions (PM-0006-14's rule); validity as PM-0006-13 (bracket B, LG's 80 % rule, steal recorded per second and reported, never V-S); a run plan with every session's VM-hours first, as PM-0006-14's; in the 60 Hz run of wave 1 and again at slice 6. Bites: a busy-wait of 20 % of the single-room p99 planted in every tick of one room must fail that room's row at k = 4 (PM-0006-13's bite, at 60 Hz); the cost script fed k60 = 1 must report 21 CCX23 and 1,831.77 EUR (21 × 86.49 + 8.99 + 6.49) and "does not hold" against both provisional figures | TICK-60-HEADROOM, confirmed by Mustafa (2026-11-03) "with one addition: 60 Hz also needs the goal budget to hold at the rooms per VM measured at 60 Hz"; ADR 0006 part 8 (the goal fleet: A-5's peak of 20 rooms plus one VM for draining, its prices and its two provisional figures), PM-0006-13 and PM-0006-14 (the setup and the counting rule) and OQ-0006-19; r2 R2-ROOM-REC2 marks 1 and 2 (lines 2775–2777: the budget is "50 % of the tick period chosen by the tick-rate unit"); r6 A-2 (lines 516–518: "a density to measure at the tick rate R1-TICK-REC2's rule picks"); the advisor's reading of 2026-11-03 (the goal budget as it stands at the call; what "holds" means) |
AFTER:
| PM-0001-63 | The goal budget holds at the rooms per VM measured at 60 Hz (part 2; TICK-60-HEADROOM's addition; ADR 0006 part 8) | k60, the most rooms of N = 48 at a 60 Hz tick for which every room meets R2-ROOM-REC2 marks 1 and 2 at 60 Hz: server tick time p99 at most 8.33 ms ([the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) at 60 Hz), max under 16.67 ms and 0 overruns, and each room's p99 within 10 % of the single-room figure (PM-0006-14's counting rule, run at 60 Hz). Then the goal fleet at k60 by ADR 0006 part 8's formula, ceil(20 ÷ k60) + 1 CCX23 at 86.49 EUR a month plus 8.99 EUR for the staging VM (the staging VM's placeholder price (the CX33's, R6-HOST-F15); its plan is OQ-0006-22) and the match records (6.49 EUR), with part 8's 19–24 USD and Sentry beside it, costs at most the goal budget as it stands when the call is made: at slice 6, the figure OQ-0006-19 fixes, and there it decides; in wave 1's early signal the cost is reported beside part 8's provisional figures (about 361 EUR at 8 rooms per VM, about 534 EUR at 4) and decides nothing (the advisor's reading, 2026-11-03). Reported beside it: PM-0006-14's count at 30 Hz on the same machine | RS (the CCX23 that part 8's goal fleet prices: the same machine), with LG | PM-0006-14's search at a 60 Hz tick, N = 48 only: in one session with bracket B (its lone room at 60 Hz), k = 4, then 6, 8 and up by 2 until a k fails, then the odd k between the last pass and the first fail; if k = 4 fails, 3, 2, 1; then the count and count + 1 again at the other 2 times of day. Each room in SS with a snapshot-receiving client on every soldier, 30 snapshots a second encoded on every second tick (part 2), P2 on LG; k room processes under the supervisor, no CPU pinning; each run 2 minutes of warm-up, then 20 minutes measured, 72,000 ticks (20 × 60 × 60), the p99 the 71,280th. The sim is the key's "The sim in the run": in wave 1 its step is padded by a busy-wait to the sim step share's ceiling at 60 Hz, 4.17 ms per tick, as PM-0006-14 pads at 30 Hz (proposed: a lower bound for any sim inside its share, provisional until slice 6), and at slice 6 the real sim runs as PM-0006-14's slice-6 count runs it. The cost by script from k60 and part 8's prices, the arithmetic shown | the figure is the lowest count of the 3 sessions (PM-0006-14's rule); validity as PM-0006-13 (bracket B, LG's 80 % rule, steal recorded per second and reported, never V-S); a run plan with every session's VM-hours first, as PM-0006-14's; in the 60 Hz run of wave 1 and again at slice 6. Bites: a busy-wait of 20 % of the single-room p99 planted in every tick of one room must fail that room's row at k = 4 (PM-0006-13's bite, at 60 Hz); the cost script fed k60 = 1 must report 21 CCX23 and 1,831.77 EUR (21 × 86.49 + 8.99 + 6.49) and "does not hold" against both provisional figures | TICK-60-HEADROOM, confirmed by Mustafa (2026-11-03) "with one addition: 60 Hz also needs the goal budget to hold at the rooms per VM measured at 60 Hz"; ADR 0006 part 8 (the goal fleet: A-5's peak of 20 rooms plus one VM for draining, its prices and its two provisional figures), PM-0006-13 and PM-0006-14 (the setup and the counting rule) and OQ-0006-19; r2 R2-ROOM-REC2 marks 1 and 2 (lines 2775–2777: the budget is "50 % of the tick period chosen by the tick-rate unit"); r6 A-2 (lines 516–518: "a density to measure at the tick rate R1-TICK-REC2's rule picks"); the advisor's reading of 2026-11-03 (the goal budget as it stands at the call; what "holds" means) |
END

### FIX F-PM-0001-17
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-17 | Every decoded snapshot is the server's state (part 6) | 100 % of decoded snapshots rebuild the server's quantised state for that tick and that client's entity set, checked by [the client-held state's hash](../research/GLOSSARY.md#state-hash); also across the 16-bit tick wrap of every S30 | L1 (48 headless clients); L2: LG's 48 clients | 10 minutes each under P0, P2, P3 and B1; the server logs each client's quantised hash per tick, the client logs its decoded hash; compared by script | L1: 3 seeds per profile; L2: 1 run per profile; 0 mismatches in each. Bites: a 16-bit tick in the snapshot header (fails at the wrap); a client quantiser that rounds differently from the server's | r2 R2-SYNC-REC1 mark 1 (lines 2017–2019); the draft, part 6 (the snapshot header's 32-bit server tick and its 16-bit offsets) |
AFTER:
| PM-0001-17 | Every decoded snapshot is the server's state (part 6) | 100 % of decoded snapshots rebuild the server's quantised state for that tick and that client's entity set, checked by [the client-held state's hash](../research/GLOSSARY.md#state-hash); also across the 16-bit tick wrap of every S30 | L1 (48 headless clients); L2: LG's 48 clients | 10 minutes each under P0, P2, P3 and B1; the server logs each client's quantised hash per tick, the client logs its decoded hash, and each LG client also logs the bytes of every datagram and stream it receives (for the replay); compared by script | L1: 3 seeds per profile; L2: 1 run per profile; 0 mismatches in each. An L2 mismatch counts only once that client's logged bytes, decoded again from the start on x86-64 (L1's decoder on CI's x86-64 runner), miss the server's logged hash too; one that shows only on LG's aarch64 is reported as a cross-architecture finding and decides nothing (the key). Bites: a 16-bit tick in the snapshot header (fails at the wrap); a client quantiser that rounds differently from the server's | r2 R2-SYNC-REC1 mark 1 (lines 2017–2019); the draft, part 6 (the snapshot header's 32-bit server tick and its 16-bit offsets) |
END

### FIX F-PM-0001-19
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-19 | A client joining mid-match, or resyncing, has the whole world fast (parts 6, 11; the failure mode "a trap in the client's Wasm sim") | every join: a complete world (the client-held state's hash equal to the server's for that client) within 300 ms of sending `Join` at P2 (a plain limit); reported: p50, p99, max, and the time from the page's first connect call | L2: LG's clients and ML's Chrome | 100 joins into a running S30 room (proposed), including rejoins after a trap injected into the client's Worker (ADR 0004's S20 injects them; this row times them) | every join. Bites: the first full state sent as datagrams instead of on its stream | r2 R2-BUD-REC3 (line 2359: 100 ms RTT and 2 % loss, restated as P2), "the one join mark" (the draft, "What a spike must prove" item 2; R2-XU-OQ6); the draft, "Failure modes", row "A trap in the client's Wasm sim" |
AFTER:
| PM-0001-19 | A client joining mid-match, or resyncing, has the whole world fast (parts 6, 11; the failure mode "a trap in the client's Wasm sim") | every join: a complete world (the client-held state's hash equal to the server's for that client) within 300 ms of sending `Join` at P2 (a plain limit); reported: p50, p99, max, and the time from the page's first connect call | L2: LG's clients and ML's Chrome | 100 joins into a running S30 room (proposed), including rejoins after a trap injected into the client's Worker (ADR 0004's S20 injects them; this row times them) | every join. ML's Chrome (Wasm) is judged as it stands; on an LG join, a hash mismatch counts only once the join's logged stream and datagram bytes, decoded again on x86-64 (L1's decoder on CI's x86-64 runner), miss the server's hash too, and one that shows only on LG's aarch64 is reported as a cross-architecture finding and decides nothing (the key); the 300 ms limit counts as measured. Bites: the first full state sent as datagrams instead of on its stream | r2 R2-BUD-REC3 (line 2359: 100 ms RTT and 2 % loss, restated as P2), "the one join mark" (the draft, "What a spike must prove" item 2; R2-XU-OQ6); the draft, "Failure modes", row "A trap in the client's Wasm sim" |
END

### FIX F-PM-0001-54
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-54 | Client-side projectile flight holds, or E3b carries the kinds that still desync (part 6; conditional) | only if live projectiles are in the load: 0 desyncs of client-side flight against the server over 1,000 throws, per projectile kind; a desync is first a defect of the shared trajectory function (ADR 0004's PM-0004-73), fixed there and the row run again; only a kind that still desyncs rides in every snapshot (E3b for that kind alone), its live count × 5.5 B per snapshot shown against the 1,067 B cap and PM-0001-12 run again with it; whole E3b is never the route, since part 6 shows it cannot pass (about 188.3 kbps in ES; 300 × 5.5 = 1,650 B per snapshot in SS) | L1; L2 with LG | 1,000 scripted throws, each landing compared with the server's | 3 seeds. Bites: a client trajectory step of 1/60 s against the server's 1/30 s | r2 R2-BUD-REC2 (line 2342); the draft, part 6 (the desync as a defect; E3b per kind) and "What a spike must prove" item 5; the perf review of 2026-11-02, finding 2 |
AFTER:
| PM-0001-54 | Client-side projectile flight holds, or E3b carries the kinds that still desync (part 6; conditional) | only if live projectiles are in the load: 0 desyncs of client-side flight against the server over 1,000 throws, per projectile kind; a desync is first a defect of the shared trajectory function (ADR 0004's PM-0004-73), fixed there and the row run again; only a kind that still desyncs rides in every snapshot (E3b for that kind alone), its live count × 5.5 B per snapshot shown against the 1,067 B cap and PM-0001-12 run again with it; whole E3b is never the route, since part 6 shows it cannot pass (about 188.3 kbps in ES; 300 × 5.5 = 1,650 B per snapshot in SS) | L1; L2 with LG | 1,000 scripted throws, each landing compared with the server's | 3 seeds. On L2, a throw whose landing on an LG client differs from the server's is fed again (its start position, velocity and fuse, as PM-0004-73 feeds throws) to the trajectory function on x86-64 (L1 on CI's x86-64 runner), and counts as a desync only if it differs there too; one that differs only on LG's aarch64 is reported as a cross-architecture finding and decides nothing (the key). Bites: a client trajectory step of 1/60 s against the server's 1/30 s | r2 R2-BUD-REC2 (line 2342); the draft, part 6 (the desync as a defect; E3b per kind) and "What a spike must prove" item 5; the perf review of 2026-11-02, finding 2 |
END

### FIX F-PM-0001-48
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-48 | Hits register as the shooter saw them (part 8; decides OQ-0001-7) | a scripted shooter at TC's sprinting target under P0 to P3: of the shots whose ray hits the target's volume in the shooter's view (the state the Worker published for the command's view time), at least 99 % register on the server up to P2 and 95 % at P3; false hits (a miss in the view counted as a hit) below 0.5 %; judged with the cap above every honest rewind of the run (the run's largest measured rewind rounded up to a whole number of ticks, recorded; proposed: the cap OQ-0001-5's rule sets, the honest rewind's p99 at P2 rounded up, may clip up to 1 % of honest shots at P2 by its definition, which alone could spend the 99 % mark's whole margin, so this mark judges registration apart from the cap, and what that cap clips is PM-0001-49's record); recorded: per shot whether the view time was accepted or replaced by the server's estimate (part 8's estimate with the buffer wait and half a tick for the blend), and the share of honest shots replaced, per profile | L1; L2 with LG's shooter | 2,000 shots per profile (proposed: about 20 misses at the 99 % mark, so a handful of shots does not decide the run) | L1: 3 seeds; L2: 1 run per profile. Bites: a rewind that leaves out the interpolation delay (at 9 m/s, 100 ms is about 0.9 m); an estimate without the buffer wait (with a buffer of 2 ticks or more, honest view times miss it by more than the 33.3 ms tolerance and are replaced: the netcode review's finding 1) | r2 R2-SYNC-REC5 mark 1 (lines 2179–2181); the draft, part 8 (the estimate and its arithmetic) and OQ-0001-7; the netcode review of 2026-11-02, finding 1 |
AFTER:
| PM-0001-48 | Hits register as the shooter saw them (part 8; decides OQ-0001-7) | a scripted shooter at TC's sprinting target under P0 to P3: of the shots whose ray hits the target's volume in the shooter's view (the state the Worker published for the command's view time), at least 99 % register on the server up to P2 and 95 % at P3; false hits (a miss in the view counted as a hit) below 0.5 %; judged with the cap above every honest rewind of the run (the run's largest measured rewind rounded up to a whole number of ticks, recorded; proposed: the cap OQ-0001-5's rule sets, the honest rewind's p99 at P2 rounded up, may clip up to 1 % of honest shots at P2 by its definition, which alone could spend the 99 % mark's whole margin, so this mark judges registration apart from the cap, and what that cap clips is PM-0001-49's record); recorded: per shot whether the view time was accepted or replaced by the server's estimate (part 8's estimate with the buffer wait and half a tick for the blend), and the share of honest shots replaced, per profile | L1; L2 with LG's shooter | 2,000 shots per profile (proposed: about 20 misses at the 99 % mark, so a handful of shots does not decide the run) | L1: 3 seeds; L2: 1 run per profile. On L2, LG's shooter logs per shot the ray and the target's interpolated pose its view test used; a shot that counts against the mark there (a view hit not registered, or a false hit) counts only once that view test, replayed with the logged inputs on x86-64 (L1's ray query on CI's x86-64 runner), gives the same result, and one that differs only on LG's aarch64 is reported as a cross-architecture finding and decides nothing (the key). Bites: a rewind that leaves out the interpolation delay (at 9 m/s, 100 ms is about 0.9 m); an estimate without the buffer wait (with a buffer of 2 ticks or more, honest view times miss it by more than the 33.3 ms tolerance and are replaced: the netcode review's finding 1) | r2 R2-SYNC-REC5 mark 1 (lines 2179–2181); the draft, part 8 (the estimate and its arithmetic) and OQ-0001-7; the netcode review of 2026-11-02, finding 1 |
END

### NOTE N-0001-NOT-MINE
FILE: docs/adr/0001-multiplayer-netcode.md
no change: ADR 0001 line 1274 (the domain, AMENDMENT 2) is item 1 of "What a spike must prove" (lines 1236–1335), not a pass-mark
key, row or checklist, so it is netcode-architect's text, not mine: its "on the domain he registers" becomes
"registered" there (author: netcode-architect). Lines 719 ("Every server mark is measured on one ..."), 756 (the
failure row "No Hetzner capacity in the EU", which stays live: AMENDMENT, HZ-OQ1) and 1293 ("on the CCX23 and the
CX33") are netcode-architect's too (AMENDMENT condition 13).
END

### NOTE N-0001-ARM-ROWS
FILE: docs/adr/0001-multiplayer-netcode.md
no change: condition 5's row reading, against netcode-architect's NOTE rows-for-test-engineer
(r6b-netcode-architect-fixes.md). Agreed: PM-0001-17 and -19 (the decode and the client-held state's hash, whose bite
plants a client quantiser, so float work), -54 (the shared trajectory function) and -48 (the shooter's ray test) are
covered; PM-0001-18 (a time per client and burst; it compares no LG-computed value), -49, -42 and -61 (LG load only)
and the other LG rows are not, so they keep their text. One difference: for PM-0001-48 its NOTE suggests way (i) (the
L2 shooter in ML's Chrome or an x86-64 client); F-PM-0001-48 takes way (ii) (LG's shooter logs each shot's ray and the
target's interpolated pose; a shot against the mark counts only once the same view test replayed on x86-64 agrees),
because ML is judged only once named (the key: "like every row on ML, RD or MAC it is judged only on a named machine")
and no x86-64 client machine is in the rig ("the rig is just the CCX23 and the CAX21"), so way (i) would hold the
row's L2 verdict back; its NOTE leaves "(ii) stays open to the row". The four rows say their way in their runs cell
(no row added) and the key's LG entry states the rule once, pointing to them.
END

### NOTE N-0001-ADDRESS-ROWS
FILE: docs/adr/0001-multiplayer-netcode.md
no change: the "address" hits in ADR 0001's rows: PM-0001-25 (line 1508: the per-address cap raised "on LG's addresses", then
on) and PM-0001-29 (line 1514: the cap's setting recorded) set the cap per run on whatever addresses LG has that day;
the key's LG entry (F-0001-KEY-RS-LG) now says those are the day's addresses, new at every run (AMENDMENT condition 11,
the clause written by the key's owner, as the computed task routes it), so the rows need no word changed. The
RS sentence at 1348–1353 keeps "every server time mark ... on one vCPU of it": true again, since every server row runs
on RS (AMENDMENT, DS dropped); F-0001-KEY-RS-LG changes only its location, his words and SH.
END

### NOTE N-0001-MAC-ROWS
FILE: docs/adr/0001-multiplayer-netcode.md
no change: the rows a GitHub macOS runner may serve as MAC under branch 1 (AMENDMENT 2: PM-0001-28, PM-0001-14, PM-0001-25's
Origin; ADR 0006's PM-0006-15) keep "MAC" in their machine cells: F-0001-KEY-MAC defines when a runner stands as MAC,
its checks (i) to (iii) and its limits, so no row's cell changes. One consequence it states: PM-0001-28's "the server
on the same local network" cannot hold for a runner, so on a runner that row's server is RS (recorded); PM-0001-28 on
the borrowed Mac keeps its local server. Every time-limited row on MAC and ADR 0003's rows stay on the borrowed Mac.
END

### FIX F-0004-KEY-RS-CX33
FILE: docs/adr/0004-simulation-core.md
BEFORE:
- **Machines.** **RS**, [one vCPU](../research/GLOSSARY.md#the-measuring-unit-one-vcpu) of
  [the reference server](../research/GLOSSARY.md#the-reference-server): a Hetzner CCX23 (4 dedicated vCPU, 16 GB,
  x86-64 AMD), Linux, one EU location (R6-HOST-REC2, r6 lines 645–650; GL lines 225–230; a working pick while
  R6-HOST-OQ1 is open), with no other room on that vCPU (GL lines 244–250); the bench process is pinned to one vCPU and
  the VM is otherwise idle (proposed: a neighbour on the same VM moves a p99; the pinning tool is the harness's to
  document). **CX33**, the shared-vCPU CX33, measured beside RS where a row says so (R6-HOST-REC2): reported, never
  gated. **ML**, the minimum laptop, and **RD**, the recommended desktop: ADR 0003's machines (R3-BUDGET-REC1, r3 lines
AFTER:
- **Machines.** **RS**, [one vCPU](../research/GLOSSARY.md#the-measuring-unit-one-vcpu) of
  [the reference server](../research/GLOSSARY.md#the-reference-server): a Hetzner CCX23 (4 dedicated vCPU, 16 GB,
  x86-64 AMD), Linux, in Falkenstein (`fsn1`) (R6-HOST-REC2, r6 lines 645–650; GL lines 225–230; R6-HOST-OQ1
  answered by Mustafa's console check, 2026-11-03, OQ-0006-1; ADR 0006's RS key quotes his words), with no other room
  on that vCPU (GL lines 244–250); the bench process is pinned to one vCPU and the VM is otherwise idle (proposed: a
  neighbour on the same VM moves a p99; the pinning tool is the harness's to document). The shared CX33 is not
  measured: it cannot be ordered in Falkenstein (Mustafa, 2026-11-03). **ML**, the minimum laptop, and **RD**, the
  recommended desktop: ADR 0003's machines (R3-BUDGET-REC1, r3 lines
END

### FIX F-PM-0004-1
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-1 | The machines exist, and every result names its machine (a precondition of every spike here) | RS, CX33 and MAC recorded (model, CPU as the operating system reports it, cores, RAM, operating system); ML and RD as PM-0003-1 names them; the session's build machine for PM-0004-33 | RS, CX33, ML, RD, MAC | a results-log header per machine, written before the first run; every later log row names its machine and build | checked before the first run of each spike that uses the machine | R6-HOST-REC2 (r6 lines 645–650); R3-BUDGET-REC1's mark (r3 lines 1114–1116); R1-TICK-OQ6 (lines 2662–2666) |
AFTER:
| PM-0004-1 | The machines exist, and every result names its machine (a precondition of every spike here) | RS and MAC recorded (model, CPU as the operating system reports it, cores, RAM, operating system); ML and RD as PM-0003-1 names them; the session's build machine for PM-0004-33 | RS, ML, RD, MAC | a results-log header per machine, written before the first run; every later log row names its machine and build | checked before the first run of each spike that uses the machine | R6-HOST-REC2 (r6 lines 645–650); R3-BUDGET-REC1's mark (r3 lines 1114–1116); R1-TICK-OQ6 (lines 2662–2666) |
END

### FIX F-PM-0004-4
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-4 | How the cost grows with N, and on the cheaper host (S1; part 1's one thread per match; decides R1-ECS-OQ4 and R1-TICK-OQ5) | reported, no pass number: mean, p50, p95, p99 and max at N = 12, 24, 36, 48 and the N = 64 headroom run, each part beside the total; the same at N = 48 on CX33; legacy's JavaScript step (0.80–1.00 ms mean at 48 soldiers) beside them as a reference, not a prediction. Passes when every cell holds a number. Parallelism inside a match is weighed only if PM-0004-3 fails | RS; CX33 | T on SS at each N | T, V per N | R1-TICK-REC2 P1 (lines 2044–2050); R1-ECS-REC1 mark 1 (lines 1930–1931); R1-ECS-OQ4 (lines 2622–2626); R1-TICK-OQ5 (lines 2658–2660); R6-HOST-REC2 (CX33 beside) |
AFTER:
| PM-0004-4 | How the cost grows with N (S1; part 1's one thread per match; decides R1-ECS-OQ4 and R1-TICK-OQ5, both from RS alone) | reported, no pass number: mean, p50, p95, p99 and max at N = 12, 24, 36, 48 and the N = 64 headroom run, each part beside the total; legacy's JavaScript step (0.80–1.00 ms mean at 48 soldiers) beside them as a reference, not a prediction. Passes when every cell holds a number. Parallelism inside a match is weighed only if PM-0004-3 fails | RS | T on SS at each N | T, V per N | R1-TICK-REC2 P1 (lines 2044–2050); R1-ECS-REC1 mark 1 (lines 1930–1931); R1-ECS-OQ4 (lines 2622–2626); R1-TICK-OQ5 (lines 2658–2660) |
END

### NOTE N-0004-OQ4-OQ5
FILE: docs/adr/0004-simulation-core.md
no change: R1-ECS-OQ4 and R1-TICK-OQ5 are still decided from RS alone (AMENDMENT condition 6 asks the author to say so):
R1-ECS-OQ4 is "Decided by: REC1 mark 1's native sim step time against the sim step share ... and R6's hosting cost
per match" (r1-simulation-core.md lines 2622–2626), both on the reference server, and R1-TICK-OQ5 is "Decided by:
REC2's P1–P3" (lines 2658–2660), run on RS; PM-0004-4's CX33 half was "reported" beside them and decided neither, so
F-PM-0004-4 drops it and its title says "both from RS alone". Neither stays open. Appendix A's rows for them (lines
1329 and 1334) name the sim core spike, unchanged.
END

### NOTE N-0004-A64
FILE: docs/adr/0004-simulation-core.md
no change: the A64 key (line 1042), S7 and PM-0004-26 stay as AMENDMENT condition 6 rules (an Arm load generator is not "an ARM
host" in part 7's sense, the advisor's reading, 2026-11-03); condition 6's one sentence is rust-reviewer's and sits in
part 7 (r6b-rust-reviewer-fixes.md, R6B-RR-1), so the key gains none (it would repeat it). The rest of ADR 0004's
pass marks names no CX33, SH, MV, location or address (grep).
END

### FIX F-0006-KEY-RIG
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  x86-64 AMD) in the location OQ-0006-2 picks (`nbg1` or `hel1`). **SH**, the shared CX33 (4 vCPU, 8 GB) beside it,
  same location. **LG**, the load generator: one or more separate VMs in the same location that run every bot client,
  never the VM under test (proposed: bot clients on the VM under test would spend the vCPUs the marks measure): CX33s
  ("the CX33 runs the simulated clients": SPIKE-MACHINES, Mustafa, 2026-11-03), as many as a 5-minute calibration run
  at the session's load shows keep each under 80 % of its
  CPU, each one's CPU use recorded every second, and a run in which any LG VM passes 80 % is invalid (proposed: 384
  encrypted sessions each receiving 30 snapshots a second may need more than one 4-vCPU VM; the server side alone may
  spend up to 1 vCPU per 48 sessions, ADR 0001); their count goes into the run plan and PM-0006-18. On LG's addresses
  the per-address connection cap (ADR 0001) is raised to the run's session count, and the raised value is recorded
  (What a spike must prove, item 2). **MV**, a
  CX33 in the same location running Prometheus, standing in for the staging game server, which exists only from slice 3
  (proposed). **ML** and **MAC**, the client machines of ADR 0003's pass marks: ML, the minimum laptop (Windows 11:
  Chrome, Edge, Firefox); MAC, a macOS machine with Safari. They carry no budget row here: every client mark in this
  record is a yes-or-no or a count. **MP**, Mustafa's PC, Brave on Windows: only what he judges or reads in his own
  accounts.
AFTER:
  x86-64 AMD) in Falkenstein (`fsn1`): "both test servers will be in Falkenstein, not Nuremberg: a CCX23 for the game
  server and a CAX21 for the 48 simulated players" (Mustafa, 2026-11-03; OQ-0006-2, answered). Every server row runs
  on RS ("the rig is just the CCX23 and the CAX21": the option he chose, in the session's wording, 2026-11-03), and
  no shared host is measured beside it: the CX33 cannot be ordered in Falkenstein (Mustafa, 2026-11-03), so K3 waits
  (part 8). **LG**, the load generator: "a CAX21 for the 48 simulated players" (Mustafa, 2026-11-03), Arm64 (4 vCPU,
  8 GB), "so their build targets aarch64" (Mustafa, 2026-11-03, his console check), created in `fsn1` on the run day;
  it runs every bot client, never the VM under test (proposed: bot clients on the VM under test would spend the vCPUs
  the marks measure). A 5-minute calibration run at the session's load shows each LG VM under 80 % of its CPU, each
  one's CPU use recorded every second, and a run in which any LG VM passes 80 % is invalid (proposed: 384 encrypted
  sessions each receiving 30 snapshots a second may need more than one 4-vCPU VM; the server side alone may spend up
  to 1 vCPU per 48 sessions, ADR 0001). A run whose bots one CAX21 cannot carry under 80 % adds as many CAX21s as its
  5-minute calibration shows are needed, inside the cost cap Mustafa OKs for that run (LG-COUNT, the option he chose,
  2026-11-03); their count goes into the run plan and PM-0006-18, and the plan names which CAX21 carries MV. On LG's
  addresses the per-address connection cap (ADR 0001) is raised to the run's session count, and the raised value is
  recorded (What a spike must prove, item 2). **MV**, the metrics role: Prometheus on LG's CAX21, never on RS, inside
  LG's 80 % rule, standing in for the staging game server (the advisor's reading, 2026-11-03), which exists only from
  slice 3; the calibration run includes it. **The day's addresses:** the VMs are new at every run ("I created both
  once to try the console and deleted them right away. They'll be created fresh on run day, so their IP addresses will
  be new then.": Mustafa, 2026-11-03), so no address is written here and each run records its own with its results,
  and every setting bound to one is made on the run day (the connection cap on LG's addresses; any firewall rule that
  names a source, as PM-0006-15's SSH and metrics rules and PM-0006-16's; the WebSocket leg's DNS record on
  shamblefrontlines.com, set before its certificate is issued), while the new SSH host keys' check goes to the proof
  test's plan with security-auditor and the WebTransport certificate is pinned by hash, not by address, and made per
  run (the advisor's reading, 2026-11-03). **ML** and **MAC**, the client machines of ADR 0003's pass marks: ML, the
  minimum laptop (Windows 11: Chrome, Edge, Firefox); MAC, a macOS machine with Safari, or for the netcode proof
  test's yes-or-no and count rows a GitHub macOS runner, under the step, checks and limits ADR 0001's MAC key sets
  (MACHINES-TIMING, Mustafa, 2026-11-03). They carry no budget row here: every client mark in this record is a
  yes-or-no or a count. **MP**, Mustafa's PC, Brave on Windows: only what he judges or reads in his own accounts.
END

### FIX F-PM-0006-11
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-11 | The reference server exists and is the machine every server row ran on (part 5; OQ-0006-1, the precondition) | a CCX23 and a CX33 created in `nbg1` or `hel1`, after PM-0006-12's round trips and OQ-0006-2's pick of the location; recorded for each: type, location, CPU model, vCPU count, memory, kernel, and the number of threads on each core that `lscpu` reports, with the map of which vCPUs are SMT siblings (if a core carries 2 threads, the transport runtime of ADR 0001's pinned rows is kept off the tick thread's sibling, and that placement is recorded: proposed, since a sibling shares the physical core and "one vCPU with no other room on it" would not isolate the tick; the CCX23's topology is not researched); the CPU model read again at every session (a change is recorded and named in the results) | RS, SH | Mustafa creates the VMs himself for each run, in a separate Hetzner project, and deletes them after it; sessions get SSH access only and no Hetzner token (SPIKE-MACHINES, Mustafa, 2026-11-03: "I create and delete the VMs myself for each run; sessions get SSH access only and no Hetzner token until the security baseline"; OQ-0006-17); a script reads `/proc/cpuinfo` and the plan at each session start | every session | R6-HOST-REC2 (r6 lines 645–650); R6-HOST-F20 (r6 lines 180–182); GLOSSARY, the reference server (lines 225–231) |
AFTER:
| PM-0006-11 | The reference server exists and is the machine every server row ran on (part 5; the precondition; OQ-0006-1, answered by Mustafa, 2026-11-03) | the CCX23 created in `fsn1` on the run day (the key); recorded: type, location, its address of the day, CPU model, vCPU count, memory, kernel, and the number of threads on each core that `lscpu` reports, with the map of which vCPUs are SMT siblings (if a core carries 2 threads, the transport runtime of ADR 0001's pinned rows is kept off the tick thread's sibling, and that placement is recorded: proposed, since a sibling shares the physical core and "one vCPU with no other room on it" would not isolate the tick; the CCX23's topology is not researched); the CPU model read again at every session (a change is recorded and named in the results) | RS | Mustafa creates the VMs himself for each run, in a separate Hetzner project, and deletes them after it; sessions get SSH access only and no Hetzner token (SPIKE-MACHINES, Mustafa, 2026-11-03: "I create and delete the VMs myself for each run; sessions get SSH access only and no Hetzner token until the security baseline"; OQ-0006-17); a script reads `/proc/cpuinfo` and the plan at each session start | every session | R6-HOST-REC2 (r6 lines 645–650); R6-HOST-F20 (r6 lines 180–182); GLOSSARY, the reference server (lines 225–231) |
END

### FIX F-PM-0006-12
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-12 | The round trip to each candidate location (part 5; feeds OQ-0006-2) | reported, no pass number: per location, the mean and the max round trip and the loss of 100 ICMP echoes, as Windows' `ping` prints them | one throwaway CX33 per offered location (`nbg1`, `hel1`, `fsn1` if offered), ICMP allowed on these only, deleted after; from ML and from MP | `ping -n 100 <address>` per location from each machine (MP: the operations checklist, step 1); the outputs pasted into the results log | 1 run per machine and location; a second run at another time of day if two locations differ by under 5 ms (proposed) | OQ-0006-2; R6-HOST-OQ2 (r6 lines 856–858); R6-HOST-F18: the firewall filters ICMP (r6 lines 169–174) |
AFTER:
| PM-0006-12 | The round trip to the test servers' location (part 5; a check) | reported, no pass number: the mean and the max round trip and the loss of 100 ICMP echoes to `fsn1`, as Windows' `ping` prints them, set beside VISION_V2 §4's "about 100 ms"; it decides nothing, and a surprise goes to Mustafa (the advisor's reading, 2026-11-03) | one of the run's CAX21s in `fsn1` (LG's, created on the run day), with ICMP allowed on it only, never on RS; from ML and from MP | `ping -n 100 <address>` to that CAX21's address of the day from each machine (MP: the operations checklist, step 1); the outputs pasted into the results log | 1 run per machine | OQ-0006-2, answered (Mustafa, 2026-11-03); VISION_V2 §4 ("about 100 ms of ping"); R6-HOST-OQ2 (r6 lines 856–858; the playtesters' round trips stay open); R6-HOST-F18: the firewall filters ICMP (r6 lines 169–174) |
END

### FIX F-PM-0006-13
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-13 | Rooms per VM: 8 rooms at N = 48 on each machine (part 5; with ADR 0001's R2-ROOM-REC2 marks 1 and 2: one set of runs serves both records) | each of 8 rooms' server tick p99 at most 1.10 × the single-room figure; on SH also R2-ROOM-REC2 mark 1 for every room (p99 at most 16.67 ms, max under 33.33 ms, 0 overruns: [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) at 30 Hz) and every room start-up (process spawn to Open) under 2 s (R6-HOST-REC2 names both machines; the RS half of marks 1 and 5 is ADR 0001's) | RS and SH, each with LG | 8 room processes under the supervisor, no CPU pinning (as production would run them; recorded); the load at N = 48 (384 bot clients on LG); bracket B; each room process's CPU share of one vCPU per second also recorded (it holds the [transport runtime time](../research/GLOSSARY.md#transport-runtime-time)) | 3 sessions per machine at the 3 times of day; every valid session passes. Validity is bracket B's and LG's 80 % rule, not ADR 0001's V-S: eight tick threads on 4 vCPUs cannot each be alone on a vCPU, and on SH steal time is what the mark measures, so each vCPU's steal is recorded per second and reported, never a validity condition (proposed; the infra review's finding 2). ADR 0001's PM-0001-27 is this set of runs, read the same way. **On SH it is an adoption mark:** a fail, or an invalid session, keeps the fleet on dedicated vCPUs (K1) and K3 out, as part 8 says; it is not a fail of this record. Bite: a busy-wait of 20 % of the single-room p99 planted in every tick of one room must fail that room's row (proposed) | R6-HOST-REC2's mark (r6 lines 667–671); R2-ROOM-REC2 marks 1, 2, 5 (r2 lines 2774–2780); A-2 (r6 lines 516–522); N32 (0006 the figure, 0001 the test) |
AFTER:
| PM-0006-13 | Rooms per VM: 8 rooms at N = 48 on RS (part 5; with ADR 0001's R2-ROOM-REC2 marks 1 and 2: one set of runs serves both records) | each of 8 rooms' server tick p99 at most 1.10 × the single-room figure (R2-ROOM-REC2 marks 1 and 5 on RS are ADR 0001's) | RS with LG | 8 room processes under the supervisor, no CPU pinning (as production would run them; recorded); the load at N = 48 (384 bot clients on LG, on as many CAX21s as its calibration shows: the key); bracket B; each room process's CPU share of one vCPU per second also recorded (it holds the [transport runtime time](../research/GLOSSARY.md#transport-runtime-time)) | 3 sessions at the 3 times of day; every valid session passes. Validity is bracket B's and LG's 80 % rule, not ADR 0001's V-S: eight tick threads on 4 vCPUs cannot each be alone on a vCPU, so each vCPU's steal is recorded per second and reported, never a validity condition (proposed; the infra review's finding 2). ADR 0001's PM-0001-27 is this set of runs, read the same way. Bite: a busy-wait of 20 % of the single-room p99 planted in every tick of one room must fail that room's row (proposed) | R6-HOST-REC2's mark (r6 lines 667–671); R2-ROOM-REC2 marks 1, 2, 5 (r2 lines 2774–2780); A-2 (r6 lines 516–522); N32 (0006 the figure, 0001 the test) |
END

### FIX F-PM-0006-14
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-14 | Rooms per VM counted, the figure that replaces A-2 (parts 5, 8; decides R6-COST-OQ1) | reported, no pass number: on each machine and at N = 48, 36 and 18, the most rooms k for which every room meets R2-ROOM-REC2 marks 1 and 2 (the draft's counting rule). In these runs the stand-in's sim step is padded by a busy-wait to the sim step share's ceiling, 8.33 ms per tick (ADR 0001 part 3), or replays the step times the sim core spike recorded, so the figure is a lower bound on rooms per VM for any sim inside its share, labelled "a lower bound, provisional until slice 6" (OQ-0006-19's option (a), the interim; proposed: a stand-in cheaper than the real sim would overstate k); slice 6 re-measures on the real sim (the draft's migration step 7). Each room's padded step time p99 is recorded, so a run whose padding fell short is invalid | RS and SH, each with LG | per N, in one session with bracket B: k = 4 (A-2's case to budget, one room per vCPU), then 6, 8 and up by 2 until a k fails, then the odd k between the last pass and the first fail; if k = 4 fails, 3, 2, 1. Then the count and count + 1 again at the other 2 times of day. The load as PM-0006-13 with N set to 48, 36 and 18 (the goal modes and the three-team start mode's upper end) | the figure is the lowest count of the 3 sessions (proposed: a budget takes the worst case); validity as PM-0006-13 (bracket B, LG's 80 % rule, steal reported, never ADR 0001's V-S). A run plan with every session's VM-hours is written before the first session (it feeds PM-0006-18) | R6-COST-REC1's mark (r6 lines 714–716); the counting rule (draft, spike 2, proposed there); R6-COST-OQ1 (r6 lines 880–882); J6 (2) and OQ-0001-2 (the stand-in's load) |
AFTER:
| PM-0006-14 | Rooms per VM counted, the figure that replaces A-2 (parts 5, 8; decides R6-COST-OQ1) | reported, no pass number: on RS and at N = 48, 36 and 18, the most rooms k for which every room meets R2-ROOM-REC2 marks 1 and 2 (the draft's counting rule). In these runs the stand-in's sim step is padded by a busy-wait to the sim step share's ceiling, 8.33 ms per tick (ADR 0001 part 3), or replays the step times the sim core spike recorded, so the figure is a lower bound on rooms per VM for any sim inside its share, labelled "a lower bound, provisional until slice 6" (OQ-0006-19's option (a), the interim; proposed: a stand-in cheaper than the real sim would overstate k); slice 6 re-measures on the real sim (the draft's migration step 7). Each room's padded step time p99 is recorded, so a run whose padding fell short is invalid | RS with LG | per N, in one session with bracket B: k = 4 (A-2's case to budget, one room per vCPU), then 6, 8 and up by 2 until a k fails, then the odd k between the last pass and the first fail; if k = 4 fails, 3, 2, 1. Then the count and count + 1 again at the other 2 times of day. The load as PM-0006-13 with N set to 48, 36 and 18 (the goal modes and the three-team start mode's upper end) | the figure is the lowest count of the 3 sessions (proposed: a budget takes the worst case); validity as PM-0006-13 (bracket B, LG's 80 % rule, steal reported, never ADR 0001's V-S). A run plan with every session's VM-hours is written before the first session (it feeds PM-0006-18) | R6-COST-REC1's mark (r6 lines 714–716); the counting rule (draft, spike 2, proposed there); R6-COST-OQ1 (r6 lines 880–882); J6 (2) and OQ-0001-2 (the stand-in's load) |
END

### FIX F-PM-0006-15
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-15 | The firewall lets both game transports in and nothing else (part 5) | from each of the four browsers, 20 of 20 WebTransport sessions over UDP and 20 of 20 WebSocket sessions over TCP (`wss`) reach a room on RS; with no rule for a port, a probe to a listening service on that port gets no answer (a TCP connect times out, never "refused"; a UDP probe gets no reply); the firewall's rule list for each VM role equals part 5's list for that role, with no rule outside it and every source limited as listed (the game VM under test: the UDP range, the TCP range, part 9's metrics rule with MV as its only source, PM-0006-16, and SSH by key from the named source addresses only; staging's delivery path and TCP 80 for http-01 only where part 5 names them). The `wss` endpoint presents a certificate the four browsers accept with no warning, and which one is recorded: a trusted certificate on the domain Mustafa registers (SPIKE-MACHINES, Mustafa, 2026-11-03: "for the WebSocket leg, a domain I register"; OQ-0006-5's development case (i): the advisor's reading, 2026-11-03); the WebSocket constructor takes only a URL and sub-protocols (R2-ROOM-F12, r2 lines 1494–1500), so a self-signed certificate cannot be pinned for `wss` | RS; ML: Chrome, Edge, Firefox; MAC: Safari; a probe from ML | WebdriverIO opens the netcode proof test's client page with the transport forced by a URL flag; an attempt succeeds when the session is ready and one datagram or message echoes back; the probe opens a TCP connection and sends a UDP datagram to one closed port of each kind, with a test listener on each; the rule list is read from Hetzner's console or API and pasted into the results | 20 attempts per browser and transport (proposed: R2-TRN-REC5's count); 1 probe pair. Bites: a rule added for the probed TCP port makes the probe connect (the negative test can fail); an SSH rule opened to every source fails the list comparison | R6-HOST-REC2's mark (r6 lines 669–671); part 5 ("and nothing else"); R6-HOST-F18 (r6 lines 169–174); R2-TRN-REC3 (r2 lines 2639–2659) |
AFTER:
| PM-0006-15 | The firewall lets both game transports in and nothing else (part 5) | from each of the four browsers, 20 of 20 WebTransport sessions over UDP and 20 of 20 WebSocket sessions over TCP (`wss`) reach a room on RS; with no rule for a port, a probe to a listening service on that port gets no answer (a TCP connect times out, never "refused"; a UDP probe gets no reply); the firewall's rule list for each VM role equals part 5's list for that role, with no rule outside it and every source limited as listed (the game VM under test: the UDP range, the TCP range, part 9's metrics rule with MV's address of the day as its only source, PM-0006-16, and SSH by key from the named source addresses only; staging's delivery path and TCP 80 for http-01 only where part 5 names them). The `wss` endpoint presents a certificate the four browsers accept with no warning, and which one is recorded: a trusted certificate on shamblefrontlines.com, the domain Mustafa registered ("DOMAIN: done. shamblefrontlines.com is registered at Cloudflare.": Mustafa, 2026-11-03; earlier, SPIKE-MACHINES: "for the WebSocket leg, a domain I register"; OQ-0006-5's development case (i): the advisor's reading, 2026-11-03), its DNS record set to RS's address of the day before the certificate is issued (the key); the WebSocket constructor takes only a URL and sub-protocols (R2-ROOM-F12, r2 lines 1494–1500), so a self-signed certificate cannot be pinned for `wss` | RS; ML: Chrome, Edge, Firefox; MAC: Safari; a probe from ML | WebdriverIO opens the netcode proof test's client page with the transport forced by a URL flag; an attempt succeeds when the session is ready and one datagram or message echoes back; the probe opens a TCP connection and sends a UDP datagram to one closed port of each kind, with a test listener on each; the rule list is read from Hetzner's console or API and pasted into the results | 20 attempts per browser and transport (proposed: R2-TRN-REC5's count); 1 probe pair. Bites: a rule added for the probed TCP port makes the probe connect (the negative test can fail); an SSH rule opened to every source fails the list comparison | R6-HOST-REC2's mark (r6 lines 669–671); part 5 ("and nothing else"); R6-HOST-F18 (r6 lines 169–174); R2-TRN-REC3 (r2 lines 2639–2659) |
END

### FIX F-PM-0006-16
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-16 | The metrics path works with only the rule it needs (part 9) | every metric of PM-0006-22 reaches Prometheus on MV; if part 9 picks the scrape: the exporter's port answers MV's address only and drops every other source (the PM-0006-15 probe from ML gets no answer); if it picks the push gateway: no inbound rule for metrics | RS, MV; a probe from ML | the path is configured as part 9 picks it; the rule names MV's address as its only source (proposed: the narrowest rule that serves a scrape) | 1 probe from ML; the metrics as PM-0006-22. Bite: the rule opened to every source makes ML's probe connect | the draft's spike 2 (proposed there: the negative test extended); R6-OBS-F4: scrape or push (r6 lines 279–284) |
AFTER:
| PM-0006-16 | The metrics path works with only the rule it needs (part 9) | every metric of PM-0006-22 reaches Prometheus on MV; if part 9 picks the scrape: the exporter's port answers MV's address of the day only (the CAX21 that carries MV) and drops every other source (the PM-0006-15 probe from ML gets no answer); if it picks the push gateway: no inbound rule for metrics | RS, MV; a probe from ML | the path is configured as part 9 picks it; the rule names that CAX21's address of the day as its only source, set on the run day (proposed: the narrowest rule that serves a scrape) | 1 probe from ML; the metrics as PM-0006-22. Bite: the rule opened to every source makes ML's probe connect | the draft's spike 2 (proposed there: the negative test extended); R6-OBS-F4: scrape or push (r6 lines 279–284) |
END

### FIX F-PM-0006-18
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-18 | The spike's own hosting bill (part 8) | under the cap OQ-0006-20 sets, excluding VAT, for every Hetzner VM-hour and traffic block the netcode proof test creates (all its marks, ADR 0001's included, LG's calibrated VMs counted); planning check before any VM is created: the run plan's hours at 0.1386 EUR (CCX23) and 0.0144 EUR (CX33) an hour, the arithmetic shown, sum under that cap, else the plan goes back to the draft first (proposed: a bill found too high afterwards cannot be undone). R6's 5 EUR (5 ÷ 0.1386 = 36 CCX23-hours if nothing else ran) predates the run counts: the rows' own plans come to about 23 RS-hours for this record alone (OQ-0006-20), so until it is answered the plan's cost goes to Mustafa before the first session | the Hetzner project of the spike; MP (Mustafa reads the bill) | VMs are created per session and deleted after it (billing rounds up to the hour, R6-HOST-F17); Mustafa reads the project's costs (the operations checklist, step 2) | 1 reading after the last session | R6-COST-REC1's mark (r6 lines 716–717); R6-HOST-F15 hourly prices (r6 lines 150–159); R6-HOST-F17 (r6 lines 164–168); prices exclude VAT (r6 lines 40–41) |
AFTER:
| PM-0006-18 | The spike's own hosting bill (part 8) | under the cap OQ-0006-20 sets, excluding VAT, for every Hetzner VM-hour and traffic block the netcode proof test creates (all its marks, ADR 0001's included, LG's calibrated CAX21s counted); planning check before any VM is created: the run plan's hours at 0.1386 EUR (CCX23) and 0.0176 EUR (CAX21) an hour (R6-HOST-F15, excluding VAT), the arithmetic shown, sum under that cap, else the plan goes back to the draft first (proposed: a bill found too high afterwards cannot be undone). R6's 5 EUR (5 ÷ 0.1386 = 36 CCX23-hours if nothing else ran) predates the run counts: the rows' own plans come to about 23 RS-hours for this record alone (OQ-0006-20), so until it is answered the plan's cost goes to Mustafa before the first session | the Hetzner project of the spike; MP (Mustafa reads the bill) | VMs are created per session and deleted after it (billing rounds up to the hour, R6-HOST-F17); Mustafa reads the project's costs (the operations checklist, step 2) | 1 reading after the last session | R6-COST-REC1's mark (r6 lines 716–717); R6-HOST-F15 hourly prices (r6 lines 150–159); R6-HOST-F17 (r6 lines 164–168); prices exclude VAT (r6 lines 40–41) |
END

### FIX F-0006-FAILS-SH
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
PM-0006-17's interest figure) only when its condition holds; a reported mark (PM-0006-7, -10, -12, -14) when its record
is complete; an adoption mark (PM-0006-13 on SH, PM-0006-14 on SH) never fails the spike: its result only keeps or
admits K3. A failing run is never re-run until green: its data, build commit, host and time are kept, and its cause is
AFTER:
PM-0006-17's interest figure) only when its condition holds; a reported mark (PM-0006-7, -10, -12, -14) when its record
is complete. No mark here is an adoption mark: no shared host is measured (the key), so K3 waits (part 8). A failing
run is never re-run until green: its data, build commit, host and time are kept, and its cause is
END

### FIX F-0006-OPS-STEP1
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
1. Open a Windows terminal and run `ping -n 100 <address>` for each address the session sends (one per candidate
   location). Expected: each run ends with a "Packets" line and an "Approximate round trip times" line; paste both into
   the chat (PM-0006-12).
AFTER:
1. Open a Windows terminal and run `ping -n 100 <address>` for the address the session sends (that day's address of
   one of the run's CAX21s in `fsn1`). Expected: the run ends with a "Packets" line and an "Approximate round trip
   times" line; paste both into the chat (PM-0006-12).
END

### NOTE N-0006-MV-ROWS
FILE: docs/adr/0006-infrastructure-operations.md
no change: PM-0006-22, -23 and -24 (lines 1278–1280: "present on MV", "RS with LG and MV", "RS, MV") keep MV as AMENDMENT
condition 9 says: MV is now the metrics role F-0006-KEY-RIG defines ("the metrics role: Prometheus on LG's CAX21,
never on RS, inside LG's 80 % rule, standing in for the staging game server", the advisor's reading, 2026-11-03), so
"on MV" means on that role's CAX21 and no cell changes; the plan names which CAX21 carries it when LG has more than one
(the key). PM-0006-17, -19 and -20 ("RS with LG", "RS") name no SH, MV, CX33 or location.
END

### NOTE N-0006-ADDRESS-ROWS
FILE: docs/adr/0006-infrastructure-operations.md
no change: the client-address rows PM-0006-21 (line 1277: "every client address the soak used ... other than the VMs' own") and
PM-0006-31 (line 1288: "the lab clients' public addresses") check logs and crash reports for addresses; they hold
whatever the day's addresses are (the run records them, F-0006-KEY-RIG), so no word changes. Line 1214 ("Every server
mark runs at the 30 Hz baseline") is about the tick rate, not the machine: no change. The "Results log" entry (every
row names "the host or machine, its type, location and CPU model") stands; the day's addresses are recorded by the
key's new sentence and PM-0006-11 / PM-0001-1.
END

### NOTE N-0006-STAYS
FILE: docs/adr/0006-infrastructure-operations.md
no change: line 1300 ("the reference server not orderable or failing: OQ-0006-1 (b) and a new reference server") stays live: VMs
created fresh on the run day mean capacity on the day is not guaranteed, and OQ-0006-1's option (b) route stays the
fallback (AMENDMENT, HZ-OQ1, the advisor's reading, 2026-11-03). The bracket's "Times of day" reason ("a shared host's
neighbours follow the day", line 1233) stays: the 3 times of day are R6-HOST-REC2's mark ("3 times of day", r6 line
669) and RS's dedicated vCPUs still sit on a shared physical host; it names no SH. The "Counts and settings" entry's
"PM-0006-12's 100 echoes" stays (the row still pings 100 times; its dropped second run had no entry there). "What has
no measurable mark here" (part 4, the staging game server, slice 3) stays: OQ-0006-22 (infra-engineer's) asks its plan
and adds no mark.
END

### NOTE N-OTHER-ADRS
FILE: docs/adr/0003-rendering.md
no change: ADR 0002, 0003 and 0005's pass-mark sections name no CX33, SH, MV, CCX13, DS, Nuremberg, nbg1, Helsinki, hel1 or
fixed address (the work list's script found none there). ADR 0003's key keeps MAC as the borrowed MacBook Air and "the
actual machines, named by Mustafa before the look proof test" (line 1095–1096): under MACHINES-TIMING the look proof
test's Safari rows stay on the borrowed Mac in either branch (AMENDMENT 2's limits), so no change there; ADR 0006's
MAC entry points to ADR 0001's MAC sentence (F-0006-KEY-RIG), as AMENDMENT 2 routes it.
END

