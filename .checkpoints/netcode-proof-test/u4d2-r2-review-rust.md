# U4d2 review round 2: rust-reviewer (notes)
Seat: rust-reviewer, read only. Log prefix rr (.gate/u4d2/r2-rr-*.log). Copy: .claude/worktrees/u4d2-build (runner only).
Round 1's tree: .gate/u4d2-r1. Read copy: .gate/u4d2-r2. Judged against: advisor-u4d2-r1.md Blocks 1-8 (packet).
(No Edit tool in this seat: the file is rewritten whole with Write each time, its content kept.)

## Plan
1. Read the packet (u4d2-r2-packet.md) and its tests side file; my round-1 notes (u4d2-review-rust.md).
2. FX-1: cargo's rustflags sources and precedence (Cargo book config.html build.rustflags, environment-variables.html;
   cargo source in the registry/toolchain if present): CARGO_ENCODED_RUSTFLAGS, RUSTFLAGS, target.<triple>.rustflags,
   target.<cfg>.rustflags, build.rustflags, CARGO_BUILD_RUSTFLAGS, CARGO_TARGET_<TRIPLE>_RUSTFLAGS, profile.rustflags
   (unstable), host.rustflags, --config CLI, the configs cargo reads (cwd ancestors, CARGO_HOME); the encoded form
   (0x1f-separated); read rustflagSources and buildInputProblems; does "every source with a flag is refused" cover each;
   is the read-back now given exactly what cargo gives rustc. Also build.target forms (CARGO_BUILD_TARGET, build.target,
   `target` in a config as array). Run r1-rr-superset.log's first environment: deps must FAIL naming both sources.
3. FX-2: loader.mjs targetFeatures: LEB128 (overlong, past end, 32-bit edge: 5th byte > 0x0f), the prefixes, missing
   or duplicated section, trailing bytes, count larger than possible; run it on both built modules (measured, bites)
   and on hand-built edge cases (a scratch .mjs under my notes folder, run by the runner).
4. FX-8: hash.rs digest_text, arm_for, compare's `on` default; panics, `as` casts, the unit test (could it pass for the
   wrong reason?); h6's test following.
5. FX-12: crates/hash-wasm/clippy.toml: clippy host measured/bites and wasm32 measured/bites as check.mjs runs it; the
   builder's plant log (does clippy read the file).
6. FX-11: Cargo.toml libm comment vs libm 0.2.16 registry source (configure.rs, arch/x86.rs, arch/x86/fma.rs).
7. Bounds: no new public Rust item, no manifest/Cargo.lock change (diff vs .gate/u4d2-r1), lib.rs doc line only.
8. Runs (runner only): clippy both builds and wasm32, fmt, doc, hash-wasm and lab tests, deps step with r1 env.
9. Verdict.

## Progress
- All plan items 1-9 done (00:38-00:50 UTC). Verdict below.
- Copies: builder's copy and read copy each 18 of 18 OK against fx-hashes-end.sha256 (sha256sum -c, 00:40 UTC);
  `diff -rq` read copy vs builder's spikes/netcode-proof: empty. r1 tree vs fixed: only Cargo.toml, check.mjs,
  check.test.mjs, hash-wasm/loader.mjs, hash-wasm/clippy.toml (new), lab/src/hash.rs, lab/src/lib.rs differ (Cargo.lock,
  every crate manifest, hash-wasm/src/lib.rs, recordings.json, std-sin-scan.txt, co-initial-dump.txt, crates/sim equal).

