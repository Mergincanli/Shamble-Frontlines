Task: netcode-proof-test, netcode-architect digest of ADR 0001 for the proof-test plan (SPIKE_PLAN item 1), 2026-11-05.

## Plan (blocks, in order)
1. Read ADR 0001 "What a spike must prove" 1237-1347 (notes)
2. Read "### Pass marks" 1348-1630 in chunks (notes -> section A rows)
3. Read "The netcode checklist" 1631-1683 (-> section D)
4. Read budgets 718-736, components 591-626, ownership 627-653, messages 654-682, bandwidth 683-717, failure 737-758, anti-cheat 759-781, test plan 1727-1758
5. Grep/read Decision part 13 (profiles, bursts) + parts for tick/snapshot rates, encoding, interest, transport/crates, input buffer, rewind cap
6. Read Open questions OQ-0001-5,-6,-7,-15,-16 and any others named by marks
7. Read GLOSSARY s2, s3, s8, time-statistics rules
8. Write sections A-I
9. DONE

Memory: no agent memory dir found (memory: local, none on disk). No earlier decisions recorded.
Blocks 1-8 done. Raw reading notes (line-cited) are in netcode-architect-notes.md beside this file.

# DIGEST (sections A-I). Lines are docs/adr/0001-multiplayer-netcode.md unless named. "Rec:" = my recommendation.

## A. Pass marks (rows in ADR order, 1511-1578; no lettered sub-marks exist)
Common keys: S30 = 48 clients joined, 60 s warm-up after last first-full-state ack, 30 min measured = 54,000 ticks, tick
counter starts 1,800 below 65,536, doors 1/s (1451-1459). "x3" = 3 runs, each passes, V-S validity, rule B-S bite (1486-1504).
Judge = script unless named (SPIKE_PLAN:146). Arm = Arm rule (ii) applies (1337-1346, 1381-1384). [60] = 60 Hz group,
OUTSIDE this change-set's close (1580-1585).

