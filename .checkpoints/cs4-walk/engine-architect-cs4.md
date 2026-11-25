Task: engine-architect design notes for change-set 4 "Walk" (Slice 1 build plan), lane 1 plan input, 2026-11-25.

## Plan (blocks, in order)
- B0. Reading: build plan (Since the plan, Walk card, Parts 2/3/4/5/7), appendix A4/A5, cs3 plan + my cs3 notes/reviews, STATUS follow-ups, reports, ADR 0002/0004/0005/0003, the merged code (crates, packages), lane 2's list. Notes line per finding.
- B1. The content crate (loader, validation, canonical writer, format name+version, schema + TS types, movement table from legacy tuning, test scene map documents, hot reload under 1 s, where files live, content across the bindings vs WB-HEALTH, outside crates weighed, marks with tests and planted defects).
- B2. The simulation's structure (soldier table, ids + generations, tick loops and order, movement's place, state and hash, zero alloc, data vs code, seams for rust-reviewer and netcode-architect).
- B3. The block (header counts, soldier section fields/strides, blend field, local player row/team, sizes at 1/12/48, Worker buffer pool), then lane 2's list row by row.
- B4. The page, folder by folder, file by file (picture.ts, stand-in, scene from map docs, follow camera + tuning panel, mouse aim, readout rows, dev-only exclusion, no per-frame alloc).
- B5. The cut and the order (recording first in devtools, devtools path edges, one or two stops, earliest playable zip, size ladder).
- B6. Room for lane 2's early-look page (three lines).
- B7. Sizes per step (code vs test lines, plus or minus), risks, not yet tried.
- Then DONE, then the final message from the notes alone.

