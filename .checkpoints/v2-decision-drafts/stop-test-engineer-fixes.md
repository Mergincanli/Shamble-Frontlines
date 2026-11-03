# test-engineer: stop-answer fix blocks (2026-11-03)

### FIX H8
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
60 Hz margin (PM-0001-39, PM-0001-40);
AFTER:
60 Hz margin and the goal budget at 60 Hz (PM-0001-39, PM-0001-40, PM-0001-63);
END

### FIX PM-0001-63-2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  tick), in every server time mark but the rooms-per-VM row PM-0001-27, which runs unpinned as ADR 0006's PM-0006-13
  sets it (eight tick threads cannot each be alone on one of four vCPUs). **SH**, the shared CX33 (4 vCPU, 8 GB) beside it, for the rooms-per-VM mark only (R6-HOST-REC2). **LG**, the load
AFTER:
  tick), in every server time mark but the rooms-per-VM rows PM-0001-27 and PM-0001-63, which run unpinned as ADR
  0006's PM-0006-13 sets it (eight tick threads cannot each be alone on one of four vCPUs). **SH**, the shared CX33
  (4 vCPU, 8 GB) beside it, for the rooms-per-VM mark only (R6-HOST-REC2). **LG**, the load
END

### FIX M16
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  generator: one or more VMs in the same location (CX33s, or a larger plan) running the headless bot clients, which
AFTER:
  generator: one or more VMs in the same location (CX33s: "the CX33 runs the simulated clients", SPIKE-MACHINES,
  Mustafa, 2026-11-03) running the headless bot clients, which
END

### FIX M16-2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  Chrome; MAC runs Safari, the fourth target browser (VISION_V2 §4). **MP**, Mustafa's PC in Brave on Windows: only what
  he judges, and the transport Brave uses; never a measured result. **L1**: the link simulator on simulated time, any
AFTER:
  Chrome; MAC runs Safari, the fourth target browser (VISION_V2 §4). **MP**, Mustafa's PC in Brave on Windows, the
  proof test's test PC (SPIKE-MACHINES, Mustafa, 2026-11-03: "Test PC: my own, in Brave"): only what he judges, and
  the transport Brave uses; never a measured result, so it stands in for none of ML, RD and MAC (the advisor's
  reading, 2026-11-03). **L1**: the link simulator on simulated time, any
END

### FIX S6
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **The sim in the run.** The proof test's throwaway sim has movement, hitscan and bots as protocol clients (the draft,
  "What a spike must prove", lines 927–928). For the 60 Hz margin the sim is the one OQ-0001-2 picks (the draft
  recommends the sim core spike's sim, (a)), recorded with each run. A 30 Hz row run on a stand-in sim proves the
AFTER:
- **The sim in the run.** The proof test's throwaway sim has movement, hitscan and bots as protocol clients (the draft,
  "What a spike must prove", lines 927–928). For the 60 Hz runs the sim is the sim core spike's sim with ADR 0004's S1
  stand-ins in wave 1's early signal, and the real sim at slice 6, where the final 60 Hz call is measured (migration
  step 8; OQ-0001-2, answered by the Advisor (2026-11-03): "(a), as an early signal only. The final 60 Hz call is
  measured again with the real sim at slice 6."), recorded with each run. A 30 Hz row run on a stand-in sim proves the
END

### FIX N4
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  Latency is the round trip, and each direction carries half the profile's latency and a uniform jitter of ± half its
  jitter, so the round trip keeps the profile's mean and range (part 13; OQ-0001-16, the Advisor's, before the test
  starts; the half split of latency is R5-TEST-REC4's, r5-toolchain-quality.md lines 1455–1456, proposed there, and
  the jitter's split and its uniform shape are part 13's proposal): at P2, 50 ± 10 ms each way. A packet's expected
AFTER:
  Latency is the round trip, and each direction carries half the profile's latency, a uniform jitter of ± half its
  jitter and the profile's loss, drawn on its own, so the round trip keeps the profile's mean and range, and at P2
  about 4 % of round trips lose a packet (1 − 0.98 × 0.98 = 3.96 %, arithmetic) (part 13; OQ-0001-16, answered by the
  Advisor (2026-11-03): "Latency and jitter split half each way; loss as P2 defines it"; loss on each direction on its
  own is the advisor's reading, 2026-11-03; the half split of latency is R5-TEST-REC4's, r5-toolchain-quality.md lines
  1455–1456, and the jitter's uniform shape is part 13's proposal): at P2, 50 ± 10 ms each way. A packet's expected
END

### FIX N5
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  human client's "100 ms of simulated latency" is P2 (part 13; OQ-0001-3, for the Advisor): if the Advisor reads it as
  100 ms each way, PM-0001-41 to PM-0001-53 run at a 200 ms round trip and say so. Reordering is what the simulator
AFTER:
  human client's "100 ms of simulated latency" is P2 (part 13; OQ-0001-3, answered by the Advisor (2026-11-03):
  "yes. '100 ms' is P2: 100 ms round trip, ±20 ms jitter, 2 % loss"). Reordering is what the simulator
END

### FIX PM-0001-63-3
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  clauses and the 30-inputs count hold for single-room marks only: the rooms-per-VM row PM-0001-27 runs under ADR 0006's
  PM-0006-13 rules instead (no pinning, its bracket B and LG's 80 % rule, 3 sessions at 3 times of day, steal recorded
  per second and reported, never a validity condition), since eight tick threads cannot each be alone on four vCPUs and
AFTER:
  clauses and the 30-inputs count hold for single-room marks only: the rooms-per-VM rows PM-0001-27 and PM-0001-63
  run under ADR 0006's PM-0006-13 rules instead (no pinning, its bracket B and LG's 80 % rule, 3 sessions at 3 times
  of day, steal recorded per second and reported, never a validity condition), since eight tick threads cannot each be
  alone on four vCPUs and
END

