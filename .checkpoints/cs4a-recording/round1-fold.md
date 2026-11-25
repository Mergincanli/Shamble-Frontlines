# Review round 1 of change-set 4a: every finding, and what happens to it

Head reviewed: the source as at eb14739. Room for both rounds: 370 lines (3,030 written, stop line 3,400). No source
is edited while a reviewer of this round still reads it. "Fix" = in 4a's fix pass; "Follow-up" names its change-set.

## netcode-architect: REVISE (light); nothing Critical or High (`netcode-architect-r1.md`)
| # | Severity | Finding | Decision |
|---|---|---|---|
| N1 | Medium (High if the report claimed it as it stands) | Plan step 8 says each planted divergence is "caught at its tick with the differing field"; plants C (a hash map's order) and D (an outside draw) are caught only by a digest over the whole run, which names neither | Fix, 0 product lines: the plants script also runs `replay check` (it names the step) and compares `replay state <rec> 600` of two processes field by field. The report says plant E is a stand-in and plant D was planted natively only; their real forms run again in 4c |
| N2 | Low | `Replay::new` and `checkReplayable` replay a recording built in memory with any seed; the design says every replay path refuses a seed that is no test seed | Fix, about 10 lines |
| N3 | Low | The live host calls `rebase` at the start, after a dropped gap and on resume; the format records none; it is safe only because `rebase` moves the schedule alone, and nothing pins that | Fix, about 10 lines: a test that a rebase before every step changes no hash |
| N4 | Low | `files.rs` and `bin/replay.rs` report the shared column's step first; once random streams join the authority's part the whole column can differ earlier | Follow-up, change-set 8 (4 lines) |
| N5 | Low | `recording.ts` stays out of the production build only because nothing imports it | Follow-up, 4c: a development marker when the paced replay imports it |
| N6 | Low | `two_processes.rs` keeps report lines that contain "-loop ": 4c's `walk` lines would be dropped (the test would fail loudly, not pass) | Fix, about 3 lines: a fixed prefix |
| N7 | Low | The rule for making a recording anew is written nowhere 4b and 4c will read it | Fix, about 4 lines of doc |
| N8 | Low | The ray is stored as `f32` bits: 4c's input call must turn it into `f32` before anything reads it, or the format is too narrow | Fix, about 2 lines of doc |
| N9 | Low | The Chromium spec does not check the hashes file's digest against the recording (Node does); the two fingerprints are never printed on success | Fix, about 3 lines |
For 4b's re-read and 4c (no line now): what `first-loop` replays on once content reaches the constructor (4b); the
frozen content copies must be listed inputs of the folder check, not unlisted files (4c); the shared length from the
world, not a constant (4c); `StateSink::f32` with a finite check (4c); the recording's header has no mode (Slice 2).

## rust-reviewer: APPROVE; nothing Critical or High (`rust-reviewer-r1.md`)
| # | Severity | Finding | Decision |
|---|---|---|---|
| R1 | Medium (latent) | `World::visit` no longer names every field in one pattern: a new field of the authority's part (the stream positions, change-set 8) compiles as soon as `visit_shared` ignores it, even if `visit` never hands it over, and it would drop out of the whole-state hash without a sound. The old `write_state` had the whole guard | Fix, about 4 lines: `visit` names every field too |
| R2 | Low | `Replay::new` does not refuse a seed without `TESTSEED` for a recording built in memory (netcode-architect's N2) | Fix with N2 |
| R3 | Low | `fnv1a_64_rounds` (test-only module): its length sizes an allocation with no cap, and `at * 31` overflows above 138,547,332 (a trap, measured in Node in both profiles) | Fix, 2 lines |
| R4 | Low | `scripts.rs` says "whole numbers only" and divides one `f64` for the host's time (the same bits on both targets, and the scripts never run in WebAssembly: only the comment is off) | Fix, 1 line |
| R5 | Low | `state_hash.rs`'s header says the whole-state hash never leaves the Worker or the server; the committed hashes files hold it for a public test seed, by design | Fix, 1 line: "over a match's own seed" |
| R6 | Low, optional | `recording.rs` promises never to panic and does not carry the room's lint guard (unwrap, expect, indexing, arithmetic); tried on a copy: 2 errors, both bounded products | Fix, about 11 lines |
| R7 | Low | `compare::field_diff` has no caller outside its unit tests (plant E's field difference is made in Python) | Fix with N1: the `replay` tool gains the comparison of two state dumps, about 10 lines |
| R8 | Low, optional | The hash can be fed straight from the visitor, with no buffer in the room: the players' file would grow by 701 bytes and not 1,794 (measured on a copy: 54,514 bytes; first-loop still replays to all 1,800 committed hashes, both columns), the export becomes `&self`, and nothing can allocate. About 60 lines | Decide with engine-architect's answer on the export's shape |
Its answers to the six questions, in short: no input makes the reader, the hashes parser, the replay or the two replay
exports panic or take memory before a count is checked (R3 aside): read, and fuzzed in wasm32 in both profiles (20,000
mutated inputs, every step count that would wrap a 32-bit product, all 256 map-id lengths; a planted overflow shows
the harness sees a trap). No float, `usize`, hash map or clock reaches the canonical bytes or the committed files.
`sha256.rs` is right at 312 lengths against Python's hashlib and at 2^29 + 1 bytes on wasm32. Where the 1,794 bytes
went: 1,307 of code (the hash 323, `Room::new` out of line 322, the visitor 206, the export's shim 181, a copy of
`Vec`'s grow path about 273), 448 of function names, 39 of tables. Its copies `.gate/rr-4a` and `.gate/rr-4a-base`
are removed with `git worktree remove` when the round is over.

## test-engineer: (running)

## engine-architect: (running)

## security-auditor: (starts when a slot is free)
