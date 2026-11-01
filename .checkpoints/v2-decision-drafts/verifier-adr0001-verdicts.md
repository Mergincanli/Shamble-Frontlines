# Verifier verdicts: ADR 0001 (multiplayer and netcode), plan step 7

Verifier (Opus 5.5, xhigh), 2026-11-01, started 04:20 Amsterdam. Document: docs/adr/0001-multiplayer-netcode.md (1485
lines, HEAD 6b175de). Ledgers: netcode-architect-adr0001-claims.md (rows 1–360) and
test-engineer-passmarks-0001-claims.md (TE1–TE249). Rows I add for claims with no ledger row: N1, N2 … Research lines at
9e7db28 (the topic files are unchanged at HEAD). Verdicts: confirmed · wrong · unsupported · citation off · unverified ·
pending (outside fetch hours). "doc:n" = line n of the assembled ADR.

## Verdict table
| # | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|
| 1 | clients send only inputs, one command per tick | R2-SYNC-REC2 | confirmed | r2:2026 "Input commands: one per tick, sent with redundancy" | |
| 2 | snapshots 30 times a second | R2-BUD-REC1 | confirmed | r2:2302 "snapshots at 30 Hz" | |
| 3 | others drawn about 100 ms in the past | R2-SYNC-REC4 | confirmed | r2:2121–2122 "At an assumed 30 Hz snapshot rate, 3 intervals is 100 ms" | |
| 4 | 30 Hz; 60 Hz only with the 60 Hz margin | TICK-30; GL §8 | confirmed (settled) | STATUS:118–120; map J1 §8 text | |
| 5 | VISION_V2: up to 48 players, fair to ~100 ms | VISION_V2 §3, §4 | confirmed | VISION:45 "12v12v12 or 24v24"; :55 "about 100 ms of ping" | |
| 6 | largest mode 24v24, N = 48 | MODES-3 (a) | confirmed | STATUS:104–105 "the biggest match is 24v24 = 48" | |
| 7 | headroom run at N = 64 | GL 355–373 | confirmed | GL:357–359 "A run above the largest mode's N is a headroom run … N = 64" | correct GLOSSARY sense of "headroom" |
| 8 | start modes 1v1–4v4, 2v2v2–6v6v6; goal 12v12v12 or 24v24 | VISION_V2 §3 | confirmed | VISION:43–45 | |
| 9 | bots fill empty slots [DEFAULT] | VISION_V2 §3; LOOP_V2 §2 | confirmed | VISION:49; LOOP:20 | |
| 10 | TICK-30 quoted | STATUS:118–120 | confirmed | verbatim | |
| 11 | read as the 60 Hz margin (TICK-60-HEADROOM) | plan; advisor C2 | confirmed (settled) | plan:83–91; advisor-verdicts:21–26 | |
| 12 | fair to ~100 ms; one EU region [DEFAULT] | VISION_V2 §4 | confirmed | VISION:55 | |
| 13 | Chrome, Edge, Firefox, Safari [DECIDED] | VISION_V2 §4 | confirmed | VISION:52 | |
| 14 | §6 items 6.1–6.6, 6.8 served | VISION_V2 §6 | confirmed | VISION:64–79 | |
| 15 | §6.4 seats [OPEN]; §6.8 kill-cams [LATER]; §6.10 [LATER] | VISION_V2 §6 | confirmed | VISION:73, 79, 81 | rule 4: nothing built for them (checked parts 6, 9, 11) |
| 16 | LOOP_V2 §7 split-feedback quote | LOOP_V2 §7 | confirmed | LOOP:92 verbatim | |
| 17 | LOOP_V2 §4, §3, §9 [OPEN], §8 ~20 min, §10 | LOOP_V2 | confirmed | LOOP:41, 25, 128–129, 100, 133 | |
| 18 | LOOK_V2 §3 interiors, §4 enterable ruins, roofs fade [DEFAULT] | LOOK_V2 §3–§4 | confirmed | LOOK:32, 36–41 | [DEFAULT] belongs to the roof fade only (ruins are [DECIDED]); wording could misread |
| 19 | settled-inputs list | STATUS; UPGRADE-SCOPE; GL §8 | confirmed (settled) | STATUS:94–120, 187–188; plan:66–78 | no settled input reopened (condition 12) |
| 20 | delta vs acked snapshot: loss, reorder, unchanged free, mid-match join, no bit-exact agreement | R2-SYNC-REC1 | confirmed | r2:2006–2010 | |
| 21 | netcode must not depend on Wasm/native identity; lockstep rejected | R1-DET-REC1 | confirmed | r1:2131–2132 "the netcode must not depend on it … Reject lockstep (Q1-E)" | |
| 22 | E3 at 30 Hz, N = 48 all in view ≈ 146.1 kbps | worked estimate; R2-BUD-REC2 | confirmed | r2:1775 "146.1 kbps"; recomputed (583.5 B × 30 + 758.4 B) × 8 = 146.1 | |
| 23 | 60 Hz snapshots ≈ 286.1 kbps, over Fiedler's 256 kbps example | worked estimate; R2-BUD-REC1 | confirmed | r2:1778; r2:2304–2305; recomputed 286.1 | |
| 24 | 9.18 of 16.67 ms at 30 Hz (10.18 re-flood) | GL 53–69 | confirmed | GL:68–69; recomputed 5.533 + 3.644 = 9.177 | |
| 25 | 7.28 of 8.33 at 60 Hz; 8.28 re-flood; server-only 4.41 of 4.17 | GL 60–69 | confirmed | GL:60–69; recomputed 3.867 + 3.410 = 7.277; +1 = 8.277; 4.410 | |
| 26 | every received byte readable; withholding the only defence | R2-INT-REC2 | confirmed | r2:2453 | |
| 27 | QUIC 1,200 B; no IP fragmentation; datagrams not fragmented | R2-BUD-REC3 | confirmed | r2:2348–2349 | |
| 28 | neither WT crate claims production readiness | R2-TRN-REC1 | confirmed | r2:2571 | |
| 29 | some networks do not carry QUIC | R2-TRN-REC3 (F12) | confirmed | r2:2643 "Some networks don't support QUIC (F12)"; F12 verifier-confirmed (r2:3673, section number only) | |
| 30 | known spread seed lets a cheat cancel spread (H2) | R2-SYNC-REC3 | confirmed | r2:2086–2088 | |
| 31 | takes from / gives to the other drafts | assignment | confirmed (scope statement) | map assignments; no research claim | |
| 32 | CS1: command and 1/60 step never wired; drop | INV:169 | confirmed | INV:169 mark "drop"; code shamble-core.js:1479 FIXED_DT=1/60, no caller of makeInputCommand | |
| 33 | CS7: pure query for a rewound snapshot; rewrite in Rust | INV:175 | confirmed | INV:175 "so the server can run it against a rewound snapshot" | |
| 34 | GC5: fixed chest origin; Math.random spread | INV:244 | confirmed | INV:244 | |
| 35 | GC2: slot id = sim id | INV:241 | confirmed | INV:241 | "without a remap" is r2:2818's "makes the rebind cheap" |
| 36 | GP2: event-time actions → press edges | INV:312 | confirmed | INV:312 | |
| 37 | GC13: preview and launch differ; one trajectory function | INV:252 | confirmed | INV:252 | |
| 38 | GN8: sound 90 m, visuals 50 m | INV:273 | confirmed | INV:273; code shamble-core.js:106 botCullR:50, botSfxR:90 | |
| 39 | GC1 + STATUS ADR-001: team allow-list; humans take bot slots | INV:240; STATUS:199 | confirmed | STATUS:199 "the server must repeat the team allow-list … humans take over bot slots" | |
| 40 | GP13: variable step and freezes cannot survive MP | INV:323 | confirmed | INV:323 "none of which can survive multiplayer" | |
| 41 | decision summary (authority, commands, delta/quantised/interest snapshots, WT + WS) | R2 recs | confirmed | r2:1996–2010, 2302, 2326, 2420, 2565, 2639 | |
| 42 | match server the authority; Worker in a local match | GL 485–499 | confirmed | GL:486–488 | |
| 43 | no client state, hit or RTT taken | R2-SYNC-REC3, REC5 | confirmed | r2:2097–2098; r2:2148–2149 "never a figure the client reports" | |
| 44 | lockstep rejected | R1-DET-REC1 | confirmed | r1:2132 | |
| 45 | option 3.5 C is an authority leak | r2 1647–1653 | confirmed | r2:1653 "An authority leak (trivially cheatable)" | |
| 46 | §6.5 "All server-side" | VISION_V2 §6.5 | confirmed | VISION:74 | |
| 47 | tick period 33.3 ms | GL Tick period | confirmed | GL:83 | |
| 48 | the 60 Hz rule and its setup | TICK-60-HEADROOM | confirmed (settled) | advisor C2 (advisor-verdicts:21–25); map J1 N3; doc:118–126 match word for word in substance | |
| 49 | 1 − 6.25 ÷ 8.33 = 25 % | arithmetic | confirmed | 6.25 ÷ 8.333 = 0.750 | |
| 50 | two shares at 60 Hz, 4.2 ms each | GL 189–213 | confirmed | GL:190 "(4.2 ms at 60 Hz, 8.3 ms at 30 Hz)" | |
| 51 | snapshots every tick; inputs sampled and sent every tick | R2-BUD-REC1 | confirmed | r2:2302–2303 | |
| 52 | 60 or 64 Hz asked only if the margin passes | R1-TICK-OQ1 | confirmed | r1:2640 | |
| 53 | legacy weapons fit one shot per tick at 30 Hz; replay halves | R1-TICK-REC2 | confirmed | r1:2035–2037 | |
| 54 | R1-TICK-REC2 decision rule quoted | r1:2061–2062 | confirmed | verbatim with ellipsis | |
| 55 | R2-BUD-REC1 tick half quoted | r2:2302 | confirmed | verbatim | |
| 56 | "the rule does not change" | r2:2415–2416 | confirmed | r2:2416 | |
| 57 | "which does not change"; §8 closing note | GL:85; map J1 | confirmed | GL:85; closing note text = map J1 | §8 not yet in GLOSSARY.md (lands in step 6) |
| 58 | 30 Hz: p99 ≤ 16.7 ms, max < 33.3 ms, 0 overruns | GL 167–188 | confirmed | GL:168–170 | |
| 59 | shares ≤ 8.3 ms p99 each, measured separately | GL 189–213; R2-BUD-REC6 | confirmed | GL:190; r2:2401 | |
| 60 | encode ≤ 50 µs per client per snapshot | R2-BUD-REC6 | confirmed | r2:2414–2415 | |
| 61 | 48 × 50 µs = 2.40 ms | arithmetic | confirmed | GL:61 | |
| 62 | rewound shot ≤ 20 µs at 700 shots/s | R2-SYNC-REC5 mark 4 | confirmed | r2:2186–2187 | |
| 63 | 700 ÷ 30 × 20 µs = 0.47 ms | arithmetic | confirmed | 466.7 µs; GL:62 | |
| 64 | region ≤ 0.2 ms, rank+pack ≤ 0.5 ms (0.70) | R2-INT-REC1 (c), REC4 | confirmed | r2:2440, 2499–2500; GL:63 | |
| 65 | pair test ≤ 0.1 µs, 768 pairs, 0.08 ms | R2-INT-REC2 (b) | confirmed | r2:2463; 48 × 32 ÷ 2 = 768; 76.8 µs | |
| 66 | re-flood ≤ 1 ms | R2-INT-REC2 (b) | confirmed | r2:2464 | |
| 67 | each send call ≤ 1 ms; sends share what is left | R2-TRN-REC1 mark 2; GL | confirmed | r2:2591; GL:201–202 | |
| 68 | 3.64 (4.64) of 8.33; 9.18 (10.18) of 16.67 | arithmetic | confirmed | GL:65–69 | |
| 69 | p99 measured directly, in one run | GL time statistics | confirmed | GL:145–146 "always measured directly"; "in one run" is GL §8 (map J1) | |
| 70 | transport runtime ≤ 1 vCPU, outside server tick time, in rooms per VM | GL 130–139 | confirmed | GL:131–134 | |
| 71 | R2-BUD-REC6 reason restated with "(at N = 54, MODES-3 (b))"; 0.05 ms; 0.24 ms over | r2:2402–2403; GL:62–75; CF-2 | confirmed, with a note | r2:2402–2403; STATUS:128–132; GL:67, 69 | arithmetic on rounded figures (exact margin 8.333 − 8.277 = 0.057 ms). The restatement inserts "re-flood ticks included" inside the quote marks; true (GL:76–77) but not r2's words (finding F9) |
| 72 | need §6.1 | R2-BUD-REC6 | confirmed | r2:2410 "§6.1 (rooms)" | |
| 73 | prediction scope | R2-SYNC-REC3; GL 501–523 | confirmed | r2:2068–2069; GL:502–505 | |
| 74 | weapon state includes active reload | r2 options 3.2 C | confirmed | r2:1628 "ammo, cooldowns, reload and active reload" | |
| 75 | reset to last run command, replay; skip when matching | R2-SYNC-REC3 | confirmed | r2:2071–2076 | |
| 76 | 200 ms (6 ticks) ≤ 1 ms p99 on ML | R1-TICK-REC2 P3 | confirmed | r1:2058–2060 | |
| 77 | no whole-world replay; 0004 may drop R1-ECS-REC1 mark 3 | R2-XT-OQ7; R1-XU-OQ2 | confirmed | r2:3367–3372 "ADR-0004 keeps or drops R1-ECS-REC1 mark 3" | |
| 78 | fade 100 ms, snap 2 m (proposed), clear on respawn | R2-SYNC-REC3 | confirmed | r2:2079–2080 | |
| 79 | own state unquantised or quantised the same | R2-SYNC-REC3 | confirmed | r2:2094–2096 | |
| 80 | never predicted: damage, deaths, hits; uplink countdown | R2-SYNC-REC3; R1-ABL-REC4 | confirmed | r2:2089–2092; r1:2425 | |
| 81 | offers, picks, pickups not predicted | proposed | confirmed (proposed, labelled) | doc:174–175, OQ-0001-10 | |
| 82 | abilities wait for ABL-PREDICT | ABL-PREDICT | confirmed (settled) | STATUS:110; r2:3284–3288 | |
| 83 | SEC-SPREAD (a) | STATUS:108–109 | confirmed (settled) | verbatim sense | |
| 84 | spread from an authority-only stream | GL 525–544; R1-DET-REC4 | confirmed | GL:527–531; r1:2188–2191 | |
| 85 | one command per tick; repeats unacked up to cap 8 | R2-SYNC-REC2 | confirmed | r2:2033–2034 | |
| 86 | command fields | R2-SYNC-REC2 | confirmed | r2:2027–2032 | |
| 87 | aim ≤ 80 bits | R2-SYNC-REC8 mark 2 | confirmed | r2:2274 | |
| 88 | input datagram carries newest snapshot (ack) | R2-SYNC-REC1 | confirmed | r2:1999 "Every client packet carries the newest snapshot it received" | doc says "decoded"; equivalent |
| 89 | server command checks | R2-SYNC-REC2 | confirmed | r2:2042–2051 (all eight checks) | |
| 90 | catch-up cap ≤ redundancy cap | R2-SYNC-REC2 | confirmed | r2:2039–2041 "proposed no larger than the redundancy cap" | Q10 says "equals" (finding F10) |
| 91 | getStats() missing on Chrome/Edge, throws on Firefox | R2-TRN-REC4 (F9) | confirmed | r2:2675–2676; F9 verifier-confirmed | |
| 92 | clock drift 1–2 ms/frame, halve over 100 ms, reset over 500 ms | R2-SYNC-REC4 | confirmed | r2:2112–2113 | |
| 93 | input buffer 1–3 ticks (proposed), adapted | R2-BUD-REC4 | confirmed | r2:2363–2364 | |
| 94 | command stamped with its server tick | proposed | confirmed (proposed, labelled in ledger) | doc:199 | |
| 95 | page posts cursor ray + view time; Worker casts with the sim ray query | R2-SYNC-REC8; R2-XT-OQ1; R4-LOS-REC3; R1-BND-REC1 m2 | confirmed | r2:2252–2254; r2:3326–3331; r4:1008–1009; r1:2262 | answers R2-XT-OQ1 as the research routes it to ADR-0001 |
| 96 | server rebuilds ray from its fire origin to the point, extends to range | R2-SYNC-REC8, REC5 | confirmed | r2:2255–2257, 2140–2141 | |
| 97 | throws and charged shots use the point | R2-SYNC-REC8 | confirmed | r2:2259–2260 | |
| 98 | aim help applied by the server | R2-SYNC-REC8 (L1) | confirmed | r2:2253–2255 | |
| 99 | the page makes no Wasm calls | R1-BND-REC1 m2 | confirmed | r1:2262 "the page makes zero calls into Wasm" | |
| 100 | delta vs acked; defaults for new; explicit removals; no baseline → defaults | R2-SYNC-REC1 | confirmed | r2:1998–2004 | |
| 101 | shared world-state ring; per-client sent ring, proposed 32 | R2-SYNC-REC1 | confirmed | r2:1997–1998, 2004–2005 | |
| 102 | 32 ≈ 1.07 s at 30 Hz | arithmetic | confirmed | 32 ÷ 30 = 1.067 | |
| 103 | E3: 1 cm in 16/16/12 bits (±327.68 m, 0–40.95 m), yaw 16, pitch 10 | R2-BUD-REC2; estimate | confirmed | r2:2326–2327, 1745–1748; 2^16 cm = 655.36 m; 2^12 cm = 40.96 m | |
| 104 | projectiles as trajectory records; re-based on relevance | R2-SYNC-REC6 | confirmed | r2:2196–2200 | |
| 105 | E3b ≈ +42 kbps | R2-BUD-OQ8; estimate | confirmed | r2:2994; 188.3 − 146.1 = 42.2 | |
| 106 | hard cap 256 kbps (proposed), UDP payload + 28 B | proposed; R2-BUD-REC2 | confirmed (proposed, labelled) | r2:2330 "enforce a hard per-client cap"; r2:2340; r2:1789 | |
| 107 | 256 kbps ≈ 1,067 B per interval | arithmetic | confirmed | 256,000 ÷ 8 ÷ 30 = 1,066.7 | |
| 108 | datagram cap min(smallest maxDatagramSize, 1,150 B); 1,000 B until measured | R2-BUD-REC3; R2-TRN-REC2; R2-XU-OQ5 | confirmed | r2:2345–2346, 2631–2632, 3158–3165 | |
| 109 | split by entity groups, decode alone, acked per group | R2-BUD-REC3; R2-SYNC-OQ4 | confirmed | r2:2346–2347, 2917–2919 | |
| 110 | first full state over a reliable stream | R2-BUD-REC3 (P4) | confirmed | r2:2347–2348 | |
| 111 | gameplay events until acked; bounded queue; resync or disconnect; cosmetic once | R2-BUD-REC2 (M4) | confirmed | r2:2329–2332 | |
| 112 | hit confirmation a gameplay event | R2-SYNC-REC5 | confirmed | r2:2171 "Hit confirmation goes back as a reliable event with the tick" | |
| 113 | remote soldier fields; action starts as cosmetic events | estimate rows | confirmed | r2:1745–1749 (position, yaw, pitch, health, flags, weapon); flag meanings and action-start events are the draft's answer to R2-XT-OQ8 (r2:3373–3378) | |
| 114 | fits 90-bit full and 60-bit delta records; flags rarely change | estimate | confirmed | r2:1750–1751 "health and flags rarely"; 44+16+10+8+8+4 = 90 | |
| 115 | vehicles: hull/turret transforms, health per part; parts and seats with parent | R2-SYNC-REC7; R2-INT-REC6 | confirmed | r2:2233–2235, 2532 | |
| 116 | sim id (kind, index, generation), never a library id | R1-ECS-REC3 | confirmed | r1:1973–1975 | |
| 117 | record names kind+index; generation on entry | proposed | confirmed (proposed) | | the generation's bytes on entry records are not added to the estimate R1-XT-OQ11 asks to recompute (r1:3082–3083); low note |
| 118 | 32-bit header tick, 16-bit offsets | proposed; R2-XT-OQ11 | confirmed (proposed) | r2:3390–3394 | |
| 119 | 16-bit wrap 36.4 min at 30 Hz, 18.2 at 60 Hz | arithmetic | confirmed | 65,536 ÷ 30 = 2,184.5 s; ÷ 60 = 1,092.3 s | |
| 120 | +2 B on the 8 B header ≈ +0.5 kbps | arithmetic | confirmed | r2:1743 (8 B, assumption); 2 × 8 × 30 = 480 bit/s | |
| 121 | wire format ≠ block; Worker decodes then writes the block | R1-BND-OQ8 | confirmed | r1:2745–2746 "couples the wire format to presentation" | |
| 122 | no state hash exchanged; compared hash leaves authority-only streams out | GL state hash; R1-DET-OQ9 | confirmed | GL:549–550; r1:2706–2709 | |
| 123 | buffer in the Worker; page interpolates the two newest blocks | R2-XT-OQ2; R1-TICK-REC5 | confirmed | r2:3334–3336; r1:2099 | |
| 124 | start 3 intervals, adapt 2–5 | R2-SYNC-REC4 | confirmed | r2:2110–2111 | |
| 125 | 66.7–166.7 ms | arithmetic | confirmed | 2 × 33.33; 5 × 33.33 | |
| 126 | extrapolate ≤ 1 interval then hold; placed respawns; linear first | R2-SYNC-REC4 | confirmed | r2:2114–2118 | |
| 127 | remote projectiles on the delayed timeline | R2-SYNC-REC6 | confirmed | r2:2205–2207 | |
| 128 | announced delay clamped, ≤ 1 interval/s | R2-SYNC-REC5 | confirmed | r2:2151–2153 | |
| 129 | three intervals: Fiedler's measured rule at 2–5 % loss | R2-SYNC-REC4 | confirmed | r2:2120–2121 | |
| 130 | history of sim hit volumes, no animation | R2-SYNC-REC5 | confirmed | r2:2134–2137 | |
| 131 | ray from server fire origin; movers at view time, statics now | R2-SYNC-REC5 | confirmed | r2:2140–2143 | |
| 132 | tolerance ≤ one interval (33.3 ms) | R2-SYNC-REC5 (H1) | confirmed | r2:2147–2148 | |
| 133 | RTT: quinn rtt(), or lowest recent own ping on WS | R2-SYNC-REC5 (H4) | confirmed | r2:2148–2151 | |
| 134 | cap bounds the final rewind; 200 or 250 ms open | R2-SYNC-REC5; R2-XU-OQ1 | confirmed | r2:2154–2157; r2:3123 | |
| 135 | history = cap + margin, ≤ 1 MB | R2-SYNC-REC5 m3 | confirmed | r2:2138; r2:2184–2185 | |
| 136 | filters; died/teleported stops | R2-SYNC-REC5 | confirmed | r2:2163–2167 | |
| 137 | bots shoot at present | R2-SYNC-REC5 (F31) | confirmed | r2:2168 | |
| 138 | view-time offsets and aims logged; detection later | R2-SYNC-REC5 | confirmed | r2:2160–2162 | |
| 139 | P2 with 100–150 ms delay needs 200–250 ms; 200 ms cap clips some | R2-SYNC-REC5 | confirmed | r2:2157–2158 "a 100 ms player with a 100–150 ms delay needs 200–250 ms (F28's formula; inferred)" | research marks it inferred |
| 140 | ~1.8 m lead without lag compensation | R2-SYNC-REC5 | confirmed | r2:2172–2174; 9 m/s × 0.2 s | |
| 141 | region from camera worst-case view, margin, hysteresis, grid | R2-INT-REC1 | confirmed | r2:2421–2427 | |
| 142 | areas, portals, flood numbers, re-flood | R2-INT-REC2 | confirmed | r2:2447–2451 | |
| 143 | area ids shared with navigation | R4-FLOORS-REC2 | confirmed | r4:927–929 "the same area ids as R2-INT-REC2's interiors-and-floors map data" | R4 recommendation; no external fact needed |
| 144 | one shared pass for bots; sight hiding only if R2-INT-OQ1 says | R2-INT-REC3 | confirmed | r2:2469–2473 | |
| 145 | own state always; nearest 8 every snapshot; all every 4 | R2-INT-REC4 | confirmed | r2:2488–2489, 2498–2499 | |
| 146 | visibility classes public/team/owner/spatial | R2-INT-REC5 | confirmed | r2:2502–2508 | |
| 147 | event radii, area test, shots at viewer, stripped shooter id, big-unit bounds | R2-INT-REC6 | confirmed | r2:2525–2532 | |
| 148 | a shooter can only hit what it was sent | r2 pointers | confirmed | r2:3431–3433 | |
| 149 | cutaway follows the same rule | R2-INT-REC2; R2-XT-OQ15 | confirmed | r2:2451–2452 | |
| 150 | spectators only if Mustafa wants them | R2-INT-REC7; R2-XU-OQ17 | confirmed | r2:2546, 3258–3259 | |
| 151 | quinn 0.11.x; pick between wtransport 0.7.x and web-transport-quinn 0.13.x; h3 and own handshake out | R2-TRN-REC1 | confirmed | r2:2565–2566 | |
| 152 | tokio-tungstenite 0.30.x; axum only if serving HTTP | R2-TRN-REC3 | confirmed | r2:2639–2640 | |
| 153 | room serves no HTTP pages | proposed | confirmed (proposed, labelled) | | |
| 154 | one transport per client; requireUnreliable; no WT over HTTP/2; newest snapshot only on WS | R2-TRN-REC3 | confirmed | r2:2640–2641, 2652–2653 | |
| 155 | WT timeout 1.5 s then WS | R2-TRN-REC3 m1 | confirmed | r2:2665 | |
| 156 | datagrams; one bidi control stream; bulk on uni streams | R2-TRN-REC2 | confirmed | r2:2617–2619 | |
| 157 | thin interface, four operations, outside the sim | R2-TRN-REC4; options Q4 A | confirmed | r2:2673; r2:1869–1870 | |
| 158 | third, loopback backend | SLICE-ORDER cond. 1 | confirmed (settled) | STATUS:97–98 | |
| 159 | decoders never panic, size first; sim thread never parses bytes | R2-TRN-REC4 | confirmed | r2:2679–2680 | |
| 160 | version in `protocols` and Join | R2-TRN-REC4 | confirmed | r2:2681–2682 ("first control message everywhere else") | |
| 161 | NewReno/Cubic; non-waiting send; outgoingMaxAge 100 ms; RTT/loss feed adaptation | R2-TRN-REC6 | confirmed | r2:2725–2729 | |
| 162 | drop low priority, then 20 Hz, never above cap | R2-BUD-REC5 | confirmed | r2:2384 | |
| 163 | 20 Hz = two of three ticks | arithmetic | confirmed | | |
| 164 | hostile-input limits | R2-TRN-REC1 m4; REC3 | confirmed | r2:2599–2606, 2654–2658 | |
| 165 | version floors; dangerous-configuration off | R2-TRN-REC1 m5 (F40) | confirmed (research and source) | r2:2610–2612; source S1: rustsec.org RUSTSEC-2026-0338/0340/0343 "Patched >=0.11.18", 0346/0348 ">=0.23.46", 2025-0009 ">=0.17.12", 0384 ">=1.54.1" | F40 was outside R2's checked set |
| 166 | advisories check; policy is the baseline's | R2-TRN-REC1 m5; GL | confirmed | r2:2612–2615; GL:691 | |
| 167 | production: certificate per hostname | R6-HOST-REC4; CF-4 | confirmed | r6:684–686; STATUS:135–137 | R6 recommendation resting on R2-TRN-REC5 (no external fact) |
| 168 | dev: self-signed, pinned, ≤ 14 days, hash from own HTTPS origin | R2-TRN-REC5 | confirmed | r2:2700–2703 | |
| 169 | need lines (loopback; congestion §4 only) | map J3 | confirmed (settled by the session's reading) | map J3 (decision-map:1134–1141) | see finding F7 on R2-XU-OQ18 |
| 170 | lifecycle, timeouts, health check, no pause, fresh sim | R2-ROOM-REC1 | confirmed | r2:2748–2753 | |
| 171 | fresh sim resets player records and tracks | UPGRADE-SCOPE | confirmed (settled) | plan:72 "reset at the start of every match" | |
| 172 | one room per process; panic = "unwind"; isolation | R2-ROOM-REC2 | confirmed | r2:2762–2768 | |
| 173 | token claims; 30 s, ≤ 5 s leeway; first control message | R2-ROOM-REC3 | confirmed | r2:2784–2789 | |
| 174 | signature scheme per the security review | R2-ROOM-OQ2 | confirmed | r2:3085–3089 | constant-time MAC compare omitted (moot with asymmetric) |
| 175 | room assigns team and squad | R2-ROOM-REC3 | confirmed | r2:2789–2790 | |
| 176 | grace 60 s; fresh token; newest connection wins | R2-ROOM-REC4 | confirmed | r2:2813–2815 | |
| 177 | private rooms: code locates, limits, host checks, faction per team | R2-ROOM-REC5 | confirmed | r2:2824–2831 | |
| 178 | slot table; bots per team; same input path | R2-ROOM-REC6 | confirmed | r2:2842–2845 | |
| 179 | room = mode id + overrides | R2-ROOM-REC7 | confirmed | r2:2857 | |
| 180 | guest id + ≥ 64-bit secret in HttpOnly/Secure/SameSite=Strict cookie | R2-ROOM-REC8 (F19) | confirmed (research and source) | r2:2869–2872; source S2: OWASP Session Management cheat sheet, "The localStorage API": "Use HttpOnly; Secure; SameSite=Strict cookies (preferred)" | F19 was outside R2's checked set |
| 181 | match record before Closed | R2-ROOM-REC1 m3 | confirmed | r2:2760–2761 | |
| 182 | hosting services: bounded life, one process each | R2-ROOM-REC1, REC2 | confirmed | r2:2754, 2769 | |
| 183 | authority over tracks, offers, picks, drops, pickups | UPGRADE-SCOPE | confirmed (settled) | plan:70–73 | |
| 184 | offers owner class; PickUpgrade; refusals | proposed | confirmed (proposed, labelled) | doc:375–378; "*Research:* no research id concerns upgrades" | |
| 185 | dropped weapons are sim entities, spatial | UPGRADE-SCOPE | confirmed (settled) | plan:72 "dropped weapons are world pickups" | |
| 186 | pickups via button edges; authority picks | proposed | confirmed (proposed) | | |
| 187 | carry slots, ammo, levelling are Mustafa's | UPGRADE-SCOPE | confirmed (settled) | plan:74–75 | |
| 188 | Quake III predicts item pickups | R2-SYNC-F17 | confirmed | r2:230–231; verifier-confirmed (r2:3594) | |
| 189 | no research id concerns upgrades | id table | confirmed | research-ids.tsv: only R5-RUSTLINT-REC6 (a toolchain upgrade) matches | |
| 190 | profiles P0–P4, B1; round trip; reordering | r2 1987–1992 | confirmed | r2:1985–1992 | |
| 191 | budgets/interest/rooms latencies restated as P2, P0–P2+P4, P0–P4 | R2-XU-OQ12; R2-XT-OQ14 | confirmed | r2:3221–3223 | |
| 192 | "100 ms of simulated latency" read as P2 | proposed (OQ-0001-3) | confirmed (proposed; the Advisor confirms) | map J6 (3) | |
| 193 | the simulator is ADR 0002's | R5-TEST-REC4 | confirmed | r5:1454; map owner | |
| 194 | proof test before the foundation, throwaway, spikes/, after approval | SLICE-ORDER c3; SPIKE-SCOPE (a) | confirmed (settled) | STATUS:101, 187–188 | |
| 195 | research spikes taken in | named recs | confirmed | r2:2411–2416, 2292–2298, 2585–2615, 2663–2671, 2774–2782; r1:2044–2060 | |
| 196 | page never handles a packet; Worker owns the connection | R1-BND-REC6 | confirmed | r1:2326–2330 | |
| 197 | sim crate depends on no transport crate | R2-TRN-REC4 m3 | confirmed | r2:2695 | |
| 198 | server tick order | R1-TICK-REC4; GL server tick time | confirmed | r1:2078–2085; GL:114–118 | |
| 199 | client tick online | R2-SYNC-REC1/3/4; R1-BND | confirmed | consistent with parts 4–7 | |
| 200 | local match: same Worker runs the local server; loopback | SLICE-ORDER c1; R1-BND-REC6 | confirmed (settled) | STATUS:97–98; r1:2327–2329 | |
| 201 | state ownership table | parts 1–12 | confirmed | each row matches its part | |
| 202 | team economy team or public: R2-INT-OQ3 | R2-INT-OQ3 | confirmed | r2:3014–3017 | |
| 203 | uplinks, countdown, steals, cache location public | R2-INT-REC5; LOOP_V2 §4 | confirmed | r2:2503–2504; LOOP:41 | |
| 204 | camera on the page; bounds as mode data | R2-INT-REC1 | confirmed | r2:2421 | |
| 205 | content hash named at join | R1-DATA-REC6 | confirmed | r1:2567 "at join the server names the hash" | R1 recommendation; research verified (C16) |
| 206 | every message type has a class; audit fails otherwise | R2-INT-REC5 | confirmed | r2:2520–2521 | |
| 207 | Input contents and checks | R2-SYNC-REC2; R2-TRN-REC1 m4 | confirmed | r2:2027–2034, 2601 | |
| 208 | Snapshot sections | estimate; R2-INT-REC5 | confirmed | r2:1743, 1756–1760; r2:2502–2508 | |
| 209 | Ping/Pong datagrams, a few per second | proposed | confirmed (proposed) | r2:2675 "own ping messages" | |
| 210 | Join/JoinAccepted/RoomState/HostCommand/RoomEnded/Leave | r2 2886–2890 | confirmed | r2:2886–2890 | |
| 211 | Join carries version and content hash | R2-TRN-REC4; R1-DATA-REC6; R2-XT-OQ6 | confirmed | r2:2681–2682; r1:2567; r2:3362–3364 | |
| 212 | Join within 2 s | R2-TRN-REC1 m4 | confirmed | r2:2603 | |
| 213 | FullState on a uni stream | R2-BUD-REC3; R2-TRN-REC2 | confirmed | r2:2347; r2:2619 | |
| 214 | ClientSettings clamp | R2-SYNC-REC5 | confirmed | r2:2151–2153 | |
| 215 | HostCommand checks | R2-ROOM-REC5, REC7; R1-DATA-REC7 | confirmed | r2:2829–2830; r1:2582–2584 | |
| 216 | PickUpgrade checks | proposed | confirmed (proposed) | | |
| 217 | ContentReload dev only, if 0005 keeps it | R1-DATA-REC5 | confirmed | r1:2543–2545; keep-or-drop is R1-DATA-OQ12 (r1:2557–2558) | |
| 218 | Kick as a message | r2 2886–2890 | confirmed, with a note | r2:2888 `HostCommand{… kick …}`, `Leave` | a room→client `Kick` reason message is the draft's addition (the research has only the host's kick command) |
| 219 | lobby calls; Origin check; limits | r2 2889–2890; REC5; REC8 | confirmed | r2:2889–2890, 2871, 2825–2828 | |
| 220 | supervisor calls | R2-ROOM-REC1/2; R6-TEL-REC1 | confirmed | r2:2752, 2763; r6:784 | |
| 221 | 584 B (609), 146.1; E3b 188.3; 44.2; 70.3; 116.2; 183.7 | estimate | confirmed | r2:1772–1779; recomputed 583.5 B, 146.1, 188.35 kbps | R2 Options text (not a recommendation): checked by me, all from R2's own stated assumptions |
| 222 | interest cuts the per-entity part and events, not the overhead | r2 pointers | confirmed | r2:3445–3447 | |
| 223 | ≤ 100 kbps mean on the Octopon-3 greybox | R2-BUD-REC2 | confirmed | r2:2340–2341 | |
| 224 | IPv6 +20 B: +4.8 kbps | estimate | confirmed | r2:1784; 20 × 8 × 30 = 4,800 | |
| 225 | Fiedler's 256 kbps example | r2:1789 (F34) | confirmed | r2:1789; R2-BUD-F34 verifier-confirmed (r2:3642) | |
| 226 | 8 B commands, newest + 3, 30 Hz ≈ 22.3 kbps | table C | confirmed | r2:1708 (44.6 at 60 sends/s); r2:1710 (U3 22.3 at 30) | |
| 227 | upstream ≤ 48 kbps (IPv4) | R2-BUD-REC1 | confirmed | r2:2324 | |
| 228 | 200 B per datagram; 143 B after 57 B | arithmetic | confirmed | 48,000 ÷ 8 ÷ 30 = 200; r2:1742 57 B | |
| 229 | 48 × 146 ≈ 7.0 Mbit/s; 3.2 GB per match-hour; bots free | estimate | confirmed | r2:1786–1787 | |
| 230 | 48 × 22.3 ≈ 1.07 Mbit/s in | arithmetic | confirmed | 1,070.4 kbps | |
| 231 | R6: 3.16 GB per room-hour inside 20 TB per VM | R6-COST-REC1 | confirmed | r6:706–708 "a full CCX23 stays inside its included 20 TB … R2's 3.16 GB per room-hour"; 20 TB rests on R6-HOST-F15, inside R6's checked set | |
| 232 | WebSocket overhead not estimated | proposed | confirmed | | |
| 233 | one vCPU of the reference server (CCX23) | GL; R6-HOST-REC2 | confirmed | GL:226–227; r6:645–647 | specs rest on R6-HOST-F15 (checked in R6's set) |
| 234 | 30 Hz p99 ≤ 16.7, max < 33.3, 0 overruns | GL; R2-BUD-REC6 | confirmed | GL:168–170; r2:2413–2414 | |
| 235 | shares ≤ 8.3 ms each, separately | GL; R2-SYNC-REC9 m1 | confirmed | GL:190; r2:2292–2296 | |
| 236 | parts at p99, reported beside, never summed | GL time statistics | confirmed | GL:143–146 | |
| 237 | 60 Hz margin row | TICK-60-HEADROOM | confirmed (settled) | | |
| 238 | replay row | R1-TICK-REC2 P3 | confirmed | r1:2058–2060 | |
| 239 | transport runtime row; 30 input datagrams/s | R2-TRN-REC1 m2 | confirmed | r2:2589–2592 (60/s "placeholder numbers until the tick and bandwidth unit sets them") | |
| 240 | downstream mean ≤ 160, p99 1-s ≤ 256 | R2-BUD-REC2 | confirmed | r2:2339–2340 | the research mark was set on the estimate scene (32 projectiles); the budgets table puts it in the stress scene (300); PM-0001-12 reports both, the table row does not say so (note N12) |
| 241 | upstream ≤ 48 kbps under P2 | R2-BUD-REC1 | confirmed | r2:2320, 2324 (±10 ms restated as P2's ±20 ms) | |
| 242 | memory ≤ 64 MB (proposed), RSS at start and 20 min | R2-ROOM-REC2 m3 | confirmed | r2:2778–2779 | |
| 243 | per-session memory flat | R2-TRN-REC6 m1 | confirmed | r2:2742 | |
| 244 | start-up < 2 s | R2-ROOM-REC2 m5 | confirmed | r2:2780 | |
| 245 | deltas within 500 ms after B1 | R2-SYNC-REC1 m2 | confirmed | r2:2020 | |
| 246 | no QUIC → WS ≤ 3 s | R2-TRN-REC3 m1 | confirmed | r2:2664–2665 | |
| 247 | browser draft move; recurring beta test | R2-TRN-OQ1 | confirmed | r2:3036–3040 | |
| 248 | window can fall to 2 datagrams | R2-TRN-REC6; R2-BUD-REC5 | confirmed | r2:2735; r2:2390 | |
| 249 | never-acking client capped | R2-BUD-REC2 | confirmed | r2:2342–2344 | |
| 250 | overrun → bounded catch-up, dropped time recorded | R1-TICK-REC4 | confirmed | r1:2082–2084 | |
| 251 | handler panic closes one connection; dead room leaves others | R2-ROOM-REC2 m4, m6 | confirmed | r2:2779–2782 | |
| 252 | trap → Worker restart, resync via FullState | R2-XT-OQ12; pointers | confirmed | r2:3395–3398, 3480–3482 | |
| 253 | hidden tab → resync within 0.5 s | R1-TICK-REC5 | confirmed | r1:2113–2114 | |
| 254 | drop → slot held; 100 % rejoin | R2-ROOM-REC4 m1 | confirmed | r2:2820–2821 | |
| 255 | lobby down → running rooms go on | R2-ROOM-REC3 | confirmed | r2:2791 "an offline check keeps the lobby off the hot join path" | |
| 256 | version/content mismatch → JoinRejected, reload | R1-DATA-REC6 | confirmed | r1:2576–2577 | |
| 257 | 60 Hz soak crosses 65,536 | arithmetic | confirmed | 108,000 > 65,536 | |
| 258 | certificate expiry: ADR 0006's alert | R6-HOST-REC4 | confirmed | r6:698–699 | |
| 259 | no Hetzner EU capacity: R6-HOST-OQ1 | R6-HOST-REC2 | confirmed | r6:652–655 | doc states no capacity fact, only the open question; F20 needs no check here |
| 260 | client sim online is a predicting copy | GL authority | confirmed | GL:487–488 | |
| 261 | forged commands rejected, counted, no effect beyond valid part | R2-SYNC-REC2 m3 | confirmed | r2:2060–2061 | |
| 262 | client aim help indistinguishable from an aimbot | R2-SYNC-REC8 | confirmed | r2:2254–2255 | |
| 263 | reader client and byte audit | R2-INT-REC2 (a), REC5 | confirmed | r2:2459–2463, 2519–2521 | |
| 264 | detection, CSP, key storage wait on the baseline | BASELINE-WHEN (b) | confirmed (settled) | STATUS:112–113; r2:3509–3512 | |
| 265 | Q1 rows A–E | r2 1612–1620 | confirmed | r2:1616–1620; D's "251 kbps" is E2's figure (r2:1699, 1780), a fair mapping | |
| 266 | legacy not deterministic (11 wall-clock reads) | STATUS:273 | confirmed | STATUS:273 | |
| 267 | Q1 B unassessed (Overwatch talk video only) | r2 hinge table | confirmed | r2:1617, 3539 | |
| 268 | Q2 rows A–D | r2 1622–1629 | confirmed | r2:1626–1629 | |
| 269 | Q3 rows A–C | r2 1631–1637 | confirmed | r2:1635–1637 | |
| 270 | Q4 rows; page buffer needs what the boundary forbids | r2 1639–1645; R2-XT-OQ2 | confirmed | r2:1643–1645; r2:3333–3336 ("100 to 167 ms delay") | |
| 271 | Q5 rows A–C ("None (Quake III)"; 1.8 m; Source; leak) | r2 1647–1653 | confirmed (research and source) | r2:1651–1653; source S3: Quake III g_weapon.c:183, :465 `trap_Trace (&tr, muzzle, NULL, NULL, end, passent, MASK_SHOT);` with no rewind anywhere in the file | R2-SYNC-F27 was outside R2's checked set; absence read from one file, as R2 says |
| 272 | Q6 rows A–C, E3/E3b | r2 1655–1664; table B 1694–1700 | citation off | E3b is r2:1701, outside "1694–1700"; right range 1694–1702 | finding F6 |
| 273 | Q7 rows; who casts | r2 1666–1671; R2-XT-OQ1; R4-LOS-REC3 | confirmed | r2:1670–1671 (A 32 bits; B "about 60–75"; doc's "at most 80" is REC8 m2, r2:2274) | |
| 274 | Q8 rows T1–T4 | table A 1675–1692 | confirmed | r2:1679–1682 | |
| 275 | T4 settled by TICK-30 | TICK-30 | confirmed (settled) | | |
| 276 | Q9 rows E1–E4 | table B 1694–1700; 1780 | citation off | E3b and E4 rows are r2:1701–1702; right range 1694–1702 (figures correct: 536, 251, 146, 188, "about 25 % below E3") | finding F6 |
| 277 | Q10 rows U1–U3 | table C 1704–1710 | confirmed, with a note | r2:1708–1710 | U1's "the catch-up cap equals the redundancy cap" vs part 5 and r2:2039–2041 "no larger than" (finding F10) |
| 278 | Q11 rows P1–P4 | table D 1712–1719 | confirmed | r2:1716–1719 | |
| 279 | Q12 settled by GL resolution 1 | GL 167–188 | confirmed (settled) | GL:173–174 | |
| 280 | Q13 rows A1–A4 | interest Q-A | confirmed | r2:1800–1803; r2:2442–2443 | |
| 281 | Q14 rows B1–B4 | interest Q-B | confirmed | r2:1809–1812 | |
| 282 | Q15 rows C1–C3 | interest Q-C | confirmed | r2:1818–1820 | |
| 283 | Q16 rows A–D | transport Q1 | confirmed | r2:1841–1844; F29–F31 verifier-confirmed | |
| 284 | Q17 rows A, C; B and D settled | transport Q2 | confirmed | r2:1852–1855 | |
| 285 | Q18 rows A–C; direct calls settled | transport Q4 | confirmed | r2:1868–1875; STATUS:97–98 | |
| 286 | Q19 rows A–C | transport Q6 | confirmed | r2:1889–1896 | |
| 287 | Q20 rows A–C | rooms 3.1 | confirmed | r2:1904–1906 | |
| 288 | Q21 rows A–C | rooms 3.2 | confirmed | r2:1912–1914 | |
| 289 | Q22 rows A–C | rooms 3.3 | confirmed | r2:1923–1930 | |
| 290 | Q23; 6 of 32 ≈ 30 bits | rooms 3.4 | confirmed | r2:1937; 32^6 = 1,073,741,824 = 2^30 | |
| 291 | Q24 rows A–C | rooms 3.5 | confirmed | r2:1947–1952 | |
| 292 | Q25 rows A–C | rooms 3.6 | confirmed | r2:1957–1962 | |
| 293 | Q26; C against VISION_V2 §5 | rooms 3.7 | confirmed | r2:1966–1971; VISION:58 | |
| 294 | 7.49 (6.49) ms left at 30 Hz | arithmetic | confirmed | 16.67 − 9.18; 16.67 − 10.18 | |
| 295 | 60 Hz leaves 0.05 ms on the worst tick | arithmetic | confirmed | as row 71 | |
| 296 | P2 needs 200–250 ms rewind | R2-SYNC-REC5 | confirmed | r2:2157–2158 | |
| 297 | +8 ms average input wait | table A T2 | confirmed | r2:1680 | |
| 298 | E3b +42 kbps | arithmetic | confirmed | | |
| 299 | WT needs a secure context | R2-TRN-F35 | confirmed | r2:1269–1270 "available only in secure contexts (HTTPS)"; verifier-confirmed (r2:3694) | |
| 300 | guest secret in an HttpOnly cookie; id may stay in KVStore successor | R2-ROOM-REC8 | confirmed | r2:2869–2871; source S2 | |
| 301 | constraints change for v2 code under SPIKE-SCOPE (a) | SPIKE-SCOPE | confirmed (settled) | STATUS:187–188 | |
| 302 | GC3 mark now | INV:242 | confirmed | INV:242 mark column | |
| 303 | GC3 proposed split | proposed | confirmed (proposed) | | |
| 304 | GN8 mark now | INV:273 | confirmed | | |
| 305 | GN8 proposed split | proposed | confirmed (proposed) | r2:2525–2526 | |
| 306 | OQ-0001-1 TICK-60-HEADROOM confirmation | advisor C2 | confirmed (settled) | advisor-verdicts:15, 21–26 | |
| 307 | OQ-0001-2 options and recommendation (a) | map J6 (2) | confirmed (proposed, as assigned) | decision-map J6 (2); options labelled "test set-ups, not research options" | not a rule-3 breach: the map asks 0001 to recommend one, the Advisor decides |
| 308 | OQ-0001-3 P2 or 100 ms each way | r2 profiles; R2-XU-OQ2 | confirmed | r2:1987–1990, 3132–3143 | |
| 309 | OQ-0001-4 options; Feel Lab on Molotov and Shield | ABL-PREDICT | confirmed (settled) | r2:3281–3288; STATUS:121–123 | |
| 310 | OQ-0001-5: 200 (default principles; security review) or 250 (sync) | R2-XU-OQ1; R2-SYNC-OQ2 | confirmed | r2:2905–2908, 3129–3131 | |
| 311 | OQ-0001-6 options; client nudging not researched | R2-SYNC-OQ3; R1-TICK-OQ8 | confirmed | r2:2909–2915; r1:2672–2673 | "skipping or repeating a tick" is the draft's, labelled not researched and not proposed |
| 312 | OQ-0001-7 no source read on sub-tick stamps | R1-TICK-OQ4 | confirmed | r1:2654–2656 | |
| 313 | OQ-0001-8 none / 50 ms / latency up to cap | R2-SYNC-OQ6 | confirmed | r2:2925–2927 | |
| 314 | OQ-0001-9 9 m/s, 1.8 m in 200 ms, half the blade's reach | R2-SYNC-OQ9 | confirmed | r2:2933–2935 (blade 3.6 m) | |
| 315 | OQ-0001-10 Quake III's predicted pickups | R2-SYNC-F17 | confirmed | r2:230–231 | |
| 316 | OQ-0001-11 code is in practice the access secret; options | R2-ROOM-OQ10 | confirmed | r2:3107–3111 | |
| 317 | OQ-0001-12 80 m gunshot radar; options | R2-INT-OQ6 | confirmed | r2:3026–3028 | |
| 318 | OQ-0001-13 fallback share, IPv6 share, address change | R2-TRN-OQ7; R2-BUD-OQ11; R2-ROOM-OQ8 | confirmed | r2:3068–3069, 3000–3001, 3101–3103 | |
| 319 | proof-test definition quoted word for word | GL §8 (brief §2) | confirmed | doc:918–924 = DRAFTS_BRIEF:60–66 (diff empty) = map J1 | |
| 320 | throwaway sim: movement, hitscan, bots as protocol clients | R5-TEST-REC5 | confirmed | r5:1475–1476 | the sim's content is the draft's |
| 321 | interop 99/100, draft recorded; swap by diff; floors | R2-TRN-REC1 m1, 3, 5 | confirmed | r2:2586–2596, 2610–2612 | |
| 322 | UDP blocked ≤ 3 s; identical hashes; WS stall p99 ≤ 250 ms at P2; ≤ 1 unsent | R2-TRN-REC3 m1–3 | confirmed | r2:2664–2669 ("100 ms RTT and 1 % loss" restated as P2) | |
| 323 | one build; RTT ±5 ms at P2; cargo tree; 10 M fuzz inputs | R2-TRN-REC4 m1–4 | confirmed | r2:2692–2697 (RTT mark at 100 ms injected RTT, restated at P2) | |
| 324 | maxDatagramSize; lost-or-late ≤ loss + 1 pp; 0 of 10,000 at P3; 1 MB ≤ 5 ms | R2-TRN-REC2 m1–4 | confirmed | r2:2631–2637 | |
| 325 | dev certs 20 of 20; 15-day cert rejected | R2-TRN-REC5 m1, 3 | confirmed | r2:2718–2722 | |
| 326 | decoded snapshots by hash; 500 ms; no oversize datagram | R2-SYNC-REC1 m1–3 | confirmed | r2:2017–2021 | doc text says "marks 1–4"; mark 4 is the budgets row, fine |
| 327 | sync cost linear at 24, 36, 48 | R2-SYNC-REC9 m2 | confirmed | r2:2297–2298 | |
| 328 | join complete within 300 ms at P2 | R2-BUD-REC3 | confirmed | r2:2359 | |
| 329 | 99.9 %; 0 of 10,000; forged; lag switch | R2-SYNC-REC2 m1–4 | confirmed | r2:2058–2066 | |
| 330 | missing inputs ≤ 0.1 % at P3 | R2-BUD-REC1 | confirmed | r2:2323–2324 | |
| 331 | throttle 128 kbps, ≤ 2/s, own state, 2 s | R2-BUD-REC5 | confirmed | r2:2395–2397 | |
| 332 | bottleneck 50 %; ≤ 3 intervals; flat; 5 %; NewReno vs Cubic | R2-TRN-REC6 m1–3 | confirmed | r2:2741–2744 | |
| 333 | flood 1,000 + 10; 10 %; memory; 403 | R2-TRN-REC1 m4 | confirmed | r2:2607–2609 | |
| 334 | handler panic closes one connection | R2-ROOM-REC2 m6 | confirmed | r2:2780–2782 | |
| 335 | 8 rooms; 10 %; killed room; CCX23 and CX33 | R2-ROOM-REC2 m2, 4; R6-HOST-REC2 | confirmed | r2:2777–2780; r6:667–671 | |
| 336 | 60 Hz margin mark | settled | confirmed (settled) | | |
| 337 | Mustafa judges "look correct"; test-engineer writes the checklist | plan rule 12 | confirmed (settled) | plan:142–147 | |
| 338 | correction, latency, replay marks | R2-SYNC-REC3 m1–3; R1-TICK-REC2 P3 | confirmed | r2:2101–2105; r1:2058–2060 | |
| 339 | interpolation marks | R2-SYNC-REC4 m1–4 | confirmed | r2:2128–2131 | |
| 340 | clock offset ≤ 5 ms after 2 s | R2-BUD-REC4 | confirmed | r2:2379–2380 | |
| 341 | hit marks 99/95 %, false < 0.5 %, cap, cheat, 1 MB, 20 µs | R2-SYNC-REC5 m1, 3–6 | confirmed | r2:2179–2192 | |
| 342 | 5 cm in 99.9 % at 0/2.5/4 m; ≤ 80 bits | R2-SYNC-REC8 m1–2 | confirmed | r2:2272–2274 | |
| 343 | no damage or kill before confirmation | R1-ABL-REC4 | confirmed | r1:2444–2446 | |
| 344 | what the proof test decides | named ids | confirmed | each id's "Decided by" names the spike (e.g. r2:3056–3058, 2978–2979, 3080–3081, 2995–2996, 2997–2999, 3142–3143) | |
| 345 | egress per match-hour and the spike's bill reported | R6-COST-REC1 | confirmed | r6:714–717 | |
| 346 | migration step 1 done-when | settled | confirmed (settled) | STATUS:101 | |
| 347 | foundation done-when | GL §8 the foundation | confirmed (settled) | map J1 brief quote "It ends when an empty scene runs through the full stack in the browser, with CI green and a preview link." | |
| 348 | step 3: input-driven, replayable from day one | SLICE-ORDER c2 | confirmed (settled) | STATUS:99–100 | |
| 349 | steps 3–8 milestone marks | cited recs | confirmed | each mark exists (r2:2058–2059, 2101–2105, 2852–2856, 2864–2867, 2758–2761, 2803–2811, 2820–2822, 2837–2841, 2881–2884, 2459–2465, 2519–2521, 2340–2341, 2437–2439; r5:1487–1489) | |
| 350 | preview behind the wall; baseline before the first online playtest | PREVIEW-WALL (a); BASELINE-WHEN (b) | confirmed (settled) | STATUS:106–107, 112–113 | |
| 351 | editor's Test plays the map with bots | EDITOR_V2 §3 | confirmed | EDITOR:41 | |
| 352 | L1 / L2 / TCP proxy | R5-TEST-REC4 | confirmed | r5:1454–1459 | R5 recommendation; rests on R5-TEST-F13 (quinn custom socket), checked in R5's set |
| 353 | 48 per room, 30 min at P2 for every test mode; 3-hour soak | R5-TEST-REC5 | confirmed, with a note | r5:1475–1482; GL test-mode list replaces R5's list (GL:375–391) | "48 per room" cannot hold for 1v1 to 6v6v6 on the list (finding F8) |
| 354 | forged picks/pickups test | proposed | confirmed (proposed) | | |
| 355 | recurring beta-channel run | R2-TRN-OQ1 | confirmed | r2:3039–3040 | |
| 356 | rollback 30 Hz → drafts; 60 Hz → stays 30 | settled | confirmed | doc part 2 | |
| 357 | crate swap touches only the transport module | R2-TRN-REC1 m3 | confirmed | r2:2595–2596 | |
| 358 | option B with unwind and containment | R2-ROOM-REC2 | confirmed | r2:2764–2765 | |
| 359 | Appendix A: 127 ids; 30/7/57/17/16 | arithmetic | confirmed | script: 127 rows, 0 duplicates, same 127 ids as the map's owner-0001 set; adopted 30, changed 7, answered 57, open 17, spike 16 | |
| 360 | Sources line ranges | files | confirmed | spot-checked: r2:1268–1272, 3414, 3532; r1:2967, 3015, 3083; r6:784; r4:1168, 1183; r5:1971, 2016 | |
| TE1 | RS: CCX23, 4 dedicated vCPU, 16 GB, x86-64 AMD, one EU location | R6-HOST-REC2; GL | confirmed | r6:645–647; GL:226–227 | specs rest on R6-HOST-F15 (in R6's checked set) |
| TE2 | a server time mark is elapsed time on one vCPU, no other room | GL measuring unit | confirmed | GL:245–246 | |
| TE3 | tick thread alone on one vCPU, runtime on the other three | proposed | confirmed (proposed, reason given) | doc:1008–1010 | |
| TE4 | SH: CX33 (4 vCPU, 8 GB) | R6-HOST-REC2 | confirmed | r6:647, 667–671 | |
| TE5 | LG: second VM with headless protocol clients | R5-TEST-REC5 | confirmed (proposed placement) | r5:1475–1476 | |
| TE6 | ML, RD, MAC as ADR 0003's pass marks name them | R3-BUDGET-REC1; 0003 PM | confirmed | r3:1096–1102; docs/adr/0003-rendering.md:892–897 | |
| TE7 | ML runs Chrome, Edge, Firefox on Win 11; MAC Safari | VISION_V2 §4; R3-BUDGET-REC1 | confirmed, with a note | 0003:953 (PM-0003-2: "ML: Chrome, Edge and Firefox (Windows)"); r3:1098–1099 names only Chrome and Firefox | Edge and MAC come from ADR 0003's pass marks, not R3-BUDGET-REC1; the doc's "as ADR 0003's pass marks name them" is the right basis |
| TE8 | MP only for what Mustafa judges | plan rule 12 | confirmed (settled) | plan:142–147 | |
| TE9 | L1 on simulated time: logic marks only | R5-TEST-REC4 | confirmed (proposed rule) | r5:1455 "seeded, on simulated time" | |
| TE10 | R2 names Chrome 156, Edge 156, Firefox 158, Safari 27 | R2-TRN-REC1 m1 | confirmed | r2:2586 | |
| TE11 | SS counts | GL stress scene | confirmed | GL:400–409, 417 | "centre field" from r2:2497 |
| TE12 | SS greybox holds enterable buildings with portals | proposed | confirmed (proposed) | doc:288–292 | |
| TE13 | ES: 48 + 15 in view, 32 projectiles, 0.44 spawns/player/s | estimate | confirmed | r2:1761, 1775 (assumption rows) | |
| TE14 | CO course | R2-SYNC-REC3 m1 | confirmed | r2:2101 | |
| TE15 | TC: 9 m/s target; static targets at 0, 2.5, 4 m | R2-SYNC-REC5 m1; REC8 m1 | confirmed | r2:2179, 2272 | |
| TE16 | proof test sim content | draft | confirmed | doc:927–928 | |
| TE17 | 60 Hz sim is OQ-0001-2's pick | draft; map J6 (2) | confirmed | doc:846–855 | |
| TE18 | stand-in rule | proposed | confirmed (proposed) | doc:853–854; doc:1279–1282 (migration step 8) | ledger cites draft lines 1030–1033 (= doc 1279–1282) |
| TE19 | profiles; round trip | r2 profiles | confirmed | r2:1985–1990 | |
| TE20 | L1/L2, TCP proxy | R5-TEST-REC4 | confirmed | r5:1454–1461 | |
| TE21 | "100 ms simulated latency" = P2, Advisor confirms | draft; map J6 (3) | confirmed | doc:396–398, 856–860 | |
| TE22 | 200 ms round trip if read each way | draft OQ-0001-3 (b) | confirmed | doc:858 | |
| TE23 | reorder share recorded; counts only if > 0 | proposed | confirmed (proposed) | r2:1991–1992 "Every spike also covers reordering" | |
| TE24 | S30 60 s warm-up then 30 min | proposed | confirmed (proposed) | r2:2413 | |
| TE25 | 54,000 / 108,000 ticks | arithmetic | confirmed | | |
| TE26 | p99 over measured ticks; max etc. over the whole run | proposed | confirmed (proposed) | | |
| TE27 | counter starts 1,800 below 65,536 | proposed | confirmed (proposed) | doc:240–243, 555 | |
| TE28 | re-flood on 3.3 % / 1.7 % of ticks | arithmetic | confirmed | 1 ÷ 30; 1 ÷ 60 | |
| TE29 | nearest-rank p99: 53,460th; 106,920th | arithmetic | confirmed | | |
| TE30 | no-statistic mark is p99; p99s never added; measured directly | GL time statistics | confirmed | GL:142–146 | |
| TE31 | one-off deadline is a plain limit | GL | confirmed | GL:146–147 | |
| TE32 | a share is a count over the whole sample | proposed | confirmed (proposed) | | |
| TE33 | bandwidth with no statistic read as a mean | proposed | confirmed (proposed) | r2:2339 (mean) | |
| TE34 | kbps = 1,000 bit/s; MB = 10^6 B | proposed | confirmed (proposed) | | |
| TE35 | counters on in every run, as in production | R6-OBS-REC2 | confirmed | r6:741–742 | |
| TE36 | clock resolution measured; sub-µs parts timed per tick | proposed | confirmed (proposed) | | |
| TE37 | CPU shares from per-thread CPU time | GL transport runtime | confirmed (proposed tool) | GL:131–132 | |
| TE38 | performance.now 5 µs isolated, 100 µs otherwise | R3-MEASURE-F7 | confirmed | r3:641–643; R3's verifier confirmed (r3:1587) | |
| TE39 | Worker isolation not researched; read in the Worker | proposed | confirmed (proposed) | | |
| TE40 | L2 times use the clock estimate (PM-0001-47 bound) | R2-BUD-REC4 | confirmed | r2:2379–2380 | |
| TE41 | presented frames, 60 Hz; one tick = two refresh periods | arithmetic | confirmed | 33.3 = 2 × 16.7 | |
| TE42 | 3 runs, each passes | R1-TICK-REC2 P1 | confirmed | r1:2047 "36,000 ticks per run, 3 runs" | |
| TE43 | steal ≤ 1 % | proposed | confirmed (proposed, "not researched" said) | | |
| TE44 | LG's 30 inputs/s within 1 % | proposed | confirmed (proposed) | doc:185 | |
| TE45 | clean reference: median of 3 within 2.5 % | proposed | confirmed (proposed) | r2:2743 (5 %) | |
| TE46 | invalid run neither pass nor fail | R5-TEST-REC6 | confirmed (proposed reading) | r5:1497–1498 | |
| TE47 | planted defect must fail each mark first | R5-GATE-REC3 | confirmed | r5:1728 "a check without a failing case can't be shown to work" | R5's own reasoning; no external source applies |
| TE48 | rule B-S | proposed | confirmed (proposed) | | |
| TE49 | rule B-N | proposed | confirmed (proposed) | | |
| TE50 | no retries; keep data | R5-TEST-REC6 | confirmed | r5:1490, 1497–1498 | |
| TE51 | machines named in the log | proposed | confirmed (proposed) | | |
| TE52 | tick thread affinity one vCPU | GL measuring unit | confirmed | GL:245–246 | |
| TE53 | bite: second room's thread on the vCPU | proposed | confirmed (proposed) | | |
| TE54 | 48 soldiers, three teams of 16, a client each | GL; R2-BUD-REC6 | confirmed | GL:400, 417; r2:2412 | |
| TE55 | 15 big vehicles, 4 very large ≥ 20 parts | GL | confirmed | GL:403, 441 | |
| TE56 | 300 projectiles; ≤ 8 effects | GL | confirmed | GL:404–405, 460 | |
| TE57 | 700 shots/s | R2-SYNC-REC5 m4; REC9 | confirmed | r2:2186–2187, 2280 | |
| TE58 | ±5 % run mean | proposed | confirmed (proposed) | | |
| TE59 | all 47 others in view on ≥ 99 % of snapshots | proposed | confirmed (proposed) | r2:2339 "all in view" | |
| TE60 | stand-in entities at the same counts | proposed | confirmed (proposed) | | |
| TE61 | bite: load script stops firing | proposed | confirmed (proposed) | | |
| TE62 | 100,000 packets within 5 %; identical fate log | R5-TEST-REC4 | confirmed | r5:1469–1471 | |
| TE63 | P0 on L2 is the real path | proposed | confirmed (proposed) | | |
| TE64 | MP's RTT within 100 ± 5 ms | proposed | confirmed (proposed) | r2:2694 (±5 ms) | |
| TE65 | injected delay = profile RTT − base RTT | proposed | confirmed (proposed) | | |
| TE66 | 30 Hz p99 ≤ 16.7, max < 33.3, 0 overruns | GL; R2-BUD-REC6 | confirmed | GL:168–170; r2:2413–2414 | |
| TE67 | 0 dropped time | R2-BUD-REC6; R1-TICK-REC4 | confirmed | r2:2414 "0 missed ticks"; r1:2083–2084 | |
| TE68 | whole server tick timed directly | GL server tick time | confirmed, with a rule note | GL:114–115 | the doc's wording "the whole tick timed directly" (doc:1097) uses a retired exact phrase (finding F1) |
| TE69 | R2-ROOM-REC2 m1's 20 min inside S30 | R2-ROOM-REC2 | confirmed | r2:2775–2776 | |
| TE70 | shares ≤ 8.3 ms, own series | GL; R2-BUD-REC6; R2-SYNC-REC9 | confirmed | GL:190; r2:2401; r2:2292–2296 | |
| TE71 | sim step = step minus lag-comp queries | proposed | confirmed (proposed) | doc:145–149, 429–432 | |
| TE72 | sim step parts are ADR 0004's | map | confirmed (settled) | | |
| TE73 | encode ≤ 50 µs; 2.40 ms | R2-BUD-REC6 | confirmed | r2:2414–2415 | |
| TE74 | 1,440 samples/s | arithmetic | confirmed | | |
| TE75 | rewound shot ≤ 20 µs; 0.47 ms | R2-SYNC-REC5 m4 | confirmed | r2:2186–2187 | |
| TE76 | history ≤ 1 MB | R2-SYNC-REC5 m3 | confirmed | r2:2184–2185 | |
| TE77 | bite: untrimmed history | proposed | confirmed (proposed) | | |
| TE78 | region query ≤ 0.2 ms | R2-INT-REC1 (c) | confirmed | r2:2440–2441 | |
| TE79 | rank/pack ≤ 0.5 ms; budget; nearest 8; every 4 | R2-INT-REC4 | confirmed | r2:2497–2500 | |
| TE80 | 1,067 B per interval | arithmetic | confirmed | | |
| TE81 | centre-field fight is R2-INT-REC4's scene | R2-INT-REC4 | confirmed | r2:2497 | |
| TE82 | bite: rank by id | proposed | confirmed (proposed) | | |
| TE83 | pair test 0.1 µs; 768; 0.08 ms | R2-INT-REC2 (b) | confirmed | r2:2463 | |
| TE84 | re-flood ≤ 1 ms; area count | R2-INT-REC2 (b) | confirmed | r2:2464 | |
| TE85 | pair test timed per tick ÷ pairs | proposed | confirmed (proposed) | | |
| TE86 | cost per unit of work at 48 ≤ 1.1 × at 24, 36 | R2-SYNC-REC9 m2 | confirmed (tolerance proposed) | r2:2297–2298 | |
| TE87 | N = 24, 36 as three teams of 8, 12 | GL scale steps | confirmed | GL:401–402 | |
| TE88 | bite: N² encoder | proposed | confirmed (proposed) | | |
| TE89 | runtime ≤ 1 vCPU; send p99 ≤ 1 ms | R2-TRN-REC1 m2 | confirmed | r2:2589–2592 | |
| TE90 | 30 input datagrams/s | draft | confirmed | doc:532; r2:2590 "placeholder numbers" | |
| TE91 | "≤ 1 vCPU" as p99 of 1-s windows | proposed | confirmed (proposed) | | |
| TE92 | downstream mean ≤ 160, p99 ≤ 256 | R2-BUD-REC2 | confirmed | r2:2339–2340 | |
| TE93 | every 1-s window ≤ 256 (hard cap) | proposed | confirmed (proposed) | | |
| TE94 | rows run in SS | draft | confirmed | doc:521–523 | see note N12 |
| TE95 | ES reported, 10 min | R2-BUD-REC2 | confirmed | r2:2339 "10-minute bot match" | |
| TE96 | IPv6 +4.8 kbps | estimate | confirmed | r2:1784 | |
| TE97 | upstream mean ≤ 48 kbps | R2-BUD-REC1 | confirmed | r2:2324 | |
| TE98 | restated against P2 | draft | confirmed | doc:394–396 | |
| TE99 | mean gates; p99, largest datagram, command size reported | proposed | confirmed (proposed) | r2:3151–3157 | |
| TE100 | 0 datagrams above cap and reported max | R2-SYNC-REC1 m3; R2-BUD-REC3 | confirmed | r2:2021, 2358–2359 | |
| TE101 | cap rule; 1,000 B until measured | R2-BUD-REC3; draft | confirmed | r2:2345–2346; doc:224–226 | |
| TE102 | 0 fragmented IP packets | R2-BUD-REC3 | confirmed | r2:2346 | |
| TE103 | RSS ≤ 64 MB at start and 20 min | R2-ROOM-REC2 m3 | confirmed | r2:2778–2779 | |
| TE104 | 1-s sampled peak ≤ 64 MB | proposed | confirmed (proposed) | | |
| TE105 | slope ≤ 1 MB / 10 min | R5-TEST-REC5 | confirmed | r5:1487–1488 | |
| TE106 | queues flat (last 10 min ≤ first 10 min) | proposed | confirmed (proposed) | r2:2742 | |
| TE107 | planted leak flagged | R5-TEST-REC5 | confirmed | r5:1488–1489 | |
| TE108 | start < 2 s, every start, ≥ 20 | R2-ROOM-REC2 m5 | confirmed | r2:2780 | |
| TE109 | 100 % decoded snapshots by hash under P0, P2, P3, B1 | R2-SYNC-REC1 m1 | confirmed | r2:2017–2019 | |
| TE110 | also across the wrap | proposed | confirmed (proposed) | | |
| TE111 | L1 3 seeds; L2 1 run | proposed | confirmed (proposed) | | |
| TE112 | bites: 16-bit header tick; quantiser mismatch | proposed | confirmed (proposed) | | |
| TE113 | B1 recovery 500 ms; 10 bursts | R2-SYNC-REC1 m2 | confirmed | r2:2020 | |
| TE114 | join within 300 ms at P2 | R2-BUD-REC3 | confirmed | r2:2359 | |
| TE115 | complete world = hash equal | proposed | confirmed (proposed) | GL:549–551 | |
| TE116 | 100 joins incl. rejoins after an S20 trap | proposed; 0004 S20 | confirmed | docs/adr/0004-simulation-core.md:842 (S20 exists) | |
| TE117 | ≥ 99.9 % before their tick at P3 | R2-SYNC-REC2 m1 | confirmed | r2:2058 | |
| TE118 | 10,000 presses: 0 lost, 0 doubled | R2-SYNC-REC2 m2 | confirmed | r2:2059 | |
| TE119 | missing inputs ≤ 0.1 % at P3 | R2-BUD-REC1 | confirmed | r2:2323–2324 | |
| TE120 | bite: redundancy cap 1 | proposed | confirmed (proposed) | | |
| TE121 | forged kinds; lag switch | R2-SYNC-REC2 m3–4 | confirmed | r2:2060–2066 | |
| TE122 | "0 effect" via state hash | proposed | confirmed (proposed) | | |
| TE123 | counts 1,000 / 100 | proposed | confirmed (proposed) | | |
| TE124 | never-acking client | R2-BUD-REC2 | confirmed | r2:2342–2344 | |
| TE125 | throttle marks | R2-BUD-REC5 | confirmed | r2:2395–2397 | |
| TE126 | 10 throttles | proposed | confirmed (proposed) | | |
| TE127 | bottleneck marks; 3 × 33.3 = 100 ms | R2-TRN-REC6 m1–3 | confirmed | r2:2741–2744 | |
| TE128 | flood marks | R2-TRN-REC1 m4 | confirmed | r2:2607–2609 | |
| TE129 | same suite over WS; ws:// refused | R2-TRN-REC3 m4 | confirmed | r2:2670–2671 | ledger cites draft test plan 1045–1046 (= doc 1294–1296) |
| TE130 | no Join after 2 s → closed | R2-TRN-REC1 m4 | confirmed | r2:2603 | |
| TE131 | panic closes one connection; 0 overruns next minute | R2-ROOM-REC2 m6 | confirmed | r2:2780–2782 | |
| TE132 | 10 injections; bite panic=abort | proposed | confirmed (proposed) | doc:348 | |
| TE133 | 8 rooms; 10 %; 20 min; SIGKILL | R2-ROOM-REC2 m2, 4; R6-HOST-REC2 | confirmed | r2:2777–2780; r6:667–668 | |
| TE134 | "unchanged" = within 5 % | R2-TRN-REC6 m2 | confirmed | r2:2743 | |
| TE135 | each room with snapshot-receiving clients | proposed | confirmed (proposed) | GL:416–417 | |
| TE136 | SH at 3 times of day | R6-HOST-REC2 | confirmed | r6:668 "repeated at 3 times of day" | |
| TE137 | each crate: 99 of 100 per browser; draft recorded | R2-TRN-REC1 m1 | confirmed | r2:2586–2588 | |
| TE138 | server on the same local network | proposed | confirmed (proposed) | r2:2588 "clean local network" | |
| TE139 | control: wrong certificate hash | proposed | confirmed (proposed) | | |
| TE140 | UDP blocked ≤ 3 s; timeout 1.5 s; 20 attempts | R2-TRN-REC3 m1 | confirmed | r2:2664–2665 | |
| TE141 | identical server hashes over both transports | R2-TRN-REC3 m2 | confirmed | r2:2666–2667 | |
| TE142 | 3 recordings of 5 min | proposed | confirmed (proposed) | | |
| TE143 | WS stall p99 ≤ 250 ms at P2; ≤ 1 unsent | R2-TRN-REC3 m3 | confirmed | r2:2668–2669 | |
| TE144 | extra delay minus 50 ms one-way | proposed | confirmed (proposed) | 100 ÷ 2 | |
| TE145 | one build, four browsers, both backends | R2-TRN-REC4 m1 | confirmed | r2:2692–2693 | |
| TE146 | "runs unchanged" = 60 s, 0 errors | proposed | confirmed (proposed) | 0003:953 same reading | |
| TE147 | RTT ±5 ms at P2; also fixed 100 ms | R2-TRN-REC4 m2; R5-TEST-REC4 | confirmed | r2:2694; r5:1471–1472 | |
| TE148 | p99 of the absolute difference | proposed | confirmed (proposed) | | |
| TE149 | bite: pongs from the tick thread | proposed | confirmed (proposed) | | |
| TE150 | fuzz 10 M per type | R2-TRN-REC4 m4 | confirmed | r2:2696–2697 | |
| TE151 | swap diff; cargo tree; version floors | R2-TRN-REC1 m3, 5; REC4 m3 | confirmed (research and source) | r2:2595–2596, 2610–2612, 2695; source S1 (rustsec.org, all four floors) | |
| TE152 | bite: quinn in the sim crate (G5) | R5-GATE-REC3 | confirmed | r5:1717 | |
| TE153 | maxDatagramSize ≥ 1,000 B, every 10 s | R2-TRN-REC2 m1 | confirmed | r2:2631–2632 | |
| TE154 | EU consumer connection | proposed | confirmed (proposed) | | |
| TE155 | lost or late ≤ 3 % at P2 | R2-TRN-REC2 m2 | confirmed | r2:2633–2634; 2 + 1 | |
| TE156 | 0 of 10,000 at 5 %; 1 MB ≤ 5 ms | R2-TRN-REC2 m3–4 | confirmed | r2:2635–2637 | |
| TE157 | bulk mark as p99 difference | proposed | confirmed (proposed) | | |
| TE158 | p99 ≤ 6.25 vs 8.33; 25 % | settled | confirmed (settled) | advisor C2; map J1 | |
| TE159 | shares ≤ 4.2; max < 16.7; 0 overruns; encode ≤ 50 µs | R2-BUD-REC6; R1-TICK-REC2 P1; GL | confirmed | r2:2413–2415; r1:2047–2049; GL:190 | |
| TE160 | margin setup | settled | confirmed (settled) | map J1 N3 | |
| TE161 | P2 in the margin run | proposed | confirmed (proposed) | r2:2292–2293 | |
| TE162 | tick 65,536 at 18.2 min | arithmetic | confirmed | 65,536 ÷ 60 ÷ 60 = 18.2 | |
| TE163 | "in one run": 3 runs, each self-contained | proposed | confirmed (proposed) | | |
| TE164 | a failure keeps 30 Hz | draft | confirmed | doc:966–967, 1307 | |
| TE165 | P1 at 60 Hz | R1-TICK-REC2 P1 | confirmed | r1:2044–2049 | |
| TE166 | P2 at 60 Hz: ≤ 6.7 ms on ML, 30+ fps on Low | R1-TICK-REC2 P2 | confirmed | r1:2054–2057; GL:215–216 | |
| TE167 | P3 at 60 Hz: 12 ticks ≤ 1 ms | R1-TICK-REC2 P3 | confirmed | r1:2058–2060 | |
| TE168 | 60 Hz needs all three and PM-0001-39 | settled | confirmed (settled) | | |
| TE169 | 0 corrections > 1 mm in time | R2-SYNC-REC3 m1 | confirmed | r2:2101–2102 | |
| TE170 | Worker counts corrections | draft | confirmed | doc:246–248 | |
| TE171 | bites | proposed | confirmed (proposed) | | |
| TE172 | P3 brawl: > 5 cm in < 1 %; ≤ 5 cm/frame | R2-SYNC-REC3 m2 | confirmed | r2:2103–2104 | |
| TE173 | 10 min; bite smoothing off | proposed | confirmed (proposed) | | |
| TE174 | input-to-screen same at P0 and P4 within one tick | R2-SYNC-REC3 m3 | confirmed | r2:2105 | |
| TE175 | on p99 and median | proposed | confirmed (proposed) | | |
| TE176 | 200 inputs via CDP | proposed | confirmed (proposed, "not researched") | | |
| TE177 | ML and RD, Chrome on WebGPU | proposed | confirmed (proposed) | r3:1096–1102 | |
| TE178 | 6-tick replay ≤ 1 ms p99 on ML | R1-TICK-REC2 P3 | confirmed | r1:2058–2060 | |
| TE179 | Chrome and Firefox | proposed | confirmed (proposed) | | |
| TE180 | replay forced every snapshot | proposed | confirmed (proposed) | | |
| TE181 | 18,000 replays; 17,820th | arithmetic | confirmed | | |
| TE182 | vehicle case only with a piloted vehicle | proposed | confirmed (proposed) | r1:2058 | |
| TE183 | extrapolation < 1 % / < 3 %; B1 ≤ 1 interval | R2-SYNC-REC4 m1–2 | confirmed | r2:2128–2129 | |
| TE184 | Worker flags extrapolated blocks | proposed | confirmed (proposed) | | |
| TE185 | bite: G11 | R5-GATE-REC3 | confirmed | r5:1722 | |
| TE186 | settles in 5 s; 0 slides in 1,000 | R2-SYNC-REC4 m3–4 | confirmed | r2:2130–2131 | |
| TE187 | "settled" defined | proposed | confirmed (proposed) | | |
| TE188 | "slide" defined | proposed | confirmed (proposed) | | |
| TE189 | delay 2–5 intervals; ≤ 1 interval/s | draft; R2-SYNC-REC5 | confirmed | r2:2111, 2153 | |
| TE190 | 20 changes each way | proposed | confirmed (proposed) | | |
| TE191 | clock ≤ 5 ms after 2 s at P2 | R2-BUD-REC4 | confirmed | r2:2379–2380 | |
| TE192 | p99 over 1,000 connects | proposed | confirmed (proposed) | | |
| TE193 | 99 % / 95 %; false < 0.5 %; 9 m/s | R2-SYNC-REC5 m1 | confirmed | r2:2179–2181 | |
| TE194 | shooter's view = state published for the view time | proposed | confirmed (proposed) | doc:202–205 | |
| TE195 | 2,000 shots ≈ 20 misses | arithmetic | confirmed | | |
| TE196 | 0.9 m at 9 m/s over 100 ms | arithmetic | confirmed | | |
| TE197 | decides OQ-0001-7 | draft | confirmed | doc:884 | |
| TE198 | 100 % within the cap | R2-SYNC-REC5 m6 | confirmed | r2:2191–2192 | |
| TE199 | clipped share per cap recorded | R2-BUD-REC1 | confirmed | r2:2320–2323 | |
| TE200 | rewind estimate measured | draft | confirmed | doc:989–990; r2:3142–3143 | |
| TE201 | bite: cap on the estimate only | R2-SYNC-REC5 | confirmed | r2:2154–2157 | |
| TE202 | cheat gains ≤ tolerance (33.3 ms), P0/P1, 1,000 | R2-SYNC-REC5 m5 | confirmed | r2:2188–2190 | |
| TE203 | 5 cm in 99.9 %; ≤ 80 bits | R2-SYNC-REC8 m1–2 | confirmed | r2:2272–2274 | |
| TE204 | 1,000 shots, one miss allowed | arithmetic | confirmed | | |
| TE205 | bite: parallel ray, 10 cm origin error | R2-SYNC-REC8 | confirmed | r2:2265–2267 | |
| TE206 | no damage/kill before the event | R1-ABL-REC4 | confirmed | r1:2444–2446 | |
| TE207 | 1,000 scripted hits | proposed | confirmed (proposed) | | |
| TE208 | Mustafa's verdict | plan rule 12 | confirmed (settled) | | |
| TE209 | 0 desyncs over 1,000 throws, else E3b | R2-BUD-REC2 | confirmed | r2:2342 | |
| TE210 | bite: 1/60 vs 1/30 step | proposed | confirmed (proposed) | | |
| TE211 | soldier beside a moving vehicle: > 5 cm in < 2 % | R2-SYNC-REC7 m3 | confirmed | r2:2244–2245 | |
| TE212 | records list | draft | confirmed | doc:985–993 | |
| TE213 | crate pick: no rule beyond marks | draft; R2-TRN-REC1 | confirmed | doc:308–309; r2:2565–2566 | |
| TE214 | smoothness measure | R1-TICK-REC5 | confirmed | r1:2111–2112 | |
| TE215 | OQ-0001-6 both options with PM-42 and PM-48 | proposed | confirmed (proposed) | doc:874–880 | |
| TE216 | conditional marks; PM-39/40 per OQ-0001-2 | proposed | confirmed (proposed) | doc:850–851 | |
| TE217 | rollback routes | draft Rollback | citation off (in the doc) | doc:1160 "by the draft's own Rollback (lines 1056–1068)"; the Rollback is doc:1305–1317 (1056–1068 are lines of the unassembled draft; doc:1001 says "the draft" is this record) | routes themselves match doc:1306–1315 (finding F2) |
| TE218 | data retry once | proposed | confirmed (proposed) | doc:1314–1315 | |
| TE219 | a mark's number changes only in the draft | R5-GATE-REC4 | confirmed (R5's rule applied to spikes, labelled) | r5:1734–1737 | |
| TE220 | part 1 has no number; no client field for state, hits, RTT | proposed | confirmed | doc:110–114, 476–495 | |
| TE221 | hiding at slice 3; pop-in at slice 5 | draft | confirmed | doc:1266–1278 | |
| TE222 | loopback is the foundation's; dev certs ADR 0006's | map | confirmed (settled) | doc:1254–1258 | |
| TE223 | lobby etc. at slices 2–3; test tokens | draft; proposed | confirmed | doc:1263–1270 | |
| TE224 | part 12 at slice 4 | draft | confirmed | doc:1271–1274 | |
| TE225 | OQs to Feel Lab, design, telemetry | draft | confirmed | doc:861–913 | |
| TE226 | ADR 0006's operations marks; ADR 0004's S17 | drafts | confirmed | docs/adr/0004-simulation-core.md:839 (S17); 0006 not re-read | |
| TE227 | 47 bots + Mustafa in the 48th slot | proposed | confirmed (proposed) | doc:361–363 | |
| TE228 | profile and cap switched from the overlay | proposed | confirmed (proposed) | | |
| TE229 | step 1: playing within 3 s | R2-TRN-REC3 m1 | confirmed | r2:2664–2665 | |
| TE230 | steps 2–3: same feel at P0 and P2 | R2-SYNC-REC3 m3 | confirmed | r2:2105 | |
| TE231 | step 4: glide (100 ms) unless > 2 m | draft | confirmed | doc:172–173 | |
| TE232 | step 5: Hermite on artefacts | draft | confirmed | doc:258–259 | |
| TE233 | step 6: respawns placed | R2-SYNC-REC4 | confirmed | r2:2116 | |
| TE234 | step 7: LOOP_V2 §7 quote | LOOP_V2 §7 | confirmed | LOOP:92 | |
| TE235 | step 8: no damage/kill before the marker | R1-ABL-REC4 | confirmed | r1:2444–2446 | |
| TE236 | step 9: lands where clicked | R2-SYNC-REC8 | confirmed | r2:2271–2274 | |
| TE237 | steps 10–11 feed OQ-0001-5 | draft | confirmed | doc:867–873 | |
| TE238 | step 12: 8 ticks = 267 ms; one correction | arithmetic | confirmed | 8 × 33.33 = 266.7 | |
| TE239 | step 12: world catches up within 0.5 s | R2-SYNC-REC1 m2 | confirmed | r2:2020 | |
| TE240 | step 13: P3/P4 test graceful degradation | r2:1991 | confirmed | r2:1991 | |
| TE241 | step 14: stalls on loss over WS | draft | confirmed | doc:815–816 | |
| TE242 | step 15: three verdicts | GL §8 | confirmed (settled) | doc:923–924 | |
| TE243 | spike name, owner 0001 | map | not in this document | the spike entry goes to SPIKE_PLAN (grep: no "Size" or "384" in the doc) | checked in the synthesis unit |
| TE244 | runs first, parallel with other spikes | SLICE-ORDER; map | not in this document | as TE243 | |
| TE245 | RS/SH orderable; LG for 384 clients | 0006 OQ; arithmetic | not in this document | 8 × 48 = 384 (arithmetic correct) | |
| TE246 | 60 Hz run under (a) needs the sim core spike first | draft | not in this document as a spike-entry row | the substance is in doc:849–852 | |
| TE247 | size L | proposed | not in this document | | |
| TE248 | subsumes part 14's research spikes | draft; map | not in this document as a row | substance in doc:400–406 | |
| TE249 | decides list | draft App. A | not in this document as a row | substance in doc:985–993 and App. A | |
