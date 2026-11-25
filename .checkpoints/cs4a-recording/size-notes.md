# 4a's size, for the report (his answer to STOP-4A, point 3; his go, points 4 and 5)

Counted by git against 4a's base 7670c7b at eb14739, the recording's data files left out (the script that splits
code from tests is in the chat's record: unit tests inside a source file count as tests). Each number is named for
what it is: "designs" is the designs' own bottom-up count; "planned" holds the 1.8; "written" is git's count.

## Step by step: the plan's table (designs' count) against what was written
| Plan's step | Designs: code / tests | Written: code / tests | Files |
|---|---|---|---|
| 1 the one visitor | 60 / 50 | 74 / 83 | `sim/src/world.rs`, `lib.rs` |
| 2 the shared hash and its pin | 25 / 30 | 27 / 75 | `room/src/lib.rs`, `sim-wasm/src/`, `step_alloc.rs`, the boundary pin |
| 3 the recording's format | 140 / 120 | 434 / 324 | `devtools/src/recording.rs` (758 lines; 260 in the plan's table) |
| 4 the runner and the comparer | 150 / 60 | 314 / 205 | `replay.rs`, `compare.rs` |
| 5 the scripts, the tools, the exports | 150 / 0 | 277 / 63 | `scripts.rs`, `bin/`, `wasm.rs`, `lib.rs` |
| not in the plan's table | 0 / 0 | 234 / 52 | `sha256.rs` (95 / 52), `files.rs` (139 / 0) |
| 6 the first recording's native tests | 0 / 200 | 0 / 343 | `tests/recordings.rs`, `two_processes.rs`, `two_instances.rs`, `hash_cost.rs` |
| 7 WebAssembly, Node and Chromium | 130 / 160 | 163 / 362 | `sim-worker/src/recording.ts` and its tests, `boundary/replay.test.mjs`, the e2e spec |
| 8 hostile bytes counted, the vector | 0 / 90 | 0 / 0 | moved by the ladder (3B-HOSTILE-ALLOC); the vector is inside steps 5 and 6 |
| **Total** | **655 / 710 = 1,365** | **1,523 / 1,507 = 3,030** | planned (holds the 1.8): 2,450; stop line 3,400 since STOP-4A (a) |

## Where the difference went (1,665 lines more than the designs' count)
- **The format, 498 more** (758 against 260): every refusal has a name and a plain sentence (an error type of 11
  cases with its text: about 90 lines); the hashes file and the manifest's lines are in this file (about 100
  lines) though the design counted them with the runner; the unit tests are 324 lines, most of it asserts that the
  formatter puts one argument a line.
- **Two files the table did not have, 286:** our own SHA-256 (the hashes file names its recording by that digest,
  the manifest lists each file with it, the workspace has no crate for it and 4a changes no manifest), and the
  folder check that names each wrong file and, for a hashes file, the first step that differs.
- **The runner and the comparer, 309 more:** the refusal of what this build cannot replay, the state as named lines
  and the field-by-field difference each got their own tests.
- **Tests on the WebAssembly side, 202 more:** the TypeScript reader is a second implementation of the format and
  got its own unit tests (157 lines); the Node test also runs the test module in both profiles.
- **The native tests, 143 more:** the folder check's five ways to fail; the two-process test's own report.
- **By kind:** of the 3,036 written lines, 366 are comments (12 %), 199 are blank (7 %), 2,471 are code. The designs
  counted statements; the project's style explains every public item and every test.

## What it means for 4b, 4c and 4d (their stop lines stay as they are: his words)
- 4a ran at 2.2 times its designs' count before review fixes; the plan multiplied by 1.8. A stop line is the planned
  size times 1.25, which is 2.25 times the designs' count. So a part that grows as 4a did ends within 2 % of its
  stop line before any review fix: 4b about 11,900 against 12,200; 4c about 16,100 against 16,450; 4d about 2,650
  against 2,750. Each part is likely to reach its stop line and come back to him, as 4a did.
- What I can do without thinning a proof: count at every step and say so in each step's summary; let a ladder's
  rung fire when its line is reached, not after; write tests as tables of cases where that says the same in fewer
  lines; keep an item out of a file's first version when the plan gives it a later home.
- What I cannot promise: that 4b to 4d land under their stop lines. The designs' counts for them were made the same
  way as 4a's.

## His go's point 4, the eight lines (draft; the splits are the main session's, from rows the designs give whole)
What the first soldier on screen does not need, each moved whole with its tests to after the early look and before
4c merges, so no proof is thinner and nothing leaves the code it proves:
1. From 4b: the canonical writer with its number text and its "writes back the same" half of content item 1 (about
   200 / 170 by the designs' way of counting); the schema writer and the content types' export (about 110 / 130);
   the tools' modes that need the writer or only report (`content-bundle`'s refusal of a file not in canonical
   form, `testlines compare-old` and `table`: about 110 / 60).
2. From 4c's first five steps: the soft push between two soldiers (about 110 / 140: one soldier is on screen); the
   Worker's larger pool, the readout's new rows, the development aim line, the guard for the first picture after a
   pause, the check that development-only code stays out of production (about 120 / 150 together).
3. Not movable: the reader and its proofs, the built-file check, the collision world, the time unit and its
   rounding report (it must exist before the first timer), the soldier's ids with their stale-id proof, `testlines
   write` and `--check` (his values enter by script), the camera panel (his words).
4. In all about 1,300 lines by the designs' count: 2,340 planned at 1.8, 2,860 at 4a's measured 2.2. At change-set
   3b's pace (2.13 hours a thousand lines) that is **5 to 6 hours sooner**, of about 49 to the early look.
5. The cost: the early look would run on content that no tool has yet written in canonical form (hand-checked
   files), and the plan's steps of 4b and 4c change order, so the advisor agent reads that order again.