## Notes (one line per finding)
- N1. No agent memory exists for engine-architect (searched .claude/agent-memory*, none). Start fresh. (No Edit tool here: notes are appended by shell heredoc, blocks written whole.)
- N2. ORDER-3C (c): recording = Walk step 1; link simulator after Walk, before cs7 (order-3c-answer.md:8). Conditions a-d (:10-13). Earliest playable + one-or-two stops asked (:14).
- N3. Takeover point 7: cs4 plan must name MUL-ADD's built-file check, 3B-RESUME-FLASH (first picture after a pause, his decision), Testlines values (takeover.md:20). Point 8 g: "where I tune once change-set 4 exists, so each value has one home" (takeover.md:29) -> content design must give each value one home. Point 8 f: 30 steps/s; if Export says 60, tell him (:28).
- N4. Walk card (build plan :334-358): content crate (loader, canonical writer, format name+version, movement table from legacy import, greybox map docs, schema+TS types generated, dev reload < 1 s); sim soldier table with ids, tick loops, own movement on parry3d queries (walk, sprint, step-up, jump), time unit from a rounding report; command with aim point (Worker casts the page's cursor ray); soldier fields in snapshot; block soldier section; interpolation between two newest blocks; stand-in; scene; follow camera at legacy framing; live camera panel; street corner + training yard (0.55 and 0.56 m steps, ledges, platforms with ramps, thin walls, 50 m lane with posts). Ladder: PM-0004-39 far-corner map + size comparison move out first; camera panel stays (:355-356).
- N5. Measured at 4 (:353-354): identical hashes native/Node/Chromium on a movement recording; movement vs data (step, apex, no tunnelling); collision tree byte for byte; ids over random spawns; even motion on his display.
- N6. Part 2 rows at 4: ADR 0005 foundation line (:461), G10 recordings "4 on" (:484), ADR 0001 :1785 replay two runs same hash met at 4 (:485), S4 time unit (:490), S6 ids -15 at 4 (:492), S9 -30,-32,-35 soldier pairs,-39,-33 own size at 4 (:493), determinism suite native x2, Node, Chromium (:494), CD-1..8 movement (:502), content items 1 and 2 (:503-504).
- N7. Part 3 rows at 4: PM-0002-14 (aim in the post), -26 smoothness, -29 sizes with content, -31, -34 checklist (even walking; tab hidden 1 and 6 min); PM-0004-28 (camera still/moved, 60/144 Hz), PM-0003-42 (smoothness per frame-cap option), PM-0004-6 (zero alloc in step, counted), -11, -15, -17 meaningful, -19, -20, -21 (three planted divergences), -22 rerun when a function joins, -24, -25 (SIMD reading), -30, -32, -33, -35, -39, -54 (yard ramps), -59, -60 (movement command fuzz); PM-0005-1..8, -10, -14, -16 (:724-739); separate tests at 4: PM-0004-33 comparison builds (:562), PM-0005-16 size half (:564); PM-0004-34 only if movement misses (COMPARE :30). Questions at 4 (:768).
- N8. Part 5 camera panel (:853-865): GP5 sliders (boom, height, fov, pitch override, ADS camera mode) + presets Classic, Tactical, Iso, Iso+, Foxhole on backquote; CD5 framing height 40, boom 56.6, fov 15, yaw 45, dead zone, smoothing; six unread CD5 keys not ported; "copy values"; leaves Queued when cs4 merges.
- N9. Part 6 (:915): local player drawn one tick behind (OQ-0001-15) = the page's drawing of one entity (4). (:911) datagram cap = splitter parameter (4).
- N10. Part 7 (:932) cargo test --release for content from 4; (:934) "cargo tree shows sim depends on content only" -> plan expects a sim -> content edge.
- N11. Soldier brief (appendix A4 :117-191): metres, axes (L, F, U), origin on the ground between the feet; 1.80 m to hat top is a placeholder, "Hang every part on one root" (:130); boxes and 8-sided cylinders; heights (:134), widths (:136); 4 materials Coat/Top/Trim/Dark per faction (:138-145); parts list (:147-171); rifle stand-in 1.12 m (:170); game camera 35.3 deg above ground, 15 deg lens, turned 45 (:125); Blender camera at (23.50, -23.50, 24.44) aimed at (0, 0, 0.9) (:191). Expected bounding box about 0.73 x 0.46 x 1.80 (:182).
- N12. Street corner (appendix A7 sheet 3 :340-356; the task's "A5" points at the addition, sizes are in A7): House A 6.1 x 4.4, eaves 2.6, ridge 4.7; House B 4.6 x 3.8, eaves 2.6, ridge 4.3; alley 1.5; wall 3.1 x 0.45 x 1.0; street 4.3 wide. Yard colours proposal (A5 C1 :217-224): street ground #B8AD98, blocks up to ~1 m stone #D0C2A2, 2 m+ plaster #E6DDC8, posts timber #4A3A2E. Plan Part 5 (:874-877): until the look carry only sizes and layout are taken; cs4 figures "plain on purpose". -> colour of greybox shapes is a look question: flag, never decide.
- N13. A5 A1 (:203): aiming pose muzzle at (-0.12, 1.17, 1.40) "shots leave from there"; plan "Since" (:68): "In the simulation a shot's start is data of its own" -> the muzzle point for the sim is content data (cs8), not a page reading.
- N14. cs3 plan (80a51e42 cs3-plan.md): 5.1 (:148-150) input = six plain numbers (post number, held bits, four packed press words); "the command's movement and aim fields, their clamp and its tests enter there [cs4]" (~40 lines). (:152-159) replay runs on the exact player file through input, step and ONE new read-only number, the state's hash (64-bit: two u32 or one BigInt read only during a replay; DEPART-3). No bytes-taking call on the shipped file (WB-HEALTH (a)). (:136) step 7 planned 275 lines: replay.rs, compare.rs, recordings/first-loop.rec (1,800 ticks, < 58 KB), sim-wasm.test.mjs. (:140-143) ladder moved L1 then the recording to 3c. (:72-74) PM-0004-21's hash report written before the first recording is committed (a recording pins the hash). (:220) NA-N3: field comes with the soldier section in cs4.
- N15. My cs3b review 2 (80a51e42 engine-architect-review2.md, Q7 L3): hash export returns the SHARED hash only (nothing seed-derived leaves), plain numbers read only in a replay (BigInt or lo/hi), never bytes, never on the live tick; the scratch buffer private to Room (reserved in Room::new behind Room::shared_hash(&mut self)); write_shared_state writes the prefix only (test: scratch.len() == SHARED_STATE_BYTES). Recording: record now_ms per tick. LocalSchedule and `unsent` are local-only and rightly stay in LocalMatch (ADR 0002 part 6).
- N16. My review 1 (Q7): u64 crosses wasm-bindgen as a BigInt (heap object); multi-value returns arrive as an Array -> a hash read as two u32 calls (hash_lo/hash_hi) or one BigInt only in replay. devtools edges: 3c needs protocol and client-endpoint; room only if named.
- N17. Security L2 (80a51e42 security-auditor-review1.md:118-121): devtools' three unused path edges: agree, leave the manifest, say so in the report; "a change reopens the batch" (dependency hold). L1 (:97-100): deferred room checks to be stated: "move length and aim: enter with the record's movement, change-set 4". Threat line (:126-129): an honest client may apply up to 255 presses of one button in one tick: every system reading edges (the jump!) applies at most what its own cooldown allows per tick.
- N18. perf-auditor review 2 (:62): POOL_SIZE 4 -> 16 at host.ts:37 is the cheapest change "for when the block grows" (lumps of 12-17 at 6 fps with key input; at N=48 a miss costs a whole block). Per miss: ArrayBuffer(blockBytes). That is 3B-POOL-16.
- N19. perf-auditor review 2 (:28-29): players' Wasm at 0dfca749 = 53,783 raw / 21,983 gzip / 18,368 brotli; growth by crate measured with wasm-split.mjs (name section 19 %).
- N20. STATUS (docs/STATUS.md:33-36): "Then change-set 3c ... with the state's hash on the players' file and devtools' three path edges ... Then change-set 4, the first outside crate in sim (its plan names 3A-FMA-CHECK)". Follow-ups for 4 (:100-103): 3B-ONE-PER-TICK (4, 8, 9, each with a test), 3B-RESUME-FLASH; later: 3B-POOL-16, 3B-CSS-PROD, 3B-HOSTILE-ALLOC, 3c's plan items.
- N21. Archive 3c items (STATUS_ARCHIVE.md:1547-1557), split per ORDER-3C (c): RECORDING part = host time recorded with every tick; state-hash export returns the shared hash only, plain numbers, replay-only, buffer private to Room; devtools' three path edges. LINK-SIM part = the simulator's clock through LocalMatch from recorded times (NA-7); reliable channel exactly once; loss and rate limit as silent drops, duplicates datagrams only; corruption as loss only; a link() accessor on LocalMatch; `late` checked vs the simulator's reorder count. 3B-HOSTILE-ALLOC (:1543-1546, "with 3c's tests") goes with whichever part comes first (question file :91-94) -> Walk.
- N22. 3A-FMA-CHECK (archive :1355-1360): "The check on the built file (no fused multiply-add inside the players' WebAssembly) is built in the same change-set that brings the first outside crate into the simulation (the collision library ...)". 3A-FEATURE-VERSION (:1391-1394): "when sim first gets a dependency": the feature compare matches by name not name+version (fails closed). 3A-SCAN-HELPERS (:1395-1398): with the change-set that adds part 7's three helpers. 3A-RULES-STEP (:1377-1380): devtools' first native-only dependency must be target-gated.
- N23. 3B-RESUME-FLASH (archive :1478-1483): his, open, "it comes back with change-set 4, when a soldier is on screen". 3B-ONE-PER-TICK (:1513-1515): written on sim::PlayerCommand; a system applies at most its own rule per tick. 3B-BLUR-TEST (:1540-1542): with the next change to the page's key handling (cs4 changes it: mouse aim) -> place in cs4. 3B-CSS-PROD (:1526-1527).
- N24. Lane 2's list (issue59 file): rows H1, H2, F1-F14, E1-E6, W1-W5, gaps 1-10. Lane 2 says the picture already gives back the muzzle point and cursor ray through the interface file (P15:117-121) (:44). Health bars stay with lane 1 (:46). Gap 8: slot/team only in JoinAccepted; F12 or the interface file; lane 1 chooses (:108). Gap 10: page reading lane 1's content tables (:110).
- N25. Part 15.2 (:117-121): interface file gives the picture each frame the two newest blocks, the events, the time; the picture gives back start, warm up, draw, set quality, the cursor's ray, the muzzle's point, a hook for lane 1's plain lines; "Lane 1 builds all of these in change-sets 2 to 5, on the plain stand-in". (:124-126): lane 1 builds into the stand-in, BEFORE its hand-over (cs5 merge), how a dead and a respawned soldier show (cs8's deaths), "read from a field of the block" -> F7 (alive) and F10 (generation) must exist in the block before cs5 merges. (:128-129) content/look and content/feel read and checked by lane 1's content crate, types by lane 1's generators. (:112-115) render/, soldier/, camera/ + GPU harness pass to lane 2 at cs5's merge.
- N26. cs1 report (2026-11-18-cs1-types.md:187-189): each entity section adds its count to the header when it enters, the first in cs4. (:263-264) "Change-set 4: the header states each section's row count and byte offset; padding between sections has one owner; the page reads the hash as soon as its four bytes exist; content's export test shares the folder rules."
- N27. Code: root Cargo.toml:35-55: every third-party crate compiled into our libraries gets a named overflow-checks override "in the change that adds it" (OVERFLOW-ALL), and 3a's build check refuses a runtime crate without one -> parry3d's whole transitive tree (nalgebra, simba, num-traits, approx, arrayvec, ...) needs one line each: a rust-reviewer seam.
- N28. Code: devtools/Cargo.toml:14-22 declares client-endpoint, protocol, room (unused: src uses only sim via rules and `use sim_wasm as _` at wasm.rs:10); sim-wasm's comment already names "the replay runner and the link simulator's tests". The three edges = L2/EA-6.
- N29. Code: sim-wasm/src/lib.rs: Sim { local: LocalMatch } (:60-62); create(seed, mode, pad) (:70-84); exports block_ptr, block_len, input(post, held, p0..p3) (:123-126), step(now_ms) (:130), rebase (:136); tick_hz free fn (:24). Block today 44 bytes (:157): tick, layout_hash, echo (held + 4 press words), link (sent, received, refused, post_used).
- N30. ADR 0002 part 3 table (:186-194): sim depends on content only; content on nothing above it; protocol on sim and content types; room and client-endpoint on sim, protocol, content; client-endpoint holds "the aim cast, the block writer and the block's layout table"; devtools "anything". Rules (:207-222): one feature set for sim, compared across server, devtools, navbake, sim-wasm.
- N31. ADR 0002 part 5 (:318-323): the post carries "a sequence number, the move input, the held-button bitmask and press and release edge counters ..., plus the camera's cursor ray and the view time of the frame the page drew"; the Worker casts the ray "against the state it published for that view time" and fills the command, aim point included. (:330-331) the control API = "create, mode flag, load the content bundle, one input call per tick, and the step" -> a content-loading call is in the draft, not per tick. (:344-346) Wasm calls per tick within R1-BND-REC1 mark 2's four. (:359-363) dev content: a Vite plugin (never in production) pushes changed files to the page, which posts them to the Worker; what a running match does is OQ-0005-CD4.
- N32. ADR 0002 part 5 (:288-292): header = tick, layout hash, counts per entity kind; aligned sections per entity kind; owner-only section for the local player; event section. (:303-305) the page keeps the two newest blocks and returns older ones to the pool. (:309-312) one layout table drives writer, offsets and hash; fixed widths, LE; each section aligned to its largest element type.
- N33. ADR 0005 CD-1 (:271-281): JSON, one canonical writer (fixed key order, shortest round-trip number text), serde_json with float_roundtrip; integers within 2^53; ids are strings. CD-2 (:282-335): stages byte cap -> untyped tree with depth limit -> format+version read, migrated -> typed load (deny_unknown_fields, checked newtypes, internally tagged, never untagged/flatten) -> garde rules -> whole-set checks -> frozen set with hash; every collection a max length; no panics (lints + fuzz). FALLBACK named in CD-2 (:299-301): "if [garde] does not [build on wasm32], checked newtypes and hand-written checks replace it and schemars attributes carry the ranges". Client entries rule (:320-330): movement data and maps need no client entry.
- N34. ADR 0005 CD-3 (:336-357): schemars (dev) writes JSON Schema 2020-12 with ranges + descriptions; ts-rs (dev) writes content's TS types; both committed in packages/types, diffed; derives behind cfg_attr(test) or a dev-only feature. CD-4 (:358-366): format name + version per file; additive via default/alias; breaking = version bump + migration N->N+1 on the untyped tree; future version refused. CD-5 (:367-376): one definition per file, one folder per kind, duplicate ids refused. CD-6 (:377-409): movement data is simulation content (the nav bake reads it too); durations authored in seconds, converted by ADR 0004 at take time with each rounding reported; tick rate never content; one top speed per unit type (:401-405).
- N35. ADR 0005 CD-7 (:410-430): dev reload; simulation content applies from the NEXT match (proposed; OQ-0005-CD4 open); client content live; an invalid set leaves the running one and reports. CD-8 (:431-450): one hashed bundle, not embedded in the Wasm (cs8). CD-10 (:464-472): legacy import, look group listed value by value, none becomes v2 content before ADR 0003's look proof test.
- N36. PM-0005 rows (docs/adr/0005:2296-2312): -2 corpus = every number in legacy-tuning.json (2,038 at 9e7db28) + 10,000 generated doubles + -0 and extremes; bits as 16 hex digits per side; -3 writer byte-identical native/Wasm, idempotent, 2^53+1 refused; RECORD every number whose canonical text differs from JSON.stringify's (:2299: "the two writers' number formats may differ for integral doubles and exponent forms"); -4 corpus C 10,000 seeded files, >= 500 per defect class, 1,000 controls, same verdict+error record natively/Wasm, depth limit recorded, garde-on-wasm32 recorded; -5 every collection bounded, one-over refused, at-bound accepted, schema scan; -6 panic lints (planted unwrap and buf[i]) + fuzz (cs4: bounded run); -7 schema range = validator range, just-outside refused + flagged by a Vitest schema-driven limit test, at-bound accepted; -8 regenerate 0 bytes, tsc strict, rename without regen fails, hand edit fails, import files typed vs generated types pass the validator; -10 local play: 20 saves pushed to the Worker validated there within 1 s, next local match runs on it, running match's per-tick hashes equal a control run; 20 malformed saves leave the match and show the error within 1 s; -14 every leaf accounted (725 leaves gameplay/simulation/progression at 9e7db28; 2,011 look leaves listed); -16 load time + bytes content adds (stub-loader build).
- N37. ADR 0005 migration plan (:2482-2484): foundation = content crate's place, generated schema+types diffed (CD-3), the dependency check keeping glTF readers out of sim and server (AP-1). Slice 1 (:2485-2487): CD-1..8 on weapons and movement, done when content items 1, 2, 6 and ability item 2 pass in CI.
- N38. legacy-tuning.json (tables T gameplay shamble-core.js:66): movement keys moveSpeed 6.3, sprintMult 1.4286, accel 14, jumpV 11, gravity 24, stepUp 0.55, adsMoveMult 0.55; STAM {max 7, regen 0.2, lockUntil 0.3} (shamble-game.js:2216); FIXED_DT 1/60 (legacy integrated at 60 Hz; v2 at 30); EJUMP_REACH 3.0708 = jumpV^2/(2*gravity) + stepUp = 2.5208 + 0.55 (my arithmetic) = START-RULES' 3.07 m. Camera keys in T: camHeight 40, camBack 56.6, fov 15, fovADS 15, adsCam "pitch", camYawBase 0.7854, camDeadX 0.6, camDeadY 0.52, camLerp 7, camYawLerp 14, camYawSens 0.006, camPushLerp 9, foxPanMax 16, lookAhead*/look*/pos* gains, weaponPitchMax 0.85; camKick* = feel (cs11). CAM_STYLES (progression group, shamble-game.js:2195): 5 presets ids 1-5 (1: 16/24/52/yaw 0; 2: 10.5/15.5/52/0; 3: 56.6/40/15/45deg; 4: 65.5/41.5/12/45deg; 5: 42/52/18/45deg, adsCam "pan").
- N39. LEGACY_INVENTORY CD5 (:87): 6 camera keys dead (camPosLean, camLookLean, maxLean, lookSideGain, posFwdGain, posSideGain); "Look brief pending"; port to TypeScript. GP5 (:316): presets Classic, Tactical, Iso, Iso+, Foxhole; Iso (3) is the code default though comments call Iso+ (4) "tuned default"; DEV.enabled true in legacy player build (cheats open to anyone) -> v2 panel dev-only.
- N40. Jump arithmetic (mine, for the seam): at 30 Hz semi-implicit Euler gives apex 2.34 m, explicit Euler 2.71 m, exact per-tick parabola samples 2.52 m (continuous 2.5208): the integrator form decides whether 3.07 m reach holds -> rust-reviewer's maths, the test names its tolerance.
- N41. Code: picture.ts (:18-31) PictureFrame = newest, previous (ArrayBuffer|null), timeMs; "How far between the two blocks a frame is drawn is the Worker's to say ... it comes as a field of the block with the first thing that moves" (:21-23); Picture = start, loop, draw, counters, resize, stop (:44-64); render/index.ts exports createPicture only. render/empty-scene.ts builds scene + PerspectiveCamera(50) at (0,14,16) (camera lives in render/ today). boot.ts:159-164 frame loop: link.frame(keys) then picture.draw({newest, previous, timeMs}).
- N42. Code: worker-link.ts reads block fields by generated offsets (:6-16), checks hash on first block of each Worker start (:263-267), exact length (:268) and >= BLOCK_BYTES (:270); keeps two newest, returns older with next post (:272-277, :317-333). messages.ts input = {post, held, p0..p3, returned} (:22-31); comment: "The view time of the drawn frame joins with the change-set that uses it, as a tick and a fraction, never as the page's milliseconds" (:18-19). host.ts POOL_SIZE = 4 (:37); step = input(6 numbers) then step(now) (:80-83); env.now() = Worker performance.now(); block message stamps startAt/doneAt on the shared clock (:197-199). worker.ts draws the seed with crypto.getRandomValues (:25) -> recordings need a fixed seed path (native only).
- N43. Archive (:1153-1158): NA-N3 = "who picks the blend fraction between two blocks (one sentence in picture.ts now, a field at change-set 4; events read from every block)" -> events must be read from EVERY block (cs8 seam), not only the two newest.
- N44. Code: layout.rs ElementType = U8, U32 only (:8-13, "A variant enters with its first use"); every section one row (:111-118 "a section with more rows brings its count into the header, and into this function, when it enters"); hash = FNV-1a over a tagged stream of names/types/offsets/strides (:199-215). Header tick @0, layout_hash @4 never move (:76-78).
- N45. ADR 0004 part 1 (:131-147): fixed-slot soldier table sized from the mode's team count and size; pools fixed at match start; no globals/statics in sim. Part 2 (:153-156): id = (kind, index, generation); soldier's slot index = its sim index; wire widths ADR 0001's. Part 3 (:161-177): authority step and prediction step over the same phase functions (prediction = cs7). Part 4 (:185-201): u32 tick only clock; dt build constant; durations in seconds converted at load into integer sub-tick units with the rounding of each value reported (units per tick from the rounding report). Part 6 (:249-251): step makes no heap allocation in steady state. Part 7 (:261-286): f32 for every real quantity in the sim; allow-list; min/max/clamp banned -> three helpers of our own (3A-SCAN-HELPERS lands with them); transcendental maths only via pinned libm; NaN never enters state (inputs checked finite where they enter); "No sim value is computed in TypeScript"; parry3d enhanced-determinism on both targets. Part 9 (:338-361): canonical state serialization, fixed widths, no usize; recorded match = seed, map and mode data, content hash, per-tick inputs. Part 10 (:362-380): parry3d queries + our own kinematic slide-and-step; one static world from the map document, immutable, each collider its sim id, BVH built once in id order; "whether soldiers block each other" a design question (START-RULES answered: soft push, never block).
- N46. ADR 0001 part 5 (:225-227): command = tick, sequence, move vector, button bitfield, press counter per button, quantised world aim point (<= 80 bits), view time (interpolation tick + fraction). Part 6 (:278-279): positions 1 cm in 16/16/12 bits (+-327.68 m, 0-40.95 m height), yaw 16 bits, pitch 10 bits; remote soldiers carry position, yaw, pitch, health, flag bits (crouch, sprint, ADS, reloading, carrying), weapon id (:305-309); generation travels in the record that brings the entity in; a generation change = removal + addition = "place, not slide" (:312-317).
- N47. Code: protocol/messages.rs Command already holds move_x/move_y (i8), buttons u16, presses [u8;16], aim: AimPoint {x i16 cm, y u16 cm, z i16 cm}, view_time {tick u32, fraction u16} (all provisional); OwnState = buttons + presses (the echo). AimPoint i16 cm = +-327.67 m: the PM-0004-39 far-corner 2 km test cannot be carried by today's command width (netcode's).
- N48. DECISIONS SLICE-FEEL (DECISIONS.md:1227-1232, his words): ADS = "I hold a key to aim"; old game's rules as Testlines has them: 0.55 of speed, cannot sprint, spread rests at sights value (cs8), camera reacts (the sights mode cs4 ports); "Coming out of a sprint, firing and aiming down sights each wait a short delay, set per gun"; starting values from his Testlines export; "Price and place both in change-set 4's plan: the input, the rule, the fields, what lands in change-set 8, and the lines they add." W1 and W4 are needs (:1234-1236). WB-HEALTH (a) (:1348): "Keep what crosses it as small as the plan has it (one type, plain numbers on every tick)". START-RULES (:1726): jump reach 3.07 m; "they push each other softly and never block".
- N49. PM-0004 rows at 4 (docs/adr/0004:1127-1181): -6 counting allocator, 0 allocations over measured ticks (cs4: N = whatever the test scene holds; burst form cs9); -11 rounding report over every duration in legacy-tuning.json, units-per-tick chosen and written, a planted 0.02 s flagged; -15 proptest: 10,000 cases of up to 1,000 spawn/despawn ops on a 64-slot pool, stale ids resolve to nothing (bite: no generation raise); -17 two instances stepped in turn = solo runs (bite: a static counter); -19 native x2, Node, Chromium (more later); -20 parry3d enhanced-determinism native vs Wasm + query time flag on/off (reading); -21 three injections (std sin, HashMap order, presentation draw) caught at their tick + digest/serializer same on 64 and 32 bit; -22 rerun when a function joins (libm functions, sqrt, the three helpers on signed zeros); -24 no NaN in state (replay runner reads floats by to_bits) + fuzzed commands' NaN rejected where they enter (bite: planted 0.0/0.0); -25 SIMD reading; -30 on LB: step 0.55 up, 0.56 stops, apex = integrated within 1 mm, 0 tunnellings through 0.2 m wall in 10,000 moves at 13.27 m/s; -32 BVH byte-identical native/Wasm, build <= 50 ms native, 10,000 rays after restore + rebuild; -33 own Wasm size + rebuild <= 10 s (comparison builds separate); -35 0 Err(Unsupported) (capsule vs boxes, wedges, convex, ground); -39 far corner of a 2 km map; -54 downward cursor ray onto a ramp within 1 cm of the slope, 1,000 of 1,000 (bite: ramps as boxes, legacy GC3); -59 reach = measured apex + step-up within 1 mm for jumpV 11, gravity 24, stepUp 0.55 (bite: legacy's closed form); -60 fuzzed commands into a running sim, 0 panics (cs4: movement commands; nightly is CI work).
- N50. Jump arithmetic for START-RULES vs PM-0004-59 (mine): exact per-tick parabola y_k = v0*k*dt - g*(k*dt)^2/2 samples a 2.52 m apex at k = 14 (continuous 2.5208), so reach 3.0700 vs formula 3.0708: 0.83 mm, inside -59's 1 mm and START-RULES' 3.07 m; semi-implicit Euler (2.34 + 0.55 = 2.89 m) would break START-RULES. -> rust-reviewer: the integrator must be the exact per-tick parabola (or the plan states the departure).
- N51. Code: room/src/lib.rs:168-175 states the checks not made yet ("the length of the move and the aim inside the map (the soldier and the map, change-set 4)"); room runs up to CATCH_UP_CAP commands per tick and authority_step keeps the newest (:251-279) -> with movement each command run must be a movement sub-step or late commands lose movement (netcode seam; security M1's "double speed" note). local_match.rs:19-20 NO_CONTENT = [0;32] content hash placeholder "the content crate brings the real one" (cs8 brings SHA-256 with the bundle; cs4 may keep the placeholder). world.rs: SHARED_STATE_BYTES prefix + seed last; shared_hash zeroes the seed tail (:127-133); state_hash.rs:5-7 FNV-1a 64 provisional "PM-0004-21's report ... confirms or replaces it before the first recording is committed".
- N52. cs3b report: CI toolchain job 4 min 58 s of 15; 16 browser tests in 3.4 of 5 minutes (the CI browser limit) -> Walk's new browser specs + Chromium replays risk the 5-minute limit (a CI edit = the CI Builder's, asked of him). Players' Wasm 53,813 B on 6abf3d2a.
- N53. ADR 0003 part 10 (:451-466): per frame: take newest blocks keeping two; interpolate between the two newest ticks; per-instance arrays; nothing allocates per frame; "the cursor ray for the aim point hits the interpolated, drawn state (I-19)".
- N54. serve.mjs:44 CSP has connect-src 'self' (a same-origin content fetch by the Worker is allowed); vite.config.mjs server.fs.allow lists the served folders (content/ is not among them). build-checks.mjs:133 DEV_MARKERS list; production scan fails on any marker, development build fails if a marker is missing.
- N55. No content/ folder exists yet; REPO_MAP has no content entry.
- B0 DONE (reading). Next: B1, the content crate.
- N56. Official docs read (2026-11-25): https://doc.rust-lang.org/std/primitive.f64.html, FromStr for f64: "Otherwise, Ok(n) where n is the closest representable floating-point number to the number represented by src (following the same rules for rounding as for the results of primitive operations)"; its grammar also accepts inf, nan, "5.", ".5", "007" (so a JSON reader must check RFC 8259's number grammar before calling it). https://doc.rust-lang.org/std/fmt/index.html and the f64 page state NOTHING about shortest round-trip output for {} / {:?} / {:e}: a writer of our own that relies on std's shortest digits rests on an undocumented behaviour, to be proven by test against JSON.stringify over PM-0005-2's corpus.
- N57. LOOP_V2 §9 (:158): "[DEFAULT] Greybox first: plain blocks, with height read through brightness (higher is lighter)". LOOP_V2 (:125): "[DECIDED] Every gun gets a short sprint-to-fire delay, longer for heavy guns." Legacy ADS = right mouse (button 2) held ('hold' default, 'toggle' option) (shamble-game.js:3284-3285); middle mouse rotates the camera (rotating), left fires.
- N58. Part 5 / DECISIONS "Queued": "Slice 1 ships with a live camera tuning panel from its first playable build" -> if Walk is cut, the camera panel must be in the FIRST part (the first merged playable build); an early-look zip is not a shipped build.

