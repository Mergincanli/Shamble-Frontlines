# security-auditor: dependency hold for 4c (libm =0.2.16 direct in sim; content as dev-dep of room, client-endpoint)

Worktree `.gate/cs4b-2`, branch `cs/4c-soldier`, head 8bee8365. Batch: patch `deps-4c.diff`, scratch worktree
`.gate/cs4b-2/.gate/deps-4c` (detached at 8bee8365). Read only; offline cargo read commands only.

## Plan
1. Digests: `sha256sum -c deps-4c.sha256` in the scratch worktree (five files) and the diff; re-diff the scratch
   worktree against 8bee8365 myself (no sixth file changed); `git apply --check` the patch on the branch.
2. Lockfile: packages set and every (name, version, source, checksum) equal before/after; only the dependency lists of
   sim, room, client-endpoint change.
3. libm features: re-run `cargo tree -e features -i libm` before (branch) and after (scratch), compare with the files.
4. Release profile overflow overrides: libm present; every package in every view has one (no new package, so as before).
5. Players' file: `cargo tree --offline --locked -p sim-wasm -e normal` (wasm32 and host) before and after: equal.
   Also the build.mjs feature lists, if read-only.
6. Manifest hygiene: no [patch], [replace], git, registry, out-of-repo path; ASCII, LF.
7. Item 3 of the brief: what the built-file check / source scan / reviewed maths list must gain when exp is called.
8. Answer, DONE.

## Notes (one line per finding, as made)
- [x] 1 digests. `sha256sum -c deps-4c.sha256` in `.gate/deps-4c`: five OK (root 099d224b..., lock b7c6ee5d..., sim
  c7432b03..., room 79b829a9..., client-endpoint 22e2c5bf...); diff 4865c3f9... (sha256sum of the file). Patch applied
  with `patch -p1` to `git show 8bee8365:<path>` copies of the five files (scratchpad) gives exactly the five digests:
  the diff is the batch's full text, no sixth file. `git apply --check` on the branch: OK. Branch tree's manifests are
  8bee8365's (git status: only .checkpoints changes). Re-checked after all cargo calls: five OK, nothing rewritten.
- [x] 2 lockfile. 56 [[package]] before and after; the only lock changes are three dependency-list lines (client-endpoint
  +content, room +content, sim +libm). No package added, removed or moved; every `source` line is crates.io's registry
  (no git, no other registry). libm 0.2.16 entry unchanged (checksum b6d2cec3..., already vetted in 4b-2: its build.rs
  and sources read then).
- [x] 3 libm features. `cargo tree --offline --locked -e features -i libm`: re-run on branch = before file byte for byte;
  re-run in scratch = after file modulo the worktree path. Before vs after: one added line (sim as a dependent of libm
  "default"); enabled features `default`, `arch` in both. Host target view.
- [x] 5 players' file. `cargo tree -p sim-wasm -e normal` (host and wasm32): the tree gains exactly one edge line,
  `sim -> libm v0.2.16` (the intended normal dependency); room and client-endpoint show no `content` child. As
  deduplicated sets (`--prefix none -f '{p} {f}'`, wasm32): 55 = 55, equal. The build.mjs views as sorted feature
  sets (players' Wasm wasm32; devtools wasm32; workspace normal,dev host; workspace normal host; workspace
  normal,dev,build host): 104/107/123/117/72 entries, each EQUAL before vs after. So sim-feature-lists.json holds.
- [x] 4 overflow overrides. No new package. After-state root Cargo.toml: 36 `[profile.release.package.*]` headers;
  every third-party crate in the players' Wasm and the native normal+dev views has one; the only ones without are
  build-machine crates (proc macros and build deps: autocfg, bumpalo, num-derive, proc-macro2, quote, rustversion, syn,
  thiserror-impl, ts-rs-macros, wasm-bindgen-macro(-support)), as before.
- [x] 6 hygiene. Patch pure ASCII, no CR; no git, registry, [patch], [replace] or out-of-repo path. Dev-dep paths are
  `../content` (in-repo).
