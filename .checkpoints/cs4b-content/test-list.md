# 4b's test list, with the two readers' points folded in (before step 1; not a review round)

_The headings carry the step numbers of `reread-sheet.md`, revision 3: 4b-1 is steps 1 to 5 (the content crate,
the reader, its proofs, the links check, the time unit), 4b-2 is steps 6 to 12 (the collision library's batch, the
built-file check, the tools and the data, content to the simulation, the collision world, the schema writer, the
scene)._

Sources: the plan's sections 5, 9 and 11; the designs' mark tables (`engine-architect-cs4.md` 1.12, `rust-reviewer-cs4.md`
blocks 2 and 5); **test-engineer's read** (`test-engineer-testlist.md`: T-must 1 to 14, T-should 1 to 12) and
**security-auditor's read** (`security-auditor-testlist.md`: S-must 1 to 12, S-should 13 to 22, S-wait 23 to 27). Where
a point changes what a step builds, it is marked. N = native `cargo test` (debug and release), W = WebAssembly in
Node (the test module unless "players' file" is said), C = Chromium. Every plant is run by the plants script on the
real source; a plant counts as caught only when the output holds the check's own rule text (T-must 1), never
because "the build failed"; after the last plant the players' file is rebuilt to its sha256 from before the plants.

## Rules for the whole part (S's "must not" list, kept as rules)
- No content path or URL from the address, a flag, storage or a message; one fetch, from the page, never from the
  Worker; no eval, `new Function`, dynamic import by data, `innerHTML` and its kin; the CSP is not loosened.
- The tools open no socket, start no process, take no path from the environment or from a file's contents, write
  only their one constant target, never under `content/source/`, never outside `content/`, and follow no link.
- The shipped reader has no run-time mode and no `unsafe` (`#![forbid(unsafe_code)]` in `content`, S-should 13).
- No fallback: a refused bundle starts no match and draws no map; there is no built-in default map or table.
- The built-file check has no skip switch, environment variable or exemption; no `.cargo/config*`, `target-cpu` or
  `target-feature` anywhere; `ENSURE_NO_PANIC` joins `CARGO_OVERRIDES`.
- Errors echo at most 64 bytes of input, each byte outside 0x20 to 0x7E written as `\xNN`, and a whole message is
  capped (about 512 bytes) in Rust, in the Worker's trap message and in the page's message check (S-must 6).

## Step 7, the built-file check (4b-2)
- The names, target-features and instruction rules run on the **twin before the bindings tool** (T-must 1a), and
  the imports rule on the production file. The check throws; it writes a stamp with the sha256 of the production
  file it walked, and `buildPage` refuses a page whose Wasm is not that file (S-must 1). Plants: no stamp, a stale
  stamp, a SIMD module put into `pkg/` after a clean walk.
- **Names:** match any path segment exactly, not only the last (a generic's v0 name ends in the crate that
  instantiated it); fail when the twin has no name section, too few names or no sentinel name (T-must 3). Unit
  names in v0 (inherent method, trait impl, generic), legacy with hash, C names; near-miss controls `fmaxf`, `fminf`.
  The dev-profile twin is checked always, not only if P2 shows a lost name (S-should 22; T-must 3).
- **libm:** any function of the `libm` crate outside an allow-list file fails the build (T-must 4; with the
  library's `std` off its sine, arc cosine and logarithm paths reach `libm`, whose mangled names are not on the
  C-name ban list). Plant: an arc cosine of the vector library reachable from an export.
- **The instruction walker:** each body decodes to exactly its declared size and ends at depth 0; the test's
  module writer emits padded 5-byte numbers (call, call_indirect, memarg, i32.const); a float constant holding
  0xFD bytes passes; unknown 0xFC and 0xFD sub-opcodes fail; the real file's instruction count is printed and must
  be above 0 (T-must 5).
- **Imports:** an exact allow-list of the glue's own names, matched by token; plants: `env.mad`, a `Math.sin`
  import, a host-time import; near-miss control `__wbg_performance` if the glue has none (S-must 2, T-should 1).