## Block 1. The content crate (DONE)

### 1.1 Files and folders it brings (lane 1 throughout, Part 15.2 :90-95)
- `crates/content/` (new crate, the sim's only dependency: ADR 0002 :186-187). `src/lib.rs` (re-exports, crate lints:
  `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing, clippy::panic, clippy::unreachable,
  clippy::todo)]` from its first line, R5-RUSTLINT-REC3 via ADR 0005 CD-2 :290-291), `src/json.rs` (bytes -> tree),
  `src/number.rs` (RFC 8259 number grammar, parse, canonical number text), `src/write.rs` (the one canonical writer),
  `src/read.rs` (typed reader with field paths), `src/field.rs` (the field tables), `src/schema.rs` (JSON Schema
  2020-12 from the tables), `src/error.rs`, `src/bundle.rs` (bundle format, the file set, whole-set checks, the frozen
  `ContentSet`), `src/movement.rs`, `src/map.rs`, `src/camera.rs`, `src/ts_export.rs` (`#[cfg(test)]`, writes TS
  types and schemas like protocol's `ts_export.rs`, "content's export test shares the folder rules": cs1 report :264).
- `content/` (new, root): `content/movement/soldier.json`, `content/maps/test-scene.json`, `content/camera/camera.json`.
  One definition per file, one folder per kind (CD-5). Ids are strings ("soldier", "test-scene", "camera": working
  names, the game-designer's or his to rename). No modes in cs4: modes are Slice 2 (ADR 0005 :2488-2490); the local
  match's setup (map id, slot count) stays code in `sim-wasm` until then.
- `packages/types/src/generated/content/` (ts-rs: `Movement.ts`, `MapDocument.ts`, `Shape.ts`, `Spawn.ts`,
  `Camera.ts`, `CameraPreset.ts`, `ContentBundle.ts`) and `.../content/schema/<kind>.schema.json` (our writer), both
  committed and diffed by the export test; exported through `packages/types/src/index.ts`.
- `crates/devtools/src/bin/` (native tools; rust-reviewer checks that bins leave the wasm32 test module's build alone,
  3A-RULES-STEP): `content-bundle` (writes `content.json` for the build; `--check` refuses any file under `content/`
  that is not byte-identical to its canonical rewrite), `legacy-import` (writes `docs/legacy-import-report.json`;
  `--check`), `testlines` (`--check`: content against his saved Export through the table, his point 8 d). Each tool's
  core is a library function that a `#[test]` also runs, so CI's existing `cargo test` catches drift with no CI edit.
- `docs/tuning/` (new): `testlines-export-<date>.json` (his Export, saved by script, whole; byte count and sha256 in
  `testlines-export-<date>.sha256` and in the commit message), `testlines-table.json` (one row per value: Export path,
  content path, unit, change-set; rendered as a table in the plan and report). `docs/legacy-import-report.json`
  (generated, committed).
- Build: `packages/client/scripts/build.mjs` runs `content-bundle` and copies `content.json` into the page's folder;
  `build-checks.mjs` refuses a page folder without it. `packages/client/vite.config.mjs` gains the development content
  plugin (1.9). `packages/client/serve.mjs` already allows a same-origin fetch (CSP `connect-src 'self'`, serve.mjs:44).

### 1.2 The outside crates ADR 0005 would bring, against a small reader of our own (question CONTENT-SOURCE)
What ADR 0005 names: serde_json "built with `float_roundtrip`" (CD-1 :274), serde's `deny_unknown_fields`,
`try_from`, internally tagged kinds (CD-2 :284-285; R1-DATA-F1..F5), serde_path_to_error for field paths (F8 :216),
garde for ranges (CD-2 :285, F17-F18), schemars for the schema (CD-3 :337), ts-rs for types (CD-3 :342; already a
workspace dev-dependency, root Cargo.toml:13). Three options:
| Option | Crates that ship in the players' Wasm | Build-time only | Our own code (block 1, code lines) | Gains | Costs, unknowns |
|---|---|---|---|---|---|
| (a) ADR 0005 as written | serde, serde_json (+ its itoa, ryu, memchr), serde_path_to_error, garde (+ what its features pull) | serde_derive, garde_derive, schemars(+derive), their proc-macro stack | about 1,180 | least code of ours; widely used, fuzzed parser; documented exact parse (`float_roundtrip`, F6) and shortest output (ryu) | 5 to 8 runtime crates under the dependency hold, each with an overflow-override line (root Cargo.toml:31-34 "A new third-party crate joins this list"); garde on wasm32 unread (R1-DATA-OQ5, F18); serde_json's writer text differs from `JSON.stringify` for integral doubles and exponent forms (the draft's own note, docs/adr/0005:2299), so the camera panel's "copy values" text is not canonical without a Rust pass; per-type deserializer code in the Wasm, unmeasured |
| (b) serde_json's untyped `Value` + our typed reader, ranges and schema | serde, serde_json, itoa, ryu, memchr | none | about 1,450 | no proc-macros; ranges and schema ours (CD-2's own fallback, :299-301) | duplicate keys pass silently (the untyped map keeps the last); the same writer-text mismatch; 5 runtime crates under the hold |
| (c) all ours (recommended) | none | none | about 1,890 | 0 outside code in the loader and in sim's tree from content; std's documented closest parse (N56); number text laid out as ECMAScript's Number::toString, so `JSON.stringify` and the canonical writer agree on every finite double but -0 (written "-0"), and the camera panel's copy is canonical by construction; duplicate keys, NaN, Infinity, depth and size refused by our own stages; R1-DATA-OQ5 dissolves; R1-DATA-OQ4's depth limit is ours by construction | about 710 more code lines and about 420 more test lines than (a) (a parser, a number writer, a typed reader, a schema writer); shortest digits rest on std's undocumented `{:e}` behaviour, proven by test against `JSON.stringify` over PM-0005-2's 12,038 numbers; a parser of ours must be fuzzed (bounded run here, the nightly is CI work, PM-0005-6) |
- Recommendation: (c). The same reasoning as his RNG-SOURCE (c) (DECISIONS.md:1066): one implementation, ours, on every side, no
  code from the internet in the simulation's tree beyond parry3d; the draft already names hand-written checks as garde's
  fallback (CD-2 :299-301). Departures it brings (for the DEPART list): CD-1's "serde_json is built with
  float_roundtrip", CD-2's serde attributes and garde, CD-3's schemars; PM-0005-7's "garde range" read as "the field
  table's range"; PM-0005-4's "whether garde builds and runs on wasm32" answered "not used".
