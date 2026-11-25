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
| R8 | Low, optional | The hash can be fed straight from the visitor, with no buffer in the room: the players' file would grow by 701 bytes and not 1,794 (measured on a copy: 54,514 bytes; first-loop still replays to all 1,800 committed hashes, both columns), the export becomes `&self`, and nothing can allocate. About 60 lines | **Fix now** (engine-architect's E1 says take it now) |
Its answers to the six questions, in short: no input makes the reader, the hashes parser, the replay or the two replay
exports panic or take memory before a count is checked (R3 aside): read, and fuzzed in wasm32 in both profiles (20,000
mutated inputs, every step count that would wrap a 32-bit product, all 256 map-id lengths; a planted overflow shows
the harness sees a trap). No float, `usize`, hash map or clock reaches the canonical bytes or the committed files.
`sha256.rs` is right at 312 lengths against Python's hashlib and at 2^29 + 1 bytes on wasm32. Where the 1,794 bytes
went: 1,307 of code (the hash 323, `Room::new` out of line 322, the visitor 206, the export's shim 181, a copy of
`Vec`'s grow path about 273), 448 of function names, 39 of tables. Its copies `.gate/rr-4a` and `.gate/rr-4a-base`
are removed with `git worktree remove` when the round is over.

## test-engineer: REVISE (small); no product defect found; the gaps are in the evidence (`test-engineer-r1.md`)
| # | Severity | Finding | Decision |
|---|---|---|---|
| T1 | Medium (High if the report said "three of three") | The same as N1: the hash map's order and the outside draw are reported only as "digests differ" (`two_processes.rs:92`): no tick, no field | Fix: the two-process test names the first step that differs (about 15 lines); the plants take the tick from `replay check` and the field from two `replay state` runs in two processes (notes) |
| T2 | Medium | Plant C catches its defect only by chance: its fold has 64 values (measured on 200,000 hash maps: two agree 1 time in 64); in the first run its half on the committed recording missed, and only the 36,000-step half caught it | Fix in the plants script (notes): fold the order into 8 bytes, as plant D does, and print both orders |
| T3 | Low | Plant B's counter survives between runs, so solo checks fail too: it cannot show what the two-matches tests add. test-engineer's own plant (state that leaks only between interleaved matches) turns exactly N3 and W2 red and leaves the solo checks green | Fix in the plants script (notes): add that plant, keep B |
| T4 | Low | No test makes `replayOn` refuse a recording: with its check deleted `recording.test.ts` stays green (the refusal lines call `checkReplayable` directly) | Fix, about 3 lines |
| T5 | Low | The TypeScript reader's refusal of a map id that is no UTF-8 is reached by no test, and its order differs from the Rust reader's by one check, against its own header comment | Fix, about 3 lines: one test, and the order made the Rust reader's |
| T6 | Low | A Rust test is named "refused before any memory is taken" and asserts only the errors | Fix, 1 line: the name |
| T7 | Low | The plants' evidence: no commit id in the result; the clean run never checks that the tree equals HEAD; the script edits the worktree other reviewers read; "11 of 11" is 6 defects on 9 targets, one field line and one control | Fix on the next run (notes): print the commit, end on an empty `git status` for `crates` and `packages`, run in a worktree of its own, count the lines for what they are |
| T8 | Low | `hash_cost.rs` cannot fail (a reading) and takes 1.7 seconds of every debug run | Leave: it is the report's reading and CI's log shows it ran |
| T9 | Low, wording for the report | Say: the two-process test runs in debug in CI (the release line is `-p sim -p protocol` until the small CI change E1); PM-0004-17 runs at the recordings' lengths (1,800 and 1,500 steps), not M10's 18,000; plant E is a stand-in and the search for a sine input where the two targets differ was not run; in CI a red replay names the step, and the field comes from two dumps by hand | The report |
Its mark-by-mark table: every mark 4a carries is shown by a test that goes red on its defect, but PM-0004-21's "each
of three at its tick with a field" (2 of 3 open: T1). Follow-ups it names for 4c: one known answer with every field
non-zero, read by both readers; the two-process test's line filter (N6); the real sine search and the three
injections on movement fields; the replay loop pinned to the live host's call order and its rebase on resume;
`scripts.rs`'s button numbers pinned to `keyboard.ts`. For the small CI change E1: the tools crate's tests in release.
It left `.gate/te-4a` (a worktree), `.gate/te-4a-target` and `.gate/te-scratch` behind: its delete was refused.

## engine-architect: APPROVE; nothing Critical, High or Medium in its lane (`engine-architect-r1.md`)
| # | Severity | Finding | Decision |
|---|---|---|---|
| E1 | Low | Its answer to R8: take it now. "Read-only" is true of `shared_hash` but rests on three function bodies, not on the signature (`&mut self`), and the room reserves a buffer by a constant that 4c turns into a length per match. Feed FNV-1a straight from the visitor (one encoder that writes into a `Vec` or into the hash's state; a plain struct, not `core::hash::Hasher`, whose docs call its bytes "not portable across platforms"), `shared_hash(&self)` at the room, the local match and `Sim`, the room's buffer gone, and one test that the streamed hash equals `fnv1a_64` over `write_shared_state`'s bytes | **Fix now, about 50 to 60 lines** (R8 and E1 agree; the players' file shrinks by 1,093 bytes by rust-reviewer's measurement) |
| E2 | Low | `StateSink`'s doc says "Every writing of the state goes through it": read to the letter that invites 4c to write the snapshot or the block through the visitor, which would tie the wire format to the hash's order and put the seed one mistake from a client | Fix, 2 lines: "every canonical writing (the hashes' bytes, the named dumps); the snapshot and the block keep their own layouts" |
| E3 | Low | `bytes(&[u8])` writes no length (rust-reviewer's 4A-BYTES-LENGTH) | Fix, about 5 lines: the type `&[u8; N]`, so a run of varying length cannot be handed to it |
| E4 | Low | `Replayable` in `recording.ts` is written by hand and nothing ties it to the generated `Sim` type: when 4c widens `input` to 13 numbers a 6-number replay would still type-check and run | Fix, about 3 lines in `recording.test.ts`, which CI type-checks with the generated types |
| E5 | Low, optional | The pin catches a new method's name but not a changed shape of `shared_hash`; `sim-wasm/src/lib.rs`'s header does not list this number | Fix, about 3 lines |
Its measurements: the players' file's imports are 3 before and after; its exports 18 before, 19 after, the one new
being `sim_shared_hash`; in the production Worker's bundle `shared_hash` appears once (the glue's definition) and
nothing calls it; none of the test-only reader's strings is in either built page. Every touched file is lane 1's.
For 4b (no line now): content enters through the constructor only; if the players' file must compute the content's
SHA-256, `sha256.rs` moves whole into the content crate, never a second copy; first-loop's header is re-made with
its frozen content set and every hash line stays byte-identical, which proves that loading content changed no state.
For 4c: `u8`, `f32` (through `to_bits`) and `row()` join `StateSink`; the paced replay reads the hash through a
development-only driver, never through the live host's interface; the button meanings get one home in Rust.

## security-auditor: APPROVE; nothing Critical, High or Medium (`security-auditor-r1.md`)
A security finding that is not fixed in the pull request counts as left open and is named to Mustafa in the report
(`.claude/agents/advisor.md`): so all four are fixed now.
| # | Severity | Finding | Decision |
|---|---|---|---|
| S1 | Low | The same as N2 and R2, plus: `Header` derives `Debug` with a public seed, unlike `World` and `MatchSetup` | Fix: the seed is checked in `Replay::new` and in `checkReplayable`, each with a refusal test; `Header`'s `Debug` is written by hand and never shows the seed (about 24 lines) |
| S2 | Low | `world.rs`'s comment calls the shared hash "the one a client's state can be compared with"; the network draft says no state hash is exchanged with clients, and from 4c the shared part holds every soldier | Fix, 2 lines: a number a replay compares across targets, never exchanged with a client |
| S3 | Low, hardening | A link committed in place of a recording file is followed: in a copy, `make_recordings` in write mode overwrote a file outside the repository through such a link, and `--check` cannot see links. CI's own check goes red on it, so it cannot merge green; the exposure is a session writing on a hostile branch | Fix, about 19 lines: `write` and `check` refuse anything that is not a plain file, with a test; found names are printed quoted. A repository-wide "no tracked links" check would be a gate change (risk tier 2): a follow-up for the quality gate step |
| S4 | Low | With `SHAMBLE_TWO_PROCESSES_CHILD` already set in the environment the two-process test passes without starting a child or comparing anything | Fix, about 2 lines: the child's role also needs the parent's own arguments |
Its conditions for later change-sets (not findings against 4a; they go to STATUS as follow-ups):
- **C1, decided at change-set 7 and binding from Slice 3's first build that talks to a real server:** before
  `Sim::shared_hash` is in a build whose authority is elsewhere, it hashes only the client's own world; no message
  carries a hash over state outside the recipient's interest set and the server never answers a client's hash with
  match or mismatch over such state; tests pin both. Why: FNV-1a can be run backwards; its script got 4 hidden bytes
  back from one hash in 0.06 seconds (`sa_fnv_unwind.py`).
- **C2, change-set 8:** the random streams' key and buffered output go only through `secret`; a test that the dumps
  of two worlds that differ only in their seed are identical.
- **C3, Slice 3:** the server refuses to start a live match on a seed that begins with `TESTSEED`.
- **C4, the paced replay in 4c:** behind a development marker the production message check refuses; its own `Sim`
  from the recording's test seed, never the live match; `replayOn` keeps calling `checkReplayable`.
It left `.gate/sa-target` behind (its delete was refused).

## The fix pass: the lines it may take
Product lines planned: about 220 (the buffer-free hash 65 with its callers; the seed checks and `Header`'s `Debug`
24; the two-process test's step and prefix and role 20; the plain-file check 19; the lint guard on the reader 11;
the `replay` tool's field comparison 12; the rest in ones to tens). 3,030 + 220 = about 3,250 of 3,400.

## What the fix pass did (source at 40bf269, plus one type fix in the browser spec)
Every "Fix" above is made; no finding of round 1 is deferred but the ones marked as follow-ups for a later change-set
(N4, N5; T8 is left as it is; T9 is the report's wording). Written lines after the pass: **3,291** (git, against
7670c7b, the recording's data files left out): 255 more than before it, 109 under the stop line of 3,400.
- R1: `World::visit` names every field again.
- R8, E1: the hash is fed straight from the visitor (`Fnv1a64`, a plain struct); `state_hash(&self)` and
  `shared_hash(&self)` take no buffer; the room's buffer is gone; `shared_hash` takes `&self` at the room, the local
  match and `Sim`. The committed hashes are unchanged. The players' file: 54,514 bytes (55,607 before the pass; 53,813
  before 4a), sha256 198477964866d717fff105ebe4b64e5fb8aee1c861e06d4e460e2bf1a7fff708 here.
- E2, S2, R5, R4, N7, N8, E5: comments and doc lines. E3: `bytes` and `secret` take runs of a fixed length only.
- N2, R2, S1: the test seed is checked in `Replay::new` and in `checkReplayable`, each with its refusal test; `Header`'s
  `Debug` never shows the seed.
- N3: a test that a rebase before every step changes no hash.
- T1, N1, N6, N9, S4: the two-process test prints every step under a fixed prefix, names the first step that differs,
  takes a child's role only with the parent's own arguments, and prints the two fingerprints.
- R7: the `replay` tool's `fields` compares two state prints (`compare::field_diff` has a caller now).
- S3: `files::write` and `check` refuse a link or anything that is no plain file, with a test that nothing is written
  through a link; names are printed quoted.
- R6: `recording.rs` carries the room's lint guard; its two products saturate.
- R3: the timing helper's length is capped.
- T4, T5, E4: the TypeScript reader checks in the Rust reader's order; its replay's two refusals are tested with no
  call made; `Replayable` is pinned to the players' generated type.
- N9: the browser spec checks the hashes file's digest against the recording.
- T6: the test's name says what it asserts.
- T2, T3, T7 (notes): the planted-defects script is rewritten: plant C folds 8 bytes; a new plant G leaks state only
  between interleaved matches and has controls that must stay green; plants C, D and E show their tick and their
  field through the tools; the result counts defects and controls apart, names its commit and says whether the tree
  was put back.

# Review round 2, on what the fix pass changed (`git diff eb14739..07903d9 -- crates packages`)
The second and last round: a third only for an open Critical or High. A finding below High rides in the next push if
it is a line or two, or becomes a follow-up.

## rust-reviewer: APPROVE; all eight of its findings fixed (`rust-reviewer-r2.md`)
It rebuilt the players' file as the build does: 54,514 bytes, sha256 19847796...f708, the same as here, and the same
function bodies as its own round 1 variant (one byte differs: a panic message's line number). The test module's fuzz
ran again at the fixed head: no trap, the same counts as round 1.
| # | Severity | Finding | Decision |
|---|---|---|---|
| R2-1 | Low | `sim-wasm/tests/step_alloc.rs`'s comment still says the hash's buffer "was reserved with the room"; the buffer is gone | Fix, 1 line, in the next push |
| R2-2 | Low | `World::write_state` writes the whole state, seed included, into a caller's buffer, and nothing outside the simulation's own unit tests calls it any more | Fix, 2 lines: `#[cfg(test)]` on it and on `STATE_BYTES` (engine-architect is asked whether 4c or change-set 7 needs it public) |
| nit | Low | `scripts.rs` line 3 is 188 characters long after the fix; the writer's reserve dropped the header, so every encode reallocates once | Fix, 3 lines |

## test-engineer: APPROVE; all seven of its fix findings fixed (`test-engineer-r2.md`)
PM-0004-21's "each of the three injections caught at its tick with the field" is shown now, on today's state. It ran
two plants of its own against the new tests: a seventh number on the generated `input` fails the type pin; a rebase
that also runs a tick turns the rebase test red.
| # | Severity | Finding | Decision |
|---|---|---|---|
| T2-1 | Low | The plants script merges Node's error output into plant E's WebAssembly state file, so the field comparison also printed four "no such line" entries, which the result hides by showing only the `player.` lines; the catch itself holds | Fix in the notes script for its next run (2 lines); one sentence in the report |
| T2-2 | Low, wording | The pull request's CI job tests the head merged into main, which since #71 runs every crate's release tests, the tools crate's among them: so "the two-process test runs in debug only" is out of date for the pull request's run | The report says: release in the pull request's CI since #71 |
The report still owes its three caveats: plant E is a stand-in; plant D was planted natively only; in CI a red replay
names the step and the field comes from the tools by hand.

## Main moved during round 2: #71 (the CI Builder's change) is taken in
Merge commit 21dff0c (main 07d673b): CI's toolchain job now runs every crate's release tests (the tools crate's among
them), the two vector generators' checks, the tools crate's own wasm32 build, and the rule tests in Firefox;
`packages/e2e/playwright.config.ts` gained a `firefox` project for `tests/rules` only. No conflict: this branch had
touched none of its files. Run here on the merged tree: the release tests of all six crates (139 passed), the tools
crate for wasm32, both generators' checks. A wish for the CI Builder, not done here: the replay spec needs no WebGL
and could join the `firefox` project, which would show the recording in a second engine (follow-up 4A-FIREFOX-REPLAY).

## netcode-architect: APPROVE; every finding of its that was due is fixed; no new finding (`netcode-architect-r2.md`)
On the buffer-free hash: nothing in its design for 4c wants it otherwise, and each of the three cases is better off (a
shared length per match: the room's one buffer sized by a constant is gone; the soldier table: runs of a fixed length
force the shape 4c needs, a count and then rows; change-set 7: a hash that borrows the world and takes no memory suits
hashing a restored or predicted world every tick). A note for 4c, no line now: hand over every slot of the soldier
table, with its occupied flag and generation, so that line i of a state print is always slot i and the field
comparison never misaligns on a missing row.

## security-auditor: APPROVE; its four Lows fixed; one new Low (`security-auditor-r2.md`)
It ran the link cases in a copy: a recording file as a link to a file outside the repository, the folder itself as a
link, a link to an endless device: write mode and `--check` refuse each, quote the path, and nothing outside is
touched. With the child's variable set but not the parent's arguments the two-process test acts as the parent and
compares. On the buffer-free hash: it removes two places where state bytes lay (the room's buffer of shared bytes
and the tools' buffer of whole-state bytes, seed included); the export is read-only by its type.
| # | Severity | Finding | Decision |
|---|---|---|---|
| SA2-1 | Low | `replay fields` prints the differing lines of two files as they are: a state print from someone else's machine could carry control characters to the terminal (shown with a title-setting sequence) | Fix, 1 line, in the next push: each printed line is escaped |
**Left open, to be named to Mustafa in the report:** S3's read side. The tests that read the recordings still follow
a link (with `first-loop.rec` linked to an endless device a test reads until memory runs out); the folder check
refuses such a link, so CI cannot go green with one, and the exposure is a test run on a hostile branch. Its fix is a
repository-wide "no tracked links" check: a gate change, risk tier 2, a follow-up for the quality gate step (2.3).
Its conditions C1 to C4 stay follow-ups for the change-sets it named.

## engine-architect: APPROVE; all five of its findings fixed (`engine-architect-r2.md`)
The export's surface, measured again on the built file: 3 imports before and after; 18 exports before, 19 after, the
one new being `sim_shared_hash`; one definition and no caller in the production Worker's bundle; none of the reader's
strings in either built page. It agrees with making `World::write_state` and `STATE_BYTES` test-only (R2-2): neither
4c nor change-set 7 needs them public.
| # | Severity | Finding | Decision |
|---|---|---|---|
| E2-1 | Low, nit | `Fnv1a64` is public and has no user outside the simulation crate | Fix, 1 line: `pub(crate)` |
| E2-2 | follow-up | `SHARED_STATE_BYTES` lost its last user outside the crate; 4c turns it into a length per match | 4c: drop the re-export then |
| E2-3 | follow-up | Once `write_state` is test-only, `secret` is the only public way seed bytes leave the crate: a wrapper that outside readers can only count would make the seed rule one the types enforce (about 10 lines) | 4c, or change-set 8 at the latest |
| E2-4 | a question for Mustafa, no fix | The replay spec needs no WebGL: one line in the browser tests' setup would run the recording in Firefox too; that widens his FIREFOX-SOURCE decision, so it needs his word | The report's "Decisions needed" (FIREFOX-REPLAY) |

## Round 2's verdicts: five of five APPROVE; no Critical, High or Medium open
The small fixes that ride in the next push (about 12 lines): R2-1 (a stale comment), R2-2 (`write_state` and
`STATE_BYTES` test-only), the two nits (a long comment line; the writer's reserve), SA2-1 (the field comparison's
print is escaped), E2-1 (`Fnv1a64` is `pub(crate)`), and T2-1 in the notes script. No reviewer round follows them: the
verifier and the advisor agent read the final head.