- **The native side:** `rustc --print cfg` cannot fail (it never reads the flags). Instead a test asserts
  `!cfg!(target_feature = "fma")`, `"avx2"`, `"fma4"` in the native tests of `sim`, `content` and `devtools`, and
  `compile_error!` under `fma` and `avx2` in `sim` (not under `simd128`: the instruction walk already refuses it
  and 4d's reading needs such a side build; the advisor agent's R6) (T-must 2, S-should 21). Plant: `RUSTFLAGS=-Ctarget-feature=+fma` on that test.
- **The scan:** the collision-only rule matches the exact path and refuses a `pub use` or alias of the library;
  the hashed-map, clock and unwrap/expect/panic rules extend to `crates/content/src` (T-should 2).
- **The feature compare by version:** a two-version fixture (T-should 3). The check's result is recorded again at
  step 8, where the library is first linked; 4b proves nothing about the cast paths 4c links (said in the report).
- Plants P1 to P5 (rust-reviewer block 2) with these cautions: P2 only after step 0 and reachable from a
  `sim-wasm` export without a manifest edge; P3 may be refused by the bindings tool's parser first, which is why
  the rules run on the twin; a manifest plant rewrites the lockfile offline and the script puts both files back
  byte for byte (T-must 1).

## Steps 2 and 3, the reader, the bundle, the kinds and the proofs (4b-1)
- **Caps on structure, not only on bytes** (S-must 3): a node cap per file and a files cap per bundle, each
  refused by name; duplicate keys found in O(k log k). Tests: a worst-density file at the cap and a bundle of
  eight with the peak heap counted under a stated ceiling; a 100,000-key object judged under a stated time, N and W.
- **The bundle's own rules** (S-must 4): a path matches `^(movement|maps)/[a-z0-9_-]{1,64}\.json$`, built from the
  kind list in code; paths strictly increasing; the stem equals the file's `id`; the folder equals the kind; no
  extra key in the bundle or in a file entry; a `text` that is itself a bundle refused; the file cap counted on the
  decoded text; 1 MiB and 1 MiB + 1, 8 MiB and 8 MiB + 1 before any parse. One test per rule.
- **Numbers** (S-must 5, T-must 7): a non-finite result refused at stage 2 (`1e400`, `-1e400`, `1e309`: JSON.parse
  gives Infinity, so "the same bits" would match); a 10,000-digit integer and a 30-digit exponent; above 2^53 in
  three spellings and negative, with 2^53 accepted; more than 17 significant digits refused (S-should 14); a scan
  that every number field has a finite minimum and maximum.
- **Ids and enums** (S-must 7): ids are `[a-z0-9_-]{1,64}`, enums fixed lists; plants: an id with a line feed, a
  space, `<`, 65 characters.
