# 4b, "Content and the world": the sheet for the advisor agent's re-read, revision 3 (2026-11-26, main 2483e9a)

**Revision 3 = revision 2 with the advisor agent's second verdict applied** (`advisor-reread.md`, "Re-read 2":
steps 1 to 5 APPROVED with conditions A1 to A7, none needing another look; steps 6 to 12 HELD for his
FEATURE-COMPARE answer and a short re-read then, with B1 to B7 fixed here already). Steps 1 to 5 may start once he
has answered SIZE-4B (whichever answer: every option builds them first) and security-auditor has answered on step
1's own path-only batch (A7). **His words "my Export at 30 steps comes before step 6" mean the plan's data step,
which is step 8 in this sheet's numbering.**

Revision 1 got: step 0 approved under conditions C1 to C7; steps 1 to 9 to revise (R1 to R11)
(`advisor-reread.md`). This revision answers each R, takes in the three reads the re-read asked for
(`netcode-architect-read.md`, `engine-architect-read.md`; `rust-reviewer-step0.md` is still being written and
touches only section F), his two messages of today (`cs4-walk/mustafa-2026-11-26-go-merge-70.md`,
`mustafa-2026-11-26-source-go.md`), and the folded test list (`test-list.md`, whose headings carry this sheet's
step numbers since revision 3). Section H says where each R is met.
**No file of 4b is edited yet.** The manifest batch of revision 1 is out of the working tree again (saved byte for
byte under `.gate/cs4b/batch/`; its diff and digests are in the notes).

## A. Two things step 0 found, both with Mustafa now (`question-2026-11-26-feature-compare.md`)
1. **FEATURE-COMPARE.** Your C1 was right, and there is a second difference no design saw: the build's feature
   compare is red with the collision library whatever its `std` is (`deps-feature-compare.txt`). Natively only:
   `safe_arch` (the x86 backend of `wide`, which no feature removes) and `thiserror`'s `default` and `std` (from
   ts-rs in test builds). With `std` on, `default` still differs and six more packages enter the lockfile, one a
   second `hashbrown`. Asked of him: two named exceptions, or the type tests in a cargo command of their own, or
   stop. rust-reviewer is reading the exceptions' exact wording.
2. **SIZE-4B.** By the designs' way of counting, 4b with today's additions is about 7,050 lines (section G), not
   5,410: about 12,700 planned at 1.8 and about 16,900 written at 4a's measured 2.4, against a stop line of
   12,200. Asked of him: one pull request, or two. This sheet is written so that both answers work: the steps are
   grouped as engine-architect's seam draws them (**4b-1**, steps 1 to 5, with no outside package; **4b-2**,
   steps 6 to 12, with the collision library). If he keeps one pull request the order is the same; steps 1 to 5 need
   neither FEATURE-COMPARE nor the library's dependency check (the advisor agent's A7).

## B. His words, applied
1. **CONTENT-SOURCE (c)**, and his second message: "A tool script reads my Export with the platform's own JSON
   reader, the game's reader stays as narrow as you proposed, and the Rust tool proves the written content file is
   canonical. No new package comes in ... The game never loads the Export itself." Section C is the reader;
   section D is every file that is not content.
2. **"Fuzz it before the first value rests on it."** The reader's proofs and the bounded fuzz run (step 3) are
   green before the first file under `content/` is committed (step 8) and before content reaches the simulation
   (step 9). Any change to the reader after step 3 runs step 3 again before the next step that rests on it, and
   the final head's run is in the report (your R2.4).
3. **DEPART-4 (a), two notes.** Entry 5 (the 48 as one named constant): 4c's block. Entry 6: section E.
4. **EARLY-LOOK-ORDER (b)**: nothing of 4b moves. **FIREFOX-REPLAY (a)**: his, to the CI Builder. **LINKS-CHECK
   (c)**: step 4. **SOURCE-GO (a)**: no kit file from this chat; every pull request that changes anything under
   `content/source/` says so in the first lines of its report.
5. His Export at 30 steps before the data step (step 8); DIFF-LIST is printed from it and asked then.

## C. The reader's grammar (engine-architect's read, answer 1; your R2)
- **Bytes.** A document holds only 0x0A and 0x20 to 0x7E. Outside strings only space and line feed are white space.
  Values: object, array, string, number. A document's top value is an object.
