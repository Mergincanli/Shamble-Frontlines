# Brief for docs-keeper: change-set 4b-1's docs pass, second part (after the reviews)

Your first part is in `docs/` at commit 495204a (your notes: `.checkpoints/cs4b-content/docs-keeper-4b1.md`). The
reviews are in and the code has its final head (`git log -1 --format=%H -- crates packages` names it). This part makes the docs true for that head and brings
STATUS down. The same rules as the first part: you edit only under `docs/` (never `docs/DECISIONS.md`, never an ADR's
text, never `docs/reports/`: the main session writes the report); notes and scripts under
`.checkpoints/cs4b-content/`; git read-only, each git command alone in its shell call with nothing in front of it;
never git add, commit, push, switch, checkout, restore, stash, merge or reset; the repository is public.

## 1. What the fix passes changed (make REPO_MAP, the 4b-1 rows and the archive's block true)
The card's steps 26 to 30 (`.checkpoints/cs4b-content.md`) list them in plain words. In short, against what your
first part wrote "at ef2edec":
- `crates/devtools/vectors/content-corpus.txt` holds **26,653 numbers** (13,067 at ef2edec): the 2,038 old ones,
  5,000 doubles from random bits, 5,000 decimals, 2,000 real ties at 2 to 25 binary places, 12,586 lines for
  the powers of two (each with the double under and over it, both signs), 29 edges. `content-verdicts.txt` was
  written again (38 lines of digests: 37 classes and the fuzz run's first inputs).
- `crates/sim/tests/rules_scan.rs` now also reads `crates/content/src` (`the_content_crate_keeps_its_rules`).
- `crates/content/src/map.rs`: a shape's form is `Form::Box` or `Form::Ramp { rises_toward }`; a shape has no
  `group`. `json.rs`: an id is none of the names Windows keeps for its devices. `number.rs`: the even text of a tie
  is taken only if it reads back as the number.
- `crates/devtools/src/content_proofs.rs`: `LIMIT_CASES` and `limit_case(at)` (seven cases, made one at a time,
  each found by its name) where `limit_cases()` was; the fuzz run has ten stages (the map's own rules are one);
  `crates/devtools/tests/content_fuzz.rs` has a fourth test (the search for a key that is there twice); `crates/devtools/src/files.rs`: `read_plain` opens once and reads through the handle,
  `write_plain` checks the name.
- The tests' readers of committed vectors go through the plain read (Rust, Node, the browser spec).
- `crates/content/src/json.rs`: every list is cut to its length when it closes.
- Written lines: 7,164 of 7,450 (git's count of added lines against main, `crates/` and `packages/`, data
  files left out).
Check each claim you write against the tree, as always (the code is the source of truth).

## 2. The reviews' follow-ups (STATUS's "From change-set 4b-1" line names the ids; the archive's block holds the text)
Give each an id in your 4B- series and say in the archive's block who found it:
- **The simulation re-exports the content crate's time unit at step 6** (engine-architect, EA-4): until then the
  number 3,000 stands twice, held equal by `crates/devtools/tests/time_unit.rs`.
- **Which way a spawn's facing of 0 degrees points is pinned at step 8**, before the first map file is written
  (engine-architect, EA-6); a spawn's `at` is pinned now (the ground between the feet). Its round 2 says what
  the line must state (0 faces +z and degrees grow toward +x, the old game's own convention; and how the
  simulation reads an angle without a std sine is chosen before the first map file, because a tighter grid
  later is a version step): `.checkpoints/cs4b-content/engine-architect-review-4b1-round2.md`.
- **4c's match setup refuses, by name, a map with fewer spawns than the match has slots** (security-auditor's
  condition on the loader asking for at least one spawn only).
- **For the CI Builder** (this chat writes no CI file): a clippy step in the toolchain job, so the content crate's
  lint guard is held by CI and not only by the source scan and by hand (security-auditor: not later than 4b-2's
  merge); `-p content` in the job's release test line (rust-reviewer, RR-2; security-auditor, SA-5;
  test-engineer, TE-8). **A correction:** the 1,000,000-input fuzz run DOES run in CI (the job's release line
  holds `-p devtools`, `.github/workflows/toolchain.yml:162`); only clippy and the content crate's own release
  tests are by hand. If your first part's 4B-CI-CONTENT says otherwise or leaves the clippy step out, correct
  and extend it; do not make a second id.
- **For 4b-2's plan** (engine-architect's notes, no findings): step 9's TypeScript cap of 8,388,608 bytes needs
  its own tie to the Rust cap; the bundle's outer shape has no Rust struct for the type writer; `f64::round`
  (`map.rs`, `time.rs`) enters the players' file at step 9, so step 7's libm rule must allow it or the code rounds
  in whole numbers.
- **What the fuzz run cannot reach** (security-auditor): the caps (its inputs are a few kilobytes), full map
  lists, more than one map in a bundle, long strings; it measures no coverage.
- **One sentence per refusal, pinned in a table** (test-engineer, TE-7): today one refusal's sentence is pinned;
  the others are held only by the digests, which the by-hand rewrite replaces without showing what changed.
- **The source scan and a file-level `#![cfg(test)]`** (engine-architect, round 2): the scan does not see such a
  file as test code (it fails closed); step 9 plans one in the content crate (`ts_export.rs`). And when the
  build lints of steps 6 and 7 land, their hashed-map and clock half must cover the content crate too.
- **Two older readers of committed vectors go around the plain read** (security-auditor, round 2; outside this
  change-set): `packages/sim-worker/boundary/rules.test.mjs` and `packages/e2e/tests/rules/wasm-rules.spec.ts`.

## 3. STATUS: state, and its length
- The in-flight row and the "Waiting on Mustafa" bullet: 4b-1 is "reviewed (two rounds, no Critical or High);
  the verifier and the advisor agent's verdict on the final head are in its report" (the pull request's number
  comes from the main session when it opens: leave "none yet"; write no head id, the report holds it).
- **STATUS is 134 lines; his limit is 120** (STATUS-DIET, 2026-11-10; it was 125 before your first part). Bring
  lane 1's own lines down as far as they go without losing what must stay in STATUS:
  - must stay, word for word: the VISION-RULE follow-up (his point 4c asks for the line in STATUS) and
    4B-AIM-REACH's sentence that names the change-set which widens the aim (his DEPART-4 note: "STATUS names the
    change-set that widens it");
  - may shrink to pointers, their text moved to the archive by script, byte for byte: the "Answered on
    2026-11-26" bullet; the long form of SIGHT-FLAG and DUR-1 (two lines with the questions in plain words are
    enough: the report's "Decisions needed" holds them whole); "Next" item 2's history; the 4b-1 in-flight row's
    list of contents (REPO_MAP and ROADMAP hold it);
  - **other lanes' lines are not yours to reword.** If STATUS is still over 120 after lane 1's lines are down,
    say exactly which other lanes' lines are stale after #73 and #74 merged (the "day guard, after
    guard-wrappers (waiting for his go...)" bullet in "Phase" is one) and how many lines each would free; the main
    session puts that in its report for Mustafa and the Advisor. Do not move them.
- Run `node .claude/scripts/repo-map-check.mjs` at the end.

## Your final answer (at most 20 lines)
What changed per file with line numbers; STATUS's length before and after; what you could not place; what you did
not check.
