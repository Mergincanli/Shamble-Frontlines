# security-auditor: dependency hold for 4c, second answer (reworded root comment, deps-4c-v2.diff)

Builds on security-auditor-deps-4c.md (APPROVE with F1, F2, item 3). Read only; offline cargo read commands only.

## Plan
1. Digests: `sha256sum -c deps-4c-v2.sha256` (five files in `.gate/deps-4c`, and the patch); diff v1 vs v2 patch: only the
   root comment's lines differ; patch applied to head copies rebuilds exactly the five digests; `git apply --check`
   on the branch head a4e93679; branch manifests equal to the patch's base.
2. Re-run `cargo metadata --offline --locked` and the libm feature tree in the scratch worktree (unchanged lock: nothing moves).
3. F1, F2 re-read against the new comment.
4. Item 3 corrected by RR1 (scan rule for libm as a collision-library-like rule) and RR4 (sincos, rem_pio2, rem_pio2_large, exp);
   sincos's code read (cached libm 0.2.16 source) for anything new: feature/arch overrides, fma, get_unchecked, panics.
5. Answer, DONE.

## Notes
- [x] 1 digests. `sha256sum -c deps-4c-v2.sha256` in `.gate/deps-4c`: five OK (root fc436ec3..., lock b7c6ee5d...,
  sim c7432b03..., room 79b829a9..., client-endpoint 22e2c5bf...); the patch itself 0edaf8f3... (sha256sum of the file
  in the notes folder; the sha file names it by a branch-relative path, so -c from the scratch worktree cannot open it).
  `diff deps-4c.diff deps-4c-v2.diff`: only the root hunk header (+6,11 -> +6,12) and the comment's 3 lines -> 4 lines;
  the `libm = "=0.2.16"` line and every other hunk byte-identical. Patch applied with `patch -p1` to `git show
  a4e93679:<path>` copies (scratchpad v2/) rebuilds exactly the five digests: no sixth file. The five manifests are
  identical at 8bee8365 and a4e93679 (`git diff --stat` empty). `git apply --check` on the branch (head a4e93679): OK.
  Patch pure ASCII, no CR. Branch tracked changes: only .checkpoints (rust-reviewer's notes).
- [x] 2 cargo. `cargo metadata --offline --locked` in scratch: exit 0. `cargo tree -e features -i libm` in scratch =
  the stored after file modulo the worktree path; libm features still `default` + `arch`. Lock digest unchanged, so the
  package set, players' Wasm view and override coverage of the first answer stand unchanged.
- [x] 3 F1/F2 anchors at a4e93679. dependency_rules.rs:141-148 still pins sim's [dependencies] to content + parry3d
  (red with the batch, as before); no tests/layering.rs in crates/devtools/tests (F2's first half unchanged). Root
  comment now "each routine it brings is on the built-file check's reviewed list with its vectors": maths-allowed.mjs
  at head holds C::round and C::__multi3 only, and its header says "The simulation itself calls no maths routine"
  (maths-allowed.mjs:9-10): both must move in the commit where exp/sincos first link.
- [x] 4a RR1 read. rules_scan.rs:52-62 STD (std names only), :385-387 path rule keys on the last name; NAMED :114-117.
  LIBRARY rule (:89-96, :325-353) allows a non-pub, non-`as` `use parry3d::..` in collision.rs; RR1's libm rule is
  stricter (no `use` at all, full-path calls only), right for libm because a bare call after an import is what the
  name list cannot see. Extra holes the rule must close (my read): `::libm::x(` (leading `::`), `extern crate libm as
  m;`, `use libm as m;`, a renamed dependency in a manifest (`m = { package = "libm" }`: the scan keys on the token
  `libm`, so F1's exact-line pin of sim's table is what closes that one), and `#[path]` (already refused).
- [x] 4b RR4 vs libmProblems (build-checks.mjs:776-788): an entry covers itself and `entry::*` only, so
  `libm::math::sincos` does not cover `libm::math::sincosf::*`, and `libm::math::exp` does not cover `expf`, `exp2`,
  `expm1` (separate modules). Entries must be exact module paths, never `libm::math`. BANNED_MATHS (:728-732) holds the
  C names sincos/exp (unmangled), untouched.
- [x] 4c libm 0.2.16 sincos family read (cargo cache). sincos.rs, rem_pio2.rs, k_sin.rs, k_cos.rs: no
  select_implementation, no fma/mul_add: one generic path on wasm32, x86_64, aarch64 (PM-0004-26 unaffected). exp.rs:86
  overrides only under x86_no_sse (as in answer 1). rem_pio2.rs: large path only for ix >= 0x413921fb (|x| >= ~2^20
  pi/2) and NaN/inf return early (:162); rem_pio2_large uses `i!` (get_unchecked in release) 49 times: sound upstream
  for its domain, unreachable for r in [0, pi/2). Our profile turns overflow checks ON for libm (Cargo.toml:110-111),
  a configuration upstream's no_panic CI does not exercise: a panic there would be a server DoS. Reachability is held by
  content's range check (finite, [0, 360)) and the quarter-turn reduction, so item 3 asks for: content refusing NaN,
  inf, negative and >= 360 facings (with bites), and the vectors run natively with overflow checks on (release
  profile), no panic. Input is content (authored, loaded server-side), not network data.
- [x] 1 re-checked after the cargo calls: `sha256sum -c` of the five in `.gate/deps-4c`: five OK (lock not rewritten).
  The scratch worktree's own git status was not read (the guard refuses git with -C, and a cd in front of git is
  refused by the brief's shell rule); the five digests and the patch's rebuild from a4e93679 carry "nothing else moved"
  for the batch, which is the patch, not the scratch tree.
- [x] 4c corrected. Scope of "one generic path": `cfg` grep of sincos.rs, rem_pio2.rs, k_sin.rs, k_cos.rs,
  rem_pio2_large.rs, floor.rs: sincos.rs:58-61 debug_assertions only (an unreachable `_` arm of `n & 3`);
  rem_pio2.rs:56 x86 without sse2 only; rem_pio2_large.rs:29/:41 the IPIO2 table's LENGTH differs by pointer width
  (66 on wasm32, 690 on 64-bit; the extra entries serve quad precision), and :237 a debug_assert. So the reached path
  (direct |x| <= pi/4, medium rem_pio2) is one code on wasm32, x86_64 and aarch64; rem_pio2_large is the one place
  that differs by target, and it is unreachable for r in [0, pi/2). Withdrawn: my "upstream's no_panic CI does not
  exercise overflow checks" (no source read for it). Kept: our profile has overflow checks on for libm
  (Cargo.toml:110-111), so the vectors run natively in the release profile must show no panic.
- [x] 2' input grounded. The facing is content: crates/content/src/map.rs:66 `FACING` range [0, 360) (min inclusive,
  max exclusive), read at :185 `entry.number(&FACING)?`; JSON has no NaN or infinity; schema.rs:605-619 bites 360 and
  the double below 0. -0.0 passes `>= 0.0` (RR3's `+ 0.0` canonicalisation answers it). Loaded server-side from our
  own bundle: a panic would be a server DoS from content data, so the check stays in scope.
- [x] 5 scan deadline. The batch's manifest line is what makes `libm::` resolvable in sim; from that commit on,
  `libm::expm1(` would compile and pass the scan. So RR1's rule lands in the commit that applies this batch (F1's
  commit), even if exp or sincos first joins later; the allowances join with their call sites.

## Answer
Verdict: APPROVE, on exactly: Cargo.toml fc436ec36d3d54808012df5f6d830a11f80fb2e2272c8f9c0ec6975da35a49a8, Cargo.lock
b7c6ee5de5fe0a89ba819a1f67159d481763758bedfa67f607401a5a25468910, crates/sim/Cargo.toml
c7432b0346817b407417e38b264c462450c8168694b0f39a84b02218288a65c8, crates/room/Cargo.toml
79b829a9ee954108082b0ec83aeeb1c4553fe67806b0c3dfc69f4c5d50319c5c, crates/client-endpoint/Cargo.toml
22e2c5bf5795e9a93584dbf23abb4dc06645121d98563e5ca560f6b3790a58f9, and the patch deps-4c-v2.diff
0edaf8f3ff1610f6a29ae71ec22ae4d9028157d5d3e6e34672e9c723ce7c8b7b. Checked: v1 -> v2 differs only in the root comment
(3 lines -> 4, its hunk header); the patch rebuilds exactly the five from a4e93679 (whose manifests equal 8bee8365's);
`git apply --check` OK; `cargo metadata --offline --locked` exit 0; libm features default + arch, tree = the first
answer's after file modulo path. Pure ASCII, no CR. No new finding on the batch.

F1 (Medium) stands as written (dependency_rules.rs:141-148 still red with the batch; tightened update in the same
commit), and gains: the exact-line pin of sim's table is also what stops a renamed libm dependency
(`m = { package = "libm" }`) slipping past the scan, which keys on the token `libm`.
F2 (Low) stands for layering.rs (still absent). Its root-comment half now closes when the same commit holds: one
maths-allowed.mjs entry per libm path the players' real build prints (expected libm::math::exp, ::sincos, ::rem_pio2,
::rem_pio2_large; exact module paths, no entry covering a sibling, never a `libm::math` prefix), each with its why;
the sincos and exp vectors; and maths-allowed.mjs:9-10's "The simulation itself calls no maths routine" rewritten.

Item 3, corrected:
- Scan (RR1, replaces answer 1's first bullet): libm under the collision library's rule, stricter: named in one file
  of sim only, never imported (no `use libm`, `use libm::{..}`, `use libm::*`, `as`, `pub use`, `extern crate`),
  full-path calls `libm::<name>(` only, one allowance per (function, routine) pair (easing-share setup -> exp,
  spawn-facing function -> sincos), every other libm path refused by construction. Bites: `libm::expm1(` and
  `libm::sincos(` in another function of that file, `libm::sincosf(`, `libm::expf(`, `use libm::{exp};` + bare
  `exp(`, `use libm::*;`, `::libm::exp(` outside the site, `extern crate libm as m;`, libm named in a second file.
  Controls: the two sites pass. Lands in the commit that applies this batch (the line makes libm resolvable).
- Maths list (RR4): the entries above, from the real build's names (libmProblems, build-checks.mjs:776-788, covers an
  entry and its `::` children only). BANNED_MATHS keeps the C names exp/sincos refused.
- PM-0004-22 vectors for sincos (RR4): the 90,000-point 0.001-degree grid on [0, 90), >= 10,000 seeded doubles, the
  edges (0, 45 and the double below it, the double below 90, r under 2^-27 sqrt 2), the full facings 0/90/180/270 and
  the double below 360 through the quarter turns (no -0.0 out: RR3's `0.0 - v` and `+ 0.0`, with the `-v` plant);
  K pinned by bits (0x3f91df46a2529d39); bit-identical natively, in Node and each engine, against the correctly
  rounded reference; and run natively in the release profile (overflow checks on for libm, Cargo.toml:110-111) with
  no panic. Exp's vectors as in answer 1.
- Domain (new): rem_pio2_large is linked but is the only target-dependent code (rem_pio2_large.rs:29/:41) and is
  unreachable only because content refuses facings outside [0, 360) (map.rs:66, :185; schema.rs:605-619). That check
  stays, with its bites; the reduction to r in [0, 90) is tested to hold for every accepted input (RR3's Sterbenz run).
- Feature pin: unchanged (default + arch; unstable-intrinsics and force-soft-floats refused). sincos needs no feature.
  Reached path (direct and rem_pio2 medium) has no arch override and no fma (cfg grep of the six files).

DONE

## Third answer (2026-11-28): F2's layering.rs reworded to read non-test tokens only

### Plan
1. Digests of the five files at the branch head (cs/4c-soldier) against dependency-check.md:30-34 and deps-4c-v2.sha256.
2. Read engine-architect-read.md block 7 (item 5 point 15, T-G), rust-reviewer-read.md block 14.
3. Read rules_scan.rs's test-only marking (~:695-718) and room's lib.rs tests (~:304-669): what counts as "test".
4. Holes a test-only marking could let through; the plants each needs.
5. Anything in blocks 7/14 that changes the batch commit's content. Answer, DONE.

### Notes
- [x] 1 digests at head bfae40ba. The five manifests in the worktree are the batch's BASE (unapplied): their own sha256
  (root a854d5e3..., lock 56bd2d52..., sim 1b19f15a..., room 83aeccc1..., client-endpoint ccf21ea7...) are not the
  approved digests, as expected; the approved digests are the patched results. `git diff --stat a4e93679 bfae40ba` on
  the five: empty (base unchanged since approval); worktree status: only this notes file modified. Patch digest
  0edaf8f3... unchanged. Head copies (scratchpad v3/) + `patch -p1` of deps-4c-v2.diff rebuild exactly the five
  approved digests (fc436ec3, b7c6ee5d, c7432b03, 79b829a9, 22e2c5bf), 5 files, no sixth. `git apply --check` at head OK.
- [x] 2 blocks read. engine-architect-read.md:1078-1081 (item 5 point 15) and :1097 (T-G: plant `use
  content::map::MapDocument;` in a copy of room's non-test source); rust-reviewer-read.md:1089 (YES to point 15),
  :886 and :918 (layering.rs in the commit that first calls exp/sincos, with the batch), :962 (row 32 = T-G). Q4
  (:1101-1104): no manifest line changes, the five batch files stay byte for byte; F1, F2, item 3 and the scan stay in
  that commit (rust-reviewer :883-890). Nothing in either block moves a batch file or drops a condition.
- [x] 3 marking read. rules_scan.rs:695-760 `in_tests`: test-only = a file whose first 8 tokens are `#![cfg(test)]`, or
  the item after the exact tokens `# [ cfg ( test ) ]` (past further attributes and `pub`/`pub(..)`) whose first
  word is in TEST_ITEMS (:691-693: mod fn use impl struct enum const static type trait unsafe async extern), to its
  first depth-0 `}` or `;`. Anything else (cfg(any(test,..)), cfg(not(test)), cfg(all(test,..)), cfg_attr(test,..),
  `macro_rules!`, statements, fields) is read as shipped code: every non-exact spelling errs toward red (false reds
  possible, e.g. cfg(all(test, unix)); no false pass). Over-marking needs a delimiter miscounted: tokenizer
  rules_scan.rs:129-227 drops comments (nested), every string kind (r#, br, c), char and byte literals, lifetimes and
  numbers, and reads `r#content` as `content` (:212-213). So a test-only item can extend over shipped code only through
  a tokenizer error on a `{` inside a literal or comment: a plant should pin that.
- [x] 3' tree. room/src: lib.rs (tests :304-305), queue.rs (:132); client-endpoint/src: 8 files, six with `#[cfg(test)]
  mod tests`, and lib.rs:11-12 `#[cfg(test)] mod layout_ts;` whose file layout_ts.rs has no `#![cfg(test)]`: a test
  module declared in another file is NOT marked by in_tests (per-file scan), so `content` there would be a false red
  (the conservative side; the author then adds `#![cfg(test)]` at its top, which is truly test-only). No `#[path`,
  `include!`, `include_str!`, `include_bytes!` in either src today; `content` appears there only in `///` comments
  (dropped), and as part of `content_hash` (a different token).
- [x] 3'' compiler. Cargo reference, Specifying Dependencies (doc.rust-lang.org/cargo/reference/specifying-dependencies.html,
  read 2026-11-28): "Dev-dependencies are not used when compiling a package for building, but are used for compiling
  tests, examples, and benchmarks", "not propagated to other packages"; renaming: a `package` key "in the
  `[dependencies]` section of which package should be depended on" (`bar = { .., package = "foo" }`). So the batch's
  dev line already keeps `content` out of a non-test build of room/endpoint (the compile fails); layering.rs is the
  readable second guard. Sure: high for the doc's words; not built here.
- [x] F3 (Low): routes a token scan of src/ cannot see. (a) a manifest edit: `content` moved to [dependencies],
  `c = { package = "content", path = "../content" }` (then `use c::..`: no `content` token), `[dependencies.content]`,
  `[target.'cfg(..)'.dependencies]`; dependency_rules.rs pins content's and sim's manifests only (its grep shows no
  room/client-endpoint), so nothing tests room's and endpoint's tables. (b) code from outside the walked tree:
  `#[path = "../x.rs"] mod x;` and `include!("..")` (the string's text is dropped, so the path is invisible). (c) sim's
  re-exports: sim/src/time.rs:8 `pub use content::time::UNITS_PER_SECOND;` is sim's one, by design (ADR 0002 part 3);
  a new `pub use content::..` in sim reaches room's shipped code as `sim::..`. (a) and (b) are in F2's reach and cheap;
  (c) is sim's layering: a list in the same test is enough. Fix in the plants below.
- [x] F3 corrected (advisor tool's point 2). dependency_rules.rs:176-187 walks every crate's manifest, for parry3d's
  askers only; nothing there tests `content` in room's or the endpoint's tables. But engine-architect's block 4
  (engine-architect-read.md:551-559) already gives layering.rs three halves: room's and the endpoint's
  `[dependencies]` name no `content` (plant: the `content = { path = "../content" }` line), the src scan (the half
  point 15 supersedes, :553), and sim's hand-on of content, today only `pub use content::time::UNITS_PER_SECOND`
  (sim/src/time.rs:8; plant `pub use content::map::MapDocument`). Point 15 supersedes only the src half, so the other
  two stand. F3 shrinks to: (a) the manifest half keys on the package, not the key: plants `c = { package = "content",
  path = "../content" }`, `[dependencies.content]`, `[target.'cfg(unix)'.dependencies]` + content; (b) `#[path` and
  `include!`/`include_str!`/`include_bytes!` refused in both src (none used today); (c) block 4's, nothing to add.
- [x] F4 (Low): the marking over-marks, traced by hand (not run). in_tests (:742) checks the word after the mark,
  not that the mark is in item position, and its walk breaks only on a depth-0 `}` or `;`, so a closing `)` at depth 0
  (saturating_sub) runs on; and the tokenizer turns `r#type` into `type` (:212-213, traced: word "r", hashes 1, no
  quote -> skip `#`, next round pushes `type`). Traces: `struct S { #[cfg(test)] r#type: u8, m: content::map::MapDocument }`
  marks from `#` to the struct's `}`: `content` read as test (false pass). `fn f(#[cfg(test)] r#fn: u8) { content::x(); }`
  runs past `)` and marks the whole function body. `fn f<#[cfg(test)] const N: usize>() { content::x(); }` (`<` not
  counted) marks the whole function (whether rustc takes cfg on a const generic: not checked). Same shape: a struct
  expression's field, a match arm `#[cfg(test)] r#type => ..`. Reach: small (the compiler already refuses a
  dev-dependency in a non-test build, 3''), so Low; but "reused as is" does not hold the rule it states. Fix in
  layering.rs's marking (or the shared one): a mark counts only in item position (file start, or after `;`, `}`,
  `{`, or another attribute's `]`), raw identifiers stay distinct from keywords (e.g. the token `r#type`), and a
  depth-0 closing `)`, `]` or `}` that the item did not open ends the marking unmarked.
- [x] F5 (Low, existing code, not this batch): the same over-mark reaches content's panic scan (rules_scan.rs:765-769
  uses in_tests): `fn f(#[cfg(test)] r#fn: u8) -> u8 { x.unwrap() }` in content would pass it. Clippy's guard in
  content's lib.rs still refuses it where clippy runs (rules_scan.rs:678-680 says the toolchain job runs none). For
  rust-reviewer, a STATUS follow-up unless the shared marking is fixed in this commit, which closes both.
- [x] (3) batch comments. Room (deps-4c-v2.diff:65-66): "tests build a collision world from a map document"; block 7
  has them build rules and spawns too (item 5.9): incomplete, not false; "never names `content`" is what the
  non-test scan holds. True enough: no reword (a reword moves a digest). Endpoint (:78-80): its tests build its copy
  of the world in step 2 (block 7 item 2, item 5.13: nothing beyond endpoint.rs:258 in step 1), so if the batch lands
  in step 1 the endpoint's dev line is unused and the comment is ahead of its tests until step 2 (F6, Low). No
  shipped effect (dev-deps stay out of builds; no `unused_crate_dependencies` lint in Cargo.toml:25-28). Accepted on one
  condition: step 2's commit holds the endpoint's tests that use it; if step 2 leaves 4c-2, the endpoint's two lines
  and comment come out in a new batch, back to me.

## Answer (third)
Verdict on the reworded layering.rs: REVISE (wording of point 15 and T-G; Lows, no blocker on the batch). The
non-test reading meets F2 and its intent: the batch's comments say "the room's code ... never names `content`", and the
room's own unit tests are not its shipped code; Cargo already keeps a dev-dependency out of a non-test build (Cargo
reference, Specifying Dependencies), so the test is the readable second guard. What does not hold is "rules_scan.rs's
marking reused" as it stands (F4). layering.rs lands in the batch's commit with all three halves (block 4's manifest
and sim hand-on halves stand; point 15 replaces only the src half), its marking tightened, and these plants, run in
layering.rs against the marking function it actually uses (shared or not):
- Walk: every `.rs` under room's and client-endpoint's `src/`, recursively; asserts both lib.rs are found (no vacuous pass).
- Rule: any non-test identifier token `content` (not only path heads). Controls: the real tree passes (room's tests
  naming content); `#[cfg(test)] mod tests { use content::map::MapDocument; }` passes; `content_hash` passes; a file
  whose first line is `#![cfg(test)]` naming content passes.
- Red, once each: `use content::map::MapDocument;` in a copy of room's AND of an endpoint source (T-G);
  `#[cfg(any(test, feature = "x"))]`, `#[cfg(not(test))]`, `#[cfg_attr(test, allow(unused))]` before that line;
  `extern crate content as c;`, `use ::content as c;`, `use r#content::map::MapDocument;`; F4's three:
  `struct S { #[cfg(test)] r#type: u8, m: content::map::MapDocument }`, `fn f(#[cfg(test)] r#fn: u8) { content::x(); }`,
  `fn f<#[cfg(test)] const N: usize>() { content::x(); }`; delimiters in literals: `#[cfg(test)] mod tests { const A:
  char = '{'; const B: &str = "{"; const C: &str = r#"{"#; /* { */ } use content::map::MapDocument;`; and
  `#[path = "../x.rs"] mod x;`, `include!("../x.rs");`, `include_bytes!("x")`.
- Manifest half (block 4) red: `content = { path = "../content" }`, `c = { package = "content", path = "../content" }`,
  `[dependencies.content]`, `[target.'cfg(unix)'.dependencies]` with content; control: the batch's dev line.
- Sim half (block 4) red: `pub use content::map::MapDocument` in a copy of sim's source; control: time.rs:8's one.
Known conservative side (no plant): `#[cfg(all(test, unix))]` and `#[cfg(test)] mod layout_ts;`'s file without
`#![cfg(test)]` (client-endpoint lib.rs:11-12) read as shipped: a false red, which the author fixes with `#![cfg(test)]`.

Digests: at head bfae40ba the five batch files are the batch's base, so their raw sha256 do not match the approved
digests, by design (the approved five are the patched results). Result: base unchanged since a4e93679 (`git diff
--stat` empty), patch 0edaf8f3... unchanged, head + patch rebuilds exactly fc436ec3, b7c6ee5d, c7432b03, 79b829a9,
22e2c5bf (five files, no sixth), `git apply --check` OK. The approval stands; no re-review of the batch.

(3) Block 7 and block 14 move no batch file and drop no condition (Q4, rust-reviewer :883-890). They add: F6 (Low,
endpoint dev line unused until step 2, accepted on step 2's commit using it); layering.rs's three halves and F4's
tightened marking in the batch's commit; F5 (Low) to rust-reviewer for content's panic scan.

Not checked: nothing built or run (the traces are by hand); whether rustc accepts cfg on a const generic; layering.rs
itself (not written); the shared-marking placement; sim-wasm's and devtools' own naming of content (allowed).

DONE

## Amendment to the third answer (after the advisor tool's second read)
- [x] Macro, traced by hand: `strip! { #[cfg(test)] mod leak { use content::map::MapDocument; } }` gives tokens
  `strip ! { # [ cfg ( test ) ] mod leak { use content :: .. ; } }`; in_tests sees the exact mark followed by `mod`
  and marks to leak's `}`: `content` read as test. A room macro whose matcher drops the attribute (e.g. `(#[$m:meta]
  $($t:tt)*) => { $($t)* }`) ships `mod leak`. False pass (the compiler still refuses the dev-dependency: Low).
- [x] F4's fix as first written does not close `fn f<#[allow(unused)] #[cfg(test)] const N: usize>() { content::x(); }`
  (the mark follows an attribute's `]`, no raw identifier, the walk ends on the body's own `}`). Withdrawn as the fix.
- [x] F4 fix, replaced: layering.rs uses the NARROW marking, not in_tests: test-only is only (i) a file whose first
  tokens are `#![cfg(test)]`, or (ii) `#[cfg(test)]`, then optional further `#[..]` attributes, then the keyword `mod`,
  an identifier and `{`, standing at the file's top level (brace, bracket and parenthesis depth 0; so never inside a
  macro's delimiters, a struct, a parameter list or a generic list), marked to that module's closing `}`. Raw
  identifiers stay distinct from keywords. Every test block in both crates already has shape (ii) at top level
  (room lib.rs:304-305, queue.rs:132-133; client-endpoint block.rs:94, endpoint.rs:342, layout.rs:251, post.rs:51,
  press.rs:82, schedule.rs:58); lib.rs:11-12's `mod layout_ts;` is a declaration, its file not marked.
- [x] Ruling, `#[cfg(test)]` on a real item that is not a module (`fn`, `use`, `impl`, `const`): truly compiled out, but
  under the narrow marking it is REFUSED (a false red on the safe side; none exists in either crate today, grep
  above): the author moves it into the tests module. A red plant pins that choice: top-level `#[cfg(test)] use
  content::map::MapDocument;` and `#[cfg(test)] fn h() { content::x(); }` are red.
- [x] Plants added to the third answer's list (all red): the `strip!` macro above; `fn f<#[allow(unused)]
  #[cfg(test)] const N: usize>() { content::x(); }`; the two top-level non-module `#[cfg(test)]` items above; and
  `#[cfg(test)] mod tests { .. }` nested inside a non-test `mod m { .. }` is read as shipped (a false red, accepted:
  none exists). Controls unchanged (the real tree, the top-level `#[cfg(test)] mod tests` with content, a `#![cfg(test)]`
  file, `content_hash`). F5 (content's panic scan, rules_scan.rs) unchanged: the narrow marking is layering.rs's; the
  shared in_tests is rust-reviewer's call.
- Verdict, digests and (3) unchanged: REVISE on point 15's "reused" wording and T-G's single plant; the batch's
  approval stands (head + patch rebuilds the five approved digests).
- Not checked, added: my agent memory was neither read nor updated (the brief's write rule limits me to this file);
  the threat-model line "the compiler keeps dev-dependencies out of shipped builds; layering.rs is the second guard"
  is for the session to record if wanted.

DONE