### FIX PM-0001-63-4
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-1 | The rig is named and the measuring unit holds (part 14; a precondition of every server mark) | RS, SH and LG named with plan, CPU model, vCPU count, memory, location and kernel; ML, RD and MAC as ADR 0003 names them; the tick thread's CPU affinity is one vCPU that no other thread of any process uses, in every run but PM-0001-27's, whose unpinned rooms record each process's affinity instead (ADR 0006's PM-0006-13); steal time recorded per second; the number of threads on each core that `lscpu` reports and the map of which vCPUs are SMT siblings, with the tick thread's sibling kept free of the transport runtime where a core carries 2 threads (as ADR 0006's PM-0006-11 records it; proposed) | RS, SH, LG, ML, RD, MAC | a results-log header per machine, written before the first run; the tick thread's affinity and the other threads' affinities read from the operating system at the start and end of every run | checked before the first run and in every run's log. Bites: a single-room run with a second room's tick thread on the same vCPU is flagged invalid (the rooms-per-VM row PM-0001-27 is exempt: it runs unpinned under PM-0006-13) | GLOSSARY, the reference server (lines 225–243) and the measuring unit (lines 244–264); r6 R6-HOST-REC2 (lines 645–650); r3 R3-BUDGET-REC1 (lines 1096–1102) |
AFTER:
| PM-0001-1 | The rig is named and the measuring unit holds (part 14; a precondition of every server mark) | RS, SH and LG named with plan, CPU model, vCPU count, memory, location and kernel; ML, RD and MAC as ADR 0003 names them; the tick thread's CPU affinity is one vCPU that no other thread of any process uses, in every run but those of PM-0001-27 and PM-0001-63, whose unpinned rooms record each process's affinity instead (ADR 0006's PM-0006-13); steal time recorded per second; the number of threads on each core that `lscpu` reports and the map of which vCPUs are SMT siblings, with the tick thread's sibling kept free of the transport runtime where a core carries 2 threads (as ADR 0006's PM-0006-11 records it; proposed) | RS, SH, LG, ML, RD, MAC | a results-log header per machine, written before the first run; the tick thread's affinity and the other threads' affinities read from the operating system at the start and end of every run | checked before the first run and in every run's log. Bites: a single-room run with a second room's tick thread on the same vCPU is flagged invalid (the rooms-per-VM rows PM-0001-27 and PM-0001-63 are exempt: they run unpinned under PM-0006-13's rules) | GLOSSARY, the reference server (lines 225–243) and the measuring unit (lines 244–264); r6 R6-HOST-REC2 (lines 645–650); r3 R3-BUDGET-REC1 (lines 1096–1102) |
END

### FIX PM-0001-63-5
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-2 | The load is the stress scene (parts 3, 14; the setup of the 60 Hz margin) | on every measured tick: 48 soldiers in three teams of 16, each with a connected snapshot-receiving client; 15 big vehicles, 4 of them very large units of at least 20 parts; 300 live projectiles; up to 8 active effects per soldier where the sim has effects; hitscan fire at 700 shots per second (run mean within ±5 %, proposed); portal changes at 1 per second (at 60 Hz on encoding ticks: the key's "Soaks"); for the downstream rows, every client's relevant set holds all 47 other soldiers on at least 99 % of its snapshots ("everyone in view"; proposed share) | RS (counters); LG | the server's own per-tick counters written to the results log and checked by script. A sim that lacks vehicles, projectiles or effects adds stand-in entities that are replicated, hit-tested and filtered by interest at the same counts, their change and spawn rates recorded (proposed: the server-only share must carry the stress scene's load, or its marks cannot fail where the real sim's would); the sim step share's missing parts are stood in as OQ-0001-2 (a) stands them in (ADR 0004's S1: 384 effect rows with the effect step's 1 ms ceiling burned per tick, and the remainder topped up to 2.80 ms at 30 Hz and 0.30 ms at 60 Hz), each stand-in named in the run's record. ES's counts and rates (the key) are checked the same way on every ES run | every run of PM-0001-4 to PM-0001-27 and PM-0001-39. Bites: a load script that stops firing after 10 minutes fails the 700 shots per second | GLOSSARY §3, the stress scene (lines 397–437); R2-BUD-REC6 (line 2412, "a snapshot-receiving client on every soldier"); R2-SYNC-REC5 mark 4 (lines 2186–2187) and R2-SYNC-REC9 (line 2280): 700 shots per second |
AFTER:
| PM-0001-2 | The load is the stress scene (parts 3, 14; the setup of the 60 Hz margin) | on every measured tick: 48 soldiers in three teams of 16, each with a connected snapshot-receiving client; 15 big vehicles, 4 of them very large units of at least 20 parts; 300 live projectiles; up to 8 active effects per soldier where the sim has effects; hitscan fire at 700 shots per second (run mean within ±5 %, proposed); portal changes at 1 per second (at 60 Hz on encoding ticks: the key's "Soaks"); for the downstream rows, every client's relevant set holds all 47 other soldiers on at least 99 % of its snapshots ("everyone in view"; proposed share) | RS (counters); LG | the server's own per-tick counters written to the results log and checked by script. A sim that lacks vehicles, projectiles or effects adds stand-in entities that are replicated, hit-tested and filtered by interest at the same counts, their change and spawn rates recorded (proposed: the server-only share must carry the stress scene's load, or its marks cannot fail where the real sim's would); the sim step share's missing parts are stood in as OQ-0001-2 (a) stands them in (ADR 0004's S1: 384 effect rows with the effect step's 1 ms ceiling burned per tick, and the remainder topped up to 2.80 ms at 30 Hz and 0.30 ms at 60 Hz), each stand-in named in the run's record. ES's counts and rates (the key) are checked the same way on every ES run | every run of PM-0001-4 to PM-0001-27, PM-0001-39 and PM-0001-63. Bites: a load script that stops firing after 10 minutes fails the 700 shots per second | GLOSSARY §3, the stress scene (lines 397–437); R2-BUD-REC6 (line 2412, "a snapshot-receiving client on every soldier"); R2-SYNC-REC5 mark 4 (lines 2186–2187) and R2-SYNC-REC9 (line 2280): 700 shots per second |
END

### FIX N6
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-3 | The network profiles are what they say (part 13) | per profile P1 to P4 and B1, on L1 and on L2, in each direction: measured loss, mean one-way delay and jitter over 100,000 packets within 5 % (relative) of the profile's per-direction values (half its latency, a uniform jitter of ± half its jitter: the key's split, OQ-0001-16), and the round trip's mean within 5 % of the profile's latency; on L1, the same seed and profile give an identical packet-fate log on two runs; P0 on L2 is the real path, its delay and jitter recorded; for MP's session, the server-measured RTT median within 100 ± 5 ms at P2, his own base RTT recorded | L1; L2 on RS with LG, ML and MP | the simulator's packet-fate log against the profile; on L2, the injected delay in each direction set to half of the profile's RTT minus the path's measured base RTT (median of 10 s of pings at session start; proposed), each direction's delays logged at the impairment layer | once per profile and layer before any mark under that profile counts, and at the start of every L2 session | r5 R5-TEST-REC4's mark (lines 1469–1473; ADR 0002's simulator, applied here to L2: proposed); r2 profiles (lines 1987–1992); R2-TRN-REC4 mark 2's ±5 ms (line 2694) for MP's RTT (proposed) |
AFTER:
| PM-0001-3 | The network profiles are what they say (part 13) | per profile P1 to P4 and B1, on L1 and on L2, in each direction: measured loss, mean one-way delay and jitter over 100,000 packets within 5 % (relative) of the profile's per-direction values (its loss, half its latency and a uniform jitter of ± half its jitter: the key's split, OQ-0001-16, answered by the Advisor, 2026-11-03), and the round trip's mean within 5 % of the profile's latency; on L1, the same seed and profile give an identical packet-fate log on two runs; P0 on L2 is the real path, its delay and jitter recorded; for MP's session, the server-measured RTT median within 100 ± 5 ms at P2, his own base RTT recorded | L1; L2 on RS with LG, ML and MP | the simulator's packet-fate log against the profile; on L2, the injected delay in each direction set to half of the profile's RTT minus the path's measured base RTT (median of 10 s of pings at session start; proposed), each direction's delays logged at the impairment layer | once per profile and layer before any mark under that profile counts, and at the start of every L2 session | r5 R5-TEST-REC4's mark (lines 1469–1473; ADR 0002's simulator, applied here to L2: proposed); r2 profiles (lines 1987–1992); R2-TRN-REC4 mark 2's ±5 ms (line 2694) for MP's RTT (proposed) |
END

### FIX RC4
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-59 | A panic on the tick thread ends the match and the process (part 11; OQ-0001-18) | after a panic injected into the sim step: the process exits with a non-zero code within 1 s of the panic (a plain limit; proposed in the draft's failure-mode row); every client gets `RoomEnded` with a crash reason or has its connection closed within that second (0 left open); the match record is written and marked crashed; the supervisor's stand-in logs the exit; the report reaches the error log (ADR 0006 part 10's endpoint once it exists) | RS with LG | 10 injections (proposed), each in its own S30 room at a random tick after warm-up; the exit code and time from the supervisor's stand-in, each client's last message and close from LG's log | every injection. Bites: a tick thread whose panic leaves the process running (the loop without its top-level catch, under `panic = "unwind"`: the thread ends and the room holds its connections and sends nothing) | the draft, part 11 (a panic on the tick thread is fatal to the room) and "Failure modes", row "A panic on the room's tick thread" (its 1 s, proposed there); r2 R2-ROOM-F8 (lines 1459–1462: an uncaught panic "will exit" only its own thread); the Rust review of 2026-11-02, finding 3; if OQ-0001-18 picks its option (b), this row is rewritten for the resume |
AFTER:
| PM-0001-59 | A panic on the tick thread ends the match and the process (part 11; OQ-0001-18, answered (a) by the Advisor, 2026-11-03) | after a panic injected into the sim step: the process exits with a non-zero code within 1 s of the panic (a plain limit; proposed in the draft's failure-mode row); every client gets `RoomEnded` with a crash reason or has its connection closed within that second (0 left open); the match record is written and marked crashed, and it and the crash report are kept (ROOM-CRASH, the Advisor, 2026-11-03: "end the match, as proposed, and keep the crash and match records"; kept under ADR 0006 part 11's retention: the advisor's reading, 2026-11-03); the supervisor's stand-in logs the exit; the report reaches the error log (ADR 0006 part 10's endpoint once it exists) | RS with LG | 10 injections (proposed), each in its own S30 room at a random tick after warm-up; the exit code and time from the supervisor's stand-in, each client's last message and close from LG's log; the match record and the crash report read back after the last injection | every injection. Bites: a tick thread whose panic leaves the process running (the loop without its top-level catch, under `panic = "unwind"`: the thread ends and the room holds its connections and sends nothing) | the draft, part 11 (a panic on the tick thread is fatal to the room) and "Failure modes", row "A panic on the room's tick thread" (its 1 s, proposed there); r2 R2-ROOM-F8 (lines 1459–1462: an uncaught panic "will exit" only its own thread); the Rust review of 2026-11-02, finding 3; OQ-0001-18 (a) |
END

### FIX H7
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| **The 60 Hz margin** | | | | | | |
| PM-0001-39 | The 60 Hz margin (part 2) | at a 60 Hz tick, every number from the same run: server tick time p99 at most 6.25 ms against [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) at 60 Hz, 8.33 ms (a 60 Hz margin of 1 − 6.25 ÷ 8.33 = 25 %); sim step time p99 and server-only work p99 at most 4.17 ms each; max under 16.67 ms; 0 overruns and 0 dropped time; encoding p99 at most 50 µs per client per snapshot; and PM-0001-17's hash check holding across tick 65,536 (reached at 18.2 minutes: 65,536 ÷ 60 ÷ 60) | one vCPU of RS | PM-0001-4's setup at a 60 Hz tick (SS at N = 48 with a snapshot-receiving client on every soldier, P2, S60, re-flood ticks at PM-0001-2's rate, each on an encoding tick: the key's "Soaks"), with 30 snapshots a second, encoded on every second tick (part 2; at every tick they would be 60 a second, about 286.1 kbps: r2-netcode.md line 1778), and with the sim OQ-0001-2 picks (recorded); the server tick time measured directly, never summed from its phases | 3 runs, each a whole measurement in itself (proposed reading of "in one run": every number of a run comes from that run, none is combined across runs), each passes; V-S; rule B-S | the draft, part 2 (the tick rate rule), the budgets table ("Budgets for the netcode proof test"), row "The 60 Hz margin run (decides 60 Hz)", and "What a spike must prove" item 3; GLOSSARY §8, [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) (decision map J1); TICK-60-HEADROOM; r2 R2-BUD-REC6 (lines 2413–2415) |
| PM-0001-40 | The research's other 60 Hz marks (part 2) | R1-TICK-REC2 P1 at 60 Hz (sim step time p99 at most 4.17 ms at N = 48, max at most 16.67 ms, 0 overruns, 36,000 ticks, 3 runs: ADR 0004's sim core spike row, run at 60 Hz); P2 at 60 Hz (the client's sim share, p99 at most 6.67 ms in Wasm in a dedicated Worker on ML while the page holds 30+ fps on Low: ADR 0004's row); P3 at 60 Hz (12 ticks of replay at most 1 ms p99 on ML: PM-0001-44's method). 60 Hz needs all three and PM-0001-39 | RS (P1); ML (P2, P3) | as each owning row says | as each owning row says | r1 R1-TICK-REC2 P1–P3 (lines 2044–2060); the draft, part 2 ("every 60 Hz mark the research sets, as written") |
AFTER:
| **The 60 Hz margin and the goal budget at 60 Hz** | | | | | | |
| PM-0001-39 | The 60 Hz margin (part 2) | at a 60 Hz tick, every number from the same run: server tick time p99 at most 6.25 ms against [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) at 60 Hz, 8.33 ms (a 60 Hz margin of 1 − 6.25 ÷ 8.33 = 25 %); sim step time p99 and server-only work p99 at most 4.17 ms each; max under 16.67 ms; 0 overruns and 0 dropped time; encoding p99 at most 50 µs per client per snapshot; and PM-0001-17's hash check holding across tick 65,536 (reached at 18.2 minutes: 65,536 ÷ 60 ÷ 60) | one vCPU of RS | PM-0001-4's setup at a 60 Hz tick (SS at N = 48 with a snapshot-receiving client on every soldier, P2, S60, re-flood ticks at PM-0001-2's rate, each on an encoding tick: the key's "Soaks"), with 30 snapshots a second, encoded on every second tick (part 2; at every tick they would be 60 a second, about 286.1 kbps: r2-netcode.md line 1778), and with the sim the key's "The sim in the run" names (recorded); the server tick time measured directly, never summed from its phases | 3 runs, each a whole measurement in itself (proposed reading of "in one run": every number of a run comes from that run, none is combined across runs), each passes; V-S; rule B-S | the draft, part 2 (the tick rate rule), the budgets table ("Budgets for the netcode proof test"), row "The 60 Hz margin run", and "What a spike must prove" item 3; GLOSSARY §8, [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) (decision map J1); TICK-60-HEADROOM (confirmed by Mustafa, 2026-11-03); r2 R2-BUD-REC6 (lines 2413–2415) |
| PM-0001-40 | The research's other 60 Hz marks (part 2) | R1-TICK-REC2 P1 at 60 Hz (sim step time p99 at most 4.17 ms at N = 48, max at most 16.67 ms, 0 overruns, 36,000 ticks, 3 runs: ADR 0004's sim core spike row, run at 60 Hz); P2 at 60 Hz (the client's sim share, p99 at most 6.67 ms in Wasm in a dedicated Worker on ML while the page holds 30+ fps on Low: ADR 0004's row); P3 at 60 Hz (12 ticks of replay at most 1 ms p99 on ML: PM-0001-44's method). 60 Hz needs all three, PM-0001-39 and PM-0001-63 | RS (P1); ML (P2, P3) | as each owning row says | as each owning row says | r1 R1-TICK-REC2 P1–P3 (lines 2044–2060); the draft, part 2 ("every 60 Hz mark the research sets, as written") |
END

### FIX PM-0001-63
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| **The human client at P2** | | | | | | |
AFTER:
| PM-0001-63 | The goal budget holds at the rooms per VM measured at 60 Hz (part 2; TICK-60-HEADROOM's addition; ADR 0006 part 8) | k60, the most rooms of N = 48 at a 60 Hz tick for which every room meets R2-ROOM-REC2 marks 1 and 2 at 60 Hz: server tick time p99 at most 8.33 ms ([the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) at 60 Hz), max under 16.67 ms and 0 overruns, and each room's p99 within 10 % of the single-room figure (PM-0006-14's counting rule, run at 60 Hz). Then the goal fleet at k60 by ADR 0006 part 8's formula, ceil(20 ÷ k60) + 1 CCX23 at 86.49 EUR a month plus the staging CX33 (8.99 EUR) and the match records (6.49 EUR), with part 8's 19–24 USD and Sentry beside it, costs at most the goal budget as it stands when the call is made: at slice 6, the figure OQ-0006-19 fixes, and there it decides; in wave 1's early signal the cost is reported beside part 8's provisional figures (about 361 EUR at 8 rooms per VM, about 534 EUR at 4) and decides nothing (the advisor's reading, 2026-11-03). Reported beside it: PM-0006-14's count at 30 Hz on the same machine | RS (the CCX23 that part 8's goal fleet prices: the same machine), with LG | PM-0006-14's search at a 60 Hz tick, N = 48 only: in one session with bracket B (its lone room at 60 Hz), k = 4, then 6, 8 and up by 2 until a k fails, then the odd k between the last pass and the first fail; if k = 4 fails, 3, 2, 1; then the count and count + 1 again at the other 2 times of day. Each room in SS with a snapshot-receiving client on every soldier, 30 snapshots a second encoded on every second tick (part 2), P2 on LG; k room processes under the supervisor, no CPU pinning; each run 2 minutes of warm-up, then 20 minutes measured, 72,000 ticks (20 × 60 × 60), the p99 the 71,280th. The sim is the key's "The sim in the run": in wave 1 its step is padded by a busy-wait to the sim step share's ceiling at 60 Hz, 4.17 ms per tick, as PM-0006-14 pads at 30 Hz (proposed: a lower bound for any sim inside its share, provisional until slice 6), and at slice 6 the real sim runs as PM-0006-14's slice-6 count runs it. The cost by script from k60 and part 8's prices, the arithmetic shown | the figure is the lowest count of the 3 sessions (PM-0006-14's rule); validity as PM-0006-13 (bracket B, LG's 80 % rule, steal recorded per second and reported, never V-S); a run plan with every session's VM-hours first, as PM-0006-14's; in the 60 Hz run of wave 1 and again at slice 6. Bites: a busy-wait of 20 % of the single-room p99 planted in every tick of one room must fail that room's row at k = 4 (PM-0006-13's bite, at 60 Hz); the cost script fed k60 = 1 must report 21 CCX23 and 1,831.77 EUR (21 × 86.49 + 8.99 + 6.49) and "does not hold" against both provisional figures | TICK-60-HEADROOM, confirmed by Mustafa (2026-11-03) "with one addition: 60 Hz also needs the goal budget to hold at the rooms per VM measured at 60 Hz"; ADR 0006 part 8 (the goal fleet: A-5's peak of 20 rooms plus one VM for draining, its prices and its two provisional figures), PM-0006-13 and PM-0006-14 (the setup and the counting rule) and OQ-0006-19; r2 R2-ROOM-REC2 marks 1 and 2 (lines 2775–2777: the budget is "50 % of the tick period chosen by the tick-rate unit"); r6 A-2 (lines 516–518: "a density to measure at the tick rate R1-TICK-REC2's rule picks"); the advisor's reading of 2026-11-03 (the goal budget as it stands at the call; what "holds" means) |
| **The human client at P2** | | | | | | |
END

### FIX RW10
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-48 | Hits register as the shooter saw them (part 8; decides OQ-0001-7) | a scripted shooter at TC's sprinting target under P0 to P3: of the shots whose ray hits the target's volume in the shooter's view (the state the Worker published for the command's view time), at least 99 % register on the server up to P2 and 95 % at P3; false hits (a miss in the view counted as a hit) below 0.5 %; judged with the cap above every honest rewind of the run (the run's largest measured rewind rounded up to a whole number of ticks, recorded; proposed: at P2 part 8's honest rewind is 233–383 ms, so a 200 or 250 ms cap would fail the mark by design, and what each candidate cap costs is PM-0001-49's record); recorded: per shot whether the view time was accepted or replaced by the server's estimate (part 8's estimate with the buffer wait and half a tick for the blend), and the share of honest shots replaced, per profile | L1; L2 with LG's shooter | 2,000 shots per profile (proposed: about 20 misses at the 99 % mark, so a handful of shots does not decide the run) | L1: 3 seeds; L2: 1 run per profile. Bites: a rewind that leaves out the interpolation delay (at 9 m/s, 100 ms is about 0.9 m); an estimate without the buffer wait (with a buffer of 2 ticks or more, honest view times miss it by more than the 33.3 ms tolerance and are replaced: the netcode review's finding 1) | r2 R2-SYNC-REC5 mark 1 (lines 2179–2181); the draft, part 8 (the estimate and its arithmetic) and OQ-0001-7; the netcode review of 2026-11-02, finding 1 |
AFTER:
| PM-0001-48 | Hits register as the shooter saw them (part 8; decides OQ-0001-7) | a scripted shooter at TC's sprinting target under P0 to P3: of the shots whose ray hits the target's volume in the shooter's view (the state the Worker published for the command's view time), at least 99 % register on the server up to P2 and 95 % at P3; false hits (a miss in the view counted as a hit) below 0.5 %; judged with the cap above every honest rewind of the run (the run's largest measured rewind rounded up to a whole number of ticks, recorded; proposed: the cap OQ-0001-5's rule sets, the honest rewind's p99 at P2 rounded up, may clip up to 1 % of honest shots at P2 by its definition, which alone could spend the 99 % mark's whole margin, so this mark judges registration apart from the cap, and what that cap clips is PM-0001-49's record); recorded: per shot whether the view time was accepted or replaced by the server's estimate (part 8's estimate with the buffer wait and half a tick for the blend), and the share of honest shots replaced, per profile | L1; L2 with LG's shooter | 2,000 shots per profile (proposed: about 20 misses at the 99 % mark, so a handful of shots does not decide the run) | L1: 3 seeds; L2: 1 run per profile. Bites: a rewind that leaves out the interpolation delay (at 9 m/s, 100 ms is about 0.9 m); an estimate without the buffer wait (with a buffer of 2 ticks or more, honest view times miss it by more than the 33.3 ms tolerance and are replaced: the netcode review's finding 1) | r2 R2-SYNC-REC5 mark 1 (lines 2179–2181); the draft, part 8 (the estimate and its arithmetic) and OQ-0001-7; the netcode review of 2026-11-02, finding 1 |
END

### FIX RW11
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-49 | The rewind cap holds; what each cap clips is known (part 8; feeds OQ-0001-5; decides R2-XU-OQ2) | 100 % of shots resolved with a rewind at most the cap, honest and cheat clients, P0 to P4; per candidate cap and profile, the share of shots the cap clips and the registration PM-0001-48 measures recorded, the candidates being 200 ms, 250 ms and the honest rewind's measured p99 at P2 (OQ-0001-5's options (a) to (c)); the honest rewind, how far behind the server's present the client's view is when its command runs (median, p99, max) per profile, with each shot's parts recorded: the RTT, the announced delay, the buffer wait, the blend fraction and the wait from the drawn frame to the tick that sent the command (part 5: the page posts once per frame, each tick takes the newest post, so 0 to one frame) | L1; L2 with LG | PM-0001-48's shots, run once per cap; the cheat clients of PM-0001-50 among them | as PM-0001-48. Bites: the cap applied to the estimate only, not to the final rewind; a client that delays every pong by 150 ms (part 8's accepted risk) must still meet the cap on 100 % of its shots | r2 R2-SYNC-REC5 mark 6 (lines 2191–2192), the cap's text (lines 2154–2159); R2-BUD-REC1 (lines 2320–2323); the draft, part 8 (the rewind cap and the share it clips; the estimate with the buffer wait and the blend) and OQ-0001-5; the netcode review of 2026-11-02, finding 1 (e), and the security review, finding 6 |
AFTER:
| PM-0001-49 | The rewind cap holds; what it clips is known (part 8; sets OQ-0001-5's value; decides R2-XU-OQ2) | 100 % of shots resolved with a rewind at most the cap, honest and cheat clients, P0 to P4; the cap set by OQ-0001-5's rule, answered by Mustafa (2026-11-03): "the cap covers the honest rewind the proof test measures at P2, rounded up; beyond the cap, the victim's view wins": the honest rewind's measured p99 at P2, rounded up to a whole number of ticks at the run's tick rate, its value recorded and the run's largest rewind reported beside it (the advisor's reading, 2026-11-03); per profile, the share of shots that cap clips and the registration PM-0001-48 measures with it recorded; the honest rewind, how far behind the server's present the client's view is when its command runs (median, p99, max) per profile, with each shot's parts recorded: the RTT, the announced delay, the buffer wait, the blend fraction and the wait from the drawn frame to the tick that sent the command (part 5: the page posts once per frame, each tick takes the newest post, so 0 to one frame) | L1; L2 with LG | PM-0001-48's shots: the honest rewind read from PM-0001-48's runs (its test cap above every honest rewind), then a run with the rule's cap; the cheat clients of PM-0001-50 among them | as PM-0001-48. Bites: the cap applied to the estimate only, not to the final rewind; a client that delays every pong by 150 ms (part 8's accepted risk) must still meet the cap on 100 % of its shots | r2 R2-SYNC-REC5 mark 6 (lines 2191–2192), the cap's text (lines 2154–2159); R2-BUD-REC1 (lines 2320–2323); the draft, part 8 (the rewind cap and the share it clips; the estimate with the buffer wait and the blend) and OQ-0001-5 (its rule answered by Mustafa, 2026-11-03; its value set when this row reports); the netcode review of 2026-11-02, finding 1 (e), and the security review, finding 6 |
END

### FIX N5-2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| PM-0001-53 | Mustafa's verdict: prediction, reconciliation and lag-compensated hits look correct at P2 (parts 4, 7, 8) | every step of the netcode checklist below judged "achieved"; 0 left "not yet" | MP, Brave on Windows | the netcode checklist below, at the profile OQ-0001-3 confirms (P2 until then), beside the measurable marks PM-0001-41 to PM-0001-52 and PM-0001-60 to PM-0001-62 | one session; a step marked "not yet" goes back to the draft and is shown again | GLOSSARY §8, the netcode proof test (brief §2); plan rule 12; the draft, "What a spike must prove" item 4 |
AFTER:
| PM-0001-53 | Mustafa's verdict: prediction, reconciliation and lag-compensated hits look correct at P2 (parts 4, 7, 8) | every step of the netcode checklist below judged "achieved"; 0 left "not yet" | MP, Brave on Windows | the netcode checklist below, at P2 (OQ-0001-3, answered by the Advisor, 2026-11-03), beside the measurable marks PM-0001-41 to PM-0001-52 and PM-0001-60 to PM-0001-62 | one session; a step marked "not yet" goes back to the draft and is shown again | GLOSSARY §8, the netcode proof test (brief §2); plan rule 12; the draft, "What a spike must prove" item 4 |
END

### FIX S7
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
(PM-0001-54, PM-0001-55) only when its condition holds; PM-0001-56 when its record is complete; PM-0001-39 and
PM-0001-40 only where OQ-0001-2 places the 60 Hz margin run (under its option (c) they move to slice 6). A failing run is
AFTER:
(PM-0001-54, PM-0001-55) only when its condition holds; PM-0001-56 when its record is complete. The 60 Hz group
(PM-0001-39, PM-0001-40, PM-0001-63) is not part of that pass: it runs in the 60 Hz run after the sim core spike, an
early signal that decides nothing, and again at slice 6 on the real sim, where it decides (migration step 8;
OQ-0001-2, answered by the Advisor (2026-11-03): "(a), as an early signal only. The final 60 Hz call is measured again
with the real sim at slice 6."; the proof test closing at 30 Hz without it is the advisor's reading, 2026-11-03). A
failing run is
END

### FIX H9
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
names, unless a route below names it (the netcode review of 2026-11-02, finding 6: a downstream or a memory failure
is not a tick-rate failure); PM-0001-39 or PM-0001-40 failing changes nothing, and the tick stays at 30 Hz; a
AFTER:
names, unless a route below names it (the netcode review of 2026-11-02, finding 6: a downstream or a memory failure
is not a tick-rate failure); PM-0001-39, PM-0001-40 or PM-0001-63 failing at slice 6 changes nothing else, and the
tick stays at 30 Hz, while in the wave-1 early signal neither a pass nor a fail decides anything: a pass starts no
60 Hz work before slice 6, and a fail closes nothing (the advisor's reading, 2026-11-03); a
END

### FIX RW11-2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- OQ-0001-6 (command timing) and the WebTransport crate pick have no pass number: the draft names the measures, not a
  rule, so PM-0001-56 records them for netcode-architect's pick.
AFTER:
- OQ-0001-6 (command timing) and the WebTransport crate pick have no pass number: PM-0001-56 records their measures for
  netcode-architect's pick; for OQ-0001-6 it takes, of the options that pass their marks, the smallest input buffer, run
  on arrival counting as none (REWIND-CAP, Mustafa, 2026-11-03: "Aim for the smallest input buffer that holds"; the
  advisor's reading, 2026-11-03).
END

### FIX PM-0001-63-6
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- Rooms-per-VM density, the firewall checks, egress per match-hour, the spike's bill, logging cost, the metrics and
  the development certificates are ADR 0006's operations marks inside this spike; the server loop's jitter and catch-up
AFTER:
- Rooms-per-VM density at 30 Hz (at 60 Hz it is PM-0001-63, above), the firewall checks, egress per match-hour, the
  spike's bill, logging cost, the metrics and the development certificates are ADR 0006's operations marks inside this
  spike; the server loop's jitter and catch-up
END

### FIX RW12
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
7. At P2, with the rewind cap at the honest rewind's p99 the proof test measured at P2 (PM-0001-49; the overlay names
   it, so the cap clips almost no honest shot), shoot the sprinting target for two minutes. Expected: when his crosshair
   is on the target as he sees it, the shot hits; when it is off, it misses. The shot (flash, tracer) is instant; the
   hit marker comes a moment later, when the server confirms ("the shot feels instant; hit feedback arrives when the hit
   is confirmed", LOOP_V2 §7).
AFTER:
7. At P2, with the rewind cap that OQ-0001-5's rule sets (the honest rewind's p99 the proof test measured at P2,
   rounded up to a whole number of ticks: PM-0001-49; the overlay names it, so the cap clips almost no honest shot),
   shoot the sprinting target for two minutes. Expected: when his crosshair is on the target as he sees it, the shot
   hits; when it is off, it misses. The shot (flash, tracer) is instant; the hit marker comes a moment later, when the
   server confirms ("the shot feels instant; hit feedback arrives when the hit is confirmed", LOOP_V2 §7).
END

### FIX RW12-2
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
10. At P2, repeat step 7 with the rewind cap at 200 ms, then at 250 ms, then again at step 7's measured cap (the overlay
    names the cap). Expected: with 200 and 250 ms, shots that looked like hits but missed, since at P2 an honest shot
    needs 233–383 ms of rewind (part 8) and both caps clip most of them; with the measured cap, hits as in step 7.
    Record: what he noticed with each cap (for OQ-0001-5's later Feel Lab weighing; no pick here).
11. At P2, run from open ground into cover while bots shoot at him, 10 runs with each of step 10's three caps. Expected:
    being hit after reaching cover on his own screen is rare, and never long after. Record: how often it felt unfair
    with each cap (OQ-0001-5, the victim's side).
12. At P2, while he moves, trigger the one-second loss burst (B1) from the overlay. Expected: during the burst the
    other soldiers stop where they are (held, not flying off) and his own soldier keeps responding; within half a
    second after it, the world catches up. His own soldier may be corrected once, because the repeated commands cover
    only 8 ticks (267 ms) of the second: a glide back, or a jump if the error is over 2 m, and then nothing more.
13. Set P3 (150 ms), then P4 (200 ms), two minutes each in the brawl and at the target course. Expected: still
    playable; his own soldier and gun still respond at once; others move a little less smoothly and hits land a little
    less often at P3. P4 is not held to P2's feel (P3 and P4 test that play degrades gracefully, r2-netcode.md line
    1991).
14. With the WebSocket fallback forced (UDP blocked) at P2, repeat steps 3, 5 and 7 briefly. Expected: playing within
    3 s; it plays like those steps, except that other soldiers may stall for a moment when a packet is lost (the
    fallback's known cost, the draft's Consequences).
15. The verdict at P2 on WebTransport. Record: "achieved" or "not yet" for each of prediction (steps 3 and 4),
    reconciliation (steps 4 and 12) and lag-compensated hits (steps 7 to 9).
AFTER:
10. At P2, run from open ground into cover while bots shoot at him, 10 runs with step 7's cap. Expected: being hit
    after reaching cover on his own screen is rare, and never long after. Record: how often it felt unfair (the window
    inside the cap; beyond the cap the victim's view wins: OQ-0001-5's rule, answered by Mustafa, 2026-11-03).
11. At P2, while he moves, trigger the one-second loss burst (B1) from the overlay. Expected: during the burst the
    other soldiers stop where they are (held, not flying off) and his own soldier keeps responding; within half a
    second after it, the world catches up. His own soldier may be corrected once, because the repeated commands cover
    only 8 ticks (267 ms) of the second: a glide back, or a jump if the error is over 2 m, and then nothing more.
12. Set P3 (150 ms), then P4 (200 ms), two minutes each in the brawl and at the target course. Expected: still
    playable; his own soldier and gun still respond at once; others move a little less smoothly and hits land a little
    less often at P3. P4 is not held to P2's feel (P3 and P4 test that play degrades gracefully, r2-netcode.md line
    1991).
13. With the WebSocket fallback forced (UDP blocked) at P2, repeat steps 3, 5 and 7 briefly. Expected: playing within
    3 s; it plays like those steps, except that other soldiers may stall for a moment when a packet is lost (the
    fallback's known cost, the draft's Consequences).
14. The verdict at P2 on WebTransport. Record: "achieved" or "not yet" for each of prediction (steps 3 and 4),
    reconciliation (steps 4 and 11) and lag-compensated hits (steps 7 to 9).
END

### FIX Q3
FILE: docs/adr/0003-rendering.md
BEFORE:
  researched). PM-0003-18 to -21 run S on ML beside their cold set. Which of the two gates is OQ-0003-20; until it is
  answered the soaked set gates and the cold set is reported beside it (its recommended option, held as the interim).
AFTER:
  researched). PM-0003-18 to -21 run S on ML beside their cold set. The soaked set gates and the cold set is reported
  beside it (OQ-0003-20, answered by the Advisor (2026-11-03): "judge after 15 minutes of play"; its option (a)).
END

### FIX F3
FILE: docs/adr/0003-rendering.md
BEFORE:
| PM-0003-44 | The very large units drop no frame (part 7's presented-frame count; VISION_V2 §6.9; feeds OQ-0003-17) | per run of 600 presented frames with the four very large units and their visual effects on screen: the frames whose interval passes 1.5 refresh periods, 25 ms on a 60 Hz display (1.5 × 16.7), counted. Gated at the share OQ-0003-17 sets; until it is answered, reported, with 0 of 600 as the proposed reading. On ML at Low the same count reported against a 33.3 ms period (intervals over 1.5 × 33.3 = 50 ms) | RD at High, Chrome on WebGPU, on a 60 Hz display (its rate recorded); ML at Low, Chrome on WebGPU (reported) | BS with the units and their effects on screen, mode P (normal vsync); each frame's interval from successive animation-frame timestamps (the loop follows the display, R3-BUDGET-F2, lines 545–549; that timestamp has "a minimal precision of 1 millisecond", R3-MEASURE-F7, lines 641–646, well inside the 8.3 ms between one refresh period and 1.5) | H (presented), V. Bites: a 20 ms busy-wait on every 60th frame must be counted 10 of 10 per 600 frames | R3-CHAR-REC4 (lines 1088–1092; it stays adopted); part 7's spectacle budget; OQ-0003-17 |
AFTER:
| PM-0003-44 | The very large units miss a refresh on at most 6 of 600 frames (part 7's presented-frame count; VISION_V2 §6.9; OQ-0003-17's reading) | per run of 600 presented frames with the four very large units and their visual effects on screen: the frames whose interval passes 1.5 refresh periods, 25 ms on a 60 Hz display (1.5 × 16.7), counted. Gated: at most 6 of 600 such frames per run (OQ-0003-17, answered by the Advisor and confirmed by Mustafa (2026-11-03): "at most 6 of 600 presented frames may miss a refresh with the very large units on screen (recommended desktop, High); the p95 rows stay as diagnostics"); the p95 frame rows are diagnostics for this reading and keep gating their own needs (the advisor's reading, 2026-11-03). On ML at Low the same count reported against a 33.3 ms period (intervals over 1.5 × 33.3 = 50 ms) | RD at High, Chrome on WebGPU, on a 60 Hz display (its rate recorded); ML at Low, Chrome on WebGPU (reported) | BS with the units and their effects on screen, mode P (normal vsync); each frame's interval from successive animation-frame timestamps (the loop follows the display, R3-BUDGET-F2, lines 545–549; that timestamp has "a minimal precision of 1 millisecond", R3-MEASURE-F7, lines 641–646, well inside the 8.3 ms between one refresh period and 1.5) | H (presented), V. Bites: a 20 ms busy-wait on every 60th frame must be counted 10 of 10 per 600 frames, which fails the 6 of 600 | R3-CHAR-REC4 (lines 1088–1092; it stays adopted); part 7's spectacle budget; OQ-0003-17 (answered by the Advisor, confirmed by Mustafa, 2026-11-03) |
END

### FIX F4+Q4
FILE: docs/adr/0003-rendering.md
BEFORE:
its condition holds; a "reported" mark (PM-0003-9, -14, -16, -31, -42, and PM-0003-44 until OQ-0003-17 sets its
share) when its record is complete; the cold set of PM-0003-18 to -21 is reported beside the soaked set that gates
until OQ-0003-20 is answered. A failing run is
AFTER:
its condition holds; a "reported" mark (PM-0003-9, -14, -16, -31, -42) when its record is complete; the cold set of
PM-0003-18 to -21 is reported beside the soaked set that gates (OQ-0003-20, answered). A failing run is
END

### FIX S13+H17
FILE: docs/adr/0004-simulation-core.md
BEFORE:
  with [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin)'s test, as TICK-60-HEADROOM rules (plan; GL §8):
  the sim's share there is read from ADR 0001's one 60 Hz run (OQ-0001-2), never summed from parts. No 60 Hz result is
  needed for the 30 Hz pass.
AFTER:
  with [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin)'s test, as TICK-60-HEADROOM rules (plan; GL §8;
  confirmed by Mustafa, 2026-11-03): the sim's share there is read from the same run, never summed from parts, in each
  of ADR 0001's two 60 Hz runs: wave 1's early signal on the sim core spike's sim with S1's stand-ins (OQ-0001-2 (a),
  answered by the Advisor, 2026-11-03: "as an early signal only"), which decides nothing, and the run at slice 6 on the
  real crate, which decides (the advisor's reading, 2026-11-03). No 60 Hz result is needed for the 30 Hz pass.
END

### FIX N10
FILE: docs/adr/0004-simulation-core.md
BEFORE:
  VISION_V2 §4's "about 100 ms of ping" (decision map J6 (3); OQ-0001-3).
AFTER:
  VISION_V2 §4's "about 100 ms of ping" (decision map J6 (3); OQ-0001-3, answered by the Advisor, 2026-11-03).
END

### FIX F3-2
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-8 | The browser runs the whole sim within [the client's sim share](../research/GLOSSARY.md#the-clients-sim-share) (S2; part 6) | p99 sim step time ≤ 13.33 ms (40 % of 33.33 ms) in Wasm in a dedicated Worker at the N that R1-TICK-OQ7 sets, while the page meets ADR 0003's minimum-laptop row over the whole run (frame time median ≤ 28 ms and p95 ≤ 33.3 ms, mode U: the reading of "holds 30+ fps on Low", since R1-TICK-REC2 P2 leaves the render budget to R3; it is the drafts' one reading of P2's "30+ fps", so ADR 0002's PM-0002-28 cites this row and sets none of its own, and a presented-frame count on ML, if OQ-0003-17 asks for one, is ADR 0003's PM-0003-44); until R1-TICK-OQ7 is answered: reported at N = 12, 24, 36 and 48, no pass | ML: Chrome and Firefox on Windows 11 (Edge reported; Safari on MAC reported, no budget row) | T on SS in the boundary spike's Worker with the sim core spike's sim; the page draws the sim's output at Low (ADR 0003's renderer if it exists by then, else the boundary spike's page: recorded which); step times from the Worker's timer around the step, isolation flag on; frame times from the page's harness, mode U (ADR 0003: vsync off) | T, V; rule B | R1-TICK-REC2 P2 (lines 2054–2057); R1-TICK-OQ7 (lines 2667–2670); R3-BUDGET-REC2's frame rows (r3 lines 1123–1124); GL lines 214–218 |
AFTER:
| PM-0004-8 | The browser runs the whole sim within [the client's sim share](../research/GLOSSARY.md#the-clients-sim-share) (S2; part 6) | p99 sim step time ≤ 13.33 ms (40 % of 33.33 ms) in Wasm in a dedicated Worker at the N that R1-TICK-OQ7 sets, while the page meets ADR 0003's minimum-laptop row over the whole run (frame time median ≤ 28 ms and p95 ≤ 33.3 ms, mode U: the reading of "holds 30+ fps on Low", since R1-TICK-REC2 P2 leaves the render budget to R3; it is the drafts' one reading of P2's "30+ fps", so ADR 0002's PM-0002-28 cites this row and sets none of its own, and the presented-frame count on ML at Low is ADR 0003's PM-0003-44, reported there, which gates on RD at High (OQ-0003-17, answered by the Advisor, confirmed by Mustafa, 2026-11-03)); until R1-TICK-OQ7 is answered: reported at N = 12, 24, 36 and 48, no pass | ML: Chrome and Firefox on Windows 11 (Edge reported; Safari on MAC reported, no budget row) | T on SS in the boundary spike's Worker with the sim core spike's sim; the page draws the sim's output at Low (ADR 0003's renderer if it exists by then, else the boundary spike's page: recorded which); step times from the Worker's timer around the step, isolation flag on; frame times from the page's harness, mode U (ADR 0003: vsync off) | T, V; rule B | R1-TICK-REC2 P2 (lines 2054–2057); R1-TICK-OQ7 (lines 2667–2670); R3-BUDGET-REC2's frame rows (r3 lines 1123–1124); GL lines 214–218 |
END

### FIX S13-2
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| PM-0004-71 | The sim step share at 60 Hz, for the 60 Hz margin only (S1, S2; part 6; TICK-60-HEADROOM) | inside ADR 0001's one 60 Hz run (its setup and R2-BUD-REC6's 30-minute soak): p99 sim step time ≤ 4.17 ms (25 % of 16.67 ms), max ≤ 16.67 ms, zero overruns; navigation and the visibility pass p99 ≤ 0.83 ms each (5 % of 16.67 ms); the client's sim share at 60 Hz, p99 ≤ 6.67 ms in Wasm (40 % of 16.67 ms) | RS (that run's tick thread); ML (the client share) | the sim's phase times read from that same run (the sim inside the proof-test server: OQ-0001-2 (a)), never added to a separately measured server-only share; the client share as PM-0004-8 on a 60 Hz build | only if ADR 0001 schedules the 60 Hz margin's run | R1-TICK-REC2 P1, P2 (lines 2047–2049, 2054–2056); GL lines 53–60; GL §8 (the 60 Hz margin); ADR 0001's spike item 3 |
| PM-0004-72 | The time mechanism and the movement checks hold at 60 Hz (S3, S4, S9, S11, S13; parts 4, 10, 12, 14) | with PM-0004-71's 60 Hz build (durations re-converted, links re-baked): every legacy interval gives the same shot count at 30 and 60 Hz within 1 shot; sight checks per game second per bot equal within 1 % at 30 and 60 Hz; PM-0004-37's tunnelling, PM-0004-38, PM-0004-48 (in the research's form: 2.9 m kept, 3.0 and 3.1 m rejected if the measured reach stays near legacy's 2.98 m) and PM-0004-55, each at 60 Hz | CI; RS | the same scripts at 60 Hz | only with the 60 Hz margin's test; each as its 30 Hz row | R1-TICK-REC1 (lines 2023–2024); R1-TICK-REC3 (lines 2075–2076); R4-PHYS-REC3, REC4 (r4 lines 758–759, 779–784); R4-FLOORS-REC1 (r4 lines 919–921); R4-OBST-REC1 (r4 lines 1037–1038) |
AFTER:
| PM-0004-71 | The sim step share at 60 Hz, for the 60 Hz margin only (S1, S2; part 6; TICK-60-HEADROOM) | inside each of ADR 0001's 60 Hz runs (its setup and R2-BUD-REC6's 30-minute soak): p99 sim step time ≤ 4.17 ms (25 % of 16.67 ms), max ≤ 16.67 ms, zero overruns; navigation and the visibility pass p99 ≤ 0.83 ms each (5 % of 16.67 ms); the client's sim share at 60 Hz, p99 ≤ 6.67 ms in Wasm (40 % of 16.67 ms) | RS (that run's tick thread); ML (the client share) | the sim's phase times read from that same run (the sim inside the proof-test server: in wave 1 the sim core spike's sim with S1's stand-ins, OQ-0001-2 (a), answered; at slice 6 the real crate), never added to a separately measured server-only share; the client share as PM-0004-8 on a 60 Hz build | with each of ADR 0001's 60 Hz runs; it decides only at slice 6 (the key) | R1-TICK-REC2 P1, P2 (lines 2047–2049, 2054–2056); GL lines 53–60; GL §8 (the 60 Hz margin); ADR 0001's spike item 3 |
| PM-0004-72 | The time mechanism and the movement checks hold at 60 Hz (S3, S4, S9, S11, S13; parts 4, 10, 12, 14) | with PM-0004-71's 60 Hz build (durations re-converted, links re-baked): every legacy interval gives the same shot count at 30 and 60 Hz within 1 shot; sight checks per game second per bot equal within 1 % at 30 and 60 Hz; PM-0004-37's tunnelling, PM-0004-38, PM-0004-48 (in the research's form: 2.9 m kept, 3.0 and 3.1 m rejected if the measured reach stays near legacy's 2.98 m) and PM-0004-55, each at 60 Hz | CI; RS | the same scripts at 60 Hz | with each of ADR 0001's 60 Hz runs, deciding only at slice 6 (the key); each as its 30 Hz row | R1-TICK-REC1 (lines 2023–2024); R1-TICK-REC3 (lines 2075–2076); R4-PHYS-REC3, REC4 (r4 lines 758–759, 779–784); R4-FLOORS-REC1 (r4 lines 919–921); R4-OBST-REC1 (r4 lines 1037–1038) |
END

### FIX N11
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  J6 (3); OQ-0001-3). Every other mark runs in one process or on localhost (P0).
AFTER:
  J6 (3); OQ-0001-3, answered by the Advisor, 2026-11-03). Every other mark runs in one process or on localhost (P0).
END

### FIX C8
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| PM-0005-4 | One crate gives one verdict everywhere (CD-2; content item 2; shared with ability item 3) | over C: 0 crashes (no panic natively, no trap in Wasm), 0 accepted invalid files, 100 % of valid control files accepted, and for 100 % of files the same verdict and the same error record (stage, file, line and column or field path, error kind) natively and in Wasm; every error names the file and either line and column (syntax) or a field path (data), and the path named is the path the generator changed in 100 % of data errors; the 50 MB files refused at the byte-size stage and the 10,000-deep files at the depth stage. Recorded: the default depth limit's value (R1-DATA-OQ4) and whether garde builds and runs on wasm32 (R1-DATA-OQ5; if not, CD-2's fallback is built and this row runs on it) | CI | C: 10,000 files from a seeded generator (regenerated byte for byte from its seed: proposed, so a failure reproduces) mutating valid seeds of every content kind (the imported set of PM-0005-14, the two maps of PM-0005-9, two modes, test track definitions); every defect class of the research at least 500 files (proposed), except the 50 MB class, 10 files generated in the run and not committed (proposed: their size adds time, not cases); 1,000 unmutated or validly mutated controls (proposed); a trap in Wasm counts as a crash and the instance is recreated | 1 run per target; joins the gate (20 identical verdicts). Bites: a planted `serde(flatten)` that lets an unknown key through; a planted fallback weapon on a dangling id | R1-DATA-REC2 marks 1–2 (lines 2513–2516); R1-DATA-OQ4 (lines 2799–2800), OQ5 (lines 2801–2803); CD-2 |
AFTER:
| PM-0005-4 | One crate gives one verdict everywhere (CD-2; content item 2; shared with ability item 3; OQ-0005-1's whole-set rule) | over C: 0 crashes (no panic natively, no trap in Wasm), 0 accepted invalid files, 100 % of valid control files accepted, and for 100 % of files the same verdict and the same error record (stage, file, line and column or field path, error kind) natively and in Wasm; every error names the file and either line and column (syntax) or a field path (data), and the path named is the path the generator changed in 100 % of data errors; the 50 MB files refused at the byte-size stage and the 10,000-deep files at the depth stage. A content set in which an id of a presented kind has no client entry is refused at the whole-set stage, naming the kind and the id, while an id of a kind with nothing to present (a mode, a tuning table) is accepted with none (OQ-0005-1, answered by the Advisor (2026-11-03): "the client's data, keyed by the same id; the content check fails if a simulation id has no client entry"; it binds the kinds the client presents: the advisor's reading, 2026-11-03), the presented kinds as CD-2 names them (from AP-2, CD-13 and AB-3: units, vehicles, weapons, factions, building parts and props). Recorded: the default depth limit's value (R1-DATA-OQ4) and whether garde builds and runs on wasm32 (R1-DATA-OQ5; if not, CD-2's fallback is built and this row runs on it) | CI | C: 10,000 files from a seeded generator (regenerated byte for byte from its seed: proposed, so a failure reproduces) mutating valid seeds of every content kind (the imported set of PM-0005-14, the two maps of PM-0005-9, two modes, test track definitions); every defect class of the research at least 500 files (proposed), except the 50 MB class, 10 files generated in the run and not committed (proposed: their size adds time, not cases); 1,000 unmutated or validly mutated controls (proposed); the client-entry class: valid content sets with one id's client entry removed, for every presented kind, and as controls sets whose mode and tuning table have no entry (proposed: at least 500 sets, as every defect class); a trap in Wasm counts as a crash and the instance is recreated | 1 run per target; joins the gate (20 identical verdicts). Bites: a planted `serde(flatten)` that lets an unknown key through; a planted fallback weapon on a dangling id; the client-entry check left out of the whole-set stage (a unit id with no client entry then loads) | R1-DATA-REC2 marks 1–2 (lines 2513–2516); R1-DATA-OQ4 (lines 2799–2800), OQ5 (lines 2801–2803); CD-2; OQ-0005-1 (answered by the Advisor, 2026-11-03) |
END

### FIX B4
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
operation for one mechanic that fails PM-0005-18 to -21, option A4 with E1's tables if many fail (OQ-0005-AB13); track
AFTER:
operation for a mechanic that fails PM-0005-18 to -21, and scripting (option A3) back only when a designed ability
needs ability-specific code that no operation can express (OQ-0005-AB13, answered by the Advisor, 2026-11-03:
"scripting comes back only when a designed ability can't be built from data"; a new reviewed operation counts as
built from data: the advisor's reading, 2026-11-03); track
END

### FIX M11
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  never the VM under test (proposed: bot clients on the VM under test would spend the vCPUs the marks measure): CX33s
  (or a larger plan), as many as a 5-minute calibration run at the session's load shows keep each under 80 % of its
AFTER:
  never the VM under test (proposed: bot clients on the VM under test would spend the vCPUs the marks measure): CX33s
  ("the CX33 runs the simulated clients": SPIKE-MACHINES, Mustafa, 2026-11-03), as many as a 5-minute calibration run
  at the session's load shows keep each under 80 % of its
END

### FIX H14
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  (TICK-30); none runs at 60 Hz: [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) is ADR 0001's run.
AFTER:
  (TICK-30); none runs at 60 Hz: [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) is ADR 0001's, and so
  is the rooms-per-VM count at 60 Hz with its goal-budget check (PM-0001-63, on PM-0006-14's counting rule).
END

### FIX S16
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  for [the stress scene](../research/GLOSSARY.md#the-stress-scene) as OQ-0001-2 sets it) at N = 48 (MODES-3 (a))
AFTER:
  for [the stress scene](../research/GLOSSARY.md#the-stress-scene) as OQ-0001-2 (a) sets it, answered by the Advisor,
  2026-11-03) at N = 48 (MODES-3 (a))
END

### FIX N13
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  1987–1990; the reading of "100 ms of simulated latency" ADR 0001 states, decision map part j, J6 (3)), injected on LG
  in both directions, half the round trip each way (R5-TEST-REC4's per-direction rule, r5 lines 1455–1456), never on
AFTER:
  1987–1990; the reading of "100 ms of simulated latency" ADR 0001 states, OQ-0001-3, answered by the Advisor,
  2026-11-03; decision map part j, J6 (3)), injected on LG in both directions, half the round trip and ± half the
  jitter each way and the profile's loss on each (OQ-0001-16, answered by the Advisor, 2026-11-03; R5-TEST-REC4's
  per-direction rule, r5 lines 1455–1456), never on
END

### FIX M11-2
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-11 | The reference server exists and is the machine every server row ran on (part 5; OQ-0006-1, the precondition) | a CCX23 and a CX33 created in `nbg1` or `hel1`, after PM-0006-12's round trips and OQ-0006-2's pick of the location; recorded for each: type, location, CPU model, vCPU count, memory, kernel, and the number of threads on each core that `lscpu` reports, with the map of which vCPUs are SMT siblings (if a core carries 2 threads, the transport runtime of ADR 0001's pinned rows is kept off the tick thread's sibling, and that placement is recorded: proposed, since a sibling shares the physical core and "one vCPU with no other room on it" would not isolate the tick; the CCX23's topology is not researched); the CPU model read again at every session (a change is recorded and named in the results) | RS, SH | Mustafa creates the VMs or the API token that creates them (his account); a script reads `/proc/cpuinfo` and the plan at each session start | every session | R6-HOST-REC2 (r6 lines 645–650); R6-HOST-F20 (r6 lines 180–182); GLOSSARY, the reference server (lines 225–231) |
AFTER:
| PM-0006-11 | The reference server exists and is the machine every server row ran on (part 5; OQ-0006-1, the precondition) | a CCX23 and a CX33 created in `nbg1` or `hel1`, after PM-0006-12's round trips and OQ-0006-2's pick of the location; recorded for each: type, location, CPU model, vCPU count, memory, kernel, and the number of threads on each core that `lscpu` reports, with the map of which vCPUs are SMT siblings (if a core carries 2 threads, the transport runtime of ADR 0001's pinned rows is kept off the tick thread's sibling, and that placement is recorded: proposed, since a sibling shares the physical core and "one vCPU with no other room on it" would not isolate the tick; the CCX23's topology is not researched); the CPU model read again at every session (a change is recorded and named in the results) | RS, SH | Mustafa creates the VMs himself for each run, in a separate Hetzner project, and deletes them after it; sessions get SSH access only and no Hetzner token (SPIKE-MACHINES, Mustafa, 2026-11-03: "I create and delete the VMs myself for each run; sessions get SSH access only and no Hetzner token until the security baseline"; OQ-0006-17); a script reads `/proc/cpuinfo` and the plan at each session start | every session | R6-HOST-REC2 (r6 lines 645–650); R6-HOST-F20 (r6 lines 180–182); GLOSSARY, the reference server (lines 225–231) |
END

### FIX M12
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| PM-0006-15 | The firewall lets both game transports in and nothing else (part 5) | from each of the four browsers, 20 of 20 WebTransport sessions over UDP and 20 of 20 WebSocket sessions over TCP (`wss`) reach a room on RS; with no rule for a port, a probe to a listening service on that port gets no answer (a TCP connect times out, never "refused"; a UDP probe gets no reply); the firewall's rule list for each VM role equals part 5's list for that role, with no rule outside it and every source limited as listed (the game VM under test: the UDP range, the TCP range, part 9's metrics rule with MV as its only source, PM-0006-16, and SSH by key from the named source addresses only; staging's delivery path and TCP 80 for http-01 only where part 5 names them). The `wss` endpoint presents a certificate the four browsers accept with no warning, and which one is recorded: the draft names none for development, and the WebSocket constructor takes only a URL and sub-protocols (R2-ROOM-F12, r2 lines 1494–1500), so a self-signed certificate cannot be pinned for `wss` | RS; ML: Chrome, Edge, Firefox; MAC: Safari; a probe from ML | WebdriverIO opens the netcode proof test's client page with the transport forced by a URL flag; an attempt succeeds when the session is ready and one datagram or message echoes back; the probe opens a TCP connection and sends a UDP datagram to one closed port of each kind, with a test listener on each; the rule list is read from Hetzner's console or API and pasted into the results | 20 attempts per browser and transport (proposed: R2-TRN-REC5's count); 1 probe pair. Bites: a rule added for the probed TCP port makes the probe connect (the negative test can fail); an SSH rule opened to every source fails the list comparison | R6-HOST-REC2's mark (r6 lines 669–671); part 5 ("and nothing else"); R6-HOST-F18 (r6 lines 169–174); R2-TRN-REC3 (r2 lines 2639–2659) |
AFTER:
| PM-0006-15 | The firewall lets both game transports in and nothing else (part 5) | from each of the four browsers, 20 of 20 WebTransport sessions over UDP and 20 of 20 WebSocket sessions over TCP (`wss`) reach a room on RS; with no rule for a port, a probe to a listening service on that port gets no answer (a TCP connect times out, never "refused"; a UDP probe gets no reply); the firewall's rule list for each VM role equals part 5's list for that role, with no rule outside it and every source limited as listed (the game VM under test: the UDP range, the TCP range, part 9's metrics rule with MV as its only source, PM-0006-16, and SSH by key from the named source addresses only; staging's delivery path and TCP 80 for http-01 only where part 5 names them). The `wss` endpoint presents a certificate the four browsers accept with no warning, and which one is recorded: a trusted certificate on the domain Mustafa registers (SPIKE-MACHINES, Mustafa, 2026-11-03: "for the WebSocket leg, a domain I register"; OQ-0006-5's development case (i): the advisor's reading, 2026-11-03); the WebSocket constructor takes only a URL and sub-protocols (R2-ROOM-F12, r2 lines 1494–1500), so a self-signed certificate cannot be pinned for `wss` | RS; ML: Chrome, Edge, Firefox; MAC: Safari; a probe from ML | WebdriverIO opens the netcode proof test's client page with the transport forced by a URL flag; an attempt succeeds when the session is ready and one datagram or message echoes back; the probe opens a TCP connection and sends a UDP datagram to one closed port of each kind, with a test listener on each; the rule list is read from Hetzner's console or API and pasted into the results | 20 attempts per browser and transport (proposed: R2-TRN-REC5's count); 1 probe pair. Bites: a rule added for the probed TCP port makes the probe connect (the negative test can fail); an SSH rule opened to every source fails the list comparison | R6-HOST-REC2's mark (r6 lines 669–671); part 5 ("and nothing else"); R6-HOST-F18 (r6 lines 169–174); R2-TRN-REC3 (r2 lines 2639–2659) |
END

### FIX H14-2
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
- No mark here runs at 60 Hz: the 60 Hz margin is ADR 0001's run. The netcode proof test's "look correct" checklist and
  its measurable marks (hits the server confirms, correction sizes, prediction error at P2) are ADR 0001's pass marks.
AFTER:
- No mark here runs at 60 Hz: the 60 Hz margin and the rooms per VM at 60 Hz with their goal-budget check (PM-0001-63)
  are ADR 0001's. The netcode proof test's "look correct" checklist and its measurable marks (hits the server confirms,
  correction sizes, prediction error at P2) are ADR 0001's pass marks.
END

### NOTE H15
no change: by the advisor's ruling on TICK-60-HEADROOM the rooms-per-VM count at 60 Hz lives in ADR 0001's 60 Hz
group as PM-0001-63, which runs PM-0006-14's counting rule at a 60 Hz tick; PM-0006-14 stays the 30 Hz count, and ADR
0006's "none runs at 60 Hz" lines stay true (FIX H14 and H14-2 point them to PM-0001-63).
END

### NOTE N9
no change: PM-0002-49 is in ADR 0002, outside this unit's files, and the map marks the change optional. The row
checks L1 against its profiles' values; how L1 applies them per direction is ADR 0002 part 8's (map N8,
engine-architect), and ADR 0001's PM-0001-3 already measures loss, delay and jitter per direction on L1 and L2 before
any mark under a profile counts.
END

### NOTE N12
no change: the Feel Lab checklist's "P2 (100 ms round trip, ±20 ms, 2 % loss)" (ADR 0005:2345-2346) is the Advisor's
P2 value for value.
END

### NOTE M13
no change at my places: PM-0006-25 and -27 (ADR 0006:1195, :1197) already test a pinned self-signed certificate for
WebTransport, which is Mustafa's "a private hash-pinned certificate for WebTransport". Spike item 2 (:1081) is
infra-engineer's place in this row.
END

### NOTE M21
no change at my place (ADR 0003's Pass marks key, Machines, :1049, "the models are OQ-0003-1"): by the advisor's
ruling (d) on SPIKE-MACHINES, "Test PC: my own, in Brave" is MP as the key already defines it (:1058-1059: only what he
judges, never a budget result), so OQ-0003-1 is unchanged in substance and goes to Mustafa as REF-MACHINES. OQ-0003-1
itself (:800-807) and migration step 1 (:1337) are graphics-engineer's.
END

### NOTE L12
no change: the advisor's ruling on LOOK-TEST-INPUTS: which look-checklist steps a blockout can judge is for the look
proof test's own plan (graphics-engineer, test-engineer), any visual call is Mustafa's, and the checklist is not edited
now.
END

### NOTE T3
no change at my place (ADR 0003:1302, the look checklist's "the toy-soldier death fall"): it is the legacy animation
item for OQ-0003-15 (GA14), not TOY-BOX's subject, and by the advisor's ruling TOY-BOX changes nothing in the nine
documents.
END

### NOTE C8-2
no change beyond FIX C8: engine-architect's CD-2 text naming them (verdict ruling on OQ-0005-1) was not on disk when FIX C8
was written, so FIX C8 names them itself, from AP-2 (units, vehicles, weapons, faction uniforms, building parts and
props name their models by id), CD-13 (a faction's colours and uniform models, keyed by the faction's id) and AB-3 (a
weapon's feel kit, keyed by the weapon's id): units, vehicles, weapons, factions, building parts and props; modes and
tuning tables are the kinds with nothing to present, as the ruling names them. The row cites "the presented kinds as
CD-2 names them": if engine-architect's list differs (for example ability kinds, which game-designer confirms), CD-2's
list governs and the bracket in PM-0005-4 follows it in fixcheck-e's round.
END

