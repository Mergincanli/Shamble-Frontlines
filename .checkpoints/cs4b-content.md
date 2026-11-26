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
- **Last update:** 2026-11-26 13:27 Amsterdam (from the clock)

## Sizes (his point 5 of 2026-11-25: which is which)
- The designs' own count for 4b, bottom-up: 5,410 lines (2,780 code, 2,520 tests, 110 configuration).
- Planned, which already holds the 1.8: 9,750. Stop line (planned times 1.25): **12,200**; it stays as it is (his
  words of 2026-11-25).
- At 4a's measured rate (2.2 times the designs' count before review fixes, 2.4 after) 4b would end near 11,900 and
  then near 13,000: the count is taken at every step and said in each step's summary.
- The ladder, fixed in the plan: at 9,750 written lines the schema writer and its tests move to 4d (with the schema
  half of PM-0005-7 and PM-0005-8; the panel's limits are written in the panel's code until then); at 11,000 the
  page's scene (step 9) moves whole, with its browser tests, to the start of 4c. Never moved: the built-file check,
  the static collision world with PM-0004-32, the loader's seven stages with the reader's proofs, the Testlines
  check. He is told before a part moves.

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

## Current step
Before any edit: the advisor agent's re-read of the plan's section 5 against main 2483e9a with his answers (it
carries 4b's exact symbols to add and to remove); then test-engineer and security-auditor read the test list (not a
review round: the advisor agent's ruling in the plan, section 17); then step 0, the one manifest batch, uncommitted
and unbuilt until security-auditor has answered on exactly that content (`dependency-check.md`; his DEP-HOLD: while
a manifest or lockfile differs from main nothing is pushed before that file exists).

## Next steps (the plan's section 5; each step ends with the count of written lines)
0. The manifest batch: the new `content` crate; `sim` on `content` and on `parry3d` =0.31.1 (default features off;
   `dim3`, `f32`, `alloc`, `enhanced-determinism`; its `std` off if that builds for both targets); the overflow
   lines the build's own check asks for; the tools crate's unused path edges dropped. The root `Cargo.toml` is not
   a file the guard protects today (read in `.claude/hooks/routine-guard.mjs`, `protectedRel`): risk tier 1.
1. The built-file check for the fused multiply-add, with its planted defects, before the collision library is used
   by anything (section 5.3). **If the first build shows a fused routine linked from the library's tree, 4b stops
   for his decision.**
2. The content crate's reader and writer (all our own): JSON to a tree with line and column, numbers, the one
   canonical writer, the typed reader with field paths, ranges and length bounds, the schema, errors that name the
   file. Only what the content files need; everything else refused, duplicate keys included.
3. The bundle and the two kinds (movement table, map document). 3b. The reader's own proofs (content items 1 and
   2; the fuzz run before the first value rests on the reader).
4. The integer time unit, 3,000 units a second, with the rounding report.
5. The tools: `content-bundle`, `testlines` (`--check`, `write`, `compare-old`, `table`).
6. The data: his new Export under `content/source/`; `content/movement/soldier.json` by the tool;
   `content/maps/test-scene.json` by hand from the Art sheet's sizes. **Needs his Export at 30 steps and his yes
   to DIFF-LIST.**
7. Content reaches the simulation once per match through the constructor; the build writes `content.json`; the
   page fetches it once and hands its bytes to the Worker.
8. The static collision world from the map; its bytes and 10,000 rays identical natively and in WebAssembly.
9. The page draws the scene from the map; the readout's content rows; the browser tests.
Then: the gate and the local checks at every step; reviewers (rust-reviewer, engine-architect, security-auditor;
then test-engineer, graphics-engineer, perf-auditor; two rounds at most); the verifier; the advisor agent on every
pushed head; docs-keeper; the report; his short browser check.

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
