# engine-architect, cs4a "The recording", review round 1

Plan (read-only review; source as at eb14739; diff `git diff c1da1bd...HEAD -- crates packages`):
1. Read: plan.md §4, netcode-architect-cs4.md block 1, my engine-architect-cs4.md (Walk design), ADR 0002 part 3,
   build plan Part 15.2, the players' file header (WB-HEALTH words), DEPART-3 wording.
2. Q1: Sim::shared_hash chain (sim-wasm -> LocalMatch -> Room -> World): read-only? live-tick reachable? buffer out?
   u64/BigInt cost, glue growth 53,813 -> 55,607 bytes, &mut self shape, the boundary pin test.
3. Q2: StateSink trait + World::visit/visit_shared; room reads SHARED_STATE_BYTES; devtools new modules, bins,
   wasm.rs test-only exports; recording.ts test-only reader; e2e replay tests + node-names.d.ts. Boundary/direction,
   test-only reach into production, StateSink shape for 4c (floats, variable-length runs).
4. Q3: lane/file ownership vs Part 15.2.
5. Q4: cheap-now shaping for 4b/4c; what must not change.
6. Each finding: severity, file:line, fix in one line, ~lines, follow-up target.

Findings (a line each as found):
