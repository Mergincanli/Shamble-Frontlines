# Advisor report: cs4a-recording (2026-11-25, branch `claude/cool-pasteur-oiqk1r`, base main 07d673b)

Change-set 4a, "The recording", the first of Walk's four parts (his WALK-CUT (a)). Draft pull request #70, lane 1,
by the chat he calls Builder V4. The branch is the cloud session's stand-in for `cs/4a-recording`. Risk tier 1 (game
code and tests; no manifest, lockfile, CI, kit or gate file is touched). Nothing a player sees, hears or feels
changes, so there is no browser check.

## Goal
A recorded match replays to the same number after every tick, on the server's kind of build and in the browser's
WebAssembly, on the very file players get; and a difference planted on purpose is found at its tick, with the field
that differs. This is the tool every later change-set proves "the same result everywhere" with (ADR 0004's
milestone), built before the soldier exists so that movement lands on it in 4c.

## Outcome
DONE. The first recording (1,800 ticks on today's state) replays to its 1,800 committed hashes natively (one
process; two more processes, with a 36,000-step match; two matches in turn), in Node on the players' file and on the
test module, and in Chromium 158 in a Worker on the players' file. One item moved by the plan's size ladder, told to
him in the question STOP-4A: the counted "no memory taken on hostile bytes" (follow-up 3B-HOSTILE-ALLOC) goes to the
link simulator's change-set, its other named home.

## His three asks for this report
### 1. Every size, named for what it is (his go, point 5)
| | 4a | 4b | 4c | 4d | Walk |
|---|---|---|---|---|---|
| **The designs' own count** (bottom-up, no factor) | 1,365 | 5,410 | 7,315 | 1,210 | 15,300 |
| **Planned: holds the 1.8** | 2,450 | 9,750 | 13,150 | 2,200 | 27,550 |
| **Stop line: planned times 1.25** (holds the 1.8 too) | 3,050, then **3,400** by his STOP-4A (a) | 12,200 | 16,450 | 2,750 | |
| **Written** (git's count, the recording's data files left out) | 3,030 before reviews; **3,300** final | | | | |

4a's written size is 2.2 times its designs' count before review fixes (he named this number) and 2.4 times at the
end; against the planned size that holds the 1.8 it is 1.35 times. The two rounds of review fixes added 270 lines.

### 2. Where the difference went, and what it means for 4b to 4d (his answer to STOP-4A, point 3)
The 1,665 lines between the designs' count (1,365) and the 3,030 written before reviews:
- **The recording's format, 498 more** (`recording.rs`: 758 lines against 260 in the plan's table). Every refusal
  has a name and a plain sentence (11 cases: about 90 lines); the hashes file and the manifest's lines live in this
  file (about 100 lines; the design counted them with the runner); its unit tests are 324 lines.
- **Two files the plan's table did not have, 286:** our own SHA-256 (`sha256.rs`: the hashes file names its
  recording by that digest, the workspace has no package for it, and 4a changes no manifest) and the folder check
  (`files.rs`: it names each wrong file and, for a hashes file, the first step that differs).
- **The runner and the comparer, 309 more:** the refusal of what this build cannot replay, the state as named lines
  and the field-by-field difference each got tests of their own.
- **The WebAssembly side, 235 more** (202 of it tests): the TypeScript reader is a second implementation of the
  format and got its own unit tests; the Node test also runs the test module in both build profiles.
- **The scripts, the tools and the exports, 190 more; the native tests, 143 more; the first two steps, 94 more.**
- **And 90 fewer:** the hostile-bytes count, which the ladder moved before it was written.
- **By kind:** 12 % of the written lines are comments and 7 % are blank. The designs counted statements; the
  project's style explains every public item and every test.

What it means, their stop lines staying as they are (his words): a stop line is 2.25 times the designs' count (1.8
times 1.25). A part that grows as 4a did reaches 2.2 times before any review fix and 2.4 times after: 4b about
11,900, then about 13,000, against 12,200; 4c about 16,100, then about 17,500, against 16,450; 4d about 2,650, then
about 2,900, against 2,750. **So each part is likely to reach its stop line and come back to him, as 4a did.** The
designs' counts for them were made the same way as 4a's. What I do without thinning a proof: count at every step
and say the count in each step's summary; let a ladder's rung fire when its line is reached, not after; write tests
as tables of cases where that says the same in fewer lines. What I cannot promise: that they land under their lines.

