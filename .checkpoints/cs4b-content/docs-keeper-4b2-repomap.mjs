// docs-keeper, 4b-2 docs pass: docs/REPO_MAP.md made true for change-set 4b-2 (code 914f3b4). Each existing row is
// found by its path cell; each change replaces one exact, unique piece of that row or appends before its last " |".
// New rows go after a named row. Run once from the root; refuses to run twice.
import { readFileSync, writeFileSync } from "node:fs";

const F = "docs/REPO_MAP.md";
const r = readFileSync(F, "utf8").split("\n");
if (r.some((l) => l.startsWith("| `crates/sim/src/collision.rs` |"))) throw new Error("already run");
const V = "[verified 2026-11-27 at 914f3b4";
const at = (path) => {
  const i = r.findIndex((l) => l.startsWith(`| \`${path}\` | `));
  if (i < 0 || r.findIndex((l, k) => k > i && l.startsWith(`| \`${path}\` | `)) >= 0) throw new Error(`row ${path}`);
  return i;
};
const swap = (path, from, to) => {
  const i = at(path);
  if (r[i].split(from).length !== 2) throw new Error(`${path}: "${from.slice(0, 50)}" not once`);
  r[i] = r[i].replace(from, () => to);
};
const append = (path, text) => {
  const i = at(path);
  if (!r[i].endsWith(" |")) throw new Error(`${path}: no end`);
  r[i] = `${r[i].slice(0, -2)}${text} |`;
};
const inserts = [];
const after = (path, rows) => inserts.push([path, rows]);

// Existing rows: what 4b-2 made untrue, and what it added.
swap("Cargo.toml", "(so far ts-rs `=12.0.1` and wasm-bindgen `=0.2.130`)",
  "(ts-rs `=12.0.1`, wasm-bindgen `=0.2.130` and, since change-set 4b-2, the collision library parry3d `=0.31.1` with its default features off and `dim3`, `f32`, `alloc` and `enhanced-determinism` on, each bump its own change-set)");
swap("Cargo.toml", "(cfg-if, thiserror, ts-rs, unicode-ident, wasm-bindgen, wasm-bindgen-shared)",
  "(cfg-if, thiserror, ts-rs, unicode-ident, wasm-bindgen, wasm-bindgen-shared) and, since change-set 4b-2, for the 29 crates of the collision library's tree (`safe_arch`, which only a native x86 build has, among them)");
append("Cargo.toml", ` ${V}: the parry3d line; 36 \`[profile.release.package.*]\` tables, \`"*"\` and the six among them]`);

swap("crates/", "the generator of its random streams)",
  "the generator of its random streams; since change-set 4b-2 also the static collision world and its own vector, below, and it depends on `content` and the collision library alone)");
swap("crates/", "since change-set 4a also `shared_hash`, the one number a replay reads)",
  "since change-set 4a also `shared_hash`, the one number a replay reads; since change-set 4b-2 its constructor takes the match's content bundle and builds the map's static world)");
swap("crates/", "`protocol` stays, for the aim reach's tie test [verified 2026-11-26 at 2039e3c: `crates/devtools/Cargo.toml`]",
  "`protocol` stays, for the aim reach's tie test, since change-set 4b-2 as a dev-dependency only; since change-set 4b-2 also the content files and their bundle tool and the static world's proofs, below [verified 2026-11-27 at 914f3b4: `crates/devtools/Cargo.toml`]");
swap("crates/", "nothing in `sim` or `sim-wasm` depends on it yet, `devtools` alone does [verified 2026-11-26 at 2039e3c: the three manifests]",
  "since change-set 4b-2 `sim`, `sim-wasm` and `devtools` depend on it, and its one dev-dependency is the type writer ts-rs [verified 2026-11-27 at 914f3b4: the four manifests]");

