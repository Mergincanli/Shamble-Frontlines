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
