# engine-architect: read for 4b's revised sheet (2026-11-26, worktree lane1-takeover, branch cs/4b-content from 2483e9a)

Asked by the advisor agent's re-read (advisor-reread.md: R1, R2, R3, R8; rulings F.2, F.6, F.8). Not a review round.
No agent memory folder exists in this repository (checked .claude/ in both trees).

## Plan
1. Read: reread-sheet.md (done), advisor-reread.md (done; handback rulings R1-R11 not in the file: look for them),
   test-list.md (done), his words go-merge-70 and source-go (done), security-auditor-testlist.md,
   test-engineer-testlist.md, my design block 1 (+3.6, 4.x) in engine-architect-cs4.md, plan.md sections 5, 11.
2. Check main's code: messages.rs AimPoint, sim-wasm lib (Sim::new/create), sim's lib/LocalMatch, room's use of sim,
   devtools vectors folder and its scripts (language, --check), CI jobs that run them, build.mjs feature compare,
   recording/first-loop route, docs/legacy-tuning.json shape, testlines notes scripts.
3. Answer 1 to 6, each saved here as finished.
4. DONE, handback.

## Notes (a line each)
- [read] reread-sheet, advisor-reread (rulings R1-R11/C1-C7 are in its handback, not the file: substance from the task and cs4b-content.md:51-59), test-list, SA and TE reads, question-2026-11-26-feature-compare (FEATURE-COMPARE, SIZE-4B), his words go-merge-70 and source-go, my block 1 (1.1-1.13), plan s5, s11.
- [main] working tree batch: devtools deps = content, sim, sim-wasm, wasm-bindgen (protocol dropped); sim = content, parry3d; sim-wasm = client-endpoint, content, protocol, room, sim; room = protocol, sim (no content). content/src/lib.rs: forbid(unsafe), deny unwrap/expect/indexing/panic/unreachable/todo/arithmetic_side_effects.
- [main] Sim::create(seed, mode, pad) sim-wasm/src/lib.rs:71-85; Sim::new :103-105; LocalMatch::new(setup, first_sequence, pad, link) local_match.rs:52; Room::new(setup, NO_CONTENT) :54; NO_CONTENT local_match.rs:20; Room::new(setup, content_hash) room/src/lib.rs:105 -> World::new(setup) :108.
- [main] AimPoint x i16, y u16, z i16 cm, protocol/src/messages.rs:137-144 ("Provisional as a whole", :132-134).
- [main] CI: toolchain.yml:165-166 names each Python vector check by line (a new Python script = a CI edit); :171 `npm test --workspaces`; client's test = `vitest run` (default include picks scripts/*.test.mjs, as build-checks.test.mjs today). tools/ is in MERGE_PLACES (routine-guard.mjs:84): a tools/-only PR merges by the PR Advisor alone.
- [finding] testlines-cs4.md:90 maps `stamina.stamina_on` (yes/no) to a content field `movement.stamina_on`: a boolean in 4b's movement kind, against the sheet's "no content field of 4b or 4c is a boolean" (reread-sheet.md:31). Proposed: not content; the script checks the Export holds `true` (as step_rate_hz is checked to be 30, testlines-cs4.md:80).
- [main] proposed content keys (walk_speed_m_per_s, speed_up_time_to_90pct_s, gravity_m_per_s2 ...) all fit [a-z][a-z0-9_]{0,63}; the Export's own keys do not (T.moveSpeed, STAM.max, "T.accel (speed-up)"): the Export never meets the game's reader.
- [adr] PM-0005-3 (docs/adr/0005:2299): 2^53+1 "refused at load with the file and field named"; corpus "plus -0" (:2298).