- **Refusals, each with its own name:** `RefusedLiteral { word }` (null, true, false, NaN, Infinity),
  `ByteOrderMark`, `Comment`, `TrailingComma`, `TopLevelNotObject`, `TrailingData`, `Empty`, `Unterminated`,
  `Whitespace { byte }` (tab, carriage return), `RefusedEscape { byte }`, `BadKey`, `DuplicateKey`, `TooDeep`,
  `TooManyNodes`, `TooLarge`, `UnexpectedByte { byte, expected }` for the rest.
- **Escapes: exactly `\"` and `\n`.** They exist only because the bundle embeds each file's text; no content
  string of its own may hold a quote, a backslash or a line feed (the key, id and enum grammars), so `\\` is
  refused like `\/ \t \r \b \f \u`.
- **Numbers.** `number.rs` keeps the whole RFC 8259 grammar for the corpus proof (`parse_number`, PM-0005-2), and
  `number_text(x)` is ECMAScript's Number::toString for every finite x. The loader refuses at the token, in this
  order: longer than 25 bytes (`NumberNotCanonical`, unparsed); `NumberGrammar`; `NumberNotFinite` (`1e400`);
  `number_text(x) != text` (`NumberNotCanonical { text, canonical }`: `1.50`, `1E5`, `-0`, `9007199254740993`).
  So no content holds -0; -0 stays a parse case of the corpus. PM-0005-3's "100 % ... written back as the same
  text as its input" is read with two stated exceptions, listed in the report with 4b's other mark readings for
  his acceptance at the drafts' next revision: -0 (written "0"), and every corpus input that is not already
  canonical (the 5,000 decimals), which is written back as `JSON.stringify(JSON.parse(t))`. The separate 2^53 rule goes (such a text is not canonical); a scan checks that every
  number field has finite bounds below 2^53.
- **The whole document is canonical or refused:** after the tree is built, `write::canonical_into` streams against
  the source bytes; the first difference is `NotCanonical { expected, found }` with its line and column. The bundle
  and every file pass through it, and `content-bundle --check` calls the same `content::load_file`, so the tool
  and the loader agree by construction.
- **Keys** `[a-z][a-z0-9_]{0,63}`; **ids** `[a-z0-9][a-z0-9_-]{0,63}`; fixed lists where code branches on the value
  (shape `type`, `rises_toward`, region `kind`, `format`).
