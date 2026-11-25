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
- **Last update:** 2026-11-25 21:02 Amsterdam (from the clock)

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

## Current step
**At the stop line: STOP-4A is asked of Mustafa** (`.checkpoints/cs4a-recording/question-2026-11-25-stop-4a.md`).
rust-reviewer writes the hash report (PM-0004-21). Until his answer only work that adds no product line.

## Next steps
1. rust-reviewer's verdict on the hash's function: KEEP, then the recording's files are committed (a commit without
   `[skip ci]`, so CI runs); REPLACE, then the function changes first and `make_recordings` makes the hashes again.
   Fold its report into `state_hash.rs`'s header (the report proposes the wording).
2. The three planted divergences (a script in the notes, like 3b's plants script), each caught at its tick.
3. His answer to STOP-4A; then the
5. Reviewers (netcode-architect, rust-reviewer, test-engineer; then engine-architect, security-auditor), the
   verifier, the advisor agent on every pushed head, the docs pass, the report, a draft pull request.
6. The docs pass also carries: his blocks of 2026-11-25 into `docs/DECISIONS.md` by script; change-set 3b's owed
   docs pass; the Hornet's entry in `docs/UNITS.md` section 0 by script, the two lines of that section left as
   they are and listed as open for his unit walk (his go, point 6).
7. The report answers his go's point 4 in at most eight lines (what the first soldier on screen does not need from
   4b and from 4c's first five steps, and the hours it would save to build those after the early look), and says
   for every size whether it is the designs' count or already holds the 1.8.

## Open findings
- The checkpoint script cannot push from this worktree (it pushes the local branch's own name): notes are committed
  by path and pushed by hand; no saver runs · this chat · open.
- The board says CI's toolchain job ran 5 minutes 20 seconds on the CI Builder's branch (information, not checked
  here): 4a adds one browser spec; its time is measured before the first push with code.

## Key numbers
- Written so far (git, against 7670c7b, data files left out): **3,030 lines** (1,523 code, 1,507 tests; unit tests
  inside source files counted as tests). 20 under the stop line.
- Ladder rung 1 fired: 3B-HOSTILE-ALLOC moves to the link simulator's change-set (not written; told to him in
  STOP-4A). Rung 2 (the 36,000-step run and the two matches in turn, both written and green): his decision.
- The hash's cost (readings on this container): native release, 4,096 bytes 5.7 us, 16,384 bytes 20.7 us;
  WebAssembly release in Node 22, 5.2 us and 19.9 us; native debug 17.4 us and 69.9 us.
- The development build's players' file: 55,607 bytes (it was 53,813 in production before 4a: production is
  measured before the first push without `[skip ci]`).

## Decisions taken
- WALK-CUT (a) and STEP-60 (a) · Mustafa · 2026-11-25, 19:36:25 UTC. No value of the 60-step Export is carried.
- His rules for this chat: no CI, kit or gate file is written here; never a push to another chat's branch; git and
  gh always start their own command. The board add-on is active.

## Running work
- rust-reviewer (the hash report), started 2026-11-25 about 20:57 Amsterdam; notes
  `.checkpoints/cs4a-recording/rust-reviewer-hash.md`, the report `.checkpoints/cs4a-recording/hash-report.md`.
- No saver runs: the main session commits the notes by hand.
