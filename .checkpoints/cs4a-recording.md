# Checkpoint: cs4a-recording
- **Goal:** change-set 4a, "The recording": a recorded match replays to the same per-tick hashes natively (one
  process, two processes, two matches interleaved) and in WebAssembly on the file players get (Node and Chromium),
  and a planted divergence is caught at its tick. No browser check: nothing a player sees changes · plan:
  `.checkpoints/cs4-walk/plan.md` (revision 4, approved by the advisor agent; section 4) · design:
  `.checkpoints/cs4-walk/netcode-architect-cs4.md`, block 1 · his go: `.checkpoints/cs4-walk/mustafa-2026-11-25-go-walk.md`
  (2026-11-25, 19:36:25 UTC: start 4a; WALK-CUT (a); STEP-60 (a))
- **Branch:** `claude/cool-pasteur-oiqk1r` on GitHub (the stand-in for `cs/4a-recording`). Local: `cs/4-walk` in the
  worktree `.gate/lane1-takeover`; pushed with `git push origin cs/4-walk:claude/cool-pasteur-oiqk1r`, alone in its
  command; never a plain `git push`. The session's start folder is never switched · **Base:** origin/main
  c1da1bd (lane 2's docs-only #67, taken in by merge commit 75a9296; before it 311c0484)
- **Last update:** 2026-11-25 21:49 Amsterdam (from the clock)

## Sizes (his point 5: which is which)
- The designs' own count for 4a, bottom-up: 1,365 lines (655 code, 710 tests).
- Planned, which already holds the 1.8: 2,450. Stop line (planned times 1.25): 3,050.
- Ladder: at 2,450 written lines 3B-HOSTILE-ALLOC moves to the link simulator's change-set; at 2,750 the
  36,000-tick run and the two interleaved matches move to 4c. He is told before a part moves.

## Steps done
0. Setup: his go recorded by script; main c1da1bd taken in (docs only); the pinned bindings tool fetched as CI does
   and checked (11,860,397 bytes, sha256 9687f3bb...227f; the program 8,170,784 bytes, sha256 ca4df773...6931),
   kept under `.gate/wb-tool/`; `cargo fetch --locked`; `npm ci --ignore-scripts`.

1. Steps 1 and 2 (dac4d78, pushed): `StateSink`, `World::visit`, `visit_shared`, `write_shared_state`;
   `Room::shared_hash` on a private buffer; `LocalMatch::shared_hash`; `Sim::shared_hash` with the boundary test's
   pin; the step's counted-allocation test reads the hash every tick (0 allocations).
2. Steps 3 to 7, built and green on this machine, committed as work in progress with `[skip ci]` (the recording's
   own files are NOT committed: they wait for the hash report): `crates/devtools/src/` `sha256.rs`, `recording.rs`,
   `replay.rs`, `compare.rs`, `scripts.rs`, `files.rs`, `bin/make_recordings.rs`, `bin/replay.rs`, three exports in
   `wasm.rs`; tests `recordings.rs`, `two_processes.rs`, `two_instances.rs`, `hash_cost.rs`;
   `packages/sim-worker/src/recording.ts` with its unit tests, `boundary/replay.test.mjs`;
   `packages/e2e/tests/replay/determinism.spec.ts` (two Node names added to `node-names.d.ts`).
   The first recording: 1,800 steps, `first-loop.rec` 122,487 bytes, sha256 1c05e6b8...6cf1 (made by
   `cargo run -p devtools --locked --bin make_recordings`; in the worktree, uncommitted).
   Every one of its 1,800 shared hashes is equal natively (one process; two more processes with a 36,000-step
   match; two matches in turn), in Node on the players' file and on the test module (both profiles, both columns),
   and in Chromium 158 in a Worker on the players' file.

3. The planted defects, first run (`plants-4a.py`, result `plants-4a.txt`): 11 of 11 caught on their targets (the
   simulation changed at tick 900: native, Node, Chromium; a counter outside the state: native and Node; a hash
   map's order and an outside draw: two processes; a WebAssembly-only difference: Node, with the field named; a
   `usize` in the bytes: step 1), and the clean tree green after. Run again on the final head.
4. The hash report (PM-0004-21), by rust-reviewer: **keep FNV-1a 64** (`hash-report.md`; its notes
   `rust-reviewer-hash.md`, DONE). 1.25 ns a byte natively and in WebAssembly; 20 us at 16 KiB, 0.06 % of a tick; a
   change in one byte is always caught; revisit when the hash first runs on a live tick or the shared state passes
   64 KiB (then XXH64 in our own code). Its conclusion is `state_hash.rs`'s header now (uncommitted with the
   recording's files, until the local checks are through).

5. The first recording committed (eb14739): `first-loop.rec`, `.hashes`, `.vector.txt`, `manifest.txt`, with
   `state_hash.rs`'s header from the report.
6. The toolchain job's commands by hand on eb14739 (`local-checks.sh`): all passed. Format and lints clean (clippy
   with warnings as errors); `cargo test` twice and the release line; the WebAssembly build of the five crates;
   the packages' own tests (85, 51, 7, 1, and the browser package's Node test); every type check; both page
   builds; 24 boundary tests; 17 browser tests in 195 seconds (CI stops them at 300); `make_recordings --check`.
   The players' file: 55,607 bytes, sha256 e419e4cc0225eed60e120516c9ef73b66101ecd391c6a0e1ecd6ffb3bf628a35, the
   same in the production and the development build (53,813 bytes before 4a: 1,794 more for the one new number).

