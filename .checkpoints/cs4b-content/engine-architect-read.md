# engine-architect: read for 4b's revised sheet (2026-11-26, worktree lane1-takeover, branch cs/4b-content from 2483e9a)

Asked by the advisor agent's re-read (advisor-reread.md: R1, R2, R3, R8; rulings F.2, F.6, F.8). Not a review round.
No agent memory folder exists in this repository (checked .claude/ in both trees).

## Plan
1. Read: reread-sheet.md (done), advisor-reread.md (done; handback rulings R1-R11 not in the file: look for them),
   test-list.md (done), his words go-merge-70 and source-go (done), security-auditor-testlist.md,
   test-engineer-testlist.md, my design block 1 (+3.6, 4.x) in engine-architect-cs4.md, plan.md sections 5, 11.
2. Check main's code: messages.rs AimPoint, sim-wasm lib (Sim::new/create), sim's lib/LocalMatch, room's use of sim,
   devtools vectors folder and its scripts (language, --check), CI jobs that run them, build.mjs feature compare,
   recording/first-loop route, docs/legacy-tuning.json shape, testlines notes scripts.
3. Answer 1 to 6, each saved here as finished.
4. DONE, handback.

## Notes (a line each)
- [read] reread-sheet, advisor-reread (rulings R1-R11/C1-C7 are in its handback, not the file: substance from the task and cs4b-content.md:51-59), test-list, SA and TE reads, question-2026-11-26-feature-compare (FEATURE-COMPARE, SIZE-4B), his words go-merge-70 and source-go, my block 1 (1.1-1.13), plan s5, s11.
- [main] working tree batch: devtools deps = content, sim, sim-wasm, wasm-bindgen (protocol dropped); sim = content, parry3d; sim-wasm = client-endpoint, content, protocol, room, sim; room = protocol, sim (no content). content/src/lib.rs: forbid(unsafe), deny unwrap/expect/indexing/panic/unreachable/todo/arithmetic_side_effects.
- [main] Sim::create(seed, mode, pad) sim-wasm/src/lib.rs:71-85; Sim::new :103-105; LocalMatch::new(setup, first_sequence, pad, link) local_match.rs:52; Room::new(setup, NO_CONTENT) :54; NO_CONTENT local_match.rs:20; Room::new(setup, content_hash) room/src/lib.rs:105 -> World::new(setup) :108.
- [main] AimPoint x i16, y u16, z i16 cm, protocol/src/messages.rs:137-144 ("Provisional as a whole", :132-134).
- [main] CI: toolchain.yml:165-166 names each Python vector check by line (a new Python script = a CI edit); :171 `npm test --workspaces`; client's test = `vitest run` (default include picks scripts/*.test.mjs, as build-checks.test.mjs today). tools/ is in MERGE_PLACES (routine-guard.mjs:84): a tools/-only PR merges by the PR Advisor alone.
- [finding] testlines-cs4.md:90 maps `stamina.stamina_on` (yes/no) to a content field `movement.stamina_on`: a boolean in 4b's movement kind, against the sheet's "no content field of 4b or 4c is a boolean" (reread-sheet.md:31). Proposed: not content; the script checks the Export holds `true` (as step_rate_hz is checked to be 30, testlines-cs4.md:80).
- [main] proposed content keys (walk_speed_m_per_s, speed_up_time_to_90pct_s, gravity_m_per_s2 ...) all fit [a-z][a-z0-9_]{0,63}; the Export's own keys do not (T.moveSpeed, STAM.max, "T.accel (speed-up)"): the Export never meets the game's reader.
- [adr] PM-0005-3 (docs/adr/0005:2299): 2^53+1 "refused at load with the file and field named"; corpus "plus -0" (:2298).
- [measured] node v22.23.5: JSON.parse reviver's 3rd argument gives `context.source` for numbers (inline probe: ["1.50","-0","1E5","1e400"]). legacy-tuning.json: 2,038 numbers, 0 non-canonical (String(Number(s)) === s), longest canonical 20 chars. Saved Export (60-step, notes copy): 351 numbers, 0 non-canonical, 5 booleans, 0 null; values.stamina.stamina_on = true.
- [measured] Python repr(1e16)='1e+16', repr(1e-5)='1e-05' vs JS "10000000000000000", "0.00001": a Python writer is not JSON.stringify's layout. Longest JS canonical text 25 bytes ("-0.0000012345678901234567"). 327.67 == 32767/100, 655.35 == 65535/100, -327.68 == -32768/100 in doubles.
- [main] callers of Sim/LocalMatch (grep): sim-wasm lib.rs:81 + tests :166 :175 :194 :218-220 :243 :253; local_match.rs:52-55 + tests :186 :371 :384; sim-wasm/tests/step_alloc.rs:79; devtools replay.rs:91; worker.ts:26; boundary sim-wasm.test.mjs :40 :55 :65 :96 :97 :113 :120-124 :164; replay.test.mjs :43 :51 :71 :76; determinism.spec.ts:20. Room::new callers room lib.rs:314 :655, room/tests/any_bytes.rs:117 (untouched if room is untouched in 4b).
- [main] recording header holds content_hash [u8;32] (zeros) and map_id (recording.rs:5-11, :50-56); line 1 of .hashes = sha256(.rec) (recording.rs:381).
- [main] 3A-PART5-CLASH = cross-crate nesting only (ADR 0002:353-357 "where one crate's types nest another's"; STATUS_ARCHIVE.md:1444-1447; build-checks.mjs:284-287). Content's types nest only inside content.

