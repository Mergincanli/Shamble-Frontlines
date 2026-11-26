# rust-reviewer: review round 1 of change-set 4b-1 "Content" (head 7cc46a1 = ef2edec + brief; branch cs/4b-content)

Brief: .checkpoints/cs4b-content/review-brief-4b1.md. Target folder: .gate/rust-reviewer-4b1-target (every cargo call,
--locked). Scratch: .gate/rust-reviewer-4b1-scratch/ (copies only) and scratch text beside this file.

## Plan
1. Read crates/content/src: lib.rs (lint guard), error.rs, json.rs, number.rs, write.rs, read.rs, bundle.rs, map.rs,
   time.rs, movement.rs. Panics, overflow, `as` casts, recursion bounds, slices, float->int; work/memory bound by
   input length and caps; float ops used and their exactness on both targets; no hash map, no clock.
2. number.rs in depth: RFC 8259 grammar, ECMAScript Number::toString, even_of_a_tie reasoning and arithmetic
   (subnormals, powers of two); any other way std's shortest text can differ from ECMAScript's.
3. devtools: content_proofs.rs (is each class's verdict independent of the loader?), time_report.rs, files.rs,
   wasm.rs, bin/time_report.rs; tests content_proofs, content_fuzz (allocator SAFETY, threads, shares), aim_reach,
   time_unit, recordings, two_instances, two_processes; sim/src/time.rs.
4. Speed: cost per byte at the 8 MiB cap, duplicate-key search, canonical compare; anything quadratic.
5. Runs: cargo clippy -D warnings (workspace, all targets), cargo test -p content, -p devtools (debug), release fuzz;
   scratch tests: 3 ties + 3 near-ties through number_text; corner 0.1+0.2; deep nesting; bundle whose file text holds
   the bundle's closing lines.
6. Departures 1, 2, 3, 6, 7, 8, 10, 13: a verdict each.
7. Final: verdict + findings (severity | file:line | what | evidence | fix), not checked.

## Findings (a line each, as found)
- [json.rs read] recursion bounded (containers refused at depth > 32 before entry, json.rs:204); nodes capped 16,384
  before each value (:199-202); numbers refused unread past 25 bytes (:404); path string bounded (32 levels x <= 66 B).
  Duplicate keys: index sort per object, O(m log m) (:308-323). One `as` (:407, constant 25). No panic path seen.
- [number.rs, PROVISIONAL, to confirm by run] even_of_a_tie (number.rs:81-103) checks only that the double sits exactly
  halfway (decimals == places-1); it never checks that the even neighbour lies inside the rounding interval. For a power
  of two that interval is lopsided (half as wide below). 2^-24 = 5.9604644775390625e-8: 16-digit neighbours ...062e-8 and
  ...063e-8 are 5e-24 away; upper margin 2^-77 = 6.6e-24 (valid), lower margin 2^-78 = 3.3e-24 (NOT valid). So std gives
  ...063 (right), ECMAScript gives ...063, the code flips to ...062, which reads back as a different double. By my
  algebra ties exist only for x = M*2^P with -25 <= P <= -2; lopsided only at fraction 0; only E = -24 has the upper
  inside and the lower outside (E = -25: both inside, flip right). So exactly +-2^-24.
- [number.rs, JS oracle] Node 22: String(2**-24) = "5.960464477539063e-8", Number("5.960464477539062e-8") !== 2**-24;
  String(2**-25) = "2.9802322387695312e-8" (a real tie, the even flip is right there). The number corpus holds no
  power of two between 2^-3 and 2^-1022 (powers listed: 0, 2^-1022, 2^-3..2^6, 2^9, 2^53 and signs): it cannot see it.
- [read.rs/map.rs/time.rs read] float->int casts: read.rs:114 (after fract==0 and range: exact), map.rs:229 (saturating
  `as i64`; inputs <= 1e7 steps; NaN never reaches the cast, inf saturates: no panic), time.rs:38 (after > u32::MAX
  check). f64::round, fract (trunc), *, / are exactly specified IEEE ops; round/trunc lower to libm calls on both
  targets (compiler-builtins on wasm32, glibc natively), exact by definition. on_grid(v) <=> v = fl(k/per_metre) for
  integer k: a clean predicate; thickness in whole steps is exact; double comparisons of on-grid values agree with step
  comparisons. 0.1+0.2 = 0.30000000000000004 -> round(300.00000000000006)=300, 0.3 != v -> OffGrid (to run).
- [content_proofs read] verdicts for refusals are worked out apart from the loader (reason, place, field by hand).
  Self-comparisons: every valid file writes its numbers with number_text (the code under test; the corpus holds the
  text side); Map::text() (content_proofs.rs:437-442) lays a map out through json::parse + canonical, so for maps the
  canonical-form check is the writer against itself (the tree corpus and File::lines hold the layout independently);
  fuzz_judge's canonical() == bytes compares canonical_into with itself (two sinks), as its doc says.
- [F1 CONFIRMED, Medium] number_text(+-2^-24) = "+-5.960464477539062e-8", which reads back as 0x3e6fffffffffffff (one
  ulp below); V8 writes "5.960464477539063e-8". Run: scratch built with the pinned rustc 1.99.1 at opt-level 3 against
  crates/content/src (rust-reviewer-scratch-4b1.rs, `numbers`), 31,750,551 doubles: every power of two with both
  neighbours and both signs; 9.9 M in the tie window x = M*2^-q (q 1..25, M odd, sampled) checked against ECMAScript's
  rule worked out apart from the crate (candidates s-1, s, s+1 at std's length, valid when std's parser reads them back,
  closest by exact u128, tie to even): 318,235 real ties, number_text right on all but 2^-24; 2 M random bit patterns.
  Node 22 (V8) String(x) over all 31,750,551 (rust-reviewer-scratch-4b1.mjs): differ on 2 distinct values only,
  +-2^-24. Consequences: the loader refuses every text of +-2^-24 and its message names a text of another double; a
  tree built in code holding 2^-24 is written as the text of 2^-24 - 2^-77 (a silent one-ulp change on write);
  number.rs:4-5's "agree on every finite number" is false for two doubles. Fix: in even_of_a_tie, return the even
  digits only when "{even}e-{decimals}".parse::<f64>() == value (exact for every double; a parse per tie only), or
  refuse the lower neighbour when fraction == 0 && exponent > 1; add +-2^-24 and +-2^-25 to the unit test and every
  power of two (both signs) to the corpus generator.
- [number.rs reasoning] ties exist only when x = M*2^P (M odd) with decimals = -P-1; shortest text <= 17 digits forces
  -25 <= P <= -2 (decimals <= -0.301P + 17). Subnormals: places-1 >= 1021 > any shortest text's decimals (<= ~340):
  the test never fires and no tie exists there (right). Arithmetic: 5^24 * M < 2^109 fits u128; every step checked.
  Other differences std vs ECMAScript: none by reasoning (std: Grisu with Dragon fallback, shortest, closest, ties up,
  boundaries inclusive iff even mantissa = ECMAScript's "F(s*10^(n-k)) is x"); none in the 31.75 M sweep.
- [spot-checks, scratch `loader`] corner 0.1+0.2 (0x3fd3333333333334) -> maps/yard.json|0:0|shapes[0].min[1]|OffGrid
  1000; 0.3 taken; bound y = 0.1+0.2 -> OffGrid 100; 327.67+0.01 (=327.68) -> BeyondAim 32767. 1,000,000 '[' in a file,
  in the bundle itself, and a file in a bundle -> TooDeep at level 33, no stack overflow; 32 nested objects taken, 33
  refused. A file whose text ends with the bundle's own closing lines -> movement/test-soldier.json|22:3|TrailingData
  (the file's own line: no confusion with the bundle's structure); those lines as the id (escapes, so backslashes) and a
  bundle inside a bundle -> RefusedEscape 92 at files[0].text. Ties: crate's 3 + 2^-25 right, near-ties (+-1 ulp) equal
  std; 2^-24 wrong (F1).
- [runs] `cargo fmt --all -- --check`: clean. `cargo clippy --workspace --all-targets --locked --offline -- -D warnings`
  (own target folder): clean. Same for `--target wasm32-unknown-unknown -p content -p devtools -p sim --lib`: clean.
  `cargo test -p content --locked`: 35 passed (debug).
- [ci, Medium, shared with security-auditor and test-engineer notes] no `cargo clippy` in CI (toolchain.yml:158-171)
  or the gate: content's deny list (lib.rs:13-24), which is the crate's no-panic guarantee, holds only when someone
  runs local-checks.sh:12 by hand. Also CI's release test line (toolchain.yml:162) has no `-p content` (local-checks
  adds it at :19). Fix: a CI step `cargo clippy --workspace --all-targets --locked -- -D warnings` (tier 2: his go), or
  record it as the open 2.3 lint-stage follow-up with this crate named.
- [counting allocator] SAFETY reasoning right: calls pass to System unchanged; count() never allocates (const-init
  thread_local of a Copy Cell, no destructor, so no lazy init or dtor registration) and never unwinds (try_with,
  saturating ops) - GlobalAlloc must not unwind; the SAFETY line should say both. realloc counts before System.realloc
  can fail (a null leads to handle_alloc_error, so only the count of an aborting run is off). Per-thread counts: work is
  measured on its own thread. Workers: input = f(seed, index) only; stage tallies summed, partition-free. catch_unwind
  works (cargo ignores panic=abort for tests).
- [dependency_rules.rs, Low] the text scan misses two ways in: cargo auto-detects crates/content/build.rs with no
  `build` key (the "no build script" claim at :47 is not held), and a root-level dotted key before [package]
  (`dependencies.x = "1"`) is valid TOML that no `[`-line shows. Fix: assert the file build.rs is absent (or require
  `build = false`), and refuse any key line outside [package]/[lints].
- [runs] `cargo test -p devtools --locked` (debug, own target, 4 test threads): every binary passed (content_fuzz 3 in
  11.8 s, content_proofs 5 in 24.4 s, recordings 5, time_unit 4, aim_reach 1, dependency_rules 2, two_* 1+1).
- [generator, reasoning] map refusal classes: the planted values are off grid / 9 mm / 1 mm past by construction
  (step() works in whole mm; +0.0005 lands half a step off whatever the rounding), and each case keeps the rules the
  loader checks first, so the verdicts are independent; duplicate ids name the later index as check_ids' (id, index)
  sort does. Corpus 'ties' (corpus-texts.mjs:76-82) are 2^44..2^52 with 1-8 binary places: a slice of the tie window
  (P in [-25,-2], any magnitude) and never a power of two, which is why F1 slipped through.
- [time_report.rs] exact path (u128 digits, half up) vs content's f64 path agree on all 69 (the report asserts it);
  units_exact's `* 1_000_000` (time_report.rs:49) can overflow u128 only past ~32 decimal places (a devtools panic on a
  malformed extract, not reachable from content): nit.