- **Caps:** bundle 8,388,608 raw bytes; each file 1,048,576 decoded bytes, counted while decoding; `MAX_FILES` 16;
  `MAX_NODES` 16,384 per document; `MAX_DEPTH` 32; `regions` at most 64. Duplicate keys: when an object closes
  its members are sorted by (key bytes, position) and equal neighbours refused; no hashed map anywhere in the
  crate (the scan's rule is extended to it).
- **The error record:** `Error { file, line, col, path, kind }`; `record()` is one line in a fixed field order,
  doubles as 16 hex digits, and it is what the native and WebAssembly verdicts are compared on, with `message()`
  (cut to 512 bytes; input echoes at most 64 bytes, every byte outside 0x20 to 0x7E and the backslash as `\xNN`).
- One correction to revision 1: the Testlines table maps `stamina.stamina_on` (a yes/no value) to a content field.
  It becomes a row the script asserts (the Export must say yes) and no content field: no field of 4b or 4c is a
  boolean.

## D. Files that are not content: scripts on the platform's own JSON reader (your R1; ruling F.6)
All in Node, in `packages/client/scripts/` (the client package's own test run picks up `scripts/*.test.mjs`; CI's
job already runs it; Python writes another number text: `1e+16` where JavaScript writes `10000000000000000`). None
is under `tools/` (a pull request there can merge without him) or under `content/`.
- `legacy-extract.mjs` (step 2): reads `docs/legacy-tuning.json` (it holds true, false, null and one string that
  is not ASCII) and writes two committed plain-text extracts with `--check`:
  `crates/devtools/vectors/legacy-numbers.txt` (a header line with the tuning file's sha256 and the count 2,038,
  then each number's text as written, from the reviver's source text) and
  `crates/devtools/vectors/legacy-durations.txt` (`<path> <s|ms> <text>`, for a reviewed list of paths that is a
  constant in the script). Rust reads only these two extracts, never the tuning file.
- `testlines.mjs` (step 8): `--check` (the test runs it on the real files), `write`, `compare-old`, `table`. One
  constant write target, `content/movement/soldier.json`; links refused; reads capped; written to a new temporary
  file and renamed; the Export's sha256 pinned in the script; values compared with `Object.is`; it writes
  `JSON.stringify(sortedKeys(value), null, 2) + "\n"`, and `content-bundle --check` (Rust) then judges that file.
  The plan's Rust `testlines` program is dropped.
- 4d's legacy import (PM-0005-14 walks every leaf of the tuning file) is a script by the same rule: said here so
  that 4d's re-read finds it.
- **Who confirms the reviewed list of duration paths** (your R11): game-designer, as a read, before step 4's
  table is committed.

## E. The aim's reach (his DEPART-4 note; your F.1, F.8, C3; two readers disagree on one centimetre)
- The rule is the map kind's own whole-set check, `content::map::check_aim_reach(&MapDocument)`, run from
  `bundle::freeze` on `bounds`; refused as `MapBeyondAim { axis, value, reach }`. It is not in
  `StaticWorld::build`: 4c's simulation-only 2 km test builds a `MapDocument` in code and never meets the loader.
- **The constants live in `content`** (it may not depend on `protocol`): `AIM_REACH_CM: i32 = 32_767` for x and z,
  `AIM_HEIGHT_MAX_CM: i32 = 65_535`, the floor at 0. **I take netcode-architect's symmetric reach** (plus and
  minus 327.67 m: his accepted entry says "327.67 m from the map's origin", and `i16::MIN` has no positive twin,
  which bites crates that deny arithmetic side effects), not engine-architect's exact type edge (-327.68 m). The
  1 cm narrowing on the negative side is stated in the constant's comment. Every bound is a whole number of
  centimetres (`BoundNotWholeCentimetres`), so a client that rounds and clamps in integers never lands outside
  the map, and bounds may equal the reach. The test is `bound == f64::from(cm) / 100.0` with `cm` the bound times
  100, rounded; never the fraction of the bound times 100 (`0.55 * 100` is `55.000000000000007`). Planted: bounds at
  0.55 m and at plus and minus 327.67 m are accepted.
- **The tie test** is `crates/devtools/tests/aim_reach.rs`: `devtools` already has a `protocol` edge with no user
  (declared in 3b), so the test becomes that edge's first user and `sim-wasm`'s manifest is not touched before
  step 9. It builds an `AimPoint` from the limits through each field's own type and checks per axis that one
  past the edge does not fit (32,768 for x and z; -32,769, since -32,768 fits `i16` on purpose and is left out by the
  rule, not by the type; 65,536 and -1 for y): a wider field type makes it fit
  and the test goes red; a field that is no integer does not compile.
- Six planted refusals (minimum and maximum of x and of z, y below 0, y above 655.35) and six accepted bounds.
- **The STATUS line** (your ruling F.1, sharpened by netcode-architect; told to him in one line, his to overrule):
  "4B-AIM-REACH: the loader refuses a playable map beyond the aim's reach (x and z plus or minus 327.67 m, y 0 to
  655.35 m). The aim widens in the first change-set whose map leaves that square (none in Slice 1; Slice 2's plan
  says whether its maps do), at the latest in Slice 3's change-set that sets the command codec for online play
  (PM-0001-51), beside 3B-ACKS and 3B-OUTAGE-256. No numbered change-set holds it yet."

