# test-engineer, review round 2 of cs4a (head ec005ad, source as at 07903d9)

## Plan
1. Read round1-fold.md (what happened to each finding) and the diff eb14739..ec005ad -- crates packages.
2. T1-T7: check each fix in the code and in plants-4a.py / plants-4a.txt / .gate/plants-4a.log.
3. PM-0004-21: C, D, E each at tick 600 with the field; is each "caught" the right check for the right reason (the needles).
4. The new tests: can each fail; is anything weaker than before (state_hash_alloc.rs, the room's test).
5. Anything the pass broke: run the native tests in my copy (.gate/te-4a-r2, own target dir); plant where cheap.
6. Verdict; each finding fixed / not fixed; new findings with size.

## Findings (one line each, as found)
- T1 FIXED (read + log): two_processes.rs:97-105 compares both columns child against child and names the step; plants C and D fail there at "first-loop, Shared: the two processes differ", step 600 (.gate/plants-4a.log:450-452, 528-530); `replay check` names step 600 (:490, :568); `replay fields` names player.presses (:513, :591). Fingerprints printed (:448).
- T2 FIXED (read + log): plant C folds the order into 8 bytes (plants-4a.py:115-117); the two processes' values differ in all 8 bytes (plants-4a.txt:14).
- T3 FIXED (read + log): plant G = my plant; N3 red at two_instances.rs:32 step 3 (:283-284); controls held: N1 one test at a time (:322-333), N2 (:335-346), W1 passed while W2 failed at replay.test.mjs:87 step 3 (verbose marks, :390-437).
- T4 FIXED (read): recording.test.ts:150-156: replayOn refuses a move and a non-test seed, calls length unchanged. Can fail (my round-1 plant would now turn it red: replayOn would call the fake and not throw).
- T5 FIXED (read): recording.ts:65-72 now cut-to-the-id, UTF-8, cut-to-the-count, as recording.rs:286-292; recording.test.ts:79-82 reaches the UTF-8 refusal, and the 86-byte case pins the new order (the old order said "header" there).
- T6 FIXED: recording.rs:608 renamed. T7 FIXED: commit in line 1, defects and controls counted apart (line 24), tree put back by byte comparison (line 25); ec005ad equals 07903d9 in crates/packages/.github (git diff empty); git status on crates/packages empty now.
- NEW-1 Low (log :765-786): plant E's Wasm state file held the vector line and three lines of Node's stderr warning (run() merges stderr; plants-4a.py:136 strips only the last line); `replay fields` reported player.presses plus 4 spurious "no such line" entries; plants-4a.txt:22 shows only the player. lines. The catch is real (the needle needs player.presses), the input was not clean. Fix in notes: stdout only, drop the vector line by its prefix.
- Run (my copy .gate/te-4a-r2 at ec005ad, own target dir): cargo test --workspace all green (26 s); cargo fmt --check clean; clippy --all-targets -D warnings clean.
- E4 type pin CAN FAIL (run): generated input given a 7th number in my copy's d.ts: tsc error at recording.test.ts(7,59) "Type 'Sim' is not assignable to type 'Replayable'" (worker.ts:29 too); clean d.ts: tsc exit 0. CI's tsc line includes $worker_tests and pkg/sim_wasm.d.ts. d.ts put back.
- Read: fnv1a_64_fed_in_pieces constant 0xbd51ea7094ee6fa1 recomputed independently (Python): right. world.rs known answers unchanged in value, so the streamed hash equals the old buffered one; the streamed test (world.rs:386-391) compares both hashes with fnv1a_64 over the Vec-written bytes: fails if the hash's byte sink differs from the buffer's.
