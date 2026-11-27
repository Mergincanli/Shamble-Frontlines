Task: netcode-architect's design read for change-set 4c ("The first soldier"), against main dae6d107 (worktree
`.gate/cs4b-2`, branch cs/4c-soldier), 2026-11-28. Input to the main session's re-read sheet (plan.md section 3 item 1).
Read only; this file is my only write. My design: `.checkpoints/cs4-walk/netcode-architect-cs4.md` (blocks 1, 3-5).

## Plan (blocks, in order)
0. Read today's code: protocol (messages.rs, codec.rs), client-endpoint (post.rs, endpoint.rs, layout.rs, block.rs),
   room/lib.rs, sim (lib.rs, world.rs, local_match.rs, collision.rs's StaticWorld and cast_ray), sim-wasm/lib.rs,
   devtools (recording.rs, replay, make_recordings, recordings/ with first-loop and first-loop.content.json),
   sim-worker (worker.ts, host.ts, messages.ts), client app/worker-link.ts, e2e replay/determinism.spec.ts; 4b's report;
   STATUS_ARCHIVE :2278-2300. A notes line per finding.
1. Step 2, the command path: symbols to add / change / remove (file:line), tests with planted defects, count
   recounted, what changed since my design.
2. Step 3, the snapshot's soldier and the block (timing side): header count, the due time per block, three newest
   blocks (DEPART-4) with the test that fails with two, 3B-POOL-16. Symbols, tests, count.
3. Step 6, the movement and lane recordings: what each records, on which frozen content; first-loop made anew; the
   three plants on movement; PM-0004-17, -19, -20, -21, -24 with test and plant; 4B-QUERY-RECORDING (native, Node,
   Chromium); 4B-RAY-WASM. Symbols, tests, count.
4. Steps 6b and 6c: paced replay in the real Worker (PM-0004-28); 2,000,000 hostile movement commands (PM-0004-60);
   far-corner map (PM-0004-39, the 13,150 rung). Symbols, tests, count.
5. RESUME-FLASH by option (a)-(d): what the code does, and where.
6. Order: can steps 2 and 3 be built before step 1's movement if MOVE-RULES / NO-SOURCE come late, without building
   twice? Facts only; the advisor agent rules.
For each step: which of his open questions (MOVE-RULES, NO-SOURCE, RESUME-FLASH, TUNE-HOME, SIZE-4C) changes what,
by option. Then DONE.

## Notes log (one line per finding)
