# Brief for test-engineer: is change-set 4b-1 "Content" proven? (its one read, on the head after round 1's fixes)

**Where and the rules.** Read `.checkpoints/cs4b-content/review-brief-4b1.md` first: what the change-set is, its
thirteen departures, and **every rule for you** (no source or doc edits; notes only; git read-only and alone in its
shell call; your own cargo target folder `.gate/test-engineer-4b1-target`, always `--locked`; no page build, no
browser run, no npx or npm; no network; the repository is public; the severity scale; the shape of the final
answer). Head: `d36e416` (round 1's three reviewers read `ef2edec`; what their fix pass changed is listed in
`review-brief-4b1-round2.md`, and their notes are beside it). Your notes: `test-engineer-review-4b1.md`. You read
the WHOLE change-set (`git diff origin/main...HEAD -- crates packages`), since you have not read it before.

**Your question: is it proven?** The plan's test list is `.checkpoints/cs4b-content/test-list.md` (your own read of
it before step 1 is `test-engineer-testlist.md`; the sheet is `reread-sheet.md`, revision 3). For each
item of the list that belongs to steps 1 to 5: is there a test, does it assert what the item says, and could it
pass on a broken loader? Mustafa's words for the reader: "Keep the reader to what the content files need and
refuse everything else, duplicate keys included. Fuzz it before the first value rests on it."

**Look hardest at:**
1. The generator's verdicts (`crates/devtools/src/content_proofs.rs`, `case()` and its 37 classes): is each
   expected refusal worked out apart from the loader, or does any class end up comparing the loader with itself?
   Known and stated: a valid file's numbers are written by `number_text`, and a map's canonical layout goes through
   `json::parse` + the canonical writer (the number corpus and the tree corpus, both made by a browser engine, hold
   those two apart from the crate). Is that enough, and is anything else circular?
2. The committed digests (`crates/devtools/vectors/content-verdicts.txt`): what exactly do they prove natively, in
   WebAssembly under Node (both profiles) and in the browsers (release only: departure 4)? Can the file be
   regenerated to hide a regression, and what stops that (`SHAMBLE_WRITE_VERDICTS=1` by hand; the classes' own
   expected records are not in the file)?
3. The fuzz run (`crates/devtools/tests/content_fuzz.rs`): 1,000,000 inputs, the stage shares with minimums, the
   seed, what it cannot reach (the caps, full map lists, more than one map, long strings; no coverage is measured).
   Are the minimum shares a real guard against a mutator that stops reaching a stage? Is the 50,000-input run that
   CI's debug `cargo test` executes enough on its own, given that the million run only runs in release?
4. The limit cases and the counting allocator (the same file, and `content.test.mjs`'s memory ceiling and times;
   `packages/e2e/tests/rules/content.spec.ts`): bounds loose enough to be meaningless, or tight enough to flake?
5. The map kind's exact rules (`crates/content/src/map.rs`: grids, thickness, bounds, the aim's reach, duplicate
   ids) and `crates/devtools/tests/aim_reach.rs` (the reach tied to the aim point's own number types).
6. The time unit (`crates/content/src/time.rs`, `crates/sim/src/time.rs`, `crates/devtools/tests/time_unit.rs`,
   `crates/devtools/src/time_report.rs` and its committed report): does the report prove what it says about the
   old tuning's 69 durations, and is the rounding rule pinned at its ties?
7. The links check (`crates/devtools/src/files.rs`, `tests/recordings.rs`, `packages/sim-worker/boundary/plain-file.ts`
   and its test): planted links on both sides; is the scan that says "every reader goes through the helper" real?
8. The source scan for the content crate (`crates/sim/tests/rules_scan.rs`, `the_content_crate_keeps_its_rules`):
   new in the fix pass; its planted breaches and controls; what it cannot see.
9. The planted defects (`plants-4b1.py`: 25 from the first run, which caught 25 after two corrections to the
   script's own expected words; five new ones a to e). Do not run it (it edits source files; the main session
   repeats the whole run on the final head). Judge the list: which check of the change-set has NO plant, and which
   plant could be "caught" by the wrong check?
10. What CI runs and what only runs by hand (`.checkpoints/cs4b-content/local-checks.sh` mirrors the toolchain
    job; the content crate's release tests, clippy and the million-input run are by hand today: follow-ups for the
    CI Builder, since this chat writes no CI file).

**Your final answer:** at most 25 lines: PASS or findings to fix, each as
`severity | file:line | what is not proven or wrong | the evidence | the fix`, most severe first; then what you did
not check. One fix pass follows your read and the others' round 2; below High a finding becomes a STATUS
follow-up, so say for each whether it can wait.
