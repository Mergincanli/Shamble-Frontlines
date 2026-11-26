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

- [x] 1 designs read (block 1, my read, sheet C-E, card 9-21)
- [x] 2 dependency rules read: content manifest has no dependency table; devtools = content, protocol, sim, sim-wasm,
  wasm-bindgen (client-endpoint, room dropped); sim and sim-wasm manifests unchanged; sim/src/time.rs = 2 consts + a
  const assert; the only players'-crate change is sim/src/lib.rs `pub mod time;` (git diff --stat).
- [x] 3/4 formats and refusals read (bundle, map, movement, read, error, json head, write head, time)
- [x] 5 boundary read (wasm.rs 4 exports, content.test.mjs, content.spec.ts, plain-file.ts, node-names.d.ts, scripts):
  nothing shipped imports plain-file.ts or the scripts (grep over packages/, excl. node_modules); sim-worker's
  package.json exports only src/index.ts and src/worker.ts; CI type-checks plain-file.ts only through the e2e specs
  line (toolchain.yml:200), never in the Worker's lines (:196-197).

## Findings (as made; severity | where | what | evidence | fix)
F1 Medium | crates/content/src/map.rs:97,103-106 | `Shape.form: Form` with `Form::Ramp(Toward)` (a newtype variant over a
  unit enum) cannot be rendered by ts-rs as the file's own shape (`"type": "ramp", "rises_toward": "+x"`): ts-rs 12.0.1
  renders an internally tagged newtype variant as `{ "type": "Ramp" } & Toward` (ts-rs-macros-12.0.1/src/types/enum.rs:
  186-208), i.e. an object intersected with a string union, and no `rises_toward` key. Step 9's generated types (the
  page's `JSON.parse` of the map is typed by them; sheet step 9, my 1.8) would be wrong or need hand types. Not a file
  format change (the reader defines the format), so no version step; cheapest now (callers: map.rs tests,
  content_proofs.rs:366,411-412). Fix: `Form::Ramp { rises_toward: Toward }`; at step 9 `#[ts(tag = "type",
  rename_all = "lowercase")]` on Form, `#[ts(flatten)]` on `Shape.form` (named.rs:86-89 flattens an enum as a union),
  per-variant renames on Toward ("+x"...) and Ground. Or record it in 4b-2's plan for step 9.
F2 Low | crates/devtools/tests/dependency_rules.rs:42-46,66 | the rule refuses every table but [package] and [lints],
  and its plant is exactly `[dev-dependencies] ts-rs`, the dependency step 9 adds to content for the generated types
  (sheet step 9 and section F; the orphan rule leaves no other home for `derive(TS)`). Stricter than the sheet's "content
  has no normal dependency" (sheet F) and not listed as a departure; 4b-2 must loosen a guard test. Fix: say in the
  rule's comment that step 9 admits exactly `[dev-dependencies] ts-rs = { workspace = true }`, or narrow the rule now
  to normal, build and target dependencies, features, build and links, and plant another crate.

## Runs (my target folder .gate/engine-architect-4b1-target; --locked --offline)
- `cargo test -p content -p devtools --lib --test dependency_rules --test time_unit --test aim_reach`: content 35
  passed, devtools lib 15, aim_reach 1, dependency_rules 2, time_unit 4; exit 0.
- `cargo tree -e normal,build`: content has no dependency; sim-wasm's tree = client-endpoint, protocol, room, sim,
  wasm-bindgen (no content); devtools = content, protocol, sim, sim-wasm, wasm-bindgen; `cargo tree -i content -e all`:
  devtools is content's only user.
- `git diff --stat origin/main HEAD -- crates/{sim,sim-wasm,protocol,room,client-endpoint} Cargo.toml
  rust-toolchain.toml`: only crates/sim/src/lib.rs (+1) and crates/sim/src/time.rs (+17).
- The players' crate, raw: `cargo build --release --target wasm32-unknown-unknown -p sim-wasm` at head and in a scratch
  copy without `pub mod time;` and time.rs (.gate/engine-architect-4b1-scratch/ws, target .../scratch-b): both
  79,068 bytes, sha256 a56b0e8b603f7a5de92fa98820d0b340681495d64970785829a7f319555bcebe. The sim's time module changes
  no byte of the players' Wasm (raw cargo output, without build.mjs's remap flag and wasm-bindgen; equal settings).

