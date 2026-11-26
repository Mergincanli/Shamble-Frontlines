# Review brief, change-set 4b-1 "Content", round 2 (and test-engineer's first read), common to every reviewer

**Where.** The git worktree `/home/user/Shamble-Frontlines/.gate/lane1-takeover`, branch `cs/4b-content`, head
`d36e416` (round 1 read `ef2edec`). Round 1's brief still holds for what the change-set is, its thirteen
departures and **every rule for you** (`.checkpoints/cs4b-content/review-brief-4b1.md`, "Rules for you": read it
first; your notes file for this round is `.checkpoints/cs4b-content/<your-name>-review-4b1-round2.md`, your target
folder `.gate/<your-name>-4b1-target` as before).

**Round 2 reads what changed, and nothing else** (the review budget: "At most two rounds, the second on what
changed; a third only for an open Critical or High"): `git diff ef2edec d36e416 -- crates packages` (the files
under `crates/devtools/vectors/` are data: two were made again by their scripts). The three round-1 notes, each
finding with its id: `engine-architect-review-4b1.md` (EA-1 to EA-8), `security-auditor-review-4b1.md` (SA-1 to
SA-7), `rust-reviewer-review-4b1.md` (RR-1 to RR-5 and nits). The card's step 27 lists the fix pass in plain words
(`.checkpoints/cs4b-content.md`).

**What the one fix pass did, by finding.**
- RR-1 (the tie at plus and minus 2^-24): `crates/content/src/number.rs`, `even_of_a_tie` ends with a read-back
  (the even text is taken only if it parses back to the number); unit cases by bits; the corpus generator
  (`packages/client/scripts/corpus-texts.mjs`) now writes 2,000 ties over 1 to 25 binary places and every power of
  two with its two neighbours, both signs: `content-corpus.txt` holds 20,360 numbers (13,067 before).
- SA-1 (a string grew by doubling): `json.rs`, `string()` measures the decoded length first and takes exactly
  that; a fifth limit case, "a bundle of fifteen texts of half a megabyte and a byte" (native counting allocator,
  Node, browsers). **New in this pass, not asked by a finding:** the limit cases are made one at a time
  (`LIMIT_CASES` and `limit_case(at)` in `crates/devtools/src/content_proofs.rs`, where `limit_cases()` built all
  of them first): with five cases held together the test module's memory reached 72 MiB, over the 64 MiB ceiling
  of `content.test.mjs`; made one at a time it is 49 MiB, as before the pass.
- SA-2 (readers around the plain read): `crates/devtools/tests/content_proofs.rs` reads through
  `devtools::files::read_plain` and writes the verdicts through `write_plain`, switch value "1" only;
  `packages/sim-worker/boundary/content.test.mjs` and `packages/e2e/tests/rules/content.spec.ts` read through
  `readPlainFile` (the spec hands the browser the text it read, no path).
- SA-3 (Windows device names): `json.rs`, `is_id`; the generator's class "an id that is no id" holds three of them;
  **the generator's own valid ids** (`random_id`) keep off them by a rule written apart from `is_id` (the committed
  seed draws `aux` once: the classes failed until the generator knew the rule).
- SA-4 (the echoed quote): `error.rs`, `Echo::of`; `record()`'s doc says it is uncapped and for tests and tools.
- EA-1 / SA-5 / RR-2 (the lints hold only by hand): `crates/sim/tests/rules_scan.rs` gains
  `the_content_crate_keeps_its_rules` (hashed maps, clocks, per-target code, `#[path]`/`include!`, and
  `.unwrap(` / `.expect(` / `panic!` / `unreachable!` / `todo!` / `unimplemented!` outside `#[cfg(test)]` items; not
  the simulation's float list), nine planted breaches and six controls, and it asserts the lint guard's lines are
  still in `lib.rs`. No CI file is written by this chat (Mustafa's rule): a clippy step and `-p content` in the
  job's release line are requests for the CI Builder, recorded as follow-ups.
- EA-2 / EA-5 (the map's shape): `map.rs`, `Form::Ramp { rises_toward: Toward }`; `group` is gone.
- EA-3 / RR-3 (the dependency rule): `crates/devtools/tests/dependency_rules.rs` reads dotted keys before the
  first table, asserts no build script, and says in a comment what step 9's ts-rs dev-dependency will need.
- EA-4, EA-7, EA-8: wording in `crates/sim/src/time.rs`, `crates/content/src/time.rs`, `error.rs`.
- SA-6 / RR-4 (the plain read and write): `crates/devtools/src/files.rs`: one name check for both, the read opens
  the file once, checks the handle's own type and reads at most the cap; `tests/recordings.rs`'s scan follows.
- SA-7 (a time in WebAssembly; "four" classes are three; the ranges scan): `content.test.mjs` asserts 5 s
  (release) and 60 s (dev) for the inputs at the caps (measured 218 ms and 3,261 ms), the browser spec 60 s for
  its whole run; `map.rs`'s test walks the map's ranges for finiteness.
- RR-5 and nits: the SAFETY note in `tests/content_fuzz.rs`, the fold's comment and threshold, "the slowest by the
  clock", `saturating_mul` in `time_report.rs`.
- Not fixed, follow-ups for STATUS (say if one should not wait): EA-4's re-export at step 6; EA-6 (facing zero
  pinned at step 8); departure 9's condition (4c refuses by name a map with fewer spawns than slots); RR-2's CI
  steps (the CI Builder); the fuzz run's blind spots (the report states them).

**The plants script** (`plants-4b1.py`) has five new plants, a to e (the tie's read-back removed; `String::new()`
for the exact size; a device name let through; the echo's quote; a panic call in content code), and W follows the
new read. Its whole run is repeated on the final head by the main session: do not run it (it edits source files).

**Your final answer:** at most 20 lines: for each of YOUR round-1 findings "closed" or "still open, because";
then anything the fix pass broke or left half done, as `severity | file:line | what | evidence | fix`; then what
you did not check. A finding below High becomes a STATUS follow-up, so say for each new one whether it can wait.
