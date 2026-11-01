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
| 18 | LOOK_V2 §3 interiors, §4 enterable ruins, roofs fade [DEFAULT] | LOOK_V2 §3–§4 | confirmed | LOOK:31, 36–41 | [DEFAULT] belongs to the roof fade only (ruins are [DECIDED]); wording could misread |
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
