# test-engineer, review round 1 of cs4a "The recording" (head 51efa35, source as at eb14739)

## Plan
1. Read the card, plan section 4 and section 9 row 4a, netcode-architect block 1.6 (marks, tests, planted defects).
2. Read the diff c1da1bd...HEAD -- crates packages: every test 4a adds or changes.
3. Map each mark (plan lines 470, 473; ADR 0001 replay line; PM-0004-17/-18/-19/-21; word-size vector;
   "one planted divergence caught at its tick with the differing field") to a test, a place it runs, and whether it can fail.
4. Read toolchain.yml 140-200: what runs where in CI.
5. Read plants-4a.py / plants-4a.txt: is each catch the right test for the right reason.
6. Think of defects the plants miss; plant a few in a copy (.gate/te-4a, own target dir) if cheap.
7. Flakiness: two_processes.rs, the 100,000-input thresholds, timing readings, order dependence.
8. Rank gaps against the 370-line room; write the verdict.

## Findings (one line each, as found)
- F-a (read): plants C and D are caught by two_processes.rs:92 (children differ) but no tick and no field is reported for either; PM-0004-21's "at its arming tick T with a field-level diff" holds for E only (1 of 3 injections).
- F-b (log .gate/plants-4a.log:3983-3986): under plant C both children gave the SAME first-loop digest (c88d7bec...); only the 36,000-step half differed. The fold is a u8 with 64 possible values (multiples of 4): about 1 in 64 per half collides. The plant is probabilistic.
- F-c (log :340-348): plant B in Node failed W2 at replay.test.mjs:87 (against the committed file), and W1 failed too; the needle ['two matches on one module instance','FAIL'] cannot tell which assertion. B is a counter that survives between runs, so it shows nothing the solo checks do not; an interleaving-only plant is missing.
- F-d (read): no test calls replayOn (recording.ts:149-154) on a recording it must refuse; dropping checkReplayable from replayOn stays green (determinism.spec.ts:69 calls checkReplayable itself).
- F-e (read): recording.test.ts never reaches "the map's id is not UTF-8 text" (recording.ts:70); TS reader's check order differs from Rust's in one place (cut before UTF-8), so recording.ts:3-4 "same checks in the same order" is not exact.
- F-f (read): Rust test name "refused before any memory is taken" (recording.rs:576) asserts errors only; no counting allocator. An allocation before the length check would stay green.
- F-g (read): TS and Rust readers agree today on every offset (checked by hand), but no shared known answer pins move/ray/view/start/sequence/content/map id across the two; first-loop leaves them zero and checkReplayable refuses non-zero. 4c.
- F-h (run, .gate/te-scratch/fold): plant C's fold over std HashMap orders: 64 distinct values, two maps agree 1 in 64 (200,000 maps). Per run, both halves collide (plant C not caught) about 1 in 4,096.
- F-i (run, my copy .gate/te-4a, own target dir): an interleaving-only plant (a step count outside the state, reset when a match starts at tick 0): one test at a time, N1 recordings.rs green, N2 two_processes green, N3 two_instances.rs:32 RED at step 3. N3 bites where the solo checks cannot. Under the default parallel test threads N1 ALSO went red, at step 2 in one test and step 5 in another: N1's three tests replay concurrently, so a static defect makes N1 fail at a varying step (timing-dependent detector; N3 is the deterministic one). Copy restored, clean.
- F-d confirmed (run, my copy): replayOn without its checkReplayable line: recording.test.ts 5 of 5 green. Copy restored, clean.
- F-j (run, my copy, players' file and test module built there with --wasm-only): the interleaving-only plant in Node: W1 green, both test-module tests green, W2 RED at replay.test.mjs:87, step 3. W2 bites where the solo checks cannot. Plant removed.
- F-k (run, my copy): N1, N2, N3 in a RELEASE build: all green against the committed (debug-made) files, the 36,000-step children equal. CI runs devtools in debug only (toolchain.yml: release line is -p sim -p protocol); PM-0004-18's "built as the server builds the sim" half is not in CI (E1).
- F-l (run): devtools suite in debug: hash_cost.rs takes 1.73 s and cannot fail (asserts fnv's determinism only); every other group under 0.7 s.