swap("crates/sim/tests/rules_scan.rs", "; a file-level `#![cfg(test)]` is not seen as test code (it fails closed) [verified 2026-11-26 at 2039e3c]",
  " [verified 2026-11-26 at 2039e3c]. Since change-set 4b-2 a file whose first line is `#![cfg(test)]` is test code as a whole; the collision library may be named in `collision.rs` only, and there neither handed on with `pub` nor renamed with `as`; a fused multiply-add is refused by its other names too (`fma`, `fmaf` and the like); a `target_feature` passes only in a `#[cfg(..)]` whose item is `compile_error!` " + V + ": the file's header, `in_tests`, `LATER_BITES`]");

append("crates/sim/src/time.rs", `. Since change-set 4b-2 \`UNITS_PER_SECOND\` is \`pub use content::time::UNITS_PER_SECOND\`: one number in one place ${V}: \`time.rs\`]`);
after("crates/sim/src/time.rs", [
  "| `crates/sim/src/collision.rs` | The static world of a match (change-set 4b-2's step 10; ADR 0004 part 10; PM-0004-32): `StaticWorld::build` makes it once from the map document when a match starts, it never changes and is never in a snapshot; each `Solid` is a box by its lowest and highest corner in `f32` and what fills it, a shape's index is its rank by id, so two documents with the same shapes give the same world byte for byte (`tree_bytes`); the tree is the collision library's (its binned build, its walk, its depth), the ray's arithmetic ours: `cast_ray` gives the first shape met, of two at the same distance the lower index, allocates nothing, and gives no hit for a number beyond `RAY_REACH` (2^20); `WorldError` for more than `MAX_SHAPES` (512) shapes, a corner that is no finite `f32`, no thickness, a tree deeper than `MAX_TREE_DEPTH` (31). The one file that names the library (held by `tests/rules_scan.rs`); no step queries the world yet (4c) " + V + ": the file's header and public items] |",
  "| `crates/sim/src/vec.rs` | `V3`, the simulation's own vector (change-set 4b-2): three `f32` in metres, y up (`new`, `to_array` so far); the state and the queries speak in it and never in the collision library's types, so what a vector may do is ADR 0004 part 7's operations, as written there |",
  "| `crates/sim/tests/native_target.rs` | The native half of the built-file check for `sim`'s tests (change-set 4b-2's step 7; his MUL-ADD (a)): the compiler is asked whether the build was told `fma` or `avx2`, and a flag from anywhere (a variable, a cargo config file) that turned one on fails here; AMD's `fma4` is not asked (the pinned compiler tells a stable build nothing of it). `crates/sim/src/lib.rs` refuses such a build with `compile_error!`; the same test stands in `content` and `devtools` (below) |",
]);

swap("crates/content/", "`Cargo.toml` with no dependency (held by",
  "`Cargo.toml` with no dependency, and since change-set 4b-2 one dev-dependency, the type writer ts-rs, for the export test (held by");
swap("crates/content/", "which way `facing_deg` 0 points is pinned at step 8)",
  "since change-set 4b-2 `facing_deg` 0 faces +z and the degrees grow toward +x, the old game's convention, and the fewest spawns is `MIN_SPAWNS`, 1)");
