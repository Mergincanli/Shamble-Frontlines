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
- **Last update:** 2026-11-26 14:52 Amsterdam (from the clock)

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

## Current step
**Waiting for Mustafa's letter for SIZE-4B** (any answer lets steps 1 to 5 start). Then: step 1's approved batch
back into the tree (digests checked), committed and pushed with the foundation check's `content` half, and step 2
(the reader and the writer). His FEATURE-COMPARE letter gates step 6 only. The question file:
`cs4b-content/question-2026-11-26-feature-compare.md` (with its two dated additions).

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