- What rust-reviewer must check (its parallel design), under (c): that `str::parse::<f64>` is only ever handed RFC 8259
  number text (the std grammar also takes inf, nan, "5.", ".5", "007": N56); that `{:e}` on f64 gives the shortest
  round-trip digits on native and wasm32 (std source, `core/src/fmt/float.rs`), and the plan's test against
  `JSON.stringify`; the parser's recursion bound (depth cap enforced before descending, or iterative) and that no path
  indexes or unwraps (the crate lints); integers above 2^53 refused; UTF-8 and escape handling (lone surrogates
  refused on read; escaping on write identical to `JSON.stringify`'s set). Under (a) or (b): exact versions; features
  (serde_json `float_roundtrip` and `std` only: no `arbitrary_precision`, no `preserve_order`, which brings a hash
  map crate; garde with no regex, url, email or phone features); whether garde builds and runs on
  wasm32-unknown-unknown; each crate's `unsafe`, MSRV against the 1.99.1 pin, transitive crates, overflow-override
  lines; serde_json's default recursion limit (R1-DATA-OQ4); 3A-FEATURE-VERSION (the feature compare matching by name
  only, STATUS_ARCHIVE.md:1391-1394) once serde sits in sim's tree. In both cases parry3d (block 2.8).

### 1.3 The loader's stages (CD-2 :283-288), each with what refuses
1. Byte cap: a file at most 1 MiB, a bundle at most 8 MiB (proposed values, reviewed like code, CD-2 :298); over it:
   `TooLarge { file, bytes }` before any parse.