### 3. What the first soldier on screen does not need (his go, point 4), in eight lines
The splits are mine, from rows the designs give whole: estimates, by the designs' way of counting.
1. Movable from 4b, whole with their tests: the schema writer and the content types' export (about 240 lines), and
   the Testlines tool's two report-only modes, `compare-old` and `table` (about 120).
2. Movable from 4c's first five steps: the soft push between two soldiers (about 250: one soldier is on screen), and
   five small page pieces (the Worker's larger pool, the readout's new rows, the development aim line, the guard for
   the first picture after a pause, the check that development-only code stays out of production: about 270).
3. Together about 880 lines by the designs' count: 1,580 planned at 1.8, about 1,940 at 4a's 2.2. At change-set 3b's
   pace (2.13 hours a thousand written lines) the early look comes **about 3.4 to 4.1 hours sooner**, of about 49.
4. Of his three examples: the reload is already after the early look (it is in 4d). **The canonical writer cannot
   move:** his values enter the content through the tool that writes with it (`testlines write`), so without it a
   value would be typed by hand, and its proof ("writes back the same") stays with it. Of the content tools only the
   two report-only modes move.
5. Also not movable: the reader and its proofs, the built-file check for the fused multiply-add (his MUL-ADD ties it
   to the part that brings the collision library), the collision world, the time unit (before the first timer), the
   soldier's ids, `testlines --check`, and the camera panel (his words: from the first playable build).
6. The cost: 4b's pieces (360 by the designs' count) would merge with 4c, so 4c reaches its stop line unless about
   810 lines of stop line move with them (4c 16,450 to 17,260; 4b 12,200 to 11,390). 4c's own pieces only change
   order inside 4c. The advisor agent reads the new order before anything is built on it.
7. Within 4c alone (line 2 only): about 2 to 2.5 hours sooner; no pull request's content and no stop line changes.
8. No proof is thinner either way: each piece moves whole with its tests and is built after the early look and
   before 4c merges. The plan is not changed for this (his words); the decision is EARLY-LOOK-ORDER below.

## Plan and approval
- **The plan:** change-set 4's plan, revision 4, section 4 (`.checkpoints/cs4-walk/plan.md`, pushed at 25c42c5;
  readable there and at VERIFIED_PARENT, the last commit that holds the notes). Eight steps: one visitor that drives
  every writing of the state; the one new number on the players' file; the recording's format; the runner and the
  comparer; the scripts and two tools; the first recording on its native runs; the same in WebAssembly; the planted
  differences.
- **The advisor agent:** REVISE on draft 1 (three High, eight Medium), APPROVE on draft 3 with eight fixes, all made
  in revision 4 (its section 20 names where). None of its conditions was 4a's alone.
- **His go** (2026-11-25, 19:36:25 UTC, word for word in `docs/DECISIONS.md`): "My go on the plan for change-set 4
  (revision 4, pushed at 25c42c5). Start 4a, the recording." With it WALK-CUT (a) and STEP-60 (a).
- **His answer to STOP-4A** (20:48:11 UTC): "(a). Finish 4a as it stands; the new stop line is 3,400. The two green
  tests stay with the code they prove." And WATCH-70 (a): this chat watches pull request #70.

## What changed
All in lane 1's files. Steps with their commits: the visitor and the new number dac4d78; the format, runner, tools
and tests with the first recording eb14739; round 1's fixes 91741e6 and 40bf269; main's #71 taken in 21dff0c; round
2's small fixes fb5d0ff (the final source of `crates/` and `packages/`); his words by script 9b5c96a.
- `crates/sim/src/world.rs`, `state_hash.rs`, `lib.rs`: `StateSink` (the visitor's trait); `World::visit` and
  `visit_shared`; `World::shared_hash` and `state_hash`, fed straight from the visitor with no buffer (`Fnv1a64`,
  crate-private); `write_state` and `STATE_BYTES` are test-only now. The seed is handed over only through `secret`,
  and the shared part holds no seed byte (tested).
- `crates/room/src/lib.rs`, `crates/sim-wasm/src/lib.rs`, `local_match.rs`: `shared_hash(&self)` at the room, the
  local match and `Sim`. **The players' file gains exactly one export, `sim_shared_hash`** (18 before, 19 after; 3
  imports before and after), read only by a replay; no live tick calls it (pinned).
