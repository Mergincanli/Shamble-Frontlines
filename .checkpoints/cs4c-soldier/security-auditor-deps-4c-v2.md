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
