Task: netcode-architect design notes for change-set 4 "Walk" (Slice 1), plan input for the main session (Builder V4, lane 1), 2026-11-25.

## Plan (blocks, in order)
0. Read the inputs (build plan card and Part 2/3 rows, cs3 plan and my cs3 notes and reviews, STATUS/ARCHIVE 3c items, handover part c, reports, ADR 0001/0002/0004 parts, merged code, lane 2 list). Source summaries go in the "Reading notes" section as I go.
1. The recording as Walk's step 1 (runner, comparer, format, hash on the players' file, marks with tests and planted defects).
2. Every item named for 3c, placed by name (two homes table).
3. The command, the snapshot and the path for Walk (Input movement+aim, wire layout, sizes, room apply, soldier record, quantisation, hash coverage, client-endpoint decode, interpolation, aim ray, SLICE-FEEL wire, lane 2 rows).
4. Two follow-ups: 3B-ONE-PER-TICK, 3B-RESUME-FLASH (question with options).
5. Boundary and pacing marks at change-set 4 (PM-0002-14, -26 smoothness, -29, -31, -34 walking steps, PM-0004-28, PM-0003-42).
6. Sizes per step, code and test lines apart, with plus or minus; first part vs second part.
Then DONE.

## Notes log (one line per finding / per finished block)

