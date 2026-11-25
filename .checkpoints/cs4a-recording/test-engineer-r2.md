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
- a_rebase_before_every_step_changes_no_hash CAN FAIL (run, my copy): LocalMatch::rebase planted to run a tick too: red at replay.rs:316 (ticks 3, 5, 7...). Copy restored, clean. Note: LocalMatch::step ignores now_ms (local_match.rs:112), so the test pins that rebase moves the schedule only.
- Read: seed refusals can fail (Rust: Replay::new returns Ok without the check, the test expects the refusal; TS: the move refusal would be thrown instead of the seed's, toThrow checks the message). Link test can fail both ways (writer through the link; check reading through it). Shape pin can fail (indexOf "    shared_hash() {" is -1 with an argument). Browser digest check can fail (not run: no browser tests).
- Not weaker (read): state_hash_alloc.rs keeps its counting control (lines 71-79) and now asserts 0 from the first call (stronger than "exactly 1"); the room's buffer assertions went with the buffer; "no memory" through the room stays counted in sim-wasm/tests/step_alloc.rs:111-114; the room test now pins `&self` at compile time (read_only: &Room). Committed recordings unchanged (git diff empty).
- Run: sim-worker unit tests 51/51 in my copy.
- NEW-2 Low (CI logs): the pull request's CI job (139533402557) checks out pull/70/merge = ec005ad into main 07d673b (#71) and runs `cargo test --release -p sim -p protocol -p room -p client-endpoint -p sim-wasm -p devtools`: recordings, two_instances, two_processes ran in RELEASE and passed (log :1200-1234). The branch's own push job (139533368926) runs the branch's workflow (release -p sim -p protocol). T9's wording "debug only in CI until E1" is out of date: say release runs in the pull request's CI (main's E1, #71). All six checks green on ec005ad.
- Leftovers of mine: .gate/te-4a-r2 (worktree at ec005ad, tracked files clean; untracked pkg/sim_wasm.d.ts copy), .gate/te-4a-r2-target; round 1's .gate/te-4a, .gate/te-4a-target, .gate/te-scratch still there.

## Verdict
APPROVE. T1 to T7 fixed; T8 left by decision; T9 to the report (with NEW-2's correction). PM-0004-21's three injections each at
tick 600 with the field: shown (C, D, E), with the report's caveats (E a stand-in, D native only, the field by the tools by hand).
New: two Lows, neither a product line: NEW-1 (plants script, 2 lines, when it next runs: 4c) and NEW-2 (report wording).
DONE