append("crates/content/", `. Since change-set 4b-2: \`bundle.rs\`'s \`FOLDERS\` is public (the content tool walks them) and \`test_bundle\` is the crates' and tools' test bundle, hidden from the documents; the kinds' types derive ts-rs under \`cfg(test)\`; and two test-only files, \`schema.rs\` and \`ts_export.rs\` (below) ${V}: \`Cargo.toml\`, \`lib.rs\`, \`map.rs:29\` and \`:58\`, \`bundle.rs\`]`);
after("crates/content/", [
  "| `crates/content/src/schema.rs` | Test builds only (`#![cfg(test)]`; change-set 4b-2's step 11; ADR 0005 CD-3, the schema halves of PM-0005-7 and PM-0005-8): the JSON Schema of each content kind, built from the reader's own constants (ranges, caps, fixed words, the id rule) and written by the crate's canonical writer; every object closed, no `default` (the reader takes none), each property's meaning and unit in a test-only table, so no shipped build holds a word of it (the advisor agent's ruling of 2026-11-27); `ts_export.rs` writes and compares the files |",
  "| `crates/content/src/ts_export.rs` | Test builds only: the content kinds' export test (change-set 4b-2's step 9; ADR 0002 part 5; PM-0005-8's types half): ts-rs writes their TypeScript into a new scratch folder, and the test fails when `packages/types/src/generated/content` does not hold the same files, with its one folder `schema/` of plain `.json` files (a link, another folder or another file there refused by name); `SHAMBLE_WRITE_TYPES=1 cargo test -p content` writes both again; the folder rules copied from the protocol crate's export test, the two crates sharing no code |",
  "| `crates/content/tests/native_target.rs` | The native half of the built-file check for the content crate's tests (change-set 4b-2): as `crates/sim/tests/native_target.rs` |",
]);

swap("crates/sim-wasm/src/lib.rs", "`Sim` (`new(seed, mode, pad)`,",
  "`Sim` (`new(seed, mode, pad)`, since change-set 4b-2 `new(seed, mode, pad_bytes, bundle)`,");
append("crates/sim-wasm/src/lib.rs", `; since change-set 4b-2 \`bundle\` is the match's content (\`content.json\`), crossing once, in the constructor, so no call can swap it mid-match, judged by the game's own loader; the match holds the static collision world of the content's map \`LOCAL_MAP\` (\`test-scene\`), and \`CreateError\` adds the loader's refusal, \`MapMissing\` and \`World\`, checked in that order; nothing of the content enters the match's state ${V}: the file]`);
append("crates/sim-wasm/src/local_match.rs", "; since change-set 4b-2 it holds the static collision world it is made with (`static_world()`), which no step queries yet (4c)");

append("crates/devtools/src/wasm.rs", "; since change-set 4b-2 `replay_hashes` and `replay_state_at` also take the recording's frozen content, and `static_world` runs the static world's proof");
after("crates/devtools/src/content_proofs.rs", [
  "| `crates/devtools/src/content_files.rs` | The content as files (change-set 4b-2's step 8): the folder `content/` read as the loader reads a bundle, one folder per kind of `<id>.json` files, and the one bundle the build ships (`content.json`). The walk asks every entry its own type without following it and refuses a link, a pipe, a folder inside a kind's folder, a folder it does not know, a name outside the id grammar; `source/` is no content and never opened; a file is read through the plain-file reader, at most the loader's cap and one byte; whether a file is content is the loader's to say. The bundle tool is its `run`, and the plain test run calls the same functions |",
  "| `crates/devtools/src/world_proofs.rs` | The static world's proof (change-set 4b-2's step 10; PM-0004-32), written once and run natively and, through the test-only module, in WebAssembly under Node and in the browsers: seeded maps of each size a world can have (none to the full 512 shapes), boxes and ramps on the map's millimetre grid, and rays straight down, along an axis, slanted and down a face's plane; one line per map (the tree's depth, a digest of its bytes, a digest of every ray's answer). Then `HOSTILE` (512 times one box, boxes inside one another, thin slabs a map long, the chain whose tree is too deep), each written as a file, read by the loader and built or refused; `judge_taken` holds these worlds and the fuzz run's to what the loader and the builder promise together |",
]);
after("crates/devtools/src/bin/time_report.rs", [
  "| `crates/devtools/src/bin/content_bundle.rs` | `cargo run -p devtools --bin content_bundle -- <folder>` (change-set 4b-2's step 8): reads `content/`, asks the loader about each file and the whole, and writes `content.json` into `<folder>`, which must exist and lie outside `content/`; with `--check` it writes nothing and exits 1 on a file the loader does not take or anything that is no content file. One folder, a constant; no path from the environment. The build runs it (`build.mjs`, `writeContentBundle`) |",
]);
swap("crates/devtools/recordings/", "`first-loop.rec` (1,800 steps on today's state, 122,487 bytes)",
  "`first-loop.rec` (1,800 steps on today's state, 122,487 bytes; since change-set 4b-2 122,497, its header naming its frozen content by sha256 and its map `test-scene`)");