- [speed, scratch `speed`, opt-level 3 native x86_64, this 4-core machine, 7 runs each, median] bundle stage (parse +
  streaming canonical compare) at ~8 MB: 48-62 ms = 6.0-7.7 ns/byte (strings), 7.0 ns/byte (numbers in file texts),
  5.8 (keys); the loader stops at the first refused file, and a valid file is small (movement ~0.5 KB, map <= ~100 KB),
  so a bundle's worst case is this stage + the valid files + one refused file of <= 1 MiB. File stage alone: 16,000
  numbers (380 KB) 14.3 ms = 37.5 ns/byte (~0.9 us a number: str::parse + number_text twice, in json.rs:423 and in the
  compare, ~4 allocations each); 16,000 distinct keys 3.6 ms (13 ns/byte); a 1 MB string 4.6 ms (4.6 ns/byte); 16,383
  equal keys 2.0-3.8 ms. Nothing quadratic: every pass is linear, the duplicate search is O(m log m) per object,
  Object::take is O(fields x members) with fields fixed (<= 18). Release fuzz (`cargo test --release -p devtools
  --test content_fuzz -- --nocapture`): 1,000,000 inputs, no panic, 35.4 s on 4 workers; histogram 1867/0/2131/10/
  1508/3306/383/601/190 in 10,000 (all above SHARES); limit cases: 8 x 1 MB bundle 8,006,189 B in, 9,443,515 B held
  (1.18x), 50.5 ms; 16,383 keys 3.8 ms/1.57 MB held; 100,000 keys refused at the node cap in 2.3 ms. Its "slowest
  input" is wall time under 4-way contention: 44.1 ms (input 905734, 2,898 B) on one run, 16.4 ms (input 291018,
  3,716 B) on the next: scheduling noise, not an input that is slow (nit: time it alone, or on thread CPU time).
