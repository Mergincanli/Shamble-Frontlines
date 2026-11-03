# Plan revision 6 (amended): the work list by script (the advisor's amended condition 1)

Search over the nine documents (ADR 0001-0006, README, SPIKE_PLAN, ENGINE_OVERVIEW) and GLOSSARY §8, for: CX33; CCX13; DS;
SH; MV; OQ-0006-1; OQ-0006-2; R6-HOST-OQ1; R6-HOST-OQ2; nbg1|hel1|fsn1|Falkenstein|Nuremberg|Helsinki; "every server
row|every native server mark|every server mark"; and "address" in the pass-mark sections (keys, rows, the operations
checklist) of ADR 0001 and 0006. Owner by section (pass marks to the migration plan: test-engineer). Each hit needs a
disposition from its owner: a FIX block, or a NOTE "no change: <reason>" (one NOTE may cover several lines if it names
each). Line numbers are the docs at the time of the search (2026-11-03, before this round's edits).

## NOT EDITED (GLOSSARY base, lines 1-743) (4 lines)
- docs/research/GLOSSARY.md:227: vCPU, 16 GB, x86-64 AMD), in one EU location, with the shared-vCPU CX33 measured beside it (R6-HOST-REC2, which names
- docs/research/GLOSSARY.md:228: it as the answer to R2-XU-OQ9). It stays a working pick while R6-HOST-OQ1 (Hetzner's EU plans showed "currently
- docs/research/GLOSSARY.md:260: R2-XU-OQ9 (every server mark on one of its vCPUs), the pointers between the R2 units (rooms per vCPU). R4: the
- docs/research/GLOSSARY.md:605: - **Definition:** the one small shared VM (proposed CX33) whose supervisor starts the room binary of the build a

## engine-architect-b (13 lines)
- docs/adr/README.md:21 [OQ-0006-1]: | [0006](0006-infrastructure-operations.md) | Infrastructure and operations | infra-engineer; pass marks: test-engineer | Proposed. Signed off by Mustafa: as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not ye
- docs/adr/README.md:133 [CX33, OQ-0006-1]: | OQ-0006-1 | Can Mustafa order Hetzner's CCX23 and CX33 in an EU location? Still open after the stop (2026-11-03): his console check. | Mustafa (a console check in his own account; agents create no account), then a rese
- docs/adr/README.md:134 [Falkenstein, Helsinki, Nuremberg, OQ-0006-2]: | OQ-0006-2 | Which EU location: Nuremberg, Helsinki, or Falkenstein if offered? | a measurement made before the reference server is created (a small check at the start of the netcode proof test's setup), confirmed by Mu
- docs/SPIKE_PLAN.md:30 [SH]: - **Machines** (named in each draft's pass-mark key): RS, the reference server, and SH, its shared neighbour (ADR 0006,
- docs/SPIKE_PLAN.md:31 [OQ-0006-1]: OQ-0006-1; [GLOSSARY: the reference server](research/GLOSSARY.md#the-reference-server)); LG, the load generator, a
- docs/SPIKE_PLAN.md:32 [CX33]: CX33 running the simulated clients (Mustafa, 2026-11-03: "the CX33 runs the simulated clients"; how many comes from
- docs/SPIKE_PLAN.md:33 [CX33, MV]: the calibration run: the advisor's reading, 2026-11-03), and MV, a CX33 running the metrics stack (ADR 0006); ML, the
- docs/SPIKE_PLAN.md:129 [CX33, OQ-0006-1]: - **Depends on:** the drafts approved; OQ-0006-1 answered (the CCX23 and CX33 orderable in one EU location, in
- docs/SPIKE_PLAN.md:130 [OQ-0006-2]: Mustafa's Hetzner account; still open after the stop), with the location measured before RS is created (OQ-0006-2);
- docs/SPIKE_PLAN.md:133 [MV, SH]: only during a run."), LG and MV beside RS and SH; the session's SSH key, made and held without the session reading
- docs/SPIKE_PLAN.md:176 [OQ-0006-2]: it feeds OQ-0006-2; it sets the rewind cap's value by OQ-0001-5's rule (Mustafa, 2026-11-03: "the cap covers the
- docs/SPIKE_PLAN.md:281 [OQ-0006-1]: - **Depends on:** the drafts approved; RS orderable (OQ-0006-1); the machines its rows name (OQ-0003-1, answered in
- docs/SPIKE_PLAN.md:513 [OQ-0006-1, OQ-0006-2]: server location (OQ-0006-1, OQ-0006-2) and the provisional goal budget (OQ-0006-19) are listed for Mustafa and the

## infra-engineer (51 lines)
- docs/adr/0006-infrastructure-operations.md:91 [every server mark]: - **Gives:** [the reference server](../research/GLOSSARY.md#the-reference-server) every server mark is measured on
- docs/adr/0006-infrastructure-operations.md:119 [CX33]: server, with the shared CX33 measured beside it. Keep the budget flat and small at the start (about 18 to 102 EUR a
- docs/adr/0006-infrastructure-operations.md:189 [CX33]: [the staging game server](../research/GLOSSARY.md#the-staging-game-server), proposed a CX33 (4 vCPU, 8 GB, 8.99 EUR
- docs/adr/0006-infrastructure-operations.md:244 [CX33]: reference server** is a CCX23 (4 dedicated vCPU, 16 GB, x86-64 AMD), and the shared CX33 (4 vCPU, 8 GB) is measured
- docs/adr/0006-infrastructure-operations.md:246 [every native server mark]: (r2-netcode.md, Cross-unit open questions, R2-XU-OQ9, lines 3197–3202), and every native server mark is measured on
- docs/adr/0006-infrastructure-operations.md:249 [OQ-0006-1]: in the Hetzner console with his own account before the netcode proof test is scheduled (OQ-0006-1). The location is
- docs/adr/0006-infrastructure-operations.md:250 [Falkenstein, Helsinki, Nuremberg, fsn1, hel1, nbg1]: Nuremberg (`nbg1`) or Helsinki (`hel1`), Falkenstein (`fsn1`) only if the plans are offered there, picked by a
- docs/adr/0006-infrastructure-operations.md:251 [OQ-0006-2]: round-trip measurement (r6-infrastructure.md, Options §1, Q4, lines 429–432; OQ-0006-2).
- docs/adr/0006-infrastructure-operations.md:254 [CX33]: VM is included; the shared CX33 is 9.6 times cheaper (8.99 against 86.49 EUR), so it is measured, not assumed
- docs/adr/0006-infrastructure-operations.md:267 [CX33]: already budgeted; on the second it needs one more VM, for example a CX33 at 8.99 EUR a month (R6-HOST-F15; a
- docs/adr/0006-infrastructure-operations.md:325 [CX33]: on all month): about 18 EUR a month lean (production CX33 8.99 + staging CX33 8.99 = 17.98 EUR, 0 USD) to about
- docs/adr/0006-infrastructure-operations.md:326 [CX33]: 102 EUR + 5 USD reference (production CCX23 86.49 + staging CX33 8.99 + match records on Hetzner Object Storage
- docs/adr/0006-infrastructure-operations.md:352 [CX33]: - *Fleet shape:* a fixed fleet (K1) first; K2 only once real concurrency data shows a clear peak; the shared CX33
- docs/adr/0006-infrastructure-operations.md:373 [CX33]: on a CX33 beside the reference server, never on it (as spike 2's load generators): 3 × (0.1386 + 0.0144) =
- docs/adr/0006-infrastructure-operations.md:607 [CX33]: | **G2. Hetzner Cloud, shared vCPU (CX33, measured beside G1)** | As G1 | CX33 8.99 EUR a month (0.0144 an hour) (F15) | "designed for variable usage patterns rather than sustained high workloads" (F14): server tick [p99
- docs/adr/0006-infrastructure-operations.md:610 [OQ-0006-1, R6-HOST-OQ1]: | Hetzner dedicated root servers; Netcup, OVHcloud, Scaleway | not researched (R6-HOST-OQ1, OQ4, r6 lines 852–855 and 862–865) | not researched | not researched | not researched | not researched | not researched | Netcup
- docs/adr/0006-infrastructure-operations.md:632 [Falkenstein, Helsinki, Nuremberg, fsn1, hel1, nbg1]: Nuremberg (`nbg1`) or Helsinki (`hel1`), in Hetzner's one EU zone; Falkenstein (`fsn1`) only if the plans are offered
- docs/adr/0006-infrastructure-operations.md:633 [HEL1, NBG1]: there (their product pages list "NBG1 HEL1", R6-HOST-F13, r6 lines 136–143). No latency source was read for any of them,
- docs/adr/0006-infrastructure-operations.md:634 [OQ-0006-2]: so the table's columns are equal but for latency, which a measurement decides (OQ-0006-2): complexity, cost and effort
- docs/adr/0006-infrastructure-operations.md:713 [CX33]: r6 lines 461–464). The shared CX33 is measured beside it because it is 9.6 times cheaper. The strongest argument against
- docs/adr/0006-infrastructure-operations.md:715 [OQ-0006-1]: provider: OQ-0006-1 makes that a check before the netcode proof test, not a surprise during it. A fixed fleet (K1)
- docs/adr/0006-infrastructure-operations.md:753 [every server mark]: - **Easier:** every server mark in ADR 0001 and ADR 0004 is measured on one named machine (N34, I-15); a preview link
- docs/adr/0006-infrastructure-operations.md:766 [OQ-0006-1]: - **Revisit when:** Hetzner's EU plans are not orderable (OQ-0006-1); PREV-DELETE fails on Pages (move to Workers
- docs/adr/0006-infrastructure-operations.md:768 [CX33]: on the real sim (the goal budget fixed, OQ-0006-19); the shared CX33 passes (K3) or
- docs/adr/0006-infrastructure-operations.md:799 [CX33, OQ-0006-1]: - **OQ-0006-1. Can Mustafa order Hetzner's CCX23 and CX33 in an EU location?**
- docs/adr/0006-infrastructure-operations.md:800 [every native server mark]: - *Why it matters:* every native server mark and the netcode proof test run on the reference server; on 2026-10-28
- docs/adr/0006-infrastructure-operations.md:803 [R6-HOST-OQ1]: page, GameLift's EU prices and server SDK, and Netcup, OVHcloud or Scaleway (none read: R6-HOST-OQ1, OQ4, r6 lines
- docs/adr/0006-infrastructure-operations.md:809 [hel1, nbg1]: - *What decides it:* the console offering both plans in `nbg1` or `hel1` before the netcode proof test is scheduled.
- docs/adr/0006-infrastructure-operations.md:810 [R6-HOST-OQ1]: - *Carries:* R6-HOST-OQ1, R6-HOST-OQ4, R6-HOST-OQ7.
- docs/adr/0006-infrastructure-operations.md:811 [Falkenstein, Helsinki, Nuremberg, OQ-0006-2]: - **OQ-0006-2. Which EU location: Nuremberg, Helsinki, or Falkenstein if offered?**
- docs/adr/0006-infrastructure-operations.md:813 [fsn1, hel1, nbg1]: - *Options:* `nbg1`, `hel1`; `fsn1` only if the plans are offered there (R6-HOST-F13, r6 lines 136–143).
- docs/adr/0006-infrastructure-operations.md:817 [R6-HOST-OQ2]: "about 100 ms" (R6-HOST-OQ2, r6 lines 856–858).
- docs/adr/0006-infrastructure-operations.md:818 [R6-HOST-OQ2]: - *Carries:* R6-HOST-OQ2.
- docs/adr/0006-infrastructure-operations.md:825 [CX33]: more VM, for example a CX33 at 8.99 EUR a month as a placeholder) (r2-netcode.md, R2-ROOM-OQ9, lines 3104–3105);
- docs/adr/0006-infrastructure-operations.md:1032 [OQ-0006-1]: - *What decides it:* his choice before the netcode proof test, with OQ-0006-1's console check.
- docs/adr/0006-infrastructure-operations.md:1073 [CX33, MV, SH]: 15 to 30 × 0.1386 = 2.08 to 4.16 EUR) and the CX33 hours of SH, MV and the load generators (as many as spike 2's
- docs/adr/0006-infrastructure-operations.md:1077 [CX33]: EUR a CCX23-hour, 0.0144 a CX33-hour) before the first session; (c) 5 EUR over everything, as written before.
- docs/adr/0006-infrastructure-operations.md:1103 [every server mark]: gives 48 and 54, 48 holds (MODES-3 (a)); every server mark runs at the 30 Hz baseline (TICK-30), and the 60 Hz margin
- docs/adr/0006-infrastructure-operations.md:1133 [OQ-0006-1]: precondition is OQ-0006-1). They prove parts 5, 7, 8 and 9:
- docs/adr/0006-infrastructure-operations.md:1134 [CX33]: - rooms per VM: on the CCX23 and on the CX33, 8 rooms of 48 players (bots) as 8 processes, each room's server tick p99
- docs/adr/0006-infrastructure-operations.md:1364 [OQ-0006-1]: of OQ-0006-1 and a Hetzner project of its own for the spikes, the tick bench and the playtest soaks, in which he
- docs/adr/0006-infrastructure-operations.md:1370 [OQ-0006-1]: the advisor's reading, 2026-11-03). *Milestone:* the reference server is orderable, or OQ-0006-1 (b) has named
- docs/adr/0006-infrastructure-operations.md:1435 [CX33]: match count, and a month of goal-scale records (about 13,000) is analysed in under 10 minutes on a CX33, the staging
- docs/adr/0006-infrastructure-operations.md:1446 [OQ-0006-1]: - **Game servers:** if the CCX23 cannot be ordered or fails the marks, read OQ-0006-1's fallbacks and re-run the
- docs/adr/0006-infrastructure-operations.md:1448 [CX33]: CX33 fails, the fleet stays on dedicated vCPUs (K1). VMs are billed by the hour with a monthly cap (R6-HOST-F17), so a
- docs/adr/0006-infrastructure-operations.md:1499 [OQ-0006-1]: | R6-HOST-REC2 | changed: the firewall's rules listed per VM role | part 5: R6-HOST-REC2 opens the two game ranges "plus nothing else" (r6 line 649); this record's own parts need more (a Prometheus scrape needs an inboun
- docs/adr/0006-infrastructure-operations.md:1510 [CX33]: | R6-TEL-REC3 | changed: the batch analysis runs off the staging VM | part 12 and B12: the staging VM runs pull-request-built code, so it holds no store key; the analysis runs on a production game VM while no room runs, 
- docs/adr/0006-infrastructure-operations.md:1511 [OQ-0006-1, R6-HOST-OQ1]: | R6-HOST-OQ1 | open: OQ-0006-1 | Mustafa's console check |
- docs/adr/0006-infrastructure-operations.md:1512 [OQ-0006-2, R6-HOST-OQ2]: | R6-HOST-OQ2 | open: OQ-0006-2 | a round-trip measurement |
- docs/adr/0006-infrastructure-operations.md:1514 [OQ-0006-1]: | R6-HOST-OQ4 | open: OQ-0006-1 | Fly.io, GameLift, Netcup, OVHcloud and Scaleway are read only if Hetzner Cloud is ruled out; Hetzner's dedicated root servers are read before part 8's goal budget is fixed, whatever OQ-0
- docs/adr/0006-infrastructure-operations.md:1517 [OQ-0006-1]: | R6-HOST-OQ7 | open: OQ-0006-1 | read at sign-up; under 1 EUR per VM either way |

## netcode-architect (3 lines)
- docs/adr/0001-multiplayer-netcode.md:719 [Every server mark]: all within the budgets that ADR 0001 sets" (GLOSSARY §8) is judged against these. Every server mark is measured on one
- docs/adr/0001-multiplayer-netcode.md:756 [R6-HOST-OQ1]: | No Hetzner capacity in the EU | the reference server cannot be ordered | R6-HOST-OQ1 (ADR 0006) |
- docs/adr/0001-multiplayer-netcode.md:1293 [CX33]: (R2-ROOM-REC2 marks 2 and 4, on the CCX23 and the CX33: R6-HOST-REC2, ADR 0006).

## test-engineer (40 lines)
- docs/adr/0001-multiplayer-netcode.md:1354 [CX33, SH]: 0006's PM-0006-13 sets it (eight tick threads cannot each be alone on one of four vCPUs). **SH**, the shared CX33
- docs/adr/0001-multiplayer-netcode.md:1356 [CX33]: generator: one or more VMs in the same location (CX33s: "the CX33 runs the simulated clients", SPIKE-MACHINES,
- docs/adr/0001-multiplayer-netcode.md:1358 [SH]: speak the real protocol (r5-toolchain-quality.md, R5-TEST-REC5, lines 1475–1480), never RS or SH; as many as a
- docs/adr/0001-multiplayer-netcode.md:1360 [address]: and their hours in the spike's bill (ADR 0006's spike item 2 and its LG key, PM-0006-18); on LG's addresses the
- docs/adr/0001-multiplayer-netcode.md:1361 [address]: per-address connection cap (part 10) is raised to the run's session count and recorded, except in PM-0001-25's run
- docs/adr/0001-multiplayer-netcode.md:1363 [address]: encrypted sessions, and the cap would refuse most sessions from one address: the infra review of 2026-11-02,
- docs/adr/0001-multiplayer-netcode.md:1463 [SH]: on SH steal is what is measured ("repeated at 3 times of day (shared hosts vary with neighbours)":
- docs/adr/0001-multiplayer-netcode.md:1481 [SH, every server mark]: | PM-0001-1 | The rig is named and the measuring unit holds (part 14; a precondition of every server mark) | RS, SH and LG named with plan, CPU model, vCPU count, memory, location and kernel; ML, RD and MAC as ADR 0003 n
- docs/adr/0001-multiplayer-netcode.md:1497 [SH]: | PM-0001-16 | A room starts fast (part 11) | every room start, process spawn to Open, under 2 s (a plain limit) | RS and SH | every room start in the spike, at least 20 (proposed), timed by the supervisor's stand-in | e
- docs/adr/0001-multiplayer-netcode.md:1508 [address]: | PM-0001-25 | Hostile input is contained on both transports (part 10) | a flood (1,000 sessions that never send `Join`, plus 10 clients sending oversize datagrams at full rate and opening streams) leaves server tick tim
- docs/adr/0001-multiplayer-netcode.md:1511 [SH]: | PM-0001-27 | Rooms share a VM fairly (part 11; with ADR 0006's density figure) | 8 rooms as 8 processes on one machine: each room's server tick time p99 within 10 % of the single-room figure over 20 minutes, that figur
- docs/adr/0001-multiplayer-netcode.md:1514 [address]: | PM-0001-29 | The fallback takes over when QUIC is blocked (part 10; decides R2-TRN-OQ2) | with UDP blocked to the server, each of the four browsers reaches a playing WebSocket session at most 3 s after the page starts 
- docs/adr/0001-multiplayer-netcode.md:1527 [CX33]: | PM-0001-63 | The goal budget holds at the rooms per VM measured at 60 Hz (part 2; TICK-60-HEADROOM's addition; ADR 0006 part 8) | k60, the most rooms of N = 48 at a 60 Hz tick for which every room meets R2-ROOM-REC2 ma
- docs/adr/0004-simulation-core.md:1030 [R6-HOST-OQ1]: R6-HOST-OQ1 is open), with no other room on that vCPU (GL lines 244–250); the bench process is pinned to one vCPU and
- docs/adr/0004-simulation-core.md:1032 [CX33]: document). **CX33**, the shared-vCPU CX33, measured beside RS where a row says so (R6-HOST-REC2): reported, never
- docs/adr/0004-simulation-core.md:1117 [CX33]: | PM-0004-1 | The machines exist, and every result names its machine (a precondition of every spike here) | RS, CX33 and MAC recorded (model, CPU as the operating system reports it, cores, RAM, operating system); ML and 
- docs/adr/0004-simulation-core.md:1120 [CX33]: | PM-0004-4 | How the cost grows with N, and on the cheaper host (S1; part 1's one thread per match; decides R1-ECS-OQ4 and R1-TICK-OQ5) | reported, no pass number: mean, p50, p95, p99 and max at N = 12, 24, 36, 48 and t
- docs/adr/0006-infrastructure-operations.md:1189 [CX33, OQ-0006-2, SH, hel1, nbg1]: x86-64 AMD) in the location OQ-0006-2 picks (`nbg1` or `hel1`). **SH**, the shared CX33 (4 vCPU, 8 GB) beside it,
- docs/adr/0006-infrastructure-operations.md:1191 [CX33]: never the VM under test (proposed: bot clients on the VM under test would spend the vCPUs the marks measure): CX33s
- docs/adr/0006-infrastructure-operations.md:1192 [CX33]: ("the CX33 runs the simulated clients": SPIKE-MACHINES, Mustafa, 2026-11-03), as many as a 5-minute calibration run
- docs/adr/0006-infrastructure-operations.md:1196 [address]: spend up to 1 vCPU per 48 sessions, ADR 0001); their count goes into the run plan and PM-0006-18. On LG's addresses
- docs/adr/0006-infrastructure-operations.md:1197 [address]: the per-address connection cap (ADR 0001) is raised to the run's session count, and the raised value is recorded
- docs/adr/0006-infrastructure-operations.md:1198 [MV]: (What a spike must prove, item 2). **MV**, a
- docs/adr/0006-infrastructure-operations.md:1199 [CX33]: CX33 in the same location running Prometheus, standing in for the staging game server, which exists only from slice 3
- docs/adr/0006-infrastructure-operations.md:1214 [Every server mark]: ([time statistics](../research/GLOSSARY.md#time-statistics)). Every server mark runs at the 30 Hz baseline
- docs/adr/0006-infrastructure-operations.md:1267 [CX33, OQ-0006-1, OQ-0006-2, SH, every server row, hel1, nbg1]: | PM-0006-11 | The reference server exists and is the machine every server row ran on (part 5; OQ-0006-1, the precondition) | a CCX23 and a CX33 created in `nbg1` or `hel1`, after PM-0006-12's round trips and OQ-0006-2's
- docs/adr/0006-infrastructure-operations.md:1268 [CX33, OQ-0006-2, R6-HOST-OQ2, address, fsn1, hel1, nbg1]: | PM-0006-12 | The round trip to each candidate location (part 5; feeds OQ-0006-2) | reported, no pass number: per location, the mean and the max round trip and the loss of 100 ICMP echoes, as Windows' `ping` prints them
- docs/adr/0006-infrastructure-operations.md:1269 [SH]: | PM-0006-13 | Rooms per VM: 8 rooms at N = 48 on each machine (part 5; with ADR 0001's R2-ROOM-REC2 marks 1 and 2: one set of runs serves both records) | each of 8 rooms' server tick p99 at most 1.10 × the single-room f
- docs/adr/0006-infrastructure-operations.md:1270 [SH]: | PM-0006-14 | Rooms per VM counted, the figure that replaces A-2 (parts 5, 8; decides R6-COST-OQ1) | reported, no pass number: on each machine and at N = 48, 36 and 18, the most rooms k for which every room meets R2-ROO
- docs/adr/0006-infrastructure-operations.md:1271 [MV, address]: | PM-0006-15 | The firewall lets both game transports in and nothing else (part 5) | from each of the four browsers, 20 of 20 WebTransport sessions over UDP and 20 of 20 WebSocket sessions over TCP (`wss`) reach a room o
- docs/adr/0006-infrastructure-operations.md:1272 [MV, address]: | PM-0006-16 | The metrics path works with only the rule it needs (part 9) | every metric of PM-0006-22 reaches Prometheus on MV; if part 9 picks the scrape: the exporter's port answers MV's address only and drops every 
- docs/adr/0006-infrastructure-operations.md:1274 [CX33]: | PM-0006-18 | The spike's own hosting bill (part 8) | under the cap OQ-0006-20 sets, excluding VAT, for every Hetzner VM-hour and traffic block the netcode proof test creates (all its marks, ADR 0001's included, LG's ca
- docs/adr/0006-infrastructure-operations.md:1277 [address]: | PM-0006-21 | Logs keep no secrets or addresses (parts 9, 11) | 0 matches in a 30-minute soak's logs for every join token, every guest secret and every client address the soak used, and for each match's random seed (ADR
- docs/adr/0006-infrastructure-operations.md:1278 [MV]: | PM-0006-22 | Every first metric exists and moves (part 9) | after a 30-minute soak of 8 rooms at N = 48: every metric part 9 lists is present on MV, and each counter the soak's script triggers has moved (proposed: a pr
- docs/adr/0006-infrastructure-operations.md:1279 [MV]: | PM-0006-23 | A stall is visible at once (part 9) | after a deliberate 1 s stall inside one tick (proposed size: longer than the catch-up bound, so time is dropped), the first scrape shows the overrun counter up by exac
- docs/adr/0006-infrastructure-operations.md:1280 [MV]: | PM-0006-24 | The metrics stay small and hold no player or match id (parts 9, 11) | the series count for 8 rooms on one VM reported and under 5,000; every series' label names within {VM, room slot, mode id, transport} p
- docs/adr/0006-infrastructure-operations.md:1288 [address]: | PM-0006-31 | Crash reports carry no IP address (parts 10, 11) | 0 IP addresses in a sample of 100 events: no match for the lab clients' public addresses and none for an IPv4 or IPv6 address pattern | Sentry's EU organi
- docs/adr/0006-infrastructure-operations.md:1292 [SH]: is complete; an adoption mark (PM-0006-13 on SH, PM-0006-14 on SH) never fails the spike: its result only keeps or
- docs/adr/0006-infrastructure-operations.md:1300 [OQ-0006-1]: OQ-0006-1 (b) and a new reference server, which every draft citing N34 follows; a firewall, logging or metrics fail:
- docs/adr/0006-infrastructure-operations.md:1349 [address]: 1. Open a Windows terminal and run `ping -n 100 <address>` for each address the session sends (one per candidate


## The advisor's list (amended condition 1): these places must be covered too, whatever the search found
- ADR 0006: 119, 189, 244-254, 267, 325-326, 352, 373, 607, 632-634, 713, 715, 753, 768, 799-816, 825, 1073-1077,
  1133-1134, 1189-1199, 1267-1280, 1300, 1364-1370, 1435, 1446-1448, 1499, 1510-1517
- ADR 0001: 1293, 1350-1357, PM-0001-1, -16, -27, -63
- ADR 0004: 1030-1032, PM-0004-1, -4
- README: 21, 23-25, 133, 134 (and a new row for OQ-0006-22)
- SPIKE_PLAN: 30-33, 129-130, 176, 281, 513
