# Claims ledger: ADR-0001 multiplayer and netcode (notes, not part of the draft)

Author: netcode-architect, 2026-10-31. One row per claim or number in `netcode-architect-adr0001-draft.md`. Lines are
at 9e7db28. Tiers: research verified (an R1 or R2 recommendation claim, or a finding its verifier checked) · research,
source to check (any other research statement: R3–R6, findings outside the checked set, open-question text, option
tables) · arithmetic (shown) · settled (a STATUS code or GLOSSARY §8) · design doc · proposed mark (with its reason).
Short forms: r1 = docs/research/r1-simulation-core.md, r2 = docs/research/r2-netcode.md, r3 = r3-rendering.md,
r4 = r4-physics-navigation.md, r5 = r5-toolchain-quality.md, r6 = r6-infrastructure.md, GL = docs/research/GLOSSARY.md,
INV = docs/LEGACY_INVENTORY.md. The test-engineer's pass-mark rows join this ledger in plan step 4.

## Part 1: summary and Context
| # | Claim (short) | Section | Citation | Tier |
|---|---|---|---|---|
| 1 | Clients send only inputs, one command per tick | Summary | r2 R2-SYNC-REC2, lines 2026–2066 | research verified |
| 2 | Snapshots 30 times a second | Summary | r2 R2-BUD-REC1 (snapshot half), lines 2302–2325 | research verified |
| 3 | Others drawn about 100 ms in the past | Summary | r2 R2-SYNC-REC4, lines 2107–2131 (3 intervals = 100 ms at 30 Hz) | research verified |
| 4 | Tick 30 Hz; 60 Hz only with the 60 Hz margin | Summary | TICK-30 (STATUS.md:118–120); GL §8 the 60 Hz margin | settled |
| 5 | VISION_V2 asks for up to 48 players, fair to about 100 ms of ping | Summary | VISION_V2 §3, §4; MODES-3 (a) | design doc |
| 6 | Largest mode 24v24, N = 48 | Context, Requirements | MODES-3 (a) (STATUS.md:104–105); GL lines 335–353 | settled |
| 7 | Headroom run at N = 64 | Context, Requirements | GL "The design point and headroom", lines 355–373 | research, source to check |
| 8 | Start modes 1v1–4v4 and 2v2v2–6v6v6; goal 12v12v12 or 24v24 | Context, Requirements | VISION_V2 §3 (lines 43–46) | design doc |
| 9 | Bots fill empty slots [DEFAULT] | Context, Requirements | VISION_V2 §3 line 49; LOOP_V2 §2 | design doc |
| 10 | TICK-30 quoted | Context, Requirements | STATUS.md:118–120 | settled |
| 11 | Read as the 60 Hz margin; TICK-60-HEADROOM confirmed at the stop | Context, Requirements | GL §8 the 60 Hz margin; TICK-60-HEADROOM (advisor, 2026-10-31) | settled |
| 12 | Fair up to about 100 ms of ping; one EU region [DEFAULT] | Context, Requirements | VISION_V2 §4 line 55 | design doc |
| 13 | Chrome, Edge, Firefox, Safari [DECIDED] | Context, Requirements | VISION_V2 §4 line 52 | design doc |
| 14 | §6 items 6.1–6.6, 6.8 served as listed | Context, Requirements | VISION_V2 §6 lines 64–79 | design doc |
| 15 | §6.4 seats [OPEN]; §6.8 kill-cams etc. [LATER]; §6.10 [LATER] | Context, Requirements | VISION_V2 §6 lines 73, 79, 81 | design doc |
| 16 | LOOP_V2 §7 split feedback quote | Context, Requirements | LOOP_V2 §7 line 92 | design doc |
| 17 | LOOP_V2 §4 uplinks, cache location told; §3 personal money; §9 aiming at height [OPEN] technical; §8 about 20 min; §10 bots | Context, Requirements | LOOP_V2 §3–§4, §8 line 100, §9 lines 128–129, §10 | design doc |
| 18 | LOOK_V2 §3 interiors and upper floors; §4 enterable ruins; roofs fade [DEFAULT] | Context, Requirements | LOOK_V2 §3 line 32, §4 lines 36–41 | design doc |
| 19 | Settled inputs list (stack, MODES-3, TICK-30, SLICE-ORDER, SEC-SPREAD, ABL-PREDICT, BASELINE-WHEN, UPGRADE-SCOPE, SPIKE-SCOPE, PREVIEW-WALL, GL §8) | Context, Requirements | STATUS.md:94–120, 187–188; UPGRADE-SCOPE (Mustafa, 2026-10-31); GL §8 | settled |
| 20 | Delta vs acked snapshot tolerates loss and reordering, unchanged entities free, mid-match join, no bit-exact agreement | Context, finding 1 | r2 R2-SYNC-REC1, lines 1996–2024 | research verified |
| 21 | Netcode must not depend on Wasm/native identity; lockstep rejected | Context, finding 1 | r1 R1-DET-REC1, lines 2129–2145 | research verified |
| 22 | E3 at 30 Hz, N = 48 all in view: about 146.1 kbps | Context, finding 2 | r2 R2-BUD-REC2 reason ("about 146 kbps"), lines 2326–2344; worked estimate, line 1775 | research verified |
| 23 | 60 Hz snapshots: about 286.1 kbps, over the 256 kbps example budget | Context, finding 2 | r2 worked estimate, lines 1778, 1789; R2-BUD-REC1 reason, lines 2304–2306 | research verified |
| 24 | Part marks sum 9.18 of 16.67 ms at 30 Hz, 10.18 on a re-flood tick | Context, finding 3 | GL lines 53–69 (sum of the 30 Hz column: 5.53 + 3.64; + 1.00) | arithmetic |
| 25 | 7.28 of 8.33 ms at 60 Hz; 8.28 on a re-flood snapshot tick; server-only 4.41 of 4.17 ms | Context, finding 3 | GL lines 60–69 (3.87 + 3.41; + 1.00; 3.41 + 1.00) | arithmetic |
| 26 | Every received byte readable; withholding is the only defence that holds | Context, finding 4 | r2 R2-INT-REC2, lines 2445–2465 | research verified |
| 27 | QUIC guarantees 1,200-byte UDP payloads, forbids IP fragmentation; datagrams cannot be fragmented | Context, finding 5 | r2 R2-BUD-REC3, lines 2345–2360 | research verified |
| 28 | Neither WebTransport crate claims production readiness | Context, finding 6 | r2 R2-TRN-REC1, lines 2565–2615 | research verified |
| 29 | Some networks do not carry QUIC | Context, finding 6 | r2 R2-TRN-REC3 reason (F12), lines 2639–2671 | research verified |
| 30 | A known spread seed lets a cheat cancel spread (H2) | Context, finding 7 | r2 R2-SYNC-REC3, lines 2068–2105 | research verified |
| 31 | Takes and gives between drafts (0002–0006) | Context | the drafts' scope (assignment; no research claim) | design doc |
| 32 | CS1: command and 1/60 step declared, never wired; mark drop | Context, legacy | INV line 169 | research, source to check |
| 33 | CS7: resolveShot a pure query for a rewound snapshot; rewrite in Rust | Context, legacy | INV line 175 | research, source to check |
| 34 | GC5: fixed chest origin; spread from Math.random; rewrite in Rust | Context, legacy | INV line 244 | research, source to check |
| 35 | GC2: slot id = sim id; rewrite in Rust | Context, legacy | INV line 241 | research, source to check |
| 36 | GP2: actions at event time; port to TypeScript | Context, legacy | INV line 312 | research, source to check |
| 37 | GC13: preview and launch from different points; rewrite in Rust | Context, legacy | INV line 252 | research, source to check |
| 38 | GN8: sound culled beyond 90 m, visuals 50 m; port to TypeScript | Context, legacy | INV line 273 | research, source to check |
| 39 | GC1 and STATUS ADR-001: server repeats team allow-list; humans take over bot slots | Context, legacy | INV line 240; STATUS.md:199 | research, source to check |
| 40 | GP13: variable step in rAF and world freezes cannot survive multiplayer; redesign | Context, legacy | INV line 323 | research, source to check |

