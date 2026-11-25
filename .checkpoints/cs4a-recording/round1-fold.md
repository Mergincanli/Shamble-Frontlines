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

## rust-reviewer: (running)

## test-engineer: (running)

## engine-architect: (running)

## security-auditor: (starts when a slot is free)
