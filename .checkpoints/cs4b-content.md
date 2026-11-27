# Checkpoint: cs4b-content
- **Goal:** change-set 4b, "Content and the world": the content crate (one loader, one canonical writer, all our own
  code by his CONTENT-SOURCE (c)), the built-file check for the fused multiply-add (his MUL-ADD), the integer time
  unit, the collision library and the static collision world, his Testlines values into the content by script, the
  page drawing the scene from the map. A short browser check (Brave, once Firefox) · plan:
  `.checkpoints/cs4-walk/plan.md` (revision 4, approved by the advisor agent; section 5, with 3, 9, 11, 14, 17) ·
  designs: `.checkpoints/cs4-walk/engine-architect-cs4.md` (blocks 1, 3.6, 3.7), `rust-reviewer-cs4.md` (blocks 1,
  2, 4), `testlines-cs4.md` · his words for it: `.checkpoints/cs4-walk/mustafa-2026-11-26-go-merge-70.md`
  (2026-11-26, 12:21:17 UTC), points 3, 4, 5, 7 and 9
- **Branch:** `claude/ecstatic-wozniak-5cbdzs` on GitHub (Builder V6's seat, from 2026-11-27 15:06 Amsterdam: the
  cloud session's stand-in for `cs/4b-content`). Local: `cs/4b-content` in the worktree `.gate/cs4b-2` of the start
  folder `/home/user/Shamble-Frontlines`, made at exactly 7f5ba39 with `git worktree add --detach` and
  `git switch -c`; pushed with `git push origin cs/4b-content:claude/ecstatic-wozniak-5cbdzs`, alone in its command;
  never a plain `git push`. The start folder is never switched (it stays on b9aad01). Builder V4's branch
  `claude/cool-pasteur-oiqk1r` ends at 7f5ba3966def8c36e2c3863c6b2fda7b4b9a9169 and is never pushed to again (his
  Lanes rule: one account per branch) · **Base:** 4b-1's origin/main 2483e9a741c4e552323c36523831a2f35de4002e (the
  merge of 4a, pull request #70); 4b-2 counts against b9aad01954be666340a7ec5c6e9fc9353b11abf1 (the merge of 4b-1)
- **Last update:** 2026-11-27 16:29 Amsterdam (from the clock)

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
  left out (the committed extracts and corpus texts are data). **4b-2's official count (the advisor agent's ruling
  of 2026-11-27, `cs4b-content/advisor-count-rule.md`):** `git diff --numstat b9aad01954be666340a7ec5c6e9fc9353b11abf1
  -- crates packages`, added lines; left out: `crates/devtools/vectors/` but for `.py` files, and what
  `make_recordings` writes into `crates/devtools/recordings/` (`.rec`, `.hashes`, `manifest.txt`, `*.content.json`);
  everything else counts, `packages/types/src/generated/` included (as Builder V4 counted it). New files are staged
  or marked (`git add -N`) first, since numstat does not see untracked files. One count decides; every count line
  also gives the raw total with the recordings in.

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
39. **The advisor agent on 2e282176 (the report's head): APPROVE**, once it had run the gate itself (the gate's
    packages installed by `npm --prefix tools ci`; the smoke on this machine's own Chromium through a scratch
    folder, `.gate/pw-1194`, since its Playwright pins another build). Two Medium, not blocking: M1, a departure
    the report does not list (the approved sheet moves the time unit to 4b-2 at 5,950 written lines; this card's
    own list said 6,600 and never went back to the advisor agent; the count was 6,091 after step 4); M2, his
    sentence at two thirds came late. Both are in pull request #75's description.
40. **His message of 2026-11-26, 18:16:35 UTC** (`cs4-walk/mustafa-2026-11-26-take-in-76.md`): main had moved
    (#76); take it in once; SIGHT-FLAG (a), DUR-1 (C), STATUS-LEN (a), WATCH-75 yes; the advisor agent's four
    conditions for 4b-2 stand; 4b-2 starts right after the merge with no new word from him.
41. **The take-in:** a merge of main 2671595 (6d3acfb), his words into the register by script and his answers as
    follow-ups 4B-SIGHT-MARK and 4B-FIRE-RATE (2542b1a); STATUS at 120 lines by script; of 378 files only the
    four clashing docs files and `docs/REPO_MAP.md` (merged without a clash) are neither as at 2e28217 nor as on
    main. **The advisor agent on 2542b1a: APPROVE.** CI green on it (gate, smoke, toolchain, both runs).
42. **His go** (2026-11-26, 18:52:56 UTC, `cs4-walk/mustafa-2026-11-26-go-merge-75.md`): "you can go merge it".
    **Pull request #75 merged** with 2542b1a0c109e54c514cf63d1c2e2de8932a1a7b as the expected head: merge commit
    **b9aad01954be666340a7ec5c6e9fc9353b11abf1** on main. Main had moved once more seconds before (#77, the PR
    Advisor routine's merge of two docs files of the Guard fix chat's, which this branch never changed): GitHub
    merged on top of it with no clash; main's STATUS is 120 lines. The board's thread is done; the chat's main
    folder is on b9aad01 by a fast-forward.

## 4b-2 (the second pull request): set-up, 2026-11-26, 19:56 Amsterdam
- Branch `cs/4b-world` from main b9aad01, in this worktree; pushed to `claude/cool-pasteur-oiqk1r` (a
  fast-forward). The notes are back byte for byte from 0b0933a and 1f6ce35 (`git archive`).
- **Stop line 8,450 written lines; two thirds is 5,633** (his sentence is sent when the count passes it: the
  advisor agent's condition, and this time at that count, not later).
- **What leaves first: the approved sheet's two rungs, unchanged** (revision 3, section G; the plan's own words in
  `cs4-walk/plan.md` lines 167 to 172): (1) at 6,750 written lines the schema writer and its tests (step 11)
  move to 4d, with the schema halves of PM-0005-7 and PM-0005-8; (2) at 8,000 the page's scene (step 12) moves
  whole, with its browser tests, to the start of 4c, and 4b-2 then changes nothing a player sees and needs no
  browser check. Never moved: the built-file check and the static collision world. The count is taken at each
  step's end; a rung fires when the count at the end of the step before has reached it, and he is told before
  the step moves. **This list goes to the advisor agent with its short re-read before step 6** (its condition).
- The advisor agent's other conditions: no report calls PM-0005-3 passed until he accepts 4B-PM5-3; if a
  movement range refuses a value of his Export at step 8, the range goes to him and his value is never changed.
- His answers that bind 4b-2: FEATURE-COMPARE (a) with four conditions (step 8 of "Steps done"); SIGHT-FLAG (a):
  no "blocks sight" mark in 4b; his Export at 30 steps is in the notes for step 8, with DIFF-LIST asked there.
- **The collision library's batch of step 0 does not fit today's main** (its content and devtools manifests are
  older than what 4b-1 merged): step 6's batch is made again on b9aad01 (root `Cargo.toml`, `crates/sim`,
  `crates/sim-wasm`, `Cargo.lock`), and goes to security-auditor by its digests before it is committed, pushed
  or built (his DEP-HOLD).
- **The advisor agent's short re-read (2026-11-26, 19:58 Amsterdam): steps 6 to 12 APPROVED**, on revision 3 against
  main b9aad01 (`advisor-reread-4b2-brief.md` is what it read). Its conditions:
  1. The remade batch resolves parry3d 0.31.1, `safe_arch` and thiserror 2.0.21 exactly as step 0's batch did
     (`deps-lockfile.diff`): a change to any of the three needs a new yes from him (his FEATURE-COMPARE (a)); any
     of the other 30 packages that moves goes back to rust-reviewer and security-auditor before a commit.
  2. Every changed manifest is in the digests (devtools' too, if `protocol` becomes a dev-dependency).
  3. The two named exceptions as rust-reviewer worded them (`rust-reviewer-step0.md`), each with a fixture, a near
     miss and a planted mistake; any other difference still stops every build.
  4. Name the change-set that first calls a collision query from the match's step: if 4b-2, the recording covers
     those queries (`first-loop` made again after step 10, or a recording of its own), compared natively, in
     Chromium and in Firefox; if 4c, 4b-2's report says so and 4c carries the condition.
  5. The facing's zero (0 faces +z, degrees grow toward +x) is a data convention and the advisor agent's call:
     taken only after it is checked against the old game's code by file and line, before the first map file.
  6. art-director joins graphics-engineer and perf-auditor for step 12 if it is built; netcode-architect for
     steps 9 and 10.
  7. 4b-1's conditions still apply (PM-0005-3; his Export's values; the first commit fixes 4B-MEMORY-UNIT).
- **How a rung fires (its ruling):** the count is taken as in 4b-1 (against b9aad01, `crates/` and `packages/`,
  data files left out: from cut B of step 9 on, the official count of the card's first section, the advisor agent's
  ruling `cs4b-content/advisor-count-rule.md`) at every step's end AND at every commit. A rung fires when the count at the end of the
  step before the rung's step has reached its number; he is told before a step moves; built code never moves.
  **Look ahead before steps 11 and 12:** the count plus the step's projected size (about 520 and 1,060) plus
  one fix pass (about 350): if that reaches 8,450, stop and ask him before the step. Past 8,450 at any commit:
  stop and ask at once. **Past 5,633: his one sentence goes in the very next chat message**, mid-step too. A new
  threshold or order goes back to the advisor agent first.
- **His word is needed:** before step 6 only for a version change (condition 1); before step 8 his yes to
  DIFF-LIST, and his decision if a range refuses a value of his Export (step 8 changes `content/source/`: the
  report's first lines say so); before step 12 his chance to veto GREY-COLOURS (asked in the chat at the
  merge of 4b-1); at the merge his short browser check if step 12 is built here.
- **Step 6, the batch made again on b9aad01 (2026-11-26, 21:41 Amsterdam): OUT OF THE TREE, UNCOMMITTED AND UNBUILT**
  (his DEP-HOLD). Five files under `.gate/cs4b/batch-4b2/`, their text as `deps-4b2.diff`, their digests as
  `deps-4b2.sha256`: root `Cargo.toml`, `crates/sim/Cargo.toml` and `crates/sim-wasm/Cargo.toml` are byte for
  byte step 0's files; `crates/devtools/Cargo.toml` makes `protocol` a dev-dependency (security-auditor's note;
  nothing under `src/` names it); `Cargo.lock` by `cargo update --workspace --offline` from the cached index:
  33 registry packages, **the same 56 packages as step 0's lockfile, every version and checksum the same**
  (parry3d 0.31.1, safe_arch 1.2.0, thiserror 2.0.21: the advisor agent's condition 1 is met; no version for
  him to say yes to again). security-auditor has it (`deps-4b2-brief.md`). To put it in the tree after its
  answer: copy the five files back, `sha256sum -c` against `deps-4b2.sha256`, then commit with the foundation
  check's `sim` half and the feature compare.
- **4b-2's first commit** (a8741fc): the unit in one test comment (4B-MEMORY-UNIT; the boundary test 7 of 7).
- **Main after the merge is green** on b9aad01: gate with smoke (run 45879443239), toolchain (run 45879443258).
- **The facing's zero, checked against the old game** (the advisor agent's condition 5): the old game sets
  `rotation.y = Math.atan2(dir.x, dir.z)` (`legacy/shamble-game.js:871`, `:2710`, `:2758`): 0 faces +z and the
  angle grows toward +x, as engine-architect proposed. To be written into the map's doc at step 8.
- **Step 6, first part** (commit 5cc96c5; 122 written lines): the feature compare's two named exceptions in
  `build-checks.mjs` (rust-reviewer's wording: `NATIVE_TESTS`, `TARGET_ONLY_ENTRIES`, `TEST_ONLY_FEATURES`; two
  versions of a package in a list refused; nothing excused outside the native tests), each with its fixture and
  near misses; the batch's real lists as data (`crates/devtools/vectors/sim-feature-lists.json`: 71, 71, 76,
  resolved in a scratch copy with `cargo tree`, nothing compiled); four planted mistakes by hand, each red.
- **security-auditor on the batch: APPROVE on its five digests** (`security-auditor-deps-4b2.md`; 2026-11-26):
  all 33 packages from crates.io, every registry entry as step 0's, checksums against the cached index and
  archives, none yanked; four build scripts and one proc macro, none reading the network, the clock or
  randomness; `protocol` as a dev-dependency is right; nothing more needed for step 6. Its findings:
  SA-4B2-1 Low: his two conditions "a change of parry3d's version needs a new yes from me" and "the library's
  std stays off" were held by process only (std on moves every list alike). FIXED in step 6: the foundation
  check's `sim` half pins the root's parry3d line and the lockfile's three versions, and its failure names his
  decision. SA-4B2-2 Low, for step 7: the players' imports as an exact allow-list; build-machine paths in the
  file refused. Its notes: parry3d, simba and approx are Apache-2.0 only and parry3d's archive carries no
  licence file: the hosted build must ship the licence texts (a follow-up for lane 2's hosting); **RustSec
  advisories for the 33 packages were not read (no network): to be read before 4b-2 merges**; after the commit,
  the committed files are checked against the five digests.
- **Step 6 DONE** (commit 298837a, pushed; 2026-11-26, 22:14 Amsterdam; the full local checks passed whole: 197 Rust
  tests in debug and 196 in release, client 100, sim-worker 51, 33 boundary tests, 18 browser tests in 208 s, the
  recordings unchanged; **243 written lines of 4b-2**, of 8,450). The committed five files equal
  security-auditor's five digests (checked from the commit). **The players' file is still 54,514 bytes but its
  digest moved** (b4a9774f...ba where main has a4d8184c...): the simulation's new dependencies, of which nothing
  is called yet; for step 7's built-file check and for rust-reviewer. One existing test pinned the list of
  crates with overflow checks on: it is the batch's 35 now. The second part: the batch in the tree (five
  digests OK); `dependency_rules.rs`'s `sim` half (the simulation's dependencies are `content` and `parry3d`
  alone; the library's line and the three versions pinned, the failure naming his FEATURE-COMPARE (a)); the
  simulation's time unit is the content crate's own (`pub use`, 4B-TIME-REEXPORT). Clippy clean with the
  library in the tree.
- **Next: step 7, the built-file check** (the sheet's row 7; security-auditor's SA-4B2-2: the players' imports
  as an exact allow-list, build-machine paths in the file refused; engine-architect: the libm rule must allow
  what `f64::round` needs at step 9, or the code rounds in whole numbers). Then step 8 (his yes to DIFF-LIST
  first), 9, 10, the look ahead before 11 and 12.
- **CI on step 6's head 298837a: green** (gate run 45896663586, with smoke; toolchain run 45896663834, 8 min 7 s
  for its build-and-test step, the first with the library's 33 packages fetched).
