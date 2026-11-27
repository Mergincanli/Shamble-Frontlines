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

## Block 2. The simulation's structure (DONE)

### 2.1 Modules of `sim` after cs4 (no globals or statics: ADR 0004 :145-146)
| Module | Holds | Imports |
|---|---|---|
| `world.rs` (exists) | `World`: tick, player records, soldier table, then the seed last; the authority step; `write_state`, `state_hash`, `shared_hash` | the rest |
| `ids.rs` (new) | `SoldierId { index: u16, generation: u16 }`, packed in a `u32` for the block and the tests; resolve-or-nothing | |
| `soldiers.rs` (new) | the fixed-slot soldier table (ADR 0004 :131), sized at match start, iterated in index order | ids |
| `time.rs` (new) | `UNITS_PER_TICK` (code constant, chosen from the rounding report, 1.11), `Units(u32)`, `from_seconds(f64) -> (Units, rounding)` | |
| `rules.rs` (new) | `Rules`: the movement numbers as f32 and `Units`, made once by `Rules::take(&content::ContentSet, movement_id)` with its `Rounding` rows; never read from content inside the step | content, time |
| `fmath.rs` (new) | part 7's three helpers (`min_f32`, `max_f32`, `clamp_f32`, plain comparisons, a defined argument on equal signed zeros, no panic on reversed bounds: ADR 0004 :271-275), `finite` checks | |
| `collide.rs` (new) | the ONE module that imports parry3d (ADR 0004 :374-375: "behind one sim module"): `StaticWorld::build(&content::MapDocument)` (shapes ranked by id, the BVH built once), capsule shape casts, the ray query (aim), the ground probe, a caller-owned `QueryScratch` | parry3d, content |
| `movement.rs` (new) | our own kinematic move (ADR 0004 :362-366): walk, sprint with stamina, jump, step-up, slide, the soft push between soldiers | collide, fmath, rules, time |
| `rng.rs`, `state_hash.rs` (exist) | unchanged; no stream draws in cs4 (spawns come from the map's spawn points), so no stream is created ("when a phase first draws", world.rs:110) | |
- The movement functions take `(&Rules, &StaticWorld, &mut Soldier, &PlayerCommand, &mut QueryScratch)`, never `&World`,
  so cs7's prediction step calls the same functions on its own copy of one soldier (ADR 0004 part 3, :169-176: "two
  entry points over the same phase functions"). The push between soldiers reads the whole table: authority only.

### 2.2 The soldier table, stable ids and generations (PM-0004-15; ADR 0004 part 2)
- One slot per player slot ("A soldier's slot index is its sim index", ADR 0004 :155); capacity = the match's slot
  count, given by the setup (cs4's local match: 1; tests: up to 64), allocated in `World::new` and never grown.
- A slot: `generation: u16` (raised, wrapping, at every spawn into it), `occupied` and `alive` bits (alive is always
  set until cs8's deaths; Part 15.2 :124-126 wants the stand-in to show dead and respawned from a block field before
  cs5 merges: block 3 carries the bit and the generation from cs4), `team: u8`, `position [f32;3]`, `velocity
  [f32;3]`, `grounded`, `sprinting`, `stamina: Units`, `aim [f32;3]` (the applied command's aim point, clamped into
  the map's bounds), `facing [f32;2]` (the horizontal unit vector toward the aim, from `sqrt` only), and, if ADS is in
  the part that builds it, `aiming` (block 5).
- `resolve(SoldierId) -> Option<&Soldier>` is `None` for a free slot or another generation.
- PM-0004-15's test: 10,000 cases of up to 1,000 spawn and despawn operations on a 64-slot table, each despawned id then
  resolves to nothing; the operations drawn from our own ChaCha8 with a fixed seed (no proptest crate: the same choice
  as RNG-SOURCE (c); a departure from the row's "a proptest property", docs/adr/0004:1136, for the DEPART list; the
  nightly fresh seeds are CI work). Bite: a table that does not raise the generation on reuse. This is also the card's
  "ids over random spawns" (build plan :354).

### 2.3 The tick's loops and their order (ADR 0004 part 1: one fixed order, from legacy's GP14)
The loops outside the step are unchanged: the Worker's host paces (pacing.ts), the step takes no time (ADR 0004 :186).
`World::authority_step(&mut self, commands: &SlotCommands)` (a fixed array: up to `CATCH_UP_CAP` commands per slot, no
allocation), per tick:
1. Commands, per slot in index order, oldest command first: each command is checked where it enters (every float
   finite; a move longer than 1 scaled to length 1; the aim clamped into the map's bounds: security-auditor's L1,
   room/src/lib.rs:168-171), and each command run is one movement sub-step of one tick's length (so a catch-up tick
   that runs two commands moves the soldier two steps, never one: the seam with netcode-architect below).
2. Movement, inside that sub-step (2.7): the wish velocity from the move vector, the sprint and stamina rule, the
   acceleration toward the wish, the jump on a new press edge (at most one jump per tick however many presses the
   counter brings: 3B-ONE-PER-TICK, STATUS_ARCHIVE.md:1513-1515), the vertical arc, the sweep with slide and
   step-up, the ground probe.
3. The soft push between soldiers, after every soldier has moved: pairs (i < j) in index order; overlapping capsules
   separate by a fraction of the overlap per tick, swept against the static world so a push never puts a soldier in a
   wall, and never stops either soldier (START-RULES, DECISIONS.md:1726: "push each other softly and never block").
   Its strength is a movement field (`push_rate`, his to change after Brave). Felt first in cs5 (walkers); in cs4 it
   is shown by a headless pair test (PM-0004-35's soldier pairs, build plan :640).
4. The tick counter (checked add, as today).
A slot that has no command this tick: what the soldier does (repeat the held inputs of the last command, as 3b's record
does for buttons, world.rs:85-86, or stand) is netcode-architect's, with 3B-LATE-BOUND (cs7).

### 2.4 What the state holds and what the hash covers (ADR 0004 part 9)
- In the shared state, written by `write_state` field by field, floats by `to_bits`, fixed widths, little-endian, no
  `usize`, in this order: the tick; the slot count as `u16`; per slot the player record (buttons, presses, move, aim);
  per slot the soldier (generation, bits, team, position, velocity, stamina, aim, facing). Then the seed (authority
  only, outside the shared hash, world.rs:127-133).
- Not state, not hashed: the static world and its BVH (immutable for the match, rebuilt after a restore, ADR 0004
  :164-165), the `Rules`, the query scratch. A recording pins them by carrying the content bundle it ran on (netcode's
  format, 5.1), so a change of tuning never silently changes an old recording.
- The hash function: FNV-1a 64 stays provisional until PM-0004-21's short report (state_hash.rs:5-7), which comes before
  the first recording is committed (Walk's step 1).

### 2.5 Zero allocation in the step (PM-0004-6; ADR 0004 :249-251)
Every table is sized at match start; the query scratch is owned and reused; nothing is collected per query. Counted by a
counting allocator: `crates/sim/tests/step_alloc.rs` (new: 1,800 warm ticks of a scripted walker in the yard that walks,
sprints, jumps, slides along the thin wall and steps up) and the existing `crates/sim-wasm/tests/step_alloc.rs` on
the moving local match. Bite: a sweep that collects its hits into a `Vec`.

### 2.6 What is data and what stays code
Data (content, block 1.6): the movement numbers, the capsule's size, the push rate, the map's shapes, spawns and bounds.
Code: the tick rate, `UNITS_PER_TICK`, the integrator's form, the slide iteration cap, the skin width, the ground probe's
reach, the order of the phases. The take step (`Rules::take`) runs at match setup only and reports each rounding.

### 2.7 For rust-reviewer (the movement maths and the collision library: its parallel design)
1. The jump integrator: the exact per-tick parabola (y += v dt - g dt^2 / 2; v -= g dt) samples a 2.52 m apex, so
   the reach is 3.0700 m against the formula's 3.0708: inside PM-0004-59's 1 mm and START-RULES' 3.07 m; semi-implicit
   Euler gives 2.89 m (N50). Confirm or show another form that meets both.
2. Slide-and-step: shape casts per iteration, the step-up (up by step_up, across, down), ground snapping on ramps
   (PM-0004-54's 1 cm), no tunnelling at 13.27 m/s (0.44 m a tick) through 0.2 m (PM-0004-30).
3. The three helpers, and the scan's update (3A-SCAN-HELPERS, STATUS_ARCHIVE.md:1395-1398: it would refuse them by
   name or path today).
4. No transcendental function in cs4's sim (axis-aligned shapes; facing as a vector). If the snapshot wants yaw and
   pitch angles, `atan2` comes through the pinned `libm` (ADR 0004 :283-284) in the crate that encodes; whether
   parry3d's `enhanced-determinism` already brings simba's libm is for rust-reviewer to read.
5. parry3d: the version; features (`dim3`, `f32`, `std`, `enhanced-determinism` on both targets, ADR 0004 :293-294; no
   SIMD feature, no `parallel`); its transitive crates, each runtime one with its overflow-override line (root
   Cargo.toml:31-34) and the build check that refuses a crate without one (3a step 8); its `unsafe`; whether the BVH is
   built deterministically in insertion order (PM-0004-32's bytes); which queries allocate and how to pass a
   caller-owned stack (PM-0004-6); the shapes: capsule against cuboid and against a convex wedge (`ConvexPolyhedron`
   from six points: deterministic hull, built at match start), 0 `Err(Unsupported)` (PM-0004-35); its share of the Wasm
   (PM-0004-33).
6. 3A-FMA-CHECK (STATUS_ARCHIVE.md:1355-1360; his MUL-ADD (a)): the check on the built file, in the step that brings
   parry3d: the players' Wasm holds no function, call or import named `fma`/`fmaf` (and no `f32.fma`-like relaxed-SIMD
   instruction); its plant: a stand-in crate that calls `mul_add`. And 3A-FEATURE-VERSION (:1391-1394), due "when sim
   first gets a dependency": the feature compare by name and version.

### 2.8 For netcode-architect (the command, the snapshot, the recording: its parallel design)
1. The command's fields exist (protocol/messages.rs: `move_x`/`move_y` i8, `aim: AimPoint` cm, `view_time`); cs4 fills
   move and aim. The page sends a world-space move vector it computed from W/A/S/D and the camera's yaw (an input, not a
   sim value: ADR 0004 :286 forbids sim values in TypeScript, not inputs) and the cursor ray; the endpoint quantizes the
   move (127 = 1.0) and casts the ray (block 3.6). `view_time` stays 0 in cs4: nothing reads it before lag
   compensation (Slice 3, build plan :899) or cs8's plan.
2. Per-command movement sub-steps under catch-up (2.3), what a tick without a command does, and the late bound
   (3B-LATE-BOUND, cs7; security M1's "double speed" warning, security-auditor-review1.md:70-79).
3. The checks at the room: the move's length and the aim inside the map (L1); 3B-ONE-PER-TICK's test on the jump.
4. The snapshot's soldier records (position 1 cm, yaw 16 bits, pitch 10 bits, flags, the generation when an entity
   enters: ADR 0001 :278-279, :305-317) and the own part; block 3 lists what the page needs, so the snapshot carries at
   least that or the endpoint derives it (velocity: derived, block 3.3). AimPoint's i16 cm (messages.rs) holds +-327.67 m:
   PM-0004-39's 2 km map cannot travel in today's command; that test runs headless on the sim (as the row's method
   allows: "the same scripts with the offset").
5. The recording (Walk's step 1, its design): it must record, per tick, the host's call numbers (the post's numbers, the
   move and the ray, `now_ms`), carry the seed and the content bundle it ran on, a format name and version, and the
   expected shared hash per tick; inputs made by a seeded scripted walker, so the recording can be made again after each
   format or state change (write mode) and the targets compared on the new hashes.

### 2.9 The marks of block 2, each with its test and its plant
| Mark | Test | Plant |
|---|---|---|
| PM-0004-6 | 2.5 | a sweep that collects into a `Vec` |
| PM-0004-11 | block 1.11 | a planted 0.02 s interval |
| PM-0004-15 | 2.2 | no generation raise on reuse |
| PM-0004-17 | two local matches in one process stepped in turn equal their solo runs (the replay runner) | a `static` counter read in a phase |
| PM-0004-19 | the movement recording natively in two processes, Node, Chromium | the -21 injections |
| PM-0004-20 | the same with `enhanced-determinism` on; parry3d's query time per tick with the flag on and off (a reading) | |
| PM-0004-21 | std `sin`, a `HashMap` order of at least 32 keys, a presentation draw, each armed at a tick and reported there with a field-level diff | each injection |
| PM-0004-22 | the float vectors run again; the helpers' signed-zero cases (+0, -0) and (-0, +0), a reversed clamp | a helper built on std `min` |
| PM-0004-24 | the replay runner flags any non-finite float it hashes; debug builds assert finite state each tick; NaN in fuzzed commands refused where it enters | a planted `0.0 / 0.0` in the movement phase |
| PM-0004-25 | the movement recording with `simd128` on (a reading; SIMD stays off) | |
| PM-0004-30 | on LB at 30 Hz: 0.55 up, 0.56 stops; apex equal to the integrated value within 1 mm; 0 tunnellings in 10,000 random moves at 13.27 m/s through the 0.2 m wall | a move applied as a position step without a sweep |
| PM-0004-32 | the BVH dumped node by node, native against Wasm (test module); build time (a reading here, judged on the reference server); 10,000 rays after a restore and rebuild | colliders inserted in authoring order on one build |
| PM-0004-33 | own Wasm size raw, gzip, brotli, with the name-section split; a one-line change rebuilds natively in <= 10 s (5 runs) | |
| PM-0004-35 | a counter on every query result: 0 `Unsupported` over -30's runs and the push pairs | an unsupported pair placed in the scene |
| PM-0004-39 | the yard moved to the far corner of a 2 km map: -30's marks hold; a 10-minute route covers the same distance per second within 0.1 % | (the ladder's first item, build plan :355-356) |
| PM-0004-54 | 1,000 downward cursor rays onto the yard's ramps land within 1 cm of the slope (the endpoint's cast is the sim's query) | ramps queried as boxes |
| PM-0004-59 | the reach (measured apex + step_up) within 1 mm at 30 Hz for 11, 24, 0.55, and equal to 3.07 m (START-RULES) | legacy's closed form returned instead |
| PM-0004-60 | a bounded run of 1,000,000 decoded movement commands over their full ranges (NaN and infinities where floats enter) into a running sim, 0 panics (the nightly is CI work) | a planted `.unwrap()` on a client-sent value |
| 3B-ONE-PER-TICK | one command bringing 255 jump presses gives one jump | a jump per press |
| START-RULES' push | two soldiers walking into each other are never stopped and end apart; neither ends inside a wall | a push that blocks |
| 3A-FMA-CHECK | 2.7 item 6 | a stand-in crate calling `mul_add` |
| 3A-FEATURE-VERSION | the compare keys on name and version | a second version of a crate in the tree |

### 2.10 Size of block 2 (bottom-up; plus or minus 30 %; the room's and the codec's side is netcode-architect's)
| Part | Code | Tests |
|---|---|---|
| ids.rs, soldiers.rs | 150 | 120 |
| time.rs, rules.rs | 120 | 80 |
| fmath.rs | 40 | 60 |
| collide.rs | 260 | 150 |
| movement.rs (walk, sprint and stamina, jump, step, slide, push) | 380 | 350 |
| world.rs (records, table, order, write_state) | 120 | 60 |
| step_alloc (sim, sim-wasm) | 0 | 80 |
| the -17, -21, -24 runners and injections (in devtools, on the replay runner) | 120 | 150 |
| 3A-FMA-CHECK, 3A-FEATURE-VERSION, 3A-SCAN-HELPERS | 75 | 95 |
| PM-0004-60 bounded fuzz | 0 | 80 |
| PM-0004-39 far corner (ladder) | 30 | 60 |
| ADS's rule, if built in the part (block 5) | 40 | 50 |
| Total | 1,335 | 1,335 |
Plus about 15 lines of configuration: parry3d's line in the root `[workspace.dependencies]` and the overflow-override
lines of its runtime crates (a guarded file once the 2.3 chat's guard protects it: build plan :219-222).
- B2 DONE. Next: B3, the block and lane 2's rows.

## Block 3. The block (DONE)

### 3.1 The layout table's changes (crates/client-endpoint/src/layout.rs, layout_ts.rs, block.rs)
- `ElementType` gains `F32` (four bytes, IEEE 754, little-endian; "a variant enters with its first use", layout.rs:6).
- `Section` gains `rows: u32`: its capacity of rows, a build constant; `end_of` (layout.rs:115-119, "a section with more
  rows brings its count into the header, and into this function, when it enters") becomes `start + rows * stride`;
  the hash's stream gains each section's `rows` (layout.rs:207-215); `layout_ts` prints `rows` beside `stride`. One
  table still drives the writer, the generated offsets and the hash (ADR 0002 :309-312).
- Fixed capacity per build, with the live count in the header (recommended), against a capacity per match with each
  section's start in the header (cs1 report :263 planned "the header states each section's row count and byte
  offset"). Fixed: every offset stays a generated constant covered by the layout hash, the page binds views with no
  arithmetic of its own, the block's length is one number per build; the cost is a block of capacity size whatever
  the count (3.5). Per match: smaller blocks in small matches, but section starts become runtime values the page must
  read and check. Departure from cs1's note, with that reason, for the report.
- Row i of the soldier section is slot i (the soldier's sim index: ADR 0004 :155): lane 2 maps instance i to row i
  with no search; a free slot's row is zero with its `occupied` bit clear.

### 3.2 The header's counts (H2; lane 2's first list; cs1 report :187-189)
`header` (rows 1, stride 12): `tick u32 @0`, `layout_hash u32 @4` (both never move, layout.rs:76-78), `soldier_count u32
@8`: the rows in use this tick (the match's slot count; the page loops `0 .. soldier_count`). `event_count` joins with
cs8's event section, `projectile_count` with cs9's projectile section.

### 3.3 The soldier section (rows 48, stride 36)
| Field | Type | Offset | Meaning | Lane 2 row |
|---|---|---|---|---|
| id | u32 | 0 | slot index in bits 0-15, generation in bits 16-31 | F9, F10 |
| pos_x, pos_y, pos_z | f32 | 4, 8, 12 | metres, y up, the map's axes (the soldier's origin on the ground between the feet: appendix A4 :129) | F1 |
| vel_x, vel_z | f32 | 16, 20 | horizontal velocity, m/s, written by the client endpoint from the soldier's two newest snapshots (difference x tick rate; 0 on a generation change); no wire cost | F4 |
| yaw | f32 | 24 | radians, the aim's facing; the convention (0 toward +z, positive toward +x, or legacy's) is stated in the layout's doc and pinned by a test | F2 |
| pitch | f32 | 28 | radians, positive up, from the aim point and the eye height | F3 |
| flags | u8 | 32 | bit 0 occupied, bit 1 alive (always set until cs8; Part 15.2 :124-126 asks it before cs5's hand-over), bit 2 grounded, bit 3 sprinting, bit 4 aiming down sights (W1, with the part that builds ADS) | F5, F6, F7, W1 |
| team | u8 | 33 | the team index | F11 (team half) |
| (reserved) | | 34-35 | written 0 by the writer, the one owner of padding (cs1 report :263) | |
- Interpolation on the page (3.7): a row whose id is the same in both blocks is blended; a new id at a row (another
  generation, or a soldier that was not there) is placed, never slid (ADR 0001 :316-317; lane 2's F10).
- The capacity 48 is the design target (VISION_V2 §3: 24v24; ADR 0004 :236 "the largest mode's N (48)"); a match with
  more slots is refused at setup. If cs5's "48 walkers" means 48 beside the player, cs5's plan raises the constant (one
  line; the layout hash changes and the page refuses old builds by itself).

### 3.4 The local player's row and team (F12; gap 8), and the blend (NA-N3)
- `own` (rows 1, stride 20): `soldier u32 @0` (the local soldier's id; 0xFFFF_FFFF before the join is answered),
  `team u8 @4` (3 bytes reserved), `aim_x, aim_y, aim_z f32 @8, 12, 16` (the aim point the endpoint put in its last
  command: the point the server tests, PM-0004-54). It is the block's owner-only section of ADR 0002 :290-292; cs8
  adds the own weapon state (F13) to it. Answer to gap 8: the block, not the interface file: the page sees only the
  block (cs1 report :267), and `JoinAccepted.slot`/`team` reach it this way.
- The blend (NA-N3, STATUS_ARCHIVE.md:1153-1158; picture.ts:21-23): the Worker says when each block is due, the app
  turns that into one fraction per frame, the picture only blends. Concretely: the host adds `dueAt` (the tick's due
  time on the shared clock, from the Pacer, which knows each tick's due time in both modes: the step returns it, ADR
  0002 :338-340) to the `block` message beside `startAt` and `doneAt` (messages.ts:42-43); the link keeps it with each of
  the two newest blocks; `packages/client/src/app/blend.ts` (new, lane 1) computes, once per frame, the render time
  (the frame's time minus a smoothed minimum of arrival-minus-due, minus one tick period) and the fraction between the
  two blocks' due times, clamped to [0, 1]; a gap of more than two periods (a pause, a rebase) snaps to 1 (this is what
  the first picture after a pause shows: the paused match as it stood, no slide; 3B-RESUME-FLASH, block 4.9).
  `PictureFrame` gains `blend: number` (block 4.1). Why on the message and not a header field: the block's bytes stay a
  pure function of the state and the layout (a recorded match replayed to the page later, PM-0003-28 at 12, needs no
  clock in them), and Rust needs no f64 element type. picture.ts's sentence changes to say so (lane 1's file; I
  review it).
- Whether the local soldier is drawn at the newest block or blended like the others (OQ-0001-15; build plan :915 "the
  page's drawing of one entity (4)"): blended in cs4 (one rule for every row); the numbers for his check come in cs7.

### 3.5 The block's size at 1, 12 and 48 soldiers
header 12 + echo 20 + link 16 + own 20 + soldiers 48 x 36 = 1,728: **1,796 bytes** at 1, 12 and 48 soldiers alike
(capacity, not count); the rows in use are 36, 432 and 1,728 bytes. At 30 blocks a second that is 53,880 bytes a second
copied once in the Worker and transferred (no second copy). With a capacity per match it would be 104, 500 and 1,796
bytes. Today's block is 44 bytes (sim-wasm/src/lib.rs:157). cs2's and cs3's padded readings used guessed row sizes
(cs2 report :365): cs4's report records the first real one, and cs5 measures the hand-off at 12 and 48 on it.

### 3.6 What the page posts, and what crosses the bindings (WB-HEALTH (a))
- The post (`PageToWorker.input`, messages.ts:22-31) gains `move_x, move_z` (the world-space move from W/A/S/D and
  the camera's yaw, length at most 1) and `ray_ox, ray_oy, ray_oz, ray_dx, ray_dy, ray_dz` (the cursor's ray from the
  camera, picture.ts's return, block 4.1): flat numbers, no inner array per frame. `isPageToWorker` refuses a non-finite
  number, a move longer than 1.5 or a zero direction (an error message, as today for a bad post).
- The bindings: `input` grows from six to fourteen plain numbers (six `u32`, eight `f32`), still one call per tick
  beside `step` (ADR 0002 :344-346, at most R1-BND-REC1 mark 2's four; wasm-bindgen passes numbers with no allocation,
  as review 1 read for six: engine-architect-review1.md, "Q2 glue"). Rust checks each float finite where it enters (ADR
  0004 :285-286) and keeps the last aim when the ray is bad. The aim cast runs in the client endpoint against its own
  copy of the static world (block 1.8), from the newest post, once per tick: hit point, else the ray's crossing of the
  ground plane, else the last aim; then quantized into `AimPoint` (cm) and clamped to the map.
- Rejected: an input area in linear memory (a second layout table, a second generated file: right when variable-length
  datagrams cross, Slice 3, as cs3's design said); a second per-tick call for the aim (three calls instead of two for
  no gain).

### 3.7 The Worker's buffer pool (3B-POOL-16)
`POOL_SIZE` 4 -> 16 (packages/sim-worker/src/host.ts:37), because cs4 grows the block from 44 to 1,796 bytes and a miss
then costs a whole block (perf-auditor-review2.md:62: returns came in lumps of 12-17 at about 6 frames a second with key
input). 16 x 1,796 = 28,736 bytes held. Test: `block-pool.test.ts` keeps 16 and lets the 17th go; the stats count misses
as today.

### 3.8 Lane 2's first list, row by row (issue #59, comment 6994614989; a lane request, never his word)
| Row | Lands | Field (name, type) or answer | Reason |
|---|---|---|---|
| H1 tick | in the block today | `header.tick` u32 | layout.rs:87 |
| H2 row counts | cs4 | `header.soldier_count` u32; `event_count` (cs8), `projectile_count` (cs9) | each section brings its count (cs1 report :187-189) |
| F1 position | cs4 | `pos_x/pos_y/pos_z` f32, metres, y up | the soldier section |
| F2 facing (aim yaw) | cs4 | `yaw` f32 radians | one facing toward the aim (gap 5) |
| F3 aim pitch | cs4 | `pitch` f32 radians | from the aim point and eye height |
| F4 velocity against facing | cs4 | `vel_x/vel_z` f32 m/s, world axes (lane 2 turns them into its facing's frame) | derived by the endpoint, no wire cost, no teleport spike (0 on a generation change) |
| F5 on the ground | cs4 | `flags` bit 2 | the snapshot carries the bit (netcode-architect places it; gap 1) |
| F6 sprinting | cs4 | `flags` bit 3 | sim state: with stamina it is not derivable from speed |
| F7 alive | cs4 (always 1 until cs8's deaths) | `flags` bit 1 | Part 15.2 :124-126: the stand-in shows dead and respawned from a block field before cs5's hand-over |
| F8 weapon held | cs8 | `weapon` u16 (index into the bundle's weapon table) | weapons enter in cs8 |
| F9 soldier id | cs4 | `id` u32 (index | generation << 16); row i = slot i | PM-0004-15's ids |
| F10 respawn marker | cs4 (respawns from cs8) | the generation in `id`; a change at a row = place, not slide | ADR 0001 :316-317 |
| F11 team, faction | team cs4; faction per team with Slice 2 | `team` u8 per soldier; a `teams` section (3 rows, `faction` u8) when factions are data (CD-13, Slice 2) | cs8's dummies take "a plain second colour" by team (build plan :797-798) |
| F12 local player and team | cs4 | `own.soldier` u32, `own.team` u8 | gap 8: in the block |
| F13 own reload progress, active reload | cs8 | in `own`: `reload` u8 (0-255 of the reload), `active_reload` u8 (state) | the owner-only weapon state, build plan :404 |
| F14 projectiles | cs9 | the projectile section (id, kind, position) | PROJ-DRAW (a): lane 1 draws them plainly in cs9, they pass to lane 2 at its merge (DECISIONS.md:1203-1204) |
| E1 shot | cs8 | event row: shooter id, weapon, tick, start point, direction | own shots in the block of the tick that fired them, others' on their interpolation timeline (ADR 0002 :298-303); the bow's release is a shot, its arrow a projectile row (cs9's plan confirms) |
| E2 where a shot ended | cs8 | per ray: point, normal, struck (soldier id or static shape id), surface class | gap 9: a `surface` tag on map shapes joins in cs8, additively |
| E3 hit confirmed | cs8 | attacker, victim, weapon, damage, body part, hit point, source point, kind | build plan :403-405 |
| E4 death | cs8 | victim, killer, weapon, the point to fall away from | gap 3: the point joins the event |
| E5 blast | cs9 | place, strength, kind, owner | one E5 row in the block whatever the wire does (gap 2: the endpoint writes it from the projectile's last trajectory change) |
| E6 reload start and end | cs8 | start: soldier, weapon, tick; end: the same and the result (normal, quick) | SCOPE-RELOAD (build plan :33) |
| W1 aiming down sights | the part that builds ADS (block 5) | `flags` bit 4 | a need by SLICE-FEEL (DECISIONS.md:1234-1236) |
| W2 weapon swap | cs8 | no field: a change of `weapon` | as lane 2 says |
| W3 grenade throw | cs9 | events `throw_start`, `throw_release` | ADR 0001 :311 (an action start) |
| W4 sprint-to-fire swing | with the per-gun delays (cs8 under ADS-DELAY (a), block 5) | `ready` u8 in the soldier row (0 sprint pose, 255 ready) | the delay is per gun (SLICE-FEEL; LOOP_V2:125); lane 2 reads it in cs10, after cs8 |
| W5 footsteps, landings | no field | from F4 and F5 | as lane 2 says |
| Gap 1 | cs4 | F4 derived, F5 a flag, F11 a byte | above |
| Gap 2 | cs9 | E5 in the block | above |
| Gap 3 | cs8 | E4's fall-from point | above |
| Gap 4 | decided | PROJ-DRAW (a) | his words of 2026-11-19 |
| Gap 5 | cs4 | one yaw in Slice 1 | the sim holds one facing toward the aim point; a second field only if a rule separates body and aim |
| Gap 6 | decided | ADS is a rule of Slice 1 (SLICE-FEEL) | his words |
| Gap 7 | Slice 3 | in local play the predicted shot (E1) at once, the confirmed end and hit (E2, E3) from the authority | lane 2 plays the tracer to the predicted end and the impact at the confirmed point |
| Gap 8 | cs4 | the block (`own`) | above |
| Gap 9 | cs8 | `surface` on shapes | additive (CD-4) |
| Gap 10 | cs4 | yes: the page holds the same bundle (block 1.8) and reads speeds, and later recoil and reload times, from it, typed by the generated types | none of these needs a block field |
Nothing on the list is refused.
- B3 DONE. Next: B4, the page.

## Block 4. The page, folder by folder (DONE)
Lane 1 owns every folder below until cs5 merges; then `render/`, `soldier/`, `camera/` (and, proposed, `content/camera/`)
pass to lane 2 (Part 15.2 :112-115). The rule of today stays: only `render/` imports three, and the app reaches the
picture through `picture.ts` alone (picture.ts:1-4).

### 4.1 `src/picture.ts` (the interface file; every change reviewed by engine-architect)
1. `PictureFrame.blend: number`: the fraction between `previous` and `newest`, in [0, 1], computed by the app (3.4);
   the sentence of picture.ts:21-23 rewritten to say who computes it and from what (the Worker's due stamps).
2. `Picture.setContent(content: PictureContent): void`, with `PictureContent { map: MapDocument; camera: Camera }` from
   `@shamble/types` (generated): builds the scene from the map document and sets the camera's values; called after
   the Worker's `ready` and again after an accepted development reload; disposes what the previous call built
   (ADR 0003 :464-466, "Between matches").
3. `Picture.cursorRay(x: number, y: number, out: Float32Array): boolean`: the ray from the camera as last drawn (the
   interpolated, drawn state, ADR 0003 :460) through the canvas point (CSS pixels), written as origin then direction
   into the app's six-float array; false before the first frame. No allocation.
4. `Picture.cameraYaw(): number`: the drawn camera's yaw, which the app uses to turn W/A/S/D into a world-space move.
5. `Picture.plainLines(points: Float32Array, count: number): void`: the hook for lane 1's plain lines (Part 15.2
   :120), drawn as one line object from a preallocated buffer (a fixed capacity, 256 segments proposed). Its first
   user in cs4: the development aim line (4.4); cs8's tracers next.
6. `start` options gain `devtools: boolean` and `panelHost: HTMLElement | null`: in development builds the picture
   puts its camera panel (in `camera/`, which is the picture's side by Part 15.2 :106) into that element.
7. The muzzle point waits for cs8: nothing draws a flash or a tracer before it, and the simulation's shot start is data
   of its own (build plan "Since the plan" :68; Part 15.2 :150).
The local soldier's row the picture reads itself (`own.soldier`, block 3.4), like every other field.

### 4.2 `src/render/` (three-picture.ts, scene.ts replacing empty-scene.ts, index.ts unchanged)
- The scene from the map document: each `box` a box mesh, each `ramp` a wedge mesh with the same six points the
  simulation's convex shape uses, each region a flat quad on the ground; the ground and the 1 m grid kept (they judge
  sizes); the plain lights of cs2. Built once per `setContent`, disposed on the next; nothing per frame.
- Colours: none decided here (question GREY-COLOURS, 4.10). Node materials only (ADR 0003 part 1, as empty-scene.ts:12-13).
- `cursorRay` from the camera of the last drawn frame (scratch vectors kept, no `new` per call); `plainLines` as one
  `LineSegments` over a preallocated `Float32Array`, its draw range set per frame.

### 4.3 `src/soldier/` (new)
- `stand-in.ts`: the plain stand-in at the brief's sizes (appendix A4 :133-136: boot top 0.36, hem 0.43, belt 1.045
  to 1.10, shoulders 1.49, chin 1.54, brow 1.66, hat top 1.80; shoulders 0.49 over the arms, waist 0.33, hem 0.61,
  hat 0.235): a few coarse parts (coat skirt, torso, head, hat, boots, arms, a 1.12 m gun bar), every part on one root
  so the whole figure scales with one number (`STAND_IN_HEIGHT_M = 1.80`, a placeholder, :130); the root turns to
  `yaw`, the gun bar tilts to `pitch`: "it shows facing and aim" (build plan :345). Plain on purpose (build plan
  :876-877); cs6 (lane 2) replaces it.
- `rows.ts`: 48 roots made at start, hidden until their row is occupied; each frame, for rows `0 .. soldier_count`,
  position and angles blended between the two blocks by `blend` (yaw by the shorter arc); a row whose `id` differs
  between the blocks is placed, not slid (block 3.3); a row whose `alive` bit is clear lies down (Part 15.2 :124-126:
  built and tested now with a crafted block, so cs8 needs nothing from lane 2). No allocation per frame.

### 4.4 `src/camera/` (new)
- `follow.ts`: the follow camera at legacy's framing, ported from CD5 (LEGACY_INVENTORY.md:87; values in
  `content/camera/camera.json`): height 40, boom 56.6, field of view 15, yaw 45 degrees, the screen dead zone 0.60 x
  0.52, the follow, push and yaw rates, and the look-ahead and position gains legacy reads (the six dead keys not
  ported); its update read from legacy's camera code at the build. It follows the local soldier's blended position.
  Legacy's middle-mouse camera turn is not in Part 5's list and is not ported (his to ask).
- `panel.ts`, `settings.ts`: the live camera tuning panel (build plan Part 5 :853-865; his "Queued" words): backquote
  opens it; sliders for boom, height, field of view, the pitch override and the sights camera mode (the mode slider
  matters once ADS exists, block 5); the five presets Classic, Tactical, Iso, Iso+, Foxhole (CAM_STYLES,
  shamble-game.js:2196-2200); "copy values", which puts the camera file's exact canonical bytes on the clipboard (block
  1.4; a text box to select when the clipboard is refused, as the report's fallback, overlay.ts:46). Slider limits come
  from the generated camera schema (CD-3 :338-340; PM-0005-7). Development builds only.
- The development aim line: a plain line from the local soldier's chest height to `own.aim` (the point the server
  tests), through the plain-lines hook, development builds only (a readout toggle), for his check of aiming at height
  on the ramps and ledges.

### 4.5 `src/app/`
- `content.ts` (new): fetches `./content.json` once, parses it (`JSON.parse`, the files by path, the camera and the map
  read field by field into their generated types, never merged into other objects: CD-9 :454-455), and hands the
  bundle's bytes to the link for the Worker; in development, `import.meta.hot?.on("shamble:content", ...)` refetches and
  posts (Vite removes `import.meta.hot` from builds).
- `blend.ts` (new): the fraction of block 3.4, a pure function with its state in numbers.
- `boot.ts`: the order becomes: picture start; content fetch; the Worker started with the bundle; on `ready`,
  `picture.setContent`; pointer listeners on the canvas (`contextmenu` prevented there); each frame: the move vector
  (input/move.ts with `picture.cameraYaw()`), `picture.cursorRay`, `link.frame(keys, move, ray)`, the blend,
  `picture.plainLines` (development), `picture.draw`.
- `worker-link.ts`: the start message carries the bundle (transferred); the input message carries the move and the
  ray; reads `soldier_count`, `own.soldier`, `own.team`, `own.aim`; keeps each block's `dueAt`; the development
  `content` message; its length check stays (>= BLOCK_BYTES before any read past the header).

### 4.6 `src/input/`
- `keyboard.ts`: unchanged buttons (W, A, S, D, Space, Shift; messages.rs:83's other ten bits enter with their systems).
- `move.ts` (new): held W/A/S/D and the camera's yaw -> a world-space move of length at most 1 (diagonals normalized).
- `pointer.ts` (new): the cursor's canvas position; mouse buttons into the same held bits and press counters as keys
  (the right button, legacy's sights button, shamble-game.js:3284-3285, as bit 6 once ADS is built); let go on blur
  and on a hidden tab, like keys. 3B-BLUR-TEST (STATUS_ARCHIVE.md:1540-1542, "with the next change to the page's key
  handling"): a browser test holds W and a mouse button, sends `blur`, expects both let go.

### 4.7 `src/overlay/` (development only, behind `__SHAMBLE_DEVTOOLS__` and dynamic imports, as boot.ts:100-157)
New readout rows: the local soldier's position, speed (from `vel`), the flags (grounded, sprinting, aiming), the aim
point, the blend fraction, `soldier_count` and block bytes (1,796), the content's files and versions, the camera preset;
the same fields in "Copy report". The key diagram stays.

### 4.8 `src/styles.css`, and what is development-only (and how it stays out of production)
- Development-only: the readout and report, the key diagram, the camera panel with "copy values", the aim line, the
  bundle-injection hook for the reload test; all behind the literal `__SHAMBLE_DEVTOOLS__` (vite.config.mjs:19-21) and
  dynamic imports, so a production build cuts them; `DEV_MARKERS` (build-checks.mjs:133) gains "camera-panel" and
  "aim-line". The Vite content plugin has `apply: "serve"`: in no build at all.
- 3B-CSS-PROD (STATUS_ARCHIVE.md:1526-1527), closed here because cs4 adds more development CSS: the development rules
  move into their own CSS file imported only by the development modules (its own chunk), and the production scan also
  reads `.css` files (build-checks.mjs:163 reads `.js` only today).
- His zip is a development build, as for cs2 and cs3 (he copies reports from it), so the panel is in it; the
  production build stays for the scan.

### 4.9 No allocation per frame, and what the first picture after a pause shows
- Per frame: the six-float ray array, the plain-lines buffer, the 48 roots and the scratch vectors exist from the start;
  the blend works on numbers. The one message per frame stays (ADR 0002 :318-329) and grows by eight numbers: the
  allocation spec's per-frame limit is measured again on the new message (allocation.spec.ts), not loosened by guess.
- 3B-RESUME-FLASH (his, open; STATUS_ARCHIVE.md:1478-1483), now with a soldier on screen. What the code will do: the
  match stands still while the tab is hidden (ADR 0004 :219-220); every key is let go on hide; on return the first
  frame draws the match as it stood (the blend snaps at the gap, 3.4, so nothing slides from old stamps), and if he was
  walking, the soldier slows to a stop over the next ticks (legacy's acceleration 14 m/s^2 stops a 6.3 m/s walk in
  about 0.45 s: 6.3 / 14). Options: (a) that; (b) the Worker also zeroes the soldier's velocity when the tab hides:
  a simulation rule tied to the page's visibility, which online play would never have (a hidden tab does not pause an
  online match, ADR 0004 :219-220), so a code path only offline (ADR 0002 :413-414). Recommendation: (a).

### 4.10 Questions this block raises (for Mustafa; look and rules are his)
- GREY-COLOURS: what colours do the greybox scene and the plain stand-in take in cs4? (a) LOOP_V2 §9's own default
  (LOOP_V2.md:158: "plain blocks, with height read through brightness (higher is lighter)") in neutral greys, the
  stand-in one mid grey with a darker gun; (b) the Art lead's palette A proposal (appendix A5 :217-224) and the soldier
  brief's faction colours (A4 :138-145) as placeholders; (c) one flat grey. Recommendation: (a), his own default rule,
  which puts no look choice on screen before the carry (build plan :874-875).
- CAM-HOME: if his Testlines Export holds camera values, which is their home? (a) the camera file, tuned in the panel
  (recommended: the panel is the tool his Queued words name); (b) the Export, the panel only for trying.

### 4.11 Size of block 4 (TypeScript and specs; plus or minus 30 %)
| Part | Code | Tests |
|---|---|---|
| picture.ts | 40 | 0 |
| render/ (scene from the map, ray, plain lines, content swap) | 260 | 60 |
| soldier/ (stand-in, rows, blend, place-not-slide, lying down) | 220 | 120 |
| camera/follow.ts | 160 | 120 |
| camera/panel.ts, settings.ts | 220 | 140 |
| app/ (content, blend, boot, worker-link) | 260 | 260 |
| input/ (pointer, move, keyboard) | 110 | 120 |
| overlay/ rows | 80 | 60 |
| sim-worker (messages, host: bundle, swap, dueAt, POOL 16; worker.ts; pacing.ts due per tick) | 200 | 220 |
| build-checks (CSS scan, markers, content.json) | 40 | 50 |
| browser specs: walk (move, jump, 0.55/0.56 in the yard through the page, aim on a ramp, camera follows), panel, reload hook, even-motion reading, blur, allocation re-measure | 0 | 520 |
| Total | 1,590 | 1,670 |
- B4 DONE. Next: B5, the cut and the order.
- N59. Checked (grep, this copy): devtools uses `sim` (rules/stream.rs:5, rules/f32_ops.rs:13), `sim-wasm` (wasm.rs:10) and wasm-bindgen; `client-endpoint`, `protocol`, `room` are named in devtools/Cargo.toml:14-17 and used nowhere.

## Block 5. The cut and the order (DONE)

### 5.1 Step 1, the recording (netcode-architect designs it), placed in `devtools`
- Files: `crates/devtools/src/replay.rs` (the runner: drives `sim_wasm::Sim` natively through the same calls the
  Worker makes, `Sim::create(seed, mode, 0, bundle)`, then per tick `input(...)` and `step(now_ms)`, reading the shared
  hash after each step; the native tests already call these methods, sim-wasm/src/lib.rs:181-200), `compare.rs` (the
  first differing tick; the field-level diff comes from two native runs, since the Wasm side gives only the hash),
  `recording.rs` (the format: read and write), `src/bin/record.rs` (write mode: a seeded scripted walker, run natively,
  writes the recording with its expected hashes), `crates/devtools/recordings/*.rec` (data). The Wasm halves:
  `packages/sim-worker/boundary/replay.test.mjs` (Node) and `packages/e2e/tests/rules/replay.spec.ts` (Chromium) feed
  the recorded calls to the players' file through `new Sim(...)`, `input`, `step` and the one new read-only export
  (DEPART-3 (a)).
- The export: `Sim::shared_hash() -> u64` (one BigInt, read only by the replay harnesses; the live host never calls it,
  pinned by a test on host.ts and by the boundary test's list of `Sim.prototype` names); it returns the shared hash only,
  its buffer private to `Room` (my 3b round 2 advice, STATUS_ARCHIVE.md:1555-1556; N15).
- Before the first recording is committed: PM-0004-21's short report (the function confirmed or replaced, its cost per
  tick; state_hash.rs:5-7; cs3 plan :72-74). In step 1 the recording runs on today's state (tick and echo); every later
  step that changes the state or the command re-makes it in write mode and the four runs compare on the new hashes, so
  "built and green before any movement code rests on it" (his condition b) holds and the recording that carries the
  marks is the one made on the moving state (step 7).
- devtools' three path edges (security-auditor's L2 = my EA-6; STATUS_ARCHIVE.md:1549-1550): after step 1 devtools
  uses `sim`, `sim-wasm`, wasm-bindgen, and from step 2 `content` (its bins). `client-endpoint`, `protocol` and `room`
  stay unused (N59): drop all three in cs4's one manifest batch (no outside code; a removal); the link-simulator
  change-set adds `protocol` back with its first user (its `TransportPair`). "No declared edge without a user at
  merge" closes L2 by name (his condition c).
- Where 3c's other named items go (his condition c; the archive's list, STATUS_ARCHIVE.md:1547-1557, N21): to Walk:
  the host's time recorded with every tick; the state-hash export (shared hash only, plain numbers, replay-only, buffer
  in `Room`); devtools' edges; 3B-HOSTILE-ALLOC (STATUS_ARCHIVE.md:1543-1546, "with 3c's tests"; the part that comes
  first takes it: a counting allocator in the two random-message tests, `crates/room/tests/any_bytes.rs` and
  `crates/client-endpoint/tests/any_bytes.rs`). To the link-simulator change-set: its clock through `LocalMatch` from
  recorded times; its reliable channel exactly once; loss and the rate limit as silent drops, duplicated datagrams
  only; corruption as loss only; a `link()` accessor on `LocalMatch`; `late` checked against its own reorder count.

### 5.2 One stop or two: two (recommended)
The bottom-up sizes (blocks 1 to 4, step 1, and the netcode side as I estimate it) come to about 11,500 lines for the
whole card under CONTENT-SOURCE (c), about 10,400 under (a), against the card's 3,900 (build plan :250, before his
SLICE-FEEL additions and before ORDER-3C put the recording in). One stop would be the largest change-set of the plan by
far and would hold the first soldier behind all of it. Cut:
- **4a, "Walk"**: the recording first, then the content crate's core, the soldier and his movement, the block, the
  page with the stand-in, the follow camera and **the camera panel** (his Queued words: "from its first playable
  build"), and every proof of the movement it merges (the movement recording on four runs and its marks). He sees and
  plays: the plain stand-in in the street corner and the yard; he walks, sprints (with stamina), jumps, steps up, aims
  with the mouse (the stand-in turns to the cursor; the development aim line shows the point the server tests), and
  tunes the camera live with sliders and five presets, "copy values" included. His Brave check: even walking (and the
  hidden-tab steps, PM-0002-34), the 0.55 m step and the 0.56 m stop felt in the yard, the 3.07 m reach against the
  ledges, aiming at height on ramps and ledges, the camera panel and its presets.
- **4b, "Walk: sights and content"**: aiming down sights by SLICE-FEEL (the right button held, 0.55 of the speed, no
  sprint, the camera's sights mode from the panel, the `aiming` flag W1), and the content's remaining proofs: content
  item 1's corpus on every target (PM-0005-2, -3), content item 2's seeded corpus and the bounded fuzz (PM-0005-4, -5,
  -6), PM-0005-7 for movement fields, PM-0005-8's typed content files, the development reload (PM-0005-10), the full
  legacy import report (PM-0005-14); the far-corner test (PM-0004-39), the bounded fuzz of movement commands
  (PM-0004-60), 3B-HOSTILE-ALLOC if not taken in step 1's change. He sees and plays: aiming down sights with the
  camera's reaction. His Brave check: holding the right button slows him to 0.55 and stops the sprint; the sights
  camera mode by preset.
- What each part meets (the plan's rows): 4a: ADR 0005's foundation line (:461); CD-1, CD-2 (core stages), CD-3, CD-4,
  CD-5 shown on movement, maps and the camera (:502); ADR 0001 :1785 (two runs, the same hashes: :485); G10's first
  recording (:484); PM-0004-6, -11, -15, -17, -19, -20, -21, -22, -24, -25 (reading), -30, -32, -33, -35, -54, -59;
  PM-0002-14 (the aim in the post), -26 (smoothness reading), -29 (sizes with content), -31 (the Worker gets content),
  -34 (his checklist); PM-0004-28 and PM-0003-42 (readings); 3A-FMA-CHECK (the change-set that brings parry3d: his
  words, STATUS_ARCHIVE.md:1355-1357); 3A-FEATURE-VERSION, 3A-SCAN-HELPERS, 3B-ONE-PER-TICK (movement's half),
  3B-POOL-16, 3B-CSS-PROD, 3B-BLUR-TEST, NA-N3, H2, START-RULES both rules. 4b: content items 1 and 2 (:503-504),
  PM-0005-1 to -8, -10, -14 (the rest), PM-0004-39, -60; SLICE-FEEL's cs4 half; W1. PM-0005-16's size half and
  PM-0004-33's comparison builds are lane 2's separate tests after 4a merges (Part 15.2 :105).
- His conditions: (a) the marks the plan places at Walk on the recording (-17, -19, -20, -21, -24, -25's reading, ADR
  0001 :1785, G10) are all in 4a, with the movement they prove; (b) step 1 is built and green before step 2; (c) 5.1
  places every 3c item by name; (d) the fallback stands: if 4a's plan cannot carry the recording inside its stop line,
  the recording alone becomes a small change-set before 4a (no browser check), and he is told before anything moves.
- One stop instead (if he prefers): the same steps, 4b's after 4a's, one merge and one Brave check at the end, about
  11,500 lines planned before the main session's ratio.

### 5.3 4a's steps, in order (each ends green: cargo debug and release, the Wasm build, Node, boundary, browser, gate)
0. The one manifest batch, unbuilt until security-auditor answers on its exact content (cs3 plan :67-71): the
   `content` crate (path); `sim` -> `content`, parry3d (+ its overflow-override lines); `sim-wasm`, `client-endpoint`
   -> `content`; devtools: drop three edges, add `content`. (Under CONTENT-SOURCE (a) or (b), serde and the rest join
   this batch.)
1. The recording (5.1), green natively in two processes, Node and Chromium, with one planted divergence caught at its
   tick; PM-0004-21's report.
2. The content crate's core (block 1.3 to 1.8, 1.10): loader, writer, the three kinds, the bundle, the constructor
   route, the build's `content.json`, the export test (types and schemas). The recording re-made (the constructor now
   takes the bundle).
3. The simulation (block 2): the time unit and the rounding report (before stamina's timer: the card's "before any timer
   exists", build plan :340-341), `Rules::take`, the table and ids, `collide` on parry3d, movement, the helpers,
   3A-FMA-CHECK, 3A-FEATURE-VERSION, his Testlines table and `--check` (his point 8; the movement values come from his
   Export once he has confirmed the differences list); the recording re-made.
4. The command, the snapshot and the block (blocks 2.8 and 3): move and aim in the command, the endpoint's cast, the
   soldier records, the block's header count, soldier and own sections, `dueAt`, the pool of 16; the recording re-made.
5. The page (block 4 without the panel), in two halves: 5a the scene from the map, the stand-in, the follow camera
   (walk, sprint, jump visible); 5b the mouse aim (pointer, ray, cast, facing, the aim line).
6. The camera panel (block 4.4), with its schema-driven limits.
7. The movement recording made on the final state and every mark that rides it; the parity marks; the readings; the
   allocation spec re-measured; then reviews, the verifier, his Brave check.

### 5.4 The earliest build he can play (an early look, not a check)
After step 5a: the soldier walks, sprints, jumps and steps up in the street corner and the yard, seen by the follow
camera at legacy's framing; no aim yet. Recommended: after step 5b, one step later, with the mouse aim. Either is a zip
of the development build from a commit on the branch, before any review, with a README that says it is an early look,
what is not in yet (the panel, sights) and that nothing is asked of him. His point 8: if his Export is not in by then,
the early look runs on legacy's numbers and says so.

### 5.5 The size ladder (each part whole with its tests, to a named place; the camera panel stays: build plan :355-356)
- 4a, if it nears twice its plan: (1) PM-0004-32's byte dump and its 10,000 rays -> 4b; (2) PM-0004-20's query-time
  reading -> 4b; (3) PM-0004-17's two instances -> 4b; (4) the schema writer with PM-0005-7 -> 4b, the panel's limits
  written in its code until then. Never moved out of 4a: the recording, -19, -21, -24 on the movement it merges, the
  parity marks -30 and -59, 3A-FMA-CHECK (his words tie it to parry3d's change-set), the camera panel.
- 4b, if it nears twice its plan: (1) PM-0004-39's far corner and (2) the size comparison (the card's own two, build
  plan :355-356) -> a follow-up before Slice 2; (3) content item 2's corpus for the kinds of cs4 -> cs8, where the
  plan meets that item with weapons (:504); he is told before any part moves.

### 5.6 Disagreements with the build plan and the drafts (for the plan to state)
1. The card's size: 3,900 lines (build plan :250) against about 11,500 bottom-up (10,400 under CONTENT-SOURCE (a));
   the card predates SLICE-FEEL and ORDER-3C, and its content line hides a parser, a writer, a schema and two tools.
2. The card says "the movement table from the legacy tuning import" (:336); his newer words (takeover point 8 b) take the
   values from his Export: the import maps and reports, the Export supplies the values (block 1.11).
3. cs1's note "the header states each section's ... byte offset" (cs1 report :263): fixed capacity makes the offsets
   constants covered by the layout hash (block 3.1).
4. The "field of the block" for the blend (picture.ts:21-23; NA-N3): a stamp on the block message instead (block 3.4).
5. PM-0004-15's "a proptest property" and ADR 0005's serde_json, garde and schemars: our own generator and, under
   CONTENT-SOURCE (c), our own reader (DEPART list).
6. PM-0004-39 cannot travel through today's command (AimPoint i16 cm, +-327.67 m); it runs headless (block 2.8).
- B5 DONE. Next: B6 (room for lane 2's early-look page).

## Block 6. Room for lane 2's early-look page (EARLY-LOOK (a), DECISIONS.md:410-416; takeover point 10) (DONE)
1. Yes, leave room, and it costs nothing now: Walk keeps the picture's whole surface in `picture.ts` (start, setContent,
   draw with blend, cursorRay, plainLines), so a second entry can drive `createPicture` with a content bundle and crafted
   blocks and import nothing from `app/` ("draws through its shape, so that the hand-over is a move of files").
2. Walk adds no second page; it only avoids one-entry assumptions in `build.mjs` and `build-checks.mjs` (the Worker-chunk
   check already keys on its source module, not on the page; the production scan should walk every HTML entry).
3. The crafted-block helper cs4's own `soldier/` tests need is written from the generated layout in `packages/types`
   (lane 1's), so lane 2's page can reuse it instead of writing block offsets by hand.

## Block 7. Sizes per step, risks, not yet tried (DONE)

### 7.1 Sizes, bottom-up (hand-written lines; code and tests apart; plus or minus 30 % each; NOT multiplied)
| Part, step | Code | Tests | Total |
|---|---|---|---|
| 4a 0: the manifest batch (configuration) | 40 | 0 | 40 |
| 4a 1: the recording (engine-side estimate; netcode-architect's design sets the real one) | 250 | 300 | 550 |
| 4a 2: the content crate's core, CONTENT-SOURCE (c) | 1,410 | 950 | 2,360 |
| 4a 3: the simulation (time unit and rounding report, table and ids, collide, movement, helpers, the 3A checks, Testlines table and check) | 1,335 | 1,105 | 2,440 |
| 4a 4: command, snapshot, block (netcode side about 300 and 300 of it) | 450 | 450 | 900 |
| 4a 5: the page without the panel (5a scene, stand-in, follow camera; 5b aim) | 1,310 | 1,390 | 2,700 |
| 4a 6: the camera panel | 220 | 200 | 420 |
| 4a 7: the movement recording and its marks, readings | 150 | 200 | 350 |
| **4a total** | **5,165** | **4,595** | **9,760** |
| 4a under CONTENT-SOURCE (a) | 4,455 | 4,175 | 8,630 |
| 4b: ADS (rule, input, sights camera, W1) | 140 | 150 | 290 |
| 4b: content proofs (corpus, seeded corpus and bounded fuzz, legacy import walk, -7 movement, -8 files, development reload) | 380 | 920 | 1,300 |
| 4b: PM-0004-39, PM-0004-60, 3B-HOSTILE-ALLOC | 30 | 180 | 210 |
| **4b total** | **550** | **1,250** | **1,800** |
| **Walk (4a + 4b)** | **5,715** | **5,845** | **11,560** |
Not counted (the plan's rule, build plan :235-237): the map document (about 150 JSON lines written by hand), the
content files written by script, generated types and schemas, the import report, recordings.
History for the main session's ratio: change-set 3's three bottom-up designs came to about 5,850 lines (cs3 plan :23-24)
and 3a and 3b ran 1.7 and 1.8 times their plans; these numbers are made the same way. To bring the early look sooner, the
plan can gather the large mark runs (the 10,000-case and 1,000,000-input runs, the allocation counts, the BVH dump) in
step 7, after step 5, each earlier step keeping the tests of its own behaviour: about 1,000 test lines move behind the
early look.

### 7.2 Risks
1. Size: 4a alone is about 2.5 times the whole card (5.6 item 1); the early look (5.4) is the answer to "play early",
   not the merge.
2. parry3d, first outside crate in `sim`: its tree under the dependency hold (one security round); its Wasm share;
   allocation inside queries; the BVH's build order; and 3A-FMA-CHECK: if the built file shows a software `fma` call
   from parry3d's tree, cs4 stops for his decision (his MUL-ADD (a) bans it in the simulation).
3. The jump integrator (N50): the wrong form misses START-RULES' 3.07 m by 18 cm (2.89 m).
4. CI's browser run: 3.4 of its 5 minutes used today (cs3b report, "Evidence"); cs4 adds walk, panel and replay specs; a
   raise is a CI edit (the CI Builder chat, his yes first on its exact text).
5. The allocation spec's per-frame limit with an input message eight numbers longer: re-measured, never loosened by
   guess.
6. His Export: the movement values wait for it (his point 8 b); if it is late, the early look runs on legacy's numbers
   and says so; if it says 60 steps a second, he is told before it is used (point 8 f).
7. Two copies of the static world in one Wasm instance (the room's and the endpoint's): small for the test scene,
   measured with the Wasm's memory in the report.
8. Under CONTENT-SOURCE (c): std's shortest digits are not documented (N56); the test against `JSON.stringify` over
   12,038 numbers is the proof.
9. More code passes to lane 2 at cs5's merge (`render/`, `soldier/`, `camera/` grow from 136 lines today: render/*.ts).

### 7.3 Not yet tried (each a first step of its build step, with rust-reviewer where Rust)
parry3d built for wasm32 in this workspace, with `enhanced-determinism`; std's `{:e}` digits on wasm32 against native and
`JSON.stringify`; fourteen numbers through one wasm-bindgen call with no allocation in the glue; the Pacer giving each
tick's due time inside a catch-up wake; a Vite dev-server plugin pushing a custom event that the page forwards to the
Worker; a TypeScript writer matching the Rust canonical writer byte for byte; ts-rs on content structs (and on structs a
`macro_rules!` writes, if the plan uses one); the BVH dump compared byte for byte across targets.

## Block 8. The questions for Mustafa, in one place, and two items the blocks left implicit (DONE)
- N60. DEPART-3 (a) accepted "three limits are code, not data, until the content crate exists in change-set 4"
  (cs3 plan :312-313, ADR 0001 line 221): with content in cs4 they would move into content. Proposed: they stay code
  until cs7 rebuilds the room's command scheduler behind one seam (build plan :389-391), so they move once; a departure
  for the list, netcode-architect's to confirm.
- N61. Part 15.2 :93-94 gives lane 1 `content/` "for movement, weapons and the test scene"; `content/camera/` is
  neither: the plan should say lane 1 owns it until cs5's merge and that it passes to lane 2 with `camera/`.
Questions (each self-contained in the plan's "Decisions needed"):
1. CONTENT-SOURCE (block 1.2): who writes the code that reads and checks the game's data files? (a) the libraries ADR
   0005 names (serde_json with float_roundtrip, serde_path_to_error, garde; schemars and ts-rs at build time): least
   code of ours, 5 to 8 outside crates in the players' file; (b) serde_json's untyped tree with our own checks and
   schema: 5 outside crates, duplicate keys pass silently; (c) all ours: no outside code, about 1,130 more planned lines
   in 4a, number text identical to the browser's own. Recommended (c).
2. WALK-CUT (block 5.2): (a) two stops: 4a the walking soldier with the camera panel and every proof of his movement,
   4b aiming down sights and the content's remaining proofs (recommended); (b) one stop; (c) two stops with sights in
   4a (4a larger, 4b then nothing to see, so no Brave check). Earliest early look: after 4a's step 5a; recommended after
   5b (with the mouse aim).
3. ADS-DELAY (SLICE-FEEL's placement): the per-gun delays after a sprint (before firing, before aiming down sights),
   W4's swing field and the spread at sights: (a) all with the guns in cs8, in each weapon's file (recommended: each
   value lands in its one home; lane 2 reads W4 in cs10, after cs8); (b) the sprint-to-sights delay as one value for
   every gun in 4b's movement file, moved per gun in cs8 (he feels it one part earlier; one migration; its home moves
   once).
4. GREY-COLOURS (block 4.10): the greybox scene and the plain stand-in: (a) LOOP_V2 §9's own default, brightness by
   height, neutral greys (recommended); (b) the Art lead's palette A and the brief's faction colours as placeholders;
   (c) one flat grey. Look: his.
5. CAM-HOME (block 4.10): camera values that his Testlines Export might also hold: (a) home is the camera file, tuned
   in the panel (recommended); (b) home is the Export.
6. RESUME-FLASH (3B-RESUME-FLASH, block 4.9): the first picture after a hidden tab: (a) the match as it stood, then he
   slows to a stop (about 0.45 s) since every key was let go (recommended); (b) the Worker also stops him when the tab
   hides (a rule only offline play would have).
7. COLLIDER-SIZE (block 1.6): the soldier's collision capsule, a starting number he can change after Brave like
   START-RULES: (a) radius 0.30 m (the coat hem's half width, 0.61 / 2 rounded) and height 1.80 m (the placeholder
   height); (b) radius 0.25 m (the shoulders' half width, 0.49 / 2 rounded); (c) the game-designer proposes first.
   Recommended (a), to be confirmed by the game-designer in the plan's review.
8. DEPART-4 (the departures, as DEPART-3 was asked): our own seeded generator for PM-0004-15 (no proptest); under
   CONTENT-SOURCE (c) our own reader in place of ADR 0005's crates, PM-0005-7's "garde range" read as "the field
   table's range"; header offsets as layout constants (cs1's note); the blend stamp on the block message (picture.ts's
   sentence); PM-0005-14's "kept value equal to legacy's" read as "not replaced by his Export"; PM-0004-39 headless only;
   DEPART-3's three limits staying code until cs7 (N60). Recommended: accept all.
DONE