- `crates/devtools/src/` (the tools crate, never in the players' file): `recording.rs` (format version 1, a reader
  that checks every count before it takes memory and never panics; a public test seed only), `replay.rs`,
  `compare.rs`, `scripts.rs`, `files.rs`, `sha256.rs`, `bin/make_recordings.rs` (`--check`), `bin/replay.rs`
  (`hashes`, `check`, `state`, `fields`), three test-only exports in `wasm.rs`.
- `crates/devtools/recordings/`: `first-loop.rec` (122,487 bytes, sha256 1c05e6b8...6cf1), `first-loop.hashes`
  (1,800 lines: tick, shared hash, whole-state hash), `first-loop.vector.txt`, `manifest.txt`.
- Tests: `crates/devtools/tests/` `recordings.rs`, `two_processes.rs`, `two_instances.rs`, `hash_cost.rs` (a
  reading); `crates/sim/tests/state_hash_alloc.rs` and `crates/sim-wasm/tests/step_alloc.rs` (0 allocations, the hash
  read every tick); `packages/sim-worker/src/recording.ts` (a test-only reader) with `recording.test.ts`;
  `packages/sim-worker/boundary/replay.test.mjs` and the export pin in `sim-wasm.test.mjs`;
  `packages/e2e/tests/replay/determinism.spec.ts` (two Node names added to `node-names.d.ts`).
- Docs: `docs/DECISIONS.md` (his five messages of 2026-11-25 to this chat, word for word, by script; his board
  add-on is not recorded, by his own words in it), `docs/UNITS.md` section 0 (the Hornet's entry in his words; the
  two lines it meets stay as they are and are listed as open for his unit walk), `docs/STATUS.md`,
  `docs/STATUS_ARCHIVE.md`, `docs/ROADMAP.md`, `docs/REPO_MAP.md` (docs-keeper; it also carries change-set 3b's owed
  docs pass).
- Against the plan: every symbol of section 4 is there. Beyond it: `sha256.rs`, `files.rs`, the tool's `fields`
  mode; `shared_hash` takes `&self` and has no buffer (the plan said a buffer private to the room; two reviewers
  asked for the change). Not built: the hostile-bytes count (the ladder).

## Evidence
- **The legacy gate:** VERIFIER_GATE. CI on the pull request: gate and smoke green.
- **The toolchain job's commands by hand on the final source** (fb5d0ff; the notes' `local-checks.sh`): all passed.
  Format and lints clean (clippy with warnings as errors); `cargo test` twice (140 passed each time); the release
  tests of all six crates (139); the two vector generators' checks; the tools crate for wasm32; the packages' own
  tests (85, 51, 7, 1); every type check; both page builds; 24 boundary tests in Node; 17 browser tests in Chromium
  in 195 seconds (CI stops them at 300); `make_recordings --check` ("the recordings are what their scripts make").
- **The replay, target by target:** natively `recordings.rs` (every recording to its hashes, both columns);
  `two_processes.rs` (two child processes, each with its own hash-map seed, print every step's two hashes; the
  parent compares them step by step, with a 36,000-step match beside the recording); `two_instances.rs` (two
  matches stepped in turn in one process); in Node `replay.test.mjs` on the players' file and on the test module in
  both profiles; in Chromium the spec's line "first-loop.rec: 1800 steps, every shared hash as in the file".
- **Each check shown failing** (in the tests themselves): one digit of a hash changed; one byte of the recording
  changed; an input call skipped in the Node loop (found at step 46); a held button changed in the browser (step
  46); a recording with a seed that is no test seed refused by every replay path.
- **The planted defects** (the notes' `plants-4a.py`, run on 07903d9 with no reviewer in the tree; result
  `plants-4a.txt`): **15 defect checks caught, 4 controls held**, the tree put back byte for byte, the clean tree
  green. The simulation changed at tick 900: found at step 900 natively, in Node and in Chromium. A counter outside
  the state, and state that leaks only between two matches in turn: the two-matches tests go red, natively and in
  Node, while the solo replay stays green (the control). A hash map's order and a draw from outside the simulation,
  each at tick 600 in two processes: the step is named, and the tool names the field (`player.presses`). A value
  that differs in WebAssembly only: the native run stays green, Node finds step 600, the tool names the field. A
  `usize` in the canonical bytes: found at step 1.
- **The hash function's report** (PM-0004-21, by rust-reviewer, before the first recording was committed): keep
  FNV-1a 64. About 1.25 ns a byte natively and in WebAssembly; 20 microseconds at 16 KiB, 0.06 % of a tick; a change
  in one byte is always caught; 910,999 changed copies of a 3 KiB state, none hashing like the original. Revisit when
  the hash first runs on a live tick or the shared state passes 64 KiB (then XXH64 in our own code).