## Reading notes (block 0)
- R-BP (build plan docs/reports/2026-11-17-build-plan-slice-1.md): Walk card :334-358 (content crate :335-338; sim soldier table, movement on parry3d, time unit from rounding report, jump at tick, OQ-0002-6 :339-342; path :343-344 "the command with its aim point (the Worker casts the page's cursor ray), soldier fields in the snapshot, the block's soldier section, interpolation between the two newest blocks on the page"; measured :353-354 "identical hashes natively, in Node and in Chromium on a movement recording; movement against its data (step, apex, no tunnelling); the collision tree byte for byte; ids over random spawns; even motion on your display"; cut-first items if near 2x :355-356: the far-corner 2 km test map (PM-0004-39) and the size comparison; camera panel stays). Cs3 card :316-332 placed replay runner, comparer, L1 in devtools at 3 (:321-323). Part 2 rows: :466 ADR0001 milestone headless half at 3 "L1 behind the loopback"; :470 ADR0004 milestone first recording per-tick hashes natively twice + Wasm, shown at 3; :473 ADR0004 foundation line: replay runner + first recording in CI at 3; :474 PM-0002-49 L1 same seed same packet-fate log and hashes, loss/delay/jitter within 5 % over 1,000,000 packets per profile, at 3; :484 G10 recordings "4 on (a recording per change-set that changes the simulation); the set fixed in 12"; :485 ADR0001 :1785 recorded input replay same per-tick hash on two runs, at 4; :492 PM-0004-15 at 4; :493 PM-0004-30,-32,-35 soldier pairs,-39,-33 own size at 4; :494 determinism suite PM-0004-19 on slice recordings "4 on: natively twice, Node, Chromium". Part 3.1 :538 two native processes with different hash-map seeds, Wasm in Node and Chromium. 3.3: PM-0002-14 at 4 "(the aim in the post)" :586; PM-0002-26 4 smoothness :594; PM-0002-29 4 with content :597; PM-0002-31 4 :599; PM-0002-34 step 4 even walking, tab hidden 1 and 6 min :602; PM-0004-28 4 camera still and moved, 60 and 144 Hz :603; -68..-70 as -26 :605. 3.4: PM-0004-6 4 zero alloc in step counted :613; -11 4 :617; -15 4 :621; -17 "3, meaningful from 4" :623; -18 3 :624; -19 3 then every recording from 4 :625; -20 4 :626; -21 4 the three planted divergences :627; -22 run again when a function joins (4) :628; -24 3,4 :630; -25 4 reading with SIMD on :631; -30,-32,-33,-35,-39,-54,-59,-60 at 4. PM-0003-42 at 4 "smoothness per frame-cap option" :714. Cs7 card :386-397 (prediction, scheduler seam both timings, L1 fake lag); cs8 :399-413 (weapons, hitscan, events, own weapon state predicted).
- R-ORDER (mustafa-2026-11-25-order-3c-answer.md :8-13): (c) split; recording = replay runner, comparer, state hash on players' file, recorded match -> Walk step 1; link simulator own small cs after Walk, merged before cs7 starts. Conditions a (no thinner proof; every mark the plan places at Walk on the recording met before Walk merges), b (recording built and green before any movement code rests on it), c (every 3c item placed by name), d (fallback: recording alone as small cs before Walk if Walk cannot carry it inside stop line). :14 plan sized at 1.8x with stop line; earliest playable point; one stop or two. Question file :91-94: state-hash export goes with recording; reviewers' L1 items stay with L1; 3B-HOSTILE-ALLOC placed by whichever part comes first. Question :55-57 cs3 plan sizes: recording 275 lines, L1 595 lines.
- R-TAKEOVER (mustafa-2026-11-25-takeover.md :20): cs4 plan names the built-file check for fused multiply-add (MUL-ADD), 3B-RESUME-FLASH (his decision), Testlines values (:22-29; game at 30 steps/s :28; one home per value :29).
- R-DEC (docs/DECISIONS.md): RELEASE-COUNT (a) :232-257 (press counters only; releases worked out from presses and held keys; "No release counter is added in change-set 3c"; follow-up 3B-RELEASE-DRAFTS). PRESS-PACE (a) :1025-1026, :1068 (second tap inside one tick lands one tick later, nothing lost). RNG-SOURCE (c) own ChaCha8 :1024-1025. DEPART-3 (a) :1026. MUL-ADD (a) :553-589: fused multiply-add banned in sim (writes a*b+c); "The check on the built file (no fused multiply-add inside the players' WebAssembly) is built in the same change-set that brings the first outside crate into the simulation (the collision library, planned for change-set 4) ... That change-set's plan names it." :587. Command timing: Slice 3 picks; cs7 builds both ways :498-500. SLICE-FEEL :1227-1231: hold a key to aim; start from old game's rules as Testlines has them: walk 0.55 of speed, cannot sprint, spread rests at sights value, camera reacts; coming out of a sprint, firing and ADS each wait a short delay set per gun; starting values from Testlines export; "Price and place both in change-set 4's plan: the input, the rule, the fields, what lands in change-set 8, and the lines they add". :1235 W1 (ADS flag) and W4 (how far the sprint-to-fire swing has got) are needs. AIM-STATE (a) :1298 (poses: aiming pose whenever not sprinting, sprint pose while sprinting; "A first shot never leaves a gun that points away from its target"). START-RULES :1726 (jump reach 3.07 m; soldiers push softly, never block; changeable after Brave).
- R-CS3PLAN (git show 80a51e42:.checkpoints/cs3b-loop-closes/cs3-plan.md): step 7 (devtools replay runner one hash per tick, comparer first differing tick shown on one planted divergence, first recording 1,800 ticks under 58 KB data, replayed natively in three processes and in Wasm in Node and Chromium on the players' file; files crates/devtools/src/replay.rs, compare.rs, crates/devtools/recordings/first-loop.rec, packages/sim-worker/boundary/sim-wasm.test.mjs; 275 lines). Step 8 L1 = crates/devtools/src/link.rs, tests/loop_over_l1.rs, tests/l1_profiles.rs, 595 lines. Section 3 item 5: PM-0004-21's short report (which hash function, cost per tick) written by main session with rust-reviewer BEFORE the first recording is committed ("a recording pins the hash"). 5.1: input = six plain numbers (post number, held bits, four packed press words); movement and aim fields + clamp + tests "enter there" (cs4, about 40 lines); creation new(seed, mode, pad), first step(now_ms) sets anchor; replay in Wasm on exact players' file via input, step and ONE new read-only number (state hash), no byte-block call (WB-HEALTH (a) "one type, plain numbers on every tick"); not in test-only module (ADR 0002 line 509: determinism on exact shipped file); 64-bit hash crosses as two u32 or one BigInt read only during replay. Section 9: PM-0004-19/-17/-18/-24 "begun"; -17's two-instance run, -18's 36,000 ticks, -24's finite-state read come with the determinism suite (cs4 on); PM-0004-21 "the report and one of the three plants; the rest with the suite". DEPART-3 list includes the input message up to about 277 bytes vs 200 per datagram (compression before online play), limits as code until content crate (cs4), JoinRejected fifth message, state-hash read-only number.
- R-MYCS3 (git show 80a51e42:.checkpoints/cs3b-loop-closes/netcode-architect-cs3.md): my cs3 design: replay.rs in sim-wasm with Recording {seed, start_tick, posts Vec<Post>, hashes Vec<u64>}, format magic "SHMBLREC" 8 | version u16 | seed 32 | start tick u32 | tick count u32 | per tick post 24 B (post number u32, move_x, move_y, buttons u16, presses 16) | per tick hash u64; MAX_RECORDING_TICKS 108,000; decoder caps before allocating; devtools run_file + compare first_difference(start_tick, expected, got) with Difference{tick, expected, got}; "Field-level diffs (ADR 0004 :352) wait for state with fields (cs4)". T7: written with SHAMBLE_WRITE_RECORDING=1, scripted posts, public test seed 0x00..0x1F, start tick 0, 57,650 B; natively in each of the two workspace runs and the release run; Node via boundary/sim-wasm.test.mjs; Chromium via Playwright; precondition PM-0004-21 report. replay_hashes(bytes)->Vec<u8> export was REJECTED by cs3 plan 5.1 (no byte-block call; input+step+one read-only hash). L1 design (link.rs, SplitMix64, five draws per datagram, profiles P0-P4, B1, CLEAN, DUP, FateLog FNV-1a, T1-T6) all still unbuilt -> goes to the L1 change-set. Findings F-P0, F-B1, F-JITTER (PM-0002-49 reading) belong to L1 change-set. T9 decoder property tests; allocation by pointer/capacity, counting allocator needed unsafe (later built in 3b as "a counting allocator of our own" per Mustafa's 22:44:50 point 3a). Block 5 #6: Input whole (U1) about 277 B at cap vs 200 B mark; U2 and counter width (R2-XU-OQ4, PM-0001-51) before online play. #7 caps as Rust consts until content crate (cs4).
- R-NAREV1 (80a51e42 netcode-architect-review1.md): F1 Medium (endpoint late vs refused) fixed as NA-1; F7 Low "(3c) L1 needs a clock handle (no link accessor) and an exactly-once reliable channel (local_match.rs:171-177)" -> L1 cs item. F5 traps (acks bounded only from above, two per group; press check vacuous at 255 covered; narrow counters wrap on long gaps).
- R-NAREV2 (80a51e42 netcode-architect-review2.md): L-1 Low any_bytes.rs cheap checks missing (per-step identity last_processed moved by run+missing; endpoint every datagram = received+late+refused; each ack == newest taken; each repeat == first sent) + nit :376 vs :380 echo model; L-2 Low queue.rs:113-115 ran(sequence) trusts caller -> record last taken + debug-assert, or pin in cs7 seam test; L-3 Low Stale doc wording queue.rs:26, lib.rs:59; L-4 Low follow-up wording: presses LOST past 255 ticks of outage. Point 6: L1 models corruption as loss only (RFC 9001 6.6, RFC 8446 5.2), RR-3 endpoint plausibility bound with the client's server-time estimate (cs7); in lockstep: newest taken + own ticks since + 30. Point 5: M1 bank bound at cs7's scheduler seam (Source sv_maxusrcmdprocessticks bank); PM-0001-21 long-hold case at next ADR edit. Observed: every honest command ran late under jitter; ~1/3 ticks ran two commands (cs7 scheduler).
- R-EAREV2 (80a51e42 engine-architect-review2.md): Q7 for 3c: L1 needs link() accessor (link private, local_match.rs:38) for fate log; time to L1 through LocalMatch (one TransportPair method, no-op default for loopback) fed from recorded times, never a clock; L1 loss/rate-limit drop silently (not SendError) else they land in block refused; L1 duplicates datagrams only (second JoinAccepted trips local_match.rs:150-153); corruption as loss. "Recording: record now_ms per tick." L3 (world.rs:127-133): export returns SHARED hash only, plain numbers read only in replay (one BigInt or lo/hi), never bytes, never on live tick; buffer private to Room (Vec reserved in Room::new behind Room::shared_hash(&mut self)), not in Sim nor LocalMatch; at source shared_hash writes prefix only (write_shared_state; write_state = shared + seed); test scratch.len() == SHARED_STATE_BYTES after call. Lows: any_bytes.rs halves never meet -> move to room/tests and client-endpoint/tests or fix the sentence; two `unsent` names (endpoint.rs:74 vs local_match.rs:43); bindings' input doc (sim-wasm lib.rs:118-120) one sentence; olderPosts across a second ready (worker-link.ts:229, latent). LocalSchedule and loopback unsent rightly stay in LocalMatch (local-only).
- R-RRREV1 (80a51e42 rust-reviewer-review1.md): Low devtools/Cargo.toml:14-16 client-endpoint, protocol, room declared unused (serve steps 7-8, now 3c); Low netcode: decodable corruption -> model as loss in L1 or plausibility bound; size cut of the two {:?} panics (-3,121 B raw); Low CI: no clippy step, room/endpoint fuzz loops (~4 s release) could become crate tests; #[must_use] gaps; pub mod wire / pub mod queue unused externally; cross-target evidence: block bytes FNV over 1M ticks equal natively and Wasm in Node (d4f18fe53057837e); speed native ~400 ns/tick, Wasm ~540 ns per input+step.
- R-SAREV1 (80a51e42 security-auditor-review1.md): M1 Medium bank bound (deferred to cs7 seam; Source S1 sv_maxusrcmdprocessticks); L1 Low room/lib.rs:165-166 + :199-206 deferred checks not stated where they go: "move length and aim: enter with the record's movement, change-set 4; view time: lag compensation; edges vs cooldowns: weapons; rate, size, Join deadline: the connection layer, Slice 3; the ack's lower bound" -> cs4 room must check move length and aim; L2 devtools' three unused path edges: leave manifest, say so in report (a change reopens the batch); L3 Sim.prototype pin is the test that catches a state/seed accessor joining the players' file ("3c adds the hash export"); online driver must count and close on second JoinAccepted. Threat-model line: cumulative press counters -> honest client after outage applies up to 255 presses in one tick: every system that reads edges applies at most what its own cooldown allows per tick (= 3B-ONE-PER-TICK). Seed never leaves Worker's Rust state: no message, block field, log, panic text, Debug, whole-state hash/bytes to the page.
- R-VER3B (80a51e42 verifier-cs3b.md): own plant room-alloc-hostile (Room::on_datagram decode-error branch allocates) NOT CAUGHT (step_alloc honest posts only; any_bytes counts no memory) = 3B-HOSTILE-ALLOC; boot-no-blur NOT CAUGHT (no spec blurs window); facts: world SHARED_STATE_BYTES 22, STATE_BYTES 54 (asserted world.rs:217); block 44 bytes (8+20+16); step_alloc 18,000 ticks 0 allocations; Wasm 53,813 B raw; CI toolchain 4 min 58 s of 15; browser 16 tests 3.3-3.4 min of 5 min limit; key-to-tick median ~240-265 ms p95 ~258-326 ms (a reading; scripted).
- R-STATUS (docs/STATUS.md): Next item 2 :33-36 "Then change-set 3c (new): the link simulator and the first recording, moved out of 3b by the size ladder, with the state's hash on the players' file and `devtools`' three path edges; its plan holds the reviewers' items (the archive, "From change-set 3b"). Then change-set 4, the first outside crate in `sim` (its plan names 3A-FMA-CHECK)." Follow-ups 3b :99-103 (3B-LATE-BOUND, 3B-PLAUSIBLE for 7; 3B-ACKS, 3B-OUTAGE-256 before online; 3B-ONE-PER-TICK for 4, 8, 9; 3B-ALLOC-METHOD for 5; 3B-KEY-POST, 3B-SATURATED for PM-0002-13/-8; 3B-REFUSED-FIELD Slice 3; 3B-LONG-HOLD, 3B-OQ6-POINTER, 3B-RELEASE-DRAFTS his; 3B-PACER-255 design; 3B-RESUME-FLASH for 4; 3B-OTHER-BROWSERS CI PR; later 3B-POOL-16, 3B-CSS-PROD, 3B-PUB-MODS, 3B-BLUR-TEST, 3B-HOSTILE-ALLOC; 3c's plan items). 3a :94-98 (3A-FMA-CHECK his: first outside crate).
- R-ARCHIVE (docs/STATUS_ARCHIVE.md :1453-1557): 3B-RESUME-FLASH :1478-1483 (his, open, no letter; comes back with cs4; Brave once a key box lit a split second after return; code: match stands still while hidden, box shows server's last echo, first tick after return lets key go; he wrote not an issue). 3B-ONE-PER-TICK :1513-1515 (cs 4, 8, 9 each with a test; at most own rule per tick; written on sim::PlayerCommand). 3B-BLUR-TEST :1540-1542 (with next change to page's key handling: browser test holds W, sends blur, expects W let go). 3B-HOSTILE-ALLOC :1543-1546 ("with 3c's tests": counting allocator in the two random-message tests). 3B-POOL-16 :1520 (once the block grows: pool 4 -> 16). For 3c's plan :1547-1557: (i) L1 clock via LocalMatch from recorded times (NA-7, EA r2); (ii) reliable channel exactly once (NA-7); (iii) loss and rate limit as silent drops, duplicated datagrams only (EA r2); (iv) corruption as loss only (NA r2, RFC 9001 6.6, RFC 8446 5.2; RR-3); (v) link() accessor on LocalMatch; (vi) the host's time recorded with every tick; (vii) state-hash export returns shared hash only, plain numbers, read only during replay, buffer private to Room (EA r2); (viii) `late` checked against the simulator's own reorder count (NA r2); plus the state's hash on players' file (DEPART-3) and devtools' three path edges (SA L2 = EA-6).
- R-HANDOVER (git show origin/claude/loving-hopper-6p0mg2:.checkpoints/handover-builder-v3.md, part c): 3c holds L1 behind transport interface (seam LocalMatch<P: TransportPair = Loopback>), first recording, one new read-only number (state hash) on players' file, devtools' three path edges; named: 3B-HOSTILE-ALLOC (counted no-allocation for hostile bytes in room AND endpoint, with 3c's tests); RELEASE-COUNT (a) no release counter in 3c. Not 3c's: 3B-LATE-BOUND, 3B-PLAUSIBLE (cs7), 3B-ACKS, 3B-OUTAGE-256 (before online). Part a: wasm-bindgen 0.2.130 must be fetched; browser suite 16 tests 3.4 of CI's 5 min; toolchain job runs no lint. Part b: size 6,909 vs 3,800 planned (1.82x).
- R-BROWSER3B (docs/reports/2026-11-25-cs3b-browser-check.md): Brave/Firefox both 0 lost/doubled/refused/errors; key to tick p95 44.8 ms (Brave, 134 fps) and 49.0 ms (Firefox, 180 fps) vs 50 ms mark (reading); RESUME-FLASH reading :38-48: match stands still while hidden (0 ticks while hidden); box shows server's last echo; Worker lets go of every key when tab hides; first tick after return carries that; picture from before the pause can show until that first tick (~1/30 s) when first frame after return drawn before first block arrives; asked: leave it or make boxes go dark the moment the tab hides; he named no letter, "not an issue". Worker lateness median 1.967 ms p99 14 ms (Brave).
- R-REPORT3B (docs/reports/2026-11-25-cs3b-the-loop-closes.md): :62-79 what changed (protocol five messages; sim match start tick, command with held buttons + press counters, state hash tick+player+seed, shared hash leaves no seed in buffer; room one slot, checks, queue window follows last run, at most two per tick, snapshot with echo; endpoint pacer, repeats, block 44 bytes with echo and link counts; sim-wasm input six numbers + step). :116-121 any_bytes properties. :122 warm tick 0 alloc in Rust (18,000 ticks counting allocator). :125 Wasm 53,813 B. :226-237 OVERFLOW-PANIC-RULE: protocol, room, endpoint/post/press deny arithmetic_side_effects; CI runs no lint step -> "every change-set that touches room, protocol or a server crate runs the lint by hand and says so in its report". :319-323 not verified: anything over a link that delays, drops or reorders (3c). Risk 2: plan 3c and cs4 at 1.82x.
- R-ADR1 (docs/adr/0001-multiplayer-netcode.md): part 4 :203-209 own-state section carries what replay reads (position, velocity, weapon state, effect rows), "sent unquantised, or the server quantises its own copy the same way each tick"; correction fade 100 ms, snap 2 m. Part 5 :220-268: command = tick, sequence, move vector, button bitfield, press counter per button, quantised world aim point (at most 80 bits), view time (interpolation tick + fraction); checks :231-239 incl. "the move vector's length is at most 1", "the aim point lies inside the map bounds (a point beyond the weapon's reach is clamped, not refused)"; aim :255-268: page posts newest input once per drawn frame with cursor ray from the camera and the view time of the frame it drew; Worker casts the ray with the sim crate's ray query against the static world AND the interpolated remote entities it published for that view time, quantises hit point into the command; server rebuilds ray from its own fire origin (playerFireOrigin rule, GC5) toward that point; top-down cursor ~28 m from look point, 44 m with ADS pan. Part 6 :276-330: E3 positions 1 cm in 16,16,12 bits (+-327.68 m, 0-40.95 m height), yaw 16 bits, pitch 10 bits; remote soldiers :309-313 position, yaw, pitch, health, flag bits for stance and action states (crouch, sprint, aiming down sights, reloading, carrying a cache), weapon id; action starts are tick-stamped cosmetic events; 90-bit full / 60-bit delta soldier records, 8 flag bits; entity id = (kind, index, generation), generation in the record that brings the entity in; generation change = removal + addition (place, not slide); header full 32-bit tick, other ticks 16-bit offsets; Worker decodes wire into its own state and writes the block; no state hash exchanged with clients. Part 7 :331-349: Worker publishes already-delayed remote poses per block; page interpolates between the two newest blocks; trails newest block by up to one tick; that fraction is inside the view time the page reports; delay 3 intervals (100 ms) within 2-5; late snapshot extrapolated at most one interval then held; respawns/teleports placed. Part 8 :366-367 half a tick for the page's blend. Catalogue :682-686. Bandwidth :718-731 command ~15-20 B; mark 48 kbps IPv4 = 200 B per datagram (48,000/8/30). Migration :1783-1786 Slice 1 done-when: "a recorded input replay gives the same per-tick state hash on two runs, and R2-SYNC-REC2 marks 1-2 and R2-SYNC-REC3 marks 1-3 pass on the loopback under L1's profiles". Test plan :1819-1822 determinism by state hash; "decoded snapshots rebuild the server's quantised state by hash (R2-SYNC-REC1 mark 1)"; :1823-1827 counter wraps test.
- R-ADR1-MARKS: PM-0001-17 (:1605) every decoded snapshot rebuilds the server's quantised state by the client-held state hash (L1, 48 headless clients; bites: 16-bit header tick, client quantiser rounding differently from server's). PM-0001-51 (:1648) aiming at height: server's hit point within 5 cm of client-picked point in >= 99.9 % at ground, 2.5 m, 4 m under P0-P3; "the aim at most 80 bits per command before delta coding"; bite: server ray built parallel to client's (direction) with planted 10 cm fire-origin error. PM-0001-13 (:1601) upstream <= 48 kbps mean, real command size whole and delta-coded reported.
- R-ADR2-P5 (docs/adr/0002-code-structure-build-toolchain.md part 5 :284-391): block header (tick, layout hash, counts per entity kind), aligned sections per entity kind, owner-only section, event section; page binds typed views, keeps the two newest blocks to interpolate between ticks (R1-TICK-REC5); block writer serializes field by field, fixed-width LE, sections aligned to largest element; one layout table drives writer, generated offsets, layout hash. Post :318-329: sequence number, move input, held-button bitmask, press and release edge counters as running totals (exact fields ADR 0001's), plus camera's cursor ray and view time of the frame drawn; Worker takes newest post, casts its ray against the state it published for that view time, fills the command's aim point; "The Worker stamps the tick it applies the input to". Control API :330-346 (one input call + one step call per tick; <= 4 Wasm calls per tick incl. glue). Content files in dev :359-363 (Vite plugin pushes changed content files to page -> Worker; PM-0002-31 measures). Part 6 :392-414 local step order (endpoint builds T's command incl. aim cast, predicts T; loopback; local server runs T, encodes T's snapshot; endpoint decodes, reconciles, writes block); "no code path exists only offline".
- R-ADR2-MARKS: PM-0002-14 (:1285) "10,000 of 10,000 posts carry the cursor ray and the view time of the frame drawn, and every network command its aim point from the newest post; the Worker receives 0 messages of any other kind from the page during play"; CI counts; PM-0002-12's run without the shim; Worker counts message kinds; bite: a separate aim message. PM-0002-26 (:1299) = PM-0004-68..70: jitter p99 <= 4 ms over 18,000 ticks; constant-speed unit's per-frame on-screen displacement std <= 5 % of mean on 60, 120, 144 Hz; hidden 60 s and 6 min: no tick ran while hidden, resumes where stopped; never > 7 ticks in one frame; smoothness on RD (Chrome, Firefox), ML, MAC; scripted mover at constant speed, page logs drawn position every frame; 3 runs of 60 s per display and browser; bites: ticks per timer callback, newest tick drawn without interpolation, Worker simulates hidden gap. PM-0002-29 (:1302) Wasm weight reported (raw, gzip, brotli, wasm-opt, opt-level s/z, lto, cgu, with parry3d), gated < 25 MiB. PM-0002-31 (:1304) content file reaches Worker within 1 s of save, 20 of 20 saves per browser (Chrome, Edge, Firefox on ML); whether Vite's hot update reaches the module Worker recorded per browser. PM-0002-34 (:1307, checklist :1409-1420) step 2 "Watch the marked soldier walking at constant speed for 30 s... moves evenly", steps 3/4 tab hidden 1 min / 6 min "exactly where he left it ... carries on smoothly". ADR 0002 :506-512: replay runner "built as the server builds the sim", gate runs it twice in two processes and once as Wasm in Node; Playwright replay page runs the sim-wasm Worker in Chromium/Firefox/WebKit returning the hash stream; every Wasm run uses the exact shipped file (checked by hash); comparer reports first mismatching tick AND a field-level diff; gate set = three short recordings (two teams, three teams, bots only); nightly 10-min bot match at N=48; rules and recordings are ADR 0004's.
- R-ADR4 (docs/adr/0004-simulation-core.md): part 1 :131-147 fixed-slot soldier table sized from mode; pools fixed capacity at match start; phases fixed order. Part 2 :153-156 id (kind, index, generation); soldier's slot index = sim index (GC2). Part 3 :161-177 snapshot/restore (hand-written clone_from, no alloc), prediction step = own movement etc., draws from no stream. Part 4 :185-201 u32 tick only clock; durations in seconds -> integer sub-tick units, one tick exact whole number, rounding reported, remainders carried; units per tick from the rounding report. Part 5 :207-225 catch-up 7 ticks; hidden tab: local play pauses. Part 6 :249-251 step makes no heap allocation in steady state. Part 7 :257-299 same-build reproducibility required; Wasm-native identity a target, tested on every change, netcode never depends on it; f32 sim; allow-list; NaN never enters state (inputs checked finite where they enter; debug/test assert finite each tick); no sim value computed in TypeScript; FMA risk from dependency SIMD; parry3d enhanced-determinism on both targets; simd128 off until suite passes with it on. Part 8 :309-337 seed never sent/logged; recorded match keeps it; no recording reaches a client while its match runs. Part 9 :338-356: canonical serialization of the whole state after each tick (fixed field order, floats by to_bits, streams' state, tick counter), fixed width LE, no usize; one unseeded hash function; suite: recorded match (seed, map and mode data, content hash, per-tick inputs) natively twice in separate processes, Wasm in Node, Wasm in Chrome/Edge, Firefox, Safari; "first mismatching tick with a field-level diff of the two states"; injected divergences (one std sin, one hash-map iteration, one presentation draw inside the sim) caught at first tick it changes state. Part 10 :362-380 parry3d pinned, enhanced-determinism; our own kinematic slide-and-step; static world BVH built once at match start in id order; movers' index rebuilt each tick.
- R-ADR4-MARKS (:1130-1160): PM-0004-13 restore and continue (cs7). PM-0004-15 stale id never resolves: proptest on pools, 10,000 cases, pool 64, <=1,000 ops; bite: no generation raise (cs4). PM-0004-16 client maps server ids with no table (cs7). PM-0004-17 two instances in one process stepped in turn with different seeds = each solo run; 0 mismatches over M10; CI native + Wasm in Node; bite: a static counter read by a phase. PM-0004-18 same seed + input log in two native processes, 36,000 of 36,000 ticks, each process logs its std HashMap seed and the two differ; bite: -21's injections. PM-0004-19 bots-only 24v24 and 12v12v12, 18,000 ticks x 3 seeds, native x2, native vs Wasm in Node, Chromium/Firefox/WebKit, Safari, ML once (Slice 2's form; at cs4 on Slice 1 recordings). PM-0004-20 parry3d flag: 0 mismatches native vs Wasm over SS 18,000 ticks; query time with flag on/off reported (rust-reviewer's). PM-0004-21 three injections (std sin with an input where std sin differs native vs Wasm else 1-ulp plant; std HashMap >= 32 keys whose order decides an outcome, two orders logged; presentation random draw inside the sim) each reported at its arming tick T with field-level diff 3/3; hash function and cost per tick reported; same digest for a committed byte string on x86-64 and wasm32, one committed state serializes to same bytes; control: presentation draw outside sim passes; bite: planted usize field changes bytes between targets. PM-0004-22 libm and sqrt vectors (run again whenever a function joins). PM-0004-24 0 non-finite values in state over every run of -13, -18..-21, read by the replay runner itself on release builds (reads every float by to_bits); finite assertion in debug/test; fuzzed commands' NaN rejected where it enters; bite: planted 0.0/0.0 in a phase trips the assertion at its tick. PM-0004-25 suite with simd128 on: 0 mismatches lets it on (decision row; at cs4 a reading). PM-0004-28 presentation never reaches the sim: in one recorded match, identical per-tick hashes for sound on/muted, camera still/moved, display 60/144 Hz; RD Chrome; page replaying one recording, hash stream read from the Worker; bite: a sim draw from the page's presentation generator.
- R-CODE-SIM (crates/sim/src): world.rs TICK_HZ 30 :8, SEED_BYTES 32 :11, BUTTONS 16 :14, SHARED_STATE_BYTES = 4+2+16 = 22 :18, STATE_BYTES 54 :21 (private); MatchSetup{seed, start_tick} :25-30 no Debug; PlayerCommand{buttons u16, presses [u8;16]} :38-42 ("A command's move, aim and view time join when a system reads them" :33; one-per-tick rule doc :35-37); World{tick, player, seed} :53-58 (player "the soldier table takes its place" :55); authority_step(&[PlayerCommand]) newest becomes record :90-98; write_state :105-121 (destructuring pattern forces new fields; order tick, buttons, presses, seed; path form u32::to_le_bytes for the rule scan :115-117); state_hash :126-129 fnv1a_64; shared_hash :135-140 writes whole then zero-fills + truncates (EA r2 wanted prefix-only writer). Known answers :228-250 from a second implementation (script). state_hash.rs fnv1a_64 :15-21 provisional "PM-0004-21's report ... confirms or replaces it before the first recording is committed" :5-7. lib.rs :10-13 re-exports; rng.rs (ChaCha8).
- R-CODE-PROTO (crates/protocol/src): messages.rs: PROTOCOL_VERSION 1, BUTTONS 16, REDUNDANCY_CAP 8, MAX_ACKS 4; Command{tick u32, sequence u16, move_x i8, move_y i8, buttons u16, presses [u8;16], aim AimPoint{x i16 cm, y u16 cm height, z i16 cm} "Provisional as a whole ... the codec sets them under PM-0001-51. The encoder clamps to the map", view_time ViewTime{tick u32, fraction u16 (1/65,536 tick)}}; OwnState{buttons u16, presses [u8;16]} "the soldier's own state joins with the soldier"; Snapshot{server_tick u32, baseline_tick Option<u32>, newest_command_received Option<u16>, last_processed_command Option<u16>, group u8, own Option<OwnState>}. codec.rs :7-18 table; command fields 32 B; Input at cap 292 B (MAX_INPUT_BYTES :44 = 1+1+4*5+1+6+32+7*33); Snapshot 0x11 flags bit0 baseline, bit1 received, bit2 processed, bit3 own; MAX_SNAPSHOT_BYTES :45 = 33; decode_snapshot(bytes)->Result<Snapshot> :364 (returns, not into); put_fields :206 / read_fields :218. wire.rs :3-4 "a bit reader joins with the first field that is (the snapshot's entity sections)" - byte reader only today.
- R-CODE-ROOM (crates/room/src/lib.rs): deny(unwrap/expect/indexing/arithmetic) outside tests :8-16; LEAD_MAX_TICKS 8 :37; CATCH_UP_CAP 2 :40; MODE_ID "skeleton" :43; RoomCounters :47-68, refused() :73-79; Room{world, content_hash, joined_at, queue, input, last_run, newest_received, newest_sent, counters} :87-100; on_control :135-166 (JoinAccepted slot 0, mode_hash zeros); on_datagram :176-224 doc :170-175 lists deferred checks: "the length of the move and the aim inside the map (the soldier and the map, change-set 4)"; builds PlayerCommand{buttons, presses} dropping move/aim/view :209-216; admit :229-245 (strictly later tick; press_edges <= covered); step :251-294 runs <= CATCH_UP_CAP admitted due commands then world.authority_step(run), snapshot own = world.player() buttons+presses; queue.ran() takes no argument :270.
- R-CODE-POST (crates/client-endpoint/src/post.rs): Post{number u32, buttons u16, presses [u8;16]} (no move, no aim yet); from_words(number, held u32, counts [u32;4]). press.rs: PressPacer, MAX_BACKLOG 255, next(page counts) one edge per button per command, backwards counted.
- R-LEGACY-CAM: legacy movement is camera-relative and the camera yaw is player-turnable: shamble-game.js:1320 `_cyaw=camYaw+T.camYawBase` (cos/sin) for the move mapping; camYaw turned by mouse while rotating (:3282, camYawSens 0.006), reset by BracketLeft (:3271); camYawBase 0.785 rad (45 deg) shamble-core.js:68; presets set the base yaw (:2202). Build plan Part 5 :857-860: panel sliders boom, height, fov, pitch override, ADS camera mode; five presets; framing yaw 45 deg. => the world move vector depends on the camera yaw, which only the page knows -> the page must post either the world move vector or the camera yaw.
- R-CODE-ENDPOINT (crates/client-endpoint/src/endpoint.rs): SENT_RING 64 :32; Readout{tick, echo_buttons, echo_presses, post_used, sent, received, refused, late, unwritten, presses_lost, presses_doubled} :58-79; Sent{sequence, tick, view_tick, post} :83-88; Endpoint fields :91-110; new(content_hash, first_sequence, pad_bytes) :117; set_post :158; write_block(refused_elsewhere) :175; build_input :226-286 (command move 0, aim 0, view_time{newest snapshot tick, 0} :254-262; newest always, repeats while unconfirmed and 1..=255 ticks apart); on_datagram :291-339 (late vs refused; echo check by last_processed against sent ring; post_used).
- R-CODE-LAYOUT (crates/client-endpoint/src/layout.rs): ElementType {U8, U32} only (a variant enters with first use) :7-13; Section{name, stride, fields}; BLOCK_LAYOUT :84-101 = header (tick u32 @0, layout_hash u32 @4; never move), echo (held, presses_0_3..12_15 u32), link (sent, received, refused, post_used u32) = 44 bytes; "Each entity section adds its row count to the header when it enters" :76-77; end_of assumes one row per section :108-115 (rows count enters with first multi-row section). Block layout = engine-architect's.
- R-CODE-SIMWASM (crates/sim-wasm/src/lib.rs): doc :5-6 WB-HEALTH quote "Keep what crosses it as small as the plan has it (one type, plain numbers on every tick)"; MODE_LOCAL 0; Sim{local: LocalMatch}; create(seed, mode, pad) start_tick 0, first_sequence 0 :71-85 (no start tick via bindings -> a players'-file recording must start at tick 0, sequence 0); exports: tick_hz, Sim::new(seed, mode, pad), block_ptr, block_len, input(post, held, p0..p3) :143-146, step(now_ms)->f64, rebase(now_ms)->f64 (+ panic_buffer in panic.rs). local_match.rs: LocalMatch<P: TransportPair = Loopback>{room, endpoint, link (private, no accessor), schedule, incoming, outgoing, unsent} :35-45; new queues Join on control :52-71; step(_now_ms) "nothing reads it yet (the link simulator's clock will)" :106-160 (order: build_input+send; server receives, on_control/on_datagram; room.step + send; client receives, on_datagram / on_control assert Joined; write_block(room refused + unsent); schedule ticked); first step carries the Join (endpoint Joining), first command is for the tick after the first snapshot; rebase :164-167.
- R-CODE-DEVTOOLS (crates/devtools): Cargo.toml crate-type cdylib+rlib; deps client-endpoint, protocol, room (the three unused path edges), sim, sim-wasm, wasm-bindgen; lib.rs :1-3 "The replay runner, the hash comparer ... So far: the rule tests' runners"; wasm.rs test-only module exports f32_vectors, chacha8_vectors, plant_u16_add (never shipped; dev build makes it in its own folder). vectors/: make_chacha8_vectors.py and make_f32_vectors.py (independent second implementations, Python stdlib, `python3 -I -B ... [--check]` compares byte for byte, writes nothing), data files + manifest with sha256.
- R-CODE-PAGE: picture.ts :18-24 (lane 2's interface file; engine-architect reviews changes) "How far between the two blocks a frame is drawn is the Worker's to say, because only the client endpoint knows the interpolation clock: it comes as a field of the block with the first thing that moves". PictureFrame{newest, previous, timeMs}. worker-link.ts keeps two newest blocks, reads tick/echo/link offsets from generated BLOCK_LAYOUT :4-15. sim-worker host.ts: HostSim input(post, held, p0..p3), step(nowMs Worker clock), rebase; block envelope {buffer, startAt, doneAt} on the shared clock (timeOrigin + now) :186-192 (messages.ts :41-44); newest post kept by number (wrap-aware) :104-118; on hidden: held = 0, pacer.pause() :121-128; POOL_SIZE 4 :37.
- R-CODE-POSTMSG (packages/sim-worker/src/messages.ts :13-31): input post = {post, held, p0..p3, returned}; doc: "The view time of the drawn frame joins with the change-set that uses it, as a tick and a fraction, never as the page's milliseconds." keyboard.ts :9-20 buttons W=0, A=1, S=2, D=3, Space=4, Shift (either) =5; repeats count as nothing :63-72; clear() on blur/hidden :76.
- R-LANE2 (scratchpad issue59-lane2-first-list.md, comment 6994614989, data not his word): rows to answer: H1 tick, H2 row counts (Plan:285; CS1 moved to cs4); F1 position (metres, height); F2 facing (aim yaw, legacy radians; ADR 0001 16 bits); F3 aim pitch (10 bits); F4 horizontal velocity m/s (least: F1 + F10 to derive); F5 grounded (changes every jump, does not fit "8 flag bits that change rarely"); F6 sprinting (draft 0001:310 flag, or derived); F7 alive (cs8); F8 weapon id (cs8); F9 stable soldier id; F10 respawn marker (generation or flag); F11 team/faction; F12 which soldier is local + its team (JoinAccepted is a message; gap 8: block or interface file, lane 1 chooses); F13 own reload progress (cs8); F14 projectiles (cs9). Events E1-E6 (cs8/9). Wishes W1 ADS flag per soldier, W4 sprint-to-fire swing progress (now needs per SLICE-FEEL), W2/W3/W5. Gaps: 1 (ADR 0001 soldier record lacks velocity, grounded, team), 5 (one yaw or two: legacy one angle aimYaw G:2782; P15:130-131 names facing and aim separately), 8 (local slot/team only in a message), 10 (page reading lane 1's content tables).
- R-TUNING (docs/legacy-tuning.json tables.T.values): moveSpeed 6.3 m/s; sprintMult 1.4286 (sprint 9.0 m/s); adsMoveMult 0.55; accel 14; gravity 24; jumpV 11 (apex 11^2/48 = 2.52 m); stepUp 0.55; arenaX 186, arenaZ 126; shieldMoveMult 0.72; STAM {max 7, regen 0.2, lockUntil 0.3} (shamble-game.js:2216); EJUMP_REACH 3.0708 (shamble-game.js:1575); FIXED_DT 1/60; BODY radii/heights (torsoR 0.52, headY 2.12, crouchDrop 0.55).
- R-LEGACY-JUMP: legacy jump is held-based: shamble-game.js:2537 `if(jumpHeld && st.grounded){ st.vy=T.jumpV; st.grounded=false; }`; EJUMP_REACH = jumpV^2/(2 gravity) + stepUp ~3.07 m (:1575). FIRE_H 1.9, FIRE_R 0.3, FIRE_F 0.35; playerFireOrigin uses facing sin/cos (:1418-1421) -> the fire origin needs the yaw in the sim (cs8).
- R-LEGACY-AIM: shamble-game.js:2702-2711: aim cast through the STABLE camera base (no shake/recoil/concussion) so cosmetic screen motion never drags the world aim; facing = atan2 toward the aim point when horizontal distance^2 > 0.0004 (2 cm), else kept (:2709-2710) -> facing is state (kept inside the dead zone); pitchWeapon toward the aim point. No facing-dependent movement found in T values.
- R-CHECKS: room/tests/any_bytes.rs :97-100, :205-224 (per-step identity: sequence moves by exactly run + missing) and client-endpoint/tests/any_bytes.rs :225-231 (every datagram = received + late + refused) -> my round-2 L-1 is done in 3b; neither file has a counting allocator (3B-HOSTILE-ALLOC open). queue.ran() takes no argument (room/src/lib.rs:270) -> L-2 done.
- R-PM3-42 (docs/adr/0003-rendering.md:1217): frame-rate cap options (60 fps cap; follow the display; a cap per tier): smoothness std/mean on 60/120/144 Hz, battery on ML (Slice 2); reported, no pass number; RD, mode P.
- Block 0 DONE (inputs read). Next: Block 1.

## Block 1. The recording, Walk's step 1, green before any movement code (condition b)

### 1.1 What is recorded, and the file format (version 1)
A recording is two committed files per name under `crates/devtools/recordings/` plus a manifest:
- `<name>.rec` (binary, little-endian): what the host hands the simulation, call for call, so that the same calls can
  be replayed on any target through the players' own entry points (`Sim::create`, `rebase`, `input`, `step`).
  - Header: magic 8 bytes `SHMBLREC`; version `u16` = 1; seed 32 bytes (a public TEST seed only: its first 8 bytes
    are `TESTSEED`; the writer refuses any other seed and every replay path refuses one, so no real match seed can
    ever sit in a recording or be replayed by a page: ADR 0004 part 8 :312-316); start tick `u32` (0 for every
    recording the players' file replays: `Sim::create` starts at tick 0, sim-wasm lib.rs:71-85); first command
    sequence `u16` (0 likewise); tick rate `u16` (30; a build at another rate refuses: the time unit is per build,
    ADR 0004 part 4); content hash 32 bytes (SHA-256 of the canonical content bundle the match ran on; all zeros =
    no content, which only step 1's first recording uses); map id: length `u8` (at most 64) + UTF-8; tick count
    `u32` (at most 108,000 = one hour at 30 Hz, checked against the bytes left BEFORE anything is allocated).
  - Then one 68-byte record per step, mirroring the two calls the live host makes per tick (host.ts:79-83):
    host time `f64` bits (8: the `now_ms` the step got; finite and never decreasing, checked by the writer and the
    readers) | post number `u32` | held buttons `u32` (low 16 bits) | press counts 4 x `u32` (button k in byte k%4
    of word k/4, as post.rs:35-48) | move `u32` (x in bits 0-7, y in bits 8-15, as `i8`; the rest 0) | cursor ray
    origin 3 x `f32` bits | ray direction 3 x `f32` bits | view tick `u32` | view fraction `u32` (low 16 bits, in
    1/65,536 of a tick). Every field Walk will ever pass is in version 1 from the start (block 3 defines move, ray
    and view time), so the movement and aim steps change no format: step 1's recording holds zeros there.
  - No trailing bytes. 3b's record had 24 bytes (my cs3 notes, replay.rs); version 1 is 68 because it already holds
    Walk's post and the host's time (the 3c item "the host's time recorded with every tick", archive :1555).
- `<name>.hashes` (text, LF): first line `shamble-hashes 1 <sha256 of the .rec> <tick count>`; then one line per step
  k = 1..n: `<world tick after step k> <shared hash, 16 hex> <whole-state hash, 16 hex>`. Text, so that a change-set
  that re-makes a recording shows in its diff exactly from which tick the hashes moved. The shared hash is the one
  every target can read (the players' file exports only it); the whole-state hash (seed and streams included) is
  computed natively only (two native processes compare it: PM-0004-18), and it is harmless to commit because the
  seed is a public test seed.
- `recordings/manifest.txt`: one line per file with its byte count and sha256 (as `vectors/chacha8.manifest.txt`).
- `recordings/content/<set>/`: frozen copies of the content files a recording runs on (step 1: none). Recommended so
  that a change of the game's own tuning (his Testlines values, block 3) does not re-make every recording: a
  recording pins the code's behaviour on fixed data; it is re-made when the code changes, not when tuning does.
  The other way (recordings on the live content) is noted as option B in 1.6.
- Why two files and not one: the inputs change only with the input format (rarely); the hashes change with every
  deliberate change of the simulation. Size at 1,800 steps: .rec 122,520 B + header (about 110 B); .hashes about
  72 KB of text. Git deltas the unchanged inputs; a re-make adds mostly the hash text.

### 1.2 The replay, the same on every target
- Calls, in the live host's order (pacing.ts:68, :103): `Sim::create(seed, MODE_LOCAL, 0)` (plus, from the content
  step on, the content call with the recording's frozen bundle, whose hash must equal the header's); `rebase(first
  host time)`; then per record: `input(...)` with every field the build's `input` takes, then `step(host time)`, then
  `shared_hash()`. A field the build's `input` cannot take yet must be zero in the record, or the replay refuses the
  recording by name ("this build cannot replay this recording's aim"): nothing recorded is ever silently dropped.
- The players' file gets ONE new read-only export (cs3-plan 5.1, DEPART-3; archive :1555-1556; engine-architect
  round 2, L3): `#[wasm_bindgen] Sim::shared_hash(&mut self) -> u64` (a JavaScript BigInt; one plain number per
  call). Rules: it returns the SHARED hash only (no seed or stream byte is in what it hashes); it is read only during
  a replay, never on a live tick (the live host still makes exactly two calls per tick: host.test.ts pins the calls
  a live tick makes); no call that takes or returns a block of bytes is added; the boundary test's pin of
  `Sim.prototype` (sim-wasm.test.mjs:143-147, security-auditor round 1 L3) gains exactly this one name.
- Where its buffer lives: private to `Room` (`hash_scratch: Vec<u8>`, reserved in `Room::new` to the shared state's
  length), behind `Room::shared_hash(&mut self) -> u64`; `LocalMatch::shared_hash` and `Sim::shared_hash` only
  delegate. At the source, `World` gets `write_shared_state` (writes the prefix only) and `write_state` = shared +
  authority part, so the scratch never holds a seed byte (today shared_hash writes all and zero-fills: world.rs:135-
  140). Tests: `scratch.len() == shared length` after the call, contents identical for two seeds that differ in
  every byte, and no allocation after `Room::new` (sim-wasm/tests/step_alloc.rs's counting allocator calls
  `shared_hash` every tick: 0 allocations).
- Native runner (`crates/devtools/src/replay.rs`): the same calls on `sim_wasm::Sim` natively (its methods are plain
  Rust off wasm32, as lib.rs's own tests use them), plus the whole-state hash through `LocalMatch::room()` ->
  `World::state_hash` with the runner's own scratch (native only, never exported). Built "as the server builds the
  sim" (ADR 0002 :506): release in CI's release line once E1 lands; debug twice in today's CI job.
- Wasm in Node (`packages/sim-worker/boundary/replay.test.mjs`): the players' file, `initSync`, a TypeScript reader of
  the format (`packages/sim-worker/src/recording.ts`, used by tests only and kept out of the production bundle by
  the build's dev-marker check), the same calls, BigInt compare with the .hashes shared column.
- Wasm in Chromium (`packages/e2e/tests/replay/determinism.spec.ts`): a test-only Worker script (in `packages/e2e`,
  never shipped) loads the built players' file (its sha256 checked equal to the build's) and runs the same loop;
  no change to the real Worker's message set is needed for this check. (The real Worker's paced replay mode, which
  PM-0004-28 needs, is block 5's, in Walk's second part.)

### 1.3 The comparer and the field-level diff (ADR 0002 :509, PM-0004-21)
- `crates/devtools/src/compare.rs`: `first_difference(start_tick, expected: &[u64], got: &[u64]) -> Option<
  Difference { tick, expected: Option<u64>, got: Option<u64> }>` (the first differing tick, or the first tick one
  stream lacks; tick via checked_add). Used for file-against-target and target-against-target.
- Field level: one visitor in `sim` drives every serialisation, so names and bytes cannot drift: `trait StateSink {
  fn u8/u16/u32(&mut self, name: Name, v); fn f32(&mut self, name: Name, v); fn bytes(..); fn authority(&mut self) }`
  with `World::visit(&self, &mut impl StateSink)`. The canonical writer (hash bytes) is one sink; a `Lines` sink in
  devtools writes `name = value` lines (floats as hex bits and decimal; every float read by `to_bits`, which is also
  PM-0004-24's finite read); the shared writer stops at `authority()`. The authority part is never printed as bytes:
  the seed never appears in any dump (ADR 0004 part 8); stream positions may (they are not secret).
- Native against native (two Worlds in one process have different std HashMap seeds): `field_diff` in-process.
  Native against Wasm: the native bin `cargo run -p devtools --bin replay -- state <rec> <tick>` and, on the
  test-only module (crates/devtools/src/wasm.rs, never shipped), `replay_state_at(content, rec, tick) -> String`;
  the two texts are diffed line by line. A bin under `src/bin/` is found by cargo without a manifest change, so
  devtools' Cargo.toml (and its three unused edges) stays untouched (security-auditor round 1 L2).
- Why not a byte export on the players' file: it would go beyond DEPART-3's "one new read-only number" and his
  WB-HEALTH words (lib.rs:5-6). The first differing tick is judged on the players' file; the fields come from the
  test module (same source, same compiler); if the test module does not reproduce a players'-file mismatch, the
  comparer says so, which is itself a finding.

### 1.4 The generator, `--check`, and when a recording is re-made
- Input scripts are Rust functions in `crates/devtools/src/scripts.rs` (deterministic, no clock, integer maths; host
  times synthetic: k x 1000/30 ms plus a seeded jitter of 0-6 ms, never decreasing). The generator is the native
  replay itself: `cargo run -p devtools --bin make_recordings` writes `.rec`, `.hashes` and the manifest;
  `--check` makes them in memory and compares byte for byte, writes nothing, exits 1 naming the file and, for a
  .hashes file, the first differing tick (the vector generators' behaviour: make_chacha8_vectors.py :8-9). The test
  `crates/devtools/tests/recordings.rs` calls the same check, so CI's plain `cargo test` runs it with no CI edit; it
  also fails on a recording file the manifest does not list, or a listed file that is missing.
- Re-made: in the commit that changes the simulation's behaviour or its state's layout on purpose, every recording
  is re-made by the generator, and the change-set's report names, per recording, the first tick whose hash moved
  (the comparer between the old and new .hashes) and why. Inputs change only with an input-format change (a new
  version; the old version is then refused by both readers). A change-set that means NOT to change the simulation
  must leave every recording byte-identical: that is the recordings' regression value. Recordings are made from
  scripts, never captured from play, so a re-make is one exact command.
- Prerequisite of the first commit of a .rec (cs3-plan section 3 item 5; state_hash.rs:5-7): PM-0004-21's short
  report, written with rust-reviewer: FNV-1a 64 kept or replaced, and its cost per tick (mean, p99) natively and in
  Wasm, from a small timing test printed with `--nocapture` (a reading).

### 1.5 The recordings, and what changes when movement lands
- Step 1 (today's state: tick, the player's applied command, the seed; no content): `first-loop` (1,800 steps:
  the 3b loop's inputs: the eight move keys' held patterns, Space and Shift, presses at varied gaps with two in one
  tick and a burst the pacer must spread, repeated post numbers for a slow page and skipped ones for a fast page,
  jittered host times; move, ray and view fields zero; public test seed; start tick 0). Green natively in three
  processes, in Node and in Chromium on the players' file BEFORE any content, collision or movement code.
- When movement lands (the soldier table, content, the command's move/aim): (1) the canonical state grows by the
  soldier table (appended in the shared part, named by the visitor), and world.rs's known answers are re-made from
  the second implementation; (2) `Sim` gains the content call: the replay passes the recording's frozen content set
  and checks its hash; (3) `input` widens: the replay passes move, ray and view time, and the "must be zero" refusal
  for them goes; (4) `first-loop` is re-made: its per-tick inputs stay byte for byte (`--check` proves the script
  still makes them), its header gains the content hash and map id, and every hash changes from step 1 (the soldier
  is in the state): the report says exactly that; (5) `walk` is added; (6) PM-0004-21's three plants are run again
  on movement fields, PM-0004-24's finite read and its bite become meaningful, PM-0004-25's reading runs (parry3d
  is in). The format stays version 1.
- At Walk's end: `first-loop` (re-made), `walk` (about 2,400 steps on the street corner and the training yard: the
  eight directions at several camera yaws, sprint, walking while aiming down sights, the sprint-to-sights wait,
  jumps by tap, by hold and by one command carrying 255 presses, a press while in the air, the 0.55 m step up and
  the 0.56 m stop, ramps up and down, thin walls at sprint speed, the 50 m lane, aim rays over ground, walls, box
  tops, ramps and sky), and `lane-walk` (about 1,200 steps of constant-speed walking up and down the lane, for the
  paced replay of block 5; Walk's second part). That is the build plan's "a recording per change-set that changes
  the simulation" (:484) and "a movement recording" (:353). Long native-only runs (PM-0004-18's 36,000 ticks) use a
  procedural script in Rust and are compared process against process, never committed.

### 1.6 Every mark that rests on the recording at Walk: test, where it runs, planted defect
(N = native `cargo test`; CI's job runs the workspace twice in debug = two processes; the release line joins with
E1, the small CI pull request. W = Wasm in Node on the players' file, CI's boundary run. C = Chromium, CI's browser
run. None needs a CI edit.)
| Mark (build plan line) | Test, where | Planted defect, must turn it red |
|---|---|---|
| ADR 0004 milestone (:470): first recording's hashes natively twice and in Wasm | N1 `devtools/tests/recordings.rs::every_recording_replays_to_its_hashes` (N, both columns); N2 two-process test (below); W1 `boundary/replay.test.mjs` (W, shared column); C1 `e2e/tests/replay/determinism.spec.ts` (C) | (a) one hex digit of one .hashes line changed: all three name that tick; (b) the sim changed at one tick (`if tick == 900 { presses[0] ^= 1 }` in authority_step): N1, W1, C1 all report tick 900; (c) the TS loop skips one input call: W1 names the first tick that differs; (d) one byte of a .rec record changed: `--check` fails |
| ADR 0004 foundation line (:473): a replay runner and the first recording in CI | the same N1, W1, C1, all in today's CI jobs | as above |
| G10 recordings (:484): one per change-set that changes the sim | N1 iterates the manifest; fails on an unlisted or missing file | an extra unlisted `.rec` in the folder; a listed one deleted |
| ADR 0001 :1785 (:485): a recorded input replay gives the same per-tick hash on two runs (met at 4) | N2 `devtools/tests/two_processes.rs`: the test spawns its own binary twice (std::process::Command on current_exe, a child-role env var, one exact test filter); each child replays every committed recording and a 36,000-tick procedural script, prints its std RandomState fingerprint (hash_one of a fixed key on a fresh RandomState) and a digest of its per-tick whole hashes; the parent asserts equal digests, 0 differences against the files, and different fingerprints (equal ones make the run invalid, not passed). On `walk` (movement) for the "met" | PM-0004-21's HashMap injection: the two children differ at its tick |
| PM-0004-19 on the slice's recordings (:494) | N1, N2, W1, C1 on `first-loop`, `walk`, `lane-walk` | the three injections of PM-0004-21 |
| PM-0004-17: nothing outside the state ("meaningful from 4") | N3 `devtools/tests/two_instances.rs`: two `Sim`s in one process, `walk` and `first-loop` stepped in turn (each a different seed and input stream), each equal to its own .hashes (both columns); W2 in `replay.test.mjs`: two `new Sim()` on one module instance, interleaved, each equal to its file. M10 (18,000 ticks) is Slice 2's form: here the recordings' lengths, said so | a `static` AtomicU32 bumped per step and added to a state field (the echo's presses in step 1, a position after movement): interleaved differs from solo |
| PM-0004-18: same build reproduces itself | N2 above (36,000 ticks; the stress scene's form at Slice 6) | the HashMap injection |
| PM-0004-21: divergence caught at its tick, with a field diff, 3 of 3; hash function and cost; word size | the report of 1.4; three injections armed at tick T = 600 by the plants script (the change-set's notes, like plants-3b.py): (i) std `f32::sin(K)` added to a state field, K found by the script where native and Wasm sinf differ (10^6 candidates; if none, a 1-ulp change on Wasm only stands in and the report says so): N vs W differ at T, field diff via the bin and `replay_state_at`; (ii) a std HashMap of 32 keys folded into a field in iteration order: two processes (N2) differ at T, in-process diff names the field; (iii) a presentation draw inside the sim (a planted JS `Math.random` import in Wasm, a per-process RandomState value natively) added at T: two runs differ at T. Control: a draw outside the sim (the page's own generator) leaves every hash equal. Word size: a committed vector (first-loop's shared bytes at tick 900 and their digest) checked natively and through the test module in Node | the mark's own bite: a planted `usize` field in the canonical bytes makes native (8 bytes) and Wasm (4 bytes) differ at tick 1 |
| PM-0004-24: NaN never enters (meaningful after movement) | the `Lines`/finite sink reads every float of the state by `to_bits` on every replay of N1-N3 (release runs from E1 on) and fails naming tick and field; the sim's debug/test finite assertion each tick (movement owner's); the post's ray floats checked finite where they enter (block 3) | `0.0 / 0.0` planted in the movement phase at tick T: the debug assertion trips at T and the release runner names T and the field |
| PM-0004-25: simd128 (a reading) | a script builds a second players' file with `-C target-feature=+simd128` into `.gate/` and runs W1 on it; mismatches reported (0 expected); nothing is decided, SIMD stays off | none (a reading) |

What the collision marks rust-reviewer designs need from this runner (their tests stay theirs):
- PM-0004-20 (parry3d's flag across targets): any recording that exercises the queries (`walk`, or a longer one
  they add), replayed by N1/W1/C1 with first-difference and the field dump; flag-off timing is outside the runner.
- PM-0004-30, -59 (step, apex, tunnelling, reach): sim-level scripted tests; if they want their cross-target
  agreement shown, they express a script as a recording and the runner replays it on W and C.
- PM-0004-32 (BVH bytes native = Wasm; rays after restore): a dump export on the TEST module (the 1.3 pattern), never
  on the players' file.
- PM-0004-35 (no Unsupported pair): a per-query result counter the native runner can print after a replay.
- PM-0004-39 (far corner of a 2 km map): must stay a sim-level script: the command's aim point (i16 cm,
  messages.rs AimPoint) cannot name a point beyond +-327.67 m, so a 2 km match through the local match is not
  possible until the codec widens the aim (a recorded limit, block 3).
- PM-0004-54 (cursor ray on a ramp within 1 cm): the query is theirs; my cast quantises to the 1 cm grid, which adds up
  to 0.87 cm (half a centimetre per axis, 3-D): they should test both the query's point and the command's point.
- PM-0004-60 (no panic from what a client can send, movement commands): the room's generator in
  room/tests/any_bytes.rs gains move and aim at their full ranges (block 3's room checks), plus a direct fuzz of
  `World::authority_step` with random commands: 0 panics, finite state.

### 1.7 Step 1's own size (bottom-up, +-30 %; the main session multiplies)
| Part | Code | Tests |
|---|---|---|
| sim: StateSink visitor, write_shared_state, shared length as a function | 60 | 50 |
| room: hash_scratch + shared_hash; sim-wasm: LocalMatch/Sim::shared_hash | 25 | 30 |
| devtools: recording.rs (format, caps before allocation, never panics) | 140 | 120 (round trip incl. non-zero move/ray/view; every refusal; 100,000 random and mutated inputs) |
| devtools: replay.rs, compare.rs (first difference, Lines sink, field diff) | 150 | 60 |
| devtools: scripts.rs (first-loop), bins make_recordings and replay, wasm.rs exports (replay_state_at, digest) | 150 | 0 |
| devtools tests: recordings.rs, two_processes.rs, two_instances.rs | 0 | 200 |
| 3B-HOSTILE-ALLOC: counting allocator in room/ and client-endpoint/tests/any_bytes.rs | 0 | 50 |
| PM-0004-21 report timing + word-size vector | 0 | 40 |
| TS: recording.ts reader; e2e test Worker | 130 | 0 |
| TS tests: reader, boundary replay (W1, W2, pin), e2e determinism spec (C1) | 0 | 160 |
| Total | 655 | 710 |
About 1,365 lines (+-30 %), 52 % tests. The cs3 plan's 275 for step 7 left out the Chromium path, the TypeScript
reader, the field-level diff, the two-process and two-instance tests and the shared-hash plumbing.

### 1.8 Findings for the plan (block 1)
- F1-1: build plan :470 "Met at 3 natively; the Wasm half in CI 2.3 after 3" — the Wasm half can be in CI at Walk
  with no CI edit (CI already builds the players' file and runs the boundary tests and every e2e spec: verifier-cs3b
  step 7 log); Firefox and WebKit stay with the small CI pull request (FIREFOX-TESTS (b)).
- F1-2: PM-0004-17 with "different seeds" proves nothing on shared hashes in cs4: nothing draws from a random stream
  yet (spread is change-set 8), so two seeds give equal shared states for equal inputs. N3 therefore uses two
  different input streams and both hash columns (the whole hash covers the seed bytes).
- F1-3: the field-level diff on the players' file is impossible without a byte export, which DEPART-3 does not cover:
  the plan should say the diff comes from the test module (1.3).
- F1-4: devtools' manifest stays as it is in Walk: bins under src/bin need no manifest line; the replay passes the
  content as bytes through the players' own call, so devtools needs no `content` edge.
- Block 1 DONE. Next: Block 2 (the 3c table).

## Block 2. Every item named for 3c, placed by name (condition c)
Homes: **R** = "the recording (Walk, step 1)"; **L** = "the link simulator's change-set (after Walk, merged before
change-set 7 starts)". Sources: STATUS.md "Next" item 2 (:33-36); STATUS_ARCHIVE.md :1453-1557; Builder V3's
handover part c; the 3b report; cs3-plan at 80a51e42; my 3b notes and reviews at 80a51e42; the question file :91-94.
| # | Item, and who named it | Home | Note |
|---|---|---|---|
| 1 | The replay runner, one state hash per tick (STATUS :33; card 3 :321; cs3-plan step 7) | R | block 1.2 |
| 2 | The comparer, first differing tick, and the field-level diff (card 3 :321; ADR 0002 :509; PM-0004-21) | R | block 1.3 |
| 3 | The first recording, natively twice and in Wasm in Node and Chromium (cs3-plan step 7; build plan :470) | R | `first-loop`, block 1.5 |
| 4 | ADR 0004's foundation line: a replay runner and the first recording in CI (build plan :473) | R | the lints part stays the gate chat's (2.3) |
| 5 | The state's hash readable on the players' file (STATUS :34; DEPART-3; cs3-plan 5.1) | R | `Sim::shared_hash`, one number |
| 6 | The export returns the shared hash only, plain numbers, read only during a replay, its buffer private to `Room` (archive :1555-1556; engine-architect round 2, L3) | R | block 1.2 |
| 7 | The host's time recorded with every tick (archive :1555; engine-architect round 2) | R | in the .rec record from version 1 |
| 8 | `Sim.prototype` pin catches a state or seed accessor joining the players' file; "3c adds the hash export" (security-auditor round 1, L3) | R | the pin gains exactly `shared_hash` |
| 9 | PM-0004-21's short report (the hash function and its cost per tick) before the first recording is committed (cs3-plan section 3 item 5; state_hash.rs:5-7) | R | prerequisite of the first .rec commit |
| 10 | PM-0004-17, -18, -19's begun parts and PM-0004-21's three plants (cs3-plan section 9, "begun") | R, re-run in Walk's movement step | block 1.6 |
| 11 | 3B-HOSTILE-ALLOC: a counted "no allocation" for hostile bytes in the room and the endpoint (verifier; archive :1543-1546; handover part c; question file :93-94 "whichever part comes first") | R | the recording step comes first; movable to L whole if R must shrink (it is unrelated to the recording) |
| 12 | RELEASE-COUNT (a): "No release counter is added in change-set 3c" (DECISIONS :256) | both | a constraint, nothing built; both parts honour it |
| 13 | The link simulator: seeded delay, jitter, loss, reordering, duplication, a rate limit, a packet-fate log, profiles P0-P4 and B1 (card 3 :322-323; cs3-plan step 8) | L | my cs3 design (link.rs, SplitMix64, five draws per datagram) stands |
| 14 | ADR 0001's milestone, "with L1 behind it in CI" (build plan :466) | L | T1: the exchange through the Worker's own step at every profile |
| 15 | PM-0002-49: same seed gives the same fate log and hashes; 5 % bands over 1,000,000 packets per profile (build plan :474) | L | read as DEPART-3 accepted: exactly 0 at P0, the burst by its own check, delay and jitter from the drawn delay |
| 16 | The simulator's clock reaches it through `LocalMatch` from recorded times (archive :1550-1551; NA-7; engine-architect round 2) | L | one `TransportPair` time method with a no-op default for the loopback; fed from item 7's recorded times |
| 17 | Its reliable channel delivers exactly once (archive :1551; NA-7) | L | keeps local_match.rs's panic on a second JoinAccepted valid |
| 18 | Loss and the rate limit as silent drops, duplicated datagrams only (archive :1551-1552; engine-architect round 2) | L | else they land in the block's `refused` |
| 19 | Corruption as loss only, what an authenticated transport delivers (archive :1552-1554; my round 2 citing RFC 9001 §6.6 and RFC 8446 §5.2; rust-reviewer round 1, RR-3) | L | |
| 20 | A `link()` accessor on `LocalMatch` (archive :1554; engine-architect round 2; my round 1 F7) | L | for the fate log |
| 21 | `late` checked against the simulator's own reorder count (archive :1556-1557; my round 2) | L | received + late + refused = delivered, and late = the simulator's reorders and duplicates |
| 22 | `devtools`' three path edges (client-endpoint, protocol, room), declared in 3b and unused (STATUS :34; security-auditor round 1 L2 = engine-architect EA-6; rust-reviewer round 1) | L | first user: L1's tests (protocol's transport, room and endpoint counters). R may use room/client-endpoint; either way no manifest change in Walk (a change reopens the dependency batch) |
| 23 | T3: L1 with every impairment off equals the loopback's hashes, on the first recording (my cs3 notes, block 3) | L | needs item 3, which exists from Walk step 1 |
| 24 | T4 (10,000 presses under 5 % loss, bursts and 1 % duplication) and T5 (every counter across its wrap at P2) (my cs3 notes) | L | the headless half of ADR 0001's press marks |
| 25 | "Not verified: anything over a link that delays, drops or reorders (3c)" (3b report :322-323) | L | closed by items 13-24 |
| 26 | My round 1 F7 (L1 needs a clock handle and an exactly-once reliable channel) | L | = items 16, 17, 20 |
| 27 | RR-3's link-simulator half (archive :1499: "whose link-simulator half is 3c's") | L | = item 19 |
Items named near 3c that fit NEITHER home (flagged, so nobody counts them as 3c's):
- 3B-PLAUSIBLE (the endpoint's plausibility bound on a snapshot's tick), 3B-LATE-BOUND (how late a command may run):
  change-set 7 (archive :1484-1499; handover part c). 3B-ACKS, 3B-OUTAGE-256: before online play.
- 3B-BLUR-TEST (archive :1540-1542: "with the next change to the page's key handling"): Walk's page-input step (the
  mouse and the aim-down-sights button change the key handling), not R and not L.
- 3B-ONE-PER-TICK and 3B-RESUME-FLASH: change-set 4 by their own text (block 4), not 3c's.
- My round 2 Lows L-1 and L-2 are already done in 3b (any_bytes.rs per-step identity and datagram accounting;
  `queue.ran()` takes no argument, room/src/lib.rs:270): nothing to place.
- Block 2 DONE. Next: Block 3 (the command, the snapshot, the path).

## Block 3. The command, the snapshot and the path for Walk (build plan :343-344)

### 3.1 The post and the one input call (page -> Worker -> endpoint)
The post gains, beside post number, held buttons and press counts (messages.ts:13-31; ADR 0002 part 5 :318-323):
- `move`: the WORLD move vector, made by the page from W/A/S/D and the camera's yaw (legacy movement is
  camera-relative and the yaw is player-turnable: shamble-game.js:1320, :3271, :3282), as two `i8` in [-127, 127]
  packed in one `u32` (x bits 0-7, y bits 8-15). Only the page knows the camera, and the vector is the player's
  intent, recorded like a key: the replay uses the recorded vector, so the camera cannot reach the simulation
  through it.
- the cursor ray: origin and direction, six plain numbers, from the camera's STABLE base (no shake, recoil or kick:
  legacy :2702-2706); Rust converts each with `as f32` (round to nearest, the same on every target). A non-finite
  number or a zero direction: the ray is ignored for that post, the aim keeps its last point, counted
  (`rays_refused`, readout and report). This is where floats enter, so PM-0004-24's "checked where it enters".
- the view time: the tick of the older of the two blocks the frame was drawn between (`u32`) and the fraction
  (`u32`, low 16 bits = floor(f x 65,536), at most 65,535) (messages.ts:19: "as a tick and a fraction").
- aiming down sights: the right mouse button held = button 6 in `held` (its press counter exists like every
  button's); no new field (lane 2's W1). The page stops the context menu.
`Sim::input` becomes 15 plain numbers (post, held, p0..p3, move, ox, oy, oz, dx, dy, dz, view_tick, view_fraction):
still one call per tick, plain numbers only (WB-HEALTH, sim-wasm lib.rs:5-6), within the four calls per tick.
`client_endpoint::post::Post` gains `move_x, move_y: i8`, `ray: Option<Ray { origin: [f32; 3], dir: [f32; 3] }>`,
`view: ViewTime`; `Post::from_words` becomes a total `from_numbers` (masks, finite check; never panics).

### 3.2 The command on the wire: no new field
- `protocol::messages::Command` already holds move (i8, i8), buttons, presses, aim (x `i16` cm, y `u16` cm, z
  `i16` cm) and view time (tick `u32`, fraction `u16`): 32 bytes of fields (codec.rs:15-18). Walk only fills what is
  zero today (endpoint.rs:254-262). No codec change, no TypeScript type regenerated for the command.
- Sizes against ADR 0001's 200 bytes per datagram (:728): in local play every command is confirmed in its own step,
  so an `Input` carries the newest command alone: 1 + 1 + 5 + 1 + 6 + 32 = 46 bytes. At the cap of 8 with one ack:
  277 bytes (292 with four acks, MAX_INPUT_BYTES codec.rs:44): above 200, the departure DEPART-3 (a) already accepted
  ("the compression must land before online play"); Walk does not change it. The aim point is 48 bits, inside
  PM-0001-51's 80, at 1 cm, inside its 5 cm.
- A limit to record (no draft states it): `i16` centimetres name points only within +-327.67 m of the origin, so
  the playable map must lie inside that square until the codec widens the aim; PM-0004-39's 2 km test stays a
  sim-level script (block 1.6). Height `u16` cm: 0 to 655.35 m.
- Client side, so an honest client never sends what the room refuses: `client_endpoint::post::clamp_move(x, y) ->
  (i8, i8)` (integer rule: while x^2 + y^2 > 127^2 in i32, step the longer component toward 0; the 8 keyboard
  directions at yaw 0, (+-127, 0), (0, +-127), (+-89, +-89), unchanged; exhaustive test over all 65,536 pairs; my cs3
  design, not built in 3b: grep finds no clamp in client-endpoint); the aim cast's point is clamped to the map's
  bounds and quantised (3.7).
- Room checks (room/src/lib.rs:170-175 names both for change-set 4; ADR 0001 :234-236): move length at most 1
  (x^2 + y^2 <= 127^2 in i32), else refused and counted `refused_move`; aim inside the map's bounds (from the map
  document), else `refused_aim`; both in `refused()`. Not checked yet, with the reason stated in on_datagram's doc:
  reach (no weapon until change-set 8: a point beyond reach is clamped, never refused), view time (lag
  compensation). Tests: each refusal counted; honest edges accepted ((127, 0), (89, 89), an aim on the boundary).

### 3.3 room -> `sim::PlayerCommand` -> the one soldier
- `PlayerCommand` gains `move_x, move_y: i8` and `aim: AimCm { x: i16, y: u16, z: i16 }` (the wire's integers; the
  sim converts to metres itself). The view time stays out until a system reads it (world.rs:33's own rule). The
  slot's player record (the last applied command) grows the same way and stays in the shared state (hashed).
- Slot = soldier index (ADR 0004 part 2; JoinAccepted.slot). Walk: one slot, soldier 0; the soldier table's capacity
  comes from the mode in content (1 in Walk; 12 to 48 in change-set 5).
- Recommended rule: a soldier's movement advances once per command that runs for it (oldest first; at most
  CATCH_UP_CAP = 2 in one world tick, room/src/lib.rs:40), and not at all in a tick with no command for it: the
  same unit as cs7's replay of own commands (ADR 0001 part 4), so prediction matches the authority exactly. Holding
  the last command on a missing tick AND running the late command later would move the soldier three steps over
  two ticks. In local play every tick has exactly one command, so play is unaffected; over a network this is where
  3B-LATE-BOUND's doubled rate becomes real movement (security-auditor round 1 M1: "once commands carry movement
  (change-set 4) that is double speed for the idle time"), which is why no room goes behind a network transport
  before change-set 7's bound.
- Per command the movement step reads: move; sprint = Shift held (button 5); aiming down sights = button 6 held;
  the jump request (block 4); the aim: the soldier stores the aim point and turns its facing toward it when it lies
  more than 2 cm away horizontally, else keeps it (legacy :2709-2710). Recommended state: facing as a horizontal
  unit vector (two f32, normalised with `sqrt`, on the allow-list), so no transcendental function enters the sim in
  Walk (yaw and pitch are computed where shown: 3.5). The movement internals are rust-reviewer's and the sim
  owner's.

### 3.4 The snapshot's soldier record
(a) The owner's own soldier, unquantised, in Walk (ADR 0001 part 4 :203-209: everything the replay reads, "sent
unquantised"). New flag bit 4 `FLAG_SOLDIER` (the echo's bit 3 stays as built), `protocol::messages::OwnSoldier`,
whole bytes, floats as their `u32` bits so `protocol` stays integer-only:
| Field | Width | Meaning |
|---|---|---|
| generation | u16 | changes on respawn; with the slot index it names the soldier (u8 would wrap at 256 respawns, which a 20-minute match with a respawn every 5 s nearly reaches) |
| placement | u8 | wrapping counter, +1 on spawn, respawn or teleport: the page places, never blends (ADR 0001 part 7 :340) |
| team | u8 | 0 to 3 |
| flags | u8 | bit 0 grounded, bit 1 sprinting, bit 2 aiming down sights; any other bit is a `Flags` error |
| position | 3 x u32 (f32 bits) | metres |
| velocity | 3 x u32 (f32 bits) | m/s |
| facing | 2 x u32 (f32 bits) | horizontal unit vector |
| aim | i16, u16, i16 | the applied aim point, cm |
| stamina | u16 | in the sim's integer time unit, if legacy's sprint stamina is kept (STAM: max 7, regen 0.2, legacy-tuning.json) |
| recovery | u16 | time-unit counts left of the sprint-to-sights wait; 0 = ready (3.8) |
47 bytes; the rule: every field of the soldier's movement state, so change-set 7's reset starts from exactly the
server's state (any field the movement design adds joins here in the same change). A Walk snapshot: kind 1 + tick 4
+ flags 1 + group 1 + received 2 + processed 2 + echo 18 + soldier 47 = 76 bytes (MAX_SNAPSHOT_BYTES 33 -> 80,
codec.rs:45); far inside ADR 0001's 1,067-byte cap. The endpoint refuses the whole snapshot when any float's bits
are not finite (a hostile server must not put NaN into the client's state; counted in `refused`), and an unknown
flag bit. The decoder test (decoder_inputs.rs) gains the new part in its generator and keeps its 10 million per type.
A first piece of PM-0001-17 comes free: a native test serialises the endpoint's decoded soldier through the same
visitor as the room's soldier and asserts equal bytes every tick of a replay.
(b) Spatial records, for every relevant soldier but the receiver: DESIGNED now, BUILT in change-set 5 with the first
remote soldier (no code without its reader): index u8; generation u16 and team u8 (only in the record that brings
the soldier into the client's set, ADR 0001 :315-316, once delta coding exists; whole bytes until then); placement
u8; flags u8; position x, z `i16` cm and y `u16` cm (E3's 1 cm step, the same +-327.67 m square); yaw `u16` binary
angle; pitch `i16` binary angle (E3's 10 bits later): 16 bytes per soldier in whole bytes (12 walkers: about 255
bytes a snapshot; 48: about 830, still under the cap; bit packing and deltas before online play). No velocity on the
wire: the Worker derives it from successive records, reset on a placement or generation change (gap 1 answered with
no draft change; fallback if change-set 10's pictures jitter: 2 x `i16` cm/s, +4 bytes). Yaw and pitch for the
records are quantised by the room's encoder from facing and aim with `libm::atan2f` (pure Rust, so the native server
and the Worker's local server agree: PM-0001-17); `libm` joins `room` in change-set 5's dependency batch.
What the state hash covers (shared part, in order): tick; per slot the player record (buttons, presses, move, aim);
the soldier table (generation, placement, team, flags, position, velocity, facing, aim, stamina, recovery, and every
other movement field); then the authority part (seed; the streams from change-set 8). Floats by `to_bits`. Nothing of
presentation (yaw and pitch are not state).

### 3.5 The block's soldier section (engine-architect owns the layout: the field list)
Header gains `soldier_count` (u32, lane 2's H2), `local_index` (u32, 0xFFFF_FFFF until joined) and `local_team`
(u32) (F12 and gap 8: in the block, from JoinAccepted, which the endpoint holds; the page reads blocks only).
One row per soldier (Walk: one), 40 bytes, aligned to 4:
| Field | Type | Range | Walk's source |
|---|---|---|---|
| pos_x, pos_y, pos_z | f32 | metres, y up; x, z within +-327.67 m; y about -20 to +42 | own soldier, exact |
| vel_x, vel_y, vel_z | f32 | m/s; horizontal up to 13.3 (legacy's fastest), vertical -35 to +11 | own soldier, exact |
| yaw | f32 | radians [-pi, pi); 0 faces +z (legacy rotation.y = atan2(dx, dz)) | atan2 of facing, in the endpoint |
| pitch | f32 | radians [-pi/2, pi/2], up positive | from the fire height (1.9 m, legacy FIRE_H) toward the aim point |
| generation | u16 | wraps | own soldier |
| index | u8 | 0 to capacity - 1 (at most 47 in Slice 1) | slot |
| placement | u8 | wraps; differs between the drawn pair -> place | own soldier |
| team | u8 | 0 to 3 | own soldier |
| flags | u8 | bit 0 grounded, bit 1 sprinting, bit 2 aiming down sights, bit 3 the local player | own soldier + slot |
| ready | u8 | 0 = just left a sprint, 255 = ready (W4) | recovery timer and the wait's length |
| pad | u8 | 0 | |
New element types F32 and U16 (layout.rs:5-13 holds U8 and U32 only: "a variant enters with its first use"). Yaw and
pitch for the block are computed in the client endpoint with `libm::atan2f` (presentation, one function for own and,
from change-set 5, remote rows); if `libm` is not already in parry3d's tree it is one small pure-Rust crate in Walk's
batch (the page computing them with `Math.atan2` is the cheaper alternative, allowed because they are not sim values).
Change-set 8 adds alive (flag bit 4), weapon id (u16), health; change-set 9 the projectile section; the events E1-E6
come with 8 and 9.

### 3.6 Interpolation, and who picks the blend fraction (my follow-up NA-N3)
- The Worker's field: each block's tick's scheduled time on the shared clock (`dueAt`, ms), stamped by the host from
  the due time the step returned for that tick (pacing.ts:103), in the block's envelope beside `startAt` and
  `doneAt` (messages.ts:41-44). Recommended over a field in the block's bytes: no f64 element type, no shared-clock
  time inside Rust, and online (change-set 7) the due time still comes from the step's return (the endpoint's
  server-time estimate), so one rule serves both modes. engine-architect may prefer it in the block (Rust writes it
  from `now_ms` if the host then passes the shared clock).
- The page's rule, every frame: render time r = frame time - P - d, with P = 1000/30 ms and d an adaptive allowance
  (the 90th percentile of the last 64 blocks' arrival minus dueAt, held within [0, P/2]); draw between the held pair
  j-1, j with dueAt(j-1) <= r < dueAt(j), at f = (r - dueAt(j-1)) / P clamped to [0, 1]; never extrapolate (hold the
  newest, or after a pause the first new block); place, never blend, when placement or generation differs. The next
  post reports view time = (tick of j-1, f).
- Two blocks or three: with only the two newest, a block that arrives later than d gives a hold, and one that arrives
  earlier gives a jump and then a hold, each as long as the difference, every tick; his Brave report has the Worker's
  lateness at 1.97 ms median and 14.07 ms p99 (2026-11-25-cs3b-browser-check.md), against a 6.94 ms frame at 144 Hz,
  so PM-0002-26's 5 % smoothness clause is unlikely with two. With the three newest held, r always falls inside a held
  pair while arrival <= d: holds only above the 90th percentile. ADR 0001 part 7 (:335-336) and ADR 0002 part 5
  (:304) say "the two newest blocks": three is a departure for the plan's list (his to accept); the Worker's pool goes
  from 4 to 6 (host.ts:37; part of 3B-POOL-16). Proof: a pure blend function in Vitest, frame times at 60, 120 and
  144 Hz, arrivals drawn from his reported lateness: std/mean of per-frame displacement <= 5 % with three; planted:
  the two-block rule, and the newest block drawn without interpolation (PM-0002-26's own bite), both red at 144 Hz.
- Who picks: the Worker gives the time, the page picks pair and fraction by that rule. picture.ts:20-23's sentence
  ("the Worker's to say ... a field of the block") holds in substance; its wording changes, and picture.ts is lane 2's
  interface file (engine-architect reviews; a lane matter for the plan).

### 3.7 The aim: what the post carries, where the ray meets the world (PM-0002-14)
- The page sends the ray with the keys in its one post per drawn frame (no second message kind).
- The client endpoint casts it each tick in build_input with the sim crate's ray query against ITS OWN static world
  (built from the same map document as the room's; online it has no room, so nothing borrows the authority's world:
  ADR 0002 part 6 :413-414), its own soldier excluded: the first hit within 500 m; none (sky, or parallel) -> the
  crossing with the horizontal plane at the soldier's fire height (its position from the newest snapshot + 1.9 m);
  no crossing -> the last aim point stays. Then clamp to the map's bounds and quantise to the 1 cm grid (`round`, on
  the allow-list). Deterministic on every target (parry3d with `enhanced-determinism`, f32) and replayed, since the
  ray is in the .rec.
- Remote soldiers join the cast when shooting needs their volumes at the post's view time (change-set 8; ADR 0001
  :258-259).

### 3.8 SLICE-FEEL at the wire level (DECISIONS :1227-1231; lane 2's W1 and W4)
- Lands in change-set 4: the input (right mouse button = button 6, held); the rule (sim; numbers from his Testlines
  Export through content): while button 6 is held and the sprint-to-sights wait is over, the soldier aims down
  sights, walks at 0.55 of his speed (legacy adsMoveMult 0.55) and cannot sprint; leaving a sprint starts the wait
  (integer time unit); the camera's sights mode follows the soldier's ADS flag from the block; the fields (flags bit
  2 and `recovery` in the snapshot's own part; flags bit 2 and `ready` in the block row). Walk has no gun: the wait is
  one stand-in value in the movement table (the reference gun's Export value), moved into each gun's row in
  change-set 8.
- Lands in change-set 8: firing waits each gun's sprint-to-fire delay (the same timer, its own length per gun); spread
  rests at each gun's sights value while aiming; the aim wait per gun; `ready` then shows the longer wait (or a second
  field if lane 2 needs both, their call then).
- Lines (+-30 %): change-set 4 about 275 (input 15 + 15 tests, inside 3.1's page work; the rule 60 + 80 tests, movement
  owner's; the fields 30 + 40 tests, inside 3.4/3.5; the camera reaction 20 + 15 tests); change-set 8 about 175 (fire
  gate 30 + 50, spread at sights 20 + 30, per-gun waits 20 + 25); remote W1 and W4 in change-set 5 about 70.

### 3.9 Lane 2's rows that are wire or snapshot questions
| Row | Answer | When |
|---|---|---|
| F2, gap 5 | one yaw: the aim yaw is the facing (legacy :2709-2710; :2782 `_animSt.aimYaw = player.rotation.y`); lane 2 turns the hips from velocity against it | Walk; others 5 |
| F3 | pitch in the row, radians, from the fire height toward the aim point | Walk; others 5 |
| F4 | velocity in the row: own exact from the sim; others derived by the Worker, reset on placement/generation; no wire field | Walk; others 5 |
| F5 | flag bit 0 grounded, from the sim; on the wire a flag bit (gap 1's "change rarely" is a bandwidth estimate, and it changes twice a jump) | Walk; others 5 |
| F6 | flag bit 1 sprinting, from the sim, not derived from speed (speed cannot tell a sprint from a fall) | Walk; others 5 |
| F9 | index (slot) and generation in every row | Walk |
| F10 | placement counter (spawn, respawn, teleport), and the generation on respawn | Walk; respawn 8 |
| F12, gap 8 | header: local_index and local_team; the block, not the interface file | Walk |
| gap 1 | velocity as F4; grounded as F5; team in every row, and on the wire with the soldier's entry | Walk; others 5 |
| H2 | soldier_count in the header | Walk |
| W1 | flag bit 2 aiming down sights | Walk; others 5 |
| W4 | `ready`, 0 to 255 | Walk; others 5 (Worker-derived) |
- Block 3 DONE. Next: Block 4.

## Block 4. The two follow-ups placed at change-set 4

### 4.1 3B-ONE-PER-TICK (archive :1513-1515; written on sim::PlayerCommand, world.rs:35-37)
- Systems that act on presses in Walk: the jump only (Space, button 4). Sprint (Shift) and aiming down sights (right
  button) are holds; the move is a vector; the camera panel's key and the replay key are page-only and never sim
  buttons. Crouch is not in Walk.
- The jump's rule, per command that runs (not per tick: a tick can run two): the request is Space held OR a press edge
  since the previous applied command (`press_edges` on button 4's counter); if requested and grounded, one jump
  (vertical speed = jumpV, grounded off); every further press of that command is dropped, never banked: a press in
  the air does nothing and is not kept for the landing (legacy is held-based and has no buffer:
  shamble-game.js:2537). The press edge keeps a tap shorter than a tick (PRESS-PACE (a)).
- Tests (sim crate, deterministic): J1 grounded, one command with Space's counter 255 ahead (an honest client after
  an outage) -> exactly one jump, apex = the integrated value (PM-0004-59's); J2 in the air, +1 -> no jump, and no jump
  at the landing without a new press or hold; J3 Space held across landings -> a jump at each landing (legacy's hold
  rule: game-designer confirms it as Walk's start rule, changeable after his Brave check, like START-RULES); J4 two
  commands in one tick, each +1 -> one jump; J5 a tap shorter than a tick (counter +1, held 0) while grounded -> one
  jump. Planted: a jump per press (vertical speed added once per new press) -> J1 red; presses banked until the
  landing -> J2 red; the held bit read alone -> J5 red.
- Change-sets 8 and 9 (same follow-up): fire, reload and its active-reload press, throw: each applies at most its own
  cooldown's worth per tick, each with its test then.

### 4.2 3B-RESUME-FLASH (archive :1478-1483; browser-check report :38-48): HIS decision
- The mechanism today: a hidden tab pauses the local match (no tick runs: host.ts:121-128, pacer.pause()); the
  Worker lets go of every held key at the hide (`#held = 0`); on the return the first tick runs one tick period after
  the rebase, carrying the let-go keys; until then the page draws the blocks from before the pause. In 3b that showed
  as a key box still lit for about a thirtieth of a second.
- With a soldier on screen (Walk): the first picture after the return is the moment of hiding, exactly (the blend
  holds the newest block, it never extrapolates across the pause: 3.6). One tick later the keys count as let go, and
  the soldier finishes his stop as if W had been released at the moment the tab hid: with legacy's acceleration 14
  that is about 6.3 / 14 = 0.45 m from a walk (if Walk keeps legacy's exponential approach; the movement owner
  confirms), then he stands. The picture may jump by one tick's motion when the first new block arrives (at most
  13.3 m/s x 1/30 s = 0.44 m, usually about 0.2 m), because the pause sits between those two blocks.
- The options for him:
  (a) Leave it: after a pause the picture is the moment of hiding; one tick later the keys count as let go and the
      soldier finishes his stop. Cost: no rule change; two guards (no extrapolation across a pause; the camera's
      smoothing takes no catch-up step from the long frame gap) and one browser test (hide with W held, return: the
      first frames draw the paused position, 0 ticks while hidden, the soldier stops within 15 ticks with no input):
      about 5 lines and 40 test lines.
  (b) As (a), and the development key diagram goes dark the moment the tab hides (what he saw in 3b no longer shows;
      the soldier is unchanged): about 10 more lines and a test.
  (c) Keys held at the hide count as still held until the page's first post after the return (the Worker stops
      letting go at the hide): the soldier walks on for up to a tick before the real keys apply. About 5 lines, but
      it undoes 3b's NA-N2 rule (a key released while away would count as held for a tick). Not recommended.
  (d) The soldier stops dead at the return (velocity set to 0): the hidden tab would become an input of the
      simulation (recorded, replayed), so the game would depend on presentation. Not recommended.
  Recommendation: (a); (b) if he also wants the 3b flash gone.
- Block 4 DONE. Next: Block 5.

## Block 5. The boundary and pacing marks the build plan places at change-set 4 (Part 3.3 :571-606; PM-0003-42 :714)
| Mark | Test, where it runs | Planted defect | Reading for his browser check |
|---|---|---|---|
| PM-0002-14 (the aim in the post; ADR 0002 :1285) | (1) native: a scripted LocalMatch run recomputes, every tick, the cast of the post handed in and compares it with the aim decoded from the Input bytes sent (the endpoint also counts `aim_not_from_post`, 0); (2) Chromium, CI: PM-0002-12's 10,000-press run without the shim (a sibling of presses.spec.ts): the Worker counts every page message by kind; 10,000 of 10,000 posts carry six finite ray numbers and a view time (the guard counts posts lacking them: 0), 0 messages of another kind during play, `stalePosts` 0 | a separate `{type: "aim"}` message each frame from which the Worker takes the aim: other kinds > 0, red; the endpoint casting the previous post's ray: (1) red | none |
| PM-0002-26, smoothness clause (= PM-0004-69; build plan :594) | the pure blend function in Vitest (3.6): frame times at 60, 120 and 144 Hz, arrivals from his reported lateness, std/mean of per-frame displacement <= 5 % (CI); in Chromium a reading only (a headless page is no display) | the newest block drawn without interpolation; the two-block rule at 144 Hz | yes: std/mean per display rate in the report, from the soldier's per-frame drawn position on constant-speed stretches (holding W along the 50 m lane; in the second part the lane-walk replay) |
| PM-0002-29 (Wasm weight with content; :597) | a size script reads the built players' file: raw, gzip level 9, Brotli level 11, name section; once, the build options table (opt-level s and z, lto off and fat, codegen-units default and 1) built into `.gate/`; every served file under 25 MiB | a 26 MB file in the output: the build's check refuses it (add that check if the build has none) | none; numbers in the report (the first limit is the Advisor's) |
| PM-0002-31 (content reaches the Worker in development; :599) | Chromium against the Vite dev server (its own Playwright project): a script saves the movement table 20 times; the Worker stamps each arrival on the shared clock; 20 of 20 within 1 s of the file's write; records whether Vite's hot update reached the module Worker itself or the page relayed it | the relay switched off: 0 of 20; a 2 s debounce: red | none (the zip is a production build) |
| PM-0002-34, walking steps (:602; ADR 0002 checklist :1413-1417) | support only: the hidden-tab spec extended with a walking soldier, a 5 s hide in CI (tick counter equal, the soldier's drawn position equal before and after, no extrapolation); the 60 s and 6-minute hides run from a script of their own, never a CI spec (3b's 10-minute pattern) | a Worker that simulates the hidden gap: the counter differs, red | yes: step 2 (the soldier walking at constant speed for 30 s: even, no stutter; his display rate), steps 3 and 4 (1 and 6 minutes away: exactly where he left it, 0 ticks while hidden, smooth after; with RESUME-FLASH's answer) |
| PM-0004-28 (presentation never reaches the sim; :603) | the real Worker's paced replay mode (development builds only; the production message guard refuses it; a new dev marker): the page sends a recording, the host runs it at live pace with recorded inputs (page posts ignored and counted), reads `shared_hash()` after each step and compares with the file; Chromium, CI: `lane-walk` four times (camera fixed, camera orbiting by script, frame cap 30, uncapped) -> each equal to the file | the host's replay takes the page's live ray instead of the recorded one: the orbiting run differs, red. Control: a cosmetic draw from the page's own generator changes nothing | yes: the same on his 60 Hz and highest-rate displays, "replay hashes equal" in the report. Sound on and muted: change-set 11 |
| PM-0003-42 (smoothness per frame-cap option; :714) | the page's frame-cap setting (a 60 fps cap; follow the display; per-tier caps wait for change-set 6's tiers): a Vitest of the cap's frame choice; the smoothness reading per option from the lane-walk replay | none (reported, no pass number) | yes: std/mean per option on each display he has |
| PM-0004-68 to -70 ("as PM-0002-26: 2 and 3, 4") | Walk's part is the smoothness row above; jitter and the hidden tab stand as 3b left them | as above | as above |
- Placement: PM-0002-14, the blend function and the hidden-tab support belong with the first playable soldier;
  PM-0004-28, PM-0003-42, the lane-walk replay and PM-0002-31 can come in Walk's second part; PM-0002-29's numbers in
  whichever part brings parry3d.
- Block 5 DONE. Next: Block 6.

## Block 6. Sizes (bottom-up, hand-written lines added, +-30 %; NOT multiplied: the main session applies 1.8)
| Step | What | Code | Tests | Total |
|---|---|---|---|---|
| R | The recording, step 1 (block 1.7) | 655 | 710 | 1,365 |
| C | The command path: post, 15-number input, the cast, the clamps, the room's two checks, PlayerCommand (block 3.1-3.3, 3.7) | 370 (Rust 225, TS 145) | 420 (Rust 275, TS 145) | 790 |
| S | The snapshot's own soldier, its decode, the block's soldier row and header, dueAt, the blend function, the pool (3.4-3.6) | 320 (Rust 215, TS 105) | 340 (Rust 220, TS 120) | 660 |
| J | The jump's press rule (tests J1-J5) and RESUME-FLASH (a)'s guards and test (block 4) | 15 | 110 | 125 |
| W | Movement recordings and the marks on movement: walk script 120, lane-walk script 30, finite sink and its bite 20, SIMD script 30 | 200 | 45 | 245 |
| B | Boundary and pacing marks (block 5): kinds counter, paced replay mode, smoothness logger, frame cap, size script | 185 | 230 | 415 |
| Mine | | 1,745 | 1,855 | 3,600 (about 2,500 to 4,700) |
| SF | SLICE-FEEL's rule and camera reaction (owners: movement and page; its input and fields are inside C and S) | 80 | 95 | 175 |
Not counted: the generated TypeScript, the recordings and frozen content (data), the plants script (notes).
- The first playable soldier needs: R (condition b: before any movement code), C without the aiming button, S, J, W's
  `walk` recording and finite bite, PM-0002-14's tests and the hidden-tab support: about 1,495 code + 1,700 tests =
  3,195 (+-30 %), before the content crate, parry3d, movement, the page's figure, camera and panel, which are other
  designs'.
- Walk's second part can take: SLICE-FEEL (the aiming button, the rule, W1 and W4 at work), PM-0004-28 and PM-0003-42
  through the paced replay mode, `lane-walk`, PM-0002-31, the SIMD reading: about 330 code + 250 tests = 580 (+-30 %),
  plus SF's 175.
- R alone is about 1,365 (x 1.8 = about 2,460, the size of a small change-set). If the first part's stop line cannot
  carry R with the soldier, his fallback (d) applies (R as its own change-set before Walk, no browser check, told to
  him before anything moves); it also gives condition (b) by construction.
- Block 6 DONE.

## Summary of disagreements and questions (for the answer)
Disagreements: cs3-plan step 7's 275 lines (bottom-up 1,365); cs3-plan :148-150 (Walk needs no L1); build plan :470
(the Wasm half can be in today's CI at Walk); build plan :343-344 read as the own soldier at Walk, spatial records at
change-set 5; ADR 0001 part 7 / ADR 0002 part 5 two blocks -> three; ADR 0001 part 6 record: velocity derived, grounded
a flag, team on entry, a placement counter added; the aim's i16 cm caps maps at +-327.67 m (PM-0004-39 sim-level);
PM-0004-17's "different seeds" proves nothing in cs4; per-command movement makes 3B-LATE-BOUND a movement issue from
Walk on (network only, cs7); picture.ts's sentence wording.
Questions for Mustafa: RESUME-FLASH (a/b/c/d, recommend a or b); BLEND-BLOCKS (three blocks, a departure; recommend
three); only if the stop line says so, the recording's home (his fallback d). Plan-level, not his: recordings on frozen
content copies (recommended) or live content; the jump's hold rule (game-designer confirms legacy's).
DONE