- **The grammar's classes for the seeded generator** (T-must 7), each with its expected error kind: `null`, `true`,
  `false`; a byte-order mark; bytes above 0x7E or control bytes outside strings; comments; the bad numbers (NaN,
  the infinities, `+1`, `.5`, `5.`, `1e`, `--1`, `0x1F`, `01`, `-01`); bytes below 0x20 or above 0x7E inside
  strings; the escapes `\t \r \b \f \/ A` and a final backslash; an unterminated string; comma and colon
  errors; unbalanced brackets; empty input; two top-level values; depth 32 accepted, 33 refused, 10,000 deep;
  duplicate keys nested and not adjacent; the caps plus and minus one byte; typed: unknown and missing keys, wrong
  types, integer fields with 1.5, -1 and 4294967296, ranges at the next float up and down, one item too many, a
  bad format or version; whole set: duplicate ids, a shape 1 cm outside its bounds, the aim-reach rule. The three
  verdicts the cut grammar left unsaid are "refuse": a tab or carriage return outside a string, a top-level value
  that is not an object, an empty key (the advisor agent: no objection; engine-architect's read names each refusal).
- **Each generated file carries its class and its expected error** (the kind, with the path the generator changed
  or the line and column of the injected token), asserted natively; the cross-target compare covers the whole
  record (variant, file, line and column or path, payload with doubles as bits, the message text), and for
  accepted files the canonical bytes and the value bits (T-must 6). A shared defect cannot hide in "N equals W".
- **The corpus is not compared with itself** (T-must 8): the random half's texts come from V8's `String(x)`, the
  decimals' expected text is `JSON.stringify(JSON.parse(t))`; the compares run in CI's jobs (the Node boundary
  test and the rules spec, which also runs in Firefox), not in a one-off script. Plants only that compare
  catches: a writer that always prints 17 digits; exponent form starting at 1e16. His Export's numbers and the
  old tuning's enter as written (T-optional).
- **The writer's structure** (T-should 4): equals `JSON.stringify(sortedKeys, null, 2) + "\n"` over random trees
  (empty object and array, the three escapes). Plant: an empty array written as `[\n]`. A non-finite number in a
  tree built in code is an error, not text (S-wait 27).
- **The bounded fuzz** (T-must 9, S-must 8; his words: before the first value rests on the reader): it enters
  through the real entry (`Sim::create(seed, mode, 0, bytes)` once step 7 exists; the loader's own entry in 3b,
  extended to `ContentSet -> StaticWorld::build` at step 8); at least 90 % structure-aware mutations of valid
  seeds (flip, insert, delete, span duplication, truncation, nesting pumps, boundary numbers, refused tokens, key
  renames, array growth, bundle paths); a per-stage histogram with minimums is asserted; the slowest input is
  recorded; a failure prints the seed, the index and the input in hex; three fresh seeds by hand go into the
  notes. The full 1,000,000 run sits in a `devtools` test in release with the debug count in the test's name (as
  `decoder_inputs.rs`), because `content` is not in CI's release line; its debug time is measured. Plants found
  within the run, each run alone with the index it was found at: an unwrap in stage 6 on a path only mutation
  reaches; `shapes[0]` in the static world.
- The export test for the generated types copies `protocol`'s write rules and refuses a link named `schema` or a
  `.json` link (S-should 18). PM-0005-5's scan includes `regions` and the bundle's `files` (S-should 15).

## Step 2, the map kind's rules (4b-1)
- **The aim reach** (his DEPART-4 note; T-must 11): the reach lives in `content` as named constants; a `devtools`
  test equates them with `AimPoint`'s field types; six planted refusals (minimum and maximum of x and of z, y
  below 0, y above 655.35) and six accepted bounds.
- Every box at least 1 cm thick on each axis, ramps really rising, every spawn and shape inside `bounds`, spawns
  at least the match's slot count; plants 1 cm outside (S-should 15, 16).

## Step 5, the time unit (4b-1; T-should 6)
The oracle is 3 times the milliseconds parsed from the number's text, not the table the tool wrote; the `u32` limit
plus and minus one; plant: truncation instead of rounding; `content`'s 3,000 equals `sim`'s `UNITS_PER_SECOND`; the
planted 0.02 s interval fails `--check`, not only adds a row.

## Step 8, the tools and the data (4b-2)
- **The tools' file rules** (S-must 10): the walk reads each entry's own type without following it and refuses
  link files, link folders, FIFOs, unknown folders and names outside the grammar; reads capped at 1 MiB + 1; the
  write target is one constant, lstat-checked, written to a new temporary file and renamed; `--check`,
  `compare-old` and `table` write nothing. Plants: table rows naming `../.claude/settings.json`, an absolute path
  and `content/source/x.json`; a link file, a link folder, a FIFO; all refused with the tree's bytes and times
  unchanged.
- **`content-bundle --check`** (T-should 7): plants CRLF line endings, no final newline, unsorted keys, `1.0`; a
  nested `source` folder refused (only the top-level `content/source` is skipped).
- **`testlines --check`** (plan 11d's three plants, and T-must 12): compare by bits; a 1-ulp plant; a row naming a
  place that does not exist; a content field no row maps. The Export's sha256 pinned in reviewed code, not only in
  the manifest beside it; `write`, then `--check`, then `content-bundle --check` stays green; 3 to 5 values pinned
  by hand from the Export's text, because a mis-mapped row is shared by `write` and `--check` (T-should 8).
- **Which reader reads his Export** (it holds `true` five times; the cut reader refuses it): a Node script on the
  platform's own JSON reader, `packages/client/scripts/testlines.mjs` (the advisor agent's ruling F.6; Mustafa's
  agreement of 2026-11-26, 13:05 UTC). The old tuning file is read the same way (`legacy-extract.mjs`).