- **The players' file:** 54,514 bytes (53,813 before 4a: 701 more for the one new number), sha256
  a4d8184c15736ec2cf2c347871afddcff289dd0a364f0d2016c87dd9866cd094 here, in the production and the development
  build. On GitHub's runner: CI_DIGEST.
- **Counted, not inferred:** the two hashes take no memory from their first call (`state_hash_alloc.rs`); a tick of
  the local match with the hash read after it takes none over 18,000 ticks (`step_alloc.rs`).
- **The verifier:** VERIFIER_RESULT.
- **CI:** CI_RESULT.

## Reviews
Two rounds, the budget's whole; Opus 5.5 at effort max. Every finding and what happened to it:
`.checkpoints/cs4a-recording/round1-fold.md` at VERIFIED_PARENT.
- **Round 1** (on eb14739): netcode-architect REVISE (light), test-engineer REVISE (small), rust-reviewer APPROVE,
  engine-architect APPROVE, security-auditor APPROVE. No Critical, no High. Four Mediums, all fixed: two of the
  three planted differences were caught only as "the run differs", with no tick and no field (the two-process test
  now names the first step that differs; the tool names the field); one plant caught its defect by chance (1 time in
  64 it would not have: widened); `World::visit` had lost the guard that makes a forgotten field a compile error
  (put back). 31 Lows, several of them one finding from two reviewers: all fixed but four (a follow-up for
  change-set 8, one for 4c, a reading left as it is, and one that is this report's wording).
  The largest fix: the hash is fed straight from the visitor, with no buffer, which two reviewers asked for (the
  players' file shrank by 1,093 bytes and the export is read-only by its type).
- **Round 2** (on what the fix pass changed): **five of five APPROVE**; no Critical, High or Medium. Eight Lows: six
  fixed in fb5d0ff, one in the notes' script, one is this report's wording; beside them two follow-ups for 4c and a
  question for him (FIREFOX-REPLAY). test-engineer planted two defects of its own against the new tests (both
  caught); security-auditor ran three link cases in a copy (each refused, nothing outside touched); rust-reviewer
  rebuilt the players' file to the same bytes and ran its fuzz of the test module again (no trap).
- **Left open, a security finding named to him as the rules ask** (Low): the tests that *read* the recordings
  follow a link, so on a hostile branch a recording file replaced by a link to an endless device makes a test read
  until memory runs out. The folder check refuses such a link, so CI cannot go green with one; the write side is
  fixed. Its fix is a repository-wide "no tracked links" check: a gate change (risk tier 2), not this chat's to
  write. Decision LINKS-CHECK below.

## Decisions and deviations
- **The stop line:** reached at 3,030 written lines before any review (the first line was 3,050); the chat stopped
  and asked (STOP-4A); his answer moved it to 3,400. Final: 3,300.
- **The ladder's first rung fired** (at 2,450): 3B-HOSTILE-ALLOC moved to the link simulator's change-set. The
  second rung (the 36,000-step run and the two matches in turn, both already written and green) stayed, by his
  words.
- **The hash has no buffer** (above): a departure from the plan's words "its buffer is private to the room", for
  the better by four reviewers' reading; the committed hashes did not change.
- **Main moved twice:** #67 (docs) and #71 (the CI Builder's change) were taken in by merge commits; no conflict.
  Since #71 the pull request's CI runs every crate's release tests, the tools crate's among them.
- **`.checkpoints/` leaves the head with this report** (CHECKPOINTS section 9). Walk's plan, the designs and his
  saved messages stay readable at VERIFIED_PARENT; 4b's first commit puts `.checkpoints/cs4-walk*` back from there.

## Not verified
- No browser but Chromium ran the recording (Node and Chromium 158 only); Firefox and Safari did not.
- The WebAssembly-only difference was planted by hand (a stand-in): the search for a real input on which the two
  targets differ was not run. The draw from outside the simulation was planted natively only. Both run again, in
  their real form and on movement fields, in 4c.
- The planted defects ran on 07903d9, before round 2's small fixes; on the final source the verifier repeated two
  of them (above), not all fifteen.
- In CI a red replay names the step; the differing field comes from the `replay` tool, by hand.
- The two matches in turn run at the recordings' lengths (1,800 and 1,500 steps), not at ten minutes.
- In the plants' run the script mixed Node's error output into one state file, so that field comparison also
  printed four "no such line" entries, which the result does not show; the catch itself holds (test-engineer read
  it), and the script is fixed for its next run, not run again.
- The hash report's own list: no browser's WebAssembly (Node only), the real soldier table (a stand-in of the
  same shape), a server with many rooms, ARM, the players' file after `wasm-opt`.
- The eight lines of ask 3 are estimates: the designs give those rows whole.

## Risks
1. Each later part is likely to reach its stop line (ask 2). It costs a stop and his answer each time, not a proof.
2. The recording runs on today's state, a tick counter and 16 press counters: the format has room for the move, the
   aim and the view time, but movement first exercises them in 4c (the recording is made anew there).
3. The shared hash on the players' file can be run backwards (FNV-1a): harmless while a client's world is its own
   match, and it must hash only the client's own world before a build talks to a real server (follow-up below).

## Follow-ups (in `docs/STATUS.md` and its archive)
- **4c:** a fixed-width count before any run of varying length in the state (4A-BYTES-LENGTH); the three planted
  differences again on movement fields, and the real search for a WebAssembly-only difference; the paced replay
  behind a development marker, with its own `Sim` from the recording's test seed; the re-export
  `SHARED_STATE_BYTES` dropped; test-engineer's list (one known answer with every field non-zero read by both
  readers; the replay loop pinned to the live host's call order).
- **Change-set 7, binding from Slice 3's first build that talks to a real server:** the shared hash covers only the
  client's own world; no message carries a hash over state outside the receiver's view.
- **Change-set 8:** the random streams' key only through `secret`; the folder check reports the whole column first.
- **Slice 3:** the server refuses a live match on a seed that begins with `TESTSEED`.
- **The link simulator's change-set:** 3B-HOSTILE-ALLOC.
- **The quality gate step (2.3):** a repository-wide "no tracked links" check (LINKS-CHECK).

## Proposed next step
Change-set 4b, "Content and the world": the content crate, the collision world, the built-file check, his values by
script. Its plan is section 5 of the approved plan; the advisor agent re-reads it before the first edit.

## Decisions needed
1. **EARLY-LOOK-ORDER: build some pieces after the early look, so that the first soldier is on his screen sooner?**
   It matters because the early look is the first time he sees and steers the soldier, and the order of 4b and 4c
   decides when. The pieces and the numbers are ask 3 above.
   - (a) Keep the plan's order.
   - (b) Reorder inside 4c only: the soft push and five small page pieces are built after the early look. About 2
     to 2.5 hours sooner; no pull request's content and no stop line changes.
   - (c) As (b), and 4b's schema writer, the content types' export and the Testlines tool's two report-only modes
     also wait until after the early look and merge with 4c. About 3.4 to 4.1 hours sooner in all; about 810 lines
     of stop line move from 4b to 4c (11,390 and 17,260).
   - Recommended: **(b)**. It is most of the gain that costs nothing; (c) buys about 1.5 hours more and moves work
     between two pull requests.
2. **CONTENT-SOURCE: who writes the code that reads the content files?** (From the plan; needed before 4b's first
   edit.) It matters because that code runs on the server and in every browser for years, and each outside package
   in the players' file is held to the same checks as the collision library.
   - (a) The packages the content draft names (serde, serde_json and what they need, a path-to-error helper, garde):
     about 1,130 fewer lines of ours by the designs' count; five to eight more outside packages in the players'
     file; whether garde runs in WebAssembly is unread; the writer's number text differs from the browser's.
   - (b) serde_json's untyped tree with our own typed reader and ranges: about 440 fewer lines than (c); five
     outside packages; duplicate keys pass silently.
   - (c) All our own: no outside package; one reader on every side; the canonical text equal to the browser's own;
     about 1,130 more lines by the designs' count (about 2,000 planned), and a reader of ours that must be fuzzed.
   - Recommended: **(c)**, engine-architect's choice, for the reason he gave when he chose our own random number
     code (RNG-SOURCE (c)). It costs the first soldier a few hours.
3. **DEPART-4: does he accept the places where Walk reads or departs from the decision drafts?** (From the plan;
   needed before 4b's first edit.) It matters because the drafts are his accepted decisions; each departure is
   recorded by a dated pointer at the drafts' next revision, never by rewording now. The list (the plan's section
   13, item 11, shortened in wording only):
   1. The page keeps the three newest blocks, where the netcode and structure drafts say two.
   2. The time the page blends by travels on the block's message, not as a field inside the block.
   3. He accepted for change-set 3 that "three limits are code, not data, until the content crate exists in
      change-set 4". The crate exists from 4b; the three limits stay code until change-set 7, which rebuilds the
      room's command scheduling, so that they move once.
   4. The card's "the follow camera at legacy's framing" becomes his Export's framing; the old game's five presets
      keep their old numbers.
   5. The block has a fixed capacity of 48 soldier rows with the live count in its header, where change-set 1's
      note planned a count and an offset per section.
   6. The aim point's 16-bit centimetres reach 327.67 m from the map's origin: the playable map lies inside that
      square until the codec widens, and the 2 km test runs on the simulation alone.
   7. The build plan places the recording's WebAssembly half in CI later: it runs in today's CI since 4a.
   8. The movement table's values come from his Export, where the card says "from the legacy tuning import";
      "every kept value equal to legacy's" reads as every value his Export does not replace.
   9. Three marks are read differently: the random-walk test uses our own seeded generator, not the tool its row
      names; the two matches in turn differ in seed and input, and until change-set 8 only the input makes them
      behave differently; the jump's planted defect is the old game's order of two sums. And three smaller ones:
      the size "after wasm-opt" is recorded without it (the toolchain has no such tool yet); the collision tree is
      read node by node through the library's own public walk; the index of moving soldiers is a pair loop in slot
      order (48 at most), not a tree.
   10. The build plan's row "`cargo tree` shows `sim` depends on `content` only" reads as content and the collision
       library's tree.
   11. The simulation draft leans on the collision library's `enhanced-determinism` flag alone; Walk also takes the
       library's `std` off, uses typed calls only and adds the built-file check; and the card's "with legacy's
       numbers" keeps the old numbers but not the old game's order of the jump's two sums.
   12. The speed's easing is an exponential (the old game's and the sandbox's): its share per tick is worked out
       once per match with the pinned maths library's exponential, which gets its own vectors.
   13. Under CONTENT-SOURCE (c): the content draft's serde_json, garde and schemars are not used, and "garde range"
       reads as the field table's range.
   14. The collision library's own `std` feature is off if that builds, so its maths goes through the pinned `libm`
       on both targets; inside itself it runs std's `min`, `max` and `clamp`, which the float rules ban only in our
       own code.
   - (a) Accept all. (b) Accept with his changes.
   - Recommended: **(a)**.
4. **FIREFOX-REPLAY: should CI also replay the recording in Firefox?** It matters because "the same hashes in every
   browser" is shown in one browser today, and the replay needs no picture, which is what kept the other tests out
   of Firefox. It would widen the Firefox entry he accepted as cut to the rule tests (RULES-ONLY-READ (a)), and it
   is a CI edit, so the CI Builder chat would make it; it has not been run in Firefox, so it may go red.
   - (a) Yes: I ask the CI Builder chat (one line in the browser tests' setup); a red comes to him before any change.
   - (b) Not now: it waits for the quality gate step, with the other browsers.
   - Recommended: **(a)**.
5. **LINKS-CHECK: who closes the open security finding, and when?** It matters because a test run on a hostile
   branch can be made to read without end through a committed link (Low; CI cannot go green with such a link).
   - (a) It waits for the quality gate step (2.3), as a named follow-up.
   - (b) The CI Builder chat adds a "no tracked links" check now (a gate change, risk tier 2, his go).
   - Recommended: **(a)**: the exposure is a session running tests on a branch someone else wrote, which the lane
     rules already keep to reviewed branches.
Waiting for their part, unchanged: DIFF-LIST (with his new Export, before 4b's step 6), MOVE-RULES, RESUME-FLASH,
TUNE-HOME, SPRINT-DELAY-4, NO-SOURCE (before 4c); GREY-COLOURS is a default he can veto before 4b's step 9.

## For Mustafa
1. **The merge of pull request #70:** his go for the head the advisor agent approved, FINAL_HEAD, after his
   Advisor's merge check. No browser check: nothing a player sees changes.
2. **Letters** for CONTENT-SOURCE and DEPART-4 (4b cannot start its first edit without them), and for
   EARLY-LOOK-ORDER, FIREFOX-REPLAY and LINKS-CHECK.
3. **His new Export at 30 steps** (his STEP-60 (a)), pasted into this chat before 4b's step 6; no value of the
   60-step Export is carried.
