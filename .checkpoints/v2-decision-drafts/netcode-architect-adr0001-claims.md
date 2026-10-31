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
| 71 | R2-BUD-REC6's reason restated with "(at N = 54, MODES-3 (b))"; at N = 48 the tick fits by 0.05 ms (8.28 of 8.33), server-only 0.24 ms over (4.41 of 4.17) | Decision 3 | r2 lines 2402–2403 (superseded); GL lines 62–75; carry-forward 2 (STATUS.md:126–132); 8.33 − 8.28 = 0.05; 4.41 − 4.17 = 0.24 | arithmetic |
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