- [x] 7a companion test. crates/devtools/tests/dependency_rules.rs:141-148 asserts sim's `[dependencies]` is exactly
  `content` and `parry3d` ("the simulation's dependencies are the content crate and the collision library, and
  nothing else"): this batch turns that test red (sim's table gains `libm = { workspace = true }`). The batch cannot be
  committed alone. The update must tighten, not loosen: name the libm line exactly, pin the root's `libm = "=0.2.16"`
  line to the byte (as LIBRARY does for parry3d, :125/:154-163), pin Cargo.lock's libm to ["0.2.16"] (PINNED), and add
  "only sim among our manifests names libm" (as the parry3d askers check, :176-194): another of our crates asking
  libm with `unstable-intrinsics` or `force-soft-floats` would change sim's maths in every build alike, and the
  feature compare (build-checks.mjs:442-466) compares builds with each other only (the committed
  sim-feature-lists.json feeds only build-checks.test.mjs:531), so it would stay silent. room/client-endpoint's
  dev-dependency lines touch no current rule (CONTENT_TABLES applies to content's manifest only).
- [x] 7b comments. room's and client-endpoint's new comments cite "devtools' tests/layering.rs holds that": no such
  file at 8bee8365 (crates/devtools/tests/ has 14 files, none layering.rs). True only once the test lands; cargo
  itself already keeps a dev-dependency out of non-test code (it does not compile), so the open gap is `#[cfg(test)]`
  code inside src/ only. Low. Root comment "each routine it brings is on the built-file check's reviewed list"
  (maths-allowed.mjs holds C::round and C::__multi3 only): true as a rule once exp is linked, not as a fact today. Low.
- [x] 7c libm 0.2.16 expf/exp read (cached source): `select_implementation!` overrides only under `x86_no_sse`
  (configure.rs:115-117: x86 without SSE), so wasm32, x86_64 and aarch64 run the same generic code; no fma or mul_add
  in expf.rs, exp.rs or generic/scalbn.rs; expf calls `scalbnf` -> `generic::scalbn` (a second libm path the built
  file may print if not inlined). Release builds use `i!` = `get_unchecked` (math/mod.rs:9-13): expf's `HALF[sign]`
  with sign = hx >> 31 in {0, 1}: sound by construction. `force_eval!` = read_volatile (mod.rs:3-6). `unstable-
  intrinsics` (configure.rs:87) and `force-soft-floats` are the two features that would change which code runs.
- [x] 7a re-read against the ADRs. ADR 0002 part 3's table (docs/adr/0002-code-structure-build-toolchain.md, part 3 row
  `sim`): "`content` only (and its maths dependencies, ADR 0004)"; ADR 0004 part 7 (:283): "Transcendental maths only
  through the pure-Rust `libm` crate, pinned". Mustafa's FEATURE-COMPARE (a) (DECISIONS.md:536) pins parry3d's version,
  safe_arch's and thiserror's, and parry3d's std: libm is not in it. So the rule's update is planned, not a loosening of
  his words: a condition on the commit, not an escalation. advisor-reread-4c.md:37 also puts "the layering test in the
  commit that adds the room's dev-dependency".
- [x] 4 corrected. Overflow overrides per view, after state: players' Wasm (wasm32, normal, no proc macros) 33
  third-party crates, native tests (host, normal+dev, no proc macros) 35: none lacks a `[profile.release.package.*]`
  block (libm included). The 11 without one appear only on build edges (v5), never linked. Unchanged before vs after.
- [x] 3 completed. Before tree lines 56-88: libm "default" (and through it "arch") asked by num-traits 0.2.19 (and
  simba); after adds sim as one more asker. The full feature sets also compared on wasm32 (views v1, v2): equal.
- [x] 5 worded. `cargo tree -p sim-wasm -e normal` DOES change, by one line: the new direct edge sim -> libm v0.2.16.
  The package set and the feature set are equal before and after. No `content` under room or client-endpoint.
- No agent memory folder in this environment (`.claude/agent-memory*` absent): earlier decisions from the 4b notes.

## Answer
Verdict: APPROVE, on exactly: Cargo.toml 099d224b85191d65ebd2f410c8e0c9a5bc5ef91d2def7ea136ed46aec56a85f5, Cargo.lock
b7c6ee5de5fe0a89ba819a1f67159d481763758bedfa67f607401a5a25468910, crates/sim/Cargo.toml
c7432b0346817b407417e38b264c462450c8168694b0f39a84b02218288a65c8, crates/room/Cargo.toml
79b829a9ee954108082b0ec83aeeb1c4553fe67806b0c3dfc69f4c5d50319c5c, crates/client-endpoint/Cargo.toml
22e2c5bf5795e9a93584dbf23abb4dc06645121d98563e5ca560f6b3790a58f9, and the patch deps-4c.diff
4865c3f9b8c52d4e223ffd77ddce718f0b3956c230d25392bc11620c0ac7e710 (it rebuilds exactly those five from 8bee8365).
Checked: no package added, removed or moved (56 = 56; three dependency-list lines only); libm's features default+arch
before and after; overrides cover every linked crate; the players' Wasm's package and feature sets unchanged (one new
edge line, sim -> libm).

Findings (none blocks the batch; each is a condition on the commit that holds it):
- F1 Medium. dependency_rules.rs:141-148 goes red with this batch (sim's table is pinned to content + parry3d). Fix in
  the same commit, tightened: the exact libm line in sim; the root's `libm = "=0.2.16"` pinned to the byte; Cargo.lock
  libm == ["0.2.16"]; only `sim` among our manifests names libm (as the parry3d askers check). Reason: another crate
  of ours asking libm's `unstable-intrinsics` or `force-soft-floats` changes sim's maths in every build alike, which
  the build-vs-build feature compare (build-checks.mjs:442-466) cannot see. ADR 0002 part 3 plans libm; no word of
  Mustafa's is loosened.
- F2 Low. room's and client-endpoint's comments cite devtools' tests/layering.rs, which does not exist at 8bee8365.
  Closes when layering.rs lands in the same commit (advisor-reread-4c.md:37); else the batch's comments must be
  reworded, which is a new batch and a new answer. Root comment "each routine it brings is on ... maths-allowed.mjs":
  true once the exp entry lands; same closing condition.

Item 3, for rust-reviewer's review of the code that first calls the exponential:
- Source scan (crates/sim/tests/rules_scan.rs:386 refuses `libm::exp(`): one allowance for one named call site (file
  and function, the easing-share setup), with a control (that site passes) and bites (the same call elsewhere;
  `f32::exp`, `.exp()`, `libm::exp2`, `libm::expm1`, `libm::powf` still refused). No crate- or file-wide allowance.
- maths-allowed.mjs: the exact paths the built file prints (`libm::math::expf` or `libm::math::exp`, plus any helper
  left un-inlined, e.g. `libm::math::scalbnf` / `libm::math::generic::scalbn`), each with its why. Never a
  `libm::math` prefix: an entry covers every step below it. BANNED_MATHS keeps the plain `exp`/`expf` names refused.
- If the routine is inlined and no libm name is printed, nothing in the built file shows it: the scan's one allowance
  and PM-0004-22's vectors are then the guards (the SIMD and fused multiply-add checks still read the inlined body).
- PM-0004-22 (ADR 0004 :1143): at least 10,000 fixed inputs (edge cases and the content's ease-time range) give
  bit-identical outputs natively and in Wasm in Node and each browser engine; largest error in ulp vs a reference;
  libm's cost reported. Also: the share computed once at match setup, never per tick, and its input from
  content checked finite and in range (no NaN reaches it).
- Feature pin: libm stays exactly `default` + `arch`; `unstable-intrinsics` and `force-soft-floats` change which code
  runs (configure.rs:87, :98). F1's rule holds the asker side.
- libm 0.2.16 read: expf/exp override arch only for x86 without SSE (configure.rs:115-117), so wasm32, x86_64 and
  aarch64 run one generic code path; no fma or mul_add in expf.rs, exp.rs or generic/scalbn.rs; release `i!` is
  `get_unchecked` (math/mod.rs:9-13), sound in expf (index = sign bit, 0 or 1); overflow checks on for libm (our
  override) with no reachable integer overflow seen (k bounded near +-150). Worth one native exhaustive run of the
  chosen routine over all 2^32 f32 inputs: no panic, and bits against the reference.

DONE