2. JSON to an untyped tree (`json::parse`): RFC 8259 only; depth at most 32 (proposed; refused before descending);
   duplicate keys refused (`DuplicateKey { file, line, col, key }`); NaN, Infinity, leading zeros, a trailing comma, a
   byte-order mark refused; every node keeps its line and column. Strings: escapes per RFC 8259, a lone surrogate
   refused. Numbers: grammar first, then `str::parse::<f64>`; an integer-looking number above 2^53 (9,007,199,254,740,992)
   refused (PM-0005-3's bite, docs/adr/0005:2299).
3. Format and version: `format` must be the kind's name (`shamble-movement`, `shamble-map`, `shamble-camera`), `version`
   a whole number; a version above the loader's newest is refused with the file, its version and the newest known (PM-0005-9's
   wording, met early); migrations run here on the tree (none in cs4: every kind is at version 1).
4. Typed load (`read::Object`): each field read by name with its path (`movement.walk_speed`); an unknown key refused
   (`UnknownKey { file, path }`); a missing key refused unless the table gives a default (CD-4's additive rule); kinds
   inside a document internally tagged by `"type"` (map shapes), never untagged.
5. Ranges, from the field table: every number finite and inside `[min, max]` with its inclusive or exclusive form
   (`OutOfRange { file, path, value, min, max }`); every array at most its `max_items` (`TooMany { file, path, max }`).
6. Whole-set checks (`bundle::freeze`): one definition per id per kind (`DuplicateId`); every reference resolves
   (the camera's `default_preset` to a preset id; every spawn and shape id unique in its map; a shape inside the map's
   bounds); no fallback anywhere. The client-entry rule (CD-2 :320-330) binds units, vehicles, weapons and factions,
   none of which exists in cs4, so it enters with weapons (cs8); movement, maps and the camera need none.
7. Frozen `ContentSet` (read-only; `&ContentSet` handed to the sim's take step, 2.6). The set's hash is cs8's (CD-8;
   SHA-256 and WebCrypto: PM-0005-12 at 8); until then `NO_CONTENT` (local_match.rs:19-20) stays in `Join`.
Every error names the file and either line and column (syntax) or a field path (data); `Error::message()` gives the
plain text the Worker posts. The loader returns `Result` everywhere; nothing in `content` can panic on bytes.

### 1.4 The canonical writer and the number text (CD-1 :271-281)
- `write::canonical(&Tree) -> String`: keys sorted bytewise at every level (one rule both languages can follow
  without the schema), arrays in order, two-space indent, `": "` after a key, one item per line, a final newline: the
  same bytes as `JSON.stringify(sortedKeys(value), null, 2) + "\n"`.
- Numbers: the shortest round-trip digits (from `{:e}`), laid out as ECMAScript's Number::toString (plain digits up
  to 1e21, exponent `e+n`/`e-n` beyond, `0.000ddd` down to 1e-6), except -0, written `-0` so that it reads back as -0
  (PM-0005-2's corpus has -0; `JSON.stringify(-0)` gives "0"). Strings escaped exactly as `JSON.stringify` escapes.
- Idempotent: write, parse, write gives the same bytes and the same bits (PM-0005-3).
- TypeScript writes content in one place in cs4, the camera panel's "copy values" (block 4): `sortedKeys` plus a
  five-line `numberText(x)` (`Object.is(x, -0) ? "-0" : String(x)`); a Vitest test compares its output with the Rust
  writer's bytes for the committed camera file.

### 1.5 Format name and version per file (CD-4 :358-366)
Every file: `"format": "shamble-<kind>"`, `"version": 1`, `"id": "<id>"`, then its fields. The bundle:
`{"files": [{"path": "movement/soldier.json", "text": "<the file's exact bytes as a JSON string>"}, ...],
"format": "shamble-content-bundle", "version": 1}` (sorted keys), files in path order. Raw texts inside one bundle keep
each file's own line and column for errors (the development path validates pushed files in the Worker) and let
JavaScript assemble a development bundle byte for byte as the Rust tool does. The production bundle is written by
`content-bundle` (Rust), after its `--check` that every file is canonical.

### 1.6 The kinds of cs4, their fields, and what is data against code
Movement (`content/movement/soldier.json`, simulation content, CD-6 :384). Each field: unit, legacy source
(docs/legacy-tuning.json `T`, shamble-core.js:66; `STAM`, shamble-game.js:2216), starting value from his Export where
it has the value (his point 8 b), legacy otherwise; ranges are proposals for review:
| Field | Unit | Legacy key, value | Range (proposed) |
|---|---|---|---|
| walk_speed | m/s | T.moveSpeed 6.3 | (0, 20] |
| sprint_multiplier | x | T.sprintMult 1.4286 | [1, 3] |
| acceleration | m/s^2 | T.accel 14 | (0, 200] |
| gravity | m/s^2 | T.gravity 24 | (0, 100] |
| jump_speed | m/s | T.jumpV 11 | [0, 30] |
| step_up | m | T.stepUp 0.55 | [0, 1.5] |
| stamina_max | s | STAM.max 7 | (0, 60] |
| stamina_regen, stamina_lock | as legacy's code uses them (unit read at the build from shamble-game.js:2216's users; Not verified) | STAM.regen 0.2, STAM.lockUntil 0.3 | reviewed |
| capsule_radius, capsule_height | m | none in legacy (BODY's radii are hit volumes, shamble-core.js:1482) | open: COLLIDER-SIZE, for the game-designer and him; the stand-in is 1.80 m to the hat top, a placeholder (appendix A4 :130) |
| ads_multiplier | x | T.adsMoveMult 0.55 (SLICE-FEEL: "walks at 0.55 of his speed") | (0, 1] (only if ADS is in the part that builds it, block 5) |
The jump's reach (3.07 m, START-RULES) is not a field: it is derived (apex at 30 Hz + step_up) and asserted by test
(block 2.7; N50). Code, never data: the tick rate (CD-6 :394; ADR 0004 part 4), the integrator's form, the
slide-and-step iteration cap and skin width, the units per tick (a sim constant chosen from the rounding report, 1.11).

Map (`content/maps/test-scene.json`, simulation content; CD-11's schema fields are EDITOR_V2's later plan, :490):
`bounds {min:[x,y,z], max:[x,y,z]}`, `spawns [{id, at:[x,y,z], facing_deg}]` (at most 16), `shapes [{id, group,
...}]` (at most 512), each shape internally tagged: `{"type":"box", "min":[x,y,z], "max":[x,y,z]}` or
`{"type":"ramp", "min":[..], "max":[..], "rises_toward":"+x"|"-x"|"+z"|"-z"}` (a wedge inside its box), and `regions
[{id, kind:"street"|"grass", min:[x,z], max:[x,z]}]` (drawn only, no collision). Axis-aligned only in cs4, so no
trigonometry enters the static world; a yawed shape comes when a map needs one. The static index of a shape is its
rank by id (bytewise), so the BVH is built "once at match start in id order" (ADR 0004 :365-366) whatever the
authoring order. ONE document holds both parts, grouped `street` and `yard`, so each shape has one home and "the tests
and the play share them" (build plan :349-350): tests spawn in the yard by spawn id. Its contents:
- street corner (appendix A7 :343-344): house A 6.1 x 4.4 m, eaves 2.6, ridge 4.7 (a box to the eaves and two
  ramps); house B 4.6 x 3.8, eaves 2.6, ridge 4.3; the alley 1.5 m between them; the cover wall 3.1 x 0.45 x 1.0;
  the street region 4.3 m wide; the rest grass region.
- yard (the marks' shapes): `step_055` (0.55 high) and `step_056` (0.56), the ledge block LB (PM-0004-30); a 0.2 m
  thin wall (PM-0004-30's tunnelling); ledges at 2.9, 3.05 and 3.1 m to feel and test the 3.07 m reach (PM-0004-59;
  START-RULES); platforms with ramps of two slopes (PM-0004-54, PM-0004-35's wedge pair); a 50 m lane with posts
  every 10 m (build plan :350). Exact placements: the plan's own table.
- Colours are not in the document (look: question GREY-COLOURS, block 4).

Camera (`content/camera/camera.json`, client content: the sim never reads it, CD-6 :385-388): exactly the live keys
of legacy's CD5 (LEGACY_INVENTORY.md:87: height 40, boom 56.6, fov 15, fov_ads 15, yaw 45 degrees, ads_mode "pitch",
dead zone 0.60 x 0.52, the follow, yaw and push rates, pan_max, the look-ahead and position gains that legacy reads;
the six dead keys not ported) and GP5's five presets (Classic, Tactical, Iso, Iso+, Foxhole: CAM_STYLES,
shamble-game.js:2196-2200), `default_preset` "iso" (legacy's code default, GP5's note: the comment calls Iso+ the tuned
default; his pick). The exact live key list is confirmed by grep at the build.

### 1.7 Where he tunes once cs4 exists: one home per value (his point 8 g)
| Values | Their one home | How he tunes | How a change lands |
|---|---|---|---|
| movement, jump, sprint, stamina (and ADS's rule numbers when built) | `content/movement/soldier.json` | in his Testlines sandbox | he pastes a new Export; the builder saves it by script and rewrites the file through `testlines-table.json`; `testlines --check` (and its `cargo test`) fails while the file differs from the saved Export |
| the camera's framing and presets | `content/camera/camera.json` | live, in the panel (backquote), in his zip | "copy values" gives the file's exact bytes; he pastes them; the builder saves them by script; `content-bundle --check` proves them canonical |
| per-gun delays, spread at sights (SLICE-FEEL's cs8 half) | each weapon's file (cs8) | Testlines | as movement |
| anything Testlines also holds for the camera (if its Export has camera values) | the camera file, unless he says otherwise | question CAM-HOME | |
The saved Export is an input record, never loaded by the game; the content file is the value's one home in the game.

### 1.8 How content reaches the simulation, and the page, without breaking WB-HEALTH
- Bindings: the bundle crosses ONCE per match, through the constructor, as the seed already does:
  `Sim::new(seed: &[u8], mode: u32, pad_bytes: u32, bundle: &[u8]) -> Result<Sim, JsError>` (`Sim::create` gains the
  same argument and the loader's error as a `CreateError::Content(String)` variant). No new export; the per-tick calls
  stay plain numbers (DECISIONS.md:1348). ADR 0002's control API names "load the content bundle" (:330); binding it to
  construction makes "one content set per match" (CD-7) true by construction: no call can swap content mid-match, which
  is also OQ-0005-CD4 (a)'s behaviour. Boundary test: a malformed bundle makes the constructor throw an `Error` whose
  text names the file and the line or path, never a trap.
- Route in a zip: the page fetches `./content.json` once (same origin), keeps the parsed camera and map for drawing
  (`JSON.parse`, typed by the generated types; read field by field, never merged into objects, CD-9 :454-455), and
  transfers the bundle's bytes to the Worker in the `start` message (`{type:"start", padBytes, bundle: ArrayBuffer}`);
  it draws the scene only after the Worker's `ready`, which means the Rust loader accepted that bundle (one verdict).
- In Rust: `LocalMatch::new(setup, ..., &ContentSet)`: the room's sim takes `sim::Rules` and the static world from the
  set; the client endpoint builds its own copy of the static world from the same set for the aim cast (it never reads
  the authority's state: ADR 0002 :413-414); one parse per match.

### 1.9 Development reload (CD-7; PM-0005-10 at 4)
- The Vite plugin (`apply: "serve"`, so never in any `vite build` output, the zip included): serves `/content.json`
  assembled from `content/**` exactly as the bundle format says; watches `content/`; on a change sends the custom event
  `shamble:content`. The page (`import.meta.hot?.on`, which Vite removes from builds) refetches and posts
  `{type:"content", bundle}`; the Worker builds a new `Sim` with it: refused -> the running match goes on and the
  Worker posts the loader's text (the readout's error line, within 1 s); accepted -> the next match starts on it at
  once (the soldier back at his spawn), the page reapplies the camera values (client content applies live, CD-7).
- Tests: the Worker's half in a browser spec on the built development page through a development-only hook that
  injects a bundle (20 valid, 20 malformed; verdict time measured in the page; the running match's per-tick hashes up to
  the swap equal a control run's: the mark's bite "a Worker that applies the pushed numbers to the running match");
  the file-watch half (save -> event) measured once by a script against Vite's dev server, 20 saves, max reported in the
  report (a reading: CI runs built pages only, playwright.config.ts:31-32).

### 1.10 The schema and TypeScript types (CD-3; PM-0005-7, -8 at 4)
- The field tables drive the reader's ranges, the schema's `minimum`/`maximum` (or exclusive forms), `maxItems`,
  `description` and `default`; ts-rs (dev-dependency, `cfg_attr(test, derive(ts_rs::TS))`) writes the types. Neither
  reaches a shipped build (CD-3 :344-348).
- The first real consumer is in cs4: the camera panel's sliders take their limits from `camera.schema.json` (CD-3
  :338-340, "minimum and maximum become slider limits"), so PM-0005-7's "flagged by the schema-driven field" is a Vitest
  test of the panel's limit function for camera fields, and the same function's test for movement fields.

### 1.11 The legacy import (CD-10; PM-0005-14) and the rounding report (PM-0004-11)
- `legacy-import` walks every leaf of `docs/legacy-tuning.json` (725 gameplay, simulation and progression leaves,
  2,011 look leaves at 9e7db28: docs/adr/0005:2310): each gets a row: mapped (v2 path + legacy `where`), dropped (with a
  reason: the dead keys of CD4, CD6, CD7, CD10, CD12, CD14; the state group), listed (look: for ADR 0003's look proof
  test), or "maps in cs8 / Slice 2 / Slice 4" (the build plan's reading of PM-0005-14 at 4: "the walk over every leaf from
  its first run; movement mapped", :737). Where his Export replaces a legacy value, the row says so and gives both: the
  mark's "every kept value equal to legacy's, exactly" (docs/adr/0005:2310) reads as "every value not replaced by his
  Export" (a reading for the plan to state; his tunings win, point 8 c).
- The rounding report (PM-0004-11, :1132): the same tool lists every legacy duration (a hand list of the duration
  leaves, about 40, written beside the tool) in sub-tick units at the chosen units-per-tick with each rounding;
  "the chosen units-per-tick figure is written into this record": into cs4's report; ADR 0004 itself is edited only in a
  docs change-set on his go (build plan :760-761). A planted 0.02 s interval is flagged.

### 1.12 The marks of block 1, each with its test and its planted defect
| Mark (where the plan puts it) | Test | Planted defect it must catch |
|---|---|---|
| ADR 0005's foundation line (build plan :461) | `cargo tree -e normal -p sim` equals {content} (+ parry3d's tree, 2.8) in a build check; the export test regenerates schema and types into scratch and diffs | `protocol` added to content's dependencies; a field renamed without regenerating; a hand edit of a generated file |
| CD-1, content item 1, PM-0005-2 (:725) | `tests/corpus.rs`: 2,038 legacy numbers + 10,000 generated (5,000 random bit patterns, 5,000 decimals up to 17 digits, -0, extremes), bits as 16 hex digits natively; the same through the test-only module (`devtools::wasm::content_corpus`) in Node and in Chromium; `JSON.parse` in Node (Vitest) and Chromium (rules spec); a script compares | a parser that rounds through f32 |
| PM-0005-3 (:726) | the writer over the corpus: same text, idempotent, byte-identical native/Wasm; the list of texts differing from `JSON.stringify` is empty but for -0 | a writer printing 15 significant digits; 2^53 + 1 in a file accepted |
| CD-2, content item 2, PM-0005-4 (:727) | a seeded generator (our ChaCha8) makes 10,000 files from valid movement, camera and map seeds, >= 500 per defect class (unknown key, `__proto__` key, wrong type, out of range, 10,000-deep nesting, duplicate key, duplicate id, dangling reference, over-bound array, bad format/version, bad UTF-8) and 1,000 controls; 10 files of 50 MB made in the run; same verdict and error record natively and in Wasm (the test module) | an unknown key let through; a fallback on a dangling `default_preset`; a depth check after descending |
| PM-0005-5 (:728) | every array of every kind has `maxItems` (a scan of the generated schemas); one over refused, at the bound accepted | a `Vec` field added without a bound |
| PM-0005-6 (:729) | the crate lints at deny; a bounded run of 1,000,000 random and mutated byte strings in release, 0 panics (the nightly cargo-fuzz job is CI work) | a planted `.unwrap()` and `buf[i]` fail clippy; a planted `.unwrap()` on a parsed id is found by the run |
| PM-0005-7 (:730) | the table's ranges equal the schema's; just outside refused by Rust and flagged by the panel's limit function (Vitest, from the generated schema); at the bound accepted by both | a range edited without regenerating; an inclusive bound written as exclusive |
| PM-0005-8 (:731) | regenerate: 0 bytes; `tsc --strict` on the generated types; each content file typed against its type (a `.ts` that imports the JSON with `satisfies`) and loaded by the crate | as the foundation line |
| PM-0005-1 (:724) | every content test prints a header (target, rust version, machine) | none (a precondition) |
| PM-0005-10 (:733) | 1.9 | a Worker that swaps the running match's numbers (hash comparison fails); a watcher that waits for the next match start |
| PM-0005-14 (:737) | `legacy-import --check` and its `cargo test`: every leaf one row; movement mapped; look listed | a leaf planted in a copy of the file shows as unaccounted; a kept value moved by one ulp fails its golden |
| PM-0004-11 (:1132) | the rounding report's rows and the chosen figure | a planted 0.02 s interval flagged |
| PM-0005-16's size half (:564, :739) | a separate comparison build (lane 2's by Part 15.2 :105): the Wasm with the loader against a stub loader, a patch, never merged; cs4 itself records the players' Wasm size and the per-crate split by its name section (perf-auditor's method, perf-auditor-review2.md:28-29) | none (a reading) |
| CD-7 / CD-8 | CD-7 by 1.9; CD-8 (bundle hash, tampering cases) is cs8's (build plan :735) | |

### 1.13 Size of block 1 (bottom-up, option (c); plus or minus 30 %)
| Part | Code | Tests |
|---|---|---|
| json.rs, number.rs, write.rs, read.rs, field.rs, schema.rs, error.rs | 790 | 500 |
| bundle.rs (format, set, whole-set checks, freeze) | 160 | 100 |
| movement.rs, map.rs, camera.rs | 300 | 160 |
| ts_export.rs (test-only) | 0 | 80 |
| content item 1 corpus (Rust + test module fn + Vitest + Chromium) | 20 | 250 |
| content item 2 corpus generator and runs, bounded fuzz | 0 | 280 |
| devtools bins: content-bundle, legacy-import (+ rounding report), testlines | 370 | 210 |
| build.mjs, build-checks.mjs, Vite plugin | 120 | 90 |
| sim-wasm constructor takes the bundle; boundary tests | 30 | 30 |
| Total | 1,790 | 1,700 |
Under (a): about 1,080 code and 1,280 tests (the parser, number, writer, typed reader and schema writer replaced by
derives and about 100 lines of glue). Not counted (the plan's rule, build plan :235-237): the map document (about 150
hand-written JSON lines; counted as configuration if the plan prefers), the movement and camera files (written by
script), generated types, schemas and the import report.
- B1 DONE. Next: B2, the simulation's structure.