| id | measures | exact threshold | setup | judge | cond | line |
|---|---|---|---|---|---|---|
| -1 | rig named, unit holds | RS/LG: plan, CPU, vCPU, mem, location, kernel, day's address; ML/RD/MAC per ADR 0003; tick thread alone on 1 vCPU (not -27/-63: affinities recorded); steal per s; lscpu threads/core + SMT map, tick's sibling free of transport | header per machine before first run; affinities at start+end of every run; bite: 2nd tick thread on same vCPU flagged invalid | script | precondition | 1511 |
| -2 | load is SS | every measured tick: 48 soldiers 3x16 each with snapshot client; 15 big vehicles (4 of >=20 parts); 300 live projectiles; <=8 effects/soldier; 700 shots/s (mean +-5 %); portals 1/s; downstream rows: all 47 others in set on >=99 % of snapshots; stand-ins at same counts; S1 stand-ins (384 rows, 1 ms burn, top-up to 2.80 ms) named; ES counts/rates checked +-5 % | RS counters, LG; every run of -4..-27, -39, -63; bite: firing stops after 10 min | script | precondition | 1512 |
| -3 | profiles true | P1-P4, B1, L1 and L2, each direction over 100,000 packets: loss, mean one-way delay, jitter within 5 % relative of per-direction values; RTT mean within 5 %; L1 same seed -> identical packet-fate log twice; P0 on L2 recorded; MP: server RTT median 100 +- 5 ms at P2, base RTT recorded | L1; L2 on RS with LG, ML, MP; base RTT = median of 10 s pings; once per profile+layer, and at start of every L2 session | script | precondition | 1513 |
| -4 | tick budget 30 Hz | p99 <= 16.67 ms; max < 33.33 ms; 0 overruns; 0 dropped time | 1 vCPU RS; SS, P2, S30; x3 | script | "stand-in" label if sim lighter (1432-1435) | 1515 |
| -5 | two shares | sim step p99 <= 8.33 ms; server-only p99 <= 8.33 ms (lag-comp queries in server-only) | -4's runs; B-S per share | script | no | 1516 |
| -6 | encode (R2-BUD-OQ9) | p99 <= 50 us per client per snapshot; reported mean, p50, max, bytes | -4's runs (1,440 samples/s) | script | no | 1517 |
| -7 | lag comp cost/size | rewound shot vs 48 candidates p99 <= 20 us at 700 shots/s; history <= 1 MB (read each min) | -4's runs; bite: untrimmed history | script | no | 1518 |
| -8 | interest | region query p99 <= 0.2 ms/tick; rank+pack p99 <= 0.5 ms/tick; 0 snapshots > 1,067 B per interval (UDP payload + 28 B/datagram); nearest 8 enemies in every snapshot; every relevant entity >=1 per 4 snapshots | -4's runs, fight in centre; bite: rank by id | script | no | 1519 |
| -9 | enclosure | pair test p99 <= 0.1 us/pair (768 pairs); re-flood p99 <= 1 ms; area count recorded | -4's runs, doors 1/s | script | no | 1520 |
| -10 | sync cost linear | mean server-only/tick / (clients x mean relevant entities) at 48 <= 1.1 x the figure at 24 and at 36 | SS N=24, 36 (3x8, 3x12), 10 min each at P2, 3 runs per N, V-S | script | no | 1521 |
| -11 | transport runtime | <= 1 vCPU (p99 of 1-s windows), 48 sessions x 30 in + 30 out/s; send call p99 <= 1 ms | -4's runs | script | no | 1522 |
| -12 | downstream | ES: mean <= 160 kbps, p99 of 1-s windows <= 256 kbps; SS: every 1-s window <= 256 kbps, 0 datagrams above cap rule, mean reported beside R2's formula at run's counts; per client, send side, UDP payload + 28 B; IPv6 figure reported | ES at P2 10 min x3; SS at P2 in -4's runs; B-N | script | no | 1523 |
| -13 | upstream (R2-XU-OQ4) | mean <= 48 kbps per client (IPv4, UDP payload + 28 B); reported: p99 1-s windows, largest datagram, commands/datagram (median, max), command size whole and delta (U1 vs U2), same at P4 | P2 in -4's runs; P4 10 min; B-N | script | no | 1524 |
| -14 | no oversize datagram | 0 snapshot datagrams > cap (min(smallest maxDatagramSize of -36, 1,150 B); 1,000 B until measured); 0 > any browser's max; 0 fragmented IP packets | RS, LG, ML+MAC; every S30 + browser session; packet capture on RS | script | runner may be MAC (1402) | 1525 |
| -15 | room memory (R2-ROOM-OQ1) | RSS <= 64 MB at start and after 20 min; peak of 1-s samples <= 64 MB; slope within 1 MB per 10 min; each per-client queue: max of last 10 min <= max of first 10 min | S30 P2, as -4; bite: planted leak | script | no | 1526 |
| -16 | room start | every start, spawn to Open, < 2 s; >= 20 starts | RS, supervisor stand-in | script | no | 1527 |
| -17 | decoded = server | 100 % of decoded snapshots rebuild server's quantised state by hash, incl. 16-bit wrap of every S30 | L1 48 (3 seeds/profile) + L2 LG 48 (1 run/profile); 10 min each P0, P2, P3, B1; LG logs every byte received | script | Arm | 1528 |
| -57 | precision | max error: positions <= 1 cm; yaw <= 0.01 deg; pitch <= one 0.18 deg step | -17's L1 runs, 3 seeds x P0/P2/P3/B1 | script | no | 1529 |
| -58 | counter wraps | 100 % hash match; 0 commands refused/lost/doubled; 0 presses lost/doubled; 0 corrections > 1 mm | L1 48; tick 30 below a multiple of 65,536, every <=16-bit counter a few steps below wrap, 10 s; P0 and P3; 3 seeds/profile | script | reorder rule (1448) | 1530 |
| -18 | burst heals | every client decodes deltas <= 500 ms after link returns (plain limit) | B1, 48 clients, 10 bursts/run; L1 3 seeds, L2 LG 1 run | script | no | 1531 |
| -19 | join/resync | every join: complete world (hash) <= 300 ms after Join at P2 (plain); reported p50, p99, max, from first connect | L2: LG + ML Chrome; 100 joins into a running S30 room, incl. rejoins after Worker traps | script | Arm (LG); ML as it stands | 1532 |
| -20 | commands in time | P3: >= 99.9 % before their tick; missing inputs <= 0.1 % of player-ticks; 10,000 presses with reordering: 0 lost, 0 doubled; buffer depth recorded | L1 3 seeds + L2 LG 1 run; 48 clients 10 min under EACH OQ-0001-6 option; bite: cap 1 | script | reorder rule | 1533 |
| -21 | forged, fast clock, lag switch | every forged kind rejected+counted, 0 effect beyond valid part (hash equality); beyond-reach aim clamped, not counted (hash equality); 1.5x clock for 60 s runs <= server ticks + lead bound, moves <= honest + that many ticks; lag switch (200 ms hold at P1) <= catch-up cap | L1; 1,000 per forged kind; 1,000 per weapon kind; 100 bursts; 10 fast-clock runs; each OQ-0001-6 option; 3 seeds | script | no | 1534 |
| -22 | never-acking client | its event queue held at cap; resynced or dropped | L1, 1 of 48, 10 min, 3 seeds | script | no | 1535 |
| -23 | throttle | 128 kbps for 10 s: <= 2 missing snapshots/s; own state in 100 % of snapshots sent; full rate back <= 2 s (plain) | L2 rate limit on RS + LG; 10 throttles, P2 | script | no | 1536 |
| -24 | bottleneck (R2-BUD-OQ10) | at 50 % of snapshot rate: newest snapshot <= 3 intervals old (100 ms); memory/session flat 10 min; tick p99 within 5 % of clean run; NewReno and Cubic each, lower p99 age picked | L2 on RS + LG; 10 min per controller | script | no | 1537 |
| -25 | hostile input | flood (1,000 never-Join sessions + 10 oversize/stream clients): tick p99 within 10 % of clean, RSS < 64 MB; same over WS + oversize frames/messages; ws:// and wrong Origin refused (403) from 4 browsers; no Join 2 s after TCP accept/QUIC first packet -> closed; 10 slow-loris closed by deadline, handshakes at cap; cap on: first over cap refused, also from 1,000 addresses in one /64 (where LG has IPv6) | RS + LG (flood), ML + MAC (Origin); 10 min per flood; twice (cap raised, cap on); 1 run per transport and cap | script | runner may be MAC for Origin | 1538 |
| -26 | connection panic | that connection closes; 0 of the other 47 drop; 0 overruns next minute | RS + LG; 10 injections in one S30 room | script | no | 1539 |
| -59 | tick panic | exit non-zero <= 1 s (plain); every client RoomEnded(crash) or closed in that second; match record marked crashed + crash report kept; supervisor stand-in logs exit; report in error log | RS + LG; 10 injections, each its own S30 room at a random tick | script | no | 1540 |
| -27 | rooms share a VM | 8 rooms = 8 processes: each p99 within 10 % of single-room (bracket B: mean of lone room's p99, first and last in session) over 20 min; SIGKILL one -> others within 5 %, 0 dropped | RS + LG; PM-0006-13 setup, unpinned, SS with snapshot clients (8 x 48 = 384 sessions); 3 sessions at 3 times of day | script | validity PM-0006-13, not V-S | 1541 |
| -28 | WT crates x browsers (R2-TRN-OQ1) | wtransport 0.7.x AND web-transport-quinn 0.13.x: Chrome, Edge, Firefox, Safari each >= 99 of 100 attempts, datagrams + 1 bidi stream; draft recorded; control: wrong hash fails 100 of 100 | ML (Chrome, Edge, Firefox), MAC (Safari); server on the same LAN | script | runner may be MAC, server then RS (1401) | 1543 |
| -29 | fallback (R2-TRN-OQ2) | UDP blocked: each of 4 browsers playing WS <= 3 s after page starts connecting (plain; WT timeout 1.5 s); reported: one attempt per browser from a TCP-443-only network | ML, MAC; RS UDP range closed; 20 attempts/browser, one at a time | script | Safari: time row -> borrowed Mac | 1544 |
| -30 | same server, both transports | one replay -> identical server hashes every tick on WT and WS; valid only if every command ran at its stamped tick and late counter = 0 | RS; 3 recordings x 5 min, P0, buffer deep enough (recorded) | script | invalid, not failed, if late > 0 | 1545 |
| -31 | WS stalls | WS at P2: p99 extra snapshot delay <= 250 ms (age at decode minus 50 ms); never > 1 unsent snapshot per client | L2 TCP proxy on RS; LG; ML Chrome; 10 min | script | no | 1546 |
| -32 | one build everywhere | same build hash, scripted 60 s per browser per backend: 0 console errors, 0 page errors; feature detection covered | ML, MAC | script | Safari -> borrowed Mac (not in 1402 list) | 1547 |
| -33 | RTT agreement | p99 of abs(app RTT - quinn rtt()) per s over 10 min <= 5 ms at P2; also recorded at fixed 100 ms, no jitter/loss | L2 on RS; ML Chrome | script | no | 1548 |
| -34 | fuzz | 10 million random+mutated inputs per message type: 0 panics, 0 allocations above limits | any machine; cargo-fuzz + counting allocator | script | no | 1549 |
| -35 | swappable, current | crate swap: no code change outside transport module; sim crate has no transport dep (cargo tree); quinn-proto >= 0.11.18, rustls >= 0.23.46, ring >= 0.17.12, tokio >= 1.54.1; wtransport dangerous-configuration off | any machine; once after swap | script | no | 1550 |
| -36 | maxDatagramSize (R2-BUD-OQ3) | recorded at start and every 10 s, 4 browsers: >= 1,000 B on loopback and on a real EU path; else cap = smallest measured | ML, MAC; local server + RS from an EU consumer connection; 10 min per browser and path | script | Safari -> borrowed Mac | 1551 |
| -37 | datagrams in time | lost-or-late snapshots <= 3 % at P2 (loss + 1 pp); late = > 66.67 ms after expected arrival (send + 50 ms); research reading reported | L2 on RS; LG 48; 10 min | script | no | 1552 |
| -38 | one-shots, bulk | P3: 0 of 10,000 one-shot actions dropped; 1 MB bulk transfer raises control-stream p99 by <= 5 ms | L2 on RS; LG; 10 transfers | script | no | 1553 |
| -39 | [60] margin | 60 Hz: tick p99 <= 6.25 ms; shares <= 4.17 ms each; max < 16.67 ms; 0 overruns/dropped; encode p99 <= 50 us; hash across tick 65,536 | 1 vCPU RS; SS P2 S60 (108,000 ticks), snapshots every 2nd tick; 3 whole runs | script | OUTSIDE close | 1555 |
| -40 | [60] R1 marks | P1 at 60 Hz (sim p99 <= 4.17 ms, max <= 16.67 ms, 0 overruns, 36,000 ticks, 3 runs); P2 (client sim share p99 <= 6.67 ms, Wasm Worker on ML, page 30+ fps on Low); P3 (12-tick replay <= 1 ms p99 on ML) | RS (P1), ML | script | OUTSIDE close | 1556 |
| -63 | [60] goal budget | k60 search; fleet ceil(20 / k60) + 1 CCX23 at 86.49 EUR + 8.99 + 6.49; wave 1: reported beside ~361 / ~534 EUR, decides nothing; bite: k60 = 1 -> 21 CCX23, 1,831.77 EUR | RS + LG; 20 min, 72,000 ticks (p99 = 71,280th) | script | OUTSIDE close | 1557 |
| -41 | prediction exact | CO, P0-P4: 0 corrections > 1 mm when every command in time; 100 % of others traced to late/lost | L1 3 seeds/profile; ML Chrome over L2, 1 run at P2; 10 min/profile | script | no | 1559 |
| -42 | corrections in a crowd | P3 brawl of 48: > 5 cm in < 1 % of snapshots; never > 5 cm in one frame below snap (2 m) | ML Chrome over L2 + LG 47; L1 3 seeds; 10 min; collision rule recorded | script | pass holds for named rule only | 1560 |
| -60 | server change to own move/fire | P2: <= 1 correction, at first snapshot carrying it; then 0 > 1 mm (in time); first correction size reported | L1 3 seeds; ML Chrome L2 1 run; 100 changes each (+-10 %) | script | no | 1561 |
| -43 | latency same at any profile | input-to-screen (own move, muzzle flash): p99(P4) - p99(P0) <= 33.33 ms, same for medians; reported: absolute P0 p50/p99 on ML and RD | ML and RD, Chrome WebGPU, presented frames, 60 Hz display; 200 CDP inputs per profile+kind; isolation flag | script | named machines only | 1562 |
| -44 | replay fits | 6-tick replay <= 1 ms p99 AND 7-tick <= 1 ms p99; reported depth P0-P4 and forced-depth times at P3/P4 | ML Chrome + Firefox, Wasm Worker; SS fight P2 10 min, forced replays; 3 runs per depth (18,000; p99 = 17,820th) | script | no | 1563 |
| -45 | extrapolation | < 1 % of frames at P2, < 3 % at P3; B1: 0 frames beyond one interval | ML Chrome L2 (1 run/profile); L1 3 seeds; 10 min/profile | script | no | 1564 |
| -46 | delay adapts, no slides | settles <= 5 s after profile change (within +- half interval of final = mean of next 30 s); stays in 2-5 intervals; announced change > 1 interval/s clamped; 0 slides in 1,000 respawns+teleports (slide: > 0.1 m from both ends) | ML Chrome L2; 20 changes each way P1<->P3 | script | no | 1565 |
| -61 | events with poses | P2: first frame playing event <= 50.00 ms before first frame drawing entity at/past its tick, never after; own/owner events in emitting tick's block; p50/p99/max reported | L1 3 seeds (0 exceptions); ML Chrome L2 + LG 47; 10 min | script | no | 1566 |
| -47 | clock | offset error <= 5 ms after 2 s at P2 (p99 over 1,000 connects, random offsets) | L1 3 seeds | script | no | 1567 |
| -48 | hits register (OQ-0001-7) | TC sprinter at 9 m/s, P0-P3: view hits registered >= 99 % up to P2, >= 95 % at P3; false hits < 0.5 %; test cap above every honest rewind; replaced-view share recorded | L1 3 seeds; L2 LG shooter 1 run/profile; 2,000 shots/profile | script | Arm | 1568 |
| -49 | cap holds (OQ-0001-5 value, R2-XU-OQ2) | 100 % of shots rewound <= cap, honest + cheat, P0-P4; cap = honest rewind p99 at P2 rounded up to whole ticks, largest reported; clipped share + registration per profile; honest rewind median/p99/max with parts (RTT, delay, buffer wait, blend, frame-to-tick wait) | L1, L2 LG; -48's shots, then a run with the rule cap; -50's cheats among them | script | no | 1569 |
| -50 | no bought rewind | edge-of-tolerance cheat gains <= 33.33 ms over honest (P0, P1); 150 ms pong delayer: with transport RTT gains <= tolerance, logged gap >= 145 ms every 1-s sample; WS extra rewind recorded | L1; 1,000 shots per profile+client; 3 seeds | script | no | 1570 |
| -51 | aim at height | ground / 2.5 m / 4 m, P0-P3: hit point within 5 cm in >= 99.9 %; aim <= 80 bits/command | L1 3 seeds; ML Chrome L2 at P2; 1,000 shots per height+profile | script | no | 1571 |
| -52 | feedback waits | 0 damage numbers/kills on others before server's event | ML Chrome L2; MP session; every browser session + 1,000 hits | script | no | 1572 |
| -62 | cues once | P2 forced replays: each own shot's flash+sound exactly once over 1,000 shots; 0 own cosmetic events sent to shooter | L1 3 seeds; ML Chrome L2 1 run | script | no | 1573 |
| -53 | "looks correct" | every checklist step "achieved"; 0 "not yet" | MP, Brave on Windows, P2, one session | Mustafa | no | 1574 |
| -54 | projectile flight / E3b per kind | 0 desyncs over 1,000 throws per kind | L1 3 seeds; L2 LG | script | only if live projectiles in load; Arm | 1576 |
| -55 | soldier beside vehicle | > 5 cm corrections in < 2 % of snapshots at P2 | L1 3 seeds, 10 min | script | only if sim moves vehicles | 1577 |
| -56 | records complete | every listed item has a number or value | as each row | script/read | no | 1578 |

Arm-rule rows: -17, -19, -48, -54 (1343); verdict from L1 x86-64 (CI's runner) or a browser; aarch64-only mismatch = cross-architecture finding, decides nothing (1344-1346).

## B. Profiles and bursts (part 13, 567-580; key 1436-1450)
| profile | RTT | jitter (RTT) | loss | per direction (each drawn on its own) |
|---|---|---|---|---|
| P0 | 0 ms | none | 0 % | 0 ms; on L2 = the real path, delay and jitter recorded (1513) |
| P1 | 50 ms | +-10 ms | 1 % | 25 ms +- 5 ms uniform, 1 % |
| P2 | 100 ms | +-20 ms | 2 % | 50 ms +- 10 ms uniform, 2 % (about 4 % of round trips lose a packet: 1 - 0.98 x 0.98 = 3.96 %, 1440) |
| P3 | 150 ms | +-30 ms | 5 % | 75 ms +- 15 ms, 5 % |
| P4 | 200 ms | +-40 ms | 5 % | 100 ms +- 20 ms, 5 % |
| B1 | P2 plus one 1 s burst of 100 % loss | | | (569; 1437) |
- Split (OQ-0001-16, 1204-1207; 570-574; 1438-1443): half the latency and a uniform +- half the jitter each way; loss at the
  profile's rate on EACH direction independently; lateness counted from expected arrival = send time + one-way delay (1443-1444).
- Other impairments (not profiles): the 128 kbps throttle for 10 s (-23, 1536); the bottleneck at 50 % of the snapshot
  rate (-24, 1537); UDP blocked (-29, 1544); the fixed 100 ms, no jitter/loss case (-33, 1548); the lag switch (200 ms
  hold under P1, -21, 1534). B1 is "one burst" (569) but -18 runs 10 bursts per run (1531, proposed).
- Reordering: whatever the jitter produces; each run records its reorder share; a mark that names reordering counts only
  if that share > 0 (1447-1450). See H2.
- Layers: L1 = our seeded link simulator on simulated time behind the thin transport interface, logic marks only, never
  a time mark (1410-1411, 1728-1729); L2 = impairment under quinn's socket plus a delaying TCP proxy for WebSocket, on
  RS, for real browsers and LG (1444-1445, 1729-1730). L2's injected delay per direction is set from "half of the
  profile's RTT minus the path's measured base RTT" (1513; see H12).

## C. Budgets for the netcode proof test (718-735, exact)
Every server mark: one vCPU of RS, SS at N = 48 with a snapshot-receiving client on every soldier, 30-min soak, re-flood ticks included, unless the row says otherwise (720-722).
| budget | value |
|---|---|
| Server tick time, 30 Hz | p99 <= 16.67 ms; max < 33.33 ms; 0 overruns (0 missed ticks) |
| Its two shares, 30 Hz | sim step p99 <= 8.33 ms (ADR 0004's parts); server-only p99 <= 8.33 ms, measured separately |
| Parts of server-only | encode <= 50 us/client/snapshot p99; rewound shot <= 20 us p99 at 700 shots/s; region query <= 0.2 ms p99; rank+pack <= 0.5 ms p99; pair test <= 0.1 us/pair p99; re-flood <= 1 ms p99 (reported beside the whole, never summed into it) |
| 60 Hz margin run (early signal) | p99 <= 6.25 ms vs 8.33 ms; each share <= 4.17 ms p99; R1-TICK-REC2 P1-P3 at 60 Hz; one run; rooms/VM at 60 Hz counted and goal budget at that count (PM-0001-63) |
| Prediction replay | 200 ms (6 ticks at 30 Hz) <= 1 ms p99, minimum laptop |
| Transport runtime | <= 1 vCPU per process with 48 sessions; p99 send call <= 1 ms (30 input datagrams/s/session) |
| Downstream per client | ES: mean <= 160 kbps and p99 of 1-s windows <= 256 kbps, UDP payload + 28 B/datagram; SS: every 1-s window <= 256 kbps, 0 datagrams above cap rule, mean reported (+~20.1 kbps of shots on R2's 146.1) |
| Upstream per client | <= 48 kbps (IPv4) under P2 |
| Memory per room | RSS reported at start and after 20 min, <= 64 MB at N = 48; per-session memory flat (no queue growth); hit history <= 1 MB |
| Room start-up | spawn to Open < 2 s |
Planning arithmetic: server-only parts 3.64 of 8.33 ms (4.64 on a re-flood tick); whole tick 9.18 of 16.67 ms (10.18) (173-175); with S1 stand-ins 4.69 ms left (1095-1096). Bandwidth: 584 B/snapshot (609 B with events), ~146.1 kbps; E3b ~188.3 kbps; cap 256 kbps = ~1,067 B per interval; upstream 25.9-43.9 kbps at P2, 43.9-53.5 at the cap of 8 if sent whole, hence U2 (684-710).

## D. Mustafa's netcode checklist (MP, Brave on Windows, at P2; 1631-1681; test-engineer writes it, 1304)
Setup (1635-1643): a proof-test room on RS with 47 bot clients and Mustafa in the 48th slot, plus TC (sprinter at 9 m/s, static
targets at ground, 2.5 m, 4 m). Overlay shows: profile, transport, server-measured RTT, interpolation delay, rewind cap,
counters (corrections, snaps, extrapolated frames, clipped shots). Profile and cap change from the overlay (a spike-only
control on the server's impairment layer); his base RTT measured at start and subtracted (PM-0001-3). Each step records
"achieved" / "not yet" (+ note) or the asked observation, in an on-page form that writes to the results log (proposed).
| step | do (condensed) | expected / record | line |
|---|---|---|---|
| 1 | open link, wait for "Playing" | playing within 3 s; overlay names transport and P2; record transport + Brave version | 1645-1646 |
| 2 | P0: walk, sprint, strafe, jump, crouch 1 min; fire at wall | instant response (the reference feel) | 1647-1648 |
| 3 | P2: same minute | own soldier and muzzle flash feel as at P0; no jump-back or stutter | 1649-1650 |
| 4 | P2: run through brawl of 47 bots 2 min | no visible correction; any pull-back glides, rare | 1651-1652 |
| 5 | P2: watch bots run/sprint/strafe/turn near and far | smooth, no warp/stutter/slide; turns not cut; record where fast movers look wrong (Hermite trigger, R2-SYNC-OQ12) | 1653-1655 |
| 6 | P2: watch deaths and respawns | respawn appears at new spot; nothing slides | 1656-1657 |
| 7 | P2 with the rule cap (from PM-0001-49, named on overlay): shoot sprinter 2 min | on-target hits, off-target misses; flash/tracer instant, hit marker on confirmation | 1658-1662 |
| 8 | P2: fight and kill bots | no damage number or kill before its hit marker | 1663 |
| 9 | P2: shoot static targets at 3 heights | every shot lands where clicked | 1664-1665 |
| 10 | P2: run into cover under bot fire, 10 runs, step 7's cap | hit-after-cover rare, never long after; record how often unfair | 1666-1668 |
| 11 | P2: trigger B1 from overlay while moving | others held (not flying off), own soldier responds; world catches up within 0.5 s; at most one correction (glide, or jump if > 2 m) | 1669-1672 |
| 12 | P3 then P4, 2 min each in brawl and at TC | still playable, own soldier/gun instant; P4 not held to P2's feel | 1673-1676 |
| 13 | WS forced (UDP blocked) at P2: steps 3, 5, 7 briefly | playing within 3 s; like those steps, brief stalls on loss allowed | 1677-1679 |
| 14 | verdict at P2 on WebTransport | "achieved"/"not yet" for prediction (3, 4), reconciliation (4, 11), lag-compensated hits (7-9) | 1680-1681 |
PM-0001-53 passes when every step is "achieved" (1574); a "not yet" goes back to the draft and is shown again (1604-1605).
On screen he needs: the overlay (above), the controls for profile/cap/B1/UDP-block, the record form. Order constraint: step
7 needs the cap PM-0001-49 sets, so his session comes after the honest-rewind runs (my reading of 1658-1659).

## E. Build list the marks imply (all throwaway, in spikes/: 1248, 582-583)
| # | component (what the marks need) | marks | design fixed by |
|---|---|---|---|
| E1 | Throwaway sim crate, native + Wasm: movement, hitscan, bots as protocol clients (1428); stand-ins at SS counts, replicated/hit-tested/interest-filtered: 15 big vehicles (4 of >=20 parts; 11 moving elsewhere), 300 live projectiles as trajectory records through one shared trajectory function, S1 effect stand-ins (384 rows, 1 ms burn, top-up to 2.80 ms) (1512); DM_Tri-sized greybox with enterable buildings, areas and portals, door script 1/s (1414-1418, 1457-1459); TC and CO courses (1425-1427); scripted respawns/teleports (-46), server-set move-speed/fire-interval stand-in attribute (-60); a named soldier collision rule (-42); seeded RNG, authority-only spread stream, no clock reads; per-tick state hash | -2, -4, -5, -17, -30, -41..-62, -54, -55 | parts 4, 6, 8, 9; 1428-1435; SPIKE_PLAN:160-164 |
| E2 | Room server process: tick thread pinned alone to 1 vCPU, async I/O runtime on the others, bounded channels only, panic = "unwind", top-level catch_unwind -> RoomEnded(crash) + crashed match record + crash report + exit non-zero; per-phase timers and counters (overruns, dropped time); supervisor stand-in (spawn, Open timing, exit log, SIGKILL) | -1, -4..-11, -15, -16, -26, -27, -59 | parts 3, 11 (482-491); 612-617 |
| E3 | Commands: Input datagram (tick, seq, move, buttons, press counters, aim point <= 80 bits, view time; newest-decoded ack), redundancy cap 8, U2 delta repeats; every server check of 231-239 (strictly newer tick, lead bound, catch-up cap <= redundancy cap, clamp beyond reach); per-client input buffer 1-3 ticks adaptive; BOTH OQ-0001-6 options switchable; late-command counter | -13, -20, -21, -30, -49, -56, -58 | part 5 (220-275); OQ-0001-6 (1135-1146) |
| E4 | Snapshots: delta vs last acked, per-type defaults, shared world ring + per-client ring of 32; E3 quantisation (1 cm in 16/16/12 bits, yaw 16, pitch 10); 32-bit header tick, 16-bit offsets; entity-group split, each datagram decodes alone, acked per group; cap 1,000 B until -36 measures; FullState on its own uni stream; Resync; bounded event queue resent until acked, overflow -> resync/drop; cosmetic once, never own shots; per-client 256 kbps cap; 20 Hz fallback under pressure; WS newest-only | -6, -8, -12, -14, -17, -18, -19, -22, -23, -31, -57, -58, -62 | part 6 (276-330); 424, 440-441 |
| E5 | Interest: region circle with hysteresis on a uniform grid; areas/portals flood-numbered, re-flood on change, constant-time pair test; priority accumulator (own state always, nearest 8 enemies every snapshot, every relevant entity >= 1 in 4) | -8, -9, -10, -12 | part 9 (396-418) |
| E6 | Transport: thin interface (send unreliable, send reliable on a named channel, receive, link state, RTT) outside the sim crate; WT on quinn 0.11.x via wtransport 0.7.x AND web-transport-quinn 0.13.x (swappable); WS via tokio-tungstenite 0.30.x binary; one bidi control stream, FullState uni stream; limits of 441-452; Join 2 s deadline from TCP accept/QUIC first packet; handshake cap; per-address cap keyed /32 and /64; Origin allow-list (403); wss only; NewReno/Cubic selectable; non-waiting send_datagram + buffer-space check; version floors | -11, -14, -24, -25, -28..-38 | part 10 (419-473) |
| E7 | Client harness (shared with the boundary spike, SPIKE_PLAN:179-181): TS page + Worker running client-endpoint as Wasm; Worker owns the connection; page posts newest input + cursor ray + view time per frame; host passes datagram arrival times in (endpoint reads no clock); WT first with 1.5 s timeout, then WS; requireUnreliable; plain renderer; error hook; per-frame logs (corrections drawn, extrapolated frames, events played, cues played, damage numbers shown); maxDatagramSize every 10 s; isolation flag + crossOriginIsolated read in the Worker; CDP-driven inputs (-43); 100-session connect page (-28); overlay + record form (D) | -19, -28, -29, -31..-33, -36, -41..-46, -51..-53, -61, -62 | parts 4, 5, 7, 10; 610-622; 1473-1485 |
| E8 | Prediction/reconciliation in client-endpoint: reset to state of last run command, replay newer, skip on match; fade 100 ms, snap 2 m, clear on respawn; own-state section incl. changed attributes; cues keyed (entity, weapon, command sequence) | -41, -42, -43, -44, -60, -62 | part 4 (185-219) |
| E9 | Interpolation: buffer in the Worker; delayed time; delay starts at 3 intervals, adapts 2-5 from arrival-stamped jitter, announced via ClientSettings and clamped (<= 1 interval/s); extrapolate <= 1 interval then hold; respawn/teleport/first appearance placed; spatial events into first block reaching their tick; linear (Hermite only on artefacts) | -45, -46, -61, -56 | part 7 (331-357) |
| E10 | Clock: Ping/Pong with hold time (server answers on the I/O runtime), Quake III drift rules, samples timed at datagram arrival | -33, -47, -50 | parts 5 (239-250), 7 |
| E11 | Lag compensation: per-tick hit-volume history (cap + margin, <= 1 MB), pure rewind query, ray from server fire origin to aim point, view-time tolerance 33.3 ms vs estimate (RTT + delay + buffer wait + half tick), server-side RTT (quinn rtt() / WS lowest ping), switchable cap (test cap above all honest rewinds; then the rule cap), filters, bots at present; per-shot log of RTT, delay, buffer wait, blend, frame-to-tick wait, rewind; quinn rtt() beside ping RTT per second | -7, -48, -49, -50, -51 | part 8 (358-395) |
| E12 | Headless bot clients on client-endpoint (native aarch64 on LG, x86-64 on L1/CI): 30 inputs/s, scripted brawl, TC shooter, cheats (forged kinds, 1.5x clock, lag switch, 150 ms pong delayer, tolerance-edge stamper, never-acker), raw byte logging for re-decode | -2, -17, -19..-22, -48, -50, -54 | 1740; 514-516; 1371-1372 |
| E13 | Load generator orchestration: clients spread over CAX21s, 5-min calibration (< 80 % CPU every second), inputs/s within 1 % check, MV role on one CAX21, per-address cap raised and recorded | V-S of every LG row; -25 | 1372-1380, 1489-1490 |
| E14 | Hostile clients and fuzzing: 1,000 never-Join sessions, oversize datagram/stream openers, slow-loris (TLS ClientHello, WS upgrade), 1,000-address /64 cycler, ws:// and wrong-Origin pages; cargo-fuzz per message type with a counting allocator | -25, -34 | part 10 (441-452); 1744-1751 |
| E15 | Network simulator: L1 (seeded, simulated time, packet-fate log, behind the thin interface) and L2 (impairment under quinn's socket per peer, delaying TCP proxy for WS, rate limit, bottleneck, per-session profile switch, B1 trigger, UDP block); reorder share per run | -3 and every P1-P4/B1 row | part 13; 1444-1450, 1728-1730; R5-TEST-OQ2 |
| E16 | Density harness: 8 room processes unpinned under the supervisor stand-in, bracket B (lone room first and last), SIGKILL, 384 sessions | -27 (later -63) | part 11; PM-0006-13 |
| E17 | Recordings and hashes: recorded input replays (3 x 5 min) driving both transports; per-tick server state hash leaving authority-only streams out (323-325); server's per-client quantised hash log; client's decoded hash log; x86-64 re-decode/replay tool for the Arm rule | -17, -19, -21, -30, -48, -54, -57, -58 | 1731-1734; 1337-1346 |
| E18 | Instrumentation: monotonic reads at tick and phase edges every tick; clock resolution per machine (10^6 reads); sub-us parts timed per tick / count; per-thread CPU from /proc each second; RSS each second; queue lengths; bytes per client at UDP level (payload + 28 B); packet capture for fragments; steal and affinity reads | -1, -4..-16, -24, -25 | 1473-1485 |
| E19 | Planted-defect builds: one bite per automated mark, 5-min runs, B-S/B-N sizing | every automated row | 1499-1504 |
| E20 | Results pipeline: per-run log (data, seed, build commit, machine, browser, profile), nearest-rank percentiles, one judging script per row, run plan with VM-hours | all | 1465-1472, 1505-1506 |
| E21 | Hosting and certificates: page served over HTTPS from its own origin on the domain; WT self-signed cert valid <= 14 days whose hash the page fetches from that origin; WS on a browser-trusted cert on shamblefrontlines.com, issuance method set with security-auditor | -25, -28..-32, -36, -53 (and ADR 0006's PM-0006-15) | 455-467; 1272-1277 |
Not needed here: lobby, real join tokens (test tokens, 1616), reconnect, private rooms, economy (1615-1618); the loopback backend is the foundation's (1613).

## F. Decisions the test makes (1321-1335; SPIKE_PLAN:196-204)
| id | rule that decides | run that feeds it |
|---|---|---|
| R2-TRN-OQ5 (the WT crate) | no rule beyond the marks; netcode-architect picks with a reason (1578, 1622); a crate failing -28, -11 or -25 is replaced by the other; both failing -> drafts, Mustafa + Advisor (1596-1597) | -28, -11, -25, -35 for both crates |
| R5-TEST-OQ2 | whether each candidate runs on a custom quinn endpoint so L2 sits under it (1322-1323) | L2 build with each crate |
| R2-BUD-OQ3 = R2-TRN-OQ3 = R2-XU-OQ5 (datagram cap) | cap = min(smallest maxDatagramSize, 1,150 B); below 1,000 B anywhere -> smallest measured (291-293, 1551) | -36 |
| E3b per projectile kind (R2-BUD-REC2 flight mark) | only a kind that still desyncs after the trajectory function is fixed (PM-0004-73) rides every snapshot; never whole E3b (1597-1599) | -54 (if projectiles in load) |
| R2-ROOM-OQ1 (process model) | one room per process unless -15 or -16 fails -> option B, many rooms per process (1599; 482-484) | -15, -16 |
| R2-BUD-OQ9 (encode cost) | measured against 50 us p99 | -6 |
| R2-BUD-OQ10, R2-TRN-OQ8 (controllers under loss) | lower p99 snapshot age of NewReno vs Cubic (1537); browsers' snapshot rate and age at P2-P4 recorded | -24, -56 |
| R2-TRN-OQ1, R2-TRN-OQ2 | negotiated draft per browser; behaviour with UDP blocked | -28, -29 |
| OQ-0001-5 value; R2-XU-OQ2 (honest rewind) | honest rewind p99 at P2, rounded up to a whole number of ticks; largest rewind reported (1329-1331, 1112-1119) | -48 (test cap) then -49 (rule cap) |
| R2-XU-OQ15, R2-XU-OQ4 (sizes) | measured record sizes per entity kind; command size whole vs delta | -13, -56 |
| R2-SYNC-OQ12 = R2-BUD-OQ7 (linear vs Hermite) | Hermite only if artefacts on fast movers at 30 Hz (340-341); smoothness measure beside checklist step 5 (1578) | -56, D step 5 |
| OQ-0001-6 (command timing; R2-SYNC-OQ3) | of the options passing corrections, fairness, fast-clock bound, the smallest buffer; run on arrival = none (1622-1625, 1142-1146) | -20, -21, -42, -48, -49 under both options at P2-P4 |
| OQ-0001-7 (sub-tick shot stamps) | R2-SYNC-REC5 mark 1 registration at 30 Hz; fail or "feels late" -> Mustafa's Feel Lab (1151-1153) | -48 |
| R2-XU-OQ6 (join) | the one join mark, 300 ms | -19 |
| R2-XU-OQ3 | interpolation marks | -45, -46 |
| R2-SYNC-OQ5 (if vehicles move) | -55's 2 % | -55 |
| OQ-0001-15 | reported only; Mustafa decides by feel with netcode-architect (1200-1202) | -43's absolute P0 p50/p99 on ML and RD |
| R6-COST-OQ1 / R6-COST-REC1 | reported, not judged (1333-1334) | egress per 48-player match-hour; the spike's bill |
| R2-XU-OQ7 | listed in SPIKE_PLAN:197 and part 8's research ids (394); its rule: not found in what I read | (-49's parts, probably) |
| OQ-0001-9, cover half | "Decider: the netcode proof test for cover" (1162-1164), but no PM row, not in item 5 or -56 (see H9) | none defined |
PM-0004-67 (S17, the loop's elapsed-time tick) runs on this spike's server (1628-1629; SPIKE_PLAN:142, 192).

## G. What else the plan must deliver
- G1. Message catalogue under docs/design/ (.claude/rules/netcode.md:17; docs/design/ does not exist yet). Rec: one file for the
  spike's wire format: the proof-test subset of 654-678 (Input, Snapshot, Ping/Pong, Join with test token, JoinAccepted/
  JoinRejected, FullState, Resync, ClientSettings, RoomEnded, Leave/Kick) plus spike-only controls (profile/cap/B1/UDP-block
  from the overlay, 1640-1641), each with channel, visibility class (655), size limit and checks; marked throwaway.
- G2. The run record: a results-log header per machine (-1); per run: data, seed, build commit, machine, browser and version,
  profile (1505-1506), the sim and its stand-ins (1432, 1512), reorder share (1448), clock resolution (1475-1477),
  crossOriginIsolated (1481), refresh rate (1483), steal and LG CPU per second (1488-1490), per-address cap setting (1378),
  collision rule (-42), buffer depth (-20, -30), "stand-in" label (1434-1435), MP's base RTT (-3), Brave transport/version (-56).
- G3. A run plan per VM session with VM-hours and a cost cap for Mustafa's OK, the CAX21 count from calibration and which carries
  MV (1373-1377; SPIKE_PLAN:175-178).
- G4. Reported, not judged: egress per 48-player match-hour, the bill (1333-1334); absolute input-to-screen at P0 p50/p99 on ML
  and RD (1335); the 443-only attempt (-29); IPv6 downstream figure (-12); upstream p99, datagram sizes, commands per datagram,
  U1 vs U2, P4 counts (-13); encode mean/p50/max/bytes (-6); area count (-9); research-reading late share (-37); real replay
  depth (-44); first correction size (-60); event offsets (-61); replaced-view share (-48); largest rewind (-49); WS extra rewind
  (-50); the re-flood share and encode parity (1463-1464); cross-architecture findings (1345).
- G5. One bite per automated mark, proven failing in a 5-min run before its pass counts (1499-1504).
- G6. Results written into ADR 0001 at the close: crate, E3 + any per-kind E3b, process model, datagram cap, rewind cap (1689-1691).
  A mark's number changes only in the draft, never inside the spike (1603-1604).
- G7. Mustafa's checklist (test-engineer, 1304) with the overlay and record form; ADR 0006's operations checklist (SPIKE_PLAN:148-149).
- G8. The runner workflow (Safari leg), reviewed by security-auditor and infra-engineer (1403-1404); any .github/ change goes to
  Mustafa for approval (his request; CLAUDE.md protocol 3).
- G9. The SSH key route and the WS certificate's issuance method, set with security-auditor and the advisor (SPIKE_PLAN:154-155, 173-175).
- G10. The list of rows run before ML/RD are named, as rough checks never judged (SPIKE_PLAN:156-159; 1390-1393).

## H. Problems, each with a proposed handling inside the drafts' rules ("D:" = a decision for the advisor or Mustafa)
- H1. No SSH or game-protocol egress from this session (ssh-probe.md: no ssh client; only 443 via a TLS-re-terminating
  gateway; WS upgrades unsupported; QUIC is UDP). Every RS/LG step (pinning, L2, firewall/UDP block for -29, floods, SIGKILL,
  packet capture, log retrieval) needs another operator. Rec: BOTH routes. (a) Hetzner "Cloud config" (cloud-init) user-data,
  a reviewed file in the repo with no secret in it, provisions every VM identically at creation (packages, sysctl, nftables
  port ranges, checkout of the public repo at a pinned commit, build on the VM so no .github/ change or artifact host is
  needed, the run scripts). (b) Claude Code on Mustafa's Windows PC runs the hands-on part over SSH: it makes its own key pair
  there (the private key never leaves his PC; he pastes the public key at VM creation), drives the run plan, finds causes of
  invalid or failing runs (1497, 1505), and keeps raw run data off the public repo. His PC also gives -36's EU consumer path
  and -28's LAN; it is an operator console, never a measured machine (MP: 1408-1410). Cloud-init alone works for scripted
  soaks but has no return path except a public HTTPS endpoint on 443 (which this session could read), so raw data and
  diagnosis suffer. This session keeps design, code, L1 runs and analysis of returned summaries. D: the route (Mustafa).
- H2. Reordering cannot appear where two marks need it (my arithmetic). Per-packet uniform jitter of +-15 ms each way at P3
  spans 30 ms, less than the 33.33 ms between 30 Hz input datagrams, so on L1 (exact send spacing) inputs and single-datagram
  snapshots never reorder at P0-P3; at P4 (+-20 ms, 40 ms span) adjacent datagrams reorder with p = (40 - 33.33)^2 / (2 x 40^2)
  = 1.39 %. Rule 1447-1450 counts a reordering mark only if the run's reorder share > 0, so -20 ("10,000 scripted presses with
  reordering" at P3, 1533) and -58 ("at P3 (so reordering crosses the wrap)", 1530) may be unable to count. Split snapshot
  groups and pings do reorder. Handling: record the reorder share per stream (inputs, snapshots, pings) on every run; D: how
  -20/-58 get reordering (e.g. a P4 run beside P3, or an explicit reorder injection in L1): a setup change.
- H3. The macOS runner cannot be the whole Safari check. Runner-as-MAC counts only in -28 (server then RS), -14, -25's Origin
  and PM-0006-15 (1400-1402); a time row stays on the borrowed Mac (1402-1403). So Safari's legs of -29 (a time), -32 and -36
  (not in the list; -36 also needs an EU consumer path) still need the borrowed Mac, against Mustafa's "if they can, that is the
  Safari check" (1396-1397). Runner preconditions: Safari >= 26.4 on the image (R2-TRN-F7), real Safari via safaridriver, one WT
  session to RS with datagrams + one WS session on the domain (1398-1400). Any .github/ workflow for it goes to Mustafa for
  approval; security-auditor and infra-engineer review it (1403-1404). D: whether -32 and -36's Safari legs may run on the runner.
- H4. Browser-support doubts, not verified in this digest: serverCertificateHashes and datagrams on Safari and Firefox
  (R2-TRN-REC5 mark 1 needs 20 of 20 pinned connects on all four, 1272-1273); Brave (MP) with WebTransport and a pinned hash:
  step 14 judges "on WebTransport" (1680), so a WS-only Brave cannot close -53. Handling: an S unit first, a smoke page (WT with
  pinned hash + datagrams + one stream; WS) run on all five browsers before any long VM session; if one fails, D (with
  security-auditor): a browser-trusted certificate for that browser's WT leg, or the borrowed Mac.
- H5. ML, RD and MAC are not named (OQ-0003-1; 1387-1393). Every row on them is judged only on a named machine (1406-1408); the
  ML stand-in gives rough checks only, never judged; nothing stands in for RD. Blocked for the close: -43 (needs RD), OQ-0001-15's
  RD report, and the ML legs of -14, -19, -25, -28, -29, -31..-33, -36, -41..-46, -51, -52, -60..-62. Handling: list these rows
  as rough-check-first with a short re-run once named (SPIKE_PLAN:156-159). D (optional): whether count-only rows (-28, -32) may
  use any Windows machine with the stable browsers.
- H6. "The server on the same local network" (-28, 1543): a LAN server has no browser-trusted cert, the page must be a secure
  context that fetches the hash from its own HTTPS origin (458), and Chrome's local-network-access rules for a public page
  calling a LAN address are not verified. Rec: run the server on the browsers' own machine with the page on http://localhost
  (a secure context), which is also -36's loopback leg. D: whether loopback counts as the "clean local network".
- H7. Arm-rule replays "on CI's x86-64 runner" (1383, 1528, 1532, 1568, 1576) imply a CI job for spikes/ (a .github/ change for
  Mustafa) or a ruling. D: whether RS (x86-64) or this session's container counts as the x86-64 replay machine.
- H8. -3's L2 setting "half of the profile's RTT minus the path's measured base RTT" (1513) can read RTT/2 - base. Rec:
  (profile RTT - base RTT) / 2 per direction, so the round trip is the profile; -3's MP check (100 +- 5 ms) confirms it. D: wording.
- H9. OQ-0001-9's cover half names the proof test as decider (1162-1164) but no row, record item or item-5 entry carries it
  (1321-1335, 1578, 1620). D: add a recorded item to -56 (rewind test with a destructible wall and a moving vehicle stand-in as
  cover) or move it out of this test.
- H10. Conditional rows on stand-ins: SS needs 300 live projectiles and 15 vehicles (1512), so -54's condition ("live projectiles
  in the load") and -55's ("the sim moves vehicles") turn on whether stand-ins count; PM-0004-73's trajectory function is the
  sim core spike's. Rec: build projectile stand-ins as trajectory records through one throwaway shared function and run -54;
  move vehicle stand-ins and name their collision with soldiers in the record for -55. D: whether stand-ins meet the conditions.
- H11. SMT: if lscpu shows 2 threads per core on the CCX23, keeping the tick's sibling free (-1, 1511) leaves the transport 2
  vCPUs, not "the other three" (1366). Harmless for -11's 1 vCPU; record it.
- H12. Byte counting: -12/-13 count "UDP payload plus 28 B per datagram" at the server; the app sees WT datagram payloads, not
  QUIC packets. Rec: count from quinn's per-connection UDP-level stats or the L2 socket wrapper, never app payload sizes.
- H13. Raw data and the public repo: LG logs every received byte for re-decode (-17, -19; my arithmetic: 48 x ~1 KB x 30/s is
  about 1.4 MB/s), failing runs' data is kept (1505), and logs hold peer addresses (Mustafa's home address in his session; the
  day's VM addresses in -1's header). None of it may enter .checkpoints or the repo. Rec: raw data stays on the VM until copied
  to Mustafa's PC (H1 (b)) or ADR 0006's store; the repo gets verdicts, hashes and redacted headers. D (security-auditor): whether
  the deleted VMs' addresses may be committed.
- H14. Certificates and DNS. The page needs an HTTPS origin with a browser-trusted cert (secure context; WT hash fetched from
  it, 458; COOP/COEP headers for the isolation flag, 1479-1481); WS needs a browser-trusted cert on the domain (462-466); the
  room serves no HTTP pages (421), so a small static server runs on RS off the tick's vCPU. Rec for security-auditor: ACME
  HTTP-01 or TLS-ALPN-01 from RS itself (no Cloudflare API token, so no secret exists), a fresh hostname per run day to stay
  under the CA's duplicate-certificate limit (Let's Encrypt's per-exact-name weekly limit, from memory: to verify). WT dev cert:
  self-signed, valid at most 14 days (457-458). DNS records for Mustafa (Rec), Cloudflare, Proxy status "DNS only" (grey):
  (1) RS: Type A, Name rs-YYYYMMDD (e.g. rs-20261110 -> rs-20261110.shamblefrontlines.com), IPv4 address = RS's public IPv4 of
  that day, TTL Auto; it serves the page, the hash, WS and WT. (2) Only if an IPv6 run is wanted (-12's IPv6 figure): Type AAAA,
  same name, RS's IPv6 of the day. (3) LG: none (it only connects out; MV's access is ADR 0006's). (4) A borrowed-Mac or LAN host:
  none (loopback, H6). Once a record exists this session can check the page host and its cert over 443 (the probe allows HTTPS).
- H15. LG capacity: -27 needs 384 encrypted sessions (1379-1380); more CAX21s by calibration inside the run's cost cap
  (LG-COUNT, 1373-1377); each is one more VM for Mustafa to create; capacity shortages make the run wait (757).
- H16. Scheduling: -27 needs 3 sessions at 3 times of day (1493-1494); -59's 10 injections "at a random tick after warm-up" in
  an S30 room (1540) could cost up to 30 min each if the tick is drawn over the whole soak. Rec: draw it within the first 5 min
  after warm-up (D: a setup reading) or run the 10 rooms concurrently on the unpinned vCPUs (not a time mark).
- H17. Order constraints: -36 sets the cap -14 uses (1,000 B until then, 293); -48's test cap -> -49's rule cap -> checklist steps
  7 and 10; the "clean run" median (1495-1496) must exist before -24/-25 compare.
- H18. Command timing arithmetic (my arithmetic, a report, no threshold change): at P3 a command carried by only 2 datagrams before
  its tick is lost with p = 0.05^2 = 0.25 % > 0.1 %, so -20's 99.9 % needs at least 3 copies before the tick, i.e. an input buffer
  of 2 ticks plus jitter margin at P3 under option (b): it bounds OQ-0001-6's "smallest buffer" and adds 66.7 ms to P3's honest rewind.
- H19. Both OQ-0001-6 options multiply the L1 and L2 runs of -20, -21, -42, -48, -49 (1533-1534, 1578); L1 runs on simulated
  time cost no VM hours (1410-1411) if the Rust toolchain is available here (not checked).
- H20. SPIKE_PLAN:141 lists PM-0001-1 to -62 for this item: -63 is in the 60 Hz group (1557, 1582), so no conflict; the kept
  harness and server code must survive for the later 60 Hz run (1298-1299).

## I. Recommended build and run order (a recommendation; S = under a day's session, M = one to three)
Build units (no VM; each ends with its L1 rows and their bites green, checkpointed):
1. S: route decision (H1), SSH key on Mustafa's PC if route (b), cloud-init file drafted; message catalogue in docs/design/ (G1); run-record format (G2).
2. M: spikes/ workspace: throwaway sim crate (E1) + codec (E3/E4 wire) + thin transport interface; `cargo tree` and swap-diff checks (-35).
3. M: L1 simulator with packet-fate log and per-stream reorder share (E15); -3 on L1. Surfaces H2 early.
4. M: room loop + commands (both OQ-0001-6 options) + snapshots + headless clients on L1 (E2-E4, E12): -17/-57/-58/-18/-20/-21/-22/-41/-47/-50/-60/-62 L1 halves.
5. M: interest + lag compensation + TC/CO + interpolation/event release in client-endpoint (E5, E9, E11): -45/-48/-49/-51/-54/-55/-61 L1 halves.
6. S: fuzz targets (-34).
7. M: transports (E6) both crates + WS + L2 under quinn (R5-TEST-OQ2); loopback tests inside this container (server and headless client on localhost need no egress).
8. M: client harness (E7) + overlay + record form; smoke in this container's Playwright Chromium against a localhost server (pinned hash).
9. S: instrumentation, judging scripts, bites (E18-E20); static page host + cert scripts (E21); cloud-init + run scripts.
10. S: five-browser smoke page (H4) on Mustafa's PC against a localhost server; macOS-runner probe (after his .github/ approval).
VM sessions (RS = CCX23, LG = CAX21s; hours are rough, before calibration; costs are infra-engineer's):
| session | content | RS h | LG h | Mustafa present |
|---|---|---|---|---|
| VS1 bring-up | provisioning, -1 headers, clock resolution, -3 on L2, LG calibration, cert + page host, runner Safari probe to RS, browser smoke incl. Brave, -16, server-mark bites (5-min runs) | 3 | 3 | yes (browsers, DNS) |
| VS2 baseline | 3 x S30 for -4..-9, -11, -12 (SS), -13, -14, -15 (about 35 min each with joins); -10 at N = 24, 36 (3 x 10 min each); -12 ES (3 x 10 min); -13 at P4; clean-run references | 6 | 6 | no |
| VS3 behaviour on L2 | -17 (4 profiles), -18, -19, -20 (2 options), -23, -24 (2 controllers), -26, -30, -31, -37, -38, -48/-49 (4 profiles), -54, -59 | 5 | 5 | no |
| VS4 hostile + fallback | -25 (WT/WS x cap raised/on), slow-loris, /64 cycler, -29 with his browsers, Origin | 2 | 2 | yes (-29, Origin) |
| VS5-7 density | -27 at 3 times of day (bracket B, 8 rooms, SIGKILL), with ADR 0006's PM-0006-13/14 | 3 x 2 | 3 x 2 x (1-2 CAX21) | no |
| VS8 human | ML-stand-in legs (rough): -33, -36 EU path, -41..-46, -51, -52, -60..-62; then the checklist (D) with the rule cap | 3 | 3 | yes |
| later | short re-run on named ML/RD (-43, -44, ML rows); borrowed Mac if H3/H4 need it | 2 | 2 | yes |
Rough total: about 27 RS hours and 27-33 LG hours over 8-10 VM sessions (my estimate). The proof test closes at 30 Hz without
the 60 Hz group (1580-1585); a failing run is kept and never re-run until green (1505-1506).

DONE