## Answer 1: the cut grammar (DONE)
- Bytes: a document is 0x0A and 0x20..0x7E only. Whitespace outside strings: space and LF only (tab, CR refused:
  `Whitespace{byte}`). Values: object, array, string, number. Refused by name: `RefusedLiteral{word}` for null, true,
  false, NaN, Infinity, -Infinity; `ByteOrderMark`; `Comment`; `TrailingComma`; `TopLevelNotObject`; `TrailingData`;
  `Empty`; `Unterminated`; `UnexpectedByte{byte,expected}` for the rest.
- (a) Escapes: exactly `\"` and `\n`. `\\` refused like `\/ \t \r \b \f \u`. Reason: every content string has a grammar
  without a backslash or a quote (keys, ids, enums, format names), so a content file holds no backslash at all; the
  bundle embeds file texts, and JSON.stringify of a text made of 0x20..0x7E and LF writes only `\"` and `\n`. No
  backslash can survive into a decoded file text, so content files' own strings never hold an escape. Plant: a bundle
  whose text holds `\\` -> `RefusedEscape{byte:0x5C}`.
- (b) Numbers: `number.rs` keeps RFC 8259 (`parse_number`, the corpus's function) and `number_text(x)` = ECMAScript
  Number::toString for every finite x, -0 included ("0", as JSON.stringify). The loader's refusal sits in stage 2, at
  the token, in `json.rs`: token longer than 25 bytes -> `NumberNotCanonical` without parsing (25 = longest
  Number::toString output, measured "-0.0000012345678901234567"); grammar -> `NumberGrammar`; not finite ->
  `NumberNotFinite`; `number_text(x) != text` -> `NumberNotCanonical{text,canonical}`. -0: "-0" reads -0, writes "0",
  so it is refused; the loader never holds -0 (revises my 1.4, which wrote "-0"). The explicit 2^53 rule goes:
  9007199254740993 is non-canonical (reads 2^53); exact large integers are refused by ranges (scan: every number field
  has finite bounds below 2^53). Then stage 2b, the whole document: `write::canonical_into(&tree, &mut Compare)`
  streamed against the source, `NotCanonical{expected,found}` at the first differing byte's line and column (key order,
  spacing, final newline). Every document (bundle and each file) must be canonical. `content-bundle --check` calls the
  same `content::load_file` per file, so the tool and the loader agree by construction; the tool adds only its walk.
  Measured: the 2,038 legacy numbers and his Export's 351 are all canonical already.
- (c) Keys `[a-z][a-z0-9_]{0,63}` (`BadKey{key}`; empty key, `__proto__`, `_x`, `9a` refused at the JSON layer; every
  proposed content key fits). Ids `[a-z0-9][a-z0-9_-]{0,63}` (no leading `-`: a stem is a file name). Fixed lists only
  where code branches on the value: shape `type` {box, ramp}, `rises_toward` {+x,-x,+z,-z}, region `kind`
  {street, grass}, `format` per kind; `group` is a name (id grammar), not a list.
