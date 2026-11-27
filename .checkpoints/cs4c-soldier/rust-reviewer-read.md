Task: rust-reviewer's design read for 4c step 1 (the simulation's soldier and movement) against main dae6d107, for the re-read sheet (plan.md section 3 item 1). Read only; no source edits; no commit.

## Plan (blocks, in order)
0. Read today's code: crates/sim/src (lib, collision, vec, world, time, rng, state_hash), crates/sim-wasm local_match.rs, crates/content/src (movement.rs, map.rs, time.rs), content/movement/soldier.json, content/source/testlines-table.json, build-checks.mjs (math routine list, FMA/SIMD rules), rules_scan.rs, parry3d 0.31.1 query API in ~/.cargo/registry (std off), game-designer-cs4.md, rules-cs4.md, STATUS_ARCHIVE 2278-2300, 4b-2 report.
1. Step 1's pieces against today's code: symbols add/change/remove (file:line), tests with planted defects, marks, counts recounted.
2. 4B-RAY-COUNTS and the simulation side of 4B-QUERY-RECORDING.
3. MOVE-RULES rows 1-5, 8 and NO-SOURCE (b) fields by option; what can be built before his answers.
4. Risks, ranked.

## Notes lines