append("crates/devtools/recordings/", `. Since change-set 4b-2 also \`first-loop.content.json\` (below) ${V}: the file sizes, \`manifest.txt\`]`);
after("crates/devtools/recordings/", [
  "| `crates/devtools/recordings/first-loop.content.json` | The first recording's frozen content (change-set 4b-2's step 9, cut B): the bundle of `content/` as it was when `make_recordings --freeze first-loop` wrote it, its one writer, 8,375 bytes; the recording's header names it by its sha256, every replay checks it, and the scripts never make it again (`files.rs`) |",
]);
append("crates/devtools/tests/", "; since change-set 4b-2 three more, below (`content_files.rs`, `static_world.rs`, `native_target.rs`)");
swap("crates/devtools/tests/dependency_rules.rs", "`content` depends on nothing |",
  "`content` depends on nothing; since change-set 4b-2 its tests on the type writer alone (one line of `[dev-dependencies]`), `sim` on `content` and the collision library alone, the library's line in the root and the locked versions of parry3d, safe_arch and thiserror pinned by his FEATURE-COMPARE (a), no crate but `sim` naming the library, no `spade` in the lockfile, and rstar 0.13.1 and heapless 0.8.0 held, so a move is seen " + V + ": the file] |");
after("crates/devtools/tests/dependency_rules.rs", [
  "| `crates/devtools/tests/content_files.rs` | The content folder's walk and the bundle tool on scratch folders (change-set 4b-2): what a clean tree gives and each thing the walk and the loader refuse, the tree left as it was; then the repository's own `content/` (his movement table and the test scene), and five of his values reaching the loader by a second route |",
  "| `crates/devtools/tests/static_world.rs` | The static world's proof natively (change-set 4b-2; PM-0004-32): the trees and rays against the committed `vectors/static-world.txt`, the hostile maps' lines among them, and the first map that differs named; then each map under `content/` against `vectors/content-worlds.txt` (natively only); `SHAMBLE_WRITE_WORLD=1` writes the files again. The same check runs in WebAssembly under Node and in the browsers |",
  "| `crates/devtools/tests/native_target.rs` | The native half of the built-file check for `devtools`' tests (change-set 4b-2): as `crates/sim/tests/native_target.rs`, and no cargo config file (`.cargo/config.toml`) is committed |",
]);
append("crates/devtools/vectors/", ". Since change-set 4b-2 also `sim-feature-lists.json` (the collision library's batch as data: the simulation's resolved features in the players' Wasm, the test-only module and the native tests, 71, 71 and 76 entries, read by `build-checks.test.mjs`), `static-world.txt` (the static world's proof: 12 seeded maps and 9 hostile ones, one of them refused) and `content-worlds.txt` (the tree of each map under `content/`: `test-scene`, 23 shapes), the last two written by `tests/static_world.rs` " + V + ": the files' first lines]");

swap("packages/types/src/generated/", "Generated from Rust by the two export tests and committed;",
  "Generated from Rust by the export tests (two; since change-set 4b-2 three, with the content kinds' in `content/`) and committed;");
