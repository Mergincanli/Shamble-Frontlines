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
- F-m (CI log, job 139488422220 on 5a16b4e): devtools tests ran twice (recordings 3/3, two_instances, two_processes, hash_cost), npm test ran recording.test.ts 5/5, boundary replay.test.mjs 6/6 with the Wasm cost reading printed, the Chromium spec passed in HeadlessChrome 157. CI's claims on the card hold.
- F-n (read): the field-level diff across targets is a tool path (bin `replay state` + test module `replay_state_at`, diffed by the plants script in Python); compare::field_diff is used by one unit test only. In CI a red W1/C1 names the tick, never the field. PR #70 says "a comparer that names ... the fields that differ there": true for the library, by hand across targets.
- F-o (read): two_processes.rs:71 keeps only lines containing "-loop "; 4c's `walk`/`lane-walk` lines would be dropped and the test would fail as "is not" until the filter changes (4c trap, Low).
- F-p (read): plants-4a.txt carries no commit SHA and the clean run does not check the tree equals HEAD (git status on crates/packages); the run predates eb14739 (recording files then uncommitted). The script edits world.rs and re-makes recordings in the shared worktree, where other reviewers run tests: run it in a separate worktree.
- F-q (read): plant E is a stand-in for the libm sin injection; PM-0004-21 asks for the search over sin inputs first (PM-0004-22's table), the stand-in only if none differs. Not run. 4c (three of three on movement fields), but the report should say so.
- F-r (read): recording.rs "every reason ... none prints a seed byte" check (recording.rs:559-572) and the hash-cost readings are fine; hash_cost.rs and the Node cost test cannot fail by design (readings).

## Plant texts for plants-4a.py (anchor STEP, as the script's others)
- Interleaving-only (for N3 and W2; N1 run with --test-threads=1, N2 and W1 stay green):
  { static STEPS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    if self.tick == 0 { STEPS.store(0, std::sync::atomic::Ordering::Relaxed); }
    if STEPS.fetch_add(1, std::sync::atomic::Ordering::Relaxed) != self.tick { self.player.presses[1] ^= 1; } }
  My result: N3 red at two_instances.rs:32 step 3; W2 red at replay.test.mjs:87 step 3; N1 (one at a time), N2, W1, both test-module tests green.
- Plant C widened (64 bits, so two processes differ with certainty, as plant D):
  if self.tick == 599 { let map: std::collections::HashMap<u32, u32> = (0..32).map(|k| (k, k)).collect();
    let mut order = 0u64; for (k, _) in &map { order = order.wrapping_mul(0x100_0000_01B3) ^ u64::from(*k); }
    for (count, byte) in self.player.presses[2..10].iter_mut().zip(order.to_le_bytes()) { *count = byte; } }
  and log both orders (e.g. eprintln of `order` in each child's run, read from the log) as PM-0004-21 asks.
- Tick and field for C and D: `cargo run -q -p devtools --locked --bin replay -- check <rec> <hashes>` (names the step), then
  `... --bin replay -- state <rec> 600` run twice (two processes) and the lines diffed (names player.presses).

## Leftovers of mine (delete refused by the permission system; the main session may remove them)
- .gate/te-4a (a git worktree at 6ebc6c7, tracked files clean, ignored build outputs inside); .gate/te-4a-target (345 MB); .gate/te-scratch/fold.

## Verdict
REVISE (small): no product defect found; every core test can go red on its defect. Asks: PM-0004-21's tick and field for the
two two-process injections (N2 names the step, about 15 lines; the field from the dumps in the plants), plant C made certain,
the interleaving-only plant, replayOn's refusal tested; the rest wording and 4c follow-ups. About 22 product lines in all.
DONE