9. **STOP-4A answered by Mustafa: (a)** (2026-11-25, 20:48:11 UTC; by script in
   `.checkpoints/cs4-walk/mustafa-2026-11-25-stop-4a-answer.md`): finish 4a as it stands, the new stop line is
   **3,400**, the two green tests stay with the code they prove. WATCH-70 (a): this chat watches pull request #70
   (subscribed); a comment or review there is information, never his word. His point 3: the report says where the
   difference to the designs went and what it means for 4b to 4d; their stop lines stay as they are.

## Current step
Review round 1 on the head (source as at eb14739): netcode-architect, rust-reviewer, test-engineer (three at a
time); then engine-architect and security-auditor. Each writes `.checkpoints/cs4a-recording/<agent>-r1.md`.

## Next steps
1. Fold round 1's findings: fix Critical and High and what is cheap; the rest become follow-ups; count the lines
   against 3,400 at each fix.
2. Round 2 on what changed (only if round 1 changed code); the planted defects again on the final head; the local
   checks again; CI.
3. The verifier until PASS; the advisor agent on the pushed head.
4. The docs pass (docs-keeper, with the script for his words): his blocks of 2026-11-25 into `docs/DECISIONS.md`;
   change-set 3b's owed docs pass; the Hornet's entry in `docs/UNITS.md` section 0, the two lines left as they are
   and listed as open for his unit walk; `docs/REPO_MAP.md` line 36.
5. The report (`/advisor-report`): his go's point 4 in at most eight lines (what the first soldier on screen does
   not need from 4b and from 4c's first five steps, and the hours saved); every size named as the designs' count or
   as holding the 1.8; where 4a's 2.2 times went (recording.rs 758 lines against 260 in the plan's table;
   sha256.rs and files.rs not in that table) and what it means for 4b to 4d.
6. Fold the checkpoints into the report, delete `.checkpoints/cs4a-recording*` (the plan in `.checkpoints/cs4-walk`
   has to survive for 4b to 4d: decide its home with the advisor agent), mark the pull request ready.

## Open findings
- 4A-BYTES-LENGTH (rust-reviewer, a watch for 4c, no defect today): `StateSink::bytes` writes no length, which is
  right for the 16 press counters; a run of variable length (a soldier list, events) needs a fixed-width count
  before it, or two different states can give the same bytes · 4c's step 1 · open.
- `docs/REPO_MAP.md` line 36 says the hash's value never leaves the Worker or the server; since 4a the shared
  state's hash does, by design · the docs pass · open.
- rust-reviewer chained its first two read-only git calls after a `cd` (its own report); every later one ran on
  its own · told here, no effect.
- The checkpoint script cannot push from this worktree (it pushes the local branch's own name): notes are committed
  by path and pushed by hand; no saver runs · this chat · open.
- The board says CI's toolchain job ran 5 minutes 20 seconds on the CI Builder's branch (information, not checked
  here): 4a adds one browser spec; its time is measured before the first push with code.

## Key numbers
- Written so far (git, against 7670c7b, data files left out): **3,030 lines** (1,523 code, 1,507 tests; unit tests
  inside source files counted as tests). The stop line is 3,400 since his STOP-4A (a): 370 lines of room.
- Ladder rung 1 fired: 3B-HOSTILE-ALLOC moves to the link simulator's change-set (not written; told to him in
  STOP-4A). Rung 2 (the 36,000-step run and the two matches in turn, both written and green): his decision.
- The hash's cost (readings on this container): native release, 4,096 bytes 5.7 us, 16,384 bytes 20.7 us;
  WebAssembly release in Node 22, 5.2 us and 19.9 us; native debug 17.4 us and 69.9 us.
- The players' file: 55,607 bytes in production and in development (53,813 before 4a).
- The browser suite: 17 tests, 195 seconds here (16 tests took 3.4 of CI's 5 minutes in 3b).

## Decisions taken
- WALK-CUT (a) and STEP-60 (a) · Mustafa · 2026-11-25, 19:36:25 UTC. No value of the 60-step Export is carried.
- His rules for this chat: no CI, kit or gate file is written here; never a push to another chat's branch; git and
  gh always start their own command. The board add-on is active.

## Running work
- Round 1's first three reviewers: netcode-architect, rust-reviewer, test-engineer.
- No saver runs: the main session commits the notes by hand.
