# 4b, "Content and the world": the sheet for the advisor agent's re-read (2026-11-26, main 2483e9a)

The plan is `.checkpoints/cs4-walk/plan.md`, revision 4, section 5 (with 3, 9, 11, 14, 17), approved by the advisor
agent and by his go of 2026-11-25. Section 3, item 1 asks that each later part starts only after the advisor agent
has read the plan again against that day's main and after his answers that part needs, and that the re-read carries
the part's exact symbols to add and to remove. This sheet is that: what moved since the plan, his answers applied,
the symbols step by step, the manifest batch, and what I ask the advisor agent to rule on. Nothing is edited yet.

## A. What moved since the plan
- **Main is 2483e9a**: 4a is merged (pull request #70). 4a left for 4b (STATUS, "From change-set 4a"): 4A-NA-4B and
  4A-EA-4B (content through the constructor only; `first-loop` made anew with its frozen content set while every
  hash line stays byte for byte; `sha256.rs` moves whole into the content crate if the players' file must compute
  the content's SHA-256: it need not in 4b, the bundle's hash is change-set 8's).
- **The hash has no buffer** (4a's fix pass): `World::shared_hash(&self)` and `state_hash(&self)` feed FNV-1a from
  the visitor. 4b adds no state, so no hash line changes; the static world is not state (ADR 0004 part 3).
- **The root `Cargo.toml` is not protected by the guard today** (`protectedRel` in `.claude/hooks/routine-guard.mjs`
  names no manifest): section 14's "risk tier 2 if the guard protects it by then" does not apply; the manifest batch
  is risk tier 1, with security-auditor's dependency check before anything is built or pushed (his DEP-HOLD).
- The devtools crate's three path edges of 3b (`client-endpoint`, `protocol`, `room`) are still unused at 2483e9a
  (docs-keeper's check at fb5d0ff; I re-check with `cargo tree`'s input before the batch): dropped in the batch.

## B. His answers of 2026-11-26 (`.checkpoints/cs4-walk/mustafa-2026-11-26-go-merge-70.md`), applied to 4b
1. **CONTENT-SOURCE (c):** "all our own. Keep the reader to what the content files need and refuse everything
   else, duplicate keys included. Fuzz it before the first value rests on it." Applied:
   - No outside package enters for content: no serde, serde_json, garde, schemars. The only new outside package of
     4b is the collision library with its tree. (ts-rs stays a test-only dependency of the content crate for the
     generated types, as it is for `protocol`: never in a shipped build; said to the advisor agent as a question
     below.)
   - **The reader's grammar is cut to what the content files need (my proposal, for the advisor agent's ruling and
     engine-architect's review):** objects, arrays, strings and numbers only. Refused by name: `null`, `true` and
     `false` (no content field of 4b or 4c is a boolean or nullable; a later kind that needs one adds it with its
     test), duplicate keys, a byte-order mark, a trailing comma, comments, NaN and Infinity, a leading zero, depth
     beyond 32, any byte above 0x7E or below 0x20 inside a string, and every string escape except the three the
     bundle's own text needs (`\"`, `\\`, `\n`: a canonical file has no tab and no carriage return). Numbers keep
     the whole RFC 8259 grammar, because the reader's proof (content item 1, PM-0005-2) parses 12,038 numbers
     with exponents and extremes to the same bits as `JSON.parse`.
   - **The fuzz run comes before any value rests on the reader:** step 3b (10,000 broken files refused the same
     way natively and in WebAssembly; a bounded run of 1,000,000 random and mutated inputs with no panic) is green
     before step 6 commits the first content file and before step 7 hands content to the simulation. The plan's
     order already has 3b before 6 and 7; the sheet makes it a rule: no file under `content/` (but
     `content/source/`, which the game never loads) is committed before 3b's run is in the notes.
2. **DEPART-4 (a), two notes.** Entry 5 ("the 48 is one named constant"): 4c's block; 4b names nothing with 48.
   Entry 6 ("until the aim codec widens, the loader refuses a playable map larger than its reach, and STATUS names
   the change-set that widens it"): in 4b's map kind, stage 6 (whole-set checks): the map's `bounds` must lie inside
   the aim point's reach as `protocol` gives it, x and z within plus or minus 327.67 m and y within 0 to 655.35 m,
   read from one pair of constants beside `AimPoint` (so the rule moves when the codec does), refused as
   `MapBeyondAim { file, axis, value, reach }`; a test plants a map 1 cm too wide. **Which change-set widens the
   aim codec is not named in any design** (question 1 below).
3. **EARLY-LOOK-ORDER (b):** inside 4c only. Nothing of 4b moves; the schema writer and the Testlines tool's
   report-only modes stay in 4b (unless 4b's own ladder fires).
4. **LINKS-CHECK (c):** "about 15 lines in 4b". `devtools::files::plain` (the check `files::check` already uses)
   becomes the one way the Rust tests read a recording (`tests/recordings.rs`, `two_processes.rs`,
   `two_instances.rs`, `hash_cost.rs` if it reads one), and the two TypeScript readers of recordings
   (`boundary/replay.test.mjs`, `e2e/tests/replay/determinism.spec.ts`) refuse a path whose `lstat` is not a plain
   file before they read it; one test per language plants a link. The repository-wide check stays 4A-S3-READ's
   follow-up for step 2.3.
5. **FIREFOX-REPLAY (a):** his, to the CI Builder chat. This chat writes no CI or test-setup file for it.
6. **His Export at 30 steps** comes before step 6; DIFF-LIST is printed from it by the notes' script and asked then.

## C. The steps, with what each adds and removes (names from the designs; the plan's sizes by the designs' count)
| Step | Adds | Removes or changes |
|---|---|---|
| 0 manifests (about 110 lines of configuration) | `crates/content/Cargo.toml` (no dependency; dev-dependency `ts-rs`, workspace); root `Cargo.toml`: `parry3d` in `[workspace.dependencies]`, one `overflow-checks` block per runtime package of the tree; `crates/sim/Cargo.toml`: `content`, `parry3d`; `crates/sim-wasm`, `crates/devtools`: `content`; `Cargo.lock` | `crates/devtools/Cargo.toml`: the three unused path edges. Section D has the exact text |
| 1 built-file check (340 / 320) | `build-checks.mjs`: `wasmFunctionNames`, `wasmCodeOpcodes`, `wasmTargetFeatures`, `wasmImports`, `fusedMultiplyAddProblems({ twin, production, imports })`, `bannedMathsProblems`, `linkedLibmFunctions` (a report), `nativeTargetProblems` (no `target_feature="fma"` or `"avx2"` in `rustc --print cfg` under the build's environment); `build.mjs`: the call in `buildWasm()` after the bindings tool, on every build; `build-checks.test.mjs`: one planted module per rule and a control each, from a small module writer in the test; `crates/sim/tests/rules_scan.rs`: two rules (the collision library's names only inside `crates/sim/src/collision.rs`; `fma`, `fmaf` and `libm::fma*` refused like `mul_add`); `simFeatureList` and `simFeatureDifferences` compare by name and version (3A-FEATURE-VERSION) | nothing removed; `CARGO_OVERRIDES` gains `ENSURE_NO_PANIC` (the libm build script reads it: rust-reviewer, block 1) |
| 2 reader and writer (790 / 500) | `crates/content/src/`: `lib.rs` (the lint guard from its first line), `json.rs` (`parse(bytes) -> Result<Tree, Error>`, `Tree`, `Node`, line and column on every node), `number.rs` (the number grammar, `parse_number`, `number_text`), `write.rs` (`canonical(&Tree) -> String`), `read.rs` (`Object`, typed getters with field paths), `field.rs` (`Field`, `Range`, the tables), `schema.rs` (`schema(&[Field]) -> String`, JSON Schema 2020-12), `error.rs` (`Error` with file and line and column or field path; `message()`) | nothing |
| 3 bundle and kinds (360 / 290) | `bundle.rs` (`Bundle`, `parse_bundle`, `freeze -> ContentSet`, the caps 1 MiB a file and 8 MiB a bundle, `MAX_DEPTH = 32`), `movement.rs` (`Movement`, format `shamble-movement` version 1, fields with his Export's names and units: section 11 of the plan), `map.rs` (`MapDocument`, `Shape::{Box, Ramp}`, `Spawn`, `Region`, format `shamble-map` version 1, at most 16 spawns, 512 shapes; the aim-reach rule of B.2), `ts_export.rs` (test-only: the types and schemas into `packages/types/src/generated/content/`) | `packages/types/src/index.ts` exports the new types |
| 3b the reader's proofs (20 / 530) | `crates/content/tests/corpus.rs`, `broken.rs`, `fuzz_bounded.rs`; `devtools::wasm::content_corpus`, `content_verdicts` (test-only exports); `packages/sim-worker/boundary/content.test.mjs`; `packages/e2e/tests/rules/` gains the corpus in Chromium | nothing |
| 4 time unit (240 / 80) | `crates/content/src/time.rs` (`units_from_seconds(f64) -> Result<u32, DurationError>`), `crates/sim/src/time.rs` (`UNITS_PER_SECOND: u32 = 3000`, `UNITS_PER_TICK`, with const assertions for 30 and 60 Hz), `crates/devtools/src/bin/time_report.rs` (`--check`) with its reviewed list of the old game's duration paths and the committed table | nothing |
| 5 tools (250 / 140) | `crates/devtools/src/bin/content_bundle.rs` (writes `content.json`; `--check` refuses a file under `content/` that is not byte for byte its canonical rewrite; skips `content/source/` by name and refuses any other folder it does not know), `testlines.rs` (`--check`, `write`, `compare-old`, `table`), each with its core as a library function a plain test runs | nothing |
| 6 data (not counted) | `content/source/`: his new Export, its manifest, `testlines-table.json`; `content/movement/soldier.json` by `testlines write`; `content/maps/test-scene.json` by hand from the Art sheet's sizes and the marks' shapes | the 60-step Export in the notes stays a note; no value of it is carried |
| 7 content to the simulation (290 / 260) | `Sim::new(seed, mode, pad_bytes, bundle: &[u8])` and `Sim::create` with `CreateError::Content(String)`; `LocalMatch::new(setup, ..., &ContentSet)`; `build.mjs` runs `content-bundle` and copies `content.json`; `build-checks.mjs` refuses a page folder without it; `packages/client/src/app/content.ts` (one fetch, the bundle's bytes to the Worker in `start`); `worker-link.ts`, `packages/sim-worker/src/messages.ts` (`start` gains `bundle: ArrayBuffer`), `host.ts`/`worker.ts` | the boundary pin: `Sim`'s constructor takes four arguments; no new export |
| 8 static collision world (260 / 170) | `crates/sim/src/collision.rs` (the one module that names the library: `StaticWorld::build(&MapDocument) -> Result<StaticWorld, WorldError>`, shapes in id order, `Bvh::from_leaves(Binned)`, depth at most 31 refused, `tree_bytes()` for PM-0004-32, `cast_ray`), `crates/sim/src/vec.rs` (`V3`, our own small vector type); tests: the tree's bytes and 10,000 rays the same natively and in WebAssembly | nothing |
| 9 the page's scene (230 / 230) | `packages/client/src/render/scene.ts` (boxes, wedges, regions from the map; greys by the default he can veto), `picture.ts`: `setContent` (map only in 4b; the camera joins in 4c), the readout's content rows; browser tests: the scene appears; a broken bundle shows the loader's sentence | `render/empty-scene.ts` is replaced by `scene.ts` |
| LINKS-CHECK (c) (about 15 lines) | as B.4 | |

Sizes: the designs' own count 5,410; planned (holds the 1.8) 9,750; stop line 12,200. The ladder as the plan fixes
it (9,750: the schema writer with its tests and the schema half of PM-0005-7 and PM-0005-8 to 4d; 11,000: step 9
whole to the start of 4c). The cut grammar of B.1 makes step 2 smaller, not larger (no `\u` escapes, no UTF-8
decoding, no booleans).

## D. The manifest batch (step 0), exactly; uncommitted and unbuilt until security-auditor has answered on it
- Root `Cargo.toml`, `[workspace.dependencies]`: `content = { path = "crates/content" }` is NOT added (the other
  crates name each other by path today; the same here); added: `parry3d = { version = "=0.31.1", default-features =
  false, features = ["dim3", "f32", "alloc", "enhanced-determinism"] }` with a comment naming ADR 0004 part 10 and
  why `std` is off (rust-reviewer, block 1; `"std"` joins only if the library does not build without it for both
  targets, and that is a second ask of security-auditor on the changed content).
- Root `Cargo.toml`: one `[profile.release.package.<name>] overflow-checks = true` block for each package
  `cargo tree -p sim-wasm --target wasm32-unknown-unknown -e normal,no-proc-macro --prefix none` lists after the
  resolve (rust-reviewer's paper list with `std` off: approx, arrayvec, bitflags, bytemuck, byteorder, downcast-rs,
  either, equivalent, foldhash, glam, glamx, hash32, hashbrown, heapless, indexmap, libm, log, num-complex,
  num-traits, ordered-float, parry3d, rstar, serde_core, simba, smallvec, stable_deref_trait, static_assertions,
  wide; thiserror has one) and one for safe_arch (native tests only). The build's own check
  (`missingOverflowOverrides`) decides the list; the paper list is the expectation.
- `crates/content/Cargo.toml`: a package like `protocol`'s; `[dev-dependencies] ts-rs = { workspace = true }`.
- `crates/sim/Cargo.toml`: `[dependencies] content = { path = "../content" }` and `parry3d = { workspace = true }`.
- `crates/sim-wasm/Cargo.toml` and `crates/devtools/Cargo.toml`: `content = { path = "../content" }` (the
  constructor parses the bundle; the tools and the test module's corpus exports).
- `crates/devtools/Cargo.toml`: `client-endpoint`, `protocol` and `room` removed if `cargo check` of today's
  devtools sources still passes without them (checked before the batch is shown to security-auditor).
- `Cargo.lock`: by `cargo metadata --format-version 1` without `--locked`, which adds the new packages and moves no
  locked one (it downloads package files to read their manifests and runs nothing of theirs); every later cargo
  command with `--locked`. Expected: 31 new packages with `std` off (33 with it), each from crates.io with its
  checksum. `libm` is not declared by us in 4b: 4c's easing brings our first own call and declares it there.
- No `package.json` or npm lockfile changes in 4b.

## E. Evidence, reviewers and his check (unchanged from the plan, section 17)
Logic by tests on the real code, each with its planted defect, run by a plants script; the players' file's size and
sha256 at each step; the legacy gate untouched and green; the toolchain job's commands by hand before each push with
code. Before step 1: test-engineer and security-auditor read the test list (not a review round). Reviewers:
rust-reviewer, engine-architect, security-auditor; then test-engineer, graphics-engineer, perf-auditor. His short
browser check (Brave, once Firefox): the scene, "errors: 0", the content rows, "Copy report".

## F. What I ask the advisor agent to rule on
1. **The change-set that widens the aim codec** (his DEPART-4 note on entry 6: STATUS must name it). No design
   names one. My proposal: change-set 7, the one that rebuilds the command path before online play (his DEPART-3
   (a): "the compression must land before online play"; 3B-ACKS and 3B-OUTAGE-256 sit there too), recorded in
   STATUS as a follow-up 4B-AIM-REACH with the loader's rule. If that is his to decide, say so and I ask him.
2. **The cut grammar of B.1**: is it the right reading of "Keep the reader to what the content files need and
   refuse everything else", and may it stand before engine-architect's review of the code, or does engine-architect
   read it first?
3. **ts-rs as the content crate's test-only dependency** for the generated types: within "all our own" (it is the
   workspace's existing test-only tool, never in a shipped build), or should the types be written by our own
   schema writer's sibling?
4. **The manifest batch of D**, as the content security-auditor is asked about: complete, and nothing in it that
   4b does not use.
5. Anything in section 5 that today's main or his answers make untrue.