## F. The collision library's batch (steps 6 and 7 of this sheet; your C1 to C7)
As revision 1's section D, with your conditions: the lockfile by `cargo update --workspace` (done once: 33
registry packages at the versions the paper reading named; `serde` and `serde_derive` are locked through
indexmap's never-built edge); the overflow blocks from the WebAssembly tree and the host tree by the build's own
function (28 and `safe_arch`); `std` off unless his FEATURE-COMPARE answer or rust-reviewer's read says otherwise;
the devtools edges: `client-endpoint` and `room` are dropped on the grep's proof (no path of either anywhere in
`crates/devtools`), `protocol` stays (section E); `room` gets no content edge, in 4b or in 4c (it takes
`sim::StaticWorld` and `sim::Rules`; `sim` re-exports `content::{MapDocument, Movement}` for room's tests);
`content`'s only dependency is ts-rs, test-only, and it enters with its first user (step 11). The whole batch
goes to security-auditor with its digests before anything is committed, pushed or built.
**The foundation line's check** (your R5; ADR 0005's line, placed in 4b): a test in `devtools`
(`tests/dependency_rules.rs`) reads the manifests' own text (the rule names direct dependencies only; a
`cargo metadata` without `--no-deps` would need every locked package, and two of them are locked but never
fetched): `content` has no
normal dependency; `sim`'s normal dependencies are `content` and `parry3d`; planted: `protocol` added to
`content`'s dependencies (the plants script rewrites the lockfile offline and puts both files back byte for byte).
Its `content` half lands with step 1, its `sim` half with step 6.

## G. The steps, in order, with what each adds and removes
Sizes are **by the designs' way of counting** (code / tests), with today's additions inside; planned and written
sizes follow the table. Marks of the plan's section 9 are placed on their step (your R8).

**4b-1 (steps 1 to 5): no outside package.**
| Step | Adds | Changes or removes | Marks |
|---|---|---|---|
| 1 the content crate, manifests (about 15 lines of configuration) | `crates/content/Cargo.toml` (no dependency), `src/lib.rs` (`#![forbid(unsafe_code)]`, the lint guard with `arithmetic_side_effects`); `crates/devtools/Cargo.toml`: `content`; `Cargo.lock`: the one path crate; the foundation check's `content` half | `crates/devtools/Cargo.toml`: `client-endpoint`, `room` removed. A manifest change: held until security-auditor's answer on exactly it (his DEP-HOLD) | ADR 0005's foundation line (the `content` half) |
| 2 the reader and the writer (1,180 / 770) | `json.rs` (`parse`, `Tree`, `Node`, line and column, the caps, the key stack), `number.rs` (`parse_number`, `number_text`), `write.rs` (`canonical`, `canonical_into`), `read.rs` (`Object`, typed getters, integer getters that refuse 1.5, -1 and above `u32`), `field.rs` (`Field`, `Range`; no `description` or `default` columns yet: they come with the schema writer), `error.rs` (`Error`, `Kind`, `Echo`, `record()`, `message()`), `bundle.rs` (`parse_bundle`, `write`, `freeze -> ContentSet`, the path rule `^(movement|maps)/<id>\.json$` built from the kind list, paths strictly increasing, stem equals id, folder equals kind, no extra key), `movement.rs` (`Movement`, format `shamble-movement` 1, his Export's names and units), `map.rs` (`MapDocument`, `Shape::{Box, Ramp}`, `Spawn`, `Region`, format `shamble-map` 1, at most 16 spawns, 512 shapes, 64 regions; every box at least 1 cm thick, ramps rising, spawns and shapes inside `bounds`; `check_aim_reach`); `load_file`; `packages/client/scripts/legacy-extract.mjs` with its test and the two extracts | the scan (`crates/sim/tests/rules_scan.rs`): its hashed-map, clock and unwrap rules also read `crates/content/src` | CD-1, CD-2, CD-4, CD-5 on the two kinds; PM-0005-3; PM-0005-5 (a scan of the field tables: every array has a bound); PM-0005-7's Rust half (ranges at the next float up and down) |
| 3 the reader's proofs (20 / 780) | `packages/client/scripts/corpus-texts.mjs` with its test: it writes the committed `crates/devtools/vectors/content-corpus.txt` (10,000 made numbers from a fixed seed: 5,000 random bit patterns as 16 hex digits with V8's `String(x)`, 5,000 decimals of up to 17 digits with their expected text `JSON.stringify(JSON.parse(t))`, -0 and the extremes) and has a `--check`; `crates/content/tests/corpus.rs` (the 2,038 old numbers as written, from `legacy-numbers.txt`, and those 10,000), `broken.rs` (the seeded generator: every class of `test-list.md` with its expected record, at least 500 files a class, 1,000 controls), `tests/aim_reach` in devtools; `crates/devtools/tests/content_fuzz.rs` (the bounded run: 1,000,000 inputs in release with the debug count in the test's name, at least 90 % structure-aware mutations, a per-stage histogram with minimums, the slowest input, seed and index printed on failure); test-only exports `devtools::wasm::{content_corpus, content_verdicts, content_limits}`; `packages/sim-worker/boundary/content.test.mjs`; `packages/e2e/tests/rules/content.spec.ts` (Chromium, and Firefox by today's setup: a Firefox red goes to him before any change) | nothing | content items 1 and 2; PM-0005-1, -2, -4, -6 |
| 4 the links check (15 / 40) | `devtools::files::read_plain(folder, name) -> Result<Vec<u8>, FilesError>` (public; the folder and the file plain files by their own type, a missing file an error, the read bounded); `packages/sim-worker/boundary/plain-file.mjs` (one Node helper) | `tests/recordings.rs:18`, `two_instances.rs:42` and `:44`, `two_processes.rs:113` and `:115`, `files.rs:115` read through it; `boundary/replay.test.mjs:15` and `e2e/tests/replay/determinism.spec.ts:68`, `:70`, `:72` through the helper (read once, digest from the same bytes). Two planted-link tests (Rust under `cfg(unix)`, Node by `symlinkSync`; the TypeScript one fails, never skips, under CI) and a text scan that no reader of a recording bypasses the helpers: these come on top of his "about 15 lines", as evidence, and the report says so | his LINKS-CHECK (c) |
| 5 the time unit, built last in 4b-1 so that its rung moves code not yet written (240 / 100) | `crates/content/src/time.rs` (`units_from_seconds(f64) -> Result<u32, DurationError>`), `crates/sim/src/time.rs` (`UNITS_PER_SECOND: u32 = 3000`, `UNITS_PER_TICK`, const assertions for 30 and 60 Hz), `crates/devtools/src/bin/time_report.rs` (`--check`; reads `legacy-durations.txt`; writes the committed table `crates/devtools/vectors/time-report.txt`); a devtools test that `content`'s 3,000 is `sim`'s (once `sim` depends on `content`, step 6, `sim`'s constant becomes a re-export of `content`'s and that test goes) | nothing | PM-0004-11 |

