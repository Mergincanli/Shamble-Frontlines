# engine-architect, review of change-set 4b-1 "Content", round 2

Head reviewed: code head `d36e416` on `cs/4b-content` (worktree `.gate/lane1-takeover`); round 1 read `ef2edec`.
Brief: `.checkpoints/cs4b-content/review-brief-4b1-round2.md` (+ round 1's "Rules for you"). Reads ONLY
`git diff ef2edec d36e416 -- crates packages`. Notes only; no source edits; target folder
`.gate/engine-architect-4b1-target`; scratch copies only under `.gate/engine-architect-4b1-scratch/`.

## Plan
1. Confirm the tree: git status, git log ef2edec..HEAD, diff --stat of crates packages.
2. EA-1: read the new `the_content_crate_keeps_its_rules` in `crates/sim/tests/rules_scan.rs`; judge placement (sim's
   tests scanning `../content/src` by path) and whether it holds what sheet step 2 planned (reread-sheet.md:156,
   :75-76, test-list.md:50-51). Not repeat rust-reviewer's round-2 gaps.
3. EA-2 / EA-5: `crates/content/src/map.rs` diff: `Form::Ramp { rises_toward: Toward }`, `group` dropped. Check the
   file format is unchanged (reader keys, writer if any, proofs/vectors), and how ts-rs 12.0.1 would render it at
   step 9 (registry source in ~/.cargo if present, offline).
4. EA-3: `crates/devtools/tests/dependency_rules.rs` comment on step 9's dev-dependency; check the rule's code.
5. EA-4, EA-7, EA-8: wording in `crates/sim/src/time.rs`, `crates/content/src/time.rs`, `crates/content/src/error.rs`.
6. EA-6: what the step-8 facing line should state (map.rs's doc of `at` as pinned now).
7. New: `LIMIT_CASES` / `limit_case(at)` in `crates/devtools/src/content_proofs.rs` and `random_id`'s device-name
   rule: boundary / layering (wasm.rs exports, who calls them, duplication of is_id's rule).
8. Runs (own target, --locked, offline): content + devtools lib + dependency_rules + time_unit + sim rules_scan;
   clippy if cheap; players' Wasm unchanged if cheap.

## Progress
- [ ] 1 tree
- [ ] 2 EA-1
- [ ] 3 EA-2 / EA-5
- [ ] 4 EA-3
- [ ] 5 EA-4 / EA-7 / EA-8
- [ ] 6 EA-6
- [ ] 7 new code
- [ ] 8 runs

## Findings (as made; severity | where | what | evidence | fix)