- (d) Caps: bundle bytes 8,388,608 on the raw bytes (stage 1, and the Worker before the copy); each file 1,048,576 on
  the decoded text, counted while decoding (never allocate past cap+1); `MAX_FILES = 16` (4b-4d need 3);
  `MAX_NODES = 16,384` per document (largest legal map by its own max_items: 512 shapes x 13 + 16 spawns x 7 +
  64 regions x 9 + ~16 = about 7,360 values); `MAX_DEPTH = 32`; `regions` max_items 64 (PM-0005-5 needs one). Duplicate
  keys: at object close, sort members by (key bytes, position) (a total order: same result for any sort), adjacent
  equal -> `DuplicateKey` at the later one; the sorted member list is kept: the writer and typed lookups use it. A tree
  is parsed, checked and dropped per file: peak = bundle bytes + decoded texts + one tree.
- (e) `Error { file: Option<FileName>, line: u32, col: u32, path: FieldPath, kind: Kind }`: line and col 1-based in
  bytes (ASCII, no tabs), 0/0 for whole-document errors; `path` filled by stage 2 too (the parser keeps a key/index
  stack, depth <= 32), so PM-0005-3's "file and field named" holds for number refusals. Payload doubles stored f64,
  written as 16 hex digits. `Echo` = at most 64 input bytes, 0x20..0x7E raw except `\`, every other byte `\xNN`, "..."
  when cut. `record(&self) -> String`: one line, fixed field order: the cross-target compare (N vs W) compares records
  and `message()`. `message(&self)`: plain sentence, cut to 509 bytes + "..." (ASCII, so any byte is a boundary).

## Answer 2: reading files that are not content (DONE)
- Language: Node, in `packages/client/scripts/`. Why: (1) Node's JSON.parse is V8's, the browser's, the very reader
  the corpus proves the Rust reader equal to; JSON.stringify(sortedKeys(v), null, 2) + "\n" is the canonical text by
  definition, so no number writer of ours is needed; Python's repr is another layout (1e+16, 1e-05). (2) client's
  `vitest run` already runs `scripts/*.test.mjs` (build-checks.test.mjs) inside `npm test --workspaces`
  (toolchain.yml:171): no CI edit; a Python script needs its own workflow line like :165-166 (the CI Builder's).
  (3) Not `tools/` (legacy toolkit; MERGE_PLACES, routine-guard.mjs:84: a tools-only PR merges by the PR Advisor
  alone); not `crates/devtools/vectors/` for the scripts (no npm test runs there).
- (a) `testlines.mjs` + `testlines.test.mjs`: modes `--check` (default in the test, on the real files), `write`,
  `compare-old`, `table`. One constant target `content/movement/soldier.json` from `import.meta.url`; lstat of file and
  folder (no link), reads capped 1 MiB + 1, write to a new temp file (`wx`) in that folder then rename. Reads the
  Export, its manifest and `content/source/testlines-table.json` with JSON.parse; the Export's sha256 pinned in the
  script. Compares by `Object.is` (bits). Writes only finite numbers: a row mapping a boolean is refused.
  `stamina.stamina_on` becomes an asserted row (must be `true`), not a content field (testlines-cs4.md:90 maps it to
  `movement.stamina_on`: a boolean the cut grammar refuses). The Rust side: `content-bundle --check` (a devtools
  `#[test]`) proves soldier.json canonical, and 3 to 5 values pinned by hand are read through the Rust loader. The
  Rust `testlines.rs` bin of the sheet is dropped.
- (b) `legacy-extract.mjs` + test: target `crates/devtools/vectors/legacy-numbers.txt`: line 1
  `shamble-legacy-numbers 1 <sha256 of docs/legacy-tuning.json> 2038`, then each number's text as written, file order,
  from the reviver's `context.source` (measured on Node 22.23.5). `--check` rewrites in memory and compares bytes. Rust
  native (devtools test, read through the plain-file helper), the test module and the Chromium spec read the same lines.
- (c) Same script, second constant target `crates/devtools/vectors/legacy-durations.txt`: line 1 as above with its
  count, then `<path> <s|ms> <text>`; the reviewed path list is a constant in the script. `time_report.rs` (Rust) reads
  the lines (split on one space; text through `parse_number`), writes its committed table, `--check`; the oracle is
  3 x ms from the text's digits.
- (d) 4d's legacy import: a script too (`legacy-import.mjs`), JSON.parse on both the legacy file and the content
  files, Object.is per kept value; the Rust `legacy-import` bin of my 1.1 is dropped. Where its report lives is 4d's.

## Answer 3: step 7's shape (DONE)
- `Sim::create(seed, mode, pad_bytes, bundle: &[u8])`, `Sim::new` the same (constructor, 4 args). `CreateError`: derive
  Debug, Clone, PartialEq (no Copy/Eq); `Content(content::Error)` (7), `MapMissing(&'static str)`,
  `World(sim::WorldError)` (8); `message(&self)`. Order pinned by a test: seed, mode, content, map, world, padding.
- Step 7: `content::load(bundle)` refuses or accepts; LocalMatch unchanged. Step 8: `set.map(LOCAL_MAP_ID)`
  (`LOCAL_MAP_ID = "test-scene"`, a sim-wasm constant: the local setup stays code until modes), `StaticWorld::build`,
  `LocalMatch::new(&setup, 0, pad, loopback(), world)` keeps it, `static_world()` accessor. One signature change each.
- room untouched in 4b (Room::new(setup, NO_CONTENT); the join keeps NO_CONTENT, the bundle hash is cs8's). C4 for 4c:
  Room::new takes sim's own types (`sim::StaticWorld`, `sim::Rules` made by sim from `content::Movement`); sim
  re-exports the document types its constructors take (`pub use content::{MapDocument, Movement}`) so room's tests
  build them in code; no content line in room's manifest, normal or dev. Byte parsing only in composition roots
  (sim-wasm, devtools, later the server binary).
- Rust callers: sim-wasm lib.rs:71-105 + tests :166 :175 :194 :218-220 :243 :253; local_match.rs:52 + tests :186 :371
  :384 (step 8); tests/step_alloc.rs:79 (step 8); devtools replay.rs:91 (`Replay::new(recording, bundle)`),
  bin/replay.rs, bin/make_recordings.rs, wasm.rs replay exports (+ `bundle: &[u8]`), tests recordings.rs,
  two_instances.rs, two_processes.rs.
- TS: worker.ts:19-26 (`start(padBytes, bundle)`, `new Sim(seed, MODE_LOCAL, padBytes, new Uint8Array(bundle))`);
  messages.ts:12 and :121-122 (`bundle: ArrayBuffer`, instanceof, 1..=8,388,608 bytes, before the glue copies);
  worker-link.ts:211-212 (transfers it); boot.ts:87 (after content.ts's fetch); tests worker-link.test.ts:54-56,
  messages.test.ts:14-15 and :34-38, host.test.ts:196, sim-wasm.test.mjs (every `new Sim`), replay.test.mjs :43 :51
  :71 :76, determinism.spec.ts:20. The TS 8 MiB tied to `content::MAX_BUNDLE_BYTES` through a test-module export
  `content_limits()` (devtools::wasm), never the players' file.
- Frozen content: devtools owns `crates/devtools/recordings/first-loop.content.json` (content-bundle's exact output at
  recording time; a manifest.txt line with bytes and sha256; outside content/, so no content change moves it).
  make_recordings writes it only when making a recording; every replay (native, test module, Node, Chromium) reads it
  through the plain-file helper and hands it to the same 4-argument constructor: no new export on the players' file.
  Whether the header's content_hash holds its sha256 is netcode-architect's; if so devtools::sha256 (native and test
  module) and node:crypto compute it, the players' file none (sha256.rs stays in devtools in 4b).
- Other Rust tests needing a valid bundle: `content::bundle::write(&[(path, text)])` (the format's own writer, which
  content-bundle also calls) over include_str! of the two committed content files; Node boundary tests: the frozen one.

## Answer 4: the aim reach (DONE)
- `content/src/map.rs`: `pub const AIM_REACH_MIN_CM: [i32; 3] = [-32_768, 0, -32_768];`
  `pub const AIM_REACH_MAX_CM: [i32; 3] = [32_767, 65_535, 32_767];` (x, y, z). Exact type edges: -327.68 m is inside.
  Compared in metres as `f64::from(cm) / 100.0` (correctly rounded; measured equal to the literals 327.67, 655.35,
  -327.68), so 327.67 accepted, 327.68 refused, no tolerance.
- `content::map::check_aim_reach(&MapDocument) -> Result<(), Error>`, called in the map kind's stage 6 from
  `bundle::freeze`, on `bounds` only (shapes and spawns are inside bounds by the rule before it).
  `MapBeyondAim { axis, value, reach }` at path `bounds.min[i]` / `bounds.max[i]`. Six refusals, six edges accepted.
- Tie test `crates/sim-wasm/tests/aim_reach.rs`: `fn fits<T: TryFrom<i32>>(_: fn(&AimPoint) -> T, cm: i32) -> bool`,
  `fn cm<T: TryFrom<i32>>(v: i32) -> T`; builds `AimPoint { x: cm(MIN[0]), y: cm(MIN[1]), z: cm(MIN[2]) }` and the max
  one, then per axis `!fits(|a| a.x, MIN[0] - 1)` and `!fits(|a| a.x, MAX[0] + 1)`: a wider field makes "one past"
  fit and the test fails ("the aim codec widened: move the reach in crates/content/src/map.rs"); a non-integer field
  fails to compile.
- Not in `StaticWorld::build` (finite, min < max, thickness, ramps, <= 512, depth <= 31 only). 4c's 2 km test builds
  a `MapDocument` in code (pub fields) and calls build directly: it never meets the loader. A match is reachable only
  through `Sim::create(bytes)`, so every playable map meets the rule.

## Answer 5: 3A-PART5-CLASH and the schema writer (DONE)
- Not reached in 4b: the clash is cross-crate nesting only (ADR 0002:353-357). Content's types nest inside content,
  all `#[cfg_attr(test, derive(ts_rs::TS))]` as protocol's (messages.rs:136); no other crate's exported type names a
  content type in 4b. No feature, so nothing new for the compare. content's ts-rs dev-dependency is the package and
  features protocol already brings into `cargo test --workspace`: no new entry. thiserror's difference comes with
  parry3d (step 0's FEATURE-COMPARE), not step 3.
- R7: nothing in steps 3-8 reads a schema (scene.ts, content.ts use ts-rs types; the panel reading schema limits is
  4c's). Moves with the writer: schema.rs + tests, ts_export's schema output and its `schema/` folder rule, the schema
  halves of PM-0005-7 and -8, the field table's `description` and `default` columns. Stays: field ranges and
  max_items, PM-0005-7's Rust half. Change: PM-0005-5 scans the field tables, not the generated schemas. Step 8b.

## Answer 6: SIZE-4B's seam (DONE)
- Sound with moves. To 4b-2 (step 8): parry3d's root line, sim's parry3d line, the 29 overflow blocks, the 33 registry
  packages; step 1's library parts: P2, the libm rule and its plant, the collision-only scan rule (with no library its
  plant fails to compile first: caught by the compiler, not the rule), the overflow check's new rows. To 4b-2 (step 7):
  ts_export.rs, generated/content/, index.ts exports, content's ts-rs dev-dependency (first reader content.ts).
  sim-wasm: `[dev-dependencies] content` in 4b-1 for the aim tie test, promoted at step 7.
- Then 4b-1's lockfile adds only the path crate: no registry package, no DEP-HOLD answer, the feature compare stays
  green (safe_arch, thiserror enter sim's closure only with parry3d). FEATURE-COMPARE gates 4b-2 only.
- Stays in 4b-1 and is used there: sim -> content (step 4: `sim::time::UNITS_PER_SECOND` re-exports content's),
  devtools -> content, the three edges dropped, own-code rules and P1, P3-P5, the version compare, reader + 3b + fuzz
  through the loader's entry, the extract script, step 4, LINKS-CHECK. No runtime dead code that uncut 4b lacks.
- His MUL-ADD words: "the same change-set that brings the first outside crate": under (b) the check merges one PR
  before the library, both inside 4b. SIZE-4B's text must say so for his choice to cover it.

DONE 2026-11-26
