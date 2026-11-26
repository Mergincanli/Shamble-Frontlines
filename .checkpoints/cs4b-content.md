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
- **Last update:** 2026-11-26 18:51 Amsterdam (from the clock)

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
11. **Step 3 DONE** (commit bdc85ca, pushed; the local checks passed whole; **CI green**: toolchain 6 min 52 s, 71 s
    more than on step 2a; Chromium 18 passed in 3.3 min; **Firefox 160 passed 3, the new content spec among them**:
    the corpus is also what Firefox's own engine gives for all 13,067 numbers): **4,824 written lines** (the sheet gave the
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
    - Three fresh seeds by hand (`SHAMBLE_FUZZ_SEED`, release, 1,000,000 inputs each): 20261127, 777 and 31337:
      no panic, every promise kept; the stages' shares within 1 in 100 of the committed seed's.

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

13. **Step 2b DONE** (commit 0d4505f, pushed; the local checks passed whole: 18 browser tests in 209 s): **5,868 written lines** (the step took
    1,044). What it holds:
    - `crates/content/src/map.rs`: the map kind (`shamble-map` 1): `bounds`, at most 16 `spawns`, 512 `shapes` (a
      box or a ramp inside its box, axis-aligned), 64 `regions`; the typed read and the map's own rules (`check`):
      the bounds on whole centimetres and every other corner on whole millimetres; at least 1 cm thick on every
      axis, counted in whole steps (0.14 to 0.15 is 1 cm although the subtraction of the two doubles gives less);
      every shape, spawn and region inside the bounds; at least one spawn; an id once in its list; **the bounds
      inside the aim's reach** (`AIM_REACH_CM` 32,767 both ways on x and z, `AIM_HEIGHT_MAX_CM` 65,535 from the
      floor at 0: his DEPART-4 note). Five new refusals (`OffGrid`, `TooThin`, `OutsideBounds`, `BeyondAim`,
      `DuplicateId`), each with its record and sentence. `read.rs` reads a corner (`numbers::<N>`); the bundle
      holds the folder `maps/`.
    - `crates/devtools/tests/aim_reach.rs`: the reach against the aim point's own field types (a wider field
      fails it, by name).
    - The proofs' generator makes maps (the document built in code, never read) and four new classes (a valid
      map; a list too long or a word it does not know; a box off its grid, too thin or outside; past the reach,
      no spawn, an id twice); bundles hold maps; the fuzz run mutates them. **37 classes; 1,000,000 fuzz inputs
      in 34 s; three hand plants in map.rs each caught** (a reach 1 cm too wide, 9 mm taken as 1 cm, a point
      under the bounds taken).
    - **Departures from the sheet, for the report:** every corner of a map but the bounds lies on whole
      millimetres (so that "at least 1 cm" is exact: a rule the sheet did not have); the loader asks for at least
      one spawn, not "the match's slot count" (the content does not know the mode; the match's own count is
      checked where the match is set up, 4c); the map's own rules name the field and no line.
14. **game-designer's read of the old tuning file's durations: DONE** (`cs4b-content/game-designer-durations.md`,
    `legacy-duration-paths.txt`; input `duration-candidates.txt`, 727 fields, by `duration-candidates.mjs`): 52
    fields holding 69 numbers are durations (50 fields in seconds, 2 in milliseconds; 42 fields rules, 10 look);
    all are whole milliseconds but `FIXED_DT`, the old game's own 60 Hz step, which it left to the builder: kept on
    the list as kind `step` (no interval of play; the tick replaces it), by script, said in the list's first line.
    **Its question for Mustafa, DUR-1** (which fire rate counts as the old game's: the old code never carries a
    timer's leftover, so the Shabzon fired about 720 shots a minute at 60 frames a second where the table's
    interval gives 833; ADR 0004 carries the leftover; its recommendation: the table's value now, tuned by play):
    goes into 4b-1's report; it binds change-set 8 (PM-0004-9), not 4b. Its note for the builder: some of the old
    game's timers are typed into its functions and are in no table (a bot's lunge 2.5 s, fire locks of 0.1 to 0.2
    s): the report cannot see them; 4d's import and change-set 8 will.