- **Step 7, the built-file check: DONE** (2026-11-26, 23:09 Amsterdam; commit d313162, pushed; the gate passes; the full local checks passed whole on the final tree: 202
  Rust tests in debug, twice, and 201 in release, clippy clean, client 115, sim-worker 51, 34 boundary tests, 18
  browser tests in 209 s, the recordings unchanged; **1,339 written lines of 4b-2**, of 8,450). What it holds:
  - `packages/client/scripts/build-checks.mjs`: a reader of WebAssembly files (sections, imports, names, target
    features), an instruction walk of every function body (`wasmCodeOpcodes`: every opcode of WebAssembly 1.0
    and of the eight features the players' file is built with, with what follows each; an opcode or type it
    does not know stops the build; a vector instruction, prefix 0xFD, is named with its body), the name rules
    (`nameSteps`: every `<length><text>` reading of a name, a superset of its path's steps, so a rule can refuse
    too much and never too little; `fusedMultiplyAddProblems`; `bannedMathsProblems`, 66 C names;
    `libmProblems`: any function of the `libm` crate, of the toolchain's `compiler_builtins` under `math` or
    `float`, and any C-named function that is not the bindings' or memory's, outside the allow-list
    `maths-allowed.mjs`, which is empty today), `twinProblems` and `productionProblems` (the two the build
    calls), `PLAYERS_IMPORTS` (the bindings' three, to the letter, hash and all: SA-4B2-2), machine paths refused
    (`/home/`, `/root/`, `/Users/`, `\Users\`, the builder's home, cargo's folder, the repository: SA-4B2-2),
    `walkedStamp` and `stampProblems`. `CARGO_OVERRIDES` holds `ENSURE_NO_PANIC`.
  - `build.mjs`: both twins built (release, dev profile) and read before the bindings tool; the players' file
    read after it; the stamp `packages/sim-worker/pkg/built-file-check.json` (build output, ignored by git); the
    page's step refuses a file the stamp does not name. No switch skips any of it.
  - `crates/sim/src/lib.rs`: `compile_error!` under `fma` or `avx2`. `rules_scan.rs`: that one `cfg` let
    through (a condition whose item is `compile_error!`), the library's name only in `collision.rs` (exact
    path; no `pub use`, no `as`), the fused family by its C names. `tests/native_target.rs` in `sim`, `content`
    and `devtools`; the tools crate's also holds that the repository has no cargo config file (`.cargo/`, beside
    the workspace or a crate) and that no manifest names `rustflags`, `target-cpu` or `target-feature` (the
    test list's line; planted: a config file committed, caught). One boundary test: the stamp names the file
    the Worker loads, and Node's own reading of its imports gives the same three.
  - **Today's files, read by the check:** the release twin 193 functions, all named, 19,504 instructions; the
    dev-profile twin 1,259 functions, 82,159 instructions; the players' file 153 bodies, **18,939
    instructions**, three imports, no vector instruction, no maths routine at all (nothing of the library is
    called yet). Its sha256 is unchanged by this step (b4a9774f...59ba).
  - **Planted in the real source, by hand through `plants-4b2.py` (the whole run is at the close):** 24 defects
    caught, each by its rule's own words, and 3 controls held: a `mul_add` exported from the bindings (named
    `fmaf` in both twins); the same through the vector library from the simulation (release: `libm::math::fma::
    fmaf`; dev: glam's `Vec3::mul_add` too: so release does keep the name, and the dev twin shows more); a
    relaxed-SIMD multiply-add (the instruction and the two features); an import named `fma`; JavaScript's sine
    and the clock imported (the imports rule); `sin()` in the bindings crate (`sinf`); the vector library's
    exponential (`libm::math::expf::expf`) and its `angle_between` (`libm::math::sqrt::sqrtf`); `round()`
    (`C::roundf`); the names stripped (`strip = true`); the path remap dropped (`/root/` found); the library
    named in `world.rs`; three weakened rules against the unit tests; no stamp, a stale stamp, another module
    in the players' place; the native tests under `+fma`, `+avx2`, `target-cpu=haswell`.
  - **Findings of the plants, each a departure from the sheet's row or the test list, for the report:**
    (1) the test list's plant "an arc cosine of the vector library" does not exist: glam 0.33.12's
    `angle_between` runs its own polynomial and brings libm's square root and absolute value only; the plant is
    the exponential. (2) `fma4` is dropped from the native assertion: the pinned compiler tells a stable build
    nothing about it (`-Ctarget-feature=+fma4` left `cfg!` false and warned "unstable feature"), so the assert
    could never fail (the advisor agent's R6 standard). (3) **For step 9 (engine-architect's note):**
    `f32::round` arrives as the C name `roundf` alone, with no `compiler_builtins` path beside it, in both
    twins; so an allow-list entry for `f64::round` would read `C::round`, with its reason, or the code rounds in
    whole numbers: decided at step 9 with rust-reviewer. (4) For step 10: the dev-profile twin shows libm's
    helper modules (`libm::math::support::*`) beside each routine; one entry `libm::math::support` would cover
    them. (5) Two functions more than the sheet names are exported for the build (`twinProblems`,
    `productionProblems`), and `nameSteps`, `stampProblems` and four lists for the tests.
- **Written lines after step 7: 1,339 of 8,450** (step 7 took 1,096: the checker 453, its tests 395, the scan
  166, the native tests 120, the build 54, the rest 8; the sheet's count for it was 890 by the designs' way, so
  1.23 times). Two thirds of the stop line is 5,633; the rungs stand at 6,750 and 8,000. Ahead by the designs'
  count: steps 8 to 12 are about 2,645; at this pull request's rate so far (1.34) the whole would end near
  4,900, at the planned 1.8 near 6,100, at 4b-1's 2.17 near 7,100: under the stop line at all three, the first
  rung reached only at the highest.
- **He sent "a" once; the chat was shown it three times** (`cs4-walk/redelivered-2026-11-26.md`). The showings
  of 22:00:07 and 22:43:17 UTC are the record's own repeats of his message of 21:16:26 (the same source id, each
  right after the session's worker restarted), not messages of his. The first repeat was wrongly saved as "his
  second message" (commit 0b068ee; the note is removed) and told to him as such; the second came after
  DIFF-LIST was asked and **was not taken as its answer: DIFF-LIST is open**. `chat-words.mjs` leaves repeats
  out from now on. Told to him.
- **DIFF-LIST printed from his Export at 30 steps** (`cs4b-content/testlines-difflist-30.md`, by the notes'
  two scripts, which now take an Export's path; the first list is reproduced byte for byte by the same
  scripts). Against the list in the plan it differs in two rows of part B and nothing else: the step rate's row
  is gone (the Export is at 30), and `soldier.gun` "smg" to "shotgun" is new (the gun in his hands when he
  exported: not carried). Parts A (23 names) and C (7 and 16 names) are the same. **Asked of him in the update
  at step 7's end; step 8 writes no content file before his yes.**
- **CI on step 7's head d313162: green** (gate run 45902998609, with smoke; toolchain run 45902998524, 8 min 23 s
  for its build-and-test step: 16 s more than step 6's, the dev-profile twin's build). So the built-file check
  passed on CI's own build, in both flavours, with CI's paths.
- **Step 8, first part: the tools, on test files: DONE** (commit 54e89d2, pushed 2026-11-26, 23:46 Amsterdam;
  the gate passes; the full local checks passed whole: 206 Rust tests twice, 205 in release, client 128,
  sim-worker 51, 34 boundary tests, 18 browser tests in 204 s, recordings unchanged, the players' file
  unchanged; **2,592 written lines of 4b-2**, of 8,450: this part took 1,253 where the sheet's count for the
  whole step is 545, mostly tests (the walk's 346, the hand-over's 266). CI's gate is green on it; its toolchain
  run was still going at 23:52.) No file under `content/` is
  written before his DIFF-LIST answer (said to him so), the map included; the drafts wait in the notes.
  - `crates/devtools/src/content_files.rs` and `src/bin/content_bundle.rs`: the walk of `content/` (one folder
    per kind, the content crate's own list, now public; `source/` skipped but held to be a real folder; every
    entry asked for its own type, never followed; a link in a file's or a folder's place, a pipe, a nested
    folder, an unknown folder, a file at the top, a name that is no `<id>.json`, a file over the loader's cap or
    no text: each refused before it is read), the bundle after the loader took each file and the whole, and the
    tool's two modes (`--check` writes nothing; `<folder>` puts `content.json` there: the folder must be a plain
    folder whose real place lies outside `content/`). `files.rs` gains `read_plain_within` (a reader's own cap),
    `is_plain_folder` and `replace_plain` (a new file made only under a name that is not there, then renamed).
    Four tests on scratch folders (`tests/content_files.rs`): 12 refused layouts and 6 refused texts, each with
    the tree's entries, bytes and times unchanged, and both modes saying the same.
  - `packages/client/scripts/testlines.mjs` (`--check`, `write`, `compare-old`, `table`) with ten tests on a
    made-up Export: the Export's sha256 pinned in the script (a9d47785...c4ab) and held against the manifest; an
    Export at another step rate than 30 or with a warning is not handed over; the table names fields, never
    files (a row with a path is refused; plants `../.claude/settings.json`, an absolute path,
    `content/source/x.json`); numbers compared bit for bit; no read or write through a link; reads capped at 1
    MiB; the target written new and renamed. `canonical.mjs` (three tests) brings a hand-written file to the
    loader's form and takes a content path only.
  - `crates/content/src/map.rs`: **the facing's zero is pinned in the map's doc** (0 faces +z, the degrees grow
    toward +x; the old game's `atan2(dir.x, dir.z)`), before any map file exists (the advisor agent's condition
    5). How the simulation reads an angle is said there for the change that first reads one (no sine or cosine
    of the standard library).
  - **Found on the way:** (1) the tool's first inside-content check was fooled by a path spelled with `..`; its
    own test caught it; the real places are compared now. (2) 4b-1's scan of the recordings' readers went red on
    the helper's new shape (the open moved into `read_plain_within`), as it should: the scan now names that
    door, and holds that the one place that makes a file makes it new. (3) The product script's comparison
    gives the notes' counts on his Export (23 different, 60 equal, none without a counterpart).
  - **Dry runs, printing and scratch only (`.gate/cs4b/draft/`, never `content/`):** the movement table his
    Export would give is taken by the loader as it stands, so **no range of the movement kind refuses a value of
    his** (the advisor agent's condition 7); the test scene's draft (`cs4b-content/test-scene-draft.json`: 23
    shapes, 8 spawns, 6 regions; the Art sheet's house sizes, the marks' steps, ledges, thin wall and ramps, a
    50 m lane with posts) is taken too; the bundle of both is 8,375 bytes. The table's draft is
    `cs4b-content/testlines-table-draft.json` (16 rows: 15 fields and "stamina is on").
  - Ten more plants in `plants-4b2.py` (T1 to T10), tried by hand, all caught.
- **Ahead, by the count, after step 10's core (3,999):** by the designs' way the work done is about 1,890 (steps
  6 and 7, about 450 of step 8, about 440 of step 10), so the rate is 2.1. What is left by the designs' way is
  about 1,760 (step 8's second part 100, step 9 830, step 10's wiring 100, step 11 240, step 12 490). At 2.1
  the whole would end near 7,700: **past the first rung (6,750: the schema writer, step 11, moves to 4d) and
  close to the second (8,000: the page's scene, step 12, moves to the start of 4c)**; with both moved it ends
  near 6,200. Two thirds of the stop line (5,633) is not reached. Told to him in the update after this step.
- **The count before step 10 (kept for the record):** steps 6, 7 and 8's first part are 2,592 written lines for about 1,545 by the designs'
  count (1.68 times). What is left (step 8's second part about 100, steps 9 to 12 about 2,100): at 1.68 the
  whole ends near 6,300; at 4b-1's 2.17 near 7,400, which passes the first rung (6,750: the schema writer moves
  to 4d) and stays under the second (8,000) and the stop line. Two thirds (5,633) is not reached.
- **While DIFF-LIST is open (asked 23:11; no answer of his yet at 23:52):**
  - **Step 9's one manifest line is with security-auditor** (his DEP-HOLD: nothing of it is in the tree):
    `crates/content/Cargo.toml` gains `[dev-dependencies] ts-rs = { workspace = true }` and `Cargo.lock`'s
    `content` entry gains that one dependency ("Locking 0 packages"). Outside the tree:
    `.gate/cs4b/batch-step9/`; digests `cs4b-content/deps-step9.sha256` (Cargo.lock 56bd2d52...9c47,
    crates/content/Cargo.toml 620a400b...8b9c); diff `deps-step9.diff`; brief `deps-step9-brief.md`. Resolved
    only (`cargo tree` in a scratch copy): the feature lists stay 71, 71 and 76 with no difference.
  - **security-auditor's answer on step 9's batch: APPROVE** (2026-11-27, 00:06) on the two digests,
    Cargo.lock 56bd2d52483a46eb0843b465e21a8d3492e3df1b9263b46bf9c7cdb61f3b9c47 and crates/content/Cargo.toml
    620a400b14da6030dce2fda1f336358f3edc13cc0d92f802bdfd260416738b9c
    (`security-auditor-deps-step9.md`). No Critical, High or Medium. It found: the lockfile's change is the one
    entry and is needed (`--locked` refuses the head's lockfile with the new manifest); no build players get
    holds the type writer (five builds' feature trees identical, head against batch; the lists stay 71, 71 and
    76); the second exception's basis is unchanged; the derive puts no code into a non-test build; no second
    batch is foreseen. **Its conditions for the commit:** the two files go in only together with the dependency
    rule's edit, then the committed files are checked against both digests. Two Lows for that commit:
    SA-STEP9-1 (the rule's refusal test still compares with `["package", "lints"]` and one of its plants is the
    line that becomes allowed: compare with the new list, plant a second dev-dependency and a ts-rs line with a
    feature); SA-STEP9-2 (the comment at `build-checks.mjs:285-288` about "the step that first nests generated
    types": say that step 9 settles it with `cfg(test)` and no feature). Not checked by it: nothing compiled;
    the advisories (asked of it next, as a question of its own).
  - **Step 10's design read is with rust-reviewer** (`rust-reviewer-step10-brief.md`), before any of it is
    built. What I read in parry3d 0.31.1: `Bvh::cast_ray` keeps the first-visited leaf of two at the same
    distance (`find_best`, `primitive_score < best_cost`), so the visiting order would decide a tie where the
    design says the lower id wins; I proposed our own depth-first walk with our own slab test and the tie by
    index. Also asked: which `libm` routines the tree's build links (for `maths-allowed.mjs`), and how to hold
    the depth of 31.
  - **rust-reviewer's design read for step 10: APPROVE the walk of our own, with our own leaf tests, under
    four conditions; the maths allow-list stays empty at step 10** (2026-11-27, 00:24;
    `rust-reviewer-step10.md`; its probe crate, with the root's parry3d line word for word and every locked
    version, built for wasm32 in release and dev and run natively and in Node, is under
    `.gate/rust-reviewer-step10-scratch/`).
    - **Why not the library's ray search (measured):** it prunes a node whose score equals the best, so a tied
      leaf is never tested: with two boxes at one distance it returned the higher index in 10,889 of 20,000
      maps. It also loses hits: a zero direction component becomes 8,388,608 instead of infinity, so a ray
      lying in a box's face plane fails the node test (a downward ray at x = 1.000 over the box [0, 1]: no hit,
      where the library's own box test and our walk give 654.0); over 60,000 random rays it was wrong 5 times.
      Aim points are whole centimetres and faces whole millimetres, so that is a real case for PM-0004-54.
    - **Its conditions:** (a) one slab function for nodes and for box leaves, on the leaf's stored corners:
      subtract, then divide; a zero component is an inclusive range check; t starts at +0.0; comparisons
      written out (the scan bans `min` and `max`). (b) A node is pruned only when its entry is greater than the
      best. (c) The hit kept is the lowest (distance, index). (d) A ramp's distance is clamped to at least its
      own box's entry.
    - **Why it is sound:** node entry <= child entry <= leaf distance (correctly rounded subtraction and
      division are monotonic; a parent box is the exact min and max of its children), so no ancestor of the
      answer is pruned whatever order the walk takes. Its probe: the lower index in 20,000 of 20,000 ties; no
      hit of the library's missed in 60,000 rays; 739 rays differ by one unit in the last place (the library
      multiplies by 1/dir); 10,000 rays native against release wasm, 0 differences.
    - **Leaf tests ours, not the library's:** its box test works in a centred frame (the centre and the half
      size round), multiplies by 1/dir, and its normal on a tie links `sqrtf`; its ramp test is an iteration
      with tolerances (on a face a box could share, its distance equalled the box's in 1,228 of 1,881 rays).
      The two descriptions of a ramp (ours for rays, the library's polyhedron for 4c's sweeps) are built from
      the same six f32 corners, and our distance is tested against the library's within 1e-4 m (measured gap at
      most 5.05e-5 m).
    - **The tree:** `from_leaves` (Binned), `traverse`, `mins`, `maxs`, `leaf_data`, `is_leaf` and
      `subtree_depth` link no `libm` routine in release or dev (checked with the project's own rule and an
      empty list). The build reads only the boxes' bits, their order and their ids: no hasher, no address, no
      thread count. Tree dumps were equal natively, in release wasm and in dev wasm for 3, 17 and 512 boxes.
      Narrowing: `as` once per corner in `StaticWorld::build`; those f32 corners are stored and used for the
      tree and the leaf tests alike; -0.0 becomes +0.0 there (a file cannot hold it, a document built in code
      can). Every millimetre within 655.35 m narrows to a distinct f32 in order.
    - **Depth:** `subtree_depth(0)` (public, exact; a leaf counts 1). **A legal map can exceed 31:** 34
      one-centimetre cubes in a geometric chain give depth 34 (its file `deep-core1-group1.txt` in the probe
      folder, in millimetres): the refusal is real and has a ready test map. At depth 31 or less the
      library's walk stacks stay inline (30 of 32 slots); nothing it built allocated.
    - **One Low:** the example entry in `maths-allowed.mjs`'s comment (`libm::math::sqrtf`) would not cover the
      real path (`libm::math::sqrt::sqrtf`): the comment is corrected with step 10's commit.
    - Not checked by it: Chromium (Node only), AArch64, 4c's capsule paths, the speed of the walk.
  - **A change of order, for the advisor agent's next read:** the part of step 10 that needs no content file
    (the static world from a map document, its tree's bytes, the ray cast, on maps made in tests) is built
    before step 9, because step 8's second part and step 9 both rest on his DIFF-LIST answer (the content files,
    the frozen content of the first recording). No step's content changes; steps 11 and 12 stay last.
- **CI on step 8's first part, 54e89d2: green** (gate run 45906827061 with smoke; toolchain run 45906826997, 8 min
  49 s for its build-and-test step).
- **The advisories are read** (the to-do from security-auditor's note on step 6's batch; 2026-11-26, 23:55):
  one request to the OSV database's batch query with the names and versions of the lockfile's 49 registry
  packages. **48 have none. `heapless` 0.8.0 has nine** (RUSTSEC-2026-0574 to -0582, published 2026-11-18, all
  "unsound", fixed in 0.9.4). The whole reading is `cs4b-content/advisories-2026-11-26.md`. In short: the
  package comes with parry3d's `alloc` feature through `rstar` 0.13.1 (which asks for heapless "0.8", so it
  cannot move alone); parry3d names `rstar` in one file that it compiles only with its `spade` feature, which
  our line leaves off (`src/transformation/mod.rs:48-49`); so both packages are compiled and nothing reaches
  them. **Made a rule of the build:** `UNLINKED_CRATES` (`heapless`, `rstar`) in `build-checks.mjs`: a function
  of either in a twin stops the build (one more test, one more plant, both twins pass today). **Commit 2f058b2,
  pushed 00:04** (the gate passes; the full local checks passed whole: 206 Rust tests twice, 205 in release,
  client 129, 34 boundary tests, 18 browser tests in 206 s; **2,627 written lines of 4b-2**). Then to
  security-auditor for its judgement, and to him in the update and the report (a package inside
  the batch he said yes to; no version of the three he pinned is touched).
- **security-auditor's judgement on the advisories: Low; keep the pinned versions; the build rule is the right
  control; no decision of his is needed, and he is told** (2026-11-27, 00:16;
  `security-auditor-advisories.md`). It read rustsec.org itself (the page for -0577 matches the saved record; a
  tenth, older advisory on the package, RUSTSEC-2020-0145, is fixed from 0.6.1 on). Its reading: heapless is
  used only by rstar and rstar only by parry3d; parry3d names rstar in two lines of the one file that needs
  `spade`; cargo resolves the same features on wasm32, x86-64 and AArch64 Linux with test dependencies in (no
  `spade`, no `std`), so a native server is in the same position; of the nine only `vec::IntoIter::drop` lies
  on rstar's own path, and its defect needs an element whose clean-up panics, which rstar's has not; both twins
  hold only names of the newer scheme and none of either crate. **Its two gaps, both Low, both closed in the
  commit e3acb41 (pushed 2026-11-27, about 00:23; the gate passes; the full local checks passed whole; **2,695
  written lines of 4b-2**):** (a) a name in the older scheme would hide a trait's method from the rule: a twin that
  holds any such name is refused now; (b) the native builds are never read: `dependency_rules.rs` now holds
  that only the simulation's manifest names the library, that `Cargo.lock` holds no `spade`, and rstar 0.13.1
  and heapless 0.8.0 by version, with a failure that says to read the advisories again. Two more plants (U2,
  U3), both caught. **For STATUS at the docs pass:** take heapless 0.9.4 or later when rstar or parry3d next
  move; the nine ids as named ignores, with this reason, once cargo-deny or cargo-audit runs in CI.
- **HEAPLESS-ADVISORY, as asked of him in the chat (00:05): withdrawn as a decision**, on security-auditor's
  judgement; it stays in the update and the report as a thing he is told.
- **Step 10, the core (the static world on maps made in tests): DONE** (commit 8e5a268, pushed 2026-11-27,
  00:42; the gate passes; the full local checks passed whole: 217 Rust tests twice, 216 in release, client 129,
  sim-worker 51, 36 boundary tests, 19 browser tests in 205 s, the recordings unchanged, the players' file
  unchanged; **3,999 written lines of 4b-2**, of 8,450: this part took 1,304, of which `collision.rs` is 845
  with its eight tests; the sheet's count for the whole step is 540). The browser's proof: Chromium gives the
  native lines, in 25 ms. Built before step 9, as said above; its wiring into the match (`LocalMatch`,
  `CreateError::World`, the map by its id from the content set) follows step 9.
  - `crates/sim/src/collision.rs` (the one file that names the library; the scan passes with it there):
    `StaticWorld::build(&MapDocument)` (at most 512 shapes; shapes by id, so a shape's index is its rank; each
    corner narrowed to `f32` once, a zero without its sign; a corner that is not finite, a shape without
    thickness and a tree deeper than 31 refused by name; the library's binned tree over the stored corners;
    `subtree_depth(0)`), `solids()`, `tree_depth()`, `tree_bytes()` (29 bytes a node, in the library's walk
    order), and `cast_ray` under rust-reviewer's four conditions: one function (`enter_box`) for a node and for
    a box, a subtraction and a division per axis on the stored corners, a zero direction as an inclusive range
    check; a node pruned only when it is entered later than the best hit; the lower index of two hits at one
    distance; a ramp (`enter_ramp`) as its box and one plane, never met before its box. No library arithmetic
    in a ray. `crates/sim/src/vec.rs`: `V3`.
  - **Tests in `collision.rs` (8):** the shapes' order (three document orders, the same bytes); every leaf's
    box is its shape's own corners; no shape, one, two; a minus zero and a number too small for `f32`; the
    refusals, each by name, with **rust-reviewer's chain of one-centimetre cubes: 31 are taken at depth 31, 32
    are refused, all 34 are refused**; rays (down, level, slanted, from inside, in a face's plane, on an edge,
    the tie on a shared plane, the limit to the last bit, numbers that are none); a ramp toward each of the
    four sides; and our ramp against the library's own wedge from the same six corners: within a tenth of a
    millimetre on more than 3,000 rays.
  - **No memory in a ray:** `tests/state_hash_alloc.rs` counts 28,000 rays into the deepest tree a world may
    have (31) and the fullest world (512): 0 allocations.
  - **The same bits on every target** (`devtools::world_proofs`, `vectors/static-world.txt`, made natively): 12
    seeded maps (0, 1, 2, 3, 17, 64, 200, 512, 512, 300, 100 and 31 shapes; boxes and ramps on the millimetre
    grid, a third of them sharing a face's plane) and 2,000 rays each (a fifth straight down a shape's own
    face plane); per map the tree's depth, a digest of its bytes and a digest of every ray's shape and
    distance bits. **Equal natively in debug and in release, and in WebAssembly under Node in the release and
    the dev profile**; the browsers' spec is in the local checks' run. The check bites (one digit changed: the
    map is named).
  - **What the library links, read with the built-file check's own rules on the test-only module** (the
    players' file does not hold the world until its wiring): release 24 functions of parry3d, dev 470; **no
    routine of the libm crate, no function of heapless or rstar, no vector instruction, no name of the older
    scheme**, in either profile. So the maths allow-list stays empty for the world, as rust-reviewer read it.
    For step 9, seen in that module: content's `f64::round` arrives as `C::round`, and a 128-bit multiply as
    `C::__multi3` (an integer helper of the toolchain's): each will need its entry with its reason, or the
    code changes, when content is first linked into the players' file.
  - **Plants (W1 to W9 without W2), all caught:** the tie rule dropped; a touching ray missing; the minus zero
    kept; the document's order kept; the depth cap raised; a ramp high on the wrong side; a ramp without its
    slope; one ray's distance moved by one unit in the last place (the committed lines differ). **W2 withdrawn
    as no defect:** with the zero-direction branch taken out, the division by zero gives the same answers
    (infinities, and a NaN every comparison lets through); the branch stays because it says the rule in words.
- **Step 9, the pieces that need no content file** (built while DIFF-LIST is open; 2026-11-27):
  - **First piece DONE** (commit c09f459, pushed 00:55; the gate passes; the full local checks passed whole;
    **4,047 written lines of 4b-2**): the content crate's one dev-dependency, the type writer, exactly as
    security-auditor approved it (both committed files equal the two digests, checked after the copy and before
    the commit); `dependency_rules.rs` holds the three tables and the one line, with four planted manifests
    (SA-STEP9-1); the comment on the "known clash" in `build-checks.mjs` (SA-STEP9-2).
  - **Second piece DONE** (commit 6290c85, pushed 2026-11-27, 01:03; the gate passes; the full local checks
    passed whole: 218 Rust tests twice with the generated types written again and unchanged, 217 in release,
    client 129, sim-worker 51, types 8, 36 boundary tests, the type checks, 19 browser tests; **4,350 written
    lines of 4b-2**, of 8,450): `crates/content/src/ts_export.rs` (the protocol
    crate's export test, copied, with the content crate's folder: the two crates share no code); the derives
    under `cfg(test)` on `Movement`, `MapDocument`, `Bounds`, `Spawn`, `Shape`, `Form`, `Toward`, `Region`,
    `Ground`; eight generated files under `packages/types/src/generated/content/` (a shape is its corners and
    `{ "type": "box" } | { "type": "ramp", rises_toward }`, the file's own keys; `Toward` is "+x" | "-x" |
    "+z" | "-z"); their exports in `packages/types/src/index.ts` and a type test (three lines that must not
    compile); and the source scan now takes a file that starts with `#![cfg(test)]` as test-only as a whole
    (4B-SCAN-CFG-TEST: three bites, one control).
  - **What of step 9 still waits for his DIFF-LIST answer:** the constructor taking the bundle, the page's
    fetch of `content.json`, the build running the bundle tool, the first recording's frozen content.
- **Step 10, one more piece that needs no content file: the maps the loader takes, through the world's builder.
  DONE (commit ad88341, pushed 2026-11-27 between 01:32 and 01:34 by the clock; the gate passes; the full local checks passed whole: 219 Rust
  tests twice, 218 in release, client 129, sim-worker 51, types 8, 36 boundary tests, the type checks, 19 browser
  tests in 3.5 min; the proof in Chromium takes 251 ms now; 4,769 written lines of 4b-2, of 8,450). The
  container was restarted in the middle of the first run of the checks (the tree and the tools under `.gate/`
  were kept; the patch saved in the notes was not needed and is removed); the second run passed whole.** The sheet's row 10 names it ("the fuzz extended to `ContentSet -> StaticWorld::build`",
  "hand-made hostile documents"); it was listed under the wiring, and needs no content file.
  - `crates/devtools/src/world_proofs.rs`: `judge_taken(&MapDocument) -> Result<bool, String>` (the builder's
    one refusal of a map that keeps the map kind's rules is a tree too deep; a built world holds the map's
    shapes by their ids' order, each by its own corners; built twice it is the same tree; a ray straight down
    onto the middle of every shape gives what a search of every shape says: to the last bit and the lowest index
    when only boxes hold the point), `fuzz_worlds(path, bytes)` (every map of an input the loader takes, through
    the judge), `HOSTILE` and `hostile(at)` (nine maps, each written as a file by the generator's writer, read
    by the loader, then built: 512 times the same box; 512 boxes inside one another; 512 slabs a centimetre
    thin and a map long; 512 centimetre cubes in the far corner; two piles a map apart; a stair of 512 ramps; a
    row of 512 boxes of one height, each over every other's middle, the lowest index in the middle of the row;
    rust-reviewer's chain of 34 cubes, refused at 34 deep; its first 31, built at depth 31). `made()` holds the
    judge on the 12 seeded maps too and writes one line per hostile map: `vectors/static-world.txt` has 22 lines
    (its first: `shamble-static-world 1 12 2000 9`; the 12 seeded lines are unchanged, bit for bit).
  - `content_proofs.rs`: `hold` and the new `map_file` are `pub(crate)`; `verdicts()` (the cross-target run of
    the fuzz run's first 2,000 inputs) calls `fuzz_worlds` too, so WebAssembly under Node and the browsers hold
    the same promises; the committed verdict digests are unchanged.
  - `tests/content_fuzz.rs`: every taken input's maps go through `fuzz_worlds` inside the same `catch_unwind`;
    the run prints and holds the worlds built (2,387 in the 50,000 inputs of a debug run: 477 in 10,000; at
    least 250; none too deep: mutation does not reach one, the chain does).
  - `tests/static_world.rs`: the hostile lines' form, the chain's refusal by its sentence, a third test (every
    hostile map through the judge; the list's end; a shape without thickness is named as the builder's own
    refusal). `static-world.test.mjs` (23 split lines, 9 hostile) and `static-world.spec.ts` (maps 12, hostile 9).
  - **A finding on the way:** the same box 512 times does NOT show a broken rule for two shapes at one distance
    (the tree's walk happens to start with the lowest index there): two planted defects passed the judge on it.
    The row of 512 boxes was added for that; with it both are caught.
  - Plants W10 to W13 in `plants-4b2.py` (the tie rule through the judge; a node pruned at the best hit's own
    distance; the builder leaving a shape out; the fuzz run sending no map on): run by hand, all four caught.
- **Step 9, third piece that needs no content file: the page folder read through no link, and the scan of the
  sources for `innerHTML` and its kin (the sheet's row 9; security-auditor's points on `filesOf` and the scan;
  2026-11-27). DONE (commit 5c6738a, pushed 01:48; the gate passes; the full local checks passed whole: 219 Rust
  tests twice, 218 in release, client 136, sim-worker 51, types 8, 36 boundary tests, the type checks, 19 browser
  tests in 3.4 min; the players' file unchanged, sha256 b4a9774f...59ba; **4,956 written lines of 4b-2**, of
  8,450). A first run of the checks failed, rightly: the edit had dropped an import (`readdirSync`) that the
  build of the players' file still uses; fixed, and the checks run again whole.**
  - `packages/client/scripts/build-checks.mjs`: `plainFilesOf(folder)` (every file of a folder in a fixed order;
    it throws at a link, a pipe or a device, by its path), `MARKUP_SINKS` (12 words: innerHTML, outerHTML,
    insertAdjacentHTML, createContextualFragment, document.write, DOMParser, setHTML, srcdoc, a javascript:
    address, eval, new Function, a dynamic import whose address is not a plain text in quotes),
    `markupSinkProblems(sources)`, `SCANNED_SOURCES` (`packages/client/src`, `packages/sim-worker/src`),
    `sourceTexts(root, folders)`.
  - `build.mjs`: `buildPage` refuses the sources before anything is built ("the page's sources are refused"),
    and lists the built page with `plainFilesOf` (its own `filesOf` is gone).
  - `build-checks.test.mjs`: 7 more tests (57 in the file): the folder reader (order, a link to a file, a deep
    link, a link to a folder, a missing folder), the scan (the 43 real sources are clean; text as text and a
    written-out import pass; 19 planted lines, every word of the list among them; two words give two lines;
    a scan of nothing is refused).
  - Plants P15 to P18 and the control P19 in `plants-4b2.py`, run by hand: the page's entry setting innerHTML and
    the Worker using `new Function` stop the production build by the rule's own words; the reader taking a link
    and the list losing innerHTML fail their tests by name; the clean production page builds.
  - What waits for the content files: a page folder without `content.json` refused, and a `content.json` over
    8 MiB refused (the same row).
- **CI, read 2026-11-27, 01:04: green on every code head up to c09f459** (gate with smoke, and toolchain): 2f058b2
  (runs 45908649135, 45908649409), e3acb41 (45910750962, 45910750974), 8e5a268 (45912515590, 45912515341: the
  first run with the static world's browser spec; the whole toolchain job took 11 min 29 s, so the browser
  tests stay under CI's 5-minute stop), c09f459 (45913683931, 45913683934). Read again at 01:34: 6290c85 green too
  (gate 45914579327, toolchain 45914579307, 9 min 6 s). ad88341: gate green (45917465677), toolchain still
  going at 01:40 (45917465670); green when read again (9 min 29 s). 5c6738a, read at 02:02: gate green
  (45918878271), toolchain green (45918878273).
- **CI's toolchain job is nearing its time limit (a finding for him and the CI Builder; no file of mine):** the
  job may run 15 minutes (`.github/workflows/toolchain.yml:38`, `timeout-minutes: 15`). On 5c6738a it took 13
  min 1 s (its build-and-test step 12 min 20 s); on the four heads before it 9 min 6 s to 11 min 29 s. The
  piece of 5c6738a adds seven small tests and a scan of 43 files, so most of the difference is the runner, but
  step 9's rest adds a tool run, a bundle and more tests. A run that passes 15 minutes fails for no defect. The
  limit is a CI file: the CI Builder's. Asked of him as CI-TIME in the chat's update of 02:02.
- **Ahead, by the count, at 4,956 (2026-11-27, 01:48):** left by the designs' way of counting about 1,490 (step
  8's second part 100, step 9's rest about 600, step 10's wiring about 60, step 11 240, step 12 490). At the
  rate so far (about 2.15) the count at step 10's end is near 6,600: under the first rung (6,750), so step 11 is
  built (look ahead: 6,600 + 520 + 350 = 7,470). At step 11's end near 7,100: under the second rung (8,000),
  but the look ahead before step 12 gives 7,100 + 1,060 + 350 = 8,510, past 8,450: **by today's numbers the
  page's scene (step 12) meets the stop-and-ask before it is built**. Two thirds (5,633) is not reached. Told to
  him in the update after this piece. Step 11 is not built ahead of its place: it is the first to leave when
  the size presses, and building it now would take that relief away and put the scene, which he sees, at risk
  instead.
- **To do before 4b-2 merges:** the licence texts for the hosted build (a follow-up for lane 2).
- **GREY-COLOURS: (a), the default greys stand** (his message of 2026-11-26, 21:16:26 UTC, one letter: "a",
  `cs4-walk/mustafa-2026-11-26-grey-colours.md`; read as his answer to the one question the chat's update before
  it asked, whose options were (a) the default greys stand and (b) he gives other colours before step 12). No
  veto: step 12 draws the scene in LOOP_V2 section 9's default greys. For DECISIONS at the docs pass.
- **The seat moved to Builder V6** (his second personal account; session_01WLuqnt1v3UNwSqbJsJre7u; Opus 5.5 at
  effort max, the advisor tool on Fable 5.2, by the session's record), on his message of 2026-11-27, 14:45
  Amsterdam (received 13:57:21 UTC; `cs4-walk/mustafa-2026-11-27-builder-v6.md`, saved by script). Builder V4's
  chat is out of usage until Tuesday and pushes nothing more.
  - **The guard check** (his item 3: `git merge --ff-only HEAD~1` in the start folder): the first run, 13:57:52
    UTC, went through: the guard loaded and asked (the record's hook entry: `permissionDecision` "ask" with the
    main-folder reason), and 1.8 s later the record shows the ask accepted as a one-time yes from the user's side
    (`"source":"user_temporary","grantScope":"once"`; the other calls read "classifier" or "rule"); git said
    "Already up to date.", nothing moved. The chat stopped and asked (GUARD-RETRY). **His answer: (a)**
    (`cs4-walk/mustafa-2026-11-27-guard-retry.md`); the second run asked at 14:02:31 UTC and he denied it at
    14:02:44 UTC (`"source":"user_reject"`). His item 1 came with its three choices unfilled: who allowed the
    first run is not recorded.
  - **The branch:** see the card's Branch line. The tip of `claude/cool-pasteur-oiqk1r` was 7f5ba39 (nothing
    newer). The new container's `.gate/` was empty: the tools that lived there are made again (HAND-OVER).
  - **His answers in that message:** DIFF-LIST **(a)** ("The list as printed from my Export at 30 steps
    (`.checkpoints/cs4b-content/question-2026-11-26-diff-list.md`) is mine, and its movement values become the
    game's starting movement table. A later Export replaces them the same way."): the hand-over's list is
    unblocked from its item 1. CI-TIME: #78 (the toolchain job's limit 15 to 25 minutes) is on main as b8f02b0:
    main is taken in once, before 4b-2 changes a shared doc. VISION-RULE, an addition (his item 9, a to e): for
    the docs pass, no code now; open, in his words: "the exact eye height, which reaches the game through my
    Export, and which points of an enemy's body the line is tested against". Saving every 25 minutes, the
    agents' work too; SEAT-OUT: stop at a whole point and report the last commit's id. The board add-on is active.
  - **Pull requests:** this chat cannot open or merge one: the guard keeps `gh api` read-only
    (`routine-guard.mjs:1466`, "GitHub writes go through the GitHub MCP tools"), the environment refuses `gh pr`,
    and no GitHub MCP tool is attached. At the close the pull request's title and text go in the notes; his
    Advisor opens the draft and merges on his tap (his item 11).
  - **`chat-words.mjs`** now names this session and writes only to a full path given on the command line (his
    standing rule of 2026-11-27 on full paths); Builder V4's saved notes keep V4's session id.
  - **The seat's start, by git's own record** (`git reflog`): the local `cs/4b-content` made at 7f5ba39 at
    14:06:16 UTC; `claude/ecstatic-wozniak-5cbdzs` created on GitHub by the first push at 14:06:28 UTC (15:06:28
    Amsterdam); the notes commit 876ac7d pushed 14:15:47; **main taken in: 35e96ad** (git's merge of 876ac7d and
    main b8f02b0, pushed 14:16:29; no clash; every file under `docs/`, `.claude/`, `.github/`, `tools/`,
    `legacy/` and `CLAUDE.md` equal to main's), its own commit with CI.
  - **CI on 35e96ad: green** (gate run 46014436944; toolchain run 46014436936, 14:16:34 to 14:25:23 UTC by the
    run's own times: under the new 25-minute limit). The count reproduced on it: **4,956** written lines against
    b9aad01 (55 files; 248 data lines of `crates/devtools/vectors/` left out).
  - **The baseline on this machine: ALL LOCAL CHECKS PASSED on 35e96ad** (14:18:25 to 14:26:59 UTC by the log's
    own times; `.gate/local-checks.log`): 219 Rust tests twice, 218 in release, clippy clean, client 136,
    sim-worker 51, types 8, net 1, 36 boundary tests, the type checks, 19 browser tests in 198 s, the recordings
    unchanged; **the players' file 54,514 bytes, sha256 b4a9774f9be1a35271cb92ce00daaf7f372b2ebb389c4dac03d10c67a64f59ba,
    as at 5c6738a**. **The gate: PASS with smoke** (its Playwright 1.56.1 asks for build 1194's
    `chromium_headless_shell-1194/chrome-linux/headless_shell`: linked to this machine's build-1243 headless
    shell; renderer SwiftShader). `repo-map-check.mjs`: OK, 237 entries cover 109 required paths (neither the
    gate nor CI runs it).
  - **Two choices for step 8's second part, for the report's departures and the advisor agent's final read:**
    (1) **The manifest beside his Export in `content/source/` is written by script**
    (`cs4b-content/content-source-manifest.mjs`), not moved byte for byte from the notes as the plan's "moves,
    byte for byte, to `content/source/` with its manifest" (`cs4-walk/plan.md:407`) could be read: a byte copy's
    first line would name the notes file it no longer sits beside, and its comparison lines name another notes
    file, and both leave the head. Each line is worked out from the Export's own bytes (and must equal the notes
    manifest's line of that name) or, for the message's two lines, copied by script; nothing is typed; the
    design's list of what the manifest holds (`cs4-walk/testlines-cs4.md:31-32`) is met whole. (2) **The second
    route takes five values that stand once among the fifteen** (walk speed, sprint multiplier, gravity, the
    refill time by its other name, push strength): three pairs are equal (the two easing times 0.164, body and
    push radius 0.4, step-up and the sights multiplier 0.55), so **a row swapped within one of those pairs
    passes every check**: said in the test and in the report as not verified. **Corrected at round 1's fix pass
    (test-engineer's TE-1, High): the gap was wider, since a swap between any two of the ten rows the second route
    does not read passed every check. It is closed by the table-name test (see "The fix pass" below), and the
    not-verified line is gone from the test's doc and the report's draft.**
- **Step 8, second part: the content files from his Export, and the tests on the real files: DONE** (commit
  **5cef19f8648860160c0e95f2bf5e4ed3a06590af**, 2026-11-27 14:50:27 UTC, pushed; on his DIFF-LIST (a); the gate
  passes with smoke; the full local checks passed whole, 14:40:27 to 14:46:45 UTC: 222 Rust tests twice, 221 in
  release, clippy clean, client 137, sim-worker 51, types 8, net 1, 36 boundary tests, the type checks, 19 browser
  tests in 196 s, the recordings unchanged, the players' file unchanged (54,514 bytes, b4a9774f...59ba); the
  committed Export's bytes read back from the commit: sha256 a9d47785...c4ab; **5,129 written lines of 4b-2**, of
  8,450: the part took 173, the sheet's count for it 100; two thirds, 5,633, not reached). **The report's first
  lines say that `content/source/` changes (his SOURCE-GO).**
  - `content/source/testlines-export.json`: `cs4b-content/testlines-export-30.json` byte for byte (`cp`, then
    sha256 a9d47785ed8f70d895d10480d180c9bba5a67f7e955e0c460c0c1bef7718c4ab: the pin in `testlines.mjs:39`;
    15,508 bytes). `content/source/testlines-export.manifest.txt` by `content-source-manifest.mjs` (the choice
    above; first commit of the Export's notes copy c2beffe3af82b1eea92f8cacc06e7e0dae455508, read from `git log
    --full-history`). `content/source/testlines-table.json`: the draft byte for byte (16 rows).
  - `content/movement/soldier.json` by `testlines.mjs write` ("15 values from his Export ... written"), then
    `--check` ("15 values, each his Export's"). **No range of the movement kind refused a value of his** (the
    loader took the file as it stands: the advisor agent's condition). **Held against the list he said yes to,
    by script** (`cs4b-content/diff-list-agree.mjs`, its output `diff-list-agree.txt`): every value agrees; 5 of
    the 15 are the rows his printed list shows (sprint multiplier 1.52 in parts A and B; air control 100, body
    radius 0.4, push strength 8, push radius 0.4 in part C), the other 10 equal the old game by the comparison's
    EQUAL list. For the report: the game takes his jump reach 3.071 (his Export's `values.jump.jump_reach_m`);
    the list's "equal" sentence names the old game's 3.0708, equal at the Export's three decimals.
  - `content/maps/test-scene.json`: the draft, then `canonical.mjs maps/test-scene.json` (6,654 bytes before
    and after: the keys sorted, no value changed, checked by script). `cargo run -p devtools --bin
    content_bundle -- --check`: "2 content files, a bundle of 8375 bytes, sha256
    b9901bc9e0244aee8888fc965284c2fe952abc8b38a49c6621d575df8ecd7fd3: every file is what the loader takes;
    nothing written".
  - Tests on the real files: `tests/content_files.rs` +2 (the repository's `content/` is the two files and the
    loader takes both; **five of his values by a second route**, found by name in his Export's text, read with
    the standard library's reader, held bit for bit against the loader's fields, no value typed);
    `tests/static_world.rs` +1 (the test scene through the loader and the builder: judged, depth 7 of at most
    31, and its tree the committed line `vectors/content-worlds.txt`: `test-scene shapes 23 depth 7 tree
    ffc96a10bc10cc1f`, written once with `SHAMBLE_WRITE_WORLD=1` on that test alone; **natively only**: no
    WebAssembly or browser run holds the real map yet, for the report); `testlines.test.mjs` +1 (`--check` on
    the real files: "15 values, each his Export's").
  - `docs/REPO_MAP.md`: one row for `content/` (the repo-map check said "missing from the map: content/";
    after the row: OK, 238 entries cover 110 required paths). The row quotes his SOURCE-GO from the register.
  - **Plants C1 to C8 in `plants-4b2.py`, run by hand, all as they should be** (4 defects caught by their checks'
    own words, 4 controls held, every file put back: sha256 of 17 files): a value one step of a double off
    (caught by the hand-over's check, C1, and by the second route, C2); two rows sent to each other's field
    with the table written from them (caught by the second route, C3; **the hand-over's own check passes it,
    C4**: what the second route is for); the last lane post a millimetre taller (the committed tree, C5); the
    three clean runs (C6 to C8). Every planted number is read from its file and changed by script. The names
    were first S1 to S8, which the stamp plants already use; renamed before any record of them was kept.
- **CI on step 8's head 5cef19f: green** (gate run 46019544265; toolchain run 46019544144, 14:50:37 to 15:03:06
  UTC by the run's own times: 12 min 29 s of its 25).
- **Step 9, cut A: the page carries the content bundle: DONE** (commit
  **ccde0d48c93f5cf3d42cdecab9daf46bec949d30**, pushed 2026-11-27 about 15:04 UTC, after CI on 5cef19f was read
  green; the gate passes with smoke; the full local checks passed whole, 14:56:38 to 15:03:16 UTC: 222 Rust tests
  twice, 221 in release, client 139, sim-worker 51, types 8, net 1, 36 boundary tests, 19 browser tests in 198 s,
  the recordings unchanged, the players' file unchanged; `content.json` 8,375 bytes in both pages;
  `cs4b-content/step9-order.md`, revision 2; **5,200 written lines of 4b-2**: the cut took 71). `build-checks.mjs`:
  `CONTENT_FILE` ("content.json"), `MAX_CONTENT_BYTES` (8 MiB, the loader's own `MAX_BUNDLE_BYTES`) and
  `contentProblems(files)` (a page folder without `content.json` beside `index.html`, or with one larger than the
  loader takes, is refused); `build.mjs`: `buildPage` runs `content_bundle` into the page's folder after Vite has
  built it (`writeContentBundle`: its own cargo call with the build's settings refusals and its own target folder,
  `.gate/target-content`), then holds the folder to `contentProblems` with the other page checks.
  `build-checks.test.mjs` +2 (the refusals at 0, at the cap and one byte above, and in a subfolder; **the cap
  read from the loader's own source line and held equal**). The production page: "2 content files, a bundle of
  8375 bytes, sha256 b9901bc9...7fd3: written to .../dist-production/content.json"; the page's table lists
  `content.json` 8,375 bytes. Plants C9 to C11 in `plants-4b2.py`, run by hand with P19: the build writing no
  bundle (the page refused by name), the cap a byte above the loader's (its tie test by name), a bundle taken
  from a subfolder (its test by name): 3 caught, the clean production page built (P19 held), every file put
  back. Main 2483e9a's recording files copied out by `git show` into `.gate/cs4b/main-recordings/` for cut B,
  before any tool writes: each equal to the branch's (sha256 1c05e6b8... rec, 6ac4b9cc... hashes, b766f0c1...
  vector, 48f07158... manifest).
- **Step 9, cut B, IN PROGRESS (uncommitted; saved 15:12 UTC as `cs4b-content/cutB-wip.patch`, 11 files of
  `crates/content` and `crates/devtools`; to restore: `git apply` it on 2a98ace, then `cargo run -p devtools
  --bin make_recordings -- --freeze first-loop` and `cargo run -p devtools --bin make_recordings`, which make the
  four recording files again):** the content crate's test map (`map::TEST_TEXT`, id `test-scene`) and
  `bundle::test_bundle()`; `scripts::Content` (a bundle's sha256 and the map), `looped`/`first_loop` taking it,
  `RECORDINGS` naming each recording's map; `replay.rs`'s `check_content` (the sha256 the header names, the
  loader's yes, the header's map) before the match is made, `ReplayError::Content`, and a header of zeros
  refused; `files.rs`'s `frozen`, `frozen_name`, `made(folder)` (the frozen content an input, first in the
  manifest), `freeze`; `make_recordings --freeze <name>`; the replay tool reading the frozen content beside a
  recording; the test module's `replay_hashes(rec, bundle)` and `replay_state_at(rec, bundle, tick)`; the tests.
  `first-loop.content.json` written by the freeze: 8,375 bytes, sha256 b9901bc9...7fd3, **the page's own
  `content.json` byte for byte**. **The proof against main's `git show` copies holds**
  (`cs4b-content/recordings-proof.mjs`, its output `recordings-proof.txt`): the 1,800 step records, the hashes
  file's lines 2 to 1,801 and the vector file the same; by design the header's content hash and map id (`""` to
  `"test-scene"`, 10 bytes), line 1 and three manifest lines. Left in cut B: the TypeScript side (`recording.ts`,
  its test, the Node replay test, the browser replay spec), then the checks.
  - (15:18 UTC) The TypeScript side is built (`checkContent` by WebCrypto; a header of zeros refused; the tests),
    rustfmt run on the eleven Rust files by name, clippy clean, the tools crate's tests and the recording unit
    tests green; plants C12 to C16 by hand: 4 caught, the control held, every file put back (20 files). The full
    local checks run on the tree (`.gate/local-checks-cutB.log`). **That first run failed, rightly**, at "the tools
    crate for wasm32 on its own line": `files.rs` imported the native-only `content_files` (`lib.rs:7-8`, gated to
    no wasm32) for `freeze`; the import and `freeze` are now gated the same way (the test-only module replays and
    never freezes); the wasm32 build is clean; the checks run again from 15:21 UTC (`.gate/local-checks-cutB2.log`).
  - **The count: past two thirds (5,633).** First read 5,668 with the recordings left out (5,672 by Builder V4's
    rule as written). **After the wasm32 fix: 5,690 by Builder V4's rule as written** (it counts the frozen
    content's 14 lines, the hashes file's 1 and the manifest's 3, the first time a recording changes in 4b-2) **and
    5,672 with `crates/devtools/recordings/` left out** as data, as the designs leave the recordings and the frozen
    content out (`cs4-walk/netcode-architect-cs4.md:569`). **That rule change is not this seat's to make alone**
    (the card: "A new threshold or order goes back to the advisor agent first"): it is asked of the advisor agent
    (`cs4b-content/advisor-count-rule.md`), and **until it rules both counts are reported and the higher decides
    every threshold.** **Its ruling (2026-11-27, about 15:29 UTC):** (1) APPROVE: what `make_recordings` writes into
    `crates/devtools/recordings/` is data and left out (the code that makes and reads it, and any hand-written file
    there, still count); (2) the generated TypeScript stays counted to the end of 4b-2 (left out, the count would
    be 5,624 and under two thirds: the method does not change part-way); (3) the higher-decides rule was right as a
    stopgap only: from now on **one official count decides** (the card's first section), the raw total beside it.
    **Cut B's official count: 5,672 (raw 5,690): past two thirds.** Its conditions: the sentence to him says what
    the count leaves out and that the generated TypeScript stays in (sent again so); the rule written into the
    card's count lines (done: the first section, the rung ruling, the HAND-OVER recipe, which now names b9aad01, its
    L1); new files marked before counting (its L2). Its look ahead before step 11: 5,672 + 520 + 350 = 6,542, under
    8,450.
- **Step 9, cut B: DONE** (commit **e1e2dd462e9f4d7c18228498a95394a153c20449**, 2026-11-27 15:28:48 UTC, pushed;
  the gate passes with smoke; the full local checks passed whole on the second run, 15:21:06 to 15:27:45 UTC: 225
  Rust tests twice, 224 in release, clippy clean, client 139, sim-worker 52, types 8, net 1, 36 boundary tests, the
  type checks, 19 browser tests in 197 s (the replay spec on the new recording: 1,800 steps, every shared hash as in
  the file), the recordings what their scripts make, the players' file unchanged (54,514 bytes, b4a9774f...59ba);
  **official count 5,672, raw 5,690**). The patch `cutB-wip.patch` is removed in this notes commit (it predated
  rustfmt, the TypeScript side and the wasm32 fix).
    **His one sentence was given in the chat at once** (before 15:18:47 UTC, the clock read right after it): "4b-2 is at 5,668 of its 8,450 written lines
    (past two thirds): step 8 is done and green (your Export's movement table and the test scene, 5cef19f), step 9's
    cut A is done and green (every page carries the content bundle, ccde0d4), and cut B (the first recording names
    its frozen content, with the proof that no state changed) is built and in its final checks, while step 9's last
    cut, step 10's wiring and the look ahead before steps 11 and 12 remain."

- **Step 9, cut C: the match is made on the content bundle: DONE** (commit
  **fbbd84daa7539df6b0662e01db4ecee1a468b501**, pushed 2026-11-27 by 16:27:24 UTC, when CI's first job started on it).
  **After the commit, on the same files:** the full local checks passed whole (16:23:36 to 16:30:25 UTC: 227 Rust
  tests twice, 226 in release, format and clippy clean, client 148, sim-worker 53, types 8, net 1, boundary 38, the
  type checks, 29 browser tests in 207 s, the recordings what their scripts make, the players' file 217,692 bytes in
  both pages); the gate passes with smoke. CI on fbbd84d: gate and smoke green; the toolchain run started 16:27:26
  UTC. **An order change, for the advisor agent's final read:** it was committed
  and pushed while its full local checks were still running (they were past formatting, clippy, every Rust test,
  the release tests and both page builds, at the boundary tests): this environment's Stop hook asks that every change
  be committed and pushed before a turn ends, as his saving rule does; `step9-order.md` says the full checks come
  before the commit. Their result is written here when they end; a defect they find gets its own commit. What it
  holds: `Sim::create`/`Sim::new(seed, mode, pad_bytes, bundle)`, the loader between the mode and the padding checks,
  `CreateError::Content` giving the loader's own sentence whole, the order test; devtools' `Replay::new` passing the
  frozen content; rust-reviewer's two allow-list entries (`C::round`, `C::__multi3`, `rust-reviewer-cutC.md`) with
  the test's pin; the Worker's `start` with `bundle` (a plain ArrayBuffer of 1 to 8,388,608 bytes, before the glue
  copies it; `MAX_BUNDLE_BYTES` exported to the page); the bounds (`THROWN_TEXT_UNITS` 512 code units for a thrown
  text, `ERROR_TEXT_UNITS` 1,040 for the page's check of an `error`, `cutText`; the Worker's own error event cut by
  `WorkerLink.workerError`); `content.ts` (one fetch, the copy before the transfer, `readBundle` of own fields after
  `ready`, kept nowhere: `PageContent`) and `boot.ts` (the content before any Worker; a failed fetch said in fixed
  words and no Worker made); the dev server's `/content.json` (`vite.config.mjs`, build.mjs's own
  `writeContentBundle` into `DEV_CONTENT_DIR`); the boundary tests (six refusal classes, each an Error and no trap,
  the module usable after; the cap read from the loader's own refusal; the 512 tied to `MESSAGE_BYTES`; a 1 MiB key
  within 512), both replays on the frozen content, the Chromium specs (a new `content.spec.ts`: one fetch, the served
  bytes the built file's, one changed value refused at its file and line, seven hostile bundles shown only as the
  loader's sentence with no element and the empty scene kept, a failed fetch with no Worker; the isolation spec's
  `/content.json` before the page loads). **Evidence before the commit:** the bindings' 16 and the devtools tests;
  Worker 53, page 139 then the app folder's 32 after the last changes, boundary 37; the type checks of the Worker,
  the page and the specs; the whole Chromium suite 28 of 29, the 29th a defect of this cut's own, fixed, then the
  page and content specs 15 of 15; cut C's plants 10 of 10 caught, the control held, every file and the players'
  file put back; every recording file byte for byte cut B's (`git diff e1e2dd4 -- crates/devtools/recordings`
  empty: content.json b9901bc9..., hashes 18fbc3a8..., rec 200a793d..., vector b766f0c1..., manifest 1320f8b8...);
  the players' file 217,692 bytes, sha256 04733d44fe90d57619f81ed1efed6ac5cef0b90228753affc9253e0c9fa96185 (54,514
  before; recorded, not gated: ADR 0002's G7). **Official count 6,504, raw 6,522.** Not yet: the full local checks'
  end, the gate, CI on this head, the plants' whole run (at the close).

- **CI on cut C's head fbbd84d: green** (gate and smoke 16:27:24 to 16:28:01 UTC; toolchain run 46033756121,
  16:27:26 to 16:37:16 UTC, 9 min 50 s: 29 Chromium tests in 3.7 min, 5 Firefox tests). **The advisor agent's two
  Q3 points, read on its log** (`.gate/ci-fbbd84d-toolchain.log`, not committed): the network was cut before the
  builds ("the network is cut: no TCP connection to ...:443 (curl exit 7)"), and under the cut the two
  development-server cases passed in 2.7 s and 2.2 s with no "Compiling" line anywhere in the browser tests' window
  (16:33:10 to 16:36:55): the content tool ran `--locked` and compiled nothing.
- **Step 10's wiring: the match holds the world of its map: DONE** (commit
  **885d382**, pushed 2026-11-27 by 16:41:26 UTC, when CI's first job started on it; after the commit, on the same
  files, the full local checks passed whole, 16:41:28 to 16:48:13 UTC: 229 Rust tests twice, 228 in release, format
  and clippy clean, boundary 38, 29 browser tests in 209 s, the recordings what their scripts make, the players'
  file 257,421 bytes in both pages; the gate passes with smoke; CI: gate and smoke green, the toolchain run started
  16:41:26 UTC). **Its follow-up, a defect of its own found by the advisor tool's question and confirmed in the
  code:** a recording whose header names another map that its content holds would have replayed on `test-scene`
  without a word (`check_content` asks only that the content hold the header's map; the match is always made on
  `LOCAL_MAP`; and since no state reads the world yet, its hashes would even have matched). Now refused before any
  call in each runner, "map: a local match is played on test-scene alone" (`replay.rs`'s `check`; `recording.ts`'s
  `checkReplayable`, with `LOCAL_MAP` tied to the Rust constant by the boundary test), with tests and plants C30 and
  C31 (both caught); official count with it 6,659, raw 6,677. Committed as c68b8d5 and dcb5e6b (the first one's
  full local checks failed on the recordings test's plain-read scan, his LINKS-CHECK: the tie read `lib.rs` with
  `readFileSync` inside `replay.test.mjs`, a recording reader; dcb5e6b moves the tie into `sim-wasm.test.mjs`; the
  failed test, the recordings tests (5) and the boundary tests (39) then pass; the full checks run again).
  **CI on 885d382: green** (gate and smoke; toolchain run 46035714024, 16:41:26 to 16:54:28 UTC, **13 min 2 s**,
  against 9 min 50 s on fbbd84d: every phase was about a third slower, the Rust tests +1:20, the page builds +38 s,
  the boundary tests +25 s, the browser tests +14 s, Firefox +20 s, so a slower runner, not one test; **11 min 58
  s under the job's limit, which is 25 minutes since #78 (his CI-TIME, `toolchain.yml:38`), taken in at 35e96ad**;
  the chat told him "two minutes under the 15-minute limit", which was wrong: corrected to him, a slip below).
  `sim_wasm::LOCAL_MAP` ("test-scene", the one map the content holds; the first recording's header names the same
  id); `CreateError::MapMissing` and `CreateError::World(WorldError)`, in the order seed, mode, content, map,
  world, padding (the order test covers the map; no map made in tests reaches the world's check, whose sentence is
  tested on its own); `local_world(&set)` builds the map's `StaticWorld` once; `LocalMatch::new(..., world)` keeps
  it with `static_world()`; no step queries it (4c), and nothing of it is hashed. Every caller: the local match's
  tests, `tests/step_alloc.rs` (the world built before the counting starts). Tests: the match's world equals the
  map's own (`solids`, `tree_depth`, `tree_bytes`); a bundle whose map has another id makes no match, natively and
  on the players' file (a seventh refusal class in the boundary test). **The built-file check where the collision
  library is first linked into the players' file:** 95,441 instructions walked (79,690 before), both twins' names
  read, nothing refused (no maths routine beyond cut C's two entries, no vector instruction, no fused multiply-add,
  the imports unchanged): rust-reviewer's step 10 read ("the maths allow-list stays empty at step 10") holds. The
  players' file: 257,421 bytes (100,215 gzip, 82,101 brotli), sha256 3660132be46b33a4fbf59404b47a85a2436cfea47251adf386a42e529b800644
  (217,692 at cut C). Evidence before the commit: the bindings' 18 and the allocation test, the devtools tests (20
  groups: the recordings replay unchanged), boundary 38, clippy and format clean, plants C28 and C29 caught (the
  match made on whatever map comes first: natively and on the players' file), every file and the players' file put
  back. **Official count at step 10's end 6,635, raw 6,653: under the first rung (6,750), so step 11 stays in 4b-2;
  the look ahead before step 11: 6,635 + 520 + 350 = 7,505, under 8,450.** Condition 4 of the advisor agent's
  re-read: no collision query is called from the match's step in 4b-2 (4c's), so the report says so.

- **Step 11, the schema writer: BUILT** (2026-11-27; his STEP11-ROOM had no answer when the advisor agent's ruling
  came back at 16:58 UTC, so it was built as the ladder says, as the chat had told him it would be). Under the
  ruling (`cs4b-content/advisor-step11.md`): `crates/content/src/schema.rs` (`#![cfg(test)]`): each kind's JSON
  Schema 2020-12 built as a content tree from the reader's own constants and written by the canonical writer
  (`movement.schema.json` from `FIELDS`; `map.schema.json` from map.rs's caps, `COORDINATE`, `FACING`, the aim's
  reach on the bounds by `prefixItems`, the fixed words as `enum`); the id rule once in `$defs` (pattern,
  `maxLength` 64, the reserved names by a `not` pattern); closed objects as `{"not": {}}`, fixed lengths by
  `minItems` = `maxItems`, a shape's two forms as `oneOf`, no `default` (Q1); the descriptions in its test-only
  table keyed by object and key (Q2: the movement fields' words from his hand-over table's "what" column, the map's
  from map.rs), every property exactly one, printable ASCII; map.rs's fixed words `pub(crate)`. The export test
  (`ts_export.rs`) admits one real folder `schema` of plain `.json` files (`committed_in`), writes and compares the
  schema files beside the types (`SHAMBLE_WRITE_TYPES=1 cargo test -p content`), and refuses a link named `schema`,
  any other folder, a folder inside `schema`, a `.json` link and a file that is no `.json` file, each by name (C6).
  The files: `packages/types/src/generated/content/schema/map.schema.json` (299 lines) and `movement.schema.json`
  (143). The Vitest test `packages/types/src/content-schema.test.mjs` (its own limits reader): each file is
  `JSON.stringify` of its sorted value; at every end of every number range of both schemas a closed end is taken and
  the next double outside flagged, an open end flagged and the next double inside taken (C1, C3: the facing's top is
  the one `exclusiveMaximum`); the bounds' reach axis by axis (C2); the id rule's plants (C4); every object closed.
  The Rust tests hold the schemas to the reader (the ranges in both forms, the caps, the reach from
  `AIM_REACH_CM` and `AIM_HEIGHT_MAX_CM`, the words, the id rule's length) and to the loader at each end, the numbers
  read back from the schema (C1). **Evidence:** content 43 tests, types 12; plants C32 to C36 caught (a low end
  always closed, any folder taken for `schema`, a property without its description, a schema file edited by hand,
  a field renamed without the files written again), C37 held; **the players' file unchanged by step 11** (sha256
  3660132b...0644, 257,421 bytes) and none of three descriptions, the id pattern or the dialect address in its bytes
  (0 hits each: Q2's condition 4). **Official count at step 11's end 7,978, raw 7,996: under the second rung
  (8,000), and the look ahead before step 12 (7,978 + about 1,300 with the map's reading + 350) is about 9,630, past
  8,450: the work stops before step 12 and asks him** (the card's rule; STEP12-GO asked in the chat). **Committed
  46c192f**, pushed by 17:10:41 UTC (CI's first job on it); after the commit, on the same files, the full local checks
  passed whole (17:09:36 to 17:16:25 UTC: 235 Rust tests twice, 228 + 6 = 234 in release, format and clippy clean,
  the generated files written again with the same digests, types 12, boundary 39, 29 browser tests in 208 s, the
  recordings what their scripts make, the players' file 257,421 bytes in both pages); the gate passes with smoke; CI:
  gate and smoke green, the toolchain run started 17:10:41 UTC.

- **The close, first point: the plants' whole run on c442a80** (started about 20:08:30 UTC, alone in the worktree).
  **A defect of the plants script found by it:** the control P19 ("the production page builds from the clean
  sources", the page's step alone) DID NOT HOLD, with "the built-file check left no stamp: the players' WebAssembly
  was not walked" (`.gate/plants-4b2.log`): in a whole run it follows P14, whose refused build leaves no stamp, as the
  stamp's design says; run by name after a clean build (as Builder V4 ran it) it held. P15 to P18 stand before it
  and are caught on the markup scan's own words, which run before the stamp's check. The fix, after this run ends
  (the running script holds a planted source file at any moment): P19 builds the players' file first, then the page
  (a chain, as C18 and C19), and the whole run is made again; both runs are told in the report. **The first whole
  run ended at 20:14:33 UTC** (about six minutes): **83 defect checks caught, 0 not caught; controls 10 held, 1 did
  not hold (P19, as above)**; every file put back (28 files by sha256); the players' file rebuilt to 3660132b...;
  the clean tree green (the players' build, the build checks' tests, the source scan: exits 0, 0, 0); its result
  file kept as `cs4b-content/plants-4b2-run1.txt`. P19 fixed, the anchors 85 of 85, the tree clean: **the second
  whole run started before 20:15:06 UTC** (the clock read right after; a first version of this line said "20:16",
  typed ahead of the clock: slip 9), on the same code (c442a80). **The second whole run ended at 20:18:20 UTC:
  83 defect checks caught, 0 not caught; controls 11 held, 0 did not hold** (P19 held, after its clean build of the
  players' file); every file put back (28 by sha256); the players' file rebuilt to 3660132b...; the clean tree
  green (exits 0, 0, 0); `cs4b-content/plants-4b2.txt` is this run's. The source tree clean after it.
- **The close, second point: the reviewers, round 1, on the code head c442a80** (`cs4b-content/review-brief-4b2.md`),
  three at a time: rust-reviewer, security-auditor, netcode-architect; then engine-architect, test-engineer,
  infra-engineer. Their notes: `cs4b-content/<name>-review-4b2.md`. The first three launched at about 20:20 UTC.
- **CI on the code head c442a80: green** (gate and smoke 20:08:23 to 20:09:07 UTC; toolchain 20:08:23 to 20:17:26,
  9 min 3 s).
- **Round 1, security-auditor: PASS with three Lows** (`cs4b-content/security-auditor-review-4b2.md`). It
  **confirms** ruling (2), the caps' departure (its P10), and ruling (4), the dev server's `/content.json`; ruling
  (1), the two maths entries, fine from its side (no new host import); dependencies: no drift (the six manifest
  digests match its two approvals). Its Lows, each to fix or accept in writing (an open security finding keeps the
  pull request needs-mustafa): **SA-1** `vite.config.mjs:27`, `new URL(request.url, base)` throws on the path `//`
  (connect catches it: a 500 page instead of Vite's own answer; its scratch run shows 200 without the plugin, 500 with
  it): fix, a failed parse passes to `next()`. **SA-2** `vite.config.mjs:26-31`, a page of another site open in the
  developer's browser can send a plain GET to `/content.json`, which starts the content tool and blocks the server
  while it runs (fixed arguments, nothing of the request reaches it, the answer unreadable to that site, the content
  public): the main session's choice is **fix**: refuse when the browser's `Sec-Fetch-Site` header is present and is
  neither `same-origin` nor `none` (a request with no such header, as Playwright's own requests and the command
  line, still served). **SA-3** `packages/client/scripts/testlines.mjs:54-57`, `:75-79` (used by `canonical.mjs:35`):
  `readPlain` checks with lstat, then opens without `O_NOFOLLOW` (a swap between them is followed; a link higher up,
  `content/` itself, too): fix, open with `O_NOFOLLOW` and check the handle by fstat, and lstat every part of the
  path under the repository. Its note: the brief's "later commits touch only `.checkpoints/`" missed
  `docs/DECISIONS.md` at 1c4906e (the brief now says so; the advisor agent is told). The fixes come in the one fix
  pass after the round, with round 2 for security-auditor on what changed.
- **Round 1, engine-architect launched** (before 20:26:57 UTC, the clock read right after; a first version of this
  line said "about 20:31", typed ahead of the clock: slip 10) in security-auditor's slot.
- **Round 1, rust-reviewer: PASS with one Medium and two Lows** (`cs4b-content/rust-reviewer-review-4b2.md`, DONE;
  its scratch probe `rust-reviewer-4b2-ramp.rs`). Its four step-10 conditions hold in `collision.rs`; nothing panics
  on content or network input; no `unsafe` added; ruling (1) fine (both entries its text word for word; its twins give
  0 lines under the build's rules), (5) fine but for its Low, (8) `fma4` fine, (9) recorded (the growth by name:
  about 85 KB the loader, 36 KB core's float text and parsing, about 30 KB the tree: parry3d about 20, our
  `StaticWorld` 9.8), (10) fine. Its runs: fmt and clippy clean; 235 tests in debug, 163 in release for sim,
  sim-wasm, content and devtools; every registry crate of sim-wasm's wasm32 tree has its overflow override. **RR-1
  (Medium)** `collision.rs:169-176` (guard `:292`): `enter_ramp`'s `f32` products overflow on finite but huge
  numbers, `inf - inf` is NaN, a NaN passes every comparison, so a ray over the slope hits the ramp's box (its probe:
  direction (2, 3, 0) misses, (2e38, 3e38, 0) hits at 0.0); and `enter_box` gives `Some((inf, inf))` for a direction
  of 1e-45 with an endless limit; `cast_ray`'s guard refuses only what is not finite. No caller in 4b-2. **Fix now**
  (the cast is built in 4b-2): `cast_ray` refuses a ray whose components or limit pass a stated bound, and the ramp
  test answers no hit on a NaN, with tests. **RR-2 (Low)** `schema.rs:247-251`, `:304`: the spawns list has
  `maxItems` only, while the loader refuses an empty one (`map.rs:338-339`): fix, `minItems` 1 with its test. **RR-3
  (Low)** `crates/sim-wasm/src/lib.rs:363`: the order test's doc counts four checks of six: fix the words. Not
  checked by it: the WebAssembly half of "the same bits" (this environment's classifier refused its vitest run; it
  cites the card and CI), cargo audit and deny (offline), aarch64, Firefox.
- **Round 1, test-engineer launched** (before 20:29:21 UTC) in rust-reviewer's slot, told the findings so far.
- **Round 1, netcode-architect: PASS, two Lows** (`cs4b-content/netcode-architect-review-4b2.md`, DONE). Route (i)
  as built (the header names the frozen bundle's sha256 and the map; the frozen file an input the scripts never
  make; every runner checks it; two_instances and two_processes replay on it too); loading content changed no state,
  checked on its own against `git show b8f02b0` (the 1,800 steps' 122,400 bytes and header bytes 0 to 50 equal; the
  header's hash is the bundle's sha256; the hashes file differs at line 1 only; the vector unchanged; nothing in the
  folder changed since e1e2dd4; plants C17 to C19 at step 1 in three runners); one content set per match (CD-7: only
  the constructor takes the bundle, the host refuses a second start); another map's recording refused natively and
  in TypeScript with `LOCAL_MAP` tied; the world held and queried by no step; condition 4 carried to 4c. Rulings (7)
  and (10) fine. Its runs (own target, `--locked --offline`): devtools recordings 5, static_world 4, two_instances 1,
  two_processes 1, library 17; sim-wasm 18 + 1 + 1; Node boundary replay, sim-wasm and static-world 22. **NA-1
  (Low)** `packages/sim-worker/boundary/replay.test.mjs:47-53`: the Node runner makes the match (`:52`) before
  `checkReplayable` runs (inside `replayOn`, `recording.ts:167`), where Rust (`replay.rs:120-122`) and Chromium
  (`determinism.spec.ts:75-78`) refuse first; the card's "refused before any call in each runner" overstates it: fix,
  `checkReplayable` before `new Sim`. **NA-2 (Low)** `crates/sim-wasm/src/local_match.rs:20`: `NO_CONTENT`'s comment
  is stale ("while there is no content data"): fix, the join names zeros until change-set 8 brings the bundle's sha256
  (ADR 0005 CD-8). Its forward note for 4c: the room gets the static world, and the step that decides movement and
  prediction uses the same world, built from the same bytes.
- **Round 1, infra-engineer launched** (before 20:31:44 UTC) in netcode-architect's slot, told the findings so far.
- **Round 1, engine-architect: one Medium, the rest Low or carried to 4c** (`cs4b-content/engine-architect-review-4b2.md`,
  DONE). **Ruling (3), the departure from its design 4.5: CONFIRMED by name** (the reading moves with its one reader;
  what stays keeps 4.5's place and order; the camera half could never land here: `bundle.rs`'s `FOLDERS` holds maps
  and movement only). Boundaries fine (content depends on nothing; sim on content and parry3d; sim-wasm the
  composition root; parry3d named only in `collision.rs`, behind sim's `V3`); the boundary's shape fine (its run of
  the boundary sim-wasm test: 14 of 14); the dev route against block 1.9 fine (one producer, the Rust tool, better
  than its planned JavaScript); the schema writer against 1.10 and CD-3 fine with the departures recorded; the unread
  generated types acceptable (type-only, held by the drift test). Its runs: content 44, devtools dependency_rules and
  static_world 7, sim-wasm 20; "test-soldier" and "Walking speed" 0 hits in the players' file. **EA-1 (Medium)**
  `packages/client/src/app/boot.ts:88-92`, `:99-104` (`content.ts:88-93`): when the loader refuses, `link.receive`
  returns the problem and the listener returns before `content.afterMessage` runs, and the Worker's error listener
  never calls it: the page's copy (up to 8 MiB) is held for the page's life, against `content.ts`'s "let go as soon
  as the match ends"; `content.test.ts` proves the class, not the wiring: fix both call sites, and test the wiring.
  **EA-2 (Low)** `map.rs:108-149` against `schema.rs:15-19`: the TypeScript types and the schema come from two sources
  (the structs with their renames; the reader's tables) with no tie, against CD-3's "from the same Rust types": fix,
  a tie test, and the report says how CD-3 is read. **EA-3 (Low)** the report's draft lacks two sentences (the
  generated types unread but for the drift test, cut C's Q2 c5; only block 1.9's serving part built, no watch or
  reload, Q3 c7): fix. **EA-4 (Low)** `content.ts:22-23`: `FORMAT` and `VERSION` copied from `bundle.rs:26-27`, tied
  only end to end by the Chromium spec: fix, a unit tie. **EA-5 (Low)** = NA-2 (`NO_CONTENT`'s comment). **EA-6
  (Low)** `bundle.rs:253-263`, `map.rs:397-400`: the test fixtures are public in the shipped crate, and the fixture
  map's id is the live map's: fix, `#[doc(hidden)]`; 4c's tests that rely on a map's shapes name their map.
  **EA-7 (Low, to 4c)** `LocalMatch` owns the authority's world; in 4c it moves to `Room`, the endpoint gets its own
  copy (design 1.8), and no rule yet keeps content out of room's and client-endpoint's manifests: 4c's plan. **EA-8
  (Low, to 4c)** the schema's limit function lives in the test alone: 4c's panel moves it into product code.
- **Round 1, infra-engineer: PASS, nothing to fix in 4b-2** (`cs4b-content/infra-engineer-review-4b2.md`, DONE; CI's
  logs read by `gh api` GET: runs 46033756121, 46035714024 and 46061540930, on c442a80). The toolchain times 9 min 3 s
  to 13 min 2 s, at most 52 % of the 25-minute limit: fine. Ruling (4) under the network cut fine (`cargo fetch
  --locked` before the cut; the content tool's target built by build.mjs first, each GET about 0.5 s; the isolation
  spec checks the answer). Preview links and hosted builds: nothing makes them differ from CI's page (relative
  addresses; `content.json` 8,375 bytes inside the 25 MiB check; the page sets no CSP of its own, as before). Ruling
  (9) fine (no cost change: the preview artifact about 1.1 MB a run, kept 7 days). Two Lows, both follow-ups for
  later work: **IE-1** the Chromium tests now take 3.6 to 3.9 minutes of the 5-minute timeout (`toolchain.yml:205`;
  4b-2 added about 22 s; `loop.spec` and `presses.spec` set most of it): track the total per head and decide on the
  limit or a split before 4c adds a long browser spec (a CI file: the CI Builder's). **IE-2** `content.json` keeps a
  fixed name with no hash beside hashed Wasm and scripts: when hosting is built, serve `index.html` and
  `content.json` as no-cache, or give the bundle a hashed name (a hosting change-set's). Not checked by it: the
  preview hand-off steps (skipped on push runs), lane 2's `preview.yml`, any host's caching.
- **Round 1, test-engineer: REVISE, one High** (`cs4b-content/test-engineer-review-4b2.md`, DONE; scratch scripts
  `test-engineer-4b2-logfails.py`, `-swap.py`). The plants' whole run holds (83 of 83, 11 of 11; every plant whose
  needle is a test title has that test among its FAILED lines; C21 failed on its own cap numbers); T-must 13 holds on
  its own evidence. **TE-1 (High, evidence that does not hold)** the not-verified sentence of step 8's second part
  (card :1055-1058, the report's draft, `content_files.rs:378-380`) states the gap too small: the second route reads
  5 of 15 fields, and the other 10 hold four more values that stand once (air control 100, jump reach 3.071, sprint
  returns at 30, sprint time 7): a swap between two of those passes every check (proved in its scratch copy: the
  table's rows and `soldier.json` swapped, `--check` exit 0, every cargo test green). Fix (its proposal): one
  assertion on the real table: every row's field is the last step of its Export name, but for the one named rename
  (`stamina.refill_time_from_empty_s` to `stamina_refill_time_s`; 14 of 15 rows follow the rule, read at 20:43
  UTC); that closes every swap, the equal pairs included, and the not-verified line goes. Its Lows: **TE-2** the cfg!
  assertions in `crates/sim/tests/native_target.rs:22` and `crates/devtools/tests/native_target.rs:22` can never fail
  (sim's `compile_error!` stops such a build first; N2 to N4 show no FAILED test line): the report says so. **TE-3**
  `build-checks.test.mjs:884-895` (and `:147`) skip when a link cannot be made, against "fails, never skips, under
  CI" (`test-list.md:179-180`): fail when `CI` is set. **TE-4** no ray that meets a shape has a -0.0 direction part:
  add one; the per-class minimum counts of the committed rays: to 4c. **TE-5** the seven hostile bundles pass on any
  refusal: pin each class's sentence. **TE-6** `static-world.spec.ts:47`'s wall-clock bound: kept (a backstop
  against a slow loop; the spec's own timeout bounds a hang), a follow-up if it ever flakes. **TE-7** the Vitest
  schema test's title overclaims: retitle. **TE-8** C20 to C29 ran on the folded players' file C19 leaves, with no
  paired clean control: a clean build before them, and controls.
- **The fix pass, round 1's findings (in hand from 20:43 UTC), each fixed or deferred:** fixed: TE-1, RR-1, EA-1,
  SA-1, SA-2, SA-3, RR-2, RR-3, NA-1, NA-2 (= EA-5), EA-2, EA-3 (the report), EA-4, EA-6, TE-2 (the report), TE-3,
  TE-4 (the -0.0 ray), TE-5, TE-7, TE-8 (the plants script). Deferred, each a follow-up with its reason: EA-7 and
  EA-8 (4c's plan), IE-1 (the CI Builder's file), IE-2 (a hosting change-set), TE-4's per-class counts (4c, with
  the casts' first caller), TE-6 (kept as a backstop). Then: the count (stop line 8,450), the full local checks, the
  gate, the plants' whole run again, CI; round 2 on what changed (security-auditor, rust-reviewer, engine-architect,
  netcode-architect, test-engineer); the verifier; docs-keeper; the report; the advisor agent.
  **At 20:49 UTC, built and tested, not committed** (saved as `cs4b-content/fixpass-wip.patch`, 18 files, the new
  `packages/client/src/app/content-ties.test.mjs` marked with `git add -N`; put back with `git apply` on c442a80's
  tree): RR-1 (`RAY_REACH`, the limit's cap, the ramp's NaN guard, a test of rust-reviewer's probe, the reach's
  edges, the tiny direction, the ramp as large as `f32`), TE-4's -0.0 rays, RR-3's words, NA-2's comment, EA-6's
  `#[doc(hidden)]`, RR-2 (`map::MIN_SPAWNS`, the schema's `minItems` 1 written again, its two tests), SA-1 and SA-2
  (`pathOf`, the cross-site refusal, the isolation spec's two checks), SA-3 (`plainFolders` from the base, `O_NOFOLLOW`
  and the handle's check, the test of a link higher up), TE-1 (the table's names held by a test on the real table;
  the second route's doc), EA-1 (`takeWorkerMessage`, `takeWorkerError`, `PageContent.holding`, two wiring tests),
  EA-4 (`BUNDLE_FORMAT`, `BUNDLE_VERSION`, `content-ties.test.mjs`). Collision 9, content 43, the tools' tests 15,
  the page's app tests 35 pass. **Official count 8,241, raw 8,259**: 209 under the stop line. Left: NA-1, EA-2,
  TE-3, TE-5, TE-7, TE-8, then the checks.
  **At 21:00 UTC (from the clock), notes commit f14905f** (pushed 21:01 UTC): the patch now holds 22 source files
  (957 lines), with NA-1 (the boundary's replay checks what it can replay before the content and the constructor),
  EA-2 (a test ties the generated types to the schemas: the same keys, corner lengths and words), TE-3 (a link test
  that cannot make its link fails under CI, never skips), TE-5 (the seven hostile bundles each pinned to the
  loader's own sentence, read from the players' file by `hostile-sentences.mjs`, never typed), TE-7 (the schema
  test's title). TE-8 in the plants script: C21 and C26 build the players' file clean first; 17 new entries: W14 to
  W16 (RR-1's reach, cap and NaN guard), C38 (TE-1: two rows of equal Export values trade fields; no content
  change, so only the table's names can see it) with its two controls C39 and C40 (the second route and the
  hand-over's own check pass it), C41 to C49 (EA-1 twice, SA-2, SA-1, SA-3, EA-4, EA-2, RR-2, TE-5), the clean
  controls C50 (every boundary test) and C51 (the replay, isolation and content specs in Chromium). C4 now runs the
  hand-over's own check alone (`-t`): the new table-name test in the same file catches C4's pair. The new entries'
  words are a failure's own where the runner prints them only then (cargo's `---- <test> stdout`, vitest's
  `<file> > <test>`, Playwright's frame of the failing line). Anchors: 100 edits of 101 plants, 0 wrong. **The 20
  changed or new plants by name, 21:01 to 21:04 UTC** (`.gate/plants-fixpass.out`): 15 caught, 0 not caught; 5
  held, 0 did not; every file put back (30 files by sha256); the players' file rebuilt to its digest from before
  (2e8866474b22...); the source tree's status the same before and after, and its diff byte for byte the saved patch.
  C38 caught on the test's own line (`movement.speed_up_time_to_90pct_s: expected 'stop_time_to_10pct_s' to be
  'speed_up_time_to_90pct_s'`); C39 and C40 held, so both value routes pass that swap, which is what TE-1 found.
  Found in the log: Playwright's frame of a failing line also prints its neighbours, so C43's and C44's first
  words (each one's own line of code) stood in both frames and told neither apart. They now read the assertion's
  own message (`-   403,` in the diff, `Expected: not 500`); run again at 21:05 UTC: both caught. **Official count
  8,348, raw 8,366** (21:05 UTC; 102 under the stop line). **The full local checks, first run (from 21:05 UTC):
  stopped** at `cargo test`: the simulation crate's source scan refused `next_up` twice in the new collision test
  (`collision.rs:756`, `:760`: a float method outside ADR 0004 part 7's list; the scan reads test code too). Fixed
  with the next float's bits (`f32::from_bits(RAY_REACH.to_bits() + 1)`, as `collision.rs:687` already does); the
  scan (9 of 9) and the collision tests (9 of 9) pass; format clean. **Second run from 21:08 UTC.** The advisor
  tool's read (about 21:07 UTC), each done: TE-1's sentence corrected in the card's step 8 entry and taken out of
  the report's draft (where the closing now stands); EA-3's two sentences added to the report's draft and the pull
  request's text (they were not there: the brief said they were); a control now holds only if it also says its
  words when it names any, and C4 and C40 (each run with `-t`) must say `Tests  1 passed | 11 skipped (12)` (their
  log already did: each ran its one test); test-engineer's own case for TE-1 added as C52 (rows and
  `soldier.json` swapped for `sprint_time_from_full_s` and `sprint_returns_at_pct`) with controls C53 (the second
  route) and C54 (the hand-over's check), for the whole run. Anchors: 106 edits of 104 plants, 0 wrong. The whole
  run comes before any round 2 reviewer starts; the brief's head and count are filled on the committed head.
  **The full local checks, second run: ALL LOCAL CHECKS PASSED** (21:08 to 21:17:14 UTC, `.gate/local-checks.log`):
  format and clippy clean; 236 Rust tests twice (the generated files written again unchanged), 235 in release (one
  more each: the reach test); client 152, sim-worker 53, types 13, net 1; boundary 39; 29 Chromium tests in 217 s;
  the recordings what their scripts make. **The gate: PASS with smoke** (`.gate/gate-fixpass.log`; +0 -0 symbols).
  **The players' file:** 257,421 bytes as before, the built-file check walks 95,441 instructions as before, sha256
  now 2e8866474b2279c0c571c85bf6e4e30a30e336c95e45410e8697a5a5c365cf00 (was 3660132b...): the file carries the path
  `crates/sim/src/collision.rs` once (the world's build has panic places there), and the fix pass moved lines in
  that file; the cast itself is in no step yet, so not in the file. (The new digest's cause is read so, not proved
  byte by byte.) **Official count 8,347, raw 8,365** (21:18 UTC; 103 under the stop line). **The code commit:
  70f11e130ee9dc0aabbb61a01bb9d3a6c013418e** (2026-11-27 21:19:05 UTC, pushed): the 22 source files, the new
  `content-ties.test.mjs` among them, and the patch out of the notes; `git status --porcelain -- crates packages`
  empty after it. **The plants' whole run on 70f11e1, 21:19 to 21:25 UTC** (`plants-4b2.txt`, 119 lines): **97
  defects caught of 97, 17 controls held of 17**, every file put back (30 files by sha256), the players' file rebuilt
  to its digest (2e8866474b22...), the clean tree green (exits [0, 0, 0]); C52 (test-engineer's own swap) caught,
  C53 and C54 held (C54 with its one test shown passing); `git status --porcelain -- crates packages Cargo.toml
  Cargo.lock content` empty after it. **CI on 70f11e1:** gate and smoke green (run 46069521913); its toolchain run
  (46069522189) was cancelled at 21:20:58 UTC by the next notes push (787c099): CI keeps only the newest push's
  toolchain run on a branch. The run read is the one on 5990e7e (46069923055; the same code): **green**, the job
  21:23:12 to 21:36:02 UTC (12 min 50 s, under its 25-minute limit); the gate and smoke green on 5990e7e too (run
  46069923030). **Round 2 from 21:26 UTC:** rust-reviewer, security-auditor and test-engineer (brief
  `review-brief-4b2-round2.md`); engine-architect and netcode-architect when a seat frees (three at a time). The
  tools on the real files at 21:30 UTC: `testlines.mjs --check` ("15 values, each his Export's", sha256
  a9d47785...), `content_bundle --check` (2 content files, a bundle of 8,375 bytes, sha256 b9901bc9...: "every file
  is what the loader takes; nothing written").
- **Round 2, security-auditor: PASS, four Lows, none blocking** (`cs4b-content/security-auditor-review-4b2-round2.md`,
  DONE; its scratch scripts `vite-round2.mjs`, `walk-round2.mjs` under `.gate/security-auditor-4b2-scratch/`). SA-1,
  SA-2, SA-3 closed (each run in a scratch copy of the real config and tools: `//` gets Vite's own page, no header,
  `same-origin` and `none` are served, `same-site`, `cross-site`, mixed case, empty and doubled headers get the 403;
  a link anywhere on the walk refused, `O_NOFOLLOW` gives ELOOP, outside the base refused). TE-5 and TE-3 fine. Its
  Lows, numbered here in its final answer's order (its notes number three of them otherwise): **L2-1** the isolation spec asks only `cross-site`, so a rule narrowed to `from === "cross-site"` would pass
  (one more ask with `same-site`, about 2 lines); **L2-2** nothing holds `canonical.mjs`'s passing of `base` (a test
  with `content/` as a link, about 4 lines); **L2-3** latent: the walk checks the normalised path while `openSync`
  opens the string as given, so `lnk/../` read outside the base in its scratch run; no caller reaches it (every path
  is made by `path.join`) (`file = path.resolve(file)` first in `readPlain` and `writePlain`, 2 lines); **L2-4**
  outside this diff, from change-set 3a: `serve.test.mjs` still skips its two link tests under CI (about 1 line, or
  a follow-up). Two risks for the main session to accept or not: a browser that sends no `Sec-Fetch-Site` is served
  like a tool (dev server only); a folder swapped for a link between the check and the open is followed (a race on
  the developer's own disk). Its own slip: three read-only git calls after `cd … &&` or `;`. **engine-architect
  launched at 21:33 UTC** in its seat.
- **Round 2, rust-reviewer: PASS, three Lows** (`cs4b-content/rust-reviewer-review-4b2-round2.md`, DONE). RR-1, RR-2,
  RR-3 closed; 2^20 a sound bound (the loader's coordinates stay within ±10,000 m, so every product stays finite and
  t stays within [0, 2^20]); step 10's four conditions unchanged; the test's values each pin one branch; the new
  branches the same bits natively and in WebAssembly (comparisons and NaN tests, no new arithmetic linked). **The
  players' file's new sha256, judged on its own builds** (release wasm32 from both heads, 288,101 and 288,103 bytes
  with names): the code section byte-identical; the data differs in 6 bytes, five of them `collision.rs` line numbers
  moved by +11 and one a line shift in another file; the name section 2 bytes longer, and the players' file strips
  that section. Its Lows: **(1)** the ramp guard looks for NaN only: an infinite `above` and `rate` give a NaN
  crossing, and an infinite `rate` alone a -0.0 crossing, each a wrong hit at t = 0 in a code-built world (probes:
  ramp [0,-1e33,-1]..[1e33,1e33,1] toward +x, origin (0,0,0), direction (0,-2^20,0), limit 100; ramp
  [0,0,-1]..[1e33,1,1], origin (0,0.5,0), direction (0,+2^20,0)); no loader-taken map reaches it (fix: `is_finite`
  for both, 0 new lines, and about 6 test lines). **(2)** `RAY_REACH`'s doc says the cap is 2^20 metres; it is 2^20 in
  t, metres only for a unit direction: the one in-reach answer the fix pass changed (origin (-5000,0.5,0.5), direction
  (1e-3,0,0), limit 1e7: a hit at t = 5e6 before the cap, none now) (about 2 doc lines). **(3)** nit: `map.rs:341`
  checks `is_empty()`, not `len() < MIN_SPAWNS` (1 line). Its runs (own target, `--locked`): fmt, clippy `-D
  warnings`, the workspace tests and the release tests of sim, content and sim-wasm (341 tests), the wasm32 release
  build. **netcode-architect launched at 21:35 UTC** in its seat.
- **Round 2, test-engineer: PASS, one new Low** (`cs4b-content/test-engineer-review-4b2-round2.md`, DONE; its plant
  script `test-engineer-4b2-r2plants.py`, its group plant's log `.gate/test-engineer-4b2-scratch/r2-group-cargo.log`).
  TE-1, TE-3, TE-4 (the -0.0 rays; the per-class counts to 4c as agreed), TE-5, TE-7, TE-8 closed; TE-2 closed (said
  in the report); TE-6 deferred as agreed. It replayed its swap on 70f11e1 (the name test fails on its own words),
  found C4's pair caught by the name test too, C21 failing only its own 2 cap tests, C43, C44 and C49 each finding
  their words inside the right test's failure block, the totals 97 of 97 and 17 of 17, and the card's step 8, the
  test's doc and the report's draft saying the gap and its closing correctly. **TE-9 (Low, a missing negative):** the
  name test holds each row's field to the last step of its Export name, not its group; his Export has
  `values.hornet.body_radius_m` 0.45 and `values.hornet.step_up_m` 0.3 beside the soldier's rows, and a row re-pointed
  to `hornet.body_radius_m` with `soldier.json` given 0.45 passes `--check`, the 12 hand-over tests and the cargo
  tests in its scratch copy. Fix (about 2 lines): the groups by name in the same test. (Its list names movement, jump,
  stamina and pushing; the table also has `aim_down_sights`, which the fix must name.) Its slip: one `git log` after
  other commands in one call, a read.

## Slips of this chat's own rules, for the report
- One git command did not start its own shell call: `git add .checkpoints/cs4b-content/review-brief-4b1.md` stood
  on the line after a `cat` that wrote that file, in one call (2026-11-26, about 16:46 Amsterdam). It staged that
  one notes file; nothing else happened. Every other git command of 4b so far started its own call.
- One chat update was headed "about 16:50 Amsterdam" when the clock read about 16:45: a time ahead of the clock.
- The card's "Last update" was once written 34 minutes ahead of the clock and corrected before it was committed.
- In 4b-2's first hours three more calls put a `git status` after other commands (a card update, a line count, a
  test run). All reads. Since then every git command stands alone in its call.
- One more, at step 7 (2026-11-26, about 22:40 Amsterdam): a `git status --short | head` stood after a `grep` in
  one call. A read. Told to him in the step's update.
- One more git command behind another program, at step 10's count (2026-11-27, about 00:40): `git status --short
  | head -5` after the count's python script, in one call. A read. Told to him.
- The card's entry for commit e3acb41 was first written with a push time of 00:31 when the clock read 00:24: a
  time ahead of the clock, typed and not read. Corrected before it was committed.
- The chat's update of 2026-11-27 was headed "00:06 Amsterdam" and was written before 00:05:10 by the machine's
  clock (the next event's own time stamp): a minute or two ahead of the clock. Told to him.
- The card's entry for commit ad88341 was first written with a push time of 01:35 when the clock read 01:34: a
  time ahead of the clock, typed and not read. Corrected before it was committed. The same again a quarter of
  an hour later: "Current step" written as 01:50 when the clock read 01:49:47; corrected before it was committed.
- Commit 2f058b2's message gives the written lines as 2,626; the count, taken after the commit instead of
  before it, is 2,627. The message stays as it is (no history is rewritten); the card and the chat have 2,627.
- The chat told him "Your second 'a' (23:00 Amsterdam)" as if he had sent a second message. He had not: it was
  his first message shown to the chat again after a restart. Corrected in the chat and in the notes.
- Two more git commands did not start their own shell call, both harmless reads or a write-back of committed text:
  a `git show HEAD:... > file` on the line after a python script (the take-in; told to him and to the advisor
  agent), and a `git log --oneline -1` after a `git status ... | head` in one call (after the merge).
- The chat told him "four new Mediums" for round 2 (they are five: the fifth was told as "one limit stays") and
  "49 MiB" for the test module's memory (46.9 MiB), and that test-engineer's own plants were all caught (two of
  five were not). The verifier found all three; corrected in the chat and in the report.
- The chat told him, in its update after round 1's fix pass, that the million-input fuzz run is by hand. It is
  not: CI's release line runs the tools crate's tests (test-engineer's TE-8). Corrected in the chat's update
  after test-engineer's read, and in the report.
- **Builder V6's, 2026-11-27:** (1) the take-in's merge commit 35e96ad carries git's default message and no
  attribution lines: the merge was made with no message of its own, and the guard refuses `git commit --amend`
  (`routine-guard.mjs:369`), so it stays as it is (no history is rewritten). (2) The card's "from 2026-11-27 15:06
  Amsterdam" for the seat's start was written before the time was read; git's record gives 14:06:28 UTC (15:06:28
  Amsterdam): the same minute, now with its source. (3) The chat told him the baseline checks "started 14:22
  UTC"; the log's own birth time is 14:18:25 UTC. Corrected to him in the chat. (4) At about 15:41 UTC one
  `git status --short` stood after a `cd` and before an `ls` in one call. A read. Since then each git command
  stands alone in its call. (5) The correction sent to the advisor agent at cut C says "15:58 UTC"; it was written
  before the clock read 15:57:53: a time typed ahead of the clock (the brief now says so). (6) At 16:42 UTC one
  `gh api` call (a read of CI's checks on 885d382) stood after a `grep` and a `date` in one call. A read. Since
  then each git and gh command stands alone in its call. (7) The card's "Current step" was first written "21:09
  Amsterdam" when the clock then read 20:07:44 UTC (21:07): a time typed ahead of the clock, corrected before it
  was committed. (8) **The chat told him twice, at step 10's end, that CI's 13 min 2 s run left "two minutes
  under the 15-minute limit".** Wrong: the toolchain job's limit is 25 minutes since #78 (his CI-TIME,
  `toolchain.yml:38`), which this branch took in at 35e96ad; the run was 11 min 58 s under it. Found at the close
  (20:10 UTC) while writing the reviewers' brief; corrected to him in the next chat message. (9) A card line said
  the second plants run started "20:16 UTC" when the clock then read 20:15:06: typed ahead of it, corrected before
  it was committed. (10) The same for engine-architect's launch: "about 20:31" written when the clock read 20:26:57;
  corrected before it was committed. From here every time in the card is written only after the clock is read.
  (11) At the fix pass (about 20:50 UTC) one `git diff --stat` stood after a `cargo test` in one call. A read.
  (12) **The chat told him two wrong counts of the fix pass's plants:** "17 new plants: 14 planted defects, 3
  controls" (about 21:00 UTC) and "each fix catching the defect it closes (17 of 17), and the 5 controls held"
  (about 21:06 UTC). The 17 new entries are 13 defects and 4 controls; the run of 20 by name was 15 defects caught
  (those 13, C21 and C26) and 5 controls held (those 4 and C4). Found by the advisor tool's read; corrected to him
  in the next chat message. The card's own entry had the right numbers. (13) For one notes commit (89866fd) the
  card's note on the board said a little more than that the add-on is active; trimmed in the next. (14) Two more git
  commands did not start their own call, both reads: at 21:00 UTC a `git status --porcelain` after a `cd` and a
  `date`, and at 21:29 UTC a `git diff --stat -- .checkpoints` after a `cd`. Told to him in the chat; since then
  each git command starts its own call (the shell's folder is the worktree already). (15) The same for gh, all reads
  of CI's state: three calls of 21:28 UTC put a `date` before `gh api`, and a watcher started at 21:27 UTC called
  `gh api` in a loop every 45 seconds (its calls never started their own shell call). The watcher was stopped at
  21:29 UTC; CI is read since then by plain `gh api` calls alone in their call. Told to him in the chat.

## HAND-OVER: where another session picks this up (written 2026-11-27, 00:44 Amsterdam; kept current)
His words of 2026-11-27, 00:43 Amsterdam (2026-11-26, 23:43:38 UTC; `cs4-walk/mustafa-2026-11-27-save-every-25-
minutes.md`, saved by script): "make sure to save your and the agents work every 25 minutes, this way my other
account can continue with what you were making. (Do not see this as a reason to cut your work to save usage,
just continue as you were)". So: the chat's work and its agents' work are committed and pushed at least every
25 minutes, and the work itself is not cut.

**Where everything is (Builder V6's seat, from 2026-11-27 15:06 Amsterdam).** Branch
`claude/ecstatic-wozniak-5cbdzs` on origin; local `cs/4b-content` in `/home/user/Shamble-Frontlines/.gate/cs4b-2`
(the card's Branch line). Builder V4's `claude/cool-pasteur-oiqk1r` ends at 7f5ba39 and nobody pushes there again.
A session of another account that takes this over works on its own branch from this branch's head and never pushes
here (his Lanes rule). The head: see the newest entries above (the card is saved with every push, at least every
25 minutes: his item 10). Code that is not yet committed is saved as a patch in the notes and named in the card's
newest entry (none at this save). No agent is running. The board add-on is active for this chat.
This seat's container is new: its `.gate/` held nothing of Builder V4's, so the machine-only list below is made
again here. SEAT-OUT from him: stop at a whole point and report the last commit's id.

**State of 4b-2, by step** (4,956 written lines of 8,450 at 5c6738a; two thirds is 5,633; rungs at 6,750 and 8,000):
- 6 the library's batch: done (298837a). 7 the built-file check: done (d313162, then 2f058b2 and e3acb41 for the
  advisories' rule). 8, first part, the two tools on test files: done (54e89d2). 10, the core, the static world
  on maps made in tests: done (8e5a268). 9, first piece, the type writer's manifest line and its rule: done
  (c09f459); 9, second piece, the generated content types: done (6290c85). 10, the fuzz run and the hostile maps through the world's builder:
  done (ad88341). 9, third piece, the page folder read through no link and the scan for `innerHTML` and its
  kin: done (5c6738a). CI: green on every code head through 5c6738a (gate 45918878271, toolchain 45918878273,
  13 min 1 s of the 15 the job may take: the card's entry "CI's toolchain job is nearing its time limit").
- **Nothing more of 4b-2 can be built without the content files**: every piece of rows 8, 9 and 10 that needs
  none is done. Step 11 keeps its place in the order (the card's "Ahead, by the count").
- **DIFF-LIST is answered: (a)** (his message to Builder V6 of 2026-11-27, 14:45 Amsterdam, item 7;
  `cs4-walk/mustafa-2026-11-27-builder-v6.md`, saved by script): "The list as printed from my Export at 30
  steps (`.checkpoints/cs4b-content/question-2026-11-26-diff-list.md`) is mine, and its movement values become
  the game's starting movement table. A later Export replaces them the same way." Step 8's second part, step 9
  and step 10's wiring are unblocked. (Before it, a bare "a" reached Builder V4's chat twice as a repeat of an
  older message: `cs4-walk/redelivered-2026-11-26.md`; `chat-words.mjs` leaves repeats out.)

**Now, in this order** (his DIFF-LIST yes is in):
0. **This seat's start** (2026-11-27): DONE. Main taken in once (35e96ad: b8f02b0, #78, CI-TIME), CI green on
   it; the machine's tools made again (the list below); the count reproduced (4,956 against b9aad01); the full
   local checks on 35e96ad passed whole, the players' file byte for byte 5c6738a's; the gate passed with smoke
   (the card's entry "The seat moved to Builder V6").
1. **Step 8, second part: DONE (5cef19f; the card's entry "Step 8, second part").** As planned:
   `content/source/testlines-export.json` = `cs4b-content/testlines-export-30.json`
   byte for byte (sha256 a9d47785ed8f70d895d10480d180c9bba5a67f7e955e0c460c0c1bef7718c4ab, the pin in
   `packages/client/scripts/testlines.mjs`); its manifest beside it (it must hold the line `sha256: <that
   digest>`); `content/source/testlines-table.json` = `cs4b-content/testlines-table-draft.json` (16 rows).
   Then `node packages/client/scripts/testlines.mjs write` and `--check` (it makes
   `content/movement/soldier.json`: 15 values; the dry run's text is in the chat's record and the loader took
   it). The map: `content/maps/test-scene.json` = `cs4b-content/test-scene-draft.json`, then
   `node packages/client/scripts/canonical.mjs maps/test-scene.json`. Then
   `cargo run -p devtools --bin content_bundle -- --check`. Tests on the real files: the real `content/` through
   `devtools::content_files::run`; `testlines.mjs --check` in `testlines.test.mjs`; three to five of his values
   held by a second route (found in the Export's own text by their names, never typed: his rule) against what
   the Rust loader reads; `StaticWorld::build` on the real map (depth, a committed tree digest). The report's
   first lines say that `content/source/` changes (his SOURCE-GO answer).
2. **Step 9** (the sheet's row 9, `cs4b-content/reread-sheet.md:167`), **built in the three cuts of
   `cs4b-content/step9-order.md`** (each ends green; the advisor agent is told of the cuts at its final read).
   Done already: its manifest line with
   the dependency rule (c09f459, as security-auditor approved it on two digests) and the generated content types
   with their export test (6290c85), the page folder's plain files and the markup-sink scan (5c6738a). **All
   three cuts DONE: A ccde0d4 (CI green), B e1e2dd4 (CI green), C fbbd84d** (the card's entries "Step 9, cut A",
   "cut B" and "cut C"; the rulings behind C: `rust-reviewer-cutC.md` (the two maths names) and
   `advisor-cutC.md` (the caps, the page's parse, the dev route), with the sheet and the test list revised in
   writing). Before step 10's first code push: CI on fbbd84d read green, and on its log the two Q3 points of
   `advisor-cutC.md` (the dev-server cases compile nothing; the network cut does not stop `cargo run --locked`).
3. **Step 10's wiring: DONE** (885d382, with its follow-up for another map's recording; the card's entry "Step
   10's wiring"). Official count at its end 6,635 (6,659 with the follow-up): the first rung (6,750) is not reached.
4. **Step 11, the schema writer** (the sheet's row 11, revised for the map's reading and for this ruling): **the
   advisor agent ruled at 16:58 UTC** (`cs4b-content/advisor-step11.md`, DONE; brief `advisor-step11-brief.md`):
   Q1 APPROVE, no `default` column (the report says why; the first field with a default goes to it first, with
   read.rs's "nothing missing" rule); Q2 APPROVE, the descriptions in `schema.rs` under `#![cfg(test)]`, keyed by
   object and key, every property exactly one, printable ASCII, the field's name, unit, range and use only, and
   after the build three of them searched for in the players' file (0 hits expected); Q3 REVISE, fold in and build:
   C1 an open end refused and the next double inside taken (a closed end taken, the next double outside refused),
   in Rust and in Vitest; C2 the aim's reach on `bounds` by `prefixItems` (x and z at least -327.67 and at most
   327.67, y at least 0 and at most 655.35, the other ends `COORDINATE`), its values tied to `AIM_REACH_CM` and
   `AIM_HEIGHT_MAX_CM`; C3 the Vitest limit test walks every range of both schemas, `prefixItems` too; C4 the id
   rule once in `$defs` (`^[a-z0-9][a-z0-9_-]*$`, `maxLength` 64, a `not` pattern for the reserved names), its
   plants in Vitest; C5 no new dependency; C6 the export test admits one real folder `schema` (its own type, never
   followed), write mode deletes only its `.json` files, tests for a link named `schema`, a folder inside it, a
   `.json` link and a file that is no `.json`; C7 each schema a content `Node` tree (no boolean, no null): lengths
   by `minItems` = `maxItems`, the shape forms as `oneOf`, each branch closed with all its keys; not step 11's: the
   bundle's `files` bound and PM-0005-5's scan (4b-1's row 2), no bundle schema. Q4 APPROVE, the schema files count
   in full. **Built** (the card's entry "Step 11, the schema writer"; his STEP11-ROOM had no answer, so the ladder
   decided). The count at its end, 7,978, was under the second rung, and the look ahead before step 12, about
   9,630, past 8,450: **he chose STEP12-GO (a) at 20:03:18 UTC: step 12, with the map's reading, moves to the
   start of 4c, and 4b-2 closes with steps 6 to 11** (the card's "Decisions taken").
5. **The close, now** (where it stands: "Current step" below; done so far: the plants' whole runs, review round 1,
   its fix pass at 70f11e1 with its whole run; round 2 running): the last code commit (a comment of `content.ts` that named step 12); `plants-4b2.py
   --anchors` (reads only), then `plants-4b2.py <commit>` whole, alone in the worktree (it edits source files one
   plant at a time); then the reviewers on the final diff (b9aad01 to the head), three at a time, one round, then
   one fix pass (a second round only on what changed): rust-reviewer, security-auditor (the dev server's route, the
   caps' departure of `advisor-cutC.md` Q1, the imports and the players' file), netcode-architect (steps 9 and 10),
   engine-architect (it confirms the departure from its design 4.5, `advisor-cutC.md` Q2), test-engineer (is it
   proven; the plants' whole run), infra-engineer (CI's time: 9 min 43 s to 13 min 2 s on this seat's heads,
   under the job's 25-minute limit (#78) and the browser tests' 5-minute one;
   the dev server's cargo run under the network cut); graphics-engineer, perf-auditor and art-director are not
   called (step 12 moved); the verifier until PASS; docs-keeper (his messages into DECISIONS by script: 18:52:56,
   21:16:26 and 23:43:38 UTC of 2026-11-26; his three messages of 2026-11-27 to Builder V6, 13:57:21 UTC (DIFF-LIST
   (a) among them), 14:02:23 UTC (GUARD-RETRY (a)) and 20:03:18 UTC (STEP12-GO (a)); **the VISION-RULE addition**
   (his item 9, a to e, which sharpens point 2c of version 2: recorded beside version 2 in DECISIONS,
   `docs/adr/README.md` and STATUS's line, with his two open points; no code); ROADMAP: step 12 with the map's
   reading at the start of 4c, step 11 built in 4b-2; STATUS follow-ups: heapless 0.9.4 when rstar or parry3d move;
   the licence texts for the hosted build; the `unit` column ships in the players' file and nothing reads it at run
   time (`advisor-step11.md`, Low); the first field with a default goes to the advisor agent first, with read.rs's
   "nothing missing" rule (`advisor-step11.md` Q1); 4c carries the re-read's condition 4 (the first collision query
   from the match's step is 4c's, so a recording that covers it is 4c's); the generated content types have no reader
   in 4b-2 but the drift test); the report (`/advisor-report`; its first lines:
   `content/source/` changes; then that 4b-2 changes nothing a player sees in a match that runs, the scene having
   moved); **the pull request's title and text in the notes** (this chat cannot open one: his Advisor opens the
   draft, his item 11); the notes out of the head; the advisor agent on the final head (it runs the gate itself;
   it is told of the order changes: step 10's core and the pieces of steps 9 and 10 that need no content file were
   built before step 8's second part, while DIFF-LIST was open; step 9 in three cuts; cut C and step 10 committed
   before their full local checks ended, as the Stop hook asks, each check then passing on the same files; and of
   the seat's move); his go; the merge, by his Advisor on his tap, with the full head as the expected head.

**Made again on Builder V6's container (2026-11-27, under `/home/user/Shamble-Frontlines/.gate/cs4b-2/.gate/`):**
the bindings tool fetched as CI fetches it (`toolchain.yml:77-128`): the archive from the release address over
https, 11,860,397 bytes and its sha256 checked against his values, its member list checked, the one program
taken out into an empty folder, 8,170,784 bytes and its sha256 checked, then moved to `wb-tool/wasm-bindgen`
(GitHub's own `gh release verify-asset` is not run here: this environment refuses `gh` porcelain; CI runs it);
`strict-line.txt` copied by script from `toolchain.yml:173`; `npm ci --ignore-scripts --no-audit --no-fund` and
`cargo fetch --locked` at the root (49 crates). This machine's browsers are `/opt/pw-browsers` build 1243
(`chromium`, `chromium_headless_shell`): the gate's `pw-1194` link is made when the gate first runs here.

**What is on this machine only (under `.gate/`, not in git) and how to make it again:**
- `.gate/wb-tool/wasm-bindgen`: the bindings tool, the file `packages/client/scripts/wasm-bindgen-tool.mjs`
  records (CI's toolchain workflow shows the download and the checks). `local-checks.sh` points
  `SHAMBLE_WASM_BINDGEN` at it.
- `.gate/strict-line.txt`: one line, read by `local-checks.sh`: `strict="--noEmit --strict
  --noUncheckedIndexedAccess --exactOptionalPropertyTypes --noImplicitOverride --noFallthroughCasesInSwitch
  --noImplicitReturns --isolatedModules --verbatimModuleSyntax --erasableSyntaxOnly
  --allowImportingTsExtensions --module preserve --target es2023 --lib es2023"` (CI's own flags).
- `.gate/pw-1194/chromium_headless_shell-1194/chrome-linux/headless_shell`: a link to the machine's Chromium,
  so that `PLAYWRIGHT_BROWSERS_PATH="$PWD/.gate/pw-1194" node tools/gate.mjs` runs (after `npm --prefix tools
  ci`); CI's smoke job is the check on the pinned browser.
- `.gate/cs4b/batch-step9/` (step 9's two files) and `.gate/rust-reviewer-step10-scratch/` (its probe; what
  the tests need of it, the chain of cubes, is in `collision.rs`).
- The local checks: `bash .checkpoints/cs4b-content/local-checks.sh > .gate/local-checks.log 2>&1` (about 13
  minutes; the last line must be ALL LOCAL CHECKS PASSED). The count, before every commit (the official count of
  the card's first section, the advisor agent's ruling of 2026-11-27): `git diff --numstat
  b9aad01954be666340a7ec5c6e9fc9353b11abf1 -- crates packages`, added lines, new files marked with `git add -N`
  first; `crates/devtools/vectors/` left out but for `.py` files, and what `make_recordings` writes into
  `crates/devtools/recordings/`; the generated TypeScript counted; the raw total with the recordings in given
  beside it.

**Rules that bind whoever continues** (his words are in the card's first sections and in `docs/DECISIONS.md`):
every git and gh command alone in its own shell call; every file a shell command writes, moves, copies or deletes
spelled out with its full path, no variable, substitution or wildcard in its name, or the Write and Edit tools
(his standing rule of 2026-11-27); no CI, kit or gate file; no value typed by hand; a manifest change held for
security-auditor's answer; times from the clock; no merge without his go on the head the advisor agent approved;
never a test cut or thinned to fit CI's limit (his CI-TIME); a toolchain run stopped at its limit is told to him
in two lines.

## Current step
4b-2 (2026-11-27, 22:28 Amsterdam, from the clock; Builder V6). **Steps 6 to 11 done; step 12 moved to the start
of 4c by his STEP12-GO (a), received 20:03:18 UTC.** On this seat: 35e96ad (main in), 5cef19f (step 8's second
part), step 9 in three cuts (ccde0d4, e1e2dd4, fbbd84d), step 10's wiring (885d382, with c68b8d5 and dcb5e6b),
step 11 (46c192f): each with its full local checks, the gate and CI green (CI on 46c192f: gate and smoke green,
its toolchain run read at the next step). **Official count 7,978 at step 11, raw 7,996, of 8,450.** In hand: **the
close** (HAND-OVER item 5): the plants' whole run twice (done), review round 1 (done, six reviewers), its one fix
pass (done: code commit **70f11e1**, 21:19 UTC; the full local checks, the gate, the plants' whole run 97 of 97 and
17 of 17; official count 8,347), round 2 on what changed (running from 21:26 UTC); then the verifier, docs-keeper,
the report, the notes out of the head, the advisor agent, his go.

## Next steps (the sheet's revision 3, section G; each step ends with the count of written lines)
- **4b-1:** 1 the content crate's manifests · 2 the reader, the writer, the bundle, the two kinds, the old tuning's
  two extracts by script · 3 the proofs (corpus, broken files, the bounded fuzz run) · 4 the links check · 5 the
  time unit (last, so that 4b-1's rung moves code not yet written). By the designs' count about 3,300; planned
  5,950; stop line 7,450 if he takes two pull requests.
- **4b-2 (held):** 6 the library's manifest batch with the feature compare · 7 the built-file check · 8 the tools
  and the data (**his Export at 30 steps; DIFF-LIST**) · 9 content to the simulation, the generated types, the
  frozen content of `first-loop` · 10 the static collision world · 11 the schema writer (first ladder rung) · 12
  the page's scene (second rung). About 3,750 by the designs' count; planned 6,750; stop line 8,450. **Steps 6 to
  11 DONE; step 12 moved to the start of 4c by his STEP12-GO (a) (2026-11-27, 20:03:18 UTC), with the map's
  reading field by field.**
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
- DIFF-LIST (a): his Export's list at 30 steps is his; its movement values become the game's starting movement
  table; a later Export replaces them the same way · Mustafa · 2026-11-27, received 13:57:21 UTC
  (`cs4-walk/mustafa-2026-11-27-builder-v6.md`, item 7).
- GUARD-RETRY (a): the guard check run once more; he denied it · Mustafa · 2026-11-27, received 14:02:23 UTC
  (`cs4-walk/mustafa-2026-11-27-guard-retry.md`).
- STEP11-ROOM (build the schema writer now, or move it to 4d so the scene fits): no answer of his; asked in the
  chat at about 16:47 UTC with "if no answer comes: (a), as the approved ladder says"; the advisor agent's ruling
  came back at 16:58 UTC, and step 11 was built under the ladder (the first rung did not fire: 6,635 < 6,750).
- **STEP12-GO (a): the page's scene (step 12), with the map's reading field by field that the advisor agent's
  cut C ruling put there, moves to the start of 4c; 4b-2 closes with steps 6 to 11** · Mustafa · 2026-11-27,
  received 20:03:18 UTC (`cs4-walk/mustafa-2026-11-27-step12-go.md`, his "A", by script). The plan's second rung,
  chosen by him (the count at step 11's end, 7,978, was 22 under it; the look ahead, about 9,630, was past his
  8,450). So 4b-2 changes nothing a player sees in a match that runs (the page still draws the empty scene; what is
  new on the page is its content's fetch and, when that fails or the loader refuses the content, the notice's
  plain text), and the reviewers for step 12 (graphics-engineer, perf-auditor, art-director) are not called.

## Running work
- From 21:26 UTC: review round 2's first three (rust-reviewer, security-auditor, test-engineer), each writing
  `<name>-review-4b2-round2.md`; then engine-architect and netcode-architect. CI's toolchain run on 5990e7e
  (46069923055) is read before the next push. Then: round 2's one fix pass if needed, the verifier, docs-keeper,
  the report, the notes out of the head, the advisor agent.
- rust-reviewer's read of cut C's two names ended (DONE). Its own disassembly run filled the disk for a while (its
  notes' warning: never `llvm-objdump -d` on the twins); it deleted that output, 54 GB free again.