## Runs (mine, through the runner; result = exit code and the runner's summary lines)
- r2-rr-probe.log (shared), exit 0: my scratch u4d2-r2-review-rust-probe.mjs (prints only), importing the copy's
  loader.mjs and check.mjs.
  - FX-2 on both built modules (copy's .gate/netcode-proof-test/wasm-step/bin): npt-hash-wasm.wasm 114,282 B (sha256
    48b5ec5f..., round 1's) and -bites 130,368 B (d196dd19...), each one target_features section, my independent strict
    varuint32 read = +bulk-memory, +bulk-memory-opt, +call-indirect-overlong, +multivalue, +mutable-globals,
    +nontrapping-fptoint, +reference-types, +sign-ext (round 1's list); moduleProblems [] for both.
  - FX-2 hand-built: refused by name: empty body, +simd128 behind an overlong 5-byte count, count 2^32+1 (5th byte
    0x10), 6-byte count, name length 2^32-1, 6-byte name length, BOM+simd128 (TextDecoder strips the BOM: fails
    closed), -simd128 then +simd128, =relaxed-simd, prefix 0x00, trailing byte, count 2 with one entry, two sections,
    none. Accepted: clean +sign-ext/-simd128, an overlong (spec-valid) 5-byte count, count 0 (empty list), "SIMD128"
    upper-case (no LLVM name). The parse takes 5th-byte values >= 16 (spec-malformed varuint32), but each such value is
    >= 2^32 and ends in an error ("ends before entry" / "runs past its end"), so no malformed LEB reads as a clean list.
  - FX-1 buildInputProblems: r1's pair refused naming both; CARGO_PROFILE_RELEASE_RUSTFLAGS, CARGO_HOST_RUSTFLAGS,
    config rustflags in [target.'cfg(..)'], dotted, spaced dots, inline table, quoted and \u-escaped key, '''literal''',
    multi-line array with a comment, [host], [profile.release], spaced header, CRLF, after a BOM: each refused; build
    target via 'target', `build . target`, `target = []`: refused; target-dir and a target table's linker: not
    (controls). Not refused while cargo would pass something: CARGO_ENCODED_RUSTFLAGS="\x1f\x1f" (cargo gives rustc
    three empty args) and RUSTFLAGS="﻿" (cargo's str::trim keeps U+FEFF); both make rustc fail (next run), so no
    wrong pass. Not refused: CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER (RR2-L1).
- r2-rr-superset.log (-x -k), exit 1 (by design): (1) r1-rr-superset.log's first environment
  `env CARGO_ENCODED_RUSTFLAGS=-Ctarget-cpu=bleeding-edge RUSTFLAGS=-Ctarget-feature=-simd128,-relaxed-simd node
  check.mjs --only deps`: FAIL, exit 1, naming CARGO_ENCODED_RUSTFLAGS ("-Ctarget-cpu=bleeding-edge") and RUSTFLAGS
  ("-Ctarget-feature=-simd128,-relaxed-simd"), "check: 1 step(s) failed" (round 1: PASS). (2) r1's second
  environment (+avx512f / -fma): FAIL, exit 1, both named. (3) `env CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER=
  /nonexistent/wasm-ld node check.mjs --only deps`: PASS, exit 0 (RR2-L1). (4) `rustc --crate-type=lib
  --emit=metadata -o /dev/null /dev/null ""`: exit 1, "error: multiple input filenames provided (first two filenames
  are `/dev/null` and ``)": an empty arg from CARGO_ENCODED_RUSTFLAGS="\x1f" fails rustc. (5) control `node check.mjs
  --only deps`: PASS, exit 0 ("given no rustflags"); it also removed the failed-steps/deps.log runs 1-2 wrote (FX-10).
- r2-rr-lint.log (-x -k), exit 0: `node check.mjs --only fmt,clippy,clippy-wasm32,doc` all 4 PASS ("check: all 4
  steps pass"; clippy and clippy-wasm32 "measured build and bites build both linted"; doc 66.1 s); `cargo test --locked
  -p npt-hash-wasm -p npt-lab`: hash-wasm 5 passed, lab lib 5 passed, lab bin 19 passed 1 ignored; same with
  --all-features: 7, 5, 19 passed 1 ignored (+1 bin test each vs round 1's 18: FX-8's unit test). Pedantic pass (warn
  only, exit 0): on the fix pass's new lines only doc_markdown at hash.rs:426 and :959 (`bitten_name's`, `arm_for's`
  unticked); no cast lint on digest_text or arm_for.
- r2-rr-cargohome.log (shared -k), exit 101 (by design): `env CARGO_HOME= cargo tree --locked -p npt-sim` exit 0
  (offline from ~/.cargo's registry: cargo falls back to ~/.cargo for an empty CARGO_HOME); control
  `CARGO_HOME=/nonexistent-cargo-home` exit 101 (no registry). PROCESS SLIP (mine, Low): the control made cargo create
  /nonexistent-cargo-home (an empty .package-cache and registry/CACHEDIR.TAG), outside .checkpoints/ and .gate/; I did
  not remove it (read-only seat): the main session removes it (`rm -r /nonexistent-cargo-home`). Nothing in any copy.
- Read, not re-run: fx-check1.log / fx-check2.log: all 11 steps PASS each, test 417 / 466, self 71 of 71, wasm 41.8 s
  and 40.5 s; fx-1-plant.log: refusal disabled, tests 38 and 63 fail by name (# pass 69, # fail 2) and the r1 env's
  deps PASSes; fx-2-tf.log: simd128 plant, wasm32 build exit 0, deps PASS, wasm FAIL naming +simd128 for both modules;
  fx-8-plant.log: the length check removed, the new unit test FAILs by name (exit 101); fx-12-plant.log: a RefCell ban
  added, clippy refuses it host measured and wasm32 bites (exit 101): clippy reads crates/hash-wasm/clippy.toml.

## Facts checked
- Cargo book config.html, build.rustflags: "four mutually exclusive sources ... checked in order, with the first one
  being used: CARGO_ENCODED_RUSTFLAGS, RUSTFLAGS, all matching target.<tuple>.rustflags and target.<cfg>.rustflags
  joined, build.rustflags". cargo master src/compiler/build_context/target_info.rs `rustflags_from_env`: a set
  CARGO_ENCODED_RUSTFLAGS, even empty, is used (empty -> no flags; else split at \x1f, no filter); a set RUSTFLAGS is
  used (split(' '), trim, empty dropped); `rustflags_from_target` falls through when empty; then build.rustflags.
  Env forms of the config keys: CARGO_BUILD_RUSTFLAGS, CARGO_TARGET_<T>_RUSTFLAGS (and the unstable
  CARGO_PROFILE_*_RUSTFLAGS, CARGO_HOST_RUSTFLAGS): all end in RUSTFLAGS, all read. So FX-1's rule ("every source
  with a flag is refused") covers every source cargo reads, and on a passing step the read-back is given what cargo
  gives rustc from them: nothing. Edge forms that escape the rule ("\x1f" only, U+FEFF only) fail rustc (run 4).
- build.target: CARGO_BUILD_TARGET and `target` keys refused in every form probed; `--target` overrides it and the
  wasm builds pass it; forced-target/default-target need cargo-features (manifestKeyProblems refuses).
- target.<tuple>.linker: the book: "passed to rustc (via -C linker)", env CARGO_TARGET_<tuple>_LINKER: not read by
  check.mjs (RR2-L1).
- CARGO_HOME: the book's hierarchy ends at `$CARGO_HOME/config.toml` (default $HOME/.cargo); cargo treats an empty
  CARGO_HOME as unset (run above); check.mjs:1771 does not (RR2-L2).
- libm 0.2.16 (registry, read only): configure.rs:82-90 emit_intrinsics_cfg, :94-101 emit_arch_cfg (`arch`, default:
  Cargo.toml:40-41); src/math/arch/mod.rs:12 wasm32 module only with intrinsics_enabled, :17-19 the x86 module under
  sse2 (sqrt, sqrtf, fma, fmaf); arch/x86.rs:13 `sqrtss` (sqrtf), :26 `sqrtsd` (sqrt); arch/x86/fma.rs:10-26 fma's
  select_once! dispatch (FMA, FMA4, generic), :28-44 fmaf's. So Cargo.toml:53-55's corrected comment holds; its
  fma.rs:9-27 cite covers fma only (RR2-L4 b).
- FX-8: digest_text (hash.rs:992-1003): no index (get), no cast, u64::from shifts; equal length means every `at`
  index is in range (at comes from own's indexes). arm_for (:428-430) is Option::filter. compare computes `on` once by
  bitten_name over `listed` (manifest order, the recordings either list holds) and passes armed(e) at :1042, :1071,
  :1083; an `--on` not listed is now a usage error (exit 2), failing closed. check.mjs always passes `--on` with the
  bite (check.mjs:2373, :2436), so the default is not exercised by the step (RR2-L3).
- FX-12: the crate's file shadows nothing (no workspace-root clippy.toml); its [lints.clippy] keeps
  disallowed_types/methods deny (hash-wasm Cargo.toml); every path resolves on both targets (clippy -D warnings exit 0;
  the lab file's tokio entries need allow-invalid, this one has none).
- Bounds (Block 8): no manifest or Cargo.lock change; hash-wasm/src/lib.rs byte-equal; Rust additions are two private
  fns and one #[test] in hash.rs and one doc line in lab/src/lib.rs: no new public item.

## Findings (a line after each)
- RR2-L1 (Low) check.mjs:2001-2005 (the limits) and loader.mjs:13-14: a linker set by CARGO_TARGET_<TUPLE>_LINKER or
  a config's target.<tuple>.linker reaches rustc as `-C linker=` and is neither refused nor named beside RUSTC_WRAPPER;
  a linker program writes the module, its target_features section included (e.g. rust-lld with --no-check-features
  --features=sign-ext), so "a flag reaching rustc by a route check.mjs does not read is refused too" does not hold for
  it. Proof: r2-rr-superset.log run 3 (deps PASS with the variable set). Same class as RUSTC_WRAPPER (allowed). Fix:
  refuse the variable and the key as COMPILER_ENV/COMPILER_KEYS do, or name it in the limits. Not before merge.
- RR2-L2 (Low) check.mjs:1771 `env.CARGO_HOME ?? join(homedir(), ".cargo")`: an empty CARGO_HOME is kept (reads
  "config.toml" relative to node's cwd), while cargo falls back to ~/.cargo (r2-rr-cargohome.log); a relative one is
  resolved against node's cwd (the usage line runs from the repo root) while cargo resolves it against HERE. With
  CARGO_HOME="" a ~/.cargo/config.toml's rustflags or build.target is neither refused (FX-1) nor given to the
  read-back, and its lint-lowering flags escape cargoConfigProblems; the Wasm features are still caught by FX-2, the
  native ones (+fma, llvm-args, --cfg feature="bites", overflow-checks=off) are not. Pre-existing code; FX-1's claim
  rests on it. Fix: `||` for `??`, a relative value resolved against `start`, one self-test. Not before merge.
- RR2-L3 (Low) hash.rs:1036-1037, :1042, :1071, :1083 with the test at :1406-1440: EA-9's new asserts test
  bitten_name and arm_for directly; compare's use of them is pinned by no test (and check.mjs always passes --on), so
  compare going back to arming every recording leaves every test and step green. Settling run (test seat): plant
  `armed(e)` -> `arm.as_ref()` at the three calls, `cargo test --locked -p npt-lab --all-features --bin npt-lab` stays
  19 passed. No verdict depends on it today. Not before merge.
- RR2-L4 (Low, texts) (a) check.mjs:1548-1550 "Cargo gives rustc the first non-empty one of ...": the book says "the
  first one being used", and cargo's rustflags_from_env uses a set-but-empty CARGO_ENCODED_RUSTFLAGS or RUSTFLAGS
  (dropping the config's); no result changes under FX-1. (b) Cargo.toml:55's `arch/x86/fma.rs:9-27` cites fma's
  dispatch only; fmaf's is :28-44. Not before merge.

## Verdict
APPROVE WITH FINDINGS: four Lows (RR2-L1 to RR2-L4), none before the merge, none a Medium or above. FX-1 meets its form
and proof (r1's first environment now FAILs naming both sources, exit 1), FX-2's parse is sound and accepts both built
modules, FX-8 has no panic or cast and its unit test bites, FX-12's file is read by clippy in both builds and for
wasm32, FX-11's libm comment matches libm 0.2.16 (one cite range short), and nothing in Rust exceeds Block 8's bounds.
Process slip (mine): /nonexistent-cargo-home, created by cargo in my control run, for the main session to remove.