- [wasm size] content is not in sim-wasm's tree (sim has no dependency on content yet: crates/sim/Cargo.toml), so the
  players' file is unchanged; only the test-only module (devtools cdylib) grows. Agree with the brief.
- [wasm runs] `node node_modules/vitest/vitest.mjs run --root packages/sim-worker --dir packages/sim-worker/boundary
  content` on the main session's module of 15:45 (built after fdd9f0a; 230d9e9 touched only tests/aim_reach.rs; HEAD
  1831a89 changes no crate or package file): 7 passed; verdict digests equal the native ones in both profiles (release
  5.6 s, dev 45.6 s); limits 112 ms release, 1.9 s dev, module memory 51.3 MB (ceiling 64 MiB).
- [no panic, grep] outside tests: no assert!; String::truncate at json.rs:280/:341 cannot panic (the path holds ASCII
  only: keys pass is_key before they are pushed); windows(2); "0".repeat(n<=21); casts after range checks. No
  HashMap/HashSet/Instant/SystemTime in content or the new devtools library code.
- [nits] content_proofs.rs:473 (and the brief's departure 3) say four classes run a tenth: three do (verdicts file: 3
  classes at 50). content_proofs.rs:1101 "the megabyte inputs are spaces": "more values than a document holds"
  (~115 KB of array entries) is folded by its length only. number.rs:74-75 cites Number::toString step 5 for the even
  rule; the normative step leaves the last digit open, the even rule is the spec's note (recommended step 5) that the
  engines follow.