## Step 9, content to the simulation and the page (4b-2)
- **"Loading content changed no state"** (T-must 13): `first-loop.hashes`' per-tick lines and the whole
  `first-loop.vector.txt` equal main 2483e9a's files (compared with `git show`'s output, not with what the tool
  just wrote); plant: content folded into the state at the start fails at step 1 in N, W and C; the replay's
  content has the header's digest.
- **The boundary** (T-should 9): six refusal classes on the players' file, each an `Error`, never a trap, the
  instance still usable after; the Worker's `start` refuses a bundle that is no `ArrayBuffer` or is over 8 MiB,
  before the glue copies it (S-must 9); the bytes reaching Wasm equal the served file; the page's `content.json`
  equals the tool's output.
- **The page draws only what the loader accepted** (S-must 9): one fetch; a copy kept before the transfer; the
  page's `JSON.parse` runs only after the Worker's `ready`. Tests: a unit test with a fake Worker that answers
  `error` (no parse, nothing drawn, exactly one fetch); a Chromium spec serving bundles through `page.route`: a
  duplicate key, `__proto__`, `1e400`, an extra file-entry key, a `source/` path, an unknown key named like an
  image tag; expected: the loader's sentence as text, no `img` element, the scene still empty.
- `filesOf` refuses entries that are no plain files in the page folder, and a `content.json` over 8 MiB; a source
  scan for `innerHTML` and its kin in the page's and the Worker's sources (S-should 19, 20).

## Step 10, the static collision world (4b-2)
- **The dump** (T-must 14): node index, child links, leaf flag, leaf data and the collider list (id, kind, minimum
  and maximum as bits); whole dumps compared and a committed known-answer hash; a document built in code with -0 equals the +0 map (no content file can hold -0:
  the loader refuses it); a
  map with its shapes permuted gives the same bytes; ray classes with minimum counts (axis-aligned with signed
  zero direction parts, grazing, starting inside, through shared edges, on the wedge's slope, misses).
- **Robustness** (T-should 10, S-must 8): `StaticWorld::build` on hand-made documents (NaN, an inverted box, 0, 1
  and 512 shapes, 512 identical shapes, nested boxes, zero thickness, everything at the reach's edge) gives Ok or
  a named refusal, never a panic, N and W; how a 33-level tree is made is said, or the depth refusal is untested.

## Step 12, the page's scene (4b-2; T-should 11)
A development hook lists the drawn objects and the spec compares them with the map; the broken-bundle spec uses a
bundle that `JSON.parse` accepts and the Rust loader refuses (a duplicate key); the loader's sentence is pinned;
the render counts are derived from the map, not copied from the new build.

## Step 4, the links check (4b-1; his LINKS-CHECK (c); S-must 11, T-should 12)
One public read function in `devtools::files` that refuses anything whose own type is not a plain file, used at
`tests/recordings.rs:18`, `two_instances.rs:42` and `:44`, `two_processes.rs:113` and `:115`, and inside
`files.rs:115`; one Node helper for `boundary/replay.test.mjs:15` and `determinism.spec.ts:68`, `:70`, `:72` (read
once, digest from the same bytes). Planted links: Rust under `cfg(unix)` (to an outside file and to `/dev/zero`),
Node by `symlinkSync` in a temporary folder; the TypeScript link test fails, never skips, under CI. A text scan
shows the four Rust test files and the two TypeScript readers read recordings only through the helpers. Every new
reader of 4b uses the helper from its first line.

## For the report, not for a test
- PM-0004-33's "after the library" is partial in 4b (the cast paths are 4c's).
- SOURCE-GO (S-wait 24): "any change under `content/source/` needs his go" is in no rule the PR Advisor reads: a
  decision for him, closed by a chat that may write kit files, before 4b merges.
- S-wait 25, 26: content from the server's own disk at change-set 7; 4d's reload rules; change-set 8's bundle hash.
