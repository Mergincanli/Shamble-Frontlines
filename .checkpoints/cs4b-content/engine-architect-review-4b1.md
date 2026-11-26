# engine-architect, review of change-set 4b-1 "Content", round 1

Head reviewed: `ef2edec` on `cs/4b-content` (worktree `.gate/lane1-takeover`), main `3b949c1` merged in.
Brief: `.checkpoints/cs4b-content/review-brief-4b1.md`. Notes only; no source edits; target folder
`.gate/engine-architect-4b1-target`.

## Plan
1. Read my earlier designs: `.checkpoints/cs4-walk/engine-architect-cs4.md` block 1, `engine-architect-read.md`, the
   sheet's sections C to E (`reread-sheet.md`), the card (`cs4b-content.md`, steps 9 to 17), `test-list.md`.
2. Dependency rules: `crates/content/Cargo.toml`, `crates/devtools/Cargo.toml`, `crates/devtools/tests/dependency_rules.rs`,
   `crates/sim/Cargo.toml`, `crates/sim-wasm/Cargo.toml`, `crates/sim/src/time.rs`, `crates/devtools/tests/time_unit.rs`.
   Run `cargo tree` (offline, --locked) to see the real graph.
3. Formats: `bundle.rs`, `movement.rs`, `map.rs`, `error.rs`, `read.rs`, `write.rs`, `number.rs`, `time.rs`, `lib.rs`.
   Judge against 4b-2 (static collision world, Sim::create, generated TS types, schema writer step 11, page scene) and 4c.
4. Refusals: `error.rs` (`Kind`, `Error`, `record()`, `message()`), the 0:0 split.
5. Boundary: `crates/devtools/src/wasm.rs`, `packages/sim-worker/boundary/content.test.mjs`,
   `packages/e2e/tests/rules/content.spec.ts`, `packages/sim-worker/boundary/plain-file.ts`, `packages/e2e/node-names.d.ts`,
   `packages/client/scripts/legacy-extract.mjs`, `corpus-texts.mjs`; who imports them.
6. Departures 1 to 13: a verdict each.
7. Point 6 (blocks-sight flag): read ADR 0004 part 13, judge the main session's answer.
8. Run the content tests and the dependency rules test natively (own target folder), clippy if cheap.

## Progress
- [ ] 1 designs read
- [ ] 2 dependency rules
- [ ] 3 formats
- [ ] 4 refusals
- [ ] 5 boundary
- [ ] 6 departures
- [ ] 7 point 6
- [ ] 8 runs

## Findings (added as made)