- [files.rs, Low] read_plain checks type and size on the path (symlink_metadata), then std::fs::read reads whatever the
  path is at that moment: the cap and the type are not tied to the bytes read. Open once, check the handle's metadata
  (is_file, len), read through take(cap + 1). Threat model: security-auditor's.
## Departures (my verdicts)
(1) fine: content stays dependency-free; the generator needs sim's ChaCha8/fnv and runs in the test module. (2) fine.
(3) fine (it is three classes, not four: nit). (6) fine: space(), refuse_start and string() all name a byte outside
0x20-0x7e first. (7) the rule is right (closest, then even: what V8/SpiderMonkey do) but its test is not exact: F1.
(8) fine: on_grid(v) <=> v = fl(k/per_metre); thickness in whole steps is exact; double comparisons of on-grid values
agree with step comparisons (fl(k/1000) strictly increasing for |k| <= 1e7; fl(m/100) = fl(10m/1000)). (10) fine (0:0
documented as "no place" in error.rs:205-207; fuzz_judge holds line==0 <=> col==0). (13) fine (fmt --check clean).
## Not checked
Chromium/Firefox runs (not run: the main session's); AArch64; the TypeScript side (corpus-texts.mjs read only, not
run); legacy-extract.mjs; the plants script's 25 plants (not rerun); a release build of content's own unit tests in
CI terms (ran debug + the release fuzz); security threat model of files.rs (security-auditor's).
DONE