F3 Medium | crates/sim/tests/rules_scan.rs:350 (unchanged) | the sheet's step 2 "Changes" column (reread-sheet.md:156:
  "its hashed-map, clock and unwrap rules also read crates/content/src"), section C (:75-76 "no hashed map anywhere in
  the crate (the scan's rule is extended to it)") and test-list.md:50-51 (T-should 2) put the scan over content; the
  diff does not touch rules_scan.rs (git diff --stat), the scan reads only sim's src (:350), it has no unwrap rule
  (grep), the plants script has no plant for it (plants-4b1.py:39-88), and the 13 departures do not list it. CI runs no
  clippy (no `clippy` in .github/workflows; local-checks.sh:12 "not in CI's job: run by hand"), so content's
  `deny(clippy::unwrap_used ...)` (content/src/lib.rs:13-24) and lib.rs:10's "the lints below hold that" hold only by
  hand, and nothing in CI keeps a hashed map or a clock out of a crate that enters the simulation's tree at step 6.
  Today content has none (grep HashMap|HashSet|Instant|SystemTime: 0). Fix: extend the scan as planned (hashed-map and
  clock rules over crates/content/src, an unwrap/expect/panic rule outside cfg(test), one plant each), or record the
  departure with its reason and move it to step 6.
F4 Low | crates/sim/src/time.rs:1-3, crates/devtools/tests/time_unit.rs:20-25 | the two copies of the unit are the
  right temporary shape (content cannot depend on sim; sim may not depend on content before step 6; the tie runs in
  CI's `cargo test --workspace`; measured green), but what replaces it is said only in the sheet's step 5 row
  (reread-sheet.md:159: re-export, the test goes), a checkpoint file deleted at 4b-1's close (CLAUDE.md checkpoints
  rule 6). sim/src/time.rs says "until this crate depends on that one" and no more. Fix: one sentence there (and a
  STATUS follow-up in the docs pass): "at 4b-2's step 6 this becomes `pub use content::time::UNITS_PER_SECOND;`, the
  first test of devtools/tests/time_unit.rs goes, the const assertion stays".
F5 Low | crates/content/src/map.rs:90-98 | `group` is a required key (map.rs:165) with no reader in 4b-1 and none named
  for 4b-2 or 4c (grep of plan.md, the sheet, the test list: only my own design 1.6 at engine-architect-cs4.md:205,
  "grouped street and yard"); EDITOR_V2's "tagged group" (:82-84) is the stamps' feature, another thing. A required key
  with no need is a "for later" field, and taking it out after the first file is a breaking change (CD-4: a version
  step and a migration). Fix: drop it now (no file exists), or name its reader. My design's mistake, not the build's.
F6 Low | crates/content/src/map.rs:56-59, :84-88 | two meanings the first map file needs are not pinned: which way
  `facing_deg` 0 points and which way it turns ("the simulation says ... change-set 4c"), and which point of the
  soldier `at` names (the feet?). 4b-2's step 8 writes test-scene.json by hand before 4c decides (sheet step 8); no
  convention exists yet (grep facing|yaw|heading in crates/*/src and ADR 0004: 0). Not a version step, but data written
  under a meaning chosen later. Fix: pin both in the doc comments now, in the map's own axes (y up), or make step 8
  pin them before the first spawn is written.
F7 Low | crates/content/src/time.rs:6-8 vs movement.rs:5-6,21-29 | content::time says "a content kind that holds a
  duration converts it here, once, when its file is read; the simulation counts units and never converts", while the
  movement kind in this same change-set keeps four durations as seconds (`speed_up_time_to_90pct_s`,
  `stop_time_to_10pct_s`, `sprint_time_from_full_s`, `stamina_refill_time_s`) and says "the simulation works out its own
  constants from these once, when a match starts". 4c will read one of the two as the rule. Fix: one wording, e.g.
  "a duration the simulation counts as a timer is converted with `units_from_seconds` once, before the match starts".
F8 Low | crates/content/src/error.rs:205-207 | the doc says line and column are 0 "for the whole document", but 0:0 with a
  path also means "this field, no place known": MissingKey (read.rs:144), IdNotFileName (bundle.rs:133-140), every map
  rule (map.rs:219-222). The record and the sentence are fine (message() leaves out a 0 line, error.rs:270). Fix: the
  doc says "0 and 0 when no place in the text is named: the whole document, or a refusal that names only a field".
- clippy `-p content -p sim --all-targets -- -D warnings`: exit 0 (the lints hold today, by hand).

## Point 3, the refusals (verdict: sound)
- One `Error` record on every target (record() fixed order, doubles as hex); message() ASCII, <= 512 bytes, safe as
  text on a page; the Worker posts one string, so `Kind` never needs to cross to TypeScript (sheet step 9 exports the
  documents' types, not the error's). Good boundary.
- The split is right: the map's rules run on the typed document, which has no places; that is what lets 4c call
  `check` on a map built in code and keeps `read` and `check` apart. A canonical file has one key a line, so a field
  path finds its line. If lines are wanted later, `load_file` still holds the tree when `map::read` calls `check`
  (bundle.rs:126-131), so a path-to-place lookup needs no format or record change. Only the doc of `Error` is off (F8).
- The record's text leans on derived Debug for most variants (error.rs:319-320) and the generator's expectations are
  written in that text (content_proofs.rs:497-522): fine while the toolchain is pinned (rust-toolchain.toml 1.99.1);
  a bump that moved derived Debug would fail natively first, loudly. Not a finding.

## Point 4, the boundary (verdict: fine)
- Four exports on the test-only module only, each one call per proof, a string in and a string out; the players'
  file has no new export and no changed byte (measured, Runs). plain-file.ts in sim-worker/boundary used from e2e:
  tests depend on tests, as determinism.spec.ts already imports sim-worker/src/recording.ts; not exported by the
  package (package.json exports), not imported by src/ (grep), type-checked through the specs line only. node-names.d.ts
  gains one name by its own rule. The scripts' home is the one my read chose (client's `vitest run` picks up
  scripts/*.test.mjs, no CI edit); nothing shipped imports them; the vectors belong to devtools' proofs.
- Note for 4b-2: my read planned `content_limits()` to give the caps for step 9's TypeScript 8,388,608 tie; the built
  `content_limits` runs the cap inputs instead, so the tie takes another route (a generated constant beside the types).

## Point 5, the departures
1 fine: devtools is where the test-only module's code lives; a generator in content would need a feature table, which
  the dependency rule refuses. 2 fine: one crate, a 50-line type; it moves when the schema writer adds columns (that
  writer will also need the map's structure, which map::read holds as code, not a table: a 4b-2/4d cost, no format
  change). 3 fine (counts are test-engineer's). 4 fine: the browsers run what ships. 5 fine: evidence, no runtime code.
6 fine: a clearer refusal. 7 fine: the writer now agrees with V8 and SpiderMonkey on ties (corpus, Node, Chromium,
  Firefox CI). 8 fine and the right direction: a strict grid can be loosened later with no version step (every old
  file stays valid); tightening later would be one. 9 fine: the content does not know the mode. 10 fine (point 3),
  doc fix F8. 11 fine: a reviewed data file is easier to review than a constant and sits with the extract it governs;
  FIXED_DT as `step` is recorded as the builder's call; shot counts are change-set 8's. 12 fine for the boundary (two
  helpers of ~40 lines, every reader through them, a scan that holds it) provided the report gives code and evidence
  apart beside his "about 15". 13 fine.

## Point 6, the "blocks sight" flag (view only)
- Agree: not in 4b. A field with no reader is a for-later field; Slice 2's sight check is its first reader.
- 4b-2 must keep: one collider per shape (never merged, never split without a link back), the shape's index in the id
  order on every leaf, and blocking kept out of the tree's structure (one tree, a per-leaf test later), so the flag and
  ADR 0004 part 14's mid-match flips of a collider's state (adr 0004:438-444) need no rebuild.
- Every map written before the flag means "every shape blocks movement, shots and sight", so the later default or
  migration is "all". CD-4 (ADR 0005:358-360) also allows the field as an additive change with a default and no version
  step; the 4b-1 reader has no default yet, so Slice 2 picks either route, both small while only test-scene.json exists.

## Notes for 4b-2 (no finding)
- The bundle has no Rust struct, so step 9 has no ts-rs type for content.json's outer shape (hand type or a struct).
- content's `f64::round` (map.rs:228, time.rs:33) enters the players' file at step 9; step 7's libm rule must allow it
  (an exact function: identical on every target) or the code rounds in integers.
- Byte parsing only in composition roots (my read, answer 3): nothing stops sim calling content::load once it depends
  on content; a scan line at step 6 could hold it if wanted.

## Not checked
json.rs, number.rs, write.rs line by line (rust-reviewer); the threat model (security-auditor); whether the evidence
proves the marks, the Node boundary and browser runs, the fuzz run (test-engineer); the shipped file after
wasm-bindgen (my compare is cargo's raw output under equal settings).

## Status: DONE (2026-11-26). Verdict: findings to fix, 2 Medium (F1, F3), 6 Low (F2, F4-F8); departures all fine.
