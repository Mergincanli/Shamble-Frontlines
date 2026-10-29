Task: v2-research consistency critic (engine-architect, xhigh): list contradictions, hidden dependencies, gaps and wrong cross-references across R1-R6 (docs/research/r1..r6); resolve none.

## Progress log
- 2026-10-29 start: notes file created, no prior notes.

## Register R1 (r1-simulation-core.md) -- in progress
- TL;DR r1:57-62: plain-data sim struct, sim-owned ids; one integer tick, 60 Hz spiked beside 30 Hz; libm, ChaCha8, per-tick state hashes, no lockstep; one Worker, one transferred block per tick; reduced GAS; JSON via one Rust content crate; hot reload dev only.
- Versions cited: bevy_ecs 0.20.0, hecs 0.11.2, std 1.99.1, slotmap 1.1.1 (r1:79,156,196); Rapier 0.36 (r1:228), "0.36.1 now" (r1:1896).
- ECS-REC1 r1:1905-1925: ref hw = "one server core of the R6 host type", VISION min laptop for Wasm. Stress load: 48 soldiers, 8 vehicles, 300 projectiles, 200 effects; mean tick <=2 ms, p99 <=4 ms native; Wasm <=2x native. Snapshot <=0.2 ms native/<=0.5 ms Wasm. Restore+replay k=ceil(0.15 s x tick) <= 8 ms Wasm min laptop; else R2 partial prediction (R1-ECS-OQ3). Native rebuild <=10 s.
- ECS-REC2 r1:1943: CI check zero HashMap iteration in sim (tooling R5's).
- ECS-REC4 r1:1965: "If R4 picks Rapier" -> sets in sim state, snapshot byte for byte.
- TICK-REC1 r1:1979: u32 tick counter; durations in seconds converted at load; tick not a server option.
- TICK-REC2 r1:1999-2026: start 60 Hz, 30 Hz beside; P1: N=12..64 soldiers + 4 big vehicles + 200 projectiles; 60 Hz p99 <=4.2 ms (25% of 16.7), 30 Hz p99 <=8.3 ms; ref hw "to be named with R3, R5 and R6". P2 Worker p99 <=40% period. P3 replay 200 ms (12 ticks at 60) <=1 ms p99. Decision rule 60 if pass else 30 (R2 sub-tick timestamps OQ4).
  NOTE intra-R1: ECS-REC1 stress = 8 vehicles/300 proj/200 effects; TICK-REC2 = 4 vehicles/200 proj; ECS replay window 0.15 s vs TICK P3 200 ms; ECS p99 <=4 ms vs TICK 4.2 ms. (check whether R1-XU-OQ lists these)
- TICK-REC4 r1:2039-2053: catch-up <=0.25 s (15 ticks at 60 Hz); snapshots after tick at R2's rate (Q3 up to 30/s); overrun metrics -> R6 monitoring. tick start jitter p99 <=2 ms.
- TICK-REC5 r1:2055: Worker owns tick; setTimeout accumulator; main thread interpolates "read from shared memory" vs BND-REC2 transferred block (R1-XU-OQ3).
- DET-REC1 r1:2088: same-build reproducibility required; Wasm/native identity target, netcode must not depend on it; no lockstep. 24v24 and 12v12v12 bot matches.
- DET-REC2 r1:2102: one float type f32/f64 (OQ3); libm pinned; simd128 off until OQ2; "native AArch64 if R6 picks it"; lint tool by R5.
- DET-REC3 r1:2125: Rapier enhanced-determinism both targets, never simd8; "if R4 keeps Rapier"; vehicle count set by R4.
- DET-REC4 r1:2137: ChaCha8Rng pinned, streams; Pcg32 fallback open (DET-OQ11).
- DET-REC6 r1:2167: recorded matches + per-tick hash on native x2, Wasm Node, Wasm browsers (R5 picks tool).
- BND-REC1 r1:2196: one dedicated module Worker, single-threaded, stable Rust; <=4 Wasm calls/tick; page frame p95 +<=1.0 ms; N=12..48.
- BND-REC2 r1:2208: one transferred ArrayBuffer per tick (B2), not shared Wasm memory; B3 SAB ring alternative; event section (hits, deaths, purchases, captures, moments).
- BND-REC3 r1:2227: one command per tick page->worker; seq, tick, move, aim yaw/pitch, bitmask, edge counters; "exact fields are R2's".
- BND-REC4 r1:2237: wasm-bindgen .d.ts + own layout generator + ts-rs; not tsify; pin wasm-bindgen CLI to lib.
- BND-REC5 r1:2248: NO cross-origin isolation for start (G1); allowed in dev/previews (G3); prod only if B2/B3 test passes; COOP same-origin + COEP require-corp if adopted.
- BND-REC6 r1:2262: Worker owns network connection (H2); same sim offline.
- ABL-REC1..6 r1:2277-2405: reduced GAS subset; weapons own bookkeeping + shared damage pipeline (B1); integer time unit where tick exact (C1); predict only local player's own abilities/weapons, never damage (D2), prediction key = command seq; one table per kind (E1); ability/effect/weapon step <=1 ms/tick at 48 soldiers x 8 effects, server core AND min laptop Wasm; "method R5 sets".
- DATA-REC1 r1:2409: JSON one format, one definition per file, serde_json float_roundtrip. REC2: one Rust content crate (garde, deny_unknown_fields). REC3: schemars JSON Schema 2020-12 drives editor; Ajv optional; CI schema diff. REC5: hot reload dev only, notify server, Vite plugin client; production pins content per match. REC6: hashed immutable bundle, SHA-256 in worker; server names hash at join; match record stores content hash + mode id (R6 telemetry). REC7: server validates private-match settings / map uploads.
- OQs of note: ECS-OQ3 (whole-world vs local replay -> R2), ECS-OQ4 (parallelism in match vs one match per core -> R6 cost), TICK-OQ1 (60 vs 64 Hz), TICK-OQ3 (shipped refs' tick rates -> "R2's reading ... and the consistency critic aligning both topics"), TICK-OQ4 (sub-tick timestamps -> R2 lag comp), TICK-OQ6 (ref hardware: R3,R5,R6 proposals, Mustafa), TICK-OQ7 (offline largest mode), TICK-OQ8 (client tick nudge -> R2 time sync), TICK-OQ9 (Rapier substeps at 30 Hz -> R4), DET-OQ3 (f32/f64 -> R4 with map size), DET-OQ6 (how much client predicts -> R2), DET-OQ7 (server CPU arch -> R6), DET-OQ9 (hash fn + prod hash compare -> R2), BND-OQ5 (third-party embeds -> Mustafa with R6), BND-OQ6 (block size; R2 snapshot contents, R3 instancing), BND-OQ8 (server snapshot layout = page block? -> R2), ABL-OQ4 (prediction model -> R2), ABL-OQ8 (tag visibility -> R2 interest), DATA-OQ3 (R2 may hold shipped source for mid-match tuning), DATA-OQ7 (R4 nav rebuild cost), DATA-OQ9 (SHA-256 crate -> rust-reviewer), DATA-OQ11 (editor file:// + Wasm -> ADR-0002 and R5 build research).
- XU-OQ1 stress scenes differ (intra-R1, already open); XU-OQ2 replay; XU-OQ3 shared mem vs transfer; XU-OQ4 W3 pacing; XU-OQ7 fire rate repr; XU-OQ8 content TS types ts-rs vs schemars (ADR-0005 with R5); XU-OQ9; XU-OQ10 tick on hot reload; XU-OQ11; XU-OQ12.
- RR-OQ1 (baseline x86-64, no +fma; AArch64 before R6 picks ARM) ; RR-OQ4 overflow-checks (ADR-0002 with R5); RR-OQ7 tokio 1.54.0 RUSTSEC-2026-0384, cargo-audit/deny (R5 CI).
- Pointers to R2 r1:2920-2934: send rate separate; 16-bit tick wraps 18.2 min at 60 Hz; R1-TICK-OQ3/OQ4/OQ8; hash compare telemetry DET-OQ9; content hash in join handshake.
- Pointers to R3 r1:2935: interpolation on 60/120/144 Hz; OffscreenCanvas option A3; per-entity/per-field layout for instancing (BND-OQ6).
- Pointers to R4 r1:2940: physics choice; spatial index for shots/LOS inside snapshot or rebuilt (R4 decides); Rapier dt = tick or fraction (TICK-OQ9); swept tests vs CCD; f32/f64; nav server only?; LOS for blasts NC-7; map/nav reload mid-match (DATA-OQ7).
- Pointers to R5 r1:2947: CI HashMap check, counting allocator, Rust+Worker tick bench in CI, DET-REC6 suite on Node+browsers, lint for DET-REC2 bans, Vite worker/Wasm, server.headers COOP/COEP in dev (G3), generated-type diff + wasm-bindgen pin, Playwright 4 browsers, schema diff, Vite plugin, golden migration fixtures, DATA-OQ11.
- Pointers to R6 r1:2955: cost per match one-per-core (ECS-OQ4); overrun metrics; matches/core 30 vs 60 Hz; server CPU arch (DET-OQ7); preview+prod hosting can set COOP/COEP/CORP; embeds/login under isolation; events carry source ability/weapon id; match records store content hash + mode id; immutable hosting of content bundles.
- R1 register DONE (2026-10-29). Next: R2 register.

## Register R2 (r2-netcode.md) -- in progress
- TL;DR r2:69-70: 30 Hz delta snapshots as datagrams, 60 Hz tick if R1's CPU mark holds, own prediction, capped rewind, camera relevance, quinn + WebSocket, one process per room, signed join tokens.
- Network profiles P0-P4, B1 r2:1983 (P2 = 100 ms RTT +-20 ms 2%). NB BUD-REC1 spike uses 100 ms RTT +-10 ms 2% (r2:2301) - different from P2's +-20.
- SYNC-REC1 r2:1990: delta vs last acked; ring 32 snapshots; datagrams; reliable events on streams; mark 4 snapshot work "within the share of the tick budget the tick-rate unit sets".
- SYNC-REC2 r2:2019: command: tick, seq, move, buttons, press counters, aim (REC8 world aim point), view time; redundancy up to cap 8 unacked commands; catch-up cap <=8.
- SYNC-REC3 r2:2061: predict own movement, own weapon state, own vehicle, cosmetic shot feedback; replay from authoritative state; error decay 100 ms, snap 2 m; spread from server-only RNG stream (H2) - client must NOT know spread seed.
- SYNC-REC4 r2:2097: interp delay 3 snapshot intervals, bounds 2-5 intervals; clock rules.
- SYNC-REC5 r2:2122: rewind cap 250 ms proposed (XU-OQ1, security prefers 200); history ~1 s; ~0.1 MB at 60 Hz for 60 movers; view-time tolerance <= one snapshot interval; history 48 soldiers + 12 vehicles+harvesters <=1 MB; rewound shot <=20 us p99 at 700 shots/s; "Detection itself belongs to R6's anti-cheat basics, deferred".
- SYNC-REC6 r2:2182: projectiles = trajectory records, one shared Rust trajectory fn; swept tests at present (legacy GC15); bit-identical Wasm/native for 10,000 throws.
- SYNC-REC8 r2:2233: quantised world aim point; "Line of sight across floors is the server's own height-aware query (R4)". aim <=80 bits.
- SYNC-REC9 r2:2261: snapshot enc + lag comp + commands together <=25% of tick budget p99 on proposed ref server.
- BUD-REC1 r2:2284: snapshots 30 Hz; tick 60 Hz if R1 CPU mark, else 30; inputs sent every tick with 3 previous repeated (4 copies). [vs SYNC-REC2 cap 8 unacked] upstream <=48 kbps/client.
- BUD-REC2 r2:2307: quantize 1 cm 16/16/12 bits, 16-bit yaw, 10-bit pitch; per-client ring ~32 (~1 s at 30 Hz); "projectiles as spawn events with client-side deterministic flight plus correction events"; mean <=160 kbps, p99 1-s <=256 kbps; Octopon-3 greybox <=100 kbps.
- BUD-REC3 r2:2325: datagram <= min(maxDatagramSize, 1150 B payload); full snapshot 48p ~1021 B, 64p ~1257 B; join snapshot over reliable stream.
- BUD-REC4 r2:2341: interp delay 100 ms default adapting up to 150 ms [vs SYNC-REC4 up to 5 intervals = 167 ms at 30 Hz]; rewind cap 200 ms [vs SYNC-REC5 250]; 1 s history; input buffer 1-3 ticks.
- BUD-REC5 r2:2360: under pressure fall back to 20 Hz snapshots.
- BUD-REC6 r2:2377: WHOLE tick (sim + lag comp + encode) p99 <=25% (<=4 ms at 60 Hz, <=8 ms at 30 Hz); ref server "one vCPU of the EU instance type R6 recommends"; encode <=50 us/client/snapshot.
  !! vs R1-TICK-REC2 P1 (r1:2014-2018): SIM ALONE p99 <=4.2 ms at 60 Hz (25%), "The 25 % leaves room for snapshot encoding, interest management and several matches per core (R2, R6)". And SYNC-REC9 gives sync alone 25%. -> three different meanings of "25%". Also ref hw "one server core" (R1) vs "one vCPU" (R2).
- INT-REC1 r2:2391: camera params as data; region query <=0.2 ms/tick for 48 clients; latencies 0/50/100/200 ms +-20 jitter.
- INT-REC2 r2:2415: areas + portals map data; flood numbers; re-flood on portal change <=1 ms (walls destroyed) - map data from editor.
- INT-REC3 r2:2436: one shared visibility pass for bots and interest; "rays the static world through the spatial index the sim already needs (CS6)"; <=10% of server tick budget; 24v24 and 16v16v16 (48 players 3 teams - note 16v16v16 not in VISION §3 goals?); "Wasm and native results identical per tick (R1's state-hash test)".
- INT-REC4 r2:2452: priority accumulator; ranking+packing <=0.5 ms/tick.
- INT-REC5 r2:2469: visibility class per field: public/team/owner/spatial.
- INT-REC7 r2:2512: spectators conditional (XU-OQ17).
- TRN-REC1 r2:2532: quinn 0.11.x; wtransport 0.7.x vs web-transport-quinn 0.13.x by spike; browsers Chrome 156, Edge 156, Firefox 158, Safari 27 (macOS); load: 60 input datagrams/s <=100 B, 30 snapshots/s <=1000 B (placeholder); transport <=1 CPU core; "reference server hardware R6 names". Version floors quinn-proto >=0.11.18, rustls >=0.23.46, ring >=0.17.12, tokio >=1.54.1; "An audit gate ... belongs to R6's security baseline (dependency audits), deferred by the Advisor" (r2:2577-2578).
- TRN-REC2 r2:2580: snapshot datagram cap 1,000 B payload [vs BUD-REC3 min(maxDatagramSize, 1150 B)]; control on one bidi stream; bulk on own uni stream.
- TRN-REC3 r2:2602: WebSocket fallback tokio-tungstenite 0.30.x OR axum 0.8.x ws (axum 0.8.9 deps tungstenite ^0.29); WT attempt timeout 1.5 s; no WT over HTTP/2.
- TRN-REC4 r2:2636: thin transport interface both sides; own ping RTT; protocol version in `protocols`; feature-detection test per browser in e2e suite (R5); cargo tree check sim has no transport dep; fuzz 10M inputs per decoder.
- TRN-REC5 r2:2662: dev+previews self-signed ECDSA P-256 <=14 days via serverCertificateHashes, hash fetched from page's HTTPS origin; production publicly trusted cert per game server hostname; pinning in prod only if R6 picks short-lived ephemeral servers.
- TRN-REC6 r2:2684: NewReno/Cubic; client input outgoingMaxAge 100 ms.
- ROOM-REC1 r2:2708: lifecycle; match ~20 min; telemetry written at Ended.
- ROOM-REC2 r2:2721: one room per OS process; sim own thread + async I/O; supervisor (R6) starts process per room; panic="unwind" server; ref host "one 4-vCPU EU cloud VM, R6 to name it"; 48p room p99 tick <=50% of tick period [vs BUD-REC6 25% whole tick]; 8 rooms on the 4-vCPU host; memory ceiling 64 MB/room; startup <2 s.
- ROOM-REC3 r2:2740: signed join token from lobby, 30 s expiry; signature scheme OQ2.
- ROOM-REC4: grace 60 s. ROOM-REC5: private room codes; factions per team.
- ROOM-REC6 r2:2799: bots fill per team; spike 3 teams x 16 slots (48).
- ROOM-REC7 r2:2813: modes as data; spike 2x2, 2x8, 3x6, 3x18, 12v12v12, 24v24 -> 3x18 = 54 players > 48 (check VISION §3 and R1 largest-mode = 48 at r1:2588).
- ROOM-REC8 r2:2822: guest id + HttpOnly cookie secret from lobby; lobby speaks HTTPS (guest create, room create, code lookup, token request) r2:2842 -> R6 must host a lobby.
- Room control: JoinAccepted{..., modeId, modeHash, serverTick} r2:2840 (R1-DATA-REC6 says server names content hash at join: modeHash vs content-set hash?).
- R2 OQs of note: SYNC-OQ2 (cap 200/250), SYNC-OQ3 (when server runs commands), SYNC-OQ4, SYNC-OQ5 (collision), SYNC-OQ8 (rewind destructibles/moving cover, R4 moving obstacles), SYNC-OQ10 (predict abilities: no source; Feel Lab), SYNC-OQ11 (shared spread seed; now server-only), BUD-OQ1 (shipped tick rates unread), BUD-OQ4 (Octopon-3 size sets position bits: +-327 m / 41 m height), BUD-OQ5 (vehicle/projectile caps; estimate assumes one vehicle per squad + 32 projectiles), BUD-OQ6 (team sizes totals or per team, Mustafa), BUD-OQ8 (spawn-only projectiles need R1 determinism), BUD-OQ11 (IPv6 share -> R6), INT-OQ1/OQ2 (look brief), INT-OQ5 (LOS cost -> R4), TRN-OQ1 (draft negotiation -> R5 CI beta channels), TRN-OQ4 (prod certs -> R6 hosting), TRN-OQ7 (fallback share -> R6 monitoring), ROOM-OQ1 (per process vs many -> R6 host sizes/costs), ROOM-OQ2 (token scheme: asymmetric; key storage R6 deferred), ROOM-OQ9 (lobby location + store -> R6), ROOM-OQ11 (guest secret: cookie; CSP R6 deferred).
- XU-OQ1 rewind cap; XU-OQ2 RTT vs one-way; XU-OQ3 interp bounds; XU-OQ4 input redundancy; XU-OQ5 datagram cap 1000/1150; XU-OQ6 join path; XU-OQ7 history; XU-OQ8 CPU accounting (R2-internal only; does NOT mention R1-TICK-REC2's 25% for sim alone); XU-OQ9 ref server (R6 instance); XU-OQ12 network profiles (R5's network simulator runs them); XU-OQ13 team sizes; XU-OQ14 top speed; XU-OQ17 spectators.
- R2 pointers to R6 r2:3300-3312: ~7 Mbit/s egress and 3.2 GB per match-hour per 48-human room at E3 all in view; IPv6 share; UDP on EU hosts and LBs, connection IDs; public certs + HTTPS matchmaker handing out cert hashes (TRN-OQ4); fallback-share telemetry; room supervisor, fleet scaling, regions, draining, per-room crash reports, lobby store, signing-key storage (ROOM-OQ9); relevance churn telemetry; ref server (XU-OQ9); security baseline deferred: dependency audit for TRN-REC1 floors, CSP, cheat detection.
- R2 pointers to R5 r2:3296: network simulator runs 0-200 ms; shared profiles XU-OQ12; e2e per browser + recurring beta channels (TRN-OQ1).
- R2 pointers to R4 r2:3291: spatial index CS6, height-aware LOS, moving obstacles as cover (SYNC-OQ8), enclosure areas/portals as map data, destructible walls opening portals, ray query cost (INT-OQ5).
- R2 pointers to R1 r2:3274: trap recovery R1 owns; transport in sim worker or own worker "R1's choice"; sim instantiable per room no globals.
- R2 cites NO R1 id at all (grep "R1-" in r2 = 0). So R1 pointers by id unanswered in R2: R1-TICK-OQ4 (sub-tick timestamps), R1-TICK-OQ8 (client tick nudge/time sync), R1-DET-OQ9 (hash fn, prod hash compare), R1-BND-OQ8 (snapshot layout vs page block), R1-DATA-OQ3 (shipped-source for mid-match tuning), R1-TICK-OQ3 (R2 also did not read Overwatch/Valve: R2-BUD-OQ1). R1-DATA-REC6 "server names content hash at join" vs R2 JoinAccepted{modeId, modeHash, serverTick} r2:2840 (no content-set hash).
- R2 register DONE. Next: R1 x R2 comparison items, then R3 register.

## Items found (R1 x R2)
I1 contradiction: tick CPU budget meaning. R1-TICK-REC2 P1 r1:2014-2018 sim-only p99<=4.2 ms (25%) "leaves room for snapshot encoding, interest management..." vs R2-BUD-REC6 r2:2377-2379 whole tick incl. sim <=25% (<=4 ms at 60 Hz) vs R2-ROOM-REC2 r2:2733 <=50% vs R1-ECS-REC1 r1:1908-1909 mean<=2 / p99<=4 ms. Existing: R2-XU-OQ8 (R2-internal), R1-XU-OQ1 (R1-internal); neither names the other topic. Decider: ADR-0001 + ADR-0004 synthesis one tick budget; spike P1/BUD-REC6.
I2 contradiction: reference server: R1 "one server core of the R6 host type" r1:1905-1906 vs R2-BUD-REC6 "one vCPU of the EU instance type R6 recommends" r2:2385 vs R2-ROOM-REC2 "one 4-vCPU EU cloud VM" r2:2732. Existing R2-XU-OQ9, R1-TICK-OQ6. Decider: R6/ADR-0006 + synthesis.
I3 contradiction: prediction scope abilities: R1-ABL-REC4 r1:2354-2360 predicts local abilities (activation, cost, cooldown, tags, Banzai speed) vs R2-SYNC-REC3 r2:2061 option 3.2 C own movement/weapon/vehicle/cosmetic; abilities = option 3.2 D left open R2-SYNC-OQ10 r2:2889, r2:3341 "stops short of abilities". Existing: R1-ABL-OQ4 (asks R2), R2-SYNC-OQ10. Decider ADR-0001/0005, Feel Lab spike.
I4 contradiction/hidden dep: spread RNG: R1-DET-REC4 r1:2137-2140 sim RNG streams incl "weapon spread", state part of sim state + hash; R1-ABL-REC3 r1:2342 spread from sim's seeded RNG; vs R2-SYNC-REC3 r2:2073-2077 authoritative spread from server-only stream, client cosmetic. Existing R2-SYNC-OQ11 (the seed choice) but not the R1 side. Decider: ADR-0001 + ADR-0004; Mustafa (SEC-SPREAD).
I5 contradiction: largest mode: R1-TICK-OQ7 r1:2588 "largest mode (48 soldiers, 47 bots)"; R2-ROOM-REC7 r2:2820 modes 3x18 (54); R2-ROOM-REC6 3x16. Existing R2-BUD-OQ6 / R2-XU-OQ13 (R2-internal). Decider Mustafa.
I6 gaps: R1 pointers unanswered by R2 (TICK-OQ4, TICK-OQ8, DET-OQ9, BND-OQ8, DATA-OQ3, TICK-OQ3).
I7 hidden dep: content hash at join R1-DATA-REC6 r1:2491 vs R2 JoinAccepted r2:2840 modeHash only.
I8 whole-world replay (R1-ECS-REC1 mark 3) vs R2-SYNC-REC3 local only: R2 answers R1-ECS-OQ3/XU-OQ2 by subject (local player + vehicle). Not a conflict, answered; R1-ECS-REC1 mark 3 then measures something R2 does not use? -> note: R2-SYNC-REC9 "only the local player and its vehicle are replayed". R1 mark 3 whole-world replay 8 ms stays as a spike mark: possible wasted spike. List as hidden dependency low rank.
I9 interp delay vs R1-TICK-REC5? not conflict.
I10 tick wrap: R1 pointer 16-bit tick wraps 18.2 min at 60 Hz r1:2924-2925; R2-ROOM-REC1 match ~20 min r2:2711; R2 wire tick width not stated? check.

## Register R3 (r3-rendering.md) -- DONE
- three.js r186 pinned (WGPU-REC2 r3:913); WebGPURenderer + TSL, WebGL 2 fallback (WGPU-REC1); compileAsync behind loading screen; backend logged; "the game is served from a secure context (HTTPS, or localhost in development)" (WGPU-REC4 r3:936-938).
- Browsers: Chrome, Edge, Firefox on Windows, Safari 26 (WGPU-REC1 mark 1 r3:906-907) [vs R2-TRN-REC1 Chrome 156, Edge 156, Firefox 158, Safari 27 r2:2553 - not necessarily conflict: R3 names Safari 26 for WebGPU].
- GL2-REC1/2/3; GPU timing best effort.
- BUDGET-REC1 r3:1088-1094: min = laptop Intel Gen12 Iris Xe class or AMD iGPU, >=4 cores/8 threads, 1080p, Windows 11 + Linux run; rec = desktop mid-range discrete card; RTX 5080 is headroom only.
- BUDGET-REC2 r3:1110-1129: scene 48 soldiers, 4 large units, 200 projectiles (R1-TICK's scene); min Low p95 <=33.3 ms; rec High <=16.7; main-thread CPU <=16/<=8 ms; "sim cost seen by the page (R1-TICK, R1-BND) <=1.0 ms"; draw calls <=800 (Low no outline), ~1,100 with hull outline, <=1,500 High; GPU mem <=512 MB / 1 GB; JS heap <=150 MB.
- BUDGET-REC3: quality tiers as data.
- MEASURE-REC1: 3 runs x 600 frames after 300 warm-up. MEASURE-REC2 r3:1186-1188: benchmark = recorded match replayed from inputs (R1-DET-REC1/REC6), URL flag on any build "including preview links (R5)". MEASURE-REC3 r3:1197-1206: CI gates only hardware-independent counts (draw calls, triangles, programs, GPU memory, zero alloc); frame times only from ref machines; CI job <5 min.
- CHAR-REC1..4: one skinned mesh/soldier; budget LOD; per-instance data from per-tick block (R1-BND-REC2, R1-TICK-REC5); large units 4 x >=20 parts.
- AUDIO-REC1..4: equalpower panners, 32/48 voices; occlusion O-C zones from map data or none; O-B rays "R4's line-of-sight work owns"; sounds from per-tick event section.
- OQs: WGPU-OQ1 Brave; GL2-OQ2 fallback share -> "client telemetry of the backend after launch (R6's telemetry)"; GL2-OQ5 drop fallback -> ADR-0003 + Mustafa; BUDGET-OQ1 models -> Mustafa/Advisor, R6 telemetry; BUDGET-OQ2 high refresh cap; BUDGET-OQ4 largest N (R1-TICK-OQ7); MEASURE-OQ2 self-hosted ref machines in CI -> "R5 and R6"; MEASURE-OQ4 isolation for profiling -> R1-BND-REC5 route; AUDIO-OQ5 occlusion ownership -> R4 + synthesis.
- Pointers r3:1398-1443: R5: three.js pin = lockfile entry, upgrade check is a CI job; headless can force WebGL 2; MEASURE-REC3 is a gate v2 check; URL flag on preview links. R6: backend+adapter one field in client's session report; hardware mix from telemetry; results logs with telemetry. R1: CHAR-REC3/4 assume R1-BND-REC2 sections carry aim yaw/pitch, ADS, reload, throw, draw state per soldier (GA15). R4: occlusion rays want same static geometry + floor data.
- R3 does NOT take up R1 pointer "rendering in a worker with OffscreenCanvas (boundary-worker option A3)" (grep offscreen = 0) nor "per-entity or per-field layout for instancing (R1-BND-OQ6)" explicitly.

## Items (R1/R2 x R3)
I11 contradiction (entity counts in stress scene, cross-topic): R1-ECS-REC1 8 vehicles/300 proj/200 effects r1:1907-1908; R1-TICK-REC2 4 big vehicles/200 proj r1:2014; R3-BUDGET-REC2 4 large units/200 proj r3:1110-1111 (takes R1-TICK side); R2-BUD worked estimate 15 vehicles (one per squad + harvester) and 32 projectiles at 48 r2:1757-1758, 1771; R2-SYNC-REC5 mark 3 "48 soldiers plus 12 vehicles and harvesters" r2:2173. Existing R1-XU-OQ1 (R1 only), R2-BUD-OQ5 (caps). Decider: game-designer caps + synthesis (ADR-0004/0001/0003 one stress scene).
I12 hidden dep: R3-CHAR pointer r3:1421-1422 assumes R1-BND-REC2 sections carry aim yaw/pitch, ADS, reload, throw, draw state; R1-BND-REC2 r1:2210-2211 lists sections by kind, no fields; R2 soldier record r2:1745 "Health, flags, weapon (8+8+4 bits)" - remote soldiers' anim state on the wire not stated. existing none. Decider ADR-0001 (snapshot fields) + ADR-0004 (block layout); R1-BND-OQ6.
I13 gap: R1 pointer to R3 r1:2937 OffscreenCanvas option A3 and per-entity/per-field layout (R1-BND-OQ6) - R3 silent on both. low.

## Register R4 (r4-physics-navigation.md) -- DONE
- TL;DR r4:40-45: parry3d + own kinematic movement, Rapier controller measured fallback; immutable static world; nav baked into map doc, sim queries; one height-aware sight query; vehicles avoided not carved. "C++ Detour does not fit R1's browser target (wasm32-unknown-unknown)".
- Ref hw r4:657-659 "the one R1-TICK-REC2 proposes (one core of the R6 server type; VISION min laptop for Wasm)".
- PHYS-REC1 r4:663: no physics engine; parry3d pinned enhanced-determinism; mark 2 movement+physics <= half of ECS-REC1 mark 1 mean (<=1 ms native mean); mark 4 size with parry3d/rapier3d/neither; mark 5 Rapier controller fallback.
- PHYS-REC2 r4:707: static world from map doc, immutable, BVH built once; mover index rebuilt per tick; not in snapshot; BVH build <=50 ms.
- PHYS-REC3 r4:731: projectiles as per-tick swept queries; no CCD; 30 and 60 Hz.
- PHYS-REC4 r4:746: vehicles kinematic compounds; no substeps at 30 Hz (answers R1-TICK-OQ9); stress keeps 8 big vehicles (R1-ECS-REC1's number) harvesters included r4:765-767.
- PHYS-REC5 r4:769: f32 whole sim up to 2 km maps (answers R1-DET-OQ3).
- NAV-REC1 r4:785: sim owns nav runtime; bake by tool into map doc (rerecast vs layered grid by spike); bake <=1 s native, <=3 s browser Worker; reachability in sim crate from Node + CI.
- NAV-REC2 r4:826: path queue; nav <=10% of tick p99 budget; P1 scene with 8 vehicles instead of 4.
- NAV-REC3: avoidance step in movement; dodgy_2d ORCA after audit; <=0.2 ms p99.
- NAV-REC4 r4:863: nav on server + browser offline only; "Prediction replays only the local player or its vehicle (R1-TICK-REC2 P3)"; offline largest mode meets P2.
- FLOORS-REC1..3: typed links by simulated movement; layers with ceilings sharing R2-INT-REC2 area ids; bake hash of movement tuning.
- LOS-REC1..3 r4:929-981: one height-aware sight query; mark 4 "R2-INT-REC3's full pass (24v24 and 16v16v16, three points) inside its 10 % budget"; LOS-REC2 4 scans/s; LOS-REC3 same static world client/server.
- OBST-REC1..3: vehicles avoided + temporary cost; structures baked both states, flag flips with R2-INT-REC2 re-flood; tiles only if bake too slow.
- OQs: PHYS-OQ1 blocking (game-designer), PHYS-OQ3 parry BVH SIMD cross-target, PHYS-OQ4 ground shape (map format), PHYS-OQ5 dynamics later; NAV-OQ1..5; LOS-OQ1 soldiers/vehicles block sight; OBST-OQ2 harvesters lanes.
- "Wave-1 items R4 changes or answers" r4:1110-1129: ECS-REC4 + ECS-REC1 mark 2 fall away; DET-REC3 flag moves to parry3d; P1 scene 4->8 vehicles (R1-XU-OQ1 for synthesis); TICK-OQ9 answered; DET-OQ3 answered (f32 <=2 km); DATA-OQ7 answered for nav; nav runs server+browser; spatial index rebuilt; R2-INT-OQ5 measured by LOS-REC1 mark 4.
- Pointers r4:1131-1174: R5: counting test for static BVH build, parity fuzz, native-vs-Wasm hash suite covers parry3d, reachability check from Node + CI, stall-rate comparison (TL13) in v2 harness. R1/ADR-0002: Detour blocker holds only while ADR-0002 keeps wasm32-unknown-unknown.

## Items (R1/R2/R3 x R4)
I14 contradiction (budget sum): R1-ECS-REC1 mark 1 mean tick <=2 ms native r1:1908-1909; R4-PHYS-REC1 mark 2 movement+physics <=1 ms mean (half) r4:691-692; R1-ABL-REC6 ability/effect/weapon step <=1 ms/tick r1:2395-2396 -> together = whole 2 ms mean; plus R4-NAV-REC2 nav <=10% of tick p99 r4:841, R2-INT-REC3 visibility <=10% r2:2447-2448, R2-INT-REC1 0.2 ms, R2-INT-REC4 0.5 ms, R4-NAV-REC3 0.2 ms, AI/bots none. Fold into I1 (tick budget) as a cross-topic sum. Existing: R2-XU-OQ8 (R2 only), R1-XU-OQ1 (R1 only).
I15 hidden dep (Rapier assumed): R1-DET-REC3 r1:2125, R1-ECS-REC4 r1:1965, R1-ECS-REC1 reason "Rapier's sets ... slot straight into the struct" r1:1894 + marks 2/5, R1 TL;DR r1:59, R1-TICK-REC2 reason "it is Rapier's default step (F8)" r1:2000; R1-RR-OQ1 (wide/simba FMA in Rapier path) vs R4-PHYS-REC1 r4:663 no physics engine. R4 lists ECS-REC4, ECS-REC1 mark 2, DET-REC3 r4:1114-1118 but not TICK-REC2's 60 Hz reason nor RR-OQ1's applicability to parry3d (R4-PHYS-OQ3 partly). Decider ADR-0004 with R4-PHYS-REC1 spike.
I16 contradiction (vehicles in the one stress scene): R4-PHYS-REC4 8 vehicles r4:765-767, R4-NAV-REC2 r4:839-840 vs R1-TICK-REC2 4 r1:2014, R3-BUDGET-REC2 4 large units r3:1110-1111, R3-CHAR-REC4 four large units r3:1079. merge with I11. Existing R1-XU-OQ1 (R4 cites it).
I17 contradiction (three-team test sizes): R1-DET-REC1 12v12v12 r1:2098 (VISION goal) vs R2-INT-REC3 16v16v16 r2:2447, R4-LOS-REC1 mark 4 / LOS-REC2 16v16v16 r4:954, r4:969, R2-ROOM-REC6 3x16 r2:2809, R2-ROOM-REC7 3x18 r2:2820. merge with I5 largest mode. Existing R2-XU-OQ13/R2-BUD-OQ6.
I18 hidden dep: R4-NAV-REC4 r4:865-866 "Prediction replays only the local player or its vehicle (R1-TICK-REC2 P3)" while R1-XU-OQ2 open (R1-ECS-REC1 mark 3 whole world). R2-SYNC-REC3 local only. Merge into replay item.

## Register R5 (r5-toolchain-quality.md) -- in progress
- TL;DR r5:53-57: one repo Cargo + npm workspaces; exact pins Rust 1.99.1, TypeScript 6.0.x; clippy -D warnings + sim bans; per-tick hashes native, Node, three engines and real Safari; R2's P0-P4 and B1 net sims; 48-bot soaks; CI cached from main; secret-free previews; gate v2 G0-G12, --prove.
- LAYOUT-REC1 r5:1151: rust-toolchain 1.99.1 + wasm32-unknown-unknown; npm workspaces; edition 2024.
- LAYOUT-REC2 r5:1168-1180: crates sim, content, protocol, sim-wasm, server ("the binary with transport and rooms"), devtools (replay runner, hash comparer, legacy-golden reader); packages client, editor, net, sim-worker, types, e2e. "No other crate or package until a §6 item asks for it." sim depends on content only.
  -> no home for R4's nav bake tool (rerecast generator; runs in editor Test, on save, CI, <=3 s browser Worker), nor for R2's lobby (HTTPS guest/room/code/token) and room supervisor.
- LAYOUT-REC3: types package; ts-rs + JSON Schema (R1-DATA-REC3) + block layout (R1-BND-REC4 D4) all in packages/types. [touches R1-XU-OQ8: R5 puts both generators in one package but does not say which is the source of content TS types]
- LAYOUT-REC4: one feature set for sim.
- LAYOUT-REC5 r5:1215: Vite two entries; Worker by URL; COOP/COEP behind flag dev/preview (R1-BND-REC5).
- TSLINT-REC1..4: TS 6.0.x; ESLint 10; Prettier.
- RUSTLINT-REC1..6: unsafe deny; clippy -D warnings host + wasm32; sim disallowed HashMap/HashSet/Instant/SystemTime/std transcendental; no-panic lints protocol+content; overflow-checks on sim+protocol in release; rustfmt.
- TEST-REC1 r5:1381-1395: layers; Safari via safaridriver; benchmarks "the tool was not read (R5-TEST-OQ8)", "never a gate pass or fail (R3-MEASURE-REC3)".
- TEST-REC2 r5:1404: determinism suite: devtools replay runner native x2, Wasm Node, Chromium/Firefox/WebKit via Playwright, real Safari; gate recordings 2 min "at the tick rate R1-TICK picks", modes two teams/three teams/bots only.
- TEST-REC3: legacy goldens via tools/preview.js legacy eval.
- TEST-REC4 r5:1446-1465: L1 own link sim behind R2-TRN-REC4 interface, "half the profile's round trip each way"; L2 impairment wrapper under quinn; "Profiles: R2's P0-P4 and B1 by name"; "one named set settles R2-XU-OQ12".
- TEST-REC5 r5:1466-1479: soak 24v24=48 and 12v12v12=36 + N=12,24,36,48; 30 min at P2; "tick p99 inside the budget ADR-0004 sets"; "against R1-TICK's budget", "against R2-BUD's budget".
- CI-REC1..5 r5:1500-1568: rust-cache v2.9.3; toolchain pinned; wasm-bindgen CLI pinned via taiki-e; setup-node v7; gate as G1 option one aggregate check; scheduled fuzz; Dependabot.
- PREV-REC1..5 r5:1573-1629: P1 shape (deploy job with host token, no PR code); host needs list; "Cloudflare Pages meets every need except full cleanup"; "The choice and its price are R6's"; preview headers COOP/COEP on preview hostnames only, noindex, application/wasm, "R6's content security policy once written"; previews public unlisted; access wall optional; "This changes once a preview talks to a real server".
- GATE-REC1 r5:1634-1653: G0..G12; G5 cargo-deny bans; G7 Wasm/JS size budgets; G10 determinism short recordings; G11 network L1; G12 e2e incl. R3-MEASURE-REC3 counts. Scheduled: soak, fuzz, cargo-mutants, beta channels (R2-TRN-OQ1), "cargo-deny advisories (F3: a new advisory must not turn an unrelated PR red; a PR that changes Cargo.lock runs the advisories check too)".
  -> vs R2-TRN-REC1 mark 5 r2:2577-2578 "An audit gate that keeps these current belongs to R6's security baseline (dependency audits), deferred by the Advisor" and R1-RR-OQ7 r1:2869-2870 "cargo-audit and cargo-deny as CI gates. Decides it: R5's CI research and the synthesis."
- GATE-REC2..4: one command node tools/gate.mjs v2; --prove mutation suite; budgets as data.
- R5 OQs of note: LAYOUT-OQ2 (wasm-pack vs CLI -> ADR-0002), TSLINT-OQ5 (innerHTML rule -> security-auditor with R6 baseline), RUSTLINT-OQ1 (disallowed-methods for float methods), RUSTLINT-OQ5 (HashMap ban crate-wide vs R1-ECS-REC2 outcome code only), TEST-OQ4 (macOS cost -> R5-CI and R6), TEST-OQ8 (bench tool not researched), CI-OQ1 r5:1801-1803 cache sizes for a workspace "bevy_ecs or hecs, Rapier, quinn and wtransport" [stale vs R1-ECS-REC1 no ECS crate + R4-PHYS-REC1 no Rapier], CI-OQ5 GPU runner (owner + R3), CI-OQ7 arm64 leg (R1-DET-OQ7, R6), PREV-OQ2 Pages deletion (spike or R6), PREV-OQ3 previews need a game server? (phase 1 + R6 hosting), PREV-OQ4 host+price (R6), GATE-OQ2 WebGPU in e2e (graphics-engineer), GATE-OQ4 advisories in PR gate or scheduled (security-auditor with R6 baseline) -> names the R1-RR-OQ7 conflict.
- XQ-OQ1 Safari macOS; XQ-OQ2 lockfile; XQ-OQ3 isolation headers always vs flag, preview vs production headers (Mustafa's Brave check on isolated preview).
- Pointers r5:1936-1951: For R6: previews needs list; settle PREV-OQ2 + PREV-OQ4; GPU runner or >10 GB cache paid?; server CPU arch decides CI-OQ7. For R6 security baseline: preview deploy secret; dependency audits; CSP.
- R5 cites R1/R2/R3 ids but NO R4 id; R4 pointers to R5 (counting test for static BVH build, parity fuzz REC2(a), hash suite covers parry3d, reachability check Node+CI NAV-REC1 mark 4, stall-rate TL13 comparison, bake stale check in CI FLOORS-REC3 r4:924-925) not taken into G0-G12 (only listed as inputs at r5:987-988).
- R1 pointers to R5 unanswered: R1-DATA-OQ11 (editor double-click + Wasm from file://: "ADR-0002 and R5's build research") - R5 no file:// mention; R1-DATA-REC4 golden migration fixtures; R1-DATA-REC5 Vite plugin; R1-ABL-REC6 "method R5 sets" vs R5-TEST-OQ8 (bench tool not researched); R1 Worker tick bench in CI (R5: benchmarks reported, never gated).
- R5 register DONE. Next: R6 register.

## Items (R1..R4 x R5)
I19 contradiction/owner: dependency advisories gate: R1-RR-OQ7 r1:2869-2870 (CI gates; R5's CI research decides) vs R2-TRN-REC1 mark 5 r2:2577-2578 (audit gate belongs to R6's deferred baseline) vs R5-GATE-REC1 r5:1652-1653 (cargo-deny advisories scheduled + on Cargo.lock change) + R5 pointer r5:1946-1948 (dependency audits are R6 baseline's). Existing R5-GATE-OQ4. Decider security-auditor/R6 baseline -> ADR-0002/0006; Advisor (deferral).
I20 hidden dep (stale assumption): R5-CI-OQ1 r5:1801-1802 assumes bevy_ecs or hecs and Rapier in the workspace vs R1-ECS-REC1 r1:1878 (no ECS crate) and R4-PHYS-REC1 r4:663 (no physics engine; parry3d). none. low.
I21 gap: R4 pointers to R5 unanswered (bake + stale check in CI, reachability, BVH counting test). R5 crate list r5:1168-1176 has no home for nav bake (rerecast) used by editor Test in a browser Worker (R4-NAV-REC1 r4:790-791, mark 1 r4:814-815). none. Decider ADR-0002 + ADR-0004.
I22 gap: R1-DATA-OQ11 r1:2731-2737 editor file:// + Wasm left to "ADR-0002 and R5's build research"; R5 silent (no file:// mention) while R5-LAYOUT-REC5 builds editor as Vite entry with Worker by URL. Existing R1-DATA-OQ11. Decider ADR-0002.
I23 gap: R1-ABL-REC6 r1:2405 "measured with the method R5 sets"; R1 pointer r1:2948 "Rust tick bench and a Worker tick bench in CI"; R5-TEST-REC1 r5:1395 bench tool not read, never gated; R5-TEST-OQ8. Existing R5-TEST-OQ8. low.
I24 hidden dep: R5-TEST-REC4 r5:1451-1457 "one named set settles R2-XU-OQ12" with R2 P0-P4/B1, but R2-BUD-REC1 r2:2301 and BUD-REC4 r2:2357 marks still at +-10 ms jitter (P2 is +-20 ms) and R2-INT-REC1/ROOM marks unnamed latency. existing R2-XU-OQ12. low.
I25 contradiction: isolation headers on previews vs prod: R5-PREV-REC4 r5:1609-1610 COOP/COEP on every preview vs R5-LAYOUT-REC5 behind flag; R1-BND-REC5 prod non-isolated; Mustafa's Brave check on a header set prod lacks. Existing R5-XQ-OQ3 (R5 internal, cites R1-BND-REC5). Decider engine-architect ADR-0002/0003 with R1-BND-REC2 mark 5.

## Register R6 (r6-infrastructure.md) -- DONE
- TL;DR r6:44-48: Cloudflare Workers static assets (or Pages) for client + previews, COOP/COEP via _headers; Hetzner Cloud one EU location, CCX23 dedicated (ref server), CX33 measured; no LB on UDP; one wildcard cert via ACME; cost ~18 EUR lean to 102 EUR + 5 USD start, 240-360 EUR + ~20 USD + Sentry goal; tracing JSON, Prometheus, Sentry EU, Reporting API; telemetry record at Ended (content hash, mode id, 10 s timeline, events with source ids, no player ids); security baseline deferred (R6-SEC-OQ1).
- HOST-REC1 r6:609: Workers static assets primary, Pages alternative; COOP/COEP on previews "when R1-BND-REC5's option G3 is on"; "Previews stay harmless rather than secret: ... talks only to the staging game server (REC3)".
- HOST-REC2 r6:627: Hetzner, supervisor per VM, no LB; CCX23 = ref server for R2 (R2-XU-OQ9); x86-64 answers R1-DET-OQ7.
- HOST-REC3 r6:653: one shared staging game server for previews; self-signed pinned certs; offline previews in the worker; binaries via CI (pointer r6:946).
- HOST-REC4 r6:664: wildcard cert departs from R2-TRN-REC5 per-hostname (stated).
- COST A-2 r6:512 8 rooms/4-vCPU VM (2 per vCPU); start scenario reads 6v6v6 = 18 (totals) r6:442; no lobby line, no CI runner line.
- OBS-REC1..4; OBS-REC3 Sentry wasmIntegration, debug files uploaded by CI; crash reports "to our own lobby endpoint" r6:730.
- TEL-REC1..3: record shape; supervisor uploads; batch analysis by content hash.
- OQs: HOST-OQ1 Hetzner availability; HOST-OQ5 private previews (Mustafa); COST-OQ1 rooms per VM; OBS-OQ5 client field perf (R3 + synthesis); TEL-OQ2 bots/private matches; SEC-OQ1 deferred list.
- R6 cites no R5 id: R5's asks (PREV-OQ2 Pages deletion, PREV-OQ4 price, GPU runner, >10 GB cache, macOS runner cost) unanswered; lobby location/store (R2-ROOM-OQ9) unanswered (only key storage deferred); R3's client session report field unanswered (R6-OBS-OQ5 points back to R3); R1-BND-OQ5 embeds/login + CORP unanswered.

## Pair comparison: DONE (R1xR2, R1xR3, R2xR3, R1..R3xR4, R1..R4xR5, R1..R5xR6). Wrong-cross-reference spot checks done (R6->r2 lines, R4->R1-DET-F6/BND-F19, R5->R1/R2 marks): all hold except the ambiguous "REC5's test" in R5-PREV-REC4.

## FULL LIST (ranked by ADR impact)
1 contradiction tick budget meaning + sum + density (R1-TICK-REC2 P1, R1-ECS-REC1 m1, R1-ABL-REC6, R4-PHYS-REC1 m2, R4-NAV-REC2, R2-BUD-REC6, R2-BUD-REC1, R2-ROOM-REC2, R2-SYNC-REC9, R2-INT-REC3, R2-TRN-REC1 m2, R6 A-2). existing R2-XU-OQ8, R1-XU-OQ1 (each internal). ADR-0001+0004 (+0006 density).
2 hidden dep cursor aim across boundary (R1-BND-REC1 m2, R1-BND-REC3, R1-DET-REC2 vs R2-SYNC-REC8, R4-LOS-REC3). none. ADR-0001+0004.
3 hidden dep Rapier assumed in R1 vs parry3d in R4 (R1-DET-REC3, ECS-REC4, ECS-REC1, TICK-REC2 reason, TL;DR, RR-OQ1 vs R4-PHYS-REC1; R4 lists some). existing: R4 "wave-1 items" + R4-PHYS-OQ3. ADR-0004.
4 hidden dep where remote interpolation buffer lives (R1-TICK-REC5, R1-BND-REC2/REC6 vs R2-SYNC-REC4, R5-TEST-REC1). none. ADR-0001+0004.
5 contradiction ability prediction scope (R1-ABL-REC4 vs R2-SYNC-REC3/OQ10). existing R1-ABL-OQ4, R2-SYNC-OQ10. ADR-0001+0005; Feel Lab spike.
6 contradiction spread RNG sim stream vs server-only (R1-DET-REC4, R1-ABL-REC3 vs R2-SYNC-REC3). existing R2-SYNC-OQ11 (R2 side only). ADR-0001+0004; Mustafa SEC-SPREAD.
7 hidden dep projectile E3 relies on cross-target identity (R2-BUD-REC2, SYNC-REC6, BUD-OQ8 vs R1-DET-REC1). existing R2-BUD-OQ8 (R1 never answers). ADR-0001+0004; spike.
8 gap lobby + supervisor: no host, cost line or crate (R2-ROOM-REC2/REC8/OQ9 vs R6-HOST-REC2, R6-COST, R6-OBS-REC3, R5-LAYOUT-REC2). existing R2-ROOM-OQ9. ADR-0006+0002+0001.
9 contradiction stress-scene entity counts (R1-ECS-REC1, R1-TICK-REC2, R3-BUDGET-REC2, R4-PHYS-REC4, R4-NAV-REC2, R2 estimate, R2-SYNC-REC5 m3). existing R1-XU-OQ1, R2-BUD-OQ5. synthesis + game-designer caps.
10 contradiction team sizes / largest mode (R1-TICK-OQ7, R1-DET-REC1, R5-TEST-REC5 vs R2-ROOM-REC7, R2-ROOM-REC6, R2-INT-REC3, R4-LOS-REC1/2, R6-COST). existing R2-BUD-OQ6, R2-XU-OQ13, R3-BUDGET-OQ4. Mustafa.
11 contradiction ref server core vs vCPU (R1-ECS-REC1, R1-TICK-REC2, R4 vs R2-BUD-REC6, R6 pointer). existing R2-XU-OQ9, R1-TICK-OQ6. ADR-0006 + synthesis.
12 gap R1 timing pointers unanswered by R2 (TICK-OQ3, OQ4, OQ8). existing those OQs. ADR-0001.
13 gap R1 protocol pointers unanswered by R2 (DET-OQ9, BND-OQ8, DATA-OQ3, DATA-REC6 hash at join vs JoinAccepted modeHash). existing those OQs. ADR-0001+0005.
14 hidden dep replay scope: R1-ECS-REC1 m3 whole world vs R2-SYNC-REC3/REC9 and R4-NAV-REC4 local only. existing R1-XU-OQ2 etc. ADR-0004.
15 hidden dep offline play assumed (R4-NAV-REC4, R6-HOST-REC3, R5-PREV-OQ3 vs R1-BND-OQ1/TICK-OQ7). existing R1-BND-OQ1, R1-TICK-OQ7, R1-XU-OQ12. Mustafa + Advisor.
16 hidden dep per-soldier anim state in block/snapshot (R3 pointer vs R1-BND-REC2, R2 record). existing R1-BND-OQ6 (partly). ADR-0001+0003+0004.
17 contradiction dependency advisories gate owner/placement (R1-RR-OQ7 vs R2-TRN-REC1 m5, R5-GATE-REC1, R6-SEC-OQ1). existing R5-GATE-OQ4, R6-SEC-OQ1. security-auditor/Advisor -> ADR-0002/0006.
18 hidden dep recommendations waiting on deferred baseline (CSP R5-PREV-REC4, R2-ROOM-OQ11, R5-TSLINT-OQ5, key storage R2-ROOM-OQ2). existing R6-SEC-OQ1. Advisor.
19 contradiction preview host Workers (R6) vs Pages-based needs/cleanup (R5); R5-PREV-OQ2 unanswered. existing R5-PREV-OQ2, R5-PREV-OQ4. ADR-0006 + ADR-0002.
20 gap multiplayer previews: staging binary delivery by CI + cert hash on page origin (R6-HOST-REC3, R6 pointer vs R5-PREV-REC1; R2-TRN-REC5 vs static host). existing R5-PREV-OQ3. ADR-0002+0006 (+0001).
21 contradiction preview privacy (R5-PREV-REC5 vs R6-HOST-REC1). existing R5-PREV-OQ1, R6-HOST-OQ5. Mustafa.
22 contradiction isolation headers on previews always vs flag (R5-PREV-REC4 vs R5-LAYOUT-REC5, R6-HOST-REC1). existing R5-XQ-OQ3. engine-architect ADR-0002 with R1-BND-REC2 m5.
23 contradiction wildcard vs per-hostname cert (R2-TRN-REC5 vs R6-HOST-REC4). existing R6-SEC-OQ1 / R2-TRN-OQ4 (stated by R6). ADR-0006 + security baseline.
24 gap nav bake + R4 checks have no home in R5 crates/gate (R4-NAV-REC1, FLOORS-REC3, pointers vs R5-LAYOUT-REC2, GATE-REC1). none. ADR-0002+0004.
25 gap editor file:// + Wasm (R1-DATA-OQ11 vs R5 silent, R5-LAYOUT-REC5, R3-WGPU-REC4). existing R1-DATA-OQ11. ADR-0002.
26 gap client telemetry of backend/hardware (R3-GL2-OQ2, R3 pointers vs R6-TEL/R6-OBS-OQ5). existing R3-GL2-OQ2, R6-OBS-OQ5. ADR-0003+0006.
27 hidden dep wire widths: generational ids + 16-bit tick wrap vs R2 7-bit index, 20-min match. none. ADR-0001.
28 gap client trap recovery owner (R2 pointer says R1; R1-RR-OQ3 says synthesis). existing R1-RR-OQ3. ADR-0002+0004.
29 contradiction HashMap ban scope (R1-DET-REC5 lookups allowed vs R5-RUSTLINT-REC2 type ban). existing R5-RUSTLINT-OQ5. ADR-0004 (engine-architect).
30 hidden dep same open question two topics: soldier/vehicle blocking (R2-SYNC-OQ5 vs R4-PHYS-OQ1/NAV-OQ5). existing both. game-designer + Mustafa.
31 gap content TS types source (R1-XU-OQ8 "with R5" vs R5-LAYOUT-REC3 both generators, no pick). existing R1-XU-OQ8. ADR-0005.
32 hidden dep network profiles settled by R5 but R2 marks unchanged (R5-TEST-REC4 vs R2-BUD-REC1/REC4). existing R2-XU-OQ12. synthesis.
33 gap CI runner costs left to R6 (GPU, macOS, self-hosted ref machines, >10 GB cache). existing R5-CI-OQ5, R5-TEST-OQ4, R3-MEASURE-OQ2. ADR-0006/0002.
34 hidden dep Sentry: CI upload secret job + G12 external-request mutation (R6 pointer vs R5-GATE-REC3, R5 secret list). none. ADR-0002/0006.
35 gap audio occlusion rays ownership (R3-AUDIO-REC3/OQ5 vs R4-LOS-REC1 users). existing R3-AUDIO-OQ5. ADR-0003/0004 (only if O-B).
36 gap bench method (R1-ABL-REC6 "method R5 sets" vs R5-TEST-OQ8). existing R5-TEST-OQ8. ADR-0002.
37 gap R1->R3 pointers (OffscreenCanvas A3, per-entity/per-field layout) unanswered. existing R1-BND-OQ6. ADR-0003/0004.
38 gap R1-BND-OQ5 embeds/login and CORP -> R6 silent. existing R1-BND-OQ5. Mustafa with R6.
39 gap R2->R3 cutaway visibility rule (look brief). existing R2-INT-OQ1. look brief.
40 hidden dep R5-CI-OQ1 stale (bevy_ecs/hecs, Rapier). none. ADR-0002 cache spike.
41 wrong cross-ref R5-PREV-REC4 "until REC5's test decides" -> deciding test is R1-BND-REC2 mark 5. none. editorial.
DONE
