Task: engine-architect adversarial review of plan revision 1 of the netcode proof test (.checkpoints/netcode-proof-test/plan.md), lens: boundaries, Rust<->TS boundary, data flow, dependency rules, VISION 6 tracing.

## Plan (blocks, in order)
B1. Read plan.md whole (notes: layout, crates, harness, determinism, room loop, kept, symbols).
B2. Read test-engineer digest sections B and C (+G11) and netcode-architect digest headers as needed.
B3. Read ADR 0002 (part 5, part 6, spikes/, Worker host, migration plan; lines 140-145, 325-373, 944-947).
B4. Read ADR 0004 part 5 (S17 room loop) and part 7 (determinism); GLOSSARY section 7.
B5. Read ENGINE_OVERVIEW.md and SPIKE_PLAN item 1 (and item 2/3 boundary spike, sim core spike) + ADR 0001 relevant marks.
B6. Cross-check findings, write findings list.
B7. DONE.

## Notes
### B1 done: plan.md read whole (700 lines). Suspects to check (plan line refs):
- S1 4.1:155-180 layout: own workspace inside spikes/netcode-proof; check vs ADR 0002:140-145, 944-947.
- S2 4.1:164 crates/server holds npt-room+npt-super+npt-host (one crate or three? unclear). 4.3:225 "npt-room (binary)" with pub items Room, run_tick_loop: a binary's items cannot be imported by an L1 test that runs server+clients in one process (6.4:334). Need lib+bin.
- S3 No crate/symbol for the L1 scenario driver (simulated clock stepping room + 48 clients): L1Net sits in npt-net (162, 222) but net cannot depend on room/client (room->net => cycle). No Clock trait in symbols; run_tick_loop (S17: elapsed time, 189) must take an injected clock for L1 to drive it.
- S4 npt-net carries quinn/wtransport/tungstenite (162-163); npt-client must build wasm32 (165-166). If client uses net's Link trait -> transport deps in Wasm. Link trait needs its own no-dep home or feature gating; client should be sans-I/O (bytes + host timestamps in, bytes out).
- S5 Symbols: no client-side transport adapters for native bots (WtServer/WsServer only, 224); bots need WT and WS clients.
- S6 Symbols: no Wasm export surface (the boundary: input call, step call, datagram in with host stamp, block out) though E7 claims ADR 0002 part 5/6 shape (194, 542).
- S7 E1 (188) "bots as protocol clients" listed in the sim crate: misplaced (bots are npt-bots on npt-client).
- S8 E17 (204) replay/re-decode tool in npt-judge: judge then depends on sim+proto; judge should read logs only.
- S9 E2 (189) panic="unwind" is profile-wide in Cargo; wasm32 client build in same workspace; plan silent on per-target profile.
- S10 Determinism (188, 616-617): only "seeded RNG, no clock reads, no transcendental math"; no hash-map ban (GLOSSARY 7), no float rules, no named stable hash for state_hash, no iteration order rule; U7 hash (542) has no scene/ticks/pass mark; comes after U4 L1 prediction rows.
- S11 E7 (194) + E21 (208) COOP/COEP + "the isolation flag" -> may pre-empt boundary spike (B2 vs B3); no word on generated types vs hand-written worker messages.
- S12 E9 (196) interpolation buffer in the Worker: how does the page get per-frame poses (call per frame?) vs ADR 0002 "page sees only the block".
- S13 "kept" (24, 648): server and harness stay on main; check ADR 0002 migration plan.
- S14 E10 (197) "samples timed at datagram arrival": who stamps in the browser (Worker host) not said in client; Instant::now on wasm32.
- S15 Wasm build only at U7 (542) though npt-client must compile wasm32 from U4; no per-unit wasm32 build check.
Next: B2 test-engineer digest B, C, G11.
### B2 done: test-engineer digest B, C, G read. Key facts:
- Harness shape (TE B:116 citing 0002:325-341, 369-373, 394-403): one input call + one step call per tick; datagrams carried in the input call each stamped with host performance.now() in its own message event; step returns when next tick due, host sets timer; catch-up <=7 per wake; fresh view of memory.buffer after every call; Worker owns connection online; page sees only the block.
- Left to boundary spike (TE B:118, SP:119; 0002:1119-1175,1249-1281): counting build, one copy per tick, B2 vs B3, generated types, isolation, page never handles a packet (PM-0002-22), busy page, size/tools, Sentry; PM-0004-8,-28,-61,-68..-70.
- BX (0002:1199-1205): netcode proof's throwaway sim "movement, hitscan, bots as protocol clients" in the Worker -> S7 is the drafts' own wording (E1 copies it): downgrade S7, but still: in npt-sim crate? note only.
- Import ban 0002:144-145, 210 (gate rule from foundation). "How kept is kept": not found (TE C:130). G11 REC: merge on main under spikes/, SHA in ADR 0001 results.
- 0002:239: crates "quinn and the WebTransport crate the netcode proof test picks". 0002:630-632 LG build is spike's own.
- PM-0004-67 design 0004:207-211: loop wakes on timer, due ticks from monotonic clock, catch-up <=0.25 s = 7 ticks.
Next: B3 ADR 0002 reads.
### B3a: ADR 0002 lines 125-424 read (parts 1-6). Facts:
- 0002:186 sim: content only, no transport, no async runtime, no clock.
- 0002:188 protocol: messages, codec, never-panic decoders, "the Rust side of the thin transport interface with its loopback backend"; no transport crate, no async runtime (builds for Wasm). -> plan puts the interface in npt-net beside quinn/tungstenite (162, 220-224): CONFLICT (S4 confirmed).
- 0002:189 room: match logic, no I/O, no async runtime, no clock (timers count ticks); bots fill slots through the input path. 0002:192 server: room process, transport, async runtime, supervisor hook, metrics. -> plan's npt-room (binary) merges room+server (225-226): CONFLICT (S2 confirmed); boundary spike needs room logic in Wasm for local mode (0002:387-403).
- 0002:190 client-endpoint: no bindings, no transport, no async runtime, no clock; prediction, reconciliation, interp buffer, decoding, aim cast, block writer + layout table; built for Wasm inside sim-wasm and natively for headless. 0002:191 sim-wasm: bindings only. -> plan's npt-client "native and wasm32" with no bindings crate named (165-166, 228): S6 confirmed; aim cast + block writer absent from plan's client symbols.
- 0002:193 supervisor links no sim/room code. 0002:194 devtools: replay runner, hash comparer, headless protocol clients. -> plan puts replay/re-decode in npt-judge (204) (S8).
- 0002:202-217 gate rules: room and client-endpoint build wasm32 and native, no async runtime, no transport, no wasm-bindgen, read no clock; sim one feature set on every target, cargo tree -e features compared across builds (feature unification).
- 0002:247-263 part 4: overflow-checks on for sim+protocol every profile, wrapping_/checked_ for counters; server unwind, Wasm keeps -Cpanic=abort; panic hook buffer read by host; baseline x86-64, never +fma, target-cpu=native, relaxed-simd; wasm-opt never --fast-math. Plan silent on all (S9/S10).
- 0002:279-373 part 5: block per tick (header, layout hash, sections), page posts newest input once per drawn frame (C1), Worker casts aim ray against the state it published for that view time; control API wasm-bindgen D1: create, mode flag, load content, one input call per tick, step; input call carries page input + each datagram with arrival stamp + time host sent previous step's datagrams + host current time; step returns where datagrams + block lie and when next tick is due; host sets timer from it; catch-up <=7 per wake; fresh memory.buffer view. Generated types: wasm-bindgen d.ts, own layout generator + layout hash checked at startup, ts-rs for protocol (G4 gate). Isolation OFF (G1); isolation flag only on profiling builds (G3). Worker owns connection (H2).
- 0002:387-407 part 6: same sim-worker build two modes (online / local, local server in Worker via loopback in protocol); local step order.
-> plan E21 (208) serves COOP/COEP headers from the page host by default: contradicts 0002:359-362 (isolation off; flag only for profiling builds). FINDING.
Next: B3b ADR 0002 part 7-8 (clock ban, test layers L1 placement), 515-521, 843-847, spike section 1112-1215, migration 1417-1484.
### B3b done: ADR 0002 425-560 (parts 7-8), 940-950, 1112-1240 (spike), 1417-1484 (migration) read. Facts:
- 0002:436-462 part 7: crates/sim/clippy.toml disallowed-types std HashMap, HashSet, Instant, SystemTime; disallowed-methods Instant::now, SystemTime::now, algebraic_*, every std f32/f64 method outside ADR 0004 part 7 allow-list; iter_over_hash_type backstop; own helpers for min/max/clamp; clock ban also in clippy.toml of room, protocol, client-endpoint, sim-wasm (std clock panics on wasm32-unknown-unknown -> trap); "only server reads Instant"; room timers count ticks. Plan's determinism rules (188, 616-617) adopt none of this mechanically. FINDING (S10 confirmed).
- 0002:433-436 clippy runs for host and wasm32.
- 0002:491-498 determinism suite: replay runner, per-tick state hash, twice in two processes + once as Wasm in Node; comparer reports first mismatching tick + field diff.
- 0002:507-514 L1 runs "the real server code and headless bot clients in one process" behind the thin transport interface. Soak/load bots on client-endpoint (0002:515-516).
- 0002:1113-1116 boundary spike shares harness + stand-in sim. 0002:1129-1135 mark 1: <=4 Wasm calls per tick online, same at N=12 and 48. 0002:1166-1167 mark 6: local match and online client run the same Wasm file and page code, differing only in the mode flag. 0002:1212-1213: "The 10-minute match: BX at N=48 in local mode (the Worker runs the local server and the client endpoint)". -> boundary spike needs the stand-in room logic in Wasm: plan's npt-room binary (tokio, threads, I/O) blocks it unless split (S2 -> High).
- 0002:1229-1233 Timers: production build (isolation flag off) 100 us; isolation flag build only for sub-ms reports. -> plan E21 COOP/COEP by default changes the configuration the netcode rows run in.
- 0002:1235-1238 counting build wraps every sim-wasm export -> needs a bindings crate with a small export set.
- Migration 0002:1418-1425: step 1 = spikes before foundation; nothing about merging spikes to main; foundation step 2.2 creates crates sim/protocol/room/client-endpoint/sim-wasm/devtools fresh; 2.5 ADR 0001 milestone "loopback backend with L1 behind it in CI". Kept-on-main is not contradicted but undefined: no end-of-life, no CI over spikes/ (plan 588, G9), import ban only from foundation G-checks.
Next: B4 ADR 0004 part 5 and part 7; GLOSSARY 7.
### B4a: ADR 0004 parts 4-9 read (185-361). Facts:
- 0004:185-187 part 4: u32 tick the sim's only clock; dt build-time constant; step takes no time arg; wrapping/checked ops; durations as integer sub-tick units.
- 0004:207-211 part 5: room loop wakes on timer, due ticks from monotonic clock, catch-up <=0.25 s = 7 ticks, longer stall dropped+recorded, overrun/dropped counters; "Snapshots go out after the tick". 0004:211-217 Worker: performance.now accumulator by setTimeout (W2), next-tick-due returned by step; host sets timer from it. 0004:225 "The loops read the clock; the step never does."
- 0004:257-262 part 7: Wasm==native identity is "a target ... that the netcode never depends on" (prediction corrected) -> but PM-0001 prediction marks (exactness) depend on it in this test: plan must adopt the float rules so a mismatch is a finding not noise.
- 0004:262-277 float allow-list (+-*/, neg, comparisons, total_cmp, sqrt floor ceil round trunc fract mul_add abs copysign, as casts); banned: powi hypot cbrt to_degrees to_radians min max clamp (own helpers), rem_euclid div_euclid %, transcendental via pinned libm only, no fast-math, no relaxed-simd, simd128 off; NaN never enters state (checked at entry; debug asserts finite each tick); "No sim value is computed in TypeScript"; baseline x86-64, no +fma, no target-cpu=native; glam scalar-math if glam; parry3d enhanced-determinism.
- 0004:288-293 ARM LG is not an ARM host; no verdict rests on AArch64 agreeing (matches plan's Arm rule 147).
- 0004:309-319 part 8: ChaCha8Rng pinned; seed drawn by host and passed in; never rand distributions; own sampling functions with test vectors; streams' state in the hash. Plan: "SimRng" only (213).
- 0004:338-355 part 9: dense arrays/slot maps iterated in id order; hash-map ban; canonical serialization (fixed field order, floats to_bits, fixed width LE, no usize; never std Hasher); fixed unseeded hash, "which function is picked in the sim core spike"; suite: native twice in separate processes, Wasm in Node, Wasm in browsers; first mismatching tick with field diff; injected divergences (std sin, hash-map iteration, presentation draw) caught at first tick.
-> plan U7 "native-against-Wasm hash" (542) has none of: recording, ticks, targets (Node vs browser), first-mismatch tick, bites. FINDING.
Next: B4b ADR 0004 spike section (S17, sim core spike scope, netcode proof refs), GLOSSARY 7.
### B4b: ADR 0004 spike rows read (980-1000, 1193, 1234-1256). Facts:
- 0004:1251 migration row 1: S17 recorded "from the netcode proof test"; units per tick and the hash function "chosen from the numbers" in the sim core spike.
- 0004:1255 slice 3: "the native room loop ... S17 ... on the real crate; the netcode proof test's marks rerun on the real sim" -> the real room loop is a slice-3 build, not the sim core spike's; sim core spike (0004:982-988) builds the sim only (bench on one vCPU, Wasm on ML). So plan's loop is not pre-empting the sim core spike, but the 60 Hz run will place the sim core spike's sim in this test's kept server: check SPIKE_PLAN 206-248 for the seam.
- PM-0004-67 (0004:1193): planted sleep on the tick thread at a fixed tick; start times from the loop's monotonic clock; bite: counting ticks from timer period.
Next: B4c GLOSSARY 7; SPIKE_PLAN item 1 + 60 Hz + boundary; ENGINE_OVERVIEW.
### B4c done: GLOSSARY 546-560 (state hash), 636-651 (isolation flag), 727-800 (s7 ban; s8 foundation, netcode proof test, 60 Hz margin). Facts:
- GL:637-642 the isolation flag is "off by default everywhere: local development, previews and production"; on only for profiling builds. -> plan E21 (208) "COOP/COEP headers" on the page host by default = CONFLICT (finding F-ISO).
- GL:547-550 state hash: fixed unseeded hash of canonical serialization; a client-vs-server compare covers only client-held state, leaving authority-only streams out -> plan's "client's decoded-state hashes" (E17) must define which subset.
- GL:729-735 hash-map ban: disallowed-types in crates/sim/clippy.toml + iter_over_hash_type backstop; BTreeMap/index Vec.
- GL:768 netcode proof test: "the Rust simulation runs natively as a server, and as Wasm in the client".
Next: B5 SPIKE_PLAN (item 1, 60 Hz run 206-248, boundary spike), ENGINE_OVERVIEW.
### B5a: SPIKE_PLAN 95-125, 170-290 read. Facts:
- SP:106-107, 181-183 the 60 Hz run uses "the proof test's kept server and harness"; SP:226-231 under OQ-0001-2 (a) "the sim core spike's sim runs inside the proof-test server" with S1 stand-ins (384 effect rows, 1 ms burned, remainder topped to 0.30 ms at 60 Hz). -> the plan's npt-room is hard-wired to npt-sim World (212, 225); no sim seam, no tick-rate constant, "7 ticks" literal (189) is the 30 Hz form (15 at 60 Hz: 0004:207-209). FINDING (hidden coupling, kept server).
- SP:255-258 boundary spike proves B2 vs B3, generated types, layout hash, isolation flag, local skeleton; SP:274-276 depends on "the netcode proof test's throwaway sim, client harness and room process"; SP:277-279 it builds first on the shared harness the CPU side of ADR 0003 part 8's measurement harness; "its result fixes the skeleton the foundation builds".
- SP:191-200 "It decides" list incl. R5-TEST-OQ2, OQ-0001-6, -7, rewind cap.
Next: B5b ENGINE_OVERVIEW skim; netcode digest E1/E7/E17 + H list for boundary points.
### B5b: ENGINE_OVERVIEW scanned (29-83: same crate rules as 0002); netcode digest E1-E21 (159-179), G, H20 read; ADR 0001 429-436 (thin interface: 3 backends WT, WS, loopback; outside sim; L1 plugs in behind it; sim thread never parses raw bytes), 595-625 (diagram, one server tick, one client tick incl. aim cast + block), 1295-1302, 1425-1435, 1473-1486 (timers: marks <=1 ms run with the isolation flag ON; crossOriginIsolated read in Worker and recorded), 1545 (-30 bite: wall-clock read on one transport's command path), 1559 (-41: 0 corrections >1 mm on CO; ML Chrome over L2 = Wasm client vs native server -> needs native==Wasm), 1694-1697, 1728-1734.
- Netcode digest E7 (165) had "host passes datagram arrival times in (endpoint reads no clock)" and "page posts newest input + cursor ray + view time per frame"; plan E7 (194) dropped both. Digest E17 (175) had "server state hash leaving authority-only streams out"; plan E17 (204) dropped it. Digest E1 (159) "authority-only spread stream"; plan E1 dropped it.
- Plan has no loopback backend, no block/layout/types statement, no net.ts in web symbols (230), no results-log schema home (written by room, bots, load, web; read by judge).
Findings list drafted next (B6).
### B6 findings (final list; ids EA-1..EA-14). Verdict: REVISE.
- EA-1 High 4.1/4.2/4.3: npt-room binary merges ADR 0002 room (no I/O/clock, Wasm) + server; blocks L1 import, boundary-spike local mode. Split npt-room lib + npt-server bin; say which checks are tick-counted (room) vs wall-time (server). Ev: plan 164, 189-198, 225-226; 0002:189, 192, 202-205, 457-459, 1166-1167, 1212-1213; 0001:620-623.
- EA-2 High 4.1/4.3: Link/Listener in npt-net beside quinn/wtransport/tungstenite/L2; no loopback backend; sync vs async unsaid. Move interface (poll-based, explicit timestamps) + Loopback to npt-proto (or leaf crate); npt-net native adapters + L2 only; add client adapters. Ev: plan 162-163, 220-224, 165; 0002:188, 0001:431-435, 1694-1695.
- EA-3 High 4.2 E2/E15, 4.3, 6.4: no home for the L1 driver + simulated clock; cycle if in npt-net; run_tick_loop reads monotonic clock. Add npt-lab (devtools analog), SimClock, fixed step order, S17 policy as pure fn tested in C with PM-0004-67 bites. Ev: plan 189, 202, 222, 225, 334; 0002:194, 507-514; 0001:1545 (-30 bite); 0004:207-211, 1193.
- EA-4 High 4.2 E7, 4.3, U7: harness not specified in part 5/6 shape; dropped digest items (arrival stamps, per-frame input+ray+view time); no bindings crate; no aim cast/block writer; no net.ts/host.ts. Ev: plan 194, 228-230, 542; 0002:190-191, 313-341, 369-373, 1129-1135, 1235-1238; NA digest 165.
- EA-5 Medium 2 (Out of scope), E7: what the harness leaves to the boundary spike not written; types (hand-written vs ts-rs) unsaid; B2 only, no B3, no counting build, online only. Ev: plan 128-130; 0002:342-353, 1149-1160; SP:252-258; TE digest B:118-119.
- EA-6 Medium E21: COOP/COEP served by default vs isolation flag off everywhere; flag only for marks <=1 ms (-44); crossOriginIsolated recorded. Ev: plan 208; GL:637-642; 0002:359-362; 0001:1479-1481.
- EA-7 High 4.2 E1, 13 risk 12: determinism rules for npt-sim missing (float allow-list, helpers, libm, NaN, hash-map ban clippy.toml, clock ban, build flags, overflow-checks, one feature set, ChaCha8 + host seed + authority-only spread). -41 ML leg needs Wasm==native. Ev: plan 188, 616-617; 0004:257-283, 309-319, 338-341; 0002:212-217, 247-263, 436-456; GL:729-735; 0001:1559; NA digest 159.
- EA-8 Medium U4/U7, E17: native-vs-Wasm hash: no method/pass mark, too late; client hash must exclude authority-only streams; hash fn is sim core spike's pick (label spike-local). Wasm build + clippy for wasm32 from U1. Ev: plan 204, 539, 542; 0004:342-354, 1251; 0002:433-436, 452-455, 491-498; GL:547-550; NA digest 175.
- EA-9 High 4.2 E1/E2, 4.3, 2 Goal: kept server not shaped for the 60 Hz run's sim swap (OQ-0001-2 (a)); literal 7; tick rate not a build constant; stand-ins 2.80 ms only. Ev: plan 24, 117-118, 188-189, 212-213, 225; SP:226-231; 0001:1297-1299, 1428-1431; 0004:185-188, 207-209.
- EA-10 Medium 2, R-14, 12.2: "Kept" undefined vs ADR 0002 migration (import ban only from foundation G-checks; crates written fresh in 2.2; no CI over spikes/; no end of life). Ev: plan 24, 588, 648; 0002:140-146, 209-210, 1422-1434; TE digest G11 (C:130).
- EA-11 Medium E17/E20/4.3: results-log schema home; replay tool in npt-judge (upward deps). Add npt-log leaf; replay in npt-lab. Ev: plan 204, 207, 229, 261-262; 0002:194.
- EA-12 Medium 4.3: symbols list too thin (per-crate list of missing items; bite switches; pub(crate) rule). Ev: plan 211-231 vs 186-209.
- EA-13 Low 4.1: npt-super / npt-host inside crates/server; supervisor links no sim/room (0002:193). Own crates, no sim deps.
- EA-14 Low E1/E2: "bots as protocol clients" listed under crate sim (188); panic strategy per target (Wasm abort + hook buffer, 0002:250-256) unsaid.
Picks: R-8 agree (container), tool in npt-lab; R-14 agree with EA-10's Kept subsection; R-22 any fallback stays below Link so no crate above npt-net changes.
DONE
