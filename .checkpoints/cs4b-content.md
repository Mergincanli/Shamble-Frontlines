# Checkpoint: cs4b-content
- **Goal:** change-set 4b, "Content and the world": the content crate (one loader, one canonical writer, all our own
  code by his CONTENT-SOURCE (c)), the built-file check for the fused multiply-add (his MUL-ADD), the integer time
  unit, the collision library and the static collision world, his Testlines values into the content by script, the
  page drawing the scene from the map. A short browser check (Brave, once Firefox) · plan:
  `.checkpoints/cs4-walk/plan.md` (revision 4, approved by the advisor agent; section 5, with 3, 9, 11, 14, 17) ·
  designs: `.checkpoints/cs4-walk/engine-architect-cs4.md` (blocks 1, 3.6, 3.7), `rust-reviewer-cs4.md` (blocks 1,
  2, 4), `testlines-cs4.md` · his words for it: `.checkpoints/cs4-walk/mustafa-2026-11-26-go-merge-70.md`
  (2026-11-26, 12:21:17 UTC), points 3, 4, 5, 7 and 9
- **Branch:** `claude/cool-pasteur-oiqk1r` on GitHub (the cloud session's stand-in for `cs/4b-content`). Local:
  `cs/4b-content` in the worktree `.gate/lane1-takeover`, made from main with `git switch -c`; pushed with
  `git push origin cs/4b-content:claude/cool-pasteur-oiqk1r`, alone in its command; never a plain `git push`. The
  session's start folder is never switched · **Base:** origin/main 2483e9a741c4e552323c36523831a2f35de4002e (the
  merge of 4a, pull request #70)
- **Last update:** 2026-11-26 16:03 Amsterdam (from the clock)

## Sizes and stop lines (his answer of 2026-11-26, 14:09:13 UTC: SIZE-4B (b); which number is which)
- **4b is two pull requests**, in engine-architect's cut; "each stands on its own: lines left over in 4b-1 do not
  move to 4b-2. At a stop line you stop and ask."
- **4b-1** (this card's first pull request: steps 1 to 5, no outside package): by the designs' way of counting
  about 3,300 lines; planned, which holds the 1.8: about 5,950; **stop line 7,450**; at 4a's measured rate (2.4
  times) about 7,950 written. **Two thirds of the line is 4,967**: at that count he gets one sentence on where
  the build stands.
- **4b-2** (steps 6 to 12): about 3,750 by the designs' count; planned about 6,750; **stop line 8,450**; about
  9,000 at 4a's rate. Its own "what leaves first" list is written before its step 6.
- Counted after every step with git against 4b-1's base (main 2483e9a), `crates/` and `packages/` only, data files
  left out (the committed extracts and corpus texts are data).

## What leaves 4b-1 first if the count runs high, in order (his point 2; fixed before step 1)
The build order puts what can leave last, so that a part that leaves is code not yet written. Order of building:
step 1 (manifests), step 2a (the reader, the writer, the bundle, the movement kind, the old tuning's numbers
extract), step 3 (the reader's proofs and the bounded fuzz run), step 2b (the map kind with its rules and the
aim-reach test; the proofs run again with it), step 4 (the links check), step 5 (the time unit).
1. **The time unit** (step 5; about 340 lines by the designs' count, about 800 written at 4a's rate) leaves first,
   whole with its tests and its report, to 4b-2. Nothing in 4b-1 rests on it. It leaves if the count after step 4
   is above 6,600.
2. **The links check** (step 4; about 55 by the designs' count, about 130 written) leaves second, to 4b-2 (his
   LINKS-CHECK (c) asks for it "in 4b": 4b-2 is still 4b). It leaves if the count after step 2b is above 7,300.
3. **The map kind** (step 2b; about 480 by the designs' count, about 1,150 written) leaves third, whole with its
   rules, the aim-reach constants and tie test and its generator classes, to 4b-2, where its first user (the
   collision world) is. It leaves if the count after step 3 is above 6,300.
Never leaves: the reader, the writer and the bundle with the movement kind; the reader's proofs (content items 1
and 2) and the bounded fuzz run (his words: before the first value rests on the reader). If these alone reach
7,450, I stop and ask.

## His answers of 2026-11-26 that bind 4b (word for word in the saved message)
- CONTENT-SOURCE (c): "all our own. Keep the reader to what the content files need and refuse everything else,
  duplicate keys included. Fuzz it before the first value rests on it."
- DEPART-4 (a): "all fourteen stand, with two notes. Entry 5: the 48 is one named constant, because my vision says
  48 and more. Entry 6: until the aim codec widens, the loader refuses a playable map larger than its reach, and
  STATUS names the change-set that widens it. If either is already so, say so in one line." Neither is built yet:
  the 48 rows are 4c's block (one named constant there, and in the generated layout); the map loader is 4b's (it
  refuses a map whose playable area leaves the square of 327.67 m around the origin); the change-set that widens
  the aim's 16-bit centimetres is named in STATUS in 4b's docs pass (which one: asked of the advisor agent at the
  re-read; no design names it yet).
- EARLY-LOOK-ORDER (b): inside 4c only; 4b's content is unchanged.
- LINKS-CHECK (c): "about 15 lines in 4b": the tests that read the recordings refuse anything that is not a plain
  file, as the folder check (`files::check`) does; the repository-wide check stays a follow-up for step 2.3.
- FIREFOX-REPLAY (a): "The CI Builder gets it from me today; a red comes to me first." Nothing for this chat to
  write.
- His Export at 30 steps comes before 4b's step 6 (his STEP-60 (a)); DIFF-LIST is asked with it. No value of the
  60-step Export is carried.

## Steps done
0. 4a merged on his go (pull request #70, main 2483e9a, by merge commit, expected head 420b16b). Walk's notes put
   back byte for byte from c17a054 (first commit of this branch). His message of 2026-11-26 saved by script.

1. The re-read sheet (`cs4b-content/reread-sheet.md`) and three reads. test-engineer's and security-auditor's
   reads of the test list (DONE both): folded into `cs4b-content/test-list.md`. **The advisor agent's re-read
   (`advisor-reread.md`, DONE): step 0 approved under conditions C1 to C7; steps 1 to 9 need a revised sheet (R1 to
   R11), which comes back to it before step 1's first edit.** Its rulings: the Testlines tool is a script on the
   platform's JSON reader, and so is every read of his Export and of `docs/legacy-tuning.json` (F.6, R1); the
   reader's numbers are narrowed to canonical text (R2); the aim reach lives in `content` with a tie test in
   `sim-wasm` (F.8, C3); the change-set that widens the aim is "the first that brings a map beyond it, at the latest
   the one that sets the command codec for online play" (F.1, its STATUS line is in its notes); ts-rs stays the
   type writer (F.3); lockfile by `cargo update --workspace` (C6).
2. His message of 2026-11-26, 13:05:03 UTC (`cs4-walk/mustafa-2026-11-26-source-go.md`): SOURCE-GO (a), into the
   Guard fix chat's queued rule change, no kit file from this chat, not a condition for 4b's merge; until then every
   pull request that changes anything under `content/source/` says so in the first lines of its report. The Export
   is read by a tool script with the platform's own JSON reader (he agrees, if the advisor agent approves: it did).
3. **Step 0, the manifest batch: in the working tree, UNCOMMITTED AND UNBUILT** (his DEP-HOLD). Root `Cargo.toml`
   (the `parry3d` line, `std` off; 29 overflow blocks), `crates/content/Cargo.toml` and an empty `src/lib.rs` with
   the lint guard, `crates/sim` (content, parry3d), `crates/sim-wasm` (content), `crates/devtools` (content; its
   three unused edges dropped on the grep's proof), `Cargo.lock` by `cargo update --workspace` (33 registry
   packages and `content`; sha256 f7233053...3667). The batch's text and digests are in the notes
   (`deps-manifests.diff`, `deps-lockfile.diff`, `deps-batch.sha256`). The build's own checks on the resolved tree,
   run with nothing compiled (`.gate/cs4b/check-crates.mjs`): overflow blocks complete for the WebAssembly and the
   host tree; **the feature compare is red** (`deps-feature-compare.txt`): `safe_arch` exists natively only, and
   `thiserror`'s `default` and `std` are turned on by the type writer in test builds only. With the library's
   `std` on, `default` still differs and six more packages enter (tried, put back).

4. Three reads, all DONE: netcode-architect (`netcode-architect-read.md`), engine-architect
   (`engine-architect-read.md`), rust-reviewer on step 0 (`rust-reviewer-step0.md`: the library's batch right as
   resolved; the two exceptions' exact wording for FEATURE-COMPARE (a); `std` stays off).
5. **The sheet's revision 3** (`reread-sheet.md`) is the plan 4b is built from. The advisor agent's second verdict
   (`advisor-reread.md`, "Re-read 2"): **steps 1 to 5 (4b-1: no outside package) APPROVED** with conditions A1 to
   A7, all applied in revision 3; **steps 6 to 12 (4b-2: the collision library and the world) HELD** for his
   FEATURE-COMPARE answer and a short re-read then (B1 to B7 already fixed in revision 3). The test list
   (`test-list.md`) carries the sheet's step numbers. **His "step 6" for the Export is step 8 in the sheet's
   numbering (the data step).**
6. The collision library's batch is out of the working tree, saved byte for byte under `.gate/cs4b/batch/`
   (digests `deps-batch.sha256`; a workspace copy for read-only tree commands under `.gate/rr-4b/ws/`).
7. **Step 1's own batch, path-only: APPROVED by security-auditor** on its three digests
   (`security-auditor-deps-4b1.md`; `dependency-check.md` is written and names them). The batch: `crates/content/`
   (`Cargo.toml` with no dependency; `src/lib.rs` with the lint guard), `crates/devtools/Cargo.toml` (`content`
   added; `client-endpoint` and `room` removed; `protocol` kept for the aim tie test), `Cargo.lock` (one path
   crate). **It is NOT in the working tree and not committed:** the advisor agent's A7 lets steps 1 to 5 start only
   once he has answered SIZE-4B. To put it back: the tracked files from `deps-4b1.diff`, the two new files from
   `cs4b-content/step1-files/` (their `.txt` copies), then compare with `deps-4b1.sha256`; a copy also lies under
   `.gate/cs4b/batch-4b1/`. security-auditor's two notes, neither blocking: `protocol` could be a dev-dependency
   of devtools (narrower; changes the digest, so it would ride with step 6's batch and its own answer); the
   comment in `content`'s manifest names `tests/dependency_rules.rs`, which arrives with step 1's check.

8. **His answers of 2026-11-26, 14:09:13 UTC** (`cs4-walk/mustafa-2026-11-26-size-4b.md`): SIZE-4B (b) as above;
   "4b-1 starts now: no outside package, no wait for my Export, no browser check"; **FEATURE-COMPARE (a)**: "Two
   named exceptions, each pinned to exact entries and versions, with tests and planted mistakes; any other
   difference still stops every build. Conditions: a change of parry3d's version or of the two packages' versions
   needs a new yes from me; the library's std stays off; the built-file check in 4b-2 keeps looking for SIMD and
   fused multiply-add in the players' file; and the recorded match exercises the collision queries from the first
   change-set that calls them, so native, Chrome and Firefox are compared on the real thing and not only on 10,000
   rays. Recorded as a dated pointer for ADR 0002's next revision." 4b-1's report says in its first lines what
   moved 4b from 5,410 to about 7,050 lines by the designs' count, and gives a new estimate for the first build he
   can move a soldier in. His Export comes before the data step (step 8, in 4b-2).

9. **Step 1 DONE** (commit d830680, pushed): the content crate with no dependency, the foundation check's
   `content` half reading the manifest's own text, devtools' two unused edges dropped.
10. **Step 2a DONE** (commit 945da99, pushed with d21b161; the local checks passed whole): the reader (`json.rs`),
    the numbers (`number.rs`), the canonical writer (`write.rs`), the typed read (`read.rs`), the bundle with `load`
    and `load_file` (`bundle.rs`), the movement kind (`movement.rs`), the refusals (`error.rs`), and
    `packages/client/scripts/legacy-extract.mjs` with the 2,038 numbers of the old tuning file as written.
    **2,888 written lines.**
11. **Step 3 BUILT, the local checks running on it (not committed yet): 4,824 written lines** (the sheet gave the
    step 800 by the designs' count; it took 1,936). What it holds:
    - `packages/client/scripts/corpus-texts.mjs` (+ test) writes `crates/devtools/vectors/content-corpus.txt`
      (13,067 numbers: the 2,038 old ones as written, 5,000 doubles from random bits, 5,000 decimals in many
      spellings, 1,000 doubles that may lie exactly between two shortest texts, 29 edges; each with the engine's
      own bits and text) and `content-trees.txt` (400 random trees, each as `JSON.stringify` writes it and as it
      writes it with sorted keys and an indent of two).
    - `crates/devtools/src/content_proofs.rs`: the corpus checks; a seeded generator of 33 classes of broken files
      and bundles, each case with the verdict it must get worked out by the generator (reason, file, line and
      column, field); `verdicts()` folds each class into a digest, committed as `vectors/content-verdicts.txt`
      (seed 20261126, 500 a class, the four megabyte classes 50, and the fuzz run's first 2,000 inputs); the
      inputs at the caps; the fuzz run's inputs and judge.
    - `crates/devtools/tests/content_proofs.rs` (5 tests) and `content_fuzz.rs` (a counting allocator; the caps'
      memory and time; **1,000,000 inputs in release, 50,000 in debug, on up to 8 workers: no panic, every promise
      kept**; each stage's minimum share asserted). Release: 26 s on 4 cores (93 s on one).
    - The test-only module's exports `content_corpus`, `content_trees`, `content_verdicts`, `content_limits`;
      `packages/sim-worker/boundary/content.test.mjs` (both profiles under Node: 7 tests, the WebAssembly verdicts
      equal the native digests; dev profile 18 s) and `packages/e2e/tests/rules/content.spec.ts` (release profile
      in the browser, 2.2 s in Chromium; the page also holds the corpus against its own engine; Firefox in CI
      only: no Firefox on this machine).
    - **A defect the corpus found in the writer, before any value rests on it:** a number exactly between its two
      shortest texts (89639294043007.125) is `…07.12` in a browser (ECMAScript: the even one) and was `…07.13`
      here (the standard library goes up). `number_text` now takes the even one (`even_of_a_tie`); 108 such
      numbers are in the corpus.
    - The reader names a byte no content file holds as that (`ForbiddenByte`) wherever it stands, not as "a key
      was expected".
    - A hand plant while building: a reader that takes tabs was caught by the class "a tab or a carriage return"
      with the file and the place (the plants script comes at the close).
    - **Departures from the sheet, for the report:** the proofs live in `devtools` (the WebAssembly side runs the
      same generator, and `content` depends on nothing), not in `crates/content/tests/`; `Field` and `Range` live
      in `read.rs`; the four classes that make megabytes run a tenth of the count; the browsers run the release
      profile only; the tree corpus is new (the test list's writer-structure point).
    - Three fresh seeds by hand: still to run and note (SHAMBLE_FUZZ_SEED).

12. **His Export at 30 steps arrived** (his message of 2026-11-26, 15:07:24 UTC, sent while the chat was working:
    "This is for later when you need it again"). Saved by script, never retyped: the whole message is
    `cs4-walk/mustafa-2026-11-26-export-30.md` (15,708 bytes); the Export alone, byte for byte, is
    `cs4b-content/testlines-export-30.json` (**15,508 bytes, sha256
    a9d47785ed8f70d895d10480d180c9bba5a67f7e955e0c460c0c1bef7718c4ab**) with `testlines-export-30.manifest.txt`
    (`export-save.mjs` wrote both): step rate 30, no warning, 18 entries changed from the start, **usable as it
    is**. Against the Export at 60 steps (4a's notes) 29 of 448 values differ and none of them is a value the game
    takes in 4b or 4c: the export's time, the step rate and its warning, the gun chosen in the sandbox
    (`values.soldier.gun`), the list of changed entries moved up by the step rate's own entry, and two measured
    pixel counts. It is needed at step 8 (4b-2), where DIFF-LIST is asked with it.

## Current step
4b-1, step 3: read the local checks (`.gate/local-checks.log`), commit, push, read CI, post on the board. Then
step 2b (the map kind with its rules, the aim reach, the tie test; the generator gets the map's classes), step 4
(the links check), step 5 (the time unit; game-designer reads the duration paths first).
**At 4,967 written lines he gets one sentence on where the build stands** (now 4,824: it comes early in step 2b).

## Next steps (the sheet's revision 3, section G; each step ends with the count of written lines)
- **4b-1:** 1 the content crate's manifests · 2 the reader, the writer, the bundle, the two kinds, the old tuning's
  two extracts by script · 3 the proofs (corpus, broken files, the bounded fuzz run) · 4 the links check · 5 the
  time unit (last, so that 4b-1's rung moves code not yet written). By the designs' count about 3,300; planned
  5,950; stop line 7,450 if he takes two pull requests.
- **4b-2 (held):** 6 the library's manifest batch with the feature compare · 7 the built-file check · 8 the tools
  and the data (**his Export at 30 steps; DIFF-LIST**) · 9 content to the simulation, the generated types, the
  frozen content of `first-loop` · 10 the static collision world · 11 the schema writer (first ladder rung) · 12
  the page's scene (second rung). About 3,750 by the designs' count; planned 6,750; stop line 8,450.
- Then per pull request: the gate and the local checks at every step; the plants script; reviewers (4b-1:
  rust-reviewer, engine-architect, security-auditor, then test-engineer; 4b-2 adds graphics-engineer and
  perf-auditor); the verifier; the advisor agent on every pushed head; docs-keeper (his messages of 2026-11-26
  into DECISIONS by script; STATUS's line 4B-AIM-REACH; the dated pointer for DEPART-4 entry 3 as he accepted it);
  the report (its first lines say when `content/source/` changes: his SOURCE-GO answer).

## Open findings
- From 4a (STATUS has them): 4A-NA-4B and 4A-EA-4B (content through the constructor only; `first-loop` made anew
  with its frozen content set and every hash line byte for byte; `sha256.rs` moves whole into the content crate if
  the players' file must compute the content's SHA-256) · 4b · open.
- The checkpoint script cannot push from this worktree: notes are committed by path and pushed by hand; no saver
  runs · this chat · open.

## Key numbers
- The players' file on main 2483e9a: 54,514 bytes, sha256 a4d8184c15736ec2cf2c347871afddcff289dd0a364f0d2016c87dd9866cd094.
- The collision library by rust-reviewer's paper reading (to be confirmed by `cargo tree` after the hold): 33 new
  lockfile packages, 31 in the players' file's tree; growth of the players' file estimated at 60 to 160 KB.

## Decisions taken
- His go for the merge of #70 and his answers to the six decisions · Mustafa · 2026-11-26, 12:21:17 UTC.
- His rules for this chat: no CI, kit or gate file is written here; never a push to another chat's branch; git and
  gh always start their own command, one git per shell call. The board add-on is active.

## Running work
- No agent runs. No saver runs: the main session commits the notes by hand.