append("packages/types/src/generated/", "; the content kinds' test holds `content/` and its one folder `schema/` the same way (since change-set 4b-2)");
after("packages/types/src/generated/block/layout.ts", [
  "| `packages/types/src/generated/content/` | The content kinds' types in TypeScript, one file each, written by ts-rs from `crates/content` (change-set 4b-2's step 9): `Bounds`, `Ground`, `MapDocument`, `Movement`, `Region`, `Shape`, `Spawn`, `Toward`, a content file's own keys beside its `format` and `version`; no reader yet but their tests (STATUS, 4B-TYPES-READER) |",
  "| `packages/types/src/generated/content/schema/` | The JSON Schemas of the two kinds (change-set 4b-2's step 11): `map.schema.json` and `movement.schema.json`, written by `crates/content/src/schema.rs` in canonical form, for the editor's inspector and the Feel Lab to build their fields from (ADR 0005 CD-3); the verdict on a file stays the loader's |",
  "| `packages/types/src/content-schema.test.mjs` | The schemas read as an editor's field will read them (change-set 4b-2's step 11; PM-0005-7's schema half, a schema-driven limit test), by this test's own limits reader (no product code reads a schema yet): each file the canonical writer's text, every range's ends, the id rule, closed objects; and the generated types and the schemas name the same keys, corner lengths and words |",
]);
append("packages/types/src/index.ts", "; since change-set 4b-2 also the content kinds' types (`Bounds`, `Ground`, `MapDocument`, `Movement`, `Region`, `Shape`, `Spawn`, `Toward`)");
append("packages/sim-worker/src/", ". Since change-set 4b-2: the page's `start` carries the content bundle, transferred, a plain `ArrayBuffer` of 1 to `MAX_BUNDLE_BYTES` (8 MiB, the loader's own cap) bytes, handed to `Sim` once (`messages.ts`, `worker.ts`); a thrown text reaches the page cut to `THROWN_TEXT_UNITS` (512), so a loader's refusal passes whole (`panic-message.ts`)");
append("packages/sim-worker/boundary/sim-wasm.test.mjs", "; since change-set 4b-2 the constructor with the content bundle and its refusals, the cap read from `messages.ts`");
after("packages/sim-worker/boundary/content.test.mjs", [
  "| `packages/sim-worker/boundary/static-world.test.mjs` | The static world's proof in WebAssembly under Node (change-set 4b-2): `crates/devtools/tests/static_world.rs`'s check, run by the test-only module in its release and its dev profile against the native build's committed lines, the vector read through `plain-file.ts`; one changed digit of one ray digest is found, with its map |",
]);
append("packages/client/vite.config.mjs", ". Since change-set 4b-2 the dev server answers `/content.json` with what the build's own content tool writes on each ask (`writeContentBundle`, into `.gate/dev-content`; no watch, no reload), refuses another site's ask (by `Sec-Fetch-Site`) with 403 and a fixed sentence, and passes an address with no path to Vite's own handlers");
append("packages/client/scripts/build.mjs", ". Since change-set 4b-2: the built-file check walks the players' WebAssembly on every build and leaves a stamp beside it (`built-file-check.json`), and no page is built from a file the stamp does not name; `writeContentBundle` writes `content.json` beside `index.html` by the content tool, in its own target folder `.gate/target-content`");
append("packages/client/scripts/build-checks.mjs", ". Since change-set 4b-2: the feature compare's two named exceptions for the native tests (`TARGET_ONLY_ENTRIES`, `TEST_ONLY_FEATURES`) and two versions of one package refused; the built-file check's readers (`wasmImports`, `wasmFunctionNames`, `wasmTargetFeatures`, `wasmCodeOpcodes`) and rules (`fusedMultiplyAddProblems`, `bannedMathsProblems`, `libmProblems` against `maths-allowed.mjs`, `unlinkedCrateProblems`, `twinProblems`, `PLAYERS_IMPORTS`, `productionProblems`, `walkedStamp`, `stampProblems`); `contentProblems` (`content.json`, at most 8 MiB); `markupSinkProblems` over `SCANNED_SOURCES`");
after("packages/client/scripts/build-checks.mjs", [
  "| `packages/client/scripts/maths-allowed.mjs` | The maths routines the players' WebAssembly may hold beside plain instructions (change-set 4b-2's step 9; read by `libmProblems`): two entries, `C::round` and `C::__multi3`, both brought by the content loader and run when a bundle loads, never per tick (rust-reviewer's decision of 2026-11-27); any other routine stops the build, and a fused multiply-add or a `BANNED_MATHS` function is refused whatever the list says |",
  "| `packages/client/scripts/testlines.mjs` | The hand-over from his Testlines Export to the game's content (change-set 4b-2's step 8), from the repository's root: `--check` (the content holds his Export's values; writes nothing), `write` (writes `content/movement/soldier.json` from his Export), `compare-old` (his Export against the old game's tables) and `table`; his Export is read here, with the platform's own JSON reader, and nowhere else; every file it reads or writes is a constant of the script; what it writes is judged afterwards by `content_bundle --check` |",
  "| `packages/client/scripts/testlines.test.mjs` | Its tests on made-up values (what it writes, what its check finds, every file it refuses to read or write), then on the repository's own files: his saved Export, the manifest beside it, the table and the movement table they give |",
  "| `packages/client/scripts/canonical.mjs` | `node packages/client/scripts/canonical.mjs <kind's folder>/<id>.json` (change-set 4b-2's step 8): rewrites one hand-written content file in the form the loader takes (the value with sorted keys, an indent of two, a last newline), refusing a minus zero and changing no value; it reads and writes under `content/` only, never under `content/source/` and never through a link; the loader judges the result |",
  "| `packages/client/scripts/canonical.test.mjs` | Its tests on scratch files: the form it gives, what it refuses, and where it will not write |",
]);
append("packages/client/src/app/", ". Since change-set 4b-2: `content.ts` (below); `boot.ts` starts the picture, then the content (one fetch; no Worker is made when it fails), then the Worker on it, then the frame loop; `worker-link.ts`'s `start` transfers the bundle's bytes, and the Worker's own error event is cut to the Worker's bound for a thrown text");
after("packages/client/src/app/", [
  "| `packages/client/src/app/content.ts` | The page's content (change-set 4b-2's step 9): one fetch of `./content.json` beside the page (`fetchContent`), a copy kept before the bytes go to the Worker, and the page's own reading of that copy (`readBundle`) only after the Worker's `ready`, so `JSON.parse` never sees a bundle the loader refused; `takeWorkerMessage` and `takeWorkerError`, which `boot.ts` calls, let the copy go when the match ends; every sentence is fixed text, never what the server sent |",
  "| `packages/client/src/app/content.test.ts` | Its Vitest tests, on bundles as the loader writes them and a fetch that counts its calls |",
  "| `packages/client/src/app/content-ties.test.mjs` | Ties `content.ts`'s `BUNDLE_FORMAT` and `BUNDLE_VERSION` to the loader's own in `crates/content/src/bundle.rs`, read from the Rust source (engine-architect's round of 2026-11-27); plain JavaScript, read with Node's own module |",
]);
append("packages/e2e/tests/skeleton/", ". Since change-set 4b-2 also `content.spec.ts`: the page's content on the built development page (one fetch, the served bytes the built file's, one changed value refused at its file and line, seven hostile bundles each shown only as the loader's own sentence, a failed fetch starting no Worker); the isolation spec also checks the dev server's `/content.json` (the build's bundle byte for byte, another site's ask refused with 403, an address with no path passed to Vite)");
append("packages/e2e/tests/rules/", ". Since change-set 4b-2 also `static-world.spec.ts`: the static world's proof on the test-only module's release profile, against the native build's committed lines (its wall-clock bound kept as a backstop: STATUS, 4B-WALLCLOCK)");

// Insert bottom up so the found rows keep their places.
const placed = inserts.map(([path, rows]) => [at(path), rows]).sort((a, b) => b[0] - a[0]);
for (const [i, rows] of placed) r.splice(i + 1, 0, ...rows);
writeFileSync(F, r.join("\n"));
console.log("rows added:", inserts.reduce((n, [, rows]) => n + rows.length, 0), "lines:", r.length);