## Part 2: Decision, parts 1–5
| # | Claim (short) | Section | Citation | Tier |
|---|---|---|---|---|
| 41 | Room process per match runs the same Rust sim as the Worker; only commands in; delta, quantised, interest-filtered snapshots at 30 Hz over WebTransport datagrams, WebSocket fallback, one interface | Decision intro | r2 R2-SYNC-REC1, R2-BUD-REC1–REC2, R2-INT-REC1, R2-TRN-REC1–REC4; stack (brief §1) | research verified |
| 42 | The match server is the authority online; the Worker in a local match | Decision 1 | GL "The authority", lines 485–499; r1 R1-BND-REC6, lines 2326–2339 | research verified |
| 43 | Server never takes client state, client-reported hits or RTT | Decision 1 | r2 R2-SYNC-REC3 ("without giving the client authority"), R2-SYNC-REC5 (the RTT is the server's own), lines 2133–2192 | research verified |
| 44 | Lockstep rejected | Decision 1 | r1 R1-DET-REC1, lines 2129–2145 | research verified |
| 45 | Option 3.5 C (client-side hit detection) is an authority leak | Decision 1 | r2 Options 3.5, lines 1647–1653 | research, source to check |
| 46 | §6.5 "All server-side" | Decision 1 | VISION_V2 §6.5 line 74 | design doc |
| 47 | Tick period 33.3 ms at 30 Hz | Decision 2 | GL "Tick period", lines 82–96 (1/30 s) | arithmetic |
| 48 | 60 Hz needs, in one run, one vCPU of the reference server, stress scene N = 48, a client on every soldier, 30-min soak, re-flood included, p99 ≤ 6.25 ms vs 8.33 ms, plus R2-BUD-REC6's shares and R1-TICK-REC2 P1–P3 at 60 Hz | Decision 2 | TICK-60-HEADROOM (the advisor's reading, 2026-10-31); GL §8 the 60 Hz margin | settled |
| 49 | 1 − 6.25 ÷ 8.33 = 25 % | Decision 2 | 6.25 / 8.333 = 0.75 | arithmetic |
| 50 | R2-BUD-REC6's two shares at 60 Hz: 4.2 ms each | Decision 2 | GL lines 189–213; r2 R2-BUD-REC6, lines 2398–2416 | research verified |
| 51 | Snapshots every tick (30 Hz); inputs sampled and sent every tick | Decision 2 | r2 R2-BUD-REC1, lines 2302–2325 (snapshots 30 Hz; inputs every tick) | research verified |
| 52 | 60 or 64 Hz asked only if the margin passes | Decision 2 | r1 R1-TICK-OQ1, lines 2640–2643; TICK-30 | settled |
| 53 | Every legacy weapon fits one shot per tick at 30 Hz; replay per correction halves | Decision 2 | r1 R1-TICK-REC2, lines 2029–2043 | research verified |
| 54 | R1-TICK-REC2's decision rule text quoted | Decision 2 | r1 lines 2061–2062 | research verified |
| 55 | R2-BUD-REC1's tick half quoted | Decision 2 | r2 line 2302 | research verified |
| 56 | "… and the rule does not change" at r2:2415–2416 | Decision 2 | r2 lines 2415–2416 | research verified |
| 57 | "… which does not change" at GL:85; replaced by GL §8's closing note | Decision 2 | GL line 85; GL §8 closing note | settled |
| 58 | Server tick budget at 30 Hz: p99 ≤ 16.7 ms (50 %), max < 33.3 ms, zero overruns | Decision 3 | GL "The server tick budget", lines 167–188 | research verified |
| 59 | Sim step share and server-only share ≤ 8.3 ms p99 each, measured separately | Decision 3 | GL lines 189–213; r2 R2-BUD-REC6 | research verified |
| 60 | Snapshot encoding ≤ 50 µs per client per snapshot | Decision 3 | r2 R2-BUD-REC6, lines 2414–2415 | research verified |
| 61 | 48 × 50 µs = 2.40 ms | Decision 3 | GL line 61 | arithmetic |
| 62 | Rewound shot ≤ 20 µs at 700 shots/s | Decision 3 | r2 R2-SYNC-REC5 mark 4, lines 2133–2192 | research verified |
| 63 | 700 ÷ 30 × 20 µs = 0.47 ms per tick | Decision 3 | GL line 62 (30 Hz column) | arithmetic |
| 64 | Region query ≤ 0.2 ms; ranking and packing ≤ 0.5 ms; 0.70 ms | Decision 3 | r2 R2-INT-REC1 (c), lines 2420–2443; R2-INT-REC4, lines 2485–2500; GL line 63 | research verified |
| 65 | Pair test ≤ 0.1 µs per enemy pair; 768 pairs at 16v16v16: 0.08 ms | Decision 3 | r2 R2-INT-REC2 (b), lines 2445–2465; GL line 64 (48 × 32 ÷ 2 = 768) | arithmetic |
| 66 | Re-flood ≤ 1 ms on a portal-change tick | Decision 3 | r2 R2-INT-REC2 (b) | research verified |
| 67 | Each send call ≤ 1 ms; commands and sends share what is left | Decision 3 | r2 R2-TRN-REC1 mark 2, lines 2565–2615; GL lines 197–202 | research verified |
| 68 | Server-only parts 3.64 of 8.33 ms (4.64 re-flood); server tick 9.18 of 16.67 (10.18) | Decision 3 | GL lines 53–69 | arithmetic |
| 69 | p99 measured directly, in one run (p99s do not add) | Decision 3 | GL "Time statistics", lines 141–165 | research, source to check |
| 70 | Transport runtime outside the tick, ≤ 1 vCPU per process with 48 sessions; in rooms per VM | Decision 3 | GL "Transport runtime time", lines 130–139; r2 R2-TRN-REC1 mark 2 | research verified |
| 71 | R2-BUD-REC6's reason restated with "(at N = 54, MODES-3 (b))"; at N = 48 the tick fits by 0.05 ms (8.28 of 8.33), server-only 0.24 ms over (4.41 of 4.17) | Decision 3 | r2 lines 2402–2403 (superseded); GL lines 62–75; carry-forward 2 (STATUS.md:128–133); 8.33 − 8.28 = 0.05; 4.41 − 4.17 = 0.24 | arithmetic |
| 72 | Need: §6.1 (rooms at 48) | Decision 3 | r2 R2-BUD-REC6 Serves line | research verified |
| 73 | Prediction scope: own movement, weapon state, piloted vehicle, cosmetic shot feedback | Decision 4 | r2 R2-SYNC-REC3, lines 2068–2105; GL lines 501–523 | research verified |
| 74 | Weapon state includes active reload | Decision 4 | r2 Options 3.2 C ("reload and active reload"), lines 1622–1629; LOOP_V2 §7 | research, source to check |
| 75 | Reset to the server's state for the last command run and replay newer commands; skip when matching (Quake III) | Decision 4 | r2 R2-SYNC-REC3 | research verified |
| 76 | Replay 200 ms (6 ticks at 30 Hz) ≤ 1 ms p99 on the minimum laptop | Decision 4 | r1 R1-TICK-REC2 P3, lines 2058–2060 | research verified |
| 77 | No whole-world replay; ADR 0004 may drop R1-ECS-REC1 mark 3 | Decision 4 | r2 R2-XT-OQ7, lines 3367–3372; r1 R1-XU-OQ2, lines 2848–2853 | research, source to check |
| 78 | Correction fade 100 ms (proposed), snap above 2 m (proposed), clear on respawn or teleport | Decision 4 | r2 R2-SYNC-REC3 (F15, F16, F24) | research verified |
| 79 | Own state unquantised, or server quantises its copy the same way | Decision 4 | r2 R2-SYNC-REC3 | research verified |
| 80 | Never predicted: damage, deaths, hits on others; uplink countdown and steals | Decision 4 | r2 R2-SYNC-REC3; r1 R1-ABL-REC4 | research verified |
| 81 | Upgrade offers and picks, weapon pickups not predicted (pickups interim) | Decision 4 | design choice of this record (UPGRADE-SCOPE side; OQ-0001-10) | proposed mark (no research covers upgrades; waiting costs one round trip on a non-time-critical action) |
| 82 | Abilities not predicted until ABL-PREDICT; ADR 0005 carries both answers | Decision 4 | ABL-PREDICT (STATUS.md:110); r2 lines 3277–3288 | settled |
| 83 | SEC-SPREAD (a): spread known only to the server; tracers from a similar-looking spread the hit corrects | Decision 4 | STATUS.md:108–109 | settled |
| 84 | Spread drawn from an authority-only stream | Decision 4 | GL lines 525–544; r1 R1-DET-REC4, lines 2184–2207 | research verified |
| 85 | One command per tick; datagram repeats every unacknowledged command up to a cap, proposed 8 | Decision 5 | r2 R2-SYNC-REC2, lines 2026–2066 | research verified |
| 86 | Command fields: tick, sequence, move, bitfield, press counters, aim point, view time | Decision 5 | r2 R2-SYNC-REC2 | research verified |
| 87 | Aim point at most 80 bits | Decision 5 | r2 R2-SYNC-REC8 mark 2, lines 2247–2274 | research verified |
| 88 | Each client packet carries the newest snapshot received (the ack) | Decision 5 | r2 R2-SYNC-REC1 | research verified |
| 89 | The server's command checks (list) | Decision 5 | r2 R2-SYNC-REC2 (fix pass M2, M3) | research verified |
| 90 | Catch-up cap no larger than the redundancy cap | Decision 5 | r2 R2-SYNC-REC2 | research verified |
| 91 | getStats() missing on Chrome and Edge, throws on Firefox | Decision 5 | r2 R2-TRN-REC4, lines 2673–2697 (F9, checked) | research verified |
| 92 | Clock drifts at most 1–2 ms per frame, halves errors over 100 ms, resets over 500 ms (Quake III) | Decision 5 | r2 R2-SYNC-REC4, lines 2107–2131 | research verified |
| 93 | Server input buffer 1–3 ticks (proposed), adapted per client | Decision 5 | r2 R2-BUD-REC4, lines 2361–2380 | research verified |
| 94 | Each command stamped with the server tick it is meant for | Decision 5 | design of this record on R2-SYNC-REC2's tick field; R2-XT-OQ5 | proposed mark (the command carries its tick per R2-SYNC-REC2; stamping for the server's tick follows from the input buffer) |
| 95 | The page posts its cursor ray and the drawn view time; the Worker casts with the sim crate's ray query against what it published | Decision 5 | r2 R2-SYNC-REC8 ("against its rendered world"), R2-XT-OQ1 lines 3326–3332; r4 R4-LOS-REC3; r1 R1-BND-REC1 mark 2 | research, source to check |
| 96 | Server rebuilds the ray from its own fire origin (playerFireOrigin rule, GC5) to the point, extends to range | Decision 5 | r2 R2-SYNC-REC8, R2-SYNC-REC5 | research verified |
| 97 | Throws and charged shots take heading and pitch from the same point | Decision 5 | r2 R2-SYNC-REC8 | research verified |
| 98 | Aim help, if adopted, applied by the server | Decision 5 | r2 R2-SYNC-REC8 (L1); LOOP_V2 §9 [OPEN] | research verified |
| 99 | The page makes no Wasm calls | Decision 5 | r1 R1-BND-REC1 mark 2, lines 2252–2264 | research verified |