15. **Step 4 DONE** (pushed with step 5 after the full local checks passed on both): **6,091 written lines**
    (the step took 223): `devtools::files::read_plain` and `packages/sim-worker/boundary/plain-file.ts`, the six
    readers moved, planted links on both sides, a text scan in `tests/recordings.rs`. The folder check of 4a also
    asks for the folder by its own path with no last separator.
16. **Step 5 DONE** (commit fdd9f0a, pushed; the local checks passed whole: 33 boundary tests, 18 browser tests in 209 s): **6,592 written lines** (the step took 501):
    `crates/content/src/time.rs` (`UNITS_PER_SECOND` 3,000, `units_from_seconds`), `crates/sim/src/time.rs` (the
    same number, `UNITS_PER_TICK` 100, a constant check for 30 and 60 Hz), `legacy-extract.mjs` writes
    `vectors/legacy-durations.txt` (69 durations as written) from the reviewed list
    `vectors/legacy-duration-paths.txt`, `devtools::time_report` with its tool (`time_report [--check]`) and the
    committed `vectors/time-report.txt`: **3,000 holds 68 of the 69 with no rounding (the 69th is the old step);
    600 rounds the 0.072 s fire interval by 0.46 %; 60 rounds 12 of them**; a rule's interval under one tick fails
    the report (a planted 0.02 s does). `tests/time_unit.rs`: the oracle is three units a millisecond of each
    text's own digits. **Departures:** the reviewed list is a data file the script reads, not a constant in the
    script; the report does not list the 675 fields that are no durations (game-designer's notes name the
    duration-like ones left off); no shot counts over 60 s (PM-0004-9 is change-set 8's).
17. **His message of 2026-11-26, 15:35:38 UTC (VISION-RULE)**, sent while the chat was working (queued command
    dcae7c82-851e-46e8-afe6-b0ee784ab13c in the transcript): a game rule to record "at 4b-1's docs pass and not
    before" (saved by script then: DECISIONS, `docs/adr/README.md`, one STATUS line; no ADR text; no code); his
    point 6 (a "blocks sight" flag in 4b?) answered in the chat when step 5 is done; his point 7: the chat's main
    folder up to main (3b949c1) by a fast-forward when step 5 is done. Main also moved under this branch (#73,
    #74: the guard, `toolchain.yml`, `packages/e2e/playwright.config.ts`, docs): merged into this branch before
    the docs pass.

18. **His point 7 DONE:** the chat's main folder (`/home/user/Shamble-Frontlines`) stood on 311c048 (the merge of
    #69) and is on main 3b949c1 by `git merge --ff-only origin/main`. **His point 6 answered in the chat** (no
    "blocks sight" flag in 4b: it comes as one added field with a format version step and Slice 2's sight check;
    4b-2's collision world keeps each collider's shape id; nothing built): his decision SIGHT-FLAG, my
    recommendation (a); engine-architect is asked for its view in its review. The one sentence at two thirds of
    the line was given with it (6,592 of 7,450; nothing left).
19. **The planted defects** (`plants-4b1.py`; its console output `.gate/plants-4b1.out`): first run on fdd9f0a: 23
    of 25 caught by the checks' own words, one WebAssembly-only (the native classes green, the Node compare names
    the class); every file put back; the clean tree green. The two others were caught, not by the words named:
    the script's expectation for one, and the aim tie's second copy of the two numbers for the other (removed:
    commit 230d9e9 "the planted defects, and the aim tie speaks with its own words"). **The whole run is repeated
    on the head the reviewers leave, and its file `plants-4b1.txt` is committed then** (a run with letters
    overwrote the first run's file).
20. **Main merged into the branch** (3b949c1: the guard fix #73 and the CI Builder's #74; no file of 4b-1's
    touched; the Firefox project now also runs the replay spec): head **ef2edec**, pushed.
21. **Review round 1 started on ef2edec** (2026-11-26, 16:47 Amsterdam): rust-reviewer, engine-architect and
    security-auditor, each at max effort on Opus, on `git diff origin/main...HEAD -- crates packages`; the common
    brief is `cs4b-content/review-brief-4b1.md`; their notes `<name>-review-4b1.md`. test-engineer follows.

22. **His VISION-RULE, version 2** (2026-11-26, 15:51:38 UTC; it replaces his block of 15:35:38 by its own
    words): saved by script (`cs4-walk/mustafa-2026-11-26-vision-rule-v2.md`) and entered in `docs/DECISIONS.md`
    with his four other messages of the day to this chat, by `decisions-4b1.mjs` (write at 4dffb47, then check:
    every quoted line byte for byte). docs-keeper has the pointers in `docs/adr/README.md` and the STATUS line.
23. **Review round 1, engine-architect: DONE** (`engine-architect-review-4b1.md`): no Critical or High; all 13
    departures fine; it agrees on SIGHT-FLAG (no flag in 4b; for 4b-2: one collider per shape with the shape's
    index on every leaf, blocking kept out of the tree's structure; maps before the flag mean "every shape blocks
    movement, shots and sight"). Its findings, for the one fix pass after all three reviewers:
    - EA-1 Medium: the scan (`crates/sim/tests/rules_scan.rs`) was to cover `crates/content/src` (hashed maps,
      clocks, unwrap/expect/panic) and does not; CI runs no clippy, so the content crate's deny lints hold only
      by hand. FIX: extend the scan as the sheet's step 2 planned, with planted breaches.
    - EA-2 Medium: `Form::Ramp(Toward)` cannot become the map's TypeScript shape at step 9 (ts-rs renders it as
      an object intersected with a string union). FIX now: `Form::Ramp { rises_toward: Toward }` (no change to
      the file format).
    - EA-3 Low: `dependency_rules.rs` refuses every table but [package] and [lints]; step 9 must give content
      exactly one dev-dependency (ts-rs). FIX: say so in the rule.
    - EA-4 Low: what replaces the two time units at step 6 is written only in the notes. FIX: one sentence in
      `sim/src/time.rs` and a STATUS follow-up.
    - EA-5 Low: `group` on a shape is a required key with no reader. FIX: drop it while no file exists.
    - EA-6 Low: which way `facing_deg` 0 points and which point `at` names are not pinned, and step 8 writes the
      first map. FIX: `at` pinned now (the ground between the feet, the design's own); the facing's zero is
      pinned at step 8 before the first spawn is written (a STATUS follow-up and a line in the map's doc).
    - EA-7 Low: `time.rs` says durations are converted when a file is read; the movement kind keeps seconds.
      FIX: one wording.
    - EA-8 Low: the doc of `Error` says 0:0 means the whole document; it also means "this field, no place".
    - Its notes for 4b-2 (no findings): step 9's TypeScript cap of 8,388,608 needs its own tie; the bundle's
      outer shape has no Rust struct for the type writer; `f64::round` (map.rs, time.rs) enters the players'
      file at step 9, so step 7's libm rule must allow it or the code rounds in whole numbers.

24. **CI on the merged head ef2edec: green** (gate; toolchain 7 min 54 s, main's new workflow: Firefox also runs
    the replay spec, through the new file helper). The toolchain run on fdd9f0a was cancelled by the next push.
25. **Review round 1, security-auditor: DONE** (`security-auditor-review-4b1.md`): no Critical or High;
    dependencies and the browser side PASS (the lockfile and the two manifests are step 1's approved digests; the
    players' file on disk unchanged, 54,514 bytes, sha256 a4d8184c...); departures 3, 6, 11, 12 fine; 9 fine on one
    condition (4c's match setup refuses by name a map with fewer spawns than slots: a STATUS follow-up). Findings:
    - SA-1 Medium: the memory ceiling (input + 4 MiB) holds only for the four limit cases: a decoded string grows
      by doubling and is never shrunk (15 texts of 2^19+1 bytes: 7.9 MB in, 15.7 MB held). FIX: each string gets
      its exact size; that bundle joins the limit cases (native, Node, browsers).
    - SA-2 Medium: three new readers of committed vectors go around the plain read
      (`tests/content_proofs.rs`, `boundary/content.test.mjs`, `rules/content.spec.ts`), and the
      `SHAMBLE_WRITE_VERDICTS` write is a plain `std::fs::write` (a committed link at the verdicts file would
      turn that by-hand run into a write to the link's target). FIX: `read_plain`/`write_plain` and
      `readPlainFile` everywhere; the switch takes only "1".
    - SA-3 Low: Windows device names (con, prn, aux, nul, com0-9, lpt0-9) are valid ids, and an id is a file
      name. FIX: refused in `is_id`.
    - SA-4 Low: the echo leaves a quote raw, so an echoed key can close the sentence's quotes. FIX: the quote as
      `\x22`; `record()` has no cap (about 2 KB at worst): kept for tests and tools, said in its doc.
    - SA-5 Low: the lints hold only by hand (the same as EA-1). FIX: the scan.
    - SA-6 Low: `write_plain` checks no name; `files.rs` words the scan as finding "another way in". FIX: the
      name check, and the honest wording.
    - SA-7 Low: no time asserted in WebAssembly; "four" megabyte classes are three; the finite-range scan covers
      the movement fields only. FIX: all three.
    - What the fuzz cannot reach, for the report: the caps (its inputs are a few KB), full map lists, more than
      one map, long strings; it is blind (no coverage measured), and about a third of its structure-aware inputs
      get no text mutation.
26. **Review round 1, rust-reviewer: DONE** (`rust-reviewer-review-4b1.md`): no Critical or High. Findings:
    - RR-1 Medium: the tie rule wrote the even neighbour without reading it back. At plus and minus 2^-24 (the
      doubles below a power of two lie twice as close) the even text is another double's: the loader refused
      every text of that number and the writer changed it by one step. Its run: 31,750,551 doubles against Node's
      own text, 2 differ. FIX: the even text is taken only if it reads back as the number; every power of two with
      its two neighbours, both signs, joins the corpus.
    - RR-2 Medium: the toolchain job runs no clippy, so the content crate's "never panics" lints hold only by
      hand; and `content` is missing from the job's release line. This chat writes no CI file: both are requests
      for the CI Builder (STATUS, 4B-CI-CONTENT); the source scan (EA-1) holds the part text can show.
    - RR-3 Low: the dependency rule missed dotted keys before the first table and a build script. FIX: both.
    - RR-4 Low: `read_plain` checked the path, then read it again by path. FIX: opened once, the handle's own
      type checked, at most the cap read.
    - RR-5 and nits: a SAFETY note on the counting allocator; three classes, not four; the fold's comment; "the
      slowest by the clock"; `* 1_000_000` can overflow. FIX: all.
    - For the report: a map's canonical layout in the generator goes through the writer itself (the tree corpus
      holds the layout apart from it).
27. **Review round 1's one fix pass: DONE** (commit d36e416, 2026-11-26, 17:38 Amsterdam; the full local checks
    passed whole: 33 boundary tests, 18 browser tests in 207 s; the million-input fuzz run in release; **6,932
    written lines**, 518 under the stop line; the pass took 341): every finding of the three reviewers fixed or
    made a follow-up.
    - The content crate: the tie's read-back (`number.rs`); each string at its exact size (`json.rs`); Windows'
      device names are no ids; the echo escapes a quote; `Form::Ramp { rises_toward }` (EA-2) and no `group`
      (EA-5); the docs EA-7, EA-8.
    - The proofs: the corpus is 20,360 numbers (2,000 ties of 1 to 25 binary places; 6,293 powers of two and
      neighbours); a fifth limit case (fifteen texts of half a megabyte and a byte); **the limit cases are made
      one at a time** (the first run of the new case held two heavy bundles at once and took the test module's
      memory to 72 MiB, over its 64 MiB ceiling: the ceiling is to measure the loader, so the list no longer holds
      them together; 49 MiB again); the generator's valid ids keep off device names by its own rule (**the
      committed seed draws the id `aux` once**: the classes found the new rule on their own); the verdicts file
      written again.
    - The source scan now reads `crates/content/src` too (`rules_scan.rs`: hashed maps, clocks, per-target code,
      and every call that can panic outside tests; 9 planted breaches, 6 controls).
    - The tests' readers: the three vector readers through `read_plain` / `readPlainFile`; the verdicts' write
      through `write_plain`, its switch only "1"; a stated time for the inputs at the caps in WebAssembly (Node:
      5 s release, 60 s dev; measured 218 ms and 3,261 ms; browsers: 60 s for the page's whole run).
    - The plants script: W's text follows the new read; five new plants a to e (the tie's read-back, the string
      size, a device name, the echoed quote, a panic call in content code).
    - Follow-ups, not fixed here: EA-3's step-9 note is a comment in the dependency rule; EA-4 (the simulation
      re-exports the time unit at step 6); EA-6 (facing zero pinned at step 8); departure 9's condition (4c
      refuses a map with fewer spawns than slots); RR-2 (the CI Builder).
28. **The fix pass pushed** (head 495204a: d36e416 and the docs pass's first part; then notes-only commits):
    **CI green on 495204a** (gate run 45860625806; toolchain run 45860625893, 8 min 1 s, Chromium and Firefox).
    The board has the step's line.
29. **test-engineer's read and round 2 started on d36e416** (2026-11-26, 17:39 Amsterdam, after the push; Opus, the highest
    effort; briefs `review-brief-4b1-test-engineer.md` and `review-brief-4b1-round2.md`): test-engineer (the whole
    change-set, its one read), rust-reviewer and security-auditor (what changed); engine-architect took the slot
    rust-reviewer left.
    - **rust-reviewer, round 2: DONE** (`rust-reviewer-review-4b1-round2.md`): no Critical or High; nothing the
      fix pass broke. RR-1 closed (its 31,750,551 doubles again: 0 round-trip failures, 0 differences from V8's
      text; 9,913,103 values in the tie range against ECMAScript's rule worked out apart from the crate: 0
      disagreements). RR-3, RR-4, RR-5 and the nits closed. RR-2 stays open for the CI Builder (no clippy in
      CI). New:
      - R2-RR-1 Medium: **the corpus's ties got thinner, not wider, and the card's two claims about them were
        not true.** `ties()` divides a full mantissa by at most 2^25, so every value lies between 2^27 and 2^52,
        where a tie fits only at about 2 to 4 binary places: the lines where the even rule changes the standard
        library's digits fell from 64 to 44, and none stand at 5 to 25 places. `powersOfTwo()` writes positive
        values only: no negative 2^-24. (The result still stands on the unit rows, the powers of two and its
        sweep.) FIX in the last pass: ties drawn as an odd whole number over 2^q, q from 1 to 25, kept only when
        the engine's own text has q - 1 decimals (a real tie); both signs of every power of two; the script's
        test asserts both.
      - R2-RR-2 Low: the content scan misses the path forms (`Option::unwrap(x)`), `unwrap_err` / `expect_err`
        and the assert macros; a `#[cfg(test)]` on a match arm swallows the rest of its block. FIX in the last
        pass.
      - R2-RR-3 Low: a refused string's length is counted to the next quote, so one throwaway allocation as
        large as the rest of the input is made. FIX: the count stops where the read would refuse.
      - R2-RR-4 Low: `number.rs`'s first lines say "the one text ECMAScript gives" (the standard leaves the
        last digit open). FIX: wording.
      - R2-RR-5 Low: the scan of `files.rs` counts only `File::open(` and `fs::read(`. FIX: the other reads too.
      - Its note: `is_id` also refuses device names for shape, spawn and region ids, which are no file names:
        harmless.
    - **security-auditor, round 2: DONE** (`security-auditor-review-4b1-round2.md`): no Critical or High;
      dependencies PASS (the lockfile and both manifests are step 1's approved digests; the pass adds nothing).
      SA-1 to SA-7 all closed (its fifteen-texts bundle now holds the input plus 4,662 bytes; it was twice the
      input). New:
      - R2-SA-1 Medium: **"held at most the input plus 4 MiB" is still true only for the five limit cases.** A
        list keeps room for four members after its first, so an object of one member holds 288 bytes for 6
        bytes of input: one file of 528 chains of 30 such objects (96,103 bytes) holds 4,619,314 (328,907
        over); a bundle with that file and as many such nodes under an unknown key of its second entry
        (1,413,367 bytes) holds 9,321,393 (3,713,722 over). Bounded all the same (about the input plus 10 MB at
        worst): no denial of service. FIX in the last pass: every list is cut to its length when it closes
        (measured by it in a copy: the input plus 1.10 MB and plus 1.06 MB), and both shapes join the limit
        cases. It must be in before a bundle is read while the game runs (4b-2's Worker).
      - R2-SA-2 Low: the content scan's gaps (the same as R2-RR-2), and its check of the lint guard looks for the
        lints' names, not for "deny". FIX in the last pass.
      - R2-SA-3 Low: `files.rs` says every reader this change-set added goes through the plain read; three
        tests read committed SOURCES with `std::fs` (`recordings.rs`, `dependency_rules.rs`, `rules_scan.rs`); a
        link swapped in after the check is followed (it needs a local writer: outside the threat model, and
        the docs do not say so); the scan counts only `fs::read(`. FIX: the wording and the scan; the race is
        said, not closed.
      - Its notes: departure 9's condition (4c refuses a map with too few spawns) is not yet in STATUS's 4b-1
        block (the docs pass's second part adds it); the CI clippy step should not wait past 4b-2's merge;
        outside this change-set, `rules.test.mjs` and `wasm-rules.spec.ts` (older) still read vectors without
        the helper (a follow-up).
    - **test-engineer's read of the whole: DONE** (`test-engineer-review-4b1.md`): no Critical or High; 3
      Medium, 5 Low, each of which can wait by its own word. What holds, in its words: the generator works out
      its verdicts apart from the loader (records typed by hand, its own layout and echo, the caps as numbers);
      every run checks each record against its class again, so writing the digests again cannot hide a wrong
      record; the aim reach, the map rules, the time unit's digit oracle, the planted links and the scan's
      planted breaches are sound; three more defects planted in a copy were each caught by their own class (the
      node cap one too high, the files cap one too high, a shape 1 mm past the bounds). Findings:
      - TE-1 Medium: the fuzz run's stage shares cannot see the map kind drop out of the run: stage 8 counts the
        renamed file together with the map's own rules (of 200,000 inputs: 3,453 against 425; duplicate id 0,
        the reach 5). FIX in the last pass: the map's own rules are a stage of their own with a minimum.
      - TE-2 Medium: the time limits cannot see a duplicate-key search that takes quadratic time (planted:
        every check stays green; 45 ms becomes 1.82 s under a 20 s limit), so the test list's point on that
        search's cost is not proven and two comments claim more than they hold. FIX: two inputs of one shape
        are compared (distinct keys against one key 16,383 times: 2.4 times clean, 120 times planted).
      - TE-3 Medium: the Node and browser memory ceilings cannot see the string-doubling defect (a module built
        with it ends at the same 49,479,680 bytes: the generator's own inputs set the high-water mark); only
        the native counting allocator does. FIX: said so in both comments and in the report (it would be High
        if the report counted those ceilings as proof of memory per case).
      - TE-4 Low: the time report's rounding is tested at one tie where half-to-even gives the same. FIX: 4.5
        units.
      - TE-5 Low: the links scan (= R2-RR-5, R2-SA-3). TE-6 Low: the content scan's gaps (= R2-RR-2, R2-SA-2).
      - TE-7 Low: one error sentence is pinned; the others only by the digests. NOT fixed here (a table of 45
        sentences; the stop line): a follow-up.
      - TE-8 Low: the million-input run is NOT by hand: CI's toolchain job runs the tools crate's tests in
        release (`toolchain.yml:162`). By hand are only clippy and the content crate's own release tests. The
        brief's item 10 and this card were wrong on that; the report says it right.
    - **engine-architect, round 2: DONE** (`engine-architect-review-4b1-round2.md`): no Critical or High; seven
      of its eight closed (EA-2 by run: ts-rs 12.0.1 on copies of the map's types gives a plain TypeScript
      union now); EA-6 open as planned (the sentence that defers the facing's zero to step 8 is already in
      `map.rs`; what step 8's line must state is in its notes: 0 faces +z and degrees grow toward +x, the old
      game's own convention; how the simulation reads an angle without a std sine is to be chosen before the
      first map file). The raw players' build is byte for byte round 1's. New, all Low: the guard check matched
      lint names, not the deny (= R2-SA-2); the content scan took the simulation's escape for a hashed map; a
      file-level `#![cfg(test)]` is not seen as test code (it fails closed; a note for step 9's plan); a limit
      case and its name were tied by position; the scan's opening comment named only the simulation.
30. **Round 2 is complete: no Critical or High from any of the four.** Within the budget, no third round (a third
    is only for an open Critical or High). **The last fix pass: DONE** (commit 2039e3c,
    2026-11-26, 18:30 Amsterdam; the full local checks passed whole: 196 Rust tests in debug and 195 in release, 33
    boundary tests, 18 browser tests in 208 s, the players' file byte for byte main's; the million-input fuzz run in
    release; **7,164 written lines, 286 under the stop line**; the pass took 232):
    - R2-RR-1: the corpus's 2,000 ties are real ties (an odd whole number over 2^places, kept when the engine's
      own text has one decimal less than the number has binary places; at most 100 for one count of places; 2
      to 25 places, 1,700 of them at 5 or more), and every power of two has both signs: **26,653 numbers**. The
      script's test asserts both.
    - R2-SA-1: every list is cut to its length when it closes; two more limit cases, seven in all (the chains
      file: 96,103 bytes in, 1,197,625 held; the bundle of two such trees: 1,413,367 in, 2,469,864 held; both
      verdicts written from reasoning and right at the first run). A limit case is found by its name.
    - TE-1: the map's own rules are the fuzz run's tenth stage (20 in 10,000; at least 10). The verdicts file
      moved in one line only, the fuzz line that folds the stage: no class's digest moved.
    - TE-2: `the_search_for_a_key_that_is_there_twice_does_not_walk_every_pair` (16,383 different keys against
      one key 16,383 times: 2.3 times in debug, 1.6 in release; refused past 20).
    - TE-3: the comments of the two WebAssembly tests say what their ceilings are: backstops; memory per case
      is proven natively.
    - TE-4: the report's rounding at 4.5 units. TE-5 / R2-RR-5 / R2-SA-3: the links scan counts every way to
      read a file; the plain read's doc says what it does not do. TE-6 / R2-RR-2 / R2-SA-2 and
      engine-architect's Lows: the content scan (calls after a path, `unwrap_err`, the assert macros; a test
      mark counts only on an item; no escape for a hashed map; the guard's seven lints read as denied, by
      tokens; 17 planted breaches, 8 controls). R2-RR-3: a refused string's count stops at the refusal.
      R2-RR-4: `number.rs`'s opening.
    - Follow-ups, not fixed: TE-7 (one sentence per refusal pinned in a table); a file-level `#![cfg(test)]`
      in the scan (step 9); the older vector readers outside this change-set; RR-2's CI steps; EA-6.
    - The plants script: five more (f to j: the list's room, the pairwise key search, the guard allowed, a
      panic call after a path, the map's rules counted away), 34 texts in all, each standing once.
31. **The last fix pass pushed** (head 21e1550; the gate green on it, run 45868324364; the toolchain run
    45868324356 was going when this was written). The board has the step's line.
32. **The planted defects, the whole run on the final code** (`plants-4b1.txt`, on 21e1550; its log
    `.gate/plants-4b1.log`): **35 of 35 caught by the words of their own check, the control held, every file put
    back (sha256 of 13 files), the clean tree green** (the test module's build, the content crate's tests, the
    proofs, the time unit, the aim tie, the recordings, the caps, the two key shapes, the fuzz run, the source
    scan, Node: every exit 0). Commit 029013d.
33. **The verifier and docs-keeper's second part started** (2026-11-26, 18:35 Amsterdam): the verifier on the
    report's draft (`report-draft-4b1.md`, a copy of the session's draft with its open marks; brief
    `verifier-brief-4b1.md`; it runs the full local checks itself, so the main session runs no cargo meanwhile);
    docs-keeper on `docs-keeper-brief-4b1-part2.md`.
34. **CI green on 21e1550, which holds the final code** (gate run 45868324364; toolchain run 45868324356, 7 min
    17 s, Chromium and Firefox).
35. **docs-keeper's second part: DONE** (commit 5662ad2): the docs true for the final code; eight follow-up ids
    from the reviews in the archive's block (and 4B-MEMORY-UNIT after the verifier, added by the main session);
    4B-CI-CONTENT corrected; STATUS from 134 to 124 lines (his limit is 120; the 4 over are other lanes' lines
    about merged pull requests, named in its notes and not reworded: the report's question STATUS-LEN).
36. **Draft pull request #75 opened** (2026-11-26, after the docs pass's second part); STATUS's row names it.
37. **The verifier: the code and every check result hold; FAIL on the report's first draft on wording and counts
    only (one Medium, seven Low); PASS on the corrected text** (`verifier-4b1.md`; its own log
    `.gate/verifier-local-checks.log`). It ran the full local checks (196 and 195 Rust tests, the packages'
    tests, 33 boundary tests, 18 browser tests in 211 s, the players' file's digest), the million-input fuzz run
    (the same histogram, the caps' bytes the same in all seven rows), the corpus (26,653; 1,998 of the 2,000 ties
    real by its own script, the other two plus and minus 2^-24), the time report, the written lines (7,164), his
    words (the script's check; every quote in the report against DECISIONS), five plants against the log, two
    plants of its own (both caught), eleven "fixed" rows in the code. Its corrections, all made: test-engineer
    planted five in a copy and two were NOT caught (its two findings; now plants g and j); "four" new Medium
    are five; the test module's memory is 49.2 million bytes, 46.9 MiB, not "49 MiB" (the comment of
    `content.test.mjs` keeps the slip until 4b-2: 4B-MEMORY-UNIT); words of test-engineer's set in quotation
    marks; a sentence on his VISION-RULE list; his message of 15:51:38 in the list of what binds; twelve
    departures fine and the seventh a finding; one number for 4b-2's projection (8,140); a number with no saved
    log (the 72,482,816 bytes of the limit cases held together: from the failing test's message in the chat).
38. **The report** `docs/reports/2026-11-26-cs4b1-content.md` is written from the corrected draft; this card and
    the notes leave the head in the commit that adds it (CHECKPOINTS.md part 9): they stay readable at the commit
    before it, which the report names.

## Slips of this chat's own rules, for the report
- One git command did not start its own shell call: `git add .checkpoints/cs4b-content/review-brief-4b1.md` stood
  on the line after a `cat` that wrote that file, in one call (2026-11-26, about 16:46 Amsterdam). It staged that
  one notes file; nothing else happened. Every other git command of 4b so far started its own call.
- One chat update was headed "about 16:50 Amsterdam" when the clock read about 16:45: a time ahead of the clock.
- The card's "Last update" was once written 34 minutes ahead of the clock and corrected before it was committed.
- The chat told him "four new Mediums" for round 2 (they are five: the fifth was told as "one limit stays") and
  "49 MiB" for the test module's memory (46.9 MiB), and that test-engineer's own plants were all caught (two of
  five were not). The verifier found all three; corrected in the chat and in the report.
- The chat told him, in its update after round 1's fix pass, that the million-input fuzz run is by hand. It is
  not: CI's release line runs the tools crate's tests (test-engineer's TE-8). Corrected in the chat's update
  after test-engineer's read, and in the report.

## Current step
4b-1, the close: everything but the last two steps is done. Now: the report into the tree with the notes out of
the head (one commit), push, CI on that head; the advisor agent on that head; the pull request's description
with that head's full id; his go. 4b-2 then starts from the notes at the commit before the report's.

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
- No agent runs. The advisor agent starts on the head that holds the report.