**4b-2 (steps 6 to 12): the collision library and the world.**
| Step | Adds | Changes or removes | Marks |
|---|---|---|---|
| 6 the library's manifest batch (about 110 lines of configuration) | section F: root `Cargo.toml` (the `parry3d` line, the overflow blocks), `crates/sim/Cargo.toml` (`content`, `parry3d`), `crates/sim-wasm/Cargo.toml` (`content`), `Cargo.lock` (33 registry packages); the foundation check's `sim` half; the feature compare by name and version (3A-FEATURE-VERSION) and whatever his FEATURE-COMPARE answer allows, each exception with a fixture and a near miss | held until security-auditor's answer | ADR 0005's foundation line (whole); PM-0004-74's compare |
| 7 the built-file check (420 / 470) | `build-checks.mjs`: `wasmFunctionNames`, `wasmCodeOpcodes`, `wasmTargetFeatures`, `wasmImports`, `fusedMultiplyAddProblems`, `bannedMathsProblems`, `libmProblems` (any `libm` function outside an allow-list file fails), `walkedStamp`; `build.mjs`: the names, target-features and instruction rules on the twin before the bindings tool (so a planted defect fails on the check's own rule), the instruction walk again and the imports rule on the production file after it (so what ships is walked), both under one stamp, the dev-profile twin always, and `buildPage` refusing a page whose Wasm is not the stamped file; `build-checks.test.mjs`: one planted module per rule and a control each; `rules_scan.rs`: the library's names only inside `crates/sim/src/collision.rs` (exact path, no `pub use` or alias), `fma`, `fmaf` and `libm::fma*` refused; native: a test asserting `!cfg!(target_feature = "fma")`, `"avx2"`, `"fma4"` in `sim`, `content` and `devtools`, and `compile_error!` under the first two in `sim` (not under `simd128`: the instruction walk refuses it and 4d's reading needs it) | `CARGO_OVERRIDES` gains `ENSURE_NO_PANIC`; **no `nativeTargetProblems`** (it could not fail: your R6) | 3A-FMA-CHECK (his MUL-ADD: in the pull request that brings the first outside crate); PM-0004-23's build check |
| 8 the tools and the data (320 / 225; data not counted) | `crates/devtools/src/bin/content_bundle.rs` (writes `content.json` to the path `build.mjs` gives; `--check`; the walk refuses links, FIFOs, unknown folders, a nested `source`); `packages/client/scripts/testlines.mjs` with its test (section D); `content/source/` (his new Export, its manifest, `testlines-table.json`); `content/movement/soldier.json` by the script; `content/maps/test-scene.json` by hand from the Art sheet's sizes, brought to canonical form by `packages/client/scripts/canonical.mjs <file>` (it writes `JSON.stringify(sortedKeys(JSON.parse(text)), null, 2) + "\n"`; the loader refuses a document that is not canonical byte for byte, and `content-bundle --check` judges the result) | nothing. **Needs his Export at 30 steps and his yes to DIFF-LIST.** The report's first lines say that `content/source/` changes | his point 8 (a to g); PM-0002-29 |
| 9 content to the simulation, with the generated types (350 / 480) | `Sim::create` and `Sim::new` take `(seed, mode, pad_bytes, bundle: &[u8])`; `CreateError::Content(content::Error)` (the type loses `Copy` and `Eq`), `message(&self)`, a test of the check order (seed, mode, content, padding); `ts_export.rs` (test-only) with `content`'s dev-dependency ts-rs, the generated types `packages/types/src/generated/content/` and `packages/types/src/index.ts` (their first reader is `content.ts`; 3A-PART5-CLASH is not reached: content's types nest only inside content, under test configuration, as `protocol`'s do); `packages/client/src/app/content.ts` (one fetch; a copy kept before the transfer; `JSON.parse` only after the Worker's `ready`); `start` gains `bundle: ArrayBuffer`, refused unless it is one of 1 to 8,388,608 bytes, before the glue copies it; `build.mjs` runs `content-bundle` and copies `content.json`; `build-checks.mjs` refuses a page folder without it, entries that are no plain files, a `content.json` over 8 MiB; a source scan for `innerHTML` and its kin; **the frozen content of `first-loop`** (netcode-architect's route (i)): `crates/devtools/recordings/first-loop.content.json`, an input written only by `make_recordings --freeze first-loop`, listed in the manifest with its length and sha256; the recording's header (format version 1 stays) carries that sha256 in the content-hash field it already has, and the map id; `Replay::new(recording, bundle)` and every runner check the bundle against the header before `Sim::new`; test-module exports `replay_hashes(rec, bundle)`, `replay_state_at(rec, bundle, tick)` | every caller of the constructor: `sim-wasm/src/lib.rs` (:71-105 and its tests), `local_match.rs` tests, `tests/step_alloc.rs:79`, `devtools/src/replay.rs:91`, `bin/replay.rs`, `bin/make_recordings.rs`, `wasm.rs`, `tests/recordings.rs` (its scratch copy takes the bundle too), `two_instances.rs`, `two_processes.rs`; `worker.ts:19-26`, `messages.ts`, `worker-link.ts`, `boot.ts`; `boundary/sim-wasm.test.mjs`, `replay.test.mjs`, `determinism.spec.ts`; `checkReplayable` refuses a header of zeros. **Proof that loading content changed no state:** `first-loop.hashes` lines 2 to 1,801, `first-loop.vector.txt` whole and the recording's 1,800 step records equal main 2483e9a's, compared with `git show`'s copies; by design line 1's digest, the header's hash and map id and three manifest lines change. Planted: one content byte folded into the shared state at construction fails at tick 1 (the first simulation tick) natively, in Node and in Chromium (never with `make_recordings` run under the plant); a valid bundle that differs in one value is refused by name before `Sim::new` | 4A-NA-4B, 4A-EA-4B; PM-0005-8's types half; the boundary pin: no new export |
| 10 the static collision world (280 / 260) | `crates/sim/src/collision.rs` (`StaticWorld::build(&MapDocument) -> Result<StaticWorld, WorldError>`, shapes in id order, the binned tree, depth at most 31 refused, `tree_bytes()`, `cast_ray`), `crates/sim/src/vec.rs` (`V3`); `CreateError::MapMissing`, `CreateError::World`; `LocalMatch::new(&setup, 0, pad, loopback(), world)` with `static_world()`; the dump with a committed known-answer hash, a map with shapes permuted, a document built in code with -0, the ray classes, hand-made hostile documents; the fuzz extended to `ContentSet -> StaticWorld::build` | `room` untouched (`Room::new(setup, NO_CONTENT)` stays) | PM-0004-32; PM-0004-33 (partial: the cast paths are 4c's; the built-file check's result is recorded again here, where the library is first linked) |
| 11 the schema writer, alone (110 / 130) | `schema.rs`, the field tables' `description` and `default` columns, the export test's schema output into `packages/types/src/generated/content/schema/` (a link named `schema` and a `.json` link refused) | nothing | PM-0005-7's schema half; PM-0005-8's schema half. **The first ladder rung moves exactly this step to 4d** (the plan's own words: the schema writer and its tests, with the schema halves of those two marks) |
| 12 the page's scene (230 / 260) | `packages/client/src/render/scene.ts` (greys by GREY-COLOURS' default, LOOP_V2 section 9: his to veto before this step), `picture.ts`'s `setContent` (the map only; the camera joins in 4c), the readout's content rows, a development hook that lists the drawn objects; browser tests: the scene equals the map; a bundle that `JSON.parse` accepts and the loader refuses shows the loader's sentence as text and an empty scene | `render/empty-scene.ts` replaced | **The second ladder rung moves exactly this step to the start of 4c** |

**Sizes, each named.** By the designs' way of counting: 4b-1 about 3,300 (1,455 code, 1,690 tests, the rest
scripts and configuration); 4b-2 about 3,750; together about **7,050** (the plan's 5,410 and about 1,640 from
today's test list, conditions and the scripts). Planned at 1.8: about 5,950 and 6,750. At 4a's measured 2.4: about
7,950 and 9,000 written. If he takes two pull requests: stop lines of 7,450 and 8,450 (planned times 1.25); the
ladder's two rungs both sit in 4b-2 (step 11 at its planned size, step 12 at planned plus 1,250), and 4b-1 gets one
rung of its own, shown to him with the split: at 5,950 written lines the time unit (step 5, built last) moves whole to 4b-2,
since nothing in 4b-1 rests on it. If he keeps one pull request: the plan's rungs at 9,750 and 11,000, steps 11
and 12 being the last two steps so that a fired rung moves code not yet written (your R7), and the stop line at
12,200 would be reached during step 9 or 10 at 4a's rate. The count is taken at each step's end against that
step's budget and said in the step's summary.

## H. Where each of your points is met
C1: section A.1, with him. C2, C3, C4: sections E and F. C5, C6, C7: section F. R1: section D. R2: section C and
B.2; engine-architect's read is done. R3: step 9's row. R4: netcode-architect's read is done. The dated
pointer of 4b's docs pass carries DEPART-4 entry 3 **as he accepted it** (the plan's own text), then "netcode-architect
confirmed it at 4b's re-read", then netcode-architect's sharper sentence marked as netcode-architect's and not his
("...stay code until change-set 7, which rebuilds the room's command scheduling behind one seam and the client
endpoint's sending; there they become content values within capacities the code keeps, with defaults equal to
today's 8, 8 and 2"). If that sentence should become the departure itself, it goes to him as a question. R5: section F. R6: step 7.
R7: section G. R8: the marks columns; 3A-PART5-CLASH at step 11; the aim rule outside `StaticWorld::build`. R9:
step 5. R10: the sentence "the compression must land before online play" is netcode-architect's wording of
DEPART-3's list, never his words; it is quoted nowhere as his. R11: step 3 (Firefox), section D (game-designer's
read), and the two one-line answers were given to him in the chat.

## I. What I ask of you
1. APPROVE or REVISE for steps 1 to 5 (they can start the moment he answers SIZE-4B with two pull requests; with
   one pull request they wait for FEATURE-COMPARE and the dependency check as well).
2. The one-centimetre choice of section E (symmetric reach).
3. The tie test in `devtools` and not `sim-wasm` (it changes your C3 by using the edge `devtools` already has).
4. Steps 6 to 12: APPROVE now with FEATURE-COMPARE's wording to follow as an addendum when rust-reviewer's read and
   his answer are in, or hold them until then.