## Part 3: Decision, parts 6–14
| # | Claim (short) | Section | Citation | Tier |
|---|---|---|---|---|
| 100 | Delta against the last acked snapshot; unchanged free; new from per-type defaults; removals explicit; no baseline → defaults | Decision 6 | r2 R2-SYNC-REC1, lines 1996–2024 | research verified |
| 101 | One shared ring of world states; per-client ring of sent snapshots, proposed 32 | Decision 6 | r2 R2-SYNC-REC1 (ring 32 as Quake III) | research verified |
| 102 | 32 snapshots ≈ 1.07 s at 30 Hz | Decision 6 | 32 ÷ 30 = 1.067 s | arithmetic |
| 103 | E3: positions 1 cm in 16/16/12 bits (±327.68 m, 0–40.95 m), yaw 16, pitch 10, changed lists | Decision 6 | r2 R2-BUD-REC2, lines 2326–2344; worked estimate lines 1745–1748 | research verified |
| 104 | Projectiles as trajectory records re-sent on change, one shared trajectory function; sent from the tick relevant, re-based | Decision 6 | r2 R2-SYNC-REC6, lines 2194–2223 | research verified |
| 105 | E3b fallback costs about +42 kbps at N = 48 | Decision 6 | r2 R2-BUD-OQ8, lines 2993–2994; worked estimate lines 1775, 1779 (188.3 − 146.1 = 42.2) | arithmetic |
| 106 | Per-client hard cap proposed 256 kbps, counted at UDP payload + 28 B | Decision 6 | r2 R2-BUD-REC2 (hard cap; p99 window mark 256 kbps) | proposed mark (the cap equals R2-BUD-REC2's p99 window mark, so the mark holds by construction; Fiedler's example budget is the same, r2 line 1789) |
| 107 | 256 kbps ≈ 1,067 B per snapshot interval at 30 Hz | Decision 6 | 256,000 ÷ 8 ÷ 30 = 1,066.7 B | arithmetic |
| 108 | Datagram cap: min(smallest measured maxDatagramSize, 1,150 B); 1,000 B in the load until measured | Decision 6 | r2 R2-BUD-REC3, lines 2345–2360; R2-TRN-REC2, lines 2617–2637; R2-XU-OQ5, lines 3158–3165 | research verified |
| 109 | Split by entity groups that decode alone, acked per group | Decision 6 | r2 R2-BUD-REC3 (P1); R2-SYNC-OQ4, lines 2916–2919 | research verified |
| 110 | First full state over a reliable stream at join and resync | Decision 6 | r2 R2-BUD-REC3 (P4) | research verified |
| 111 | Gameplay events in snapshots, resent until acked; bounded queue; overflow → resync or disconnect; cosmetic once | Decision 6 | r2 R2-BUD-REC2 (M4) | research verified |
| 112 | Hit confirmation a gameplay event (LOOP_V2 §7) | Decision 6 | r2 R2-SYNC-REC5 (reliable event with the tick) | research verified |
| 113 | Remote soldier fields: position, yaw, pitch, health, flag bits, weapon id; action starts as cosmetic events | Decision 6 | r2 worked estimate rows, lines 1745–1751, 1759 | research, source to check |
| 114 | Fits the 90-bit full and 60-bit delta records, flags change rarely | Decision 6 | r2 worked estimate lines 1750–1751 | research, source to check |
| 115 | Vehicles: hull and turret transforms, per-part health; parts and seated players with parent | Decision 6 | r2 R2-SYNC-REC7, lines 2225–2245; R2-INT-REC6 | research verified |
| 116 | Entities named by the sim's id (kind, index, generation), never a library id | Decision 6 | r1 R1-ECS-REC3, lines 1973–1990 | research verified |
| 117 | Record names kind and index; generation on entry | Decision 6 | design of this record over R1-ECS-REC3 and r2 line 1751 (7-bit index) | proposed mark (keeps R1-ECS-REC3's stale-id rejection while paying the generation only once per entry) |
| 118 | 32-bit tick in the header, 16-bit offsets elsewhere | Decision 6 | design of this record (R2-XT-OQ11, lines 3390–3394) | proposed mark (a 16-bit tick wraps inside a 60 Hz soak, row 119) |
| 119 | 16-bit tick wraps after 36.4 min at 30 Hz, 18.2 min at 60 Hz | Decision 6 | 65,536 ÷ 30 = 2,184.5 s; 65,536 ÷ 60 = 1,092.3 s | arithmetic |
| 120 | +2 B on the 8 B header ≈ +0.5 kbps at 30 Hz | Decision 6 | r2 line 1743 (8 B header, assumption); 2 B × 8 × 30 = 480 bit/s | arithmetic |
| 121 | The wire format is not the block; the Worker decodes then writes ADR 0002's layout | Decision 6 | r1 R1-BND-OQ8, lines 2745–2746 (coupling warning); r1 R1-BND-REC6 | research, source to check |
| 122 | No state hash exchanged: a client holds only its interest set; a compared hash leaves authority-only streams out | Decision 6 | GL "State hash", lines 546–558; r1 R1-DET-OQ9, lines 2706–2709 | research, source to check |
| 123 | Interpolation buffer in the Worker; the page interpolates between the two newest blocks | Decision 7 | r2 R2-XT-OQ2, lines 3333–3339 (the option); r1 R1-TICK-REC5, lines 2094–2116 | research verified |
| 124 | Start 3 intervals (100 ms at 30 Hz), adaptive within 2–5 intervals | Decision 7 | r2 R2-SYNC-REC4, lines 2107–2131 | research verified |
| 125 | 2–5 intervals = 66.7–166.7 ms at 30 Hz | Decision 7 | 2 × 33.3; 5 × 33.3 | arithmetic |
| 126 | Extrapolate at most one interval, then hold; respawns placed; linear first, Hermite on artefacts | Decision 7 | r2 R2-SYNC-REC4 | research verified |
| 127 | Remote projectiles on the delayed timeline | Decision 7 | r2 R2-SYNC-REC6 | research verified |
| 128 | Announced delay clamped, changes ≤ one interval per second | Decision 7 | r2 R2-SYNC-REC5 (view time) | research verified |
| 129 | Three intervals is Fiedler's measured rule for 2–5 % loss | Decision 7 | r2 R2-SYNC-REC4 reason (F21) | research verified |
| 130 | History of sim-owned hit volumes (soldiers, vehicles, harvesters), no animation | Decision 8 | r2 R2-SYNC-REC5, lines 2133–2192 | research verified |
| 131 | Ray from the server's fire origin to the aim point; movers at view time, statics at present | Decision 8 | r2 R2-SYNC-REC5 | research verified |
| 132 | View-time tolerance ≤ one snapshot interval (33.3 ms at 30 Hz) | Decision 8 | r2 R2-SYNC-REC5 (H1); 1 ÷ 30 s | research verified |
| 133 | RTT from quinn's rtt(), or lowest recent ping over WebSocket | Decision 8 | r2 R2-SYNC-REC5 (H4) | research verified |
| 134 | The cap bounds the final rewind; 200 or 250 ms open | Decision 8 | r2 R2-SYNC-REC5; R2-XU-OQ1, lines 3123–3131 | research verified |
| 135 | History = cap + margin, ≤ 1 MB for the stress scene | Decision 8 | r2 R2-SYNC-REC5 mark 3 | research verified |
| 136 | Filters: team, sent and acked, reach; died or teleported stops | Decision 8 | r2 R2-SYNC-REC5 | research verified |
| 137 | Bots shoot at present, no rewind | Decision 8 | r2 R2-SYNC-REC5 (F31) | research verified |
| 138 | View-time offsets and aims logged; detection is the security baseline's | Decision 8 | r2 R2-SYNC-REC5 (H1); BASELINE-WHEN (b) | research verified |
| 139 | At P2 with 100–150 ms delay the rewind needed is 200–250 ms; a 200 ms cap clips some shots | Decision 8 | r2 R2-SYNC-REC5 (F28's formula, inferred) | research verified |
| 140 | Without lag compensation a 100 ms player leads a sprinter by about 1.8 m | Decision 8 | r2 R2-SYNC-REC5 reason (arithmetic) | research verified |
| 141 | Region from camera worst-case view as data, motion margin, hysteresis, uniform grid | Decision 9 (a) | r2 R2-INT-REC1, lines 2420–2443 | research verified |
| 142 | Enclosures as areas, portals, flood numbers, constant-time compare, re-flood on portal change | Decision 9 (b) | r2 R2-INT-REC2, lines 2445–2465 | research verified |
| 143 | Area ids shared with navigation | Decision 9 (b) | r4 R4-FLOORS-REC2 (from line 925) | research, source to check |
| 144 | One shared visibility pass for bots; sight hiding only if R2-INT-OQ1's rule says so | Decision 9 (c) | r2 R2-INT-REC3, lines 2467–2483; R2-INT-OQ1 (DESIGN) | research verified |
| 145 | Accumulator: own state always; nearest 8 enemies every snapshot; everyone every 4 snapshots | Decision 9 (d) | r2 R2-INT-REC4 mark, lines 2485–2500 | research verified |
| 146 | Visibility class on every field, event, message type: public, team, owner, spatial | Decision 9 (e) | r2 R2-INT-REC5, lines 2502–2521 | research verified |
| 147 | Event radius per type; area test; shots at the viewer always; shooter id stripped, origin clipped; big units by bounds + radius | Decision 9 (f) | r2 R2-INT-REC6, lines 2523–2543 | research verified |
| 148 | A shooter can only hit what it was sent | Decision 9 | r2 pointers, lines 3431–3433 | research, source to check |
| 149 | Cutaway follows the same rule (R2-XT-OQ15, ADR 0003) | Decision 9 | r2 R2-INT-REC2; R2-XT-OQ15 | research verified |
| 150 | Spectators: none unless Mustafa wants them | Decision 9 | r2 R2-INT-REC7, lines 2545–2561; R2-XU-OQ17 | research verified |
| 151 | quinn 0.11.x; WebTransport layer by spike between wtransport 0.7.x and web-transport-quinn 0.13.x; h3 and own handshake out | Decision 10 | r2 R2-TRN-REC1, lines 2565–2615 | research verified |
| 152 | Fallback tokio-tungstenite 0.30.x; axum's ws only if the server serves HTTP through axum | Decision 10 | r2 R2-TRN-REC3, lines 2639–2671 | research verified |
| 153 | The room process serves no HTTP pages | Decision 10 | r2 R2-TRN-REC5 (hash fetched from the page's own HTTPS origin); R2-ROOM-REC2 (lobby hands out host and port) | proposed mark (design of this record: the lobby serves HTTPS, the room only game sessions) |
| 154 | One transport per client; requireUnreliable; no WT over HTTP/2; newest snapshot only over WebSocket | Decision 10 | r2 R2-TRN-REC3 | research verified |
| 155 | WebTransport timeout proposed 1.5 s, then WebSocket | Decision 10 | r2 R2-TRN-REC3 mark 1 | research verified |
| 156 | Mapping: datagrams for inputs, snapshots, pings; one bidirectional control stream; bulk on unidirectional streams | Decision 10 | r2 R2-TRN-REC2, lines 2617–2637 | research verified |
| 157 | Thin interface, four operations, outside the sim crate | Decision 10 | r2 R2-TRN-REC4, lines 2673–2697; Options Q4 A, lines 1866–1875 | research verified |
| 158 | A third loopback backend for the local server | Decision 10 | SLICE-ORDER condition 1 (STATUS.md:96–97); r5 R5-TEST-REC4 (L1 behind the interface), lines 1454–1474 | settled |
| 159 | Decoders never panic, check sizes first; the sim thread never parses raw bytes | Decision 10 | r2 R2-TRN-REC4 (M9) | research verified |
| 160 | Protocol version in `protocols` and in the first control message | Decision 10 | r2 R2-TRN-REC4 | research verified |
| 161 | NewReno or Cubic by measurement, BBR out; non-waiting send_datagram with buffer check; outgoingMaxAge proposed 100 ms; RTT and loss feed rate adaptation | Decision 10 | r2 R2-TRN-REC6, lines 2724–2744 | research verified |
| 162 | Under pressure drop low-priority entities, then 20 Hz snapshots, never above the cap | Decision 10 | r2 R2-BUD-REC5, lines 2381–2397 | research verified |
| 163 | 20 Hz at a 30 Hz tick = two of every three ticks | Decision 10 | 30 × 2 ÷ 3 = 20 | arithmetic |
| 164 | Hostile-input limits: 2 streams each way, ~64 KB, ~128 KB, ~16 KB, size drop, rate limits, Join within 2 s, per-IP cap above 1, retry, H3_EXCESSIVE_LOAD, Origin 403, wss, WebSocket limits | Decision 10 | r2 R2-TRN-REC1 mark 4; R2-TRN-REC3 | research verified |
| 165 | Version floors quinn-proto ≥ 0.11.18, rustls ≥ 0.23.46, ring ≥ 0.17.12, tokio ≥ 1.54.1; dangerous-configuration off | Decision 10 | r2 R2-TRN-REC1 mark 5 (F40) | research verified |
| 166 | Kept current by the advisories check; its policy is the security baseline's | Decision 10 | GL lines 691–705; BASELINE-WHEN (b) | settled |
| 167 | Production: a certificate per hostname (R6-HOST-REC4; carry-forward 4) | Decision 10 | r6 R6-HOST-REC4, lines 684–701; carry-forward 4, STATUS.md:135–137 | research, source to check |
| 168 | Dev and preview: self-signed, pinned, ≤ 14 days, hash from the page's own HTTPS origin | Decision 10 | r2 R2-TRN-REC5, lines 2699–2722 | research verified |
| 169 | Need: loopback backend only SLICE-ORDER condition 1 and GL §8 foundation; congestion VISION_V2 §4 only | Decision 10 | the drafts' trace reading; R2-XU-OQ18 | settled |
| 170 | Lifecycle Created → Open → Countdown → Running → Ended → Closed, timeouts, health check, no pause, fresh sim per match | Decision 11 | r2 R2-ROOM-REC1, lines 2748–2761 | research verified |
| 171 | Fresh sim per match resets every player record, tracks included | Decision 11 | UPGRADE-SCOPE ("reset at the start of every match") | settled |
| 172 | One room per process; sim thread, async I/O; panic = "unwind"; connection work isolated; B only if too costly | Decision 11 | r2 R2-ROOM-REC2, lines 2762–2782 | research verified |
| 173 | Join token claims; expiry 30 s, leeway ≤ 5 s; first control-stream message, never a URL | Decision 11 | r2 R2-ROOM-REC3, lines 2783–2811 | research verified |
| 174 | Signature per the security review (asymmetric, pinned algorithm, token type, key id, verify before cache, bounded cache, re-check team) | Decision 11 | r2 R2-ROOM-OQ2, lines 3082–3090 | research, source to check |
| 175 | The room assigns team and squad | Decision 11 | r2 R2-ROOM-REC3 | research verified |
| 176 | Grace window proposed 60 s; fresh token; newest connection wins | Decision 11 | r2 R2-ROOM-REC4, lines 2812–2822 | research verified |
| 177 | Private rooms: code locates; secure random; limits per client, address, global; guest creation per address; host commands checked; settings fixed at start; faction per team | Decision 11 | r2 R2-ROOM-REC5, lines 2823–2841; VISION_V2 §3 [DECIDED] | research verified |
| 178 | Bots: slot table; bots per team = size − humans; human takes a bot's slot keeping sim id; same input commands | Decision 11 | r2 R2-ROOM-REC6, lines 2842–2856; R2-SYNC-REC9 | research verified |
| 179 | A room = mode id + allowed overrides; format ADR 0005 | Decision 11 | r2 R2-ROOM-REC7, lines 2857–2867 | research verified |
| 180 | Guest id and ≥ 64-bit secret in an HttpOnly, Secure, SameSite=Strict cookie; room sees only the token | Decision 11 | r2 R2-ROOM-REC8, lines 2868–2888 | research verified |
| 181 | The room writes the match record before Closed | Decision 11 | r2 R2-ROOM-REC1 mark 3; r6 R6-TEL-REC1, lines 784–811 | research verified |
| 182 | Hosting services model a session as a bounded life, one process each | Decision 11 | r2 R2-ROOM-REC1, REC2 reasons | research verified |
| 183 | Server authority over tracks, offers, picks, drops, pickups | Decision 12 | UPGRADE-SCOPE; this record's part 1 | settled |
| 184 | Offers owner class with an offer id; PickUpgrade on the control stream; refusal rules; counted like forged commands | Decision 12 | design of this record on r2 R2-INT-REC5 (classes) and R2-SYNC-REC2 (validation) | proposed mark (a pick is a rare reliable action; refusing what was never offered is the anti-cheat rule J5 assigns) |
| 185 | Dropped weapons are sim entities replicated as spatial | Decision 12 | UPGRADE-SCOPE ("dropped weapons are world pickups"); r2 R2-INT-REC1–REC2 | settled |
| 186 | Pickups through existing button edges; authority resolves which pickup; refuses unplaced, taken, out-of-rule | Decision 12 | design of this record on R2-SYNC-REC2 | proposed mark (no new message; the command cannot name an entity into existence) |
| 187 | Carry slots, ammo for looted weapons, levelling speed are Mustafa's (ADR 0005 lists) | Decision 12 | UPGRADE-SCOPE ("Still open") | settled |
| 188 | Quake III predicts item pickups | Decision 12 | r2 R2-SYNC-F17, line 230 | research verified |
| 189 | No research id concerns upgrades | Decision 12 | the id table (no upgrade id) | settled |
| 190 | Profiles P0–P4 and B1 values; latency is the round trip; runs reorder | Decision 13 | r2 lines 1987–1992 | research verified |
| 191 | Budgets marks' "100 ms RTT, ±10 ms, 2 %" → P2; interest's "0, 50, 100, 200 ms ±20 ms 0–5 %" → P0, P1, P2, P4; rooms' "0–200 ms, 0–5 %" → P0–P4 | Decision 13 | r2 R2-XU-OQ12, lines 3220–3225; R2-XT-OQ14, lines 3404–3408 | research verified |
| 192 | "100 ms of simulated latency" read as P2 | Decision 13 | GL §8 the netcode proof test; VISION_V2 §4 | proposed mark (VISION_V2 §4 says "ping", a round trip; R2's profiles inject round-trip time; OQ-0001-3) |
| 193 | The simulator is ADR 0002's | Decision 13 | r5 R5-TEST-REC4 (owner 0002) | research, source to check |
| 194 | The proof test runs before the foundation, throwaway, in spikes/, after approval | Decision 14 | SLICE-ORDER condition 3 (STATUS.md:101); SPIKE-SCOPE (a) (STATUS.md:187–188) | settled |
| 195 | It takes in the research spikes of R2-BUD-REC6, R2-SYNC-REC9, R2-TRN-REC1, R2-TRN-REC3, R2-ROOM-REC2, R1-TICK-REC2 P1 and P3, with R2-SYNC-REC3 and REC5 | Decision 14 | the named recommendations' spike marks | research verified |

## Part 4: Decision subsections (components, ownership, catalogue, bandwidth, budgets, failure modes, anti-cheat)
| # | Claim (short) | Section | Citation | Tier |
|---|---|---|---|---|
| 196 | The page never handles a network packet; the Worker owns the connection | Components | r1 R1-BND-REC6, lines 2326–2339 | research verified |
| 197 | The sim crate depends on no transport crate | Components | r2 R2-TRN-REC4 mark 3 | research verified |
| 198 | Server tick order: due ticks, commands from bounded queues, sim step with bots and lag-compensated hitscan, hit history, interest, encode, non-waiting sends, counters | Components | r1 R1-TICK-REC4 (lines 2078–2093: snapshots after the tick); r2 GL "Server tick time" lines 113–128; R2-TRN-REC4 (bounded queues); R2-TRN-REC6 | research verified |
| 199 | Client tick online: decode and ack, reset and replay, interpolation clock, aim ray, command, block | Components | r2 R2-SYNC-REC1, REC3, REC4; r1 R1-BND-REC2, REC6 | research verified |
| 200 | A local match: the same Worker build runs the local server with bots; loopback with the same messages; tests put the simulator's profiles between | Components | SLICE-ORDER condition 1; r1 R1-BND-REC6; r5 R5-TEST-REC4 | settled |
| 201 | State ownership table: each row as parts 1, 4, 6, 8, 9, 11, 12 (authority, prediction, class) | State ownership | rows 42–43, 73–84, 141–150, 170–189 | research verified |
| 202 | Team economy and tech team or public: R2-INT-OQ3 a design question | State ownership | r2 R2-INT-OQ3, lines 3014–3017 | research, source to check |
| 203 | Uplinks, countdown, steals and cache location public | State ownership | r2 R2-INT-REC5; LOOP_V2 §4 ("everyone is told where") | research verified |
| 204 | Camera and framing on the page; bounds as mode data | State ownership | r2 R2-INT-REC1 | research verified |
| 205 | Content hash: ADR 0005 builds; the room names it at join | State ownership | r1 R1-DATA-REC6, lines 2565–2580 | research, source to check |
| 206 | Every message type has a class; the audit fails on an unclassified one | Message catalogue | r2 R2-INT-REC5 mark | research verified |
| 207 | Input: ack per entity group plus unacked commands; checks; size ≤ command size × cap; rate limit | Message catalogue | r2 R2-SYNC-REC2; R2-TRN-REC1 mark 4 | research verified |
| 208 | Snapshot sections: header, own state, spatial, team, public, events | Message catalogue | r2 R2-SYNC-REC1, R2-INT-REC5; worked estimate lines 1743, 1756–1760 | research verified |
| 209 | Ping and Pong as datagrams, a few per second (proposed) | Message catalogue | r2 R2-TRN-REC4 (own ping messages); R2-BUD-REC4 (RTT samples) | proposed mark (the clock needs samples; the rate is set by the proof test's clock mark, What a spike must prove item 4) |
| 210 | Join, JoinAccepted{slot, team, squad, modeId, modeHash, serverTick}, JoinRejected, RoomState, HostCommand, RoomEnded, Leave on the control stream | Message catalogue | r2 room control messages, lines 2886–2890 | research, source to check |
| 211 | Join carries the protocol version and content hash; JoinAccepted adds the content hash and the rates | Message catalogue | r2 R2-TRN-REC4 (version in the first control message); r1 R1-DATA-REC6 (the server names the hash at join); R2-XT-OQ6 | research verified |
| 212 | Join within 2 s | Message catalogue | r2 R2-TRN-REC1 mark 4 | research verified |
| 213 | FullState on a unidirectional stream at join and resync | Message catalogue | r2 R2-BUD-REC3 (P4); R2-TRN-REC2 (bulk on its own stream) | research verified |
| 214 | ClientSettings: announced delay, clamped, ≤ one interval per second | Message catalogue | r2 R2-SYNC-REC5 | research verified |
| 215 | HostCommand checks: sender is the host; values inside allowed overrides | Message catalogue | r2 R2-ROOM-REC5; R2-ROOM-REC7; r1 R1-DATA-REC7 (allow list), lines 2582–2594 | research verified |
| 216 | PickUpgrade with its checks | Message catalogue | design of this record (row 184) | proposed mark (as row 184) |
| 217 | ContentReload in development builds only, if ADR 0005 keeps reload over the game connection | Message catalogue | r1 R1-DATA-REC5, lines 2542–2563 | research, source to check |
| 218 | Kick as a message | Message catalogue | r2 room control messages (`HostCommand{kick}`), lines 2886–2890 | research, source to check |
| 219 | Lobby calls: guest create, private-room create, code lookup, token request; Origin check; limits | Message catalogue | r2 lines 2889–2890; R2-ROOM-REC5; R2-ROOM-REC8 | research verified |
| 220 | Supervisor calls: spawn, health check, exit, record upload (ADR 0006) | Message catalogue | r2 R2-ROOM-REC1 (health check), R2-ROOM-REC2; r6 R6-TEL-REC1 | research verified |
| 221 | About 584 B per snapshot (609 with events), 146.1 kbps; E3b 188.3; 4v4 44.2; 6v6v6 70.3; 12v12v12 116.2; N = 64 183.7 | Bandwidth | r2 worked estimate, lines 1764–1782 | research, source to check |
| 222 | Interest cuts the per-entity part and the events, not the overhead | Bandwidth | r2 pointers ("Interest and budgets"), lines 3445–3447 | research, source to check |
| 223 | Mean ≤ 100 kbps with interest on the Octopon-3 greybox | Bandwidth | r2 R2-BUD-REC2 mark | research verified |
| 224 | IPv6 adds 20 B per datagram: +4.8 kbps at 30 Hz | Bandwidth | r2 line 1784 | research, source to check |
| 225 | Fiedler's example budget 256 kbps | Bandwidth | r2 line 1789 (F34) | research, source to check |
| 226 | Upstream with 8 B commands, newest + 3, at 30 Hz: about 22.3 kbps (U1's 44.6 halved) | Bandwidth | r2 table C, lines 1704–1710; 44.6 ÷ 2 = 22.3 | arithmetic |
| 227 | Upstream mark ≤ 48 kbps (IPv4) | Bandwidth | r2 R2-BUD-REC1 mark | research verified |
| 228 | 48 kbps at 30 sends/s = 200 B per datagram; 143 B after 57 B overhead | Bandwidth | 48,000 ÷ 8 ÷ 30 = 200; 200 − 57 = 143; r2 line 1742 | arithmetic |
| 229 | 48 × 146 kbps ≈ 7.0 Mbit/s out, about 3.2 GB per match-hour; bots cost no traffic | Bandwidth | r2 lines 1786–1787 | research, source to check |
| 230 | 48 × 22.3 kbps ≈ 1.07 Mbit/s in | Bandwidth | 48 × 22.3 = 1,070.4 kbps | arithmetic |
| 231 | R6 prices 3.16 GB per room-hour inside the 20 TB included per VM | Bandwidth | r6 R6-COST-REC1, lines 703–719 | research, source to check |
| 232 | WebSocket overhead not estimated; measured | Bandwidth | none (a stated gap) | proposed mark (no research figure; the proof test measures it) |
| 233 | Server marks on one vCPU of the reference server (R6-HOST-REC2's CCX23) | Budgets | GL "The reference server", lines 225–243; r6 R6-HOST-REC2, lines 645–671 | research, source to check |
| 234 | 30 Hz: p99 ≤ 16.7 ms; max < 33.3 ms; 0 overruns | Budgets | rows 58; r2 R2-BUD-REC6 (0 missed ticks) | research verified |
| 235 | Shares ≤ 8.3 ms p99 each, measured separately | Budgets | row 59; r2 R2-SYNC-REC9 mark 1 | research verified |
| 236 | Server-only parts at p99, reported beside the whole, never summed into it | Budgets | rows 60–67; GL lines 141–165 | research verified |
| 237 | 60 Hz margin row | Budgets | row 48 | settled |
| 238 | Prediction replay row | Budgets | row 76 | research verified |
| 239 | Transport runtime ≤ 1 vCPU, p99 send call ≤ 1 ms; load restated at 30 Hz: 30 input datagrams/s per session | Budgets | r2 R2-TRN-REC1 mark 2 (60 datagrams/s placeholder at the 60 Hz form) | arithmetic |
| 240 | Downstream: mean ≤ 160 kbps, p99 1-s windows ≤ 256 kbps, UDP payload + 28 B | Budgets | r2 R2-BUD-REC2 mark, lines 2326–2344 | research verified |
| 241 | Upstream ≤ 48 kbps under P2 (restated) | Budgets | r2 R2-BUD-REC1 mark (100 ms RTT, ±10 ms, 2 %) restated against P2 (row 191) | research verified |
| 242 | Memory per room ≤ 64 MB at N = 48 (proposed), RSS at start and after 20 min | Budgets | r2 R2-ROOM-REC2 mark 3, lines 2762–2782 | research verified |
| 243 | Memory per session flat over the soak | Budgets | r2 R2-TRN-REC6 mark 1 | research verified |
| 244 | Room start-up < 2 s | Budgets | r2 R2-ROOM-REC2 mark 5 | research verified |
| 245 | Failure: deltas again within 500 ms after B1 | Failure modes | r2 R2-SYNC-REC1 mark 2 | research verified |
| 246 | Failure: no QUIC → WebSocket ≤ 3 s | Failure modes | r2 R2-TRN-REC3 mark 1 | research verified |
| 247 | Failure: a browser moves to a new draft; recurring beta test | Failure modes | r2 R2-TRN-OQ1, lines 3036–3040 | research, source to check |
| 248 | Failure: the congestion window can fall to 2 datagrams | Failure modes | r2 R2-TRN-REC6 reason (F24); R2-BUD-REC5 reason | research verified |
| 249 | Failure: never-acking client capped, resynced or dropped | Failure modes | r2 R2-BUD-REC2 (M4) | research verified |
| 250 | Failure: overrun → bounded catch-up, dropped time recorded | Failure modes | r1 R1-TICK-REC4, lines 2078–2093 | research verified |
| 251 | Failure: a handler panic closes one connection; a dead room leaves others unaffected | Failure modes | r2 R2-ROOM-REC2 marks 4, 6 | research verified |
| 252 | Failure: a Wasm trap → Worker restart (ADR 0004), resync through FullState | Failure modes | r2 R2-XT-OQ12 (owner 0004), lines 3395–3398; pointers lines 3480–3482 | research, source to check |
| 253 | Failure: hidden tab → resync, back in step within 0.5 s | Failure modes | r1 R1-TICK-REC5 mark, lines 2094–2116 | research verified |
| 254 | Failure: a drop → slot held; 100 % rejoin | Failure modes | r2 R2-ROOM-REC4 mark 1 | research verified |
| 255 | Failure: lobby down → running rooms go on (tokens checked offline) | Failure modes | r2 R2-ROOM-REC3 reason | research verified |
| 256 | Failure: version or content mismatch → JoinRejected; bundle reloaded | Failure modes | r1 R1-DATA-REC6 mark | research, source to check |
| 257 | Failure: the 60 Hz soak crosses 65,536 ticks | Failure modes | 30 min × 60 × 60 = 108,000 ticks > 65,536 | arithmetic |
| 258 | Failure: certificate expiry → ADR 0006's alert | Failure modes | r6 R6-HOST-REC4 mark | research, source to check |
| 259 | Failure: no Hetzner EU capacity (R6-HOST-OQ1) | Failure modes | r6 R6-HOST-REC2 (F20, unchecked) | research, source to check |
| 260 | Anti-cheat: the client's sim online is a predicting copy, never the authority | Anti-cheat | GL lines 485–499 | research verified |
| 261 | Anti-cheat: forged commands rejected, counted, no effect beyond their valid part | Anti-cheat | r2 R2-SYNC-REC2 mark 3 | research verified |
| 262 | Anti-cheat: aim help on a client cannot be told from an aimbot | Anti-cheat | r2 R2-SYNC-REC8 (L1) | research verified |
| 263 | Anti-cheat: reader client and byte audit prove the withholding | Anti-cheat | r2 R2-INT-REC2 (a), R2-INT-REC5 | research verified |
| 264 | Anti-cheat: detection, CSP and key storage wait on the security baseline (BASELINE-WHEN (b)) | Anti-cheat | STATUS.md:112–113; r2 pointers (R6), lines 3509–3512 | settled |

## Part 5: Options, trade-offs, consequences
| # | Claim (short) | Section | Citation | Tier |
|---|---|---|---|---|
| 265 | Q1 rows A–E (lockstep, predict-all, state sync, full snapshots, delta) with their facts | Options Q1 | r2 Options 3.1, lines 1612–1620 | research, source to check |
| 266 | Q1 A: legacy is not deterministic (11 wall-clock reads) | Options Q1 | STATUS.md:273 | research, source to check |
| 267 | Q1 B unassessed (Overwatch talk video only) | Options Q1 | r2 hinge table, lines 3532–3546 | research, source to check |
| 268 | Q2 rows A–D | Options Q2 | r2 Options 3.2, lines 1622–1629; R2-SYNC-REC3 | research, source to check |
| 269 | Q3 rows A–C (Quake III 100 ms; 10 % per update, snap 2 m) | Options Q3 | r2 Options 3.3, lines 1631–1637 | research, source to check |
| 270 | Q4 rows A–C; page-side buffer needs view time, aim ray and sim maths the boundary forbids | Options Q4 | r2 Options 3.4, lines 1639–1645; R2-XT-OQ2, lines 3333–3339; r1 R1-BND-REC1 | research, source to check |
| 271 | Q5 rows A–C (1.8 m lead; Source's model; authority leak) | Options Q5 | r2 Options 3.5, lines 1647–1653 | research, source to check |
| 272 | Q6 rows A–C and E3, E3b (146 and 188 kbps, +42) | Options Q6 | r2 Options 3.6, lines 1655–1664; table B, lines 1694–1700 | research, source to check |
| 273 | Q7 rows A, B (32 bits; up to 80 bits); who casts the ray | Options Q7 | r2 Options 3.7, lines 1666–1671; R2-XT-OQ1; r4 R4-LOS-REC3 | research, source to check |
| 274 | Q8 rows T1–T4 (2 × step cost, ±8 ms, +8 ms wait, 286 kbps, interp 50–85 ms) | Options Q8 | r2 table A, lines 1675–1692 | research, source to check |
| 275 | Q8 T4 settled by TICK-30 | Options Q8 | TICK-30 | settled |
| 276 | Q9 rows E1–E4 (536, 251, 146, 188 kbps; 25 % below E3; 1,373 B over the datagram) | Options Q9 | r2 table B, lines 1694–1700; worked estimate line 1780 | research, source to check |
| 277 | Q10 rows U1–U3 (22.3 at 30 Hz arithmetic; 36 kbps; 22.3 kbps; loss of four in a row) | Options Q10 | r2 table C, lines 1704–1710 | research, source to check |
| 278 | Q11 rows P1–P4 | Options Q11 | r2 table D, lines 1712–1719 | research, source to check |
| 279 | Q12 settled by GL resolution 1 (25 % for the whole weighed and not taken) | Options Q12 | GL lines 167–188 | settled |
| 280 | Q13 rows A1–A4 (about 10 times fewer bytes spread out, none in a centre fight) | Options Q13 | r2 interest Q-A, lines 1796–1803; R2-INT-REC1 mark (d) | research, source to check |
| 281 | Q14 rows B1–B4 | Options Q14 | r2 interest Q-B, lines 1805–1812 | research, source to check |
| 282 | Q15 rows C1–C3 | Options Q15 | r2 interest Q-C, lines 1814–1820 | research, source to check |
| 283 | Q16 rows A–D (wtransport "not considered completely production-ready"; h3 "still very experimental", idle since 2025-05) | Options Q16 | r2 transport Q1, lines 1837–1846 | research, source to check |
| 284 | Q17 rows A, C; B and D settled by the stack | Options Q17 | r2 transport Q2, lines 1848–1855; brief §1 | research, source to check |
| 285 | Q18 rows A–C; direct calls settled by SLICE-ORDER condition 1 | Options Q18 | r2 transport Q4, lines 1866–1875; STATUS.md:96–97 | research, source to check |
| 286 | Q19 rows A–C combine | Options Q19 | r2 transport Q6, lines 1887–1896 | research, source to check |
| 287 | Q20 rows A–C (GameLift and PlayFab default; legacy resets in place, GE12) | Options Q20 | r2 rooms 3.1, lines 1900–1906; R2-ROOM-REC1 | research, source to check |
| 288 | Q21 rows A–C | Options Q21 | r2 rooms 3.2, lines 1908–1919 | research, source to check |
| 289 | Q22 rows A–C | Options Q22 | r2 rooms 3.3, lines 1921–1931 | research, source to check |
| 290 | Q23 rows A–C; a 6-symbol code from 32 is about 30 bits | Options Q23 | r2 rooms 3.4, lines 1933–1943 (32^6 = 1,073,741,824 ≈ 2^30) | arithmetic |
| 291 | Q24 rows A–C | Options Q24 | r2 rooms 3.5, lines 1945–1953 | research, source to check |
| 292 | Q25 rows A–C | Options Q25 | r2 rooms 3.6, lines 1955–1962 | research, source to check |
| 293 | Q26 rows A, B; C contradicts VISION_V2 §5 [DECIDED] | Options Q26 | r2 rooms 3.7, lines 1964–1971; VISION_V2 §5 line 58 | design doc |
| 294 | 30 Hz leaves 7.49 ms of 16.67 (6.49 on a re-flood tick) | Trade-offs | 16.67 − 9.18; 16.67 − 10.18 | arithmetic |
| 295 | The 60 Hz marks leave 0.05 ms on the worst tick and overrun the server-only share there | Trade-offs | row 71 | arithmetic |
| 296 | At P2 a 30 Hz client needs 200–250 ms of rewind | Trade-offs | row 139 | research verified |
| 297 | About +8 ms average input wait against 60 Hz | Trade-offs | r2 table A, T2 row | research, source to check |
| 298 | E3b at +42 kbps | Trade-offs | row 105 | arithmetic |
| 299 | WebTransport needs a secure context (HTTPS) | Consequences | r2 R2-TRN-F35, lines 1267–1271 (checked) | research verified |
| 300 | Guest secret in an HttpOnly cookie; guest id may stay in the KVStore successor | Consequences | r2 R2-ROOM-REC8 | research verified |
| 301 | Legacy constraints change for v2 code under SPIKE-SCOPE (a); nothing in CLAUDE.md changes in this PR | Consequences | SPIKE-SCOPE (a), STATUS.md:187–188 | settled |
| 302 | GC3 mark now: port to TypeScript; look brief pending; height-aim rule pending LOOP_V2 §9 | Consequences, rows | INV line 242 | research, source to check |
| 303 | GC3 proposed split (TypeScript ray from the camera; Rust aim query in the Worker) | Consequences, rows | part 5; r2 R2-SYNC-REC8 | proposed mark (follows part 5) |
| 304 | GN8 mark now: port to TypeScript; look brief pending; radii 90 m and 50 m | Consequences, rows | INV line 273 | research, source to check |
| 305 | GN8 proposed split (server event radii in Rust as data; client cull in TypeScript) | Consequences, rows | part 9 (f); r2 R2-INT-REC6 | proposed mark (follows part 9) |
